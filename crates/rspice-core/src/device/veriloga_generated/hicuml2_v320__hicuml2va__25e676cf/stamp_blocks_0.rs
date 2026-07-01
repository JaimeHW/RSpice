#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let assign00_e596: f64 = (p.p148 * (nv8 - nv6));
        locals.var_vbiei = assign00_e596;
        locals.var_vbiei_dn6 = (-p.p148);
        locals.var_vbiei_dn8 = p.p148;

        let assign10_e599: f64 = (p.p148 * (nv8 - nv5));
        locals.var_vbici = assign10_e599;
        locals.var_vbici_dn5 = (-p.p148);
        locals.var_vbici_dn8 = p.p148;

        let assign20_e602: f64 = (locals.var_vbiei - locals.var_vbici);
        locals.var_vciei = assign20_e602;
        locals.var_vciei_dn5 = (-locals.var_vbici_dn5);
        locals.var_vciei_dn6 = locals.var_vbiei_dn6;
        locals.var_vciei_dn8 = (locals.var_vbiei_dn8 - locals.var_vbici_dn8);

        let assign30_e605: f64 = (p.p148 * (nv7 - nv6));
        locals.var_vbpei = assign30_e605;
        locals.var_vbpei_dn6 = (-p.p148);
        locals.var_vbpei_dn7 = p.p148;

        let assign40_e608: f64 = (p.p148 * (nv7 - nv5));
        locals.var_vbpci = assign40_e608;
        locals.var_vbpci_dn5 = (-p.p148);
        locals.var_vbpci_dn7 = p.p148;

        let assign50_e611: f64 = (p.p148 * (nv1 - nv5));
        locals.var_vbci = assign50_e611;
        locals.var_vbci_dn1 = p.p148;
        locals.var_vbci_dn5 = (-p.p148);

        let assign60_e614: f64 = (p.p148 * (nv9 - nv5));
        locals.var_vsici = assign60_e614;
        locals.var_vsici_dn5 = (-p.p148);
        locals.var_vsici_dn9 = p.p148;

        let assign70_e617: f64 = (p.p148 * (nv3 - nv0));
        locals.var_vsc = assign70_e617;
        locals.var_vsc_dn0 = (-p.p148);
        locals.var_vsc_dn3 = p.p148;

        let assign80_e620: f64 = if p.p0 <= 310.0 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign80_e620;

        let (assign90_e624,) = {
    if (locals.var_guard4 != 0.0) {
        (1.6021918e-19,)
    } else {
        (locals.var_p_qel,)
    }
};
        locals.var_p_qel = assign90_e624;

        let (assign100_e628,) = {
    if (locals.var_guard4 != 0.0) {
        (1.3806226e-23,)
    } else {
        (locals.var_p_kb,)
    }
};
        locals.var_p_kb = assign100_e628;

        let (assign110_e633,) = {
    if (locals.var_guard4 == 0.0) {
        (1.602176634e-19,)
    } else {
        (locals.var_p_qel,)
    }
};
        locals.var_p_qel = assign110_e633;

        let (assign120_e638,) = {
    if (locals.var_guard4 == 0.0) {
        (1.380649e-23,)
    } else {
        (locals.var_p_kb,)
    }
};
        locals.var_p_kb = assign120_e638;

        let assign130_e641: f64 = 0.0;
        locals.var_gmin = assign130_e641;

        let assign140_e644: f64 = (p.p146 + 273.15);
        locals.var_tnom = assign140_e644;

        let assign150_e645: f64 = ctx_temp;
        locals.var_tamb = assign150_e645;

        let assign160_e648: f64 = (locals.var_p_kb / locals.var_p_qel);
        locals.var_kb2q = assign160_e648;

        let assign170_e651: f64 = (locals.var_kb2q * 300.0);
        locals.var_vt300 = assign170_e651;

        let assign180_e654: f64 = (locals.var_kb2q * locals.var_tnom);
        locals.var_vtnom = assign180_e654;

        let assign190_e657: f64 = (1.0 / locals.var_vtnom);
        locals.var_ovtnom = assign190_e657;

        let assign200_e660: f64 = (p.p121 * locals.var_tnom);
        let assign200_e662: f64 = (locals.var_tnom).ln();
        let assign200_e663: f64 = (assign200_e660 * assign200_e662);
        locals.var_k10 = assign200_e663;

        let assign210_e666: f64 = (p.p122 * locals.var_tnom);
        locals.var_k20 = assign210_e666;

        let assign220_e669: f64 = (p.p131 * locals.var_tnom);
        locals.var_avs = assign220_e669;

        let assign230_e672: f64 = (p.p117 + locals.var_k10);
        let assign230_e674: f64 = (assign230_e672 + locals.var_k20);
        locals.var_vgb_tnom = assign230_e674;

        let assign240_e677: f64 = (p.p118 + locals.var_k10);
        let assign240_e679: f64 = (assign240_e677 + locals.var_k20);
        locals.var_vge_tnom = assign240_e679;

        let assign250_e682: f64 = (p.p119 + locals.var_k10);
        let assign250_e684: f64 = (assign250_e682 + locals.var_k20);
        locals.var_vgc_tnom = assign250_e684;

        let assign260_e687: f64 = (locals.var_vgb_tnom + locals.var_vge_tnom);
        let assign260_e689: f64 = (assign260_e687 * 0.5);
        locals.var_vgbe_tnom = assign260_e689;

        let assign270_e692: f64 = (locals.var_vgb_tnom + locals.var_vgc_tnom);
        let assign270_e694: f64 = (assign270_e692 * 0.5);
        locals.var_vgbc_tnom = assign270_e694;

        let assign280_e697: f64 = (p.p117 + p.p118);
        let assign280_e699: f64 = (assign280_e697 * 0.5);
        locals.var_vgbe0 = assign280_e699;

        let assign290_e702: f64 = (p.p117 + p.p119);
        let assign290_e704: f64 = (assign290_e702 * 0.5);
        locals.var_vgbc0 = assign290_e704;

        let assign300_e707: f64 = (p.p120 + p.p119);
        let assign300_e709: f64 = (assign300_e707 * 0.5);
        locals.var_vgsc0 = assign300_e709;

        let assign310_e713: f64 = (p.p121 / locals.var_kb2q);
        let assign310_e714: f64 = (3.0 - assign310_e713);
        locals.var_mg = assign310_e714;

        let assign320_e717: f64 = (locals.var_mg + 1.0);
        let assign320_e719: f64 = (assign320_e717 - p.p130);
        locals.var_zetabci = assign320_e719;

        let assign330_e722: f64 = (locals.var_mg + 1.0);
        let assign330_e724: f64 = (assign330_e722 - p.p138);
        locals.var_zetabcxt = assign330_e724;

        let assign340_e727: f64 = (locals.var_mg - 1.5);
        locals.var_zetasct = assign340_e727;

        let assign350_e730: f64 = (1.0 - p.p107);
        let assign350_e733: f64 = (p.p52 + p.p106);
        let assign350_e734: f64 = (assign350_e730 * assign350_e733);
        locals.var_c_1 = assign350_e734;

        let assign360_e737: f64 = if locals.var_c_1 >= p.p106 { 1.0 } else { 0.0 };
        locals.var_guard5 = assign360_e737;

        let (assign390_e751,) = {
    if (locals.var_guard5 != 0.0) {
        let assign390_e749: f64 = (locals.var_c_1 - p.p106);
        (assign390_e749,)
    } else {
        (locals.var_cjcx01,)
    }
};
        locals.var_cjcx01 = assign390_e751;

        let (assign400_e757,) = {
    if (locals.var_guard5 != 0.0) {
        let assign400_e755: f64 = (p.p52 - locals.var_cjcx01);
        (assign400_e755,)
    } else {
        (locals.var_cjcx02,)
    }
};
        locals.var_cjcx02 = assign400_e757;

        let (assign430_e774,) = {
    if (locals.var_guard5 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cjcx01,)
    }
};
        locals.var_cjcx01 = assign430_e774;

        let (assign440_e779,) = {
    if (locals.var_guard5 == 0.0) {
        (p.p52,)
    } else {
        (locals.var_cjcx02,)
    }
};
        locals.var_cjcx02 = assign440_e779;

        let assign470_e788: f64 = if p.p22 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign470_e788;

        let (assign480_e794,) = {
    if (locals.var_guard6 != 0.0) {
        let assign480_e792: f64 = (1.0 / p.p22);
        (assign480_e792,)
    } else {
        (locals.var_otbhrec,)
    }
};
        locals.var_otbhrec = assign480_e794;

        let (assign490_e799,) = {
    if (locals.var_guard6 == 0.0) {
        (0.0,)
    } else {
        (locals.var_otbhrec,)
    }
};
        locals.var_otbhrec = assign490_e799;

        let assign500_e802: f64 = if p.p0 <= 300.0 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign500_e802;

        let (assign510_e806,) = {
    if (locals.var_guard7 != 0.0) {
        (0.0,)
    } else {
        (locals.var_v_btbmax,)
    }
};
        locals.var_v_btbmax = assign510_e806;

        let (assign520_e811,) = {
    if (locals.var_guard7 == 0.0) {
        (0.7,)
    } else {
        (locals.var_v_btbmax,)
    }
};
        locals.var_v_btbmax = assign520_e811;

        locals.var_iavl = 0.0;
        locals.var_iavl_dn0 = 0.0;
        locals.var_iavl_dn1 = 0.0;
        locals.var_iavl_dn3 = 0.0;
        locals.var_iavl_dn4 = 0.0;
        locals.var_iavl_dn5 = 0.0;
        locals.var_iavl_dn6 = 0.0;
        locals.var_iavl_dn7 = 0.0;
        locals.var_iavl_dn8 = 0.0;
        locals.var_iavl_dn9 = 0.0;

        let assign540_e819: f64 = if ((p.p32 > 0.0) && (p.p47 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard8 = assign540_e819;

        let (assign550_e823,) = {
    if (locals.var_guard8 != 0.0) {
        (1.0,)
    } else {
        (locals.var_use_aval,)
    }
};
        locals.var_use_aval = assign550_e823;

        let (assign560_e828,) = {
    if (locals.var_guard8 == 0.0) {
        (0.0,)
    } else {
        (locals.var_use_aval,)
    }
};
        locals.var_use_aval = assign560_e828;

        locals.var_use_nqs = p.p86;

        let assign580_e832: f64 = if p.p86 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign580_e832;

        let assign590_e843: f64 = if (((p.p88 == 0.0) && (p.p87 == 0.0)) || (p.p66 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard10 = assign590_e843;

        let (assign600_e849,) = {
    if ((locals.var_guard9 != 0.0) && (locals.var_guard10 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_use_nqs,)
    }
};
        locals.var_use_nqs = assign600_e849;

        let assign610_e856: f64 = if ((p.p115 >= 0.01) || (p.p116 >= 0.01)) { 1.0 } else { 0.0 };
        locals.var_guard11 = assign610_e856;

        let (assign620_e864,) = {
    if (locals.var_guard11 != 0.0) {
        let assign620_e861: f64 = (p.p115 - p.p116);
        let assign620_e862: f64 = (0.5 * assign620_e861);
        (assign620_e862,)
    } else {
        (locals.var_lat_delta,)
    }
};
        locals.var_lat_delta = assign620_e864;

        let assign630_e867: f64 = if p.p116 < p.p115 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign630_e867;

        let (assign640_e873,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard12 != 0.0)) {
        (p.p116,)
    } else {
        (locals.var_latmin,)
    }
};
        locals.var_latmin = assign640_e873;

        let (assign650_e879,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard12 != 0.0)) {
        (p.p115,)
    } else {
        (locals.var_latmax,)
    }
};
        locals.var_latmax = assign650_e879;

        let (assign660_e886,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard12 == 0.0)) {
        (p.p115,)
    } else {
        (locals.var_latmin,)
    }
};
        locals.var_latmin = assign660_e886;

        let (assign670_e893,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard12 == 0.0)) {
        (p.p116,)
    } else {
        (locals.var_latmax,)
    }
};
        locals.var_latmax = assign670_e893;

        let assign680_e896: f64 = if locals.var_latmin < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign680_e896;

        let (assign690_e902,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard13 != 0.0)) {
        (1000000000.0,)
    } else {
        (locals.var_inv_latb,)
    }
};
        locals.var_inv_latb = assign690_e902;

        let (assign700_e908,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard13 != 0.0)) {
        (1000000000.0,)
    } else {
        (locals.var_inv_latl,)
    }
};
        locals.var_inv_latl = assign700_e908;

        let (assign710_e914,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard13 != 0.0)) {
        (170000000.0,)
    } else {
        (locals.var_latb_6,)
    }
};
        locals.var_latb_6 = assign710_e914;

        let (assign720_e920,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard13 != 0.0)) {
        (170000000.0,)
    } else {
        (locals.var_latl_6,)
    }
};
        locals.var_latl_6 = assign720_e920;

        let (assign730_e929,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard13 != 0.0)) {
        let assign730_e926: f64 = (1.0 + locals.var_latmax);
        let assign730_e927: f64 = (assign730_e926).ln();
        (assign730_e927,)
    } else {
        (locals.var_ln_lat,)
    }
};
        locals.var_ln_lat = assign730_e929;

        let (assign740_e938,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard13 == 0.0)) {
        let assign740_e936: f64 = (1.0 / p.p115);
        (assign740_e936,)
    } else {
        (locals.var_inv_latb,)
    }
};
        locals.var_inv_latb = assign740_e938;

        let (assign750_e947,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard13 == 0.0)) {
        let assign750_e945: f64 = (1.0 / p.p116);
        (assign750_e945,)
    } else {
        (locals.var_inv_latl,)
    }
};
        locals.var_inv_latl = assign750_e947;

        let (assign760_e956,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard13 == 0.0)) {
        let assign760_e954: f64 = (p.p115 / 6.0);
        (assign760_e954,)
    } else {
        (locals.var_latb_6,)
    }
};
        locals.var_latb_6 = assign760_e956;

        let (assign770_e965,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard13 == 0.0)) {
        let assign770_e963: f64 = (p.p116 / 6.0);
        (assign770_e963,)
    } else {
        (locals.var_latl_6,)
    }
};
        locals.var_latl_6 = assign770_e965;

        let (assign780_e979,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard13 == 0.0)) {
        let assign780_e972: f64 = (1.0 + p.p115);
        let assign780_e975: f64 = (1.0 + p.p116);
        let assign780_e976: f64 = (assign780_e972 / assign780_e975);
        let assign780_e977: f64 = (assign780_e976).ln();
        (assign780_e977,)
    } else {
        (locals.var_ln_lat,)
    }
};
        locals.var_ln_lat = assign780_e979;

        let (assign790_e984,) = {
    if (locals.var_guard11 == 0.0) {
        (0.0,)
    } else {
        (locals.var_lat_delta,)
    }
};
        locals.var_lat_delta = assign790_e984;

        let (assign800_e989,) = {
    if (locals.var_guard11 == 0.0) {
        (1000000000.0,)
    } else {
        (locals.var_inv_latb,)
    }
};
        locals.var_inv_latb = assign800_e989;

        let (assign810_e994,) = {
    if (locals.var_guard11 == 0.0) {
        (1000000000.0,)
    } else {
        (locals.var_inv_latl,)
    }
};
        locals.var_inv_latl = assign810_e994;

        let (assign820_e999,) = {
    if (locals.var_guard11 == 0.0) {
        (170000000.0,)
    } else {
        (locals.var_latb_6,)
    }
};
        locals.var_latb_6 = assign820_e999;

        let (assign830_e1004,) = {
    if (locals.var_guard11 == 0.0) {
        (170000000.0,)
    } else {
        (locals.var_latl_6,)
    }
};
        locals.var_latl_6 = assign830_e1004;

        let (assign840_e1009,) = {
    if (locals.var_guard11 == 0.0) {
        (p.p116,)
    } else {
        (locals.var_latmin,)
    }
};
        locals.var_latmin = assign840_e1009;

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign850_e1014,) = {
    if (locals.var_guard11 == 0.0) {
        (p.p115,)
    } else {
        (locals.var_latmax,)
    }
};
        locals.var_latmax = assign850_e1014;

        let (assign860_e1019,) = {
    if (locals.var_guard11 == 0.0) {
        (0.0,)
    } else {
        (locals.var_ln_lat,)
    }
};
        locals.var_ln_lat = assign860_e1019;

        let assign870_e1022: f64 = (locals.var_tamb + p.p147);
        let assign870_e1024: f64 = assign870_e1022;
        locals.var_tdev = assign870_e1024;
        locals.var_tdev_dn4 = 0.0;

        let assign880_e1027: f64 = (-200.0);
        let assign880_e1029: f64 = (assign880_e1027 + 273.15);
        let assign880_e1030: f64 = if locals.var_tdev < assign880_e1029 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign880_e1030;

        let (assign890_e1037, assign890_e1037_d_n4,) = {
    if (locals.var_guard14 != 0.0) {
        let assign890_e1033: f64 = (-200.0);
        let assign890_e1035: f64 = (assign890_e1033 + 273.15);
        (assign890_e1035, 0.0,)
    } else {
        (locals.var_tdev, locals.var_tdev_dn4,)
    }
};
        locals.var_tdev = assign890_e1037;
        locals.var_tdev_dn4 = assign890_e1037_d_n4;

        let assign900_e1041: f64 = (326.85 + 273.15);
        let assign900_e1042: f64 = if locals.var_tdev > assign900_e1041 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign900_e1042;

        let (assign910_e1051, assign910_e1051_d_n4,) = {
    if ((locals.var_guard14 == 0.0) && (locals.var_guard15 != 0.0)) {
        let assign910_e1049: f64 = (326.85 + 273.15);
        (assign910_e1049, 0.0,)
    } else {
        (locals.var_tdev, locals.var_tdev_dn4,)
    }
};
        locals.var_tdev = assign910_e1051;
        locals.var_tdev_dn4 = assign910_e1051_d_n4;

        let assign920_e1054: f64 = (locals.var_kb2q * locals.var_tdev);
        locals.var_vt = assign920_e1054;
        locals.var_vt_dn4 = (locals.var_kb2q * locals.var_tdev_dn4);

        let assign930_e1057: f64 = (1.0 / locals.var_vt);
        locals.var_ovt = assign930_e1057;
        locals.var_ovt_dn4 = (-(locals.var_vt_dn4 / (locals.var_vt * locals.var_vt)));

        let assign940_e1060: f64 = (locals.var_tdev - locals.var_tnom);
        locals.var_dtdev = assign940_e1060;
        locals.var_dtdev_dn4 = locals.var_tdev_dn4;

        let assign950_e1063: f64 = (locals.var_tnom / locals.var_tdev);
        locals.var_tn2td = assign950_e1063;
        locals.var_tn2td_dn4 = (-((locals.var_tnom * locals.var_tdev_dn4) / (locals.var_tdev * locals.var_tdev)));

        let assign960_e1066: f64 = (locals.var_tdev / locals.var_tnom);
        locals.var_qtt0 = assign960_e1066;
        locals.var_qtt0_dn4 = (locals.var_tdev_dn4 / locals.var_tnom);

        let assign970_e1068: f64 = (locals.var_qtt0).ln();
        locals.var_ln_qtt0 = assign970_e1068;
        locals.var_ln_qtt0_dn4 = (locals.var_qtt0_dn4 / locals.var_qtt0);

        let assign980_e1071: f64 = (p.p121 * locals.var_tdev);
        let assign980_e1073: f64 = (locals.var_tdev).ln();
        let assign980_e1074: f64 = (assign980_e1071 * assign980_e1073);
        locals.var_k1 = assign980_e1074;
        locals.var_k1_dn4 = (((p.p121 * locals.var_tdev_dn4) * assign980_e1073) + (assign980_e1071 * (locals.var_tdev_dn4 / locals.var_tdev)));

        let assign990_e1077: f64 = (p.p122 * locals.var_tdev);
        locals.var_k2 = assign990_e1077;
        locals.var_k2_dn4 = (p.p122 * locals.var_tdev_dn4);

        let assign1000_e1080: f64 = (p.p117 + locals.var_k1);
        let assign1000_e1082: f64 = (assign1000_e1080 + locals.var_k2);
        locals.var_vgb_t = assign1000_e1082;
        locals.var_vgb_t_dn4 = (locals.var_k1_dn4 + locals.var_k2_dn4);

        let assign1010_e1085: f64 = (p.p118 + locals.var_k1);
        let assign1010_e1087: f64 = (assign1010_e1085 + locals.var_k2);
        locals.var_vge_t = assign1010_e1087;
        locals.var_vge_t_dn4 = (locals.var_k1_dn4 + locals.var_k2_dn4);

        let assign1020_e1090: f64 = (p.p119 + locals.var_k1);
        let assign1020_e1092: f64 = (assign1020_e1090 + locals.var_k2);
        locals.var_vgc_t = assign1020_e1092;
        locals.var_vgc_t_dn4 = (locals.var_k1_dn4 + locals.var_k2_dn4);

        let assign1030_e1095: f64 = (locals.var_vgb_t + locals.var_vge_t);
        let assign1030_e1097: f64 = (assign1030_e1095 * 0.5);
        locals.var_vgbe_t = assign1030_e1097;
        locals.var_vgbe_t_dn4 = ((locals.var_vgb_t_dn4 + locals.var_vge_t_dn4) * 0.5);

        let assign1040_e1100: f64 = (locals.var_vgb_t + locals.var_vgc_t);
        let assign1040_e1102: f64 = (assign1040_e1100 * 0.5);
        locals.var_vgbc_t = assign1040_e1102;
        locals.var_vgbc_t_dn4 = ((locals.var_vgb_t_dn4 + locals.var_vgc_t_dn4) * 0.5);

        let assign1050_e1105: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1050_e1105;

        let (assign1060_e1127,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1060_e1109: f64 = (2.0 * locals.var_vtnom);
        let assign1060_e1112: f64 = (p.p40 * 0.5);
        let assign1060_e1114: f64 = (assign1060_e1112 * locals.var_ovtnom);
        let assign1060_e1115: f64 = (assign1060_e1114).exp();
        let assign1060_e1117: f64 = (-0.5);
        let assign1060_e1119: f64 = (assign1060_e1117 * p.p40);
        let assign1060_e1121: f64 = (assign1060_e1119 * locals.var_ovtnom);
        let assign1060_e1122: f64 = (assign1060_e1121).exp();
        let assign1060_e1123: f64 = (assign1060_e1115 - assign1060_e1122);
        let assign1060_e1124: f64 = (assign1060_e1123).ln();
        let assign1060_e1125: f64 = (assign1060_e1109 * assign1060_e1124);
        (assign1060_e1125,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign1060_e1127;

        let (assign1070_e1145, assign1070_e1145_d_n4,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1070_e1131: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign1070_e1135: f64 = (1.0 - locals.var_qtt0);
        let assign1070_e1136: f64 = (locals.var_vgbe0 * assign1070_e1135);
        let assign1070_e1137: f64 = (assign1070_e1131 + assign1070_e1136);
        let assign1070_e1140: f64 = (locals.var_mg * locals.var_vt);
        let assign1070_e1142: f64 = (assign1070_e1140 * locals.var_ln_qtt0);
        let assign1070_e1143: f64 = (assign1070_e1137 - assign1070_e1142);
        (assign1070_e1143, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgbe0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign1070_e1140 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign1070_e1145;
        locals.var_vdj_t_dn4 = assign1070_e1145_d_n4;

        let (assign1080_e1169, assign1080_e1169_d_n4,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1080_e1150: f64 = (2.0 * locals.var_vt);
        let assign1080_e1156: f64 = (-locals.var_vdj_t);
        let assign1080_e1158: f64 = (assign1080_e1156 * locals.var_ovt);
        let assign1080_e1159: f64 = (assign1080_e1158).exp();
        let assign1080_e1160: f64 = (4.0 * assign1080_e1159);
        let assign1080_e1161: f64 = (1.0 + assign1080_e1160);
        let assign1080_e1162: f64 = (assign1080_e1161).sqrt();
        let assign1080_e1163: f64 = (1.0 + assign1080_e1162);
        let assign1080_e1164: f64 = (0.5 * assign1080_e1163);
        let assign1080_e1165: f64 = (assign1080_e1164).ln();
        let assign1080_e1166: f64 = (assign1080_e1150 * assign1080_e1165);
        let assign1080_e1167: f64 = (locals.var_vdj_t + assign1080_e1166);
        (assign1080_e1167, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign1080_e1165) + (assign1080_e1150 * ((0.5 * ((4.0 * (assign1080_e1159 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign1080_e1156 * locals.var_ovt_dn4)))) / (2.0 * assign1080_e1162))) / assign1080_e1164)))),)
    } else {
        (locals.var_vdei_t, locals.var_vdei_t_dn4,)
    }
};
        locals.var_vdei_t = assign1080_e1169;
        locals.var_vdei_t_dn4 = assign1080_e1169_d_n4;

        let (assign1090_e1181, assign1090_e1181_d_n4,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1090_e1175: f64 = (p.p40 / locals.var_vdei_t);
        let assign1090_e1176: f64 = (assign1090_e1175).ln();
        let assign1090_e1177: f64 = (p.p41 * assign1090_e1176);
        let assign1090_e1178: f64 = (assign1090_e1177).exp();
        let assign1090_e1179: f64 = (p.p39 * assign1090_e1178);
        (assign1090_e1179, (p.p39 * (assign1090_e1178 * (p.p41 * ((-((p.p40 * locals.var_vdei_t_dn4) / (locals.var_vdei_t * locals.var_vdei_t))) / assign1090_e1175)))),)
    } else {
        (locals.var_cjei0_t, locals.var_cjei0_t_dn4,)
    }
};
        locals.var_cjei0_t = assign1090_e1181;
        locals.var_cjei0_t_dn4 = assign1090_e1181_d_n4;

        let (assign1100_e1186, assign1100_e1186_d_n4,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1100_e1184: f64 = (p.p42).abs();
        (assign1100_e1184, 0.0,)
    } else {
        (locals.var_ajei_t, locals.var_ajei_t_dn4,)
    }
};
        locals.var_ajei_t = assign1100_e1186;
        locals.var_ajei_t_dn4 = assign1100_e1186_d_n4;

        let assign1110_e1189: f64 = if p.p42 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1110_e1189;

        let (assign1120_e1199, assign1120_e1199_d_n4,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 != 0.0)) {
        let assign1120_e1195: f64 = (p.p42 * locals.var_vdei_t);
        let assign1120_e1197: f64 = (assign1120_e1195 / p.p40);
        (assign1120_e1197, ((p.p42 * locals.var_vdei_t_dn4) / p.p40),)
    } else {
        (locals.var_ajei_t, locals.var_ajei_t_dn4,)
    }
};
        locals.var_ajei_t = assign1120_e1199;
        locals.var_ajei_t_dn4 = assign1120_e1199_d_n4;

        let (assign1130_e1204, assign1130_e1204_d_n4,) = {
    if (locals.var_guard16 == 0.0) {
        (p.p39, 0.0,)
    } else {
        (locals.var_cjei0_t, locals.var_cjei0_t_dn4,)
    }
};
        locals.var_cjei0_t = assign1130_e1204;
        locals.var_cjei0_t_dn4 = assign1130_e1204_d_n4;

        let (assign1140_e1209, assign1140_e1209_d_n4,) = {
    if (locals.var_guard16 == 0.0) {
        (p.p40, 0.0,)
    } else {
        (locals.var_vdei_t, locals.var_vdei_t_dn4,)
    }
};
        locals.var_vdei_t = assign1140_e1209;
        locals.var_vdei_t_dn4 = assign1140_e1209_d_n4;

        let (assign1150_e1214, assign1150_e1214_d_n4,) = {
    if (locals.var_guard16 == 0.0) {
        (p.p42, 0.0,)
    } else {
        (locals.var_ajei_t, locals.var_ajei_t_dn4,)
    }
};
        locals.var_ajei_t = assign1150_e1214;
        locals.var_ajei_t_dn4 = assign1150_e1214_d_n4;

        let assign1160_e1218: f64 = (p.p124 * locals.var_ln_qtt0);
        let assign1160_e1221: f64 = (p.p118 * locals.var_ovtnom);
        let assign1160_e1224: f64 = (1.0 - locals.var_tn2td);
        let assign1160_e1225: f64 = (assign1160_e1221 * assign1160_e1224);
        let assign1160_e1226: f64 = (assign1160_e1218 + assign1160_e1225);
        let assign1160_e1227: f64 = (assign1160_e1226).exp();
        let assign1160_e1228: f64 = (p.p14 * assign1160_e1227);
        locals.var_ibeis_t = assign1160_e1228;
        locals.var_ibeis_t_dn4 = (p.p14 * (assign1160_e1227 * ((p.p124 * locals.var_ln_qtt0_dn4) + (assign1160_e1221 * (-locals.var_tn2td_dn4)))));

        let assign1170_e1232: f64 = (locals.var_mg / p.p17);
        let assign1170_e1234: f64 = (assign1170_e1232 * locals.var_ln_qtt0);
        let assign1170_e1237: f64 = (locals.var_vgbe0 * locals.var_ovtnom);
        let assign1170_e1240: f64 = (1.0 - locals.var_tn2td);
        let assign1170_e1241: f64 = (assign1170_e1237 * assign1170_e1240);
        let assign1170_e1243: f64 = (assign1170_e1241 / p.p17);
        let assign1170_e1244: f64 = (assign1170_e1234 + assign1170_e1243);
        let assign1170_e1245: f64 = (assign1170_e1244).exp();
        let assign1170_e1246: f64 = (p.p16 * assign1170_e1245);
        locals.var_ireis_t = assign1170_e1246;
        locals.var_ireis_t_dn4 = (p.p16 * (assign1170_e1245 * ((assign1170_e1232 * locals.var_ln_qtt0_dn4) + ((assign1170_e1237 * (-locals.var_tn2td_dn4)) / p.p17))));

        let assign1180_e1249: f64 = if p.p47 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign1180_e1249;

        let (assign1190_e1271,) = {
    if (locals.var_guard18 != 0.0) {
        let assign1190_e1253: f64 = (2.0 * locals.var_vtnom);
        let assign1190_e1256: f64 = (p.p48 * 0.5);
        let assign1190_e1258: f64 = (assign1190_e1256 * locals.var_ovtnom);
        let assign1190_e1259: f64 = (assign1190_e1258).exp();
        let assign1190_e1261: f64 = (-0.5);
        let assign1190_e1263: f64 = (assign1190_e1261 * p.p48);
        let assign1190_e1265: f64 = (assign1190_e1263 * locals.var_ovtnom);
        let assign1190_e1266: f64 = (assign1190_e1265).exp();
        let assign1190_e1267: f64 = (assign1190_e1259 - assign1190_e1266);
        let assign1190_e1268: f64 = (assign1190_e1267).ln();
        let assign1190_e1269: f64 = (assign1190_e1253 * assign1190_e1268);
        (assign1190_e1269,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign1190_e1271;

        let (assign1200_e1289, assign1200_e1289_d_n4,) = {
    if (locals.var_guard18 != 0.0) {
        let assign1200_e1275: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign1200_e1279: f64 = (1.0 - locals.var_qtt0);
        let assign1200_e1280: f64 = (locals.var_vgbc0 * assign1200_e1279);
        let assign1200_e1281: f64 = (assign1200_e1275 + assign1200_e1280);
        let assign1200_e1284: f64 = (locals.var_mg * locals.var_vt);
        let assign1200_e1286: f64 = (assign1200_e1284 * locals.var_ln_qtt0);
        let assign1200_e1287: f64 = (assign1200_e1281 - assign1200_e1286);
        (assign1200_e1287, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgbc0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign1200_e1284 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign1200_e1289;
        locals.var_vdj_t_dn4 = assign1200_e1289_d_n4;

        let (assign1210_e1313, assign1210_e1313_d_n4,) = {
    if (locals.var_guard18 != 0.0) {
        let assign1210_e1294: f64 = (2.0 * locals.var_vt);
        let assign1210_e1300: f64 = (-locals.var_vdj_t);
        let assign1210_e1302: f64 = (assign1210_e1300 * locals.var_ovt);
        let assign1210_e1303: f64 = (assign1210_e1302).exp();
        let assign1210_e1304: f64 = (4.0 * assign1210_e1303);
        let assign1210_e1305: f64 = (1.0 + assign1210_e1304);
        let assign1210_e1306: f64 = (assign1210_e1305).sqrt();
        let assign1210_e1307: f64 = (1.0 + assign1210_e1306);
        let assign1210_e1308: f64 = (0.5 * assign1210_e1307);
        let assign1210_e1309: f64 = (assign1210_e1308).ln();
        let assign1210_e1310: f64 = (assign1210_e1294 * assign1210_e1309);
        let assign1210_e1311: f64 = (locals.var_vdj_t + assign1210_e1310);
        (assign1210_e1311, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign1210_e1309) + (assign1210_e1294 * ((0.5 * ((4.0 * (assign1210_e1303 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign1210_e1300 * locals.var_ovt_dn4)))) / (2.0 * assign1210_e1306))) / assign1210_e1308)))),)
    } else {
        (locals.var_vdci_t, locals.var_vdci_t_dn4,)
    }
};
        locals.var_vdci_t = assign1210_e1313;
        locals.var_vdci_t_dn4 = assign1210_e1313_d_n4;

        let (assign1220_e1325, assign1220_e1325_d_n4,) = {
    if (locals.var_guard18 != 0.0) {
        let assign1220_e1319: f64 = (p.p48 / locals.var_vdci_t);
        let assign1220_e1320: f64 = (assign1220_e1319).ln();
        let assign1220_e1321: f64 = (p.p49 * assign1220_e1320);
        let assign1220_e1322: f64 = (assign1220_e1321).exp();
        let assign1220_e1323: f64 = (p.p47 * assign1220_e1322);
        (assign1220_e1323, (p.p47 * (assign1220_e1322 * (p.p49 * ((-((p.p48 * locals.var_vdci_t_dn4) / (locals.var_vdci_t * locals.var_vdci_t))) / assign1220_e1319)))),)
    } else {
        (locals.var_cjci0_t, locals.var_cjci0_t_dn4,)
    }
};
        locals.var_cjci0_t = assign1220_e1325;
        locals.var_cjci0_t_dn4 = assign1220_e1325_d_n4;

        let (assign1230_e1330, assign1230_e1330_d_n4,) = {
    if (locals.var_guard18 != 0.0) {
        let assign1230_e1328: f64 = (p.p50).abs();
        (assign1230_e1328, 0.0,)
    } else {
        (locals.var_ajci_t, locals.var_ajci_t_dn4,)
    }
};
        locals.var_ajci_t = assign1230_e1330;
        locals.var_ajci_t_dn4 = assign1230_e1330_d_n4;

        let assign1240_e1333: f64 = if p.p50 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard19 = assign1240_e1333;

        let (assign1250_e1343, assign1250_e1343_d_n4,) = {
    if ((locals.var_guard18 != 0.0) && (locals.var_guard19 != 0.0)) {
        let assign1250_e1339: f64 = (p.p50 * locals.var_vdci_t);
        let assign1250_e1341: f64 = (assign1250_e1339 / p.p48);
        (assign1250_e1341, ((p.p50 * locals.var_vdci_t_dn4) / p.p48),)
    } else {
        (locals.var_ajci_t, locals.var_ajci_t_dn4,)
    }
};
        locals.var_ajci_t = assign1250_e1343;
        locals.var_ajci_t_dn4 = assign1250_e1343_d_n4;

        let (assign1260_e1348, assign1260_e1348_d_n4,) = {
    if (locals.var_guard18 == 0.0) {
        (p.p47, 0.0,)
    } else {
        (locals.var_cjci0_t, locals.var_cjci0_t_dn4,)
    }
};
        locals.var_cjci0_t = assign1260_e1348;
        locals.var_cjci0_t_dn4 = assign1260_e1348_d_n4;

        let (assign1270_e1353, assign1270_e1353_d_n4,) = {
    if (locals.var_guard18 == 0.0) {
        (p.p48, 0.0,)
    } else {
        (locals.var_vdci_t, locals.var_vdci_t_dn4,)
    }
};
        locals.var_vdci_t = assign1270_e1353;
        locals.var_vdci_t_dn4 = assign1270_e1353_d_n4;

        let (assign1280_e1358, assign1280_e1358_d_n4,) = {
    if (locals.var_guard18 == 0.0) {
        (p.p50, 0.0,)
    } else {
        (locals.var_ajci_t, locals.var_ajci_t_dn4,)
    }
};
        locals.var_ajci_t = assign1280_e1358;
        locals.var_ajci_t_dn4 = assign1280_e1358_d_n4;

        let assign1290_e1361: f64 = if p.p0 <= 300.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1290_e1361;

        let (assign1300_e1365, assign1300_e1365_d_n4,) = {
    if (locals.var_guard20 != 0.0) {
        (2.4, 0.0,)
    } else {
        (locals.var_ajci_t, locals.var_ajci_t_dn4,)
    }
};
        locals.var_ajci_t = assign1300_e1365;
        locals.var_ajci_t_dn4 = assign1300_e1365_d_n4;

        let assign1310_e1369: f64 = (locals.var_zetabci * locals.var_ln_qtt0);
        let assign1310_e1372: f64 = (p.p119 * locals.var_ovtnom);
        let assign1310_e1375: f64 = (1.0 - locals.var_tn2td);
        let assign1310_e1376: f64 = (assign1310_e1372 * assign1310_e1375);
        let assign1310_e1377: f64 = (assign1310_e1369 + assign1310_e1376);
        let assign1310_e1378: f64 = (assign1310_e1377).exp();
        let assign1310_e1379: f64 = (p.p23 * assign1310_e1378);
        locals.var_ibcis_t = assign1310_e1379;
        locals.var_ibcis_t_dn4 = (p.p23 * (assign1310_e1378 * ((locals.var_zetabci * locals.var_ln_qtt0_dn4) + (assign1310_e1372 * (-locals.var_tn2td_dn4)))));

        let assign1320_e1385: f64 = (locals.var_vdei_t / p.p40);
        let assign1320_e1386: f64 = (assign1320_e1385).ln();
        let assign1320_e1387: f64 = (p.p41 * assign1320_e1386);
        let assign1320_e1388: f64 = (assign1320_e1387).exp();
        let assign1320_e1389: f64 = (2.0 - assign1320_e1388);
        let assign1320_e1390: f64 = (p.p2 * assign1320_e1389);
        locals.var_qp0_t = assign1320_e1390;
        locals.var_qp0_t_dn4 = (p.p2 * (-(assign1320_e1388 * (p.p41 * ((locals.var_vdei_t_dn4 / p.p40) / assign1320_e1385)))));

        let assign1330_e1394: f64 = (p.p123 * locals.var_ln_qtt0);
        let assign1330_e1397: f64 = (p.p117 * locals.var_ovtnom);
        let assign1330_e1400: f64 = (1.0 - locals.var_tn2td);
        let assign1330_e1401: f64 = (assign1330_e1397 * assign1330_e1400);
        let assign1330_e1402: f64 = (assign1330_e1394 + assign1330_e1401);
        let assign1330_e1403: f64 = (assign1330_e1402).exp();
        let assign1330_e1404: f64 = (p.p1 * assign1330_e1403);
        locals.var_c10_t = assign1330_e1404;
        locals.var_c10_t_dn4 = (p.p1 * (assign1330_e1403 * ((p.p123 * locals.var_ln_qtt0_dn4) + (assign1330_e1397 * (-locals.var_tn2td_dn4)))));

        let assign1340_e1408: f64 = (p.p126 * locals.var_ln_qtt0);
        let assign1340_e1409: f64 = (assign1340_e1408).exp();
        let assign1340_e1410: f64 = (p.p10 * assign1340_e1409);
        locals.var_ahjei_t = assign1340_e1410;
        locals.var_ahjei_t_dn4 = (p.p10 * (assign1340_e1409 * (p.p126 * locals.var_ln_qtt0_dn4)));

        let assign1350_e1416: f64 = (p.p8 - 1.0);
        let assign1350_e1417: f64 = (assign1350_e1416).abs();
        let assign1350_e1420: f64 = if ((p.p0 <= 300.0) && (assign1350_e1417 < 1e-5)) { 1.0 } else { 0.0 };
        locals.var_guard21 = assign1350_e1420;

        let (assign1360_e1436, assign1360_e1436_d_n4,) = {
    if (locals.var_guard21 != 0.0) {
        let assign1360_e1425: f64 = (p.p125 * locals.var_ovt);
        let assign1360_e1428: f64 = (p.p127 * locals.var_ln_qtt0);
        let assign1360_e1429: f64 = (assign1360_e1428).exp();
        let assign1360_e1431: f64 = (assign1360_e1429 - 1.0);
        let assign1360_e1432: f64 = (assign1360_e1425 * assign1360_e1431);
        let assign1360_e1433: f64 = (assign1360_e1432).exp();
        let assign1360_e1434: f64 = (p.p9 * assign1360_e1433);
        (assign1360_e1434, (p.p9 * (assign1360_e1433 * (((p.p125 * locals.var_ovt_dn4) * assign1360_e1431) + (assign1360_e1425 * (assign1360_e1429 * (p.p127 * locals.var_ln_qtt0_dn4)))))),)
    } else {
        (locals.var_hjei0_t, locals.var_hjei0_t_dn4,)
    }
};
        locals.var_hjei0_t = assign1360_e1436;
        locals.var_hjei0_t_dn4 = assign1360_e1436_d_n4;

        let (assign1370_e1453, assign1370_e1453_d_n4,) = {
    if (locals.var_guard21 == 0.0) {
        let assign1370_e1442: f64 = (p.p125 * locals.var_ovt);
        let assign1370_e1445: f64 = (p.p127 * locals.var_ln_qtt0);
        let assign1370_e1446: f64 = (assign1370_e1445).exp();
        let assign1370_e1448: f64 = (assign1370_e1446 - 1.0);
        let assign1370_e1449: f64 = (assign1370_e1442 * assign1370_e1448);
        let assign1370_e1450: f64 = (assign1370_e1449).exp();
        let assign1370_e1451: f64 = (p.p8 * assign1370_e1450);
        (assign1370_e1451, (p.p8 * (assign1370_e1450 * (((p.p125 * locals.var_ovt_dn4) * assign1370_e1448) + (assign1370_e1442 * (assign1370_e1446 * (p.p127 * locals.var_ln_qtt0_dn4)))))),)
    } else {
        (locals.var_hjei0_t, locals.var_hjei0_t_dn4,)
    }
};
        locals.var_hjei0_t = assign1370_e1453;
        locals.var_hjei0_t_dn4 = assign1370_e1453_d_n4;

        let assign1380_e1457: f64 = (p.p125 * locals.var_ovtnom);
        let assign1380_e1460: f64 = (1.0 - locals.var_tn2td);
        let assign1380_e1461: f64 = (assign1380_e1457 * assign1380_e1460);
        let assign1380_e1462: f64 = (assign1380_e1461).exp();
        let assign1380_e1463: f64 = (p.p3 * assign1380_e1462);
        locals.var_hf0_t = assign1380_e1463;
        locals.var_hf0_t_dn4 = (p.p3 * (assign1380_e1462 * (assign1380_e1457 * (-locals.var_tn2td_dn4))));

        let assign1390_e1467: f64 = (p.p117 - p.p118);
        let assign1390_e1469: f64 = (assign1390_e1467 * locals.var_ovtnom);
        let assign1390_e1472: f64 = (1.0 - locals.var_tn2td);
        let assign1390_e1473: f64 = (assign1390_e1469 * assign1390_e1472);
        let assign1390_e1474: f64 = (assign1390_e1473).exp();
        let assign1390_e1475: f64 = (p.p4 * assign1390_e1474);
        locals.var_hfe_t = assign1390_e1475;
        locals.var_hfe_t_dn4 = (p.p4 * (assign1390_e1474 * (assign1390_e1469 * (-locals.var_tn2td_dn4))));

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1400_e1479: f64 = (p.p117 - p.p119);
        let assign1400_e1481: f64 = (assign1400_e1479 * locals.var_ovtnom);
        let assign1400_e1484: f64 = (1.0 - locals.var_tn2td);
        let assign1400_e1485: f64 = (assign1400_e1481 * assign1400_e1484);
        let assign1400_e1486: f64 = (assign1400_e1485).exp();
        let assign1400_e1487: f64 = (p.p6 * assign1400_e1486);
        locals.var_hfc_t = assign1400_e1487;
        locals.var_hfc_t_dn4 = (p.p6 * (assign1400_e1486 * (assign1400_e1481 * (-locals.var_tn2td_dn4))));

        let assign1410_e1491: f64 = (p.p130 - locals.var_avs);
        let assign1410_e1493: f64 = (assign1410_e1491 * locals.var_ln_qtt0);
        let assign1410_e1494: f64 = (assign1410_e1493).exp();
        let assign1410_e1495: f64 = (p.p75 * assign1410_e1494);
        locals.var_vlim_t = assign1410_e1495;
        locals.var_vlim_t_dn4 = (p.p75 * (assign1410_e1494 * (assign1410_e1491 * locals.var_ln_qtt0_dn4)));

        let assign1420_e1499: f64 = (p.p130 * locals.var_ln_qtt0);
        let assign1420_e1500: f64 = (assign1420_e1499).exp();
        let assign1420_e1501: f64 = (p.p74 * assign1420_e1500);
        locals.var_rci0_t = assign1420_e1501;
        locals.var_rci0_t_dn4 = (p.p74 * (assign1420_e1500 * (p.p130 * locals.var_ln_qtt0_dn4)));

        let assign1430_e1504: f64 = (1.0 / locals.var_rci0_t);
        locals.var_orci0_t = assign1430_e1504;
        locals.var_orci0_t_dn4 = (-(locals.var_rci0_t_dn4 / (locals.var_rci0_t * locals.var_rci0_t)));

        let assign1440_e1507: f64 = if p.p79 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign1440_e1507;

        let (assign1450_e1517, assign1450_e1517_d_n4,) = {
    if (locals.var_guard22 != 0.0) {
        let assign1450_e1513: f64 = (p.p133 * locals.var_dtdev);
        let assign1450_e1514: f64 = (1.0 - assign1450_e1513);
        let assign1450_e1515: f64 = (p.p79 * assign1450_e1514);
        (assign1450_e1515, (p.p79 * (-(p.p133 * locals.var_dtdev_dn4))),)
    } else {
        (locals.var_vdck_t, locals.var_vdck_t_dn4,)
    }
};
        locals.var_vdck_t = assign1450_e1517;
        locals.var_vdck_t_dn4 = assign1450_e1517_d_n4;

        let (assign1460_e1521, assign1460_e1521_d_n4,) = {
    if (locals.var_guard22 != 0.0) {
        (p.p78, 0.0,)
    } else {
        (locals.var_vces_t, locals.var_vces_t_dn4,)
    }
};
        locals.var_vces_t = assign1460_e1521;
        locals.var_vces_t_dn4 = assign1460_e1521_d_n4;

        let (assign1470_e1532, assign1470_e1532_d_n4,) = {
    if (locals.var_guard22 == 0.0) {
        let assign1470_e1528: f64 = (p.p132 * locals.var_dtdev);
        let assign1470_e1529: f64 = (1.0 + assign1470_e1528);
        let assign1470_e1530: f64 = (p.p78 * assign1470_e1529);
        (assign1470_e1530, (p.p78 * (p.p132 * locals.var_dtdev_dn4)),)
    } else {
        (locals.var_vces_t, locals.var_vces_t_dn4,)
    }
};
        locals.var_vces_t = assign1470_e1532;
        locals.var_vces_t_dn4 = assign1470_e1532_d_n4;

        let (assign1480_e1537, assign1480_e1537_d_n4,) = {
    if (locals.var_guard22 == 0.0) {
        (p.p79, 0.0,)
    } else {
        (locals.var_vdck_t, locals.var_vdck_t_dn4,)
    }
};
        locals.var_vdck_t = assign1480_e1537;
        locals.var_vdck_t_dn4 = assign1480_e1537_d_n4;

        let assign1490_e1542: f64 = (p.p128 * locals.var_dtdev);
        let assign1490_e1543: f64 = (1.0 + assign1490_e1542);
        let assign1490_e1546: f64 = (p.p129 * locals.var_dtdev);
        let assign1490_e1548: f64 = (assign1490_e1546 * locals.var_dtdev);
        let assign1490_e1549: f64 = (assign1490_e1543 + assign1490_e1548);
        let assign1490_e1550: f64 = (p.p66 * assign1490_e1549);
        locals.var_t0_t = assign1490_e1550;
        locals.var_t0_t_dn4 = (p.p66 * ((p.p128 * locals.var_dtdev_dn4) + (((p.p129 * locals.var_dtdev_dn4) * locals.var_dtdev) + (assign1490_e1546 * locals.var_dtdev_dn4))));

        locals.var_tef0_t = p.p69;

        let assign1510_e1555: f64 = (p.p130 - 1.0);
        let assign1510_e1557: f64 = (assign1510_e1555 * locals.var_ln_qtt0);
        let assign1510_e1558: f64 = (assign1510_e1557).exp();
        let assign1510_e1559: f64 = (p.p71 * assign1510_e1558);
        locals.var_thcs_t = assign1510_e1559;
        locals.var_thcs_t_dn4 = (p.p71 * (assign1510_e1558 * (assign1510_e1555 * locals.var_ln_qtt0_dn4)));

        let assign1520_e1562: f64 = if locals.var_use_aval == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign1520_e1562;

        let (assign1530_e1571, assign1530_e1571_d_n4,) = {
    if (locals.var_guard23 != 0.0) {
        let assign1530_e1567: f64 = (p.p139 * locals.var_dtdev);
        let assign1530_e1568: f64 = (assign1530_e1567).exp();
        let assign1530_e1569: f64 = (p.p32 * assign1530_e1568);
        (assign1530_e1569, (p.p32 * (assign1530_e1568 * (p.p139 * locals.var_dtdev_dn4))),)
    } else {
        (locals.var_favl_t, locals.var_favl_t_dn4,)
    }
};
        locals.var_favl_t = assign1530_e1571;
        locals.var_favl_t_dn4 = assign1530_e1571_d_n4;

        let (assign1540_e1580, assign1540_e1580_d_n4,) = {
    if (locals.var_guard23 != 0.0) {
        let assign1540_e1576: f64 = (p.p140 * locals.var_dtdev);
        let assign1540_e1577: f64 = (assign1540_e1576).exp();
        let assign1540_e1578: f64 = (p.p33 * assign1540_e1577);
        (assign1540_e1578, (p.p33 * (assign1540_e1577 * (p.p140 * locals.var_dtdev_dn4))),)
    } else {
        (locals.var_qavl_t, locals.var_qavl_t_dn4,)
    }
};
        locals.var_qavl_t = assign1540_e1580;
        locals.var_qavl_t_dn4 = assign1540_e1580_d_n4;

        let (assign1550_e1585, assign1550_e1585_d_n4,) = {
    if (locals.var_guard23 == 0.0) {
        (p.p32, 0.0,)
    } else {
        (locals.var_favl_t, locals.var_favl_t_dn4,)
    }
};
        locals.var_favl_t = assign1550_e1585;
        locals.var_favl_t_dn4 = assign1550_e1585_d_n4;

        let (assign1560_e1590, assign1560_e1590_d_n4,) = {
    if (locals.var_guard23 == 0.0) {
        (p.p33, 0.0,)
    } else {
        (locals.var_qavl_t, locals.var_qavl_t_dn4,)
    }
};
        locals.var_qavl_t = assign1560_e1590;
        locals.var_qavl_t_dn4 = assign1560_e1590_d_n4;

        let assign1570_e1597: f64 = if ((p.p37 > 0.0) && (locals.var_vbici < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard24 = assign1570_e1597;

        let (assign1580_e1601, assign1580_e1601_d_n0, assign1580_e1601_d_n1, assign1580_e1601_d_n3, assign1580_e1601_d_n4, assign1580_e1601_d_n5, assign1580_e1601_d_n6, assign1580_e1601_d_n7, assign1580_e1601_d_n8, assign1580_e1601_d_n9,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p37, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibcts_t, locals.var_ibcts_t_dn0, locals.var_ibcts_t_dn1, locals.var_ibcts_t_dn3, locals.var_ibcts_t_dn4, locals.var_ibcts_t_dn5, locals.var_ibcts_t_dn6, locals.var_ibcts_t_dn7, locals.var_ibcts_t_dn8, locals.var_ibcts_t_dn9,)
    }
};
        locals.var_ibcts_t = assign1580_e1601;
        locals.var_ibcts_t_dn0 = assign1580_e1601_d_n0;
        locals.var_ibcts_t_dn1 = assign1580_e1601_d_n1;
        locals.var_ibcts_t_dn3 = assign1580_e1601_d_n3;
        locals.var_ibcts_t_dn4 = assign1580_e1601_d_n4;
        locals.var_ibcts_t_dn5 = assign1580_e1601_d_n5;
        locals.var_ibcts_t_dn6 = assign1580_e1601_d_n6;
        locals.var_ibcts_t_dn7 = assign1580_e1601_d_n7;
        locals.var_ibcts_t_dn8 = assign1580_e1601_d_n8;
        locals.var_ibcts_t_dn9 = assign1580_e1601_d_n9;

        let (assign1590_e1605, assign1590_e1605_d_n0, assign1590_e1605_d_n1, assign1590_e1605_d_n3, assign1590_e1605_d_n4, assign1590_e1605_d_n5, assign1590_e1605_d_n6, assign1590_e1605_d_n7, assign1590_e1605_d_n8, assign1590_e1605_d_n9,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p38, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_abct_t, locals.var_abct_t_dn0, locals.var_abct_t_dn1, locals.var_abct_t_dn3, locals.var_abct_t_dn4, locals.var_abct_t_dn5, locals.var_abct_t_dn6, locals.var_abct_t_dn7, locals.var_abct_t_dn8, locals.var_abct_t_dn9,)
    }
};
        locals.var_abct_t = assign1590_e1605;
        locals.var_abct_t_dn0 = assign1590_e1605_d_n0;
        locals.var_abct_t_dn1 = assign1590_e1605_d_n1;
        locals.var_abct_t_dn3 = assign1590_e1605_d_n3;
        locals.var_abct_t_dn4 = assign1590_e1605_d_n4;
        locals.var_abct_t_dn5 = assign1590_e1605_d_n5;
        locals.var_abct_t_dn6 = assign1590_e1605_d_n6;
        locals.var_abct_t_dn7 = assign1590_e1605_d_n7;
        locals.var_abct_t_dn8 = assign1590_e1605_d_n8;
        locals.var_abct_t_dn9 = assign1590_e1605_d_n9;

        let assign1600_e1612: f64 = if ((p.p47 > 0.0) && (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard25 = assign1600_e1612;

        let (assign1610_e1620, assign1610_e1620_d_n4,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard25 != 0.0)) {
        let assign1610_e1618: f64 = (locals.var_vgbc_tnom / locals.var_vgbc_t);
        (assign1610_e1618, (-((locals.var_vgbc_tnom * locals.var_vgbc_t_dn4) / (locals.var_vgbc_t * locals.var_vgbc_t))),)
    } else {
        (locals.var_dum_e, locals.var_dum_e_dn4,)
    }
};
        locals.var_dum_e = assign1610_e1620;
        locals.var_dum_e_dn4 = assign1610_e1620_d_n4;

        let (assign1620_e1628, assign1620_e1628_d_n4,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard25 != 0.0)) {
        let assign1620_e1626: f64 = (locals.var_vdci_t / p.p48);
        (assign1620_e1626, (locals.var_vdci_t_dn4 / p.p48),)
    } else {
        (locals.var_dum_v, locals.var_dum_v_dn4,)
    }
};
        locals.var_dum_v = assign1620_e1628;
        locals.var_dum_v_dn4 = assign1620_e1628_d_n4;

        let (assign1630_e1641, assign1630_e1641_d_n0, assign1630_e1641_d_n1, assign1630_e1641_d_n3, assign1630_e1641_d_n4, assign1630_e1641_d_n5, assign1630_e1641_d_n6, assign1630_e1641_d_n7, assign1630_e1641_d_n8, assign1630_e1641_d_n9,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard25 != 0.0)) {
        let assign1630_e1633: f64 = (locals.var_dum_e).sqrt();
        let assign1630_e1635: f64 = (assign1630_e1633 * locals.var_dum_v);
        let assign1630_e1637: f64 = (assign1630_e1635 * locals.var_cjci0_t);
        let assign1630_e1639: f64 = (assign1630_e1637 / p.p47);
        (assign1630_e1639, 0.0, 0.0, 0.0, ((((((locals.var_dum_e_dn4 / (2.0 * assign1630_e1633)) * locals.var_dum_v) + (assign1630_e1633 * locals.var_dum_v_dn4)) * locals.var_cjci0_t) + (assign1630_e1635 * locals.var_cjci0_t_dn4)) / p.p47), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_c, locals.var_dum_c_dn0, locals.var_dum_c_dn1, locals.var_dum_c_dn3, locals.var_dum_c_dn4, locals.var_dum_c_dn5, locals.var_dum_c_dn6, locals.var_dum_c_dn7, locals.var_dum_c_dn8, locals.var_dum_c_dn9,)
    }
};
        locals.var_dum_c = assign1630_e1641;
        locals.var_dum_c_dn0 = assign1630_e1641_d_n0;
        locals.var_dum_c_dn1 = assign1630_e1641_d_n1;
        locals.var_dum_c_dn3 = assign1630_e1641_d_n3;
        locals.var_dum_c_dn4 = assign1630_e1641_d_n4;
        locals.var_dum_c_dn5 = assign1630_e1641_d_n5;
        locals.var_dum_c_dn6 = assign1630_e1641_d_n6;
        locals.var_dum_c_dn7 = assign1630_e1641_d_n7;
        locals.var_dum_c_dn8 = assign1630_e1641_d_n8;
        locals.var_dum_c_dn9 = assign1630_e1641_d_n9;

        let (assign1640_e1651, assign1640_e1651_d_n0, assign1640_e1651_d_n1, assign1640_e1651_d_n3, assign1640_e1651_d_n4, assign1640_e1651_d_n5, assign1640_e1651_d_n6, assign1640_e1651_d_n7, assign1640_e1651_d_n8, assign1640_e1651_d_n9,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard25 != 0.0)) {
        let assign1640_e1647: f64 = (p.p37 * locals.var_dum_c);
        let assign1640_e1649: f64 = (assign1640_e1647 * locals.var_dum_v);
        (assign1640_e1649, ((p.p37 * locals.var_dum_c_dn0) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn1) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn3) * locals.var_dum_v), (((p.p37 * locals.var_dum_c_dn4) * locals.var_dum_v) + (assign1640_e1647 * locals.var_dum_v_dn4)), ((p.p37 * locals.var_dum_c_dn5) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn6) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn7) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn8) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn9) * locals.var_dum_v),)
    } else {
        (locals.var_ibcts_t, locals.var_ibcts_t_dn0, locals.var_ibcts_t_dn1, locals.var_ibcts_t_dn3, locals.var_ibcts_t_dn4, locals.var_ibcts_t_dn5, locals.var_ibcts_t_dn6, locals.var_ibcts_t_dn7, locals.var_ibcts_t_dn8, locals.var_ibcts_t_dn9,)
    }
};
        locals.var_ibcts_t = assign1640_e1651;
        locals.var_ibcts_t_dn0 = assign1640_e1651_d_n0;
        locals.var_ibcts_t_dn1 = assign1640_e1651_d_n1;
        locals.var_ibcts_t_dn3 = assign1640_e1651_d_n3;
        locals.var_ibcts_t_dn4 = assign1640_e1651_d_n4;
        locals.var_ibcts_t_dn5 = assign1640_e1651_d_n5;
        locals.var_ibcts_t_dn6 = assign1640_e1651_d_n6;
        locals.var_ibcts_t_dn7 = assign1640_e1651_d_n7;
        locals.var_ibcts_t_dn8 = assign1640_e1651_d_n8;
        locals.var_ibcts_t_dn9 = assign1640_e1651_d_n9;

        let (assign1650_e1661, assign1650_e1661_d_n0, assign1650_e1661_d_n1, assign1650_e1661_d_n3, assign1650_e1661_d_n4, assign1650_e1661_d_n5, assign1650_e1661_d_n6, assign1650_e1661_d_n7, assign1650_e1661_d_n8, assign1650_e1661_d_n9,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard25 != 0.0)) {
        let assign1650_e1658: f64 = (locals.var_dum_c * locals.var_dum_e);
        let assign1650_e1659: f64 = (p.p38 / assign1650_e1658);
        (assign1650_e1659, (-((p.p38 * (locals.var_dum_c_dn0 * locals.var_dum_e)) / (assign1650_e1658 * assign1650_e1658))), (-((p.p38 * (locals.var_dum_c_dn1 * locals.var_dum_e)) / (assign1650_e1658 * assign1650_e1658))), (-((p.p38 * (locals.var_dum_c_dn3 * locals.var_dum_e)) / (assign1650_e1658 * assign1650_e1658))), (-((p.p38 * ((locals.var_dum_c_dn4 * locals.var_dum_e) + (locals.var_dum_c * locals.var_dum_e_dn4))) / (assign1650_e1658 * assign1650_e1658))), (-((p.p38 * (locals.var_dum_c_dn5 * locals.var_dum_e)) / (assign1650_e1658 * assign1650_e1658))), (-((p.p38 * (locals.var_dum_c_dn6 * locals.var_dum_e)) / (assign1650_e1658 * assign1650_e1658))), (-((p.p38 * (locals.var_dum_c_dn7 * locals.var_dum_e)) / (assign1650_e1658 * assign1650_e1658))), (-((p.p38 * (locals.var_dum_c_dn8 * locals.var_dum_e)) / (assign1650_e1658 * assign1650_e1658))), (-((p.p38 * (locals.var_dum_c_dn9 * locals.var_dum_e)) / (assign1650_e1658 * assign1650_e1658))),)
    } else {
        (locals.var_abct_t, locals.var_abct_t_dn0, locals.var_abct_t_dn1, locals.var_abct_t_dn3, locals.var_abct_t_dn4, locals.var_abct_t_dn5, locals.var_abct_t_dn6, locals.var_abct_t_dn7, locals.var_abct_t_dn8, locals.var_abct_t_dn9,)
    }
};
        locals.var_abct_t = assign1650_e1661;
        locals.var_abct_t_dn0 = assign1650_e1661_d_n0;
        locals.var_abct_t_dn1 = assign1650_e1661_d_n1;
        locals.var_abct_t_dn3 = assign1650_e1661_d_n3;
        locals.var_abct_t_dn4 = assign1650_e1661_d_n4;
        locals.var_abct_t_dn5 = assign1650_e1661_d_n5;
        locals.var_abct_t_dn6 = assign1650_e1661_d_n6;
        locals.var_abct_t_dn7 = assign1650_e1661_d_n7;
        locals.var_abct_t_dn8 = assign1650_e1661_d_n8;
        locals.var_abct_t_dn9 = assign1650_e1661_d_n9;

        let (assign1660_e1666, assign1660_e1666_d_n0, assign1660_e1666_d_n1, assign1660_e1666_d_n3, assign1660_e1666_d_n4, assign1660_e1666_d_n5, assign1660_e1666_d_n6, assign1660_e1666_d_n7, assign1660_e1666_d_n8, assign1660_e1666_d_n9,) = {
    if (locals.var_guard24 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibcts_t, locals.var_ibcts_t_dn0, locals.var_ibcts_t_dn1, locals.var_ibcts_t_dn3, locals.var_ibcts_t_dn4, locals.var_ibcts_t_dn5, locals.var_ibcts_t_dn6, locals.var_ibcts_t_dn7, locals.var_ibcts_t_dn8, locals.var_ibcts_t_dn9,)
    }
};
        locals.var_ibcts_t = assign1660_e1666;
        locals.var_ibcts_t_dn0 = assign1660_e1666_d_n0;
        locals.var_ibcts_t_dn1 = assign1660_e1666_d_n1;
        locals.var_ibcts_t_dn3 = assign1660_e1666_d_n3;
        locals.var_ibcts_t_dn4 = assign1660_e1666_d_n4;
        locals.var_ibcts_t_dn5 = assign1660_e1666_d_n5;
        locals.var_ibcts_t_dn6 = assign1660_e1666_d_n6;
        locals.var_ibcts_t_dn7 = assign1660_e1666_d_n7;
        locals.var_ibcts_t_dn8 = assign1660_e1666_d_n8;
        locals.var_ibcts_t_dn9 = assign1660_e1666_d_n9;

        let (assign1670_e1671, assign1670_e1671_d_n0, assign1670_e1671_d_n1, assign1670_e1671_d_n3, assign1670_e1671_d_n4, assign1670_e1671_d_n5, assign1670_e1671_d_n6, assign1670_e1671_d_n7, assign1670_e1671_d_n8, assign1670_e1671_d_n9,) = {
    if (locals.var_guard24 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_abct_t, locals.var_abct_t_dn0, locals.var_abct_t_dn1, locals.var_abct_t_dn3, locals.var_abct_t_dn4, locals.var_abct_t_dn5, locals.var_abct_t_dn6, locals.var_abct_t_dn7, locals.var_abct_t_dn8, locals.var_abct_t_dn9,)
    }
};
        locals.var_abct_t = assign1670_e1671;
        locals.var_abct_t_dn0 = assign1670_e1671_d_n0;
        locals.var_abct_t_dn1 = assign1670_e1671_d_n1;
        locals.var_abct_t_dn3 = assign1670_e1671_d_n3;
        locals.var_abct_t_dn4 = assign1670_e1671_d_n4;
        locals.var_abct_t_dn5 = assign1670_e1671_d_n5;
        locals.var_abct_t_dn6 = assign1670_e1671_d_n6;
        locals.var_abct_t_dn7 = assign1670_e1671_d_n7;
        locals.var_abct_t_dn8 = assign1670_e1671_d_n8;
        locals.var_abct_t_dn9 = assign1670_e1671_d_n9;

        let assign1680_e1675: f64 = (p.p134 * locals.var_ln_qtt0);
        let assign1680_e1676: f64 = (assign1680_e1675).exp();
        let assign1680_e1677: f64 = (p.p89 * assign1680_e1676);
        locals.var_rbi0_t = assign1680_e1677;
        locals.var_rbi0_t_dn4 = (p.p89 * (assign1680_e1676 * (p.p134 * locals.var_ln_qtt0_dn4)));

        let assign1690_e1680: f64 = if p.p43 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign1690_e1680;

        let (assign1700_e1702,) = {
    if (locals.var_guard26 != 0.0) {
        let assign1700_e1684: f64 = (2.0 * locals.var_vtnom);
        let assign1700_e1687: f64 = (p.p44 * 0.5);
        let assign1700_e1689: f64 = (assign1700_e1687 * locals.var_ovtnom);
        let assign1700_e1690: f64 = (assign1700_e1689).exp();
        let assign1700_e1692: f64 = (-0.5);
        let assign1700_e1694: f64 = (assign1700_e1692 * p.p44);
        let assign1700_e1696: f64 = (assign1700_e1694 * locals.var_ovtnom);
        let assign1700_e1697: f64 = (assign1700_e1696).exp();
        let assign1700_e1698: f64 = (assign1700_e1690 - assign1700_e1697);
        let assign1700_e1699: f64 = (assign1700_e1698).ln();
        let assign1700_e1700: f64 = (assign1700_e1684 * assign1700_e1699);
        (assign1700_e1700,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign1700_e1702;

        let (assign1710_e1720, assign1710_e1720_d_n4,) = {
    if (locals.var_guard26 != 0.0) {
        let assign1710_e1706: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign1710_e1710: f64 = (1.0 - locals.var_qtt0);
        let assign1710_e1711: f64 = (locals.var_vgbe0 * assign1710_e1710);
        let assign1710_e1712: f64 = (assign1710_e1706 + assign1710_e1711);
        let assign1710_e1715: f64 = (locals.var_mg * locals.var_vt);
        let assign1710_e1717: f64 = (assign1710_e1715 * locals.var_ln_qtt0);
        let assign1710_e1718: f64 = (assign1710_e1712 - assign1710_e1717);
        (assign1710_e1718, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgbe0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign1710_e1715 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign1710_e1720;
        locals.var_vdj_t_dn4 = assign1710_e1720_d_n4;

        let (assign1720_e1744, assign1720_e1744_d_n4,) = {
    if (locals.var_guard26 != 0.0) {
        let assign1720_e1725: f64 = (2.0 * locals.var_vt);
        let assign1720_e1731: f64 = (-locals.var_vdj_t);
        let assign1720_e1733: f64 = (assign1720_e1731 * locals.var_ovt);
        let assign1720_e1734: f64 = (assign1720_e1733).exp();
        let assign1720_e1735: f64 = (4.0 * assign1720_e1734);
        let assign1720_e1736: f64 = (1.0 + assign1720_e1735);
        let assign1720_e1737: f64 = (assign1720_e1736).sqrt();
        let assign1720_e1738: f64 = (1.0 + assign1720_e1737);
        let assign1720_e1739: f64 = (0.5 * assign1720_e1738);
        let assign1720_e1740: f64 = (assign1720_e1739).ln();
        let assign1720_e1741: f64 = (assign1720_e1725 * assign1720_e1740);
        let assign1720_e1742: f64 = (locals.var_vdj_t + assign1720_e1741);
        (assign1720_e1742, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign1720_e1740) + (assign1720_e1725 * ((0.5 * ((4.0 * (assign1720_e1734 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign1720_e1731 * locals.var_ovt_dn4)))) / (2.0 * assign1720_e1737))) / assign1720_e1739)))),)
    } else {
        (locals.var_vdep_t, locals.var_vdep_t_dn4,)
    }
};
        locals.var_vdep_t = assign1720_e1744;
        locals.var_vdep_t_dn4 = assign1720_e1744_d_n4;

        let (assign1730_e1756, assign1730_e1756_d_n4,) = {
    if (locals.var_guard26 != 0.0) {
        let assign1730_e1750: f64 = (p.p44 / locals.var_vdep_t);
        let assign1730_e1751: f64 = (assign1730_e1750).ln();
        let assign1730_e1752: f64 = (p.p45 * assign1730_e1751);
        let assign1730_e1753: f64 = (assign1730_e1752).exp();
        let assign1730_e1754: f64 = (p.p43 * assign1730_e1753);
        (assign1730_e1754, (p.p43 * (assign1730_e1753 * (p.p45 * ((-((p.p44 * locals.var_vdep_t_dn4) / (locals.var_vdep_t * locals.var_vdep_t))) / assign1730_e1750)))),)
    } else {
        (locals.var_cjep0_t, locals.var_cjep0_t_dn4,)
    }
};
        locals.var_cjep0_t = assign1730_e1756;
        locals.var_cjep0_t_dn4 = assign1730_e1756_d_n4;

        let (assign1740_e1761, assign1740_e1761_d_n4,) = {
    if (locals.var_guard26 != 0.0) {
        let assign1740_e1759: f64 = (p.p46).abs();
        (assign1740_e1759, 0.0,)
    } else {
        (locals.var_ajep_t, locals.var_ajep_t_dn4,)
    }
};
        locals.var_ajep_t = assign1740_e1761;
        locals.var_ajep_t_dn4 = assign1740_e1761_d_n4;

        let assign1750_e1764: f64 = if p.p46 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign1750_e1764;

        let (assign1760_e1774, assign1760_e1774_d_n4,) = {
    if ((locals.var_guard26 != 0.0) && (locals.var_guard27 != 0.0)) {
        let assign1760_e1770: f64 = (p.p46 * locals.var_vdep_t);
        let assign1760_e1772: f64 = (assign1760_e1770 / p.p44);
        (assign1760_e1772, ((p.p46 * locals.var_vdep_t_dn4) / p.p44),)
    } else {
        (locals.var_ajep_t, locals.var_ajep_t_dn4,)
    }
};
        locals.var_ajep_t = assign1760_e1774;
        locals.var_ajep_t_dn4 = assign1760_e1774_d_n4;

        let (assign1770_e1779, assign1770_e1779_d_n4,) = {
    if (locals.var_guard26 == 0.0) {
        (p.p43, 0.0,)
    } else {
        (locals.var_cjep0_t, locals.var_cjep0_t_dn4,)
    }
};
        locals.var_cjep0_t = assign1770_e1779;
        locals.var_cjep0_t_dn4 = assign1770_e1779_d_n4;

        let (assign1780_e1784, assign1780_e1784_d_n4,) = {
    if (locals.var_guard26 == 0.0) {
        (p.p44, 0.0,)
    } else {
        (locals.var_vdep_t, locals.var_vdep_t_dn4,)
    }
};
        locals.var_vdep_t = assign1780_e1784;
        locals.var_vdep_t_dn4 = assign1780_e1784_d_n4;

        let (assign1790_e1789, assign1790_e1789_d_n4,) = {
    if (locals.var_guard26 == 0.0) {
        (p.p46, 0.0,)
    } else {
        (locals.var_ajep_t, locals.var_ajep_t_dn4,)
    }
};
        locals.var_ajep_t = assign1790_e1789;
        locals.var_ajep_t_dn4 = assign1790_e1789_d_n4;

        let assign1800_e1793: f64 = (p.p124 * locals.var_ln_qtt0);
        let assign1800_e1796: f64 = (p.p118 * locals.var_ovtnom);
        let assign1800_e1799: f64 = (1.0 - locals.var_tn2td);
        let assign1800_e1800: f64 = (assign1800_e1796 * assign1800_e1799);
        let assign1800_e1801: f64 = (assign1800_e1793 + assign1800_e1800);
        let assign1800_e1802: f64 = (assign1800_e1801).exp();
        let assign1800_e1803: f64 = (p.p18 * assign1800_e1802);
        locals.var_ibeps_t = assign1800_e1803;
        locals.var_ibeps_t_dn4 = (p.p18 * (assign1800_e1802 * ((p.p124 * locals.var_ln_qtt0_dn4) + (assign1800_e1796 * (-locals.var_tn2td_dn4)))));

        let assign1810_e1807: f64 = (locals.var_mg / p.p21);
        let assign1810_e1809: f64 = (assign1810_e1807 * locals.var_ln_qtt0);
        let assign1810_e1812: f64 = (locals.var_vgbe0 * locals.var_ovtnom);
        let assign1810_e1815: f64 = (1.0 - locals.var_tn2td);
        let assign1810_e1816: f64 = (assign1810_e1812 * assign1810_e1815);
        let assign1810_e1818: f64 = (assign1810_e1816 / p.p21);
        let assign1810_e1819: f64 = (assign1810_e1809 + assign1810_e1818);
        let assign1810_e1820: f64 = (assign1810_e1819).exp();
        let assign1810_e1821: f64 = (p.p20 * assign1810_e1820);
        locals.var_ireps_t = assign1810_e1821;
        locals.var_ireps_t_dn4 = (p.p20 * (assign1810_e1820 * ((assign1810_e1807 * locals.var_ln_qtt0_dn4) + ((assign1810_e1812 * (-locals.var_tn2td_dn4)) / p.p21))));

        let assign1820_e1832: f64 = if ((p.p27 > 0.0) && ((locals.var_vbpei < locals.var_v_btbmax) || (locals.var_vbiei < locals.var_v_btbmax))) { 1.0 } else { 0.0 };
        locals.var_guard28 = assign1820_e1832;

        let (assign1830_e1836, assign1830_e1836_d_n0, assign1830_e1836_d_n1, assign1830_e1836_d_n3, assign1830_e1836_d_n4, assign1830_e1836_d_n5, assign1830_e1836_d_n6, assign1830_e1836_d_n7, assign1830_e1836_d_n8, assign1830_e1836_d_n9,) = {
    if (locals.var_guard28 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_a, locals.var_dum_a_dn0, locals.var_dum_a_dn1, locals.var_dum_a_dn3, locals.var_dum_a_dn4, locals.var_dum_a_dn5, locals.var_dum_a_dn6, locals.var_dum_a_dn7, locals.var_dum_a_dn8, locals.var_dum_a_dn9,)
    }
};
        locals.var_dum_a = assign1830_e1836;
        locals.var_dum_a_dn0 = assign1830_e1836_d_n0;
        locals.var_dum_a_dn1 = assign1830_e1836_d_n1;
        locals.var_dum_a_dn3 = assign1830_e1836_d_n3;
        locals.var_dum_a_dn4 = assign1830_e1836_d_n4;
        locals.var_dum_a_dn5 = assign1830_e1836_d_n5;
        locals.var_dum_a_dn6 = assign1830_e1836_d_n6;
        locals.var_dum_a_dn7 = assign1830_e1836_d_n7;
        locals.var_dum_a_dn8 = assign1830_e1836_d_n8;
        locals.var_dum_a_dn9 = assign1830_e1836_d_n9;

        let (assign1840_e1840, assign1840_e1840_d_n0, assign1840_e1840_d_n1, assign1840_e1840_d_n3, assign1840_e1840_d_n4, assign1840_e1840_d_n5, assign1840_e1840_d_n6, assign1840_e1840_d_n7, assign1840_e1840_d_n8, assign1840_e1840_d_n9,) = {
    if (locals.var_guard28 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_b, locals.var_dum_b_dn0, locals.var_dum_b_dn1, locals.var_dum_b_dn3, locals.var_dum_b_dn4, locals.var_dum_b_dn5, locals.var_dum_b_dn6, locals.var_dum_b_dn7, locals.var_dum_b_dn8, locals.var_dum_b_dn9,)
    }
};
        locals.var_dum_b = assign1840_e1840;
        locals.var_dum_b_dn0 = assign1840_e1840_d_n0;
        locals.var_dum_b_dn1 = assign1840_e1840_d_n1;
        locals.var_dum_b_dn3 = assign1840_e1840_d_n3;
        locals.var_dum_b_dn4 = assign1840_e1840_d_n4;
        locals.var_dum_b_dn5 = assign1840_e1840_d_n5;
        locals.var_dum_b_dn6 = assign1840_e1840_d_n6;
        locals.var_dum_b_dn7 = assign1840_e1840_d_n7;
        locals.var_dum_b_dn8 = assign1840_e1840_d_n8;
        locals.var_dum_b_dn9 = assign1840_e1840_d_n9;

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1850_e1846, assign1850_e1846_d_n4,) = {
    if (locals.var_guard28 != 0.0) {
        let assign1850_e1844: f64 = (locals.var_vgbe_tnom / locals.var_vgbe_t);
        (assign1850_e1844, (-((locals.var_vgbe_tnom * locals.var_vgbe_t_dn4) / (locals.var_vgbe_t * locals.var_vgbe_t))),)
    } else {
        (locals.var_dum_e, locals.var_dum_e_dn4,)
    }
};
        locals.var_dum_e = assign1850_e1846;
        locals.var_dum_e_dn4 = assign1850_e1846_d_n4;

        let assign1860_e1857: f64 = if (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard29 = assign1860_e1857;

        let (assign1870_e1865, assign1870_e1865_d_n4,) = {
    if ((locals.var_guard28 != 0.0) && (locals.var_guard29 != 0.0)) {
        let assign1870_e1863: f64 = (locals.var_vdep_t / p.p44);
        (assign1870_e1863, (locals.var_vdep_t_dn4 / p.p44),)
    } else {
        (locals.var_dum_v, locals.var_dum_v_dn4,)
    }
};
        locals.var_dum_v = assign1870_e1865;
        locals.var_dum_v_dn4 = assign1870_e1865_d_n4;

        let (assign1880_e1880, assign1880_e1880_d_n0, assign1880_e1880_d_n1, assign1880_e1880_d_n3, assign1880_e1880_d_n4, assign1880_e1880_d_n5, assign1880_e1880_d_n6, assign1880_e1880_d_n7, assign1880_e1880_d_n8, assign1880_e1880_d_n9,) = {
    if ((locals.var_guard28 != 0.0) && (locals.var_guard29 != 0.0)) {
        let assign1880_e1871: f64 = (locals.var_cjep0_t / p.p43);
        let assign1880_e1873: f64 = (locals.var_dum_e).sqrt();
        let assign1880_e1874: f64 = (assign1880_e1871 * assign1880_e1873);
        let assign1880_e1876: f64 = (assign1880_e1874 * locals.var_dum_v);
        let assign1880_e1878: f64 = (assign1880_e1876 * locals.var_dum_v);
        (assign1880_e1878, 0.0, 0.0, 0.0, (((((((locals.var_cjep0_t_dn4 / p.p43) * assign1880_e1873) + (assign1880_e1871 * (locals.var_dum_e_dn4 / (2.0 * assign1880_e1873)))) * locals.var_dum_v) + (assign1880_e1874 * locals.var_dum_v_dn4)) * locals.var_dum_v) + (assign1880_e1876 * locals.var_dum_v_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_b, locals.var_dum_b_dn0, locals.var_dum_b_dn1, locals.var_dum_b_dn3, locals.var_dum_b_dn4, locals.var_dum_b_dn5, locals.var_dum_b_dn6, locals.var_dum_b_dn7, locals.var_dum_b_dn8, locals.var_dum_b_dn9,)
    }
};
        locals.var_dum_b = assign1880_e1880;
        locals.var_dum_b_dn0 = assign1880_e1880_d_n0;
        locals.var_dum_b_dn1 = assign1880_e1880_d_n1;
        locals.var_dum_b_dn3 = assign1880_e1880_d_n3;
        locals.var_dum_b_dn4 = assign1880_e1880_d_n4;
        locals.var_dum_b_dn5 = assign1880_e1880_d_n5;
        locals.var_dum_b_dn6 = assign1880_e1880_d_n6;
        locals.var_dum_b_dn7 = assign1880_e1880_d_n7;
        locals.var_dum_b_dn8 = assign1880_e1880_d_n8;
        locals.var_dum_b_dn9 = assign1880_e1880_d_n9;

        let (assign1890_e1895, assign1890_e1895_d_n0, assign1890_e1895_d_n1, assign1890_e1895_d_n3, assign1890_e1895_d_n4, assign1890_e1895_d_n5, assign1890_e1895_d_n6, assign1890_e1895_d_n7, assign1890_e1895_d_n8, assign1890_e1895_d_n9,) = {
    if ((locals.var_guard28 != 0.0) && (locals.var_guard29 != 0.0)) {
        let assign1890_e1886: f64 = (p.p43 / locals.var_cjep0_t);
        let assign1890_e1889: f64 = (-1.5);
        let assign1890_e1890: f64 = (locals.var_dum_e).powf(assign1890_e1889);
        let assign1890_e1891: f64 = (assign1890_e1886 * assign1890_e1890);
        let assign1890_e1893: f64 = (assign1890_e1891 / locals.var_dum_v);
        (assign1890_e1893, 0.0, 0.0, 0.0, ((((((-((p.p43 * locals.var_cjep0_t_dn4) / (locals.var_cjep0_t * locals.var_cjep0_t))) * assign1890_e1890) + (assign1890_e1886 * if 0.0 == 0.0 && ((assign1890_e1889) as f64).is_finite() && ((assign1890_e1889) as f64).fract() == 0.0 { if assign1890_e1889 == 0.0 { 0.0 } else { (assign1890_e1889 * ((locals.var_dum_e).powf(assign1890_e1889 - 1.0) * locals.var_dum_e_dn4)) } } else { (assign1890_e1890 * (assign1890_e1889 * (locals.var_dum_e_dn4 / locals.var_dum_e))) })) * locals.var_dum_v) - (assign1890_e1891 * locals.var_dum_v_dn4)) / (locals.var_dum_v * locals.var_dum_v)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_a, locals.var_dum_a_dn0, locals.var_dum_a_dn1, locals.var_dum_a_dn3, locals.var_dum_a_dn4, locals.var_dum_a_dn5, locals.var_dum_a_dn6, locals.var_dum_a_dn7, locals.var_dum_a_dn8, locals.var_dum_a_dn9,)
    }
};
        locals.var_dum_a = assign1890_e1895;
        locals.var_dum_a_dn0 = assign1890_e1895_d_n0;
        locals.var_dum_a_dn1 = assign1890_e1895_d_n1;
        locals.var_dum_a_dn3 = assign1890_e1895_d_n3;
        locals.var_dum_a_dn4 = assign1890_e1895_d_n4;
        locals.var_dum_a_dn5 = assign1890_e1895_d_n5;
        locals.var_dum_a_dn6 = assign1890_e1895_d_n6;
        locals.var_dum_a_dn7 = assign1890_e1895_d_n7;
        locals.var_dum_a_dn8 = assign1890_e1895_d_n8;
        locals.var_dum_a_dn9 = assign1890_e1895_d_n9;

        let assign1900_e1906: f64 = if (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard30 = assign1900_e1906;

        let (assign1910_e1917, assign1910_e1917_d_n4,) = {
    if (((locals.var_guard28 != 0.0) && (locals.var_guard29 == 0.0)) && (locals.var_guard30 != 0.0)) {
        let assign1910_e1915: f64 = (locals.var_vdei_t / p.p40);
        (assign1910_e1915, (locals.var_vdei_t_dn4 / p.p40),)
    } else {
        (locals.var_dum_v, locals.var_dum_v_dn4,)
    }
};
        locals.var_dum_v = assign1910_e1917;
        locals.var_dum_v_dn4 = assign1910_e1917_d_n4;

        let (assign1920_e1935, assign1920_e1935_d_n0, assign1920_e1935_d_n1, assign1920_e1935_d_n3, assign1920_e1935_d_n4, assign1920_e1935_d_n5, assign1920_e1935_d_n6, assign1920_e1935_d_n7, assign1920_e1935_d_n8, assign1920_e1935_d_n9,) = {
    if (((locals.var_guard28 != 0.0) && (locals.var_guard29 == 0.0)) && (locals.var_guard30 != 0.0)) {
        let assign1920_e1926: f64 = (locals.var_cjei0_t / p.p39);
        let assign1920_e1928: f64 = (locals.var_dum_e).sqrt();
        let assign1920_e1929: f64 = (assign1920_e1926 * assign1920_e1928);
        let assign1920_e1931: f64 = (assign1920_e1929 * locals.var_dum_v);
        let assign1920_e1933: f64 = (assign1920_e1931 * locals.var_dum_v);
        (assign1920_e1933, 0.0, 0.0, 0.0, (((((((locals.var_cjei0_t_dn4 / p.p39) * assign1920_e1928) + (assign1920_e1926 * (locals.var_dum_e_dn4 / (2.0 * assign1920_e1928)))) * locals.var_dum_v) + (assign1920_e1929 * locals.var_dum_v_dn4)) * locals.var_dum_v) + (assign1920_e1931 * locals.var_dum_v_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_b, locals.var_dum_b_dn0, locals.var_dum_b_dn1, locals.var_dum_b_dn3, locals.var_dum_b_dn4, locals.var_dum_b_dn5, locals.var_dum_b_dn6, locals.var_dum_b_dn7, locals.var_dum_b_dn8, locals.var_dum_b_dn9,)
    }
};
        locals.var_dum_b = assign1920_e1935;
        locals.var_dum_b_dn0 = assign1920_e1935_d_n0;
        locals.var_dum_b_dn1 = assign1920_e1935_d_n1;
        locals.var_dum_b_dn3 = assign1920_e1935_d_n3;
        locals.var_dum_b_dn4 = assign1920_e1935_d_n4;
        locals.var_dum_b_dn5 = assign1920_e1935_d_n5;
        locals.var_dum_b_dn6 = assign1920_e1935_d_n6;
        locals.var_dum_b_dn7 = assign1920_e1935_d_n7;
        locals.var_dum_b_dn8 = assign1920_e1935_d_n8;
        locals.var_dum_b_dn9 = assign1920_e1935_d_n9;

        let (assign1930_e1953, assign1930_e1953_d_n0, assign1930_e1953_d_n1, assign1930_e1953_d_n3, assign1930_e1953_d_n4, assign1930_e1953_d_n5, assign1930_e1953_d_n6, assign1930_e1953_d_n7, assign1930_e1953_d_n8, assign1930_e1953_d_n9,) = {
    if (((locals.var_guard28 != 0.0) && (locals.var_guard29 == 0.0)) && (locals.var_guard30 != 0.0)) {
        let assign1930_e1944: f64 = (p.p39 / locals.var_cjei0_t);
        let assign1930_e1947: f64 = (-1.5);
        let assign1930_e1948: f64 = (locals.var_dum_e).powf(assign1930_e1947);
        let assign1930_e1949: f64 = (assign1930_e1944 * assign1930_e1948);
        let assign1930_e1951: f64 = (assign1930_e1949 / locals.var_dum_v);
        (assign1930_e1951, 0.0, 0.0, 0.0, ((((((-((p.p39 * locals.var_cjei0_t_dn4) / (locals.var_cjei0_t * locals.var_cjei0_t))) * assign1930_e1948) + (assign1930_e1944 * if 0.0 == 0.0 && ((assign1930_e1947) as f64).is_finite() && ((assign1930_e1947) as f64).fract() == 0.0 { if assign1930_e1947 == 0.0 { 0.0 } else { (assign1930_e1947 * ((locals.var_dum_e).powf(assign1930_e1947 - 1.0) * locals.var_dum_e_dn4)) } } else { (assign1930_e1948 * (assign1930_e1947 * (locals.var_dum_e_dn4 / locals.var_dum_e))) })) * locals.var_dum_v) - (assign1930_e1949 * locals.var_dum_v_dn4)) / (locals.var_dum_v * locals.var_dum_v)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_a, locals.var_dum_a_dn0, locals.var_dum_a_dn1, locals.var_dum_a_dn3, locals.var_dum_a_dn4, locals.var_dum_a_dn5, locals.var_dum_a_dn6, locals.var_dum_a_dn7, locals.var_dum_a_dn8, locals.var_dum_a_dn9,)
    }
};
        locals.var_dum_a = assign1930_e1953;
        locals.var_dum_a_dn0 = assign1930_e1953_d_n0;
        locals.var_dum_a_dn1 = assign1930_e1953_d_n1;
        locals.var_dum_a_dn3 = assign1930_e1953_d_n3;
        locals.var_dum_a_dn4 = assign1930_e1953_d_n4;
        locals.var_dum_a_dn5 = assign1930_e1953_d_n5;
        locals.var_dum_a_dn6 = assign1930_e1953_d_n6;
        locals.var_dum_a_dn7 = assign1930_e1953_d_n7;
        locals.var_dum_a_dn8 = assign1930_e1953_d_n8;
        locals.var_dum_a_dn9 = assign1930_e1953_d_n9;

        let (assign1940_e1959, assign1940_e1959_d_n0, assign1940_e1959_d_n1, assign1940_e1959_d_n3, assign1940_e1959_d_n4, assign1940_e1959_d_n5, assign1940_e1959_d_n6, assign1940_e1959_d_n7, assign1940_e1959_d_n8, assign1940_e1959_d_n9,) = {
    if (locals.var_guard28 != 0.0) {
        let assign1940_e1957: f64 = (p.p27 * locals.var_dum_b);
        (assign1940_e1957, (p.p27 * locals.var_dum_b_dn0), (p.p27 * locals.var_dum_b_dn1), (p.p27 * locals.var_dum_b_dn3), (p.p27 * locals.var_dum_b_dn4), (p.p27 * locals.var_dum_b_dn5), (p.p27 * locals.var_dum_b_dn6), (p.p27 * locals.var_dum_b_dn7), (p.p27 * locals.var_dum_b_dn8), (p.p27 * locals.var_dum_b_dn9),)
    } else {
        (locals.var_ibets_t, locals.var_ibets_t_dn0, locals.var_ibets_t_dn1, locals.var_ibets_t_dn3, locals.var_ibets_t_dn4, locals.var_ibets_t_dn5, locals.var_ibets_t_dn6, locals.var_ibets_t_dn7, locals.var_ibets_t_dn8, locals.var_ibets_t_dn9,)
    }
};
        locals.var_ibets_t = assign1940_e1959;
        locals.var_ibets_t_dn0 = assign1940_e1959_d_n0;
        locals.var_ibets_t_dn1 = assign1940_e1959_d_n1;
        locals.var_ibets_t_dn3 = assign1940_e1959_d_n3;
        locals.var_ibets_t_dn4 = assign1940_e1959_d_n4;
        locals.var_ibets_t_dn5 = assign1940_e1959_d_n5;
        locals.var_ibets_t_dn6 = assign1940_e1959_d_n6;
        locals.var_ibets_t_dn7 = assign1940_e1959_d_n7;
        locals.var_ibets_t_dn8 = assign1940_e1959_d_n8;
        locals.var_ibets_t_dn9 = assign1940_e1959_d_n9;

        let (assign1950_e1965, assign1950_e1965_d_n0, assign1950_e1965_d_n1, assign1950_e1965_d_n3, assign1950_e1965_d_n4, assign1950_e1965_d_n5, assign1950_e1965_d_n6, assign1950_e1965_d_n7, assign1950_e1965_d_n8, assign1950_e1965_d_n9,) = {
    if (locals.var_guard28 != 0.0) {
        let assign1950_e1963: f64 = (p.p28 * locals.var_dum_a);
        (assign1950_e1963, (p.p28 * locals.var_dum_a_dn0), (p.p28 * locals.var_dum_a_dn1), (p.p28 * locals.var_dum_a_dn3), (p.p28 * locals.var_dum_a_dn4), (p.p28 * locals.var_dum_a_dn5), (p.p28 * locals.var_dum_a_dn6), (p.p28 * locals.var_dum_a_dn7), (p.p28 * locals.var_dum_a_dn8), (p.p28 * locals.var_dum_a_dn9),)
    } else {
        (locals.var_abet_t, locals.var_abet_t_dn0, locals.var_abet_t_dn1, locals.var_abet_t_dn3, locals.var_abet_t_dn4, locals.var_abet_t_dn5, locals.var_abet_t_dn6, locals.var_abet_t_dn7, locals.var_abet_t_dn8, locals.var_abet_t_dn9,)
    }
};
        locals.var_abet_t = assign1950_e1965;
        locals.var_abet_t_dn0 = assign1950_e1965_d_n0;
        locals.var_abet_t_dn1 = assign1950_e1965_d_n1;
        locals.var_abet_t_dn3 = assign1950_e1965_d_n3;
        locals.var_abet_t_dn4 = assign1950_e1965_d_n4;
        locals.var_abet_t_dn5 = assign1950_e1965_d_n5;
        locals.var_abet_t_dn6 = assign1950_e1965_d_n6;
        locals.var_abet_t_dn7 = assign1950_e1965_d_n7;
        locals.var_abet_t_dn8 = assign1950_e1965_d_n8;
        locals.var_abet_t_dn9 = assign1950_e1965_d_n9;

        let (assign1960_e1970, assign1960_e1970_d_n0, assign1960_e1970_d_n1, assign1960_e1970_d_n3, assign1960_e1970_d_n4, assign1960_e1970_d_n5, assign1960_e1970_d_n6, assign1960_e1970_d_n7, assign1960_e1970_d_n8, assign1960_e1970_d_n9,) = {
    if (locals.var_guard28 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibets_t, locals.var_ibets_t_dn0, locals.var_ibets_t_dn1, locals.var_ibets_t_dn3, locals.var_ibets_t_dn4, locals.var_ibets_t_dn5, locals.var_ibets_t_dn6, locals.var_ibets_t_dn7, locals.var_ibets_t_dn8, locals.var_ibets_t_dn9,)
    }
};
        locals.var_ibets_t = assign1960_e1970;
        locals.var_ibets_t_dn0 = assign1960_e1970_d_n0;
        locals.var_ibets_t_dn1 = assign1960_e1970_d_n1;
        locals.var_ibets_t_dn3 = assign1960_e1970_d_n3;
        locals.var_ibets_t_dn4 = assign1960_e1970_d_n4;
        locals.var_ibets_t_dn5 = assign1960_e1970_d_n5;
        locals.var_ibets_t_dn6 = assign1960_e1970_d_n6;
        locals.var_ibets_t_dn7 = assign1960_e1970_d_n7;
        locals.var_ibets_t_dn8 = assign1960_e1970_d_n8;
        locals.var_ibets_t_dn9 = assign1960_e1970_d_n9;

        let (assign1970_e1975, assign1970_e1975_d_n0, assign1970_e1975_d_n1, assign1970_e1975_d_n3, assign1970_e1975_d_n4, assign1970_e1975_d_n5, assign1970_e1975_d_n6, assign1970_e1975_d_n7, assign1970_e1975_d_n8, assign1970_e1975_d_n9,) = {
    if (locals.var_guard28 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_abet_t, locals.var_abet_t_dn0, locals.var_abet_t_dn1, locals.var_abet_t_dn3, locals.var_abet_t_dn4, locals.var_abet_t_dn5, locals.var_abet_t_dn6, locals.var_abet_t_dn7, locals.var_abet_t_dn8, locals.var_abet_t_dn9,)
    }
};
        locals.var_abet_t = assign1970_e1975;
        locals.var_abet_t_dn0 = assign1970_e1975_d_n0;
        locals.var_abet_t_dn1 = assign1970_e1975_d_n1;
        locals.var_abet_t_dn3 = assign1970_e1975_d_n3;
        locals.var_abet_t_dn4 = assign1970_e1975_d_n4;
        locals.var_abet_t_dn5 = assign1970_e1975_d_n5;
        locals.var_abet_t_dn6 = assign1970_e1975_d_n6;
        locals.var_abet_t_dn7 = assign1970_e1975_d_n7;
        locals.var_abet_t_dn8 = assign1970_e1975_d_n8;
        locals.var_abet_t_dn9 = assign1970_e1975_d_n9;

        let assign1980_e1979: f64 = (locals.var_vdei_t - p.p40);
        let assign1980_e1980: f64 = (-assign1980_e1979);
        let assign1980_e1982: f64 = (assign1980_e1980 / p.p31);
        let assign1980_e1983: f64 = (assign1980_e1982).exp();
        let assign1980_e1984: f64 = (p.p30 * assign1980_e1983);
        locals.var_ibetat0_t = assign1980_e1984;
        locals.var_ibetat0_t_dn4 = (p.p30 * (assign1980_e1983 * ((-locals.var_vdei_t_dn4) / p.p31)));

        let assign1990_e1987: f64 = if 1.0 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign1990_e1987;

        let (assign2000_e2009,) = {
    if (locals.var_guard31 != 0.0) {
        let assign2000_e1991: f64 = (2.0 * locals.var_vtnom);
        let assign2000_e1994: f64 = (p.p53 * 0.5);
        let assign2000_e1996: f64 = (assign2000_e1994 * locals.var_ovtnom);
        let assign2000_e1997: f64 = (assign2000_e1996).exp();
        let assign2000_e1999: f64 = (-0.5);
        let assign2000_e2001: f64 = (assign2000_e1999 * p.p53);
        let assign2000_e2003: f64 = (assign2000_e2001 * locals.var_ovtnom);
        let assign2000_e2004: f64 = (assign2000_e2003).exp();
        let assign2000_e2005: f64 = (assign2000_e1997 - assign2000_e2004);
        let assign2000_e2006: f64 = (assign2000_e2005).ln();
        let assign2000_e2007: f64 = (assign2000_e1991 * assign2000_e2006);
        (assign2000_e2007,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign2000_e2009;

        let (assign2010_e2027, assign2010_e2027_d_n4,) = {
    if (locals.var_guard31 != 0.0) {
        let assign2010_e2013: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign2010_e2017: f64 = (1.0 - locals.var_qtt0);
        let assign2010_e2018: f64 = (locals.var_vgbc0 * assign2010_e2017);
        let assign2010_e2019: f64 = (assign2010_e2013 + assign2010_e2018);
        let assign2010_e2022: f64 = (locals.var_mg * locals.var_vt);
        let assign2010_e2024: f64 = (assign2010_e2022 * locals.var_ln_qtt0);
        let assign2010_e2025: f64 = (assign2010_e2019 - assign2010_e2024);
        (assign2010_e2025, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgbc0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign2010_e2022 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign2010_e2027;
        locals.var_vdj_t_dn4 = assign2010_e2027_d_n4;

        let (assign2020_e2051, assign2020_e2051_d_n4,) = {
    if (locals.var_guard31 != 0.0) {
        let assign2020_e2032: f64 = (2.0 * locals.var_vt);
        let assign2020_e2038: f64 = (-locals.var_vdj_t);
        let assign2020_e2040: f64 = (assign2020_e2038 * locals.var_ovt);
        let assign2020_e2041: f64 = (assign2020_e2040).exp();
        let assign2020_e2042: f64 = (4.0 * assign2020_e2041);
        let assign2020_e2043: f64 = (1.0 + assign2020_e2042);
        let assign2020_e2044: f64 = (assign2020_e2043).sqrt();
        let assign2020_e2045: f64 = (1.0 + assign2020_e2044);
        let assign2020_e2046: f64 = (0.5 * assign2020_e2045);
        let assign2020_e2047: f64 = (assign2020_e2046).ln();
        let assign2020_e2048: f64 = (assign2020_e2032 * assign2020_e2047);
        let assign2020_e2049: f64 = (locals.var_vdj_t + assign2020_e2048);
        (assign2020_e2049, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign2020_e2047) + (assign2020_e2032 * ((0.5 * ((4.0 * (assign2020_e2041 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign2020_e2038 * locals.var_ovt_dn4)))) / (2.0 * assign2020_e2044))) / assign2020_e2046)))),)
    } else {
        (locals.var_vdcx_t, locals.var_vdcx_t_dn4,)
    }
};
        locals.var_vdcx_t = assign2020_e2051;
        locals.var_vdcx_t_dn4 = assign2020_e2051_d_n4;

        let (assign2030_e2063, assign2030_e2063_d_n4,) = {
    if (locals.var_guard31 != 0.0) {
        let assign2030_e2057: f64 = (p.p53 / locals.var_vdcx_t);
        let assign2030_e2058: f64 = (assign2030_e2057).ln();
        let assign2030_e2059: f64 = (p.p54 * assign2030_e2058);
        let assign2030_e2060: f64 = (assign2030_e2059).exp();
        let assign2030_e2061: f64 = assign2030_e2060;
        (assign2030_e2061, (assign2030_e2060 * (p.p54 * ((-((p.p53 * locals.var_vdcx_t_dn4) / (locals.var_vdcx_t * locals.var_vdcx_t))) / assign2030_e2057))),)
    } else {
        (locals.var_cratio_t, locals.var_cratio_t_dn4,)
    }
};
        locals.var_cratio_t = assign2030_e2063;
        locals.var_cratio_t_dn4 = assign2030_e2063_d_n4;

        let (assign2040_e2068, assign2040_e2068_d_n4,) = {
    if (locals.var_guard31 != 0.0) {
        let assign2040_e2066: f64 = (p.p55).abs();
        (assign2040_e2066, 0.0,)
    } else {
        (locals.var_ajcx_t, locals.var_ajcx_t_dn4,)
    }
};
        locals.var_ajcx_t = assign2040_e2068;
        locals.var_ajcx_t_dn4 = assign2040_e2068_d_n4;

        let assign2050_e2071: f64 = if p.p55 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign2050_e2071;

        let (assign2060_e2081, assign2060_e2081_d_n4,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_guard32 != 0.0)) {
        let assign2060_e2077: f64 = (p.p55 * locals.var_vdcx_t);
        let assign2060_e2079: f64 = (assign2060_e2077 / p.p53);
        (assign2060_e2079, ((p.p55 * locals.var_vdcx_t_dn4) / p.p53),)
    } else {
        (locals.var_ajcx_t, locals.var_ajcx_t_dn4,)
    }
};
        locals.var_ajcx_t = assign2060_e2081;
        locals.var_ajcx_t_dn4 = assign2060_e2081_d_n4;

        let (assign2070_e2086, assign2070_e2086_d_n4,) = {
    if (locals.var_guard31 == 0.0) {
        (1.0, 0.0,)
    } else {
        (locals.var_cratio_t, locals.var_cratio_t_dn4,)
    }
};
        locals.var_cratio_t = assign2070_e2086;
        locals.var_cratio_t_dn4 = assign2070_e2086_d_n4;

        let (assign2080_e2091, assign2080_e2091_d_n4,) = {
    if (locals.var_guard31 == 0.0) {
        (p.p53, 0.0,)
    } else {
        (locals.var_vdcx_t, locals.var_vdcx_t_dn4,)
    }
};
        locals.var_vdcx_t = assign2080_e2091;
        locals.var_vdcx_t_dn4 = assign2080_e2091_d_n4;

        let (assign2090_e2096, assign2090_e2096_d_n4,) = {
    if (locals.var_guard31 == 0.0) {
        (p.p55, 0.0,)
    } else {
        (locals.var_ajcx_t, locals.var_ajcx_t_dn4,)
    }
};
        locals.var_ajcx_t = assign2090_e2096;
        locals.var_ajcx_t_dn4 = assign2090_e2096_d_n4;

        let assign2100_e2099: f64 = if p.p0 <= 300.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign2100_e2099;

        let (assign2110_e2103, assign2110_e2103_d_n4,) = {
    if (locals.var_guard33 != 0.0) {
        (2.4, 0.0,)
    } else {
        (locals.var_ajcx_t, locals.var_ajcx_t_dn4,)
    }
};
        locals.var_ajcx_t = assign2110_e2103;
        locals.var_ajcx_t_dn4 = assign2110_e2103_d_n4;

        let assign2120_e2106: f64 = (locals.var_cratio_t * locals.var_cjcx01);
        locals.var_cjcx01_t = assign2120_e2106;
        locals.var_cjcx01_t_dn4 = (locals.var_cratio_t_dn4 * locals.var_cjcx01);

        let assign2130_e2109: f64 = (locals.var_cratio_t * locals.var_cjcx02);
        locals.var_cjcx02_t = assign2130_e2109;
        locals.var_cjcx02_t_dn4 = (locals.var_cratio_t_dn4 * locals.var_cjcx02);

        let assign2140_e2113: f64 = (locals.var_zetabcxt * locals.var_ln_qtt0);
        let assign2140_e2116: f64 = (p.p119 * locals.var_ovtnom);
        let assign2140_e2119: f64 = (1.0 - locals.var_tn2td);
        let assign2140_e2120: f64 = (assign2140_e2116 * assign2140_e2119);
        let assign2140_e2121: f64 = (assign2140_e2113 + assign2140_e2120);
        let assign2140_e2122: f64 = (assign2140_e2121).exp();
        let assign2140_e2123: f64 = (p.p25 * assign2140_e2122);
        locals.var_ibcxs_t = assign2140_e2123;
        locals.var_ibcxs_t_dn4 = (p.p25 * (assign2140_e2122 * ((locals.var_zetabcxt * locals.var_ln_qtt0_dn4) + (assign2140_e2116 * (-locals.var_tn2td_dn4)))));

        let assign2150_e2126: f64 = if p.p0 <= 300.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign2150_e2126;

        let assign2160_e2129: f64 = if p.p57 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign2160_e2129;

        let (assign2170_e2153,) = {
    if ((locals.var_guard34 != 0.0) && (locals.var_guard35 != 0.0)) {
        let assign2170_e2135: f64 = (2.0 * locals.var_vtnom);
        let assign2170_e2138: f64 = (p.p58 * 0.5);
        let assign2170_e2140: f64 = (assign2170_e2138 * locals.var_ovtnom);
        let assign2170_e2141: f64 = (assign2170_e2140).exp();
        let assign2170_e2143: f64 = (-0.5);
        let assign2170_e2145: f64 = (assign2170_e2143 * p.p58);
        let assign2170_e2147: f64 = (assign2170_e2145 * locals.var_ovtnom);
        let assign2170_e2148: f64 = (assign2170_e2147).exp();
        let assign2170_e2149: f64 = (assign2170_e2141 - assign2170_e2148);
        let assign2170_e2150: f64 = (assign2170_e2149).ln();
        let assign2170_e2151: f64 = (assign2170_e2135 * assign2170_e2150);
        (assign2170_e2151,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign2170_e2153;

        let (assign2180_e2173, assign2180_e2173_d_n4,) = {
    if ((locals.var_guard34 != 0.0) && (locals.var_guard35 != 0.0)) {
        let assign2180_e2159: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign2180_e2163: f64 = (1.0 - locals.var_qtt0);
        let assign2180_e2164: f64 = (locals.var_vgsc0 * assign2180_e2163);
        let assign2180_e2165: f64 = (assign2180_e2159 + assign2180_e2164);
        let assign2180_e2168: f64 = (locals.var_mg * locals.var_vt);
        let assign2180_e2170: f64 = (assign2180_e2168 * locals.var_ln_qtt0);
        let assign2180_e2171: f64 = (assign2180_e2165 - assign2180_e2170);
        (assign2180_e2171, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgsc0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign2180_e2168 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign2180_e2173;
        locals.var_vdj_t_dn4 = assign2180_e2173_d_n4;

        let (assign2190_e2199, assign2190_e2199_d_n4,) = {
    if ((locals.var_guard34 != 0.0) && (locals.var_guard35 != 0.0)) {
        let assign2190_e2180: f64 = (2.0 * locals.var_vt);
        let assign2190_e2186: f64 = (-locals.var_vdj_t);
        let assign2190_e2188: f64 = (assign2190_e2186 * locals.var_ovt);
        let assign2190_e2189: f64 = (assign2190_e2188).exp();
        let assign2190_e2190: f64 = (4.0 * assign2190_e2189);
        let assign2190_e2191: f64 = (1.0 + assign2190_e2190);
        let assign2190_e2192: f64 = (assign2190_e2191).sqrt();
        let assign2190_e2193: f64 = (1.0 + assign2190_e2192);
        let assign2190_e2194: f64 = (0.5 * assign2190_e2193);
        let assign2190_e2195: f64 = (assign2190_e2194).ln();
        let assign2190_e2196: f64 = (assign2190_e2180 * assign2190_e2195);
        let assign2190_e2197: f64 = (locals.var_vdj_t + assign2190_e2196);
        (assign2190_e2197, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign2190_e2195) + (assign2190_e2180 * ((0.5 * ((4.0 * (assign2190_e2189 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign2190_e2186 * locals.var_ovt_dn4)))) / (2.0 * assign2190_e2192))) / assign2190_e2194)))),)
    } else {
        (locals.var_vds_t, locals.var_vds_t_dn4,)
    }
};
        locals.var_vds_t = assign2190_e2199;
        locals.var_vds_t_dn4 = assign2190_e2199_d_n4;

        let (assign2200_e2213, assign2200_e2213_d_n4,) = {
    if ((locals.var_guard34 != 0.0) && (locals.var_guard35 != 0.0)) {
        let assign2200_e2207: f64 = (p.p58 / locals.var_vds_t);
        let assign2200_e2208: f64 = (assign2200_e2207).ln();
        let assign2200_e2209: f64 = (p.p59 * assign2200_e2208);
        let assign2200_e2210: f64 = (assign2200_e2209).exp();
        let assign2200_e2211: f64 = (p.p57 * assign2200_e2210);
        (assign2200_e2211, (p.p57 * (assign2200_e2210 * (p.p59 * ((-((p.p58 * locals.var_vds_t_dn4) / (locals.var_vds_t * locals.var_vds_t))) / assign2200_e2207)))),)
    } else {
        (locals.var_cjs0_t, locals.var_cjs0_t_dn4,)
    }
};
        locals.var_cjs0_t = assign2200_e2213;
        locals.var_cjs0_t_dn4 = assign2200_e2213_d_n4;

        let (assign2210_e2221, assign2210_e2221_d_n4,) = {
    if ((locals.var_guard34 != 0.0) && (locals.var_guard35 != 0.0)) {
        let assign2210_e2218: f64 = (-2.4);
        let assign2210_e2219: f64 = (assign2210_e2218).abs();
        (assign2210_e2219, 0.0,)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign2210_e2221;
        locals.var_ajs_t_dn4 = assign2210_e2221_d_n4;

        let assign2220_e2223: f64 = (-2.4);
        let assign2220_e2225: f64 = if assign2220_e2223 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign2220_e2225;

        let (assign2230_e2238, assign2230_e2238_d_n4,) = {
    if (((locals.var_guard34 != 0.0) && (locals.var_guard35 != 0.0)) && (locals.var_guard36 != 0.0)) {
        let assign2230_e2232: f64 = (-2.4);
        let assign2230_e2234: f64 = (assign2230_e2232 * locals.var_vds_t);
        let assign2230_e2236: f64 = (assign2230_e2234 / p.p58);
        (assign2230_e2236, ((assign2230_e2232 * locals.var_vds_t_dn4) / p.p58),)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign2230_e2238;
        locals.var_ajs_t_dn4 = assign2230_e2238_d_n4;

        let (assign2240_e2245, assign2240_e2245_d_n4,) = {
    if ((locals.var_guard34 != 0.0) && (locals.var_guard35 == 0.0)) {
        (p.p57, 0.0,)
    } else {
        (locals.var_cjs0_t, locals.var_cjs0_t_dn4,)
    }
};
        locals.var_cjs0_t = assign2240_e2245;
        locals.var_cjs0_t_dn4 = assign2240_e2245_d_n4;

        let (assign2250_e2252, assign2250_e2252_d_n4,) = {
    if ((locals.var_guard34 != 0.0) && (locals.var_guard35 == 0.0)) {
        (p.p58, 0.0,)
    } else {
        (locals.var_vds_t, locals.var_vds_t_dn4,)
    }
};
        locals.var_vds_t = assign2250_e2252;
        locals.var_vds_t_dn4 = assign2250_e2252_d_n4;

    }

    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let (assign2260_e2260, assign2260_e2260_d_n4,) = {
    if ((locals.var_guard34 != 0.0) && (locals.var_guard35 == 0.0)) {
        let assign2260_e2258: f64 = (-2.4);
        (assign2260_e2258, 0.0,)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign2260_e2260;
        locals.var_ajs_t_dn4 = assign2260_e2260_d_n4;

        let (assign2270_e2264,) = {
    if (locals.var_guard34 != 0.0) {
        (2.4,)
    } else {
        (locals.var_a_jsp,)
    }
};
        locals.var_a_jsp = assign2270_e2264;

        let assign2280_e2267: f64 = if p.p57 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign2280_e2267;

        let (assign2290_e2292,) = {
    if ((locals.var_guard34 == 0.0) && (locals.var_guard37 != 0.0)) {
        let assign2290_e2274: f64 = (2.0 * locals.var_vtnom);
        let assign2290_e2277: f64 = (p.p58 * 0.5);
        let assign2290_e2279: f64 = (assign2290_e2277 * locals.var_ovtnom);
        let assign2290_e2280: f64 = (assign2290_e2279).exp();
        let assign2290_e2282: f64 = (-0.5);
        let assign2290_e2284: f64 = (assign2290_e2282 * p.p58);
        let assign2290_e2286: f64 = (assign2290_e2284 * locals.var_ovtnom);
        let assign2290_e2287: f64 = (assign2290_e2286).exp();
        let assign2290_e2288: f64 = (assign2290_e2280 - assign2290_e2287);
        let assign2290_e2289: f64 = (assign2290_e2288).ln();
        let assign2290_e2290: f64 = (assign2290_e2274 * assign2290_e2289);
        (assign2290_e2290,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign2290_e2292;

        let (assign2300_e2313, assign2300_e2313_d_n4,) = {
    if ((locals.var_guard34 == 0.0) && (locals.var_guard37 != 0.0)) {
        let assign2300_e2299: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign2300_e2303: f64 = (1.0 - locals.var_qtt0);
        let assign2300_e2304: f64 = (locals.var_vgsc0 * assign2300_e2303);
        let assign2300_e2305: f64 = (assign2300_e2299 + assign2300_e2304);
        let assign2300_e2308: f64 = (locals.var_mg * locals.var_vt);
        let assign2300_e2310: f64 = (assign2300_e2308 * locals.var_ln_qtt0);
        let assign2300_e2311: f64 = (assign2300_e2305 - assign2300_e2310);
        (assign2300_e2311, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgsc0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign2300_e2308 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign2300_e2313;
        locals.var_vdj_t_dn4 = assign2300_e2313_d_n4;

        let (assign2310_e2340, assign2310_e2340_d_n4,) = {
    if ((locals.var_guard34 == 0.0) && (locals.var_guard37 != 0.0)) {
        let assign2310_e2321: f64 = (2.0 * locals.var_vt);
        let assign2310_e2327: f64 = (-locals.var_vdj_t);
        let assign2310_e2329: f64 = (assign2310_e2327 * locals.var_ovt);
        let assign2310_e2330: f64 = (assign2310_e2329).exp();
        let assign2310_e2331: f64 = (4.0 * assign2310_e2330);
        let assign2310_e2332: f64 = (1.0 + assign2310_e2331);
        let assign2310_e2333: f64 = (assign2310_e2332).sqrt();
        let assign2310_e2334: f64 = (1.0 + assign2310_e2333);
        let assign2310_e2335: f64 = (0.5 * assign2310_e2334);
        let assign2310_e2336: f64 = (assign2310_e2335).ln();
        let assign2310_e2337: f64 = (assign2310_e2321 * assign2310_e2336);
        let assign2310_e2338: f64 = (locals.var_vdj_t + assign2310_e2337);
        (assign2310_e2338, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign2310_e2336) + (assign2310_e2321 * ((0.5 * ((4.0 * (assign2310_e2330 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign2310_e2327 * locals.var_ovt_dn4)))) / (2.0 * assign2310_e2333))) / assign2310_e2335)))),)
    } else {
        (locals.var_vds_t, locals.var_vds_t_dn4,)
    }
};
        locals.var_vds_t = assign2310_e2340;
        locals.var_vds_t_dn4 = assign2310_e2340_d_n4;

        let (assign2320_e2355, assign2320_e2355_d_n4,) = {
    if ((locals.var_guard34 == 0.0) && (locals.var_guard37 != 0.0)) {
        let assign2320_e2349: f64 = (p.p58 / locals.var_vds_t);
        let assign2320_e2350: f64 = (assign2320_e2349).ln();
        let assign2320_e2351: f64 = (p.p59 * assign2320_e2350);
        let assign2320_e2352: f64 = (assign2320_e2351).exp();
        let assign2320_e2353: f64 = (p.p57 * assign2320_e2352);
        (assign2320_e2353, (p.p57 * (assign2320_e2352 * (p.p59 * ((-((p.p58 * locals.var_vds_t_dn4) / (locals.var_vds_t * locals.var_vds_t))) / assign2320_e2349)))),)
    } else {
        (locals.var_cjs0_t, locals.var_cjs0_t_dn4,)
    }
};
        locals.var_cjs0_t = assign2320_e2355;
        locals.var_cjs0_t_dn4 = assign2320_e2355_d_n4;

        let (assign2330_e2364, assign2330_e2364_d_n4,) = {
    if ((locals.var_guard34 == 0.0) && (locals.var_guard37 != 0.0)) {
        let assign2330_e2361: f64 = (-p.p60);
        let assign2330_e2362: f64 = (assign2330_e2361).abs();
        (assign2330_e2362, 0.0,)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign2330_e2364;
        locals.var_ajs_t_dn4 = assign2330_e2364_d_n4;

        let assign2340_e2366: f64 = (-p.p60);
        let assign2340_e2368: f64 = if assign2340_e2366 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign2340_e2368;

        let (assign2350_e2382, assign2350_e2382_d_n4,) = {
    if (((locals.var_guard34 == 0.0) && (locals.var_guard37 != 0.0)) && (locals.var_guard38 != 0.0)) {
        let assign2350_e2376: f64 = (-p.p60);
        let assign2350_e2378: f64 = (assign2350_e2376 * locals.var_vds_t);
        let assign2350_e2380: f64 = (assign2350_e2378 / p.p58);
        (assign2350_e2380, ((assign2350_e2376 * locals.var_vds_t_dn4) / p.p58),)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign2350_e2382;
        locals.var_ajs_t_dn4 = assign2350_e2382_d_n4;

        let (assign2360_e2390, assign2360_e2390_d_n4,) = {
    if ((locals.var_guard34 == 0.0) && (locals.var_guard37 == 0.0)) {
        (p.p57, 0.0,)
    } else {
        (locals.var_cjs0_t, locals.var_cjs0_t_dn4,)
    }
};
        locals.var_cjs0_t = assign2360_e2390;
        locals.var_cjs0_t_dn4 = assign2360_e2390_d_n4;

        let (assign2370_e2398, assign2370_e2398_d_n4,) = {
    if ((locals.var_guard34 == 0.0) && (locals.var_guard37 == 0.0)) {
        (p.p58, 0.0,)
    } else {
        (locals.var_vds_t, locals.var_vds_t_dn4,)
    }
};
        locals.var_vds_t = assign2370_e2398;
        locals.var_vds_t_dn4 = assign2370_e2398_d_n4;

        let (assign2380_e2407, assign2380_e2407_d_n4,) = {
    if ((locals.var_guard34 == 0.0) && (locals.var_guard37 == 0.0)) {
        let assign2380_e2405: f64 = (-p.p60);
        (assign2380_e2405, 0.0,)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign2380_e2407;
        locals.var_ajs_t_dn4 = assign2380_e2407_d_n4;

        let (assign2390_e2412,) = {
    if (locals.var_guard34 == 0.0) {
        (p.p60,)
    } else {
        (locals.var_a_jsp,)
    }
};
        locals.var_a_jsp = assign2390_e2412;

        let assign2400_e2416: f64 = (locals.var_zetasct * locals.var_ln_qtt0);
        let assign2400_e2419: f64 = (p.p120 * locals.var_ovtnom);
        let assign2400_e2422: f64 = (1.0 - locals.var_tn2td);
        let assign2400_e2423: f64 = (assign2400_e2419 * assign2400_e2422);
        let assign2400_e2424: f64 = (assign2400_e2416 + assign2400_e2423);
        let assign2400_e2425: f64 = (assign2400_e2424).exp();
        let assign2400_e2426: f64 = (p.p99 * assign2400_e2425);
        locals.var_iscs_t = assign2400_e2426;
        locals.var_iscs_t_dn4 = (p.p99 * (assign2400_e2425 * ((locals.var_zetasct * locals.var_ln_qtt0_dn4) + (assign2400_e2419 * (-locals.var_tn2td_dn4)))));

        let assign2410_e2430: f64 = (locals.var_zetasct * locals.var_ln_qtt0);
        let assign2410_e2433: f64 = (p.p119 * locals.var_ovtnom);
        let assign2410_e2436: f64 = (1.0 - locals.var_tn2td);
        let assign2410_e2437: f64 = (assign2410_e2433 * assign2410_e2436);
        let assign2410_e2438: f64 = (assign2410_e2430 + assign2410_e2437);
        let assign2410_e2439: f64 = (assign2410_e2438).exp();
        let assign2410_e2440: f64 = (p.p97 * assign2410_e2439);
        locals.var_itss_t = assign2410_e2440;
        locals.var_itss_t_dn4 = (p.p97 * (assign2410_e2439 * ((locals.var_zetasct * locals.var_ln_qtt0_dn4) + (assign2410_e2433 * (-locals.var_tn2td_dn4)))));

        let assign2420_e2444: f64 = (p.p138 - 1.0);
        let assign2420_e2446: f64 = (assign2420_e2444 * locals.var_ln_qtt0);
        let assign2420_e2447: f64 = (assign2420_e2446).exp();
        let assign2420_e2448: f64 = (p.p101 * assign2420_e2447);
        locals.var_tsf_t = assign2420_e2448;
        locals.var_tsf_t_dn4 = (p.p101 * (assign2420_e2447 * (assign2420_e2444 * locals.var_ln_qtt0_dn4)));

        let assign2430_e2451: f64 = if p.p63 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign2430_e2451;

        let assign2440_e2454: f64 = if p.p62 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign2440_e2454;

        let (assign2450_e2478,) = {
    if ((locals.var_guard39 != 0.0) && (locals.var_guard40 != 0.0)) {
        let assign2450_e2460: f64 = (2.0 * locals.var_vtnom);
        let assign2450_e2463: f64 = (p.p63 * 0.5);
        let assign2450_e2465: f64 = (assign2450_e2463 * locals.var_ovtnom);
        let assign2450_e2466: f64 = (assign2450_e2465).exp();
        let assign2450_e2468: f64 = (-0.5);
        let assign2450_e2470: f64 = (assign2450_e2468 * p.p63);
        let assign2450_e2472: f64 = (assign2450_e2470 * locals.var_ovtnom);
        let assign2450_e2473: f64 = (assign2450_e2472).exp();
        let assign2450_e2474: f64 = (assign2450_e2466 - assign2450_e2473);
        let assign2450_e2475: f64 = (assign2450_e2474).ln();
        let assign2450_e2476: f64 = (assign2450_e2460 * assign2450_e2475);
        (assign2450_e2476,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign2450_e2478;

        let (assign2460_e2498, assign2460_e2498_d_n4,) = {
    if ((locals.var_guard39 != 0.0) && (locals.var_guard40 != 0.0)) {
        let assign2460_e2484: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign2460_e2488: f64 = (1.0 - locals.var_qtt0);
        let assign2460_e2489: f64 = (locals.var_vgsc0 * assign2460_e2488);
        let assign2460_e2490: f64 = (assign2460_e2484 + assign2460_e2489);
        let assign2460_e2493: f64 = (locals.var_mg * locals.var_vt);
        let assign2460_e2495: f64 = (assign2460_e2493 * locals.var_ln_qtt0);
        let assign2460_e2496: f64 = (assign2460_e2490 - assign2460_e2495);
        (assign2460_e2496, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgsc0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign2460_e2493 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign2460_e2498;
        locals.var_vdj_t_dn4 = assign2460_e2498_d_n4;

        let (assign2470_e2524, assign2470_e2524_d_n4,) = {
    if ((locals.var_guard39 != 0.0) && (locals.var_guard40 != 0.0)) {
        let assign2470_e2505: f64 = (2.0 * locals.var_vt);
        let assign2470_e2511: f64 = (-locals.var_vdj_t);
        let assign2470_e2513: f64 = (assign2470_e2511 * locals.var_ovt);
        let assign2470_e2514: f64 = (assign2470_e2513).exp();
        let assign2470_e2515: f64 = (4.0 * assign2470_e2514);
        let assign2470_e2516: f64 = (1.0 + assign2470_e2515);
        let assign2470_e2517: f64 = (assign2470_e2516).sqrt();
        let assign2470_e2518: f64 = (1.0 + assign2470_e2517);
        let assign2470_e2519: f64 = (0.5 * assign2470_e2518);
        let assign2470_e2520: f64 = (assign2470_e2519).ln();
        let assign2470_e2521: f64 = (assign2470_e2505 * assign2470_e2520);
        let assign2470_e2522: f64 = (locals.var_vdj_t + assign2470_e2521);
        (assign2470_e2522, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign2470_e2520) + (assign2470_e2505 * ((0.5 * ((4.0 * (assign2470_e2514 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign2470_e2511 * locals.var_ovt_dn4)))) / (2.0 * assign2470_e2517))) / assign2470_e2519)))),)
    } else {
        (locals.var_vdsp_t, locals.var_vdsp_t_dn4,)
    }
};
        locals.var_vdsp_t = assign2470_e2524;
        locals.var_vdsp_t_dn4 = assign2470_e2524_d_n4;

        let (assign2480_e2538, assign2480_e2538_d_n4,) = {
    if ((locals.var_guard39 != 0.0) && (locals.var_guard40 != 0.0)) {
        let assign2480_e2532: f64 = (p.p63 / locals.var_vdsp_t);
        let assign2480_e2533: f64 = (assign2480_e2532).ln();
        let assign2480_e2534: f64 = (p.p64 * assign2480_e2533);
        let assign2480_e2535: f64 = (assign2480_e2534).exp();
        let assign2480_e2536: f64 = (p.p62 * assign2480_e2535);
        (assign2480_e2536, (p.p62 * (assign2480_e2535 * (p.p64 * ((-((p.p63 * locals.var_vdsp_t_dn4) / (locals.var_vdsp_t * locals.var_vdsp_t))) / assign2480_e2532)))),)
    } else {
        (locals.var_cscp0_t, locals.var_cscp0_t_dn4,)
    }
};
        locals.var_cscp0_t = assign2480_e2538;
        locals.var_cscp0_t_dn4 = assign2480_e2538_d_n4;

        let (assign2490_e2546, assign2490_e2546_d_n4,) = {
    if ((locals.var_guard39 != 0.0) && (locals.var_guard40 != 0.0)) {
        let assign2490_e2543: f64 = (-locals.var_a_jsp);
        let assign2490_e2544: f64 = (assign2490_e2543).abs();
        (assign2490_e2544, 0.0,)
    } else {
        (locals.var_ajsp_t, locals.var_ajsp_t_dn4,)
    }
};
        locals.var_ajsp_t = assign2490_e2546;
        locals.var_ajsp_t_dn4 = assign2490_e2546_d_n4;

        let assign2500_e2548: f64 = (-locals.var_a_jsp);
        let assign2500_e2550: f64 = if assign2500_e2548 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign2500_e2550;

        let (assign2510_e2563, assign2510_e2563_d_n4,) = {
    if (((locals.var_guard39 != 0.0) && (locals.var_guard40 != 0.0)) && (locals.var_guard41 != 0.0)) {
        let assign2510_e2557: f64 = (-locals.var_a_jsp);
        let assign2510_e2559: f64 = (assign2510_e2557 * locals.var_vdsp_t);
        let assign2510_e2561: f64 = (assign2510_e2559 / p.p63);
        (assign2510_e2561, ((assign2510_e2557 * locals.var_vdsp_t_dn4) / p.p63),)
    } else {
        (locals.var_ajsp_t, locals.var_ajsp_t_dn4,)
    }
};
        locals.var_ajsp_t = assign2510_e2563;
        locals.var_ajsp_t_dn4 = assign2510_e2563_d_n4;

        let (assign2520_e2570, assign2520_e2570_d_n4,) = {
    if ((locals.var_guard39 != 0.0) && (locals.var_guard40 == 0.0)) {
        (p.p62, 0.0,)
    } else {
        (locals.var_cscp0_t, locals.var_cscp0_t_dn4,)
    }
};
        locals.var_cscp0_t = assign2520_e2570;
        locals.var_cscp0_t_dn4 = assign2520_e2570_d_n4;

        let (assign2530_e2577, assign2530_e2577_d_n4,) = {
    if ((locals.var_guard39 != 0.0) && (locals.var_guard40 == 0.0)) {
        (p.p63, 0.0,)
    } else {
        (locals.var_vdsp_t, locals.var_vdsp_t_dn4,)
    }
};
        locals.var_vdsp_t = assign2530_e2577;
        locals.var_vdsp_t_dn4 = assign2530_e2577_d_n4;

        let (assign2540_e2585, assign2540_e2585_d_n4,) = {
    if ((locals.var_guard39 != 0.0) && (locals.var_guard40 == 0.0)) {
        let assign2540_e2583: f64 = (-locals.var_a_jsp);
        (assign2540_e2583, 0.0,)
    } else {
        (locals.var_ajsp_t, locals.var_ajsp_t_dn4,)
    }
};
        locals.var_ajsp_t = assign2540_e2585;
        locals.var_ajsp_t_dn4 = assign2540_e2585_d_n4;

        let (assign2550_e2590, assign2550_e2590_d_n4,) = {
    if (locals.var_guard39 == 0.0) {
        (p.p62, 0.0,)
    } else {
        (locals.var_cscp0_t, locals.var_cscp0_t_dn4,)
    }
};
        locals.var_cscp0_t = assign2550_e2590;
        locals.var_cscp0_t_dn4 = assign2550_e2590_d_n4;

        let (assign2560_e2595, assign2560_e2595_d_n4,) = {
    if (locals.var_guard39 == 0.0) {
        (p.p63, 0.0,)
    } else {
        (locals.var_vdsp_t, locals.var_vdsp_t_dn4,)
    }
};
        locals.var_vdsp_t = assign2560_e2595;
        locals.var_vdsp_t_dn4 = assign2560_e2595_d_n4;

        let (assign2570_e2600, assign2570_e2600_d_n4,) = {
    if (locals.var_guard39 == 0.0) {
        (locals.var_a_jsp, 0.0,)
    } else {
        (locals.var_ajsp_t, locals.var_ajsp_t_dn4,)
    }
};
        locals.var_ajsp_t = assign2570_e2600;
        locals.var_ajsp_t_dn4 = assign2570_e2600_d_n4;

        let assign2580_e2604: f64 = (p.p136 * locals.var_ln_qtt0);
        let assign2580_e2605: f64 = (assign2580_e2604).exp();
        let assign2580_e2606: f64 = (p.p96 * assign2580_e2605);
        locals.var_rcx_t = assign2580_e2606;
        locals.var_rcx_t_dn4 = (p.p96 * (assign2580_e2605 * (p.p136 * locals.var_ln_qtt0_dn4)));

        let assign2590_e2610: f64 = (p.p135 * locals.var_ln_qtt0);
        let assign2590_e2611: f64 = (assign2590_e2610).exp();
        let assign2590_e2612: f64 = (p.p90 * assign2590_e2611);
        locals.var_rbx_t = assign2590_e2612;
        locals.var_rbx_t_dn4 = (p.p90 * (assign2590_e2611 * (p.p135 * locals.var_ln_qtt0_dn4)));

        let assign2600_e2616: f64 = (p.p137 * locals.var_ln_qtt0);
        let assign2600_e2617: f64 = (assign2600_e2616).exp();
        let assign2600_e2618: f64 = (p.p95 * assign2600_e2617);
        locals.var_re_t = assign2600_e2618;
        locals.var_re_t_dn4 = (p.p95 * (assign2600_e2617 * (p.p137 * locals.var_ln_qtt0_dn4)));

        let assign2610_e2622: f64 = (p.p143 * locals.var_ln_qtt0);
        let assign2610_e2623: f64 = (assign2610_e2622).exp();
        let assign2610_e2624: f64 = (p.p142 * assign2610_e2623);
        let assign2610_e2628: f64 = (p.p144 * locals.var_dtdev);
        let assign2610_e2629: f64 = (1.0 + assign2610_e2628);
        let assign2610_e2630: f64 = (assign2610_e2624 * assign2610_e2629);
        locals.var_rth_t = assign2610_e2630;
        locals.var_rth_t_dn4 = (((p.p142 * (assign2610_e2623 * (p.p143 * locals.var_ln_qtt0_dn4))) * assign2610_e2629) + (assign2610_e2624 * (p.p144 * locals.var_dtdev_dn4)));

        let assign2620_e2641: f64 = if (((p.p141 != 0.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard42 = assign2620_e2641;

        let (assign2630_e2649, assign2630_e2649_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2630_e2645: f64 = (locals.var_tamb + p.p147);
        let assign2630_e2647: f64 = (assign2630_e2645 + (nv4 - 0.0));
        (assign2630_e2647, 1.0,)
    } else {
        (locals.var_tdev, locals.var_tdev_dn4,)
    }
};
        locals.var_tdev = assign2630_e2649;
        locals.var_tdev_dn4 = assign2630_e2649_d_n4;

        let assign2640_e2652: f64 = (-200.0);
        let assign2640_e2654: f64 = (assign2640_e2652 + 273.15);
        let assign2640_e2655: f64 = if locals.var_tdev < assign2640_e2654 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign2640_e2655;

        let (assign2650_e2664, assign2650_e2664_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard43 != 0.0)) {
        let assign2650_e2660: f64 = (-200.0);
        let assign2650_e2662: f64 = (assign2650_e2660 + 273.15);
        (assign2650_e2662, 0.0,)
    } else {
        (locals.var_tdev, locals.var_tdev_dn4,)
    }
};
        locals.var_tdev = assign2650_e2664;
        locals.var_tdev_dn4 = assign2650_e2664_d_n4;

        let assign2660_e2668: f64 = (326.85 + 273.15);
        let assign2660_e2669: f64 = if locals.var_tdev > assign2660_e2668 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign2660_e2669;

        let (assign2670_e2680, assign2670_e2680_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard43 == 0.0)) && (locals.var_guard44 != 0.0)) {
        let assign2670_e2678: f64 = (326.85 + 273.15);
        (assign2670_e2678, 0.0,)
    } else {
        (locals.var_tdev, locals.var_tdev_dn4,)
    }
};
        locals.var_tdev = assign2670_e2680;
        locals.var_tdev_dn4 = assign2670_e2680_d_n4;

        let (assign2680_e2686, assign2680_e2686_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2680_e2684: f64 = (locals.var_kb2q * locals.var_tdev);
        (assign2680_e2684, (locals.var_kb2q * locals.var_tdev_dn4),)
    } else {
        (locals.var_vt, locals.var_vt_dn4,)
    }
};
        locals.var_vt = assign2680_e2686;
        locals.var_vt_dn4 = assign2680_e2686_d_n4;

        let (assign2690_e2692, assign2690_e2692_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2690_e2690: f64 = (1.0 / locals.var_vt);
        (assign2690_e2690, (-(locals.var_vt_dn4 / (locals.var_vt * locals.var_vt))),)
    } else {
        (locals.var_ovt, locals.var_ovt_dn4,)
    }
};
        locals.var_ovt = assign2690_e2692;
        locals.var_ovt_dn4 = assign2690_e2692_d_n4;

        let (assign2700_e2698, assign2700_e2698_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2700_e2696: f64 = (locals.var_tdev - locals.var_tnom);
        (assign2700_e2696, locals.var_tdev_dn4,)
    } else {
        (locals.var_dtdev, locals.var_dtdev_dn4,)
    }
};
        locals.var_dtdev = assign2700_e2698;
        locals.var_dtdev_dn4 = assign2700_e2698_d_n4;

        let (assign2710_e2704, assign2710_e2704_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2710_e2702: f64 = (locals.var_tnom / locals.var_tdev);
        (assign2710_e2702, (-((locals.var_tnom * locals.var_tdev_dn4) / (locals.var_tdev * locals.var_tdev))),)
    } else {
        (locals.var_tn2td, locals.var_tn2td_dn4,)
    }
};
        locals.var_tn2td = assign2710_e2704;
        locals.var_tn2td_dn4 = assign2710_e2704_d_n4;

        let (assign2720_e2710, assign2720_e2710_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2720_e2708: f64 = (locals.var_tdev / locals.var_tnom);
        (assign2720_e2708, (locals.var_tdev_dn4 / locals.var_tnom),)
    } else {
        (locals.var_qtt0, locals.var_qtt0_dn4,)
    }
};
        locals.var_qtt0 = assign2720_e2710;
        locals.var_qtt0_dn4 = assign2720_e2710_d_n4;

        let (assign2730_e2715, assign2730_e2715_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2730_e2713: f64 = (locals.var_qtt0).ln();
        (assign2730_e2713, (locals.var_qtt0_dn4 / locals.var_qtt0),)
    } else {
        (locals.var_ln_qtt0, locals.var_ln_qtt0_dn4,)
    }
};
        locals.var_ln_qtt0 = assign2730_e2715;
        locals.var_ln_qtt0_dn4 = assign2730_e2715_d_n4;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2740_e2724, assign2740_e2724_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2740_e2719: f64 = (p.p121 * locals.var_tdev);
        let assign2740_e2721: f64 = (locals.var_tdev).ln();
        let assign2740_e2722: f64 = (assign2740_e2719 * assign2740_e2721);
        (assign2740_e2722, (((p.p121 * locals.var_tdev_dn4) * assign2740_e2721) + (assign2740_e2719 * (locals.var_tdev_dn4 / locals.var_tdev))),)
    } else {
        (locals.var_k1, locals.var_k1_dn4,)
    }
};
        locals.var_k1 = assign2740_e2724;
        locals.var_k1_dn4 = assign2740_e2724_d_n4;

        let (assign2750_e2730, assign2750_e2730_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2750_e2728: f64 = (p.p122 * locals.var_tdev);
        (assign2750_e2728, (p.p122 * locals.var_tdev_dn4),)
    } else {
        (locals.var_k2, locals.var_k2_dn4,)
    }
};
        locals.var_k2 = assign2750_e2730;
        locals.var_k2_dn4 = assign2750_e2730_d_n4;

        let (assign2760_e2738, assign2760_e2738_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2760_e2734: f64 = (p.p117 + locals.var_k1);
        let assign2760_e2736: f64 = (assign2760_e2734 + locals.var_k2);
        (assign2760_e2736, (locals.var_k1_dn4 + locals.var_k2_dn4),)
    } else {
        (locals.var_vgb_t, locals.var_vgb_t_dn4,)
    }
};
        locals.var_vgb_t = assign2760_e2738;
        locals.var_vgb_t_dn4 = assign2760_e2738_d_n4;

        let (assign2770_e2746, assign2770_e2746_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2770_e2742: f64 = (p.p118 + locals.var_k1);
        let assign2770_e2744: f64 = (assign2770_e2742 + locals.var_k2);
        (assign2770_e2744, (locals.var_k1_dn4 + locals.var_k2_dn4),)
    } else {
        (locals.var_vge_t, locals.var_vge_t_dn4,)
    }
};
        locals.var_vge_t = assign2770_e2746;
        locals.var_vge_t_dn4 = assign2770_e2746_d_n4;

        let (assign2780_e2754, assign2780_e2754_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2780_e2750: f64 = (p.p119 + locals.var_k1);
        let assign2780_e2752: f64 = (assign2780_e2750 + locals.var_k2);
        (assign2780_e2752, (locals.var_k1_dn4 + locals.var_k2_dn4),)
    } else {
        (locals.var_vgc_t, locals.var_vgc_t_dn4,)
    }
};
        locals.var_vgc_t = assign2780_e2754;
        locals.var_vgc_t_dn4 = assign2780_e2754_d_n4;

        let (assign2790_e2762, assign2790_e2762_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2790_e2758: f64 = (locals.var_vgb_t + locals.var_vge_t);
        let assign2790_e2760: f64 = (assign2790_e2758 * 0.5);
        (assign2790_e2760, ((locals.var_vgb_t_dn4 + locals.var_vge_t_dn4) * 0.5),)
    } else {
        (locals.var_vgbe_t, locals.var_vgbe_t_dn4,)
    }
};
        locals.var_vgbe_t = assign2790_e2762;
        locals.var_vgbe_t_dn4 = assign2790_e2762_d_n4;

        let (assign2800_e2770, assign2800_e2770_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2800_e2766: f64 = (locals.var_vgb_t + locals.var_vgc_t);
        let assign2800_e2768: f64 = (assign2800_e2766 * 0.5);
        (assign2800_e2768, ((locals.var_vgb_t_dn4 + locals.var_vgc_t_dn4) * 0.5),)
    } else {
        (locals.var_vgbc_t, locals.var_vgbc_t_dn4,)
    }
};
        locals.var_vgbc_t = assign2800_e2770;
        locals.var_vgbc_t_dn4 = assign2800_e2770_d_n4;

        let assign2810_e2773: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign2810_e2773;

        let (assign2820_e2797,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard45 != 0.0)) {
        let assign2820_e2779: f64 = (2.0 * locals.var_vtnom);
        let assign2820_e2782: f64 = (p.p40 * 0.5);
        let assign2820_e2784: f64 = (assign2820_e2782 * locals.var_ovtnom);
        let assign2820_e2785: f64 = (assign2820_e2784).exp();
        let assign2820_e2787: f64 = (-0.5);
        let assign2820_e2789: f64 = (assign2820_e2787 * p.p40);
        let assign2820_e2791: f64 = (assign2820_e2789 * locals.var_ovtnom);
        let assign2820_e2792: f64 = (assign2820_e2791).exp();
        let assign2820_e2793: f64 = (assign2820_e2785 - assign2820_e2792);
        let assign2820_e2794: f64 = (assign2820_e2793).ln();
        let assign2820_e2795: f64 = (assign2820_e2779 * assign2820_e2794);
        (assign2820_e2795,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign2820_e2797;

        let (assign2830_e2817, assign2830_e2817_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard45 != 0.0)) {
        let assign2830_e2803: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign2830_e2807: f64 = (1.0 - locals.var_qtt0);
        let assign2830_e2808: f64 = (locals.var_vgbe0 * assign2830_e2807);
        let assign2830_e2809: f64 = (assign2830_e2803 + assign2830_e2808);
        let assign2830_e2812: f64 = (locals.var_mg * locals.var_vt);
        let assign2830_e2814: f64 = (assign2830_e2812 * locals.var_ln_qtt0);
        let assign2830_e2815: f64 = (assign2830_e2809 - assign2830_e2814);
        (assign2830_e2815, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgbe0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign2830_e2812 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign2830_e2817;
        locals.var_vdj_t_dn4 = assign2830_e2817_d_n4;

        let (assign2840_e2843, assign2840_e2843_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard45 != 0.0)) {
        let assign2840_e2824: f64 = (2.0 * locals.var_vt);
        let assign2840_e2830: f64 = (-locals.var_vdj_t);
        let assign2840_e2832: f64 = (assign2840_e2830 * locals.var_ovt);
        let assign2840_e2833: f64 = (assign2840_e2832).exp();
        let assign2840_e2834: f64 = (4.0 * assign2840_e2833);
        let assign2840_e2835: f64 = (1.0 + assign2840_e2834);
        let assign2840_e2836: f64 = (assign2840_e2835).sqrt();
        let assign2840_e2837: f64 = (1.0 + assign2840_e2836);
        let assign2840_e2838: f64 = (0.5 * assign2840_e2837);
        let assign2840_e2839: f64 = (assign2840_e2838).ln();
        let assign2840_e2840: f64 = (assign2840_e2824 * assign2840_e2839);
        let assign2840_e2841: f64 = (locals.var_vdj_t + assign2840_e2840);
        (assign2840_e2841, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign2840_e2839) + (assign2840_e2824 * ((0.5 * ((4.0 * (assign2840_e2833 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign2840_e2830 * locals.var_ovt_dn4)))) / (2.0 * assign2840_e2836))) / assign2840_e2838)))),)
    } else {
        (locals.var_vdei_t, locals.var_vdei_t_dn4,)
    }
};
        locals.var_vdei_t = assign2840_e2843;
        locals.var_vdei_t_dn4 = assign2840_e2843_d_n4;

        let (assign2850_e2857, assign2850_e2857_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard45 != 0.0)) {
        let assign2850_e2851: f64 = (p.p40 / locals.var_vdei_t);
        let assign2850_e2852: f64 = (assign2850_e2851).ln();
        let assign2850_e2853: f64 = (p.p41 * assign2850_e2852);
        let assign2850_e2854: f64 = (assign2850_e2853).exp();
        let assign2850_e2855: f64 = (p.p39 * assign2850_e2854);
        (assign2850_e2855, (p.p39 * (assign2850_e2854 * (p.p41 * ((-((p.p40 * locals.var_vdei_t_dn4) / (locals.var_vdei_t * locals.var_vdei_t))) / assign2850_e2851)))),)
    } else {
        (locals.var_cjei0_t, locals.var_cjei0_t_dn4,)
    }
};
        locals.var_cjei0_t = assign2850_e2857;
        locals.var_cjei0_t_dn4 = assign2850_e2857_d_n4;

        let (assign2860_e2864, assign2860_e2864_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard45 != 0.0)) {
        let assign2860_e2862: f64 = (p.p42).abs();
        (assign2860_e2862, 0.0,)
    } else {
        (locals.var_ajei_t, locals.var_ajei_t_dn4,)
    }
};
        locals.var_ajei_t = assign2860_e2864;
        locals.var_ajei_t_dn4 = assign2860_e2864_d_n4;

        let assign2870_e2867: f64 = if p.p42 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign2870_e2867;

        let (assign2880_e2879, assign2880_e2879_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard45 != 0.0)) && (locals.var_guard46 != 0.0)) {
        let assign2880_e2875: f64 = (p.p42 * locals.var_vdei_t);
        let assign2880_e2877: f64 = (assign2880_e2875 / p.p40);
        (assign2880_e2877, ((p.p42 * locals.var_vdei_t_dn4) / p.p40),)
    } else {
        (locals.var_ajei_t, locals.var_ajei_t_dn4,)
    }
};
        locals.var_ajei_t = assign2880_e2879;
        locals.var_ajei_t_dn4 = assign2880_e2879_d_n4;

        let (assign2890_e2886, assign2890_e2886_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard45 == 0.0)) {
        (p.p39, 0.0,)
    } else {
        (locals.var_cjei0_t, locals.var_cjei0_t_dn4,)
    }
};
        locals.var_cjei0_t = assign2890_e2886;
        locals.var_cjei0_t_dn4 = assign2890_e2886_d_n4;

        let (assign2900_e2893, assign2900_e2893_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard45 == 0.0)) {
        (p.p40, 0.0,)
    } else {
        (locals.var_vdei_t, locals.var_vdei_t_dn4,)
    }
};
        locals.var_vdei_t = assign2900_e2893;
        locals.var_vdei_t_dn4 = assign2900_e2893_d_n4;

        let (assign2910_e2900, assign2910_e2900_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard45 == 0.0)) {
        (p.p42, 0.0,)
    } else {
        (locals.var_ajei_t, locals.var_ajei_t_dn4,)
    }
};
        locals.var_ajei_t = assign2910_e2900;
        locals.var_ajei_t_dn4 = assign2910_e2900_d_n4;

        let (assign2920_e2917, assign2920_e2917_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2920_e2905: f64 = (p.p124 * locals.var_ln_qtt0);
        let assign2920_e2908: f64 = (p.p118 * locals.var_ovtnom);
        let assign2920_e2911: f64 = (1.0 - locals.var_tn2td);
        let assign2920_e2912: f64 = (assign2920_e2908 * assign2920_e2911);
        let assign2920_e2913: f64 = (assign2920_e2905 + assign2920_e2912);
        let assign2920_e2914: f64 = (assign2920_e2913).exp();
        let assign2920_e2915: f64 = (p.p14 * assign2920_e2914);
        (assign2920_e2915, (p.p14 * (assign2920_e2914 * ((p.p124 * locals.var_ln_qtt0_dn4) + (assign2920_e2908 * (-locals.var_tn2td_dn4))))),)
    } else {
        (locals.var_ibeis_t, locals.var_ibeis_t_dn4,)
    }
};
        locals.var_ibeis_t = assign2920_e2917;
        locals.var_ibeis_t_dn4 = assign2920_e2917_d_n4;

        let (assign2930_e2938, assign2930_e2938_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2930_e2922: f64 = (locals.var_mg / p.p17);
        let assign2930_e2924: f64 = (assign2930_e2922 * locals.var_ln_qtt0);
        let assign2930_e2927: f64 = (locals.var_vgbe0 * locals.var_ovtnom);
        let assign2930_e2930: f64 = (1.0 - locals.var_tn2td);
        let assign2930_e2931: f64 = (assign2930_e2927 * assign2930_e2930);
        let assign2930_e2933: f64 = (assign2930_e2931 / p.p17);
        let assign2930_e2934: f64 = (assign2930_e2924 + assign2930_e2933);
        let assign2930_e2935: f64 = (assign2930_e2934).exp();
        let assign2930_e2936: f64 = (p.p16 * assign2930_e2935);
        (assign2930_e2936, (p.p16 * (assign2930_e2935 * ((assign2930_e2922 * locals.var_ln_qtt0_dn4) + ((assign2930_e2927 * (-locals.var_tn2td_dn4)) / p.p17)))),)
    } else {
        (locals.var_ireis_t, locals.var_ireis_t_dn4,)
    }
};
        locals.var_ireis_t = assign2930_e2938;
        locals.var_ireis_t_dn4 = assign2930_e2938_d_n4;

        let assign2940_e2941: f64 = if p.p47 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign2940_e2941;

        let (assign2950_e2965,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard47 != 0.0)) {
        let assign2950_e2947: f64 = (2.0 * locals.var_vtnom);
        let assign2950_e2950: f64 = (p.p48 * 0.5);
        let assign2950_e2952: f64 = (assign2950_e2950 * locals.var_ovtnom);
        let assign2950_e2953: f64 = (assign2950_e2952).exp();
        let assign2950_e2955: f64 = (-0.5);
        let assign2950_e2957: f64 = (assign2950_e2955 * p.p48);
        let assign2950_e2959: f64 = (assign2950_e2957 * locals.var_ovtnom);
        let assign2950_e2960: f64 = (assign2950_e2959).exp();
        let assign2950_e2961: f64 = (assign2950_e2953 - assign2950_e2960);
        let assign2950_e2962: f64 = (assign2950_e2961).ln();
        let assign2950_e2963: f64 = (assign2950_e2947 * assign2950_e2962);
        (assign2950_e2963,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign2950_e2965;

        let (assign2960_e2985, assign2960_e2985_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard47 != 0.0)) {
        let assign2960_e2971: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign2960_e2975: f64 = (1.0 - locals.var_qtt0);
        let assign2960_e2976: f64 = (locals.var_vgbc0 * assign2960_e2975);
        let assign2960_e2977: f64 = (assign2960_e2971 + assign2960_e2976);
        let assign2960_e2980: f64 = (locals.var_mg * locals.var_vt);
        let assign2960_e2982: f64 = (assign2960_e2980 * locals.var_ln_qtt0);
        let assign2960_e2983: f64 = (assign2960_e2977 - assign2960_e2982);
        (assign2960_e2983, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgbc0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign2960_e2980 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign2960_e2985;
        locals.var_vdj_t_dn4 = assign2960_e2985_d_n4;

        let (assign2970_e3011, assign2970_e3011_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard47 != 0.0)) {
        let assign2970_e2992: f64 = (2.0 * locals.var_vt);
        let assign2970_e2998: f64 = (-locals.var_vdj_t);
        let assign2970_e3000: f64 = (assign2970_e2998 * locals.var_ovt);
        let assign2970_e3001: f64 = (assign2970_e3000).exp();
        let assign2970_e3002: f64 = (4.0 * assign2970_e3001);
        let assign2970_e3003: f64 = (1.0 + assign2970_e3002);
        let assign2970_e3004: f64 = (assign2970_e3003).sqrt();
        let assign2970_e3005: f64 = (1.0 + assign2970_e3004);
        let assign2970_e3006: f64 = (0.5 * assign2970_e3005);
        let assign2970_e3007: f64 = (assign2970_e3006).ln();
        let assign2970_e3008: f64 = (assign2970_e2992 * assign2970_e3007);
        let assign2970_e3009: f64 = (locals.var_vdj_t + assign2970_e3008);
        (assign2970_e3009, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign2970_e3007) + (assign2970_e2992 * ((0.5 * ((4.0 * (assign2970_e3001 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign2970_e2998 * locals.var_ovt_dn4)))) / (2.0 * assign2970_e3004))) / assign2970_e3006)))),)
    } else {
        (locals.var_vdci_t, locals.var_vdci_t_dn4,)
    }
};
        locals.var_vdci_t = assign2970_e3011;
        locals.var_vdci_t_dn4 = assign2970_e3011_d_n4;

        let (assign2980_e3025, assign2980_e3025_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard47 != 0.0)) {
        let assign2980_e3019: f64 = (p.p48 / locals.var_vdci_t);
        let assign2980_e3020: f64 = (assign2980_e3019).ln();
        let assign2980_e3021: f64 = (p.p49 * assign2980_e3020);
        let assign2980_e3022: f64 = (assign2980_e3021).exp();
        let assign2980_e3023: f64 = (p.p47 * assign2980_e3022);
        (assign2980_e3023, (p.p47 * (assign2980_e3022 * (p.p49 * ((-((p.p48 * locals.var_vdci_t_dn4) / (locals.var_vdci_t * locals.var_vdci_t))) / assign2980_e3019)))),)
    } else {
        (locals.var_cjci0_t, locals.var_cjci0_t_dn4,)
    }
};
        locals.var_cjci0_t = assign2980_e3025;
        locals.var_cjci0_t_dn4 = assign2980_e3025_d_n4;

        let (assign2990_e3032, assign2990_e3032_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard47 != 0.0)) {
        let assign2990_e3030: f64 = (p.p50).abs();
        (assign2990_e3030, 0.0,)
    } else {
        (locals.var_ajci_t, locals.var_ajci_t_dn4,)
    }
};
        locals.var_ajci_t = assign2990_e3032;
        locals.var_ajci_t_dn4 = assign2990_e3032_d_n4;

        let assign3000_e3035: f64 = if p.p50 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign3000_e3035;

        let (assign3010_e3047, assign3010_e3047_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard47 != 0.0)) && (locals.var_guard48 != 0.0)) {
        let assign3010_e3043: f64 = (p.p50 * locals.var_vdci_t);
        let assign3010_e3045: f64 = (assign3010_e3043 / p.p48);
        (assign3010_e3045, ((p.p50 * locals.var_vdci_t_dn4) / p.p48),)
    } else {
        (locals.var_ajci_t, locals.var_ajci_t_dn4,)
    }
};
        locals.var_ajci_t = assign3010_e3047;
        locals.var_ajci_t_dn4 = assign3010_e3047_d_n4;

        let (assign3020_e3054, assign3020_e3054_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard47 == 0.0)) {
        (p.p47, 0.0,)
    } else {
        (locals.var_cjci0_t, locals.var_cjci0_t_dn4,)
    }
};
        locals.var_cjci0_t = assign3020_e3054;
        locals.var_cjci0_t_dn4 = assign3020_e3054_d_n4;

        let (assign3030_e3061, assign3030_e3061_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard47 == 0.0)) {
        (p.p48, 0.0,)
    } else {
        (locals.var_vdci_t, locals.var_vdci_t_dn4,)
    }
};
        locals.var_vdci_t = assign3030_e3061;
        locals.var_vdci_t_dn4 = assign3030_e3061_d_n4;

        let (assign3040_e3068, assign3040_e3068_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard47 == 0.0)) {
        (p.p50, 0.0,)
    } else {
        (locals.var_ajci_t, locals.var_ajci_t_dn4,)
    }
};
        locals.var_ajci_t = assign3040_e3068;
        locals.var_ajci_t_dn4 = assign3040_e3068_d_n4;

        let assign3050_e3071: f64 = if p.p0 <= 300.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign3050_e3071;

        let (assign3060_e3077, assign3060_e3077_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard49 != 0.0)) {
        (2.4, 0.0,)
    } else {
        (locals.var_ajci_t, locals.var_ajci_t_dn4,)
    }
};
        locals.var_ajci_t = assign3060_e3077;
        locals.var_ajci_t_dn4 = assign3060_e3077_d_n4;

        let (assign3070_e3094, assign3070_e3094_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3070_e3082: f64 = (locals.var_zetabci * locals.var_ln_qtt0);
        let assign3070_e3085: f64 = (p.p119 * locals.var_ovtnom);
        let assign3070_e3088: f64 = (1.0 - locals.var_tn2td);
        let assign3070_e3089: f64 = (assign3070_e3085 * assign3070_e3088);
        let assign3070_e3090: f64 = (assign3070_e3082 + assign3070_e3089);
        let assign3070_e3091: f64 = (assign3070_e3090).exp();
        let assign3070_e3092: f64 = (p.p23 * assign3070_e3091);
        (assign3070_e3092, (p.p23 * (assign3070_e3091 * ((locals.var_zetabci * locals.var_ln_qtt0_dn4) + (assign3070_e3085 * (-locals.var_tn2td_dn4))))),)
    } else {
        (locals.var_ibcis_t, locals.var_ibcis_t_dn4,)
    }
};
        locals.var_ibcis_t = assign3070_e3094;
        locals.var_ibcis_t_dn4 = assign3070_e3094_d_n4;

        let (assign3080_e3108, assign3080_e3108_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3080_e3101: f64 = (locals.var_vdei_t / p.p40);
        let assign3080_e3102: f64 = (assign3080_e3101).ln();
        let assign3080_e3103: f64 = (p.p41 * assign3080_e3102);
        let assign3080_e3104: f64 = (assign3080_e3103).exp();
        let assign3080_e3105: f64 = (2.0 - assign3080_e3104);
        let assign3080_e3106: f64 = (p.p2 * assign3080_e3105);
        (assign3080_e3106, (p.p2 * (-(assign3080_e3104 * (p.p41 * ((locals.var_vdei_t_dn4 / p.p40) / assign3080_e3101))))),)
    } else {
        (locals.var_qp0_t, locals.var_qp0_t_dn4,)
    }
};
        locals.var_qp0_t = assign3080_e3108;
        locals.var_qp0_t_dn4 = assign3080_e3108_d_n4;

        let (assign3090_e3125, assign3090_e3125_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3090_e3113: f64 = (p.p123 * locals.var_ln_qtt0);
        let assign3090_e3116: f64 = (p.p117 * locals.var_ovtnom);
        let assign3090_e3119: f64 = (1.0 - locals.var_tn2td);
        let assign3090_e3120: f64 = (assign3090_e3116 * assign3090_e3119);
        let assign3090_e3121: f64 = (assign3090_e3113 + assign3090_e3120);
        let assign3090_e3122: f64 = (assign3090_e3121).exp();
        let assign3090_e3123: f64 = (p.p1 * assign3090_e3122);
        (assign3090_e3123, (p.p1 * (assign3090_e3122 * ((p.p123 * locals.var_ln_qtt0_dn4) + (assign3090_e3116 * (-locals.var_tn2td_dn4))))),)
    } else {
        (locals.var_c10_t, locals.var_c10_t_dn4,)
    }
};
        locals.var_c10_t = assign3090_e3125;
        locals.var_c10_t_dn4 = assign3090_e3125_d_n4;

        let (assign3100_e3134, assign3100_e3134_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3100_e3130: f64 = (p.p126 * locals.var_ln_qtt0);
        let assign3100_e3131: f64 = (assign3100_e3130).exp();
        let assign3100_e3132: f64 = (p.p10 * assign3100_e3131);
        (assign3100_e3132, (p.p10 * (assign3100_e3131 * (p.p126 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_ahjei_t, locals.var_ahjei_t_dn4,)
    }
};
        locals.var_ahjei_t = assign3100_e3134;
        locals.var_ahjei_t_dn4 = assign3100_e3134_d_n4;

        let assign3110_e3140: f64 = (p.p8 - 1.0);
        let assign3110_e3141: f64 = (assign3110_e3140).abs();
        let assign3110_e3144: f64 = if ((p.p0 <= 300.0) && (assign3110_e3141 < 1e-5)) { 1.0 } else { 0.0 };
        locals.var_guard50 = assign3110_e3144;

        let (assign3120_e3162, assign3120_e3162_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard50 != 0.0)) {
        let assign3120_e3151: f64 = (p.p125 * locals.var_ovt);
        let assign3120_e3154: f64 = (p.p127 * locals.var_ln_qtt0);
        let assign3120_e3155: f64 = (assign3120_e3154).exp();
        let assign3120_e3157: f64 = (assign3120_e3155 - 1.0);
        let assign3120_e3158: f64 = (assign3120_e3151 * assign3120_e3157);
        let assign3120_e3159: f64 = (assign3120_e3158).exp();
        let assign3120_e3160: f64 = (p.p9 * assign3120_e3159);
        (assign3120_e3160, (p.p9 * (assign3120_e3159 * (((p.p125 * locals.var_ovt_dn4) * assign3120_e3157) + (assign3120_e3151 * (assign3120_e3155 * (p.p127 * locals.var_ln_qtt0_dn4)))))),)
    } else {
        (locals.var_hjei0_t, locals.var_hjei0_t_dn4,)
    }
};
        locals.var_hjei0_t = assign3120_e3162;
        locals.var_hjei0_t_dn4 = assign3120_e3162_d_n4;

        let (assign3130_e3181, assign3130_e3181_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard50 == 0.0)) {
        let assign3130_e3170: f64 = (p.p125 * locals.var_ovt);
        let assign3130_e3173: f64 = (p.p127 * locals.var_ln_qtt0);
        let assign3130_e3174: f64 = (assign3130_e3173).exp();
        let assign3130_e3176: f64 = (assign3130_e3174 - 1.0);
        let assign3130_e3177: f64 = (assign3130_e3170 * assign3130_e3176);
        let assign3130_e3178: f64 = (assign3130_e3177).exp();
        let assign3130_e3179: f64 = (p.p8 * assign3130_e3178);
        (assign3130_e3179, (p.p8 * (assign3130_e3178 * (((p.p125 * locals.var_ovt_dn4) * assign3130_e3176) + (assign3130_e3170 * (assign3130_e3174 * (p.p127 * locals.var_ln_qtt0_dn4)))))),)
    } else {
        (locals.var_hjei0_t, locals.var_hjei0_t_dn4,)
    }
};
        locals.var_hjei0_t = assign3130_e3181;
        locals.var_hjei0_t_dn4 = assign3130_e3181_d_n4;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3140_e3194, assign3140_e3194_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3140_e3186: f64 = (p.p125 * locals.var_ovtnom);
        let assign3140_e3189: f64 = (1.0 - locals.var_tn2td);
        let assign3140_e3190: f64 = (assign3140_e3186 * assign3140_e3189);
        let assign3140_e3191: f64 = (assign3140_e3190).exp();
        let assign3140_e3192: f64 = (p.p3 * assign3140_e3191);
        (assign3140_e3192, (p.p3 * (assign3140_e3191 * (assign3140_e3186 * (-locals.var_tn2td_dn4)))),)
    } else {
        (locals.var_hf0_t, locals.var_hf0_t_dn4,)
    }
};
        locals.var_hf0_t = assign3140_e3194;
        locals.var_hf0_t_dn4 = assign3140_e3194_d_n4;

        let (assign3150_e3209, assign3150_e3209_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3150_e3199: f64 = (p.p117 - p.p118);
        let assign3150_e3201: f64 = (assign3150_e3199 * locals.var_ovtnom);
        let assign3150_e3204: f64 = (1.0 - locals.var_tn2td);
        let assign3150_e3205: f64 = (assign3150_e3201 * assign3150_e3204);
        let assign3150_e3206: f64 = (assign3150_e3205).exp();
        let assign3150_e3207: f64 = (p.p4 * assign3150_e3206);
        (assign3150_e3207, (p.p4 * (assign3150_e3206 * (assign3150_e3201 * (-locals.var_tn2td_dn4)))),)
    } else {
        (locals.var_hfe_t, locals.var_hfe_t_dn4,)
    }
};
        locals.var_hfe_t = assign3150_e3209;
        locals.var_hfe_t_dn4 = assign3150_e3209_d_n4;

        let (assign3160_e3224, assign3160_e3224_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3160_e3214: f64 = (p.p117 - p.p119);
        let assign3160_e3216: f64 = (assign3160_e3214 * locals.var_ovtnom);
        let assign3160_e3219: f64 = (1.0 - locals.var_tn2td);
        let assign3160_e3220: f64 = (assign3160_e3216 * assign3160_e3219);
        let assign3160_e3221: f64 = (assign3160_e3220).exp();
        let assign3160_e3222: f64 = (p.p6 * assign3160_e3221);
        (assign3160_e3222, (p.p6 * (assign3160_e3221 * (assign3160_e3216 * (-locals.var_tn2td_dn4)))),)
    } else {
        (locals.var_hfc_t, locals.var_hfc_t_dn4,)
    }
};
        locals.var_hfc_t = assign3160_e3224;
        locals.var_hfc_t_dn4 = assign3160_e3224_d_n4;

        let (assign3170_e3235, assign3170_e3235_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3170_e3229: f64 = (p.p130 - locals.var_avs);
        let assign3170_e3231: f64 = (assign3170_e3229 * locals.var_ln_qtt0);
        let assign3170_e3232: f64 = (assign3170_e3231).exp();
        let assign3170_e3233: f64 = (p.p75 * assign3170_e3232);
        (assign3170_e3233, (p.p75 * (assign3170_e3232 * (assign3170_e3229 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vlim_t, locals.var_vlim_t_dn4,)
    }
};
        locals.var_vlim_t = assign3170_e3235;
        locals.var_vlim_t_dn4 = assign3170_e3235_d_n4;

        let (assign3180_e3244, assign3180_e3244_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3180_e3240: f64 = (p.p130 * locals.var_ln_qtt0);
        let assign3180_e3241: f64 = (assign3180_e3240).exp();
        let assign3180_e3242: f64 = (p.p74 * assign3180_e3241);
        (assign3180_e3242, (p.p74 * (assign3180_e3241 * (p.p130 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_rci0_t, locals.var_rci0_t_dn4,)
    }
};
        locals.var_rci0_t = assign3180_e3244;
        locals.var_rci0_t_dn4 = assign3180_e3244_d_n4;

        let (assign3190_e3250, assign3190_e3250_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3190_e3248: f64 = (1.0 / locals.var_rci0_t);
        (assign3190_e3248, (-(locals.var_rci0_t_dn4 / (locals.var_rci0_t * locals.var_rci0_t))),)
    } else {
        (locals.var_orci0_t, locals.var_orci0_t_dn4,)
    }
};
        locals.var_orci0_t = assign3190_e3250;
        locals.var_orci0_t_dn4 = assign3190_e3250_d_n4;

        let assign3200_e3253: f64 = if p.p79 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign3200_e3253;

        let (assign3210_e3265, assign3210_e3265_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign3210_e3261: f64 = (p.p133 * locals.var_dtdev);
        let assign3210_e3262: f64 = (1.0 - assign3210_e3261);
        let assign3210_e3263: f64 = (p.p79 * assign3210_e3262);
        (assign3210_e3263, (p.p79 * (-(p.p133 * locals.var_dtdev_dn4))),)
    } else {
        (locals.var_vdck_t, locals.var_vdck_t_dn4,)
    }
};
        locals.var_vdck_t = assign3210_e3265;
        locals.var_vdck_t_dn4 = assign3210_e3265_d_n4;

        let (assign3220_e3271, assign3220_e3271_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard51 != 0.0)) {
        (p.p78, 0.0,)
    } else {
        (locals.var_vces_t, locals.var_vces_t_dn4,)
    }
};
        locals.var_vces_t = assign3220_e3271;
        locals.var_vces_t_dn4 = assign3220_e3271_d_n4;

        let (assign3230_e3284, assign3230_e3284_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard51 == 0.0)) {
        let assign3230_e3280: f64 = (p.p132 * locals.var_dtdev);
        let assign3230_e3281: f64 = (1.0 + assign3230_e3280);
        let assign3230_e3282: f64 = (p.p78 * assign3230_e3281);
        (assign3230_e3282, (p.p78 * (p.p132 * locals.var_dtdev_dn4)),)
    } else {
        (locals.var_vces_t, locals.var_vces_t_dn4,)
    }
};
        locals.var_vces_t = assign3230_e3284;
        locals.var_vces_t_dn4 = assign3230_e3284_d_n4;

        let (assign3240_e3291, assign3240_e3291_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard51 == 0.0)) {
        (p.p79, 0.0,)
    } else {
        (locals.var_vdck_t, locals.var_vdck_t_dn4,)
    }
};
        locals.var_vdck_t = assign3240_e3291;
        locals.var_vdck_t_dn4 = assign3240_e3291_d_n4;

        let (assign3250_e3307, assign3250_e3307_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3250_e3297: f64 = (p.p128 * locals.var_dtdev);
        let assign3250_e3298: f64 = (1.0 + assign3250_e3297);
        let assign3250_e3301: f64 = (p.p129 * locals.var_dtdev);
        let assign3250_e3303: f64 = (assign3250_e3301 * locals.var_dtdev);
        let assign3250_e3304: f64 = (assign3250_e3298 + assign3250_e3303);
        let assign3250_e3305: f64 = (p.p66 * assign3250_e3304);
        (assign3250_e3305, (p.p66 * ((p.p128 * locals.var_dtdev_dn4) + (((p.p129 * locals.var_dtdev_dn4) * locals.var_dtdev) + (assign3250_e3301 * locals.var_dtdev_dn4)))),)
    } else {
        (locals.var_t0_t, locals.var_t0_t_dn4,)
    }
};
        locals.var_t0_t = assign3250_e3307;
        locals.var_t0_t_dn4 = assign3250_e3307_d_n4;

        let (assign3260_e3311,) = {
    if (locals.var_guard42 != 0.0) {
        (p.p69,)
    } else {
        (locals.var_tef0_t,)
    }
};
        locals.var_tef0_t = assign3260_e3311;

        let (assign3270_e3322, assign3270_e3322_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3270_e3316: f64 = (p.p130 - 1.0);
        let assign3270_e3318: f64 = (assign3270_e3316 * locals.var_ln_qtt0);
        let assign3270_e3319: f64 = (assign3270_e3318).exp();
        let assign3270_e3320: f64 = (p.p71 * assign3270_e3319);
        (assign3270_e3320, (p.p71 * (assign3270_e3319 * (assign3270_e3316 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_thcs_t, locals.var_thcs_t_dn4,)
    }
};
        locals.var_thcs_t = assign3270_e3322;
        locals.var_thcs_t_dn4 = assign3270_e3322_d_n4;

        let assign3280_e3325: f64 = if locals.var_use_aval == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign3280_e3325;

        let (assign3290_e3336, assign3290_e3336_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard52 != 0.0)) {
        let assign3290_e3332: f64 = (p.p139 * locals.var_dtdev);
        let assign3290_e3333: f64 = (assign3290_e3332).exp();
        let assign3290_e3334: f64 = (p.p32 * assign3290_e3333);
        (assign3290_e3334, (p.p32 * (assign3290_e3333 * (p.p139 * locals.var_dtdev_dn4))),)
    } else {
        (locals.var_favl_t, locals.var_favl_t_dn4,)
    }
};
        locals.var_favl_t = assign3290_e3336;
        locals.var_favl_t_dn4 = assign3290_e3336_d_n4;

        let (assign3300_e3347, assign3300_e3347_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard52 != 0.0)) {
        let assign3300_e3343: f64 = (p.p140 * locals.var_dtdev);
        let assign3300_e3344: f64 = (assign3300_e3343).exp();
        let assign3300_e3345: f64 = (p.p33 * assign3300_e3344);
        (assign3300_e3345, (p.p33 * (assign3300_e3344 * (p.p140 * locals.var_dtdev_dn4))),)
    } else {
        (locals.var_qavl_t, locals.var_qavl_t_dn4,)
    }
};
        locals.var_qavl_t = assign3300_e3347;
        locals.var_qavl_t_dn4 = assign3300_e3347_d_n4;

        let (assign3310_e3354, assign3310_e3354_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard52 == 0.0)) {
        (p.p32, 0.0,)
    } else {
        (locals.var_favl_t, locals.var_favl_t_dn4,)
    }
};
        locals.var_favl_t = assign3310_e3354;
        locals.var_favl_t_dn4 = assign3310_e3354_d_n4;

        let (assign3320_e3361, assign3320_e3361_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard52 == 0.0)) {
        (p.p33, 0.0,)
    } else {
        (locals.var_qavl_t, locals.var_qavl_t_dn4,)
    }
};
        locals.var_qavl_t = assign3320_e3361;
        locals.var_qavl_t_dn4 = assign3320_e3361_d_n4;

        let assign3330_e3368: f64 = if ((p.p37 > 0.0) && (locals.var_vbici < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard53 = assign3330_e3368;

        let (assign3340_e3374, assign3340_e3374_d_n0, assign3340_e3374_d_n1, assign3340_e3374_d_n3, assign3340_e3374_d_n4, assign3340_e3374_d_n5, assign3340_e3374_d_n6, assign3340_e3374_d_n7, assign3340_e3374_d_n8, assign3340_e3374_d_n9,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard53 != 0.0)) {
        (p.p37, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibcts_t, locals.var_ibcts_t_dn0, locals.var_ibcts_t_dn1, locals.var_ibcts_t_dn3, locals.var_ibcts_t_dn4, locals.var_ibcts_t_dn5, locals.var_ibcts_t_dn6, locals.var_ibcts_t_dn7, locals.var_ibcts_t_dn8, locals.var_ibcts_t_dn9,)
    }
};
        locals.var_ibcts_t = assign3340_e3374;
        locals.var_ibcts_t_dn0 = assign3340_e3374_d_n0;
        locals.var_ibcts_t_dn1 = assign3340_e3374_d_n1;
        locals.var_ibcts_t_dn3 = assign3340_e3374_d_n3;
        locals.var_ibcts_t_dn4 = assign3340_e3374_d_n4;
        locals.var_ibcts_t_dn5 = assign3340_e3374_d_n5;
        locals.var_ibcts_t_dn6 = assign3340_e3374_d_n6;
        locals.var_ibcts_t_dn7 = assign3340_e3374_d_n7;
        locals.var_ibcts_t_dn8 = assign3340_e3374_d_n8;
        locals.var_ibcts_t_dn9 = assign3340_e3374_d_n9;

        let (assign3350_e3380, assign3350_e3380_d_n0, assign3350_e3380_d_n1, assign3350_e3380_d_n3, assign3350_e3380_d_n4, assign3350_e3380_d_n5, assign3350_e3380_d_n6, assign3350_e3380_d_n7, assign3350_e3380_d_n8, assign3350_e3380_d_n9,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard53 != 0.0)) {
        (p.p38, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_abct_t, locals.var_abct_t_dn0, locals.var_abct_t_dn1, locals.var_abct_t_dn3, locals.var_abct_t_dn4, locals.var_abct_t_dn5, locals.var_abct_t_dn6, locals.var_abct_t_dn7, locals.var_abct_t_dn8, locals.var_abct_t_dn9,)
    }
};
        locals.var_abct_t = assign3350_e3380;
        locals.var_abct_t_dn0 = assign3350_e3380_d_n0;
        locals.var_abct_t_dn1 = assign3350_e3380_d_n1;
        locals.var_abct_t_dn3 = assign3350_e3380_d_n3;
        locals.var_abct_t_dn4 = assign3350_e3380_d_n4;
        locals.var_abct_t_dn5 = assign3350_e3380_d_n5;
        locals.var_abct_t_dn6 = assign3350_e3380_d_n6;
        locals.var_abct_t_dn7 = assign3350_e3380_d_n7;
        locals.var_abct_t_dn8 = assign3350_e3380_d_n8;
        locals.var_abct_t_dn9 = assign3350_e3380_d_n9;

        let assign3360_e3387: f64 = if ((p.p47 > 0.0) && (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard54 = assign3360_e3387;

        let (assign3370_e3397, assign3370_e3397_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        let assign3370_e3395: f64 = (locals.var_vgbc_tnom / locals.var_vgbc_t);
        (assign3370_e3395, (-((locals.var_vgbc_tnom * locals.var_vgbc_t_dn4) / (locals.var_vgbc_t * locals.var_vgbc_t))),)
    } else {
        (locals.var_dum_e, locals.var_dum_e_dn4,)
    }
};
        locals.var_dum_e = assign3370_e3397;
        locals.var_dum_e_dn4 = assign3370_e3397_d_n4;

        let (assign3380_e3407, assign3380_e3407_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        let assign3380_e3405: f64 = (locals.var_vdci_t / p.p48);
        (assign3380_e3405, (locals.var_vdci_t_dn4 / p.p48),)
    } else {
        (locals.var_dum_v, locals.var_dum_v_dn4,)
    }
};
        locals.var_dum_v = assign3380_e3407;
        locals.var_dum_v_dn4 = assign3380_e3407_d_n4;

        let (assign3390_e3422, assign3390_e3422_d_n0, assign3390_e3422_d_n1, assign3390_e3422_d_n3, assign3390_e3422_d_n4, assign3390_e3422_d_n5, assign3390_e3422_d_n6, assign3390_e3422_d_n7, assign3390_e3422_d_n8, assign3390_e3422_d_n9,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        let assign3390_e3414: f64 = (locals.var_dum_e).sqrt();
        let assign3390_e3416: f64 = (assign3390_e3414 * locals.var_dum_v);
        let assign3390_e3418: f64 = (assign3390_e3416 * locals.var_cjci0_t);
        let assign3390_e3420: f64 = (assign3390_e3418 / p.p47);
        (assign3390_e3420, 0.0, 0.0, 0.0, ((((((locals.var_dum_e_dn4 / (2.0 * assign3390_e3414)) * locals.var_dum_v) + (assign3390_e3414 * locals.var_dum_v_dn4)) * locals.var_cjci0_t) + (assign3390_e3416 * locals.var_cjci0_t_dn4)) / p.p47), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_c, locals.var_dum_c_dn0, locals.var_dum_c_dn1, locals.var_dum_c_dn3, locals.var_dum_c_dn4, locals.var_dum_c_dn5, locals.var_dum_c_dn6, locals.var_dum_c_dn7, locals.var_dum_c_dn8, locals.var_dum_c_dn9,)
    }
};
        locals.var_dum_c = assign3390_e3422;
        locals.var_dum_c_dn0 = assign3390_e3422_d_n0;
        locals.var_dum_c_dn1 = assign3390_e3422_d_n1;
        locals.var_dum_c_dn3 = assign3390_e3422_d_n3;
        locals.var_dum_c_dn4 = assign3390_e3422_d_n4;
        locals.var_dum_c_dn5 = assign3390_e3422_d_n5;
        locals.var_dum_c_dn6 = assign3390_e3422_d_n6;
        locals.var_dum_c_dn7 = assign3390_e3422_d_n7;
        locals.var_dum_c_dn8 = assign3390_e3422_d_n8;
        locals.var_dum_c_dn9 = assign3390_e3422_d_n9;

        let (assign3400_e3434, assign3400_e3434_d_n0, assign3400_e3434_d_n1, assign3400_e3434_d_n3, assign3400_e3434_d_n4, assign3400_e3434_d_n5, assign3400_e3434_d_n6, assign3400_e3434_d_n7, assign3400_e3434_d_n8, assign3400_e3434_d_n9,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        let assign3400_e3430: f64 = (p.p37 * locals.var_dum_c);
        let assign3400_e3432: f64 = (assign3400_e3430 * locals.var_dum_v);
        (assign3400_e3432, ((p.p37 * locals.var_dum_c_dn0) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn1) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn3) * locals.var_dum_v), (((p.p37 * locals.var_dum_c_dn4) * locals.var_dum_v) + (assign3400_e3430 * locals.var_dum_v_dn4)), ((p.p37 * locals.var_dum_c_dn5) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn6) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn7) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn8) * locals.var_dum_v), ((p.p37 * locals.var_dum_c_dn9) * locals.var_dum_v),)
    } else {
        (locals.var_ibcts_t, locals.var_ibcts_t_dn0, locals.var_ibcts_t_dn1, locals.var_ibcts_t_dn3, locals.var_ibcts_t_dn4, locals.var_ibcts_t_dn5, locals.var_ibcts_t_dn6, locals.var_ibcts_t_dn7, locals.var_ibcts_t_dn8, locals.var_ibcts_t_dn9,)
    }
};
        locals.var_ibcts_t = assign3400_e3434;
        locals.var_ibcts_t_dn0 = assign3400_e3434_d_n0;
        locals.var_ibcts_t_dn1 = assign3400_e3434_d_n1;
        locals.var_ibcts_t_dn3 = assign3400_e3434_d_n3;
        locals.var_ibcts_t_dn4 = assign3400_e3434_d_n4;
        locals.var_ibcts_t_dn5 = assign3400_e3434_d_n5;
        locals.var_ibcts_t_dn6 = assign3400_e3434_d_n6;
        locals.var_ibcts_t_dn7 = assign3400_e3434_d_n7;
        locals.var_ibcts_t_dn8 = assign3400_e3434_d_n8;
        locals.var_ibcts_t_dn9 = assign3400_e3434_d_n9;

        let (assign3410_e3446, assign3410_e3446_d_n0, assign3410_e3446_d_n1, assign3410_e3446_d_n3, assign3410_e3446_d_n4, assign3410_e3446_d_n5, assign3410_e3446_d_n6, assign3410_e3446_d_n7, assign3410_e3446_d_n8, assign3410_e3446_d_n9,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        let assign3410_e3443: f64 = (locals.var_dum_c * locals.var_dum_e);
        let assign3410_e3444: f64 = (p.p38 / assign3410_e3443);
        (assign3410_e3444, (-((p.p38 * (locals.var_dum_c_dn0 * locals.var_dum_e)) / (assign3410_e3443 * assign3410_e3443))), (-((p.p38 * (locals.var_dum_c_dn1 * locals.var_dum_e)) / (assign3410_e3443 * assign3410_e3443))), (-((p.p38 * (locals.var_dum_c_dn3 * locals.var_dum_e)) / (assign3410_e3443 * assign3410_e3443))), (-((p.p38 * ((locals.var_dum_c_dn4 * locals.var_dum_e) + (locals.var_dum_c * locals.var_dum_e_dn4))) / (assign3410_e3443 * assign3410_e3443))), (-((p.p38 * (locals.var_dum_c_dn5 * locals.var_dum_e)) / (assign3410_e3443 * assign3410_e3443))), (-((p.p38 * (locals.var_dum_c_dn6 * locals.var_dum_e)) / (assign3410_e3443 * assign3410_e3443))), (-((p.p38 * (locals.var_dum_c_dn7 * locals.var_dum_e)) / (assign3410_e3443 * assign3410_e3443))), (-((p.p38 * (locals.var_dum_c_dn8 * locals.var_dum_e)) / (assign3410_e3443 * assign3410_e3443))), (-((p.p38 * (locals.var_dum_c_dn9 * locals.var_dum_e)) / (assign3410_e3443 * assign3410_e3443))),)
    } else {
        (locals.var_abct_t, locals.var_abct_t_dn0, locals.var_abct_t_dn1, locals.var_abct_t_dn3, locals.var_abct_t_dn4, locals.var_abct_t_dn5, locals.var_abct_t_dn6, locals.var_abct_t_dn7, locals.var_abct_t_dn8, locals.var_abct_t_dn9,)
    }
};
        locals.var_abct_t = assign3410_e3446;
        locals.var_abct_t_dn0 = assign3410_e3446_d_n0;
        locals.var_abct_t_dn1 = assign3410_e3446_d_n1;
        locals.var_abct_t_dn3 = assign3410_e3446_d_n3;
        locals.var_abct_t_dn4 = assign3410_e3446_d_n4;
        locals.var_abct_t_dn5 = assign3410_e3446_d_n5;
        locals.var_abct_t_dn6 = assign3410_e3446_d_n6;
        locals.var_abct_t_dn7 = assign3410_e3446_d_n7;
        locals.var_abct_t_dn8 = assign3410_e3446_d_n8;
        locals.var_abct_t_dn9 = assign3410_e3446_d_n9;

        let (assign3420_e3453, assign3420_e3453_d_n0, assign3420_e3453_d_n1, assign3420_e3453_d_n3, assign3420_e3453_d_n4, assign3420_e3453_d_n5, assign3420_e3453_d_n6, assign3420_e3453_d_n7, assign3420_e3453_d_n8, assign3420_e3453_d_n9,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard53 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibcts_t, locals.var_ibcts_t_dn0, locals.var_ibcts_t_dn1, locals.var_ibcts_t_dn3, locals.var_ibcts_t_dn4, locals.var_ibcts_t_dn5, locals.var_ibcts_t_dn6, locals.var_ibcts_t_dn7, locals.var_ibcts_t_dn8, locals.var_ibcts_t_dn9,)
    }
};
        locals.var_ibcts_t = assign3420_e3453;
        locals.var_ibcts_t_dn0 = assign3420_e3453_d_n0;
        locals.var_ibcts_t_dn1 = assign3420_e3453_d_n1;
        locals.var_ibcts_t_dn3 = assign3420_e3453_d_n3;
        locals.var_ibcts_t_dn4 = assign3420_e3453_d_n4;
        locals.var_ibcts_t_dn5 = assign3420_e3453_d_n5;
        locals.var_ibcts_t_dn6 = assign3420_e3453_d_n6;
        locals.var_ibcts_t_dn7 = assign3420_e3453_d_n7;
        locals.var_ibcts_t_dn8 = assign3420_e3453_d_n8;
        locals.var_ibcts_t_dn9 = assign3420_e3453_d_n9;

        let (assign3430_e3460, assign3430_e3460_d_n0, assign3430_e3460_d_n1, assign3430_e3460_d_n3, assign3430_e3460_d_n4, assign3430_e3460_d_n5, assign3430_e3460_d_n6, assign3430_e3460_d_n7, assign3430_e3460_d_n8, assign3430_e3460_d_n9,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard53 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_abct_t, locals.var_abct_t_dn0, locals.var_abct_t_dn1, locals.var_abct_t_dn3, locals.var_abct_t_dn4, locals.var_abct_t_dn5, locals.var_abct_t_dn6, locals.var_abct_t_dn7, locals.var_abct_t_dn8, locals.var_abct_t_dn9,)
    }
};
        locals.var_abct_t = assign3430_e3460;
        locals.var_abct_t_dn0 = assign3430_e3460_d_n0;
        locals.var_abct_t_dn1 = assign3430_e3460_d_n1;
        locals.var_abct_t_dn3 = assign3430_e3460_d_n3;
        locals.var_abct_t_dn4 = assign3430_e3460_d_n4;
        locals.var_abct_t_dn5 = assign3430_e3460_d_n5;
        locals.var_abct_t_dn6 = assign3430_e3460_d_n6;
        locals.var_abct_t_dn7 = assign3430_e3460_d_n7;
        locals.var_abct_t_dn8 = assign3430_e3460_d_n8;
        locals.var_abct_t_dn9 = assign3430_e3460_d_n9;

        let (assign3440_e3469, assign3440_e3469_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3440_e3465: f64 = (p.p134 * locals.var_ln_qtt0);
        let assign3440_e3466: f64 = (assign3440_e3465).exp();
        let assign3440_e3467: f64 = (p.p89 * assign3440_e3466);
        (assign3440_e3467, (p.p89 * (assign3440_e3466 * (p.p134 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_rbi0_t, locals.var_rbi0_t_dn4,)
    }
};
        locals.var_rbi0_t = assign3440_e3469;
        locals.var_rbi0_t_dn4 = assign3440_e3469_d_n4;

        let assign3450_e3472: f64 = if p.p43 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign3450_e3472;

        let (assign3460_e3496,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard55 != 0.0)) {
        let assign3460_e3478: f64 = (2.0 * locals.var_vtnom);
        let assign3460_e3481: f64 = (p.p44 * 0.5);
        let assign3460_e3483: f64 = (assign3460_e3481 * locals.var_ovtnom);
        let assign3460_e3484: f64 = (assign3460_e3483).exp();
        let assign3460_e3486: f64 = (-0.5);
        let assign3460_e3488: f64 = (assign3460_e3486 * p.p44);
        let assign3460_e3490: f64 = (assign3460_e3488 * locals.var_ovtnom);
        let assign3460_e3491: f64 = (assign3460_e3490).exp();
        let assign3460_e3492: f64 = (assign3460_e3484 - assign3460_e3491);
        let assign3460_e3493: f64 = (assign3460_e3492).ln();
        let assign3460_e3494: f64 = (assign3460_e3478 * assign3460_e3493);
        (assign3460_e3494,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign3460_e3496;

        let (assign3470_e3516, assign3470_e3516_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard55 != 0.0)) {
        let assign3470_e3502: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign3470_e3506: f64 = (1.0 - locals.var_qtt0);
        let assign3470_e3507: f64 = (locals.var_vgbe0 * assign3470_e3506);
        let assign3470_e3508: f64 = (assign3470_e3502 + assign3470_e3507);
        let assign3470_e3511: f64 = (locals.var_mg * locals.var_vt);
        let assign3470_e3513: f64 = (assign3470_e3511 * locals.var_ln_qtt0);
        let assign3470_e3514: f64 = (assign3470_e3508 - assign3470_e3513);
        (assign3470_e3514, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgbe0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign3470_e3511 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign3470_e3516;
        locals.var_vdj_t_dn4 = assign3470_e3516_d_n4;

        let (assign3480_e3542, assign3480_e3542_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard55 != 0.0)) {
        let assign3480_e3523: f64 = (2.0 * locals.var_vt);
        let assign3480_e3529: f64 = (-locals.var_vdj_t);
        let assign3480_e3531: f64 = (assign3480_e3529 * locals.var_ovt);
        let assign3480_e3532: f64 = (assign3480_e3531).exp();
        let assign3480_e3533: f64 = (4.0 * assign3480_e3532);
        let assign3480_e3534: f64 = (1.0 + assign3480_e3533);
        let assign3480_e3535: f64 = (assign3480_e3534).sqrt();
        let assign3480_e3536: f64 = (1.0 + assign3480_e3535);
        let assign3480_e3537: f64 = (0.5 * assign3480_e3536);
        let assign3480_e3538: f64 = (assign3480_e3537).ln();
        let assign3480_e3539: f64 = (assign3480_e3523 * assign3480_e3538);
        let assign3480_e3540: f64 = (locals.var_vdj_t + assign3480_e3539);
        (assign3480_e3540, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign3480_e3538) + (assign3480_e3523 * ((0.5 * ((4.0 * (assign3480_e3532 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign3480_e3529 * locals.var_ovt_dn4)))) / (2.0 * assign3480_e3535))) / assign3480_e3537)))),)
    } else {
        (locals.var_vdep_t, locals.var_vdep_t_dn4,)
    }
};
        locals.var_vdep_t = assign3480_e3542;
        locals.var_vdep_t_dn4 = assign3480_e3542_d_n4;

        let (assign3490_e3556, assign3490_e3556_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard55 != 0.0)) {
        let assign3490_e3550: f64 = (p.p44 / locals.var_vdep_t);
        let assign3490_e3551: f64 = (assign3490_e3550).ln();
        let assign3490_e3552: f64 = (p.p45 * assign3490_e3551);
        let assign3490_e3553: f64 = (assign3490_e3552).exp();
        let assign3490_e3554: f64 = (p.p43 * assign3490_e3553);
        (assign3490_e3554, (p.p43 * (assign3490_e3553 * (p.p45 * ((-((p.p44 * locals.var_vdep_t_dn4) / (locals.var_vdep_t * locals.var_vdep_t))) / assign3490_e3550)))),)
    } else {
        (locals.var_cjep0_t, locals.var_cjep0_t_dn4,)
    }
};
        locals.var_cjep0_t = assign3490_e3556;
        locals.var_cjep0_t_dn4 = assign3490_e3556_d_n4;

        let (assign3500_e3563, assign3500_e3563_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard55 != 0.0)) {
        let assign3500_e3561: f64 = (p.p46).abs();
        (assign3500_e3561, 0.0,)
    } else {
        (locals.var_ajep_t, locals.var_ajep_t_dn4,)
    }
};
        locals.var_ajep_t = assign3500_e3563;
        locals.var_ajep_t_dn4 = assign3500_e3563_d_n4;

        let assign3510_e3566: f64 = if p.p46 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard56 = assign3510_e3566;

        let (assign3520_e3578, assign3520_e3578_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard55 != 0.0)) && (locals.var_guard56 != 0.0)) {
        let assign3520_e3574: f64 = (p.p46 * locals.var_vdep_t);
        let assign3520_e3576: f64 = (assign3520_e3574 / p.p44);
        (assign3520_e3576, ((p.p46 * locals.var_vdep_t_dn4) / p.p44),)
    } else {
        (locals.var_ajep_t, locals.var_ajep_t_dn4,)
    }
};
        locals.var_ajep_t = assign3520_e3578;
        locals.var_ajep_t_dn4 = assign3520_e3578_d_n4;

        let (assign3530_e3585, assign3530_e3585_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard55 == 0.0)) {
        (p.p43, 0.0,)
    } else {
        (locals.var_cjep0_t, locals.var_cjep0_t_dn4,)
    }
};
        locals.var_cjep0_t = assign3530_e3585;
        locals.var_cjep0_t_dn4 = assign3530_e3585_d_n4;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3540_e3592, assign3540_e3592_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard55 == 0.0)) {
        (p.p44, 0.0,)
    } else {
        (locals.var_vdep_t, locals.var_vdep_t_dn4,)
    }
};
        locals.var_vdep_t = assign3540_e3592;
        locals.var_vdep_t_dn4 = assign3540_e3592_d_n4;

        let (assign3550_e3599, assign3550_e3599_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard55 == 0.0)) {
        (p.p46, 0.0,)
    } else {
        (locals.var_ajep_t, locals.var_ajep_t_dn4,)
    }
};
        locals.var_ajep_t = assign3550_e3599;
        locals.var_ajep_t_dn4 = assign3550_e3599_d_n4;

        let (assign3560_e3616, assign3560_e3616_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3560_e3604: f64 = (p.p124 * locals.var_ln_qtt0);
        let assign3560_e3607: f64 = (p.p118 * locals.var_ovtnom);
        let assign3560_e3610: f64 = (1.0 - locals.var_tn2td);
        let assign3560_e3611: f64 = (assign3560_e3607 * assign3560_e3610);
        let assign3560_e3612: f64 = (assign3560_e3604 + assign3560_e3611);
        let assign3560_e3613: f64 = (assign3560_e3612).exp();
        let assign3560_e3614: f64 = (p.p18 * assign3560_e3613);
        (assign3560_e3614, (p.p18 * (assign3560_e3613 * ((p.p124 * locals.var_ln_qtt0_dn4) + (assign3560_e3607 * (-locals.var_tn2td_dn4))))),)
    } else {
        (locals.var_ibeps_t, locals.var_ibeps_t_dn4,)
    }
};
        locals.var_ibeps_t = assign3560_e3616;
        locals.var_ibeps_t_dn4 = assign3560_e3616_d_n4;

        let (assign3570_e3637, assign3570_e3637_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3570_e3621: f64 = (locals.var_mg / p.p21);
        let assign3570_e3623: f64 = (assign3570_e3621 * locals.var_ln_qtt0);
        let assign3570_e3626: f64 = (locals.var_vgbe0 * locals.var_ovtnom);
        let assign3570_e3629: f64 = (1.0 - locals.var_tn2td);
        let assign3570_e3630: f64 = (assign3570_e3626 * assign3570_e3629);
        let assign3570_e3632: f64 = (assign3570_e3630 / p.p21);
        let assign3570_e3633: f64 = (assign3570_e3623 + assign3570_e3632);
        let assign3570_e3634: f64 = (assign3570_e3633).exp();
        let assign3570_e3635: f64 = (p.p20 * assign3570_e3634);
        (assign3570_e3635, (p.p20 * (assign3570_e3634 * ((assign3570_e3621 * locals.var_ln_qtt0_dn4) + ((assign3570_e3626 * (-locals.var_tn2td_dn4)) / p.p21)))),)
    } else {
        (locals.var_ireps_t, locals.var_ireps_t_dn4,)
    }
};
        locals.var_ireps_t = assign3570_e3637;
        locals.var_ireps_t_dn4 = assign3570_e3637_d_n4;

        let assign3580_e3648: f64 = if ((p.p27 > 0.0) && ((locals.var_vbpei < locals.var_v_btbmax) || (locals.var_vbiei < locals.var_v_btbmax))) { 1.0 } else { 0.0 };
        locals.var_guard57 = assign3580_e3648;

        let (assign3590_e3654, assign3590_e3654_d_n0, assign3590_e3654_d_n1, assign3590_e3654_d_n3, assign3590_e3654_d_n4, assign3590_e3654_d_n5, assign3590_e3654_d_n6, assign3590_e3654_d_n7, assign3590_e3654_d_n8, assign3590_e3654_d_n9,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard57 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_a, locals.var_dum_a_dn0, locals.var_dum_a_dn1, locals.var_dum_a_dn3, locals.var_dum_a_dn4, locals.var_dum_a_dn5, locals.var_dum_a_dn6, locals.var_dum_a_dn7, locals.var_dum_a_dn8, locals.var_dum_a_dn9,)
    }
};
        locals.var_dum_a = assign3590_e3654;
        locals.var_dum_a_dn0 = assign3590_e3654_d_n0;
        locals.var_dum_a_dn1 = assign3590_e3654_d_n1;
        locals.var_dum_a_dn3 = assign3590_e3654_d_n3;
        locals.var_dum_a_dn4 = assign3590_e3654_d_n4;
        locals.var_dum_a_dn5 = assign3590_e3654_d_n5;
        locals.var_dum_a_dn6 = assign3590_e3654_d_n6;
        locals.var_dum_a_dn7 = assign3590_e3654_d_n7;
        locals.var_dum_a_dn8 = assign3590_e3654_d_n8;
        locals.var_dum_a_dn9 = assign3590_e3654_d_n9;

        let (assign3600_e3660, assign3600_e3660_d_n0, assign3600_e3660_d_n1, assign3600_e3660_d_n3, assign3600_e3660_d_n4, assign3600_e3660_d_n5, assign3600_e3660_d_n6, assign3600_e3660_d_n7, assign3600_e3660_d_n8, assign3600_e3660_d_n9,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard57 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_b, locals.var_dum_b_dn0, locals.var_dum_b_dn1, locals.var_dum_b_dn3, locals.var_dum_b_dn4, locals.var_dum_b_dn5, locals.var_dum_b_dn6, locals.var_dum_b_dn7, locals.var_dum_b_dn8, locals.var_dum_b_dn9,)
    }
};
        locals.var_dum_b = assign3600_e3660;
        locals.var_dum_b_dn0 = assign3600_e3660_d_n0;
        locals.var_dum_b_dn1 = assign3600_e3660_d_n1;
        locals.var_dum_b_dn3 = assign3600_e3660_d_n3;
        locals.var_dum_b_dn4 = assign3600_e3660_d_n4;
        locals.var_dum_b_dn5 = assign3600_e3660_d_n5;
        locals.var_dum_b_dn6 = assign3600_e3660_d_n6;
        locals.var_dum_b_dn7 = assign3600_e3660_d_n7;
        locals.var_dum_b_dn8 = assign3600_e3660_d_n8;
        locals.var_dum_b_dn9 = assign3600_e3660_d_n9;

        let (assign3610_e3668, assign3610_e3668_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard57 != 0.0)) {
        let assign3610_e3666: f64 = (locals.var_vgbe_tnom / locals.var_vgbe_t);
        (assign3610_e3666, (-((locals.var_vgbe_tnom * locals.var_vgbe_t_dn4) / (locals.var_vgbe_t * locals.var_vgbe_t))),)
    } else {
        (locals.var_dum_e, locals.var_dum_e_dn4,)
    }
};
        locals.var_dum_e = assign3610_e3668;
        locals.var_dum_e_dn4 = assign3610_e3668_d_n4;

        let assign3620_e3679: f64 = if (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard58 = assign3620_e3679;

        let (assign3630_e3689, assign3630_e3689_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard57 != 0.0)) && (locals.var_guard58 != 0.0)) {
        let assign3630_e3687: f64 = (locals.var_vdep_t / p.p44);
        (assign3630_e3687, (locals.var_vdep_t_dn4 / p.p44),)
    } else {
        (locals.var_dum_v, locals.var_dum_v_dn4,)
    }
};
        locals.var_dum_v = assign3630_e3689;
        locals.var_dum_v_dn4 = assign3630_e3689_d_n4;

        let (assign3640_e3706, assign3640_e3706_d_n0, assign3640_e3706_d_n1, assign3640_e3706_d_n3, assign3640_e3706_d_n4, assign3640_e3706_d_n5, assign3640_e3706_d_n6, assign3640_e3706_d_n7, assign3640_e3706_d_n8, assign3640_e3706_d_n9,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard57 != 0.0)) && (locals.var_guard58 != 0.0)) {
        let assign3640_e3697: f64 = (locals.var_cjep0_t / p.p43);
        let assign3640_e3699: f64 = (locals.var_dum_e).sqrt();
        let assign3640_e3700: f64 = (assign3640_e3697 * assign3640_e3699);
        let assign3640_e3702: f64 = (assign3640_e3700 * locals.var_dum_v);
        let assign3640_e3704: f64 = (assign3640_e3702 * locals.var_dum_v);
        (assign3640_e3704, 0.0, 0.0, 0.0, (((((((locals.var_cjep0_t_dn4 / p.p43) * assign3640_e3699) + (assign3640_e3697 * (locals.var_dum_e_dn4 / (2.0 * assign3640_e3699)))) * locals.var_dum_v) + (assign3640_e3700 * locals.var_dum_v_dn4)) * locals.var_dum_v) + (assign3640_e3702 * locals.var_dum_v_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_b, locals.var_dum_b_dn0, locals.var_dum_b_dn1, locals.var_dum_b_dn3, locals.var_dum_b_dn4, locals.var_dum_b_dn5, locals.var_dum_b_dn6, locals.var_dum_b_dn7, locals.var_dum_b_dn8, locals.var_dum_b_dn9,)
    }
};
        locals.var_dum_b = assign3640_e3706;
        locals.var_dum_b_dn0 = assign3640_e3706_d_n0;
        locals.var_dum_b_dn1 = assign3640_e3706_d_n1;
        locals.var_dum_b_dn3 = assign3640_e3706_d_n3;
        locals.var_dum_b_dn4 = assign3640_e3706_d_n4;
        locals.var_dum_b_dn5 = assign3640_e3706_d_n5;
        locals.var_dum_b_dn6 = assign3640_e3706_d_n6;
        locals.var_dum_b_dn7 = assign3640_e3706_d_n7;
        locals.var_dum_b_dn8 = assign3640_e3706_d_n8;
        locals.var_dum_b_dn9 = assign3640_e3706_d_n9;

        let (assign3650_e3723, assign3650_e3723_d_n0, assign3650_e3723_d_n1, assign3650_e3723_d_n3, assign3650_e3723_d_n4, assign3650_e3723_d_n5, assign3650_e3723_d_n6, assign3650_e3723_d_n7, assign3650_e3723_d_n8, assign3650_e3723_d_n9,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard57 != 0.0)) && (locals.var_guard58 != 0.0)) {
        let assign3650_e3714: f64 = (p.p43 / locals.var_cjep0_t);
        let assign3650_e3717: f64 = (-1.5);
        let assign3650_e3718: f64 = (locals.var_dum_e).powf(assign3650_e3717);
        let assign3650_e3719: f64 = (assign3650_e3714 * assign3650_e3718);
        let assign3650_e3721: f64 = (assign3650_e3719 / locals.var_dum_v);
        (assign3650_e3721, 0.0, 0.0, 0.0, ((((((-((p.p43 * locals.var_cjep0_t_dn4) / (locals.var_cjep0_t * locals.var_cjep0_t))) * assign3650_e3718) + (assign3650_e3714 * if 0.0 == 0.0 && ((assign3650_e3717) as f64).is_finite() && ((assign3650_e3717) as f64).fract() == 0.0 { if assign3650_e3717 == 0.0 { 0.0 } else { (assign3650_e3717 * ((locals.var_dum_e).powf(assign3650_e3717 - 1.0) * locals.var_dum_e_dn4)) } } else { (assign3650_e3718 * (assign3650_e3717 * (locals.var_dum_e_dn4 / locals.var_dum_e))) })) * locals.var_dum_v) - (assign3650_e3719 * locals.var_dum_v_dn4)) / (locals.var_dum_v * locals.var_dum_v)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_a, locals.var_dum_a_dn0, locals.var_dum_a_dn1, locals.var_dum_a_dn3, locals.var_dum_a_dn4, locals.var_dum_a_dn5, locals.var_dum_a_dn6, locals.var_dum_a_dn7, locals.var_dum_a_dn8, locals.var_dum_a_dn9,)
    }
};
        locals.var_dum_a = assign3650_e3723;
        locals.var_dum_a_dn0 = assign3650_e3723_d_n0;
        locals.var_dum_a_dn1 = assign3650_e3723_d_n1;
        locals.var_dum_a_dn3 = assign3650_e3723_d_n3;
        locals.var_dum_a_dn4 = assign3650_e3723_d_n4;
        locals.var_dum_a_dn5 = assign3650_e3723_d_n5;
        locals.var_dum_a_dn6 = assign3650_e3723_d_n6;
        locals.var_dum_a_dn7 = assign3650_e3723_d_n7;
        locals.var_dum_a_dn8 = assign3650_e3723_d_n8;
        locals.var_dum_a_dn9 = assign3650_e3723_d_n9;

        let assign3660_e3734: f64 = if (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard59 = assign3660_e3734;

        let (assign3670_e3747, assign3670_e3747_d_n4,) = {
    if ((((locals.var_guard42 != 0.0) && (locals.var_guard57 != 0.0)) && (locals.var_guard58 == 0.0)) && (locals.var_guard59 != 0.0)) {
        let assign3670_e3745: f64 = (locals.var_vdei_t / p.p40);
        (assign3670_e3745, (locals.var_vdei_t_dn4 / p.p40),)
    } else {
        (locals.var_dum_v, locals.var_dum_v_dn4,)
    }
};
        locals.var_dum_v = assign3670_e3747;
        locals.var_dum_v_dn4 = assign3670_e3747_d_n4;

        let (assign3680_e3767, assign3680_e3767_d_n0, assign3680_e3767_d_n1, assign3680_e3767_d_n3, assign3680_e3767_d_n4, assign3680_e3767_d_n5, assign3680_e3767_d_n6, assign3680_e3767_d_n7, assign3680_e3767_d_n8, assign3680_e3767_d_n9,) = {
    if ((((locals.var_guard42 != 0.0) && (locals.var_guard57 != 0.0)) && (locals.var_guard58 == 0.0)) && (locals.var_guard59 != 0.0)) {
        let assign3680_e3758: f64 = (locals.var_cjei0_t / p.p39);
        let assign3680_e3760: f64 = (locals.var_dum_e).sqrt();
        let assign3680_e3761: f64 = (assign3680_e3758 * assign3680_e3760);
        let assign3680_e3763: f64 = (assign3680_e3761 * locals.var_dum_v);
        let assign3680_e3765: f64 = (assign3680_e3763 * locals.var_dum_v);
        (assign3680_e3765, 0.0, 0.0, 0.0, (((((((locals.var_cjei0_t_dn4 / p.p39) * assign3680_e3760) + (assign3680_e3758 * (locals.var_dum_e_dn4 / (2.0 * assign3680_e3760)))) * locals.var_dum_v) + (assign3680_e3761 * locals.var_dum_v_dn4)) * locals.var_dum_v) + (assign3680_e3763 * locals.var_dum_v_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_b, locals.var_dum_b_dn0, locals.var_dum_b_dn1, locals.var_dum_b_dn3, locals.var_dum_b_dn4, locals.var_dum_b_dn5, locals.var_dum_b_dn6, locals.var_dum_b_dn7, locals.var_dum_b_dn8, locals.var_dum_b_dn9,)
    }
};
        locals.var_dum_b = assign3680_e3767;
        locals.var_dum_b_dn0 = assign3680_e3767_d_n0;
        locals.var_dum_b_dn1 = assign3680_e3767_d_n1;
        locals.var_dum_b_dn3 = assign3680_e3767_d_n3;
        locals.var_dum_b_dn4 = assign3680_e3767_d_n4;
        locals.var_dum_b_dn5 = assign3680_e3767_d_n5;
        locals.var_dum_b_dn6 = assign3680_e3767_d_n6;
        locals.var_dum_b_dn7 = assign3680_e3767_d_n7;
        locals.var_dum_b_dn8 = assign3680_e3767_d_n8;
        locals.var_dum_b_dn9 = assign3680_e3767_d_n9;

        let (assign3690_e3787, assign3690_e3787_d_n0, assign3690_e3787_d_n1, assign3690_e3787_d_n3, assign3690_e3787_d_n4, assign3690_e3787_d_n5, assign3690_e3787_d_n6, assign3690_e3787_d_n7, assign3690_e3787_d_n8, assign3690_e3787_d_n9,) = {
    if ((((locals.var_guard42 != 0.0) && (locals.var_guard57 != 0.0)) && (locals.var_guard58 == 0.0)) && (locals.var_guard59 != 0.0)) {
        let assign3690_e3778: f64 = (p.p39 / locals.var_cjei0_t);
        let assign3690_e3781: f64 = (-1.5);
        let assign3690_e3782: f64 = (locals.var_dum_e).powf(assign3690_e3781);
        let assign3690_e3783: f64 = (assign3690_e3778 * assign3690_e3782);
        let assign3690_e3785: f64 = (assign3690_e3783 / locals.var_dum_v);
        (assign3690_e3785, 0.0, 0.0, 0.0, ((((((-((p.p39 * locals.var_cjei0_t_dn4) / (locals.var_cjei0_t * locals.var_cjei0_t))) * assign3690_e3782) + (assign3690_e3778 * if 0.0 == 0.0 && ((assign3690_e3781) as f64).is_finite() && ((assign3690_e3781) as f64).fract() == 0.0 { if assign3690_e3781 == 0.0 { 0.0 } else { (assign3690_e3781 * ((locals.var_dum_e).powf(assign3690_e3781 - 1.0) * locals.var_dum_e_dn4)) } } else { (assign3690_e3782 * (assign3690_e3781 * (locals.var_dum_e_dn4 / locals.var_dum_e))) })) * locals.var_dum_v) - (assign3690_e3783 * locals.var_dum_v_dn4)) / (locals.var_dum_v * locals.var_dum_v)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dum_a, locals.var_dum_a_dn0, locals.var_dum_a_dn1, locals.var_dum_a_dn3, locals.var_dum_a_dn4, locals.var_dum_a_dn5, locals.var_dum_a_dn6, locals.var_dum_a_dn7, locals.var_dum_a_dn8, locals.var_dum_a_dn9,)
    }
};
        locals.var_dum_a = assign3690_e3787;
        locals.var_dum_a_dn0 = assign3690_e3787_d_n0;
        locals.var_dum_a_dn1 = assign3690_e3787_d_n1;
        locals.var_dum_a_dn3 = assign3690_e3787_d_n3;
        locals.var_dum_a_dn4 = assign3690_e3787_d_n4;
        locals.var_dum_a_dn5 = assign3690_e3787_d_n5;
        locals.var_dum_a_dn6 = assign3690_e3787_d_n6;
        locals.var_dum_a_dn7 = assign3690_e3787_d_n7;
        locals.var_dum_a_dn8 = assign3690_e3787_d_n8;
        locals.var_dum_a_dn9 = assign3690_e3787_d_n9;

        let (assign3700_e3795, assign3700_e3795_d_n0, assign3700_e3795_d_n1, assign3700_e3795_d_n3, assign3700_e3795_d_n4, assign3700_e3795_d_n5, assign3700_e3795_d_n6, assign3700_e3795_d_n7, assign3700_e3795_d_n8, assign3700_e3795_d_n9,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard57 != 0.0)) {
        let assign3700_e3793: f64 = (p.p27 * locals.var_dum_b);
        (assign3700_e3793, (p.p27 * locals.var_dum_b_dn0), (p.p27 * locals.var_dum_b_dn1), (p.p27 * locals.var_dum_b_dn3), (p.p27 * locals.var_dum_b_dn4), (p.p27 * locals.var_dum_b_dn5), (p.p27 * locals.var_dum_b_dn6), (p.p27 * locals.var_dum_b_dn7), (p.p27 * locals.var_dum_b_dn8), (p.p27 * locals.var_dum_b_dn9),)
    } else {
        (locals.var_ibets_t, locals.var_ibets_t_dn0, locals.var_ibets_t_dn1, locals.var_ibets_t_dn3, locals.var_ibets_t_dn4, locals.var_ibets_t_dn5, locals.var_ibets_t_dn6, locals.var_ibets_t_dn7, locals.var_ibets_t_dn8, locals.var_ibets_t_dn9,)
    }
};
        locals.var_ibets_t = assign3700_e3795;
        locals.var_ibets_t_dn0 = assign3700_e3795_d_n0;
        locals.var_ibets_t_dn1 = assign3700_e3795_d_n1;
        locals.var_ibets_t_dn3 = assign3700_e3795_d_n3;
        locals.var_ibets_t_dn4 = assign3700_e3795_d_n4;
        locals.var_ibets_t_dn5 = assign3700_e3795_d_n5;
        locals.var_ibets_t_dn6 = assign3700_e3795_d_n6;
        locals.var_ibets_t_dn7 = assign3700_e3795_d_n7;
        locals.var_ibets_t_dn8 = assign3700_e3795_d_n8;
        locals.var_ibets_t_dn9 = assign3700_e3795_d_n9;

        let (assign3710_e3803, assign3710_e3803_d_n0, assign3710_e3803_d_n1, assign3710_e3803_d_n3, assign3710_e3803_d_n4, assign3710_e3803_d_n5, assign3710_e3803_d_n6, assign3710_e3803_d_n7, assign3710_e3803_d_n8, assign3710_e3803_d_n9,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard57 != 0.0)) {
        let assign3710_e3801: f64 = (p.p28 * locals.var_dum_a);
        (assign3710_e3801, (p.p28 * locals.var_dum_a_dn0), (p.p28 * locals.var_dum_a_dn1), (p.p28 * locals.var_dum_a_dn3), (p.p28 * locals.var_dum_a_dn4), (p.p28 * locals.var_dum_a_dn5), (p.p28 * locals.var_dum_a_dn6), (p.p28 * locals.var_dum_a_dn7), (p.p28 * locals.var_dum_a_dn8), (p.p28 * locals.var_dum_a_dn9),)
    } else {
        (locals.var_abet_t, locals.var_abet_t_dn0, locals.var_abet_t_dn1, locals.var_abet_t_dn3, locals.var_abet_t_dn4, locals.var_abet_t_dn5, locals.var_abet_t_dn6, locals.var_abet_t_dn7, locals.var_abet_t_dn8, locals.var_abet_t_dn9,)
    }
};
        locals.var_abet_t = assign3710_e3803;
        locals.var_abet_t_dn0 = assign3710_e3803_d_n0;
        locals.var_abet_t_dn1 = assign3710_e3803_d_n1;
        locals.var_abet_t_dn3 = assign3710_e3803_d_n3;
        locals.var_abet_t_dn4 = assign3710_e3803_d_n4;
        locals.var_abet_t_dn5 = assign3710_e3803_d_n5;
        locals.var_abet_t_dn6 = assign3710_e3803_d_n6;
        locals.var_abet_t_dn7 = assign3710_e3803_d_n7;
        locals.var_abet_t_dn8 = assign3710_e3803_d_n8;
        locals.var_abet_t_dn9 = assign3710_e3803_d_n9;

        let (assign3720_e3810, assign3720_e3810_d_n0, assign3720_e3810_d_n1, assign3720_e3810_d_n3, assign3720_e3810_d_n4, assign3720_e3810_d_n5, assign3720_e3810_d_n6, assign3720_e3810_d_n7, assign3720_e3810_d_n8, assign3720_e3810_d_n9,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard57 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibets_t, locals.var_ibets_t_dn0, locals.var_ibets_t_dn1, locals.var_ibets_t_dn3, locals.var_ibets_t_dn4, locals.var_ibets_t_dn5, locals.var_ibets_t_dn6, locals.var_ibets_t_dn7, locals.var_ibets_t_dn8, locals.var_ibets_t_dn9,)
    }
};
        locals.var_ibets_t = assign3720_e3810;
        locals.var_ibets_t_dn0 = assign3720_e3810_d_n0;
        locals.var_ibets_t_dn1 = assign3720_e3810_d_n1;
        locals.var_ibets_t_dn3 = assign3720_e3810_d_n3;
        locals.var_ibets_t_dn4 = assign3720_e3810_d_n4;
        locals.var_ibets_t_dn5 = assign3720_e3810_d_n5;
        locals.var_ibets_t_dn6 = assign3720_e3810_d_n6;
        locals.var_ibets_t_dn7 = assign3720_e3810_d_n7;
        locals.var_ibets_t_dn8 = assign3720_e3810_d_n8;
        locals.var_ibets_t_dn9 = assign3720_e3810_d_n9;

        let (assign3730_e3817, assign3730_e3817_d_n0, assign3730_e3817_d_n1, assign3730_e3817_d_n3, assign3730_e3817_d_n4, assign3730_e3817_d_n5, assign3730_e3817_d_n6, assign3730_e3817_d_n7, assign3730_e3817_d_n8, assign3730_e3817_d_n9,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard57 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_abet_t, locals.var_abet_t_dn0, locals.var_abet_t_dn1, locals.var_abet_t_dn3, locals.var_abet_t_dn4, locals.var_abet_t_dn5, locals.var_abet_t_dn6, locals.var_abet_t_dn7, locals.var_abet_t_dn8, locals.var_abet_t_dn9,)
    }
};
        locals.var_abet_t = assign3730_e3817;
        locals.var_abet_t_dn0 = assign3730_e3817_d_n0;
        locals.var_abet_t_dn1 = assign3730_e3817_d_n1;
        locals.var_abet_t_dn3 = assign3730_e3817_d_n3;
        locals.var_abet_t_dn4 = assign3730_e3817_d_n4;
        locals.var_abet_t_dn5 = assign3730_e3817_d_n5;
        locals.var_abet_t_dn6 = assign3730_e3817_d_n6;
        locals.var_abet_t_dn7 = assign3730_e3817_d_n7;
        locals.var_abet_t_dn8 = assign3730_e3817_d_n8;
        locals.var_abet_t_dn9 = assign3730_e3817_d_n9;

        let (assign3740_e3829, assign3740_e3829_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3740_e3822: f64 = (locals.var_vdei_t - p.p40);
        let assign3740_e3823: f64 = (-assign3740_e3822);
        let assign3740_e3825: f64 = (assign3740_e3823 / p.p31);
        let assign3740_e3826: f64 = (assign3740_e3825).exp();
        let assign3740_e3827: f64 = (p.p30 * assign3740_e3826);
        (assign3740_e3827, (p.p30 * (assign3740_e3826 * ((-locals.var_vdei_t_dn4) / p.p31))),)
    } else {
        (locals.var_ibetat0_t, locals.var_ibetat0_t_dn4,)
    }
};
        locals.var_ibetat0_t = assign3740_e3829;
        locals.var_ibetat0_t_dn4 = assign3740_e3829_d_n4;

        let assign3750_e3832: f64 = if 1.0 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard60 = assign3750_e3832;

        let (assign3760_e3856,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard60 != 0.0)) {
        let assign3760_e3838: f64 = (2.0 * locals.var_vtnom);
        let assign3760_e3841: f64 = (p.p53 * 0.5);
        let assign3760_e3843: f64 = (assign3760_e3841 * locals.var_ovtnom);
        let assign3760_e3844: f64 = (assign3760_e3843).exp();
        let assign3760_e3846: f64 = (-0.5);
        let assign3760_e3848: f64 = (assign3760_e3846 * p.p53);
        let assign3760_e3850: f64 = (assign3760_e3848 * locals.var_ovtnom);
        let assign3760_e3851: f64 = (assign3760_e3850).exp();
        let assign3760_e3852: f64 = (assign3760_e3844 - assign3760_e3851);
        let assign3760_e3853: f64 = (assign3760_e3852).ln();
        let assign3760_e3854: f64 = (assign3760_e3838 * assign3760_e3853);
        (assign3760_e3854,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign3760_e3856;

        let (assign3770_e3876, assign3770_e3876_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard60 != 0.0)) {
        let assign3770_e3862: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign3770_e3866: f64 = (1.0 - locals.var_qtt0);
        let assign3770_e3867: f64 = (locals.var_vgbc0 * assign3770_e3866);
        let assign3770_e3868: f64 = (assign3770_e3862 + assign3770_e3867);
        let assign3770_e3871: f64 = (locals.var_mg * locals.var_vt);
        let assign3770_e3873: f64 = (assign3770_e3871 * locals.var_ln_qtt0);
        let assign3770_e3874: f64 = (assign3770_e3868 - assign3770_e3873);
        (assign3770_e3874, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgbc0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign3770_e3871 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign3770_e3876;
        locals.var_vdj_t_dn4 = assign3770_e3876_d_n4;

        let (assign3780_e3902, assign3780_e3902_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard60 != 0.0)) {
        let assign3780_e3883: f64 = (2.0 * locals.var_vt);
        let assign3780_e3889: f64 = (-locals.var_vdj_t);
        let assign3780_e3891: f64 = (assign3780_e3889 * locals.var_ovt);
        let assign3780_e3892: f64 = (assign3780_e3891).exp();
        let assign3780_e3893: f64 = (4.0 * assign3780_e3892);
        let assign3780_e3894: f64 = (1.0 + assign3780_e3893);
        let assign3780_e3895: f64 = (assign3780_e3894).sqrt();
        let assign3780_e3896: f64 = (1.0 + assign3780_e3895);
        let assign3780_e3897: f64 = (0.5 * assign3780_e3896);
        let assign3780_e3898: f64 = (assign3780_e3897).ln();
        let assign3780_e3899: f64 = (assign3780_e3883 * assign3780_e3898);
        let assign3780_e3900: f64 = (locals.var_vdj_t + assign3780_e3899);
        (assign3780_e3900, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign3780_e3898) + (assign3780_e3883 * ((0.5 * ((4.0 * (assign3780_e3892 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign3780_e3889 * locals.var_ovt_dn4)))) / (2.0 * assign3780_e3895))) / assign3780_e3897)))),)
    } else {
        (locals.var_vdcx_t, locals.var_vdcx_t_dn4,)
    }
};
        locals.var_vdcx_t = assign3780_e3902;
        locals.var_vdcx_t_dn4 = assign3780_e3902_d_n4;

        let (assign3790_e3916, assign3790_e3916_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard60 != 0.0)) {
        let assign3790_e3910: f64 = (p.p53 / locals.var_vdcx_t);
        let assign3790_e3911: f64 = (assign3790_e3910).ln();
        let assign3790_e3912: f64 = (p.p54 * assign3790_e3911);
        let assign3790_e3913: f64 = (assign3790_e3912).exp();
        let assign3790_e3914: f64 = assign3790_e3913;
        (assign3790_e3914, (assign3790_e3913 * (p.p54 * ((-((p.p53 * locals.var_vdcx_t_dn4) / (locals.var_vdcx_t * locals.var_vdcx_t))) / assign3790_e3910))),)
    } else {
        (locals.var_cratio_t, locals.var_cratio_t_dn4,)
    }
};
        locals.var_cratio_t = assign3790_e3916;
        locals.var_cratio_t_dn4 = assign3790_e3916_d_n4;

        let (assign3800_e3923, assign3800_e3923_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard60 != 0.0)) {
        let assign3800_e3921: f64 = (p.p55).abs();
        (assign3800_e3921, 0.0,)
    } else {
        (locals.var_ajcx_t, locals.var_ajcx_t_dn4,)
    }
};
        locals.var_ajcx_t = assign3800_e3923;
        locals.var_ajcx_t_dn4 = assign3800_e3923_d_n4;

        let assign3810_e3926: f64 = if p.p55 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard61 = assign3810_e3926;

        let (assign3820_e3938, assign3820_e3938_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard60 != 0.0)) && (locals.var_guard61 != 0.0)) {
        let assign3820_e3934: f64 = (p.p55 * locals.var_vdcx_t);
        let assign3820_e3936: f64 = (assign3820_e3934 / p.p53);
        (assign3820_e3936, ((p.p55 * locals.var_vdcx_t_dn4) / p.p53),)
    } else {
        (locals.var_ajcx_t, locals.var_ajcx_t_dn4,)
    }
};
        locals.var_ajcx_t = assign3820_e3938;
        locals.var_ajcx_t_dn4 = assign3820_e3938_d_n4;

        let (assign3830_e3945, assign3830_e3945_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard60 == 0.0)) {
        (1.0, 0.0,)
    } else {
        (locals.var_cratio_t, locals.var_cratio_t_dn4,)
    }
};
        locals.var_cratio_t = assign3830_e3945;
        locals.var_cratio_t_dn4 = assign3830_e3945_d_n4;

        let (assign3840_e3952, assign3840_e3952_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard60 == 0.0)) {
        (p.p53, 0.0,)
    } else {
        (locals.var_vdcx_t, locals.var_vdcx_t_dn4,)
    }
};
        locals.var_vdcx_t = assign3840_e3952;
        locals.var_vdcx_t_dn4 = assign3840_e3952_d_n4;

        let (assign3850_e3959, assign3850_e3959_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard60 == 0.0)) {
        (p.p55, 0.0,)
    } else {
        (locals.var_ajcx_t, locals.var_ajcx_t_dn4,)
    }
};
        locals.var_ajcx_t = assign3850_e3959;
        locals.var_ajcx_t_dn4 = assign3850_e3959_d_n4;

        let assign3860_e3962: f64 = if p.p0 <= 300.0 { 1.0 } else { 0.0 };
        locals.var_guard62 = assign3860_e3962;

        let (assign3870_e3968, assign3870_e3968_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard62 != 0.0)) {
        (2.4, 0.0,)
    } else {
        (locals.var_ajcx_t, locals.var_ajcx_t_dn4,)
    }
};
        locals.var_ajcx_t = assign3870_e3968;
        locals.var_ajcx_t_dn4 = assign3870_e3968_d_n4;

        let (assign3880_e3974, assign3880_e3974_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3880_e3972: f64 = (locals.var_cratio_t * locals.var_cjcx01);
        (assign3880_e3972, (locals.var_cratio_t_dn4 * locals.var_cjcx01),)
    } else {
        (locals.var_cjcx01_t, locals.var_cjcx01_t_dn4,)
    }
};
        locals.var_cjcx01_t = assign3880_e3974;
        locals.var_cjcx01_t_dn4 = assign3880_e3974_d_n4;

        let (assign3890_e3980, assign3890_e3980_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3890_e3978: f64 = (locals.var_cratio_t * locals.var_cjcx02);
        (assign3890_e3978, (locals.var_cratio_t_dn4 * locals.var_cjcx02),)
    } else {
        (locals.var_cjcx02_t, locals.var_cjcx02_t_dn4,)
    }
};
        locals.var_cjcx02_t = assign3890_e3980;
        locals.var_cjcx02_t_dn4 = assign3890_e3980_d_n4;

        let (assign3900_e3997, assign3900_e3997_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign3900_e3985: f64 = (locals.var_zetabcxt * locals.var_ln_qtt0);
        let assign3900_e3988: f64 = (p.p119 * locals.var_ovtnom);
        let assign3900_e3991: f64 = (1.0 - locals.var_tn2td);
        let assign3900_e3992: f64 = (assign3900_e3988 * assign3900_e3991);
        let assign3900_e3993: f64 = (assign3900_e3985 + assign3900_e3992);
        let assign3900_e3994: f64 = (assign3900_e3993).exp();
        let assign3900_e3995: f64 = (p.p25 * assign3900_e3994);
        (assign3900_e3995, (p.p25 * (assign3900_e3994 * ((locals.var_zetabcxt * locals.var_ln_qtt0_dn4) + (assign3900_e3988 * (-locals.var_tn2td_dn4))))),)
    } else {
        (locals.var_ibcxs_t, locals.var_ibcxs_t_dn4,)
    }
};
        locals.var_ibcxs_t = assign3900_e3997;
        locals.var_ibcxs_t_dn4 = assign3900_e3997_d_n4;

        let assign3910_e4000: f64 = if p.p0 <= 300.0 { 1.0 } else { 0.0 };
        locals.var_guard63 = assign3910_e4000;

        let assign3920_e4003: f64 = if p.p57 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard64 = assign3920_e4003;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3930_e4029,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 != 0.0)) && (locals.var_guard64 != 0.0)) {
        let assign3930_e4011: f64 = (2.0 * locals.var_vtnom);
        let assign3930_e4014: f64 = (p.p58 * 0.5);
        let assign3930_e4016: f64 = (assign3930_e4014 * locals.var_ovtnom);
        let assign3930_e4017: f64 = (assign3930_e4016).exp();
        let assign3930_e4019: f64 = (-0.5);
        let assign3930_e4021: f64 = (assign3930_e4019 * p.p58);
        let assign3930_e4023: f64 = (assign3930_e4021 * locals.var_ovtnom);
        let assign3930_e4024: f64 = (assign3930_e4023).exp();
        let assign3930_e4025: f64 = (assign3930_e4017 - assign3930_e4024);
        let assign3930_e4026: f64 = (assign3930_e4025).ln();
        let assign3930_e4027: f64 = (assign3930_e4011 * assign3930_e4026);
        (assign3930_e4027,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign3930_e4029;

        let (assign3940_e4051, assign3940_e4051_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 != 0.0)) && (locals.var_guard64 != 0.0)) {
        let assign3940_e4037: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign3940_e4041: f64 = (1.0 - locals.var_qtt0);
        let assign3940_e4042: f64 = (locals.var_vgsc0 * assign3940_e4041);
        let assign3940_e4043: f64 = (assign3940_e4037 + assign3940_e4042);
        let assign3940_e4046: f64 = (locals.var_mg * locals.var_vt);
        let assign3940_e4048: f64 = (assign3940_e4046 * locals.var_ln_qtt0);
        let assign3940_e4049: f64 = (assign3940_e4043 - assign3940_e4048);
        (assign3940_e4049, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgsc0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign3940_e4046 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign3940_e4051;
        locals.var_vdj_t_dn4 = assign3940_e4051_d_n4;

        let (assign3950_e4079, assign3950_e4079_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 != 0.0)) && (locals.var_guard64 != 0.0)) {
        let assign3950_e4060: f64 = (2.0 * locals.var_vt);
        let assign3950_e4066: f64 = (-locals.var_vdj_t);
        let assign3950_e4068: f64 = (assign3950_e4066 * locals.var_ovt);
        let assign3950_e4069: f64 = (assign3950_e4068).exp();
        let assign3950_e4070: f64 = (4.0 * assign3950_e4069);
        let assign3950_e4071: f64 = (1.0 + assign3950_e4070);
        let assign3950_e4072: f64 = (assign3950_e4071).sqrt();
        let assign3950_e4073: f64 = (1.0 + assign3950_e4072);
        let assign3950_e4074: f64 = (0.5 * assign3950_e4073);
        let assign3950_e4075: f64 = (assign3950_e4074).ln();
        let assign3950_e4076: f64 = (assign3950_e4060 * assign3950_e4075);
        let assign3950_e4077: f64 = (locals.var_vdj_t + assign3950_e4076);
        (assign3950_e4077, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign3950_e4075) + (assign3950_e4060 * ((0.5 * ((4.0 * (assign3950_e4069 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign3950_e4066 * locals.var_ovt_dn4)))) / (2.0 * assign3950_e4072))) / assign3950_e4074)))),)
    } else {
        (locals.var_vds_t, locals.var_vds_t_dn4,)
    }
};
        locals.var_vds_t = assign3950_e4079;
        locals.var_vds_t_dn4 = assign3950_e4079_d_n4;

        let (assign3960_e4095, assign3960_e4095_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 != 0.0)) && (locals.var_guard64 != 0.0)) {
        let assign3960_e4089: f64 = (p.p58 / locals.var_vds_t);
        let assign3960_e4090: f64 = (assign3960_e4089).ln();
        let assign3960_e4091: f64 = (p.p59 * assign3960_e4090);
        let assign3960_e4092: f64 = (assign3960_e4091).exp();
        let assign3960_e4093: f64 = (p.p57 * assign3960_e4092);
        (assign3960_e4093, (p.p57 * (assign3960_e4092 * (p.p59 * ((-((p.p58 * locals.var_vds_t_dn4) / (locals.var_vds_t * locals.var_vds_t))) / assign3960_e4089)))),)
    } else {
        (locals.var_cjs0_t, locals.var_cjs0_t_dn4,)
    }
};
        locals.var_cjs0_t = assign3960_e4095;
        locals.var_cjs0_t_dn4 = assign3960_e4095_d_n4;

        let (assign3970_e4105, assign3970_e4105_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 != 0.0)) && (locals.var_guard64 != 0.0)) {
        let assign3970_e4102: f64 = (-2.4);
        let assign3970_e4103: f64 = (assign3970_e4102).abs();
        (assign3970_e4103, 0.0,)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign3970_e4105;
        locals.var_ajs_t_dn4 = assign3970_e4105_d_n4;

        let assign3980_e4107: f64 = (-2.4);
        let assign3980_e4109: f64 = if assign3980_e4107 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard65 = assign3980_e4109;

        let (assign3990_e4124, assign3990_e4124_d_n4,) = {
    if ((((locals.var_guard42 != 0.0) && (locals.var_guard63 != 0.0)) && (locals.var_guard64 != 0.0)) && (locals.var_guard65 != 0.0)) {
        let assign3990_e4118: f64 = (-2.4);
        let assign3990_e4120: f64 = (assign3990_e4118 * locals.var_vds_t);
        let assign3990_e4122: f64 = (assign3990_e4120 / p.p58);
        (assign3990_e4122, ((assign3990_e4118 * locals.var_vds_t_dn4) / p.p58),)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign3990_e4124;
        locals.var_ajs_t_dn4 = assign3990_e4124_d_n4;

        let (assign4000_e4133, assign4000_e4133_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 != 0.0)) && (locals.var_guard64 == 0.0)) {
        (p.p57, 0.0,)
    } else {
        (locals.var_cjs0_t, locals.var_cjs0_t_dn4,)
    }
};
        locals.var_cjs0_t = assign4000_e4133;
        locals.var_cjs0_t_dn4 = assign4000_e4133_d_n4;

        let (assign4010_e4142, assign4010_e4142_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 != 0.0)) && (locals.var_guard64 == 0.0)) {
        (p.p58, 0.0,)
    } else {
        (locals.var_vds_t, locals.var_vds_t_dn4,)
    }
};
        locals.var_vds_t = assign4010_e4142;
        locals.var_vds_t_dn4 = assign4010_e4142_d_n4;

        let (assign4020_e4152, assign4020_e4152_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 != 0.0)) && (locals.var_guard64 == 0.0)) {
        let assign4020_e4150: f64 = (-2.4);
        (assign4020_e4150, 0.0,)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign4020_e4152;
        locals.var_ajs_t_dn4 = assign4020_e4152_d_n4;

        let (assign4030_e4158,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard63 != 0.0)) {
        (2.4,)
    } else {
        (locals.var_a_jsp,)
    }
};
        locals.var_a_jsp = assign4030_e4158;

        let assign4040_e4161: f64 = if p.p57 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard66 = assign4040_e4161;

        let (assign4050_e4188,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard66 != 0.0)) {
        let assign4050_e4170: f64 = (2.0 * locals.var_vtnom);
        let assign4050_e4173: f64 = (p.p58 * 0.5);
        let assign4050_e4175: f64 = (assign4050_e4173 * locals.var_ovtnom);
        let assign4050_e4176: f64 = (assign4050_e4175).exp();
        let assign4050_e4178: f64 = (-0.5);
        let assign4050_e4180: f64 = (assign4050_e4178 * p.p58);
        let assign4050_e4182: f64 = (assign4050_e4180 * locals.var_ovtnom);
        let assign4050_e4183: f64 = (assign4050_e4182).exp();
        let assign4050_e4184: f64 = (assign4050_e4176 - assign4050_e4183);
        let assign4050_e4185: f64 = (assign4050_e4184).ln();
        let assign4050_e4186: f64 = (assign4050_e4170 * assign4050_e4185);
        (assign4050_e4186,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign4050_e4188;

        let (assign4060_e4211, assign4060_e4211_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard66 != 0.0)) {
        let assign4060_e4197: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign4060_e4201: f64 = (1.0 - locals.var_qtt0);
        let assign4060_e4202: f64 = (locals.var_vgsc0 * assign4060_e4201);
        let assign4060_e4203: f64 = (assign4060_e4197 + assign4060_e4202);
        let assign4060_e4206: f64 = (locals.var_mg * locals.var_vt);
        let assign4060_e4208: f64 = (assign4060_e4206 * locals.var_ln_qtt0);
        let assign4060_e4209: f64 = (assign4060_e4203 - assign4060_e4208);
        (assign4060_e4209, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgsc0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign4060_e4206 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign4060_e4211;
        locals.var_vdj_t_dn4 = assign4060_e4211_d_n4;

        let (assign4070_e4240, assign4070_e4240_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard66 != 0.0)) {
        let assign4070_e4221: f64 = (2.0 * locals.var_vt);
        let assign4070_e4227: f64 = (-locals.var_vdj_t);
        let assign4070_e4229: f64 = (assign4070_e4227 * locals.var_ovt);
        let assign4070_e4230: f64 = (assign4070_e4229).exp();
        let assign4070_e4231: f64 = (4.0 * assign4070_e4230);
        let assign4070_e4232: f64 = (1.0 + assign4070_e4231);
        let assign4070_e4233: f64 = (assign4070_e4232).sqrt();
        let assign4070_e4234: f64 = (1.0 + assign4070_e4233);
        let assign4070_e4235: f64 = (0.5 * assign4070_e4234);
        let assign4070_e4236: f64 = (assign4070_e4235).ln();
        let assign4070_e4237: f64 = (assign4070_e4221 * assign4070_e4236);
        let assign4070_e4238: f64 = (locals.var_vdj_t + assign4070_e4237);
        (assign4070_e4238, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign4070_e4236) + (assign4070_e4221 * ((0.5 * ((4.0 * (assign4070_e4230 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign4070_e4227 * locals.var_ovt_dn4)))) / (2.0 * assign4070_e4233))) / assign4070_e4235)))),)
    } else {
        (locals.var_vds_t, locals.var_vds_t_dn4,)
    }
};
        locals.var_vds_t = assign4070_e4240;
        locals.var_vds_t_dn4 = assign4070_e4240_d_n4;

        let (assign4080_e4257, assign4080_e4257_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard66 != 0.0)) {
        let assign4080_e4251: f64 = (p.p58 / locals.var_vds_t);
        let assign4080_e4252: f64 = (assign4080_e4251).ln();
        let assign4080_e4253: f64 = (p.p59 * assign4080_e4252);
        let assign4080_e4254: f64 = (assign4080_e4253).exp();
        let assign4080_e4255: f64 = (p.p57 * assign4080_e4254);
        (assign4080_e4255, (p.p57 * (assign4080_e4254 * (p.p59 * ((-((p.p58 * locals.var_vds_t_dn4) / (locals.var_vds_t * locals.var_vds_t))) / assign4080_e4251)))),)
    } else {
        (locals.var_cjs0_t, locals.var_cjs0_t_dn4,)
    }
};
        locals.var_cjs0_t = assign4080_e4257;
        locals.var_cjs0_t_dn4 = assign4080_e4257_d_n4;

        let (assign4090_e4268, assign4090_e4268_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard66 != 0.0)) {
        let assign4090_e4265: f64 = (-p.p60);
        let assign4090_e4266: f64 = (assign4090_e4265).abs();
        (assign4090_e4266, 0.0,)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign4090_e4268;
        locals.var_ajs_t_dn4 = assign4090_e4268_d_n4;

        let assign4100_e4270: f64 = (-p.p60);
        let assign4100_e4272: f64 = if assign4100_e4270 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign4100_e4272;

        let (assign4110_e4288, assign4110_e4288_d_n4,) = {
    if ((((locals.var_guard42 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard66 != 0.0)) && (locals.var_guard67 != 0.0)) {
        let assign4110_e4282: f64 = (-p.p60);
        let assign4110_e4284: f64 = (assign4110_e4282 * locals.var_vds_t);
        let assign4110_e4286: f64 = (assign4110_e4284 / p.p58);
        (assign4110_e4286, ((assign4110_e4282 * locals.var_vds_t_dn4) / p.p58),)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign4110_e4288;
        locals.var_ajs_t_dn4 = assign4110_e4288_d_n4;

        let (assign4120_e4298, assign4120_e4298_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard66 == 0.0)) {
        (p.p57, 0.0,)
    } else {
        (locals.var_cjs0_t, locals.var_cjs0_t_dn4,)
    }
};
        locals.var_cjs0_t = assign4120_e4298;
        locals.var_cjs0_t_dn4 = assign4120_e4298_d_n4;

        let (assign4130_e4308, assign4130_e4308_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard66 == 0.0)) {
        (p.p58, 0.0,)
    } else {
        (locals.var_vds_t, locals.var_vds_t_dn4,)
    }
};
        locals.var_vds_t = assign4130_e4308;
        locals.var_vds_t_dn4 = assign4130_e4308_d_n4;

        let (assign4140_e4319, assign4140_e4319_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard66 == 0.0)) {
        let assign4140_e4317: f64 = (-p.p60);
        (assign4140_e4317, 0.0,)
    } else {
        (locals.var_ajs_t, locals.var_ajs_t_dn4,)
    }
};
        locals.var_ajs_t = assign4140_e4319;
        locals.var_ajs_t_dn4 = assign4140_e4319_d_n4;

        let (assign4150_e4326,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard63 == 0.0)) {
        (p.p60,)
    } else {
        (locals.var_a_jsp,)
    }
};
        locals.var_a_jsp = assign4150_e4326;

        let (assign4160_e4343, assign4160_e4343_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign4160_e4331: f64 = (locals.var_zetasct * locals.var_ln_qtt0);
        let assign4160_e4334: f64 = (p.p120 * locals.var_ovtnom);
        let assign4160_e4337: f64 = (1.0 - locals.var_tn2td);
        let assign4160_e4338: f64 = (assign4160_e4334 * assign4160_e4337);
        let assign4160_e4339: f64 = (assign4160_e4331 + assign4160_e4338);
        let assign4160_e4340: f64 = (assign4160_e4339).exp();
        let assign4160_e4341: f64 = (p.p99 * assign4160_e4340);
        (assign4160_e4341, (p.p99 * (assign4160_e4340 * ((locals.var_zetasct * locals.var_ln_qtt0_dn4) + (assign4160_e4334 * (-locals.var_tn2td_dn4))))),)
    } else {
        (locals.var_iscs_t, locals.var_iscs_t_dn4,)
    }
};
        locals.var_iscs_t = assign4160_e4343;
        locals.var_iscs_t_dn4 = assign4160_e4343_d_n4;

        let (assign4170_e4360, assign4170_e4360_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign4170_e4348: f64 = (locals.var_zetasct * locals.var_ln_qtt0);
        let assign4170_e4351: f64 = (p.p119 * locals.var_ovtnom);
        let assign4170_e4354: f64 = (1.0 - locals.var_tn2td);
        let assign4170_e4355: f64 = (assign4170_e4351 * assign4170_e4354);
        let assign4170_e4356: f64 = (assign4170_e4348 + assign4170_e4355);
        let assign4170_e4357: f64 = (assign4170_e4356).exp();
        let assign4170_e4358: f64 = (p.p97 * assign4170_e4357);
        (assign4170_e4358, (p.p97 * (assign4170_e4357 * ((locals.var_zetasct * locals.var_ln_qtt0_dn4) + (assign4170_e4351 * (-locals.var_tn2td_dn4))))),)
    } else {
        (locals.var_itss_t, locals.var_itss_t_dn4,)
    }
};
        locals.var_itss_t = assign4170_e4360;
        locals.var_itss_t_dn4 = assign4170_e4360_d_n4;

        let (assign4180_e4371, assign4180_e4371_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign4180_e4365: f64 = (p.p138 - 1.0);
        let assign4180_e4367: f64 = (assign4180_e4365 * locals.var_ln_qtt0);
        let assign4180_e4368: f64 = (assign4180_e4367).exp();
        let assign4180_e4369: f64 = (p.p101 * assign4180_e4368);
        (assign4180_e4369, (p.p101 * (assign4180_e4368 * (assign4180_e4365 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_tsf_t, locals.var_tsf_t_dn4,)
    }
};
        locals.var_tsf_t = assign4180_e4371;
        locals.var_tsf_t_dn4 = assign4180_e4371_d_n4;

        let assign4190_e4374: f64 = if p.p63 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign4190_e4374;

        let assign4200_e4377: f64 = if p.p62 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign4200_e4377;

        let (assign4210_e4403,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard68 != 0.0)) && (locals.var_guard69 != 0.0)) {
        let assign4210_e4385: f64 = (2.0 * locals.var_vtnom);
        let assign4210_e4388: f64 = (p.p63 * 0.5);
        let assign4210_e4390: f64 = (assign4210_e4388 * locals.var_ovtnom);
        let assign4210_e4391: f64 = (assign4210_e4390).exp();
        let assign4210_e4393: f64 = (-0.5);
        let assign4210_e4395: f64 = (assign4210_e4393 * p.p63);
        let assign4210_e4397: f64 = (assign4210_e4395 * locals.var_ovtnom);
        let assign4210_e4398: f64 = (assign4210_e4397).exp();
        let assign4210_e4399: f64 = (assign4210_e4391 - assign4210_e4398);
        let assign4210_e4400: f64 = (assign4210_e4399).ln();
        let assign4210_e4401: f64 = (assign4210_e4385 * assign4210_e4400);
        (assign4210_e4401,)
    } else {
        (locals.var_vdj_t0,)
    }
};
        locals.var_vdj_t0 = assign4210_e4403;

        let (assign4220_e4425, assign4220_e4425_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard68 != 0.0)) && (locals.var_guard69 != 0.0)) {
        let assign4220_e4411: f64 = (locals.var_vdj_t0 * locals.var_qtt0);
        let assign4220_e4415: f64 = (1.0 - locals.var_qtt0);
        let assign4220_e4416: f64 = (locals.var_vgsc0 * assign4220_e4415);
        let assign4220_e4417: f64 = (assign4220_e4411 + assign4220_e4416);
        let assign4220_e4420: f64 = (locals.var_mg * locals.var_vt);
        let assign4220_e4422: f64 = (assign4220_e4420 * locals.var_ln_qtt0);
        let assign4220_e4423: f64 = (assign4220_e4417 - assign4220_e4422);
        (assign4220_e4423, (((locals.var_vdj_t0 * locals.var_qtt0_dn4) + (locals.var_vgsc0 * (-locals.var_qtt0_dn4))) - (((locals.var_mg * locals.var_vt_dn4) * locals.var_ln_qtt0) + (assign4220_e4420 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_vdj_t, locals.var_vdj_t_dn4,)
    }
};
        locals.var_vdj_t = assign4220_e4425;
        locals.var_vdj_t_dn4 = assign4220_e4425_d_n4;

        let (assign4230_e4453, assign4230_e4453_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard68 != 0.0)) && (locals.var_guard69 != 0.0)) {
        let assign4230_e4434: f64 = (2.0 * locals.var_vt);
        let assign4230_e4440: f64 = (-locals.var_vdj_t);
        let assign4230_e4442: f64 = (assign4230_e4440 * locals.var_ovt);
        let assign4230_e4443: f64 = (assign4230_e4442).exp();
        let assign4230_e4444: f64 = (4.0 * assign4230_e4443);
        let assign4230_e4445: f64 = (1.0 + assign4230_e4444);
        let assign4230_e4446: f64 = (assign4230_e4445).sqrt();
        let assign4230_e4447: f64 = (1.0 + assign4230_e4446);
        let assign4230_e4448: f64 = (0.5 * assign4230_e4447);
        let assign4230_e4449: f64 = (assign4230_e4448).ln();
        let assign4230_e4450: f64 = (assign4230_e4434 * assign4230_e4449);
        let assign4230_e4451: f64 = (locals.var_vdj_t + assign4230_e4450);
        (assign4230_e4451, (locals.var_vdj_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign4230_e4449) + (assign4230_e4434 * ((0.5 * ((4.0 * (assign4230_e4443 * (((-locals.var_vdj_t_dn4) * locals.var_ovt) + (assign4230_e4440 * locals.var_ovt_dn4)))) / (2.0 * assign4230_e4446))) / assign4230_e4448)))),)
    } else {
        (locals.var_vdsp_t, locals.var_vdsp_t_dn4,)
    }
};
        locals.var_vdsp_t = assign4230_e4453;
        locals.var_vdsp_t_dn4 = assign4230_e4453_d_n4;

        let (assign4240_e4469, assign4240_e4469_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard68 != 0.0)) && (locals.var_guard69 != 0.0)) {
        let assign4240_e4463: f64 = (p.p63 / locals.var_vdsp_t);
        let assign4240_e4464: f64 = (assign4240_e4463).ln();
        let assign4240_e4465: f64 = (p.p64 * assign4240_e4464);
        let assign4240_e4466: f64 = (assign4240_e4465).exp();
        let assign4240_e4467: f64 = (p.p62 * assign4240_e4466);
        (assign4240_e4467, (p.p62 * (assign4240_e4466 * (p.p64 * ((-((p.p63 * locals.var_vdsp_t_dn4) / (locals.var_vdsp_t * locals.var_vdsp_t))) / assign4240_e4463)))),)
    } else {
        (locals.var_cscp0_t, locals.var_cscp0_t_dn4,)
    }
};
        locals.var_cscp0_t = assign4240_e4469;
        locals.var_cscp0_t_dn4 = assign4240_e4469_d_n4;

        let (assign4250_e4479, assign4250_e4479_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard68 != 0.0)) && (locals.var_guard69 != 0.0)) {
        let assign4250_e4476: f64 = (-locals.var_a_jsp);
        let assign4250_e4477: f64 = (assign4250_e4476).abs();
        (assign4250_e4477, 0.0,)
    } else {
        (locals.var_ajsp_t, locals.var_ajsp_t_dn4,)
    }
};
        locals.var_ajsp_t = assign4250_e4479;
        locals.var_ajsp_t_dn4 = assign4250_e4479_d_n4;

        let assign4260_e4481: f64 = (-locals.var_a_jsp);
        let assign4260_e4483: f64 = if assign4260_e4481 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign4260_e4483;

        let (assign4270_e4498, assign4270_e4498_d_n4,) = {
    if ((((locals.var_guard42 != 0.0) && (locals.var_guard68 != 0.0)) && (locals.var_guard69 != 0.0)) && (locals.var_guard70 != 0.0)) {
        let assign4270_e4492: f64 = (-locals.var_a_jsp);
        let assign4270_e4494: f64 = (assign4270_e4492 * locals.var_vdsp_t);
        let assign4270_e4496: f64 = (assign4270_e4494 / p.p63);
        (assign4270_e4496, ((assign4270_e4492 * locals.var_vdsp_t_dn4) / p.p63),)
    } else {
        (locals.var_ajsp_t, locals.var_ajsp_t_dn4,)
    }
};
        locals.var_ajsp_t = assign4270_e4498;
        locals.var_ajsp_t_dn4 = assign4270_e4498_d_n4;

        let (assign4280_e4507, assign4280_e4507_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard68 != 0.0)) && (locals.var_guard69 == 0.0)) {
        (p.p62, 0.0,)
    } else {
        (locals.var_cscp0_t, locals.var_cscp0_t_dn4,)
    }
};
        locals.var_cscp0_t = assign4280_e4507;
        locals.var_cscp0_t_dn4 = assign4280_e4507_d_n4;

        let (assign4290_e4516, assign4290_e4516_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard68 != 0.0)) && (locals.var_guard69 == 0.0)) {
        (p.p63, 0.0,)
    } else {
        (locals.var_vdsp_t, locals.var_vdsp_t_dn4,)
    }
};
        locals.var_vdsp_t = assign4290_e4516;
        locals.var_vdsp_t_dn4 = assign4290_e4516_d_n4;

        let (assign4300_e4526, assign4300_e4526_d_n4,) = {
    if (((locals.var_guard42 != 0.0) && (locals.var_guard68 != 0.0)) && (locals.var_guard69 == 0.0)) {
        let assign4300_e4524: f64 = (-locals.var_a_jsp);
        (assign4300_e4524, 0.0,)
    } else {
        (locals.var_ajsp_t, locals.var_ajsp_t_dn4,)
    }
};
        locals.var_ajsp_t = assign4300_e4526;
        locals.var_ajsp_t_dn4 = assign4300_e4526_d_n4;

        let (assign4310_e4533, assign4310_e4533_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard68 == 0.0)) {
        (p.p62, 0.0,)
    } else {
        (locals.var_cscp0_t, locals.var_cscp0_t_dn4,)
    }
};
        locals.var_cscp0_t = assign4310_e4533;
        locals.var_cscp0_t_dn4 = assign4310_e4533_d_n4;

        let (assign4320_e4540, assign4320_e4540_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard68 == 0.0)) {
        (p.p63, 0.0,)
    } else {
        (locals.var_vdsp_t, locals.var_vdsp_t_dn4,)
    }
};
        locals.var_vdsp_t = assign4320_e4540;
        locals.var_vdsp_t_dn4 = assign4320_e4540_d_n4;

        let (assign4330_e4547, assign4330_e4547_d_n4,) = {
    if ((locals.var_guard42 != 0.0) && (locals.var_guard68 == 0.0)) {
        (locals.var_a_jsp, 0.0,)
    } else {
        (locals.var_ajsp_t, locals.var_ajsp_t_dn4,)
    }
};
        locals.var_ajsp_t = assign4330_e4547;
        locals.var_ajsp_t_dn4 = assign4330_e4547_d_n4;

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4340_e4556, assign4340_e4556_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign4340_e4552: f64 = (p.p136 * locals.var_ln_qtt0);
        let assign4340_e4553: f64 = (assign4340_e4552).exp();
        let assign4340_e4554: f64 = (p.p96 * assign4340_e4553);
        (assign4340_e4554, (p.p96 * (assign4340_e4553 * (p.p136 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_rcx_t, locals.var_rcx_t_dn4,)
    }
};
        locals.var_rcx_t = assign4340_e4556;
        locals.var_rcx_t_dn4 = assign4340_e4556_d_n4;

        let (assign4350_e4565, assign4350_e4565_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign4350_e4561: f64 = (p.p135 * locals.var_ln_qtt0);
        let assign4350_e4562: f64 = (assign4350_e4561).exp();
        let assign4350_e4563: f64 = (p.p90 * assign4350_e4562);
        (assign4350_e4563, (p.p90 * (assign4350_e4562 * (p.p135 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_rbx_t, locals.var_rbx_t_dn4,)
    }
};
        locals.var_rbx_t = assign4350_e4565;
        locals.var_rbx_t_dn4 = assign4350_e4565_d_n4;

        let (assign4360_e4574, assign4360_e4574_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign4360_e4570: f64 = (p.p137 * locals.var_ln_qtt0);
        let assign4360_e4571: f64 = (assign4360_e4570).exp();
        let assign4360_e4572: f64 = (p.p95 * assign4360_e4571);
        (assign4360_e4572, (p.p95 * (assign4360_e4571 * (p.p137 * locals.var_ln_qtt0_dn4))),)
    } else {
        (locals.var_re_t, locals.var_re_t_dn4,)
    }
};
        locals.var_re_t = assign4360_e4574;
        locals.var_re_t_dn4 = assign4360_e4574_d_n4;

        let (assign4370_e4589, assign4370_e4589_d_n4,) = {
    if (locals.var_guard42 != 0.0) {
        let assign4370_e4579: f64 = (p.p143 * locals.var_ln_qtt0);
        let assign4370_e4580: f64 = (assign4370_e4579).exp();
        let assign4370_e4581: f64 = (p.p142 * assign4370_e4580);
        let assign4370_e4585: f64 = (p.p144 * locals.var_dtdev);
        let assign4370_e4586: f64 = (1.0 + assign4370_e4585);
        let assign4370_e4587: f64 = (assign4370_e4581 * assign4370_e4586);
        (assign4370_e4587, (((p.p142 * (assign4370_e4580 * (p.p143 * locals.var_ln_qtt0_dn4))) * assign4370_e4586) + (assign4370_e4581 * (p.p144 * locals.var_dtdev_dn4))),)
    } else {
        (locals.var_rth_t, locals.var_rth_t_dn4,)
    }
};
        locals.var_rth_t = assign4370_e4589;
        locals.var_rth_t_dn4 = assign4370_e4589_d_n4;

        let assign4380_e4592: f64 = if p.p14 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign4380_e4592;

        let (assign4390_e4600, assign4390_e4600_d_n4, assign4390_e4600_d_n5, assign4390_e4600_d_n6, assign4390_e4600_d_n7, assign4390_e4600_d_n8, assign4390_e4600_d_n9,) = {
    if (locals.var_guard89 != 0.0) {
        let assign4390_e4597: f64 = (p.p15 * locals.var_vt);
        let assign4390_e4598: f64 = (locals.var_vbiei / assign4390_e4597);
        (assign4390_e4598, (-((locals.var_vbiei * (p.p15 * locals.var_vt_dn4)) / (assign4390_e4597 * assign4390_e4597))), 0.0, (locals.var_vbiei_dn6 / assign4390_e4597), 0.0, (locals.var_vbiei_dn8 / assign4390_e4597), 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign4390_e4600;
        locals.var_dio_y_dn4 = assign4390_e4600_d_n4;
        locals.var_dio_y_dn5 = assign4390_e4600_d_n5;
        locals.var_dio_y_dn6 = assign4390_e4600_d_n6;
        locals.var_dio_y_dn7 = assign4390_e4600_d_n7;
        locals.var_dio_y_dn8 = assign4390_e4600_d_n8;
        locals.var_dio_y_dn9 = assign4390_e4600_d_n9;

        let assign4400_e4603: f64 = if locals.var_dio_y > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign4400_e4603;

        let (assign4410_e4613, assign4410_e4613_d_n4, assign4410_e4613_d_n5, assign4410_e4613_d_n6, assign4410_e4613_d_n7, assign4410_e4613_d_n8, assign4410_e4613_d_n9,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard90 != 0.0)) {
        let assign4410_e4610: f64 = (locals.var_dio_y - 80.0);
        let assign4410_e4611: f64 = (1.0 + assign4410_e4610);
        (assign4410_e4611, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign4410_e4613;
        locals.var_dio_le_dn4 = assign4410_e4613_d_n4;
        locals.var_dio_le_dn5 = assign4410_e4613_d_n5;
        locals.var_dio_le_dn6 = assign4410_e4613_d_n6;
        locals.var_dio_le_dn7 = assign4410_e4613_d_n7;
        locals.var_dio_le_dn8 = assign4410_e4613_d_n8;
        locals.var_dio_le_dn9 = assign4410_e4613_d_n9;

        let (assign4420_e4619, assign4420_e4619_d_n4, assign4420_e4619_d_n5, assign4420_e4619_d_n6, assign4420_e4619_d_n7, assign4420_e4619_d_n8, assign4420_e4619_d_n9,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard90 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign4420_e4619;
        locals.var_dio_y_dn4 = assign4420_e4619_d_n4;
        locals.var_dio_y_dn5 = assign4420_e4619_d_n5;
        locals.var_dio_y_dn6 = assign4420_e4619_d_n6;
        locals.var_dio_y_dn7 = assign4420_e4619_d_n7;
        locals.var_dio_y_dn8 = assign4420_e4619_d_n8;
        locals.var_dio_y_dn9 = assign4420_e4619_d_n9;

        let (assign4430_e4626, assign4430_e4626_d_n4, assign4430_e4626_d_n5, assign4430_e4626_d_n6, assign4430_e4626_d_n7, assign4430_e4626_d_n8, assign4430_e4626_d_n9,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard90 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign4430_e4626;
        locals.var_dio_le_dn4 = assign4430_e4626_d_n4;
        locals.var_dio_le_dn5 = assign4430_e4626_d_n5;
        locals.var_dio_le_dn6 = assign4430_e4626_d_n6;
        locals.var_dio_le_dn7 = assign4430_e4626_d_n7;
        locals.var_dio_le_dn8 = assign4430_e4626_d_n8;
        locals.var_dio_le_dn9 = assign4430_e4626_d_n9;

        let (assign4440_e4637, assign4440_e4637_d_n4, assign4440_e4637_d_n5, assign4440_e4637_d_n6, assign4440_e4637_d_n7, assign4440_e4637_d_n8, assign4440_e4637_d_n9,) = {
    if (locals.var_guard89 != 0.0) {
        let assign4440_e4631: f64 = { let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign4440_e4632: f64 = (locals.var_dio_le * assign4440_e4631);
        let assign4440_e4634: f64 = (assign4440_e4632 - 1.0);
        let assign4440_e4635: f64 = (locals.var_ibeis_t * assign4440_e4634);
        (assign4440_e4635, ((locals.var_ibeis_t_dn4 * assign4440_e4634) + (locals.var_ibeis_t * ((locals.var_dio_le_dn4 * assign4440_e4631) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn4))))), (locals.var_ibeis_t * ((locals.var_dio_le_dn5 * assign4440_e4631) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn5)))), (locals.var_ibeis_t * ((locals.var_dio_le_dn6 * assign4440_e4631) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn6)))), (locals.var_ibeis_t * ((locals.var_dio_le_dn7 * assign4440_e4631) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn7)))), (locals.var_ibeis_t * ((locals.var_dio_le_dn8 * assign4440_e4631) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn8)))), (locals.var_ibeis_t * ((locals.var_dio_le_dn9 * assign4440_e4631) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn9)))),)
    } else {
        (locals.var_ibei, locals.var_ibei_dn4, locals.var_ibei_dn5, locals.var_ibei_dn6, locals.var_ibei_dn7, locals.var_ibei_dn8, locals.var_ibei_dn9,)
    }
};
        locals.var_ibei = assign4440_e4637;
        locals.var_ibei_dn4 = assign4440_e4637_d_n4;
        locals.var_ibei_dn5 = assign4440_e4637_d_n5;
        locals.var_ibei_dn6 = assign4440_e4637_d_n6;
        locals.var_ibei_dn7 = assign4440_e4637_d_n7;
        locals.var_ibei_dn8 = assign4440_e4637_d_n8;
        locals.var_ibei_dn9 = assign4440_e4637_d_n9;

        let (assign4450_e4642, assign4450_e4642_d_n4, assign4450_e4642_d_n5, assign4450_e4642_d_n6, assign4450_e4642_d_n7, assign4450_e4642_d_n8, assign4450_e4642_d_n9,) = {
    if (locals.var_guard89 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibei, locals.var_ibei_dn4, locals.var_ibei_dn5, locals.var_ibei_dn6, locals.var_ibei_dn7, locals.var_ibei_dn8, locals.var_ibei_dn9,)
    }
};
        locals.var_ibei = assign4450_e4642;
        locals.var_ibei_dn4 = assign4450_e4642_d_n4;
        locals.var_ibei_dn5 = assign4450_e4642_d_n5;
        locals.var_ibei_dn6 = assign4450_e4642_d_n6;
        locals.var_ibei_dn7 = assign4450_e4642_d_n7;
        locals.var_ibei_dn8 = assign4450_e4642_d_n8;
        locals.var_ibei_dn9 = assign4450_e4642_d_n9;

        let assign4460_e4645: f64 = if p.p16 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign4460_e4645;

        let (assign4470_e4653, assign4470_e4653_d_n4, assign4470_e4653_d_n5, assign4470_e4653_d_n6, assign4470_e4653_d_n7, assign4470_e4653_d_n8, assign4470_e4653_d_n9,) = {
    if (locals.var_guard91 != 0.0) {
        let assign4470_e4650: f64 = (p.p17 * locals.var_vt);
        let assign4470_e4651: f64 = (locals.var_vbiei / assign4470_e4650);
        (assign4470_e4651, (-((locals.var_vbiei * (p.p17 * locals.var_vt_dn4)) / (assign4470_e4650 * assign4470_e4650))), 0.0, (locals.var_vbiei_dn6 / assign4470_e4650), 0.0, (locals.var_vbiei_dn8 / assign4470_e4650), 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign4470_e4653;
        locals.var_dio_y_dn4 = assign4470_e4653_d_n4;
        locals.var_dio_y_dn5 = assign4470_e4653_d_n5;
        locals.var_dio_y_dn6 = assign4470_e4653_d_n6;
        locals.var_dio_y_dn7 = assign4470_e4653_d_n7;
        locals.var_dio_y_dn8 = assign4470_e4653_d_n8;
        locals.var_dio_y_dn9 = assign4470_e4653_d_n9;

        let assign4480_e4656: f64 = if locals.var_dio_y > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign4480_e4656;

        let (assign4490_e4666, assign4490_e4666_d_n4, assign4490_e4666_d_n5, assign4490_e4666_d_n6, assign4490_e4666_d_n7, assign4490_e4666_d_n8, assign4490_e4666_d_n9,) = {
    if ((locals.var_guard91 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign4490_e4663: f64 = (locals.var_dio_y - 80.0);
        let assign4490_e4664: f64 = (1.0 + assign4490_e4663);
        (assign4490_e4664, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign4490_e4666;
        locals.var_dio_le_dn4 = assign4490_e4666_d_n4;
        locals.var_dio_le_dn5 = assign4490_e4666_d_n5;
        locals.var_dio_le_dn6 = assign4490_e4666_d_n6;
        locals.var_dio_le_dn7 = assign4490_e4666_d_n7;
        locals.var_dio_le_dn8 = assign4490_e4666_d_n8;
        locals.var_dio_le_dn9 = assign4490_e4666_d_n9;

        let (assign4500_e4672, assign4500_e4672_d_n4, assign4500_e4672_d_n5, assign4500_e4672_d_n6, assign4500_e4672_d_n7, assign4500_e4672_d_n8, assign4500_e4672_d_n9,) = {
    if ((locals.var_guard91 != 0.0) && (locals.var_guard92 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign4500_e4672;
        locals.var_dio_y_dn4 = assign4500_e4672_d_n4;
        locals.var_dio_y_dn5 = assign4500_e4672_d_n5;
        locals.var_dio_y_dn6 = assign4500_e4672_d_n6;
        locals.var_dio_y_dn7 = assign4500_e4672_d_n7;
        locals.var_dio_y_dn8 = assign4500_e4672_d_n8;
        locals.var_dio_y_dn9 = assign4500_e4672_d_n9;

        let (assign4510_e4679, assign4510_e4679_d_n4, assign4510_e4679_d_n5, assign4510_e4679_d_n6, assign4510_e4679_d_n7, assign4510_e4679_d_n8, assign4510_e4679_d_n9,) = {
    if ((locals.var_guard91 != 0.0) && (locals.var_guard92 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign4510_e4679;
        locals.var_dio_le_dn4 = assign4510_e4679_d_n4;
        locals.var_dio_le_dn5 = assign4510_e4679_d_n5;
        locals.var_dio_le_dn6 = assign4510_e4679_d_n6;
        locals.var_dio_le_dn7 = assign4510_e4679_d_n7;
        locals.var_dio_le_dn8 = assign4510_e4679_d_n8;
        locals.var_dio_le_dn9 = assign4510_e4679_d_n9;

        let (assign4520_e4690, assign4520_e4690_d_n4, assign4520_e4690_d_n5, assign4520_e4690_d_n6, assign4520_e4690_d_n7, assign4520_e4690_d_n8, assign4520_e4690_d_n9,) = {
    if (locals.var_guard91 != 0.0) {
        let assign4520_e4684: f64 = { let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign4520_e4685: f64 = (locals.var_dio_le * assign4520_e4684);
        let assign4520_e4687: f64 = (assign4520_e4685 - 1.0);
        let assign4520_e4688: f64 = (locals.var_ireis_t * assign4520_e4687);
        (assign4520_e4688, ((locals.var_ireis_t_dn4 * assign4520_e4687) + (locals.var_ireis_t * ((locals.var_dio_le_dn4 * assign4520_e4684) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn4))))), (locals.var_ireis_t * ((locals.var_dio_le_dn5 * assign4520_e4684) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn5)))), (locals.var_ireis_t * ((locals.var_dio_le_dn6 * assign4520_e4684) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn6)))), (locals.var_ireis_t * ((locals.var_dio_le_dn7 * assign4520_e4684) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn7)))), (locals.var_ireis_t * ((locals.var_dio_le_dn8 * assign4520_e4684) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn8)))), (locals.var_ireis_t * ((locals.var_dio_le_dn9 * assign4520_e4684) + (locals.var_dio_le * ({ let limexp_arg = locals.var_dio_y; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_dio_y_dn9)))),)
    } else {
        (locals.var_irei, locals.var_irei_dn4, locals.var_irei_dn5, locals.var_irei_dn6, locals.var_irei_dn7, locals.var_irei_dn8, locals.var_irei_dn9,)
    }
};
        locals.var_irei = assign4520_e4690;
        locals.var_irei_dn4 = assign4520_e4690_d_n4;
        locals.var_irei_dn5 = assign4520_e4690_d_n5;
        locals.var_irei_dn6 = assign4520_e4690_d_n6;
        locals.var_irei_dn7 = assign4520_e4690_d_n7;
        locals.var_irei_dn8 = assign4520_e4690_d_n8;
        locals.var_irei_dn9 = assign4520_e4690_d_n9;

        let (assign4530_e4695, assign4530_e4695_d_n4, assign4530_e4695_d_n5, assign4530_e4695_d_n6, assign4530_e4695_d_n7, assign4530_e4695_d_n8, assign4530_e4695_d_n9,) = {
    if (locals.var_guard91 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_irei, locals.var_irei_dn4, locals.var_irei_dn5, locals.var_irei_dn6, locals.var_irei_dn7, locals.var_irei_dn8, locals.var_irei_dn9,)
    }
};
        locals.var_irei = assign4530_e4695;
        locals.var_irei_dn4 = assign4530_e4695_d_n4;
        locals.var_irei_dn5 = assign4530_e4695_d_n5;
        locals.var_irei_dn6 = assign4530_e4695_d_n6;
        locals.var_irei_dn7 = assign4530_e4695_d_n7;
        locals.var_irei_dn8 = assign4530_e4695_d_n8;
        locals.var_irei_dn9 = assign4530_e4695_d_n9;

        let assign4540_e4699: f64 = (locals.var_vbiei * locals.var_ovt);
        let assign4540_e4701: f64 = (assign4540_e4699 / p.p13);
        let assign4540_e4702: f64 = { let limexp_arg = assign4540_e4701; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign4540_e4703: f64 = (locals.var_c10_t * assign4540_e4702);
        locals.var_i_0f = assign4540_e4703;
        locals.var_i_0f_dn4 = ((locals.var_c10_t_dn4 * assign4540_e4702) + (locals.var_c10_t * ({ let limexp_arg = assign4540_e4701; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((locals.var_vbiei * locals.var_ovt_dn4) / p.p13))));
        locals.var_i_0f_dn6 = (locals.var_c10_t * ({ let limexp_arg = assign4540_e4701; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((locals.var_vbiei_dn6 * locals.var_ovt) / p.p13)));
        locals.var_i_0f_dn8 = (locals.var_c10_t * ({ let limexp_arg = assign4540_e4701; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((locals.var_vbiei_dn8 * locals.var_ovt) / p.p13)));

        let assign4550_e4707: f64 = (locals.var_vbici * locals.var_ovt);
        let assign4550_e4708: f64 = { let limexp_arg = assign4550_e4707; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign4550_e4709: f64 = (locals.var_c10_t * assign4550_e4708);
        locals.var_i_0r = assign4550_e4709;
        locals.var_i_0r_dn4 = ((locals.var_c10_t_dn4 * assign4550_e4708) + (locals.var_c10_t * ({ let limexp_arg = assign4550_e4707; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (locals.var_vbici * locals.var_ovt_dn4))));
        locals.var_i_0r_dn5 = (locals.var_c10_t * ({ let limexp_arg = assign4550_e4707; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (locals.var_vbici_dn5 * locals.var_ovt)));
        locals.var_i_0r_dn8 = (locals.var_c10_t * ({ let limexp_arg = assign4550_e4707; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (locals.var_vbici_dn8 * locals.var_ovt)));

        let assign4560_e4712: f64 = if locals.var_cjei0_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign4560_e4712;

        let (assign4570_e4725, assign4570_e4725_d_n4,) = {
    if (locals.var_guard93 != 0.0) {
        let assign4570_e4717: f64 = (locals.var_ajei_t).ln();
        let assign4570_e4718: f64 = (-assign4570_e4717);
        let assign4570_e4720: f64 = (assign4570_e4718 / p.p41);
        let assign4570_e4721: f64 = (assign4570_e4720).exp();
        let assign4570_e4722: f64 = (1.0 - assign4570_e4721);
        let assign4570_e4723: f64 = (locals.var_vdei_t * assign4570_e4722);
        (assign4570_e4723, ((locals.var_vdei_t_dn4 * assign4570_e4722) + (locals.var_vdei_t * (-(assign4570_e4721 * ((-(locals.var_ajei_t_dn4 / locals.var_ajei_t)) / p.p41))))),)
    } else {
        (locals.var_dfv_f, locals.var_dfv_f_dn4,)
    }
};
        locals.var_dfv_f = assign4570_e4725;
        locals.var_dfv_f_dn4 = assign4570_e4725_d_n4;

        let (assign4580_e4733, assign4580_e4733_d_n0, assign4580_e4733_d_n1, assign4580_e4733_d_n3, assign4580_e4733_d_n4, assign4580_e4733_d_n5, assign4580_e4733_d_n6, assign4580_e4733_d_n7, assign4580_e4733_d_n8, assign4580_e4733_d_n9,) = {
    if (locals.var_guard93 != 0.0) {
        let assign4580_e4729: f64 = (locals.var_dfv_f - locals.var_vbiei);
        let assign4580_e4731: f64 = (assign4580_e4729 * locals.var_ovt);
        (assign4580_e4731, 0.0, 0.0, 0.0, ((locals.var_dfv_f_dn4 * locals.var_ovt) + (assign4580_e4729 * locals.var_ovt_dn4)), 0.0, ((-locals.var_vbiei_dn6) * locals.var_ovt), 0.0, ((-locals.var_vbiei_dn8) * locals.var_ovt), 0.0,)
    } else {
        (locals.var_dfx, locals.var_dfx_dn0, locals.var_dfx_dn1, locals.var_dfx_dn3, locals.var_dfx_dn4, locals.var_dfx_dn5, locals.var_dfx_dn6, locals.var_dfx_dn7, locals.var_dfx_dn8, locals.var_dfx_dn9,)
    }
};
        locals.var_dfx = assign4580_e4733;
        locals.var_dfx_dn0 = assign4580_e4733_d_n0;
        locals.var_dfx_dn1 = assign4580_e4733_d_n1;
        locals.var_dfx_dn3 = assign4580_e4733_d_n3;
        locals.var_dfx_dn4 = assign4580_e4733_d_n4;
        locals.var_dfx_dn5 = assign4580_e4733_d_n5;
        locals.var_dfx_dn6 = assign4580_e4733_d_n6;
        locals.var_dfx_dn7 = assign4580_e4733_d_n7;
        locals.var_dfx_dn8 = assign4580_e4733_d_n8;
        locals.var_dfx_dn9 = assign4580_e4733_d_n9;

        let (assign4590_e4742, assign4590_e4742_d_n0, assign4590_e4742_d_n1, assign4590_e4742_d_n3, assign4590_e4742_d_n4, assign4590_e4742_d_n5, assign4590_e4742_d_n6, assign4590_e4742_d_n7, assign4590_e4742_d_n8, assign4590_e4742_d_n9,) = {
    if (locals.var_guard93 != 0.0) {
        let assign4590_e4737: f64 = (locals.var_dfx * locals.var_dfx);
        let assign4590_e4739: f64 = (assign4590_e4737 + 1.921812);
        let assign4590_e4740: f64 = (assign4590_e4739).sqrt();
        (assign4590_e4740, (((locals.var_dfx_dn0 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn0)) / (2.0 * assign4590_e4740)), (((locals.var_dfx_dn1 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn1)) / (2.0 * assign4590_e4740)), (((locals.var_dfx_dn3 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn3)) / (2.0 * assign4590_e4740)), (((locals.var_dfx_dn4 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn4)) / (2.0 * assign4590_e4740)), (((locals.var_dfx_dn5 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn5)) / (2.0 * assign4590_e4740)), (((locals.var_dfx_dn6 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn6)) / (2.0 * assign4590_e4740)), (((locals.var_dfx_dn7 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn7)) / (2.0 * assign4590_e4740)), (((locals.var_dfx_dn8 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn8)) / (2.0 * assign4590_e4740)), (((locals.var_dfx_dn9 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn9)) / (2.0 * assign4590_e4740)),)
    } else {
        (locals.var_dfs_q, locals.var_dfs_q_dn0, locals.var_dfs_q_dn1, locals.var_dfs_q_dn3, locals.var_dfs_q_dn4, locals.var_dfs_q_dn5, locals.var_dfs_q_dn6, locals.var_dfs_q_dn7, locals.var_dfs_q_dn8, locals.var_dfs_q_dn9,)
    }
};
        locals.var_dfs_q = assign4590_e4742;
        locals.var_dfs_q_dn0 = assign4590_e4742_d_n0;
        locals.var_dfs_q_dn1 = assign4590_e4742_d_n1;
        locals.var_dfs_q_dn3 = assign4590_e4742_d_n3;
        locals.var_dfs_q_dn4 = assign4590_e4742_d_n4;
        locals.var_dfs_q_dn5 = assign4590_e4742_d_n5;
        locals.var_dfs_q_dn6 = assign4590_e4742_d_n6;
        locals.var_dfs_q_dn7 = assign4590_e4742_d_n7;
        locals.var_dfs_q_dn8 = assign4590_e4742_d_n8;
        locals.var_dfs_q_dn9 = assign4590_e4742_d_n9;

        let (assign4600_e4750, assign4600_e4750_d_n0, assign4600_e4750_d_n1, assign4600_e4750_d_n3, assign4600_e4750_d_n4, assign4600_e4750_d_n5, assign4600_e4750_d_n6, assign4600_e4750_d_n7, assign4600_e4750_d_n8, assign4600_e4750_d_n9,) = {
    if (locals.var_guard93 != 0.0) {
        let assign4600_e4746: f64 = (locals.var_dfx + locals.var_dfs_q);
        let assign4600_e4748: f64 = (assign4600_e4746 * 0.5);
        (assign4600_e4748, ((locals.var_dfx_dn0 + locals.var_dfs_q_dn0) * 0.5), ((locals.var_dfx_dn1 + locals.var_dfs_q_dn1) * 0.5), ((locals.var_dfx_dn3 + locals.var_dfs_q_dn3) * 0.5), ((locals.var_dfx_dn4 + locals.var_dfs_q_dn4) * 0.5), ((locals.var_dfx_dn5 + locals.var_dfs_q_dn5) * 0.5), ((locals.var_dfx_dn6 + locals.var_dfs_q_dn6) * 0.5), ((locals.var_dfx_dn7 + locals.var_dfs_q_dn7) * 0.5), ((locals.var_dfx_dn8 + locals.var_dfs_q_dn8) * 0.5), ((locals.var_dfx_dn9 + locals.var_dfs_q_dn9) * 0.5),)
    } else {
        (locals.var_dfs_q2, locals.var_dfs_q2_dn0, locals.var_dfs_q2_dn1, locals.var_dfs_q2_dn3, locals.var_dfs_q2_dn4, locals.var_dfs_q2_dn5, locals.var_dfs_q2_dn6, locals.var_dfs_q2_dn7, locals.var_dfs_q2_dn8, locals.var_dfs_q2_dn9,)
    }
};
        locals.var_dfs_q2 = assign4600_e4750;
        locals.var_dfs_q2_dn0 = assign4600_e4750_d_n0;
        locals.var_dfs_q2_dn1 = assign4600_e4750_d_n1;
        locals.var_dfs_q2_dn3 = assign4600_e4750_d_n3;
        locals.var_dfs_q2_dn4 = assign4600_e4750_d_n4;
        locals.var_dfs_q2_dn5 = assign4600_e4750_d_n5;
        locals.var_dfs_q2_dn6 = assign4600_e4750_d_n6;
        locals.var_dfs_q2_dn7 = assign4600_e4750_d_n7;
        locals.var_dfs_q2_dn8 = assign4600_e4750_d_n8;
        locals.var_dfs_q2_dn9 = assign4600_e4750_d_n9;

        let (assign4610_e4758, assign4610_e4758_d_n0, assign4610_e4758_d_n1, assign4610_e4758_d_n3, assign4610_e4758_d_n4, assign4610_e4758_d_n5, assign4610_e4758_d_n6, assign4610_e4758_d_n7, assign4610_e4758_d_n8, assign4610_e4758_d_n9,) = {
    if (locals.var_guard93 != 0.0) {
        let assign4610_e4755: f64 = (locals.var_vt * locals.var_dfs_q2);
        let assign4610_e4756: f64 = (locals.var_dfv_f - assign4610_e4755);
        (assign4610_e4756, (-(locals.var_vt * locals.var_dfs_q2_dn0)), (-(locals.var_vt * locals.var_dfs_q2_dn1)), (-(locals.var_vt * locals.var_dfs_q2_dn3)), (locals.var_dfv_f_dn4 - ((locals.var_vt_dn4 * locals.var_dfs_q2) + (locals.var_vt * locals.var_dfs_q2_dn4))), (-(locals.var_vt * locals.var_dfs_q2_dn5)), (-(locals.var_vt * locals.var_dfs_q2_dn6)), (-(locals.var_vt * locals.var_dfs_q2_dn7)), (-(locals.var_vt * locals.var_dfs_q2_dn8)), (-(locals.var_vt * locals.var_dfs_q2_dn9)),)
    } else {
        (locals.var_dfv_j, locals.var_dfv_j_dn0, locals.var_dfv_j_dn1, locals.var_dfv_j_dn3, locals.var_dfv_j_dn4, locals.var_dfv_j_dn5, locals.var_dfv_j_dn6, locals.var_dfv_j_dn7, locals.var_dfv_j_dn8, locals.var_dfv_j_dn9,)
    }
};
        locals.var_dfv_j = assign4610_e4758;
        locals.var_dfv_j_dn0 = assign4610_e4758_d_n0;
        locals.var_dfv_j_dn1 = assign4610_e4758_d_n1;
        locals.var_dfv_j_dn3 = assign4610_e4758_d_n3;
        locals.var_dfv_j_dn4 = assign4610_e4758_d_n4;
        locals.var_dfv_j_dn5 = assign4610_e4758_d_n5;
        locals.var_dfv_j_dn6 = assign4610_e4758_d_n6;
        locals.var_dfv_j_dn7 = assign4610_e4758_d_n7;
        locals.var_dfv_j_dn8 = assign4610_e4758_d_n8;
        locals.var_dfv_j_dn9 = assign4610_e4758_d_n9;

        let (assign4620_e4764, assign4620_e4764_d_n0, assign4620_e4764_d_n1, assign4620_e4764_d_n3, assign4620_e4764_d_n4, assign4620_e4764_d_n5, assign4620_e4764_d_n6, assign4620_e4764_d_n7, assign4620_e4764_d_n8, assign4620_e4764_d_n9,) = {
    if (locals.var_guard93 != 0.0) {
        let assign4620_e4762: f64 = (locals.var_dfs_q2 / locals.var_dfs_q);
        (assign4620_e4762, (((locals.var_dfs_q2_dn0 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn0)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn1 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn1)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn3 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn3)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn4 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn4)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn5 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn5)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn6 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn6)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn7 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn7)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn8 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn8)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn9 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn9)) / (locals.var_dfs_q * locals.var_dfs_q)),)
    } else {
        (locals.var_dfdvj_dv, locals.var_dfdvj_dv_dn0, locals.var_dfdvj_dv_dn1, locals.var_dfdvj_dv_dn3, locals.var_dfdvj_dv_dn4, locals.var_dfdvj_dv_dn5, locals.var_dfdvj_dv_dn6, locals.var_dfdvj_dv_dn7, locals.var_dfdvj_dv_dn8, locals.var_dfdvj_dv_dn9,)
    }
};
        locals.var_dfdvj_dv = assign4620_e4764;
        locals.var_dfdvj_dv_dn0 = assign4620_e4764_d_n0;
        locals.var_dfdvj_dv_dn1 = assign4620_e4764_d_n1;
        locals.var_dfdvj_dv_dn3 = assign4620_e4764_d_n3;
        locals.var_dfdvj_dv_dn4 = assign4620_e4764_d_n4;
        locals.var_dfdvj_dv_dn5 = assign4620_e4764_d_n5;
        locals.var_dfdvj_dv_dn6 = assign4620_e4764_d_n6;
        locals.var_dfdvj_dv_dn7 = assign4620_e4764_d_n7;
        locals.var_dfdvj_dv_dn8 = assign4620_e4764_d_n8;
        locals.var_dfdvj_dv_dn9 = assign4620_e4764_d_n9;

        let (assign4630_e4773, assign4630_e4773_d_n0, assign4630_e4773_d_n1, assign4630_e4773_d_n3, assign4630_e4773_d_n4, assign4630_e4773_d_n5, assign4630_e4773_d_n6, assign4630_e4773_d_n7, assign4630_e4773_d_n8, assign4630_e4773_d_n9,) = {
    if (locals.var_guard93 != 0.0) {
        let assign4630_e4769: f64 = (locals.var_dfv_j / locals.var_vdei_t);
        let assign4630_e4770: f64 = (1.0 - assign4630_e4769);
        let assign4630_e4771: f64 = (assign4630_e4770).ln();
        (assign4630_e4771, ((-(locals.var_dfv_j_dn0 / locals.var_vdei_t)) / assign4630_e4770), ((-(locals.var_dfv_j_dn1 / locals.var_vdei_t)) / assign4630_e4770), ((-(locals.var_dfv_j_dn3 / locals.var_vdei_t)) / assign4630_e4770), ((-(((locals.var_dfv_j_dn4 * locals.var_vdei_t) - (locals.var_dfv_j * locals.var_vdei_t_dn4)) / (locals.var_vdei_t * locals.var_vdei_t))) / assign4630_e4770), ((-(locals.var_dfv_j_dn5 / locals.var_vdei_t)) / assign4630_e4770), ((-(locals.var_dfv_j_dn6 / locals.var_vdei_t)) / assign4630_e4770), ((-(locals.var_dfv_j_dn7 / locals.var_vdei_t)) / assign4630_e4770), ((-(locals.var_dfv_j_dn8 / locals.var_vdei_t)) / assign4630_e4770), ((-(locals.var_dfv_j_dn9 / locals.var_vdei_t)) / assign4630_e4770),)
    } else {
        (locals.var_dfb, locals.var_dfb_dn0, locals.var_dfb_dn1, locals.var_dfb_dn3, locals.var_dfb_dn4, locals.var_dfb_dn5, locals.var_dfb_dn6, locals.var_dfb_dn7, locals.var_dfb_dn8, locals.var_dfb_dn9,)
    }
};
        locals.var_dfb = assign4630_e4773;
        locals.var_dfb_dn0 = assign4630_e4773_d_n0;
        locals.var_dfb_dn1 = assign4630_e4773_d_n1;
        locals.var_dfb_dn3 = assign4630_e4773_d_n3;
        locals.var_dfb_dn4 = assign4630_e4773_d_n4;
        locals.var_dfb_dn5 = assign4630_e4773_d_n5;
        locals.var_dfb_dn6 = assign4630_e4773_d_n6;
        locals.var_dfb_dn7 = assign4630_e4773_d_n7;
        locals.var_dfb_dn8 = assign4630_e4773_d_n8;
        locals.var_dfb_dn9 = assign4630_e4773_d_n9;

        let (assign4640_e4783, assign4640_e4783_d_n0, assign4640_e4783_d_n1, assign4640_e4783_d_n3, assign4640_e4783_d_n4, assign4640_e4783_d_n5, assign4640_e4783_d_n6, assign4640_e4783_d_n7, assign4640_e4783_d_n8, assign4640_e4783_d_n9,) = {
    if (locals.var_guard93 != 0.0) {
        let assign4640_e4776: f64 = (-p.p41);
        let assign4640_e4778: f64 = (assign4640_e4776 * locals.var_dfb);
        let assign4640_e4779: f64 = (assign4640_e4778).exp();
        let assign4640_e4781: f64 = (assign4640_e4779 * locals.var_dfdvj_dv);
        (assign4640_e4781, (((assign4640_e4779 * (assign4640_e4776 * locals.var_dfb_dn0)) * locals.var_dfdvj_dv) + (assign4640_e4779 * locals.var_dfdvj_dv_dn0)), (((assign4640_e4779 * (assign4640_e4776 * locals.var_dfb_dn1)) * locals.var_dfdvj_dv) + (assign4640_e4779 * locals.var_dfdvj_dv_dn1)), (((assign4640_e4779 * (assign4640_e4776 * locals.var_dfb_dn3)) * locals.var_dfdvj_dv) + (assign4640_e4779 * locals.var_dfdvj_dv_dn3)), (((assign4640_e4779 * (assign4640_e4776 * locals.var_dfb_dn4)) * locals.var_dfdvj_dv) + (assign4640_e4779 * locals.var_dfdvj_dv_dn4)), (((assign4640_e4779 * (assign4640_e4776 * locals.var_dfb_dn5)) * locals.var_dfdvj_dv) + (assign4640_e4779 * locals.var_dfdvj_dv_dn5)), (((assign4640_e4779 * (assign4640_e4776 * locals.var_dfb_dn6)) * locals.var_dfdvj_dv) + (assign4640_e4779 * locals.var_dfdvj_dv_dn6)), (((assign4640_e4779 * (assign4640_e4776 * locals.var_dfb_dn7)) * locals.var_dfdvj_dv) + (assign4640_e4779 * locals.var_dfdvj_dv_dn7)), (((assign4640_e4779 * (assign4640_e4776 * locals.var_dfb_dn8)) * locals.var_dfdvj_dv) + (assign4640_e4779 * locals.var_dfdvj_dv_dn8)), (((assign4640_e4779 * (assign4640_e4776 * locals.var_dfb_dn9)) * locals.var_dfdvj_dv) + (assign4640_e4779 * locals.var_dfdvj_dv_dn9)),)
    } else {
        (locals.var_dfc_j1, locals.var_dfc_j1_dn0, locals.var_dfc_j1_dn1, locals.var_dfc_j1_dn3, locals.var_dfc_j1_dn4, locals.var_dfc_j1_dn5, locals.var_dfc_j1_dn6, locals.var_dfc_j1_dn7, locals.var_dfc_j1_dn8, locals.var_dfc_j1_dn9,)
    }
};
        locals.var_dfc_j1 = assign4640_e4783;
        locals.var_dfc_j1_dn0 = assign4640_e4783_d_n0;
        locals.var_dfc_j1_dn1 = assign4640_e4783_d_n1;
        locals.var_dfc_j1_dn3 = assign4640_e4783_d_n3;
        locals.var_dfc_j1_dn4 = assign4640_e4783_d_n4;
        locals.var_dfc_j1_dn5 = assign4640_e4783_d_n5;
        locals.var_dfc_j1_dn6 = assign4640_e4783_d_n6;
        locals.var_dfc_j1_dn7 = assign4640_e4783_d_n7;
        locals.var_dfc_j1_dn8 = assign4640_e4783_d_n8;
        locals.var_dfc_j1_dn9 = assign4640_e4783_d_n9;

        let (assign4650_e4795, assign4650_e4795_d_n0, assign4650_e4795_d_n1, assign4650_e4795_d_n3, assign4650_e4795_d_n4, assign4650_e4795_d_n5, assign4650_e4795_d_n6, assign4650_e4795_d_n7, assign4650_e4795_d_n8, assign4650_e4795_d_n9,) = {
    if (locals.var_guard93 != 0.0) {
        let assign4650_e4790: f64 = (1.0 - locals.var_dfdvj_dv);
        let assign4650_e4791: f64 = (locals.var_ajei_t * assign4650_e4790);
        let assign4650_e4792: f64 = (locals.var_dfc_j1 + assign4650_e4791);
        let assign4650_e4793: f64 = (locals.var_cjei0_t * assign4650_e4792);
        (assign4650_e4793, (locals.var_cjei0_t * (locals.var_dfc_j1_dn0 + (locals.var_ajei_t * (-locals.var_dfdvj_dv_dn0)))), (locals.var_cjei0_t * (locals.var_dfc_j1_dn1 + (locals.var_ajei_t * (-locals.var_dfdvj_dv_dn1)))), (locals.var_cjei0_t * (locals.var_dfc_j1_dn3 + (locals.var_ajei_t * (-locals.var_dfdvj_dv_dn3)))), ((locals.var_cjei0_t_dn4 * assign4650_e4792) + (locals.var_cjei0_t * (locals.var_dfc_j1_dn4 + ((locals.var_ajei_t_dn4 * assign4650_e4790) + (locals.var_ajei_t * (-locals.var_dfdvj_dv_dn4)))))), (locals.var_cjei0_t * (locals.var_dfc_j1_dn5 + (locals.var_ajei_t * (-locals.var_dfdvj_dv_dn5)))), (locals.var_cjei0_t * (locals.var_dfc_j1_dn6 + (locals.var_ajei_t * (-locals.var_dfdvj_dv_dn6)))), (locals.var_cjei0_t * (locals.var_dfc_j1_dn7 + (locals.var_ajei_t * (-locals.var_dfdvj_dv_dn7)))), (locals.var_cjei0_t * (locals.var_dfc_j1_dn8 + (locals.var_ajei_t * (-locals.var_dfdvj_dv_dn8)))), (locals.var_cjei0_t * (locals.var_dfc_j1_dn9 + (locals.var_ajei_t * (-locals.var_dfdvj_dv_dn9)))),)
    } else {
        (locals.var_cjei, locals.var_cjei_dn0, locals.var_cjei_dn1, locals.var_cjei_dn3, locals.var_cjei_dn4, locals.var_cjei_dn5, locals.var_cjei_dn6, locals.var_cjei_dn7, locals.var_cjei_dn8, locals.var_cjei_dn9,)
    }
};
        locals.var_cjei = assign4650_e4795;
        locals.var_cjei_dn0 = assign4650_e4795_d_n0;
        locals.var_cjei_dn1 = assign4650_e4795_d_n1;
        locals.var_cjei_dn3 = assign4650_e4795_d_n3;
        locals.var_cjei_dn4 = assign4650_e4795_d_n4;
        locals.var_cjei_dn5 = assign4650_e4795_d_n5;
        locals.var_cjei_dn6 = assign4650_e4795_d_n6;
        locals.var_cjei_dn7 = assign4650_e4795_d_n7;
        locals.var_cjei_dn8 = assign4650_e4795_d_n8;
        locals.var_cjei_dn9 = assign4650_e4795_d_n9;

        let (assign4660_e4812, assign4660_e4812_d_n0, assign4660_e4812_d_n1, assign4660_e4812_d_n3, assign4660_e4812_d_n4, assign4660_e4812_d_n5, assign4660_e4812_d_n6, assign4660_e4812_d_n7, assign4660_e4812_d_n8, assign4660_e4812_d_n9,) = {
    if (locals.var_guard93 != 0.0) {
        let assign4660_e4802: f64 = (1.0 - p.p41);
        let assign4660_e4803: f64 = (locals.var_dfb * assign4660_e4802);
        let assign4660_e4804: f64 = (assign4660_e4803).exp();
        let assign4660_e4805: f64 = (1.0 - assign4660_e4804);
        let assign4660_e4806: f64 = (locals.var_vdei_t * assign4660_e4805);
        let assign4660_e4809: f64 = (1.0 - p.p41);
        let assign4660_e4810: f64 = (assign4660_e4806 / assign4660_e4809);
        (assign4660_e4810, ((locals.var_vdei_t * (-(assign4660_e4804 * (locals.var_dfb_dn0 * assign4660_e4802)))) / assign4660_e4809), ((locals.var_vdei_t * (-(assign4660_e4804 * (locals.var_dfb_dn1 * assign4660_e4802)))) / assign4660_e4809), ((locals.var_vdei_t * (-(assign4660_e4804 * (locals.var_dfb_dn3 * assign4660_e4802)))) / assign4660_e4809), (((locals.var_vdei_t_dn4 * assign4660_e4805) + (locals.var_vdei_t * (-(assign4660_e4804 * (locals.var_dfb_dn4 * assign4660_e4802))))) / assign4660_e4809), ((locals.var_vdei_t * (-(assign4660_e4804 * (locals.var_dfb_dn5 * assign4660_e4802)))) / assign4660_e4809), ((locals.var_vdei_t * (-(assign4660_e4804 * (locals.var_dfb_dn6 * assign4660_e4802)))) / assign4660_e4809), ((locals.var_vdei_t * (-(assign4660_e4804 * (locals.var_dfb_dn7 * assign4660_e4802)))) / assign4660_e4809), ((locals.var_vdei_t * (-(assign4660_e4804 * (locals.var_dfb_dn8 * assign4660_e4802)))) / assign4660_e4809), ((locals.var_vdei_t * (-(assign4660_e4804 * (locals.var_dfb_dn9 * assign4660_e4802)))) / assign4660_e4809),)
    } else {
        (locals.var_dfq_j1, locals.var_dfq_j1_dn0, locals.var_dfq_j1_dn1, locals.var_dfq_j1_dn3, locals.var_dfq_j1_dn4, locals.var_dfq_j1_dn5, locals.var_dfq_j1_dn6, locals.var_dfq_j1_dn7, locals.var_dfq_j1_dn8, locals.var_dfq_j1_dn9,)
    }
};
        locals.var_dfq_j1 = assign4660_e4812;
        locals.var_dfq_j1_dn0 = assign4660_e4812_d_n0;
        locals.var_dfq_j1_dn1 = assign4660_e4812_d_n1;
        locals.var_dfq_j1_dn3 = assign4660_e4812_d_n3;
        locals.var_dfq_j1_dn4 = assign4660_e4812_d_n4;
        locals.var_dfq_j1_dn5 = assign4660_e4812_d_n5;
        locals.var_dfq_j1_dn6 = assign4660_e4812_d_n6;
        locals.var_dfq_j1_dn7 = assign4660_e4812_d_n7;
        locals.var_dfq_j1_dn8 = assign4660_e4812_d_n8;
        locals.var_dfq_j1_dn9 = assign4660_e4812_d_n9;

        let (assign4670_e4824, assign4670_e4824_d_n0, assign4670_e4824_d_n1, assign4670_e4824_d_n3, assign4670_e4824_d_n4, assign4670_e4824_d_n5, assign4670_e4824_d_n6, assign4670_e4824_d_n7, assign4670_e4824_d_n8, assign4670_e4824_d_n9,) = {
    if (locals.var_guard93 != 0.0) {
        let assign4670_e4819: f64 = (locals.var_vbiei - locals.var_dfv_j);
        let assign4670_e4820: f64 = (locals.var_ajei_t * assign4670_e4819);
        let assign4670_e4821: f64 = (locals.var_dfq_j1 + assign4670_e4820);
        let assign4670_e4822: f64 = (locals.var_cjei0_t * assign4670_e4821);
        (assign4670_e4822, (locals.var_cjei0_t * (locals.var_dfq_j1_dn0 + (locals.var_ajei_t * (-locals.var_dfv_j_dn0)))), (locals.var_cjei0_t * (locals.var_dfq_j1_dn1 + (locals.var_ajei_t * (-locals.var_dfv_j_dn1)))), (locals.var_cjei0_t * (locals.var_dfq_j1_dn3 + (locals.var_ajei_t * (-locals.var_dfv_j_dn3)))), ((locals.var_cjei0_t_dn4 * assign4670_e4821) + (locals.var_cjei0_t * (locals.var_dfq_j1_dn4 + ((locals.var_ajei_t_dn4 * assign4670_e4819) + (locals.var_ajei_t * (-locals.var_dfv_j_dn4)))))), (locals.var_cjei0_t * (locals.var_dfq_j1_dn5 + (locals.var_ajei_t * (-locals.var_dfv_j_dn5)))), (locals.var_cjei0_t * (locals.var_dfq_j1_dn6 + (locals.var_ajei_t * (locals.var_vbiei_dn6 - locals.var_dfv_j_dn6)))), (locals.var_cjei0_t * (locals.var_dfq_j1_dn7 + (locals.var_ajei_t * (-locals.var_dfv_j_dn7)))), (locals.var_cjei0_t * (locals.var_dfq_j1_dn8 + (locals.var_ajei_t * (locals.var_vbiei_dn8 - locals.var_dfv_j_dn8)))), (locals.var_cjei0_t * (locals.var_dfq_j1_dn9 + (locals.var_ajei_t * (-locals.var_dfv_j_dn9)))),)
    } else {
        (locals.var_qjei, locals.var_qjei_dn0, locals.var_qjei_dn1, locals.var_qjei_dn3, locals.var_qjei_dn4, locals.var_qjei_dn5, locals.var_qjei_dn6, locals.var_qjei_dn7, locals.var_qjei_dn8, locals.var_qjei_dn9,)
    }
};
        locals.var_qjei = assign4670_e4824;
        locals.var_qjei_dn0 = assign4670_e4824_d_n0;
        locals.var_qjei_dn1 = assign4670_e4824_d_n1;
        locals.var_qjei_dn3 = assign4670_e4824_d_n3;
        locals.var_qjei_dn4 = assign4670_e4824_d_n4;
        locals.var_qjei_dn5 = assign4670_e4824_d_n5;
        locals.var_qjei_dn6 = assign4670_e4824_d_n6;
        locals.var_qjei_dn7 = assign4670_e4824_d_n7;
        locals.var_qjei_dn8 = assign4670_e4824_d_n8;
        locals.var_qjei_dn9 = assign4670_e4824_d_n9;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4680_e4829, assign4680_e4829_d_n0, assign4680_e4829_d_n1, assign4680_e4829_d_n3, assign4680_e4829_d_n4, assign4680_e4829_d_n5, assign4680_e4829_d_n6, assign4680_e4829_d_n7, assign4680_e4829_d_n8, assign4680_e4829_d_n9,) = {
    if (locals.var_guard93 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cjei, locals.var_cjei_dn0, locals.var_cjei_dn1, locals.var_cjei_dn3, locals.var_cjei_dn4, locals.var_cjei_dn5, locals.var_cjei_dn6, locals.var_cjei_dn7, locals.var_cjei_dn8, locals.var_cjei_dn9,)
    }
};
        locals.var_cjei = assign4680_e4829;
        locals.var_cjei_dn0 = assign4680_e4829_d_n0;
        locals.var_cjei_dn1 = assign4680_e4829_d_n1;
        locals.var_cjei_dn3 = assign4680_e4829_d_n3;
        locals.var_cjei_dn4 = assign4680_e4829_d_n4;
        locals.var_cjei_dn5 = assign4680_e4829_d_n5;
        locals.var_cjei_dn6 = assign4680_e4829_d_n6;
        locals.var_cjei_dn7 = assign4680_e4829_d_n7;
        locals.var_cjei_dn8 = assign4680_e4829_d_n8;
        locals.var_cjei_dn9 = assign4680_e4829_d_n9;

        let (assign4690_e4834, assign4690_e4834_d_n0, assign4690_e4834_d_n1, assign4690_e4834_d_n3, assign4690_e4834_d_n4, assign4690_e4834_d_n5, assign4690_e4834_d_n6, assign4690_e4834_d_n7, assign4690_e4834_d_n8, assign4690_e4834_d_n9,) = {
    if (locals.var_guard93 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjei, locals.var_qjei_dn0, locals.var_qjei_dn1, locals.var_qjei_dn3, locals.var_qjei_dn4, locals.var_qjei_dn5, locals.var_qjei_dn6, locals.var_qjei_dn7, locals.var_qjei_dn8, locals.var_qjei_dn9,)
    }
};
        locals.var_qjei = assign4690_e4834;
        locals.var_qjei_dn0 = assign4690_e4834_d_n0;
        locals.var_qjei_dn1 = assign4690_e4834_d_n1;
        locals.var_qjei_dn3 = assign4690_e4834_d_n3;
        locals.var_qjei_dn4 = assign4690_e4834_d_n4;
        locals.var_qjei_dn5 = assign4690_e4834_d_n5;
        locals.var_qjei_dn6 = assign4690_e4834_d_n6;
        locals.var_qjei_dn7 = assign4690_e4834_d_n7;
        locals.var_qjei_dn8 = assign4690_e4834_d_n8;
        locals.var_qjei_dn9 = assign4690_e4834_d_n9;

        let assign4700_e4837: f64 = if p.p51 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign4700_e4837;

        let assign4710_e4840: f64 = if locals.var_cjci0_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign4710_e4840;

        let (assign4720_e4848,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4720_e4846: f64 = (p.p49 / 4.0);
        (assign4720_e4846,)
    } else {
        (locals.var_dz_r,)
    }
};
        locals.var_dz_r = assign4720_e4848;

        let (assign4730_e4856, assign4730_e4856_d_n4,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4730_e4854: f64 = (p.p51 - locals.var_vdci_t);
        (assign4730_e4854, (-locals.var_vdci_t_dn4),)
    } else {
        (locals.var_dv_p, locals.var_dv_p_dn4,)
    }
};
        locals.var_dv_p = assign4730_e4856;
        locals.var_dv_p_dn4 = assign4730_e4856_d_n4;

        let (assign4740_e4871, assign4740_e4871_d_n4,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4740_e4863: f64 = (locals.var_ajci_t).ln();
        let assign4740_e4864: f64 = (-assign4740_e4863);
        let assign4740_e4866: f64 = (assign4740_e4864 / p.p49);
        let assign4740_e4867: f64 = (assign4740_e4866).exp();
        let assign4740_e4868: f64 = (1.0 - assign4740_e4867);
        let assign4740_e4869: f64 = (locals.var_vdci_t * assign4740_e4868);
        (assign4740_e4869, ((locals.var_vdci_t_dn4 * assign4740_e4868) + (locals.var_vdci_t * (-(assign4740_e4867 * ((-(locals.var_ajci_t_dn4 / locals.var_ajci_t)) / p.p49))))),)
    } else {
        (locals.var_dv_f, locals.var_dv_f_dn4,)
    }
};
        locals.var_dv_f = assign4740_e4871;
        locals.var_dv_f_dn4 = assign4740_e4871_d_n4;

        let (assign4750_e4879, assign4750_e4879_d_n4,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4750_e4877: f64 = (locals.var_ajci_t * locals.var_cjci0_t);
        (assign4750_e4877, ((locals.var_ajci_t_dn4 * locals.var_cjci0_t) + (locals.var_ajci_t * locals.var_cjci0_t_dn4)),)
    } else {
        (locals.var_dc_max, locals.var_dc_max_dn4,)
    }
};
        locals.var_dc_max = assign4750_e4879;
        locals.var_dc_max_dn4 = assign4750_e4879_d_n4;

        let (assign4760_e4895, assign4760_e4895_d_n4,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4760_e4886: f64 = (locals.var_dz_r - p.p49);
        let assign4760_e4889: f64 = (p.p51 / locals.var_vdci_t);
        let assign4760_e4890: f64 = (assign4760_e4889).ln();
        let assign4760_e4891: f64 = (assign4760_e4886 * assign4760_e4890);
        let assign4760_e4892: f64 = (assign4760_e4891).exp();
        let assign4760_e4893: f64 = (locals.var_cjci0_t * assign4760_e4892);
        (assign4760_e4893, ((locals.var_cjci0_t_dn4 * assign4760_e4892) + (locals.var_cjci0_t * (assign4760_e4892 * (assign4760_e4886 * ((-((p.p51 * locals.var_vdci_t_dn4) / (locals.var_vdci_t * locals.var_vdci_t))) / assign4760_e4889))))),)
    } else {
        (locals.var_dc_c, locals.var_dc_c_dn4,)
    }
};
        locals.var_dc_c = assign4760_e4895;
        locals.var_dc_c_dn4 = assign4760_e4895_d_n4;

        let (assign4770_e4905, assign4770_e4905_d_n0, assign4770_e4905_d_n1, assign4770_e4905_d_n3, assign4770_e4905_d_n4, assign4770_e4905_d_n5, assign4770_e4905_d_n7, assign4770_e4905_d_n8, assign4770_e4905_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4770_e4901: f64 = (locals.var_dv_f - locals.var_vbici);
        let assign4770_e4903: f64 = (assign4770_e4901 * locals.var_ovt);
        (assign4770_e4903, 0.0, 0.0, 0.0, ((locals.var_dv_f_dn4 * locals.var_ovt) + (assign4770_e4901 * locals.var_ovt_dn4)), ((-locals.var_vbici_dn5) * locals.var_ovt), 0.0, ((-locals.var_vbici_dn8) * locals.var_ovt), 0.0,)
    } else {
        (locals.var_dv_e, locals.var_dv_e_dn0, locals.var_dv_e_dn1, locals.var_dv_e_dn3, locals.var_dv_e_dn4, locals.var_dv_e_dn5, locals.var_dv_e_dn7, locals.var_dv_e_dn8, locals.var_dv_e_dn9,)
    }
};
        locals.var_dv_e = assign4770_e4905;
        locals.var_dv_e_dn0 = assign4770_e4905_d_n0;
        locals.var_dv_e_dn1 = assign4770_e4905_d_n1;
        locals.var_dv_e_dn3 = assign4770_e4905_d_n3;
        locals.var_dv_e_dn4 = assign4770_e4905_d_n4;
        locals.var_dv_e_dn5 = assign4770_e4905_d_n5;
        locals.var_dv_e_dn7 = assign4770_e4905_d_n7;
        locals.var_dv_e_dn8 = assign4770_e4905_d_n8;
        locals.var_dv_e_dn9 = assign4770_e4905_d_n9;

        let assign4780_e4908: f64 = if locals.var_dv_e < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign4780_e4908;

        let (assign4790_e4917, assign4790_e4917_d_n0, assign4790_e4917_d_n1, assign4790_e4917_d_n3, assign4790_e4917_d_n4, assign4790_e4917_d_n5, assign4790_e4917_d_n7, assign4790_e4917_d_n8, assign4790_e4917_d_n9,) = {
    if (((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign4790_e4915: f64 = (locals.var_dv_e).exp();
        (assign4790_e4915, (assign4790_e4915 * locals.var_dv_e_dn0), (assign4790_e4915 * locals.var_dv_e_dn1), (assign4790_e4915 * locals.var_dv_e_dn3), (assign4790_e4915 * locals.var_dv_e_dn4), (assign4790_e4915 * locals.var_dv_e_dn5), (assign4790_e4915 * locals.var_dv_e_dn7), (assign4790_e4915 * locals.var_dv_e_dn8), (assign4790_e4915 * locals.var_dv_e_dn9),)
    } else {
        (locals.var_de, locals.var_de_dn0, locals.var_de_dn1, locals.var_de_dn3, locals.var_de_dn4, locals.var_de_dn5, locals.var_de_dn7, locals.var_de_dn8, locals.var_de_dn9,)
    }
};
        locals.var_de = assign4790_e4917;
        locals.var_de_dn0 = assign4790_e4917_d_n0;
        locals.var_de_dn1 = assign4790_e4917_d_n1;
        locals.var_de_dn3 = assign4790_e4917_d_n3;
        locals.var_de_dn4 = assign4790_e4917_d_n4;
        locals.var_de_dn5 = assign4790_e4917_d_n5;
        locals.var_de_dn7 = assign4790_e4917_d_n7;
        locals.var_de_dn8 = assign4790_e4917_d_n8;
        locals.var_de_dn9 = assign4790_e4917_d_n9;

        let (assign4800_e4929, assign4800_e4929_d_n0, assign4800_e4929_d_n1, assign4800_e4929_d_n3, assign4800_e4929_d_n4, assign4800_e4929_d_n5, assign4800_e4929_d_n7, assign4800_e4929_d_n8, assign4800_e4929_d_n9,) = {
    if (((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign4800_e4926: f64 = (1.0 + locals.var_de);
        let assign4800_e4927: f64 = (locals.var_de / assign4800_e4926);
        (assign4800_e4927, (((locals.var_de_dn0 * assign4800_e4926) - (locals.var_de * locals.var_de_dn0)) / (assign4800_e4926 * assign4800_e4926)), (((locals.var_de_dn1 * assign4800_e4926) - (locals.var_de * locals.var_de_dn1)) / (assign4800_e4926 * assign4800_e4926)), (((locals.var_de_dn3 * assign4800_e4926) - (locals.var_de * locals.var_de_dn3)) / (assign4800_e4926 * assign4800_e4926)), (((locals.var_de_dn4 * assign4800_e4926) - (locals.var_de * locals.var_de_dn4)) / (assign4800_e4926 * assign4800_e4926)), (((locals.var_de_dn5 * assign4800_e4926) - (locals.var_de * locals.var_de_dn5)) / (assign4800_e4926 * assign4800_e4926)), (((locals.var_de_dn7 * assign4800_e4926) - (locals.var_de * locals.var_de_dn7)) / (assign4800_e4926 * assign4800_e4926)), (((locals.var_de_dn8 * assign4800_e4926) - (locals.var_de * locals.var_de_dn8)) / (assign4800_e4926 * assign4800_e4926)), (((locals.var_de_dn9 * assign4800_e4926) - (locals.var_de * locals.var_de_dn9)) / (assign4800_e4926 * assign4800_e4926)),)
    } else {
        (locals.var_de_1, locals.var_de_1_dn0, locals.var_de_1_dn1, locals.var_de_1_dn3, locals.var_de_1_dn4, locals.var_de_1_dn5, locals.var_de_1_dn7, locals.var_de_1_dn8, locals.var_de_1_dn9,)
    }
};
        locals.var_de_1 = assign4800_e4929;
        locals.var_de_1_dn0 = assign4800_e4929_d_n0;
        locals.var_de_1_dn1 = assign4800_e4929_d_n1;
        locals.var_de_1_dn3 = assign4800_e4929_d_n3;
        locals.var_de_1_dn4 = assign4800_e4929_d_n4;
        locals.var_de_1_dn5 = assign4800_e4929_d_n5;
        locals.var_de_1_dn7 = assign4800_e4929_d_n7;
        locals.var_de_1_dn8 = assign4800_e4929_d_n8;
        locals.var_de_1_dn9 = assign4800_e4929_d_n9;

        let (assign4810_e4944, assign4810_e4944_d_n0, assign4810_e4944_d_n1, assign4810_e4944_d_n3, assign4810_e4944_d_n4, assign4810_e4944_d_n5, assign4810_e4944_d_n7, assign4810_e4944_d_n8, assign4810_e4944_d_n9,) = {
    if (((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign4810_e4939: f64 = (1.0 + locals.var_de);
        let assign4810_e4940: f64 = (assign4810_e4939).ln();
        let assign4810_e4941: f64 = (locals.var_vt * assign4810_e4940);
        let assign4810_e4942: f64 = (locals.var_dv_f - assign4810_e4941);
        (assign4810_e4942, (-(locals.var_vt * (locals.var_de_dn0 / assign4810_e4939))), (-(locals.var_vt * (locals.var_de_dn1 / assign4810_e4939))), (-(locals.var_vt * (locals.var_de_dn3 / assign4810_e4939))), (locals.var_dv_f_dn4 - ((locals.var_vt_dn4 * assign4810_e4940) + (locals.var_vt * (locals.var_de_dn4 / assign4810_e4939)))), (-(locals.var_vt * (locals.var_de_dn5 / assign4810_e4939))), (-(locals.var_vt * (locals.var_de_dn7 / assign4810_e4939))), (-(locals.var_vt * (locals.var_de_dn8 / assign4810_e4939))), (-(locals.var_vt * (locals.var_de_dn9 / assign4810_e4939))),)
    } else {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    }
};
        locals.var_dv_j1 = assign4810_e4944;
        locals.var_dv_j1_dn0 = assign4810_e4944_d_n0;
        locals.var_dv_j1_dn1 = assign4810_e4944_d_n1;
        locals.var_dv_j1_dn3 = assign4810_e4944_d_n3;
        locals.var_dv_j1_dn4 = assign4810_e4944_d_n4;
        locals.var_dv_j1_dn5 = assign4810_e4944_d_n5;
        locals.var_dv_j1_dn7 = assign4810_e4944_d_n7;
        locals.var_dv_j1_dn8 = assign4810_e4944_d_n8;
        locals.var_dv_j1_dn9 = assign4810_e4944_d_n9;

        let (assign4820_e4953, assign4820_e4953_d_n0, assign4820_e4953_d_n1, assign4820_e4953_d_n3, assign4820_e4953_d_n4, assign4820_e4953_d_n5, assign4820_e4953_d_n7, assign4820_e4953_d_n8, assign4820_e4953_d_n9,) = {
    if (((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_de_1, locals.var_de_1_dn0, locals.var_de_1_dn1, locals.var_de_1_dn3, locals.var_de_1_dn4, locals.var_de_1_dn5, locals.var_de_1_dn7, locals.var_de_1_dn8, locals.var_de_1_dn9,)
    }
};
        locals.var_de_1 = assign4820_e4953;
        locals.var_de_1_dn0 = assign4820_e4953_d_n0;
        locals.var_de_1_dn1 = assign4820_e4953_d_n1;
        locals.var_de_1_dn3 = assign4820_e4953_d_n3;
        locals.var_de_1_dn4 = assign4820_e4953_d_n4;
        locals.var_de_1_dn5 = assign4820_e4953_d_n5;
        locals.var_de_1_dn7 = assign4820_e4953_d_n7;
        locals.var_de_1_dn8 = assign4820_e4953_d_n8;
        locals.var_de_1_dn9 = assign4820_e4953_d_n9;

        let (assign4830_e4962, assign4830_e4962_d_n0, assign4830_e4962_d_n1, assign4830_e4962_d_n3, assign4830_e4962_d_n4, assign4830_e4962_d_n5, assign4830_e4962_d_n7, assign4830_e4962_d_n8, assign4830_e4962_d_n9,) = {
    if (((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 == 0.0)) {
        (locals.var_vbici, 0.0, 0.0, 0.0, 0.0, locals.var_vbici_dn5, 0.0, locals.var_vbici_dn8, 0.0,)
    } else {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    }
};
        locals.var_dv_j1 = assign4830_e4962;
        locals.var_dv_j1_dn0 = assign4830_e4962_d_n0;
        locals.var_dv_j1_dn1 = assign4830_e4962_d_n1;
        locals.var_dv_j1_dn3 = assign4830_e4962_d_n3;
        locals.var_dv_j1_dn4 = assign4830_e4962_d_n4;
        locals.var_dv_j1_dn5 = assign4830_e4962_d_n5;
        locals.var_dv_j1_dn7 = assign4830_e4962_d_n7;
        locals.var_dv_j1_dn8 = assign4830_e4962_d_n8;
        locals.var_dv_j1_dn9 = assign4830_e4962_d_n9;

        let (assign4840_e4974, assign4840_e4974_d_n4,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4840_e4968: f64 = (0.1 * locals.var_dv_p);
        let assign4840_e4971: f64 = (4.0 * locals.var_vt);
        let assign4840_e4972: f64 = (assign4840_e4968 + assign4840_e4971);
        (assign4840_e4972, ((0.1 * locals.var_dv_p_dn4) + (4.0 * locals.var_vt_dn4)),)
    } else {
        (locals.var_da, locals.var_da_dn4,)
    }
};
        locals.var_da = assign4840_e4974;
        locals.var_da_dn4 = assign4840_e4974_d_n4;

        let (assign4850_e4984, assign4850_e4984_d_n0, assign4850_e4984_d_n1, assign4850_e4984_d_n3, assign4850_e4984_d_n4, assign4850_e4984_d_n5, assign4850_e4984_d_n7, assign4850_e4984_d_n8, assign4850_e4984_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4850_e4980: f64 = (locals.var_dv_p + locals.var_dv_j1);
        let assign4850_e4982: f64 = (assign4850_e4980 / locals.var_da);
        (assign4850_e4982, (locals.var_dv_j1_dn0 / locals.var_da), (locals.var_dv_j1_dn1 / locals.var_da), (locals.var_dv_j1_dn3 / locals.var_da), ((((locals.var_dv_p_dn4 + locals.var_dv_j1_dn4) * locals.var_da) - (assign4850_e4980 * locals.var_da_dn4)) / (locals.var_da * locals.var_da)), (locals.var_dv_j1_dn5 / locals.var_da), (locals.var_dv_j1_dn7 / locals.var_da), (locals.var_dv_j1_dn8 / locals.var_da), (locals.var_dv_j1_dn9 / locals.var_da),)
    } else {
        (locals.var_dv_r, locals.var_dv_r_dn0, locals.var_dv_r_dn1, locals.var_dv_r_dn3, locals.var_dv_r_dn4, locals.var_dv_r_dn5, locals.var_dv_r_dn7, locals.var_dv_r_dn8, locals.var_dv_r_dn9,)
    }
};
        locals.var_dv_r = assign4850_e4984;
        locals.var_dv_r_dn0 = assign4850_e4984_d_n0;
        locals.var_dv_r_dn1 = assign4850_e4984_d_n1;
        locals.var_dv_r_dn3 = assign4850_e4984_d_n3;
        locals.var_dv_r_dn4 = assign4850_e4984_d_n4;
        locals.var_dv_r_dn5 = assign4850_e4984_d_n5;
        locals.var_dv_r_dn7 = assign4850_e4984_d_n7;
        locals.var_dv_r_dn8 = assign4850_e4984_d_n8;
        locals.var_dv_r_dn9 = assign4850_e4984_d_n9;

        let assign4860_e4987: f64 = if locals.var_dv_r < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign4860_e4987;

        let (assign4870_e4996, assign4870_e4996_d_n0, assign4870_e4996_d_n1, assign4870_e4996_d_n3, assign4870_e4996_d_n4, assign4870_e4996_d_n5, assign4870_e4996_d_n7, assign4870_e4996_d_n8, assign4870_e4996_d_n9,) = {
    if (((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard97 != 0.0)) {
        let assign4870_e4994: f64 = (locals.var_dv_r).exp();
        (assign4870_e4994, (assign4870_e4994 * locals.var_dv_r_dn0), (assign4870_e4994 * locals.var_dv_r_dn1), (assign4870_e4994 * locals.var_dv_r_dn3), (assign4870_e4994 * locals.var_dv_r_dn4), (assign4870_e4994 * locals.var_dv_r_dn5), (assign4870_e4994 * locals.var_dv_r_dn7), (assign4870_e4994 * locals.var_dv_r_dn8), (assign4870_e4994 * locals.var_dv_r_dn9),)
    } else {
        (locals.var_de, locals.var_de_dn0, locals.var_de_dn1, locals.var_de_dn3, locals.var_de_dn4, locals.var_de_dn5, locals.var_de_dn7, locals.var_de_dn8, locals.var_de_dn9,)
    }
};
        locals.var_de = assign4870_e4996;
        locals.var_de_dn0 = assign4870_e4996_d_n0;
        locals.var_de_dn1 = assign4870_e4996_d_n1;
        locals.var_de_dn3 = assign4870_e4996_d_n3;
        locals.var_de_dn4 = assign4870_e4996_d_n4;
        locals.var_de_dn5 = assign4870_e4996_d_n5;
        locals.var_de_dn7 = assign4870_e4996_d_n7;
        locals.var_de_dn8 = assign4870_e4996_d_n8;
        locals.var_de_dn9 = assign4870_e4996_d_n9;

        let (assign4880_e5008, assign4880_e5008_d_n0, assign4880_e5008_d_n1, assign4880_e5008_d_n3, assign4880_e5008_d_n4, assign4880_e5008_d_n5, assign4880_e5008_d_n7, assign4880_e5008_d_n8, assign4880_e5008_d_n9,) = {
    if (((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard97 != 0.0)) {
        let assign4880_e5005: f64 = (1.0 + locals.var_de);
        let assign4880_e5006: f64 = (locals.var_de / assign4880_e5005);
        (assign4880_e5006, (((locals.var_de_dn0 * assign4880_e5005) - (locals.var_de * locals.var_de_dn0)) / (assign4880_e5005 * assign4880_e5005)), (((locals.var_de_dn1 * assign4880_e5005) - (locals.var_de * locals.var_de_dn1)) / (assign4880_e5005 * assign4880_e5005)), (((locals.var_de_dn3 * assign4880_e5005) - (locals.var_de * locals.var_de_dn3)) / (assign4880_e5005 * assign4880_e5005)), (((locals.var_de_dn4 * assign4880_e5005) - (locals.var_de * locals.var_de_dn4)) / (assign4880_e5005 * assign4880_e5005)), (((locals.var_de_dn5 * assign4880_e5005) - (locals.var_de * locals.var_de_dn5)) / (assign4880_e5005 * assign4880_e5005)), (((locals.var_de_dn7 * assign4880_e5005) - (locals.var_de * locals.var_de_dn7)) / (assign4880_e5005 * assign4880_e5005)), (((locals.var_de_dn8 * assign4880_e5005) - (locals.var_de * locals.var_de_dn8)) / (assign4880_e5005 * assign4880_e5005)), (((locals.var_de_dn9 * assign4880_e5005) - (locals.var_de * locals.var_de_dn9)) / (assign4880_e5005 * assign4880_e5005)),)
    } else {
        (locals.var_de_2, locals.var_de_2_dn0, locals.var_de_2_dn1, locals.var_de_2_dn3, locals.var_de_2_dn4, locals.var_de_2_dn5, locals.var_de_2_dn7, locals.var_de_2_dn8, locals.var_de_2_dn9,)
    }
};
        locals.var_de_2 = assign4880_e5008;
        locals.var_de_2_dn0 = assign4880_e5008_d_n0;
        locals.var_de_2_dn1 = assign4880_e5008_d_n1;
        locals.var_de_2_dn3 = assign4880_e5008_d_n3;
        locals.var_de_2_dn4 = assign4880_e5008_d_n4;
        locals.var_de_2_dn5 = assign4880_e5008_d_n5;
        locals.var_de_2_dn7 = assign4880_e5008_d_n7;
        locals.var_de_2_dn8 = assign4880_e5008_d_n8;
        locals.var_de_2_dn9 = assign4880_e5008_d_n9;

        let (assign4890_e5032, assign4890_e5032_d_n0, assign4890_e5032_d_n1, assign4890_e5032_d_n3, assign4890_e5032_d_n4, assign4890_e5032_d_n5, assign4890_e5032_d_n7, assign4890_e5032_d_n8, assign4890_e5032_d_n9,) = {
    if (((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard97 != 0.0)) {
        let assign4890_e5015: f64 = (-locals.var_dv_p);
        let assign4890_e5019: f64 = (1.0 + locals.var_de);
        let assign4890_e5020: f64 = (assign4890_e5019).ln();
        let assign4890_e5023: f64 = (locals.var_dv_p + locals.var_dv_f);
        let assign4890_e5024: f64 = (-assign4890_e5023);
        let assign4890_e5026: f64 = (assign4890_e5024 / locals.var_da);
        let assign4890_e5027: f64 = (assign4890_e5026).exp();
        let assign4890_e5028: f64 = (assign4890_e5020 - assign4890_e5027);
        let assign4890_e5029: f64 = (locals.var_da * assign4890_e5028);
        let assign4890_e5030: f64 = (assign4890_e5015 + assign4890_e5029);
        (assign4890_e5030, (locals.var_da * (locals.var_de_dn0 / assign4890_e5019)), (locals.var_da * (locals.var_de_dn1 / assign4890_e5019)), (locals.var_da * (locals.var_de_dn3 / assign4890_e5019)), ((-locals.var_dv_p_dn4) + ((locals.var_da_dn4 * assign4890_e5028) + (locals.var_da * ((locals.var_de_dn4 / assign4890_e5019) - (assign4890_e5027 * ((((-(locals.var_dv_p_dn4 + locals.var_dv_f_dn4)) * locals.var_da) - (assign4890_e5024 * locals.var_da_dn4)) / (locals.var_da * locals.var_da))))))), (locals.var_da * (locals.var_de_dn5 / assign4890_e5019)), (locals.var_da * (locals.var_de_dn7 / assign4890_e5019)), (locals.var_da * (locals.var_de_dn8 / assign4890_e5019)), (locals.var_da * (locals.var_de_dn9 / assign4890_e5019)),)
    } else {
        (locals.var_dv_j2, locals.var_dv_j2_dn0, locals.var_dv_j2_dn1, locals.var_dv_j2_dn3, locals.var_dv_j2_dn4, locals.var_dv_j2_dn5, locals.var_dv_j2_dn7, locals.var_dv_j2_dn8, locals.var_dv_j2_dn9,)
    }
};
        locals.var_dv_j2 = assign4890_e5032;
        locals.var_dv_j2_dn0 = assign4890_e5032_d_n0;
        locals.var_dv_j2_dn1 = assign4890_e5032_d_n1;
        locals.var_dv_j2_dn3 = assign4890_e5032_d_n3;
        locals.var_dv_j2_dn4 = assign4890_e5032_d_n4;
        locals.var_dv_j2_dn5 = assign4890_e5032_d_n5;
        locals.var_dv_j2_dn7 = assign4890_e5032_d_n7;
        locals.var_dv_j2_dn8 = assign4890_e5032_d_n8;
        locals.var_dv_j2_dn9 = assign4890_e5032_d_n9;

        let (assign4900_e5041, assign4900_e5041_d_n0, assign4900_e5041_d_n1, assign4900_e5041_d_n3, assign4900_e5041_d_n4, assign4900_e5041_d_n5, assign4900_e5041_d_n7, assign4900_e5041_d_n8, assign4900_e5041_d_n9,) = {
    if (((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard97 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_de_2, locals.var_de_2_dn0, locals.var_de_2_dn1, locals.var_de_2_dn3, locals.var_de_2_dn4, locals.var_de_2_dn5, locals.var_de_2_dn7, locals.var_de_2_dn8, locals.var_de_2_dn9,)
    }
};
        locals.var_de_2 = assign4900_e5041;
        locals.var_de_2_dn0 = assign4900_e5041_d_n0;
        locals.var_de_2_dn1 = assign4900_e5041_d_n1;
        locals.var_de_2_dn3 = assign4900_e5041_d_n3;
        locals.var_de_2_dn4 = assign4900_e5041_d_n4;
        locals.var_de_2_dn5 = assign4900_e5041_d_n5;
        locals.var_de_2_dn7 = assign4900_e5041_d_n7;
        locals.var_de_2_dn8 = assign4900_e5041_d_n8;
        locals.var_de_2_dn9 = assign4900_e5041_d_n9;

        let (assign4910_e5050, assign4910_e5050_d_n0, assign4910_e5050_d_n1, assign4910_e5050_d_n3, assign4910_e5050_d_n4, assign4910_e5050_d_n5, assign4910_e5050_d_n7, assign4910_e5050_d_n8, assign4910_e5050_d_n9,) = {
    if (((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard97 == 0.0)) {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    } else {
        (locals.var_dv_j2, locals.var_dv_j2_dn0, locals.var_dv_j2_dn1, locals.var_dv_j2_dn3, locals.var_dv_j2_dn4, locals.var_dv_j2_dn5, locals.var_dv_j2_dn7, locals.var_dv_j2_dn8, locals.var_dv_j2_dn9,)
    }
};
        locals.var_dv_j2 = assign4910_e5050;
        locals.var_dv_j2_dn0 = assign4910_e5050_d_n0;
        locals.var_dv_j2_dn1 = assign4910_e5050_d_n1;
        locals.var_dv_j2_dn3 = assign4910_e5050_d_n3;
        locals.var_dv_j2_dn4 = assign4910_e5050_d_n4;
        locals.var_dv_j2_dn5 = assign4910_e5050_d_n5;
        locals.var_dv_j2_dn7 = assign4910_e5050_d_n7;
        locals.var_dv_j2_dn8 = assign4910_e5050_d_n8;
        locals.var_dv_j2_dn9 = assign4910_e5050_d_n9;

        let (assign4920_e5058, assign4920_e5058_d_n0, assign4920_e5058_d_n1, assign4920_e5058_d_n3, assign4920_e5058_d_n4, assign4920_e5058_d_n5, assign4920_e5058_d_n7, assign4920_e5058_d_n8, assign4920_e5058_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4920_e5056: f64 = (locals.var_vbici - locals.var_dv_j1);
        (assign4920_e5056, (-locals.var_dv_j1_dn0), (-locals.var_dv_j1_dn1), (-locals.var_dv_j1_dn3), (-locals.var_dv_j1_dn4), (locals.var_vbici_dn5 - locals.var_dv_j1_dn5), (-locals.var_dv_j1_dn7), (locals.var_vbici_dn8 - locals.var_dv_j1_dn8), (-locals.var_dv_j1_dn9),)
    } else {
        (locals.var_dv_j4, locals.var_dv_j4_dn0, locals.var_dv_j4_dn1, locals.var_dv_j4_dn3, locals.var_dv_j4_dn4, locals.var_dv_j4_dn5, locals.var_dv_j4_dn7, locals.var_dv_j4_dn8, locals.var_dv_j4_dn9,)
    }
};
        locals.var_dv_j4 = assign4920_e5058;
        locals.var_dv_j4_dn0 = assign4920_e5058_d_n0;
        locals.var_dv_j4_dn1 = assign4920_e5058_d_n1;
        locals.var_dv_j4_dn3 = assign4920_e5058_d_n3;
        locals.var_dv_j4_dn4 = assign4920_e5058_d_n4;
        locals.var_dv_j4_dn5 = assign4920_e5058_d_n5;
        locals.var_dv_j4_dn7 = assign4920_e5058_d_n7;
        locals.var_dv_j4_dn8 = assign4920_e5058_d_n8;
        locals.var_dv_j4_dn9 = assign4920_e5058_d_n9;

        let (assign4930_e5069, assign4930_e5069_d_n0, assign4930_e5069_d_n1, assign4930_e5069_d_n3, assign4930_e5069_d_n4, assign4930_e5069_d_n5, assign4930_e5069_d_n7, assign4930_e5069_d_n8, assign4930_e5069_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4930_e5065: f64 = (locals.var_dv_j1 / locals.var_vdci_t);
        let assign4930_e5066: f64 = (1.0 - assign4930_e5065);
        let assign4930_e5067: f64 = (assign4930_e5066).ln();
        (assign4930_e5067, ((-(locals.var_dv_j1_dn0 / locals.var_vdci_t)) / assign4930_e5066), ((-(locals.var_dv_j1_dn1 / locals.var_vdci_t)) / assign4930_e5066), ((-(locals.var_dv_j1_dn3 / locals.var_vdci_t)) / assign4930_e5066), ((-(((locals.var_dv_j1_dn4 * locals.var_vdci_t) - (locals.var_dv_j1 * locals.var_vdci_t_dn4)) / (locals.var_vdci_t * locals.var_vdci_t))) / assign4930_e5066), ((-(locals.var_dv_j1_dn5 / locals.var_vdci_t)) / assign4930_e5066), ((-(locals.var_dv_j1_dn7 / locals.var_vdci_t)) / assign4930_e5066), ((-(locals.var_dv_j1_dn8 / locals.var_vdci_t)) / assign4930_e5066), ((-(locals.var_dv_j1_dn9 / locals.var_vdci_t)) / assign4930_e5066),)
    } else {
        (locals.var_dcln1, locals.var_dcln1_dn0, locals.var_dcln1_dn1, locals.var_dcln1_dn3, locals.var_dcln1_dn4, locals.var_dcln1_dn5, locals.var_dcln1_dn7, locals.var_dcln1_dn8, locals.var_dcln1_dn9,)
    }
};
        locals.var_dcln1 = assign4930_e5069;
        locals.var_dcln1_dn0 = assign4930_e5069_d_n0;
        locals.var_dcln1_dn1 = assign4930_e5069_d_n1;
        locals.var_dcln1_dn3 = assign4930_e5069_d_n3;
        locals.var_dcln1_dn4 = assign4930_e5069_d_n4;
        locals.var_dcln1_dn5 = assign4930_e5069_d_n5;
        locals.var_dcln1_dn7 = assign4930_e5069_d_n7;
        locals.var_dcln1_dn8 = assign4930_e5069_d_n8;
        locals.var_dcln1_dn9 = assign4930_e5069_d_n9;

        let (assign4940_e5080, assign4940_e5080_d_n0, assign4940_e5080_d_n1, assign4940_e5080_d_n3, assign4940_e5080_d_n4, assign4940_e5080_d_n5, assign4940_e5080_d_n7, assign4940_e5080_d_n8, assign4940_e5080_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4940_e5076: f64 = (locals.var_dv_j2 / locals.var_vdci_t);
        let assign4940_e5077: f64 = (1.0 - assign4940_e5076);
        let assign4940_e5078: f64 = (assign4940_e5077).ln();
        (assign4940_e5078, ((-(locals.var_dv_j2_dn0 / locals.var_vdci_t)) / assign4940_e5077), ((-(locals.var_dv_j2_dn1 / locals.var_vdci_t)) / assign4940_e5077), ((-(locals.var_dv_j2_dn3 / locals.var_vdci_t)) / assign4940_e5077), ((-(((locals.var_dv_j2_dn4 * locals.var_vdci_t) - (locals.var_dv_j2 * locals.var_vdci_t_dn4)) / (locals.var_vdci_t * locals.var_vdci_t))) / assign4940_e5077), ((-(locals.var_dv_j2_dn5 / locals.var_vdci_t)) / assign4940_e5077), ((-(locals.var_dv_j2_dn7 / locals.var_vdci_t)) / assign4940_e5077), ((-(locals.var_dv_j2_dn8 / locals.var_vdci_t)) / assign4940_e5077), ((-(locals.var_dv_j2_dn9 / locals.var_vdci_t)) / assign4940_e5077),)
    } else {
        (locals.var_dcln2, locals.var_dcln2_dn0, locals.var_dcln2_dn1, locals.var_dcln2_dn3, locals.var_dcln2_dn4, locals.var_dcln2_dn5, locals.var_dcln2_dn7, locals.var_dcln2_dn8, locals.var_dcln2_dn9,)
    }
};
        locals.var_dcln2 = assign4940_e5080;
        locals.var_dcln2_dn0 = assign4940_e5080_d_n0;
        locals.var_dcln2_dn1 = assign4940_e5080_d_n1;
        locals.var_dcln2_dn3 = assign4940_e5080_d_n3;
        locals.var_dcln2_dn4 = assign4940_e5080_d_n4;
        locals.var_dcln2_dn5 = assign4940_e5080_d_n5;
        locals.var_dcln2_dn7 = assign4940_e5080_d_n7;
        locals.var_dcln2_dn8 = assign4940_e5080_d_n8;
        locals.var_dcln2_dn9 = assign4940_e5080_d_n9;

        let (assign4950_e5088,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4950_e5086: f64 = (1.0 - p.p49);
        (assign4950_e5086,)
    } else {
        (locals.var_dz1,)
    }
};
        locals.var_dz1 = assign4950_e5088;

        let (assign4960_e5096,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4960_e5094: f64 = (1.0 - locals.var_dz_r);
        (assign4960_e5094,)
    } else {
        (locals.var_dzr1,)
    }
};
        locals.var_dzr1 = assign4960_e5096;

        let (assign4970_e5112, assign4970_e5112_d_n0, assign4970_e5112_d_n1, assign4970_e5112_d_n3, assign4970_e5112_d_n4, assign4970_e5112_d_n5, assign4970_e5112_d_n7, assign4970_e5112_d_n8, assign4970_e5112_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4970_e5103: f64 = (-p.p49);
        let assign4970_e5104: f64 = (locals.var_dcln2 * assign4970_e5103);
        let assign4970_e5105: f64 = (assign4970_e5104).exp();
        let assign4970_e5106: f64 = (locals.var_cjci0_t * assign4970_e5105);
        let assign4970_e5108: f64 = (assign4970_e5106 * locals.var_de_1);
        let assign4970_e5110: f64 = (assign4970_e5108 * locals.var_de_2);
        (assign4970_e5110, (((((locals.var_cjci0_t * (assign4970_e5105 * (locals.var_dcln2_dn0 * assign4970_e5103))) * locals.var_de_1) + (assign4970_e5106 * locals.var_de_1_dn0)) * locals.var_de_2) + (assign4970_e5108 * locals.var_de_2_dn0)), (((((locals.var_cjci0_t * (assign4970_e5105 * (locals.var_dcln2_dn1 * assign4970_e5103))) * locals.var_de_1) + (assign4970_e5106 * locals.var_de_1_dn1)) * locals.var_de_2) + (assign4970_e5108 * locals.var_de_2_dn1)), (((((locals.var_cjci0_t * (assign4970_e5105 * (locals.var_dcln2_dn3 * assign4970_e5103))) * locals.var_de_1) + (assign4970_e5106 * locals.var_de_1_dn3)) * locals.var_de_2) + (assign4970_e5108 * locals.var_de_2_dn3)), ((((((locals.var_cjci0_t_dn4 * assign4970_e5105) + (locals.var_cjci0_t * (assign4970_e5105 * (locals.var_dcln2_dn4 * assign4970_e5103)))) * locals.var_de_1) + (assign4970_e5106 * locals.var_de_1_dn4)) * locals.var_de_2) + (assign4970_e5108 * locals.var_de_2_dn4)), (((((locals.var_cjci0_t * (assign4970_e5105 * (locals.var_dcln2_dn5 * assign4970_e5103))) * locals.var_de_1) + (assign4970_e5106 * locals.var_de_1_dn5)) * locals.var_de_2) + (assign4970_e5108 * locals.var_de_2_dn5)), (((((locals.var_cjci0_t * (assign4970_e5105 * (locals.var_dcln2_dn7 * assign4970_e5103))) * locals.var_de_1) + (assign4970_e5106 * locals.var_de_1_dn7)) * locals.var_de_2) + (assign4970_e5108 * locals.var_de_2_dn7)), (((((locals.var_cjci0_t * (assign4970_e5105 * (locals.var_dcln2_dn8 * assign4970_e5103))) * locals.var_de_1) + (assign4970_e5106 * locals.var_de_1_dn8)) * locals.var_de_2) + (assign4970_e5108 * locals.var_de_2_dn8)), (((((locals.var_cjci0_t * (assign4970_e5105 * (locals.var_dcln2_dn9 * assign4970_e5103))) * locals.var_de_1) + (assign4970_e5106 * locals.var_de_1_dn9)) * locals.var_de_2) + (assign4970_e5108 * locals.var_de_2_dn9)),)
    } else {
        (locals.var_dc_j1, locals.var_dc_j1_dn0, locals.var_dc_j1_dn1, locals.var_dc_j1_dn3, locals.var_dc_j1_dn4, locals.var_dc_j1_dn5, locals.var_dc_j1_dn7, locals.var_dc_j1_dn8, locals.var_dc_j1_dn9,)
    }
};
        locals.var_dc_j1 = assign4970_e5112;
        locals.var_dc_j1_dn0 = assign4970_e5112_d_n0;
        locals.var_dc_j1_dn1 = assign4970_e5112_d_n1;
        locals.var_dc_j1_dn3 = assign4970_e5112_d_n3;
        locals.var_dc_j1_dn4 = assign4970_e5112_d_n4;
        locals.var_dc_j1_dn5 = assign4970_e5112_d_n5;
        locals.var_dc_j1_dn7 = assign4970_e5112_d_n7;
        locals.var_dc_j1_dn8 = assign4970_e5112_d_n8;
        locals.var_dc_j1_dn9 = assign4970_e5112_d_n9;

        let (assign4980_e5128, assign4980_e5128_d_n0, assign4980_e5128_d_n1, assign4980_e5128_d_n3, assign4980_e5128_d_n4, assign4980_e5128_d_n5, assign4980_e5128_d_n7, assign4980_e5128_d_n8, assign4980_e5128_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4980_e5119: f64 = (-locals.var_dz_r);
        let assign4980_e5120: f64 = (locals.var_dcln1 * assign4980_e5119);
        let assign4980_e5121: f64 = (assign4980_e5120).exp();
        let assign4980_e5122: f64 = (locals.var_dc_c * assign4980_e5121);
        let assign4980_e5125: f64 = (1.0 - locals.var_de_2);
        let assign4980_e5126: f64 = (assign4980_e5122 * assign4980_e5125);
        (assign4980_e5126, (((locals.var_dc_c * (assign4980_e5121 * (locals.var_dcln1_dn0 * assign4980_e5119))) * assign4980_e5125) + (assign4980_e5122 * (-locals.var_de_2_dn0))), (((locals.var_dc_c * (assign4980_e5121 * (locals.var_dcln1_dn1 * assign4980_e5119))) * assign4980_e5125) + (assign4980_e5122 * (-locals.var_de_2_dn1))), (((locals.var_dc_c * (assign4980_e5121 * (locals.var_dcln1_dn3 * assign4980_e5119))) * assign4980_e5125) + (assign4980_e5122 * (-locals.var_de_2_dn3))), ((((locals.var_dc_c_dn4 * assign4980_e5121) + (locals.var_dc_c * (assign4980_e5121 * (locals.var_dcln1_dn4 * assign4980_e5119)))) * assign4980_e5125) + (assign4980_e5122 * (-locals.var_de_2_dn4))), (((locals.var_dc_c * (assign4980_e5121 * (locals.var_dcln1_dn5 * assign4980_e5119))) * assign4980_e5125) + (assign4980_e5122 * (-locals.var_de_2_dn5))), (((locals.var_dc_c * (assign4980_e5121 * (locals.var_dcln1_dn7 * assign4980_e5119))) * assign4980_e5125) + (assign4980_e5122 * (-locals.var_de_2_dn7))), (((locals.var_dc_c * (assign4980_e5121 * (locals.var_dcln1_dn8 * assign4980_e5119))) * assign4980_e5125) + (assign4980_e5122 * (-locals.var_de_2_dn8))), (((locals.var_dc_c * (assign4980_e5121 * (locals.var_dcln1_dn9 * assign4980_e5119))) * assign4980_e5125) + (assign4980_e5122 * (-locals.var_de_2_dn9))),)
    } else {
        (locals.var_dc_j2, locals.var_dc_j2_dn0, locals.var_dc_j2_dn1, locals.var_dc_j2_dn3, locals.var_dc_j2_dn4, locals.var_dc_j2_dn5, locals.var_dc_j2_dn7, locals.var_dc_j2_dn8, locals.var_dc_j2_dn9,)
    }
};
        locals.var_dc_j2 = assign4980_e5128;
        locals.var_dc_j2_dn0 = assign4980_e5128_d_n0;
        locals.var_dc_j2_dn1 = assign4980_e5128_d_n1;
        locals.var_dc_j2_dn3 = assign4980_e5128_d_n3;
        locals.var_dc_j2_dn4 = assign4980_e5128_d_n4;
        locals.var_dc_j2_dn5 = assign4980_e5128_d_n5;
        locals.var_dc_j2_dn7 = assign4980_e5128_d_n7;
        locals.var_dc_j2_dn8 = assign4980_e5128_d_n8;
        locals.var_dc_j2_dn9 = assign4980_e5128_d_n9;

        let (assign4990_e5138, assign4990_e5138_d_n0, assign4990_e5138_d_n1, assign4990_e5138_d_n3, assign4990_e5138_d_n4, assign4990_e5138_d_n5, assign4990_e5138_d_n7, assign4990_e5138_d_n8, assign4990_e5138_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign4990_e5135: f64 = (1.0 - locals.var_de_1);
        let assign4990_e5136: f64 = (locals.var_dc_max * assign4990_e5135);
        (assign4990_e5136, (locals.var_dc_max * (-locals.var_de_1_dn0)), (locals.var_dc_max * (-locals.var_de_1_dn1)), (locals.var_dc_max * (-locals.var_de_1_dn3)), ((locals.var_dc_max_dn4 * assign4990_e5135) + (locals.var_dc_max * (-locals.var_de_1_dn4))), (locals.var_dc_max * (-locals.var_de_1_dn5)), (locals.var_dc_max * (-locals.var_de_1_dn7)), (locals.var_dc_max * (-locals.var_de_1_dn8)), (locals.var_dc_max * (-locals.var_de_1_dn9)),)
    } else {
        (locals.var_dc_j3, locals.var_dc_j3_dn0, locals.var_dc_j3_dn1, locals.var_dc_j3_dn3, locals.var_dc_j3_dn4, locals.var_dc_j3_dn5, locals.var_dc_j3_dn7, locals.var_dc_j3_dn8, locals.var_dc_j3_dn9,)
    }
};
        locals.var_dc_j3 = assign4990_e5138;
        locals.var_dc_j3_dn0 = assign4990_e5138_d_n0;
        locals.var_dc_j3_dn1 = assign4990_e5138_d_n1;
        locals.var_dc_j3_dn3 = assign4990_e5138_d_n3;
        locals.var_dc_j3_dn4 = assign4990_e5138_d_n4;
        locals.var_dc_j3_dn5 = assign4990_e5138_d_n5;
        locals.var_dc_j3_dn7 = assign4990_e5138_d_n7;
        locals.var_dc_j3_dn8 = assign4990_e5138_d_n8;
        locals.var_dc_j3_dn9 = assign4990_e5138_d_n9;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5000_e5148, assign5000_e5148_d_n0, assign5000_e5148_d_n1, assign5000_e5148_d_n3, assign5000_e5148_d_n4, assign5000_e5148_d_n5, assign5000_e5148_d_n6, assign5000_e5148_d_n7, assign5000_e5148_d_n8, assign5000_e5148_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign5000_e5144: f64 = (locals.var_dc_j1 + locals.var_dc_j2);
        let assign5000_e5146: f64 = (assign5000_e5144 + locals.var_dc_j3);
        (assign5000_e5146, ((locals.var_dc_j1_dn0 + locals.var_dc_j2_dn0) + locals.var_dc_j3_dn0), ((locals.var_dc_j1_dn1 + locals.var_dc_j2_dn1) + locals.var_dc_j3_dn1), ((locals.var_dc_j1_dn3 + locals.var_dc_j2_dn3) + locals.var_dc_j3_dn3), ((locals.var_dc_j1_dn4 + locals.var_dc_j2_dn4) + locals.var_dc_j3_dn4), ((locals.var_dc_j1_dn5 + locals.var_dc_j2_dn5) + locals.var_dc_j3_dn5), 0.0, ((locals.var_dc_j1_dn7 + locals.var_dc_j2_dn7) + locals.var_dc_j3_dn7), ((locals.var_dc_j1_dn8 + locals.var_dc_j2_dn8) + locals.var_dc_j3_dn8), ((locals.var_dc_j1_dn9 + locals.var_dc_j2_dn9) + locals.var_dc_j3_dn9),)
    } else {
        (locals.var_cjci, locals.var_cjci_dn0, locals.var_cjci_dn1, locals.var_cjci_dn3, locals.var_cjci_dn4, locals.var_cjci_dn5, locals.var_cjci_dn6, locals.var_cjci_dn7, locals.var_cjci_dn8, locals.var_cjci_dn9,)
    }
};
        locals.var_cjci = assign5000_e5148;
        locals.var_cjci_dn0 = assign5000_e5148_d_n0;
        locals.var_cjci_dn1 = assign5000_e5148_d_n1;
        locals.var_cjci_dn3 = assign5000_e5148_d_n3;
        locals.var_cjci_dn4 = assign5000_e5148_d_n4;
        locals.var_cjci_dn5 = assign5000_e5148_d_n5;
        locals.var_cjci_dn6 = assign5000_e5148_d_n6;
        locals.var_cjci_dn7 = assign5000_e5148_d_n7;
        locals.var_cjci_dn8 = assign5000_e5148_d_n8;
        locals.var_cjci_dn9 = assign5000_e5148_d_n9;

        let (assign5010_e5163, assign5010_e5163_d_n0, assign5010_e5163_d_n1, assign5010_e5163_d_n3, assign5010_e5163_d_n4, assign5010_e5163_d_n5, assign5010_e5163_d_n7, assign5010_e5163_d_n8, assign5010_e5163_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign5010_e5156: f64 = (locals.var_dcln2 * locals.var_dz1);
        let assign5010_e5157: f64 = (assign5010_e5156).exp();
        let assign5010_e5158: f64 = (1.0 - assign5010_e5157);
        let assign5010_e5159: f64 = (locals.var_cjci0_t * assign5010_e5158);
        let assign5010_e5161: f64 = (assign5010_e5159 / locals.var_dz1);
        (assign5010_e5161, ((locals.var_cjci0_t * (-(assign5010_e5157 * (locals.var_dcln2_dn0 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjci0_t * (-(assign5010_e5157 * (locals.var_dcln2_dn1 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjci0_t * (-(assign5010_e5157 * (locals.var_dcln2_dn3 * locals.var_dz1)))) / locals.var_dz1), (((locals.var_cjci0_t_dn4 * assign5010_e5158) + (locals.var_cjci0_t * (-(assign5010_e5157 * (locals.var_dcln2_dn4 * locals.var_dz1))))) / locals.var_dz1), ((locals.var_cjci0_t * (-(assign5010_e5157 * (locals.var_dcln2_dn5 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjci0_t * (-(assign5010_e5157 * (locals.var_dcln2_dn7 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjci0_t * (-(assign5010_e5157 * (locals.var_dcln2_dn8 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjci0_t * (-(assign5010_e5157 * (locals.var_dcln2_dn9 * locals.var_dz1)))) / locals.var_dz1),)
    } else {
        (locals.var_dq_j1, locals.var_dq_j1_dn0, locals.var_dq_j1_dn1, locals.var_dq_j1_dn3, locals.var_dq_j1_dn4, locals.var_dq_j1_dn5, locals.var_dq_j1_dn7, locals.var_dq_j1_dn8, locals.var_dq_j1_dn9,)
    }
};
        locals.var_dq_j1 = assign5010_e5163;
        locals.var_dq_j1_dn0 = assign5010_e5163_d_n0;
        locals.var_dq_j1_dn1 = assign5010_e5163_d_n1;
        locals.var_dq_j1_dn3 = assign5010_e5163_d_n3;
        locals.var_dq_j1_dn4 = assign5010_e5163_d_n4;
        locals.var_dq_j1_dn5 = assign5010_e5163_d_n5;
        locals.var_dq_j1_dn7 = assign5010_e5163_d_n7;
        locals.var_dq_j1_dn8 = assign5010_e5163_d_n8;
        locals.var_dq_j1_dn9 = assign5010_e5163_d_n9;

        let (assign5020_e5178, assign5020_e5178_d_n0, assign5020_e5178_d_n1, assign5020_e5178_d_n3, assign5020_e5178_d_n4, assign5020_e5178_d_n5, assign5020_e5178_d_n7, assign5020_e5178_d_n8, assign5020_e5178_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign5020_e5171: f64 = (locals.var_dcln1 * locals.var_dzr1);
        let assign5020_e5172: f64 = (assign5020_e5171).exp();
        let assign5020_e5173: f64 = (1.0 - assign5020_e5172);
        let assign5020_e5174: f64 = (locals.var_dc_c * assign5020_e5173);
        let assign5020_e5176: f64 = (assign5020_e5174 / locals.var_dzr1);
        (assign5020_e5176, ((locals.var_dc_c * (-(assign5020_e5172 * (locals.var_dcln1_dn0 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5020_e5172 * (locals.var_dcln1_dn1 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5020_e5172 * (locals.var_dcln1_dn3 * locals.var_dzr1)))) / locals.var_dzr1), (((locals.var_dc_c_dn4 * assign5020_e5173) + (locals.var_dc_c * (-(assign5020_e5172 * (locals.var_dcln1_dn4 * locals.var_dzr1))))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5020_e5172 * (locals.var_dcln1_dn5 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5020_e5172 * (locals.var_dcln1_dn7 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5020_e5172 * (locals.var_dcln1_dn8 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5020_e5172 * (locals.var_dcln1_dn9 * locals.var_dzr1)))) / locals.var_dzr1),)
    } else {
        (locals.var_dq_j2, locals.var_dq_j2_dn0, locals.var_dq_j2_dn1, locals.var_dq_j2_dn3, locals.var_dq_j2_dn4, locals.var_dq_j2_dn5, locals.var_dq_j2_dn7, locals.var_dq_j2_dn8, locals.var_dq_j2_dn9,)
    }
};
        locals.var_dq_j2 = assign5020_e5178;
        locals.var_dq_j2_dn0 = assign5020_e5178_d_n0;
        locals.var_dq_j2_dn1 = assign5020_e5178_d_n1;
        locals.var_dq_j2_dn3 = assign5020_e5178_d_n3;
        locals.var_dq_j2_dn4 = assign5020_e5178_d_n4;
        locals.var_dq_j2_dn5 = assign5020_e5178_d_n5;
        locals.var_dq_j2_dn7 = assign5020_e5178_d_n7;
        locals.var_dq_j2_dn8 = assign5020_e5178_d_n8;
        locals.var_dq_j2_dn9 = assign5020_e5178_d_n9;

        let (assign5030_e5193, assign5030_e5193_d_n0, assign5030_e5193_d_n1, assign5030_e5193_d_n3, assign5030_e5193_d_n4, assign5030_e5193_d_n5, assign5030_e5193_d_n7, assign5030_e5193_d_n8, assign5030_e5193_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign5030_e5186: f64 = (locals.var_dcln2 * locals.var_dzr1);
        let assign5030_e5187: f64 = (assign5030_e5186).exp();
        let assign5030_e5188: f64 = (1.0 - assign5030_e5187);
        let assign5030_e5189: f64 = (locals.var_dc_c * assign5030_e5188);
        let assign5030_e5191: f64 = (assign5030_e5189 / locals.var_dzr1);
        (assign5030_e5191, ((locals.var_dc_c * (-(assign5030_e5187 * (locals.var_dcln2_dn0 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5030_e5187 * (locals.var_dcln2_dn1 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5030_e5187 * (locals.var_dcln2_dn3 * locals.var_dzr1)))) / locals.var_dzr1), (((locals.var_dc_c_dn4 * assign5030_e5188) + (locals.var_dc_c * (-(assign5030_e5187 * (locals.var_dcln2_dn4 * locals.var_dzr1))))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5030_e5187 * (locals.var_dcln2_dn5 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5030_e5187 * (locals.var_dcln2_dn7 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5030_e5187 * (locals.var_dcln2_dn8 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign5030_e5187 * (locals.var_dcln2_dn9 * locals.var_dzr1)))) / locals.var_dzr1),)
    } else {
        (locals.var_dq_j3, locals.var_dq_j3_dn0, locals.var_dq_j3_dn1, locals.var_dq_j3_dn3, locals.var_dq_j3_dn4, locals.var_dq_j3_dn5, locals.var_dq_j3_dn7, locals.var_dq_j3_dn8, locals.var_dq_j3_dn9,)
    }
};
        locals.var_dq_j3 = assign5030_e5193;
        locals.var_dq_j3_dn0 = assign5030_e5193_d_n0;
        locals.var_dq_j3_dn1 = assign5030_e5193_d_n1;
        locals.var_dq_j3_dn3 = assign5030_e5193_d_n3;
        locals.var_dq_j3_dn4 = assign5030_e5193_d_n4;
        locals.var_dq_j3_dn5 = assign5030_e5193_d_n5;
        locals.var_dq_j3_dn7 = assign5030_e5193_d_n7;
        locals.var_dq_j3_dn8 = assign5030_e5193_d_n8;
        locals.var_dq_j3_dn9 = assign5030_e5193_d_n9;

        let (assign5040_e5209, assign5040_e5209_d_n0, assign5040_e5209_d_n1, assign5040_e5209_d_n3, assign5040_e5209_d_n4, assign5040_e5209_d_n5, assign5040_e5209_d_n6, assign5040_e5209_d_n7, assign5040_e5209_d_n8, assign5040_e5209_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign5040_e5199: f64 = (locals.var_dq_j1 + locals.var_dq_j2);
        let assign5040_e5201: f64 = (assign5040_e5199 - locals.var_dq_j3);
        let assign5040_e5203: f64 = (assign5040_e5201 * locals.var_vdci_t);
        let assign5040_e5206: f64 = (locals.var_dc_max * locals.var_dv_j4);
        let assign5040_e5207: f64 = (assign5040_e5203 + assign5040_e5206);
        (assign5040_e5207, ((((locals.var_dq_j1_dn0 + locals.var_dq_j2_dn0) - locals.var_dq_j3_dn0) * locals.var_vdci_t) + (locals.var_dc_max * locals.var_dv_j4_dn0)), ((((locals.var_dq_j1_dn1 + locals.var_dq_j2_dn1) - locals.var_dq_j3_dn1) * locals.var_vdci_t) + (locals.var_dc_max * locals.var_dv_j4_dn1)), ((((locals.var_dq_j1_dn3 + locals.var_dq_j2_dn3) - locals.var_dq_j3_dn3) * locals.var_vdci_t) + (locals.var_dc_max * locals.var_dv_j4_dn3)), (((((locals.var_dq_j1_dn4 + locals.var_dq_j2_dn4) - locals.var_dq_j3_dn4) * locals.var_vdci_t) + (assign5040_e5201 * locals.var_vdci_t_dn4)) + ((locals.var_dc_max_dn4 * locals.var_dv_j4) + (locals.var_dc_max * locals.var_dv_j4_dn4))), ((((locals.var_dq_j1_dn5 + locals.var_dq_j2_dn5) - locals.var_dq_j3_dn5) * locals.var_vdci_t) + (locals.var_dc_max * locals.var_dv_j4_dn5)), 0.0, ((((locals.var_dq_j1_dn7 + locals.var_dq_j2_dn7) - locals.var_dq_j3_dn7) * locals.var_vdci_t) + (locals.var_dc_max * locals.var_dv_j4_dn7)), ((((locals.var_dq_j1_dn8 + locals.var_dq_j2_dn8) - locals.var_dq_j3_dn8) * locals.var_vdci_t) + (locals.var_dc_max * locals.var_dv_j4_dn8)), ((((locals.var_dq_j1_dn9 + locals.var_dq_j2_dn9) - locals.var_dq_j3_dn9) * locals.var_vdci_t) + (locals.var_dc_max * locals.var_dv_j4_dn9)),)
    } else {
        (locals.var_qjci, locals.var_qjci_dn0, locals.var_qjci_dn1, locals.var_qjci_dn3, locals.var_qjci_dn4, locals.var_qjci_dn5, locals.var_qjci_dn6, locals.var_qjci_dn7, locals.var_qjci_dn8, locals.var_qjci_dn9,)
    }
};
        locals.var_qjci = assign5040_e5209;
        locals.var_qjci_dn0 = assign5040_e5209_d_n0;
        locals.var_qjci_dn1 = assign5040_e5209_d_n1;
        locals.var_qjci_dn3 = assign5040_e5209_d_n3;
        locals.var_qjci_dn4 = assign5040_e5209_d_n4;
        locals.var_qjci_dn5 = assign5040_e5209_d_n5;
        locals.var_qjci_dn6 = assign5040_e5209_d_n6;
        locals.var_qjci_dn7 = assign5040_e5209_d_n7;
        locals.var_qjci_dn8 = assign5040_e5209_d_n8;
        locals.var_qjci_dn9 = assign5040_e5209_d_n9;

        let (assign5050_e5216, assign5050_e5216_d_n0, assign5050_e5216_d_n1, assign5050_e5216_d_n3, assign5050_e5216_d_n4, assign5050_e5216_d_n5, assign5050_e5216_d_n6, assign5050_e5216_d_n7, assign5050_e5216_d_n8, assign5050_e5216_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cjci, locals.var_cjci_dn0, locals.var_cjci_dn1, locals.var_cjci_dn3, locals.var_cjci_dn4, locals.var_cjci_dn5, locals.var_cjci_dn6, locals.var_cjci_dn7, locals.var_cjci_dn8, locals.var_cjci_dn9,)
    }
};
        locals.var_cjci = assign5050_e5216;
        locals.var_cjci_dn0 = assign5050_e5216_d_n0;
        locals.var_cjci_dn1 = assign5050_e5216_d_n1;
        locals.var_cjci_dn3 = assign5050_e5216_d_n3;
        locals.var_cjci_dn4 = assign5050_e5216_d_n4;
        locals.var_cjci_dn5 = assign5050_e5216_d_n5;
        locals.var_cjci_dn6 = assign5050_e5216_d_n6;
        locals.var_cjci_dn7 = assign5050_e5216_d_n7;
        locals.var_cjci_dn8 = assign5050_e5216_d_n8;
        locals.var_cjci_dn9 = assign5050_e5216_d_n9;

        let (assign5060_e5223, assign5060_e5223_d_n0, assign5060_e5223_d_n1, assign5060_e5223_d_n3, assign5060_e5223_d_n4, assign5060_e5223_d_n5, assign5060_e5223_d_n6, assign5060_e5223_d_n7, assign5060_e5223_d_n8, assign5060_e5223_d_n9,) = {
    if ((locals.var_guard94 != 0.0) && (locals.var_guard95 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjci, locals.var_qjci_dn0, locals.var_qjci_dn1, locals.var_qjci_dn3, locals.var_qjci_dn4, locals.var_qjci_dn5, locals.var_qjci_dn6, locals.var_qjci_dn7, locals.var_qjci_dn8, locals.var_qjci_dn9,)
    }
};
        locals.var_qjci = assign5060_e5223;
        locals.var_qjci_dn0 = assign5060_e5223_d_n0;
        locals.var_qjci_dn1 = assign5060_e5223_d_n1;
        locals.var_qjci_dn3 = assign5060_e5223_d_n3;
        locals.var_qjci_dn4 = assign5060_e5223_d_n4;
        locals.var_qjci_dn5 = assign5060_e5223_d_n5;
        locals.var_qjci_dn6 = assign5060_e5223_d_n6;
        locals.var_qjci_dn7 = assign5060_e5223_d_n7;
        locals.var_qjci_dn8 = assign5060_e5223_d_n8;
        locals.var_qjci_dn9 = assign5060_e5223_d_n9;

        let assign5070_e5226: f64 = if locals.var_cjci0_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign5070_e5226;

        let (assign5080_e5242, assign5080_e5242_d_n4,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5080_e5234: f64 = (locals.var_ajci_t).ln();
        let assign5080_e5235: f64 = (-assign5080_e5234);
        let assign5080_e5237: f64 = (assign5080_e5235 / p.p49);
        let assign5080_e5238: f64 = (assign5080_e5237).exp();
        let assign5080_e5239: f64 = (1.0 - assign5080_e5238);
        let assign5080_e5240: f64 = (locals.var_vdci_t * assign5080_e5239);
        (assign5080_e5240, ((locals.var_vdci_t_dn4 * assign5080_e5239) + (locals.var_vdci_t * (-(assign5080_e5238 * ((-(locals.var_ajci_t_dn4 / locals.var_ajci_t)) / p.p49))))),)
    } else {
        (locals.var_dfv_f, locals.var_dfv_f_dn4,)
    }
};
        locals.var_dfv_f = assign5080_e5242;
        locals.var_dfv_f_dn4 = assign5080_e5242_d_n4;

        let (assign5090_e5253, assign5090_e5253_d_n0, assign5090_e5253_d_n1, assign5090_e5253_d_n3, assign5090_e5253_d_n4, assign5090_e5253_d_n5, assign5090_e5253_d_n6, assign5090_e5253_d_n7, assign5090_e5253_d_n8, assign5090_e5253_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5090_e5249: f64 = (locals.var_dfv_f - locals.var_vbici);
        let assign5090_e5251: f64 = (assign5090_e5249 * locals.var_ovt);
        (assign5090_e5251, 0.0, 0.0, 0.0, ((locals.var_dfv_f_dn4 * locals.var_ovt) + (assign5090_e5249 * locals.var_ovt_dn4)), ((-locals.var_vbici_dn5) * locals.var_ovt), 0.0, 0.0, ((-locals.var_vbici_dn8) * locals.var_ovt), 0.0,)
    } else {
        (locals.var_dfx, locals.var_dfx_dn0, locals.var_dfx_dn1, locals.var_dfx_dn3, locals.var_dfx_dn4, locals.var_dfx_dn5, locals.var_dfx_dn6, locals.var_dfx_dn7, locals.var_dfx_dn8, locals.var_dfx_dn9,)
    }
};
        locals.var_dfx = assign5090_e5253;
        locals.var_dfx_dn0 = assign5090_e5253_d_n0;
        locals.var_dfx_dn1 = assign5090_e5253_d_n1;
        locals.var_dfx_dn3 = assign5090_e5253_d_n3;
        locals.var_dfx_dn4 = assign5090_e5253_d_n4;
        locals.var_dfx_dn5 = assign5090_e5253_d_n5;
        locals.var_dfx_dn6 = assign5090_e5253_d_n6;
        locals.var_dfx_dn7 = assign5090_e5253_d_n7;
        locals.var_dfx_dn8 = assign5090_e5253_d_n8;
        locals.var_dfx_dn9 = assign5090_e5253_d_n9;

        let (assign5100_e5265, assign5100_e5265_d_n0, assign5100_e5265_d_n1, assign5100_e5265_d_n3, assign5100_e5265_d_n4, assign5100_e5265_d_n5, assign5100_e5265_d_n6, assign5100_e5265_d_n7, assign5100_e5265_d_n8, assign5100_e5265_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5100_e5260: f64 = (locals.var_dfx * locals.var_dfx);
        let assign5100_e5262: f64 = (assign5100_e5260 + 1.921812);
        let assign5100_e5263: f64 = (assign5100_e5262).sqrt();
        (assign5100_e5263, (((locals.var_dfx_dn0 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn0)) / (2.0 * assign5100_e5263)), (((locals.var_dfx_dn1 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn1)) / (2.0 * assign5100_e5263)), (((locals.var_dfx_dn3 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn3)) / (2.0 * assign5100_e5263)), (((locals.var_dfx_dn4 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn4)) / (2.0 * assign5100_e5263)), (((locals.var_dfx_dn5 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn5)) / (2.0 * assign5100_e5263)), (((locals.var_dfx_dn6 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn6)) / (2.0 * assign5100_e5263)), (((locals.var_dfx_dn7 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn7)) / (2.0 * assign5100_e5263)), (((locals.var_dfx_dn8 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn8)) / (2.0 * assign5100_e5263)), (((locals.var_dfx_dn9 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn9)) / (2.0 * assign5100_e5263)),)
    } else {
        (locals.var_dfs_q, locals.var_dfs_q_dn0, locals.var_dfs_q_dn1, locals.var_dfs_q_dn3, locals.var_dfs_q_dn4, locals.var_dfs_q_dn5, locals.var_dfs_q_dn6, locals.var_dfs_q_dn7, locals.var_dfs_q_dn8, locals.var_dfs_q_dn9,)
    }
};
        locals.var_dfs_q = assign5100_e5265;
        locals.var_dfs_q_dn0 = assign5100_e5265_d_n0;
        locals.var_dfs_q_dn1 = assign5100_e5265_d_n1;
        locals.var_dfs_q_dn3 = assign5100_e5265_d_n3;
        locals.var_dfs_q_dn4 = assign5100_e5265_d_n4;
        locals.var_dfs_q_dn5 = assign5100_e5265_d_n5;
        locals.var_dfs_q_dn6 = assign5100_e5265_d_n6;
        locals.var_dfs_q_dn7 = assign5100_e5265_d_n7;
        locals.var_dfs_q_dn8 = assign5100_e5265_d_n8;
        locals.var_dfs_q_dn9 = assign5100_e5265_d_n9;

        let (assign5110_e5276, assign5110_e5276_d_n0, assign5110_e5276_d_n1, assign5110_e5276_d_n3, assign5110_e5276_d_n4, assign5110_e5276_d_n5, assign5110_e5276_d_n6, assign5110_e5276_d_n7, assign5110_e5276_d_n8, assign5110_e5276_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5110_e5272: f64 = (locals.var_dfx + locals.var_dfs_q);
        let assign5110_e5274: f64 = (assign5110_e5272 * 0.5);
        (assign5110_e5274, ((locals.var_dfx_dn0 + locals.var_dfs_q_dn0) * 0.5), ((locals.var_dfx_dn1 + locals.var_dfs_q_dn1) * 0.5), ((locals.var_dfx_dn3 + locals.var_dfs_q_dn3) * 0.5), ((locals.var_dfx_dn4 + locals.var_dfs_q_dn4) * 0.5), ((locals.var_dfx_dn5 + locals.var_dfs_q_dn5) * 0.5), ((locals.var_dfx_dn6 + locals.var_dfs_q_dn6) * 0.5), ((locals.var_dfx_dn7 + locals.var_dfs_q_dn7) * 0.5), ((locals.var_dfx_dn8 + locals.var_dfs_q_dn8) * 0.5), ((locals.var_dfx_dn9 + locals.var_dfs_q_dn9) * 0.5),)
    } else {
        (locals.var_dfs_q2, locals.var_dfs_q2_dn0, locals.var_dfs_q2_dn1, locals.var_dfs_q2_dn3, locals.var_dfs_q2_dn4, locals.var_dfs_q2_dn5, locals.var_dfs_q2_dn6, locals.var_dfs_q2_dn7, locals.var_dfs_q2_dn8, locals.var_dfs_q2_dn9,)
    }
};
        locals.var_dfs_q2 = assign5110_e5276;
        locals.var_dfs_q2_dn0 = assign5110_e5276_d_n0;
        locals.var_dfs_q2_dn1 = assign5110_e5276_d_n1;
        locals.var_dfs_q2_dn3 = assign5110_e5276_d_n3;
        locals.var_dfs_q2_dn4 = assign5110_e5276_d_n4;
        locals.var_dfs_q2_dn5 = assign5110_e5276_d_n5;
        locals.var_dfs_q2_dn6 = assign5110_e5276_d_n6;
        locals.var_dfs_q2_dn7 = assign5110_e5276_d_n7;
        locals.var_dfs_q2_dn8 = assign5110_e5276_d_n8;
        locals.var_dfs_q2_dn9 = assign5110_e5276_d_n9;

        let (assign5120_e5287, assign5120_e5287_d_n0, assign5120_e5287_d_n1, assign5120_e5287_d_n3, assign5120_e5287_d_n4, assign5120_e5287_d_n5, assign5120_e5287_d_n6, assign5120_e5287_d_n7, assign5120_e5287_d_n8, assign5120_e5287_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5120_e5284: f64 = (locals.var_vt * locals.var_dfs_q2);
        let assign5120_e5285: f64 = (locals.var_dfv_f - assign5120_e5284);
        (assign5120_e5285, (-(locals.var_vt * locals.var_dfs_q2_dn0)), (-(locals.var_vt * locals.var_dfs_q2_dn1)), (-(locals.var_vt * locals.var_dfs_q2_dn3)), (locals.var_dfv_f_dn4 - ((locals.var_vt_dn4 * locals.var_dfs_q2) + (locals.var_vt * locals.var_dfs_q2_dn4))), (-(locals.var_vt * locals.var_dfs_q2_dn5)), (-(locals.var_vt * locals.var_dfs_q2_dn6)), (-(locals.var_vt * locals.var_dfs_q2_dn7)), (-(locals.var_vt * locals.var_dfs_q2_dn8)), (-(locals.var_vt * locals.var_dfs_q2_dn9)),)
    } else {
        (locals.var_dfv_j, locals.var_dfv_j_dn0, locals.var_dfv_j_dn1, locals.var_dfv_j_dn3, locals.var_dfv_j_dn4, locals.var_dfv_j_dn5, locals.var_dfv_j_dn6, locals.var_dfv_j_dn7, locals.var_dfv_j_dn8, locals.var_dfv_j_dn9,)
    }
};
        locals.var_dfv_j = assign5120_e5287;
        locals.var_dfv_j_dn0 = assign5120_e5287_d_n0;
        locals.var_dfv_j_dn1 = assign5120_e5287_d_n1;
        locals.var_dfv_j_dn3 = assign5120_e5287_d_n3;
        locals.var_dfv_j_dn4 = assign5120_e5287_d_n4;
        locals.var_dfv_j_dn5 = assign5120_e5287_d_n5;
        locals.var_dfv_j_dn6 = assign5120_e5287_d_n6;
        locals.var_dfv_j_dn7 = assign5120_e5287_d_n7;
        locals.var_dfv_j_dn8 = assign5120_e5287_d_n8;
        locals.var_dfv_j_dn9 = assign5120_e5287_d_n9;

        let (assign5130_e5296, assign5130_e5296_d_n0, assign5130_e5296_d_n1, assign5130_e5296_d_n3, assign5130_e5296_d_n4, assign5130_e5296_d_n5, assign5130_e5296_d_n6, assign5130_e5296_d_n7, assign5130_e5296_d_n8, assign5130_e5296_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5130_e5294: f64 = (locals.var_dfs_q2 / locals.var_dfs_q);
        (assign5130_e5294, (((locals.var_dfs_q2_dn0 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn0)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn1 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn1)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn3 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn3)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn4 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn4)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn5 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn5)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn6 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn6)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn7 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn7)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn8 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn8)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn9 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn9)) / (locals.var_dfs_q * locals.var_dfs_q)),)
    } else {
        (locals.var_dfdvj_dv, locals.var_dfdvj_dv_dn0, locals.var_dfdvj_dv_dn1, locals.var_dfdvj_dv_dn3, locals.var_dfdvj_dv_dn4, locals.var_dfdvj_dv_dn5, locals.var_dfdvj_dv_dn6, locals.var_dfdvj_dv_dn7, locals.var_dfdvj_dv_dn8, locals.var_dfdvj_dv_dn9,)
    }
};
        locals.var_dfdvj_dv = assign5130_e5296;
        locals.var_dfdvj_dv_dn0 = assign5130_e5296_d_n0;
        locals.var_dfdvj_dv_dn1 = assign5130_e5296_d_n1;
        locals.var_dfdvj_dv_dn3 = assign5130_e5296_d_n3;
        locals.var_dfdvj_dv_dn4 = assign5130_e5296_d_n4;
        locals.var_dfdvj_dv_dn5 = assign5130_e5296_d_n5;
        locals.var_dfdvj_dv_dn6 = assign5130_e5296_d_n6;
        locals.var_dfdvj_dv_dn7 = assign5130_e5296_d_n7;
        locals.var_dfdvj_dv_dn8 = assign5130_e5296_d_n8;
        locals.var_dfdvj_dv_dn9 = assign5130_e5296_d_n9;

        let (assign5140_e5308, assign5140_e5308_d_n0, assign5140_e5308_d_n1, assign5140_e5308_d_n3, assign5140_e5308_d_n4, assign5140_e5308_d_n5, assign5140_e5308_d_n6, assign5140_e5308_d_n7, assign5140_e5308_d_n8, assign5140_e5308_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5140_e5304: f64 = (locals.var_dfv_j / locals.var_vdci_t);
        let assign5140_e5305: f64 = (1.0 - assign5140_e5304);
        let assign5140_e5306: f64 = (assign5140_e5305).ln();
        (assign5140_e5306, ((-(locals.var_dfv_j_dn0 / locals.var_vdci_t)) / assign5140_e5305), ((-(locals.var_dfv_j_dn1 / locals.var_vdci_t)) / assign5140_e5305), ((-(locals.var_dfv_j_dn3 / locals.var_vdci_t)) / assign5140_e5305), ((-(((locals.var_dfv_j_dn4 * locals.var_vdci_t) - (locals.var_dfv_j * locals.var_vdci_t_dn4)) / (locals.var_vdci_t * locals.var_vdci_t))) / assign5140_e5305), ((-(locals.var_dfv_j_dn5 / locals.var_vdci_t)) / assign5140_e5305), ((-(locals.var_dfv_j_dn6 / locals.var_vdci_t)) / assign5140_e5305), ((-(locals.var_dfv_j_dn7 / locals.var_vdci_t)) / assign5140_e5305), ((-(locals.var_dfv_j_dn8 / locals.var_vdci_t)) / assign5140_e5305), ((-(locals.var_dfv_j_dn9 / locals.var_vdci_t)) / assign5140_e5305),)
    } else {
        (locals.var_dfb, locals.var_dfb_dn0, locals.var_dfb_dn1, locals.var_dfb_dn3, locals.var_dfb_dn4, locals.var_dfb_dn5, locals.var_dfb_dn6, locals.var_dfb_dn7, locals.var_dfb_dn8, locals.var_dfb_dn9,)
    }
};
        locals.var_dfb = assign5140_e5308;
        locals.var_dfb_dn0 = assign5140_e5308_d_n0;
        locals.var_dfb_dn1 = assign5140_e5308_d_n1;
        locals.var_dfb_dn3 = assign5140_e5308_d_n3;
        locals.var_dfb_dn4 = assign5140_e5308_d_n4;
        locals.var_dfb_dn5 = assign5140_e5308_d_n5;
        locals.var_dfb_dn6 = assign5140_e5308_d_n6;
        locals.var_dfb_dn7 = assign5140_e5308_d_n7;
        locals.var_dfb_dn8 = assign5140_e5308_d_n8;
        locals.var_dfb_dn9 = assign5140_e5308_d_n9;

        let (assign5150_e5321, assign5150_e5321_d_n0, assign5150_e5321_d_n1, assign5150_e5321_d_n3, assign5150_e5321_d_n4, assign5150_e5321_d_n5, assign5150_e5321_d_n6, assign5150_e5321_d_n7, assign5150_e5321_d_n8, assign5150_e5321_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5150_e5314: f64 = (-p.p49);
        let assign5150_e5316: f64 = (assign5150_e5314 * locals.var_dfb);
        let assign5150_e5317: f64 = (assign5150_e5316).exp();
        let assign5150_e5319: f64 = (assign5150_e5317 * locals.var_dfdvj_dv);
        (assign5150_e5319, (((assign5150_e5317 * (assign5150_e5314 * locals.var_dfb_dn0)) * locals.var_dfdvj_dv) + (assign5150_e5317 * locals.var_dfdvj_dv_dn0)), (((assign5150_e5317 * (assign5150_e5314 * locals.var_dfb_dn1)) * locals.var_dfdvj_dv) + (assign5150_e5317 * locals.var_dfdvj_dv_dn1)), (((assign5150_e5317 * (assign5150_e5314 * locals.var_dfb_dn3)) * locals.var_dfdvj_dv) + (assign5150_e5317 * locals.var_dfdvj_dv_dn3)), (((assign5150_e5317 * (assign5150_e5314 * locals.var_dfb_dn4)) * locals.var_dfdvj_dv) + (assign5150_e5317 * locals.var_dfdvj_dv_dn4)), (((assign5150_e5317 * (assign5150_e5314 * locals.var_dfb_dn5)) * locals.var_dfdvj_dv) + (assign5150_e5317 * locals.var_dfdvj_dv_dn5)), (((assign5150_e5317 * (assign5150_e5314 * locals.var_dfb_dn6)) * locals.var_dfdvj_dv) + (assign5150_e5317 * locals.var_dfdvj_dv_dn6)), (((assign5150_e5317 * (assign5150_e5314 * locals.var_dfb_dn7)) * locals.var_dfdvj_dv) + (assign5150_e5317 * locals.var_dfdvj_dv_dn7)), (((assign5150_e5317 * (assign5150_e5314 * locals.var_dfb_dn8)) * locals.var_dfdvj_dv) + (assign5150_e5317 * locals.var_dfdvj_dv_dn8)), (((assign5150_e5317 * (assign5150_e5314 * locals.var_dfb_dn9)) * locals.var_dfdvj_dv) + (assign5150_e5317 * locals.var_dfdvj_dv_dn9)),)
    } else {
        (locals.var_dfc_j1, locals.var_dfc_j1_dn0, locals.var_dfc_j1_dn1, locals.var_dfc_j1_dn3, locals.var_dfc_j1_dn4, locals.var_dfc_j1_dn5, locals.var_dfc_j1_dn6, locals.var_dfc_j1_dn7, locals.var_dfc_j1_dn8, locals.var_dfc_j1_dn9,)
    }
};
        locals.var_dfc_j1 = assign5150_e5321;
        locals.var_dfc_j1_dn0 = assign5150_e5321_d_n0;
        locals.var_dfc_j1_dn1 = assign5150_e5321_d_n1;
        locals.var_dfc_j1_dn3 = assign5150_e5321_d_n3;
        locals.var_dfc_j1_dn4 = assign5150_e5321_d_n4;
        locals.var_dfc_j1_dn5 = assign5150_e5321_d_n5;
        locals.var_dfc_j1_dn6 = assign5150_e5321_d_n6;
        locals.var_dfc_j1_dn7 = assign5150_e5321_d_n7;
        locals.var_dfc_j1_dn8 = assign5150_e5321_d_n8;
        locals.var_dfc_j1_dn9 = assign5150_e5321_d_n9;

        let (assign5160_e5336, assign5160_e5336_d_n0, assign5160_e5336_d_n1, assign5160_e5336_d_n3, assign5160_e5336_d_n4, assign5160_e5336_d_n5, assign5160_e5336_d_n6, assign5160_e5336_d_n7, assign5160_e5336_d_n8, assign5160_e5336_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5160_e5331: f64 = (1.0 - locals.var_dfdvj_dv);
        let assign5160_e5332: f64 = (locals.var_ajci_t * assign5160_e5331);
        let assign5160_e5333: f64 = (locals.var_dfc_j1 + assign5160_e5332);
        let assign5160_e5334: f64 = (locals.var_cjci0_t * assign5160_e5333);
        (assign5160_e5334, (locals.var_cjci0_t * (locals.var_dfc_j1_dn0 + (locals.var_ajci_t * (-locals.var_dfdvj_dv_dn0)))), (locals.var_cjci0_t * (locals.var_dfc_j1_dn1 + (locals.var_ajci_t * (-locals.var_dfdvj_dv_dn1)))), (locals.var_cjci0_t * (locals.var_dfc_j1_dn3 + (locals.var_ajci_t * (-locals.var_dfdvj_dv_dn3)))), ((locals.var_cjci0_t_dn4 * assign5160_e5333) + (locals.var_cjci0_t * (locals.var_dfc_j1_dn4 + ((locals.var_ajci_t_dn4 * assign5160_e5331) + (locals.var_ajci_t * (-locals.var_dfdvj_dv_dn4)))))), (locals.var_cjci0_t * (locals.var_dfc_j1_dn5 + (locals.var_ajci_t * (-locals.var_dfdvj_dv_dn5)))), (locals.var_cjci0_t * (locals.var_dfc_j1_dn6 + (locals.var_ajci_t * (-locals.var_dfdvj_dv_dn6)))), (locals.var_cjci0_t * (locals.var_dfc_j1_dn7 + (locals.var_ajci_t * (-locals.var_dfdvj_dv_dn7)))), (locals.var_cjci0_t * (locals.var_dfc_j1_dn8 + (locals.var_ajci_t * (-locals.var_dfdvj_dv_dn8)))), (locals.var_cjci0_t * (locals.var_dfc_j1_dn9 + (locals.var_ajci_t * (-locals.var_dfdvj_dv_dn9)))),)
    } else {
        (locals.var_cjci, locals.var_cjci_dn0, locals.var_cjci_dn1, locals.var_cjci_dn3, locals.var_cjci_dn4, locals.var_cjci_dn5, locals.var_cjci_dn6, locals.var_cjci_dn7, locals.var_cjci_dn8, locals.var_cjci_dn9,)
    }
};
        locals.var_cjci = assign5160_e5336;
        locals.var_cjci_dn0 = assign5160_e5336_d_n0;
        locals.var_cjci_dn1 = assign5160_e5336_d_n1;
        locals.var_cjci_dn3 = assign5160_e5336_d_n3;
        locals.var_cjci_dn4 = assign5160_e5336_d_n4;
        locals.var_cjci_dn5 = assign5160_e5336_d_n5;
        locals.var_cjci_dn6 = assign5160_e5336_d_n6;
        locals.var_cjci_dn7 = assign5160_e5336_d_n7;
        locals.var_cjci_dn8 = assign5160_e5336_d_n8;
        locals.var_cjci_dn9 = assign5160_e5336_d_n9;

        let (assign5170_e5356, assign5170_e5356_d_n0, assign5170_e5356_d_n1, assign5170_e5356_d_n3, assign5170_e5356_d_n4, assign5170_e5356_d_n5, assign5170_e5356_d_n6, assign5170_e5356_d_n7, assign5170_e5356_d_n8, assign5170_e5356_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5170_e5346: f64 = (1.0 - p.p49);
        let assign5170_e5347: f64 = (locals.var_dfb * assign5170_e5346);
        let assign5170_e5348: f64 = (assign5170_e5347).exp();
        let assign5170_e5349: f64 = (1.0 - assign5170_e5348);
        let assign5170_e5350: f64 = (locals.var_vdci_t * assign5170_e5349);
        let assign5170_e5353: f64 = (1.0 - p.p49);
        let assign5170_e5354: f64 = (assign5170_e5350 / assign5170_e5353);
        (assign5170_e5354, ((locals.var_vdci_t * (-(assign5170_e5348 * (locals.var_dfb_dn0 * assign5170_e5346)))) / assign5170_e5353), ((locals.var_vdci_t * (-(assign5170_e5348 * (locals.var_dfb_dn1 * assign5170_e5346)))) / assign5170_e5353), ((locals.var_vdci_t * (-(assign5170_e5348 * (locals.var_dfb_dn3 * assign5170_e5346)))) / assign5170_e5353), (((locals.var_vdci_t_dn4 * assign5170_e5349) + (locals.var_vdci_t * (-(assign5170_e5348 * (locals.var_dfb_dn4 * assign5170_e5346))))) / assign5170_e5353), ((locals.var_vdci_t * (-(assign5170_e5348 * (locals.var_dfb_dn5 * assign5170_e5346)))) / assign5170_e5353), ((locals.var_vdci_t * (-(assign5170_e5348 * (locals.var_dfb_dn6 * assign5170_e5346)))) / assign5170_e5353), ((locals.var_vdci_t * (-(assign5170_e5348 * (locals.var_dfb_dn7 * assign5170_e5346)))) / assign5170_e5353), ((locals.var_vdci_t * (-(assign5170_e5348 * (locals.var_dfb_dn8 * assign5170_e5346)))) / assign5170_e5353), ((locals.var_vdci_t * (-(assign5170_e5348 * (locals.var_dfb_dn9 * assign5170_e5346)))) / assign5170_e5353),)
    } else {
        (locals.var_dfq_j1, locals.var_dfq_j1_dn0, locals.var_dfq_j1_dn1, locals.var_dfq_j1_dn3, locals.var_dfq_j1_dn4, locals.var_dfq_j1_dn5, locals.var_dfq_j1_dn6, locals.var_dfq_j1_dn7, locals.var_dfq_j1_dn8, locals.var_dfq_j1_dn9,)
    }
};
        locals.var_dfq_j1 = assign5170_e5356;
        locals.var_dfq_j1_dn0 = assign5170_e5356_d_n0;
        locals.var_dfq_j1_dn1 = assign5170_e5356_d_n1;
        locals.var_dfq_j1_dn3 = assign5170_e5356_d_n3;
        locals.var_dfq_j1_dn4 = assign5170_e5356_d_n4;
        locals.var_dfq_j1_dn5 = assign5170_e5356_d_n5;
        locals.var_dfq_j1_dn6 = assign5170_e5356_d_n6;
        locals.var_dfq_j1_dn7 = assign5170_e5356_d_n7;
        locals.var_dfq_j1_dn8 = assign5170_e5356_d_n8;
        locals.var_dfq_j1_dn9 = assign5170_e5356_d_n9;

        let (assign5180_e5371, assign5180_e5371_d_n0, assign5180_e5371_d_n1, assign5180_e5371_d_n3, assign5180_e5371_d_n4, assign5180_e5371_d_n5, assign5180_e5371_d_n6, assign5180_e5371_d_n7, assign5180_e5371_d_n8, assign5180_e5371_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5180_e5366: f64 = (locals.var_vbici - locals.var_dfv_j);
        let assign5180_e5367: f64 = (locals.var_ajci_t * assign5180_e5366);
        let assign5180_e5368: f64 = (locals.var_dfq_j1 + assign5180_e5367);
        let assign5180_e5369: f64 = (locals.var_cjci0_t * assign5180_e5368);
        (assign5180_e5369, (locals.var_cjci0_t * (locals.var_dfq_j1_dn0 + (locals.var_ajci_t * (-locals.var_dfv_j_dn0)))), (locals.var_cjci0_t * (locals.var_dfq_j1_dn1 + (locals.var_ajci_t * (-locals.var_dfv_j_dn1)))), (locals.var_cjci0_t * (locals.var_dfq_j1_dn3 + (locals.var_ajci_t * (-locals.var_dfv_j_dn3)))), ((locals.var_cjci0_t_dn4 * assign5180_e5368) + (locals.var_cjci0_t * (locals.var_dfq_j1_dn4 + ((locals.var_ajci_t_dn4 * assign5180_e5366) + (locals.var_ajci_t * (-locals.var_dfv_j_dn4)))))), (locals.var_cjci0_t * (locals.var_dfq_j1_dn5 + (locals.var_ajci_t * (locals.var_vbici_dn5 - locals.var_dfv_j_dn5)))), (locals.var_cjci0_t * (locals.var_dfq_j1_dn6 + (locals.var_ajci_t * (-locals.var_dfv_j_dn6)))), (locals.var_cjci0_t * (locals.var_dfq_j1_dn7 + (locals.var_ajci_t * (-locals.var_dfv_j_dn7)))), (locals.var_cjci0_t * (locals.var_dfq_j1_dn8 + (locals.var_ajci_t * (locals.var_vbici_dn8 - locals.var_dfv_j_dn8)))), (locals.var_cjci0_t * (locals.var_dfq_j1_dn9 + (locals.var_ajci_t * (-locals.var_dfv_j_dn9)))),)
    } else {
        (locals.var_qjci, locals.var_qjci_dn0, locals.var_qjci_dn1, locals.var_qjci_dn3, locals.var_qjci_dn4, locals.var_qjci_dn5, locals.var_qjci_dn6, locals.var_qjci_dn7, locals.var_qjci_dn8, locals.var_qjci_dn9,)
    }
};
        locals.var_qjci = assign5180_e5371;
        locals.var_qjci_dn0 = assign5180_e5371_d_n0;
        locals.var_qjci_dn1 = assign5180_e5371_d_n1;
        locals.var_qjci_dn3 = assign5180_e5371_d_n3;
        locals.var_qjci_dn4 = assign5180_e5371_d_n4;
        locals.var_qjci_dn5 = assign5180_e5371_d_n5;
        locals.var_qjci_dn6 = assign5180_e5371_d_n6;
        locals.var_qjci_dn7 = assign5180_e5371_d_n7;
        locals.var_qjci_dn8 = assign5180_e5371_d_n8;
        locals.var_qjci_dn9 = assign5180_e5371_d_n9;

        let (assign5190_e5379, assign5190_e5379_d_n0, assign5190_e5379_d_n1, assign5190_e5379_d_n3, assign5190_e5379_d_n4, assign5190_e5379_d_n5, assign5190_e5379_d_n6, assign5190_e5379_d_n7, assign5190_e5379_d_n8, assign5190_e5379_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cjci, locals.var_cjci_dn0, locals.var_cjci_dn1, locals.var_cjci_dn3, locals.var_cjci_dn4, locals.var_cjci_dn5, locals.var_cjci_dn6, locals.var_cjci_dn7, locals.var_cjci_dn8, locals.var_cjci_dn9,)
    }
};
        locals.var_cjci = assign5190_e5379;
        locals.var_cjci_dn0 = assign5190_e5379_d_n0;
        locals.var_cjci_dn1 = assign5190_e5379_d_n1;
        locals.var_cjci_dn3 = assign5190_e5379_d_n3;
        locals.var_cjci_dn4 = assign5190_e5379_d_n4;
        locals.var_cjci_dn5 = assign5190_e5379_d_n5;
        locals.var_cjci_dn6 = assign5190_e5379_d_n6;
        locals.var_cjci_dn7 = assign5190_e5379_d_n7;
        locals.var_cjci_dn8 = assign5190_e5379_d_n8;
        locals.var_cjci_dn9 = assign5190_e5379_d_n9;

        let (assign5200_e5387, assign5200_e5387_d_n0, assign5200_e5387_d_n1, assign5200_e5387_d_n3, assign5200_e5387_d_n4, assign5200_e5387_d_n5, assign5200_e5387_d_n6, assign5200_e5387_d_n7, assign5200_e5387_d_n8, assign5200_e5387_d_n9,) = {
    if ((locals.var_guard94 == 0.0) && (locals.var_guard98 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjci, locals.var_qjci_dn0, locals.var_qjci_dn1, locals.var_qjci_dn3, locals.var_qjci_dn4, locals.var_qjci_dn5, locals.var_qjci_dn6, locals.var_qjci_dn7, locals.var_qjci_dn8, locals.var_qjci_dn9,)
    }
};
        locals.var_qjci = assign5200_e5387;
        locals.var_qjci_dn0 = assign5200_e5387_d_n0;
        locals.var_qjci_dn1 = assign5200_e5387_d_n1;
        locals.var_qjci_dn3 = assign5200_e5387_d_n3;
        locals.var_qjci_dn4 = assign5200_e5387_d_n4;
        locals.var_qjci_dn5 = assign5200_e5387_d_n5;
        locals.var_qjci_dn6 = assign5200_e5387_d_n6;
        locals.var_qjci_dn7 = assign5200_e5387_d_n7;
        locals.var_qjci_dn8 = assign5200_e5387_d_n8;
        locals.var_qjci_dn9 = assign5200_e5387_d_n9;

        let assign5210_e5390: f64 = if p.p10 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign5210_e5390;

        let (assign5220_e5396, assign5220_e5396_d_n4,) = {
    if (locals.var_guard99 != 0.0) {
        let assign5220_e5394: f64 = (p.p11 * locals.var_vt);
        (assign5220_e5394, (p.p11 * locals.var_vt_dn4),)
    } else {
        (locals.var_hje_rvt, locals.var_hje_rvt_dn4,)
    }
};
        locals.var_hje_rvt = assign5220_e5396;
        locals.var_hje_rvt_dn4 = assign5220_e5396_d_n4;

        let (assign5230_e5404, assign5230_e5404_d_n4, assign5230_e5404_d_n6, assign5230_e5404_d_n8,) = {
    if (locals.var_guard99 != 0.0) {
        let assign5230_e5400: f64 = (locals.var_vdei_t - locals.var_vbiei);
        let assign5230_e5402: f64 = (assign5230_e5400 / locals.var_hje_rvt);
        (assign5230_e5402, (((locals.var_vdei_t_dn4 * locals.var_hje_rvt) - (assign5230_e5400 * locals.var_hje_rvt_dn4)) / (locals.var_hje_rvt * locals.var_hje_rvt)), ((-locals.var_vbiei_dn6) / locals.var_hje_rvt), ((-locals.var_vbiei_dn8) / locals.var_hje_rvt),)
    } else {
        (locals.var_hje_vr, locals.var_hje_vr_dn4, locals.var_hje_vr_dn6, locals.var_hje_vr_dn8,)
    }
};
        locals.var_hje_vr = assign5230_e5404;
        locals.var_hje_vr_dn4 = assign5230_e5404_d_n4;
        locals.var_hje_vr_dn6 = assign5230_e5404_d_n6;
        locals.var_hje_vr_dn8 = assign5230_e5404_d_n8;

        let (assign5240_e5421, assign5240_e5421_d_n4, assign5240_e5421_d_n6, assign5240_e5421_d_n8,) = {
    if (locals.var_guard99 != 0.0) {
        let assign5240_e5411: f64 = (locals.var_hje_vr * locals.var_hje_vr);
        let assign5240_e5413: f64 = (assign5240_e5411 + 1.921812);
        let assign5240_e5414: f64 = (assign5240_e5413).sqrt();
        let assign5240_e5415: f64 = (locals.var_hje_vr + assign5240_e5414);
        let assign5240_e5416: f64 = (locals.var_hje_rvt * assign5240_e5415);
        let assign5240_e5418: f64 = (assign5240_e5416 * 0.5);
        let assign5240_e5419: f64 = (locals.var_vdei_t - assign5240_e5418);
        (assign5240_e5419, (locals.var_vdei_t_dn4 - (((locals.var_hje_rvt_dn4 * assign5240_e5415) + (locals.var_hje_rvt * (locals.var_hje_vr_dn4 + (((locals.var_hje_vr_dn4 * locals.var_hje_vr) + (locals.var_hje_vr * locals.var_hje_vr_dn4)) / (2.0 * assign5240_e5414))))) * 0.5)), (-((locals.var_hje_rvt * (locals.var_hje_vr_dn6 + (((locals.var_hje_vr_dn6 * locals.var_hje_vr) + (locals.var_hje_vr * locals.var_hje_vr_dn6)) / (2.0 * assign5240_e5414)))) * 0.5)), (-((locals.var_hje_rvt * (locals.var_hje_vr_dn8 + (((locals.var_hje_vr_dn8 * locals.var_hje_vr) + (locals.var_hje_vr * locals.var_hje_vr_dn8)) / (2.0 * assign5240_e5414)))) * 0.5)),)
    } else {
        (locals.var_hje_vju, locals.var_hje_vju_dn4, locals.var_hje_vju_dn6, locals.var_hje_vju_dn8,)
    }
};
        locals.var_hje_vju = assign5240_e5421;
        locals.var_hje_vju_dn4 = assign5240_e5421_d_n4;
        locals.var_hje_vju_dn6 = assign5240_e5421_d_n6;
        locals.var_hje_vju_dn8 = assign5240_e5421_d_n8;

        let (assign5250_e5437, assign5250_e5437_d_n4, assign5250_e5437_d_n6, assign5250_e5437_d_n8,) = {
    if (locals.var_guard99 != 0.0) {
        let assign5250_e5429: f64 = (locals.var_hje_vju / locals.var_vdei_t);
        let assign5250_e5430: f64 = (1.0 - assign5250_e5429);
        let assign5250_e5431: f64 = (assign5250_e5430).ln();
        let assign5250_e5432: f64 = (p.p41 * assign5250_e5431);
        let assign5250_e5433: f64 = (assign5250_e5432).exp();
        let assign5250_e5434: f64 = (1.0 - assign5250_e5433);
        let assign5250_e5435: f64 = (locals.var_ahjei_t * assign5250_e5434);
        (assign5250_e5435, ((locals.var_ahjei_t_dn4 * assign5250_e5434) + (locals.var_ahjei_t * (-(assign5250_e5433 * (p.p41 * ((-(((locals.var_hje_vju_dn4 * locals.var_vdei_t) - (locals.var_hje_vju * locals.var_vdei_t_dn4)) / (locals.var_vdei_t * locals.var_vdei_t))) / assign5250_e5430)))))), (locals.var_ahjei_t * (-(assign5250_e5433 * (p.p41 * ((-(locals.var_hje_vju_dn6 / locals.var_vdei_t)) / assign5250_e5430))))), (locals.var_ahjei_t * (-(assign5250_e5433 * (p.p41 * ((-(locals.var_hje_vju_dn8 / locals.var_vdei_t)) / assign5250_e5430))))),)
    } else {
        (locals.var_hje_u, locals.var_hje_u_dn4, locals.var_hje_u_dn6, locals.var_hje_u_dn8,)
    }
};
        locals.var_hje_u = assign5250_e5437;
        locals.var_hje_u_dn4 = assign5250_e5437_d_n4;
        locals.var_hje_u_dn6 = assign5250_e5437_d_n6;
        locals.var_hje_u_dn8 = assign5250_e5437_d_n8;

        let assign5260_e5439: f64 = (locals.var_hje_u).abs();
        let assign5260_e5441: f64 = if assign5260_e5439 > 0.001 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign5260_e5441;

        let (assign5270_e5454, assign5270_e5454_d_n4, assign5270_e5454_d_n6, assign5270_e5454_d_n8,) = {
    if ((locals.var_guard99 != 0.0) && (locals.var_guard104 != 0.0)) {
        let assign5270_e5447: f64 = (locals.var_hje_u).exp();
        let assign5270_e5449: f64 = (assign5270_e5447 - 1.0);
        let assign5270_e5450: f64 = (locals.var_hjei0_t * assign5270_e5449);
        let assign5270_e5452: f64 = (assign5270_e5450 / locals.var_hje_u);
        (assign5270_e5452, (((((locals.var_hjei0_t_dn4 * assign5270_e5449) + (locals.var_hjei0_t * (assign5270_e5447 * locals.var_hje_u_dn4))) * locals.var_hje_u) - (assign5270_e5450 * locals.var_hje_u_dn4)) / (locals.var_hje_u * locals.var_hje_u)), ((((locals.var_hjei0_t * (assign5270_e5447 * locals.var_hje_u_dn6)) * locals.var_hje_u) - (assign5270_e5450 * locals.var_hje_u_dn6)) / (locals.var_hje_u * locals.var_hje_u)), ((((locals.var_hjei0_t * (assign5270_e5447 * locals.var_hje_u_dn8)) * locals.var_hje_u) - (assign5270_e5450 * locals.var_hje_u_dn8)) / (locals.var_hje_u * locals.var_hje_u)),)
    } else {
        (locals.var_hjei_tb, locals.var_hjei_tb_dn4, locals.var_hjei_tb_dn6, locals.var_hjei_tb_dn8,)
    }
};
        locals.var_hjei_tb = assign5270_e5454;
        locals.var_hjei_tb_dn4 = assign5270_e5454_d_n4;
        locals.var_hjei_tb_dn6 = assign5270_e5454_d_n6;
        locals.var_hjei_tb_dn8 = assign5270_e5454_d_n8;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5280_e5467, assign5280_e5467_d_n4, assign5280_e5467_d_n6, assign5280_e5467_d_n8,) = {
    if ((locals.var_guard99 != 0.0) && (locals.var_guard104 == 0.0)) {
        let assign5280_e5463: f64 = (locals.var_hje_u * 0.5);
        let assign5280_e5464: f64 = (1.0 + assign5280_e5463);
        let assign5280_e5465: f64 = (locals.var_hjei0_t * assign5280_e5464);
        (assign5280_e5465, ((locals.var_hjei0_t_dn4 * assign5280_e5464) + (locals.var_hjei0_t * (locals.var_hje_u_dn4 * 0.5))), (locals.var_hjei0_t * (locals.var_hje_u_dn6 * 0.5)), (locals.var_hjei0_t * (locals.var_hje_u_dn8 * 0.5)),)
    } else {
        (locals.var_hjei_tb, locals.var_hjei_tb_dn4, locals.var_hjei_tb_dn6, locals.var_hjei_tb_dn8,)
    }
};
        locals.var_hjei_tb = assign5280_e5467;
        locals.var_hjei_tb_dn4 = assign5280_e5467_d_n4;
        locals.var_hjei_tb_dn6 = assign5280_e5467_d_n6;
        locals.var_hjei_tb_dn8 = assign5280_e5467_d_n8;

        let (assign5290_e5472, assign5290_e5472_d_n4, assign5290_e5472_d_n6, assign5290_e5472_d_n8,) = {
    if (locals.var_guard99 == 0.0) {
        (locals.var_hjei0_t, locals.var_hjei0_t_dn4, 0.0, 0.0,)
    } else {
        (locals.var_hjei_tb, locals.var_hjei_tb_dn4, locals.var_hjei_tb_dn6, locals.var_hjei_tb_dn8,)
    }
};
        locals.var_hjei_tb = assign5290_e5472;
        locals.var_hjei_tb_dn4 = assign5290_e5472_d_n4;
        locals.var_hjei_tb_dn6 = assign5290_e5472_d_n6;
        locals.var_hjei_tb_dn8 = assign5290_e5472_d_n8;

        let assign5300_e5476: f64 = (locals.var_hjei_tb * locals.var_qjei);
        let assign5300_e5477: f64 = (locals.var_qp0_t + assign5300_e5476);
        let assign5300_e5480: f64 = (p.p12 * locals.var_qjci);
        let assign5300_e5481: f64 = (assign5300_e5477 + assign5300_e5480);
        locals.var_q0_pt = assign5300_e5481;
        locals.var_q0_pt_dn0 = ((locals.var_hjei_tb * locals.var_qjei_dn0) + (p.p12 * locals.var_qjci_dn0));
        locals.var_q0_pt_dn1 = ((locals.var_hjei_tb * locals.var_qjei_dn1) + (p.p12 * locals.var_qjci_dn1));
        locals.var_q0_pt_dn3 = ((locals.var_hjei_tb * locals.var_qjei_dn3) + (p.p12 * locals.var_qjci_dn3));
        locals.var_q0_pt_dn4 = ((locals.var_qp0_t_dn4 + ((locals.var_hjei_tb_dn4 * locals.var_qjei) + (locals.var_hjei_tb * locals.var_qjei_dn4))) + (p.p12 * locals.var_qjci_dn4));
        locals.var_q0_pt_dn5 = ((locals.var_hjei_tb * locals.var_qjei_dn5) + (p.p12 * locals.var_qjci_dn5));
        locals.var_q0_pt_dn6 = (((locals.var_hjei_tb_dn6 * locals.var_qjei) + (locals.var_hjei_tb * locals.var_qjei_dn6)) + (p.p12 * locals.var_qjci_dn6));
        locals.var_q0_pt_dn7 = ((locals.var_hjei_tb * locals.var_qjei_dn7) + (p.p12 * locals.var_qjci_dn7));
        locals.var_q0_pt_dn8 = (((locals.var_hjei_tb_dn8 * locals.var_qjei) + (locals.var_hjei_tb * locals.var_qjei_dn8)) + (p.p12 * locals.var_qjci_dn8));
        locals.var_q0_pt_dn9 = ((locals.var_hjei_tb * locals.var_qjei_dn9) + (p.p12 * locals.var_qjci_dn9));

        let assign5310_e5484: f64 = (0.05 * locals.var_qp0_t);
        locals.var_q_bpt = assign5310_e5484;
        locals.var_q_bpt_dn4 = (0.05 * locals.var_qp0_t_dn4);

        let assign5320_e5487: f64 = (locals.var_q0_pt / locals.var_q_bpt);
        let assign5320_e5489: f64 = (assign5320_e5487 - 1.0);
        locals.var_b_q = assign5320_e5489;
        locals.var_b_q_dn0 = (locals.var_q0_pt_dn0 / locals.var_q_bpt);
        locals.var_b_q_dn1 = (locals.var_q0_pt_dn1 / locals.var_q_bpt);
        locals.var_b_q_dn3 = (locals.var_q0_pt_dn3 / locals.var_q_bpt);
        locals.var_b_q_dn4 = (((locals.var_q0_pt_dn4 * locals.var_q_bpt) - (locals.var_q0_pt * locals.var_q_bpt_dn4)) / (locals.var_q_bpt * locals.var_q_bpt));
        locals.var_b_q_dn5 = (locals.var_q0_pt_dn5 / locals.var_q_bpt);
        locals.var_b_q_dn6 = (locals.var_q0_pt_dn6 / locals.var_q_bpt);
        locals.var_b_q_dn7 = (locals.var_q0_pt_dn7 / locals.var_q_bpt);
        locals.var_b_q_dn8 = (locals.var_q0_pt_dn8 / locals.var_q_bpt);
        locals.var_b_q_dn9 = (locals.var_q0_pt_dn9 / locals.var_q_bpt);

        let assign5330_e5495: f64 = (locals.var_b_q * locals.var_b_q);
        let assign5330_e5497: f64 = (assign5330_e5495 + 1.921812);
        let assign5330_e5498: f64 = (assign5330_e5497).sqrt();
        let assign5330_e5499: f64 = (locals.var_b_q + assign5330_e5498);
        let assign5330_e5501: f64 = (assign5330_e5499 * 0.5);
        let assign5330_e5502: f64 = (1.0 + assign5330_e5501);
        let assign5330_e5503: f64 = (locals.var_q_bpt * assign5330_e5502);
        locals.var_q0_pt = assign5330_e5503;
        locals.var_q0_pt_dn0 = (locals.var_q_bpt * ((locals.var_b_q_dn0 + (((locals.var_b_q_dn0 * locals.var_b_q) + (locals.var_b_q * locals.var_b_q_dn0)) / (2.0 * assign5330_e5498))) * 0.5));
        locals.var_q0_pt_dn1 = (locals.var_q_bpt * ((locals.var_b_q_dn1 + (((locals.var_b_q_dn1 * locals.var_b_q) + (locals.var_b_q * locals.var_b_q_dn1)) / (2.0 * assign5330_e5498))) * 0.5));
        locals.var_q0_pt_dn3 = (locals.var_q_bpt * ((locals.var_b_q_dn3 + (((locals.var_b_q_dn3 * locals.var_b_q) + (locals.var_b_q * locals.var_b_q_dn3)) / (2.0 * assign5330_e5498))) * 0.5));
        locals.var_q0_pt_dn4 = ((locals.var_q_bpt_dn4 * assign5330_e5502) + (locals.var_q_bpt * ((locals.var_b_q_dn4 + (((locals.var_b_q_dn4 * locals.var_b_q) + (locals.var_b_q * locals.var_b_q_dn4)) / (2.0 * assign5330_e5498))) * 0.5)));
        locals.var_q0_pt_dn5 = (locals.var_q_bpt * ((locals.var_b_q_dn5 + (((locals.var_b_q_dn5 * locals.var_b_q) + (locals.var_b_q * locals.var_b_q_dn5)) / (2.0 * assign5330_e5498))) * 0.5));
        locals.var_q0_pt_dn6 = (locals.var_q_bpt * ((locals.var_b_q_dn6 + (((locals.var_b_q_dn6 * locals.var_b_q) + (locals.var_b_q * locals.var_b_q_dn6)) / (2.0 * assign5330_e5498))) * 0.5));
        locals.var_q0_pt_dn7 = (locals.var_q_bpt * ((locals.var_b_q_dn7 + (((locals.var_b_q_dn7 * locals.var_b_q) + (locals.var_b_q * locals.var_b_q_dn7)) / (2.0 * assign5330_e5498))) * 0.5));
        locals.var_q0_pt_dn8 = (locals.var_q_bpt * ((locals.var_b_q_dn8 + (((locals.var_b_q_dn8 * locals.var_b_q) + (locals.var_b_q * locals.var_b_q_dn8)) / (2.0 * assign5330_e5498))) * 0.5));
        locals.var_q0_pt_dn9 = (locals.var_q_bpt * ((locals.var_b_q_dn9 + (((locals.var_b_q_dn9 * locals.var_b_q) + (locals.var_b_q * locals.var_b_q_dn9)) / (2.0 * assign5330_e5498))) * 0.5));

        let assign5340_e5507: f64 = (2.4_f64).ln();
        let assign5340_e5508: f64 = (-assign5340_e5507);
        let assign5340_e5510: f64 = (assign5340_e5508 / p.p49);
        let assign5340_e5511: f64 = (assign5340_e5510).exp();
        let assign5340_e5512: f64 = (1.0 - assign5340_e5511);
        let assign5340_e5513: f64 = (locals.var_vdci_t * assign5340_e5512);
        locals.var_cv_f = assign5340_e5513;
        locals.var_cv_f_dn4 = (locals.var_vdci_t_dn4 * assign5340_e5512);

        let assign5350_e5516: f64 = (locals.var_cv_f - locals.var_vbici);
        let assign5350_e5518: f64 = (assign5350_e5516 * locals.var_ovt);
        locals.var_cv_e = assign5350_e5518;
        locals.var_cv_e_dn4 = ((locals.var_cv_f_dn4 * locals.var_ovt) + (assign5350_e5516 * locals.var_ovt_dn4));
        locals.var_cv_e_dn5 = ((-locals.var_vbici_dn5) * locals.var_ovt);
        locals.var_cv_e_dn8 = ((-locals.var_vbici_dn8) * locals.var_ovt);

        let assign5360_e5521: f64 = (locals.var_cv_e * locals.var_cv_e);
        let assign5360_e5523: f64 = (assign5360_e5521 + 1.921812);
        let assign5360_e5524: f64 = (assign5360_e5523).sqrt();
        locals.var_cs_q = assign5360_e5524;
        locals.var_cs_q_dn4 = (((locals.var_cv_e_dn4 * locals.var_cv_e) + (locals.var_cv_e * locals.var_cv_e_dn4)) / (2.0 * assign5360_e5524));
        locals.var_cs_q_dn5 = (((locals.var_cv_e_dn5 * locals.var_cv_e) + (locals.var_cv_e * locals.var_cv_e_dn5)) / (2.0 * assign5360_e5524));
        locals.var_cs_q_dn8 = (((locals.var_cv_e_dn8 * locals.var_cv_e) + (locals.var_cv_e * locals.var_cv_e_dn8)) / (2.0 * assign5360_e5524));

        let assign5370_e5527: f64 = (locals.var_cv_e + locals.var_cs_q);
        let assign5370_e5529: f64 = (assign5370_e5527 * 0.5);
        locals.var_cs_q2 = assign5370_e5529;
        locals.var_cs_q2_dn4 = ((locals.var_cv_e_dn4 + locals.var_cs_q_dn4) * 0.5);
        locals.var_cs_q2_dn5 = ((locals.var_cv_e_dn5 + locals.var_cs_q_dn5) * 0.5);
        locals.var_cs_q2_dn8 = ((locals.var_cv_e_dn8 + locals.var_cs_q_dn8) * 0.5);

        let assign5380_e5533: f64 = (locals.var_vt * locals.var_cs_q2);
        let assign5380_e5534: f64 = (locals.var_cv_f - assign5380_e5533);
        locals.var_cv_j = assign5380_e5534;
        locals.var_cv_j_dn4 = (locals.var_cv_f_dn4 - ((locals.var_vt_dn4 * locals.var_cs_q2) + (locals.var_vt * locals.var_cs_q2_dn4)));
        locals.var_cv_j_dn5 = (-(locals.var_vt * locals.var_cs_q2_dn5));
        locals.var_cv_j_dn8 = (-(locals.var_vt * locals.var_cs_q2_dn8));

        let assign5390_e5537: f64 = (locals.var_cs_q2 / locals.var_cs_q);
        locals.var_cdvj_dv = assign5390_e5537;
        locals.var_cdvj_dv_dn4 = (((locals.var_cs_q2_dn4 * locals.var_cs_q) - (locals.var_cs_q2 * locals.var_cs_q_dn4)) / (locals.var_cs_q * locals.var_cs_q));
        locals.var_cdvj_dv_dn5 = (((locals.var_cs_q2_dn5 * locals.var_cs_q) - (locals.var_cs_q2 * locals.var_cs_q_dn5)) / (locals.var_cs_q * locals.var_cs_q));
        locals.var_cdvj_dv_dn8 = (((locals.var_cs_q2_dn8 * locals.var_cs_q) - (locals.var_cs_q2 * locals.var_cs_q_dn8)) / (locals.var_cs_q * locals.var_cs_q));

        let assign5400_e5539: f64 = (-p.p49);
        let assign5400_e5543: f64 = (locals.var_cv_j / locals.var_vdci_t);
        let assign5400_e5544: f64 = (1.0 - assign5400_e5543);
        let assign5400_e5545: f64 = (assign5400_e5544).ln();
        let assign5400_e5546: f64 = (assign5400_e5539 * assign5400_e5545);
        let assign5400_e5547: f64 = (assign5400_e5546).exp();
        let assign5400_e5549: f64 = (assign5400_e5547 * locals.var_cdvj_dv);
        let assign5400_e5553: f64 = (1.0 - locals.var_cdvj_dv);
        let assign5400_e5554: f64 = (2.4 * assign5400_e5553);
        let assign5400_e5555: f64 = (assign5400_e5549 + assign5400_e5554);
        locals.var_cci2cci0 = assign5400_e5555;
        locals.var_cci2cci0_dn4 = ((((assign5400_e5547 * (assign5400_e5539 * ((-(((locals.var_cv_j_dn4 * locals.var_vdci_t) - (locals.var_cv_j * locals.var_vdci_t_dn4)) / (locals.var_vdci_t * locals.var_vdci_t))) / assign5400_e5544))) * locals.var_cdvj_dv) + (assign5400_e5547 * locals.var_cdvj_dv_dn4)) + (2.4 * (-locals.var_cdvj_dv_dn4)));
        locals.var_cci2cci0_dn5 = ((((assign5400_e5547 * (assign5400_e5539 * ((-(locals.var_cv_j_dn5 / locals.var_vdci_t)) / assign5400_e5544))) * locals.var_cdvj_dv) + (assign5400_e5547 * locals.var_cdvj_dv_dn5)) + (2.4 * (-locals.var_cdvj_dv_dn5)));
        locals.var_cci2cci0_dn8 = ((((assign5400_e5547 * (assign5400_e5539 * ((-(locals.var_cv_j_dn8 / locals.var_vdci_t)) / assign5400_e5544))) * locals.var_cdvj_dv) + (assign5400_e5547 * locals.var_cdvj_dv_dn8)) + (2.4 * (-locals.var_cdvj_dv_dn8)));

        let assign5410_e5560: f64 = (1.0 / locals.var_cci2cci0);
        let assign5410_e5562: f64 = (assign5410_e5560 - 1.0);
        let assign5410_e5563: f64 = (p.p67 * assign5410_e5562);
        let assign5410_e5564: f64 = (locals.var_t0_t + assign5410_e5563);
        let assign5410_e5568: f64 = (locals.var_cci2cci0 - 1.0);
        let assign5410_e5569: f64 = (p.p68 * assign5410_e5568);
        let assign5410_e5570: f64 = (assign5410_e5564 + assign5410_e5569);
        locals.var_t_f0 = assign5410_e5570;
        locals.var_t_f0_dn4 = ((locals.var_t0_t_dn4 + (p.p67 * (-(locals.var_cci2cci0_dn4 / (locals.var_cci2cci0 * locals.var_cci2cci0))))) + (p.p68 * locals.var_cci2cci0_dn4));
        locals.var_t_f0_dn5 = ((p.p67 * (-(locals.var_cci2cci0_dn5 / (locals.var_cci2cci0 * locals.var_cci2cci0)))) + (p.p68 * locals.var_cci2cci0_dn5));
        locals.var_t_f0_dn8 = ((p.p67 * (-(locals.var_cci2cci0_dn8 / (locals.var_cci2cci0 * locals.var_cci2cci0)))) + (p.p68 * locals.var_cci2cci0_dn8));

        let assign5420_e5573: f64 = if p.p79 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign5420_e5573;

        let (assign5430_e5579, assign5430_e5579_d_n4, assign5430_e5579_d_n5, assign5430_e5579_d_n6, assign5430_e5579_d_n8,) = {
    if (locals.var_guard111 != 0.0) {
        let assign5430_e5577: f64 = (locals.var_vdck_t - locals.var_vbici);
        (assign5430_e5577, locals.var_vdck_t_dn4, (-locals.var_vbici_dn5), 0.0, (-locals.var_vbici_dn8),)
    } else {
        (locals.var_vc, locals.var_vc_dn4, locals.var_vc_dn5, locals.var_vc_dn6, locals.var_vc_dn8,)
    }
};
        locals.var_vc = assign5430_e5579;
        locals.var_vc_dn4 = assign5430_e5579_d_n4;
        locals.var_vc_dn5 = assign5430_e5579_d_n5;
        locals.var_vc_dn6 = assign5430_e5579_d_n6;
        locals.var_vc_dn8 = assign5430_e5579_d_n8;

        let (assign5440_e5586, assign5440_e5586_d_n4, assign5440_e5586_d_n5, assign5440_e5586_d_n6, assign5440_e5586_d_n8,) = {
    if (locals.var_guard111 == 0.0) {
        let assign5440_e5584: f64 = (locals.var_vciei - locals.var_vces_t);
        (assign5440_e5584, (-locals.var_vces_t_dn4), locals.var_vciei_dn5, locals.var_vciei_dn6, locals.var_vciei_dn8,)
    } else {
        (locals.var_vc, locals.var_vc_dn4, locals.var_vc_dn5, locals.var_vc_dn6, locals.var_vc_dn8,)
    }
};
        locals.var_vc = assign5440_e5586;
        locals.var_vc_dn4 = assign5440_e5586_d_n4;
        locals.var_vc_dn5 = assign5440_e5586_d_n5;
        locals.var_vc_dn6 = assign5440_e5586_d_n6;
        locals.var_vc_dn8 = assign5440_e5586_d_n8;

        let assign5450_e5589: f64 = if p.p0 <= 300.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign5450_e5589;

        let (assign5460_e5597, assign5460_e5597_d_n4, assign5460_e5597_d_n5, assign5460_e5597_d_n6, assign5460_e5597_d_n8,) = {
    if (locals.var_guard119 != 0.0) {
        let assign5460_e5593: f64 = (locals.var_vc - locals.var_vt);
        let assign5460_e5595: f64 = (assign5460_e5593 * locals.var_ovt);
        (assign5460_e5595, (((locals.var_vc_dn4 - locals.var_vt_dn4) * locals.var_ovt) + (assign5460_e5593 * locals.var_ovt_dn4)), (locals.var_vc_dn5 * locals.var_ovt), (locals.var_vc_dn6 * locals.var_ovt), (locals.var_vc_dn8 * locals.var_ovt),)
    } else {
        (locals.var_r_v, locals.var_r_v_dn4, locals.var_r_v_dn5, locals.var_r_v_dn6, locals.var_r_v_dn8,)
    }
};
        locals.var_r_v = assign5460_e5597;
        locals.var_r_v_dn4 = assign5460_e5597_d_n4;
        locals.var_r_v_dn5 = assign5460_e5597_d_n5;
        locals.var_r_v_dn6 = assign5460_e5597_d_n6;
        locals.var_r_v_dn8 = assign5460_e5597_d_n8;

        let (assign5470_e5614, assign5470_e5614_d_n4, assign5470_e5614_d_n5, assign5470_e5614_d_n6, assign5470_e5614_d_n8,) = {
    if (locals.var_guard119 != 0.0) {
        let assign5470_e5604: f64 = (locals.var_r_v * locals.var_r_v);
        let assign5470_e5606: f64 = (assign5470_e5604 + 1.921812);
        let assign5470_e5607: f64 = (assign5470_e5606).sqrt();
        let assign5470_e5608: f64 = (locals.var_r_v + assign5470_e5607);
        let assign5470_e5610: f64 = (assign5470_e5608 * 0.5);
        let assign5470_e5611: f64 = (locals.var_vt * assign5470_e5610);
        let assign5470_e5612: f64 = (locals.var_vt + assign5470_e5611);
        (assign5470_e5612, (locals.var_vt_dn4 + ((locals.var_vt_dn4 * assign5470_e5610) + (locals.var_vt * ((locals.var_r_v_dn4 + (((locals.var_r_v_dn4 * locals.var_r_v) + (locals.var_r_v * locals.var_r_v_dn4)) / (2.0 * assign5470_e5607))) * 0.5)))), (locals.var_vt * ((locals.var_r_v_dn5 + (((locals.var_r_v_dn5 * locals.var_r_v) + (locals.var_r_v * locals.var_r_v_dn5)) / (2.0 * assign5470_e5607))) * 0.5)), (locals.var_vt * ((locals.var_r_v_dn6 + (((locals.var_r_v_dn6 * locals.var_r_v) + (locals.var_r_v * locals.var_r_v_dn6)) / (2.0 * assign5470_e5607))) * 0.5)), (locals.var_vt * ((locals.var_r_v_dn8 + (((locals.var_r_v_dn8 * locals.var_r_v) + (locals.var_r_v * locals.var_r_v_dn8)) / (2.0 * assign5470_e5607))) * 0.5)),)
    } else {
        (locals.var_vceff, locals.var_vceff_dn4, locals.var_vceff_dn5, locals.var_vceff_dn6, locals.var_vceff_dn8,)
    }
};
        locals.var_vceff = assign5470_e5614;
        locals.var_vceff_dn4 = assign5470_e5614_d_n4;
        locals.var_vceff_dn5 = assign5470_e5614_d_n5;
        locals.var_vceff_dn6 = assign5470_e5614_d_n6;
        locals.var_vceff_dn8 = assign5470_e5614_d_n8;

        let (assign5480_e5621, assign5480_e5621_d_n4, assign5480_e5621_d_n5, assign5480_e5621_d_n6, assign5480_e5621_d_n8,) = {
    if (locals.var_guard119 == 0.0) {
        let assign5480_e5619: f64 = (locals.var_vc / locals.var_vt300);
        (assign5480_e5619, (locals.var_vc_dn4 / locals.var_vt300), (locals.var_vc_dn5 / locals.var_vt300), (locals.var_vc_dn6 / locals.var_vt300), (locals.var_vc_dn8 / locals.var_vt300),)
    } else {
        (locals.var_r_v, locals.var_r_v_dn4, locals.var_r_v_dn5, locals.var_r_v_dn6, locals.var_r_v_dn8,)
    }
};
        locals.var_r_v = assign5480_e5621;
        locals.var_r_v_dn4 = assign5480_e5621_d_n4;
        locals.var_r_v_dn5 = assign5480_e5621_d_n5;
        locals.var_r_v_dn6 = assign5480_e5621_d_n6;
        locals.var_r_v_dn8 = assign5480_e5621_d_n8;

        let (assign5490_e5637, assign5490_e5637_d_n4, assign5490_e5637_d_n5, assign5490_e5637_d_n6, assign5490_e5637_d_n8,) = {
    if (locals.var_guard119 == 0.0) {
        let assign5490_e5628: f64 = (locals.var_r_v * locals.var_r_v);
        let assign5490_e5630: f64 = (assign5490_e5628 + p.p80);
        let assign5490_e5631: f64 = (assign5490_e5630).sqrt();
        let assign5490_e5632: f64 = (locals.var_r_v + assign5490_e5631);
        let assign5490_e5634: f64 = (assign5490_e5632 * 0.5);
        let assign5490_e5635: f64 = (locals.var_vt300 * assign5490_e5634);
        (assign5490_e5635, (locals.var_vt300 * ((locals.var_r_v_dn4 + (((locals.var_r_v_dn4 * locals.var_r_v) + (locals.var_r_v * locals.var_r_v_dn4)) / (2.0 * assign5490_e5631))) * 0.5)), (locals.var_vt300 * ((locals.var_r_v_dn5 + (((locals.var_r_v_dn5 * locals.var_r_v) + (locals.var_r_v * locals.var_r_v_dn5)) / (2.0 * assign5490_e5631))) * 0.5)), (locals.var_vt300 * ((locals.var_r_v_dn6 + (((locals.var_r_v_dn6 * locals.var_r_v) + (locals.var_r_v * locals.var_r_v_dn6)) / (2.0 * assign5490_e5631))) * 0.5)), (locals.var_vt300 * ((locals.var_r_v_dn8 + (((locals.var_r_v_dn8 * locals.var_r_v) + (locals.var_r_v * locals.var_r_v_dn8)) / (2.0 * assign5490_e5631))) * 0.5)),)
    } else {
        (locals.var_vceff, locals.var_vceff_dn4, locals.var_vceff_dn5, locals.var_vceff_dn6, locals.var_vceff_dn8,)
    }
};
        locals.var_vceff = assign5490_e5637;
        locals.var_vceff_dn4 = assign5490_e5637_d_n4;
        locals.var_vceff_dn5 = assign5490_e5637_d_n5;
        locals.var_vceff_dn6 = assign5490_e5637_d_n6;
        locals.var_vceff_dn8 = assign5490_e5637_d_n8;

        let assign5500_e5640: f64 = (locals.var_vceff / locals.var_vlim_t);
        locals.var_vc2vlim = assign5500_e5640;
        locals.var_vc2vlim_dn4 = (((locals.var_vceff_dn4 * locals.var_vlim_t) - (locals.var_vceff * locals.var_vlim_t_dn4)) / (locals.var_vlim_t * locals.var_vlim_t));
        locals.var_vc2vlim_dn5 = (locals.var_vceff_dn5 / locals.var_vlim_t);
        locals.var_vc2vlim_dn6 = (locals.var_vceff_dn6 / locals.var_vlim_t);
        locals.var_vc2vlim_dn8 = (locals.var_vceff_dn8 / locals.var_vlim_t);

        let assign5510_e5643: f64 = (locals.var_vceff * locals.var_orci0_t);
        locals.var_ick_ohm = assign5510_e5643;
        locals.var_ick_ohm_dn4 = ((locals.var_vceff_dn4 * locals.var_orci0_t) + (locals.var_vceff * locals.var_orci0_t_dn4));
        locals.var_ick_ohm_dn5 = (locals.var_vceff_dn5 * locals.var_orci0_t);
        locals.var_ick_ohm_dn6 = (locals.var_vceff_dn6 * locals.var_orci0_t);
        locals.var_ick_ohm_dn8 = (locals.var_vceff_dn8 * locals.var_orci0_t);

        let assign5520_e5647: f64 = (locals.var_vc2vlim).ln();
        let assign5520_e5648: f64 = (p.p77 * assign5520_e5647);
        let assign5520_e5649: f64 = (assign5520_e5648).exp();
        let assign5520_e5650: f64 = (1.0 + assign5520_e5649);
        let assign5520_e5651: f64 = (assign5520_e5650).ln();
        let assign5520_e5653: f64 = (assign5520_e5651 / p.p77);
        let assign5520_e5654: f64 = (assign5520_e5653).exp();
        locals.var_ff_ick = assign5520_e5654;
        locals.var_ff_ick_dn4 = (assign5520_e5654 * (((assign5520_e5649 * (p.p77 * (locals.var_vc2vlim_dn4 / locals.var_vc2vlim))) / assign5520_e5650) / p.p77));
        locals.var_ff_ick_dn5 = (assign5520_e5654 * (((assign5520_e5649 * (p.p77 * (locals.var_vc2vlim_dn5 / locals.var_vc2vlim))) / assign5520_e5650) / p.p77));
        locals.var_ff_ick_dn6 = (assign5520_e5654 * (((assign5520_e5649 * (p.p77 * (locals.var_vc2vlim_dn6 / locals.var_vc2vlim))) / assign5520_e5650) / p.p77));
        locals.var_ff_ick_dn8 = (assign5520_e5654 * (((assign5520_e5649 * (p.p77 * (locals.var_vc2vlim_dn8 / locals.var_vc2vlim))) / assign5520_e5650) / p.p77));

        let assign5530_e5657: f64 = (locals.var_ick_ohm / locals.var_ff_ick);
        locals.var_ick_low = assign5530_e5657;
        locals.var_ick_low_dn4 = (((locals.var_ick_ohm_dn4 * locals.var_ff_ick) - (locals.var_ick_ohm * locals.var_ff_ick_dn4)) / (locals.var_ff_ick * locals.var_ff_ick));
        locals.var_ick_low_dn5 = (((locals.var_ick_ohm_dn5 * locals.var_ff_ick) - (locals.var_ick_ohm * locals.var_ff_ick_dn5)) / (locals.var_ff_ick * locals.var_ff_ick));
        locals.var_ick_low_dn6 = (((locals.var_ick_ohm_dn6 * locals.var_ff_ick) - (locals.var_ick_ohm * locals.var_ff_ick_dn6)) / (locals.var_ff_ick * locals.var_ff_ick));
        locals.var_ick_low_dn8 = (((locals.var_ick_ohm_dn8 * locals.var_ff_ick) - (locals.var_ick_ohm * locals.var_ff_ick_dn8)) / (locals.var_ff_ick * locals.var_ff_ick));

        let assign5540_e5660: f64 = (locals.var_vceff - locals.var_vlim_t);
        let assign5540_e5662: f64 = (assign5540_e5660 / p.p76);
        locals.var_vick_vpt = assign5540_e5662;
        locals.var_vick_vpt_dn4 = ((locals.var_vceff_dn4 - locals.var_vlim_t_dn4) / p.p76);
        locals.var_vick_vpt_dn5 = (locals.var_vceff_dn5 / p.p76);
        locals.var_vick_vpt_dn6 = (locals.var_vceff_dn6 / p.p76);
        locals.var_vick_vpt_dn8 = (locals.var_vceff_dn8 / p.p76);

        let assign5550_e5669: f64 = (locals.var_vick_vpt * locals.var_vick_vpt);
        let assign5550_e5671: f64 = (assign5550_e5669 + p.p81);
        let assign5550_e5672: f64 = (assign5550_e5671).sqrt();
        let assign5550_e5673: f64 = (locals.var_vick_vpt + assign5550_e5672);
        let assign5550_e5674: f64 = (0.5 * assign5550_e5673);
        let assign5550_e5675: f64 = (1.0 + assign5550_e5674);
        let assign5550_e5676: f64 = (locals.var_ick_low * assign5550_e5675);
        locals.var_ick = assign5550_e5676;
        locals.var_ick_dn4 = ((locals.var_ick_low_dn4 * assign5550_e5675) + (locals.var_ick_low * (0.5 * (locals.var_vick_vpt_dn4 + (((locals.var_vick_vpt_dn4 * locals.var_vick_vpt) + (locals.var_vick_vpt * locals.var_vick_vpt_dn4)) / (2.0 * assign5550_e5672))))));
        locals.var_ick_dn5 = ((locals.var_ick_low_dn5 * assign5550_e5675) + (locals.var_ick_low * (0.5 * (locals.var_vick_vpt_dn5 + (((locals.var_vick_vpt_dn5 * locals.var_vick_vpt) + (locals.var_vick_vpt * locals.var_vick_vpt_dn5)) / (2.0 * assign5550_e5672))))));
        locals.var_ick_dn6 = ((locals.var_ick_low_dn6 * assign5550_e5675) + (locals.var_ick_low * (0.5 * (locals.var_vick_vpt_dn6 + (((locals.var_vick_vpt_dn6 * locals.var_vick_vpt) + (locals.var_vick_vpt * locals.var_vick_vpt_dn6)) / (2.0 * assign5550_e5672))))));
        locals.var_ick_dn8 = ((locals.var_ick_low_dn8 * assign5550_e5675) + (locals.var_ick_low * (0.5 * (locals.var_vick_vpt_dn8 + (((locals.var_vick_vpt_dn8 * locals.var_vick_vpt) + (locals.var_vick_vpt * locals.var_vick_vpt_dn8)) / (2.0 * assign5550_e5672))))));

        locals.var_q_pt = locals.var_q0_pt;
        locals.var_q_pt_dn0 = locals.var_q0_pt_dn0;
        locals.var_q_pt_dn1 = locals.var_q0_pt_dn1;
        locals.var_q_pt_dn3 = locals.var_q0_pt_dn3;
        locals.var_q_pt_dn4 = locals.var_q0_pt_dn4;
        locals.var_q_pt_dn5 = locals.var_q0_pt_dn5;
        locals.var_q_pt_dn6 = locals.var_q0_pt_dn6;
        locals.var_q_pt_dn7 = locals.var_q0_pt_dn7;
        locals.var_q_pt_dn8 = locals.var_q0_pt_dn8;
        locals.var_q_pt_dn9 = locals.var_q0_pt_dn9;

        let assign5570_e5684: f64 = if ((locals.var_t_f0 > 0.0) || (p.p85 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard120 = assign5570_e5684;

        let (assign5580_e5690, assign5580_e5690_d_n0, assign5580_e5690_d_n1, assign5580_e5690_d_n3, assign5580_e5690_d_n4, assign5580_e5690_d_n5, assign5580_e5690_d_n6, assign5580_e5690_d_n7, assign5580_e5690_d_n8, assign5580_e5690_d_n9,) = {
    if (locals.var_guard120 != 0.0) {
        let assign5580_e5688: f64 = (0.5 * locals.var_q0_pt);
        (assign5580_e5688, (0.5 * locals.var_q0_pt_dn0), (0.5 * locals.var_q0_pt_dn1), (0.5 * locals.var_q0_pt_dn3), (0.5 * locals.var_q0_pt_dn4), (0.5 * locals.var_q0_pt_dn5), (0.5 * locals.var_q0_pt_dn6), (0.5 * locals.var_q0_pt_dn7), (0.5 * locals.var_q0_pt_dn8), (0.5 * locals.var_q0_pt_dn9),)
    } else {
        (locals.var_a, locals.var_a_dn0, locals.var_a_dn1, locals.var_a_dn3, locals.var_a_dn4, locals.var_a_dn5, locals.var_a_dn6, locals.var_a_dn7, locals.var_a_dn8, locals.var_a_dn9,)
    }
};
        locals.var_a = assign5580_e5690;
        locals.var_a_dn0 = assign5580_e5690_d_n0;
        locals.var_a_dn1 = assign5580_e5690_d_n1;
        locals.var_a_dn3 = assign5580_e5690_d_n3;
        locals.var_a_dn4 = assign5580_e5690_d_n4;
        locals.var_a_dn5 = assign5580_e5690_d_n5;
        locals.var_a_dn6 = assign5580_e5690_d_n6;
        locals.var_a_dn7 = assign5580_e5690_d_n7;
        locals.var_a_dn8 = assign5580_e5690_d_n8;
        locals.var_a_dn9 = assign5580_e5690_d_n9;

        let assign5590_e5693: f64 = if p.p0 <= 300.0 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign5590_e5693;

        let (assign5600_e5712, assign5600_e5712_d_n0, assign5600_e5712_d_n1, assign5600_e5712_d_n3, assign5600_e5712_d_n4, assign5600_e5712_d_n5, assign5600_e5712_d_n6, assign5600_e5712_d_n7, assign5600_e5712_d_n8, assign5600_e5712_d_n9,) = {
    if ((locals.var_guard120 != 0.0) && (locals.var_guard122 != 0.0)) {
        let assign5600_e5700: f64 = (locals.var_a * locals.var_a);
        let assign5600_e5703: f64 = (locals.var_t_f0 * locals.var_i_0f);
        let assign5600_e5704: f64 = (assign5600_e5700 + assign5600_e5703);
        let assign5600_e5707: f64 = (p.p85 * locals.var_i_0r);
        let assign5600_e5708: f64 = (assign5600_e5704 + assign5600_e5707);
        let assign5600_e5709: f64 = (assign5600_e5708).sqrt();
        let assign5600_e5710: f64 = (locals.var_a + assign5600_e5709);
        (assign5600_e5710, (locals.var_a_dn0 + (((locals.var_a_dn0 * locals.var_a) + (locals.var_a * locals.var_a_dn0)) / (2.0 * assign5600_e5709))), (locals.var_a_dn1 + (((locals.var_a_dn1 * locals.var_a) + (locals.var_a * locals.var_a_dn1)) / (2.0 * assign5600_e5709))), (locals.var_a_dn3 + (((locals.var_a_dn3 * locals.var_a) + (locals.var_a * locals.var_a_dn3)) / (2.0 * assign5600_e5709))), (locals.var_a_dn4 + (((((locals.var_a_dn4 * locals.var_a) + (locals.var_a * locals.var_a_dn4)) + ((locals.var_t_f0_dn4 * locals.var_i_0f) + (locals.var_t_f0 * locals.var_i_0f_dn4))) + (p.p85 * locals.var_i_0r_dn4)) / (2.0 * assign5600_e5709))), (locals.var_a_dn5 + (((((locals.var_a_dn5 * locals.var_a) + (locals.var_a * locals.var_a_dn5)) + (locals.var_t_f0_dn5 * locals.var_i_0f)) + (p.p85 * locals.var_i_0r_dn5)) / (2.0 * assign5600_e5709))), (locals.var_a_dn6 + ((((locals.var_a_dn6 * locals.var_a) + (locals.var_a * locals.var_a_dn6)) + (locals.var_t_f0 * locals.var_i_0f_dn6)) / (2.0 * assign5600_e5709))), (locals.var_a_dn7 + (((locals.var_a_dn7 * locals.var_a) + (locals.var_a * locals.var_a_dn7)) / (2.0 * assign5600_e5709))), (locals.var_a_dn8 + (((((locals.var_a_dn8 * locals.var_a) + (locals.var_a * locals.var_a_dn8)) + ((locals.var_t_f0_dn8 * locals.var_i_0f) + (locals.var_t_f0 * locals.var_i_0f_dn8))) + (p.p85 * locals.var_i_0r_dn8)) / (2.0 * assign5600_e5709))), (locals.var_a_dn9 + (((locals.var_a_dn9 * locals.var_a) + (locals.var_a * locals.var_a_dn9)) / (2.0 * assign5600_e5709))),)
    } else {
        (locals.var_q_pt, locals.var_q_pt_dn0, locals.var_q_pt_dn1, locals.var_q_pt_dn3, locals.var_q_pt_dn4, locals.var_q_pt_dn5, locals.var_q_pt_dn6, locals.var_q_pt_dn7, locals.var_q_pt_dn8, locals.var_q_pt_dn9,)
    }
};
        locals.var_q_pt = assign5600_e5712;
        locals.var_q_pt_dn0 = assign5600_e5712_d_n0;
        locals.var_q_pt_dn1 = assign5600_e5712_d_n1;
        locals.var_q_pt_dn3 = assign5600_e5712_d_n3;
        locals.var_q_pt_dn4 = assign5600_e5712_d_n4;
        locals.var_q_pt_dn5 = assign5600_e5712_d_n5;
        locals.var_q_pt_dn6 = assign5600_e5712_d_n6;
        locals.var_q_pt_dn7 = assign5600_e5712_d_n7;
        locals.var_q_pt_dn8 = assign5600_e5712_d_n8;
        locals.var_q_pt_dn9 = assign5600_e5712_d_n9;

        let (assign5610_e5734, assign5610_e5734_d_n0, assign5610_e5734_d_n1, assign5610_e5734_d_n3, assign5610_e5734_d_n4, assign5610_e5734_d_n5, assign5610_e5734_d_n6, assign5610_e5734_d_n7, assign5610_e5734_d_n8, assign5610_e5734_d_n9,) = {
    if ((locals.var_guard120 != 0.0) && (locals.var_guard122 == 0.0)) {
        let assign5610_e5720: f64 = (locals.var_a * locals.var_a);
        let assign5610_e5723: f64 = (locals.var_hf0_t * locals.var_t0_t);
        let assign5610_e5725: f64 = (assign5610_e5723 * locals.var_i_0f);
        let assign5610_e5726: f64 = (assign5610_e5720 + assign5610_e5725);
        let assign5610_e5729: f64 = (p.p85 * locals.var_i_0r);
        let assign5610_e5730: f64 = (assign5610_e5726 + assign5610_e5729);
        let assign5610_e5731: f64 = (assign5610_e5730).sqrt();
        let assign5610_e5732: f64 = (locals.var_a + assign5610_e5731);
        (assign5610_e5732, (locals.var_a_dn0 + (((locals.var_a_dn0 * locals.var_a) + (locals.var_a * locals.var_a_dn0)) / (2.0 * assign5610_e5731))), (locals.var_a_dn1 + (((locals.var_a_dn1 * locals.var_a) + (locals.var_a * locals.var_a_dn1)) / (2.0 * assign5610_e5731))), (locals.var_a_dn3 + (((locals.var_a_dn3 * locals.var_a) + (locals.var_a * locals.var_a_dn3)) / (2.0 * assign5610_e5731))), (locals.var_a_dn4 + (((((locals.var_a_dn4 * locals.var_a) + (locals.var_a * locals.var_a_dn4)) + ((((locals.var_hf0_t_dn4 * locals.var_t0_t) + (locals.var_hf0_t * locals.var_t0_t_dn4)) * locals.var_i_0f) + (assign5610_e5723 * locals.var_i_0f_dn4))) + (p.p85 * locals.var_i_0r_dn4)) / (2.0 * assign5610_e5731))), (locals.var_a_dn5 + ((((locals.var_a_dn5 * locals.var_a) + (locals.var_a * locals.var_a_dn5)) + (p.p85 * locals.var_i_0r_dn5)) / (2.0 * assign5610_e5731))), (locals.var_a_dn6 + ((((locals.var_a_dn6 * locals.var_a) + (locals.var_a * locals.var_a_dn6)) + (assign5610_e5723 * locals.var_i_0f_dn6)) / (2.0 * assign5610_e5731))), (locals.var_a_dn7 + (((locals.var_a_dn7 * locals.var_a) + (locals.var_a * locals.var_a_dn7)) / (2.0 * assign5610_e5731))), (locals.var_a_dn8 + (((((locals.var_a_dn8 * locals.var_a) + (locals.var_a * locals.var_a_dn8)) + (assign5610_e5723 * locals.var_i_0f_dn8)) + (p.p85 * locals.var_i_0r_dn8)) / (2.0 * assign5610_e5731))), (locals.var_a_dn9 + (((locals.var_a_dn9 * locals.var_a) + (locals.var_a * locals.var_a_dn9)) / (2.0 * assign5610_e5731))),)
    } else {
        (locals.var_q_pt, locals.var_q_pt_dn0, locals.var_q_pt_dn1, locals.var_q_pt_dn3, locals.var_q_pt_dn4, locals.var_q_pt_dn5, locals.var_q_pt_dn6, locals.var_q_pt_dn7, locals.var_q_pt_dn8, locals.var_q_pt_dn9,)
    }
};
        locals.var_q_pt = assign5610_e5734;
        locals.var_q_pt_dn0 = assign5610_e5734_d_n0;
        locals.var_q_pt_dn1 = assign5610_e5734_d_n1;
        locals.var_q_pt_dn3 = assign5610_e5734_d_n3;
        locals.var_q_pt_dn4 = assign5610_e5734_d_n4;
        locals.var_q_pt_dn5 = assign5610_e5734_d_n5;
        locals.var_q_pt_dn6 = assign5610_e5734_d_n6;
        locals.var_q_pt_dn7 = assign5610_e5734_d_n7;
        locals.var_q_pt_dn8 = assign5610_e5734_d_n8;
        locals.var_q_pt_dn9 = assign5610_e5734_d_n9;

        let assign5620_e5737: f64 = (locals.var_i_0f / locals.var_q_pt);
        locals.var_itf = assign5620_e5737;
        locals.var_itf_dn0 = (-((locals.var_i_0f * locals.var_q_pt_dn0) / (locals.var_q_pt * locals.var_q_pt)));
        locals.var_itf_dn1 = (-((locals.var_i_0f * locals.var_q_pt_dn1) / (locals.var_q_pt * locals.var_q_pt)));
        locals.var_itf_dn3 = (-((locals.var_i_0f * locals.var_q_pt_dn3) / (locals.var_q_pt * locals.var_q_pt)));
        locals.var_itf_dn4 = (((locals.var_i_0f_dn4 * locals.var_q_pt) - (locals.var_i_0f * locals.var_q_pt_dn4)) / (locals.var_q_pt * locals.var_q_pt));
        locals.var_itf_dn5 = (-((locals.var_i_0f * locals.var_q_pt_dn5) / (locals.var_q_pt * locals.var_q_pt)));
        locals.var_itf_dn6 = (((locals.var_i_0f_dn6 * locals.var_q_pt) - (locals.var_i_0f * locals.var_q_pt_dn6)) / (locals.var_q_pt * locals.var_q_pt));
        locals.var_itf_dn7 = (-((locals.var_i_0f * locals.var_q_pt_dn7) / (locals.var_q_pt * locals.var_q_pt)));
        locals.var_itf_dn8 = (((locals.var_i_0f_dn8 * locals.var_q_pt) - (locals.var_i_0f * locals.var_q_pt_dn8)) / (locals.var_q_pt * locals.var_q_pt));
        locals.var_itf_dn9 = (-((locals.var_i_0f * locals.var_q_pt_dn9) / (locals.var_q_pt * locals.var_q_pt)));

        let assign5630_e5740: f64 = (locals.var_i_0r / locals.var_q_pt);
        locals.var_itr = assign5630_e5740;
        locals.var_itr_dn0 = (-((locals.var_i_0r * locals.var_q_pt_dn0) / (locals.var_q_pt * locals.var_q_pt)));
        locals.var_itr_dn1 = (-((locals.var_i_0r * locals.var_q_pt_dn1) / (locals.var_q_pt * locals.var_q_pt)));
        locals.var_itr_dn3 = (-((locals.var_i_0r * locals.var_q_pt_dn3) / (locals.var_q_pt * locals.var_q_pt)));
        locals.var_itr_dn4 = (((locals.var_i_0r_dn4 * locals.var_q_pt) - (locals.var_i_0r * locals.var_q_pt_dn4)) / (locals.var_q_pt * locals.var_q_pt));
        locals.var_itr_dn5 = (((locals.var_i_0r_dn5 * locals.var_q_pt) - (locals.var_i_0r * locals.var_q_pt_dn5)) / (locals.var_q_pt * locals.var_q_pt));
        locals.var_itr_dn6 = (-((locals.var_i_0r * locals.var_q_pt_dn6) / (locals.var_q_pt * locals.var_q_pt)));
        locals.var_itr_dn7 = (-((locals.var_i_0r * locals.var_q_pt_dn7) / (locals.var_q_pt * locals.var_q_pt)));
        locals.var_itr_dn8 = (((locals.var_i_0r_dn8 * locals.var_q_pt) - (locals.var_i_0r * locals.var_q_pt_dn8)) / (locals.var_q_pt * locals.var_q_pt));
        locals.var_itr_dn9 = (-((locals.var_i_0r * locals.var_q_pt_dn9) / (locals.var_q_pt * locals.var_q_pt)));

        locals.var_tf = locals.var_t_f0;
        locals.var_tf_dn0 = 0.0;
        locals.var_tf_dn1 = 0.0;
        locals.var_tf_dn3 = 0.0;
        locals.var_tf_dn4 = locals.var_t_f0_dn4;
        locals.var_tf_dn5 = locals.var_t_f0_dn5;
        locals.var_tf_dn6 = 0.0;
        locals.var_tf_dn7 = 0.0;
        locals.var_tf_dn8 = locals.var_t_f0_dn8;
        locals.var_tf_dn9 = 0.0;

        let assign5650_e5744: f64 = (locals.var_t_f0 * locals.var_itf);
        locals.var_qf = assign5650_e5744;
        locals.var_qf_dn0 = (locals.var_t_f0 * locals.var_itf_dn0);
        locals.var_qf_dn1 = (locals.var_t_f0 * locals.var_itf_dn1);
        locals.var_qf_dn3 = (locals.var_t_f0 * locals.var_itf_dn3);
        locals.var_qf_dn4 = ((locals.var_t_f0_dn4 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn4));
        locals.var_qf_dn5 = ((locals.var_t_f0_dn5 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn5));
        locals.var_qf_dn6 = (locals.var_t_f0 * locals.var_itf_dn6);
        locals.var_qf_dn7 = (locals.var_t_f0 * locals.var_itf_dn7);
        locals.var_qf_dn8 = ((locals.var_t_f0_dn8 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn8));
        locals.var_qf_dn9 = (locals.var_t_f0 * locals.var_itf_dn9);

        let assign5660_e5747: f64 = if p.p0 >= 310.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign5660_e5747;

        let (assign5670_e5753, assign5670_e5753_d_n0, assign5670_e5753_d_n1, assign5670_e5753_d_n3, assign5670_e5753_d_n4, assign5670_e5753_d_n5, assign5670_e5753_d_n6, assign5670_e5753_d_n7, assign5670_e5753_d_n8, assign5670_e5753_d_n9,) = {
    if (locals.var_guard123 != 0.0) {
        let assign5670_e5751: f64 = (locals.var_hf0_t * locals.var_t0_t);
        (assign5670_e5751, 0.0, 0.0, 0.0, ((locals.var_hf0_t_dn4 * locals.var_t0_t) + (locals.var_hf0_t * locals.var_t0_t_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
        locals.var_t_ft = assign5670_e5753;
        locals.var_t_ft_dn0 = assign5670_e5753_d_n0;
        locals.var_t_ft_dn1 = assign5670_e5753_d_n1;
        locals.var_t_ft_dn3 = assign5670_e5753_d_n3;
        locals.var_t_ft_dn4 = assign5670_e5753_d_n4;
        locals.var_t_ft_dn5 = assign5670_e5753_d_n5;
        locals.var_t_ft_dn6 = assign5670_e5753_d_n6;
        locals.var_t_ft_dn7 = assign5670_e5753_d_n7;
        locals.var_t_ft_dn8 = assign5670_e5753_d_n8;
        locals.var_t_ft_dn9 = assign5670_e5753_d_n9;

        let (assign5680_e5759, assign5680_e5759_d_n0, assign5680_e5759_d_n1, assign5680_e5759_d_n3, assign5680_e5759_d_n4, assign5680_e5759_d_n5, assign5680_e5759_d_n6, assign5680_e5759_d_n7, assign5680_e5759_d_n8, assign5680_e5759_d_n9,) = {
    if (locals.var_guard123 != 0.0) {
        let assign5680_e5757: f64 = (locals.var_t_ft * locals.var_itf);
        (assign5680_e5757, ((locals.var_t_ft_dn0 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn0)), ((locals.var_t_ft_dn1 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn1)), ((locals.var_t_ft_dn3 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn3)), ((locals.var_t_ft_dn4 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn4)), ((locals.var_t_ft_dn5 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn5)), ((locals.var_t_ft_dn6 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn6)), ((locals.var_t_ft_dn7 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn7)), ((locals.var_t_ft_dn8 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn8)), ((locals.var_t_ft_dn9 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn9)),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
        locals.var_q_ft = assign5680_e5759;
        locals.var_q_ft_dn0 = assign5680_e5759_d_n0;
        locals.var_q_ft_dn1 = assign5680_e5759_d_n1;
        locals.var_q_ft_dn3 = assign5680_e5759_d_n3;
        locals.var_q_ft_dn4 = assign5680_e5759_d_n4;
        locals.var_q_ft_dn5 = assign5680_e5759_d_n5;
        locals.var_q_ft_dn6 = assign5680_e5759_d_n6;
        locals.var_q_ft_dn7 = assign5680_e5759_d_n7;
        locals.var_q_ft_dn8 = assign5680_e5759_d_n8;
        locals.var_q_ft_dn9 = assign5680_e5759_d_n9;

        let (assign5690_e5766, assign5690_e5766_d_n0, assign5690_e5766_d_n1, assign5690_e5766_d_n3, assign5690_e5766_d_n4, assign5690_e5766_d_n5, assign5690_e5766_d_n6, assign5690_e5766_d_n7, assign5690_e5766_d_n8, assign5690_e5766_d_n9,) = {
    if (locals.var_guard123 == 0.0) {
        let assign5690_e5764: f64 = (locals.var_hf0_t * locals.var_qf);
        (assign5690_e5764, (locals.var_hf0_t * locals.var_qf_dn0), (locals.var_hf0_t * locals.var_qf_dn1), (locals.var_hf0_t * locals.var_qf_dn3), ((locals.var_hf0_t_dn4 * locals.var_qf) + (locals.var_hf0_t * locals.var_qf_dn4)), (locals.var_hf0_t * locals.var_qf_dn5), (locals.var_hf0_t * locals.var_qf_dn6), (locals.var_hf0_t * locals.var_qf_dn7), (locals.var_hf0_t * locals.var_qf_dn8), (locals.var_hf0_t * locals.var_qf_dn9),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
        locals.var_q_ft = assign5690_e5766;
        locals.var_q_ft_dn0 = assign5690_e5766_d_n0;
        locals.var_q_ft_dn1 = assign5690_e5766_d_n1;
        locals.var_q_ft_dn3 = assign5690_e5766_d_n3;
        locals.var_q_ft_dn4 = assign5690_e5766_d_n4;
        locals.var_q_ft_dn5 = assign5690_e5766_d_n5;
        locals.var_q_ft_dn6 = assign5690_e5766_d_n6;
        locals.var_q_ft_dn7 = assign5690_e5766_d_n7;
        locals.var_q_ft_dn8 = assign5690_e5766_d_n8;
        locals.var_q_ft_dn9 = assign5690_e5766_d_n9;

    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5700_e5773, assign5700_e5773_d_n0, assign5700_e5773_d_n1, assign5700_e5773_d_n3, assign5700_e5773_d_n4, assign5700_e5773_d_n5, assign5700_e5773_d_n6, assign5700_e5773_d_n7, assign5700_e5773_d_n8, assign5700_e5773_d_n9,) = {
    if (locals.var_guard123 == 0.0) {
        let assign5700_e5771: f64 = (locals.var_hf0_t * locals.var_tf);
        (assign5700_e5771, (locals.var_hf0_t * locals.var_tf_dn0), (locals.var_hf0_t * locals.var_tf_dn1), (locals.var_hf0_t * locals.var_tf_dn3), ((locals.var_hf0_t_dn4 * locals.var_tf) + (locals.var_hf0_t * locals.var_tf_dn4)), (locals.var_hf0_t * locals.var_tf_dn5), (locals.var_hf0_t * locals.var_tf_dn6), (locals.var_hf0_t * locals.var_tf_dn7), (locals.var_hf0_t * locals.var_tf_dn8), (locals.var_hf0_t * locals.var_tf_dn9),)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
        locals.var_t_ft = assign5700_e5773;
        locals.var_t_ft_dn0 = assign5700_e5773_d_n0;
        locals.var_t_ft_dn1 = assign5700_e5773_d_n1;
        locals.var_t_ft_dn3 = assign5700_e5773_d_n3;
        locals.var_t_ft_dn4 = assign5700_e5773_d_n4;
        locals.var_t_ft_dn5 = assign5700_e5773_d_n5;
        locals.var_t_ft_dn6 = assign5700_e5773_d_n6;
        locals.var_t_ft_dn7 = assign5700_e5773_d_n7;
        locals.var_t_ft_dn8 = assign5700_e5773_d_n8;
        locals.var_t_ft_dn9 = assign5700_e5773_d_n9;

        locals.var_q_bf = 0.0;
        locals.var_q_bf_dn0 = 0.0;
        locals.var_q_bf_dn1 = 0.0;
        locals.var_q_bf_dn3 = 0.0;
        locals.var_q_bf_dn4 = 0.0;
        locals.var_q_bf_dn5 = 0.0;
        locals.var_q_bf_dn6 = 0.0;
        locals.var_q_bf_dn7 = 0.0;
        locals.var_q_bf_dn8 = 0.0;
        locals.var_q_bf_dn9 = 0.0;

        let assign5720_e5778: f64 = (1e-6 * locals.var_ick);
        let assign5720_e5783: f64 = if ((locals.var_itf >= assign5720_e5778) || (p.p0 >= 320.0)) { 1.0 } else { 0.0 };
        locals.var_guard124 = assign5720_e5783;

        let (assign5730_e5789, assign5730_e5789_d_n0, assign5730_e5789_d_n1, assign5730_e5789_d_n3, assign5730_e5789_d_n4, assign5730_e5789_d_n5, assign5730_e5789_d_n6, assign5730_e5789_d_n7, assign5730_e5789_d_n8, assign5730_e5789_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign5730_e5787: f64 = (locals.var_itf / locals.var_ick);
        (assign5730_e5787, (locals.var_itf_dn0 / locals.var_ick), (locals.var_itf_dn1 / locals.var_ick), (locals.var_itf_dn3 / locals.var_ick), (((locals.var_itf_dn4 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn4)) / (locals.var_ick * locals.var_ick)), (((locals.var_itf_dn5 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn5)) / (locals.var_ick * locals.var_ick)), (((locals.var_itf_dn6 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn6)) / (locals.var_ick * locals.var_ick)), (locals.var_itf_dn7 / locals.var_ick), (((locals.var_itf_dn8 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn8)) / (locals.var_ick * locals.var_ick)), (locals.var_itf_dn9 / locals.var_ick),)
    } else {
        (locals.var_ffitf_ick, locals.var_ffitf_ick_dn0, locals.var_ffitf_ick_dn1, locals.var_ffitf_ick_dn3, locals.var_ffitf_ick_dn4, locals.var_ffitf_ick_dn5, locals.var_ffitf_ick_dn6, locals.var_ffitf_ick_dn7, locals.var_ffitf_ick_dn8, locals.var_ffitf_ick_dn9,)
    }
};
        locals.var_ffitf_ick = assign5730_e5789;
        locals.var_ffitf_ick_dn0 = assign5730_e5789_d_n0;
        locals.var_ffitf_ick_dn1 = assign5730_e5789_d_n1;
        locals.var_ffitf_ick_dn3 = assign5730_e5789_d_n3;
        locals.var_ffitf_ick_dn4 = assign5730_e5789_d_n4;
        locals.var_ffitf_ick_dn5 = assign5730_e5789_d_n5;
        locals.var_ffitf_ick_dn6 = assign5730_e5789_d_n6;
        locals.var_ffitf_ick_dn7 = assign5730_e5789_d_n7;
        locals.var_ffitf_ick_dn8 = assign5730_e5789_d_n8;
        locals.var_ffitf_ick_dn9 = assign5730_e5789_d_n9;

        let (assign5740_e5799, assign5740_e5799_d_n0, assign5740_e5799_d_n1, assign5740_e5799_d_n3, assign5740_e5799_d_n4, assign5740_e5799_d_n5, assign5740_e5799_d_n6, assign5740_e5799_d_n7, assign5740_e5799_d_n8, assign5740_e5799_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign5740_e5794: f64 = (locals.var_ffitf_ick).ln();
        let assign5740_e5795: f64 = (p.p70 * assign5740_e5794);
        let assign5740_e5796: f64 = (assign5740_e5795).exp();
        let assign5740_e5797: f64 = (locals.var_tef0_t * assign5740_e5796);
        (assign5740_e5797, (locals.var_tef0_t * (assign5740_e5796 * (p.p70 * (locals.var_ffitf_ick_dn0 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign5740_e5796 * (p.p70 * (locals.var_ffitf_ick_dn1 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign5740_e5796 * (p.p70 * (locals.var_ffitf_ick_dn3 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign5740_e5796 * (p.p70 * (locals.var_ffitf_ick_dn4 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign5740_e5796 * (p.p70 * (locals.var_ffitf_ick_dn5 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign5740_e5796 * (p.p70 * (locals.var_ffitf_ick_dn6 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign5740_e5796 * (p.p70 * (locals.var_ffitf_ick_dn7 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign5740_e5796 * (p.p70 * (locals.var_ffitf_ick_dn8 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign5740_e5796 * (p.p70 * (locals.var_ffitf_ick_dn9 / locals.var_ffitf_ick)))),)
    } else {
        (locals.var_ffdtef, locals.var_ffdtef_dn0, locals.var_ffdtef_dn1, locals.var_ffdtef_dn3, locals.var_ffdtef_dn4, locals.var_ffdtef_dn5, locals.var_ffdtef_dn6, locals.var_ffdtef_dn7, locals.var_ffdtef_dn8, locals.var_ffdtef_dn9,)
    }
};
        locals.var_ffdtef = assign5740_e5799;
        locals.var_ffdtef_dn0 = assign5740_e5799_d_n0;
        locals.var_ffdtef_dn1 = assign5740_e5799_d_n1;
        locals.var_ffdtef_dn3 = assign5740_e5799_d_n3;
        locals.var_ffdtef_dn4 = assign5740_e5799_d_n4;
        locals.var_ffdtef_dn5 = assign5740_e5799_d_n5;
        locals.var_ffdtef_dn6 = assign5740_e5799_d_n6;
        locals.var_ffdtef_dn7 = assign5740_e5799_d_n7;
        locals.var_ffdtef_dn8 = assign5740_e5799_d_n8;
        locals.var_ffdtef_dn9 = assign5740_e5799_d_n9;

        let (assign5750_e5809, assign5750_e5809_d_n0, assign5750_e5809_d_n1, assign5750_e5809_d_n3, assign5750_e5809_d_n4, assign5750_e5809_d_n5, assign5750_e5809_d_n6, assign5750_e5809_d_n7, assign5750_e5809_d_n8, assign5750_e5809_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign5750_e5803: f64 = (locals.var_ffdtef * locals.var_itf);
        let assign5750_e5806: f64 = (1.0 + p.p70);
        let assign5750_e5807: f64 = (assign5750_e5803 / assign5750_e5806);
        (assign5750_e5807, (((locals.var_ffdtef_dn0 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn0)) / assign5750_e5806), (((locals.var_ffdtef_dn1 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn1)) / assign5750_e5806), (((locals.var_ffdtef_dn3 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn3)) / assign5750_e5806), (((locals.var_ffdtef_dn4 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn4)) / assign5750_e5806), (((locals.var_ffdtef_dn5 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn5)) / assign5750_e5806), (((locals.var_ffdtef_dn6 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn6)) / assign5750_e5806), (((locals.var_ffdtef_dn7 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn7)) / assign5750_e5806), (((locals.var_ffdtef_dn8 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn8)) / assign5750_e5806), (((locals.var_ffdtef_dn9 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn9)) / assign5750_e5806),)
    } else {
        (locals.var_ffdqef, locals.var_ffdqef_dn0, locals.var_ffdqef_dn1, locals.var_ffdqef_dn3, locals.var_ffdqef_dn4, locals.var_ffdqef_dn5, locals.var_ffdqef_dn6, locals.var_ffdqef_dn7, locals.var_ffdqef_dn8, locals.var_ffdqef_dn9,)
    }
};
        locals.var_ffdqef = assign5750_e5809;
        locals.var_ffdqef_dn0 = assign5750_e5809_d_n0;
        locals.var_ffdqef_dn1 = assign5750_e5809_d_n1;
        locals.var_ffdqef_dn3 = assign5750_e5809_d_n3;
        locals.var_ffdqef_dn4 = assign5750_e5809_d_n4;
        locals.var_ffdqef_dn5 = assign5750_e5809_d_n5;
        locals.var_ffdqef_dn6 = assign5750_e5809_d_n6;
        locals.var_ffdqef_dn7 = assign5750_e5809_d_n7;
        locals.var_ffdqef_dn8 = assign5750_e5809_d_n8;
        locals.var_ffdqef_dn9 = assign5750_e5809_d_n9;

        let assign5760_e5814: f64 = (p.p75 / p.p74);
        let assign5760_e5815: f64 = (0.05 * assign5760_e5814);
        let assign5760_e5816: f64 = if p.p83 < assign5760_e5815 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign5760_e5816;

        let (assign5770_e5822, assign5770_e5822_d_n0, assign5770_e5822_d_n1, assign5770_e5822_d_n3, assign5770_e5822_d_n4, assign5770_e5822_d_n5, assign5770_e5822_d_n6, assign5770_e5822_d_n7, assign5770_e5822_d_n8, assign5770_e5822_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard125 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ffdvc, locals.var_ffdvc_dn0, locals.var_ffdvc_dn1, locals.var_ffdvc_dn3, locals.var_ffdvc_dn4, locals.var_ffdvc_dn5, locals.var_ffdvc_dn6, locals.var_ffdvc_dn7, locals.var_ffdvc_dn8, locals.var_ffdvc_dn9,)
    }
};
        locals.var_ffdvc = assign5770_e5822;
        locals.var_ffdvc_dn0 = assign5770_e5822_d_n0;
        locals.var_ffdvc_dn1 = assign5770_e5822_d_n1;
        locals.var_ffdvc_dn3 = assign5770_e5822_d_n3;
        locals.var_ffdvc_dn4 = assign5770_e5822_d_n4;
        locals.var_ffdvc_dn5 = assign5770_e5822_d_n5;
        locals.var_ffdvc_dn6 = assign5770_e5822_d_n6;
        locals.var_ffdvc_dn7 = assign5770_e5822_d_n7;
        locals.var_ffdvc_dn8 = assign5770_e5822_d_n8;
        locals.var_ffdvc_dn9 = assign5770_e5822_d_n9;

        let (assign5780_e5828, assign5780_e5828_d_n0, assign5780_e5828_d_n1, assign5780_e5828_d_n3, assign5780_e5828_d_n4, assign5780_e5828_d_n5, assign5780_e5828_d_n6, assign5780_e5828_d_n7, assign5780_e5828_d_n8, assign5780_e5828_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard125 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ffdvc_ditf, locals.var_ffdvc_ditf_dn0, locals.var_ffdvc_ditf_dn1, locals.var_ffdvc_ditf_dn3, locals.var_ffdvc_ditf_dn4, locals.var_ffdvc_ditf_dn5, locals.var_ffdvc_ditf_dn6, locals.var_ffdvc_ditf_dn7, locals.var_ffdvc_ditf_dn8, locals.var_ffdvc_ditf_dn9,)
    }
};
        locals.var_ffdvc_ditf = assign5780_e5828;
        locals.var_ffdvc_ditf_dn0 = assign5780_e5828_d_n0;
        locals.var_ffdvc_ditf_dn1 = assign5780_e5828_d_n1;
        locals.var_ffdvc_ditf_dn3 = assign5780_e5828_d_n3;
        locals.var_ffdvc_ditf_dn4 = assign5780_e5828_d_n4;
        locals.var_ffdvc_ditf_dn5 = assign5780_e5828_d_n5;
        locals.var_ffdvc_ditf_dn6 = assign5780_e5828_d_n6;
        locals.var_ffdvc_ditf_dn7 = assign5780_e5828_d_n7;
        locals.var_ffdvc_ditf_dn8 = assign5780_e5828_d_n8;
        locals.var_ffdvc_ditf_dn9 = assign5780_e5828_d_n9;

        let (assign5790_e5839, assign5790_e5839_d_n0, assign5790_e5839_d_n1, assign5790_e5839_d_n3, assign5790_e5839_d_n4, assign5790_e5839_d_n5, assign5790_e5839_d_n6, assign5790_e5839_d_n7, assign5790_e5839_d_n8, assign5790_e5839_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard125 == 0.0)) {
        let assign5790_e5835: f64 = (locals.var_itf - locals.var_ick);
        let assign5790_e5837: f64 = (assign5790_e5835 / p.p83);
        (assign5790_e5837, (locals.var_itf_dn0 / p.p83), (locals.var_itf_dn1 / p.p83), (locals.var_itf_dn3 / p.p83), ((locals.var_itf_dn4 - locals.var_ick_dn4) / p.p83), ((locals.var_itf_dn5 - locals.var_ick_dn5) / p.p83), ((locals.var_itf_dn6 - locals.var_ick_dn6) / p.p83), (locals.var_itf_dn7 / p.p83), ((locals.var_itf_dn8 - locals.var_ick_dn8) / p.p83), (locals.var_itf_dn9 / p.p83),)
    } else {
        (locals.var_ffib, locals.var_ffib_dn0, locals.var_ffib_dn1, locals.var_ffib_dn3, locals.var_ffib_dn4, locals.var_ffib_dn5, locals.var_ffib_dn6, locals.var_ffib_dn7, locals.var_ffib_dn8, locals.var_ffib_dn9,)
    }
};
        locals.var_ffib = assign5790_e5839;
        locals.var_ffib_dn0 = assign5790_e5839_d_n0;
        locals.var_ffib_dn1 = assign5790_e5839_d_n1;
        locals.var_ffib_dn3 = assign5790_e5839_d_n3;
        locals.var_ffib_dn4 = assign5790_e5839_d_n4;
        locals.var_ffib_dn5 = assign5790_e5839_d_n5;
        locals.var_ffib_dn6 = assign5790_e5839_d_n6;
        locals.var_ffib_dn7 = assign5790_e5839_d_n7;
        locals.var_ffib_dn8 = assign5790_e5839_d_n8;
        locals.var_ffib_dn9 = assign5790_e5839_d_n9;

        let assign5800_e5842: f64 = (-10000000000.0);
        let assign5800_e5843: f64 = if locals.var_ffib < assign5800_e5842 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign5800_e5843;

        let (assign5810_e5853, assign5810_e5853_d_n0, assign5810_e5853_d_n1, assign5810_e5853_d_n3, assign5810_e5853_d_n4, assign5810_e5853_d_n5, assign5810_e5853_d_n6, assign5810_e5853_d_n7, assign5810_e5853_d_n8, assign5810_e5853_d_n9,) = {
    if (((locals.var_guard124 != 0.0) && (locals.var_guard125 == 0.0)) && (locals.var_guard126 != 0.0)) {
        let assign5810_e5851: f64 = (-10000000000.0);
        (assign5810_e5851, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ffib, locals.var_ffib_dn0, locals.var_ffib_dn1, locals.var_ffib_dn3, locals.var_ffib_dn4, locals.var_ffib_dn5, locals.var_ffib_dn6, locals.var_ffib_dn7, locals.var_ffib_dn8, locals.var_ffib_dn9,)
    }
};
        locals.var_ffib = assign5810_e5853;
        locals.var_ffib_dn0 = assign5810_e5853_d_n0;
        locals.var_ffib_dn1 = assign5810_e5853_d_n1;
        locals.var_ffib_dn3 = assign5810_e5853_d_n3;
        locals.var_ffib_dn4 = assign5810_e5853_d_n4;
        locals.var_ffib_dn5 = assign5810_e5853_d_n5;
        locals.var_ffib_dn6 = assign5810_e5853_d_n6;
        locals.var_ffib_dn7 = assign5810_e5853_d_n7;
        locals.var_ffib_dn8 = assign5810_e5853_d_n8;
        locals.var_ffib_dn9 = assign5810_e5853_d_n9;

        let (assign5820_e5865, assign5820_e5865_d_n0, assign5820_e5865_d_n1, assign5820_e5865_d_n3, assign5820_e5865_d_n4, assign5820_e5865_d_n5, assign5820_e5865_d_n6, assign5820_e5865_d_n7, assign5820_e5865_d_n8, assign5820_e5865_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard125 == 0.0)) {
        let assign5820_e5860: f64 = (locals.var_ffib * locals.var_ffib);
        let assign5820_e5862: f64 = (assign5820_e5860 + p.p84);
        let assign5820_e5863: f64 = (assign5820_e5862).sqrt();
        (assign5820_e5863, (((locals.var_ffib_dn0 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn0)) / (2.0 * assign5820_e5863)), (((locals.var_ffib_dn1 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn1)) / (2.0 * assign5820_e5863)), (((locals.var_ffib_dn3 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn3)) / (2.0 * assign5820_e5863)), (((locals.var_ffib_dn4 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn4)) / (2.0 * assign5820_e5863)), (((locals.var_ffib_dn5 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn5)) / (2.0 * assign5820_e5863)), (((locals.var_ffib_dn6 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn6)) / (2.0 * assign5820_e5863)), (((locals.var_ffib_dn7 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn7)) / (2.0 * assign5820_e5863)), (((locals.var_ffib_dn8 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn8)) / (2.0 * assign5820_e5863)), (((locals.var_ffib_dn9 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn9)) / (2.0 * assign5820_e5863)),)
    } else {
        (locals.var_fffcbar, locals.var_fffcbar_dn0, locals.var_fffcbar_dn1, locals.var_fffcbar_dn3, locals.var_fffcbar_dn4, locals.var_fffcbar_dn5, locals.var_fffcbar_dn6, locals.var_fffcbar_dn7, locals.var_fffcbar_dn8, locals.var_fffcbar_dn9,)
    }
};
        locals.var_fffcbar = assign5820_e5865;
        locals.var_fffcbar_dn0 = assign5820_e5865_d_n0;
        locals.var_fffcbar_dn1 = assign5820_e5865_d_n1;
        locals.var_fffcbar_dn3 = assign5820_e5865_d_n3;
        locals.var_fffcbar_dn4 = assign5820_e5865_d_n4;
        locals.var_fffcbar_dn5 = assign5820_e5865_d_n5;
        locals.var_fffcbar_dn6 = assign5820_e5865_d_n6;
        locals.var_fffcbar_dn7 = assign5820_e5865_d_n7;
        locals.var_fffcbar_dn8 = assign5820_e5865_d_n8;
        locals.var_fffcbar_dn9 = assign5820_e5865_d_n9;

        let (assign5830_e5880, assign5830_e5880_d_n0, assign5830_e5880_d_n1, assign5830_e5880_d_n3, assign5830_e5880_d_n4, assign5830_e5880_d_n5, assign5830_e5880_d_n6, assign5830_e5880_d_n7, assign5830_e5880_d_n8, assign5830_e5880_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard125 == 0.0)) {
        let assign5830_e5872: f64 = (-2.0);
        let assign5830_e5875: f64 = (locals.var_ffib + locals.var_fffcbar);
        let assign5830_e5876: f64 = (assign5830_e5872 / assign5830_e5875);
        let assign5830_e5877: f64 = (assign5830_e5876).exp();
        let assign5830_e5878: f64 = (p.p82 * assign5830_e5877);
        (assign5830_e5878, (p.p82 * (assign5830_e5877 * (-((assign5830_e5872 * (locals.var_ffib_dn0 + locals.var_fffcbar_dn0)) / (assign5830_e5875 * assign5830_e5875))))), (p.p82 * (assign5830_e5877 * (-((assign5830_e5872 * (locals.var_ffib_dn1 + locals.var_fffcbar_dn1)) / (assign5830_e5875 * assign5830_e5875))))), (p.p82 * (assign5830_e5877 * (-((assign5830_e5872 * (locals.var_ffib_dn3 + locals.var_fffcbar_dn3)) / (assign5830_e5875 * assign5830_e5875))))), (p.p82 * (assign5830_e5877 * (-((assign5830_e5872 * (locals.var_ffib_dn4 + locals.var_fffcbar_dn4)) / (assign5830_e5875 * assign5830_e5875))))), (p.p82 * (assign5830_e5877 * (-((assign5830_e5872 * (locals.var_ffib_dn5 + locals.var_fffcbar_dn5)) / (assign5830_e5875 * assign5830_e5875))))), (p.p82 * (assign5830_e5877 * (-((assign5830_e5872 * (locals.var_ffib_dn6 + locals.var_fffcbar_dn6)) / (assign5830_e5875 * assign5830_e5875))))), (p.p82 * (assign5830_e5877 * (-((assign5830_e5872 * (locals.var_ffib_dn7 + locals.var_fffcbar_dn7)) / (assign5830_e5875 * assign5830_e5875))))), (p.p82 * (assign5830_e5877 * (-((assign5830_e5872 * (locals.var_ffib_dn8 + locals.var_fffcbar_dn8)) / (assign5830_e5875 * assign5830_e5875))))), (p.p82 * (assign5830_e5877 * (-((assign5830_e5872 * (locals.var_ffib_dn9 + locals.var_fffcbar_dn9)) / (assign5830_e5875 * assign5830_e5875))))),)
    } else {
        (locals.var_ffdvc, locals.var_ffdvc_dn0, locals.var_ffdvc_dn1, locals.var_ffdvc_dn3, locals.var_ffdvc_dn4, locals.var_ffdvc_dn5, locals.var_ffdvc_dn6, locals.var_ffdvc_dn7, locals.var_ffdvc_dn8, locals.var_ffdvc_dn9,)
    }
};
        locals.var_ffdvc = assign5830_e5880;
        locals.var_ffdvc_dn0 = assign5830_e5880_d_n0;
        locals.var_ffdvc_dn1 = assign5830_e5880_d_n1;
        locals.var_ffdvc_dn3 = assign5830_e5880_d_n3;
        locals.var_ffdvc_dn4 = assign5830_e5880_d_n4;
        locals.var_ffdvc_dn5 = assign5830_e5880_d_n5;
        locals.var_ffdvc_dn6 = assign5830_e5880_d_n6;
        locals.var_ffdvc_dn7 = assign5830_e5880_d_n7;
        locals.var_ffdvc_dn8 = assign5830_e5880_d_n8;
        locals.var_ffdvc_dn9 = assign5830_e5880_d_n9;

        let (assign5840_e5897, assign5840_e5897_d_n0, assign5840_e5897_d_n1, assign5840_e5897_d_n3, assign5840_e5897_d_n4, assign5840_e5897_d_n5, assign5840_e5897_d_n6, assign5840_e5897_d_n7, assign5840_e5897_d_n8, assign5840_e5897_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard125 == 0.0)) {
        let assign5840_e5887: f64 = (2.0 * locals.var_ffdvc);
        let assign5840_e5890: f64 = (p.p83 * locals.var_fffcbar);
        let assign5840_e5893: f64 = (locals.var_ffib + locals.var_fffcbar);
        let assign5840_e5894: f64 = (assign5840_e5890 * assign5840_e5893);
        let assign5840_e5895: f64 = (assign5840_e5887 / assign5840_e5894);
        (assign5840_e5895, ((((2.0 * locals.var_ffdvc_dn0) * assign5840_e5894) - (assign5840_e5887 * (((p.p83 * locals.var_fffcbar_dn0) * assign5840_e5893) + (assign5840_e5890 * (locals.var_ffib_dn0 + locals.var_fffcbar_dn0))))) / (assign5840_e5894 * assign5840_e5894)), ((((2.0 * locals.var_ffdvc_dn1) * assign5840_e5894) - (assign5840_e5887 * (((p.p83 * locals.var_fffcbar_dn1) * assign5840_e5893) + (assign5840_e5890 * (locals.var_ffib_dn1 + locals.var_fffcbar_dn1))))) / (assign5840_e5894 * assign5840_e5894)), ((((2.0 * locals.var_ffdvc_dn3) * assign5840_e5894) - (assign5840_e5887 * (((p.p83 * locals.var_fffcbar_dn3) * assign5840_e5893) + (assign5840_e5890 * (locals.var_ffib_dn3 + locals.var_fffcbar_dn3))))) / (assign5840_e5894 * assign5840_e5894)), ((((2.0 * locals.var_ffdvc_dn4) * assign5840_e5894) - (assign5840_e5887 * (((p.p83 * locals.var_fffcbar_dn4) * assign5840_e5893) + (assign5840_e5890 * (locals.var_ffib_dn4 + locals.var_fffcbar_dn4))))) / (assign5840_e5894 * assign5840_e5894)), ((((2.0 * locals.var_ffdvc_dn5) * assign5840_e5894) - (assign5840_e5887 * (((p.p83 * locals.var_fffcbar_dn5) * assign5840_e5893) + (assign5840_e5890 * (locals.var_ffib_dn5 + locals.var_fffcbar_dn5))))) / (assign5840_e5894 * assign5840_e5894)), ((((2.0 * locals.var_ffdvc_dn6) * assign5840_e5894) - (assign5840_e5887 * (((p.p83 * locals.var_fffcbar_dn6) * assign5840_e5893) + (assign5840_e5890 * (locals.var_ffib_dn6 + locals.var_fffcbar_dn6))))) / (assign5840_e5894 * assign5840_e5894)), ((((2.0 * locals.var_ffdvc_dn7) * assign5840_e5894) - (assign5840_e5887 * (((p.p83 * locals.var_fffcbar_dn7) * assign5840_e5893) + (assign5840_e5890 * (locals.var_ffib_dn7 + locals.var_fffcbar_dn7))))) / (assign5840_e5894 * assign5840_e5894)), ((((2.0 * locals.var_ffdvc_dn8) * assign5840_e5894) - (assign5840_e5887 * (((p.p83 * locals.var_fffcbar_dn8) * assign5840_e5893) + (assign5840_e5890 * (locals.var_ffib_dn8 + locals.var_fffcbar_dn8))))) / (assign5840_e5894 * assign5840_e5894)), ((((2.0 * locals.var_ffdvc_dn9) * assign5840_e5894) - (assign5840_e5887 * (((p.p83 * locals.var_fffcbar_dn9) * assign5840_e5893) + (assign5840_e5890 * (locals.var_ffib_dn9 + locals.var_fffcbar_dn9))))) / (assign5840_e5894 * assign5840_e5894)),)
    } else {
        (locals.var_ffdvc_ditf, locals.var_ffdvc_ditf_dn0, locals.var_ffdvc_ditf_dn1, locals.var_ffdvc_ditf_dn3, locals.var_ffdvc_ditf_dn4, locals.var_ffdvc_ditf_dn5, locals.var_ffdvc_ditf_dn6, locals.var_ffdvc_ditf_dn7, locals.var_ffdvc_ditf_dn8, locals.var_ffdvc_ditf_dn9,)
    }
};
        locals.var_ffdvc_ditf = assign5840_e5897;
        locals.var_ffdvc_ditf_dn0 = assign5840_e5897_d_n0;
        locals.var_ffdvc_ditf_dn1 = assign5840_e5897_d_n1;
        locals.var_ffdvc_ditf_dn3 = assign5840_e5897_d_n3;
        locals.var_ffdvc_ditf_dn4 = assign5840_e5897_d_n4;
        locals.var_ffdvc_ditf_dn5 = assign5840_e5897_d_n5;
        locals.var_ffdvc_ditf_dn6 = assign5840_e5897_d_n6;
        locals.var_ffdvc_ditf_dn7 = assign5840_e5897_d_n7;
        locals.var_ffdvc_ditf_dn8 = assign5840_e5897_d_n8;
        locals.var_ffdvc_ditf_dn9 = assign5840_e5897_d_n9;

        let (assign5850_e5912, assign5850_e5912_d_n0, assign5850_e5912_d_n1, assign5850_e5912_d_n3, assign5850_e5912_d_n4, assign5850_e5912_d_n5, assign5850_e5912_d_n6, assign5850_e5912_d_n7, assign5850_e5912_d_n8, assign5850_e5912_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign5850_e5901: f64 = (1.0 - p.p73);
        let assign5850_e5903: f64 = (assign5850_e5901 * locals.var_thcs_t);
        let assign5850_e5906: f64 = (locals.var_ffdvc * locals.var_ovt);
        let assign5850_e5907: f64 = (assign5850_e5906).exp();
        let assign5850_e5909: f64 = (assign5850_e5907 - 1.0);
        let assign5850_e5910: f64 = (assign5850_e5903 * assign5850_e5909);
        (assign5850_e5910, (assign5850_e5903 * (assign5850_e5907 * (locals.var_ffdvc_dn0 * locals.var_ovt))), (assign5850_e5903 * (assign5850_e5907 * (locals.var_ffdvc_dn1 * locals.var_ovt))), (assign5850_e5903 * (assign5850_e5907 * (locals.var_ffdvc_dn3 * locals.var_ovt))), (((assign5850_e5901 * locals.var_thcs_t_dn4) * assign5850_e5909) + (assign5850_e5903 * (assign5850_e5907 * ((locals.var_ffdvc_dn4 * locals.var_ovt) + (locals.var_ffdvc * locals.var_ovt_dn4))))), (assign5850_e5903 * (assign5850_e5907 * (locals.var_ffdvc_dn5 * locals.var_ovt))), (assign5850_e5903 * (assign5850_e5907 * (locals.var_ffdvc_dn6 * locals.var_ovt))), (assign5850_e5903 * (assign5850_e5907 * (locals.var_ffdvc_dn7 * locals.var_ovt))), (assign5850_e5903 * (assign5850_e5907 * (locals.var_ffdvc_dn8 * locals.var_ovt))), (assign5850_e5903 * (assign5850_e5907 * (locals.var_ffdvc_dn9 * locals.var_ovt))),)
    } else {
        (locals.var_ffdqbfb, locals.var_ffdqbfb_dn0, locals.var_ffdqbfb_dn1, locals.var_ffdqbfb_dn3, locals.var_ffdqbfb_dn4, locals.var_ffdqbfb_dn5, locals.var_ffdqbfb_dn6, locals.var_ffdqbfb_dn7, locals.var_ffdqbfb_dn8, locals.var_ffdqbfb_dn9,)
    }
};
        locals.var_ffdqbfb = assign5850_e5912;
        locals.var_ffdqbfb_dn0 = assign5850_e5912_d_n0;
        locals.var_ffdqbfb_dn1 = assign5850_e5912_d_n1;
        locals.var_ffdqbfb_dn3 = assign5850_e5912_d_n3;
        locals.var_ffdqbfb_dn4 = assign5850_e5912_d_n4;
        locals.var_ffdqbfb_dn5 = assign5850_e5912_d_n5;
        locals.var_ffdqbfb_dn6 = assign5850_e5912_d_n6;
        locals.var_ffdqbfb_dn7 = assign5850_e5912_d_n7;
        locals.var_ffdqbfb_dn8 = assign5850_e5912_d_n8;
        locals.var_ffdqbfb_dn9 = assign5850_e5912_d_n9;

        let (assign5860_e5933, assign5860_e5933_d_n0, assign5860_e5933_d_n1, assign5860_e5933_d_n3, assign5860_e5933_d_n4, assign5860_e5933_d_n5, assign5860_e5933_d_n6, assign5860_e5933_d_n7, assign5860_e5933_d_n8, assign5860_e5933_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign5860_e5917: f64 = (1.0 - p.p73);
        let assign5860_e5919: f64 = (assign5860_e5917 * locals.var_thcs_t);
        let assign5860_e5921: f64 = (assign5860_e5919 * locals.var_itf);
        let assign5860_e5924: f64 = (locals.var_ffdvc * locals.var_ovt);
        let assign5860_e5925: f64 = (assign5860_e5924).exp();
        let assign5860_e5926: f64 = (assign5860_e5921 * assign5860_e5925);
        let assign5860_e5928: f64 = (assign5860_e5926 * locals.var_ovt);
        let assign5860_e5930: f64 = (assign5860_e5928 * locals.var_ffdvc_ditf);
        let assign5860_e5931: f64 = (locals.var_ffdqbfb + assign5860_e5930);
        (assign5860_e5931, (locals.var_ffdqbfb_dn0 + ((((((assign5860_e5919 * locals.var_itf_dn0) * assign5860_e5925) + (assign5860_e5921 * (assign5860_e5925 * (locals.var_ffdvc_dn0 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign5860_e5928 * locals.var_ffdvc_ditf_dn0))), (locals.var_ffdqbfb_dn1 + ((((((assign5860_e5919 * locals.var_itf_dn1) * assign5860_e5925) + (assign5860_e5921 * (assign5860_e5925 * (locals.var_ffdvc_dn1 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign5860_e5928 * locals.var_ffdvc_ditf_dn1))), (locals.var_ffdqbfb_dn3 + ((((((assign5860_e5919 * locals.var_itf_dn3) * assign5860_e5925) + (assign5860_e5921 * (assign5860_e5925 * (locals.var_ffdvc_dn3 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign5860_e5928 * locals.var_ffdvc_ditf_dn3))), (locals.var_ffdqbfb_dn4 + (((((((((assign5860_e5917 * locals.var_thcs_t_dn4) * locals.var_itf) + (assign5860_e5919 * locals.var_itf_dn4)) * assign5860_e5925) + (assign5860_e5921 * (assign5860_e5925 * ((locals.var_ffdvc_dn4 * locals.var_ovt) + (locals.var_ffdvc * locals.var_ovt_dn4))))) * locals.var_ovt) + (assign5860_e5926 * locals.var_ovt_dn4)) * locals.var_ffdvc_ditf) + (assign5860_e5928 * locals.var_ffdvc_ditf_dn4))), (locals.var_ffdqbfb_dn5 + ((((((assign5860_e5919 * locals.var_itf_dn5) * assign5860_e5925) + (assign5860_e5921 * (assign5860_e5925 * (locals.var_ffdvc_dn5 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign5860_e5928 * locals.var_ffdvc_ditf_dn5))), (locals.var_ffdqbfb_dn6 + ((((((assign5860_e5919 * locals.var_itf_dn6) * assign5860_e5925) + (assign5860_e5921 * (assign5860_e5925 * (locals.var_ffdvc_dn6 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign5860_e5928 * locals.var_ffdvc_ditf_dn6))), (locals.var_ffdqbfb_dn7 + ((((((assign5860_e5919 * locals.var_itf_dn7) * assign5860_e5925) + (assign5860_e5921 * (assign5860_e5925 * (locals.var_ffdvc_dn7 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign5860_e5928 * locals.var_ffdvc_ditf_dn7))), (locals.var_ffdqbfb_dn8 + ((((((assign5860_e5919 * locals.var_itf_dn8) * assign5860_e5925) + (assign5860_e5921 * (assign5860_e5925 * (locals.var_ffdvc_dn8 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign5860_e5928 * locals.var_ffdvc_ditf_dn8))), (locals.var_ffdqbfb_dn9 + ((((((assign5860_e5919 * locals.var_itf_dn9) * assign5860_e5925) + (assign5860_e5921 * (assign5860_e5925 * (locals.var_ffdvc_dn9 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign5860_e5928 * locals.var_ffdvc_ditf_dn9))),)
    } else {
        (locals.var_ffdtbfb, locals.var_ffdtbfb_dn0, locals.var_ffdtbfb_dn1, locals.var_ffdtbfb_dn3, locals.var_ffdtbfb_dn4, locals.var_ffdtbfb_dn5, locals.var_ffdtbfb_dn6, locals.var_ffdtbfb_dn7, locals.var_ffdtbfb_dn8, locals.var_ffdtbfb_dn9,)
    }
};
        locals.var_ffdtbfb = assign5860_e5933;
        locals.var_ffdtbfb_dn0 = assign5860_e5933_d_n0;
        locals.var_ffdtbfb_dn1 = assign5860_e5933_d_n1;
        locals.var_ffdtbfb_dn3 = assign5860_e5933_d_n3;
        locals.var_ffdtbfb_dn4 = assign5860_e5933_d_n4;
        locals.var_ffdtbfb_dn5 = assign5860_e5933_d_n5;
        locals.var_ffdtbfb_dn6 = assign5860_e5933_d_n6;
        locals.var_ffdtbfb_dn7 = assign5860_e5933_d_n7;
        locals.var_ffdtbfb_dn8 = assign5860_e5933_d_n8;
        locals.var_ffdtbfb_dn9 = assign5860_e5933_d_n9;

        let (assign5870_e5941, assign5870_e5941_d_n0, assign5870_e5941_d_n1, assign5870_e5941_d_n3, assign5870_e5941_d_n4, assign5870_e5941_d_n5, assign5870_e5941_d_n6, assign5870_e5941_d_n7, assign5870_e5941_d_n8, assign5870_e5941_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign5870_e5938: f64 = (1.0 / locals.var_ffitf_ick);
        let assign5870_e5939: f64 = (1.0 - assign5870_e5938);
        (assign5870_e5939, (-(-(locals.var_ffitf_ick_dn0 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn1 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn3 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn4 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn5 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn6 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn7 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn8 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn9 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))),)
    } else {
        (locals.var_ffic, locals.var_ffic_dn0, locals.var_ffic_dn1, locals.var_ffic_dn3, locals.var_ffic_dn4, locals.var_ffic_dn5, locals.var_ffic_dn6, locals.var_ffic_dn7, locals.var_ffic_dn8, locals.var_ffic_dn9,)
    }
};
        locals.var_ffic = assign5870_e5941;
        locals.var_ffic_dn0 = assign5870_e5941_d_n0;
        locals.var_ffic_dn1 = assign5870_e5941_d_n1;
        locals.var_ffic_dn3 = assign5870_e5941_d_n3;
        locals.var_ffic_dn4 = assign5870_e5941_d_n4;
        locals.var_ffic_dn5 = assign5870_e5941_d_n5;
        locals.var_ffic_dn6 = assign5870_e5941_d_n6;
        locals.var_ffic_dn7 = assign5870_e5941_d_n7;
        locals.var_ffic_dn8 = assign5870_e5941_d_n8;
        locals.var_ffic_dn9 = assign5870_e5941_d_n9;

        let (assign5880_e5959, assign5880_e5959_d_n0, assign5880_e5959_d_n1, assign5880_e5959_d_n3, assign5880_e5959_d_n4, assign5880_e5959_d_n5, assign5880_e5959_d_n6, assign5880_e5959_d_n7, assign5880_e5959_d_n8, assign5880_e5959_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign5880_e5946: f64 = (locals.var_ffic * locals.var_ffic);
        let assign5880_e5948: f64 = (assign5880_e5946 + p.p72);
        let assign5880_e5949: f64 = (assign5880_e5948).sqrt();
        let assign5880_e5950: f64 = (locals.var_ffic + assign5880_e5949);
        let assign5880_e5954: f64 = (1.0 + p.p72);
        let assign5880_e5955: f64 = (assign5880_e5954).sqrt();
        let assign5880_e5956: f64 = (1.0 + assign5880_e5955);
        let assign5880_e5957: f64 = (assign5880_e5950 / assign5880_e5956);
        (assign5880_e5957, ((locals.var_ffic_dn0 + (((locals.var_ffic_dn0 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn0)) / (2.0 * assign5880_e5949))) / assign5880_e5956), ((locals.var_ffic_dn1 + (((locals.var_ffic_dn1 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn1)) / (2.0 * assign5880_e5949))) / assign5880_e5956), ((locals.var_ffic_dn3 + (((locals.var_ffic_dn3 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn3)) / (2.0 * assign5880_e5949))) / assign5880_e5956), ((locals.var_ffic_dn4 + (((locals.var_ffic_dn4 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn4)) / (2.0 * assign5880_e5949))) / assign5880_e5956), ((locals.var_ffic_dn5 + (((locals.var_ffic_dn5 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn5)) / (2.0 * assign5880_e5949))) / assign5880_e5956), ((locals.var_ffic_dn6 + (((locals.var_ffic_dn6 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn6)) / (2.0 * assign5880_e5949))) / assign5880_e5956), ((locals.var_ffic_dn7 + (((locals.var_ffic_dn7 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn7)) / (2.0 * assign5880_e5949))) / assign5880_e5956), ((locals.var_ffic_dn8 + (((locals.var_ffic_dn8 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn8)) / (2.0 * assign5880_e5949))) / assign5880_e5956), ((locals.var_ffic_dn9 + (((locals.var_ffic_dn9 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn9)) / (2.0 * assign5880_e5949))) / assign5880_e5956),)
    } else {
        (locals.var_ffw, locals.var_ffw_dn0, locals.var_ffw_dn1, locals.var_ffw_dn3, locals.var_ffw_dn4, locals.var_ffw_dn5, locals.var_ffw_dn6, locals.var_ffw_dn7, locals.var_ffw_dn8, locals.var_ffw_dn9,)
    }
};
        locals.var_ffw = assign5880_e5959;
        locals.var_ffw_dn0 = assign5880_e5959_d_n0;
        locals.var_ffw_dn1 = assign5880_e5959_d_n1;
        locals.var_ffw_dn3 = assign5880_e5959_d_n3;
        locals.var_ffw_dn4 = assign5880_e5959_d_n4;
        locals.var_ffw_dn5 = assign5880_e5959_d_n5;
        locals.var_ffw_dn6 = assign5880_e5959_d_n6;
        locals.var_ffw_dn7 = assign5880_e5959_d_n7;
        locals.var_ffw_dn8 = assign5880_e5959_d_n8;
        locals.var_ffw_dn9 = assign5880_e5959_d_n9;

        let (assign5890_e5968, assign5890_e5968_d_n0, assign5890_e5968_d_n1, assign5890_e5968_d_n3, assign5890_e5968_d_n4, assign5890_e5968_d_n5, assign5890_e5968_d_n6, assign5890_e5968_d_n7, assign5890_e5968_d_n8, assign5890_e5968_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign5890_e5963: f64 = (locals.var_ffdvc - p.p82);
        let assign5890_e5965: f64 = (assign5890_e5963 * locals.var_ovt);
        let assign5890_e5966: f64 = (assign5890_e5965).exp();
        (assign5890_e5966, (assign5890_e5966 * (locals.var_ffdvc_dn0 * locals.var_ovt)), (assign5890_e5966 * (locals.var_ffdvc_dn1 * locals.var_ovt)), (assign5890_e5966 * (locals.var_ffdvc_dn3 * locals.var_ovt)), (assign5890_e5966 * ((locals.var_ffdvc_dn4 * locals.var_ovt) + (assign5890_e5963 * locals.var_ovt_dn4))), (assign5890_e5966 * (locals.var_ffdvc_dn5 * locals.var_ovt)), (assign5890_e5966 * (locals.var_ffdvc_dn6 * locals.var_ovt)), (assign5890_e5966 * (locals.var_ffdvc_dn7 * locals.var_ovt)), (assign5890_e5966 * (locals.var_ffdvc_dn8 * locals.var_ovt)), (assign5890_e5966 * (locals.var_ffdvc_dn9 * locals.var_ovt)),)
    } else {
        (locals.var_ffvc_exp, locals.var_ffvc_exp_dn0, locals.var_ffvc_exp_dn1, locals.var_ffvc_exp_dn3, locals.var_ffvc_exp_dn4, locals.var_ffvc_exp_dn5, locals.var_ffvc_exp_dn6, locals.var_ffvc_exp_dn7, locals.var_ffvc_exp_dn8, locals.var_ffvc_exp_dn9,)
    }
};
        locals.var_ffvc_exp = assign5890_e5968;
        locals.var_ffvc_exp_dn0 = assign5890_e5968_d_n0;
        locals.var_ffvc_exp_dn1 = assign5890_e5968_d_n1;
        locals.var_ffvc_exp_dn3 = assign5890_e5968_d_n3;
        locals.var_ffvc_exp_dn4 = assign5890_e5968_d_n4;
        locals.var_ffvc_exp_dn5 = assign5890_e5968_d_n5;
        locals.var_ffvc_exp_dn6 = assign5890_e5968_d_n6;
        locals.var_ffvc_exp_dn7 = assign5890_e5968_d_n7;
        locals.var_ffvc_exp_dn8 = assign5890_e5968_d_n8;
        locals.var_ffvc_exp_dn9 = assign5890_e5968_d_n9;

        let (assign5900_e5978, assign5900_e5978_d_n0, assign5900_e5978_d_n1, assign5900_e5978_d_n3, assign5900_e5978_d_n4, assign5900_e5978_d_n5, assign5900_e5978_d_n6, assign5900_e5978_d_n7, assign5900_e5978_d_n8, assign5900_e5978_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign5900_e5972: f64 = (locals.var_thcs_t * locals.var_ffw);
        let assign5900_e5974: f64 = (assign5900_e5972 * locals.var_ffw);
        let assign5900_e5976: f64 = (assign5900_e5974 * locals.var_ffvc_exp);
        (assign5900_e5976, (((((locals.var_thcs_t * locals.var_ffw_dn0) * locals.var_ffw) + (assign5900_e5972 * locals.var_ffw_dn0)) * locals.var_ffvc_exp) + (assign5900_e5974 * locals.var_ffvc_exp_dn0)), (((((locals.var_thcs_t * locals.var_ffw_dn1) * locals.var_ffw) + (assign5900_e5972 * locals.var_ffw_dn1)) * locals.var_ffvc_exp) + (assign5900_e5974 * locals.var_ffvc_exp_dn1)), (((((locals.var_thcs_t * locals.var_ffw_dn3) * locals.var_ffw) + (assign5900_e5972 * locals.var_ffw_dn3)) * locals.var_ffvc_exp) + (assign5900_e5974 * locals.var_ffvc_exp_dn3)), ((((((locals.var_thcs_t_dn4 * locals.var_ffw) + (locals.var_thcs_t * locals.var_ffw_dn4)) * locals.var_ffw) + (assign5900_e5972 * locals.var_ffw_dn4)) * locals.var_ffvc_exp) + (assign5900_e5974 * locals.var_ffvc_exp_dn4)), (((((locals.var_thcs_t * locals.var_ffw_dn5) * locals.var_ffw) + (assign5900_e5972 * locals.var_ffw_dn5)) * locals.var_ffvc_exp) + (assign5900_e5974 * locals.var_ffvc_exp_dn5)), (((((locals.var_thcs_t * locals.var_ffw_dn6) * locals.var_ffw) + (assign5900_e5972 * locals.var_ffw_dn6)) * locals.var_ffvc_exp) + (assign5900_e5974 * locals.var_ffvc_exp_dn6)), (((((locals.var_thcs_t * locals.var_ffw_dn7) * locals.var_ffw) + (assign5900_e5972 * locals.var_ffw_dn7)) * locals.var_ffvc_exp) + (assign5900_e5974 * locals.var_ffvc_exp_dn7)), (((((locals.var_thcs_t * locals.var_ffw_dn8) * locals.var_ffw) + (assign5900_e5972 * locals.var_ffw_dn8)) * locals.var_ffvc_exp) + (assign5900_e5974 * locals.var_ffvc_exp_dn8)), (((((locals.var_thcs_t * locals.var_ffw_dn9) * locals.var_ffw) + (assign5900_e5972 * locals.var_ffw_dn9)) * locals.var_ffvc_exp) + (assign5900_e5974 * locals.var_ffvc_exp_dn9)),)
    } else {
        (locals.var_ffdqfhc, locals.var_ffdqfhc_dn0, locals.var_ffdqfhc_dn1, locals.var_ffdqfhc_dn3, locals.var_ffdqfhc_dn4, locals.var_ffdqfhc_dn5, locals.var_ffdqfhc_dn6, locals.var_ffdqfhc_dn7, locals.var_ffdqfhc_dn8, locals.var_ffdqfhc_dn9,)
    }
};
        locals.var_ffdqfhc = assign5900_e5978;
        locals.var_ffdqfhc_dn0 = assign5900_e5978_d_n0;
        locals.var_ffdqfhc_dn1 = assign5900_e5978_d_n1;
        locals.var_ffdqfhc_dn3 = assign5900_e5978_d_n3;
        locals.var_ffdqfhc_dn4 = assign5900_e5978_d_n4;
        locals.var_ffdqfhc_dn5 = assign5900_e5978_d_n5;
        locals.var_ffdqfhc_dn6 = assign5900_e5978_d_n6;
        locals.var_ffdqfhc_dn7 = assign5900_e5978_d_n7;
        locals.var_ffdqfhc_dn8 = assign5900_e5978_d_n8;
        locals.var_ffdqfhc_dn9 = assign5900_e5978_d_n9;

        let (assign5910_e6001, assign5910_e6001_d_n0, assign5910_e6001_d_n1, assign5910_e6001_d_n3, assign5910_e6001_d_n4, assign5910_e6001_d_n5, assign5910_e6001_d_n6, assign5910_e6001_d_n7, assign5910_e6001_d_n8, assign5910_e6001_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign5910_e5986: f64 = (locals.var_ffic * locals.var_ffic);
        let assign5910_e5988: f64 = (assign5910_e5986 + p.p72);
        let assign5910_e5989: f64 = (assign5910_e5988).sqrt();
        let assign5910_e5990: f64 = (locals.var_ffitf_ick * assign5910_e5989);
        let assign5910_e5991: f64 = (2.0 / assign5910_e5990);
        let assign5910_e5992: f64 = (1.0 + assign5910_e5991);
        let assign5910_e5995: f64 = (locals.var_ovt * locals.var_itf);
        let assign5910_e5997: f64 = (assign5910_e5995 * locals.var_ffdvc_ditf);
        let assign5910_e5998: f64 = (assign5910_e5992 + assign5910_e5997);
        let assign5910_e5999: f64 = (locals.var_ffdqfhc * assign5910_e5998);
        (assign5910_e5999, ((locals.var_ffdqfhc_dn0 * assign5910_e5998) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn0 * assign5910_e5989) + (locals.var_ffitf_ick * (((locals.var_ffic_dn0 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn0)) / (2.0 * assign5910_e5989))))) / (assign5910_e5990 * assign5910_e5990))) + (((locals.var_ovt * locals.var_itf_dn0) * locals.var_ffdvc_ditf) + (assign5910_e5995 * locals.var_ffdvc_ditf_dn0))))), ((locals.var_ffdqfhc_dn1 * assign5910_e5998) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn1 * assign5910_e5989) + (locals.var_ffitf_ick * (((locals.var_ffic_dn1 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn1)) / (2.0 * assign5910_e5989))))) / (assign5910_e5990 * assign5910_e5990))) + (((locals.var_ovt * locals.var_itf_dn1) * locals.var_ffdvc_ditf) + (assign5910_e5995 * locals.var_ffdvc_ditf_dn1))))), ((locals.var_ffdqfhc_dn3 * assign5910_e5998) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn3 * assign5910_e5989) + (locals.var_ffitf_ick * (((locals.var_ffic_dn3 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn3)) / (2.0 * assign5910_e5989))))) / (assign5910_e5990 * assign5910_e5990))) + (((locals.var_ovt * locals.var_itf_dn3) * locals.var_ffdvc_ditf) + (assign5910_e5995 * locals.var_ffdvc_ditf_dn3))))), ((locals.var_ffdqfhc_dn4 * assign5910_e5998) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn4 * assign5910_e5989) + (locals.var_ffitf_ick * (((locals.var_ffic_dn4 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn4)) / (2.0 * assign5910_e5989))))) / (assign5910_e5990 * assign5910_e5990))) + ((((locals.var_ovt_dn4 * locals.var_itf) + (locals.var_ovt * locals.var_itf_dn4)) * locals.var_ffdvc_ditf) + (assign5910_e5995 * locals.var_ffdvc_ditf_dn4))))), ((locals.var_ffdqfhc_dn5 * assign5910_e5998) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn5 * assign5910_e5989) + (locals.var_ffitf_ick * (((locals.var_ffic_dn5 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn5)) / (2.0 * assign5910_e5989))))) / (assign5910_e5990 * assign5910_e5990))) + (((locals.var_ovt * locals.var_itf_dn5) * locals.var_ffdvc_ditf) + (assign5910_e5995 * locals.var_ffdvc_ditf_dn5))))), ((locals.var_ffdqfhc_dn6 * assign5910_e5998) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn6 * assign5910_e5989) + (locals.var_ffitf_ick * (((locals.var_ffic_dn6 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn6)) / (2.0 * assign5910_e5989))))) / (assign5910_e5990 * assign5910_e5990))) + (((locals.var_ovt * locals.var_itf_dn6) * locals.var_ffdvc_ditf) + (assign5910_e5995 * locals.var_ffdvc_ditf_dn6))))), ((locals.var_ffdqfhc_dn7 * assign5910_e5998) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn7 * assign5910_e5989) + (locals.var_ffitf_ick * (((locals.var_ffic_dn7 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn7)) / (2.0 * assign5910_e5989))))) / (assign5910_e5990 * assign5910_e5990))) + (((locals.var_ovt * locals.var_itf_dn7) * locals.var_ffdvc_ditf) + (assign5910_e5995 * locals.var_ffdvc_ditf_dn7))))), ((locals.var_ffdqfhc_dn8 * assign5910_e5998) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn8 * assign5910_e5989) + (locals.var_ffitf_ick * (((locals.var_ffic_dn8 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn8)) / (2.0 * assign5910_e5989))))) / (assign5910_e5990 * assign5910_e5990))) + (((locals.var_ovt * locals.var_itf_dn8) * locals.var_ffdvc_ditf) + (assign5910_e5995 * locals.var_ffdvc_ditf_dn8))))), ((locals.var_ffdqfhc_dn9 * assign5910_e5998) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn9 * assign5910_e5989) + (locals.var_ffitf_ick * (((locals.var_ffic_dn9 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn9)) / (2.0 * assign5910_e5989))))) / (assign5910_e5990 * assign5910_e5990))) + (((locals.var_ovt * locals.var_itf_dn9) * locals.var_ffdvc_ditf) + (assign5910_e5995 * locals.var_ffdvc_ditf_dn9))))),)
    } else {
        (locals.var_ffdtfhc, locals.var_ffdtfhc_dn0, locals.var_ffdtfhc_dn1, locals.var_ffdtfhc_dn3, locals.var_ffdtfhc_dn4, locals.var_ffdtfhc_dn5, locals.var_ffdtfhc_dn6, locals.var_ffdtfhc_dn7, locals.var_ffdtfhc_dn8, locals.var_ffdtfhc_dn9,)
    }
};
        locals.var_ffdtfhc = assign5910_e6001;
        locals.var_ffdtfhc_dn0 = assign5910_e6001_d_n0;
        locals.var_ffdtfhc_dn1 = assign5910_e6001_d_n1;
        locals.var_ffdtfhc_dn3 = assign5910_e6001_d_n3;
        locals.var_ffdtfhc_dn4 = assign5910_e6001_d_n4;
        locals.var_ffdtfhc_dn5 = assign5910_e6001_d_n5;
        locals.var_ffdtfhc_dn6 = assign5910_e6001_d_n6;
        locals.var_ffdtfhc_dn7 = assign5910_e6001_d_n7;
        locals.var_ffdtfhc_dn8 = assign5910_e6001_d_n8;
        locals.var_ffdtfhc_dn9 = assign5910_e6001_d_n9;

        let assign5920_e6011: f64 = (locals.var_ffw * p.p115);
        let assign5920_e6017: f64 = (locals.var_ffw * p.p116);
        let assign5920_e6020: f64 = if ((((p.p115 < 0.01) && (p.p116 < 0.01)) && (assign5920_e6011 < 0.005)) && (assign5920_e6017 < 0.005)) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign5920_e6020;

        let (assign5930_e6030, assign5930_e6030_d_n0, assign5930_e6030_d_n1, assign5930_e6030_d_n3, assign5930_e6030_d_n4, assign5930_e6030_d_n5, assign5930_e6030_d_n6, assign5930_e6030_d_n7, assign5930_e6030_d_n8, assign5930_e6030_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard127 != 0.0)) {
        let assign5930_e6026: f64 = (p.p73 * locals.var_ffdqfhc);
        let assign5930_e6028: f64 = (assign5930_e6026 * locals.var_itf);
        (assign5930_e6028, (((p.p73 * locals.var_ffdqfhc_dn0) * locals.var_itf) + (assign5930_e6026 * locals.var_itf_dn0)), (((p.p73 * locals.var_ffdqfhc_dn1) * locals.var_itf) + (assign5930_e6026 * locals.var_itf_dn1)), (((p.p73 * locals.var_ffdqfhc_dn3) * locals.var_itf) + (assign5930_e6026 * locals.var_itf_dn3)), (((p.p73 * locals.var_ffdqfhc_dn4) * locals.var_itf) + (assign5930_e6026 * locals.var_itf_dn4)), (((p.p73 * locals.var_ffdqfhc_dn5) * locals.var_itf) + (assign5930_e6026 * locals.var_itf_dn5)), (((p.p73 * locals.var_ffdqfhc_dn6) * locals.var_itf) + (assign5930_e6026 * locals.var_itf_dn6)), (((p.p73 * locals.var_ffdqfhc_dn7) * locals.var_itf) + (assign5930_e6026 * locals.var_itf_dn7)), (((p.p73 * locals.var_ffdqfhc_dn8) * locals.var_itf) + (assign5930_e6026 * locals.var_itf_dn8)), (((p.p73 * locals.var_ffdqfhc_dn9) * locals.var_itf) + (assign5930_e6026 * locals.var_itf_dn9)),)
    } else {
        (locals.var_ffdqcfc, locals.var_ffdqcfc_dn0, locals.var_ffdqcfc_dn1, locals.var_ffdqcfc_dn3, locals.var_ffdqcfc_dn4, locals.var_ffdqcfc_dn5, locals.var_ffdqcfc_dn6, locals.var_ffdqcfc_dn7, locals.var_ffdqcfc_dn8, locals.var_ffdqcfc_dn9,)
    }
};
        locals.var_ffdqcfc = assign5930_e6030;
        locals.var_ffdqcfc_dn0 = assign5930_e6030_d_n0;
        locals.var_ffdqcfc_dn1 = assign5930_e6030_d_n1;
        locals.var_ffdqcfc_dn3 = assign5930_e6030_d_n3;
        locals.var_ffdqcfc_dn4 = assign5930_e6030_d_n4;
        locals.var_ffdqcfc_dn5 = assign5930_e6030_d_n5;
        locals.var_ffdqcfc_dn6 = assign5930_e6030_d_n6;
        locals.var_ffdqcfc_dn7 = assign5930_e6030_d_n7;
        locals.var_ffdqcfc_dn8 = assign5930_e6030_d_n8;
        locals.var_ffdqcfc_dn9 = assign5930_e6030_d_n9;

        let (assign5940_e6038, assign5940_e6038_d_n0, assign5940_e6038_d_n1, assign5940_e6038_d_n3, assign5940_e6038_d_n4, assign5940_e6038_d_n5, assign5940_e6038_d_n6, assign5940_e6038_d_n7, assign5940_e6038_d_n8, assign5940_e6038_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard127 != 0.0)) {
        let assign5940_e6036: f64 = (p.p73 * locals.var_ffdtfhc);
        (assign5940_e6036, (p.p73 * locals.var_ffdtfhc_dn0), (p.p73 * locals.var_ffdtfhc_dn1), (p.p73 * locals.var_ffdtfhc_dn3), (p.p73 * locals.var_ffdtfhc_dn4), (p.p73 * locals.var_ffdtfhc_dn5), (p.p73 * locals.var_ffdtfhc_dn6), (p.p73 * locals.var_ffdtfhc_dn7), (p.p73 * locals.var_ffdtfhc_dn8), (p.p73 * locals.var_ffdtfhc_dn9),)
    } else {
        (locals.var_ffdtcfc, locals.var_ffdtcfc_dn0, locals.var_ffdtcfc_dn1, locals.var_ffdtcfc_dn3, locals.var_ffdtcfc_dn4, locals.var_ffdtcfc_dn5, locals.var_ffdtcfc_dn6, locals.var_ffdtcfc_dn7, locals.var_ffdtcfc_dn8, locals.var_ffdtcfc_dn9,)
    }
};
        locals.var_ffdtcfc = assign5940_e6038;
        locals.var_ffdtcfc_dn0 = assign5940_e6038_d_n0;
        locals.var_ffdtcfc_dn1 = assign5940_e6038_d_n1;
        locals.var_ffdtcfc_dn3 = assign5940_e6038_d_n3;
        locals.var_ffdtcfc_dn4 = assign5940_e6038_d_n4;
        locals.var_ffdtcfc_dn5 = assign5940_e6038_d_n5;
        locals.var_ffdtcfc_dn6 = assign5940_e6038_d_n6;
        locals.var_ffdtcfc_dn7 = assign5940_e6038_d_n7;
        locals.var_ffdtcfc_dn8 = assign5940_e6038_d_n8;
        locals.var_ffdtcfc_dn9 = assign5940_e6038_d_n9;

        let (assign5950_e6047, assign5950_e6047_d_n0, assign5950_e6047_d_n1, assign5950_e6047_d_n3, assign5950_e6047_d_n4, assign5950_e6047_d_n5, assign5950_e6047_d_n6, assign5950_e6047_d_n7, assign5950_e6047_d_n8, assign5950_e6047_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign5950_e6045: f64 = (1.0 - locals.var_ffw);
        (assign5950_e6045, (-locals.var_ffw_dn0), (-locals.var_ffw_dn1), (-locals.var_ffw_dn3), (-locals.var_ffw_dn4), (-locals.var_ffw_dn5), (-locals.var_ffw_dn6), (-locals.var_ffw_dn7), (-locals.var_ffw_dn8), (-locals.var_ffw_dn9),)
    } else {
        (locals.var_fcick, locals.var_fcick_dn0, locals.var_fcick_dn1, locals.var_fcick_dn3, locals.var_fcick_dn4, locals.var_fcick_dn5, locals.var_fcick_dn6, locals.var_fcick_dn7, locals.var_fcick_dn8, locals.var_fcick_dn9,)
    }
};
        locals.var_fcick = assign5950_e6047;
        locals.var_fcick_dn0 = assign5950_e6047_d_n0;
        locals.var_fcick_dn1 = assign5950_e6047_d_n1;
        locals.var_fcick_dn3 = assign5950_e6047_d_n3;
        locals.var_fcick_dn4 = assign5950_e6047_d_n4;
        locals.var_fcick_dn5 = assign5950_e6047_d_n5;
        locals.var_fcick_dn6 = assign5950_e6047_d_n6;
        locals.var_fcick_dn7 = assign5950_e6047_d_n7;
        locals.var_fcick_dn8 = assign5950_e6047_d_n8;
        locals.var_fcick_dn9 = assign5950_e6047_d_n9;

        let (assign5960_e6069, assign5960_e6069_d_n0, assign5960_e6069_d_n1, assign5960_e6069_d_n3, assign5960_e6069_d_n4, assign5960_e6069_d_n5, assign5960_e6069_d_n6, assign5960_e6069_d_n7, assign5960_e6069_d_n8, assign5960_e6069_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign5960_e6054: f64 = (locals.var_fcick - 1.0);
        let assign5960_e6057: f64 = (1.0 - locals.var_ffic);
        let assign5960_e6058: f64 = (assign5960_e6054 * assign5960_e6057);
        let assign5960_e6061: f64 = (locals.var_ffic * locals.var_ffic);
        let assign5960_e6063: f64 = (assign5960_e6061 + p.p72);
        let assign5960_e6064: f64 = (assign5960_e6063).sqrt();
        let assign5960_e6066: f64 = (assign5960_e6064 * locals.var_itf);
        let assign5960_e6067: f64 = (assign5960_e6058 / assign5960_e6066);
        (assign5960_e6067, (((((locals.var_fcick_dn0 * assign5960_e6057) + (assign5960_e6054 * (-locals.var_ffic_dn0))) * assign5960_e6066) - (assign5960_e6058 * (((((locals.var_ffic_dn0 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn0)) / (2.0 * assign5960_e6064)) * locals.var_itf) + (assign5960_e6064 * locals.var_itf_dn0)))) / (assign5960_e6066 * assign5960_e6066)), (((((locals.var_fcick_dn1 * assign5960_e6057) + (assign5960_e6054 * (-locals.var_ffic_dn1))) * assign5960_e6066) - (assign5960_e6058 * (((((locals.var_ffic_dn1 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn1)) / (2.0 * assign5960_e6064)) * locals.var_itf) + (assign5960_e6064 * locals.var_itf_dn1)))) / (assign5960_e6066 * assign5960_e6066)), (((((locals.var_fcick_dn3 * assign5960_e6057) + (assign5960_e6054 * (-locals.var_ffic_dn3))) * assign5960_e6066) - (assign5960_e6058 * (((((locals.var_ffic_dn3 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn3)) / (2.0 * assign5960_e6064)) * locals.var_itf) + (assign5960_e6064 * locals.var_itf_dn3)))) / (assign5960_e6066 * assign5960_e6066)), (((((locals.var_fcick_dn4 * assign5960_e6057) + (assign5960_e6054 * (-locals.var_ffic_dn4))) * assign5960_e6066) - (assign5960_e6058 * (((((locals.var_ffic_dn4 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn4)) / (2.0 * assign5960_e6064)) * locals.var_itf) + (assign5960_e6064 * locals.var_itf_dn4)))) / (assign5960_e6066 * assign5960_e6066)), (((((locals.var_fcick_dn5 * assign5960_e6057) + (assign5960_e6054 * (-locals.var_ffic_dn5))) * assign5960_e6066) - (assign5960_e6058 * (((((locals.var_ffic_dn5 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn5)) / (2.0 * assign5960_e6064)) * locals.var_itf) + (assign5960_e6064 * locals.var_itf_dn5)))) / (assign5960_e6066 * assign5960_e6066)), (((((locals.var_fcick_dn6 * assign5960_e6057) + (assign5960_e6054 * (-locals.var_ffic_dn6))) * assign5960_e6066) - (assign5960_e6058 * (((((locals.var_ffic_dn6 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn6)) / (2.0 * assign5960_e6064)) * locals.var_itf) + (assign5960_e6064 * locals.var_itf_dn6)))) / (assign5960_e6066 * assign5960_e6066)), (((((locals.var_fcick_dn7 * assign5960_e6057) + (assign5960_e6054 * (-locals.var_ffic_dn7))) * assign5960_e6066) - (assign5960_e6058 * (((((locals.var_ffic_dn7 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn7)) / (2.0 * assign5960_e6064)) * locals.var_itf) + (assign5960_e6064 * locals.var_itf_dn7)))) / (assign5960_e6066 * assign5960_e6066)), (((((locals.var_fcick_dn8 * assign5960_e6057) + (assign5960_e6054 * (-locals.var_ffic_dn8))) * assign5960_e6066) - (assign5960_e6058 * (((((locals.var_ffic_dn8 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn8)) / (2.0 * assign5960_e6064)) * locals.var_itf) + (assign5960_e6064 * locals.var_itf_dn8)))) / (assign5960_e6066 * assign5960_e6066)), (((((locals.var_fcick_dn9 * assign5960_e6057) + (assign5960_e6054 * (-locals.var_ffic_dn9))) * assign5960_e6066) - (assign5960_e6058 * (((((locals.var_ffic_dn9 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn9)) / (2.0 * assign5960_e6064)) * locals.var_itf) + (assign5960_e6064 * locals.var_itf_dn9)))) / (assign5960_e6066 * assign5960_e6066)),)
    } else {
        (locals.var_fcdick_ditf, locals.var_fcdick_ditf_dn0, locals.var_fcdick_ditf_dn1, locals.var_fcdick_ditf_dn3, locals.var_fcdick_ditf_dn4, locals.var_fcdick_ditf_dn5, locals.var_fcdick_ditf_dn6, locals.var_fcdick_ditf_dn7, locals.var_fcdick_ditf_dn8, locals.var_fcdick_ditf_dn9,)
    }
};
        locals.var_fcdick_ditf = assign5960_e6069;
        locals.var_fcdick_ditf_dn0 = assign5960_e6069_d_n0;
        locals.var_fcdick_ditf_dn1 = assign5960_e6069_d_n1;
        locals.var_fcdick_ditf_dn3 = assign5960_e6069_d_n3;
        locals.var_fcdick_ditf_dn4 = assign5960_e6069_d_n4;
        locals.var_fcdick_ditf_dn5 = assign5960_e6069_d_n5;
        locals.var_fcdick_ditf_dn6 = assign5960_e6069_d_n6;
        locals.var_fcdick_ditf_dn7 = assign5960_e6069_d_n7;
        locals.var_fcdick_ditf_dn8 = assign5960_e6069_d_n8;
        locals.var_fcdick_ditf_dn9 = assign5960_e6069_d_n9;

        let assign5970_e6071: f64 = (locals.var_lat_delta).abs();
        let assign5970_e6073: f64 = if assign5970_e6071 > 0.001 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign5970_e6073;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5980_e6087, assign5980_e6087_d_n0, assign5980_e6087_d_n1, assign5980_e6087_d_n3, assign5980_e6087_d_n4, assign5980_e6087_d_n5, assign5980_e6087_d_n6, assign5980_e6087_d_n7, assign5980_e6087_d_n8, assign5980_e6087_d_n9,) = {
    if (((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) {
        let assign5980_e6082: f64 = (locals.var_fcick - 1.0);
        let assign5980_e6084: f64 = (assign5980_e6082 * locals.var_ln_lat);
        let assign5980_e6085: f64 = (assign5980_e6084).exp();
        (assign5980_e6085, (assign5980_e6085 * (locals.var_fcick_dn0 * locals.var_ln_lat)), (assign5980_e6085 * (locals.var_fcick_dn1 * locals.var_ln_lat)), (assign5980_e6085 * (locals.var_fcick_dn3 * locals.var_ln_lat)), (assign5980_e6085 * (locals.var_fcick_dn4 * locals.var_ln_lat)), (assign5980_e6085 * (locals.var_fcick_dn5 * locals.var_ln_lat)), (assign5980_e6085 * (locals.var_fcick_dn6 * locals.var_ln_lat)), (assign5980_e6085 * (locals.var_fcick_dn7 * locals.var_ln_lat)), (assign5980_e6085 * (locals.var_fcick_dn8 * locals.var_ln_lat)), (assign5980_e6085 * (locals.var_fcick_dn9 * locals.var_ln_lat)),)
    } else {
        (locals.var_fck, locals.var_fck_dn0, locals.var_fck_dn1, locals.var_fck_dn3, locals.var_fck_dn4, locals.var_fck_dn5, locals.var_fck_dn6, locals.var_fck_dn7, locals.var_fck_dn8, locals.var_fck_dn9,)
    }
};
        locals.var_fck = assign5980_e6087;
        locals.var_fck_dn0 = assign5980_e6087_d_n0;
        locals.var_fck_dn1 = assign5980_e6087_d_n1;
        locals.var_fck_dn3 = assign5980_e6087_d_n3;
        locals.var_fck_dn4 = assign5980_e6087_d_n4;
        locals.var_fck_dn5 = assign5980_e6087_d_n5;
        locals.var_fck_dn6 = assign5980_e6087_d_n6;
        locals.var_fck_dn7 = assign5980_e6087_d_n7;
        locals.var_fck_dn8 = assign5980_e6087_d_n8;
        locals.var_fck_dn9 = assign5980_e6087_d_n9;

        let assign5990_e6090: f64 = if locals.var_latmin < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard129 = assign5990_e6090;

        let (assign6000_e6107, assign6000_e6107_d_n0, assign6000_e6107_d_n1, assign6000_e6107_d_n3, assign6000_e6107_d_n4, assign6000_e6107_d_n5, assign6000_e6107_d_n6, assign6000_e6107_d_n7, assign6000_e6107_d_n8, assign6000_e6107_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 != 0.0)) {
        let assign6000_e6101: f64 = (1.0 - locals.var_fck);
        let assign6000_e6104: f64 = (locals.var_fck * locals.var_latmax);
        let assign6000_e6105: f64 = (assign6000_e6101 / assign6000_e6104);
        (assign6000_e6105, ((((-locals.var_fck_dn0) * assign6000_e6104) - (assign6000_e6101 * (locals.var_fck_dn0 * locals.var_latmax))) / (assign6000_e6104 * assign6000_e6104)), ((((-locals.var_fck_dn1) * assign6000_e6104) - (assign6000_e6101 * (locals.var_fck_dn1 * locals.var_latmax))) / (assign6000_e6104 * assign6000_e6104)), ((((-locals.var_fck_dn3) * assign6000_e6104) - (assign6000_e6101 * (locals.var_fck_dn3 * locals.var_latmax))) / (assign6000_e6104 * assign6000_e6104)), ((((-locals.var_fck_dn4) * assign6000_e6104) - (assign6000_e6101 * (locals.var_fck_dn4 * locals.var_latmax))) / (assign6000_e6104 * assign6000_e6104)), ((((-locals.var_fck_dn5) * assign6000_e6104) - (assign6000_e6101 * (locals.var_fck_dn5 * locals.var_latmax))) / (assign6000_e6104 * assign6000_e6104)), ((((-locals.var_fck_dn6) * assign6000_e6104) - (assign6000_e6101 * (locals.var_fck_dn6 * locals.var_latmax))) / (assign6000_e6104 * assign6000_e6104)), ((((-locals.var_fck_dn7) * assign6000_e6104) - (assign6000_e6101 * (locals.var_fck_dn7 * locals.var_latmax))) / (assign6000_e6104 * assign6000_e6104)), ((((-locals.var_fck_dn8) * assign6000_e6104) - (assign6000_e6101 * (locals.var_fck_dn8 * locals.var_latmax))) / (assign6000_e6104 * assign6000_e6104)), ((((-locals.var_fck_dn9) * assign6000_e6104) - (assign6000_e6101 * (locals.var_fck_dn9 * locals.var_latmax))) / (assign6000_e6104 * assign6000_e6104)),)
    } else {
        (locals.var_fcw, locals.var_fcw_dn0, locals.var_fcw_dn1, locals.var_fcw_dn3, locals.var_fcw_dn4, locals.var_fcw_dn5, locals.var_fcw_dn6, locals.var_fcw_dn7, locals.var_fcw_dn8, locals.var_fcw_dn9,)
    }
};
        locals.var_fcw = assign6000_e6107;
        locals.var_fcw_dn0 = assign6000_e6107_d_n0;
        locals.var_fcw_dn1 = assign6000_e6107_d_n1;
        locals.var_fcw_dn3 = assign6000_e6107_d_n3;
        locals.var_fcw_dn4 = assign6000_e6107_d_n4;
        locals.var_fcw_dn5 = assign6000_e6107_d_n5;
        locals.var_fcw_dn6 = assign6000_e6107_d_n6;
        locals.var_fcw_dn7 = assign6000_e6107_d_n7;
        locals.var_fcw_dn8 = assign6000_e6107_d_n8;
        locals.var_fcw_dn9 = assign6000_e6107_d_n9;

        let (assign6010_e6122, assign6010_e6122_d_n0, assign6010_e6122_d_n1, assign6010_e6122_d_n3, assign6010_e6122_d_n4, assign6010_e6122_d_n5, assign6010_e6122_d_n6, assign6010_e6122_d_n7, assign6010_e6122_d_n8, assign6010_e6122_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 != 0.0)) {
        let assign6010_e6119: f64 = (locals.var_latmax * locals.var_fcw);
        let assign6010_e6120: f64 = (1.0 + assign6010_e6119);
        (assign6010_e6120, (locals.var_latmax * locals.var_fcw_dn0), (locals.var_latmax * locals.var_fcw_dn1), (locals.var_latmax * locals.var_fcw_dn3), (locals.var_latmax * locals.var_fcw_dn4), (locals.var_latmax * locals.var_fcw_dn5), (locals.var_latmax * locals.var_fcw_dn6), (locals.var_latmax * locals.var_fcw_dn7), (locals.var_latmax * locals.var_fcw_dn8), (locals.var_latmax * locals.var_fcw_dn9),)
    } else {
        (locals.var_fclatw_p1, locals.var_fclatw_p1_dn0, locals.var_fclatw_p1_dn1, locals.var_fclatw_p1_dn3, locals.var_fclatw_p1_dn4, locals.var_fclatw_p1_dn5, locals.var_fclatw_p1_dn6, locals.var_fclatw_p1_dn7, locals.var_fclatw_p1_dn8, locals.var_fclatw_p1_dn9,)
    }
};
        locals.var_fclatw_p1 = assign6010_e6122;
        locals.var_fclatw_p1_dn0 = assign6010_e6122_d_n0;
        locals.var_fclatw_p1_dn1 = assign6010_e6122_d_n1;
        locals.var_fclatw_p1_dn3 = assign6010_e6122_d_n3;
        locals.var_fclatw_p1_dn4 = assign6010_e6122_d_n4;
        locals.var_fclatw_p1_dn5 = assign6010_e6122_d_n5;
        locals.var_fclatw_p1_dn6 = assign6010_e6122_d_n6;
        locals.var_fclatw_p1_dn7 = assign6010_e6122_d_n7;
        locals.var_fclatw_p1_dn8 = assign6010_e6122_d_n8;
        locals.var_fclatw_p1_dn9 = assign6010_e6122_d_n9;

        let (assign6020_e6154, assign6020_e6154_d_n0, assign6020_e6154_d_n1, assign6020_e6154_d_n3, assign6020_e6154_d_n4, assign6020_e6154_d_n5, assign6020_e6154_d_n6, assign6020_e6154_d_n7, assign6020_e6154_d_n8, assign6020_e6154_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 != 0.0)) {
        let assign6020_e6134: f64 = (locals.var_latmax * locals.var_fcw);
        let assign6020_e6138: f64 = (0.25 * locals.var_latmax);
        let assign6020_e6140: f64 = (assign6020_e6138 * locals.var_fcw);
        let assign6020_e6141: f64 = (0.5 + assign6020_e6140);
        let assign6020_e6142: f64 = (assign6020_e6134 * assign6020_e6141);
        let assign6020_e6145: f64 = (locals.var_fclatw_p1).ln();
        let assign6020_e6146: f64 = (0.5 * assign6020_e6145);
        let assign6020_e6147: f64 = (assign6020_e6142 - assign6020_e6146);
        let assign6020_e6148: f64 = (2.0 * assign6020_e6147);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_latmax;
        let assign6020_e6150: f64 = (assign6020_e6148 * __rspice_inv_cse_0);
        let assign6020_e6152: f64 = (assign6020_e6150 * __rspice_inv_cse_0);
        (assign6020_e6152, (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn0) * assign6020_e6141) + (assign6020_e6134 * (assign6020_e6138 * locals.var_fcw_dn0))) - (0.5 * (locals.var_fclatw_p1_dn0 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn1) * assign6020_e6141) + (assign6020_e6134 * (assign6020_e6138 * locals.var_fcw_dn1))) - (0.5 * (locals.var_fclatw_p1_dn1 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn3) * assign6020_e6141) + (assign6020_e6134 * (assign6020_e6138 * locals.var_fcw_dn3))) - (0.5 * (locals.var_fclatw_p1_dn3 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn4) * assign6020_e6141) + (assign6020_e6134 * (assign6020_e6138 * locals.var_fcw_dn4))) - (0.5 * (locals.var_fclatw_p1_dn4 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn5) * assign6020_e6141) + (assign6020_e6134 * (assign6020_e6138 * locals.var_fcw_dn5))) - (0.5 * (locals.var_fclatw_p1_dn5 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn6) * assign6020_e6141) + (assign6020_e6134 * (assign6020_e6138 * locals.var_fcw_dn6))) - (0.5 * (locals.var_fclatw_p1_dn6 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn7) * assign6020_e6141) + (assign6020_e6134 * (assign6020_e6138 * locals.var_fcw_dn7))) - (0.5 * (locals.var_fclatw_p1_dn7 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn8) * assign6020_e6141) + (assign6020_e6134 * (assign6020_e6138 * locals.var_fcw_dn8))) - (0.5 * (locals.var_fclatw_p1_dn8 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn9) * assign6020_e6141) + (assign6020_e6134 * (assign6020_e6138 * locals.var_fcw_dn9))) - (0.5 * (locals.var_fclatw_p1_dn9 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax),)
    } else {
        (locals.var_fcf_ci, locals.var_fcf_ci_dn0, locals.var_fcf_ci_dn1, locals.var_fcf_ci_dn3, locals.var_fcf_ci_dn4, locals.var_fcf_ci_dn5, locals.var_fcf_ci_dn6, locals.var_fcf_ci_dn7, locals.var_fcf_ci_dn8, locals.var_fcf_ci_dn9,)
    }
};
        locals.var_fcf_ci = assign6020_e6154;
        locals.var_fcf_ci_dn0 = assign6020_e6154_d_n0;
        locals.var_fcf_ci_dn1 = assign6020_e6154_d_n1;
        locals.var_fcf_ci_dn3 = assign6020_e6154_d_n3;
        locals.var_fcf_ci_dn4 = assign6020_e6154_d_n4;
        locals.var_fcf_ci_dn5 = assign6020_e6154_d_n5;
        locals.var_fcf_ci_dn6 = assign6020_e6154_d_n6;
        locals.var_fcf_ci_dn7 = assign6020_e6154_d_n7;
        locals.var_fcf_ci_dn8 = assign6020_e6154_d_n8;
        locals.var_fcf_ci_dn9 = assign6020_e6154_d_n9;

        let (assign6030_e6172, assign6030_e6172_d_n0, assign6030_e6172_d_n1, assign6030_e6172_d_n3, assign6030_e6172_d_n4, assign6030_e6172_d_n5, assign6030_e6172_d_n6, assign6030_e6172_d_n7, assign6030_e6172_d_n8, assign6030_e6172_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 != 0.0)) {
        let assign6030_e6164: f64 = (-locals.var_ln_lat);
        let assign6030_e6166: f64 = (assign6030_e6164 * locals.var_fcdick_ditf);
        let assign6030_e6169: f64 = (locals.var_fck * locals.var_latmax);
        let assign6030_e6170: f64 = (assign6030_e6166 / assign6030_e6169);
        (assign6030_e6170, ((((assign6030_e6164 * locals.var_fcdick_ditf_dn0) * assign6030_e6169) - (assign6030_e6166 * (locals.var_fck_dn0 * locals.var_latmax))) / (assign6030_e6169 * assign6030_e6169)), ((((assign6030_e6164 * locals.var_fcdick_ditf_dn1) * assign6030_e6169) - (assign6030_e6166 * (locals.var_fck_dn1 * locals.var_latmax))) / (assign6030_e6169 * assign6030_e6169)), ((((assign6030_e6164 * locals.var_fcdick_ditf_dn3) * assign6030_e6169) - (assign6030_e6166 * (locals.var_fck_dn3 * locals.var_latmax))) / (assign6030_e6169 * assign6030_e6169)), ((((assign6030_e6164 * locals.var_fcdick_ditf_dn4) * assign6030_e6169) - (assign6030_e6166 * (locals.var_fck_dn4 * locals.var_latmax))) / (assign6030_e6169 * assign6030_e6169)), ((((assign6030_e6164 * locals.var_fcdick_ditf_dn5) * assign6030_e6169) - (assign6030_e6166 * (locals.var_fck_dn5 * locals.var_latmax))) / (assign6030_e6169 * assign6030_e6169)), ((((assign6030_e6164 * locals.var_fcdick_ditf_dn6) * assign6030_e6169) - (assign6030_e6166 * (locals.var_fck_dn6 * locals.var_latmax))) / (assign6030_e6169 * assign6030_e6169)), ((((assign6030_e6164 * locals.var_fcdick_ditf_dn7) * assign6030_e6169) - (assign6030_e6166 * (locals.var_fck_dn7 * locals.var_latmax))) / (assign6030_e6169 * assign6030_e6169)), ((((assign6030_e6164 * locals.var_fcdick_ditf_dn8) * assign6030_e6169) - (assign6030_e6166 * (locals.var_fck_dn8 * locals.var_latmax))) / (assign6030_e6169 * assign6030_e6169)), ((((assign6030_e6164 * locals.var_fcdick_ditf_dn9) * assign6030_e6169) - (assign6030_e6166 * (locals.var_fck_dn9 * locals.var_latmax))) / (assign6030_e6169 * assign6030_e6169)),)
    } else {
        (locals.var_fcdw_ditf, locals.var_fcdw_ditf_dn0, locals.var_fcdw_ditf_dn1, locals.var_fcdw_ditf_dn3, locals.var_fcdw_ditf_dn4, locals.var_fcdw_ditf_dn5, locals.var_fcdw_ditf_dn6, locals.var_fcdw_ditf_dn7, locals.var_fcdw_ditf_dn8, locals.var_fcdw_ditf_dn9,)
    }
};
        locals.var_fcdw_ditf = assign6030_e6172;
        locals.var_fcdw_ditf_dn0 = assign6030_e6172_d_n0;
        locals.var_fcdw_ditf_dn1 = assign6030_e6172_d_n1;
        locals.var_fcdw_ditf_dn3 = assign6030_e6172_d_n3;
        locals.var_fcdw_ditf_dn4 = assign6030_e6172_d_n4;
        locals.var_fcdw_ditf_dn5 = assign6030_e6172_d_n5;
        locals.var_fcdw_ditf_dn6 = assign6030_e6172_d_n6;
        locals.var_fcdw_ditf_dn7 = assign6030_e6172_d_n7;
        locals.var_fcdw_ditf_dn8 = assign6030_e6172_d_n8;
        locals.var_fcdw_ditf_dn9 = assign6030_e6172_d_n9;

        let (assign6040_e6191, assign6040_e6191_d_n0, assign6040_e6191_d_n1, assign6040_e6191_d_n3, assign6040_e6191_d_n4, assign6040_e6191_d_n5, assign6040_e6191_d_n6, assign6040_e6191_d_n7, assign6040_e6191_d_n8, assign6040_e6191_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 != 0.0)) {
        let assign6040_e6183: f64 = (1.0 + locals.var_fclatw_p1);
        let assign6040_e6185: f64 = (assign6040_e6183 * locals.var_fcw);
        let assign6040_e6187: f64 = (assign6040_e6185 * locals.var_fcdw_ditf);
        let assign6040_e6189: f64 = (assign6040_e6187 / locals.var_fclatw_p1);
        (assign6040_e6189, (((((((locals.var_fclatw_p1_dn0 * locals.var_fcw) + (assign6040_e6183 * locals.var_fcw_dn0)) * locals.var_fcdw_ditf) + (assign6040_e6185 * locals.var_fcdw_ditf_dn0)) * locals.var_fclatw_p1) - (assign6040_e6187 * locals.var_fclatw_p1_dn0)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn1 * locals.var_fcw) + (assign6040_e6183 * locals.var_fcw_dn1)) * locals.var_fcdw_ditf) + (assign6040_e6185 * locals.var_fcdw_ditf_dn1)) * locals.var_fclatw_p1) - (assign6040_e6187 * locals.var_fclatw_p1_dn1)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn3 * locals.var_fcw) + (assign6040_e6183 * locals.var_fcw_dn3)) * locals.var_fcdw_ditf) + (assign6040_e6185 * locals.var_fcdw_ditf_dn3)) * locals.var_fclatw_p1) - (assign6040_e6187 * locals.var_fclatw_p1_dn3)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn4 * locals.var_fcw) + (assign6040_e6183 * locals.var_fcw_dn4)) * locals.var_fcdw_ditf) + (assign6040_e6185 * locals.var_fcdw_ditf_dn4)) * locals.var_fclatw_p1) - (assign6040_e6187 * locals.var_fclatw_p1_dn4)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn5 * locals.var_fcw) + (assign6040_e6183 * locals.var_fcw_dn5)) * locals.var_fcdw_ditf) + (assign6040_e6185 * locals.var_fcdw_ditf_dn5)) * locals.var_fclatw_p1) - (assign6040_e6187 * locals.var_fclatw_p1_dn5)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn6 * locals.var_fcw) + (assign6040_e6183 * locals.var_fcw_dn6)) * locals.var_fcdw_ditf) + (assign6040_e6185 * locals.var_fcdw_ditf_dn6)) * locals.var_fclatw_p1) - (assign6040_e6187 * locals.var_fclatw_p1_dn6)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn7 * locals.var_fcw) + (assign6040_e6183 * locals.var_fcw_dn7)) * locals.var_fcdw_ditf) + (assign6040_e6185 * locals.var_fcdw_ditf_dn7)) * locals.var_fclatw_p1) - (assign6040_e6187 * locals.var_fclatw_p1_dn7)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn8 * locals.var_fcw) + (assign6040_e6183 * locals.var_fcw_dn8)) * locals.var_fcdw_ditf) + (assign6040_e6185 * locals.var_fcdw_ditf_dn8)) * locals.var_fclatw_p1) - (assign6040_e6187 * locals.var_fclatw_p1_dn8)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn9 * locals.var_fcw) + (assign6040_e6183 * locals.var_fcw_dn9)) * locals.var_fcdw_ditf) + (assign6040_e6185 * locals.var_fcdw_ditf_dn9)) * locals.var_fclatw_p1) - (assign6040_e6187 * locals.var_fclatw_p1_dn9)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)),)
    } else {
        (locals.var_fcdfc_ditf, locals.var_fcdfc_ditf_dn0, locals.var_fcdfc_ditf_dn1, locals.var_fcdfc_ditf_dn3, locals.var_fcdfc_ditf_dn4, locals.var_fcdfc_ditf_dn5, locals.var_fcdfc_ditf_dn6, locals.var_fcdfc_ditf_dn7, locals.var_fcdfc_ditf_dn8, locals.var_fcdfc_ditf_dn9,)
    }
};
        locals.var_fcdfc_ditf = assign6040_e6191;
        locals.var_fcdfc_ditf_dn0 = assign6040_e6191_d_n0;
        locals.var_fcdfc_ditf_dn1 = assign6040_e6191_d_n1;
        locals.var_fcdfc_ditf_dn3 = assign6040_e6191_d_n3;
        locals.var_fcdfc_ditf_dn4 = assign6040_e6191_d_n4;
        locals.var_fcdfc_ditf_dn5 = assign6040_e6191_d_n5;
        locals.var_fcdfc_ditf_dn6 = assign6040_e6191_d_n6;
        locals.var_fcdfc_ditf_dn7 = assign6040_e6191_d_n7;
        locals.var_fcdfc_ditf_dn8 = assign6040_e6191_d_n8;
        locals.var_fcdfc_ditf_dn9 = assign6040_e6191_d_n9;

        let (assign6050_e6207, assign6050_e6207_d_n0, assign6050_e6207_d_n1, assign6050_e6207_d_n3, assign6050_e6207_d_n4, assign6050_e6207_d_n5, assign6050_e6207_d_n6, assign6050_e6207_d_n7, assign6050_e6207_d_n8, assign6050_e6207_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6050_e6204: f64 = (locals.var_fck * p.p115);
        let assign6050_e6205: f64 = (p.p116 - assign6050_e6204);
        (assign6050_e6205, (-(locals.var_fck_dn0 * p.p115)), (-(locals.var_fck_dn1 * p.p115)), (-(locals.var_fck_dn3 * p.p115)), (-(locals.var_fck_dn4 * p.p115)), (-(locals.var_fck_dn5 * p.p115)), (-(locals.var_fck_dn6 * p.p115)), (-(locals.var_fck_dn7 * p.p115)), (-(locals.var_fck_dn8 * p.p115)), (-(locals.var_fck_dn9 * p.p115)),)
    } else {
        (locals.var_fckdelta, locals.var_fckdelta_dn0, locals.var_fckdelta_dn1, locals.var_fckdelta_dn3, locals.var_fckdelta_dn4, locals.var_fckdelta_dn5, locals.var_fckdelta_dn6, locals.var_fckdelta_dn7, locals.var_fckdelta_dn8, locals.var_fckdelta_dn9,)
    }
};
        locals.var_fckdelta = assign6050_e6207;
        locals.var_fckdelta_dn0 = assign6050_e6207_d_n0;
        locals.var_fckdelta_dn1 = assign6050_e6207_d_n1;
        locals.var_fckdelta_dn3 = assign6050_e6207_d_n3;
        locals.var_fckdelta_dn4 = assign6050_e6207_d_n4;
        locals.var_fckdelta_dn5 = assign6050_e6207_d_n5;
        locals.var_fckdelta_dn6 = assign6050_e6207_d_n6;
        locals.var_fckdelta_dn7 = assign6050_e6207_d_n7;
        locals.var_fckdelta_dn8 = assign6050_e6207_d_n8;
        locals.var_fckdelta_dn9 = assign6050_e6207_d_n9;

        let (assign6060_e6223, assign6060_e6223_d_n0, assign6060_e6223_d_n1, assign6060_e6223_d_n3, assign6060_e6223_d_n4, assign6060_e6223_d_n5, assign6060_e6223_d_n6, assign6060_e6223_d_n7, assign6060_e6223_d_n8, assign6060_e6223_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6060_e6219: f64 = (locals.var_fck - 1.0);
        let assign6060_e6221: f64 = (assign6060_e6219 / locals.var_fckdelta);
        (assign6060_e6221, (((locals.var_fck_dn0 * locals.var_fckdelta) - (assign6060_e6219 * locals.var_fckdelta_dn0)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn1 * locals.var_fckdelta) - (assign6060_e6219 * locals.var_fckdelta_dn1)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn3 * locals.var_fckdelta) - (assign6060_e6219 * locals.var_fckdelta_dn3)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn4 * locals.var_fckdelta) - (assign6060_e6219 * locals.var_fckdelta_dn4)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn5 * locals.var_fckdelta) - (assign6060_e6219 * locals.var_fckdelta_dn5)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn6 * locals.var_fckdelta) - (assign6060_e6219 * locals.var_fckdelta_dn6)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn7 * locals.var_fckdelta) - (assign6060_e6219 * locals.var_fckdelta_dn7)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn8 * locals.var_fckdelta) - (assign6060_e6219 * locals.var_fckdelta_dn8)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn9 * locals.var_fckdelta) - (assign6060_e6219 * locals.var_fckdelta_dn9)) / (locals.var_fckdelta * locals.var_fckdelta)),)
    } else {
        (locals.var_fcw, locals.var_fcw_dn0, locals.var_fcw_dn1, locals.var_fcw_dn3, locals.var_fcw_dn4, locals.var_fcw_dn5, locals.var_fcw_dn6, locals.var_fcw_dn7, locals.var_fcw_dn8, locals.var_fcw_dn9,)
    }
};
        locals.var_fcw = assign6060_e6223;
        locals.var_fcw_dn0 = assign6060_e6223_d_n0;
        locals.var_fcw_dn1 = assign6060_e6223_d_n1;
        locals.var_fcw_dn3 = assign6060_e6223_d_n3;
        locals.var_fcw_dn4 = assign6060_e6223_d_n4;
        locals.var_fcw_dn5 = assign6060_e6223_d_n5;
        locals.var_fcw_dn6 = assign6060_e6223_d_n6;
        locals.var_fcw_dn7 = assign6060_e6223_d_n7;
        locals.var_fcw_dn8 = assign6060_e6223_d_n8;
        locals.var_fcw_dn9 = assign6060_e6223_d_n9;

        let (assign6070_e6239, assign6070_e6239_d_n0, assign6070_e6239_d_n1, assign6070_e6239_d_n3, assign6070_e6239_d_n4, assign6070_e6239_d_n5, assign6070_e6239_d_n6, assign6070_e6239_d_n7, assign6070_e6239_d_n8, assign6070_e6239_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6070_e6236: f64 = (p.p116 * locals.var_fcw);
        let assign6070_e6237: f64 = (1.0 + assign6070_e6236);
        (assign6070_e6237, (p.p116 * locals.var_fcw_dn0), (p.p116 * locals.var_fcw_dn1), (p.p116 * locals.var_fcw_dn3), (p.p116 * locals.var_fcw_dn4), (p.p116 * locals.var_fcw_dn5), (p.p116 * locals.var_fcw_dn6), (p.p116 * locals.var_fcw_dn7), (p.p116 * locals.var_fcw_dn8), (p.p116 * locals.var_fcw_dn9),)
    } else {
        (locals.var_fciwzb_p1, locals.var_fciwzb_p1_dn0, locals.var_fciwzb_p1_dn1, locals.var_fciwzb_p1_dn3, locals.var_fciwzb_p1_dn4, locals.var_fciwzb_p1_dn5, locals.var_fciwzb_p1_dn6, locals.var_fciwzb_p1_dn7, locals.var_fciwzb_p1_dn8, locals.var_fciwzb_p1_dn9,)
    }
};
        locals.var_fciwzb_p1 = assign6070_e6239;
        locals.var_fciwzb_p1_dn0 = assign6070_e6239_d_n0;
        locals.var_fciwzb_p1_dn1 = assign6070_e6239_d_n1;
        locals.var_fciwzb_p1_dn3 = assign6070_e6239_d_n3;
        locals.var_fciwzb_p1_dn4 = assign6070_e6239_d_n4;
        locals.var_fciwzb_p1_dn5 = assign6070_e6239_d_n5;
        locals.var_fciwzb_p1_dn6 = assign6070_e6239_d_n6;
        locals.var_fciwzb_p1_dn7 = assign6070_e6239_d_n7;
        locals.var_fciwzb_p1_dn8 = assign6070_e6239_d_n8;
        locals.var_fciwzb_p1_dn9 = assign6070_e6239_d_n9;

        let (assign6080_e6252, assign6080_e6252_d_n0, assign6080_e6252_d_n1, assign6080_e6252_d_n3, assign6080_e6252_d_n4, assign6080_e6252_d_n5, assign6080_e6252_d_n6, assign6080_e6252_d_n7, assign6080_e6252_d_n8, assign6080_e6252_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6080_e6250: f64 = (locals.var_fciwzb_p1).ln();
        (assign6080_e6250, (locals.var_fciwzb_p1_dn0 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn1 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn3 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn4 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn5 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn6 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn7 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn8 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn9 / locals.var_fciwzb_p1),)
    } else {
        (locals.var_fcilnw_bl, locals.var_fcilnw_bl_dn0, locals.var_fcilnw_bl_dn1, locals.var_fcilnw_bl_dn3, locals.var_fcilnw_bl_dn4, locals.var_fcilnw_bl_dn5, locals.var_fcilnw_bl_dn6, locals.var_fcilnw_bl_dn7, locals.var_fcilnw_bl_dn8, locals.var_fcilnw_bl_dn9,)
    }
};
        locals.var_fcilnw_bl = assign6080_e6252;
        locals.var_fcilnw_bl_dn0 = assign6080_e6252_d_n0;
        locals.var_fcilnw_bl_dn1 = assign6080_e6252_d_n1;
        locals.var_fcilnw_bl_dn3 = assign6080_e6252_d_n3;
        locals.var_fcilnw_bl_dn4 = assign6080_e6252_d_n4;
        locals.var_fcilnw_bl_dn5 = assign6080_e6252_d_n5;
        locals.var_fcilnw_bl_dn6 = assign6080_e6252_d_n6;
        locals.var_fcilnw_bl_dn7 = assign6080_e6252_d_n7;
        locals.var_fcilnw_bl_dn8 = assign6080_e6252_d_n8;
        locals.var_fcilnw_bl_dn9 = assign6080_e6252_d_n9;

        let (assign6090_e6266,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6090_e6264: f64 = (locals.var_latb_6 * locals.var_inv_latl);
        (assign6090_e6264,)
    } else {
        (locals.var_fcia,)
    }
};
        locals.var_fcia = assign6090_e6266;

        let (assign6100_e6292, assign6100_e6292_d_n0, assign6100_e6292_d_n1, assign6100_e6292_d_n3, assign6100_e6292_d_n4, assign6100_e6292_d_n5, assign6100_e6292_d_n6, assign6100_e6292_d_n7, assign6100_e6292_d_n8, assign6100_e6292_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6100_e6279: f64 = (0.5 - locals.var_fcia);
        let assign6100_e6280: f64 = (locals.var_fcilnw_bl * assign6100_e6279);
        let assign6100_e6282: f64 = (assign6100_e6280 * locals.var_inv_latl);
        let assign6100_e6286: f64 = (locals.var_latb_6 * locals.var_fcw);
        let assign6100_e6287: f64 = (locals.var_fcia + assign6100_e6286);
        let assign6100_e6289: f64 = (assign6100_e6287 * locals.var_fcw);
        let assign6100_e6290: f64 = (assign6100_e6282 + assign6100_e6289);
        (assign6100_e6290, (((locals.var_fcilnw_bl_dn0 * assign6100_e6279) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn0) * locals.var_fcw) + (assign6100_e6287 * locals.var_fcw_dn0))), (((locals.var_fcilnw_bl_dn1 * assign6100_e6279) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn1) * locals.var_fcw) + (assign6100_e6287 * locals.var_fcw_dn1))), (((locals.var_fcilnw_bl_dn3 * assign6100_e6279) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn3) * locals.var_fcw) + (assign6100_e6287 * locals.var_fcw_dn3))), (((locals.var_fcilnw_bl_dn4 * assign6100_e6279) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn4) * locals.var_fcw) + (assign6100_e6287 * locals.var_fcw_dn4))), (((locals.var_fcilnw_bl_dn5 * assign6100_e6279) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn5) * locals.var_fcw) + (assign6100_e6287 * locals.var_fcw_dn5))), (((locals.var_fcilnw_bl_dn6 * assign6100_e6279) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn6) * locals.var_fcw) + (assign6100_e6287 * locals.var_fcw_dn6))), (((locals.var_fcilnw_bl_dn7 * assign6100_e6279) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn7) * locals.var_fcw) + (assign6100_e6287 * locals.var_fcw_dn7))), (((locals.var_fcilnw_bl_dn8 * assign6100_e6279) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn8) * locals.var_fcw) + (assign6100_e6287 * locals.var_fcw_dn8))), (((locals.var_fcilnw_bl_dn9 * assign6100_e6279) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn9) * locals.var_fcw) + (assign6100_e6287 * locals.var_fcw_dn9))),)
    } else {
        (locals.var_fcf_csl, locals.var_fcf_csl_dn0, locals.var_fcf_csl_dn1, locals.var_fcf_csl_dn3, locals.var_fcf_csl_dn4, locals.var_fcf_csl_dn5, locals.var_fcf_csl_dn6, locals.var_fcf_csl_dn7, locals.var_fcf_csl_dn8, locals.var_fcf_csl_dn9,)
    }
};
        locals.var_fcf_csl = assign6100_e6292;
        locals.var_fcf_csl_dn0 = assign6100_e6292_d_n0;
        locals.var_fcf_csl_dn1 = assign6100_e6292_d_n1;
        locals.var_fcf_csl_dn3 = assign6100_e6292_d_n3;
        locals.var_fcf_csl_dn4 = assign6100_e6292_d_n4;
        locals.var_fcf_csl_dn5 = assign6100_e6292_d_n5;
        locals.var_fcf_csl_dn6 = assign6100_e6292_d_n6;
        locals.var_fcf_csl_dn7 = assign6100_e6292_d_n7;
        locals.var_fcf_csl_dn8 = assign6100_e6292_d_n8;
        locals.var_fcf_csl_dn9 = assign6100_e6292_d_n9;

        let (assign6110_e6316, assign6110_e6316_d_n0, assign6110_e6316_d_n1, assign6110_e6316_d_n3, assign6110_e6316_d_n4, assign6110_e6316_d_n5, assign6110_e6316_d_n6, assign6110_e6316_d_n7, assign6110_e6316_d_n8, assign6110_e6316_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6110_e6304: f64 = (0.5 - locals.var_fcia);
        let assign6110_e6306: f64 = (assign6110_e6304 / locals.var_fciwzb_p1);
        let assign6110_e6308: f64 = (assign6110_e6306 + locals.var_fcia);
        let assign6110_e6311: f64 = (locals.var_fcw * locals.var_latb_6);
        let assign6110_e6313: f64 = (assign6110_e6311 * 2.0);
        let assign6110_e6314: f64 = (assign6110_e6308 + assign6110_e6313);
        (assign6110_e6314, ((-((assign6110_e6304 * locals.var_fciwzb_p1_dn0) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn0 * locals.var_latb_6) * 2.0)), ((-((assign6110_e6304 * locals.var_fciwzb_p1_dn1) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn1 * locals.var_latb_6) * 2.0)), ((-((assign6110_e6304 * locals.var_fciwzb_p1_dn3) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn3 * locals.var_latb_6) * 2.0)), ((-((assign6110_e6304 * locals.var_fciwzb_p1_dn4) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn4 * locals.var_latb_6) * 2.0)), ((-((assign6110_e6304 * locals.var_fciwzb_p1_dn5) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn5 * locals.var_latb_6) * 2.0)), ((-((assign6110_e6304 * locals.var_fciwzb_p1_dn6) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn6 * locals.var_latb_6) * 2.0)), ((-((assign6110_e6304 * locals.var_fciwzb_p1_dn7) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn7 * locals.var_latb_6) * 2.0)), ((-((assign6110_e6304 * locals.var_fciwzb_p1_dn8) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn8 * locals.var_latb_6) * 2.0)), ((-((assign6110_e6304 * locals.var_fciwzb_p1_dn9) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn9 * locals.var_latb_6) * 2.0)),)
    } else {
        (locals.var_fcdfcsl_dw, locals.var_fcdfcsl_dw_dn0, locals.var_fcdfcsl_dw_dn1, locals.var_fcdfcsl_dw_dn3, locals.var_fcdfcsl_dw_dn4, locals.var_fcdfcsl_dw_dn5, locals.var_fcdfcsl_dw_dn6, locals.var_fcdfcsl_dw_dn7, locals.var_fcdfcsl_dw_dn8, locals.var_fcdfcsl_dw_dn9,)
    }
};
        locals.var_fcdfcsl_dw = assign6110_e6316;
        locals.var_fcdfcsl_dw_dn0 = assign6110_e6316_d_n0;
        locals.var_fcdfcsl_dw_dn1 = assign6110_e6316_d_n1;
        locals.var_fcdfcsl_dw_dn3 = assign6110_e6316_d_n3;
        locals.var_fcdfcsl_dw_dn4 = assign6110_e6316_d_n4;
        locals.var_fcdfcsl_dw_dn5 = assign6110_e6316_d_n5;
        locals.var_fcdfcsl_dw_dn6 = assign6110_e6316_d_n6;
        locals.var_fcdfcsl_dw_dn7 = assign6110_e6316_d_n7;
        locals.var_fcdfcsl_dw_dn8 = assign6110_e6316_d_n8;
        locals.var_fcdfcsl_dw_dn9 = assign6110_e6316_d_n9;

        let (assign6120_e6332, assign6120_e6332_d_n0, assign6120_e6332_d_n1, assign6120_e6332_d_n3, assign6120_e6332_d_n4, assign6120_e6332_d_n5, assign6120_e6332_d_n6, assign6120_e6332_d_n7, assign6120_e6332_d_n8, assign6120_e6332_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6120_e6329: f64 = (p.p115 * locals.var_fcw);
        let assign6120_e6330: f64 = (1.0 + assign6120_e6329);
        (assign6120_e6330, (p.p115 * locals.var_fcw_dn0), (p.p115 * locals.var_fcw_dn1), (p.p115 * locals.var_fcw_dn3), (p.p115 * locals.var_fcw_dn4), (p.p115 * locals.var_fcw_dn5), (p.p115 * locals.var_fcw_dn6), (p.p115 * locals.var_fcw_dn7), (p.p115 * locals.var_fcw_dn8), (p.p115 * locals.var_fcw_dn9),)
    } else {
        (locals.var_fciwzb_p1, locals.var_fciwzb_p1_dn0, locals.var_fciwzb_p1_dn1, locals.var_fciwzb_p1_dn3, locals.var_fciwzb_p1_dn4, locals.var_fciwzb_p1_dn5, locals.var_fciwzb_p1_dn6, locals.var_fciwzb_p1_dn7, locals.var_fciwzb_p1_dn8, locals.var_fciwzb_p1_dn9,)
    }
};
        locals.var_fciwzb_p1 = assign6120_e6332;
        locals.var_fciwzb_p1_dn0 = assign6120_e6332_d_n0;
        locals.var_fciwzb_p1_dn1 = assign6120_e6332_d_n1;
        locals.var_fciwzb_p1_dn3 = assign6120_e6332_d_n3;
        locals.var_fciwzb_p1_dn4 = assign6120_e6332_d_n4;
        locals.var_fciwzb_p1_dn5 = assign6120_e6332_d_n5;
        locals.var_fciwzb_p1_dn6 = assign6120_e6332_d_n6;
        locals.var_fciwzb_p1_dn7 = assign6120_e6332_d_n7;
        locals.var_fciwzb_p1_dn8 = assign6120_e6332_d_n8;
        locals.var_fciwzb_p1_dn9 = assign6120_e6332_d_n9;

        let (assign6130_e6345, assign6130_e6345_d_n0, assign6130_e6345_d_n1, assign6130_e6345_d_n3, assign6130_e6345_d_n4, assign6130_e6345_d_n5, assign6130_e6345_d_n6, assign6130_e6345_d_n7, assign6130_e6345_d_n8, assign6130_e6345_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6130_e6343: f64 = (locals.var_fciwzb_p1).ln();
        (assign6130_e6343, (locals.var_fciwzb_p1_dn0 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn1 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn3 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn4 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn5 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn6 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn7 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn8 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn9 / locals.var_fciwzb_p1),)
    } else {
        (locals.var_fcilnw_bl, locals.var_fcilnw_bl_dn0, locals.var_fcilnw_bl_dn1, locals.var_fcilnw_bl_dn3, locals.var_fcilnw_bl_dn4, locals.var_fcilnw_bl_dn5, locals.var_fcilnw_bl_dn6, locals.var_fcilnw_bl_dn7, locals.var_fcilnw_bl_dn8, locals.var_fcilnw_bl_dn9,)
    }
};
        locals.var_fcilnw_bl = assign6130_e6345;
        locals.var_fcilnw_bl_dn0 = assign6130_e6345_d_n0;
        locals.var_fcilnw_bl_dn1 = assign6130_e6345_d_n1;
        locals.var_fcilnw_bl_dn3 = assign6130_e6345_d_n3;
        locals.var_fcilnw_bl_dn4 = assign6130_e6345_d_n4;
        locals.var_fcilnw_bl_dn5 = assign6130_e6345_d_n5;
        locals.var_fcilnw_bl_dn6 = assign6130_e6345_d_n6;
        locals.var_fcilnw_bl_dn7 = assign6130_e6345_d_n7;
        locals.var_fcilnw_bl_dn8 = assign6130_e6345_d_n8;
        locals.var_fcilnw_bl_dn9 = assign6130_e6345_d_n9;

        let (assign6140_e6359,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6140_e6357: f64 = (locals.var_latl_6 * locals.var_inv_latb);
        (assign6140_e6357,)
    } else {
        (locals.var_fcia,)
    }
};
        locals.var_fcia = assign6140_e6359;

        let (assign6150_e6385, assign6150_e6385_d_n0, assign6150_e6385_d_n1, assign6150_e6385_d_n3, assign6150_e6385_d_n4, assign6150_e6385_d_n5, assign6150_e6385_d_n6, assign6150_e6385_d_n7, assign6150_e6385_d_n8, assign6150_e6385_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6150_e6372: f64 = (0.5 - locals.var_fcia);
        let assign6150_e6373: f64 = (locals.var_fcilnw_bl * assign6150_e6372);
        let assign6150_e6375: f64 = (assign6150_e6373 * locals.var_inv_latb);
        let assign6150_e6379: f64 = (locals.var_latl_6 * locals.var_fcw);
        let assign6150_e6380: f64 = (locals.var_fcia + assign6150_e6379);
        let assign6150_e6382: f64 = (assign6150_e6380 * locals.var_fcw);
        let assign6150_e6383: f64 = (assign6150_e6375 + assign6150_e6382);
        (assign6150_e6383, (((locals.var_fcilnw_bl_dn0 * assign6150_e6372) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn0) * locals.var_fcw) + (assign6150_e6380 * locals.var_fcw_dn0))), (((locals.var_fcilnw_bl_dn1 * assign6150_e6372) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn1) * locals.var_fcw) + (assign6150_e6380 * locals.var_fcw_dn1))), (((locals.var_fcilnw_bl_dn3 * assign6150_e6372) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn3) * locals.var_fcw) + (assign6150_e6380 * locals.var_fcw_dn3))), (((locals.var_fcilnw_bl_dn4 * assign6150_e6372) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn4) * locals.var_fcw) + (assign6150_e6380 * locals.var_fcw_dn4))), (((locals.var_fcilnw_bl_dn5 * assign6150_e6372) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn5) * locals.var_fcw) + (assign6150_e6380 * locals.var_fcw_dn5))), (((locals.var_fcilnw_bl_dn6 * assign6150_e6372) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn6) * locals.var_fcw) + (assign6150_e6380 * locals.var_fcw_dn6))), (((locals.var_fcilnw_bl_dn7 * assign6150_e6372) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn7) * locals.var_fcw) + (assign6150_e6380 * locals.var_fcw_dn7))), (((locals.var_fcilnw_bl_dn8 * assign6150_e6372) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn8) * locals.var_fcw) + (assign6150_e6380 * locals.var_fcw_dn8))), (((locals.var_fcilnw_bl_dn9 * assign6150_e6372) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn9) * locals.var_fcw) + (assign6150_e6380 * locals.var_fcw_dn9))),)
    } else {
        (locals.var_fcf_csb, locals.var_fcf_csb_dn0, locals.var_fcf_csb_dn1, locals.var_fcf_csb_dn3, locals.var_fcf_csb_dn4, locals.var_fcf_csb_dn5, locals.var_fcf_csb_dn6, locals.var_fcf_csb_dn7, locals.var_fcf_csb_dn8, locals.var_fcf_csb_dn9,)
    }
};
        locals.var_fcf_csb = assign6150_e6385;
        locals.var_fcf_csb_dn0 = assign6150_e6385_d_n0;
        locals.var_fcf_csb_dn1 = assign6150_e6385_d_n1;
        locals.var_fcf_csb_dn3 = assign6150_e6385_d_n3;
        locals.var_fcf_csb_dn4 = assign6150_e6385_d_n4;
        locals.var_fcf_csb_dn5 = assign6150_e6385_d_n5;
        locals.var_fcf_csb_dn6 = assign6150_e6385_d_n6;
        locals.var_fcf_csb_dn7 = assign6150_e6385_d_n7;
        locals.var_fcf_csb_dn8 = assign6150_e6385_d_n8;
        locals.var_fcf_csb_dn9 = assign6150_e6385_d_n9;

        let (assign6160_e6409, assign6160_e6409_d_n0, assign6160_e6409_d_n1, assign6160_e6409_d_n3, assign6160_e6409_d_n4, assign6160_e6409_d_n5, assign6160_e6409_d_n6, assign6160_e6409_d_n7, assign6160_e6409_d_n8, assign6160_e6409_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6160_e6397: f64 = (0.5 - locals.var_fcia);
        let assign6160_e6399: f64 = (assign6160_e6397 / locals.var_fciwzb_p1);
        let assign6160_e6401: f64 = (assign6160_e6399 + locals.var_fcia);
        let assign6160_e6404: f64 = (locals.var_fcw * locals.var_latl_6);
        let assign6160_e6406: f64 = (assign6160_e6404 * 2.0);
        let assign6160_e6407: f64 = (assign6160_e6401 + assign6160_e6406);
        (assign6160_e6407, ((-((assign6160_e6397 * locals.var_fciwzb_p1_dn0) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn0 * locals.var_latl_6) * 2.0)), ((-((assign6160_e6397 * locals.var_fciwzb_p1_dn1) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn1 * locals.var_latl_6) * 2.0)), ((-((assign6160_e6397 * locals.var_fciwzb_p1_dn3) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn3 * locals.var_latl_6) * 2.0)), ((-((assign6160_e6397 * locals.var_fciwzb_p1_dn4) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn4 * locals.var_latl_6) * 2.0)), ((-((assign6160_e6397 * locals.var_fciwzb_p1_dn5) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn5 * locals.var_latl_6) * 2.0)), ((-((assign6160_e6397 * locals.var_fciwzb_p1_dn6) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn6 * locals.var_latl_6) * 2.0)), ((-((assign6160_e6397 * locals.var_fciwzb_p1_dn7) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn7 * locals.var_latl_6) * 2.0)), ((-((assign6160_e6397 * locals.var_fciwzb_p1_dn8) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn8 * locals.var_latl_6) * 2.0)), ((-((assign6160_e6397 * locals.var_fciwzb_p1_dn9) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn9 * locals.var_latl_6) * 2.0)),)
    } else {
        (locals.var_fcdfcsb_dw, locals.var_fcdfcsb_dw_dn0, locals.var_fcdfcsb_dw_dn1, locals.var_fcdfcsb_dw_dn3, locals.var_fcdfcsb_dw_dn4, locals.var_fcdfcsb_dw_dn5, locals.var_fcdfcsb_dw_dn6, locals.var_fcdfcsb_dw_dn7, locals.var_fcdfcsb_dw_dn8, locals.var_fcdfcsb_dw_dn9,)
    }
};
        locals.var_fcdfcsb_dw = assign6160_e6409;
        locals.var_fcdfcsb_dw_dn0 = assign6160_e6409_d_n0;
        locals.var_fcdfcsb_dw_dn1 = assign6160_e6409_d_n1;
        locals.var_fcdfcsb_dw_dn3 = assign6160_e6409_d_n3;
        locals.var_fcdfcsb_dw_dn4 = assign6160_e6409_d_n4;
        locals.var_fcdfcsb_dw_dn5 = assign6160_e6409_d_n5;
        locals.var_fcdfcsb_dw_dn6 = assign6160_e6409_d_n6;
        locals.var_fcdfcsb_dw_dn7 = assign6160_e6409_d_n7;
        locals.var_fcdfcsb_dw_dn8 = assign6160_e6409_d_n8;
        locals.var_fcdfcsb_dw_dn9 = assign6160_e6409_d_n9;

        let (assign6170_e6425, assign6170_e6425_d_n0, assign6170_e6425_d_n1, assign6170_e6425_d_n3, assign6170_e6425_d_n4, assign6170_e6425_d_n5, assign6170_e6425_d_n6, assign6170_e6425_d_n7, assign6170_e6425_d_n8, assign6170_e6425_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6170_e6421: f64 = (locals.var_fcf_csl - locals.var_fcf_csb);
        let assign6170_e6423: f64 = (assign6170_e6421 / locals.var_lat_delta);
        (assign6170_e6423, ((locals.var_fcf_csl_dn0 - locals.var_fcf_csb_dn0) / locals.var_lat_delta), ((locals.var_fcf_csl_dn1 - locals.var_fcf_csb_dn1) / locals.var_lat_delta), ((locals.var_fcf_csl_dn3 - locals.var_fcf_csb_dn3) / locals.var_lat_delta), ((locals.var_fcf_csl_dn4 - locals.var_fcf_csb_dn4) / locals.var_lat_delta), ((locals.var_fcf_csl_dn5 - locals.var_fcf_csb_dn5) / locals.var_lat_delta), ((locals.var_fcf_csl_dn6 - locals.var_fcf_csb_dn6) / locals.var_lat_delta), ((locals.var_fcf_csl_dn7 - locals.var_fcf_csb_dn7) / locals.var_lat_delta), ((locals.var_fcf_csl_dn8 - locals.var_fcf_csb_dn8) / locals.var_lat_delta), ((locals.var_fcf_csl_dn9 - locals.var_fcf_csb_dn9) / locals.var_lat_delta),)
    } else {
        (locals.var_fcf_ci, locals.var_fcf_ci_dn0, locals.var_fcf_ci_dn1, locals.var_fcf_ci_dn3, locals.var_fcf_ci_dn4, locals.var_fcf_ci_dn5, locals.var_fcf_ci_dn6, locals.var_fcf_ci_dn7, locals.var_fcf_ci_dn8, locals.var_fcf_ci_dn9,)
    }
};
        locals.var_fcf_ci = assign6170_e6425;
        locals.var_fcf_ci_dn0 = assign6170_e6425_d_n0;
        locals.var_fcf_ci_dn1 = assign6170_e6425_d_n1;
        locals.var_fcf_ci_dn3 = assign6170_e6425_d_n3;
        locals.var_fcf_ci_dn4 = assign6170_e6425_d_n4;
        locals.var_fcf_ci_dn5 = assign6170_e6425_d_n5;
        locals.var_fcf_ci_dn6 = assign6170_e6425_d_n6;
        locals.var_fcf_ci_dn7 = assign6170_e6425_d_n7;
        locals.var_fcf_ci_dn8 = assign6170_e6425_d_n8;
        locals.var_fcf_ci_dn9 = assign6170_e6425_d_n9;

        let (assign6180_e6450, assign6180_e6450_d_n0, assign6180_e6450_d_n1, assign6180_e6450_d_n3, assign6180_e6450_d_n4, assign6180_e6450_d_n5, assign6180_e6450_d_n6, assign6180_e6450_d_n7, assign6180_e6450_d_n8, assign6180_e6450_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6180_e6436: f64 = (-2.0);
        let assign6180_e6438: f64 = (assign6180_e6436 * locals.var_lat_delta);
        let assign6180_e6441: f64 = (locals.var_fckdelta * locals.var_fckdelta);
        let assign6180_e6442: f64 = (assign6180_e6438 / assign6180_e6441);
        let assign6180_e6444: f64 = (assign6180_e6442 * locals.var_fck);
        let assign6180_e6446: f64 = (assign6180_e6444 * locals.var_ln_lat);
        let assign6180_e6448: f64 = (assign6180_e6446 * locals.var_fcdick_ditf);
        (assign6180_e6448, ((((((-((assign6180_e6438 * ((locals.var_fckdelta_dn0 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn0))) / (assign6180_e6441 * assign6180_e6441))) * locals.var_fck) + (assign6180_e6442 * locals.var_fck_dn0)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6180_e6446 * locals.var_fcdick_ditf_dn0)), ((((((-((assign6180_e6438 * ((locals.var_fckdelta_dn1 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn1))) / (assign6180_e6441 * assign6180_e6441))) * locals.var_fck) + (assign6180_e6442 * locals.var_fck_dn1)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6180_e6446 * locals.var_fcdick_ditf_dn1)), ((((((-((assign6180_e6438 * ((locals.var_fckdelta_dn3 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn3))) / (assign6180_e6441 * assign6180_e6441))) * locals.var_fck) + (assign6180_e6442 * locals.var_fck_dn3)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6180_e6446 * locals.var_fcdick_ditf_dn3)), ((((((-((assign6180_e6438 * ((locals.var_fckdelta_dn4 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn4))) / (assign6180_e6441 * assign6180_e6441))) * locals.var_fck) + (assign6180_e6442 * locals.var_fck_dn4)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6180_e6446 * locals.var_fcdick_ditf_dn4)), ((((((-((assign6180_e6438 * ((locals.var_fckdelta_dn5 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn5))) / (assign6180_e6441 * assign6180_e6441))) * locals.var_fck) + (assign6180_e6442 * locals.var_fck_dn5)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6180_e6446 * locals.var_fcdick_ditf_dn5)), ((((((-((assign6180_e6438 * ((locals.var_fckdelta_dn6 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn6))) / (assign6180_e6441 * assign6180_e6441))) * locals.var_fck) + (assign6180_e6442 * locals.var_fck_dn6)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6180_e6446 * locals.var_fcdick_ditf_dn6)), ((((((-((assign6180_e6438 * ((locals.var_fckdelta_dn7 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn7))) / (assign6180_e6441 * assign6180_e6441))) * locals.var_fck) + (assign6180_e6442 * locals.var_fck_dn7)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6180_e6446 * locals.var_fcdick_ditf_dn7)), ((((((-((assign6180_e6438 * ((locals.var_fckdelta_dn8 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn8))) / (assign6180_e6441 * assign6180_e6441))) * locals.var_fck) + (assign6180_e6442 * locals.var_fck_dn8)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6180_e6446 * locals.var_fcdick_ditf_dn8)), ((((((-((assign6180_e6438 * ((locals.var_fckdelta_dn9 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn9))) / (assign6180_e6441 * assign6180_e6441))) * locals.var_fck) + (assign6180_e6442 * locals.var_fck_dn9)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6180_e6446 * locals.var_fcdick_ditf_dn9)),)
    } else {
        (locals.var_fcdw_ditf, locals.var_fcdw_ditf_dn0, locals.var_fcdw_ditf_dn1, locals.var_fcdw_ditf_dn3, locals.var_fcdw_ditf_dn4, locals.var_fcdw_ditf_dn5, locals.var_fcdw_ditf_dn6, locals.var_fcdw_ditf_dn7, locals.var_fcdw_ditf_dn8, locals.var_fcdw_ditf_dn9,)
    }
};
        locals.var_fcdw_ditf = assign6180_e6450;
        locals.var_fcdw_ditf_dn0 = assign6180_e6450_d_n0;
        locals.var_fcdw_ditf_dn1 = assign6180_e6450_d_n1;
        locals.var_fcdw_ditf_dn3 = assign6180_e6450_d_n3;
        locals.var_fcdw_ditf_dn4 = assign6180_e6450_d_n4;
        locals.var_fcdw_ditf_dn5 = assign6180_e6450_d_n5;
        locals.var_fcdw_ditf_dn6 = assign6180_e6450_d_n6;
        locals.var_fcdw_ditf_dn7 = assign6180_e6450_d_n7;
        locals.var_fcdw_ditf_dn8 = assign6180_e6450_d_n8;
        locals.var_fcdw_ditf_dn9 = assign6180_e6450_d_n9;

        let (assign6190_e6468, assign6190_e6468_d_n0, assign6190_e6468_d_n1, assign6190_e6468_d_n3, assign6190_e6468_d_n4, assign6190_e6468_d_n5, assign6190_e6468_d_n6, assign6190_e6468_d_n7, assign6190_e6468_d_n8, assign6190_e6468_d_n9,) = {
    if ((((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign6190_e6462: f64 = (locals.var_fcdfcsl_dw - locals.var_fcdfcsb_dw);
        let assign6190_e6464: f64 = (assign6190_e6462 * locals.var_fcdw_ditf);
        let assign6190_e6466: f64 = (assign6190_e6464 / locals.var_lat_delta);
        (assign6190_e6466, ((((locals.var_fcdfcsl_dw_dn0 - locals.var_fcdfcsb_dw_dn0) * locals.var_fcdw_ditf) + (assign6190_e6462 * locals.var_fcdw_ditf_dn0)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn1 - locals.var_fcdfcsb_dw_dn1) * locals.var_fcdw_ditf) + (assign6190_e6462 * locals.var_fcdw_ditf_dn1)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn3 - locals.var_fcdfcsb_dw_dn3) * locals.var_fcdw_ditf) + (assign6190_e6462 * locals.var_fcdw_ditf_dn3)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn4 - locals.var_fcdfcsb_dw_dn4) * locals.var_fcdw_ditf) + (assign6190_e6462 * locals.var_fcdw_ditf_dn4)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn5 - locals.var_fcdfcsb_dw_dn5) * locals.var_fcdw_ditf) + (assign6190_e6462 * locals.var_fcdw_ditf_dn5)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn6 - locals.var_fcdfcsb_dw_dn6) * locals.var_fcdw_ditf) + (assign6190_e6462 * locals.var_fcdw_ditf_dn6)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn7 - locals.var_fcdfcsb_dw_dn7) * locals.var_fcdw_ditf) + (assign6190_e6462 * locals.var_fcdw_ditf_dn7)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn8 - locals.var_fcdfcsb_dw_dn8) * locals.var_fcdw_ditf) + (assign6190_e6462 * locals.var_fcdw_ditf_dn8)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn9 - locals.var_fcdfcsb_dw_dn9) * locals.var_fcdw_ditf) + (assign6190_e6462 * locals.var_fcdw_ditf_dn9)) / locals.var_lat_delta),)
    } else {
        (locals.var_fcdfc_ditf, locals.var_fcdfc_ditf_dn0, locals.var_fcdfc_ditf_dn1, locals.var_fcdfc_ditf_dn3, locals.var_fcdfc_ditf_dn4, locals.var_fcdfc_ditf_dn5, locals.var_fcdfc_ditf_dn6, locals.var_fcdfc_ditf_dn7, locals.var_fcdfc_ditf_dn8, locals.var_fcdfc_ditf_dn9,)
    }
};
        locals.var_fcdfc_ditf = assign6190_e6468;
        locals.var_fcdfc_ditf_dn0 = assign6190_e6468_d_n0;
        locals.var_fcdfc_ditf_dn1 = assign6190_e6468_d_n1;
        locals.var_fcdfc_ditf_dn3 = assign6190_e6468_d_n3;
        locals.var_fcdfc_ditf_dn4 = assign6190_e6468_d_n4;
        locals.var_fcdfc_ditf_dn5 = assign6190_e6468_d_n5;
        locals.var_fcdfc_ditf_dn6 = assign6190_e6468_d_n6;
        locals.var_fcdfc_ditf_dn7 = assign6190_e6468_d_n7;
        locals.var_fcdfc_ditf_dn8 = assign6190_e6468_d_n8;
        locals.var_fcdfc_ditf_dn9 = assign6190_e6468_d_n9;

        let (assign6200_e6486, assign6200_e6486_d_n0, assign6200_e6486_d_n1, assign6200_e6486_d_n3, assign6200_e6486_d_n4, assign6200_e6486_d_n5, assign6200_e6486_d_n6, assign6200_e6486_d_n7, assign6200_e6486_d_n8, assign6200_e6486_d_n9,) = {
    if (((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 == 0.0)) {
        let assign6200_e6478: f64 = (1.0 - locals.var_fcick);
        let assign6200_e6482: f64 = (locals.var_fcick * p.p115);
        let assign6200_e6483: f64 = (1.0 + assign6200_e6482);
        let assign6200_e6484: f64 = (assign6200_e6478 / assign6200_e6483);
        (assign6200_e6484, ((((-locals.var_fcick_dn0) * assign6200_e6483) - (assign6200_e6478 * (locals.var_fcick_dn0 * p.p115))) / (assign6200_e6483 * assign6200_e6483)), ((((-locals.var_fcick_dn1) * assign6200_e6483) - (assign6200_e6478 * (locals.var_fcick_dn1 * p.p115))) / (assign6200_e6483 * assign6200_e6483)), ((((-locals.var_fcick_dn3) * assign6200_e6483) - (assign6200_e6478 * (locals.var_fcick_dn3 * p.p115))) / (assign6200_e6483 * assign6200_e6483)), ((((-locals.var_fcick_dn4) * assign6200_e6483) - (assign6200_e6478 * (locals.var_fcick_dn4 * p.p115))) / (assign6200_e6483 * assign6200_e6483)), ((((-locals.var_fcick_dn5) * assign6200_e6483) - (assign6200_e6478 * (locals.var_fcick_dn5 * p.p115))) / (assign6200_e6483 * assign6200_e6483)), ((((-locals.var_fcick_dn6) * assign6200_e6483) - (assign6200_e6478 * (locals.var_fcick_dn6 * p.p115))) / (assign6200_e6483 * assign6200_e6483)), ((((-locals.var_fcick_dn7) * assign6200_e6483) - (assign6200_e6478 * (locals.var_fcick_dn7 * p.p115))) / (assign6200_e6483 * assign6200_e6483)), ((((-locals.var_fcick_dn8) * assign6200_e6483) - (assign6200_e6478 * (locals.var_fcick_dn8 * p.p115))) / (assign6200_e6483 * assign6200_e6483)), ((((-locals.var_fcick_dn9) * assign6200_e6483) - (assign6200_e6478 * (locals.var_fcick_dn9 * p.p115))) / (assign6200_e6483 * assign6200_e6483)),)
    } else {
        (locals.var_fcw, locals.var_fcw_dn0, locals.var_fcw_dn1, locals.var_fcw_dn3, locals.var_fcw_dn4, locals.var_fcw_dn5, locals.var_fcw_dn6, locals.var_fcw_dn7, locals.var_fcw_dn8, locals.var_fcw_dn9,)
    }
};
        locals.var_fcw = assign6200_e6486;
        locals.var_fcw_dn0 = assign6200_e6486_d_n0;
        locals.var_fcw_dn1 = assign6200_e6486_d_n1;
        locals.var_fcw_dn3 = assign6200_e6486_d_n3;
        locals.var_fcw_dn4 = assign6200_e6486_d_n4;
        locals.var_fcw_dn5 = assign6200_e6486_d_n5;
        locals.var_fcw_dn6 = assign6200_e6486_d_n6;
        locals.var_fcw_dn7 = assign6200_e6486_d_n7;
        locals.var_fcw_dn8 = assign6200_e6486_d_n8;
        locals.var_fcw_dn9 = assign6200_e6486_d_n9;

        let (assign6210_e6500, assign6210_e6500_d_n0, assign6210_e6500_d_n1, assign6210_e6500_d_n3, assign6210_e6500_d_n4, assign6210_e6500_d_n5, assign6210_e6500_d_n6, assign6210_e6500_d_n7, assign6210_e6500_d_n8, assign6210_e6500_d_n9,) = {
    if (((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 == 0.0)) {
        let assign6210_e6497: f64 = (p.p115 * locals.var_fcw);
        let assign6210_e6498: f64 = (1.0 + assign6210_e6497);
        (assign6210_e6498, (p.p115 * locals.var_fcw_dn0), (p.p115 * locals.var_fcw_dn1), (p.p115 * locals.var_fcw_dn3), (p.p115 * locals.var_fcw_dn4), (p.p115 * locals.var_fcw_dn5), (p.p115 * locals.var_fcw_dn6), (p.p115 * locals.var_fcw_dn7), (p.p115 * locals.var_fcw_dn8), (p.p115 * locals.var_fcw_dn9),)
    } else {
        (locals.var_fclatbw, locals.var_fclatbw_dn0, locals.var_fclatbw_dn1, locals.var_fclatbw_dn3, locals.var_fclatbw_dn4, locals.var_fclatbw_dn5, locals.var_fclatbw_dn6, locals.var_fclatbw_dn7, locals.var_fclatbw_dn8, locals.var_fclatbw_dn9,)
    }
};
        locals.var_fclatbw = assign6210_e6500;
        locals.var_fclatbw_dn0 = assign6210_e6500_d_n0;
        locals.var_fclatbw_dn1 = assign6210_e6500_d_n1;
        locals.var_fclatbw_dn3 = assign6210_e6500_d_n3;
        locals.var_fclatbw_dn4 = assign6210_e6500_d_n4;
        locals.var_fclatbw_dn5 = assign6210_e6500_d_n5;
        locals.var_fclatbw_dn6 = assign6210_e6500_d_n6;
        locals.var_fclatbw_dn7 = assign6210_e6500_d_n7;
        locals.var_fclatbw_dn8 = assign6210_e6500_d_n8;
        locals.var_fclatbw_dn9 = assign6210_e6500_d_n9;

        let (assign6220_e6522, assign6220_e6522_d_n0, assign6220_e6522_d_n1, assign6220_e6522_d_n3, assign6220_e6522_d_n4, assign6220_e6522_d_n5, assign6220_e6522_d_n6, assign6220_e6522_d_n7, assign6220_e6522_d_n8, assign6220_e6522_d_n9,) = {
    if (((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 == 0.0)) {
        let assign6220_e6510: f64 = (locals.var_fcw * locals.var_fcw);
        let assign6220_e6514: f64 = (locals.var_latb_6 * 2.0);
        let assign6220_e6516: f64 = (assign6220_e6514 * locals.var_fcw);
        let assign6220_e6517: f64 = (1.0 + assign6220_e6516);
        let assign6220_e6518: f64 = (assign6220_e6510 * assign6220_e6517);
        let assign6220_e6520: f64 = (assign6220_e6518 / locals.var_fclatbw);
        (assign6220_e6520, (((((((locals.var_fcw_dn0 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn0)) * assign6220_e6517) + (assign6220_e6510 * (assign6220_e6514 * locals.var_fcw_dn0))) * locals.var_fclatbw) - (assign6220_e6518 * locals.var_fclatbw_dn0)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn1 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn1)) * assign6220_e6517) + (assign6220_e6510 * (assign6220_e6514 * locals.var_fcw_dn1))) * locals.var_fclatbw) - (assign6220_e6518 * locals.var_fclatbw_dn1)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn3 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn3)) * assign6220_e6517) + (assign6220_e6510 * (assign6220_e6514 * locals.var_fcw_dn3))) * locals.var_fclatbw) - (assign6220_e6518 * locals.var_fclatbw_dn3)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn4 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn4)) * assign6220_e6517) + (assign6220_e6510 * (assign6220_e6514 * locals.var_fcw_dn4))) * locals.var_fclatbw) - (assign6220_e6518 * locals.var_fclatbw_dn4)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn5 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn5)) * assign6220_e6517) + (assign6220_e6510 * (assign6220_e6514 * locals.var_fcw_dn5))) * locals.var_fclatbw) - (assign6220_e6518 * locals.var_fclatbw_dn5)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn6 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn6)) * assign6220_e6517) + (assign6220_e6510 * (assign6220_e6514 * locals.var_fcw_dn6))) * locals.var_fclatbw) - (assign6220_e6518 * locals.var_fclatbw_dn6)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn7 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn7)) * assign6220_e6517) + (assign6220_e6510 * (assign6220_e6514 * locals.var_fcw_dn7))) * locals.var_fclatbw) - (assign6220_e6518 * locals.var_fclatbw_dn7)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn8 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn8)) * assign6220_e6517) + (assign6220_e6510 * (assign6220_e6514 * locals.var_fcw_dn8))) * locals.var_fclatbw) - (assign6220_e6518 * locals.var_fclatbw_dn8)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn9 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn9)) * assign6220_e6517) + (assign6220_e6510 * (assign6220_e6514 * locals.var_fcw_dn9))) * locals.var_fclatbw) - (assign6220_e6518 * locals.var_fclatbw_dn9)) / (locals.var_fclatbw * locals.var_fclatbw)),)
    } else {
        (locals.var_fcf_ci, locals.var_fcf_ci_dn0, locals.var_fcf_ci_dn1, locals.var_fcf_ci_dn3, locals.var_fcf_ci_dn4, locals.var_fcf_ci_dn5, locals.var_fcf_ci_dn6, locals.var_fcf_ci_dn7, locals.var_fcf_ci_dn8, locals.var_fcf_ci_dn9,)
    }
};
        locals.var_fcf_ci = assign6220_e6522;
        locals.var_fcf_ci_dn0 = assign6220_e6522_d_n0;
        locals.var_fcf_ci_dn1 = assign6220_e6522_d_n1;
        locals.var_fcf_ci_dn3 = assign6220_e6522_d_n3;
        locals.var_fcf_ci_dn4 = assign6220_e6522_d_n4;
        locals.var_fcf_ci_dn5 = assign6220_e6522_d_n5;
        locals.var_fcf_ci_dn6 = assign6220_e6522_d_n6;
        locals.var_fcf_ci_dn7 = assign6220_e6522_d_n7;
        locals.var_fcf_ci_dn8 = assign6220_e6522_d_n8;
        locals.var_fcf_ci_dn9 = assign6220_e6522_d_n9;

    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6230_e6541, assign6230_e6541_d_n0, assign6230_e6541_d_n1, assign6230_e6541_d_n3, assign6230_e6541_d_n4, assign6230_e6541_d_n5, assign6230_e6541_d_n6, assign6230_e6541_d_n7, assign6230_e6541_d_n8, assign6230_e6541_d_n9,) = {
    if (((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 == 0.0)) {
        let assign6230_e6531: f64 = (-locals.var_fcdick_ditf);
        let assign6230_e6533: f64 = (assign6230_e6531 * locals.var_fclatbw);
        let assign6230_e6537: f64 = (locals.var_fcick * p.p115);
        let assign6230_e6538: f64 = (1.0 + assign6230_e6537);
        let assign6230_e6539: f64 = (assign6230_e6533 / assign6230_e6538);
        (assign6230_e6539, ((((((-locals.var_fcdick_ditf_dn0) * locals.var_fclatbw) + (assign6230_e6531 * locals.var_fclatbw_dn0)) * assign6230_e6538) - (assign6230_e6533 * (locals.var_fcick_dn0 * p.p115))) / (assign6230_e6538 * assign6230_e6538)), ((((((-locals.var_fcdick_ditf_dn1) * locals.var_fclatbw) + (assign6230_e6531 * locals.var_fclatbw_dn1)) * assign6230_e6538) - (assign6230_e6533 * (locals.var_fcick_dn1 * p.p115))) / (assign6230_e6538 * assign6230_e6538)), ((((((-locals.var_fcdick_ditf_dn3) * locals.var_fclatbw) + (assign6230_e6531 * locals.var_fclatbw_dn3)) * assign6230_e6538) - (assign6230_e6533 * (locals.var_fcick_dn3 * p.p115))) / (assign6230_e6538 * assign6230_e6538)), ((((((-locals.var_fcdick_ditf_dn4) * locals.var_fclatbw) + (assign6230_e6531 * locals.var_fclatbw_dn4)) * assign6230_e6538) - (assign6230_e6533 * (locals.var_fcick_dn4 * p.p115))) / (assign6230_e6538 * assign6230_e6538)), ((((((-locals.var_fcdick_ditf_dn5) * locals.var_fclatbw) + (assign6230_e6531 * locals.var_fclatbw_dn5)) * assign6230_e6538) - (assign6230_e6533 * (locals.var_fcick_dn5 * p.p115))) / (assign6230_e6538 * assign6230_e6538)), ((((((-locals.var_fcdick_ditf_dn6) * locals.var_fclatbw) + (assign6230_e6531 * locals.var_fclatbw_dn6)) * assign6230_e6538) - (assign6230_e6533 * (locals.var_fcick_dn6 * p.p115))) / (assign6230_e6538 * assign6230_e6538)), ((((((-locals.var_fcdick_ditf_dn7) * locals.var_fclatbw) + (assign6230_e6531 * locals.var_fclatbw_dn7)) * assign6230_e6538) - (assign6230_e6533 * (locals.var_fcick_dn7 * p.p115))) / (assign6230_e6538 * assign6230_e6538)), ((((((-locals.var_fcdick_ditf_dn8) * locals.var_fclatbw) + (assign6230_e6531 * locals.var_fclatbw_dn8)) * assign6230_e6538) - (assign6230_e6533 * (locals.var_fcick_dn8 * p.p115))) / (assign6230_e6538 * assign6230_e6538)), ((((((-locals.var_fcdick_ditf_dn9) * locals.var_fclatbw) + (assign6230_e6531 * locals.var_fclatbw_dn9)) * assign6230_e6538) - (assign6230_e6533 * (locals.var_fcick_dn9 * p.p115))) / (assign6230_e6538 * assign6230_e6538)),)
    } else {
        (locals.var_fcdw_ditf, locals.var_fcdw_ditf_dn0, locals.var_fcdw_ditf_dn1, locals.var_fcdw_ditf_dn3, locals.var_fcdw_ditf_dn4, locals.var_fcdw_ditf_dn5, locals.var_fcdw_ditf_dn6, locals.var_fcdw_ditf_dn7, locals.var_fcdw_ditf_dn8, locals.var_fcdw_ditf_dn9,)
    }
};
        locals.var_fcdw_ditf = assign6230_e6541;
        locals.var_fcdw_ditf_dn0 = assign6230_e6541_d_n0;
        locals.var_fcdw_ditf_dn1 = assign6230_e6541_d_n1;
        locals.var_fcdw_ditf_dn3 = assign6230_e6541_d_n3;
        locals.var_fcdw_ditf_dn4 = assign6230_e6541_d_n4;
        locals.var_fcdw_ditf_dn5 = assign6230_e6541_d_n5;
        locals.var_fcdw_ditf_dn6 = assign6230_e6541_d_n6;
        locals.var_fcdw_ditf_dn7 = assign6230_e6541_d_n7;
        locals.var_fcdw_ditf_dn8 = assign6230_e6541_d_n8;
        locals.var_fcdw_ditf_dn9 = assign6230_e6541_d_n9;

        let (assign6240_e6561, assign6240_e6561_d_n0, assign6240_e6561_d_n1, assign6240_e6561_d_n3, assign6240_e6561_d_n4, assign6240_e6561_d_n5, assign6240_e6561_d_n6, assign6240_e6561_d_n7, assign6240_e6561_d_n8, assign6240_e6561_d_n9,) = {
    if (((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) && (locals.var_guard128 == 0.0)) {
        let assign6240_e6554: f64 = (locals.var_fclatbw * locals.var_fclatbw);
        let assign6240_e6555: f64 = (1.0 / assign6240_e6554);
        let assign6240_e6556: f64 = (1.0 + assign6240_e6555);
        let assign6240_e6557: f64 = (locals.var_fcw * assign6240_e6556);
        let assign6240_e6559: f64 = (assign6240_e6557 * locals.var_fcdw_ditf);
        (assign6240_e6559, ((((locals.var_fcw_dn0 * assign6240_e6556) + (locals.var_fcw * (-(((locals.var_fclatbw_dn0 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn0)) / (assign6240_e6554 * assign6240_e6554))))) * locals.var_fcdw_ditf) + (assign6240_e6557 * locals.var_fcdw_ditf_dn0)), ((((locals.var_fcw_dn1 * assign6240_e6556) + (locals.var_fcw * (-(((locals.var_fclatbw_dn1 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn1)) / (assign6240_e6554 * assign6240_e6554))))) * locals.var_fcdw_ditf) + (assign6240_e6557 * locals.var_fcdw_ditf_dn1)), ((((locals.var_fcw_dn3 * assign6240_e6556) + (locals.var_fcw * (-(((locals.var_fclatbw_dn3 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn3)) / (assign6240_e6554 * assign6240_e6554))))) * locals.var_fcdw_ditf) + (assign6240_e6557 * locals.var_fcdw_ditf_dn3)), ((((locals.var_fcw_dn4 * assign6240_e6556) + (locals.var_fcw * (-(((locals.var_fclatbw_dn4 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn4)) / (assign6240_e6554 * assign6240_e6554))))) * locals.var_fcdw_ditf) + (assign6240_e6557 * locals.var_fcdw_ditf_dn4)), ((((locals.var_fcw_dn5 * assign6240_e6556) + (locals.var_fcw * (-(((locals.var_fclatbw_dn5 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn5)) / (assign6240_e6554 * assign6240_e6554))))) * locals.var_fcdw_ditf) + (assign6240_e6557 * locals.var_fcdw_ditf_dn5)), ((((locals.var_fcw_dn6 * assign6240_e6556) + (locals.var_fcw * (-(((locals.var_fclatbw_dn6 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn6)) / (assign6240_e6554 * assign6240_e6554))))) * locals.var_fcdw_ditf) + (assign6240_e6557 * locals.var_fcdw_ditf_dn6)), ((((locals.var_fcw_dn7 * assign6240_e6556) + (locals.var_fcw * (-(((locals.var_fclatbw_dn7 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn7)) / (assign6240_e6554 * assign6240_e6554))))) * locals.var_fcdw_ditf) + (assign6240_e6557 * locals.var_fcdw_ditf_dn7)), ((((locals.var_fcw_dn8 * assign6240_e6556) + (locals.var_fcw * (-(((locals.var_fclatbw_dn8 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn8)) / (assign6240_e6554 * assign6240_e6554))))) * locals.var_fcdw_ditf) + (assign6240_e6557 * locals.var_fcdw_ditf_dn8)), ((((locals.var_fcw_dn9 * assign6240_e6556) + (locals.var_fcw * (-(((locals.var_fclatbw_dn9 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn9)) / (assign6240_e6554 * assign6240_e6554))))) * locals.var_fcdw_ditf) + (assign6240_e6557 * locals.var_fcdw_ditf_dn9)),)
    } else {
        (locals.var_fcdfc_ditf, locals.var_fcdfc_ditf_dn0, locals.var_fcdfc_ditf_dn1, locals.var_fcdfc_ditf_dn3, locals.var_fcdfc_ditf_dn4, locals.var_fcdfc_ditf_dn5, locals.var_fcdfc_ditf_dn6, locals.var_fcdfc_ditf_dn7, locals.var_fcdfc_ditf_dn8, locals.var_fcdfc_ditf_dn9,)
    }
};
        locals.var_fcdfc_ditf = assign6240_e6561;
        locals.var_fcdfc_ditf_dn0 = assign6240_e6561_d_n0;
        locals.var_fcdfc_ditf_dn1 = assign6240_e6561_d_n1;
        locals.var_fcdfc_ditf_dn3 = assign6240_e6561_d_n3;
        locals.var_fcdfc_ditf_dn4 = assign6240_e6561_d_n4;
        locals.var_fcdfc_ditf_dn5 = assign6240_e6561_d_n5;
        locals.var_fcdfc_ditf_dn6 = assign6240_e6561_d_n6;
        locals.var_fcdfc_ditf_dn7 = assign6240_e6561_d_n7;
        locals.var_fcdfc_ditf_dn8 = assign6240_e6561_d_n8;
        locals.var_fcdfc_ditf_dn9 = assign6240_e6561_d_n9;

        let (assign6250_e6572, assign6250_e6572_d_n0, assign6250_e6572_d_n1, assign6250_e6572_d_n3, assign6250_e6572_d_n4, assign6250_e6572_d_n5, assign6250_e6572_d_n6, assign6250_e6572_d_n7, assign6250_e6572_d_n8, assign6250_e6572_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign6250_e6568: f64 = (p.p73 * locals.var_thcs_t);
        let assign6250_e6570: f64 = (assign6250_e6568 * locals.var_ffvc_exp);
        (assign6250_e6570, (assign6250_e6568 * locals.var_ffvc_exp_dn0), (assign6250_e6568 * locals.var_ffvc_exp_dn1), (assign6250_e6568 * locals.var_ffvc_exp_dn3), (((p.p73 * locals.var_thcs_t_dn4) * locals.var_ffvc_exp) + (assign6250_e6568 * locals.var_ffvc_exp_dn4)), (assign6250_e6568 * locals.var_ffvc_exp_dn5), (assign6250_e6568 * locals.var_ffvc_exp_dn6), (assign6250_e6568 * locals.var_ffvc_exp_dn7), (assign6250_e6568 * locals.var_ffvc_exp_dn8), (assign6250_e6568 * locals.var_ffvc_exp_dn9),)
    } else {
        (locals.var_dum_a, locals.var_dum_a_dn0, locals.var_dum_a_dn1, locals.var_dum_a_dn3, locals.var_dum_a_dn4, locals.var_dum_a_dn5, locals.var_dum_a_dn6, locals.var_dum_a_dn7, locals.var_dum_a_dn8, locals.var_dum_a_dn9,)
    }
};
        locals.var_dum_a = assign6250_e6572;
        locals.var_dum_a_dn0 = assign6250_e6572_d_n0;
        locals.var_dum_a_dn1 = assign6250_e6572_d_n1;
        locals.var_dum_a_dn3 = assign6250_e6572_d_n3;
        locals.var_dum_a_dn4 = assign6250_e6572_d_n4;
        locals.var_dum_a_dn5 = assign6250_e6572_d_n5;
        locals.var_dum_a_dn6 = assign6250_e6572_d_n6;
        locals.var_dum_a_dn7 = assign6250_e6572_d_n7;
        locals.var_dum_a_dn8 = assign6250_e6572_d_n8;
        locals.var_dum_a_dn9 = assign6250_e6572_d_n9;

        let (assign6260_e6581, assign6260_e6581_d_n0, assign6260_e6581_d_n1, assign6260_e6581_d_n3, assign6260_e6581_d_n4, assign6260_e6581_d_n5, assign6260_e6581_d_n6, assign6260_e6581_d_n7, assign6260_e6581_d_n8, assign6260_e6581_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign6260_e6579: f64 = (locals.var_dum_a * locals.var_fcf_ci);
        (assign6260_e6579, ((locals.var_dum_a_dn0 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn0)), ((locals.var_dum_a_dn1 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn1)), ((locals.var_dum_a_dn3 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn3)), ((locals.var_dum_a_dn4 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn4)), ((locals.var_dum_a_dn5 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn5)), ((locals.var_dum_a_dn6 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn6)), ((locals.var_dum_a_dn7 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn7)), ((locals.var_dum_a_dn8 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn8)), ((locals.var_dum_a_dn9 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn9)),)
    } else {
        (locals.var_dum_b, locals.var_dum_b_dn0, locals.var_dum_b_dn1, locals.var_dum_b_dn3, locals.var_dum_b_dn4, locals.var_dum_b_dn5, locals.var_dum_b_dn6, locals.var_dum_b_dn7, locals.var_dum_b_dn8, locals.var_dum_b_dn9,)
    }
};
        locals.var_dum_b = assign6260_e6581;
        locals.var_dum_b_dn0 = assign6260_e6581_d_n0;
        locals.var_dum_b_dn1 = assign6260_e6581_d_n1;
        locals.var_dum_b_dn3 = assign6260_e6581_d_n3;
        locals.var_dum_b_dn4 = assign6260_e6581_d_n4;
        locals.var_dum_b_dn5 = assign6260_e6581_d_n5;
        locals.var_dum_b_dn6 = assign6260_e6581_d_n6;
        locals.var_dum_b_dn7 = assign6260_e6581_d_n7;
        locals.var_dum_b_dn8 = assign6260_e6581_d_n8;
        locals.var_dum_b_dn9 = assign6260_e6581_d_n9;

        let (assign6270_e6590, assign6270_e6590_d_n0, assign6270_e6590_d_n1, assign6270_e6590_d_n3, assign6270_e6590_d_n4, assign6270_e6590_d_n5, assign6270_e6590_d_n6, assign6270_e6590_d_n7, assign6270_e6590_d_n8, assign6270_e6590_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign6270_e6588: f64 = (locals.var_dum_b * locals.var_itf);
        (assign6270_e6588, ((locals.var_dum_b_dn0 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn0)), ((locals.var_dum_b_dn1 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn1)), ((locals.var_dum_b_dn3 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn3)), ((locals.var_dum_b_dn4 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn4)), ((locals.var_dum_b_dn5 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn5)), ((locals.var_dum_b_dn6 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn6)), ((locals.var_dum_b_dn7 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn7)), ((locals.var_dum_b_dn8 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn8)), ((locals.var_dum_b_dn9 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn9)),)
    } else {
        (locals.var_ffdqcfc, locals.var_ffdqcfc_dn0, locals.var_ffdqcfc_dn1, locals.var_ffdqcfc_dn3, locals.var_ffdqcfc_dn4, locals.var_ffdqcfc_dn5, locals.var_ffdqcfc_dn6, locals.var_ffdqcfc_dn7, locals.var_ffdqcfc_dn8, locals.var_ffdqcfc_dn9,)
    }
};
        locals.var_ffdqcfc = assign6270_e6590;
        locals.var_ffdqcfc_dn0 = assign6270_e6590_d_n0;
        locals.var_ffdqcfc_dn1 = assign6270_e6590_d_n1;
        locals.var_ffdqcfc_dn3 = assign6270_e6590_d_n3;
        locals.var_ffdqcfc_dn4 = assign6270_e6590_d_n4;
        locals.var_ffdqcfc_dn5 = assign6270_e6590_d_n5;
        locals.var_ffdqcfc_dn6 = assign6270_e6590_d_n6;
        locals.var_ffdqcfc_dn7 = assign6270_e6590_d_n7;
        locals.var_ffdqcfc_dn8 = assign6270_e6590_d_n8;
        locals.var_ffdqcfc_dn9 = assign6270_e6590_d_n9;

        let (assign6280_e6609, assign6280_e6609_d_n0, assign6280_e6609_d_n1, assign6280_e6609_d_n3, assign6280_e6609_d_n4, assign6280_e6609_d_n5, assign6280_e6609_d_n6, assign6280_e6609_d_n7, assign6280_e6609_d_n8, assign6280_e6609_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign6280_e6598: f64 = (locals.var_ffdqcfc * locals.var_ffdvc_ditf);
        let assign6280_e6600: f64 = (assign6280_e6598 * locals.var_ovt);
        let assign6280_e6601: f64 = (locals.var_dum_b + assign6280_e6600);
        let assign6280_e6604: f64 = (locals.var_dum_a * locals.var_itf);
        let assign6280_e6606: f64 = (assign6280_e6604 * locals.var_fcdfc_ditf);
        let assign6280_e6607: f64 = (assign6280_e6601 + assign6280_e6606);
        (assign6280_e6607, ((locals.var_dum_b_dn0 + (((locals.var_ffdqcfc_dn0 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn0)) * locals.var_ovt)) + ((((locals.var_dum_a_dn0 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn0)) * locals.var_fcdfc_ditf) + (assign6280_e6604 * locals.var_fcdfc_ditf_dn0))), ((locals.var_dum_b_dn1 + (((locals.var_ffdqcfc_dn1 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn1)) * locals.var_ovt)) + ((((locals.var_dum_a_dn1 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn1)) * locals.var_fcdfc_ditf) + (assign6280_e6604 * locals.var_fcdfc_ditf_dn1))), ((locals.var_dum_b_dn3 + (((locals.var_ffdqcfc_dn3 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn3)) * locals.var_ovt)) + ((((locals.var_dum_a_dn3 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn3)) * locals.var_fcdfc_ditf) + (assign6280_e6604 * locals.var_fcdfc_ditf_dn3))), ((locals.var_dum_b_dn4 + ((((locals.var_ffdqcfc_dn4 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn4)) * locals.var_ovt) + (assign6280_e6598 * locals.var_ovt_dn4))) + ((((locals.var_dum_a_dn4 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn4)) * locals.var_fcdfc_ditf) + (assign6280_e6604 * locals.var_fcdfc_ditf_dn4))), ((locals.var_dum_b_dn5 + (((locals.var_ffdqcfc_dn5 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn5)) * locals.var_ovt)) + ((((locals.var_dum_a_dn5 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn5)) * locals.var_fcdfc_ditf) + (assign6280_e6604 * locals.var_fcdfc_ditf_dn5))), ((locals.var_dum_b_dn6 + (((locals.var_ffdqcfc_dn6 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn6)) * locals.var_ovt)) + ((((locals.var_dum_a_dn6 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn6)) * locals.var_fcdfc_ditf) + (assign6280_e6604 * locals.var_fcdfc_ditf_dn6))), ((locals.var_dum_b_dn7 + (((locals.var_ffdqcfc_dn7 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn7)) * locals.var_ovt)) + ((((locals.var_dum_a_dn7 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn7)) * locals.var_fcdfc_ditf) + (assign6280_e6604 * locals.var_fcdfc_ditf_dn7))), ((locals.var_dum_b_dn8 + (((locals.var_ffdqcfc_dn8 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn8)) * locals.var_ovt)) + ((((locals.var_dum_a_dn8 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn8)) * locals.var_fcdfc_ditf) + (assign6280_e6604 * locals.var_fcdfc_ditf_dn8))), ((locals.var_dum_b_dn9 + (((locals.var_ffdqcfc_dn9 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn9)) * locals.var_ovt)) + ((((locals.var_dum_a_dn9 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn9)) * locals.var_fcdfc_ditf) + (assign6280_e6604 * locals.var_fcdfc_ditf_dn9))),)
    } else {
        (locals.var_ffdtcfc, locals.var_ffdtcfc_dn0, locals.var_ffdtcfc_dn1, locals.var_ffdtcfc_dn3, locals.var_ffdtcfc_dn4, locals.var_ffdtcfc_dn5, locals.var_ffdtcfc_dn6, locals.var_ffdtcfc_dn7, locals.var_ffdtcfc_dn8, locals.var_ffdtcfc_dn9,)
    }
};
        locals.var_ffdtcfc = assign6280_e6609;
        locals.var_ffdtcfc_dn0 = assign6280_e6609_d_n0;
        locals.var_ffdtcfc_dn1 = assign6280_e6609_d_n1;
        locals.var_ffdtcfc_dn3 = assign6280_e6609_d_n3;
        locals.var_ffdtcfc_dn4 = assign6280_e6609_d_n4;
        locals.var_ffdtcfc_dn5 = assign6280_e6609_d_n5;
        locals.var_ffdtcfc_dn6 = assign6280_e6609_d_n6;
        locals.var_ffdtcfc_dn7 = assign6280_e6609_d_n7;
        locals.var_ffdtcfc_dn8 = assign6280_e6609_d_n8;
        locals.var_ffdtcfc_dn9 = assign6280_e6609_d_n9;

        let (assign6290_e6619, assign6290_e6619_d_n0, assign6290_e6619_d_n1, assign6290_e6619_d_n3, assign6290_e6619_d_n4, assign6290_e6619_d_n5, assign6290_e6619_d_n6, assign6290_e6619_d_n7, assign6290_e6619_d_n8, assign6290_e6619_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign6290_e6613: f64 = (1.0 - p.p73);
        let assign6290_e6615: f64 = (assign6290_e6613 * locals.var_ffdqfhc);
        let assign6290_e6617: f64 = (assign6290_e6615 * locals.var_itf);
        (assign6290_e6617, (((assign6290_e6613 * locals.var_ffdqfhc_dn0) * locals.var_itf) + (assign6290_e6615 * locals.var_itf_dn0)), (((assign6290_e6613 * locals.var_ffdqfhc_dn1) * locals.var_itf) + (assign6290_e6615 * locals.var_itf_dn1)), (((assign6290_e6613 * locals.var_ffdqfhc_dn3) * locals.var_itf) + (assign6290_e6615 * locals.var_itf_dn3)), (((assign6290_e6613 * locals.var_ffdqfhc_dn4) * locals.var_itf) + (assign6290_e6615 * locals.var_itf_dn4)), (((assign6290_e6613 * locals.var_ffdqfhc_dn5) * locals.var_itf) + (assign6290_e6615 * locals.var_itf_dn5)), (((assign6290_e6613 * locals.var_ffdqfhc_dn6) * locals.var_itf) + (assign6290_e6615 * locals.var_itf_dn6)), (((assign6290_e6613 * locals.var_ffdqfhc_dn7) * locals.var_itf) + (assign6290_e6615 * locals.var_itf_dn7)), (((assign6290_e6613 * locals.var_ffdqfhc_dn8) * locals.var_itf) + (assign6290_e6615 * locals.var_itf_dn8)), (((assign6290_e6613 * locals.var_ffdqfhc_dn9) * locals.var_itf) + (assign6290_e6615 * locals.var_itf_dn9)),)
    } else {
        (locals.var_ffdqbfc, locals.var_ffdqbfc_dn0, locals.var_ffdqbfc_dn1, locals.var_ffdqbfc_dn3, locals.var_ffdqbfc_dn4, locals.var_ffdqbfc_dn5, locals.var_ffdqbfc_dn6, locals.var_ffdqbfc_dn7, locals.var_ffdqbfc_dn8, locals.var_ffdqbfc_dn9,)
    }
};
        locals.var_ffdqbfc = assign6290_e6619;
        locals.var_ffdqbfc_dn0 = assign6290_e6619_d_n0;
        locals.var_ffdqbfc_dn1 = assign6290_e6619_d_n1;
        locals.var_ffdqbfc_dn3 = assign6290_e6619_d_n3;
        locals.var_ffdqbfc_dn4 = assign6290_e6619_d_n4;
        locals.var_ffdqbfc_dn5 = assign6290_e6619_d_n5;
        locals.var_ffdqbfc_dn6 = assign6290_e6619_d_n6;
        locals.var_ffdqbfc_dn7 = assign6290_e6619_d_n7;
        locals.var_ffdqbfc_dn8 = assign6290_e6619_d_n8;
        locals.var_ffdqbfc_dn9 = assign6290_e6619_d_n9;

        let (assign6300_e6627, assign6300_e6627_d_n0, assign6300_e6627_d_n1, assign6300_e6627_d_n3, assign6300_e6627_d_n4, assign6300_e6627_d_n5, assign6300_e6627_d_n6, assign6300_e6627_d_n7, assign6300_e6627_d_n8, assign6300_e6627_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign6300_e6623: f64 = (1.0 - p.p73);
        let assign6300_e6625: f64 = (assign6300_e6623 * locals.var_ffdtfhc);
        (assign6300_e6625, (assign6300_e6623 * locals.var_ffdtfhc_dn0), (assign6300_e6623 * locals.var_ffdtfhc_dn1), (assign6300_e6623 * locals.var_ffdtfhc_dn3), (assign6300_e6623 * locals.var_ffdtfhc_dn4), (assign6300_e6623 * locals.var_ffdtfhc_dn5), (assign6300_e6623 * locals.var_ffdtfhc_dn6), (assign6300_e6623 * locals.var_ffdtfhc_dn7), (assign6300_e6623 * locals.var_ffdtfhc_dn8), (assign6300_e6623 * locals.var_ffdtfhc_dn9),)
    } else {
        (locals.var_ffdtbfc, locals.var_ffdtbfc_dn0, locals.var_ffdtbfc_dn1, locals.var_ffdtbfc_dn3, locals.var_ffdtbfc_dn4, locals.var_ffdtbfc_dn5, locals.var_ffdtbfc_dn6, locals.var_ffdtbfc_dn7, locals.var_ffdtbfc_dn8, locals.var_ffdtbfc_dn9,)
    }
};
        locals.var_ffdtbfc = assign6300_e6627;
        locals.var_ffdtbfc_dn0 = assign6300_e6627_d_n0;
        locals.var_ffdtbfc_dn1 = assign6300_e6627_d_n1;
        locals.var_ffdtbfc_dn3 = assign6300_e6627_d_n3;
        locals.var_ffdtbfc_dn4 = assign6300_e6627_d_n4;
        locals.var_ffdtbfc_dn5 = assign6300_e6627_d_n5;
        locals.var_ffdtbfc_dn6 = assign6300_e6627_d_n6;
        locals.var_ffdtbfc_dn7 = assign6300_e6627_d_n7;
        locals.var_ffdtbfc_dn8 = assign6300_e6627_d_n8;
        locals.var_ffdtbfc_dn9 = assign6300_e6627_d_n9;

        let (assign6310_e6635, assign6310_e6635_d_n0, assign6310_e6635_d_n1, assign6310_e6635_d_n3, assign6310_e6635_d_n4, assign6310_e6635_d_n5, assign6310_e6635_d_n6, assign6310_e6635_d_n7, assign6310_e6635_d_n8, assign6310_e6635_d_n9,) = {
    if (locals.var_guard124 != 0.0) {
        let assign6310_e6631: f64 = (locals.var_ffdqbfb * locals.var_itf);
        let assign6310_e6633: f64 = (assign6310_e6631 + locals.var_ffdqbfc);
        (assign6310_e6633, (((locals.var_ffdqbfb_dn0 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn0)) + locals.var_ffdqbfc_dn0), (((locals.var_ffdqbfb_dn1 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn1)) + locals.var_ffdqbfc_dn1), (((locals.var_ffdqbfb_dn3 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn3)) + locals.var_ffdqbfc_dn3), (((locals.var_ffdqbfb_dn4 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn4)) + locals.var_ffdqbfc_dn4), (((locals.var_ffdqbfb_dn5 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn5)) + locals.var_ffdqbfc_dn5), (((locals.var_ffdqbfb_dn6 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn6)) + locals.var_ffdqbfc_dn6), (((locals.var_ffdqbfb_dn7 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn7)) + locals.var_ffdqbfc_dn7), (((locals.var_ffdqbfb_dn8 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn8)) + locals.var_ffdqbfc_dn8), (((locals.var_ffdqbfb_dn9 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn9)) + locals.var_ffdqbfc_dn9),)
    } else {
        (locals.var_q_bf, locals.var_q_bf_dn0, locals.var_q_bf_dn1, locals.var_q_bf_dn3, locals.var_q_bf_dn4, locals.var_q_bf_dn5, locals.var_q_bf_dn6, locals.var_q_bf_dn7, locals.var_q_bf_dn8, locals.var_q_bf_dn9,)
    }
};
        locals.var_q_bf = assign6310_e6635;
        locals.var_q_bf_dn0 = assign6310_e6635_d_n0;
        locals.var_q_bf_dn1 = assign6310_e6635_d_n1;
        locals.var_q_bf_dn3 = assign6310_e6635_d_n3;
        locals.var_q_bf_dn4 = assign6310_e6635_d_n4;
        locals.var_q_bf_dn5 = assign6310_e6635_d_n5;
        locals.var_q_bf_dn6 = assign6310_e6635_d_n6;
        locals.var_q_bf_dn7 = assign6310_e6635_d_n7;
        locals.var_q_bf_dn8 = assign6310_e6635_d_n8;
        locals.var_q_bf_dn9 = assign6310_e6635_d_n9;

        let assign6320_e6638: f64 = if p.p0 >= 310.0 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign6320_e6638;

        let (assign6330_e6650, assign6330_e6650_d_n0, assign6330_e6650_d_n1, assign6330_e6650_d_n3, assign6330_e6650_d_n4, assign6330_e6650_d_n5, assign6330_e6650_d_n6, assign6330_e6650_d_n7, assign6330_e6650_d_n8, assign6330_e6650_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard130 != 0.0)) {
        let assign6330_e6644: f64 = (locals.var_qf + locals.var_q_bf);
        let assign6330_e6646: f64 = (assign6330_e6644 + locals.var_ffdqef);
        let assign6330_e6648: f64 = (assign6330_e6646 + locals.var_ffdqcfc);
        (assign6330_e6648, (((locals.var_qf_dn0 + locals.var_q_bf_dn0) + locals.var_ffdqef_dn0) + locals.var_ffdqcfc_dn0), (((locals.var_qf_dn1 + locals.var_q_bf_dn1) + locals.var_ffdqef_dn1) + locals.var_ffdqcfc_dn1), (((locals.var_qf_dn3 + locals.var_q_bf_dn3) + locals.var_ffdqef_dn3) + locals.var_ffdqcfc_dn3), (((locals.var_qf_dn4 + locals.var_q_bf_dn4) + locals.var_ffdqef_dn4) + locals.var_ffdqcfc_dn4), (((locals.var_qf_dn5 + locals.var_q_bf_dn5) + locals.var_ffdqef_dn5) + locals.var_ffdqcfc_dn5), (((locals.var_qf_dn6 + locals.var_q_bf_dn6) + locals.var_ffdqef_dn6) + locals.var_ffdqcfc_dn6), (((locals.var_qf_dn7 + locals.var_q_bf_dn7) + locals.var_ffdqef_dn7) + locals.var_ffdqcfc_dn7), (((locals.var_qf_dn8 + locals.var_q_bf_dn8) + locals.var_ffdqef_dn8) + locals.var_ffdqcfc_dn8), (((locals.var_qf_dn9 + locals.var_q_bf_dn9) + locals.var_ffdqef_dn9) + locals.var_ffdqcfc_dn9),)
    } else {
        (locals.var_qf, locals.var_qf_dn0, locals.var_qf_dn1, locals.var_qf_dn3, locals.var_qf_dn4, locals.var_qf_dn5, locals.var_qf_dn6, locals.var_qf_dn7, locals.var_qf_dn8, locals.var_qf_dn9,)
    }
};
        locals.var_qf = assign6330_e6650;
        locals.var_qf_dn0 = assign6330_e6650_d_n0;
        locals.var_qf_dn1 = assign6330_e6650_d_n1;
        locals.var_qf_dn3 = assign6330_e6650_d_n3;
        locals.var_qf_dn4 = assign6330_e6650_d_n4;
        locals.var_qf_dn5 = assign6330_e6650_d_n5;
        locals.var_qf_dn6 = assign6330_e6650_d_n6;
        locals.var_qf_dn7 = assign6330_e6650_d_n7;
        locals.var_qf_dn8 = assign6330_e6650_d_n8;
        locals.var_qf_dn9 = assign6330_e6650_d_n9;

        let (assign6340_e6664, assign6340_e6664_d_n0, assign6340_e6664_d_n1, assign6340_e6664_d_n3, assign6340_e6664_d_n4, assign6340_e6664_d_n5, assign6340_e6664_d_n6, assign6340_e6664_d_n7, assign6340_e6664_d_n8, assign6340_e6664_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard130 != 0.0)) {
        let assign6340_e6657: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign6340_e6658: f64 = (locals.var_tf + assign6340_e6657);
        let assign6340_e6660: f64 = (assign6340_e6658 + locals.var_ffdtef);
        let assign6340_e6662: f64 = (assign6340_e6660 + locals.var_ffdtcfc);
        (assign6340_e6662, (((locals.var_tf_dn0 + (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0)) + locals.var_ffdtef_dn0) + locals.var_ffdtcfc_dn0), (((locals.var_tf_dn1 + (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1)) + locals.var_ffdtef_dn1) + locals.var_ffdtcfc_dn1), (((locals.var_tf_dn3 + (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3)) + locals.var_ffdtef_dn3) + locals.var_ffdtcfc_dn3), (((locals.var_tf_dn4 + (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4)) + locals.var_ffdtef_dn4) + locals.var_ffdtcfc_dn4), (((locals.var_tf_dn5 + (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5)) + locals.var_ffdtef_dn5) + locals.var_ffdtcfc_dn5), (((locals.var_tf_dn6 + (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6)) + locals.var_ffdtef_dn6) + locals.var_ffdtcfc_dn6), (((locals.var_tf_dn7 + (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7)) + locals.var_ffdtef_dn7) + locals.var_ffdtcfc_dn7), (((locals.var_tf_dn8 + (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8)) + locals.var_ffdtef_dn8) + locals.var_ffdtcfc_dn8), (((locals.var_tf_dn9 + (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9)) + locals.var_ffdtef_dn9) + locals.var_ffdtcfc_dn9),)
    } else {
        (locals.var_tf, locals.var_tf_dn0, locals.var_tf_dn1, locals.var_tf_dn3, locals.var_tf_dn4, locals.var_tf_dn5, locals.var_tf_dn6, locals.var_tf_dn7, locals.var_tf_dn8, locals.var_tf_dn9,)
    }
};
        locals.var_tf = assign6340_e6664;
        locals.var_tf_dn0 = assign6340_e6664_d_n0;
        locals.var_tf_dn1 = assign6340_e6664_d_n1;
        locals.var_tf_dn3 = assign6340_e6664_d_n3;
        locals.var_tf_dn4 = assign6340_e6664_d_n4;
        locals.var_tf_dn5 = assign6340_e6664_d_n5;
        locals.var_tf_dn6 = assign6340_e6664_d_n6;
        locals.var_tf_dn7 = assign6340_e6664_d_n7;
        locals.var_tf_dn8 = assign6340_e6664_d_n8;
        locals.var_tf_dn9 = assign6340_e6664_d_n9;

        let (assign6350_e6682, assign6350_e6682_d_n0, assign6350_e6682_d_n1, assign6350_e6682_d_n3, assign6350_e6682_d_n4, assign6350_e6682_d_n5, assign6350_e6682_d_n6, assign6350_e6682_d_n7, assign6350_e6682_d_n8, assign6350_e6682_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard130 != 0.0)) {
        let assign6350_e6671: f64 = (p.p5 * locals.var_q_bf);
        let assign6350_e6672: f64 = (locals.var_q_ft + assign6350_e6671);
        let assign6350_e6675: f64 = (locals.var_hfe_t * locals.var_ffdqef);
        let assign6350_e6676: f64 = (assign6350_e6672 + assign6350_e6675);
        let assign6350_e6679: f64 = (locals.var_hfc_t * locals.var_ffdqcfc);
        let assign6350_e6680: f64 = (assign6350_e6676 + assign6350_e6679);
        (assign6350_e6680, (((locals.var_q_ft_dn0 + (p.p5 * locals.var_q_bf_dn0)) + (locals.var_hfe_t * locals.var_ffdqef_dn0)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn0)), (((locals.var_q_ft_dn1 + (p.p5 * locals.var_q_bf_dn1)) + (locals.var_hfe_t * locals.var_ffdqef_dn1)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn1)), (((locals.var_q_ft_dn3 + (p.p5 * locals.var_q_bf_dn3)) + (locals.var_hfe_t * locals.var_ffdqef_dn3)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn3)), (((locals.var_q_ft_dn4 + (p.p5 * locals.var_q_bf_dn4)) + ((locals.var_hfe_t_dn4 * locals.var_ffdqef) + (locals.var_hfe_t * locals.var_ffdqef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdqcfc) + (locals.var_hfc_t * locals.var_ffdqcfc_dn4))), (((locals.var_q_ft_dn5 + (p.p5 * locals.var_q_bf_dn5)) + (locals.var_hfe_t * locals.var_ffdqef_dn5)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn5)), (((locals.var_q_ft_dn6 + (p.p5 * locals.var_q_bf_dn6)) + (locals.var_hfe_t * locals.var_ffdqef_dn6)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn6)), (((locals.var_q_ft_dn7 + (p.p5 * locals.var_q_bf_dn7)) + (locals.var_hfe_t * locals.var_ffdqef_dn7)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn7)), (((locals.var_q_ft_dn8 + (p.p5 * locals.var_q_bf_dn8)) + (locals.var_hfe_t * locals.var_ffdqef_dn8)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn8)), (((locals.var_q_ft_dn9 + (p.p5 * locals.var_q_bf_dn9)) + (locals.var_hfe_t * locals.var_ffdqef_dn9)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn9)),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
        locals.var_q_ft = assign6350_e6682;
        locals.var_q_ft_dn0 = assign6350_e6682_d_n0;
        locals.var_q_ft_dn1 = assign6350_e6682_d_n1;
        locals.var_q_ft_dn3 = assign6350_e6682_d_n3;
        locals.var_q_ft_dn4 = assign6350_e6682_d_n4;
        locals.var_q_ft_dn5 = assign6350_e6682_d_n5;
        locals.var_q_ft_dn6 = assign6350_e6682_d_n6;
        locals.var_q_ft_dn7 = assign6350_e6682_d_n7;
        locals.var_q_ft_dn8 = assign6350_e6682_d_n8;
        locals.var_q_ft_dn9 = assign6350_e6682_d_n9;

        let (assign6360_e6702, assign6360_e6702_d_n0, assign6360_e6702_d_n1, assign6360_e6702_d_n3, assign6360_e6702_d_n4, assign6360_e6702_d_n5, assign6360_e6702_d_n6, assign6360_e6702_d_n7, assign6360_e6702_d_n8, assign6360_e6702_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard130 != 0.0)) {
        let assign6360_e6690: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign6360_e6691: f64 = (p.p5 * assign6360_e6690);
        let assign6360_e6692: f64 = (locals.var_t_ft + assign6360_e6691);
        let assign6360_e6695: f64 = (locals.var_hfe_t * locals.var_ffdtef);
        let assign6360_e6696: f64 = (assign6360_e6692 + assign6360_e6695);
        let assign6360_e6699: f64 = (locals.var_hfc_t * locals.var_ffdtcfc);
        let assign6360_e6700: f64 = (assign6360_e6696 + assign6360_e6699);
        (assign6360_e6700, (((locals.var_t_ft_dn0 + (p.p5 * (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0))) + (locals.var_hfe_t * locals.var_ffdtef_dn0)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn0)), (((locals.var_t_ft_dn1 + (p.p5 * (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1))) + (locals.var_hfe_t * locals.var_ffdtef_dn1)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn1)), (((locals.var_t_ft_dn3 + (p.p5 * (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3))) + (locals.var_hfe_t * locals.var_ffdtef_dn3)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn3)), (((locals.var_t_ft_dn4 + (p.p5 * (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4))) + ((locals.var_hfe_t_dn4 * locals.var_ffdtef) + (locals.var_hfe_t * locals.var_ffdtef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdtcfc) + (locals.var_hfc_t * locals.var_ffdtcfc_dn4))), (((locals.var_t_ft_dn5 + (p.p5 * (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5))) + (locals.var_hfe_t * locals.var_ffdtef_dn5)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn5)), (((locals.var_t_ft_dn6 + (p.p5 * (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6))) + (locals.var_hfe_t * locals.var_ffdtef_dn6)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn6)), (((locals.var_t_ft_dn7 + (p.p5 * (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7))) + (locals.var_hfe_t * locals.var_ffdtef_dn7)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn7)), (((locals.var_t_ft_dn8 + (p.p5 * (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8))) + (locals.var_hfe_t * locals.var_ffdtef_dn8)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn8)), (((locals.var_t_ft_dn9 + (p.p5 * (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9))) + (locals.var_hfe_t * locals.var_ffdtef_dn9)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn9)),)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
        locals.var_t_ft = assign6360_e6702;
        locals.var_t_ft_dn0 = assign6360_e6702_d_n0;
        locals.var_t_ft_dn1 = assign6360_e6702_d_n1;
        locals.var_t_ft_dn3 = assign6360_e6702_d_n3;
        locals.var_t_ft_dn4 = assign6360_e6702_d_n4;
        locals.var_t_ft_dn5 = assign6360_e6702_d_n5;
        locals.var_t_ft_dn6 = assign6360_e6702_d_n6;
        locals.var_t_ft_dn7 = assign6360_e6702_d_n7;
        locals.var_t_ft_dn8 = assign6360_e6702_d_n8;
        locals.var_t_ft_dn9 = assign6360_e6702_d_n9;

        let (assign6370_e6721, assign6370_e6721_d_n0, assign6370_e6721_d_n1, assign6370_e6721_d_n3, assign6370_e6721_d_n4, assign6370_e6721_d_n5, assign6370_e6721_d_n6, assign6370_e6721_d_n7, assign6370_e6721_d_n8, assign6370_e6721_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard130 == 0.0)) {
        let assign6370_e6709: f64 = (locals.var_hf0_t * locals.var_qf);
        let assign6370_e6711: f64 = (assign6370_e6709 + locals.var_q_bf);
        let assign6370_e6714: f64 = (locals.var_hfe_t * locals.var_ffdqef);
        let assign6370_e6715: f64 = (assign6370_e6711 + assign6370_e6714);
        let assign6370_e6718: f64 = (locals.var_hfc_t * locals.var_ffdqcfc);
        let assign6370_e6719: f64 = (assign6370_e6715 + assign6370_e6718);
        (assign6370_e6719, ((((locals.var_hf0_t * locals.var_qf_dn0) + locals.var_q_bf_dn0) + (locals.var_hfe_t * locals.var_ffdqef_dn0)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn0)), ((((locals.var_hf0_t * locals.var_qf_dn1) + locals.var_q_bf_dn1) + (locals.var_hfe_t * locals.var_ffdqef_dn1)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn1)), ((((locals.var_hf0_t * locals.var_qf_dn3) + locals.var_q_bf_dn3) + (locals.var_hfe_t * locals.var_ffdqef_dn3)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn3)), (((((locals.var_hf0_t_dn4 * locals.var_qf) + (locals.var_hf0_t * locals.var_qf_dn4)) + locals.var_q_bf_dn4) + ((locals.var_hfe_t_dn4 * locals.var_ffdqef) + (locals.var_hfe_t * locals.var_ffdqef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdqcfc) + (locals.var_hfc_t * locals.var_ffdqcfc_dn4))), ((((locals.var_hf0_t * locals.var_qf_dn5) + locals.var_q_bf_dn5) + (locals.var_hfe_t * locals.var_ffdqef_dn5)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn5)), ((((locals.var_hf0_t * locals.var_qf_dn6) + locals.var_q_bf_dn6) + (locals.var_hfe_t * locals.var_ffdqef_dn6)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn6)), ((((locals.var_hf0_t * locals.var_qf_dn7) + locals.var_q_bf_dn7) + (locals.var_hfe_t * locals.var_ffdqef_dn7)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn7)), ((((locals.var_hf0_t * locals.var_qf_dn8) + locals.var_q_bf_dn8) + (locals.var_hfe_t * locals.var_ffdqef_dn8)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn8)), ((((locals.var_hf0_t * locals.var_qf_dn9) + locals.var_q_bf_dn9) + (locals.var_hfe_t * locals.var_ffdqef_dn9)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn9)),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
        locals.var_q_ft = assign6370_e6721;
        locals.var_q_ft_dn0 = assign6370_e6721_d_n0;
        locals.var_q_ft_dn1 = assign6370_e6721_d_n1;
        locals.var_q_ft_dn3 = assign6370_e6721_d_n3;
        locals.var_q_ft_dn4 = assign6370_e6721_d_n4;
        locals.var_q_ft_dn5 = assign6370_e6721_d_n5;
        locals.var_q_ft_dn6 = assign6370_e6721_d_n6;
        locals.var_q_ft_dn7 = assign6370_e6721_d_n7;
        locals.var_q_ft_dn8 = assign6370_e6721_d_n8;
        locals.var_q_ft_dn9 = assign6370_e6721_d_n9;

        let (assign6380_e6734, assign6380_e6734_d_n0, assign6380_e6734_d_n1, assign6380_e6734_d_n3, assign6380_e6734_d_n4, assign6380_e6734_d_n5, assign6380_e6734_d_n6, assign6380_e6734_d_n7, assign6380_e6734_d_n8, assign6380_e6734_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard130 == 0.0)) {
        let assign6380_e6728: f64 = (locals.var_qf + locals.var_q_bf);
        let assign6380_e6730: f64 = (assign6380_e6728 + locals.var_ffdqef);
        let assign6380_e6732: f64 = (assign6380_e6730 + locals.var_ffdqcfc);
        (assign6380_e6732, (((locals.var_qf_dn0 + locals.var_q_bf_dn0) + locals.var_ffdqef_dn0) + locals.var_ffdqcfc_dn0), (((locals.var_qf_dn1 + locals.var_q_bf_dn1) + locals.var_ffdqef_dn1) + locals.var_ffdqcfc_dn1), (((locals.var_qf_dn3 + locals.var_q_bf_dn3) + locals.var_ffdqef_dn3) + locals.var_ffdqcfc_dn3), (((locals.var_qf_dn4 + locals.var_q_bf_dn4) + locals.var_ffdqef_dn4) + locals.var_ffdqcfc_dn4), (((locals.var_qf_dn5 + locals.var_q_bf_dn5) + locals.var_ffdqef_dn5) + locals.var_ffdqcfc_dn5), (((locals.var_qf_dn6 + locals.var_q_bf_dn6) + locals.var_ffdqef_dn6) + locals.var_ffdqcfc_dn6), (((locals.var_qf_dn7 + locals.var_q_bf_dn7) + locals.var_ffdqef_dn7) + locals.var_ffdqcfc_dn7), (((locals.var_qf_dn8 + locals.var_q_bf_dn8) + locals.var_ffdqef_dn8) + locals.var_ffdqcfc_dn8), (((locals.var_qf_dn9 + locals.var_q_bf_dn9) + locals.var_ffdqef_dn9) + locals.var_ffdqcfc_dn9),)
    } else {
        (locals.var_qf, locals.var_qf_dn0, locals.var_qf_dn1, locals.var_qf_dn3, locals.var_qf_dn4, locals.var_qf_dn5, locals.var_qf_dn6, locals.var_qf_dn7, locals.var_qf_dn8, locals.var_qf_dn9,)
    }
};
        locals.var_qf = assign6380_e6734;
        locals.var_qf_dn0 = assign6380_e6734_d_n0;
        locals.var_qf_dn1 = assign6380_e6734_d_n1;
        locals.var_qf_dn3 = assign6380_e6734_d_n3;
        locals.var_qf_dn4 = assign6380_e6734_d_n4;
        locals.var_qf_dn5 = assign6380_e6734_d_n5;
        locals.var_qf_dn6 = assign6380_e6734_d_n6;
        locals.var_qf_dn7 = assign6380_e6734_d_n7;
        locals.var_qf_dn8 = assign6380_e6734_d_n8;
        locals.var_qf_dn9 = assign6380_e6734_d_n9;

        let (assign6390_e6755, assign6390_e6755_d_n0, assign6390_e6755_d_n1, assign6390_e6755_d_n3, assign6390_e6755_d_n4, assign6390_e6755_d_n5, assign6390_e6755_d_n6, assign6390_e6755_d_n7, assign6390_e6755_d_n8, assign6390_e6755_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard130 == 0.0)) {
        let assign6390_e6741: f64 = (locals.var_hf0_t * locals.var_tf);
        let assign6390_e6744: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign6390_e6745: f64 = (assign6390_e6741 + assign6390_e6744);
        let assign6390_e6748: f64 = (locals.var_hfe_t * locals.var_ffdtef);
        let assign6390_e6749: f64 = (assign6390_e6745 + assign6390_e6748);
        let assign6390_e6752: f64 = (locals.var_hfc_t * locals.var_ffdtcfc);
        let assign6390_e6753: f64 = (assign6390_e6749 + assign6390_e6752);
        (assign6390_e6753, ((((locals.var_hf0_t * locals.var_tf_dn0) + (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0)) + (locals.var_hfe_t * locals.var_ffdtef_dn0)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn0)), ((((locals.var_hf0_t * locals.var_tf_dn1) + (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1)) + (locals.var_hfe_t * locals.var_ffdtef_dn1)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn1)), ((((locals.var_hf0_t * locals.var_tf_dn3) + (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3)) + (locals.var_hfe_t * locals.var_ffdtef_dn3)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn3)), (((((locals.var_hf0_t_dn4 * locals.var_tf) + (locals.var_hf0_t * locals.var_tf_dn4)) + (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4)) + ((locals.var_hfe_t_dn4 * locals.var_ffdtef) + (locals.var_hfe_t * locals.var_ffdtef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdtcfc) + (locals.var_hfc_t * locals.var_ffdtcfc_dn4))), ((((locals.var_hf0_t * locals.var_tf_dn5) + (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5)) + (locals.var_hfe_t * locals.var_ffdtef_dn5)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn5)), ((((locals.var_hf0_t * locals.var_tf_dn6) + (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6)) + (locals.var_hfe_t * locals.var_ffdtef_dn6)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn6)), ((((locals.var_hf0_t * locals.var_tf_dn7) + (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7)) + (locals.var_hfe_t * locals.var_ffdtef_dn7)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn7)), ((((locals.var_hf0_t * locals.var_tf_dn8) + (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8)) + (locals.var_hfe_t * locals.var_ffdtef_dn8)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn8)), ((((locals.var_hf0_t * locals.var_tf_dn9) + (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9)) + (locals.var_hfe_t * locals.var_ffdtef_dn9)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn9)),)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
        locals.var_t_ft = assign6390_e6755;
        locals.var_t_ft_dn0 = assign6390_e6755_d_n0;
        locals.var_t_ft_dn1 = assign6390_e6755_d_n1;
        locals.var_t_ft_dn3 = assign6390_e6755_d_n3;
        locals.var_t_ft_dn4 = assign6390_e6755_d_n4;
        locals.var_t_ft_dn5 = assign6390_e6755_d_n5;
        locals.var_t_ft_dn6 = assign6390_e6755_d_n6;
        locals.var_t_ft_dn7 = assign6390_e6755_d_n7;
        locals.var_t_ft_dn8 = assign6390_e6755_d_n8;
        locals.var_t_ft_dn9 = assign6390_e6755_d_n9;

        let (assign6400_e6770, assign6400_e6770_d_n0, assign6400_e6770_d_n1, assign6400_e6770_d_n3, assign6400_e6770_d_n4, assign6400_e6770_d_n5, assign6400_e6770_d_n6, assign6400_e6770_d_n7, assign6400_e6770_d_n8, assign6400_e6770_d_n9,) = {
    if ((locals.var_guard124 != 0.0) && (locals.var_guard130 == 0.0)) {
        let assign6400_e6763: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign6400_e6764: f64 = (locals.var_tf + assign6400_e6763);
        let assign6400_e6766: f64 = (assign6400_e6764 + locals.var_ffdtef);
        let assign6400_e6768: f64 = (assign6400_e6766 + locals.var_ffdtcfc);
        (assign6400_e6768, (((locals.var_tf_dn0 + (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0)) + locals.var_ffdtef_dn0) + locals.var_ffdtcfc_dn0), (((locals.var_tf_dn1 + (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1)) + locals.var_ffdtef_dn1) + locals.var_ffdtcfc_dn1), (((locals.var_tf_dn3 + (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3)) + locals.var_ffdtef_dn3) + locals.var_ffdtcfc_dn3), (((locals.var_tf_dn4 + (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4)) + locals.var_ffdtef_dn4) + locals.var_ffdtcfc_dn4), (((locals.var_tf_dn5 + (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5)) + locals.var_ffdtef_dn5) + locals.var_ffdtcfc_dn5), (((locals.var_tf_dn6 + (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6)) + locals.var_ffdtef_dn6) + locals.var_ffdtcfc_dn6), (((locals.var_tf_dn7 + (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7)) + locals.var_ffdtef_dn7) + locals.var_ffdtcfc_dn7), (((locals.var_tf_dn8 + (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8)) + locals.var_ffdtef_dn8) + locals.var_ffdtcfc_dn8), (((locals.var_tf_dn9 + (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9)) + locals.var_ffdtef_dn9) + locals.var_ffdtcfc_dn9),)
    } else {
        (locals.var_tf, locals.var_tf_dn0, locals.var_tf_dn1, locals.var_tf_dn3, locals.var_tf_dn4, locals.var_tf_dn5, locals.var_tf_dn6, locals.var_tf_dn7, locals.var_tf_dn8, locals.var_tf_dn9,)
    }
};
        locals.var_tf = assign6400_e6770;
        locals.var_tf_dn0 = assign6400_e6770_d_n0;
        locals.var_tf_dn1 = assign6400_e6770_d_n1;
        locals.var_tf_dn3 = assign6400_e6770_d_n3;
        locals.var_tf_dn4 = assign6400_e6770_d_n4;
        locals.var_tf_dn5 = assign6400_e6770_d_n5;
        locals.var_tf_dn6 = assign6400_e6770_d_n6;
        locals.var_tf_dn7 = assign6400_e6770_d_n7;
        locals.var_tf_dn8 = assign6400_e6770_d_n8;
        locals.var_tf_dn9 = assign6400_e6770_d_n9;

        let assign6410_e6773: f64 = (p.p85 * locals.var_itr);
        locals.var_qr = assign6410_e6773;
        locals.var_qr_dn0 = (p.p85 * locals.var_itr_dn0);
        locals.var_qr_dn1 = (p.p85 * locals.var_itr_dn1);
        locals.var_qr_dn3 = (p.p85 * locals.var_itr_dn3);
        locals.var_qr_dn4 = (p.p85 * locals.var_itr_dn4);
        locals.var_qr_dn5 = (p.p85 * locals.var_itr_dn5);
        locals.var_qr_dn6 = (p.p85 * locals.var_itr_dn6);
        locals.var_qr_dn7 = (p.p85 * locals.var_itr_dn7);
        locals.var_qr_dn8 = (p.p85 * locals.var_itr_dn8);
        locals.var_qr_dn9 = (p.p85 * locals.var_itr_dn9);

        locals.var_l_it = 0.0;

        let assign6430_e6781: f64 = 1e-5;
        let assign6430_e6783: f64 = (assign6430_e6781 * locals.var_q_pt);
        let assign6430_e6792: f64 = 1e-5;
        let assign6430_e6794: f64 = (assign6430_e6792 * locals.var_q_pt);
        let assign6430_e6797: f64 = if (((p.p0 >= 310.0) && (locals.var_q_ft > assign6430_e6783)) || ((p.p0 <= 300.0) && (locals.var_qf > assign6430_e6794))) { 1.0 } else { 0.0 };
        locals.var_guard131 = assign6430_e6797;

        let (assign6440_e6806, assign6440_e6806_d_n0, assign6440_e6806_d_n1, assign6440_e6806_d_n3, assign6440_e6806_d_n4, assign6440_e6806_d_n5, assign6440_e6806_d_n6, assign6440_e6806_d_n7, assign6440_e6806_d_n8, assign6440_e6806_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6440_e6801: f64 = (locals.var_t_f0 * locals.var_itf);
        let assign6440_e6803: f64 = (assign6440_e6801 * locals.var_q_ft);
        let assign6440_e6804: f64 = (assign6440_e6803).sqrt();
        (assign6440_e6804, ((((locals.var_t_f0 * locals.var_itf_dn0) * locals.var_q_ft) + (assign6440_e6801 * locals.var_q_ft_dn0)) / (2.0 * assign6440_e6804)), ((((locals.var_t_f0 * locals.var_itf_dn1) * locals.var_q_ft) + (assign6440_e6801 * locals.var_q_ft_dn1)) / (2.0 * assign6440_e6804)), ((((locals.var_t_f0 * locals.var_itf_dn3) * locals.var_q_ft) + (assign6440_e6801 * locals.var_q_ft_dn3)) / (2.0 * assign6440_e6804)), (((((locals.var_t_f0_dn4 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn4)) * locals.var_q_ft) + (assign6440_e6801 * locals.var_q_ft_dn4)) / (2.0 * assign6440_e6804)), (((((locals.var_t_f0_dn5 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn5)) * locals.var_q_ft) + (assign6440_e6801 * locals.var_q_ft_dn5)) / (2.0 * assign6440_e6804)), ((((locals.var_t_f0 * locals.var_itf_dn6) * locals.var_q_ft) + (assign6440_e6801 * locals.var_q_ft_dn6)) / (2.0 * assign6440_e6804)), ((((locals.var_t_f0 * locals.var_itf_dn7) * locals.var_q_ft) + (assign6440_e6801 * locals.var_q_ft_dn7)) / (2.0 * assign6440_e6804)), (((((locals.var_t_f0_dn8 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn8)) * locals.var_q_ft) + (assign6440_e6801 * locals.var_q_ft_dn8)) / (2.0 * assign6440_e6804)), ((((locals.var_t_f0 * locals.var_itf_dn9) * locals.var_q_ft) + (assign6440_e6801 * locals.var_q_ft_dn9)) / (2.0 * assign6440_e6804)),)
    } else {
        (locals.var_qf, locals.var_qf_dn0, locals.var_qf_dn1, locals.var_qf_dn3, locals.var_qf_dn4, locals.var_qf_dn5, locals.var_qf_dn6, locals.var_qf_dn7, locals.var_qf_dn8, locals.var_qf_dn9,)
    }
};
        locals.var_qf = assign6440_e6806;
        locals.var_qf_dn0 = assign6440_e6806_d_n0;
        locals.var_qf_dn1 = assign6440_e6806_d_n1;
        locals.var_qf_dn3 = assign6440_e6806_d_n3;
        locals.var_qf_dn4 = assign6440_e6806_d_n4;
        locals.var_qf_dn5 = assign6440_e6806_d_n5;
        locals.var_qf_dn6 = assign6440_e6806_d_n6;
        locals.var_qf_dn7 = assign6440_e6806_d_n7;
        locals.var_qf_dn8 = assign6440_e6806_d_n8;
        locals.var_qf_dn9 = assign6440_e6806_d_n9;

        let (assign6450_e6816, assign6450_e6816_d_n0, assign6450_e6816_d_n1, assign6450_e6816_d_n3, assign6450_e6816_d_n4, assign6450_e6816_d_n5, assign6450_e6816_d_n6, assign6450_e6816_d_n7, assign6450_e6816_d_n8, assign6450_e6816_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6450_e6810: f64 = (locals.var_q0_pt + locals.var_qf);
        let assign6450_e6813: f64 = (p.p7 * locals.var_qr);
        let assign6450_e6814: f64 = (assign6450_e6810 + assign6450_e6813);
        (assign6450_e6814, ((locals.var_q0_pt_dn0 + locals.var_qf_dn0) + (p.p7 * locals.var_qr_dn0)), ((locals.var_q0_pt_dn1 + locals.var_qf_dn1) + (p.p7 * locals.var_qr_dn1)), ((locals.var_q0_pt_dn3 + locals.var_qf_dn3) + (p.p7 * locals.var_qr_dn3)), ((locals.var_q0_pt_dn4 + locals.var_qf_dn4) + (p.p7 * locals.var_qr_dn4)), ((locals.var_q0_pt_dn5 + locals.var_qf_dn5) + (p.p7 * locals.var_qr_dn5)), ((locals.var_q0_pt_dn6 + locals.var_qf_dn6) + (p.p7 * locals.var_qr_dn6)), ((locals.var_q0_pt_dn7 + locals.var_qf_dn7) + (p.p7 * locals.var_qr_dn7)), ((locals.var_q0_pt_dn8 + locals.var_qf_dn8) + (p.p7 * locals.var_qr_dn8)), ((locals.var_q0_pt_dn9 + locals.var_qf_dn9) + (p.p7 * locals.var_qr_dn9)),)
    } else {
        (locals.var_q_pt, locals.var_q_pt_dn0, locals.var_q_pt_dn1, locals.var_q_pt_dn3, locals.var_q_pt_dn4, locals.var_q_pt_dn5, locals.var_q_pt_dn6, locals.var_q_pt_dn7, locals.var_q_pt_dn8, locals.var_q_pt_dn9,)
    }
};
        locals.var_q_pt = assign6450_e6816;
        locals.var_q_pt_dn0 = assign6450_e6816_d_n0;
        locals.var_q_pt_dn1 = assign6450_e6816_d_n1;
        locals.var_q_pt_dn3 = assign6450_e6816_d_n3;
        locals.var_q_pt_dn4 = assign6450_e6816_d_n4;
        locals.var_q_pt_dn5 = assign6450_e6816_d_n5;
        locals.var_q_pt_dn6 = assign6450_e6816_d_n6;
        locals.var_q_pt_dn7 = assign6450_e6816_d_n7;
        locals.var_q_pt_dn8 = assign6450_e6816_d_n8;
        locals.var_q_pt_dn9 = assign6450_e6816_d_n9;

        let (assign6460_e6820, assign6460_e6820_d_n0, assign6460_e6820_d_n1, assign6460_e6820_d_n3, assign6460_e6820_d_n4, assign6460_e6820_d_n5, assign6460_e6820_d_n6, assign6460_e6820_d_n7, assign6460_e6820_d_n8, assign6460_e6820_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        (locals.var_q_pt, locals.var_q_pt_dn0, locals.var_q_pt_dn1, locals.var_q_pt_dn3, locals.var_q_pt_dn4, locals.var_q_pt_dn5, locals.var_q_pt_dn6, locals.var_q_pt_dn7, locals.var_q_pt_dn8, locals.var_q_pt_dn9,)
    } else {
        (locals.var_d_q, locals.var_d_q_dn0, locals.var_d_q_dn1, locals.var_d_q_dn3, locals.var_d_q_dn4, locals.var_d_q_dn5, locals.var_d_q_dn6, locals.var_d_q_dn7, locals.var_d_q_dn8, locals.var_d_q_dn9,)
    }
};
        locals.var_d_q = assign6460_e6820;
        locals.var_d_q_dn0 = assign6460_e6820_d_n0;
        locals.var_d_q_dn1 = assign6460_e6820_d_n1;
        locals.var_d_q_dn3 = assign6460_e6820_d_n3;
        locals.var_d_q_dn4 = assign6460_e6820_d_n4;
        locals.var_d_q_dn5 = assign6460_e6820_d_n5;
        locals.var_d_q_dn6 = assign6460_e6820_d_n6;
        locals.var_d_q_dn7 = assign6460_e6820_d_n7;
        locals.var_d_q_dn8 = assign6460_e6820_d_n8;
        locals.var_d_q_dn9 = assign6460_e6820_d_n9;

    }
}

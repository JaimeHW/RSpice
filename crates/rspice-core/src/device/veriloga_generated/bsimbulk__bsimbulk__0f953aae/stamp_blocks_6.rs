#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_96(
        locals: &mut StampLocals,
    ) {
        let (assign30680_e40836, assign30680_e40836_d_n0, assign30680_e40836_d_n2, assign30680_e40836_d_n3, assign30680_e40836_d_n4, assign30680_e40836_d_n5, assign30680_e40836_d_n6, assign30680_e40836_d_n7, assign30680_e40836_d_n8, assign30680_e40836_d_n9, assign30680_e40836_d_n10, assign30680_e40836_d_n11, assign30680_e40836_d_n12, assign30680_e40836_d_n13, assign30680_e40836_d_n14,) = {
    if (locals.var_guard704 != 0.0) {
        let assign30680_e40834: f64 = (1.0 + locals.var_t3);
        (assign30680_e40834, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign30680_e40836;
        locals.var_t4_dn0 = assign30680_e40836_d_n0;
        locals.var_t4_dn2 = assign30680_e40836_d_n2;
        locals.var_t4_dn3 = assign30680_e40836_d_n3;
        locals.var_t4_dn4 = assign30680_e40836_d_n4;
        locals.var_t4_dn5 = assign30680_e40836_d_n5;
        locals.var_t4_dn6 = assign30680_e40836_d_n6;
        locals.var_t4_dn7 = assign30680_e40836_d_n7;
        locals.var_t4_dn8 = assign30680_e40836_d_n8;
        locals.var_t4_dn9 = assign30680_e40836_d_n9;
        locals.var_t4_dn10 = assign30680_e40836_d_n10;
        locals.var_t4_dn11 = assign30680_e40836_d_n11;
        locals.var_t4_dn12 = assign30680_e40836_d_n12;
        locals.var_t4_dn13 = assign30680_e40836_d_n13;
        locals.var_t4_dn14 = assign30680_e40836_d_n14;

        let assign30690_e40842: f64 = (-2500.0);
        let assign30690_e40844: f64 = (assign30690_e40842 * 0.0015);
        let assign30690_e40846: f64 = if ((1.0 == 0.0) && (locals.var_t4 < assign30690_e40844)) { 1.0 } else { 0.0 };
        locals.var_guard723 = assign30690_e40846;

        let (assign30700_e40859, assign30700_e40859_d_n0, assign30700_e40859_d_n2, assign30700_e40859_d_n3, assign30700_e40859_d_n4, assign30700_e40859_d_n5, assign30700_e40859_d_n6, assign30700_e40859_d_n7, assign30700_e40859_d_n8, assign30700_e40859_d_n9, assign30700_e40859_d_n10, assign30700_e40859_d_n11, assign30700_e40859_d_n12, assign30700_e40859_d_n13, assign30700_e40859_d_n14,) = {
    if ((locals.var_guard704 != 0.0) && (locals.var_guard723 != 0.0)) {
        let assign30700_e40851: f64 = (-0.0015);
        let assign30700_e40853: f64 = (assign30700_e40851 * 0.0015);
        let assign30700_e40856: f64 = (16.0 * locals.var_t4);
        let assign30700_e40857: f64 = (assign30700_e40853 / assign30700_e40856);
        (assign30700_e40857, (-((assign30700_e40853 * (16.0 * locals.var_t4_dn0)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn2)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn3)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn4)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn5)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn6)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn7)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn8)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn9)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn10)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn11)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn12)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn13)) / (assign30700_e40856 * assign30700_e40856))), (-((assign30700_e40853 * (16.0 * locals.var_t4_dn14)) / (assign30700_e40856 * assign30700_e40856))),)
    } else {
        (locals.var_dmob, locals.var_dmob_dn0, locals.var_dmob_dn2, locals.var_dmob_dn3, locals.var_dmob_dn4, locals.var_dmob_dn5, locals.var_dmob_dn6, locals.var_dmob_dn7, locals.var_dmob_dn8, locals.var_dmob_dn9, locals.var_dmob_dn10, locals.var_dmob_dn11, locals.var_dmob_dn12, locals.var_dmob_dn13, locals.var_dmob_dn14,)
    }
};
        locals.var_dmob = assign30700_e40859;
        locals.var_dmob_dn0 = assign30700_e40859_d_n0;
        locals.var_dmob_dn2 = assign30700_e40859_d_n2;
        locals.var_dmob_dn3 = assign30700_e40859_d_n3;
        locals.var_dmob_dn4 = assign30700_e40859_d_n4;
        locals.var_dmob_dn5 = assign30700_e40859_d_n5;
        locals.var_dmob_dn6 = assign30700_e40859_d_n6;
        locals.var_dmob_dn7 = assign30700_e40859_d_n7;
        locals.var_dmob_dn8 = assign30700_e40859_d_n8;
        locals.var_dmob_dn9 = assign30700_e40859_d_n9;
        locals.var_dmob_dn10 = assign30700_e40859_d_n10;
        locals.var_dmob_dn11 = assign30700_e40859_d_n11;
        locals.var_dmob_dn12 = assign30700_e40859_d_n12;
        locals.var_dmob_dn13 = assign30700_e40859_d_n13;
        locals.var_dmob_dn14 = assign30700_e40859_d_n14;

        let (assign30710_e40885, assign30710_e40885_d_n0, assign30710_e40885_d_n2, assign30710_e40885_d_n3, assign30710_e40885_d_n4, assign30710_e40885_d_n5, assign30710_e40885_d_n6, assign30710_e40885_d_n7, assign30710_e40885_d_n8, assign30710_e40885_d_n9, assign30710_e40885_d_n10, assign30710_e40885_d_n11, assign30710_e40885_d_n12, assign30710_e40885_d_n13, assign30710_e40885_d_n14,) = {
    if ((locals.var_guard704 != 0.0) && (locals.var_guard723 == 0.0)) {
        let assign30710_e40867: f64 = (locals.var_t4 + 1.0);
        let assign30710_e40870: f64 = (locals.var_t4 - 1.0);
        let assign30710_e40873: f64 = (locals.var_t4 - 1.0);
        let assign30710_e40874: f64 = (assign30710_e40870 * assign30710_e40873);
        let assign30710_e40877: f64 = (0.25 * 0.0015);
        let assign30710_e40879: f64 = (assign30710_e40877 * 0.0015);
        let assign30710_e40880: f64 = (assign30710_e40874 + assign30710_e40879);
        let assign30710_e40881: f64 = (assign30710_e40880).sqrt();
        let assign30710_e40882: f64 = (assign30710_e40867 + assign30710_e40881);
        let assign30710_e40883: f64 = (0.5 * assign30710_e40882);
        (assign30710_e40883, (0.5 * (locals.var_t4_dn0 + (((locals.var_t4_dn0 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn0)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn2 + (((locals.var_t4_dn2 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn2)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn3)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn4)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn5)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn6)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn7)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn8)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn9)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn10)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn11)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn12 + (((locals.var_t4_dn12 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn12)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn13 + (((locals.var_t4_dn13 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn13)) / (2.0 * assign30710_e40881)))), (0.5 * (locals.var_t4_dn14 + (((locals.var_t4_dn14 * assign30710_e40873) + (assign30710_e40870 * locals.var_t4_dn14)) / (2.0 * assign30710_e40881)))),)
    } else {
        (locals.var_dmob, locals.var_dmob_dn0, locals.var_dmob_dn2, locals.var_dmob_dn3, locals.var_dmob_dn4, locals.var_dmob_dn5, locals.var_dmob_dn6, locals.var_dmob_dn7, locals.var_dmob_dn8, locals.var_dmob_dn9, locals.var_dmob_dn10, locals.var_dmob_dn11, locals.var_dmob_dn12, locals.var_dmob_dn13, locals.var_dmob_dn14,)
    }
};
        locals.var_dmob = assign30710_e40885;
        locals.var_dmob_dn0 = assign30710_e40885_d_n0;
        locals.var_dmob_dn2 = assign30710_e40885_d_n2;
        locals.var_dmob_dn3 = assign30710_e40885_d_n3;
        locals.var_dmob_dn4 = assign30710_e40885_d_n4;
        locals.var_dmob_dn5 = assign30710_e40885_d_n5;
        locals.var_dmob_dn6 = assign30710_e40885_d_n6;
        locals.var_dmob_dn7 = assign30710_e40885_d_n7;
        locals.var_dmob_dn8 = assign30710_e40885_d_n8;
        locals.var_dmob_dn9 = assign30710_e40885_d_n9;
        locals.var_dmob_dn10 = assign30710_e40885_d_n10;
        locals.var_dmob_dn11 = assign30710_e40885_d_n11;
        locals.var_dmob_dn12 = assign30710_e40885_d_n12;
        locals.var_dmob_dn13 = assign30710_e40885_d_n13;
        locals.var_dmob_dn14 = assign30710_e40885_d_n14;

        let (assign30720_e40899, assign30720_e40899_d_n0, assign30720_e40899_d_n2, assign30720_e40899_d_n3, assign30720_e40899_d_n4, assign30720_e40899_d_n5, assign30720_e40899_d_n6, assign30720_e40899_d_n7, assign30720_e40899_d_n8, assign30720_e40899_d_n9, assign30720_e40899_d_n10, assign30720_e40899_d_n11, assign30720_e40899_d_n12, assign30720_e40899_d_n13, assign30720_e40899_d_n14,) = {
    if (locals.var_guard704 != 0.0) {
        let assign30720_e40890: f64 = (locals.var_u0_a / locals.var_dmob);
        let assign30720_e40891: f64 = (2.0 * assign30720_e40890);
        let assign30720_e40893: f64 = (assign30720_e40891 * locals.var_vt);
        let assign30720_e40896: f64 = (locals.var_vsatcv_t * locals.var_lact);
        let assign30720_e40897: f64 = (assign30720_e40893 / assign30720_e40896);
        (assign30720_e40897, (((((2.0 * (((locals.var_u0_a_dn0 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn0)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn0 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn2 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn2)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn2 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn3 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn3)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn3 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), ((((((2.0 * (((locals.var_u0_a_dn4 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn4)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) + (assign30720_e40891 * locals.var_vt_dn4)) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn4 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn5 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn5)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn5 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn6 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn6)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn6 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn7 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn7)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn7 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn8 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn8)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn8 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn9 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn9)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn9 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn10 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn10)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn10 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn11 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn11)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn11 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn12 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn12)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn12 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn13 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn13)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn13 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)), (((((2.0 * (((locals.var_u0_a_dn14 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn14)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign30720_e40896) - (assign30720_e40893 * (locals.var_vsatcv_t_dn14 * locals.var_lact))) / (assign30720_e40896 * assign30720_e40896)),)
    } else {
        (locals.var_lambdac, locals.var_lambdac_dn0, locals.var_lambdac_dn2, locals.var_lambdac_dn3, locals.var_lambdac_dn4, locals.var_lambdac_dn5, locals.var_lambdac_dn6, locals.var_lambdac_dn7, locals.var_lambdac_dn8, locals.var_lambdac_dn9, locals.var_lambdac_dn10, locals.var_lambdac_dn11, locals.var_lambdac_dn12, locals.var_lambdac_dn13, locals.var_lambdac_dn14,)
    }
};
        locals.var_lambdac = assign30720_e40899;
        locals.var_lambdac_dn0 = assign30720_e40899_d_n0;
        locals.var_lambdac_dn2 = assign30720_e40899_d_n2;
        locals.var_lambdac_dn3 = assign30720_e40899_d_n3;
        locals.var_lambdac_dn4 = assign30720_e40899_d_n4;
        locals.var_lambdac_dn5 = assign30720_e40899_d_n5;
        locals.var_lambdac_dn6 = assign30720_e40899_d_n6;
        locals.var_lambdac_dn7 = assign30720_e40899_d_n7;
        locals.var_lambdac_dn8 = assign30720_e40899_d_n8;
        locals.var_lambdac_dn9 = assign30720_e40899_d_n9;
        locals.var_lambdac_dn10 = assign30720_e40899_d_n10;
        locals.var_lambdac_dn11 = assign30720_e40899_d_n11;
        locals.var_lambdac_dn12 = assign30720_e40899_d_n12;
        locals.var_lambdac_dn13 = assign30720_e40899_d_n13;
        locals.var_lambdac_dn14 = assign30720_e40899_d_n14;

        let (assign30730_e40905, assign30730_e40905_d_n0, assign30730_e40905_d_n2, assign30730_e40905_d_n3, assign30730_e40905_d_n4, assign30730_e40905_d_n5, assign30730_e40905_d_n6, assign30730_e40905_d_n7, assign30730_e40905_d_n8, assign30730_e40905_d_n9, assign30730_e40905_d_n10, assign30730_e40905_d_n11, assign30730_e40905_d_n12, assign30730_e40905_d_n13, assign30730_e40905_d_n14,) = {
    if (locals.var_guard704 != 0.0) {
        let assign30730_e40903: f64 = (locals.var_qs_1 - locals.var_qdeff);
        (assign30730_e40903, (locals.var_qs_1_dn0 - locals.var_qdeff_dn0), (locals.var_qs_1_dn2 - locals.var_qdeff_dn2), (locals.var_qs_1_dn3 - locals.var_qdeff_dn3), (locals.var_qs_1_dn4 - locals.var_qdeff_dn4), (locals.var_qs_1_dn5 - locals.var_qdeff_dn5), (locals.var_qs_1_dn6 - locals.var_qdeff_dn6), (locals.var_qs_1_dn7 - locals.var_qdeff_dn7), (locals.var_qs_1_dn8 - locals.var_qdeff_dn8), (locals.var_qs_1_dn9 - locals.var_qdeff_dn9), (locals.var_qs_1_dn10 - locals.var_qdeff_dn10), (locals.var_qs_1_dn11 - locals.var_qdeff_dn11), (locals.var_qs_1_dn12 - locals.var_qdeff_dn12), (locals.var_qs_1_dn13 - locals.var_qdeff_dn13), (locals.var_qs_1_dn14 - locals.var_qdeff_dn14),)
    } else {
        (locals.var_dps, locals.var_dps_dn0, locals.var_dps_dn2, locals.var_dps_dn3, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn11, locals.var_dps_dn12, locals.var_dps_dn13, locals.var_dps_dn14,)
    }
};
        locals.var_dps = assign30730_e40905;
        locals.var_dps_dn0 = assign30730_e40905_d_n0;
        locals.var_dps_dn2 = assign30730_e40905_d_n2;
        locals.var_dps_dn3 = assign30730_e40905_d_n3;
        locals.var_dps_dn4 = assign30730_e40905_d_n4;
        locals.var_dps_dn5 = assign30730_e40905_d_n5;
        locals.var_dps_dn6 = assign30730_e40905_d_n6;
        locals.var_dps_dn7 = assign30730_e40905_d_n7;
        locals.var_dps_dn8 = assign30730_e40905_d_n8;
        locals.var_dps_dn9 = assign30730_e40905_d_n9;
        locals.var_dps_dn10 = assign30730_e40905_d_n10;
        locals.var_dps_dn11 = assign30730_e40905_d_n11;
        locals.var_dps_dn12 = assign30730_e40905_d_n12;
        locals.var_dps_dn13 = assign30730_e40905_d_n13;
        locals.var_dps_dn14 = assign30730_e40905_d_n14;

        let (assign30740_e40917, assign30740_e40917_d_n0, assign30740_e40917_d_n2, assign30740_e40917_d_n3, assign30740_e40917_d_n4, assign30740_e40917_d_n5, assign30740_e40917_d_n6, assign30740_e40917_d_n7, assign30740_e40917_d_n8, assign30740_e40917_d_n9, assign30740_e40917_d_n10, assign30740_e40917_d_n11, assign30740_e40917_d_n12, assign30740_e40917_d_n13, assign30740_e40917_d_n14,) = {
    if (locals.var_guard704 != 0.0) {
        let assign30740_e40910: f64 = (locals.var_lambdac * locals.var_dps);
        let assign30740_e40911: f64 = (2.0 * assign30740_e40910);
        let assign30740_e40914: f64 = (locals.var_lambdac * locals.var_dps);
        let assign30740_e40915: f64 = (assign30740_e40911 * assign30740_e40914);
        (assign30740_e40915, (((2.0 * ((locals.var_lambdac_dn0 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn0))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn0 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn0)))), (((2.0 * ((locals.var_lambdac_dn2 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn2))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn2 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn2)))), (((2.0 * ((locals.var_lambdac_dn3 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn3))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn3 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn3)))), (((2.0 * ((locals.var_lambdac_dn4 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn4))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn4 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn4)))), (((2.0 * ((locals.var_lambdac_dn5 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn5))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn5 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn5)))), (((2.0 * ((locals.var_lambdac_dn6 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn6))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn6 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn6)))), (((2.0 * ((locals.var_lambdac_dn7 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn7))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn7 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn7)))), (((2.0 * ((locals.var_lambdac_dn8 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn8))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn8 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn8)))), (((2.0 * ((locals.var_lambdac_dn9 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn9))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn9 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn9)))), (((2.0 * ((locals.var_lambdac_dn10 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn10))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn10 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn10)))), (((2.0 * ((locals.var_lambdac_dn11 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn11))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn11 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn11)))), (((2.0 * ((locals.var_lambdac_dn12 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn12))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn12 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn12)))), (((2.0 * ((locals.var_lambdac_dn13 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn13))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn13 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn13)))), (((2.0 * ((locals.var_lambdac_dn14 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn14))) * assign30740_e40914) + (assign30740_e40911 * ((locals.var_lambdac_dn14 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn14)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign30740_e40917;
        locals.var_t1_dn0 = assign30740_e40917_d_n0;
        locals.var_t1_dn2 = assign30740_e40917_d_n2;
        locals.var_t1_dn3 = assign30740_e40917_d_n3;
        locals.var_t1_dn4 = assign30740_e40917_d_n4;
        locals.var_t1_dn5 = assign30740_e40917_d_n5;
        locals.var_t1_dn6 = assign30740_e40917_d_n6;
        locals.var_t1_dn7 = assign30740_e40917_d_n7;
        locals.var_t1_dn8 = assign30740_e40917_d_n8;
        locals.var_t1_dn9 = assign30740_e40917_d_n9;
        locals.var_t1_dn10 = assign30740_e40917_d_n10;
        locals.var_t1_dn11 = assign30740_e40917_d_n11;
        locals.var_t1_dn12 = assign30740_e40917_d_n12;
        locals.var_t1_dn13 = assign30740_e40917_d_n13;
        locals.var_t1_dn14 = assign30740_e40917_d_n14;

        let (assign30750_e40924, assign30750_e40924_d_n0, assign30750_e40924_d_n2, assign30750_e40924_d_n3, assign30750_e40924_d_n4, assign30750_e40924_d_n5, assign30750_e40924_d_n6, assign30750_e40924_d_n7, assign30750_e40924_d_n8, assign30750_e40924_d_n9, assign30750_e40924_d_n10, assign30750_e40924_d_n11, assign30750_e40924_d_n12, assign30750_e40924_d_n13, assign30750_e40924_d_n14,) = {
    if (locals.var_guard704 != 0.0) {
        let assign30750_e40921: f64 = (1.0 + locals.var_t1);
        let assign30750_e40922: f64 = (assign30750_e40921).sqrt();
        (assign30750_e40922, (locals.var_t1_dn0 / (2.0 * assign30750_e40922)), (locals.var_t1_dn2 / (2.0 * assign30750_e40922)), (locals.var_t1_dn3 / (2.0 * assign30750_e40922)), (locals.var_t1_dn4 / (2.0 * assign30750_e40922)), (locals.var_t1_dn5 / (2.0 * assign30750_e40922)), (locals.var_t1_dn6 / (2.0 * assign30750_e40922)), (locals.var_t1_dn7 / (2.0 * assign30750_e40922)), (locals.var_t1_dn8 / (2.0 * assign30750_e40922)), (locals.var_t1_dn9 / (2.0 * assign30750_e40922)), (locals.var_t1_dn10 / (2.0 * assign30750_e40922)), (locals.var_t1_dn11 / (2.0 * assign30750_e40922)), (locals.var_t1_dn12 / (2.0 * assign30750_e40922)), (locals.var_t1_dn13 / (2.0 * assign30750_e40922)), (locals.var_t1_dn14 / (2.0 * assign30750_e40922)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn0, locals.var_zsat_dn2, locals.var_zsat_dn3, locals.var_zsat_dn4, locals.var_zsat_dn5, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8, locals.var_zsat_dn9, locals.var_zsat_dn10, locals.var_zsat_dn11, locals.var_zsat_dn12, locals.var_zsat_dn13, locals.var_zsat_dn14,)
    }
};
        locals.var_zsat = assign30750_e40924;
        locals.var_zsat_dn0 = assign30750_e40924_d_n0;
        locals.var_zsat_dn2 = assign30750_e40924_d_n2;
        locals.var_zsat_dn3 = assign30750_e40924_d_n3;
        locals.var_zsat_dn4 = assign30750_e40924_d_n4;
        locals.var_zsat_dn5 = assign30750_e40924_d_n5;
        locals.var_zsat_dn6 = assign30750_e40924_d_n6;
        locals.var_zsat_dn7 = assign30750_e40924_d_n7;
        locals.var_zsat_dn8 = assign30750_e40924_d_n8;
        locals.var_zsat_dn9 = assign30750_e40924_d_n9;
        locals.var_zsat_dn10 = assign30750_e40924_d_n10;
        locals.var_zsat_dn11 = assign30750_e40924_d_n11;
        locals.var_zsat_dn12 = assign30750_e40924_d_n12;
        locals.var_zsat_dn13 = assign30750_e40924_d_n13;
        locals.var_zsat_dn14 = assign30750_e40924_d_n14;

        let (assign30760_e40932, assign30760_e40932_d_n0, assign30760_e40932_d_n2, assign30760_e40932_d_n3, assign30760_e40932_d_n4, assign30760_e40932_d_n5, assign30760_e40932_d_n6, assign30760_e40932_d_n7, assign30760_e40932_d_n8, assign30760_e40932_d_n9, assign30760_e40932_d_n10, assign30760_e40932_d_n11, assign30760_e40932_d_n12, assign30760_e40932_d_n13, assign30760_e40932_d_n14,) = {
    if (locals.var_guard704 != 0.0) {
        let assign30760_e40929: f64 = (1.0 + locals.var_zsat);
        let assign30760_e40930: f64 = (0.5 * assign30760_e40929);
        (assign30760_e40930, (0.5 * locals.var_zsat_dn0), (0.5 * locals.var_zsat_dn2), (0.5 * locals.var_zsat_dn3), (0.5 * locals.var_zsat_dn4), (0.5 * locals.var_zsat_dn5), (0.5 * locals.var_zsat_dn6), (0.5 * locals.var_zsat_dn7), (0.5 * locals.var_zsat_dn8), (0.5 * locals.var_zsat_dn9), (0.5 * locals.var_zsat_dn10), (0.5 * locals.var_zsat_dn11), (0.5 * locals.var_zsat_dn12), (0.5 * locals.var_zsat_dn13), (0.5 * locals.var_zsat_dn14),)
    } else {
        (locals.var_dvsat, locals.var_dvsat_dn0, locals.var_dvsat_dn2, locals.var_dvsat_dn3, locals.var_dvsat_dn4, locals.var_dvsat_dn5, locals.var_dvsat_dn6, locals.var_dvsat_dn7, locals.var_dvsat_dn8, locals.var_dvsat_dn9, locals.var_dvsat_dn10, locals.var_dvsat_dn11, locals.var_dvsat_dn12, locals.var_dvsat_dn13, locals.var_dvsat_dn14,)
    }
};
        locals.var_dvsat = assign30760_e40932;
        locals.var_dvsat_dn0 = assign30760_e40932_d_n0;
        locals.var_dvsat_dn2 = assign30760_e40932_d_n2;
        locals.var_dvsat_dn3 = assign30760_e40932_d_n3;
        locals.var_dvsat_dn4 = assign30760_e40932_d_n4;
        locals.var_dvsat_dn5 = assign30760_e40932_d_n5;
        locals.var_dvsat_dn6 = assign30760_e40932_d_n6;
        locals.var_dvsat_dn7 = assign30760_e40932_d_n7;
        locals.var_dvsat_dn8 = assign30760_e40932_d_n8;
        locals.var_dvsat_dn9 = assign30760_e40932_d_n9;
        locals.var_dvsat_dn10 = assign30760_e40932_d_n10;
        locals.var_dvsat_dn11 = assign30760_e40932_d_n11;
        locals.var_dvsat_dn12 = assign30760_e40932_d_n12;
        locals.var_dvsat_dn13 = assign30760_e40932_d_n13;
        locals.var_dvsat_dn14 = assign30760_e40932_d_n14;

        let (assign30770_e40942, assign30770_e40942_d_n0, assign30770_e40942_d_n2, assign30770_e40942_d_n3, assign30770_e40942_d_n4, assign30770_e40942_d_n5, assign30770_e40942_d_n6, assign30770_e40942_d_n7, assign30770_e40942_d_n8, assign30770_e40942_d_n9, assign30770_e40942_d_n10, assign30770_e40942_d_n11, assign30770_e40942_d_n12, assign30770_e40942_d_n13, assign30770_e40942_d_n14,) = {
    if (locals.var_guard704 != 0.0) {
        let assign30770_e40936: f64 = (2.0 * locals.var_vsatcv_t);
        let assign30770_e40939: f64 = (locals.var_u0_a / locals.var_dmob);
        let assign30770_e40940: f64 = (assign30770_e40936 / assign30770_e40939);
        (assign30770_e40940, ((((2.0 * locals.var_vsatcv_t_dn0) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn0 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn0)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn2) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn2 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn2)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn3) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn3 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn3)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn4) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn4 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn4)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn5) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn5 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn5)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn6) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn6 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn6)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn7) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn7 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn7)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn8) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn8 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn8)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn9) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn9 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn9)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn10) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn10 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn10)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn11) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn11 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn11)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn12) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn12 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn12)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn13) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn13 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn13)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)), ((((2.0 * locals.var_vsatcv_t_dn14) * assign30770_e40939) - (assign30770_e40936 * (((locals.var_u0_a_dn14 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn14)) / (locals.var_dmob * locals.var_dmob)))) / (assign30770_e40939 * assign30770_e40939)),)
    } else {
        (locals.var_esat, locals.var_esat_dn0, locals.var_esat_dn2, locals.var_esat_dn3, locals.var_esat_dn4, locals.var_esat_dn5, locals.var_esat_dn6, locals.var_esat_dn7, locals.var_esat_dn8, locals.var_esat_dn9, locals.var_esat_dn10, locals.var_esat_dn11, locals.var_esat_dn12, locals.var_esat_dn13, locals.var_esat_dn14,)
    }
};
        locals.var_esat = assign30770_e40942;
        locals.var_esat_dn0 = assign30770_e40942_d_n0;
        locals.var_esat_dn2 = assign30770_e40942_d_n2;
        locals.var_esat_dn3 = assign30770_e40942_d_n3;
        locals.var_esat_dn4 = assign30770_e40942_d_n4;
        locals.var_esat_dn5 = assign30770_e40942_d_n5;
        locals.var_esat_dn6 = assign30770_e40942_d_n6;
        locals.var_esat_dn7 = assign30770_e40942_d_n7;
        locals.var_esat_dn8 = assign30770_e40942_d_n8;
        locals.var_esat_dn9 = assign30770_e40942_d_n9;
        locals.var_esat_dn10 = assign30770_e40942_d_n10;
        locals.var_esat_dn11 = assign30770_e40942_d_n11;
        locals.var_esat_dn12 = assign30770_e40942_d_n12;
        locals.var_esat_dn13 = assign30770_e40942_d_n13;
        locals.var_esat_dn14 = assign30770_e40942_d_n14;

        let (assign30780_e40948, assign30780_e40948_d_n0, assign30780_e40948_d_n2, assign30780_e40948_d_n3, assign30780_e40948_d_n4, assign30780_e40948_d_n5, assign30780_e40948_d_n6, assign30780_e40948_d_n7, assign30780_e40948_d_n8, assign30780_e40948_d_n9, assign30780_e40948_d_n10, assign30780_e40948_d_n11, assign30780_e40948_d_n12, assign30780_e40948_d_n13, assign30780_e40948_d_n14,) = {
    if (locals.var_guard704 != 0.0) {
        let assign30780_e40946: f64 = (locals.var_esat * locals.var_lact);
        (assign30780_e40946, (locals.var_esat_dn0 * locals.var_lact), (locals.var_esat_dn2 * locals.var_lact), (locals.var_esat_dn3 * locals.var_lact), (locals.var_esat_dn4 * locals.var_lact), (locals.var_esat_dn5 * locals.var_lact), (locals.var_esat_dn6 * locals.var_lact), (locals.var_esat_dn7 * locals.var_lact), (locals.var_esat_dn8 * locals.var_lact), (locals.var_esat_dn9 * locals.var_lact), (locals.var_esat_dn10 * locals.var_lact), (locals.var_esat_dn11 * locals.var_lact), (locals.var_esat_dn12 * locals.var_lact), (locals.var_esat_dn13 * locals.var_lact), (locals.var_esat_dn14 * locals.var_lact),)
    } else {
        (locals.var_esatl, locals.var_esatl_dn0, locals.var_esatl_dn2, locals.var_esatl_dn3, locals.var_esatl_dn4, locals.var_esatl_dn5, locals.var_esatl_dn6, locals.var_esatl_dn7, locals.var_esatl_dn8, locals.var_esatl_dn9, locals.var_esatl_dn10, locals.var_esatl_dn11, locals.var_esatl_dn12, locals.var_esatl_dn13, locals.var_esatl_dn14,)
    }
};
        locals.var_esatl = assign30780_e40948;
        locals.var_esatl_dn0 = assign30780_e40948_d_n0;
        locals.var_esatl_dn2 = assign30780_e40948_d_n2;
        locals.var_esatl_dn3 = assign30780_e40948_d_n3;
        locals.var_esatl_dn4 = assign30780_e40948_d_n4;
        locals.var_esatl_dn5 = assign30780_e40948_d_n5;
        locals.var_esatl_dn6 = assign30780_e40948_d_n6;
        locals.var_esatl_dn7 = assign30780_e40948_d_n7;
        locals.var_esatl_dn8 = assign30780_e40948_d_n8;
        locals.var_esatl_dn9 = assign30780_e40948_d_n9;
        locals.var_esatl_dn10 = assign30780_e40948_d_n10;
        locals.var_esatl_dn11 = assign30780_e40948_d_n11;
        locals.var_esatl_dn12 = assign30780_e40948_d_n12;
        locals.var_esatl_dn13 = assign30780_e40948_d_n13;
        locals.var_esatl_dn14 = assign30780_e40948_d_n14;

        let (assign30790_e40954, assign30790_e40954_d_n0, assign30790_e40954_d_n2, assign30790_e40954_d_n3, assign30790_e40954_d_n4, assign30790_e40954_d_n5, assign30790_e40954_d_n6, assign30790_e40954_d_n7, assign30790_e40954_d_n8, assign30790_e40954_d_n9, assign30790_e40954_d_n10, assign30790_e40954_d_n11, assign30790_e40954_d_n12, assign30790_e40954_d_n13, assign30790_e40954_d_n14,) = {
    if (locals.var_guard704 != 0.0) {
        let assign30790_e40952: f64 = (locals.var_vdssatcv + locals.var_esatl);
        (assign30790_e40952, (locals.var_vdssatcv_dn0 + locals.var_esatl_dn0), (locals.var_vdssatcv_dn2 + locals.var_esatl_dn2), (locals.var_vdssatcv_dn3 + locals.var_esatl_dn3), (locals.var_vdssatcv_dn4 + locals.var_esatl_dn4), (locals.var_vdssatcv_dn5 + locals.var_esatl_dn5), (locals.var_vdssatcv_dn6 + locals.var_esatl_dn6), (locals.var_vdssatcv_dn7 + locals.var_esatl_dn7), (locals.var_vdssatcv_dn8 + locals.var_esatl_dn8), (locals.var_vdssatcv_dn9 + locals.var_esatl_dn9), (locals.var_vdssatcv_dn10 + locals.var_esatl_dn10), (locals.var_vdssatcv_dn11 + locals.var_esatl_dn11), (locals.var_vdssatcv_dn12 + locals.var_esatl_dn12), (locals.var_vdssatcv_dn13 + locals.var_esatl_dn13), (locals.var_vdssatcv_dn14 + locals.var_esatl_dn14),)
    } else {
        (locals.var_vasat, locals.var_vasat_dn0, locals.var_vasat_dn2, locals.var_vasat_dn3, locals.var_vasat_dn4, locals.var_vasat_dn5, locals.var_vasat_dn6, locals.var_vasat_dn7, locals.var_vasat_dn8, locals.var_vasat_dn9, locals.var_vasat_dn10, locals.var_vasat_dn11, locals.var_vasat_dn12, locals.var_vasat_dn13, locals.var_vasat_dn14,)
    }
};
        locals.var_vasat = assign30790_e40954;
        locals.var_vasat_dn0 = assign30790_e40954_d_n0;
        locals.var_vasat_dn2 = assign30790_e40954_d_n2;
        locals.var_vasat_dn3 = assign30790_e40954_d_n3;
        locals.var_vasat_dn4 = assign30790_e40954_d_n4;
        locals.var_vasat_dn5 = assign30790_e40954_d_n5;
        locals.var_vasat_dn6 = assign30790_e40954_d_n6;
        locals.var_vasat_dn7 = assign30790_e40954_d_n7;
        locals.var_vasat_dn8 = assign30790_e40954_d_n8;
        locals.var_vasat_dn9 = assign30790_e40954_d_n9;
        locals.var_vasat_dn10 = assign30790_e40954_d_n10;
        locals.var_vasat_dn11 = assign30790_e40954_d_n11;
        locals.var_vasat_dn12 = assign30790_e40954_d_n12;
        locals.var_vasat_dn13 = assign30790_e40954_d_n13;
        locals.var_vasat_dn14 = assign30790_e40954_d_n14;

        let (assign30800_e40960, assign30800_e40960_d_n0, assign30800_e40960_d_n2, assign30800_e40960_d_n3, assign30800_e40960_d_n4, assign30800_e40960_d_n5, assign30800_e40960_d_n6, assign30800_e40960_d_n7, assign30800_e40960_d_n8, assign30800_e40960_d_n9, assign30800_e40960_d_n10, assign30800_e40960_d_n11, assign30800_e40960_d_n12, assign30800_e40960_d_n13, assign30800_e40960_d_n14,) = {
    if (locals.var_guard704 != 0.0) {
        let assign30800_e40958: f64 = (locals.var_vdscv - locals.var_vdseff);
        (assign30800_e40958, (-locals.var_vdseff_dn0), (-locals.var_vdseff_dn2), (-locals.var_vdseff_dn3), (-locals.var_vdseff_dn4), (locals.var_vdscv_dn5 - locals.var_vdseff_dn5), (locals.var_vdscv_dn6 - locals.var_vdseff_dn6), (locals.var_vdscv_dn7 - locals.var_vdseff_dn7), (-locals.var_vdseff_dn8), (-locals.var_vdseff_dn9), (-locals.var_vdseff_dn10), (locals.var_vdscv_dn11 - locals.var_vdseff_dn11), (-locals.var_vdseff_dn12), (-locals.var_vdseff_dn13), (-locals.var_vdseff_dn14),)
    } else {
        (locals.var_diffvds, locals.var_diffvds_dn0, locals.var_diffvds_dn2, locals.var_diffvds_dn3, locals.var_diffvds_dn4, locals.var_diffvds_dn5, locals.var_diffvds_dn6, locals.var_diffvds_dn7, locals.var_diffvds_dn8, locals.var_diffvds_dn9, locals.var_diffvds_dn10, locals.var_diffvds_dn11, locals.var_diffvds_dn12, locals.var_diffvds_dn13, locals.var_diffvds_dn14,)
    }
};
        locals.var_diffvds = assign30800_e40960;
        locals.var_diffvds_dn0 = assign30800_e40960_d_n0;
        locals.var_diffvds_dn2 = assign30800_e40960_d_n2;
        locals.var_diffvds_dn3 = assign30800_e40960_d_n3;
        locals.var_diffvds_dn4 = assign30800_e40960_d_n4;
        locals.var_diffvds_dn5 = assign30800_e40960_d_n5;
        locals.var_diffvds_dn6 = assign30800_e40960_d_n6;
        locals.var_diffvds_dn7 = assign30800_e40960_d_n7;
        locals.var_diffvds_dn8 = assign30800_e40960_d_n8;
        locals.var_diffvds_dn9 = assign30800_e40960_d_n9;
        locals.var_diffvds_dn10 = assign30800_e40960_d_n10;
        locals.var_diffvds_dn11 = assign30800_e40960_d_n11;
        locals.var_diffvds_dn12 = assign30800_e40960_d_n12;
        locals.var_diffvds_dn13 = assign30800_e40960_d_n13;
        locals.var_diffvds_dn14 = assign30800_e40960_d_n14;

        let assign30810_e40963: f64 = if locals.var_pclmcv_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard724 = assign30810_e40963;

        let (assign30820_e40980, assign30820_e40980_d_n0, assign30820_e40980_d_n2, assign30820_e40980_d_n3, assign30820_e40980_d_n4, assign30820_e40980_d_n5, assign30820_e40980_d_n6, assign30820_e40980_d_n7, assign30820_e40980_d_n8, assign30820_e40980_d_n9, assign30820_e40980_d_n10, assign30820_e40980_d_n11, assign30820_e40980_d_n12, assign30820_e40980_d_n13, assign30820_e40980_d_n14,) = {
    if (locals.var_guard724 != 0.0) {
        let assign30820_e40970: f64 = (locals.var_diffvds / locals.var_pclmcv_i);
        let assign30820_e40972: f64 = (assign30820_e40970 / locals.var_vasat);
        let assign30820_e40973: f64 = (1.0 + assign30820_e40972);
        let assign30820_e40975: f64 = (assign30820_e40973).max(1e-38);
        let assign30820_e40976: f64 = (assign30820_e40975).ln();
        let assign30820_e40977: f64 = (locals.var_pclmcv_i * assign30820_e40976);
        let assign30820_e40978: f64 = (1.0 + assign30820_e40977);
        (assign30820_e40978, (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn0 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn0)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn2 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn2)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn3 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn3)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn4 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn4)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn5 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn5)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn6 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn6)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn7 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn7)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn8 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn8)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn9 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn9)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn10 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn10)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn11 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn11)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn12 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn12)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn13 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn13)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)), (locals.var_pclmcv_i * (if assign30820_e40973 >= 1e-38 { ((((locals.var_diffvds_dn14 / locals.var_pclmcv_i) * locals.var_vasat) - (assign30820_e40970 * locals.var_vasat_dn14)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign30820_e40975)),)
    } else {
        (locals.var_mdl, locals.var_mdl_dn0, locals.var_mdl_dn2, locals.var_mdl_dn3, locals.var_mdl_dn4, locals.var_mdl_dn5, locals.var_mdl_dn6, locals.var_mdl_dn7, locals.var_mdl_dn8, locals.var_mdl_dn9, locals.var_mdl_dn10, locals.var_mdl_dn11, locals.var_mdl_dn12, locals.var_mdl_dn13, locals.var_mdl_dn14,)
    }
};
        locals.var_mdl = assign30820_e40980;
        locals.var_mdl_dn0 = assign30820_e40980_d_n0;
        locals.var_mdl_dn2 = assign30820_e40980_d_n2;
        locals.var_mdl_dn3 = assign30820_e40980_d_n3;
        locals.var_mdl_dn4 = assign30820_e40980_d_n4;
        locals.var_mdl_dn5 = assign30820_e40980_d_n5;
        locals.var_mdl_dn6 = assign30820_e40980_d_n6;
        locals.var_mdl_dn7 = assign30820_e40980_d_n7;
        locals.var_mdl_dn8 = assign30820_e40980_d_n8;
        locals.var_mdl_dn9 = assign30820_e40980_d_n9;
        locals.var_mdl_dn10 = assign30820_e40980_d_n10;
        locals.var_mdl_dn11 = assign30820_e40980_d_n11;
        locals.var_mdl_dn12 = assign30820_e40980_d_n12;
        locals.var_mdl_dn13 = assign30820_e40980_d_n13;
        locals.var_mdl_dn14 = assign30820_e40980_d_n14;

        let (assign30830_e40985, assign30830_e40985_d_n0, assign30830_e40985_d_n2, assign30830_e40985_d_n3, assign30830_e40985_d_n4, assign30830_e40985_d_n5, assign30830_e40985_d_n6, assign30830_e40985_d_n7, assign30830_e40985_d_n8, assign30830_e40985_d_n9, assign30830_e40985_d_n10, assign30830_e40985_d_n11, assign30830_e40985_d_n12, assign30830_e40985_d_n13, assign30830_e40985_d_n14,) = {
    if (locals.var_guard724 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mdl, locals.var_mdl_dn0, locals.var_mdl_dn2, locals.var_mdl_dn3, locals.var_mdl_dn4, locals.var_mdl_dn5, locals.var_mdl_dn6, locals.var_mdl_dn7, locals.var_mdl_dn8, locals.var_mdl_dn9, locals.var_mdl_dn10, locals.var_mdl_dn11, locals.var_mdl_dn12, locals.var_mdl_dn13, locals.var_mdl_dn14,)
    }
};
        locals.var_mdl = assign30830_e40985;
        locals.var_mdl_dn0 = assign30830_e40985_d_n0;
        locals.var_mdl_dn2 = assign30830_e40985_d_n2;
        locals.var_mdl_dn3 = assign30830_e40985_d_n3;
        locals.var_mdl_dn4 = assign30830_e40985_d_n4;
        locals.var_mdl_dn5 = assign30830_e40985_d_n5;
        locals.var_mdl_dn6 = assign30830_e40985_d_n6;
        locals.var_mdl_dn7 = assign30830_e40985_d_n7;
        locals.var_mdl_dn8 = assign30830_e40985_d_n8;
        locals.var_mdl_dn9 = assign30830_e40985_d_n9;
        locals.var_mdl_dn10 = assign30830_e40985_d_n10;
        locals.var_mdl_dn11 = assign30830_e40985_d_n11;
        locals.var_mdl_dn12 = assign30830_e40985_d_n12;
        locals.var_mdl_dn13 = assign30830_e40985_d_n13;
        locals.var_mdl_dn14 = assign30830_e40985_d_n14;

        let assign30840_e40988: f64 = (locals.var_mdl * locals.var_mdl);
        locals.var_mdl_2 = assign30840_e40988;
        locals.var_mdl_2_dn0 = ((locals.var_mdl_dn0 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn0));
        locals.var_mdl_2_dn2 = ((locals.var_mdl_dn2 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn2));
        locals.var_mdl_2_dn3 = ((locals.var_mdl_dn3 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn3));
        locals.var_mdl_2_dn4 = ((locals.var_mdl_dn4 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn4));
        locals.var_mdl_2_dn5 = ((locals.var_mdl_dn5 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn5));
        locals.var_mdl_2_dn6 = ((locals.var_mdl_dn6 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn6));
        locals.var_mdl_2_dn7 = ((locals.var_mdl_dn7 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn7));
        locals.var_mdl_2_dn8 = ((locals.var_mdl_dn8 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn8));
        locals.var_mdl_2_dn9 = ((locals.var_mdl_dn9 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn9));
        locals.var_mdl_2_dn10 = ((locals.var_mdl_dn10 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn10));
        locals.var_mdl_2_dn11 = ((locals.var_mdl_dn11 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn11));
        locals.var_mdl_2_dn12 = ((locals.var_mdl_dn12 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn12));
        locals.var_mdl_2_dn13 = ((locals.var_mdl_dn13 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn13));
        locals.var_mdl_2_dn14 = ((locals.var_mdl_dn14 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn14));

        let assign30850_e40991: f64 = (1.0 / locals.var_mdl);
        locals.var_inv_mdl = assign30850_e40991;
        locals.var_inv_mdl_dn0 = (-(locals.var_mdl_dn0 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn2 = (-(locals.var_mdl_dn2 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn3 = (-(locals.var_mdl_dn3 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn4 = (-(locals.var_mdl_dn4 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn5 = (-(locals.var_mdl_dn5 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn6 = (-(locals.var_mdl_dn6 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn7 = (-(locals.var_mdl_dn7 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn8 = (-(locals.var_mdl_dn8 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn9 = (-(locals.var_mdl_dn9 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn10 = (-(locals.var_mdl_dn10 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn11 = (-(locals.var_mdl_dn11 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn12 = (-(locals.var_mdl_dn12 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn13 = (-(locals.var_mdl_dn13 / (locals.var_mdl * locals.var_mdl)));
        locals.var_inv_mdl_dn14 = (-(locals.var_mdl_dn14 / (locals.var_mdl * locals.var_mdl)));

        let assign30860_e40994: f64 = (1.0 / locals.var_mdl_2);
        locals.var_inv_mdl_2 = assign30860_e40994;
        locals.var_inv_mdl_2_dn0 = (-(locals.var_mdl_2_dn0 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn2 = (-(locals.var_mdl_2_dn2 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn3 = (-(locals.var_mdl_2_dn3 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn4 = (-(locals.var_mdl_2_dn4 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn5 = (-(locals.var_mdl_2_dn5 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn6 = (-(locals.var_mdl_2_dn6 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn7 = (-(locals.var_mdl_2_dn7 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn8 = (-(locals.var_mdl_2_dn8 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn9 = (-(locals.var_mdl_2_dn9 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn10 = (-(locals.var_mdl_2_dn10 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn11 = (-(locals.var_mdl_2_dn11 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn12 = (-(locals.var_mdl_2_dn12 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn13 = (-(locals.var_mdl_2_dn13 / (locals.var_mdl_2 * locals.var_mdl_2)));
        locals.var_inv_mdl_2_dn14 = (-(locals.var_mdl_2_dn14 / (locals.var_mdl_2 * locals.var_mdl_2)));

        let assign30870_e40997: f64 = (locals.var_mdl - 1.0);
        locals.var_mdl_less_1 = assign30870_e40997;
        locals.var_mdl_less_1_dn0 = locals.var_mdl_dn0;
        locals.var_mdl_less_1_dn2 = locals.var_mdl_dn2;
        locals.var_mdl_less_1_dn3 = locals.var_mdl_dn3;
        locals.var_mdl_less_1_dn4 = locals.var_mdl_dn4;
        locals.var_mdl_less_1_dn5 = locals.var_mdl_dn5;
        locals.var_mdl_less_1_dn6 = locals.var_mdl_dn6;
        locals.var_mdl_less_1_dn7 = locals.var_mdl_dn7;
        locals.var_mdl_less_1_dn8 = locals.var_mdl_dn8;
        locals.var_mdl_less_1_dn9 = locals.var_mdl_dn9;
        locals.var_mdl_less_1_dn10 = locals.var_mdl_dn10;
        locals.var_mdl_less_1_dn11 = locals.var_mdl_dn11;
        locals.var_mdl_less_1_dn12 = locals.var_mdl_dn12;
        locals.var_mdl_less_1_dn13 = locals.var_mdl_dn13;
        locals.var_mdl_less_1_dn14 = locals.var_mdl_dn14;

        let assign30880_e41000: f64 = (locals.var_vgfbcv - locals.var_psip);
        locals.var_vgpqm = assign30880_e41000;
        locals.var_vgpqm_dn0 = (locals.var_vgfbcv_dn0 - locals.var_psip_dn0);
        locals.var_vgpqm_dn2 = (locals.var_vgfbcv_dn2 - locals.var_psip_dn2);
        locals.var_vgpqm_dn3 = (locals.var_vgfbcv_dn3 - locals.var_psip_dn3);
        locals.var_vgpqm_dn4 = (locals.var_vgfbcv_dn4 - locals.var_psip_dn4);
        locals.var_vgpqm_dn5 = (locals.var_vgfbcv_dn5 - locals.var_psip_dn5);
        locals.var_vgpqm_dn6 = (locals.var_vgfbcv_dn6 - locals.var_psip_dn6);
        locals.var_vgpqm_dn7 = (locals.var_vgfbcv_dn7 - locals.var_psip_dn7);
        locals.var_vgpqm_dn8 = (locals.var_vgfbcv_dn8 - locals.var_psip_dn8);
        locals.var_vgpqm_dn9 = (locals.var_vgfbcv_dn9 - locals.var_psip_dn9);
        locals.var_vgpqm_dn10 = (locals.var_vgfbcv_dn10 - locals.var_psip_dn10);
        locals.var_vgpqm_dn11 = (locals.var_vgfbcv_dn11 - locals.var_psip_dn11);
        locals.var_vgpqm_dn12 = (locals.var_vgfbcv_dn12 - locals.var_psip_dn12);
        locals.var_vgpqm_dn13 = (locals.var_vgfbcv_dn13 - locals.var_psip_dn13);
        locals.var_vgpqm_dn14 = (locals.var_vgfbcv_dn14 - locals.var_psip_dn14);

        let assign30890_e41003: f64 = (locals.var_qs_1 - locals.var_qdeff);
        locals.var_dqsd = assign30890_e41003;
        locals.var_dqsd_dn0 = (locals.var_qs_1_dn0 - locals.var_qdeff_dn0);
        locals.var_dqsd_dn2 = (locals.var_qs_1_dn2 - locals.var_qdeff_dn2);
        locals.var_dqsd_dn3 = (locals.var_qs_1_dn3 - locals.var_qdeff_dn3);
        locals.var_dqsd_dn4 = (locals.var_qs_1_dn4 - locals.var_qdeff_dn4);
        locals.var_dqsd_dn5 = (locals.var_qs_1_dn5 - locals.var_qdeff_dn5);
        locals.var_dqsd_dn6 = (locals.var_qs_1_dn6 - locals.var_qdeff_dn6);
        locals.var_dqsd_dn7 = (locals.var_qs_1_dn7 - locals.var_qdeff_dn7);
        locals.var_dqsd_dn8 = (locals.var_qs_1_dn8 - locals.var_qdeff_dn8);
        locals.var_dqsd_dn9 = (locals.var_qs_1_dn9 - locals.var_qdeff_dn9);
        locals.var_dqsd_dn10 = (locals.var_qs_1_dn10 - locals.var_qdeff_dn10);
        locals.var_dqsd_dn11 = (locals.var_qs_1_dn11 - locals.var_qdeff_dn11);
        locals.var_dqsd_dn12 = (locals.var_qs_1_dn12 - locals.var_qdeff_dn12);
        locals.var_dqsd_dn13 = (locals.var_qs_1_dn13 - locals.var_qdeff_dn13);
        locals.var_dqsd_dn14 = (locals.var_qs_1_dn14 - locals.var_qdeff_dn14);

        let assign30900_e41006: f64 = (locals.var_qs_1 - locals.var_qdeff);
        let assign30900_e41009: f64 = (locals.var_qs_1 - locals.var_qdeff);
        let assign30900_e41010: f64 = (assign30900_e41006 * assign30900_e41009);
        locals.var_dqsd2 = assign30900_e41010;
        locals.var_dqsd2_dn0 = (((locals.var_qs_1_dn0 - locals.var_qdeff_dn0) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn0 - locals.var_qdeff_dn0)));
        locals.var_dqsd2_dn2 = (((locals.var_qs_1_dn2 - locals.var_qdeff_dn2) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn2 - locals.var_qdeff_dn2)));
        locals.var_dqsd2_dn3 = (((locals.var_qs_1_dn3 - locals.var_qdeff_dn3) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn3 - locals.var_qdeff_dn3)));
        locals.var_dqsd2_dn4 = (((locals.var_qs_1_dn4 - locals.var_qdeff_dn4) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn4 - locals.var_qdeff_dn4)));
        locals.var_dqsd2_dn5 = (((locals.var_qs_1_dn5 - locals.var_qdeff_dn5) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn5 - locals.var_qdeff_dn5)));
        locals.var_dqsd2_dn6 = (((locals.var_qs_1_dn6 - locals.var_qdeff_dn6) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn6 - locals.var_qdeff_dn6)));
        locals.var_dqsd2_dn7 = (((locals.var_qs_1_dn7 - locals.var_qdeff_dn7) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn7 - locals.var_qdeff_dn7)));
        locals.var_dqsd2_dn8 = (((locals.var_qs_1_dn8 - locals.var_qdeff_dn8) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn8 - locals.var_qdeff_dn8)));
        locals.var_dqsd2_dn9 = (((locals.var_qs_1_dn9 - locals.var_qdeff_dn9) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn9 - locals.var_qdeff_dn9)));
        locals.var_dqsd2_dn10 = (((locals.var_qs_1_dn10 - locals.var_qdeff_dn10) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn10 - locals.var_qdeff_dn10)));
        locals.var_dqsd2_dn11 = (((locals.var_qs_1_dn11 - locals.var_qdeff_dn11) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn11 - locals.var_qdeff_dn11)));
        locals.var_dqsd2_dn12 = (((locals.var_qs_1_dn12 - locals.var_qdeff_dn12) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn12 - locals.var_qdeff_dn12)));
        locals.var_dqsd2_dn13 = (((locals.var_qs_1_dn13 - locals.var_qdeff_dn13) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn13 - locals.var_qdeff_dn13)));
        locals.var_dqsd2_dn14 = (((locals.var_qs_1_dn14 - locals.var_qdeff_dn14) * assign30900_e41009) + (assign30900_e41006 * (locals.var_qs_1_dn14 - locals.var_qdeff_dn14)));

        let assign30910_e41014: f64 = (2.0 * locals.var_qs_1);
        let assign30910_e41015: f64 = (locals.var_vgpqm + assign30910_e41014);
        locals.var_sis = assign30910_e41015;
        locals.var_sis_dn0 = (locals.var_vgpqm_dn0 + (2.0 * locals.var_qs_1_dn0));
        locals.var_sis_dn2 = (locals.var_vgpqm_dn2 + (2.0 * locals.var_qs_1_dn2));
        locals.var_sis_dn3 = (locals.var_vgpqm_dn3 + (2.0 * locals.var_qs_1_dn3));
        locals.var_sis_dn4 = (locals.var_vgpqm_dn4 + (2.0 * locals.var_qs_1_dn4));
        locals.var_sis_dn5 = (locals.var_vgpqm_dn5 + (2.0 * locals.var_qs_1_dn5));
        locals.var_sis_dn6 = (locals.var_vgpqm_dn6 + (2.0 * locals.var_qs_1_dn6));
        locals.var_sis_dn7 = (locals.var_vgpqm_dn7 + (2.0 * locals.var_qs_1_dn7));
        locals.var_sis_dn8 = (locals.var_vgpqm_dn8 + (2.0 * locals.var_qs_1_dn8));
        locals.var_sis_dn9 = (locals.var_vgpqm_dn9 + (2.0 * locals.var_qs_1_dn9));
        locals.var_sis_dn10 = (locals.var_vgpqm_dn10 + (2.0 * locals.var_qs_1_dn10));
        locals.var_sis_dn11 = (locals.var_vgpqm_dn11 + (2.0 * locals.var_qs_1_dn11));
        locals.var_sis_dn12 = (locals.var_vgpqm_dn12 + (2.0 * locals.var_qs_1_dn12));
        locals.var_sis_dn13 = (locals.var_vgpqm_dn13 + (2.0 * locals.var_qs_1_dn13));
        locals.var_sis_dn14 = (locals.var_vgpqm_dn14 + (2.0 * locals.var_qs_1_dn14));

    }

    pub(super) fn stamp_transient_block_97(
        locals: &mut StampLocals,
    ) {
        let assign30920_e41019: f64 = (2.0 * locals.var_qdeff);
        let assign30920_e41020: f64 = (locals.var_vgpqm + assign30920_e41019);
        locals.var_sid = assign30920_e41020;
        locals.var_sid_dn0 = (locals.var_vgpqm_dn0 + (2.0 * locals.var_qdeff_dn0));
        locals.var_sid_dn2 = (locals.var_vgpqm_dn2 + (2.0 * locals.var_qdeff_dn2));
        locals.var_sid_dn3 = (locals.var_vgpqm_dn3 + (2.0 * locals.var_qdeff_dn3));
        locals.var_sid_dn4 = (locals.var_vgpqm_dn4 + (2.0 * locals.var_qdeff_dn4));
        locals.var_sid_dn5 = (locals.var_vgpqm_dn5 + (2.0 * locals.var_qdeff_dn5));
        locals.var_sid_dn6 = (locals.var_vgpqm_dn6 + (2.0 * locals.var_qdeff_dn6));
        locals.var_sid_dn7 = (locals.var_vgpqm_dn7 + (2.0 * locals.var_qdeff_dn7));
        locals.var_sid_dn8 = (locals.var_vgpqm_dn8 + (2.0 * locals.var_qdeff_dn8));
        locals.var_sid_dn9 = (locals.var_vgpqm_dn9 + (2.0 * locals.var_qdeff_dn9));
        locals.var_sid_dn10 = (locals.var_vgpqm_dn10 + (2.0 * locals.var_qdeff_dn10));
        locals.var_sid_dn11 = (locals.var_vgpqm_dn11 + (2.0 * locals.var_qdeff_dn11));
        locals.var_sid_dn12 = (locals.var_vgpqm_dn12 + (2.0 * locals.var_qdeff_dn12));
        locals.var_sid_dn13 = (locals.var_vgpqm_dn13 + (2.0 * locals.var_qdeff_dn13));
        locals.var_sid_dn14 = (locals.var_vgpqm_dn14 + (2.0 * locals.var_qdeff_dn14));

        let assign30930_e41026: f64 = (-2500.0);
        let assign30930_e41028: f64 = (assign30930_e41026 * 0.5);
        let assign30930_e41030: f64 = if ((0.0 == 0.0) && (locals.var_sis < assign30930_e41028)) { 1.0 } else { 0.0 };
        locals.var_guard725 = assign30930_e41030;

        let (assign30940_e41041, assign30940_e41041_d_n0, assign30940_e41041_d_n2, assign30940_e41041_d_n3, assign30940_e41041_d_n4, assign30940_e41041_d_n5, assign30940_e41041_d_n6, assign30940_e41041_d_n7, assign30940_e41041_d_n8, assign30940_e41041_d_n9, assign30940_e41041_d_n10, assign30940_e41041_d_n11, assign30940_e41041_d_n12, assign30940_e41041_d_n13, assign30940_e41041_d_n14,) = {
    if (locals.var_guard725 != 0.0) {
        let assign30940_e41033: f64 = (-0.5);
        let assign30940_e41035: f64 = (assign30940_e41033 * 0.5);
        let assign30940_e41038: f64 = (16.0 * locals.var_sis);
        let assign30940_e41039: f64 = (assign30940_e41035 / assign30940_e41038);
        (assign30940_e41039, (-((assign30940_e41035 * (16.0 * locals.var_sis_dn0)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn2)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn3)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn4)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn5)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn6)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn7)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn8)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn9)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn10)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn11)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn12)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn13)) / (assign30940_e41038 * assign30940_e41038))), (-((assign30940_e41035 * (16.0 * locals.var_sis_dn14)) / (assign30940_e41038 * assign30940_e41038))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign30940_e41041;
        locals.var_t1_dn0 = assign30940_e41041_d_n0;
        locals.var_t1_dn2 = assign30940_e41041_d_n2;
        locals.var_t1_dn3 = assign30940_e41041_d_n3;
        locals.var_t1_dn4 = assign30940_e41041_d_n4;
        locals.var_t1_dn5 = assign30940_e41041_d_n5;
        locals.var_t1_dn6 = assign30940_e41041_d_n6;
        locals.var_t1_dn7 = assign30940_e41041_d_n7;
        locals.var_t1_dn8 = assign30940_e41041_d_n8;
        locals.var_t1_dn9 = assign30940_e41041_d_n9;
        locals.var_t1_dn10 = assign30940_e41041_d_n10;
        locals.var_t1_dn11 = assign30940_e41041_d_n11;
        locals.var_t1_dn12 = assign30940_e41041_d_n12;
        locals.var_t1_dn13 = assign30940_e41041_d_n13;
        locals.var_t1_dn14 = assign30940_e41041_d_n14;

        let (assign30950_e41065, assign30950_e41065_d_n0, assign30950_e41065_d_n2, assign30950_e41065_d_n3, assign30950_e41065_d_n4, assign30950_e41065_d_n5, assign30950_e41065_d_n6, assign30950_e41065_d_n7, assign30950_e41065_d_n8, assign30950_e41065_d_n9, assign30950_e41065_d_n10, assign30950_e41065_d_n11, assign30950_e41065_d_n12, assign30950_e41065_d_n13, assign30950_e41065_d_n14,) = {
    if (locals.var_guard725 == 0.0) {
        let assign30950_e41047: f64 = locals.var_sis;
        let assign30950_e41050: f64 = locals.var_sis;
        let assign30950_e41053: f64 = locals.var_sis;
        let assign30950_e41054: f64 = (assign30950_e41050 * assign30950_e41053);
        let assign30950_e41057: f64 = (0.25 * 0.5);
        let assign30950_e41059: f64 = (assign30950_e41057 * 0.5);
        let assign30950_e41060: f64 = (assign30950_e41054 + assign30950_e41059);
        let assign30950_e41061: f64 = (assign30950_e41060).sqrt();
        let assign30950_e41062: f64 = (assign30950_e41047 + assign30950_e41061);
        let assign30950_e41063: f64 = (0.5 * assign30950_e41062);
        (assign30950_e41063, (0.5 * (locals.var_sis_dn0 + (((locals.var_sis_dn0 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn0)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn2 + (((locals.var_sis_dn2 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn2)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn3 + (((locals.var_sis_dn3 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn3)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn4 + (((locals.var_sis_dn4 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn4)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn5 + (((locals.var_sis_dn5 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn5)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn6 + (((locals.var_sis_dn6 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn6)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn7 + (((locals.var_sis_dn7 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn7)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn8 + (((locals.var_sis_dn8 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn8)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn9 + (((locals.var_sis_dn9 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn9)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn10 + (((locals.var_sis_dn10 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn10)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn11 + (((locals.var_sis_dn11 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn11)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn12 + (((locals.var_sis_dn12 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn12)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn13 + (((locals.var_sis_dn13 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn13)) / (2.0 * assign30950_e41061)))), (0.5 * (locals.var_sis_dn14 + (((locals.var_sis_dn14 * assign30950_e41053) + (assign30950_e41050 * locals.var_sis_dn14)) / (2.0 * assign30950_e41061)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign30950_e41065;
        locals.var_t1_dn0 = assign30950_e41065_d_n0;
        locals.var_t1_dn2 = assign30950_e41065_d_n2;
        locals.var_t1_dn3 = assign30950_e41065_d_n3;
        locals.var_t1_dn4 = assign30950_e41065_d_n4;
        locals.var_t1_dn5 = assign30950_e41065_d_n5;
        locals.var_t1_dn6 = assign30950_e41065_d_n6;
        locals.var_t1_dn7 = assign30950_e41065_d_n7;
        locals.var_t1_dn8 = assign30950_e41065_d_n8;
        locals.var_t1_dn9 = assign30950_e41065_d_n9;
        locals.var_t1_dn10 = assign30950_e41065_d_n10;
        locals.var_t1_dn11 = assign30950_e41065_d_n11;
        locals.var_t1_dn12 = assign30950_e41065_d_n12;
        locals.var_t1_dn13 = assign30950_e41065_d_n13;
        locals.var_t1_dn14 = assign30950_e41065_d_n14;

        let assign30960_e41071: f64 = (-2500.0);
        let assign30960_e41073: f64 = (assign30960_e41071 * 0.5);
        let assign30960_e41075: f64 = if ((0.0 == 0.0) && (locals.var_sid < assign30960_e41073)) { 1.0 } else { 0.0 };
        locals.var_guard726 = assign30960_e41075;

        let (assign30970_e41086, assign30970_e41086_d_n0, assign30970_e41086_d_n2, assign30970_e41086_d_n3, assign30970_e41086_d_n4, assign30970_e41086_d_n5, assign30970_e41086_d_n6, assign30970_e41086_d_n7, assign30970_e41086_d_n8, assign30970_e41086_d_n9, assign30970_e41086_d_n10, assign30970_e41086_d_n11, assign30970_e41086_d_n12, assign30970_e41086_d_n13, assign30970_e41086_d_n14,) = {
    if (locals.var_guard726 != 0.0) {
        let assign30970_e41078: f64 = (-0.5);
        let assign30970_e41080: f64 = (assign30970_e41078 * 0.5);
        let assign30970_e41083: f64 = (16.0 * locals.var_sid);
        let assign30970_e41084: f64 = (assign30970_e41080 / assign30970_e41083);
        (assign30970_e41084, (-((assign30970_e41080 * (16.0 * locals.var_sid_dn0)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn2)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn3)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn4)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn5)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn6)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn7)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn8)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn9)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn10)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn11)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn12)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn13)) / (assign30970_e41083 * assign30970_e41083))), (-((assign30970_e41080 * (16.0 * locals.var_sid_dn14)) / (assign30970_e41083 * assign30970_e41083))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign30970_e41086;
        locals.var_t2_dn0 = assign30970_e41086_d_n0;
        locals.var_t2_dn2 = assign30970_e41086_d_n2;
        locals.var_t2_dn3 = assign30970_e41086_d_n3;
        locals.var_t2_dn4 = assign30970_e41086_d_n4;
        locals.var_t2_dn5 = assign30970_e41086_d_n5;
        locals.var_t2_dn6 = assign30970_e41086_d_n6;
        locals.var_t2_dn7 = assign30970_e41086_d_n7;
        locals.var_t2_dn8 = assign30970_e41086_d_n8;
        locals.var_t2_dn9 = assign30970_e41086_d_n9;
        locals.var_t2_dn10 = assign30970_e41086_d_n10;
        locals.var_t2_dn11 = assign30970_e41086_d_n11;
        locals.var_t2_dn12 = assign30970_e41086_d_n12;
        locals.var_t2_dn13 = assign30970_e41086_d_n13;
        locals.var_t2_dn14 = assign30970_e41086_d_n14;

        let (assign30980_e41110, assign30980_e41110_d_n0, assign30980_e41110_d_n2, assign30980_e41110_d_n3, assign30980_e41110_d_n4, assign30980_e41110_d_n5, assign30980_e41110_d_n6, assign30980_e41110_d_n7, assign30980_e41110_d_n8, assign30980_e41110_d_n9, assign30980_e41110_d_n10, assign30980_e41110_d_n11, assign30980_e41110_d_n12, assign30980_e41110_d_n13, assign30980_e41110_d_n14,) = {
    if (locals.var_guard726 == 0.0) {
        let assign30980_e41092: f64 = locals.var_sid;
        let assign30980_e41095: f64 = locals.var_sid;
        let assign30980_e41098: f64 = locals.var_sid;
        let assign30980_e41099: f64 = (assign30980_e41095 * assign30980_e41098);
        let assign30980_e41102: f64 = (0.25 * 0.5);
        let assign30980_e41104: f64 = (assign30980_e41102 * 0.5);
        let assign30980_e41105: f64 = (assign30980_e41099 + assign30980_e41104);
        let assign30980_e41106: f64 = (assign30980_e41105).sqrt();
        let assign30980_e41107: f64 = (assign30980_e41092 + assign30980_e41106);
        let assign30980_e41108: f64 = (0.5 * assign30980_e41107);
        (assign30980_e41108, (0.5 * (locals.var_sid_dn0 + (((locals.var_sid_dn0 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn0)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn2 + (((locals.var_sid_dn2 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn2)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn3 + (((locals.var_sid_dn3 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn3)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn4 + (((locals.var_sid_dn4 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn4)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn5 + (((locals.var_sid_dn5 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn5)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn6 + (((locals.var_sid_dn6 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn6)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn7 + (((locals.var_sid_dn7 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn7)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn8 + (((locals.var_sid_dn8 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn8)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn9 + (((locals.var_sid_dn9 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn9)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn10 + (((locals.var_sid_dn10 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn10)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn11 + (((locals.var_sid_dn11 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn11)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn12 + (((locals.var_sid_dn12 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn12)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn13 + (((locals.var_sid_dn13 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn13)) / (2.0 * assign30980_e41106)))), (0.5 * (locals.var_sid_dn14 + (((locals.var_sid_dn14 * assign30980_e41098) + (assign30980_e41095 * locals.var_sid_dn14)) / (2.0 * assign30980_e41106)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign30980_e41110;
        locals.var_t2_dn0 = assign30980_e41110_d_n0;
        locals.var_t2_dn2 = assign30980_e41110_d_n2;
        locals.var_t2_dn3 = assign30980_e41110_d_n3;
        locals.var_t2_dn4 = assign30980_e41110_d_n4;
        locals.var_t2_dn5 = assign30980_e41110_d_n5;
        locals.var_t2_dn6 = assign30980_e41110_d_n6;
        locals.var_t2_dn7 = assign30980_e41110_d_n7;
        locals.var_t2_dn8 = assign30980_e41110_d_n8;
        locals.var_t2_dn9 = assign30980_e41110_d_n9;
        locals.var_t2_dn10 = assign30980_e41110_d_n10;
        locals.var_t2_dn11 = assign30980_e41110_d_n11;
        locals.var_t2_dn12 = assign30980_e41110_d_n12;
        locals.var_t2_dn13 = assign30980_e41110_d_n13;
        locals.var_t2_dn14 = assign30980_e41110_d_n14;

        let assign30990_e41114: f64 = (locals.var_t1 * locals.var_invgamg2);
        let assign30990_e41115: f64 = (0.25 + assign30990_e41114);
        let assign30990_e41116: f64 = (assign30990_e41115).sqrt();
        locals.var_temps = assign30990_e41116;
        locals.var_temps_dn0 = ((locals.var_t1_dn0 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn2 = ((locals.var_t1_dn2 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn3 = ((locals.var_t1_dn3 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn4 = (((locals.var_t1_dn4 * locals.var_invgamg2) + (locals.var_t1 * locals.var_invgamg2_dn4)) / (2.0 * assign30990_e41116));
        locals.var_temps_dn5 = ((locals.var_t1_dn5 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn6 = ((locals.var_t1_dn6 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn7 = ((locals.var_t1_dn7 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn8 = ((locals.var_t1_dn8 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn9 = ((locals.var_t1_dn9 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn10 = ((locals.var_t1_dn10 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn11 = ((locals.var_t1_dn11 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn12 = ((locals.var_t1_dn12 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn13 = ((locals.var_t1_dn13 * locals.var_invgamg2) / (2.0 * assign30990_e41116));
        locals.var_temps_dn14 = ((locals.var_t1_dn14 * locals.var_invgamg2) / (2.0 * assign30990_e41116));

        let assign31000_e41120: f64 = (locals.var_t2 * locals.var_invgamg2);
        let assign31000_e41121: f64 = (0.25 + assign31000_e41120);
        let assign31000_e41122: f64 = (assign31000_e41121).sqrt();
        locals.var_tempd = assign31000_e41122;
        locals.var_tempd_dn0 = ((locals.var_t2_dn0 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn2 = ((locals.var_t2_dn2 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn3 = ((locals.var_t2_dn3 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn4 = (((locals.var_t2_dn4 * locals.var_invgamg2) + (locals.var_t2 * locals.var_invgamg2_dn4)) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn5 = ((locals.var_t2_dn5 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn6 = ((locals.var_t2_dn6 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn7 = ((locals.var_t2_dn7 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn8 = ((locals.var_t2_dn8 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn9 = ((locals.var_t2_dn9 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn10 = ((locals.var_t2_dn10 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn11 = ((locals.var_t2_dn11 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn12 = ((locals.var_t2_dn12 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn13 = ((locals.var_t2_dn13 * locals.var_invgamg2) / (2.0 * assign31000_e41122));
        locals.var_tempd_dn14 = ((locals.var_t2_dn14 * locals.var_invgamg2) / (2.0 * assign31000_e41122));

        let assign31010_e41127: f64 = (2.0 * locals.var_temps);
        let assign31010_e41128: f64 = (1.0 + assign31010_e41127);
        let assign31010_e41129: f64 = (locals.var_sis / assign31010_e41128);
        locals.var_t1 = assign31010_e41129;
        locals.var_t1_dn0 = (((locals.var_sis_dn0 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn0))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn2 = (((locals.var_sis_dn2 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn2))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn3 = (((locals.var_sis_dn3 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn3))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn4 = (((locals.var_sis_dn4 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn4))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn5 = (((locals.var_sis_dn5 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn5))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn6 = (((locals.var_sis_dn6 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn6))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn7 = (((locals.var_sis_dn7 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn7))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn8 = (((locals.var_sis_dn8 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn8))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn9 = (((locals.var_sis_dn9 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn9))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn10 = (((locals.var_sis_dn10 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn10))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn11 = (((locals.var_sis_dn11 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn11))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn12 = (((locals.var_sis_dn12 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn12))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn13 = (((locals.var_sis_dn13 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn13))) / (assign31010_e41128 * assign31010_e41128));
        locals.var_t1_dn14 = (((locals.var_sis_dn14 * assign31010_e41128) - (locals.var_sis * (2.0 * locals.var_temps_dn14))) / (assign31010_e41128 * assign31010_e41128));

        let assign31020_e41134: f64 = (2.0 * locals.var_tempd);
        let assign31020_e41135: f64 = (1.0 + assign31020_e41134);
        let assign31020_e41136: f64 = (locals.var_sid / assign31020_e41135);
        locals.var_t2 = assign31020_e41136;
        locals.var_t2_dn0 = (((locals.var_sid_dn0 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn0))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn2 = (((locals.var_sid_dn2 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn2))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn3 = (((locals.var_sid_dn3 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn3))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn4 = (((locals.var_sid_dn4 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn4))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn5 = (((locals.var_sid_dn5 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn5))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn6 = (((locals.var_sid_dn6 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn6))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn7 = (((locals.var_sid_dn7 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn7))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn8 = (((locals.var_sid_dn8 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn8))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn9 = (((locals.var_sid_dn9 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn9))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn10 = (((locals.var_sid_dn10 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn10))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn11 = (((locals.var_sid_dn11 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn11))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn12 = (((locals.var_sid_dn12 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn12))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn13 = (((locals.var_sid_dn13 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn13))) / (assign31020_e41135 * assign31020_e41135));
        locals.var_t2_dn14 = (((locals.var_sid_dn14 * assign31020_e41135) - (locals.var_sid * (2.0 * locals.var_tempd_dn14))) / (assign31020_e41135 * assign31020_e41135));

        let assign31030_e41139: f64 = (locals.var_temps + locals.var_tempd);
        locals.var_t3 = assign31030_e41139;
        locals.var_t3_dn0 = (locals.var_temps_dn0 + locals.var_tempd_dn0);
        locals.var_t3_dn2 = (locals.var_temps_dn2 + locals.var_tempd_dn2);
        locals.var_t3_dn3 = (locals.var_temps_dn3 + locals.var_tempd_dn3);
        locals.var_t3_dn4 = (locals.var_temps_dn4 + locals.var_tempd_dn4);
        locals.var_t3_dn5 = (locals.var_temps_dn5 + locals.var_tempd_dn5);
        locals.var_t3_dn6 = (locals.var_temps_dn6 + locals.var_tempd_dn6);
        locals.var_t3_dn7 = (locals.var_temps_dn7 + locals.var_tempd_dn7);
        locals.var_t3_dn8 = (locals.var_temps_dn8 + locals.var_tempd_dn8);
        locals.var_t3_dn9 = (locals.var_temps_dn9 + locals.var_tempd_dn9);
        locals.var_t3_dn10 = (locals.var_temps_dn10 + locals.var_tempd_dn10);
        locals.var_t3_dn11 = (locals.var_temps_dn11 + locals.var_tempd_dn11);
        locals.var_t3_dn12 = (locals.var_temps_dn12 + locals.var_tempd_dn12);
        locals.var_t3_dn13 = (locals.var_temps_dn13 + locals.var_tempd_dn13);
        locals.var_t3_dn14 = (locals.var_temps_dn14 + locals.var_tempd_dn14);

        let assign31040_e41144: f64 = (locals.var_t3 * locals.var_t3);
        let assign31040_e41146: f64 = (assign31040_e41144 * locals.var_t3);
        let assign31040_e41147: f64 = (locals.var_dqsd2 / assign31040_e41146);
        let assign31040_e41148: f64 = (0.3333333333333333 * assign31040_e41147);
        locals.var_t4 = assign31040_e41148;
        locals.var_t4_dn0 = (0.3333333333333333 * (((locals.var_dqsd2_dn0 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn0)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn2 = (0.3333333333333333 * (((locals.var_dqsd2_dn2 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn2)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn3 = (0.3333333333333333 * (((locals.var_dqsd2_dn3 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn3)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn4 = (0.3333333333333333 * (((locals.var_dqsd2_dn4 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn4)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn5 = (0.3333333333333333 * (((locals.var_dqsd2_dn5 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn5)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn6 = (0.3333333333333333 * (((locals.var_dqsd2_dn6 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn6)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn7 = (0.3333333333333333 * (((locals.var_dqsd2_dn7 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn7)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn8 = (0.3333333333333333 * (((locals.var_dqsd2_dn8 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn8)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn9 = (0.3333333333333333 * (((locals.var_dqsd2_dn9 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn9)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn10 = (0.3333333333333333 * (((locals.var_dqsd2_dn10 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn10)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn11 = (0.3333333333333333 * (((locals.var_dqsd2_dn11 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn11)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn12 = (0.3333333333333333 * (((locals.var_dqsd2_dn12 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn12)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn13 = (0.3333333333333333 * (((locals.var_dqsd2_dn13 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn13)))) / (assign31040_e41146 * assign31040_e41146)));
        locals.var_t4_dn14 = (0.3333333333333333 * (((locals.var_dqsd2_dn14 * assign31040_e41146) - (locals.var_dqsd2 * ((((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) * locals.var_t3) + (assign31040_e41144 * locals.var_t3_dn14)))) / (assign31040_e41146 * assign31040_e41146)));

        let assign31050_e41151: f64 = (locals.var_abulkcv * locals.var_dvsat);
        let assign31050_e41153: f64 = (assign31050_e41151 * locals.var_inv_mdl);
        let assign31050_e41156: f64 = (1.0 + locals.var_qs_1);
        let assign31050_e41158: f64 = (assign31050_e41156 + locals.var_qdeff);
        let assign31050_e41159: f64 = (assign31050_e41153 / assign31050_e41158);
        locals.var_t5 = assign31050_e41159;
        locals.var_t5_dn0 = (((((((locals.var_abulkcv_dn0 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn0)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn0)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn0 + locals.var_qdeff_dn0))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn2 = (((((((locals.var_abulkcv_dn2 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn2)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn2)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn2 + locals.var_qdeff_dn2))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn3 = (((((((locals.var_abulkcv_dn3 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn3)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn3)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn4 = (((((((locals.var_abulkcv_dn4 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn4)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn4)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn5 = (((((((locals.var_abulkcv_dn5 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn5)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn5)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn6 = (((((((locals.var_abulkcv_dn6 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn6)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn6)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn7 = (((((((locals.var_abulkcv_dn7 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn7)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn7)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn8 = (((((((locals.var_abulkcv_dn8 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn8)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn8)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn9 = (((((((locals.var_abulkcv_dn9 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn9)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn9)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn10 = (((((((locals.var_abulkcv_dn10 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn10)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn10)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn11 = (((((((locals.var_abulkcv_dn11 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn11)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn11)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn12 = (((((((locals.var_abulkcv_dn12 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn12)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn12)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn12 + locals.var_qdeff_dn12))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn13 = (((((((locals.var_abulkcv_dn13 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn13)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn13)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn13 + locals.var_qdeff_dn13))) / (assign31050_e41158 * assign31050_e41158));
        locals.var_t5_dn14 = (((((((locals.var_abulkcv_dn14 * locals.var_dvsat) + (locals.var_abulkcv * locals.var_dvsat_dn14)) * locals.var_inv_mdl) + (assign31050_e41151 * locals.var_inv_mdl_dn14)) * assign31050_e41158) - (assign31050_e41153 * (locals.var_qs_1_dn14 + locals.var_qdeff_dn14))) / (assign31050_e41158 * assign31050_e41158));

        let assign31060_e41163: f64 = (locals.var_t3 * locals.var_t3);
        let assign31060_e41166: f64 = (locals.var_temps * locals.var_tempd);
        let assign31060_e41167: f64 = (assign31060_e41163 + assign31060_e41166);
        let assign31060_e41168: f64 = (0.8 * assign31060_e41167);
        let assign31060_e41170: f64 = (assign31060_e41168 * locals.var_t5);
        locals.var_t6 = assign31060_e41170;
        locals.var_t6_dn0 = (((0.8 * (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) + ((locals.var_temps_dn0 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn0)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn0));
        locals.var_t6_dn2 = (((0.8 * (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) + ((locals.var_temps_dn2 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn2)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn2));
        locals.var_t6_dn3 = (((0.8 * (((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3)) + ((locals.var_temps_dn3 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn3)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn3));
        locals.var_t6_dn4 = (((0.8 * (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) + ((locals.var_temps_dn4 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn4)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn4));
        locals.var_t6_dn5 = (((0.8 * (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) + ((locals.var_temps_dn5 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn5)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn5));
        locals.var_t6_dn6 = (((0.8 * (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) + ((locals.var_temps_dn6 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn6)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn6));
        locals.var_t6_dn7 = (((0.8 * (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) + ((locals.var_temps_dn7 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn7)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn7));
        locals.var_t6_dn8 = (((0.8 * (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) + ((locals.var_temps_dn8 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn8)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn8));
        locals.var_t6_dn9 = (((0.8 * (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) + ((locals.var_temps_dn9 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn9)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn9));
        locals.var_t6_dn10 = (((0.8 * (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) + ((locals.var_temps_dn10 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn10)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn10));
        locals.var_t6_dn11 = (((0.8 * (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) + ((locals.var_temps_dn11 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn11)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn11));
        locals.var_t6_dn12 = (((0.8 * (((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12)) + ((locals.var_temps_dn12 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn12)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn12));
        locals.var_t6_dn13 = (((0.8 * (((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13)) + ((locals.var_temps_dn13 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn13)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn13));
        locals.var_t6_dn14 = (((0.8 * (((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) + ((locals.var_temps_dn14 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn14)))) * locals.var_t5) + (assign31060_e41168 * locals.var_t5_dn14));

        let assign31070_e41174: f64 = (2.0 * locals.var_invgamg2);
        let assign31070_e41175: f64 = (locals.var_t6 + assign31070_e41174);
        locals.var_t7 = assign31070_e41175;
        locals.var_t7_dn0 = locals.var_t6_dn0;
        locals.var_t7_dn2 = locals.var_t6_dn2;
        locals.var_t7_dn3 = locals.var_t6_dn3;
        locals.var_t7_dn4 = (locals.var_t6_dn4 + (2.0 * locals.var_invgamg2_dn4));
        locals.var_t7_dn5 = locals.var_t6_dn5;
        locals.var_t7_dn6 = locals.var_t6_dn6;
        locals.var_t7_dn7 = locals.var_t6_dn7;
        locals.var_t7_dn8 = locals.var_t6_dn8;
        locals.var_t7_dn9 = locals.var_t6_dn9;
        locals.var_t7_dn10 = locals.var_t6_dn10;
        locals.var_t7_dn11 = locals.var_t6_dn11;
        locals.var_t7_dn12 = locals.var_t6_dn12;
        locals.var_t7_dn13 = locals.var_t6_dn13;
        locals.var_t7_dn14 = locals.var_t6_dn14;

        let assign31080_e41178: f64 = (0.3333333333333333 * locals.var_dqsd2);
        let assign31080_e41180: f64 = (assign31080_e41178 * locals.var_t5);
        locals.var_t8 = assign31080_e41180;
        locals.var_t8_dn0 = (((0.3333333333333333 * locals.var_dqsd2_dn0) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn0));
        locals.var_t8_dn2 = (((0.3333333333333333 * locals.var_dqsd2_dn2) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn2));
        locals.var_t8_dn3 = (((0.3333333333333333 * locals.var_dqsd2_dn3) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn3));
        locals.var_t8_dn4 = (((0.3333333333333333 * locals.var_dqsd2_dn4) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn4));
        locals.var_t8_dn5 = (((0.3333333333333333 * locals.var_dqsd2_dn5) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn5));
        locals.var_t8_dn6 = (((0.3333333333333333 * locals.var_dqsd2_dn6) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn6));
        locals.var_t8_dn7 = (((0.3333333333333333 * locals.var_dqsd2_dn7) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn7));
        locals.var_t8_dn8 = (((0.3333333333333333 * locals.var_dqsd2_dn8) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn8));
        locals.var_t8_dn9 = (((0.3333333333333333 * locals.var_dqsd2_dn9) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn9));
        locals.var_t8_dn10 = (((0.3333333333333333 * locals.var_dqsd2_dn10) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn10));
        locals.var_t8_dn11 = (((0.3333333333333333 * locals.var_dqsd2_dn11) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn11));
        locals.var_t8_dn12 = (((0.3333333333333333 * locals.var_dqsd2_dn12) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn12));
        locals.var_t8_dn13 = (((0.3333333333333333 * locals.var_dqsd2_dn13) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn13));
        locals.var_t8_dn14 = (((0.3333333333333333 * locals.var_dqsd2_dn14) * locals.var_t5) + (assign31080_e41178 * locals.var_t5_dn14));

        let assign31090_e41184: f64 = (2.0 * locals.var_tempd);
        let assign31090_e41186: f64 = (assign31090_e41184 - 1.0);
        let assign31090_e41187: f64 = (locals.var_sid * assign31090_e41186);
        let assign31090_e41190: f64 = (2.0 * locals.var_tempd);
        let assign31090_e41192: f64 = (assign31090_e41190 + 1.0);
        let assign31090_e41193: f64 = (assign31090_e41187 / assign31090_e41192);
        locals.var_dqgeff = assign31090_e41193;
        locals.var_dqgeff_dn0 = (((((locals.var_sid_dn0 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn0))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn0))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn2 = (((((locals.var_sid_dn2 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn2))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn2))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn3 = (((((locals.var_sid_dn3 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn3))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn3))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn4 = (((((locals.var_sid_dn4 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn4))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn4))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn5 = (((((locals.var_sid_dn5 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn5))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn5))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn6 = (((((locals.var_sid_dn6 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn6))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn6))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn7 = (((((locals.var_sid_dn7 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn7))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn7))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn8 = (((((locals.var_sid_dn8 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn8))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn8))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn9 = (((((locals.var_sid_dn9 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn9))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn9))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn10 = (((((locals.var_sid_dn10 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn10))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn10))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn11 = (((((locals.var_sid_dn11 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn11))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn11))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn12 = (((((locals.var_sid_dn12 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn12))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn12))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn13 = (((((locals.var_sid_dn13 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn13))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn13))) / (assign31090_e41192 * assign31090_e41192));
        locals.var_dqgeff_dn14 = (((((locals.var_sid_dn14 * assign31090_e41186) + (locals.var_sid * (2.0 * locals.var_tempd_dn14))) * assign31090_e41192) - (assign31090_e41187 * (2.0 * locals.var_tempd_dn14))) / (assign31090_e41192 * assign31090_e41192));

        let assign31100_e41198: f64 = (locals.var_nq - 1.0);
        let assign31100_e41199: f64 = (2.0 * assign31100_e41198);
        let assign31100_e41201: f64 = (assign31100_e41199 * locals.var_qdeff);
        let assign31100_e41202: f64 = (locals.var_vgpqm - assign31100_e41201);
        let assign31100_e41204: f64 = (assign31100_e41202 + locals.var_dqgeff);
        locals.var_qbeff = assign31100_e41204;
        locals.var_qbeff_dn0 = ((locals.var_vgpqm_dn0 - (((2.0 * locals.var_nq_dn0) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn0))) + locals.var_dqgeff_dn0);
        locals.var_qbeff_dn2 = ((locals.var_vgpqm_dn2 - (((2.0 * locals.var_nq_dn2) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn2))) + locals.var_dqgeff_dn2);
        locals.var_qbeff_dn3 = ((locals.var_vgpqm_dn3 - (((2.0 * locals.var_nq_dn3) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn3))) + locals.var_dqgeff_dn3);
        locals.var_qbeff_dn4 = ((locals.var_vgpqm_dn4 - (((2.0 * locals.var_nq_dn4) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn4))) + locals.var_dqgeff_dn4);
        locals.var_qbeff_dn5 = ((locals.var_vgpqm_dn5 - (((2.0 * locals.var_nq_dn5) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn5))) + locals.var_dqgeff_dn5);
        locals.var_qbeff_dn6 = ((locals.var_vgpqm_dn6 - (((2.0 * locals.var_nq_dn6) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn6))) + locals.var_dqgeff_dn6);
        locals.var_qbeff_dn7 = ((locals.var_vgpqm_dn7 - (((2.0 * locals.var_nq_dn7) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn7))) + locals.var_dqgeff_dn7);
        locals.var_qbeff_dn8 = ((locals.var_vgpqm_dn8 - (((2.0 * locals.var_nq_dn8) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn8))) + locals.var_dqgeff_dn8);
        locals.var_qbeff_dn9 = ((locals.var_vgpqm_dn9 - (((2.0 * locals.var_nq_dn9) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn9))) + locals.var_dqgeff_dn9);
        locals.var_qbeff_dn10 = ((locals.var_vgpqm_dn10 - (((2.0 * locals.var_nq_dn10) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn10))) + locals.var_dqgeff_dn10);
        locals.var_qbeff_dn11 = ((locals.var_vgpqm_dn11 - (((2.0 * locals.var_nq_dn11) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn11))) + locals.var_dqgeff_dn11);
        locals.var_qbeff_dn12 = ((locals.var_vgpqm_dn12 - (((2.0 * locals.var_nq_dn12) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn12))) + locals.var_dqgeff_dn12);
        locals.var_qbeff_dn13 = ((locals.var_vgpqm_dn13 - (((2.0 * locals.var_nq_dn13) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn13))) + locals.var_dqgeff_dn13);
        locals.var_qbeff_dn14 = ((locals.var_vgpqm_dn14 - (((2.0 * locals.var_nq_dn14) * locals.var_qdeff) + (assign31100_e41199 * locals.var_qdeff_dn14))) + locals.var_dqgeff_dn14);

        let assign31110_e41208: f64 = (locals.var_t1 + locals.var_t2);
        let assign31110_e41211: f64 = (locals.var_t4 * locals.var_t7);
        let assign31110_e41215: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign31110_e41217: f64 = (assign31110_e41215 + locals.var_t8);
        let assign31110_e41218: f64 = (locals.var_nq * assign31110_e41217);
        let assign31110_e41219: f64 = (assign31110_e41211 - assign31110_e41218);
        let assign31110_e41220: f64 = (assign31110_e41208 + assign31110_e41219);
        let assign31110_e41221: f64 = (locals.var_inv_mdl * assign31110_e41220);
        let assign31110_e41224: f64 = (locals.var_mdl_less_1 * locals.var_qbeff);
        let assign31110_e41225: f64 = (assign31110_e41221 + assign31110_e41224);
        locals.var_qb_1 = assign31110_e41225;
        locals.var_qb_1_dn0 = (((locals.var_inv_mdl_dn0 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn0 + locals.var_t2_dn0) + (((locals.var_t4_dn0 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn0)) - ((locals.var_nq_dn0 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn0 + locals.var_qdeff_dn0) + locals.var_t8_dn0))))))) + ((locals.var_mdl_less_1_dn0 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn0)));
        locals.var_qb_1_dn2 = (((locals.var_inv_mdl_dn2 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn2 + locals.var_t2_dn2) + (((locals.var_t4_dn2 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn2)) - ((locals.var_nq_dn2 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn2 + locals.var_qdeff_dn2) + locals.var_t8_dn2))))))) + ((locals.var_mdl_less_1_dn2 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn2)));
        locals.var_qb_1_dn3 = (((locals.var_inv_mdl_dn3 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn3 + locals.var_t2_dn3) + (((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) - ((locals.var_nq_dn3 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn3 + locals.var_qdeff_dn3) + locals.var_t8_dn3))))))) + ((locals.var_mdl_less_1_dn3 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn3)));
        locals.var_qb_1_dn4 = (((locals.var_inv_mdl_dn4 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn4 + locals.var_t2_dn4) + (((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) - ((locals.var_nq_dn4 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn4 + locals.var_qdeff_dn4) + locals.var_t8_dn4))))))) + ((locals.var_mdl_less_1_dn4 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn4)));
        locals.var_qb_1_dn5 = (((locals.var_inv_mdl_dn5 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn5 + locals.var_t2_dn5) + (((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) - ((locals.var_nq_dn5 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn5 + locals.var_qdeff_dn5) + locals.var_t8_dn5))))))) + ((locals.var_mdl_less_1_dn5 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn5)));
        locals.var_qb_1_dn6 = (((locals.var_inv_mdl_dn6 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn6 + locals.var_t2_dn6) + (((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) - ((locals.var_nq_dn6 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn6 + locals.var_qdeff_dn6) + locals.var_t8_dn6))))))) + ((locals.var_mdl_less_1_dn6 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn6)));
        locals.var_qb_1_dn7 = (((locals.var_inv_mdl_dn7 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn7 + locals.var_t2_dn7) + (((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) - ((locals.var_nq_dn7 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn7 + locals.var_qdeff_dn7) + locals.var_t8_dn7))))))) + ((locals.var_mdl_less_1_dn7 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn7)));
        locals.var_qb_1_dn8 = (((locals.var_inv_mdl_dn8 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn8 + locals.var_t2_dn8) + (((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) - ((locals.var_nq_dn8 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn8 + locals.var_qdeff_dn8) + locals.var_t8_dn8))))))) + ((locals.var_mdl_less_1_dn8 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn8)));
        locals.var_qb_1_dn9 = (((locals.var_inv_mdl_dn9 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn9 + locals.var_t2_dn9) + (((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) - ((locals.var_nq_dn9 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn9 + locals.var_qdeff_dn9) + locals.var_t8_dn9))))))) + ((locals.var_mdl_less_1_dn9 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn9)));
        locals.var_qb_1_dn10 = (((locals.var_inv_mdl_dn10 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn10 + locals.var_t2_dn10) + (((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) - ((locals.var_nq_dn10 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn10 + locals.var_qdeff_dn10) + locals.var_t8_dn10))))))) + ((locals.var_mdl_less_1_dn10 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn10)));
        locals.var_qb_1_dn11 = (((locals.var_inv_mdl_dn11 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn11 + locals.var_t2_dn11) + (((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) - ((locals.var_nq_dn11 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn11 + locals.var_qdeff_dn11) + locals.var_t8_dn11))))))) + ((locals.var_mdl_less_1_dn11 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn11)));
        locals.var_qb_1_dn12 = (((locals.var_inv_mdl_dn12 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn12 + locals.var_t2_dn12) + (((locals.var_t4_dn12 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn12)) - ((locals.var_nq_dn12 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn12 + locals.var_qdeff_dn12) + locals.var_t8_dn12))))))) + ((locals.var_mdl_less_1_dn12 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn12)));
        locals.var_qb_1_dn13 = (((locals.var_inv_mdl_dn13 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn13 + locals.var_t2_dn13) + (((locals.var_t4_dn13 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn13)) - ((locals.var_nq_dn13 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn13 + locals.var_qdeff_dn13) + locals.var_t8_dn13))))))) + ((locals.var_mdl_less_1_dn13 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn13)));
        locals.var_qb_1_dn14 = (((locals.var_inv_mdl_dn14 * assign31110_e41220) + (locals.var_inv_mdl * ((locals.var_t1_dn14 + locals.var_t2_dn14) + (((locals.var_t4_dn14 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn14)) - ((locals.var_nq_dn14 * assign31110_e41217) + (locals.var_nq * ((locals.var_qs_1_dn14 + locals.var_qdeff_dn14) + locals.var_t8_dn14))))))) + ((locals.var_mdl_less_1_dn14 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn14)));

        let assign31120_e41228: f64 = (locals.var_qs_1 + locals.var_qdeff);
        locals.var_t9 = assign31120_e41228;
        locals.var_t9_dn0 = (locals.var_qs_1_dn0 + locals.var_qdeff_dn0);
        locals.var_t9_dn2 = (locals.var_qs_1_dn2 + locals.var_qdeff_dn2);
        locals.var_t9_dn3 = (locals.var_qs_1_dn3 + locals.var_qdeff_dn3);
        locals.var_t9_dn4 = (locals.var_qs_1_dn4 + locals.var_qdeff_dn4);
        locals.var_t9_dn5 = (locals.var_qs_1_dn5 + locals.var_qdeff_dn5);
        locals.var_t9_dn6 = (locals.var_qs_1_dn6 + locals.var_qdeff_dn6);
        locals.var_t9_dn7 = (locals.var_qs_1_dn7 + locals.var_qdeff_dn7);
        locals.var_t9_dn8 = (locals.var_qs_1_dn8 + locals.var_qdeff_dn8);
        locals.var_t9_dn9 = (locals.var_qs_1_dn9 + locals.var_qdeff_dn9);
        locals.var_t9_dn10 = (locals.var_qs_1_dn10 + locals.var_qdeff_dn10);
        locals.var_t9_dn11 = (locals.var_qs_1_dn11 + locals.var_qdeff_dn11);
        locals.var_t9_dn12 = (locals.var_qs_1_dn12 + locals.var_qdeff_dn12);
        locals.var_t9_dn13 = (locals.var_qs_1_dn13 + locals.var_qdeff_dn13);
        locals.var_t9_dn14 = (locals.var_qs_1_dn14 + locals.var_qdeff_dn14);

        let assign31130_e41231: f64 = (locals.var_dqsd2 * locals.var_t5);
        let assign31130_e41233: f64 = (assign31130_e41231 * locals.var_t5);
        locals.var_t10 = assign31130_e41233;
        locals.var_t10_dn0 = ((((locals.var_dqsd2_dn0 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn0)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn0));
        locals.var_t10_dn2 = ((((locals.var_dqsd2_dn2 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn2)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn2));
        locals.var_t10_dn3 = ((((locals.var_dqsd2_dn3 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn3)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn3));
        locals.var_t10_dn4 = ((((locals.var_dqsd2_dn4 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn4)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn4));
        locals.var_t10_dn5 = ((((locals.var_dqsd2_dn5 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn5)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn5));
        locals.var_t10_dn6 = ((((locals.var_dqsd2_dn6 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn6)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn6));
        locals.var_t10_dn7 = ((((locals.var_dqsd2_dn7 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn7)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn7));
        locals.var_t10_dn8 = ((((locals.var_dqsd2_dn8 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn8)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn8));
        locals.var_t10_dn9 = ((((locals.var_dqsd2_dn9 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn9)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn9));
        locals.var_t10_dn10 = ((((locals.var_dqsd2_dn10 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn10)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn10));
        locals.var_t10_dn11 = ((((locals.var_dqsd2_dn11 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn11)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn11));
        locals.var_t10_dn12 = ((((locals.var_dqsd2_dn12 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn12)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn12));
        locals.var_t10_dn13 = ((((locals.var_dqsd2_dn13 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn13)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn13));
        locals.var_t10_dn14 = ((((locals.var_dqsd2_dn14 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn14)) * locals.var_t5) + (assign31130_e41231 * locals.var_t5_dn14));

        let assign31140_e41236: f64 = (locals.var_nq * locals.var_inv_mdl);
        let assign31140_e41240: f64 = (0.3333333333333333 * locals.var_dqsd2);
        let assign31140_e41242: f64 = (assign31140_e41240 * locals.var_t5);
        let assign31140_e41243: f64 = (locals.var_t9 + assign31140_e41242);
        let assign31140_e41244: f64 = (assign31140_e41236 * assign31140_e41243);
        let assign31140_e41247: f64 = (2.0 * locals.var_nq);
        let assign31140_e41249: f64 = (assign31140_e41247 * locals.var_mdl_less_1);
        let assign31140_e41251: f64 = (assign31140_e41249 * locals.var_qdeff);
        let assign31140_e41252: f64 = (assign31140_e41244 + assign31140_e41251);
        locals.var_qi = assign31140_e41252;
        locals.var_qi_dn0 = (((((locals.var_nq_dn0 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn0)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn0 + (((0.3333333333333333 * locals.var_dqsd2_dn0) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn0))))) + (((((2.0 * locals.var_nq_dn0) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn0)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn0)));
        locals.var_qi_dn2 = (((((locals.var_nq_dn2 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn2)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn2 + (((0.3333333333333333 * locals.var_dqsd2_dn2) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn2))))) + (((((2.0 * locals.var_nq_dn2) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn2)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn2)));
        locals.var_qi_dn3 = (((((locals.var_nq_dn3 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn3)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn3 + (((0.3333333333333333 * locals.var_dqsd2_dn3) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn3))))) + (((((2.0 * locals.var_nq_dn3) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn3)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn3)));
        locals.var_qi_dn4 = (((((locals.var_nq_dn4 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn4)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn4 + (((0.3333333333333333 * locals.var_dqsd2_dn4) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn4))))) + (((((2.0 * locals.var_nq_dn4) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn4)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn4)));
        locals.var_qi_dn5 = (((((locals.var_nq_dn5 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn5)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn5 + (((0.3333333333333333 * locals.var_dqsd2_dn5) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn5))))) + (((((2.0 * locals.var_nq_dn5) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn5)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn5)));
        locals.var_qi_dn6 = (((((locals.var_nq_dn6 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn6)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn6 + (((0.3333333333333333 * locals.var_dqsd2_dn6) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn6))))) + (((((2.0 * locals.var_nq_dn6) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn6)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn6)));
        locals.var_qi_dn7 = (((((locals.var_nq_dn7 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn7)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn7 + (((0.3333333333333333 * locals.var_dqsd2_dn7) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn7))))) + (((((2.0 * locals.var_nq_dn7) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn7)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn7)));
        locals.var_qi_dn8 = (((((locals.var_nq_dn8 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn8)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn8 + (((0.3333333333333333 * locals.var_dqsd2_dn8) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn8))))) + (((((2.0 * locals.var_nq_dn8) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn8)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn8)));
        locals.var_qi_dn9 = (((((locals.var_nq_dn9 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn9)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn9 + (((0.3333333333333333 * locals.var_dqsd2_dn9) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn9))))) + (((((2.0 * locals.var_nq_dn9) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn9)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn9)));
        locals.var_qi_dn10 = (((((locals.var_nq_dn10 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn10)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn10 + (((0.3333333333333333 * locals.var_dqsd2_dn10) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn10))))) + (((((2.0 * locals.var_nq_dn10) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn10)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn10)));
        locals.var_qi_dn11 = (((((locals.var_nq_dn11 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn11)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn11 + (((0.3333333333333333 * locals.var_dqsd2_dn11) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn11))))) + (((((2.0 * locals.var_nq_dn11) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn11)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn11)));
        locals.var_qi_dn12 = (((((locals.var_nq_dn12 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn12)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn12 + (((0.3333333333333333 * locals.var_dqsd2_dn12) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn12))))) + (((((2.0 * locals.var_nq_dn12) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn12)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn12)));
        locals.var_qi_dn13 = (((((locals.var_nq_dn13 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn13)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn13 + (((0.3333333333333333 * locals.var_dqsd2_dn13) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn13))))) + (((((2.0 * locals.var_nq_dn13) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn13)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn13)));
        locals.var_qi_dn14 = (((((locals.var_nq_dn14 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn14)) * assign31140_e41243) + (assign31140_e41236 * (locals.var_t9_dn14 + (((0.3333333333333333 * locals.var_dqsd2_dn14) * locals.var_t5) + (assign31140_e41240 * locals.var_t5_dn14))))) + (((((2.0 * locals.var_nq_dn14) * locals.var_mdl_less_1) + (assign31140_e41247 * locals.var_mdl_less_1_dn14)) * locals.var_qdeff) + (assign31140_e41249 * locals.var_qdeff_dn14)));

        let assign31150_e41255: f64 = (locals.var_nq * locals.var_inv_mdl_2);
        let assign31150_e41258: f64 = (0.5 * locals.var_t9);
        let assign31150_e41261: f64 = (locals.var_dqsd / 6.0);
        let assign31150_e41265: f64 = (locals.var_dqsd * locals.var_t5);
        let assign31150_e41266: f64 = (1.0 - assign31150_e41265);
        let assign31150_e41269: f64 = (0.2 * locals.var_t10);
        let assign31150_e41270: f64 = (assign31150_e41266 - assign31150_e41269);
        let assign31150_e41271: f64 = (assign31150_e41261 * assign31150_e41270);
        let assign31150_e41272: f64 = (assign31150_e41258 - assign31150_e41271);
        let assign31150_e41273: f64 = (assign31150_e41255 * assign31150_e41272);
        locals.var_qd1 = assign31150_e41273;
        locals.var_qd1_dn0 = ((((locals.var_nq_dn0 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn0)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn0) - (((locals.var_dqsd_dn0 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn0 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn0))) - (0.2 * locals.var_t10_dn0)))))));
        locals.var_qd1_dn2 = ((((locals.var_nq_dn2 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn2)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn2) - (((locals.var_dqsd_dn2 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn2 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn2))) - (0.2 * locals.var_t10_dn2)))))));
        locals.var_qd1_dn3 = ((((locals.var_nq_dn3 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn3)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn3) - (((locals.var_dqsd_dn3 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn3 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn3))) - (0.2 * locals.var_t10_dn3)))))));
        locals.var_qd1_dn4 = ((((locals.var_nq_dn4 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn4)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn4) - (((locals.var_dqsd_dn4 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn4 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn4))) - (0.2 * locals.var_t10_dn4)))))));
        locals.var_qd1_dn5 = ((((locals.var_nq_dn5 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn5)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn5) - (((locals.var_dqsd_dn5 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn5 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn5))) - (0.2 * locals.var_t10_dn5)))))));
        locals.var_qd1_dn6 = ((((locals.var_nq_dn6 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn6)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn6) - (((locals.var_dqsd_dn6 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn6 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn6))) - (0.2 * locals.var_t10_dn6)))))));
        locals.var_qd1_dn7 = ((((locals.var_nq_dn7 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn7)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn7) - (((locals.var_dqsd_dn7 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn7 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn7))) - (0.2 * locals.var_t10_dn7)))))));
        locals.var_qd1_dn8 = ((((locals.var_nq_dn8 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn8)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn8) - (((locals.var_dqsd_dn8 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn8 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn8))) - (0.2 * locals.var_t10_dn8)))))));
        locals.var_qd1_dn9 = ((((locals.var_nq_dn9 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn9)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn9) - (((locals.var_dqsd_dn9 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn9 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn9))) - (0.2 * locals.var_t10_dn9)))))));
        locals.var_qd1_dn10 = ((((locals.var_nq_dn10 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn10)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn10) - (((locals.var_dqsd_dn10 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn10 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn10))) - (0.2 * locals.var_t10_dn10)))))));
        locals.var_qd1_dn11 = ((((locals.var_nq_dn11 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn11)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn11) - (((locals.var_dqsd_dn11 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn11 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn11))) - (0.2 * locals.var_t10_dn11)))))));
        locals.var_qd1_dn12 = ((((locals.var_nq_dn12 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn12)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn12) - (((locals.var_dqsd_dn12 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn12 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn12))) - (0.2 * locals.var_t10_dn12)))))));
        locals.var_qd1_dn13 = ((((locals.var_nq_dn13 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn13)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn13) - (((locals.var_dqsd_dn13 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn13 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn13))) - (0.2 * locals.var_t10_dn13)))))));
        locals.var_qd1_dn14 = ((((locals.var_nq_dn14 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn14)) * assign31150_e41272) + (assign31150_e41255 * ((0.5 * locals.var_t9_dn14) - (((locals.var_dqsd_dn14 / 6.0) * assign31150_e41270) + (assign31150_e41261 * ((-((locals.var_dqsd_dn14 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn14))) - (0.2 * locals.var_t10_dn14)))))));

    }

    pub(super) fn stamp_transient_block_98(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign31160_e41277: f64 = (locals.var_mdl - locals.var_inv_mdl);
        let assign31160_e41278: f64 = (locals.var_nq * assign31160_e41277);
        let assign31160_e41280: f64 = (assign31160_e41278 * locals.var_qdeff);
        locals.var_qd2 = assign31160_e41280;
        locals.var_qd2_dn0 = ((((locals.var_nq_dn0 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn0 - locals.var_inv_mdl_dn0))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn0));
        locals.var_qd2_dn2 = ((((locals.var_nq_dn2 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn2 - locals.var_inv_mdl_dn2))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn2));
        locals.var_qd2_dn3 = ((((locals.var_nq_dn3 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn3 - locals.var_inv_mdl_dn3))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn3));
        locals.var_qd2_dn4 = ((((locals.var_nq_dn4 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn4 - locals.var_inv_mdl_dn4))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn4));
        locals.var_qd2_dn5 = ((((locals.var_nq_dn5 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn5 - locals.var_inv_mdl_dn5))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn5));
        locals.var_qd2_dn6 = ((((locals.var_nq_dn6 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn6 - locals.var_inv_mdl_dn6))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn6));
        locals.var_qd2_dn7 = ((((locals.var_nq_dn7 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn7 - locals.var_inv_mdl_dn7))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn7));
        locals.var_qd2_dn8 = ((((locals.var_nq_dn8 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn8 - locals.var_inv_mdl_dn8))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn8));
        locals.var_qd2_dn9 = ((((locals.var_nq_dn9 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn9 - locals.var_inv_mdl_dn9))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn9));
        locals.var_qd2_dn10 = ((((locals.var_nq_dn10 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn10 - locals.var_inv_mdl_dn10))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn10));
        locals.var_qd2_dn11 = ((((locals.var_nq_dn11 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn11 - locals.var_inv_mdl_dn11))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn11));
        locals.var_qd2_dn12 = ((((locals.var_nq_dn12 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn12 - locals.var_inv_mdl_dn12))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn12));
        locals.var_qd2_dn13 = ((((locals.var_nq_dn13 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn13 - locals.var_inv_mdl_dn13))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn13));
        locals.var_qd2_dn14 = ((((locals.var_nq_dn14 * assign31160_e41277) + (locals.var_nq * (locals.var_mdl_dn14 - locals.var_inv_mdl_dn14))) * locals.var_qdeff) + (assign31160_e41278 * locals.var_qdeff_dn14));

        let assign31170_e41283: f64 = (locals.var_qd1 + locals.var_qd2);
        locals.var_qd = assign31170_e41283;
        locals.var_qd_dn0 = (locals.var_qd1_dn0 + locals.var_qd2_dn0);
        locals.var_qd_dn2 = (locals.var_qd1_dn2 + locals.var_qd2_dn2);
        locals.var_qd_dn3 = (locals.var_qd1_dn3 + locals.var_qd2_dn3);
        locals.var_qd_dn4 = (locals.var_qd1_dn4 + locals.var_qd2_dn4);
        locals.var_qd_dn5 = (locals.var_qd1_dn5 + locals.var_qd2_dn5);
        locals.var_qd_dn6 = (locals.var_qd1_dn6 + locals.var_qd2_dn6);
        locals.var_qd_dn7 = (locals.var_qd1_dn7 + locals.var_qd2_dn7);
        locals.var_qd_dn8 = (locals.var_qd1_dn8 + locals.var_qd2_dn8);
        locals.var_qd_dn9 = (locals.var_qd1_dn9 + locals.var_qd2_dn9);
        locals.var_qd_dn10 = (locals.var_qd1_dn10 + locals.var_qd2_dn10);
        locals.var_qd_dn11 = (locals.var_qd1_dn11 + locals.var_qd2_dn11);
        locals.var_qd_dn12 = (locals.var_qd1_dn12 + locals.var_qd2_dn12);
        locals.var_qd_dn13 = (locals.var_qd1_dn13 + locals.var_qd2_dn13);
        locals.var_qd_dn14 = (locals.var_qd1_dn14 + locals.var_qd2_dn14);

        let assign31180_e41286: f64 = (locals.var_qi - locals.var_qd);
        locals.var_qs = assign31180_e41286;
        locals.var_qs_dn0 = (locals.var_qi_dn0 - locals.var_qd_dn0);
        locals.var_qs_dn2 = (locals.var_qi_dn2 - locals.var_qd_dn2);
        locals.var_qs_dn3 = (locals.var_qi_dn3 - locals.var_qd_dn3);
        locals.var_qs_dn4 = (locals.var_qi_dn4 - locals.var_qd_dn4);
        locals.var_qs_dn5 = (locals.var_qi_dn5 - locals.var_qd_dn5);
        locals.var_qs_dn6 = (locals.var_qi_dn6 - locals.var_qd_dn6);
        locals.var_qs_dn7 = (locals.var_qi_dn7 - locals.var_qd_dn7);
        locals.var_qs_dn8 = (locals.var_qi_dn8 - locals.var_qd_dn8);
        locals.var_qs_dn9 = (locals.var_qi_dn9 - locals.var_qd_dn9);
        locals.var_qs_dn10 = (locals.var_qi_dn10 - locals.var_qd_dn10);
        locals.var_qs_dn11 = (locals.var_qi_dn11 - locals.var_qd_dn11);
        locals.var_qs_dn12 = (locals.var_qi_dn12 - locals.var_qd_dn12);
        locals.var_qs_dn13 = (locals.var_qi_dn13 - locals.var_qd_dn13);
        locals.var_qs_dn14 = (locals.var_qi_dn14 - locals.var_qd_dn14);

        let assign31190_e41292: f64 = (locals.var_vt * locals.var_qb_1);
        let assign31190_e41294: f64 = (-2500.0);
        let assign31190_e41296: f64 = (assign31190_e41294 * p.p694);
        let assign31190_e41298: f64 = if ((0.0 == 0.0) && (assign31190_e41292 < assign31190_e41296)) { 1.0 } else { 0.0 };
        locals.var_guard727 = assign31190_e41298;

        let (assign31200_e41311, assign31200_e41311_d_n0, assign31200_e41311_d_n2, assign31200_e41311_d_n3, assign31200_e41311_d_n4, assign31200_e41311_d_n5, assign31200_e41311_d_n6, assign31200_e41311_d_n7, assign31200_e41311_d_n8, assign31200_e41311_d_n9, assign31200_e41311_d_n10, assign31200_e41311_d_n11, assign31200_e41311_d_n12, assign31200_e41311_d_n13, assign31200_e41311_d_n14,) = {
    if (locals.var_guard727 != 0.0) {
        let assign31200_e41301: f64 = (-p.p694);
        let assign31200_e41303: f64 = (assign31200_e41301 * p.p694);
        let assign31200_e41307: f64 = (locals.var_vt * locals.var_qb_1);
        let assign31200_e41308: f64 = (16.0 * assign31200_e41307);
        let assign31200_e41309: f64 = (assign31200_e41303 / assign31200_e41308);
        (assign31200_e41309, (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn0))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn2))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn3))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * ((locals.var_vt_dn4 * locals.var_qb_1) + (locals.var_vt * locals.var_qb_1_dn4)))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn5))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn6))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn7))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn8))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn9))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn10))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn11))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn12))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn13))) / (assign31200_e41308 * assign31200_e41308))), (-((assign31200_e41303 * (16.0 * (locals.var_vt * locals.var_qb_1_dn14))) / (assign31200_e41308 * assign31200_e41308))),)
    } else {
        (locals.var_qbacv, locals.var_qbacv_dn0, locals.var_qbacv_dn2, locals.var_qbacv_dn3, locals.var_qbacv_dn4, locals.var_qbacv_dn5, locals.var_qbacv_dn6, locals.var_qbacv_dn7, locals.var_qbacv_dn8, locals.var_qbacv_dn9, locals.var_qbacv_dn10, locals.var_qbacv_dn11, locals.var_qbacv_dn12, locals.var_qbacv_dn13, locals.var_qbacv_dn14,)
    }
};
        locals.var_qbacv = assign31200_e41311;
        locals.var_qbacv_dn0 = assign31200_e41311_d_n0;
        locals.var_qbacv_dn2 = assign31200_e41311_d_n2;
        locals.var_qbacv_dn3 = assign31200_e41311_d_n3;
        locals.var_qbacv_dn4 = assign31200_e41311_d_n4;
        locals.var_qbacv_dn5 = assign31200_e41311_d_n5;
        locals.var_qbacv_dn6 = assign31200_e41311_d_n6;
        locals.var_qbacv_dn7 = assign31200_e41311_d_n7;
        locals.var_qbacv_dn8 = assign31200_e41311_d_n8;
        locals.var_qbacv_dn9 = assign31200_e41311_d_n9;
        locals.var_qbacv_dn10 = assign31200_e41311_d_n10;
        locals.var_qbacv_dn11 = assign31200_e41311_d_n11;
        locals.var_qbacv_dn12 = assign31200_e41311_d_n12;
        locals.var_qbacv_dn13 = assign31200_e41311_d_n13;
        locals.var_qbacv_dn14 = assign31200_e41311_d_n14;

        let (assign31210_e41341, assign31210_e41341_d_n0, assign31210_e41341_d_n2, assign31210_e41341_d_n3, assign31210_e41341_d_n4, assign31210_e41341_d_n5, assign31210_e41341_d_n6, assign31210_e41341_d_n7, assign31210_e41341_d_n8, assign31210_e41341_d_n9, assign31210_e41341_d_n10, assign31210_e41341_d_n11, assign31210_e41341_d_n12, assign31210_e41341_d_n13, assign31210_e41341_d_n14,) = {
    if (locals.var_guard727 == 0.0) {
        let assign31210_e41317: f64 = (locals.var_vt * locals.var_qb_1);
        let assign31210_e41319: f64 = assign31210_e41317;
        let assign31210_e41322: f64 = (locals.var_vt * locals.var_qb_1);
        let assign31210_e41324: f64 = assign31210_e41322;
        let assign31210_e41327: f64 = (locals.var_vt * locals.var_qb_1);
        let assign31210_e41329: f64 = assign31210_e41327;
        let assign31210_e41330: f64 = (assign31210_e41324 * assign31210_e41329);
        let assign31210_e41333: f64 = (0.25 * p.p694);
        let assign31210_e41335: f64 = (assign31210_e41333 * p.p694);
        let assign31210_e41336: f64 = (assign31210_e41330 + assign31210_e41335);
        let assign31210_e41337: f64 = (assign31210_e41336).sqrt();
        let assign31210_e41338: f64 = (assign31210_e41319 + assign31210_e41337);
        let assign31210_e41339: f64 = (0.5 * assign31210_e41338);
        (assign31210_e41339, (0.5 * ((locals.var_vt * locals.var_qb_1_dn0) + ((((locals.var_vt * locals.var_qb_1_dn0) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn0))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn2) + ((((locals.var_vt * locals.var_qb_1_dn2) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn2))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn3) + ((((locals.var_vt * locals.var_qb_1_dn3) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn3))) / (2.0 * assign31210_e41337)))), (0.5 * (((locals.var_vt_dn4 * locals.var_qb_1) + (locals.var_vt * locals.var_qb_1_dn4)) + (((((locals.var_vt_dn4 * locals.var_qb_1) + (locals.var_vt * locals.var_qb_1_dn4)) * assign31210_e41329) + (assign31210_e41324 * ((locals.var_vt_dn4 * locals.var_qb_1) + (locals.var_vt * locals.var_qb_1_dn4)))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn5) + ((((locals.var_vt * locals.var_qb_1_dn5) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn5))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn6) + ((((locals.var_vt * locals.var_qb_1_dn6) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn6))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn7) + ((((locals.var_vt * locals.var_qb_1_dn7) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn7))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn8) + ((((locals.var_vt * locals.var_qb_1_dn8) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn8))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn9) + ((((locals.var_vt * locals.var_qb_1_dn9) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn9))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn10) + ((((locals.var_vt * locals.var_qb_1_dn10) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn10))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn11) + ((((locals.var_vt * locals.var_qb_1_dn11) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn11))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn12) + ((((locals.var_vt * locals.var_qb_1_dn12) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn12))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn13) + ((((locals.var_vt * locals.var_qb_1_dn13) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn13))) / (2.0 * assign31210_e41337)))), (0.5 * ((locals.var_vt * locals.var_qb_1_dn14) + ((((locals.var_vt * locals.var_qb_1_dn14) * assign31210_e41329) + (assign31210_e41324 * (locals.var_vt * locals.var_qb_1_dn14))) / (2.0 * assign31210_e41337)))),)
    } else {
        (locals.var_qbacv, locals.var_qbacv_dn0, locals.var_qbacv_dn2, locals.var_qbacv_dn3, locals.var_qbacv_dn4, locals.var_qbacv_dn5, locals.var_qbacv_dn6, locals.var_qbacv_dn7, locals.var_qbacv_dn8, locals.var_qbacv_dn9, locals.var_qbacv_dn10, locals.var_qbacv_dn11, locals.var_qbacv_dn12, locals.var_qbacv_dn13, locals.var_qbacv_dn14,)
    }
};
        locals.var_qbacv = assign31210_e41341;
        locals.var_qbacv_dn0 = assign31210_e41341_d_n0;
        locals.var_qbacv_dn2 = assign31210_e41341_d_n2;
        locals.var_qbacv_dn3 = assign31210_e41341_d_n3;
        locals.var_qbacv_dn4 = assign31210_e41341_d_n4;
        locals.var_qbacv_dn5 = assign31210_e41341_d_n5;
        locals.var_qbacv_dn6 = assign31210_e41341_d_n6;
        locals.var_qbacv_dn7 = assign31210_e41341_d_n7;
        locals.var_qbacv_dn8 = assign31210_e41341_d_n8;
        locals.var_qbacv_dn9 = assign31210_e41341_d_n9;
        locals.var_qbacv_dn10 = assign31210_e41341_d_n10;
        locals.var_qbacv_dn11 = assign31210_e41341_d_n11;
        locals.var_qbacv_dn12 = assign31210_e41341_d_n12;
        locals.var_qbacv_dn13 = assign31210_e41341_d_n13;
        locals.var_qbacv_dn14 = assign31210_e41341_d_n14;

        let assign31220_e41345: f64 = (locals.var_qs + locals.var_qd);
        let assign31220_e41346: f64 = (locals.var_vt * assign31220_e41345);
        locals.var_qiacv = assign31220_e41346;
        locals.var_qiacv_dn0 = (locals.var_vt * (locals.var_qs_dn0 + locals.var_qd_dn0));
        locals.var_qiacv_dn2 = (locals.var_vt * (locals.var_qs_dn2 + locals.var_qd_dn2));
        locals.var_qiacv_dn3 = (locals.var_vt * (locals.var_qs_dn3 + locals.var_qd_dn3));
        locals.var_qiacv_dn4 = ((locals.var_vt_dn4 * assign31220_e41345) + (locals.var_vt * (locals.var_qs_dn4 + locals.var_qd_dn4)));
        locals.var_qiacv_dn5 = (locals.var_vt * (locals.var_qs_dn5 + locals.var_qd_dn5));
        locals.var_qiacv_dn6 = (locals.var_vt * (locals.var_qs_dn6 + locals.var_qd_dn6));
        locals.var_qiacv_dn7 = (locals.var_vt * (locals.var_qs_dn7 + locals.var_qd_dn7));
        locals.var_qiacv_dn8 = (locals.var_vt * (locals.var_qs_dn8 + locals.var_qd_dn8));
        locals.var_qiacv_dn9 = (locals.var_vt * (locals.var_qs_dn9 + locals.var_qd_dn9));
        locals.var_qiacv_dn10 = (locals.var_vt * (locals.var_qs_dn10 + locals.var_qd_dn10));
        locals.var_qiacv_dn11 = (locals.var_vt * (locals.var_qs_dn11 + locals.var_qd_dn11));
        locals.var_qiacv_dn12 = (locals.var_vt * (locals.var_qs_dn12 + locals.var_qd_dn12));
        locals.var_qiacv_dn13 = (locals.var_vt * (locals.var_qs_dn13 + locals.var_qd_dn13));
        locals.var_qiacv_dn14 = (locals.var_vt * (locals.var_qs_dn14 + locals.var_qd_dn14));

        let assign31230_e41350: f64 = (p.p208 * locals.var_qbacv);
        let assign31230_e41351: f64 = (locals.var_qiacv + assign31230_e41350);
        let assign31230_e41353: f64 = (assign31230_e41351 / p.p207);
        locals.var_t0 = assign31230_e41353;
        locals.var_t0_dn0 = ((locals.var_qiacv_dn0 + (p.p208 * locals.var_qbacv_dn0)) / p.p207);
        locals.var_t0_dn2 = ((locals.var_qiacv_dn2 + (p.p208 * locals.var_qbacv_dn2)) / p.p207);
        locals.var_t0_dn3 = ((locals.var_qiacv_dn3 + (p.p208 * locals.var_qbacv_dn3)) / p.p207);
        locals.var_t0_dn4 = ((locals.var_qiacv_dn4 + (p.p208 * locals.var_qbacv_dn4)) / p.p207);
        locals.var_t0_dn5 = ((locals.var_qiacv_dn5 + (p.p208 * locals.var_qbacv_dn5)) / p.p207);
        locals.var_t0_dn6 = ((locals.var_qiacv_dn6 + (p.p208 * locals.var_qbacv_dn6)) / p.p207);
        locals.var_t0_dn7 = ((locals.var_qiacv_dn7 + (p.p208 * locals.var_qbacv_dn7)) / p.p207);
        locals.var_t0_dn8 = ((locals.var_qiacv_dn8 + (p.p208 * locals.var_qbacv_dn8)) / p.p207);
        locals.var_t0_dn9 = ((locals.var_qiacv_dn9 + (p.p208 * locals.var_qbacv_dn9)) / p.p207);
        locals.var_t0_dn10 = ((locals.var_qiacv_dn10 + (p.p208 * locals.var_qbacv_dn10)) / p.p207);
        locals.var_t0_dn11 = ((locals.var_qiacv_dn11 + (p.p208 * locals.var_qbacv_dn11)) / p.p207);
        locals.var_t0_dn12 = ((locals.var_qiacv_dn12 + (p.p208 * locals.var_qbacv_dn12)) / p.p207);
        locals.var_t0_dn13 = ((locals.var_qiacv_dn13 + (p.p208 * locals.var_qbacv_dn13)) / p.p207);
        locals.var_t0_dn14 = ((locals.var_qiacv_dn14 + (p.p208 * locals.var_qbacv_dn14)) / p.p207);

        let assign31240_e41358: f64 = (0.7 * p.p206);
        let assign31240_e41359: f64 = (locals.var_t0).powf(assign31240_e41358);
        let assign31240_e41360: f64 = (1.0 + assign31240_e41359);
        locals.var_t1 = assign31240_e41360;
        locals.var_t1_dn0 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn0)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn0 / locals.var_t0))) };
        locals.var_t1_dn2 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn2)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn2 / locals.var_t0))) };
        locals.var_t1_dn3 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn3)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn3 / locals.var_t0))) };
        locals.var_t1_dn4 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn4)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn4 / locals.var_t0))) };
        locals.var_t1_dn5 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn5)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn5 / locals.var_t0))) };
        locals.var_t1_dn6 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn6)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn6 / locals.var_t0))) };
        locals.var_t1_dn7 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn7)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn7 / locals.var_t0))) };
        locals.var_t1_dn8 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn8)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn8 / locals.var_t0))) };
        locals.var_t1_dn9 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn9)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn9 / locals.var_t0))) };
        locals.var_t1_dn10 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn10)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn10 / locals.var_t0))) };
        locals.var_t1_dn11 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn11)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn11 / locals.var_t0))) };
        locals.var_t1_dn12 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn12)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn12 / locals.var_t0))) };
        locals.var_t1_dn13 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn13)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn13 / locals.var_t0))) };
        locals.var_t1_dn14 = if 0.0 == 0.0 && ((assign31240_e41358) as f64).is_finite() && ((assign31240_e41358) as f64).fract() == 0.0 { if assign31240_e41358 == 0.0 { 0.0 } else { (assign31240_e41358 * ((locals.var_t0).powf(assign31240_e41358 - 1.0) * locals.var_t0_dn14)) } } else { (assign31240_e41359 * (assign31240_e41358 * (locals.var_t0_dn14 / locals.var_t0))) };

        let assign31250_e41363: f64 = (p.p205 * 1.9e-9);
        let assign31250_e41365: f64 = (assign31250_e41363 / locals.var_t1);
        locals.var_xdcinv = assign31250_e41365;
        locals.var_xdcinv_dn0 = (-((assign31250_e41363 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn2 = (-((assign31250_e41363 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn3 = (-((assign31250_e41363 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn4 = (-((assign31250_e41363 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn5 = (-((assign31250_e41363 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn6 = (-((assign31250_e41363 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn7 = (-((assign31250_e41363 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn8 = (-((assign31250_e41363 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn9 = (-((assign31250_e41363 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn10 = (-((assign31250_e41363 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn11 = (-((assign31250_e41363 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn12 = (-((assign31250_e41363 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn13 = (-((assign31250_e41363 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn14 = (-((assign31250_e41363 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1)));

        let assign31260_e41368: f64 = (3.9 * 8.85418e-12);
        let assign31260_e41371: f64 = (locals.var_bsimbulktoxp * 3.9);
        let assign31260_e41373: f64 = (assign31260_e41371 / p.p111);
        let assign31260_e41376: f64 = (locals.var_xdcinv / locals.var_epsratio);
        let assign31260_e41377: f64 = (assign31260_e41373 + assign31260_e41376);
        let assign31260_e41378: f64 = (assign31260_e41368 / assign31260_e41377);
        locals.var_coxeffinv = assign31260_e41378;
        locals.var_coxeffinv_dn0 = (-((assign31260_e41368 * (locals.var_xdcinv_dn0 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn2 = (-((assign31260_e41368 * (locals.var_xdcinv_dn2 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn3 = (-((assign31260_e41368 * (locals.var_xdcinv_dn3 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn4 = (-((assign31260_e41368 * (locals.var_xdcinv_dn4 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn5 = (-((assign31260_e41368 * (locals.var_xdcinv_dn5 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn6 = (-((assign31260_e41368 * (locals.var_xdcinv_dn6 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn7 = (-((assign31260_e41368 * (locals.var_xdcinv_dn7 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn8 = (-((assign31260_e41368 * (locals.var_xdcinv_dn8 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn9 = (-((assign31260_e41368 * (locals.var_xdcinv_dn9 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn10 = (-((assign31260_e41368 * (locals.var_xdcinv_dn10 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn11 = (-((assign31260_e41368 * (locals.var_xdcinv_dn11 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn12 = (-((assign31260_e41368 * (locals.var_xdcinv_dn12 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn13 = (-((assign31260_e41368 * (locals.var_xdcinv_dn13 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn14 = (-((assign31260_e41368 * (locals.var_xdcinv_dn14 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));

        let assign31270_e41380: f64 = (-p.p2);
        let assign31270_e41382: f64 = (assign31270_e41380 * locals.var_wact);
        let assign31270_e41384: f64 = (assign31270_e41382 * locals.var_lact);
        let assign31270_e41387: f64 = (8.85418e-12 * p.p111);
        let assign31270_e41389: f64 = (assign31270_e41387 / locals.var_bsimbulktoxp);
        let assign31270_e41390: f64 = (assign31270_e41384 * assign31270_e41389);
        let assign31270_e41392: f64 = (assign31270_e41390 * locals.var_vt);
        let assign31270_e41394: f64 = (assign31270_e41392 * locals.var_qb_1);
        locals.var_qbi = assign31270_e41394;
        locals.var_qbi_dn0 = (assign31270_e41392 * locals.var_qb_1_dn0);
        locals.var_qbi_dn2 = (assign31270_e41392 * locals.var_qb_1_dn2);
        locals.var_qbi_dn3 = (assign31270_e41392 * locals.var_qb_1_dn3);
        locals.var_qbi_dn4 = (((assign31270_e41390 * locals.var_vt_dn4) * locals.var_qb_1) + (assign31270_e41392 * locals.var_qb_1_dn4));
        locals.var_qbi_dn5 = (assign31270_e41392 * locals.var_qb_1_dn5);
        locals.var_qbi_dn6 = (assign31270_e41392 * locals.var_qb_1_dn6);
        locals.var_qbi_dn7 = (assign31270_e41392 * locals.var_qb_1_dn7);
        locals.var_qbi_dn8 = (assign31270_e41392 * locals.var_qb_1_dn8);
        locals.var_qbi_dn9 = (assign31270_e41392 * locals.var_qb_1_dn9);
        locals.var_qbi_dn10 = (assign31270_e41392 * locals.var_qb_1_dn10);
        locals.var_qbi_dn11 = (assign31270_e41392 * locals.var_qb_1_dn11);
        locals.var_qbi_dn12 = (assign31270_e41392 * locals.var_qb_1_dn12);
        locals.var_qbi_dn13 = (assign31270_e41392 * locals.var_qb_1_dn13);
        locals.var_qbi_dn14 = (assign31270_e41392 * locals.var_qb_1_dn14);

        let assign31280_e41397: f64 = (p.p2 * locals.var_wact);
        let assign31280_e41399: f64 = (assign31280_e41397 * locals.var_lact);
        let assign31280_e41401: f64 = (assign31280_e41399 * locals.var_coxeffinv);
        let assign31280_e41403: f64 = (assign31280_e41401 * locals.var_vt);
        locals.var_wlcoxvtinv = assign31280_e41403;
        locals.var_wlcoxvtinv_dn0 = ((assign31280_e41399 * locals.var_coxeffinv_dn0) * locals.var_vt);
        locals.var_wlcoxvtinv_dn2 = ((assign31280_e41399 * locals.var_coxeffinv_dn2) * locals.var_vt);
        locals.var_wlcoxvtinv_dn3 = ((assign31280_e41399 * locals.var_coxeffinv_dn3) * locals.var_vt);
        locals.var_wlcoxvtinv_dn4 = (((assign31280_e41399 * locals.var_coxeffinv_dn4) * locals.var_vt) + (assign31280_e41401 * locals.var_vt_dn4));
        locals.var_wlcoxvtinv_dn5 = ((assign31280_e41399 * locals.var_coxeffinv_dn5) * locals.var_vt);
        locals.var_wlcoxvtinv_dn6 = ((assign31280_e41399 * locals.var_coxeffinv_dn6) * locals.var_vt);
        locals.var_wlcoxvtinv_dn7 = ((assign31280_e41399 * locals.var_coxeffinv_dn7) * locals.var_vt);
        locals.var_wlcoxvtinv_dn8 = ((assign31280_e41399 * locals.var_coxeffinv_dn8) * locals.var_vt);
        locals.var_wlcoxvtinv_dn9 = ((assign31280_e41399 * locals.var_coxeffinv_dn9) * locals.var_vt);
        locals.var_wlcoxvtinv_dn10 = ((assign31280_e41399 * locals.var_coxeffinv_dn10) * locals.var_vt);
        locals.var_wlcoxvtinv_dn11 = ((assign31280_e41399 * locals.var_coxeffinv_dn11) * locals.var_vt);
        locals.var_wlcoxvtinv_dn12 = ((assign31280_e41399 * locals.var_coxeffinv_dn12) * locals.var_vt);
        locals.var_wlcoxvtinv_dn13 = ((assign31280_e41399 * locals.var_coxeffinv_dn13) * locals.var_vt);
        locals.var_wlcoxvtinv_dn14 = ((assign31280_e41399 * locals.var_coxeffinv_dn14) * locals.var_vt);

        let assign31290_e41405: f64 = (-locals.var_wlcoxvtinv);
        let assign31290_e41407: f64 = (assign31290_e41405 * locals.var_qs);
        locals.var_qsi = assign31290_e41407;
        locals.var_qsi_dn0 = (((-locals.var_wlcoxvtinv_dn0) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn0));
        locals.var_qsi_dn2 = (((-locals.var_wlcoxvtinv_dn2) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn2));
        locals.var_qsi_dn3 = (((-locals.var_wlcoxvtinv_dn3) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn3));
        locals.var_qsi_dn4 = (((-locals.var_wlcoxvtinv_dn4) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn4));
        locals.var_qsi_dn5 = (((-locals.var_wlcoxvtinv_dn5) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn5));
        locals.var_qsi_dn6 = (((-locals.var_wlcoxvtinv_dn6) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn6));
        locals.var_qsi_dn7 = (((-locals.var_wlcoxvtinv_dn7) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn7));
        locals.var_qsi_dn8 = (((-locals.var_wlcoxvtinv_dn8) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn8));
        locals.var_qsi_dn9 = (((-locals.var_wlcoxvtinv_dn9) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn9));
        locals.var_qsi_dn10 = (((-locals.var_wlcoxvtinv_dn10) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn10));
        locals.var_qsi_dn11 = (((-locals.var_wlcoxvtinv_dn11) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn11));
        locals.var_qsi_dn12 = (((-locals.var_wlcoxvtinv_dn12) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn12));
        locals.var_qsi_dn13 = (((-locals.var_wlcoxvtinv_dn13) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn13));
        locals.var_qsi_dn14 = (((-locals.var_wlcoxvtinv_dn14) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn14));

        let assign31300_e41409: f64 = (-locals.var_wlcoxvtinv);
        let assign31300_e41411: f64 = (assign31300_e41409 * locals.var_qd);
        locals.var_qdi = assign31300_e41411;
        locals.var_qdi_dn0 = (((-locals.var_wlcoxvtinv_dn0) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn0));
        locals.var_qdi_dn2 = (((-locals.var_wlcoxvtinv_dn2) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn2));
        locals.var_qdi_dn3 = (((-locals.var_wlcoxvtinv_dn3) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn3));
        locals.var_qdi_dn4 = (((-locals.var_wlcoxvtinv_dn4) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn4));
        locals.var_qdi_dn5 = (((-locals.var_wlcoxvtinv_dn5) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn5));
        locals.var_qdi_dn6 = (((-locals.var_wlcoxvtinv_dn6) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn6));
        locals.var_qdi_dn7 = (((-locals.var_wlcoxvtinv_dn7) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn7));
        locals.var_qdi_dn8 = (((-locals.var_wlcoxvtinv_dn8) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn8));
        locals.var_qdi_dn9 = (((-locals.var_wlcoxvtinv_dn9) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn9));
        locals.var_qdi_dn10 = (((-locals.var_wlcoxvtinv_dn10) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn10));
        locals.var_qdi_dn11 = (((-locals.var_wlcoxvtinv_dn11) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn11));
        locals.var_qdi_dn12 = (((-locals.var_wlcoxvtinv_dn12) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn12));
        locals.var_qdi_dn13 = (((-locals.var_wlcoxvtinv_dn13) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn13));
        locals.var_qdi_dn14 = (((-locals.var_wlcoxvtinv_dn14) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn14));

        let assign31310_e41414: f64 = (locals.var_qbi + locals.var_qsi);
        let assign31310_e41416: f64 = (assign31310_e41414 + locals.var_qdi);
        let assign31310_e41417: f64 = (-assign31310_e41416);
        locals.var_qgi = assign31310_e41417;
        locals.var_qgi_dn0 = (-((locals.var_qbi_dn0 + locals.var_qsi_dn0) + locals.var_qdi_dn0));
        locals.var_qgi_dn2 = (-((locals.var_qbi_dn2 + locals.var_qsi_dn2) + locals.var_qdi_dn2));
        locals.var_qgi_dn3 = (-((locals.var_qbi_dn3 + locals.var_qsi_dn3) + locals.var_qdi_dn3));
        locals.var_qgi_dn4 = (-((locals.var_qbi_dn4 + locals.var_qsi_dn4) + locals.var_qdi_dn4));
        locals.var_qgi_dn5 = (-((locals.var_qbi_dn5 + locals.var_qsi_dn5) + locals.var_qdi_dn5));
        locals.var_qgi_dn6 = (-((locals.var_qbi_dn6 + locals.var_qsi_dn6) + locals.var_qdi_dn6));
        locals.var_qgi_dn7 = (-((locals.var_qbi_dn7 + locals.var_qsi_dn7) + locals.var_qdi_dn7));
        locals.var_qgi_dn8 = (-((locals.var_qbi_dn8 + locals.var_qsi_dn8) + locals.var_qdi_dn8));
        locals.var_qgi_dn9 = (-((locals.var_qbi_dn9 + locals.var_qsi_dn9) + locals.var_qdi_dn9));
        locals.var_qgi_dn10 = (-((locals.var_qbi_dn10 + locals.var_qsi_dn10) + locals.var_qdi_dn10));
        locals.var_qgi_dn11 = (-((locals.var_qbi_dn11 + locals.var_qsi_dn11) + locals.var_qdi_dn11));
        locals.var_qgi_dn12 = (-((locals.var_qbi_dn12 + locals.var_qsi_dn12) + locals.var_qdi_dn12));
        locals.var_qgi_dn13 = (-((locals.var_qbi_dn13 + locals.var_qsi_dn13) + locals.var_qdi_dn13));
        locals.var_qgi_dn14 = (-((locals.var_qbi_dn14 + locals.var_qsi_dn14) + locals.var_qdi_dn14));

        let assign31320_e41420: f64 = if (!param_given[666]) { 1.0 } else { 0.0 };
        locals.var_guard728 = assign31320_e41420;

        let (assign31330_e41441,) = {
    if (locals.var_guard728 != 0.0) {
        let assign31330_e41424: f64 = (2.0 * p.p111);
        let assign31330_e41426: f64 = (assign31330_e41424 * 8.85418e-12);
        let assign31330_e41428: f64 = (assign31330_e41426 / 3.141592653589793);
        let assign31330_e41433: f64 = (4e-7 / p.p77);
        let assign31330_e41434: f64 = (1.0 + assign31330_e41433);
        let assign31330_e41435: f64 = (p.p670 * assign31330_e41434);
        let assign31330_e41437: f64 = (assign31330_e41435).max(1e-38);
        let assign31330_e41438: f64 = (assign31330_e41437).ln();
        let assign31330_e41439: f64 = (assign31330_e41428 * assign31330_e41438);
        (assign31330_e41439,)
    } else {
        (locals.var_cf_i,)
    }
};
        locals.var_cf_i = assign31330_e41441;

        let assign31340_e41444: f64 = (p.p671 + locals.var_cf_i);
        locals.var_cgsof = assign31340_e41444;

        let assign31350_e41447: f64 = (p.p672 + locals.var_cf_i);
        locals.var_cgdof = assign31350_e41447;

        let assign31360_e41450: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard729 = assign31360_e41450;

        let (assign31370_e41461, assign31370_e41461_d_n0, assign31370_e41461_d_n2, assign31370_e41461_d_n3, assign31370_e41461_d_n4, assign31370_e41461_d_n5, assign31370_e41461_d_n6, assign31370_e41461_d_n7, assign31370_e41461_d_n8, assign31370_e41461_d_n9, assign31370_e41461_d_n10, assign31370_e41461_d_n11, assign31370_e41461_d_n12, assign31370_e41461_d_n13, assign31370_e41461_d_n14,) = {
    if (locals.var_guard729 != 0.0) {
        let assign31370_e41453: f64 = (-locals.var_wact);
        let assign31370_e41455: f64 = (assign31370_e41453 * p.p2);
        let assign31370_e41457: f64 = (assign31370_e41455 * locals.var_cgsof);
        let assign31370_e41459: f64 = (assign31370_e41457 * locals.var_vgs_ov_noswap);
        (assign31370_e41459, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (assign31370_e41457 * locals.var_vgs_ov_noswap_dn7), 0.0, 0.0, (assign31370_e41457 * locals.var_vgs_ov_noswap_dn10), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn13, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign31370_e41461;
        locals.var_qovs_dn0 = assign31370_e41461_d_n0;
        locals.var_qovs_dn2 = assign31370_e41461_d_n2;
        locals.var_qovs_dn3 = assign31370_e41461_d_n3;
        locals.var_qovs_dn4 = assign31370_e41461_d_n4;
        locals.var_qovs_dn5 = assign31370_e41461_d_n5;
        locals.var_qovs_dn6 = assign31370_e41461_d_n6;
        locals.var_qovs_dn7 = assign31370_e41461_d_n7;
        locals.var_qovs_dn8 = assign31370_e41461_d_n8;
        locals.var_qovs_dn9 = assign31370_e41461_d_n9;
        locals.var_qovs_dn10 = assign31370_e41461_d_n10;
        locals.var_qovs_dn11 = assign31370_e41461_d_n11;
        locals.var_qovs_dn12 = assign31370_e41461_d_n12;
        locals.var_qovs_dn13 = assign31370_e41461_d_n13;
        locals.var_qovs_dn14 = assign31370_e41461_d_n14;

        let (assign31380_e41472, assign31380_e41472_d_n0, assign31380_e41472_d_n2, assign31380_e41472_d_n3, assign31380_e41472_d_n4, assign31380_e41472_d_n5, assign31380_e41472_d_n6, assign31380_e41472_d_n7, assign31380_e41472_d_n8, assign31380_e41472_d_n9, assign31380_e41472_d_n10, assign31380_e41472_d_n11, assign31380_e41472_d_n12, assign31380_e41472_d_n13, assign31380_e41472_d_n14,) = {
    if (locals.var_guard729 != 0.0) {
        let assign31380_e41464: f64 = (-locals.var_wact);
        let assign31380_e41466: f64 = (assign31380_e41464 * p.p2);
        let assign31380_e41468: f64 = (assign31380_e41466 * locals.var_cgdof);
        let assign31380_e41470: f64 = (assign31380_e41468 * locals.var_vgd_ov_noswapcv);
        (assign31380_e41470, 0.0, 0.0, 0.0, 0.0, (assign31380_e41468 * locals.var_vgd_ov_noswapcv_dn5), (assign31380_e41468 * locals.var_vgd_ov_noswapcv_dn6), (assign31380_e41468 * locals.var_vgd_ov_noswapcv_dn7), 0.0, 0.0, (assign31380_e41468 * locals.var_vgd_ov_noswapcv_dn10), (assign31380_e41468 * locals.var_vgd_ov_noswapcv_dn11), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn13, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign31380_e41472;
        locals.var_qovd_dn0 = assign31380_e41472_d_n0;
        locals.var_qovd_dn2 = assign31380_e41472_d_n2;
        locals.var_qovd_dn3 = assign31380_e41472_d_n3;
        locals.var_qovd_dn4 = assign31380_e41472_d_n4;
        locals.var_qovd_dn5 = assign31380_e41472_d_n5;
        locals.var_qovd_dn6 = assign31380_e41472_d_n6;
        locals.var_qovd_dn7 = assign31380_e41472_d_n7;
        locals.var_qovd_dn8 = assign31380_e41472_d_n8;
        locals.var_qovd_dn9 = assign31380_e41472_d_n9;
        locals.var_qovd_dn10 = assign31380_e41472_d_n10;
        locals.var_qovd_dn11 = assign31380_e41472_d_n11;
        locals.var_qovd_dn12 = assign31380_e41472_d_n12;
        locals.var_qovd_dn13 = assign31380_e41472_d_n13;
        locals.var_qovd_dn14 = assign31380_e41472_d_n14;

        let (assign31390_e41492, assign31390_e41492_d_n0, assign31390_e41492_d_n2, assign31390_e41492_d_n3, assign31390_e41492_d_n4, assign31390_e41492_d_n5, assign31390_e41492_d_n6, assign31390_e41492_d_n7, assign31390_e41492_d_n8, assign31390_e41492_d_n9, assign31390_e41492_d_n10, assign31390_e41492_d_n11, assign31390_e41492_d_n12, assign31390_e41492_d_n13, assign31390_e41492_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31390_e41477: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign31390_e41479: f64 = (assign31390_e41477 + 0.02);
        let assign31390_e41482: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign31390_e41484: f64 = (assign31390_e41482 + 0.02);
        let assign31390_e41485: f64 = (assign31390_e41479 * assign31390_e41484);
        let assign31390_e41488: f64 = (4.0 * 0.02);
        let assign31390_e41489: f64 = (assign31390_e41485 + assign31390_e41488);
        let assign31390_e41490: f64 = (assign31390_e41489).sqrt();
        (assign31390_e41490, 0.0, 0.0, 0.0, ((((-locals.var_vfbsdr_dn4) * assign31390_e41484) + (assign31390_e41479 * (-locals.var_vfbsdr_dn4))) / (2.0 * assign31390_e41490)), 0.0, 0.0, (((locals.var_vgs_ov_noswap_dn7 * assign31390_e41484) + (assign31390_e41479 * locals.var_vgs_ov_noswap_dn7)) / (2.0 * assign31390_e41490)), 0.0, 0.0, (((locals.var_vgs_ov_noswap_dn10 * assign31390_e41484) + (assign31390_e41479 * locals.var_vgs_ov_noswap_dn10)) / (2.0 * assign31390_e41490)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31390_e41492;
        locals.var_t0_dn0 = assign31390_e41492_d_n0;
        locals.var_t0_dn2 = assign31390_e41492_d_n2;
        locals.var_t0_dn3 = assign31390_e41492_d_n3;
        locals.var_t0_dn4 = assign31390_e41492_d_n4;
        locals.var_t0_dn5 = assign31390_e41492_d_n5;
        locals.var_t0_dn6 = assign31390_e41492_d_n6;
        locals.var_t0_dn7 = assign31390_e41492_d_n7;
        locals.var_t0_dn8 = assign31390_e41492_d_n8;
        locals.var_t0_dn9 = assign31390_e41492_d_n9;
        locals.var_t0_dn10 = assign31390_e41492_d_n10;
        locals.var_t0_dn11 = assign31390_e41492_d_n11;
        locals.var_t0_dn12 = assign31390_e41492_d_n12;
        locals.var_t0_dn13 = assign31390_e41492_d_n13;
        locals.var_t0_dn14 = assign31390_e41492_d_n14;

        let (assign31400_e41505, assign31400_e41505_d_n0, assign31400_e41505_d_n2, assign31400_e41505_d_n3, assign31400_e41505_d_n4, assign31400_e41505_d_n5, assign31400_e41505_d_n6, assign31400_e41505_d_n7, assign31400_e41505_d_n8, assign31400_e41505_d_n9, assign31400_e41505_d_n10, assign31400_e41505_d_n11, assign31400_e41505_d_n12, assign31400_e41505_d_n13, assign31400_e41505_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31400_e41498: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign31400_e41500: f64 = (assign31400_e41498 + 0.02);
        let assign31400_e41502: f64 = (assign31400_e41500 - locals.var_t0);
        let assign31400_e41503: f64 = (0.5 * assign31400_e41502);
        (assign31400_e41503, (0.5 * (-locals.var_t0_dn0)), (0.5 * (-locals.var_t0_dn2)), (0.5 * (-locals.var_t0_dn3)), (0.5 * ((-locals.var_vfbsdr_dn4) - locals.var_t0_dn4)), (0.5 * (-locals.var_t0_dn5)), (0.5 * (-locals.var_t0_dn6)), (0.5 * (locals.var_vgs_ov_noswap_dn7 - locals.var_t0_dn7)), (0.5 * (-locals.var_t0_dn8)), (0.5 * (-locals.var_t0_dn9)), (0.5 * (locals.var_vgs_ov_noswap_dn10 - locals.var_t0_dn10)), (0.5 * (-locals.var_t0_dn11)), (0.5 * (-locals.var_t0_dn12)), (0.5 * (-locals.var_t0_dn13)), (0.5 * (-locals.var_t0_dn14)),)
    } else {
        (locals.var_vgsov, locals.var_vgsov_dn0, locals.var_vgsov_dn2, locals.var_vgsov_dn3, locals.var_vgsov_dn4, locals.var_vgsov_dn5, locals.var_vgsov_dn6, locals.var_vgsov_dn7, locals.var_vgsov_dn8, locals.var_vgsov_dn9, locals.var_vgsov_dn10, locals.var_vgsov_dn11, locals.var_vgsov_dn12, locals.var_vgsov_dn13, locals.var_vgsov_dn14,)
    }
};
        locals.var_vgsov = assign31400_e41505;
        locals.var_vgsov_dn0 = assign31400_e41505_d_n0;
        locals.var_vgsov_dn2 = assign31400_e41505_d_n2;
        locals.var_vgsov_dn3 = assign31400_e41505_d_n3;
        locals.var_vgsov_dn4 = assign31400_e41505_d_n4;
        locals.var_vgsov_dn5 = assign31400_e41505_d_n5;
        locals.var_vgsov_dn6 = assign31400_e41505_d_n6;
        locals.var_vgsov_dn7 = assign31400_e41505_d_n7;
        locals.var_vgsov_dn8 = assign31400_e41505_d_n8;
        locals.var_vgsov_dn9 = assign31400_e41505_d_n9;
        locals.var_vgsov_dn10 = assign31400_e41505_d_n10;
        locals.var_vgsov_dn11 = assign31400_e41505_d_n11;
        locals.var_vgsov_dn12 = assign31400_e41505_d_n12;
        locals.var_vgsov_dn13 = assign31400_e41505_d_n13;
        locals.var_vgsov_dn14 = assign31400_e41505_d_n14;

        let (assign31410_e41523, assign31410_e41523_d_n0, assign31410_e41523_d_n2, assign31410_e41523_d_n3, assign31410_e41523_d_n4, assign31410_e41523_d_n5, assign31410_e41523_d_n6, assign31410_e41523_d_n7, assign31410_e41523_d_n8, assign31410_e41523_d_n9, assign31410_e41523_d_n10, assign31410_e41523_d_n11, assign31410_e41523_d_n12, assign31410_e41523_d_n13, assign31410_e41523_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31410_e41511: f64 = (-locals.var_vgsov);
        let assign31410_e41513: f64 = (assign31410_e41511 / p.p692);
        let assign31410_e41515: f64 = (assign31410_e41513).powf(p.p693);
        let assign31410_e41516: f64 = (1.0 + assign31410_e41515);
        let assign31410_e41519: f64 = (1.0 / p.p693);
        let assign31410_e41520: f64 = (assign31410_e41516).powf(assign31410_e41519);
        let assign31410_e41521: f64 = (locals.var_vgsov / assign31410_e41520);
        (assign31410_e41521, (((locals.var_vgsov_dn0 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn0) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn0) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn0) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn0) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn2 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn2) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn2) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn2) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn2) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn3 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn3) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn3) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn3) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn3) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn4 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn4) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn4) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn4) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn4) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn5 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn5) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn5) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn5) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn5) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn6 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn6) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn6) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn6) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn6) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn7 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn7) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn7) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn7) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn7) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn8 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn8) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn8) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn8) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn8) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn9 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn9) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn9) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn9) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn9) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn10 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn10) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn10) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn10) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn10) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn11 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn11) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn11) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn11) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn11) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn12 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn12) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn12) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn12) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn12) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn13 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn13) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn13) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn13) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn13) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn14 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn14) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn14) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn14) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn14) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign31410_e41523;
        locals.var_t6_dn0 = assign31410_e41523_d_n0;
        locals.var_t6_dn2 = assign31410_e41523_d_n2;
        locals.var_t6_dn3 = assign31410_e41523_d_n3;
        locals.var_t6_dn4 = assign31410_e41523_d_n4;
        locals.var_t6_dn5 = assign31410_e41523_d_n5;
        locals.var_t6_dn6 = assign31410_e41523_d_n6;
        locals.var_t6_dn7 = assign31410_e41523_d_n7;
        locals.var_t6_dn8 = assign31410_e41523_d_n8;
        locals.var_t6_dn9 = assign31410_e41523_d_n9;
        locals.var_t6_dn10 = assign31410_e41523_d_n10;
        locals.var_t6_dn11 = assign31410_e41523_d_n11;
        locals.var_t6_dn12 = assign31410_e41523_d_n12;
        locals.var_t6_dn13 = assign31410_e41523_d_n13;
        locals.var_t6_dn14 = assign31410_e41523_d_n14;

    }

    pub(super) fn stamp_transient_block_99(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (assign31420_e41535, assign31420_e41535_d_n0, assign31420_e41535_d_n2, assign31420_e41535_d_n3, assign31420_e41535_d_n4, assign31420_e41535_d_n5, assign31420_e41535_d_n6, assign31420_e41535_d_n7, assign31420_e41535_d_n8, assign31420_e41535_d_n9, assign31420_e41535_d_n10, assign31420_e41535_d_n11, assign31420_e41535_d_n12, assign31420_e41535_d_n13, assign31420_e41535_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31420_e41529: f64 = (4.0 * locals.var_t6);
        let assign31420_e41531: f64 = (assign31420_e41529 / locals.var_ckappas_i);
        let assign31420_e41532: f64 = (1.0 - assign31420_e41531);
        let assign31420_e41533: f64 = (assign31420_e41532).sqrt();
        (assign31420_e41533, ((-((4.0 * locals.var_t6_dn0) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn2) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn3) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn4) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn5) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn6) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn7) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn8) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn9) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn10) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn11) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn12) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn13) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn14) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31420_e41535;
        locals.var_t1_dn0 = assign31420_e41535_d_n0;
        locals.var_t1_dn2 = assign31420_e41535_d_n2;
        locals.var_t1_dn3 = assign31420_e41535_d_n3;
        locals.var_t1_dn4 = assign31420_e41535_d_n4;
        locals.var_t1_dn5 = assign31420_e41535_d_n5;
        locals.var_t1_dn6 = assign31420_e41535_d_n6;
        locals.var_t1_dn7 = assign31420_e41535_d_n7;
        locals.var_t1_dn8 = assign31420_e41535_d_n8;
        locals.var_t1_dn9 = assign31420_e41535_d_n9;
        locals.var_t1_dn10 = assign31420_e41535_d_n10;
        locals.var_t1_dn11 = assign31420_e41535_d_n11;
        locals.var_t1_dn12 = assign31420_e41535_d_n12;
        locals.var_t1_dn13 = assign31420_e41535_d_n13;
        locals.var_t1_dn14 = assign31420_e41535_d_n14;

        let (assign31430_e41564, assign31430_e41564_d_n0, assign31430_e41564_d_n2, assign31430_e41564_d_n3, assign31430_e41564_d_n4, assign31430_e41564_d_n5, assign31430_e41564_d_n6, assign31430_e41564_d_n7, assign31430_e41564_d_n8, assign31430_e41564_d_n9, assign31430_e41564_d_n10, assign31430_e41564_d_n11, assign31430_e41564_d_n12, assign31430_e41564_d_n13, assign31430_e41564_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31430_e41539: f64 = (-locals.var_wact);
        let assign31430_e41541: f64 = (assign31430_e41539 * p.p2);
        let assign31430_e41544: f64 = (locals.var_cgsof * locals.var_vgs_ov_noswap);
        let assign31430_e41548: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign31430_e41550: f64 = (assign31430_e41548 - locals.var_vgsov);
        let assign31430_e41553: f64 = (0.5 * locals.var_ckappas_i);
        let assign31430_e41555: f64 = (-1.0);
        let assign31430_e41557: f64 = (assign31430_e41555 + locals.var_t1);
        let assign31430_e41558: f64 = (assign31430_e41553 * assign31430_e41557);
        let assign31430_e41559: f64 = (assign31430_e41550 - assign31430_e41558);
        let assign31430_e41560: f64 = (locals.var_cgsl_i * assign31430_e41559);
        let assign31430_e41561: f64 = (assign31430_e41544 + assign31430_e41560);
        let assign31430_e41562: f64 = (assign31430_e41541 * assign31430_e41561);
        (assign31430_e41562, (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn0) - (assign31430_e41553 * locals.var_t1_dn0)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn2) - (assign31430_e41553 * locals.var_t1_dn2)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn3) - (assign31430_e41553 * locals.var_t1_dn3)))), (assign31430_e41541 * (locals.var_cgsl_i * (((-locals.var_vfbsdr_dn4) - locals.var_vgsov_dn4) - (assign31430_e41553 * locals.var_t1_dn4)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn5) - (assign31430_e41553 * locals.var_t1_dn5)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn6) - (assign31430_e41553 * locals.var_t1_dn6)))), (assign31430_e41541 * ((locals.var_cgsof * locals.var_vgs_ov_noswap_dn7) + (locals.var_cgsl_i * ((locals.var_vgs_ov_noswap_dn7 - locals.var_vgsov_dn7) - (assign31430_e41553 * locals.var_t1_dn7))))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn8) - (assign31430_e41553 * locals.var_t1_dn8)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn9) - (assign31430_e41553 * locals.var_t1_dn9)))), (assign31430_e41541 * ((locals.var_cgsof * locals.var_vgs_ov_noswap_dn10) + (locals.var_cgsl_i * ((locals.var_vgs_ov_noswap_dn10 - locals.var_vgsov_dn10) - (assign31430_e41553 * locals.var_t1_dn10))))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn11) - (assign31430_e41553 * locals.var_t1_dn11)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn12) - (assign31430_e41553 * locals.var_t1_dn12)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn13) - (assign31430_e41553 * locals.var_t1_dn13)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn14) - (assign31430_e41553 * locals.var_t1_dn14)))),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn13, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign31430_e41564;
        locals.var_qovs_dn0 = assign31430_e41564_d_n0;
        locals.var_qovs_dn2 = assign31430_e41564_d_n2;
        locals.var_qovs_dn3 = assign31430_e41564_d_n3;
        locals.var_qovs_dn4 = assign31430_e41564_d_n4;
        locals.var_qovs_dn5 = assign31430_e41564_d_n5;
        locals.var_qovs_dn6 = assign31430_e41564_d_n6;
        locals.var_qovs_dn7 = assign31430_e41564_d_n7;
        locals.var_qovs_dn8 = assign31430_e41564_d_n8;
        locals.var_qovs_dn9 = assign31430_e41564_d_n9;
        locals.var_qovs_dn10 = assign31430_e41564_d_n10;
        locals.var_qovs_dn11 = assign31430_e41564_d_n11;
        locals.var_qovs_dn12 = assign31430_e41564_d_n12;
        locals.var_qovs_dn13 = assign31430_e41564_d_n13;
        locals.var_qovs_dn14 = assign31430_e41564_d_n14;

        let (assign31440_e41584, assign31440_e41584_d_n0, assign31440_e41584_d_n2, assign31440_e41584_d_n3, assign31440_e41584_d_n4, assign31440_e41584_d_n5, assign31440_e41584_d_n6, assign31440_e41584_d_n7, assign31440_e41584_d_n8, assign31440_e41584_d_n9, assign31440_e41584_d_n10, assign31440_e41584_d_n11, assign31440_e41584_d_n12, assign31440_e41584_d_n13, assign31440_e41584_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31440_e41569: f64 = (locals.var_vgd_ov_noswapcv - locals.var_vfbsdr);
        let assign31440_e41571: f64 = (assign31440_e41569 + 0.02);
        let assign31440_e41574: f64 = (locals.var_vgd_ov_noswapcv - locals.var_vfbsdr);
        let assign31440_e41576: f64 = (assign31440_e41574 + 0.02);
        let assign31440_e41577: f64 = (assign31440_e41571 * assign31440_e41576);
        let assign31440_e41580: f64 = (4.0 * 0.02);
        let assign31440_e41581: f64 = (assign31440_e41577 + assign31440_e41580);
        let assign31440_e41582: f64 = (assign31440_e41581).sqrt();
        (assign31440_e41582, 0.0, 0.0, 0.0, ((((-locals.var_vfbsdr_dn4) * assign31440_e41576) + (assign31440_e41571 * (-locals.var_vfbsdr_dn4))) / (2.0 * assign31440_e41582)), (((locals.var_vgd_ov_noswapcv_dn5 * assign31440_e41576) + (assign31440_e41571 * locals.var_vgd_ov_noswapcv_dn5)) / (2.0 * assign31440_e41582)), (((locals.var_vgd_ov_noswapcv_dn6 * assign31440_e41576) + (assign31440_e41571 * locals.var_vgd_ov_noswapcv_dn6)) / (2.0 * assign31440_e41582)), (((locals.var_vgd_ov_noswapcv_dn7 * assign31440_e41576) + (assign31440_e41571 * locals.var_vgd_ov_noswapcv_dn7)) / (2.0 * assign31440_e41582)), 0.0, 0.0, (((locals.var_vgd_ov_noswapcv_dn10 * assign31440_e41576) + (assign31440_e41571 * locals.var_vgd_ov_noswapcv_dn10)) / (2.0 * assign31440_e41582)), (((locals.var_vgd_ov_noswapcv_dn11 * assign31440_e41576) + (assign31440_e41571 * locals.var_vgd_ov_noswapcv_dn11)) / (2.0 * assign31440_e41582)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31440_e41584;
        locals.var_t0_dn0 = assign31440_e41584_d_n0;
        locals.var_t0_dn2 = assign31440_e41584_d_n2;
        locals.var_t0_dn3 = assign31440_e41584_d_n3;
        locals.var_t0_dn4 = assign31440_e41584_d_n4;
        locals.var_t0_dn5 = assign31440_e41584_d_n5;
        locals.var_t0_dn6 = assign31440_e41584_d_n6;
        locals.var_t0_dn7 = assign31440_e41584_d_n7;
        locals.var_t0_dn8 = assign31440_e41584_d_n8;
        locals.var_t0_dn9 = assign31440_e41584_d_n9;
        locals.var_t0_dn10 = assign31440_e41584_d_n10;
        locals.var_t0_dn11 = assign31440_e41584_d_n11;
        locals.var_t0_dn12 = assign31440_e41584_d_n12;
        locals.var_t0_dn13 = assign31440_e41584_d_n13;
        locals.var_t0_dn14 = assign31440_e41584_d_n14;

        let (assign31450_e41597, assign31450_e41597_d_n0, assign31450_e41597_d_n2, assign31450_e41597_d_n3, assign31450_e41597_d_n4, assign31450_e41597_d_n5, assign31450_e41597_d_n6, assign31450_e41597_d_n7, assign31450_e41597_d_n8, assign31450_e41597_d_n9, assign31450_e41597_d_n10, assign31450_e41597_d_n11, assign31450_e41597_d_n12, assign31450_e41597_d_n13, assign31450_e41597_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31450_e41590: f64 = (locals.var_vgd_ov_noswapcv - locals.var_vfbsdr);
        let assign31450_e41592: f64 = (assign31450_e41590 + 0.02);
        let assign31450_e41594: f64 = (assign31450_e41592 - locals.var_t0);
        let assign31450_e41595: f64 = (0.5 * assign31450_e41594);
        (assign31450_e41595, (0.5 * (-locals.var_t0_dn0)), (0.5 * (-locals.var_t0_dn2)), (0.5 * (-locals.var_t0_dn3)), (0.5 * ((-locals.var_vfbsdr_dn4) - locals.var_t0_dn4)), (0.5 * (locals.var_vgd_ov_noswapcv_dn5 - locals.var_t0_dn5)), (0.5 * (locals.var_vgd_ov_noswapcv_dn6 - locals.var_t0_dn6)), (0.5 * (locals.var_vgd_ov_noswapcv_dn7 - locals.var_t0_dn7)), (0.5 * (-locals.var_t0_dn8)), (0.5 * (-locals.var_t0_dn9)), (0.5 * (locals.var_vgd_ov_noswapcv_dn10 - locals.var_t0_dn10)), (0.5 * (locals.var_vgd_ov_noswapcv_dn11 - locals.var_t0_dn11)), (0.5 * (-locals.var_t0_dn12)), (0.5 * (-locals.var_t0_dn13)), (0.5 * (-locals.var_t0_dn14)),)
    } else {
        (locals.var_vgdov, locals.var_vgdov_dn0, locals.var_vgdov_dn2, locals.var_vgdov_dn3, locals.var_vgdov_dn4, locals.var_vgdov_dn5, locals.var_vgdov_dn6, locals.var_vgdov_dn7, locals.var_vgdov_dn8, locals.var_vgdov_dn9, locals.var_vgdov_dn10, locals.var_vgdov_dn11, locals.var_vgdov_dn12, locals.var_vgdov_dn13, locals.var_vgdov_dn14,)
    }
};
        locals.var_vgdov = assign31450_e41597;
        locals.var_vgdov_dn0 = assign31450_e41597_d_n0;
        locals.var_vgdov_dn2 = assign31450_e41597_d_n2;
        locals.var_vgdov_dn3 = assign31450_e41597_d_n3;
        locals.var_vgdov_dn4 = assign31450_e41597_d_n4;
        locals.var_vgdov_dn5 = assign31450_e41597_d_n5;
        locals.var_vgdov_dn6 = assign31450_e41597_d_n6;
        locals.var_vgdov_dn7 = assign31450_e41597_d_n7;
        locals.var_vgdov_dn8 = assign31450_e41597_d_n8;
        locals.var_vgdov_dn9 = assign31450_e41597_d_n9;
        locals.var_vgdov_dn10 = assign31450_e41597_d_n10;
        locals.var_vgdov_dn11 = assign31450_e41597_d_n11;
        locals.var_vgdov_dn12 = assign31450_e41597_d_n12;
        locals.var_vgdov_dn13 = assign31450_e41597_d_n13;
        locals.var_vgdov_dn14 = assign31450_e41597_d_n14;

        let (assign31460_e41615, assign31460_e41615_d_n0, assign31460_e41615_d_n2, assign31460_e41615_d_n3, assign31460_e41615_d_n4, assign31460_e41615_d_n5, assign31460_e41615_d_n6, assign31460_e41615_d_n7, assign31460_e41615_d_n8, assign31460_e41615_d_n9, assign31460_e41615_d_n10, assign31460_e41615_d_n11, assign31460_e41615_d_n12, assign31460_e41615_d_n13, assign31460_e41615_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31460_e41603: f64 = (-locals.var_vgdov);
        let assign31460_e41605: f64 = (assign31460_e41603 / p.p690);
        let assign31460_e41607: f64 = (assign31460_e41605).powf(p.p691);
        let assign31460_e41608: f64 = (1.0 + assign31460_e41607);
        let assign31460_e41611: f64 = (1.0 / p.p691);
        let assign31460_e41612: f64 = (assign31460_e41608).powf(assign31460_e41611);
        let assign31460_e41613: f64 = (locals.var_vgdov / assign31460_e41612);
        (assign31460_e41613, (((locals.var_vgdov_dn0 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn0) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn0) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn0) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn0) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn2 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn2) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn2) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn2) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn2) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn3 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn3) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn3) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn3) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn3) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn4 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn4) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn4) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn4) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn4) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn5 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn5) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn5) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn5) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn5) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn6 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn6) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn6) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn6) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn6) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn7 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn7) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn7) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn7) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn7) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn8 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn8) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn8) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn8) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn8) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn9 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn9) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn9) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn9) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn9) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn10 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn10) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn10) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn10) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn10) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn11 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn11) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn11) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn11) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn11) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn12 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn12) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn12) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn12) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn12) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn13 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn13) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn13) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn13) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn13) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn14 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn14) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn14) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn14) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn14) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign31460_e41615;
        locals.var_t6_dn0 = assign31460_e41615_d_n0;
        locals.var_t6_dn2 = assign31460_e41615_d_n2;
        locals.var_t6_dn3 = assign31460_e41615_d_n3;
        locals.var_t6_dn4 = assign31460_e41615_d_n4;
        locals.var_t6_dn5 = assign31460_e41615_d_n5;
        locals.var_t6_dn6 = assign31460_e41615_d_n6;
        locals.var_t6_dn7 = assign31460_e41615_d_n7;
        locals.var_t6_dn8 = assign31460_e41615_d_n8;
        locals.var_t6_dn9 = assign31460_e41615_d_n9;
        locals.var_t6_dn10 = assign31460_e41615_d_n10;
        locals.var_t6_dn11 = assign31460_e41615_d_n11;
        locals.var_t6_dn12 = assign31460_e41615_d_n12;
        locals.var_t6_dn13 = assign31460_e41615_d_n13;
        locals.var_t6_dn14 = assign31460_e41615_d_n14;

        let (assign31470_e41627, assign31470_e41627_d_n0, assign31470_e41627_d_n2, assign31470_e41627_d_n3, assign31470_e41627_d_n4, assign31470_e41627_d_n5, assign31470_e41627_d_n6, assign31470_e41627_d_n7, assign31470_e41627_d_n8, assign31470_e41627_d_n9, assign31470_e41627_d_n10, assign31470_e41627_d_n11, assign31470_e41627_d_n12, assign31470_e41627_d_n13, assign31470_e41627_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31470_e41621: f64 = (4.0 * locals.var_t6);
        let assign31470_e41623: f64 = (assign31470_e41621 / locals.var_ckappad_i);
        let assign31470_e41624: f64 = (1.0 - assign31470_e41623);
        let assign31470_e41625: f64 = (assign31470_e41624).sqrt();
        (assign31470_e41625, ((-((4.0 * locals.var_t6_dn0) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn2) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn3) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn4) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn5) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn6) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn7) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn8) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn9) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn10) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn11) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn12) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn13) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn14) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign31470_e41627;
        locals.var_t2_dn0 = assign31470_e41627_d_n0;
        locals.var_t2_dn2 = assign31470_e41627_d_n2;
        locals.var_t2_dn3 = assign31470_e41627_d_n3;
        locals.var_t2_dn4 = assign31470_e41627_d_n4;
        locals.var_t2_dn5 = assign31470_e41627_d_n5;
        locals.var_t2_dn6 = assign31470_e41627_d_n6;
        locals.var_t2_dn7 = assign31470_e41627_d_n7;
        locals.var_t2_dn8 = assign31470_e41627_d_n8;
        locals.var_t2_dn9 = assign31470_e41627_d_n9;
        locals.var_t2_dn10 = assign31470_e41627_d_n10;
        locals.var_t2_dn11 = assign31470_e41627_d_n11;
        locals.var_t2_dn12 = assign31470_e41627_d_n12;
        locals.var_t2_dn13 = assign31470_e41627_d_n13;
        locals.var_t2_dn14 = assign31470_e41627_d_n14;

        let (assign31480_e41656, assign31480_e41656_d_n0, assign31480_e41656_d_n2, assign31480_e41656_d_n3, assign31480_e41656_d_n4, assign31480_e41656_d_n5, assign31480_e41656_d_n6, assign31480_e41656_d_n7, assign31480_e41656_d_n8, assign31480_e41656_d_n9, assign31480_e41656_d_n10, assign31480_e41656_d_n11, assign31480_e41656_d_n12, assign31480_e41656_d_n13, assign31480_e41656_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31480_e41631: f64 = (-locals.var_wact);
        let assign31480_e41633: f64 = (assign31480_e41631 * p.p2);
        let assign31480_e41636: f64 = (locals.var_cgdof * locals.var_vgd_ov_noswapcv);
        let assign31480_e41640: f64 = (locals.var_vgd_ov_noswapcv - locals.var_vfbsdr);
        let assign31480_e41642: f64 = (assign31480_e41640 - locals.var_vgdov);
        let assign31480_e41645: f64 = (0.5 * locals.var_ckappad_i);
        let assign31480_e41647: f64 = (-1.0);
        let assign31480_e41649: f64 = (assign31480_e41647 + locals.var_t2);
        let assign31480_e41650: f64 = (assign31480_e41645 * assign31480_e41649);
        let assign31480_e41651: f64 = (assign31480_e41642 - assign31480_e41650);
        let assign31480_e41652: f64 = (locals.var_cgdl_i * assign31480_e41651);
        let assign31480_e41653: f64 = (assign31480_e41636 + assign31480_e41652);
        let assign31480_e41654: f64 = (assign31480_e41633 * assign31480_e41653);
        (assign31480_e41654, (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn0) - (assign31480_e41645 * locals.var_t2_dn0)))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn2) - (assign31480_e41645 * locals.var_t2_dn2)))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn3) - (assign31480_e41645 * locals.var_t2_dn3)))), (assign31480_e41633 * (locals.var_cgdl_i * (((-locals.var_vfbsdr_dn4) - locals.var_vgdov_dn4) - (assign31480_e41645 * locals.var_t2_dn4)))), (assign31480_e41633 * ((locals.var_cgdof * locals.var_vgd_ov_noswapcv_dn5) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswapcv_dn5 - locals.var_vgdov_dn5) - (assign31480_e41645 * locals.var_t2_dn5))))), (assign31480_e41633 * ((locals.var_cgdof * locals.var_vgd_ov_noswapcv_dn6) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswapcv_dn6 - locals.var_vgdov_dn6) - (assign31480_e41645 * locals.var_t2_dn6))))), (assign31480_e41633 * ((locals.var_cgdof * locals.var_vgd_ov_noswapcv_dn7) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswapcv_dn7 - locals.var_vgdov_dn7) - (assign31480_e41645 * locals.var_t2_dn7))))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn8) - (assign31480_e41645 * locals.var_t2_dn8)))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn9) - (assign31480_e41645 * locals.var_t2_dn9)))), (assign31480_e41633 * ((locals.var_cgdof * locals.var_vgd_ov_noswapcv_dn10) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswapcv_dn10 - locals.var_vgdov_dn10) - (assign31480_e41645 * locals.var_t2_dn10))))), (assign31480_e41633 * ((locals.var_cgdof * locals.var_vgd_ov_noswapcv_dn11) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswapcv_dn11 - locals.var_vgdov_dn11) - (assign31480_e41645 * locals.var_t2_dn11))))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn12) - (assign31480_e41645 * locals.var_t2_dn12)))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn13) - (assign31480_e41645 * locals.var_t2_dn13)))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn14) - (assign31480_e41645 * locals.var_t2_dn14)))),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn13, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign31480_e41656;
        locals.var_qovd_dn0 = assign31480_e41656_d_n0;
        locals.var_qovd_dn2 = assign31480_e41656_d_n2;
        locals.var_qovd_dn3 = assign31480_e41656_d_n3;
        locals.var_qovd_dn4 = assign31480_e41656_d_n4;
        locals.var_qovd_dn5 = assign31480_e41656_d_n5;
        locals.var_qovd_dn6 = assign31480_e41656_d_n6;
        locals.var_qovd_dn7 = assign31480_e41656_d_n7;
        locals.var_qovd_dn8 = assign31480_e41656_d_n8;
        locals.var_qovd_dn9 = assign31480_e41656_d_n9;
        locals.var_qovd_dn10 = assign31480_e41656_d_n10;
        locals.var_qovd_dn11 = assign31480_e41656_d_n11;
        locals.var_qovd_dn12 = assign31480_e41656_d_n12;
        locals.var_qovd_dn13 = assign31480_e41656_d_n13;
        locals.var_qovd_dn14 = assign31480_e41656_d_n14;

        let assign31490_e41658: f64 = (-locals.var_devsign);
        let assign31490_e41660: f64 = (assign31490_e41658 * p.p2);
        let assign31490_e41662: f64 = (assign31490_e41660 * locals.var_lact);
        let assign31490_e41664: f64 = (assign31490_e41662 * p.p673);
        let assign31490_e41666: f64 = (assign31490_e41664 * (nv10 - nv11));
        locals.var_qovb = assign31490_e41666;
        locals.var_qovb_dn0 = 0.0;
        locals.var_qovb_dn2 = 0.0;
        locals.var_qovb_dn3 = 0.0;
        locals.var_qovb_dn4 = 0.0;
        locals.var_qovb_dn5 = 0.0;
        locals.var_qovb_dn6 = 0.0;
        locals.var_qovb_dn7 = 0.0;
        locals.var_qovb_dn8 = 0.0;
        locals.var_qovb_dn9 = 0.0;
        locals.var_qovb_dn10 = assign31490_e41664;
        locals.var_qovb_dn11 = (-assign31490_e41664);
        locals.var_qovb_dn12 = 0.0;
        locals.var_qovb_dn13 = 0.0;
        locals.var_qovb_dn14 = 0.0;

        let assign31510_e41675: f64 = if p.p37 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard730 = assign31510_e41675;

        let (assign31520_e41684, assign31520_e41684_d_n0, assign31520_e41684_d_n2, assign31520_e41684_d_n3, assign31520_e41684_d_n4, assign31520_e41684_d_n5, assign31520_e41684_d_n6, assign31520_e41684_d_n7, assign31520_e41684_d_n8, assign31520_e41684_d_n9, assign31520_e41684_d_n10, assign31520_e41684_d_n11, assign31520_e41684_d_n12, assign31520_e41684_d_n13, assign31520_e41684_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31520_e41679: f64 = (locals.var_ndepedge_i / locals.var_ni);
        let assign31520_e41681: f64 = (assign31520_e41679).max(1e-38);
        let assign31520_e41682: f64 = (assign31520_e41681).ln();
        (assign31520_e41682, (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn12) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681),)
    } else {
        (locals.var_phib_edge, locals.var_phib_edge_dn0, locals.var_phib_edge_dn2, locals.var_phib_edge_dn3, locals.var_phib_edge_dn4, locals.var_phib_edge_dn5, locals.var_phib_edge_dn6, locals.var_phib_edge_dn7, locals.var_phib_edge_dn8, locals.var_phib_edge_dn9, locals.var_phib_edge_dn10, locals.var_phib_edge_dn11, locals.var_phib_edge_dn12, locals.var_phib_edge_dn13, locals.var_phib_edge_dn14,)
    }
};
        locals.var_phib_edge = assign31520_e41684;
        locals.var_phib_edge_dn0 = assign31520_e41684_d_n0;
        locals.var_phib_edge_dn2 = assign31520_e41684_d_n2;
        locals.var_phib_edge_dn3 = assign31520_e41684_d_n3;
        locals.var_phib_edge_dn4 = assign31520_e41684_d_n4;
        locals.var_phib_edge_dn5 = assign31520_e41684_d_n5;
        locals.var_phib_edge_dn6 = assign31520_e41684_d_n6;
        locals.var_phib_edge_dn7 = assign31520_e41684_d_n7;
        locals.var_phib_edge_dn8 = assign31520_e41684_d_n8;
        locals.var_phib_edge_dn9 = assign31520_e41684_d_n9;
        locals.var_phib_edge_dn10 = assign31520_e41684_d_n10;
        locals.var_phib_edge_dn11 = assign31520_e41684_d_n11;
        locals.var_phib_edge_dn12 = assign31520_e41684_d_n12;
        locals.var_phib_edge_dn13 = assign31520_e41684_d_n13;
        locals.var_phib_edge_dn14 = assign31520_e41684_d_n14;

        let (assign31530_e41696, assign31530_e41696_d_n0, assign31530_e41696_d_n2, assign31530_e41696_d_n3, assign31530_e41696_d_n4, assign31530_e41696_d_n5, assign31530_e41696_d_n6, assign31530_e41696_d_n7, assign31530_e41696_d_n8, assign31530_e41696_d_n9, assign31530_e41696_d_n10, assign31530_e41696_d_n11, assign31530_e41696_d_n12, assign31530_e41696_d_n13, assign31530_e41696_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31530_e41689: f64 = (locals.var_vt * locals.var_phib_edge);
        let assign31530_e41690: f64 = (0.4 + assign31530_e41689);
        let assign31530_e41692: f64 = (assign31530_e41690 + locals.var_phin_i);
        let assign31530_e41694: f64 = (assign31530_e41692).max(0.4);
        (assign31530_e41694, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn0) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn2) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn3) } else { 0.0 }, if assign31530_e41692 >= 0.4 { ((locals.var_vt_dn4 * locals.var_phib_edge) + (locals.var_vt * locals.var_phib_edge_dn4)) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn5) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn6) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn7) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn8) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn9) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn10) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn11) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn12) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn13) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn14) } else { 0.0 },)
    } else {
        (locals.var_phist, locals.var_phist_dn0, locals.var_phist_dn2, locals.var_phist_dn3, locals.var_phist_dn4, locals.var_phist_dn5, locals.var_phist_dn6, locals.var_phist_dn7, locals.var_phist_dn8, locals.var_phist_dn9, locals.var_phist_dn10, locals.var_phist_dn11, locals.var_phist_dn12, locals.var_phist_dn13, locals.var_phist_dn14,)
    }
};
        locals.var_phist = assign31530_e41696;
        locals.var_phist_dn0 = assign31530_e41696_d_n0;
        locals.var_phist_dn2 = assign31530_e41696_d_n2;
        locals.var_phist_dn3 = assign31530_e41696_d_n3;
        locals.var_phist_dn4 = assign31530_e41696_d_n4;
        locals.var_phist_dn5 = assign31530_e41696_d_n5;
        locals.var_phist_dn6 = assign31530_e41696_d_n6;
        locals.var_phist_dn7 = assign31530_e41696_d_n7;
        locals.var_phist_dn8 = assign31530_e41696_d_n8;
        locals.var_phist_dn9 = assign31530_e41696_d_n9;
        locals.var_phist_dn10 = assign31530_e41696_d_n10;
        locals.var_phist_dn11 = assign31530_e41696_d_n11;
        locals.var_phist_dn12 = assign31530_e41696_d_n12;
        locals.var_phist_dn13 = assign31530_e41696_d_n13;
        locals.var_phist_dn14 = assign31530_e41696_d_n14;

        let (assign31540_e41707, assign31540_e41707_d_n0, assign31540_e41707_d_n2, assign31540_e41707_d_n3, assign31540_e41707_d_n4, assign31540_e41707_d_n5, assign31540_e41707_d_n6, assign31540_e41707_d_n7, assign31540_e41707_d_n8, assign31540_e41707_d_n9, assign31540_e41707_d_n10, assign31540_e41707_d_n11, assign31540_e41707_d_n12, assign31540_e41707_d_n13, assign31540_e41707_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31540_e41700: f64 = (2.0 * locals.var_epssi);
        let assign31540_e41703: f64 = (1.60219e-19 * locals.var_ndepedge_i);
        let assign31540_e41704: f64 = (assign31540_e41700 / assign31540_e41703);
        let assign31540_e41705: f64 = (assign31540_e41704).sqrt();
        (assign31540_e41705, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1dep, locals.var_t1dep_dn0, locals.var_t1dep_dn2, locals.var_t1dep_dn3, locals.var_t1dep_dn4, locals.var_t1dep_dn5, locals.var_t1dep_dn6, locals.var_t1dep_dn7, locals.var_t1dep_dn8, locals.var_t1dep_dn9, locals.var_t1dep_dn10, locals.var_t1dep_dn11, locals.var_t1dep_dn12, locals.var_t1dep_dn13, locals.var_t1dep_dn14,)
    }
};
        locals.var_t1dep = assign31540_e41707;
        locals.var_t1dep_dn0 = assign31540_e41707_d_n0;
        locals.var_t1dep_dn2 = assign31540_e41707_d_n2;
        locals.var_t1dep_dn3 = assign31540_e41707_d_n3;
        locals.var_t1dep_dn4 = assign31540_e41707_d_n4;
        locals.var_t1dep_dn5 = assign31540_e41707_d_n5;
        locals.var_t1dep_dn6 = assign31540_e41707_d_n6;
        locals.var_t1dep_dn7 = assign31540_e41707_d_n7;
        locals.var_t1dep_dn8 = assign31540_e41707_d_n8;
        locals.var_t1dep_dn9 = assign31540_e41707_d_n9;
        locals.var_t1dep_dn10 = assign31540_e41707_d_n10;
        locals.var_t1dep_dn11 = assign31540_e41707_d_n11;
        locals.var_t1dep_dn12 = assign31540_e41707_d_n12;
        locals.var_t1dep_dn13 = assign31540_e41707_d_n13;
        locals.var_t1dep_dn14 = assign31540_e41707_d_n14;

        let (assign31550_e41784, assign31550_e41784_d_n4,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31550_e41714: f64 = (locals.var_tratio - 1.0);
        let assign31550_e41715: f64 = (locals.var_tnfactoredge_i * assign31550_e41714);
        let assign31550_e41716: f64 = (1.0 + assign31550_e41715);
        let assign31550_e41718: f64 = (-10000.0);
        let assign31550_e41720: f64 = (assign31550_e41718 * 0.001);
        let (assign31550_e41781, assign31550_e41781_d_n4,) = {
            if (!(assign31550_e41716 < assign31550_e41720)) {
                let assign31550_e41728: f64 = (locals.var_tratio - 1.0);
                let assign31550_e41729: f64 = (locals.var_tnfactoredge_i * assign31550_e41728);
                let assign31550_e41730: f64 = (1.0 + assign31550_e41729);
                let assign31550_e41735: f64 = (locals.var_tratio - 1.0);
                let assign31550_e41736: f64 = (locals.var_tnfactoredge_i * assign31550_e41735);
                let assign31550_e41737: f64 = (1.0 + assign31550_e41736);
                let assign31550_e41742: f64 = (locals.var_tratio - 1.0);
                let assign31550_e41743: f64 = (locals.var_tnfactoredge_i * assign31550_e41742);
                let assign31550_e41744: f64 = (1.0 + assign31550_e41743);
                let assign31550_e41745: f64 = (assign31550_e41737 * assign31550_e41744);
                let assign31550_e41748: f64 = (4.0 * 0.001);
                let assign31550_e41750: f64 = (assign31550_e41748 * 0.001);
                let assign31550_e41751: f64 = (assign31550_e41745 + assign31550_e41750);
                let assign31550_e41752: f64 = (assign31550_e41751).sqrt();
                let assign31550_e41753: f64 = (assign31550_e41730 + assign31550_e41752);
                let assign31550_e41754: f64 = (0.5 * assign31550_e41753);
                (assign31550_e41754, (0.5 * ((locals.var_tnfactoredge_i * locals.var_tratio_dn4) + ((((locals.var_tnfactoredge_i * locals.var_tratio_dn4) * assign31550_e41744) + (assign31550_e41737 * (locals.var_tnfactoredge_i * locals.var_tratio_dn4))) / (2.0 * assign31550_e41752)))),)
            } else {
                let assign31550_e41759: f64 = (locals.var_tratio - 1.0);
                let assign31550_e41760: f64 = (locals.var_tnfactoredge_i * assign31550_e41759);
                let assign31550_e41761: f64 = (1.0 + assign31550_e41760);
                let assign31550_e41763: f64 = (-10000.0);
                let assign31550_e41765: f64 = (assign31550_e41763 * 0.001);
                let (assign31550_e41780, assign31550_e41780_d_n4,) = {
                    if (assign31550_e41761 < assign31550_e41765) {
                        let assign31550_e41768: f64 = (-0.001);
                        let assign31550_e41770: f64 = (assign31550_e41768 * 0.001);
                        let assign31550_e41775: f64 = (locals.var_tratio - 1.0);
                        let assign31550_e41776: f64 = (locals.var_tnfactoredge_i * assign31550_e41775);
                        let assign31550_e41777: f64 = (1.0 + assign31550_e41776);
                        let assign31550_e41778: f64 = (assign31550_e41770 / assign31550_e41777);
                        (assign31550_e41778, (-((assign31550_e41770 * (locals.var_tnfactoredge_i * locals.var_tratio_dn4)) / (assign31550_e41777 * assign31550_e41777))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign31550_e41780, assign31550_e41780_d_n4,)
            }
        };
        let assign31550_e41782: f64 = (locals.var_nfactoredge_i * assign31550_e41781);
        (assign31550_e41782, (locals.var_nfactoredge_i * assign31550_e41781_d_n4),)
    } else {
        (locals.var_nfactoredge_t, locals.var_nfactoredge_t_dn4,)
    }
};
        locals.var_nfactoredge_t = assign31550_e41784;
        locals.var_nfactoredge_t_dn4 = assign31550_e41784_d_n4;

        let (assign31560_e41796, assign31560_e41796_d_n0, assign31560_e41796_d_n2, assign31560_e41796_d_n3, assign31560_e41796_d_n4, assign31560_e41796_d_n5, assign31560_e41796_d_n6, assign31560_e41796_d_n7, assign31560_e41796_d_n8, assign31560_e41796_d_n9, assign31560_e41796_d_n10, assign31560_e41796_d_n11, assign31560_e41796_d_n12, assign31560_e41796_d_n13, assign31560_e41796_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31560_e41791: f64 = (locals.var_tratio - 1.0);
        let assign31560_e41792: f64 = (locals.var_teta0edge_i * assign31560_e41791);
        let assign31560_e41793: f64 = (1.0 + assign31560_e41792);
        let assign31560_e41794: f64 = (locals.var_eta0edge_i * assign31560_e41793);
        (assign31560_e41794, (locals.var_eta0edge_i_dn0 * assign31560_e41793), (locals.var_eta0edge_i_dn2 * assign31560_e41793), (locals.var_eta0edge_i_dn3 * assign31560_e41793), ((locals.var_eta0edge_i_dn4 * assign31560_e41793) + (locals.var_eta0edge_i * (locals.var_teta0edge_i * locals.var_tratio_dn4))), (locals.var_eta0edge_i_dn5 * assign31560_e41793), (locals.var_eta0edge_i_dn6 * assign31560_e41793), (locals.var_eta0edge_i_dn7 * assign31560_e41793), (locals.var_eta0edge_i_dn8 * assign31560_e41793), (locals.var_eta0edge_i_dn9 * assign31560_e41793), (locals.var_eta0edge_i_dn10 * assign31560_e41793), (locals.var_eta0edge_i_dn11 * assign31560_e41793), (locals.var_eta0edge_i_dn12 * assign31560_e41793), (locals.var_eta0edge_i_dn13 * assign31560_e41793), (locals.var_eta0edge_i_dn14 * assign31560_e41793),)
    } else {
        (locals.var_eta0edge_t, locals.var_eta0edge_t_dn0, locals.var_eta0edge_t_dn2, locals.var_eta0edge_t_dn3, locals.var_eta0edge_t_dn4, locals.var_eta0edge_t_dn5, locals.var_eta0edge_t_dn6, locals.var_eta0edge_t_dn7, locals.var_eta0edge_t_dn8, locals.var_eta0edge_t_dn9, locals.var_eta0edge_t_dn10, locals.var_eta0edge_t_dn11, locals.var_eta0edge_t_dn12, locals.var_eta0edge_t_dn13, locals.var_eta0edge_t_dn14,)
    }
};
        locals.var_eta0edge_t = assign31560_e41796;
        locals.var_eta0edge_t_dn0 = assign31560_e41796_d_n0;
        locals.var_eta0edge_t_dn2 = assign31560_e41796_d_n2;
        locals.var_eta0edge_t_dn3 = assign31560_e41796_d_n3;
        locals.var_eta0edge_t_dn4 = assign31560_e41796_d_n4;
        locals.var_eta0edge_t_dn5 = assign31560_e41796_d_n5;
        locals.var_eta0edge_t_dn6 = assign31560_e41796_d_n6;
        locals.var_eta0edge_t_dn7 = assign31560_e41796_d_n7;
        locals.var_eta0edge_t_dn8 = assign31560_e41796_d_n8;
        locals.var_eta0edge_t_dn9 = assign31560_e41796_d_n9;
        locals.var_eta0edge_t_dn10 = assign31560_e41796_d_n10;
        locals.var_eta0edge_t_dn11 = assign31560_e41796_d_n11;
        locals.var_eta0edge_t_dn12 = assign31560_e41796_d_n12;
        locals.var_eta0edge_t_dn13 = assign31560_e41796_d_n13;
        locals.var_eta0edge_t_dn14 = assign31560_e41796_d_n14;

        let assign31570_e41802: f64 = (locals.var_phist - locals.var_vbsx);
        let assign31570_e41804: f64 = (-2500.0);
        let assign31570_e41806: f64 = (assign31570_e41804 * 0.1);
        let assign31570_e41808: f64 = if ((0.05 == 0.0) && (assign31570_e41802 < assign31570_e41806)) { 1.0 } else { 0.0 };
        locals.var_guard731 = assign31570_e41808;

        let (assign31580_e41823, assign31580_e41823_d_n0, assign31580_e41823_d_n2, assign31580_e41823_d_n3, assign31580_e41823_d_n4, assign31580_e41823_d_n5, assign31580_e41823_d_n6, assign31580_e41823_d_n7, assign31580_e41823_d_n8, assign31580_e41823_d_n9, assign31580_e41823_d_n10, assign31580_e41823_d_n11, assign31580_e41823_d_n12, assign31580_e41823_d_n13, assign31580_e41823_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard731 != 0.0)) {
        let assign31580_e41813: f64 = (-0.1);
        let assign31580_e41815: f64 = (assign31580_e41813 * 0.1);
        let assign31580_e41819: f64 = (locals.var_phist - locals.var_vbsx);
        let assign31580_e41820: f64 = (16.0 * assign31580_e41819);
        let assign31580_e41821: f64 = (assign31580_e41815 / assign31580_e41820);
        (assign31580_e41821, (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn0 - locals.var_vbsx_dn0))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn2 - locals.var_vbsx_dn2))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn3 - locals.var_vbsx_dn3))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn4 - locals.var_vbsx_dn4))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn5 - locals.var_vbsx_dn5))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn6 - locals.var_vbsx_dn6))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn7 - locals.var_vbsx_dn7))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn8 - locals.var_vbsx_dn8))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn9 - locals.var_vbsx_dn9))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn10 - locals.var_vbsx_dn10))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn11 - locals.var_vbsx_dn11))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn12 - locals.var_vbsx_dn12))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn13 - locals.var_vbsx_dn13))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn14 - locals.var_vbsx_dn14))) / (assign31580_e41820 * assign31580_e41820))),)
    } else {
        (locals.var_phistvbs, locals.var_phistvbs_dn0, locals.var_phistvbs_dn2, locals.var_phistvbs_dn3, locals.var_phistvbs_dn4, locals.var_phistvbs_dn5, locals.var_phistvbs_dn6, locals.var_phistvbs_dn7, locals.var_phistvbs_dn8, locals.var_phistvbs_dn9, locals.var_phistvbs_dn10, locals.var_phistvbs_dn11, locals.var_phistvbs_dn12, locals.var_phistvbs_dn13, locals.var_phistvbs_dn14,)
    }
};
        locals.var_phistvbs = assign31580_e41823;
        locals.var_phistvbs_dn0 = assign31580_e41823_d_n0;
        locals.var_phistvbs_dn2 = assign31580_e41823_d_n2;
        locals.var_phistvbs_dn3 = assign31580_e41823_d_n3;
        locals.var_phistvbs_dn4 = assign31580_e41823_d_n4;
        locals.var_phistvbs_dn5 = assign31580_e41823_d_n5;
        locals.var_phistvbs_dn6 = assign31580_e41823_d_n6;
        locals.var_phistvbs_dn7 = assign31580_e41823_d_n7;
        locals.var_phistvbs_dn8 = assign31580_e41823_d_n8;
        locals.var_phistvbs_dn9 = assign31580_e41823_d_n9;
        locals.var_phistvbs_dn10 = assign31580_e41823_d_n10;
        locals.var_phistvbs_dn11 = assign31580_e41823_d_n11;
        locals.var_phistvbs_dn12 = assign31580_e41823_d_n12;
        locals.var_phistvbs_dn13 = assign31580_e41823_d_n13;
        locals.var_phistvbs_dn14 = assign31580_e41823_d_n14;

        let (assign31590_e41855, assign31590_e41855_d_n0, assign31590_e41855_d_n2, assign31590_e41855_d_n3, assign31590_e41855_d_n4, assign31590_e41855_d_n5, assign31590_e41855_d_n6, assign31590_e41855_d_n7, assign31590_e41855_d_n8, assign31590_e41855_d_n9, assign31590_e41855_d_n10, assign31590_e41855_d_n11, assign31590_e41855_d_n12, assign31590_e41855_d_n13, assign31590_e41855_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard731 == 0.0)) {
        let assign31590_e41831: f64 = (locals.var_phist - locals.var_vbsx);
        let assign31590_e41833: f64 = (assign31590_e41831 + 0.05);
        let assign31590_e41836: f64 = (locals.var_phist - locals.var_vbsx);
        let assign31590_e41838: f64 = (assign31590_e41836 - 0.05);
        let assign31590_e41841: f64 = (locals.var_phist - locals.var_vbsx);
        let assign31590_e41843: f64 = (assign31590_e41841 - 0.05);
        let assign31590_e41844: f64 = (assign31590_e41838 * assign31590_e41843);
        let assign31590_e41847: f64 = (0.25 * 0.1);
        let assign31590_e41849: f64 = (assign31590_e41847 * 0.1);
        let assign31590_e41850: f64 = (assign31590_e41844 + assign31590_e41849);
        let assign31590_e41851: f64 = (assign31590_e41850).sqrt();
        let assign31590_e41852: f64 = (assign31590_e41833 + assign31590_e41851);
        let assign31590_e41853: f64 = (0.5 * assign31590_e41852);
        (assign31590_e41853, (0.5 * ((locals.var_phist_dn0 - locals.var_vbsx_dn0) + ((((locals.var_phist_dn0 - locals.var_vbsx_dn0) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn0 - locals.var_vbsx_dn0))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn2 - locals.var_vbsx_dn2) + ((((locals.var_phist_dn2 - locals.var_vbsx_dn2) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn2 - locals.var_vbsx_dn2))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn3 - locals.var_vbsx_dn3) + ((((locals.var_phist_dn3 - locals.var_vbsx_dn3) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn3 - locals.var_vbsx_dn3))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn4 - locals.var_vbsx_dn4) + ((((locals.var_phist_dn4 - locals.var_vbsx_dn4) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn4 - locals.var_vbsx_dn4))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn5 - locals.var_vbsx_dn5) + ((((locals.var_phist_dn5 - locals.var_vbsx_dn5) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn5 - locals.var_vbsx_dn5))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn6 - locals.var_vbsx_dn6) + ((((locals.var_phist_dn6 - locals.var_vbsx_dn6) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn6 - locals.var_vbsx_dn6))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn7 - locals.var_vbsx_dn7) + ((((locals.var_phist_dn7 - locals.var_vbsx_dn7) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn7 - locals.var_vbsx_dn7))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn8 - locals.var_vbsx_dn8) + ((((locals.var_phist_dn8 - locals.var_vbsx_dn8) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn8 - locals.var_vbsx_dn8))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn9 - locals.var_vbsx_dn9) + ((((locals.var_phist_dn9 - locals.var_vbsx_dn9) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn9 - locals.var_vbsx_dn9))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn10 - locals.var_vbsx_dn10) + ((((locals.var_phist_dn10 - locals.var_vbsx_dn10) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn10 - locals.var_vbsx_dn10))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn11 - locals.var_vbsx_dn11) + ((((locals.var_phist_dn11 - locals.var_vbsx_dn11) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn11 - locals.var_vbsx_dn11))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn12 - locals.var_vbsx_dn12) + ((((locals.var_phist_dn12 - locals.var_vbsx_dn12) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn12 - locals.var_vbsx_dn12))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn13 - locals.var_vbsx_dn13) + ((((locals.var_phist_dn13 - locals.var_vbsx_dn13) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn13 - locals.var_vbsx_dn13))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn14 - locals.var_vbsx_dn14) + ((((locals.var_phist_dn14 - locals.var_vbsx_dn14) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn14 - locals.var_vbsx_dn14))) / (2.0 * assign31590_e41851)))),)
    } else {
        (locals.var_phistvbs, locals.var_phistvbs_dn0, locals.var_phistvbs_dn2, locals.var_phistvbs_dn3, locals.var_phistvbs_dn4, locals.var_phistvbs_dn5, locals.var_phistvbs_dn6, locals.var_phistvbs_dn7, locals.var_phistvbs_dn8, locals.var_phistvbs_dn9, locals.var_phistvbs_dn10, locals.var_phistvbs_dn11, locals.var_phistvbs_dn12, locals.var_phistvbs_dn13, locals.var_phistvbs_dn14,)
    }
};
        locals.var_phistvbs = assign31590_e41855;
        locals.var_phistvbs_dn0 = assign31590_e41855_d_n0;
        locals.var_phistvbs_dn2 = assign31590_e41855_d_n2;
        locals.var_phistvbs_dn3 = assign31590_e41855_d_n3;
        locals.var_phistvbs_dn4 = assign31590_e41855_d_n4;
        locals.var_phistvbs_dn5 = assign31590_e41855_d_n5;
        locals.var_phistvbs_dn6 = assign31590_e41855_d_n6;
        locals.var_phistvbs_dn7 = assign31590_e41855_d_n7;
        locals.var_phistvbs_dn8 = assign31590_e41855_d_n8;
        locals.var_phistvbs_dn9 = assign31590_e41855_d_n9;
        locals.var_phistvbs_dn10 = assign31590_e41855_d_n10;
        locals.var_phistvbs_dn11 = assign31590_e41855_d_n11;
        locals.var_phistvbs_dn12 = assign31590_e41855_d_n12;
        locals.var_phistvbs_dn13 = assign31590_e41855_d_n13;
        locals.var_phistvbs_dn14 = assign31590_e41855_d_n14;

        let (assign31600_e41860, assign31600_e41860_d_n0, assign31600_e41860_d_n2, assign31600_e41860_d_n3, assign31600_e41860_d_n4, assign31600_e41860_d_n5, assign31600_e41860_d_n6, assign31600_e41860_d_n7, assign31600_e41860_d_n8, assign31600_e41860_d_n9, assign31600_e41860_d_n10, assign31600_e41860_d_n11, assign31600_e41860_d_n12, assign31600_e41860_d_n13, assign31600_e41860_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31600_e41858: f64 = (locals.var_phistvbs).sqrt();
        (assign31600_e41858, (locals.var_phistvbs_dn0 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn2 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn3 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn4 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn5 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn6 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn7 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn8 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn9 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn10 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn11 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn12 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn13 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn14 / (2.0 * assign31600_e41858)),)
    } else {
        (locals.var_sqrtphistvbs, locals.var_sqrtphistvbs_dn0, locals.var_sqrtphistvbs_dn2, locals.var_sqrtphistvbs_dn3, locals.var_sqrtphistvbs_dn4, locals.var_sqrtphistvbs_dn5, locals.var_sqrtphistvbs_dn6, locals.var_sqrtphistvbs_dn7, locals.var_sqrtphistvbs_dn8, locals.var_sqrtphistvbs_dn9, locals.var_sqrtphistvbs_dn10, locals.var_sqrtphistvbs_dn11, locals.var_sqrtphistvbs_dn12, locals.var_sqrtphistvbs_dn13, locals.var_sqrtphistvbs_dn14,)
    }
};
        locals.var_sqrtphistvbs = assign31600_e41860;
        locals.var_sqrtphistvbs_dn0 = assign31600_e41860_d_n0;
        locals.var_sqrtphistvbs_dn2 = assign31600_e41860_d_n2;
        locals.var_sqrtphistvbs_dn3 = assign31600_e41860_d_n3;
        locals.var_sqrtphistvbs_dn4 = assign31600_e41860_d_n4;
        locals.var_sqrtphistvbs_dn5 = assign31600_e41860_d_n5;
        locals.var_sqrtphistvbs_dn6 = assign31600_e41860_d_n6;
        locals.var_sqrtphistvbs_dn7 = assign31600_e41860_d_n7;
        locals.var_sqrtphistvbs_dn8 = assign31600_e41860_d_n8;
        locals.var_sqrtphistvbs_dn9 = assign31600_e41860_d_n9;
        locals.var_sqrtphistvbs_dn10 = assign31600_e41860_d_n10;
        locals.var_sqrtphistvbs_dn11 = assign31600_e41860_d_n11;
        locals.var_sqrtphistvbs_dn12 = assign31600_e41860_d_n12;
        locals.var_sqrtphistvbs_dn13 = assign31600_e41860_d_n13;
        locals.var_sqrtphistvbs_dn14 = assign31600_e41860_d_n14;

    }

    pub(super) fn stamp_transient_block_100(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31610_e41866, assign31610_e41866_d_n0, assign31610_e41866_d_n2, assign31610_e41866_d_n3, assign31610_e41866_d_n4, assign31610_e41866_d_n5, assign31610_e41866_d_n6, assign31610_e41866_d_n7, assign31610_e41866_d_n8, assign31610_e41866_d_n9, assign31610_e41866_d_n10, assign31610_e41866_d_n11, assign31610_e41866_d_n12, assign31610_e41866_d_n13, assign31610_e41866_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31610_e41864: f64 = (locals.var_t1dep * locals.var_sqrtphistvbs);
        (assign31610_e41864, ((locals.var_t1dep_dn0 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn0)), ((locals.var_t1dep_dn2 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn2)), ((locals.var_t1dep_dn3 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn3)), ((locals.var_t1dep_dn4 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn4)), ((locals.var_t1dep_dn5 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn5)), ((locals.var_t1dep_dn6 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn6)), ((locals.var_t1dep_dn7 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn7)), ((locals.var_t1dep_dn8 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn8)), ((locals.var_t1dep_dn9 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn9)), ((locals.var_t1dep_dn10 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn10)), ((locals.var_t1dep_dn11 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn11)), ((locals.var_t1dep_dn12 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn12)), ((locals.var_t1dep_dn13 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn13)), ((locals.var_t1dep_dn14 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn14)),)
    } else {
        (locals.var_xdep, locals.var_xdep_dn0, locals.var_xdep_dn2, locals.var_xdep_dn3, locals.var_xdep_dn4, locals.var_xdep_dn5, locals.var_xdep_dn6, locals.var_xdep_dn7, locals.var_xdep_dn8, locals.var_xdep_dn9, locals.var_xdep_dn10, locals.var_xdep_dn11, locals.var_xdep_dn12, locals.var_xdep_dn13, locals.var_xdep_dn14,)
    }
};
        locals.var_xdep = assign31610_e41866;
        locals.var_xdep_dn0 = assign31610_e41866_d_n0;
        locals.var_xdep_dn2 = assign31610_e41866_d_n2;
        locals.var_xdep_dn3 = assign31610_e41866_d_n3;
        locals.var_xdep_dn4 = assign31610_e41866_d_n4;
        locals.var_xdep_dn5 = assign31610_e41866_d_n5;
        locals.var_xdep_dn6 = assign31610_e41866_d_n6;
        locals.var_xdep_dn7 = assign31610_e41866_d_n7;
        locals.var_xdep_dn8 = assign31610_e41866_d_n8;
        locals.var_xdep_dn9 = assign31610_e41866_d_n9;
        locals.var_xdep_dn10 = assign31610_e41866_d_n10;
        locals.var_xdep_dn11 = assign31610_e41866_d_n11;
        locals.var_xdep_dn12 = assign31610_e41866_d_n12;
        locals.var_xdep_dn13 = assign31610_e41866_d_n13;
        locals.var_xdep_dn14 = assign31610_e41866_d_n14;

        let (assign31620_e41872, assign31620_e41872_d_n0, assign31620_e41872_d_n2, assign31620_e41872_d_n3, assign31620_e41872_d_n4, assign31620_e41872_d_n5, assign31620_e41872_d_n6, assign31620_e41872_d_n7, assign31620_e41872_d_n8, assign31620_e41872_d_n9, assign31620_e41872_d_n10, assign31620_e41872_d_n11, assign31620_e41872_d_n12, assign31620_e41872_d_n13, assign31620_e41872_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31620_e41870: f64 = (locals.var_epssi / locals.var_xdep);
        (assign31620_e41870, (-((locals.var_epssi * locals.var_xdep_dn0) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn2) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn3) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn4) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn5) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn6) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn7) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn8) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn9) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn10) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn11) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn12) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn13) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn14) / (locals.var_xdep * locals.var_xdep))),)
    } else {
        (locals.var_cdep, locals.var_cdep_dn0, locals.var_cdep_dn2, locals.var_cdep_dn3, locals.var_cdep_dn4, locals.var_cdep_dn5, locals.var_cdep_dn6, locals.var_cdep_dn7, locals.var_cdep_dn8, locals.var_cdep_dn9, locals.var_cdep_dn10, locals.var_cdep_dn11, locals.var_cdep_dn12, locals.var_cdep_dn13, locals.var_cdep_dn14,)
    }
};
        locals.var_cdep = assign31620_e41872;
        locals.var_cdep_dn0 = assign31620_e41872_d_n0;
        locals.var_cdep_dn2 = assign31620_e41872_d_n2;
        locals.var_cdep_dn3 = assign31620_e41872_d_n3;
        locals.var_cdep_dn4 = assign31620_e41872_d_n4;
        locals.var_cdep_dn5 = assign31620_e41872_d_n5;
        locals.var_cdep_dn6 = assign31620_e41872_d_n6;
        locals.var_cdep_dn7 = assign31620_e41872_d_n7;
        locals.var_cdep_dn8 = assign31620_e41872_d_n8;
        locals.var_cdep_dn9 = assign31620_e41872_d_n9;
        locals.var_cdep_dn10 = assign31620_e41872_d_n10;
        locals.var_cdep_dn11 = assign31620_e41872_d_n11;
        locals.var_cdep_dn12 = assign31620_e41872_d_n12;
        locals.var_cdep_dn13 = assign31620_e41872_d_n13;
        locals.var_cdep_dn14 = assign31620_e41872_d_n14;

        let (assign31630_e41886, assign31630_e41886_d_n0, assign31630_e41886_d_n2, assign31630_e41886_d_n3, assign31630_e41886_d_n4, assign31630_e41886_d_n5, assign31630_e41886_d_n6, assign31630_e41886_d_n7, assign31630_e41886_d_n8, assign31630_e41886_d_n9, assign31630_e41886_d_n10, assign31630_e41886_d_n11, assign31630_e41886_d_n12, assign31630_e41886_d_n13, assign31630_e41886_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31630_e41876: f64 = (locals.var_citedge_i + locals.var_nfactoredge_t);
        let assign31630_e41879: f64 = (locals.var_cdscdedge_i * locals.var_vdsx);
        let assign31630_e41880: f64 = (assign31630_e41876 + assign31630_e41879);
        let assign31630_e41883: f64 = (locals.var_cdscbedge_i * locals.var_vbsx);
        let assign31630_e41884: f64 = (assign31630_e41880 - assign31630_e41883);
        (assign31630_e41884, ((locals.var_cdscdedge_i * locals.var_vdsx_dn0) - (locals.var_cdscbedge_i * locals.var_vbsx_dn0)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn2) - (locals.var_cdscbedge_i * locals.var_vbsx_dn2)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn3) - (locals.var_cdscbedge_i * locals.var_vbsx_dn3)), ((locals.var_nfactoredge_t_dn4 + (locals.var_cdscdedge_i * locals.var_vdsx_dn4)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn4)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn5) - (locals.var_cdscbedge_i * locals.var_vbsx_dn5)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn6) - (locals.var_cdscbedge_i * locals.var_vbsx_dn6)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn7) - (locals.var_cdscbedge_i * locals.var_vbsx_dn7)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn8) - (locals.var_cdscbedge_i * locals.var_vbsx_dn8)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn9) - (locals.var_cdscbedge_i * locals.var_vbsx_dn9)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn10) - (locals.var_cdscbedge_i * locals.var_vbsx_dn10)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn11) - (locals.var_cdscbedge_i * locals.var_vbsx_dn11)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn12) - (locals.var_cdscbedge_i * locals.var_vbsx_dn12)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn13) - (locals.var_cdscbedge_i * locals.var_vbsx_dn13)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn14) - (locals.var_cdscbedge_i * locals.var_vbsx_dn14)),)
    } else {
        (locals.var_cdsc, locals.var_cdsc_dn0, locals.var_cdsc_dn2, locals.var_cdsc_dn3, locals.var_cdsc_dn4, locals.var_cdsc_dn5, locals.var_cdsc_dn6, locals.var_cdsc_dn7, locals.var_cdsc_dn8, locals.var_cdsc_dn9, locals.var_cdsc_dn10, locals.var_cdsc_dn11, locals.var_cdsc_dn12, locals.var_cdsc_dn13, locals.var_cdsc_dn14,)
    }
};
        locals.var_cdsc = assign31630_e41886;
        locals.var_cdsc_dn0 = assign31630_e41886_d_n0;
        locals.var_cdsc_dn2 = assign31630_e41886_d_n2;
        locals.var_cdsc_dn3 = assign31630_e41886_d_n3;
        locals.var_cdsc_dn4 = assign31630_e41886_d_n4;
        locals.var_cdsc_dn5 = assign31630_e41886_d_n5;
        locals.var_cdsc_dn6 = assign31630_e41886_d_n6;
        locals.var_cdsc_dn7 = assign31630_e41886_d_n7;
        locals.var_cdsc_dn8 = assign31630_e41886_d_n8;
        locals.var_cdsc_dn9 = assign31630_e41886_d_n9;
        locals.var_cdsc_dn10 = assign31630_e41886_d_n10;
        locals.var_cdsc_dn11 = assign31630_e41886_d_n11;
        locals.var_cdsc_dn12 = assign31630_e41886_d_n12;
        locals.var_cdsc_dn13 = assign31630_e41886_d_n13;
        locals.var_cdsc_dn14 = assign31630_e41886_d_n14;

        let (assign31640_e41894, assign31640_e41894_d_n0, assign31640_e41894_d_n2, assign31640_e41894_d_n3, assign31640_e41894_d_n4, assign31640_e41894_d_n5, assign31640_e41894_d_n6, assign31640_e41894_d_n7, assign31640_e41894_d_n8, assign31640_e41894_d_n9, assign31640_e41894_d_n10, assign31640_e41894_d_n11, assign31640_e41894_d_n12, assign31640_e41894_d_n13, assign31640_e41894_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31640_e41891: f64 = (locals.var_cdsc / locals.var_cox);
        let assign31640_e41892: f64 = (1.0 + assign31640_e41891);
        (assign31640_e41892, (locals.var_cdsc_dn0 / locals.var_cox), (locals.var_cdsc_dn2 / locals.var_cox), (locals.var_cdsc_dn3 / locals.var_cox), (locals.var_cdsc_dn4 / locals.var_cox), (locals.var_cdsc_dn5 / locals.var_cox), (locals.var_cdsc_dn6 / locals.var_cox), (locals.var_cdsc_dn7 / locals.var_cox), (locals.var_cdsc_dn8 / locals.var_cox), (locals.var_cdsc_dn9 / locals.var_cox), (locals.var_cdsc_dn10 / locals.var_cox), (locals.var_cdsc_dn11 / locals.var_cox), (locals.var_cdsc_dn12 / locals.var_cox), (locals.var_cdsc_dn13 / locals.var_cox), (locals.var_cdsc_dn14 / locals.var_cox),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31640_e41894;
        locals.var_t1_dn0 = assign31640_e41894_d_n0;
        locals.var_t1_dn2 = assign31640_e41894_d_n2;
        locals.var_t1_dn3 = assign31640_e41894_d_n3;
        locals.var_t1_dn4 = assign31640_e41894_d_n4;
        locals.var_t1_dn5 = assign31640_e41894_d_n5;
        locals.var_t1_dn6 = assign31640_e41894_d_n6;
        locals.var_t1_dn7 = assign31640_e41894_d_n7;
        locals.var_t1_dn8 = assign31640_e41894_d_n8;
        locals.var_t1_dn9 = assign31640_e41894_d_n9;
        locals.var_t1_dn10 = assign31640_e41894_d_n10;
        locals.var_t1_dn11 = assign31640_e41894_d_n11;
        locals.var_t1_dn12 = assign31640_e41894_d_n12;
        locals.var_t1_dn13 = assign31640_e41894_d_n13;
        locals.var_t1_dn14 = assign31640_e41894_d_n14;

        let assign31650_e41900: f64 = (-2500.0);
        let assign31650_e41902: f64 = (assign31650_e41900 * 0.05);
        let assign31650_e41904: f64 = if ((1.0 == 0.0) && (locals.var_t1 < assign31650_e41902)) { 1.0 } else { 0.0 };
        locals.var_guard732 = assign31650_e41904;

        let (assign31660_e41917, assign31660_e41917_d_n0, assign31660_e41917_d_n2, assign31660_e41917_d_n3, assign31660_e41917_d_n4, assign31660_e41917_d_n5, assign31660_e41917_d_n6, assign31660_e41917_d_n7, assign31660_e41917_d_n8, assign31660_e41917_d_n9, assign31660_e41917_d_n10, assign31660_e41917_d_n11, assign31660_e41917_d_n12, assign31660_e41917_d_n13, assign31660_e41917_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard732 != 0.0)) {
        let assign31660_e41909: f64 = (-0.05);
        let assign31660_e41911: f64 = (assign31660_e41909 * 0.05);
        let assign31660_e41914: f64 = (16.0 * locals.var_t1);
        let assign31660_e41915: f64 = (assign31660_e41911 / assign31660_e41914);
        (assign31660_e41915, (-((assign31660_e41911 * (16.0 * locals.var_t1_dn0)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn2)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn3)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn4)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn5)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn6)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn7)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn8)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn9)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn10)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn11)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn12)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn13)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn14)) / (assign31660_e41914 * assign31660_e41914))),)
    } else {
        (locals.var_n, locals.var_n_dn0, locals.var_n_dn2, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11, locals.var_n_dn12, locals.var_n_dn13, locals.var_n_dn14,)
    }
};
        locals.var_n = assign31660_e41917;
        locals.var_n_dn0 = assign31660_e41917_d_n0;
        locals.var_n_dn2 = assign31660_e41917_d_n2;
        locals.var_n_dn3 = assign31660_e41917_d_n3;
        locals.var_n_dn4 = assign31660_e41917_d_n4;
        locals.var_n_dn5 = assign31660_e41917_d_n5;
        locals.var_n_dn6 = assign31660_e41917_d_n6;
        locals.var_n_dn7 = assign31660_e41917_d_n7;
        locals.var_n_dn8 = assign31660_e41917_d_n8;
        locals.var_n_dn9 = assign31660_e41917_d_n9;
        locals.var_n_dn10 = assign31660_e41917_d_n10;
        locals.var_n_dn11 = assign31660_e41917_d_n11;
        locals.var_n_dn12 = assign31660_e41917_d_n12;
        locals.var_n_dn13 = assign31660_e41917_d_n13;
        locals.var_n_dn14 = assign31660_e41917_d_n14;

        let (assign31670_e41943, assign31670_e41943_d_n0, assign31670_e41943_d_n2, assign31670_e41943_d_n3, assign31670_e41943_d_n4, assign31670_e41943_d_n5, assign31670_e41943_d_n6, assign31670_e41943_d_n7, assign31670_e41943_d_n8, assign31670_e41943_d_n9, assign31670_e41943_d_n10, assign31670_e41943_d_n11, assign31670_e41943_d_n12, assign31670_e41943_d_n13, assign31670_e41943_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard732 == 0.0)) {
        let assign31670_e41925: f64 = (locals.var_t1 + 1.0);
        let assign31670_e41928: f64 = (locals.var_t1 - 1.0);
        let assign31670_e41931: f64 = (locals.var_t1 - 1.0);
        let assign31670_e41932: f64 = (assign31670_e41928 * assign31670_e41931);
        let assign31670_e41935: f64 = (0.25 * 0.05);
        let assign31670_e41937: f64 = (assign31670_e41935 * 0.05);
        let assign31670_e41938: f64 = (assign31670_e41932 + assign31670_e41937);
        let assign31670_e41939: f64 = (assign31670_e41938).sqrt();
        let assign31670_e41940: f64 = (assign31670_e41925 + assign31670_e41939);
        let assign31670_e41941: f64 = (0.5 * assign31670_e41940);
        (assign31670_e41941, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn0)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn2)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn3)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn4)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn5)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn6)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn7)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn8)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn9)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn10)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn11)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn12 + (((locals.var_t1_dn12 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn12)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn13)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn14)) / (2.0 * assign31670_e41939)))),)
    } else {
        (locals.var_n, locals.var_n_dn0, locals.var_n_dn2, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11, locals.var_n_dn12, locals.var_n_dn13, locals.var_n_dn14,)
    }
};
        locals.var_n = assign31670_e41943;
        locals.var_n_dn0 = assign31670_e41943_d_n0;
        locals.var_n_dn2 = assign31670_e41943_d_n2;
        locals.var_n_dn3 = assign31670_e41943_d_n3;
        locals.var_n_dn4 = assign31670_e41943_d_n4;
        locals.var_n_dn5 = assign31670_e41943_d_n5;
        locals.var_n_dn6 = assign31670_e41943_d_n6;
        locals.var_n_dn7 = assign31670_e41943_d_n7;
        locals.var_n_dn8 = assign31670_e41943_d_n8;
        locals.var_n_dn9 = assign31670_e41943_d_n9;
        locals.var_n_dn10 = assign31670_e41943_d_n10;
        locals.var_n_dn11 = assign31670_e41943_d_n11;
        locals.var_n_dn12 = assign31670_e41943_d_n12;
        locals.var_n_dn13 = assign31670_e41943_d_n13;
        locals.var_n_dn14 = assign31670_e41943_d_n14;

        let (assign31680_e41949, assign31680_e41949_d_n0, assign31680_e41949_d_n2, assign31680_e41949_d_n3, assign31680_e41949_d_n4, assign31680_e41949_d_n5, assign31680_e41949_d_n6, assign31680_e41949_d_n7, assign31680_e41949_d_n8, assign31680_e41949_d_n9, assign31680_e41949_d_n10, assign31680_e41949_d_n11, assign31680_e41949_d_n12, assign31680_e41949_d_n13, assign31680_e41949_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31680_e41947: f64 = (locals.var_n * locals.var_vt);
        (assign31680_e41947, (locals.var_n_dn0 * locals.var_vt), (locals.var_n_dn2 * locals.var_vt), (locals.var_n_dn3 * locals.var_vt), ((locals.var_n_dn4 * locals.var_vt) + (locals.var_n * locals.var_vt_dn4)), (locals.var_n_dn5 * locals.var_vt), (locals.var_n_dn6 * locals.var_vt), (locals.var_n_dn7 * locals.var_vt), (locals.var_n_dn8 * locals.var_vt), (locals.var_n_dn9 * locals.var_vt), (locals.var_n_dn10 * locals.var_vt), (locals.var_n_dn11 * locals.var_vt), (locals.var_n_dn12 * locals.var_vt), (locals.var_n_dn13 * locals.var_vt), (locals.var_n_dn14 * locals.var_vt),)
    } else {
        (locals.var_nvt, locals.var_nvt_dn0, locals.var_nvt_dn2, locals.var_nvt_dn3, locals.var_nvt_dn4, locals.var_nvt_dn5, locals.var_nvt_dn6, locals.var_nvt_dn7, locals.var_nvt_dn8, locals.var_nvt_dn9, locals.var_nvt_dn10, locals.var_nvt_dn11, locals.var_nvt_dn12, locals.var_nvt_dn13, locals.var_nvt_dn14,)
    }
};
        locals.var_nvt = assign31680_e41949;
        locals.var_nvt_dn0 = assign31680_e41949_d_n0;
        locals.var_nvt_dn2 = assign31680_e41949_d_n2;
        locals.var_nvt_dn3 = assign31680_e41949_d_n3;
        locals.var_nvt_dn4 = assign31680_e41949_d_n4;
        locals.var_nvt_dn5 = assign31680_e41949_d_n5;
        locals.var_nvt_dn6 = assign31680_e41949_d_n6;
        locals.var_nvt_dn7 = assign31680_e41949_d_n7;
        locals.var_nvt_dn8 = assign31680_e41949_d_n8;
        locals.var_nvt_dn9 = assign31680_e41949_d_n9;
        locals.var_nvt_dn10 = assign31680_e41949_d_n10;
        locals.var_nvt_dn11 = assign31680_e41949_d_n11;
        locals.var_nvt_dn12 = assign31680_e41949_d_n12;
        locals.var_nvt_dn13 = assign31680_e41949_d_n13;
        locals.var_nvt_dn14 = assign31680_e41949_d_n14;

        let (assign31690_e41955, assign31690_e41955_d_n0, assign31690_e41955_d_n2, assign31690_e41955_d_n3, assign31690_e41955_d_n4, assign31690_e41955_d_n5, assign31690_e41955_d_n6, assign31690_e41955_d_n7, assign31690_e41955_d_n8, assign31690_e41955_d_n9, assign31690_e41955_d_n10, assign31690_e41955_d_n11, assign31690_e41955_d_n12, assign31690_e41955_d_n13, assign31690_e41955_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31690_e41953: f64 = (1.0 / locals.var_nvt);
        (assign31690_e41953, (-(locals.var_nvt_dn0 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn2 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn3 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn4 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn5 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn6 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn7 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn8 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn9 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn10 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn11 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn12 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn13 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn14 / (locals.var_nvt * locals.var_nvt))),)
    } else {
        (locals.var_inv_nvt, locals.var_inv_nvt_dn0, locals.var_inv_nvt_dn2, locals.var_inv_nvt_dn3, locals.var_inv_nvt_dn4, locals.var_inv_nvt_dn5, locals.var_inv_nvt_dn6, locals.var_inv_nvt_dn7, locals.var_inv_nvt_dn8, locals.var_inv_nvt_dn9, locals.var_inv_nvt_dn10, locals.var_inv_nvt_dn11, locals.var_inv_nvt_dn12, locals.var_inv_nvt_dn13, locals.var_inv_nvt_dn14,)
    }
};
        locals.var_inv_nvt = assign31690_e41955;
        locals.var_inv_nvt_dn0 = assign31690_e41955_d_n0;
        locals.var_inv_nvt_dn2 = assign31690_e41955_d_n2;
        locals.var_inv_nvt_dn3 = assign31690_e41955_d_n3;
        locals.var_inv_nvt_dn4 = assign31690_e41955_d_n4;
        locals.var_inv_nvt_dn5 = assign31690_e41955_d_n5;
        locals.var_inv_nvt_dn6 = assign31690_e41955_d_n6;
        locals.var_inv_nvt_dn7 = assign31690_e41955_d_n7;
        locals.var_inv_nvt_dn8 = assign31690_e41955_d_n8;
        locals.var_inv_nvt_dn9 = assign31690_e41955_d_n9;
        locals.var_inv_nvt_dn10 = assign31690_e41955_d_n10;
        locals.var_inv_nvt_dn11 = assign31690_e41955_d_n11;
        locals.var_inv_nvt_dn12 = assign31690_e41955_d_n12;
        locals.var_inv_nvt_dn13 = assign31690_e41955_d_n13;
        locals.var_inv_nvt_dn14 = assign31690_e41955_d_n14;

        let (assign31700_e41961, assign31700_e41961_d_n0, assign31700_e41961_d_n2, assign31700_e41961_d_n3, assign31700_e41961_d_n4, assign31700_e41961_d_n5, assign31700_e41961_d_n6, assign31700_e41961_d_n7, assign31700_e41961_d_n8, assign31700_e41961_d_n9, assign31700_e41961_d_n10, assign31700_e41961_d_n11, assign31700_e41961_d_n12, assign31700_e41961_d_n13, assign31700_e41961_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31700_e41959: f64 = (locals.var_vg * locals.var_inv_nvt);
        (assign31700_e41959, (locals.var_vg * locals.var_inv_nvt_dn0), (locals.var_vg * locals.var_inv_nvt_dn2), (locals.var_vg * locals.var_inv_nvt_dn3), (locals.var_vg * locals.var_inv_nvt_dn4), (locals.var_vg * locals.var_inv_nvt_dn5), (locals.var_vg * locals.var_inv_nvt_dn6), (locals.var_vg * locals.var_inv_nvt_dn7), (locals.var_vg * locals.var_inv_nvt_dn8), ((locals.var_vg_dn9 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn9)), (locals.var_vg * locals.var_inv_nvt_dn10), ((locals.var_vg_dn11 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn11)), (locals.var_vg * locals.var_inv_nvt_dn12), (locals.var_vg * locals.var_inv_nvt_dn13), (locals.var_vg * locals.var_inv_nvt_dn14),)
    } else {
        (locals.var_vg_1, locals.var_vg_1_dn0, locals.var_vg_1_dn2, locals.var_vg_1_dn3, locals.var_vg_1_dn4, locals.var_vg_1_dn5, locals.var_vg_1_dn6, locals.var_vg_1_dn7, locals.var_vg_1_dn8, locals.var_vg_1_dn9, locals.var_vg_1_dn10, locals.var_vg_1_dn11, locals.var_vg_1_dn12, locals.var_vg_1_dn13, locals.var_vg_1_dn14,)
    }
};
        locals.var_vg_1 = assign31700_e41961;
        locals.var_vg_1_dn0 = assign31700_e41961_d_n0;
        locals.var_vg_1_dn2 = assign31700_e41961_d_n2;
        locals.var_vg_1_dn3 = assign31700_e41961_d_n3;
        locals.var_vg_1_dn4 = assign31700_e41961_d_n4;
        locals.var_vg_1_dn5 = assign31700_e41961_d_n5;
        locals.var_vg_1_dn6 = assign31700_e41961_d_n6;
        locals.var_vg_1_dn7 = assign31700_e41961_d_n7;
        locals.var_vg_1_dn8 = assign31700_e41961_d_n8;
        locals.var_vg_1_dn9 = assign31700_e41961_d_n9;
        locals.var_vg_1_dn10 = assign31700_e41961_d_n10;
        locals.var_vg_1_dn11 = assign31700_e41961_d_n11;
        locals.var_vg_1_dn12 = assign31700_e41961_d_n12;
        locals.var_vg_1_dn13 = assign31700_e41961_d_n13;
        locals.var_vg_1_dn14 = assign31700_e41961_d_n14;

        let (assign31710_e41967, assign31710_e41967_d_n0, assign31710_e41967_d_n2, assign31710_e41967_d_n3, assign31710_e41967_d_n4, assign31710_e41967_d_n5, assign31710_e41967_d_n6, assign31710_e41967_d_n7, assign31710_e41967_d_n8, assign31710_e41967_d_n9, assign31710_e41967_d_n10, assign31710_e41967_d_n11, assign31710_e41967_d_n12, assign31710_e41967_d_n13, assign31710_e41967_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31710_e41965: f64 = (locals.var_vs * locals.var_inv_nvt);
        (assign31710_e41965, (locals.var_vs * locals.var_inv_nvt_dn0), (locals.var_vs * locals.var_inv_nvt_dn2), (locals.var_vs * locals.var_inv_nvt_dn3), (locals.var_vs * locals.var_inv_nvt_dn4), ((locals.var_vs_dn5 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn5)), (locals.var_vs * locals.var_inv_nvt_dn6), ((locals.var_vs_dn7 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn7)), (locals.var_vs * locals.var_inv_nvt_dn8), (locals.var_vs * locals.var_inv_nvt_dn9), (locals.var_vs * locals.var_inv_nvt_dn10), ((locals.var_vs_dn11 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn11)), (locals.var_vs * locals.var_inv_nvt_dn12), (locals.var_vs * locals.var_inv_nvt_dn13), (locals.var_vs * locals.var_inv_nvt_dn14),)
    } else {
        (locals.var_vs_1, locals.var_vs_1_dn0, locals.var_vs_1_dn2, locals.var_vs_1_dn3, locals.var_vs_1_dn4, locals.var_vs_1_dn5, locals.var_vs_1_dn6, locals.var_vs_1_dn7, locals.var_vs_1_dn8, locals.var_vs_1_dn9, locals.var_vs_1_dn10, locals.var_vs_1_dn11, locals.var_vs_1_dn12, locals.var_vs_1_dn13, locals.var_vs_1_dn14,)
    }
};
        locals.var_vs_1 = assign31710_e41967;
        locals.var_vs_1_dn0 = assign31710_e41967_d_n0;
        locals.var_vs_1_dn2 = assign31710_e41967_d_n2;
        locals.var_vs_1_dn3 = assign31710_e41967_d_n3;
        locals.var_vs_1_dn4 = assign31710_e41967_d_n4;
        locals.var_vs_1_dn5 = assign31710_e41967_d_n5;
        locals.var_vs_1_dn6 = assign31710_e41967_d_n6;
        locals.var_vs_1_dn7 = assign31710_e41967_d_n7;
        locals.var_vs_1_dn8 = assign31710_e41967_d_n8;
        locals.var_vs_1_dn9 = assign31710_e41967_d_n9;
        locals.var_vs_1_dn10 = assign31710_e41967_d_n10;
        locals.var_vs_1_dn11 = assign31710_e41967_d_n11;
        locals.var_vs_1_dn12 = assign31710_e41967_d_n12;
        locals.var_vs_1_dn13 = assign31710_e41967_d_n13;
        locals.var_vs_1_dn14 = assign31710_e41967_d_n14;

        let (assign31720_e41973, assign31720_e41973_d_n0, assign31720_e41973_d_n2, assign31720_e41973_d_n3, assign31720_e41973_d_n4, assign31720_e41973_d_n5, assign31720_e41973_d_n6, assign31720_e41973_d_n7, assign31720_e41973_d_n8, assign31720_e41973_d_n9, assign31720_e41973_d_n10, assign31720_e41973_d_n11, assign31720_e41973_d_n12, assign31720_e41973_d_n13, assign31720_e41973_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31720_e41971: f64 = (locals.var_vfb_i * locals.var_inv_nvt);
        (assign31720_e41971, ((locals.var_vfb_i_dn0 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn0)), ((locals.var_vfb_i_dn2 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn2)), ((locals.var_vfb_i_dn3 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn3)), ((locals.var_vfb_i_dn4 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn4)), ((locals.var_vfb_i_dn5 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn5)), ((locals.var_vfb_i_dn6 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn6)), ((locals.var_vfb_i_dn7 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn7)), ((locals.var_vfb_i_dn8 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn8)), ((locals.var_vfb_i_dn9 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn9)), ((locals.var_vfb_i_dn10 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn10)), ((locals.var_vfb_i_dn11 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn11)), ((locals.var_vfb_i_dn12 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn12)), ((locals.var_vfb_i_dn13 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn13)), ((locals.var_vfb_i_dn14 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn14)),)
    } else {
        (locals.var_vfb, locals.var_vfb_dn0, locals.var_vfb_dn2, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12, locals.var_vfb_dn13, locals.var_vfb_dn14,)
    }
};
        locals.var_vfb = assign31720_e41973;
        locals.var_vfb_dn0 = assign31720_e41973_d_n0;
        locals.var_vfb_dn2 = assign31720_e41973_d_n2;
        locals.var_vfb_dn3 = assign31720_e41973_d_n3;
        locals.var_vfb_dn4 = assign31720_e41973_d_n4;
        locals.var_vfb_dn5 = assign31720_e41973_d_n5;
        locals.var_vfb_dn6 = assign31720_e41973_d_n6;
        locals.var_vfb_dn7 = assign31720_e41973_d_n7;
        locals.var_vfb_dn8 = assign31720_e41973_d_n8;
        locals.var_vfb_dn9 = assign31720_e41973_d_n9;
        locals.var_vfb_dn10 = assign31720_e41973_d_n10;
        locals.var_vfb_dn11 = assign31720_e41973_d_n11;
        locals.var_vfb_dn12 = assign31720_e41973_d_n12;
        locals.var_vfb_dn13 = assign31720_e41973_d_n13;
        locals.var_vfb_dn14 = assign31720_e41973_d_n14;

        let (assign31730_e41984, assign31730_e41984_d_n0, assign31730_e41984_d_n2, assign31730_e41984_d_n3, assign31730_e41984_d_n4, assign31730_e41984_d_n5, assign31730_e41984_d_n6, assign31730_e41984_d_n7, assign31730_e41984_d_n8, assign31730_e41984_d_n9, assign31730_e41984_d_n10, assign31730_e41984_d_n11, assign31730_e41984_d_n12, assign31730_e41984_d_n13, assign31730_e41984_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31730_e41978: f64 = (locals.var_etabedge_i * locals.var_vbsx);
        let assign31730_e41979: f64 = (locals.var_eta0edge_t + assign31730_e41978);
        let assign31730_e41980: f64 = (-assign31730_e41979);
        let assign31730_e41982: f64 = (assign31730_e41980 * locals.var_vdsx);
        (assign31730_e41982, (((-(locals.var_eta0edge_t_dn0 + (locals.var_etabedge_i * locals.var_vbsx_dn0))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn0)), (((-(locals.var_eta0edge_t_dn2 + (locals.var_etabedge_i * locals.var_vbsx_dn2))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn2)), (((-(locals.var_eta0edge_t_dn3 + (locals.var_etabedge_i * locals.var_vbsx_dn3))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn3)), (((-(locals.var_eta0edge_t_dn4 + (locals.var_etabedge_i * locals.var_vbsx_dn4))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn4)), (((-(locals.var_eta0edge_t_dn5 + (locals.var_etabedge_i * locals.var_vbsx_dn5))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn5)), (((-(locals.var_eta0edge_t_dn6 + (locals.var_etabedge_i * locals.var_vbsx_dn6))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn6)), (((-(locals.var_eta0edge_t_dn7 + (locals.var_etabedge_i * locals.var_vbsx_dn7))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn7)), (((-(locals.var_eta0edge_t_dn8 + (locals.var_etabedge_i * locals.var_vbsx_dn8))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn8)), (((-(locals.var_eta0edge_t_dn9 + (locals.var_etabedge_i * locals.var_vbsx_dn9))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn9)), (((-(locals.var_eta0edge_t_dn10 + (locals.var_etabedge_i * locals.var_vbsx_dn10))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn10)), (((-(locals.var_eta0edge_t_dn11 + (locals.var_etabedge_i * locals.var_vbsx_dn11))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn11)), (((-(locals.var_eta0edge_t_dn12 + (locals.var_etabedge_i * locals.var_vbsx_dn12))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn12)), (((-(locals.var_eta0edge_t_dn13 + (locals.var_etabedge_i * locals.var_vbsx_dn13))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn13)), (((-(locals.var_eta0edge_t_dn14 + (locals.var_etabedge_i * locals.var_vbsx_dn14))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn14)),)
    } else {
        (locals.var_dvth_dibl_1, locals.var_dvth_dibl_1_dn0, locals.var_dvth_dibl_1_dn2, locals.var_dvth_dibl_1_dn3, locals.var_dvth_dibl_1_dn4, locals.var_dvth_dibl_1_dn5, locals.var_dvth_dibl_1_dn6, locals.var_dvth_dibl_1_dn7, locals.var_dvth_dibl_1_dn8, locals.var_dvth_dibl_1_dn9, locals.var_dvth_dibl_1_dn10, locals.var_dvth_dibl_1_dn11, locals.var_dvth_dibl_1_dn12, locals.var_dvth_dibl_1_dn13, locals.var_dvth_dibl_1_dn14,)
    }
};
        locals.var_dvth_dibl_1 = assign31730_e41984;
        locals.var_dvth_dibl_1_dn0 = assign31730_e41984_d_n0;
        locals.var_dvth_dibl_1_dn2 = assign31730_e41984_d_n2;
        locals.var_dvth_dibl_1_dn3 = assign31730_e41984_d_n3;
        locals.var_dvth_dibl_1_dn4 = assign31730_e41984_d_n4;
        locals.var_dvth_dibl_1_dn5 = assign31730_e41984_d_n5;
        locals.var_dvth_dibl_1_dn6 = assign31730_e41984_d_n6;
        locals.var_dvth_dibl_1_dn7 = assign31730_e41984_d_n7;
        locals.var_dvth_dibl_1_dn8 = assign31730_e41984_d_n8;
        locals.var_dvth_dibl_1_dn9 = assign31730_e41984_d_n9;
        locals.var_dvth_dibl_1_dn10 = assign31730_e41984_d_n10;
        locals.var_dvth_dibl_1_dn11 = assign31730_e41984_d_n11;
        locals.var_dvth_dibl_1_dn12 = assign31730_e41984_d_n12;
        locals.var_dvth_dibl_1_dn13 = assign31730_e41984_d_n13;
        locals.var_dvth_dibl_1_dn14 = assign31730_e41984_d_n14;

        let (assign31740_e42002, assign31740_e42002_d_n0, assign31740_e42002_d_n2, assign31740_e42002_d_n3, assign31740_e42002_d_n4, assign31740_e42002_d_n5, assign31740_e42002_d_n6, assign31740_e42002_d_n7, assign31740_e42002_d_n8, assign31740_e42002_d_n9, assign31740_e42002_d_n10, assign31740_e42002_d_n11, assign31740_e42002_d_n12, assign31740_e42002_d_n13, assign31740_e42002_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31740_e41989: f64 = (locals.var_kt1ledge_i / locals.var_leff);
        let assign31740_e41990: f64 = (locals.var_kt1edge_i + assign31740_e41989);
        let assign31740_e41993: f64 = (locals.var_kt2edge_i * locals.var_vbsx);
        let assign31740_e41994: f64 = (assign31740_e41990 + assign31740_e41993);
        let assign31740_e41997: f64 = (locals.var_tratio).powf(locals.var_kt1expedge_i);
        let assign31740_e41999: f64 = (assign31740_e41997 - 1.0);
        let assign31740_e42000: f64 = (assign31740_e41994 * assign31740_e41999);
        (assign31740_e42000, ((locals.var_kt2edge_i * locals.var_vbsx_dn0) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn2) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn3) * assign31740_e41999), (((locals.var_kt2edge_i * locals.var_vbsx_dn4) * assign31740_e41999) + (assign31740_e41994 * if 0.0 == 0.0 && ((locals.var_kt1expedge_i) as f64).is_finite() && ((locals.var_kt1expedge_i) as f64).fract() == 0.0 { if locals.var_kt1expedge_i == 0.0 { 0.0 } else { (locals.var_kt1expedge_i * ((locals.var_tratio).powf(locals.var_kt1expedge_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign31740_e41997 * (locals.var_kt1expedge_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), ((locals.var_kt2edge_i * locals.var_vbsx_dn5) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn6) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn7) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn8) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn9) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn10) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn11) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn12) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn13) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn14) * assign31740_e41999),)
    } else {
        (locals.var_dvth_temp, locals.var_dvth_temp_dn0, locals.var_dvth_temp_dn2, locals.var_dvth_temp_dn3, locals.var_dvth_temp_dn4, locals.var_dvth_temp_dn5, locals.var_dvth_temp_dn6, locals.var_dvth_temp_dn7, locals.var_dvth_temp_dn8, locals.var_dvth_temp_dn9, locals.var_dvth_temp_dn10, locals.var_dvth_temp_dn11, locals.var_dvth_temp_dn12, locals.var_dvth_temp_dn13, locals.var_dvth_temp_dn14,)
    }
};
        locals.var_dvth_temp = assign31740_e42002;
        locals.var_dvth_temp_dn0 = assign31740_e42002_d_n0;
        locals.var_dvth_temp_dn2 = assign31740_e42002_d_n2;
        locals.var_dvth_temp_dn3 = assign31740_e42002_d_n3;
        locals.var_dvth_temp_dn4 = assign31740_e42002_d_n4;
        locals.var_dvth_temp_dn5 = assign31740_e42002_d_n5;
        locals.var_dvth_temp_dn6 = assign31740_e42002_d_n6;
        locals.var_dvth_temp_dn7 = assign31740_e42002_d_n7;
        locals.var_dvth_temp_dn8 = assign31740_e42002_d_n8;
        locals.var_dvth_temp_dn9 = assign31740_e42002_d_n9;
        locals.var_dvth_temp_dn10 = assign31740_e42002_d_n10;
        locals.var_dvth_temp_dn11 = assign31740_e42002_d_n11;
        locals.var_dvth_temp_dn12 = assign31740_e42002_d_n12;
        locals.var_dvth_temp_dn13 = assign31740_e42002_d_n13;
        locals.var_dvth_temp_dn14 = assign31740_e42002_d_n14;

        let (assign31750_e42012, assign31750_e42012_d_n0, assign31750_e42012_d_n2, assign31750_e42012_d_n3, assign31750_e42012_d_n4, assign31750_e42012_d_n5, assign31750_e42012_d_n6, assign31750_e42012_d_n7, assign31750_e42012_d_n8, assign31750_e42012_d_n9, assign31750_e42012_d_n10, assign31750_e42012_d_n11, assign31750_e42012_d_n12, assign31750_e42012_d_n13, assign31750_e42012_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31750_e42008: f64 = (p.p1016 * locals.var_vbsx);
        let assign31750_e42009: f64 = (1.0 + assign31750_e42008);
        let assign31750_e42010: f64 = (locals.var_litl * assign31750_e42009);
        (assign31750_e42010, (locals.var_litl * (p.p1016 * locals.var_vbsx_dn0)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn2)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn3)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn4)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn5)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn6)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn7)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn8)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn9)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn10)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn11)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn12)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn13)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn14)),)
    } else {
        (locals.var_litl_edge, locals.var_litl_edge_dn0, locals.var_litl_edge_dn2, locals.var_litl_edge_dn3, locals.var_litl_edge_dn4, locals.var_litl_edge_dn5, locals.var_litl_edge_dn6, locals.var_litl_edge_dn7, locals.var_litl_edge_dn8, locals.var_litl_edge_dn9, locals.var_litl_edge_dn10, locals.var_litl_edge_dn11, locals.var_litl_edge_dn12, locals.var_litl_edge_dn13, locals.var_litl_edge_dn14,)
    }
};
        locals.var_litl_edge = assign31750_e42012;
        locals.var_litl_edge_dn0 = assign31750_e42012_d_n0;
        locals.var_litl_edge_dn2 = assign31750_e42012_d_n2;
        locals.var_litl_edge_dn3 = assign31750_e42012_d_n3;
        locals.var_litl_edge_dn4 = assign31750_e42012_d_n4;
        locals.var_litl_edge_dn5 = assign31750_e42012_d_n5;
        locals.var_litl_edge_dn6 = assign31750_e42012_d_n6;
        locals.var_litl_edge_dn7 = assign31750_e42012_d_n7;
        locals.var_litl_edge_dn8 = assign31750_e42012_d_n8;
        locals.var_litl_edge_dn9 = assign31750_e42012_d_n9;
        locals.var_litl_edge_dn10 = assign31750_e42012_d_n10;
        locals.var_litl_edge_dn11 = assign31750_e42012_d_n11;
        locals.var_litl_edge_dn12 = assign31750_e42012_d_n12;
        locals.var_litl_edge_dn13 = assign31750_e42012_d_n13;
        locals.var_litl_edge_dn14 = assign31750_e42012_d_n14;

        let assign31760_e42015: f64 = if locals.var_litl_edge > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard733 = assign31760_e42015;

        let (assign31770_e42025, assign31770_e42025_d_n0, assign31770_e42025_d_n2, assign31770_e42025_d_n3, assign31770_e42025_d_n4, assign31770_e42025_d_n5, assign31770_e42025_d_n6, assign31770_e42025_d_n7, assign31770_e42025_d_n8, assign31770_e42025_d_n9, assign31770_e42025_d_n10, assign31770_e42025_d_n11, assign31770_e42025_d_n12, assign31770_e42025_d_n13, assign31770_e42025_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard733 != 0.0)) {
        let assign31770_e42021: f64 = (p.p1015 * locals.var_leff);
        let assign31770_e42023: f64 = (assign31770_e42021 / locals.var_litl_edge);
        (assign31770_e42023, (-((assign31770_e42021 * locals.var_litl_edge_dn0) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn2) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn3) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn4) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn5) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn6) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn7) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn8) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn9) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn10) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn11) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn12) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn13) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn14) / (locals.var_litl_edge * locals.var_litl_edge))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31770_e42025;
        locals.var_t0_dn0 = assign31770_e42025_d_n0;
        locals.var_t0_dn2 = assign31770_e42025_d_n2;
        locals.var_t0_dn3 = assign31770_e42025_d_n3;
        locals.var_t0_dn4 = assign31770_e42025_d_n4;
        locals.var_t0_dn5 = assign31770_e42025_d_n5;
        locals.var_t0_dn6 = assign31770_e42025_d_n6;
        locals.var_t0_dn7 = assign31770_e42025_d_n7;
        locals.var_t0_dn8 = assign31770_e42025_d_n8;
        locals.var_t0_dn9 = assign31770_e42025_d_n9;
        locals.var_t0_dn10 = assign31770_e42025_d_n10;
        locals.var_t0_dn11 = assign31770_e42025_d_n11;
        locals.var_t0_dn12 = assign31770_e42025_d_n12;
        locals.var_t0_dn13 = assign31770_e42025_d_n13;
        locals.var_t0_dn14 = assign31770_e42025_d_n14;

        let assign31780_e42028: f64 = if locals.var_t0 < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard734 = assign31780_e42028;

        let (assign31790_e42043, assign31790_e42043_d_n0, assign31790_e42043_d_n2, assign31790_e42043_d_n3, assign31790_e42043_d_n4, assign31790_e42043_d_n5, assign31790_e42043_d_n6, assign31790_e42043_d_n7, assign31790_e42043_d_n8, assign31790_e42043_d_n9, assign31790_e42043_d_n10, assign31790_e42043_d_n11, assign31790_e42043_d_n12, assign31790_e42043_d_n13, assign31790_e42043_d_n14,) = {
    if (((locals.var_guard730 != 0.0) && (locals.var_guard733 != 0.0)) && (locals.var_guard734 != 0.0)) {
        let assign31790_e42036: f64 = (0.5 * p.p1014);
        let assign31790_e42038: f64 = (locals.var_t0).cosh();
        let assign31790_e42040: f64 = (assign31790_e42038 - 1.0);
        let assign31790_e42041: f64 = (assign31790_e42036 / assign31790_e42040);
        (assign31790_e42041, (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn0)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn2)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn3)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn4)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn5)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn6)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn7)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn8)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn9)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn10)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn11)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn12)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn13)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn14)) / (assign31790_e42040 * assign31790_e42040))),)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn0, locals.var_theta_sce_edge_dn2, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11, locals.var_theta_sce_edge_dn12, locals.var_theta_sce_edge_dn13, locals.var_theta_sce_edge_dn14,)
    }
};
        locals.var_theta_sce_edge = assign31790_e42043;
        locals.var_theta_sce_edge_dn0 = assign31790_e42043_d_n0;
        locals.var_theta_sce_edge_dn2 = assign31790_e42043_d_n2;
        locals.var_theta_sce_edge_dn3 = assign31790_e42043_d_n3;
        locals.var_theta_sce_edge_dn4 = assign31790_e42043_d_n4;
        locals.var_theta_sce_edge_dn5 = assign31790_e42043_d_n5;
        locals.var_theta_sce_edge_dn6 = assign31790_e42043_d_n6;
        locals.var_theta_sce_edge_dn7 = assign31790_e42043_d_n7;
        locals.var_theta_sce_edge_dn8 = assign31790_e42043_d_n8;
        locals.var_theta_sce_edge_dn9 = assign31790_e42043_d_n9;
        locals.var_theta_sce_edge_dn10 = assign31790_e42043_d_n10;
        locals.var_theta_sce_edge_dn11 = assign31790_e42043_d_n11;
        locals.var_theta_sce_edge_dn12 = assign31790_e42043_d_n12;
        locals.var_theta_sce_edge_dn13 = assign31790_e42043_d_n13;
        locals.var_theta_sce_edge_dn14 = assign31790_e42043_d_n14;

        let (assign31800_e42056, assign31800_e42056_d_n0, assign31800_e42056_d_n2, assign31800_e42056_d_n3, assign31800_e42056_d_n4, assign31800_e42056_d_n5, assign31800_e42056_d_n6, assign31800_e42056_d_n7, assign31800_e42056_d_n8, assign31800_e42056_d_n9, assign31800_e42056_d_n10, assign31800_e42056_d_n11, assign31800_e42056_d_n12, assign31800_e42056_d_n13, assign31800_e42056_d_n14,) = {
    if (((locals.var_guard730 != 0.0) && (locals.var_guard733 != 0.0)) && (locals.var_guard734 == 0.0)) {
        let assign31800_e42052: f64 = (-locals.var_t0);
        let assign31800_e42053: f64 = { let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign31800_e42054: f64 = (p.p1014 * assign31800_e42053);
        (assign31800_e42054, (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn3))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn12))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))),)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn0, locals.var_theta_sce_edge_dn2, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11, locals.var_theta_sce_edge_dn12, locals.var_theta_sce_edge_dn13, locals.var_theta_sce_edge_dn14,)
    }
};
        locals.var_theta_sce_edge = assign31800_e42056;
        locals.var_theta_sce_edge_dn0 = assign31800_e42056_d_n0;
        locals.var_theta_sce_edge_dn2 = assign31800_e42056_d_n2;
        locals.var_theta_sce_edge_dn3 = assign31800_e42056_d_n3;
        locals.var_theta_sce_edge_dn4 = assign31800_e42056_d_n4;
        locals.var_theta_sce_edge_dn5 = assign31800_e42056_d_n5;
        locals.var_theta_sce_edge_dn6 = assign31800_e42056_d_n6;
        locals.var_theta_sce_edge_dn7 = assign31800_e42056_d_n7;
        locals.var_theta_sce_edge_dn8 = assign31800_e42056_d_n8;
        locals.var_theta_sce_edge_dn9 = assign31800_e42056_d_n9;
        locals.var_theta_sce_edge_dn10 = assign31800_e42056_d_n10;
        locals.var_theta_sce_edge_dn11 = assign31800_e42056_d_n11;
        locals.var_theta_sce_edge_dn12 = assign31800_e42056_d_n12;
        locals.var_theta_sce_edge_dn13 = assign31800_e42056_d_n13;
        locals.var_theta_sce_edge_dn14 = assign31800_e42056_d_n14;

        let (assign31810_e42063, assign31810_e42063_d_n0, assign31810_e42063_d_n2, assign31810_e42063_d_n3, assign31810_e42063_d_n4, assign31810_e42063_d_n5, assign31810_e42063_d_n6, assign31810_e42063_d_n7, assign31810_e42063_d_n8, assign31810_e42063_d_n9, assign31810_e42063_d_n10, assign31810_e42063_d_n11, assign31810_e42063_d_n12, assign31810_e42063_d_n13, assign31810_e42063_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard733 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn0, locals.var_theta_sce_edge_dn2, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11, locals.var_theta_sce_edge_dn12, locals.var_theta_sce_edge_dn13, locals.var_theta_sce_edge_dn14,)
    }
};
        locals.var_theta_sce_edge = assign31810_e42063;
        locals.var_theta_sce_edge_dn0 = assign31810_e42063_d_n0;
        locals.var_theta_sce_edge_dn2 = assign31810_e42063_d_n2;
        locals.var_theta_sce_edge_dn3 = assign31810_e42063_d_n3;
        locals.var_theta_sce_edge_dn4 = assign31810_e42063_d_n4;
        locals.var_theta_sce_edge_dn5 = assign31810_e42063_d_n5;
        locals.var_theta_sce_edge_dn6 = assign31810_e42063_d_n6;
        locals.var_theta_sce_edge_dn7 = assign31810_e42063_d_n7;
        locals.var_theta_sce_edge_dn8 = assign31810_e42063_d_n8;
        locals.var_theta_sce_edge_dn9 = assign31810_e42063_d_n9;
        locals.var_theta_sce_edge_dn10 = assign31810_e42063_d_n10;
        locals.var_theta_sce_edge_dn11 = assign31810_e42063_d_n11;
        locals.var_theta_sce_edge_dn12 = assign31810_e42063_d_n12;
        locals.var_theta_sce_edge_dn13 = assign31810_e42063_d_n13;
        locals.var_theta_sce_edge_dn14 = assign31810_e42063_d_n14;

        let (assign31820_e42071, assign31820_e42071_d_n0, assign31820_e42071_d_n2, assign31820_e42071_d_n3, assign31820_e42071_d_n4, assign31820_e42071_d_n5, assign31820_e42071_d_n6, assign31820_e42071_d_n7, assign31820_e42071_d_n8, assign31820_e42071_d_n9, assign31820_e42071_d_n10, assign31820_e42071_d_n11, assign31820_e42071_d_n12, assign31820_e42071_d_n13, assign31820_e42071_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31820_e42068: f64 = (locals.var_vbi_edge - locals.var_phist);
        let assign31820_e42069: f64 = (locals.var_theta_sce_edge * assign31820_e42068);
        (assign31820_e42069, ((locals.var_theta_sce_edge_dn0 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn0 - locals.var_phist_dn0))), ((locals.var_theta_sce_edge_dn2 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn2 - locals.var_phist_dn2))), ((locals.var_theta_sce_edge_dn3 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn3 - locals.var_phist_dn3))), ((locals.var_theta_sce_edge_dn4 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn4 - locals.var_phist_dn4))), ((locals.var_theta_sce_edge_dn5 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn5 - locals.var_phist_dn5))), ((locals.var_theta_sce_edge_dn6 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn6 - locals.var_phist_dn6))), ((locals.var_theta_sce_edge_dn7 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn7 - locals.var_phist_dn7))), ((locals.var_theta_sce_edge_dn8 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn8 - locals.var_phist_dn8))), ((locals.var_theta_sce_edge_dn9 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn9 - locals.var_phist_dn9))), ((locals.var_theta_sce_edge_dn10 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn10 - locals.var_phist_dn10))), ((locals.var_theta_sce_edge_dn11 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn11 - locals.var_phist_dn11))), ((locals.var_theta_sce_edge_dn12 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn12 - locals.var_phist_dn12))), ((locals.var_theta_sce_edge_dn13 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn13 - locals.var_phist_dn13))), ((locals.var_theta_sce_edge_dn14 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn14 - locals.var_phist_dn14))),)
    } else {
        (locals.var_dvth_sce, locals.var_dvth_sce_dn0, locals.var_dvth_sce_dn2, locals.var_dvth_sce_dn3, locals.var_dvth_sce_dn4, locals.var_dvth_sce_dn5, locals.var_dvth_sce_dn6, locals.var_dvth_sce_dn7, locals.var_dvth_sce_dn8, locals.var_dvth_sce_dn9, locals.var_dvth_sce_dn10, locals.var_dvth_sce_dn11, locals.var_dvth_sce_dn12, locals.var_dvth_sce_dn13, locals.var_dvth_sce_dn14,)
    }
};
        locals.var_dvth_sce = assign31820_e42071;
        locals.var_dvth_sce_dn0 = assign31820_e42071_d_n0;
        locals.var_dvth_sce_dn2 = assign31820_e42071_d_n2;
        locals.var_dvth_sce_dn3 = assign31820_e42071_d_n3;
        locals.var_dvth_sce_dn4 = assign31820_e42071_d_n4;
        locals.var_dvth_sce_dn5 = assign31820_e42071_d_n5;
        locals.var_dvth_sce_dn6 = assign31820_e42071_d_n6;
        locals.var_dvth_sce_dn7 = assign31820_e42071_d_n7;
        locals.var_dvth_sce_dn8 = assign31820_e42071_d_n8;
        locals.var_dvth_sce_dn9 = assign31820_e42071_d_n9;
        locals.var_dvth_sce_dn10 = assign31820_e42071_d_n10;
        locals.var_dvth_sce_dn11 = assign31820_e42071_d_n11;
        locals.var_dvth_sce_dn12 = assign31820_e42071_d_n12;
        locals.var_dvth_sce_dn13 = assign31820_e42071_d_n13;
        locals.var_dvth_sce_dn14 = assign31820_e42071_d_n14;

    }

    pub(super) fn stamp_transient_block_101(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31830_e42091, assign31830_e42091_d_n0, assign31830_e42091_d_n2, assign31830_e42091_d_n3, assign31830_e42091_d_n4, assign31830_e42091_d_n5, assign31830_e42091_d_n6, assign31830_e42091_d_n7, assign31830_e42091_d_n8, assign31830_e42091_d_n9, assign31830_e42091_d_n10, assign31830_e42091_d_n11, assign31830_e42091_d_n12, assign31830_e42091_d_n13, assign31830_e42091_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31830_e42075: f64 = (locals.var_dvth_dibl_1 - locals.var_dvth_temp);
        let assign31830_e42077: f64 = (assign31830_e42075 + locals.var_dvth_sce);
        let assign31830_e42079: f64 = (assign31830_e42077 + p.p961);
        let assign31830_e42081: f64 = (assign31830_e42079 + locals.var_vth0_stress_edge);
        let assign31830_e42084: f64 = (locals.var_k2edge_i + locals.var_k2_well_edge);
        let assign31830_e42086: f64 = (assign31830_e42084 * locals.var_vbsx);
        let assign31830_e42087: f64 = (assign31830_e42081 - assign31830_e42086);
        let assign31830_e42089: f64 = (assign31830_e42087 + locals.var_vth0_well_edge);
        (assign31830_e42089, (((((locals.var_dvth_dibl_1_dn0 - locals.var_dvth_temp_dn0) + locals.var_dvth_sce_dn0) + locals.var_vth0_stress_edge_dn0) - (((locals.var_k2edge_i_dn0 + locals.var_k2_well_edge_dn0) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn0))) + locals.var_vth0_well_edge_dn0), (((((locals.var_dvth_dibl_1_dn2 - locals.var_dvth_temp_dn2) + locals.var_dvth_sce_dn2) + locals.var_vth0_stress_edge_dn2) - (((locals.var_k2edge_i_dn2 + locals.var_k2_well_edge_dn2) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn2))) + locals.var_vth0_well_edge_dn2), (((((locals.var_dvth_dibl_1_dn3 - locals.var_dvth_temp_dn3) + locals.var_dvth_sce_dn3) + locals.var_vth0_stress_edge_dn3) - (((locals.var_k2edge_i_dn3 + locals.var_k2_well_edge_dn3) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn3))) + locals.var_vth0_well_edge_dn3), (((((locals.var_dvth_dibl_1_dn4 - locals.var_dvth_temp_dn4) + locals.var_dvth_sce_dn4) + locals.var_vth0_stress_edge_dn4) - (((locals.var_k2edge_i_dn4 + locals.var_k2_well_edge_dn4) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn4))) + locals.var_vth0_well_edge_dn4), (((((locals.var_dvth_dibl_1_dn5 - locals.var_dvth_temp_dn5) + locals.var_dvth_sce_dn5) + locals.var_vth0_stress_edge_dn5) - (((locals.var_k2edge_i_dn5 + locals.var_k2_well_edge_dn5) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn5))) + locals.var_vth0_well_edge_dn5), (((((locals.var_dvth_dibl_1_dn6 - locals.var_dvth_temp_dn6) + locals.var_dvth_sce_dn6) + locals.var_vth0_stress_edge_dn6) - (((locals.var_k2edge_i_dn6 + locals.var_k2_well_edge_dn6) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn6))) + locals.var_vth0_well_edge_dn6), (((((locals.var_dvth_dibl_1_dn7 - locals.var_dvth_temp_dn7) + locals.var_dvth_sce_dn7) + locals.var_vth0_stress_edge_dn7) - (((locals.var_k2edge_i_dn7 + locals.var_k2_well_edge_dn7) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn7))) + locals.var_vth0_well_edge_dn7), (((((locals.var_dvth_dibl_1_dn8 - locals.var_dvth_temp_dn8) + locals.var_dvth_sce_dn8) + locals.var_vth0_stress_edge_dn8) - (((locals.var_k2edge_i_dn8 + locals.var_k2_well_edge_dn8) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn8))) + locals.var_vth0_well_edge_dn8), (((((locals.var_dvth_dibl_1_dn9 - locals.var_dvth_temp_dn9) + locals.var_dvth_sce_dn9) + locals.var_vth0_stress_edge_dn9) - (((locals.var_k2edge_i_dn9 + locals.var_k2_well_edge_dn9) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn9))) + locals.var_vth0_well_edge_dn9), (((((locals.var_dvth_dibl_1_dn10 - locals.var_dvth_temp_dn10) + locals.var_dvth_sce_dn10) + locals.var_vth0_stress_edge_dn10) - (((locals.var_k2edge_i_dn10 + locals.var_k2_well_edge_dn10) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn10))) + locals.var_vth0_well_edge_dn10), (((((locals.var_dvth_dibl_1_dn11 - locals.var_dvth_temp_dn11) + locals.var_dvth_sce_dn11) + locals.var_vth0_stress_edge_dn11) - (((locals.var_k2edge_i_dn11 + locals.var_k2_well_edge_dn11) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn11))) + locals.var_vth0_well_edge_dn11), (((((locals.var_dvth_dibl_1_dn12 - locals.var_dvth_temp_dn12) + locals.var_dvth_sce_dn12) + locals.var_vth0_stress_edge_dn12) - (((locals.var_k2edge_i_dn12 + locals.var_k2_well_edge_dn12) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn12))) + locals.var_vth0_well_edge_dn12), (((((locals.var_dvth_dibl_1_dn13 - locals.var_dvth_temp_dn13) + locals.var_dvth_sce_dn13) + locals.var_vth0_stress_edge_dn13) - (((locals.var_k2edge_i_dn13 + locals.var_k2_well_edge_dn13) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn13))) + locals.var_vth0_well_edge_dn13), (((((locals.var_dvth_dibl_1_dn14 - locals.var_dvth_temp_dn14) + locals.var_dvth_sce_dn14) + locals.var_vth0_stress_edge_dn14) - (((locals.var_k2edge_i_dn14 + locals.var_k2_well_edge_dn14) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn14))) + locals.var_vth0_well_edge_dn14),)
    } else {
        (locals.var_vth_shift, locals.var_vth_shift_dn0, locals.var_vth_shift_dn2, locals.var_vth_shift_dn3, locals.var_vth_shift_dn4, locals.var_vth_shift_dn5, locals.var_vth_shift_dn6, locals.var_vth_shift_dn7, locals.var_vth_shift_dn8, locals.var_vth_shift_dn9, locals.var_vth_shift_dn10, locals.var_vth_shift_dn11, locals.var_vth_shift_dn12, locals.var_vth_shift_dn13, locals.var_vth_shift_dn14,)
    }
};
        locals.var_vth_shift = assign31830_e42091;
        locals.var_vth_shift_dn0 = assign31830_e42091_d_n0;
        locals.var_vth_shift_dn2 = assign31830_e42091_d_n2;
        locals.var_vth_shift_dn3 = assign31830_e42091_d_n3;
        locals.var_vth_shift_dn4 = assign31830_e42091_d_n4;
        locals.var_vth_shift_dn5 = assign31830_e42091_d_n5;
        locals.var_vth_shift_dn6 = assign31830_e42091_d_n6;
        locals.var_vth_shift_dn7 = assign31830_e42091_d_n7;
        locals.var_vth_shift_dn8 = assign31830_e42091_d_n8;
        locals.var_vth_shift_dn9 = assign31830_e42091_d_n9;
        locals.var_vth_shift_dn10 = assign31830_e42091_d_n10;
        locals.var_vth_shift_dn11 = assign31830_e42091_d_n11;
        locals.var_vth_shift_dn12 = assign31830_e42091_d_n12;
        locals.var_vth_shift_dn13 = assign31830_e42091_d_n13;
        locals.var_vth_shift_dn14 = assign31830_e42091_d_n14;

        let (assign31840_e42101, assign31840_e42101_d_n0, assign31840_e42101_d_n2, assign31840_e42101_d_n3, assign31840_e42101_d_n4, assign31840_e42101_d_n5, assign31840_e42101_d_n6, assign31840_e42101_d_n7, assign31840_e42101_d_n8, assign31840_e42101_d_n9, assign31840_e42101_d_n10, assign31840_e42101_d_n11, assign31840_e42101_d_n12, assign31840_e42101_d_n13, assign31840_e42101_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31840_e42095: f64 = (locals.var_vg_1 - locals.var_vfb);
        let assign31840_e42098: f64 = (locals.var_vth_shift * locals.var_inv_nvt);
        let assign31840_e42099: f64 = (assign31840_e42095 - assign31840_e42098);
        (assign31840_e42099, ((locals.var_vg_1_dn0 - locals.var_vfb_dn0) - ((locals.var_vth_shift_dn0 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn0))), ((locals.var_vg_1_dn2 - locals.var_vfb_dn2) - ((locals.var_vth_shift_dn2 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn2))), ((locals.var_vg_1_dn3 - locals.var_vfb_dn3) - ((locals.var_vth_shift_dn3 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn3))), ((locals.var_vg_1_dn4 - locals.var_vfb_dn4) - ((locals.var_vth_shift_dn4 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn4))), ((locals.var_vg_1_dn5 - locals.var_vfb_dn5) - ((locals.var_vth_shift_dn5 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn5))), ((locals.var_vg_1_dn6 - locals.var_vfb_dn6) - ((locals.var_vth_shift_dn6 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn6))), ((locals.var_vg_1_dn7 - locals.var_vfb_dn7) - ((locals.var_vth_shift_dn7 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn7))), ((locals.var_vg_1_dn8 - locals.var_vfb_dn8) - ((locals.var_vth_shift_dn8 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn8))), ((locals.var_vg_1_dn9 - locals.var_vfb_dn9) - ((locals.var_vth_shift_dn9 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn9))), ((locals.var_vg_1_dn10 - locals.var_vfb_dn10) - ((locals.var_vth_shift_dn10 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn10))), ((locals.var_vg_1_dn11 - locals.var_vfb_dn11) - ((locals.var_vth_shift_dn11 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn11))), ((locals.var_vg_1_dn12 - locals.var_vfb_dn12) - ((locals.var_vth_shift_dn12 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn12))), ((locals.var_vg_1_dn13 - locals.var_vfb_dn13) - ((locals.var_vth_shift_dn13 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn13))), ((locals.var_vg_1_dn14 - locals.var_vfb_dn14) - ((locals.var_vth_shift_dn14 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn14))),)
    } else {
        (locals.var_vgfb, locals.var_vgfb_dn0, locals.var_vgfb_dn2, locals.var_vgfb_dn3, locals.var_vgfb_dn4, locals.var_vgfb_dn5, locals.var_vgfb_dn6, locals.var_vgfb_dn7, locals.var_vgfb_dn8, locals.var_vgfb_dn9, locals.var_vgfb_dn10, locals.var_vgfb_dn11, locals.var_vgfb_dn12, locals.var_vgfb_dn13, locals.var_vgfb_dn14,)
    }
};
        locals.var_vgfb = assign31840_e42101;
        locals.var_vgfb_dn0 = assign31840_e42101_d_n0;
        locals.var_vgfb_dn2 = assign31840_e42101_d_n2;
        locals.var_vgfb_dn3 = assign31840_e42101_d_n3;
        locals.var_vgfb_dn4 = assign31840_e42101_d_n4;
        locals.var_vgfb_dn5 = assign31840_e42101_d_n5;
        locals.var_vgfb_dn6 = assign31840_e42101_d_n6;
        locals.var_vgfb_dn7 = assign31840_e42101_d_n7;
        locals.var_vgfb_dn8 = assign31840_e42101_d_n8;
        locals.var_vgfb_dn9 = assign31840_e42101_d_n9;
        locals.var_vgfb_dn10 = assign31840_e42101_d_n10;
        locals.var_vgfb_dn11 = assign31840_e42101_d_n11;
        locals.var_vgfb_dn12 = assign31840_e42101_d_n12;
        locals.var_vgfb_dn13 = assign31840_e42101_d_n13;
        locals.var_vgfb_dn14 = assign31840_e42101_d_n14;

        let (assign31850_e42114,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31850_e42108: f64 = (-p.p960);
        let assign31850_e42109: f64 = (locals.var_leff).powf(assign31850_e42108);
        let assign31850_e42110: f64 = (p.p959 * assign31850_e42109);
        let assign31850_e42111: f64 = (1.0 + assign31850_e42110);
        let assign31850_e42112: f64 = (p.p958 * assign31850_e42111);
        (assign31850_e42112,)
    } else {
        (locals.var_dgammaedge_i,)
    }
};
        locals.var_dgammaedge_i = assign31850_e42114;

        let (assign31860_e42129, assign31860_e42129_d_n0, assign31860_e42129_d_n2, assign31860_e42129_d_n3, assign31860_e42129_d_n4, assign31860_e42129_d_n5, assign31860_e42129_d_n6, assign31860_e42129_d_n7, assign31860_e42129_d_n8, assign31860_e42129_d_n9, assign31860_e42129_d_n10, assign31860_e42129_d_n11, assign31860_e42129_d_n12, assign31860_e42129_d_n13, assign31860_e42129_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31860_e42118: f64 = (2.0 * 1.60219e-19);
        let assign31860_e42120: f64 = (assign31860_e42118 * locals.var_epssi);
        let assign31860_e42122: f64 = (assign31860_e42120 * locals.var_ndepedge_i);
        let assign31860_e42124: f64 = (assign31860_e42122 * locals.var_inv_nvt);
        let assign31860_e42125: f64 = (assign31860_e42124).sqrt();
        let assign31860_e42127: f64 = (assign31860_e42125 / locals.var_cox);
        (assign31860_e42127, (((assign31860_e42122 * locals.var_inv_nvt_dn0) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn2) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn3) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn4) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn5) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn6) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn7) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn8) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn9) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn10) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn11) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn12) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn13) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn14) / (2.0 * assign31860_e42125)) / locals.var_cox),)
    } else {
        (locals.var_gam_edge, locals.var_gam_edge_dn0, locals.var_gam_edge_dn2, locals.var_gam_edge_dn3, locals.var_gam_edge_dn4, locals.var_gam_edge_dn5, locals.var_gam_edge_dn6, locals.var_gam_edge_dn7, locals.var_gam_edge_dn8, locals.var_gam_edge_dn9, locals.var_gam_edge_dn10, locals.var_gam_edge_dn11, locals.var_gam_edge_dn12, locals.var_gam_edge_dn13, locals.var_gam_edge_dn14,)
    }
};
        locals.var_gam_edge = assign31860_e42129;
        locals.var_gam_edge_dn0 = assign31860_e42129_d_n0;
        locals.var_gam_edge_dn2 = assign31860_e42129_d_n2;
        locals.var_gam_edge_dn3 = assign31860_e42129_d_n3;
        locals.var_gam_edge_dn4 = assign31860_e42129_d_n4;
        locals.var_gam_edge_dn5 = assign31860_e42129_d_n5;
        locals.var_gam_edge_dn6 = assign31860_e42129_d_n6;
        locals.var_gam_edge_dn7 = assign31860_e42129_d_n7;
        locals.var_gam_edge_dn8 = assign31860_e42129_d_n8;
        locals.var_gam_edge_dn9 = assign31860_e42129_d_n9;
        locals.var_gam_edge_dn10 = assign31860_e42129_d_n10;
        locals.var_gam_edge_dn11 = assign31860_e42129_d_n11;
        locals.var_gam_edge_dn12 = assign31860_e42129_d_n12;
        locals.var_gam_edge_dn13 = assign31860_e42129_d_n13;
        locals.var_gam_edge_dn14 = assign31860_e42129_d_n14;

        let (assign31870_e42137, assign31870_e42137_d_n0, assign31870_e42137_d_n2, assign31870_e42137_d_n3, assign31870_e42137_d_n4, assign31870_e42137_d_n5, assign31870_e42137_d_n6, assign31870_e42137_d_n7, assign31870_e42137_d_n8, assign31870_e42137_d_n9, assign31870_e42137_d_n10, assign31870_e42137_d_n11, assign31870_e42137_d_n12, assign31870_e42137_d_n13, assign31870_e42137_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31870_e42134: f64 = (1.0 + locals.var_dgammaedge_i);
        let assign31870_e42135: f64 = (locals.var_gam_edge * assign31870_e42134);
        (assign31870_e42135, (locals.var_gam_edge_dn0 * assign31870_e42134), (locals.var_gam_edge_dn2 * assign31870_e42134), (locals.var_gam_edge_dn3 * assign31870_e42134), (locals.var_gam_edge_dn4 * assign31870_e42134), (locals.var_gam_edge_dn5 * assign31870_e42134), (locals.var_gam_edge_dn6 * assign31870_e42134), (locals.var_gam_edge_dn7 * assign31870_e42134), (locals.var_gam_edge_dn8 * assign31870_e42134), (locals.var_gam_edge_dn9 * assign31870_e42134), (locals.var_gam_edge_dn10 * assign31870_e42134), (locals.var_gam_edge_dn11 * assign31870_e42134), (locals.var_gam_edge_dn12 * assign31870_e42134), (locals.var_gam_edge_dn13 * assign31870_e42134), (locals.var_gam_edge_dn14 * assign31870_e42134),)
    } else {
        (locals.var_gam_edge, locals.var_gam_edge_dn0, locals.var_gam_edge_dn2, locals.var_gam_edge_dn3, locals.var_gam_edge_dn4, locals.var_gam_edge_dn5, locals.var_gam_edge_dn6, locals.var_gam_edge_dn7, locals.var_gam_edge_dn8, locals.var_gam_edge_dn9, locals.var_gam_edge_dn10, locals.var_gam_edge_dn11, locals.var_gam_edge_dn12, locals.var_gam_edge_dn13, locals.var_gam_edge_dn14,)
    }
};
        locals.var_gam_edge = assign31870_e42137;
        locals.var_gam_edge_dn0 = assign31870_e42137_d_n0;
        locals.var_gam_edge_dn2 = assign31870_e42137_d_n2;
        locals.var_gam_edge_dn3 = assign31870_e42137_d_n3;
        locals.var_gam_edge_dn4 = assign31870_e42137_d_n4;
        locals.var_gam_edge_dn5 = assign31870_e42137_d_n5;
        locals.var_gam_edge_dn6 = assign31870_e42137_d_n6;
        locals.var_gam_edge_dn7 = assign31870_e42137_d_n7;
        locals.var_gam_edge_dn8 = assign31870_e42137_d_n8;
        locals.var_gam_edge_dn9 = assign31870_e42137_d_n9;
        locals.var_gam_edge_dn10 = assign31870_e42137_d_n10;
        locals.var_gam_edge_dn11 = assign31870_e42137_d_n11;
        locals.var_gam_edge_dn12 = assign31870_e42137_d_n12;
        locals.var_gam_edge_dn13 = assign31870_e42137_d_n13;
        locals.var_gam_edge_dn14 = assign31870_e42137_d_n14;

        let (assign31880_e42143, assign31880_e42143_d_n0, assign31880_e42143_d_n2, assign31880_e42143_d_n3, assign31880_e42143_d_n4, assign31880_e42143_d_n5, assign31880_e42143_d_n6, assign31880_e42143_d_n7, assign31880_e42143_d_n8, assign31880_e42143_d_n9, assign31880_e42143_d_n10, assign31880_e42143_d_n11, assign31880_e42143_d_n12, assign31880_e42143_d_n13, assign31880_e42143_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31880_e42141: f64 = (locals.var_phib_edge / locals.var_n);
        (assign31880_e42141, (((locals.var_phib_edge_dn0 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn0)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn2 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn2)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn3 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn3)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn4 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn4)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn5 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn5)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn6 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn6)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn7 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn7)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn8 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn8)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn9 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn9)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn10 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn10)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn11 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn11)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn12 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn12)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn13 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn13)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn14 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn14)) / (locals.var_n * locals.var_n)),)
    } else {
        (locals.var_phib_n_edge, locals.var_phib_n_edge_dn0, locals.var_phib_n_edge_dn2, locals.var_phib_n_edge_dn3, locals.var_phib_n_edge_dn4, locals.var_phib_n_edge_dn5, locals.var_phib_n_edge_dn6, locals.var_phib_n_edge_dn7, locals.var_phib_n_edge_dn8, locals.var_phib_n_edge_dn9, locals.var_phib_n_edge_dn10, locals.var_phib_n_edge_dn11, locals.var_phib_n_edge_dn12, locals.var_phib_n_edge_dn13, locals.var_phib_n_edge_dn14,)
    }
};
        locals.var_phib_n_edge = assign31880_e42143;
        locals.var_phib_n_edge_dn0 = assign31880_e42143_d_n0;
        locals.var_phib_n_edge_dn2 = assign31880_e42143_d_n2;
        locals.var_phib_n_edge_dn3 = assign31880_e42143_d_n3;
        locals.var_phib_n_edge_dn4 = assign31880_e42143_d_n4;
        locals.var_phib_n_edge_dn5 = assign31880_e42143_d_n5;
        locals.var_phib_n_edge_dn6 = assign31880_e42143_d_n6;
        locals.var_phib_n_edge_dn7 = assign31880_e42143_d_n7;
        locals.var_phib_n_edge_dn8 = assign31880_e42143_d_n8;
        locals.var_phib_n_edge_dn9 = assign31880_e42143_d_n9;
        locals.var_phib_n_edge_dn10 = assign31880_e42143_d_n10;
        locals.var_phib_n_edge_dn11 = assign31880_e42143_d_n11;
        locals.var_phib_n_edge_dn12 = assign31880_e42143_d_n12;
        locals.var_phib_n_edge_dn13 = assign31880_e42143_d_n13;
        locals.var_phib_n_edge_dn14 = assign31880_e42143_d_n14;

        let (assign31890_e42149, assign31890_e42149_d_n0, assign31890_e42149_d_n2, assign31890_e42149_d_n3, assign31890_e42149_d_n4, assign31890_e42149_d_n5, assign31890_e42149_d_n6, assign31890_e42149_d_n7, assign31890_e42149_d_n8, assign31890_e42149_d_n9, assign31890_e42149_d_n10, assign31890_e42149_d_n11, assign31890_e42149_d_n12, assign31890_e42149_d_n13, assign31890_e42149_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31890_e42147: f64 = 1.0;
        (assign31890_e42147, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31890_e42149;
        locals.var_t1_dn0 = assign31890_e42149_d_n0;
        locals.var_t1_dn2 = assign31890_e42149_d_n2;
        locals.var_t1_dn3 = assign31890_e42149_d_n3;
        locals.var_t1_dn4 = assign31890_e42149_d_n4;
        locals.var_t1_dn5 = assign31890_e42149_d_n5;
        locals.var_t1_dn6 = assign31890_e42149_d_n6;
        locals.var_t1_dn7 = assign31890_e42149_d_n7;
        locals.var_t1_dn8 = assign31890_e42149_d_n8;
        locals.var_t1_dn9 = assign31890_e42149_d_n9;
        locals.var_t1_dn10 = assign31890_e42149_d_n10;
        locals.var_t1_dn11 = assign31890_e42149_d_n11;
        locals.var_t1_dn12 = assign31890_e42149_d_n12;
        locals.var_t1_dn13 = assign31890_e42149_d_n13;
        locals.var_t1_dn14 = assign31890_e42149_d_n14;

        let (assign31900_e42155, assign31900_e42155_d_n0, assign31900_e42155_d_n2, assign31900_e42155_d_n3, assign31900_e42155_d_n4, assign31900_e42155_d_n5, assign31900_e42155_d_n6, assign31900_e42155_d_n7, assign31900_e42155_d_n8, assign31900_e42155_d_n9, assign31900_e42155_d_n10, assign31900_e42155_d_n11, assign31900_e42155_d_n12, assign31900_e42155_d_n13, assign31900_e42155_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31900_e42153: f64 = (locals.var_vgfb / locals.var_t1);
        (assign31900_e42153, (((locals.var_vgfb_dn0 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn2 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn3 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn4 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn5 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn6 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn7 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn8 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn9 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn10 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn11 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn12 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn13 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn14 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_vgfbpd, locals.var_vgfbpd_dn0, locals.var_vgfbpd_dn2, locals.var_vgfbpd_dn3, locals.var_vgfbpd_dn4, locals.var_vgfbpd_dn5, locals.var_vgfbpd_dn6, locals.var_vgfbpd_dn7, locals.var_vgfbpd_dn8, locals.var_vgfbpd_dn9, locals.var_vgfbpd_dn10, locals.var_vgfbpd_dn11, locals.var_vgfbpd_dn12, locals.var_vgfbpd_dn13, locals.var_vgfbpd_dn14,)
    }
};
        locals.var_vgfbpd = assign31900_e42155;
        locals.var_vgfbpd_dn0 = assign31900_e42155_d_n0;
        locals.var_vgfbpd_dn2 = assign31900_e42155_d_n2;
        locals.var_vgfbpd_dn3 = assign31900_e42155_d_n3;
        locals.var_vgfbpd_dn4 = assign31900_e42155_d_n4;
        locals.var_vgfbpd_dn5 = assign31900_e42155_d_n5;
        locals.var_vgfbpd_dn6 = assign31900_e42155_d_n6;
        locals.var_vgfbpd_dn7 = assign31900_e42155_d_n7;
        locals.var_vgfbpd_dn8 = assign31900_e42155_d_n8;
        locals.var_vgfbpd_dn9 = assign31900_e42155_d_n9;
        locals.var_vgfbpd_dn10 = assign31900_e42155_d_n10;
        locals.var_vgfbpd_dn11 = assign31900_e42155_d_n11;
        locals.var_vgfbpd_dn12 = assign31900_e42155_d_n12;
        locals.var_vgfbpd_dn13 = assign31900_e42155_d_n13;
        locals.var_vgfbpd_dn14 = assign31900_e42155_d_n14;

        let (assign31910_e42161, assign31910_e42161_d_n0, assign31910_e42161_d_n2, assign31910_e42161_d_n3, assign31910_e42161_d_n4, assign31910_e42161_d_n5, assign31910_e42161_d_n6, assign31910_e42161_d_n7, assign31910_e42161_d_n8, assign31910_e42161_d_n9, assign31910_e42161_d_n10, assign31910_e42161_d_n11, assign31910_e42161_d_n12, assign31910_e42161_d_n13, assign31910_e42161_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31910_e42159: f64 = (locals.var_gam_edge / locals.var_t1);
        (assign31910_e42159, (((locals.var_gam_edge_dn0 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn2 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn3 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn4 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn5 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn6 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn7 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn8 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn9 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn10 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn11 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn12 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn13 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn14 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_gammapd, locals.var_gammapd_dn0, locals.var_gammapd_dn2, locals.var_gammapd_dn3, locals.var_gammapd_dn4, locals.var_gammapd_dn5, locals.var_gammapd_dn6, locals.var_gammapd_dn7, locals.var_gammapd_dn8, locals.var_gammapd_dn9, locals.var_gammapd_dn10, locals.var_gammapd_dn11, locals.var_gammapd_dn12, locals.var_gammapd_dn13, locals.var_gammapd_dn14,)
    }
};
        locals.var_gammapd = assign31910_e42161;
        locals.var_gammapd_dn0 = assign31910_e42161_d_n0;
        locals.var_gammapd_dn2 = assign31910_e42161_d_n2;
        locals.var_gammapd_dn3 = assign31910_e42161_d_n3;
        locals.var_gammapd_dn4 = assign31910_e42161_d_n4;
        locals.var_gammapd_dn5 = assign31910_e42161_d_n5;
        locals.var_gammapd_dn6 = assign31910_e42161_d_n6;
        locals.var_gammapd_dn7 = assign31910_e42161_d_n7;
        locals.var_gammapd_dn8 = assign31910_e42161_d_n8;
        locals.var_gammapd_dn9 = assign31910_e42161_d_n9;
        locals.var_gammapd_dn10 = assign31910_e42161_d_n10;
        locals.var_gammapd_dn11 = assign31910_e42161_d_n11;
        locals.var_gammapd_dn12 = assign31910_e42161_d_n12;
        locals.var_gammapd_dn13 = assign31910_e42161_d_n13;
        locals.var_gammapd_dn14 = assign31910_e42161_d_n14;

        let (assign31920_e42175, assign31920_e42175_d_n0, assign31920_e42175_d_n2, assign31920_e42175_d_n3, assign31920_e42175_d_n4, assign31920_e42175_d_n5, assign31920_e42175_d_n6, assign31920_e42175_d_n7, assign31920_e42175_d_n8, assign31920_e42175_d_n9, assign31920_e42175_d_n10, assign31920_e42175_d_n11, assign31920_e42175_d_n12, assign31920_e42175_d_n13, assign31920_e42175_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31920_e42165: f64 = (0.5 * locals.var_vgfbpd);
        let assign31920_e42170: f64 = (locals.var_gammapd / 1.4142135623730951);
        let assign31920_e42171: f64 = (1.0 + assign31920_e42170);
        let assign31920_e42172: f64 = (3.0 * assign31920_e42171);
        let assign31920_e42173: f64 = (assign31920_e42165 - assign31920_e42172);
        (assign31920_e42173, ((0.5 * locals.var_vgfbpd_dn0) - (3.0 * (locals.var_gammapd_dn0 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn2) - (3.0 * (locals.var_gammapd_dn2 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn3) - (3.0 * (locals.var_gammapd_dn3 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn4) - (3.0 * (locals.var_gammapd_dn4 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn5) - (3.0 * (locals.var_gammapd_dn5 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn6) - (3.0 * (locals.var_gammapd_dn6 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn7) - (3.0 * (locals.var_gammapd_dn7 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn8) - (3.0 * (locals.var_gammapd_dn8 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn9) - (3.0 * (locals.var_gammapd_dn9 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn10) - (3.0 * (locals.var_gammapd_dn10 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn11) - (3.0 * (locals.var_gammapd_dn11 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn12) - (3.0 * (locals.var_gammapd_dn12 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn13) - (3.0 * (locals.var_gammapd_dn13 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn14) - (3.0 * (locals.var_gammapd_dn14 / 1.4142135623730951))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31920_e42175;
        locals.var_t1_dn0 = assign31920_e42175_d_n0;
        locals.var_t1_dn2 = assign31920_e42175_d_n2;
        locals.var_t1_dn3 = assign31920_e42175_d_n3;
        locals.var_t1_dn4 = assign31920_e42175_d_n4;
        locals.var_t1_dn5 = assign31920_e42175_d_n5;
        locals.var_t1_dn6 = assign31920_e42175_d_n6;
        locals.var_t1_dn7 = assign31920_e42175_d_n7;
        locals.var_t1_dn8 = assign31920_e42175_d_n8;
        locals.var_t1_dn9 = assign31920_e42175_d_n9;
        locals.var_t1_dn10 = assign31920_e42175_d_n10;
        locals.var_t1_dn11 = assign31920_e42175_d_n11;
        locals.var_t1_dn12 = assign31920_e42175_d_n12;
        locals.var_t1_dn13 = assign31920_e42175_d_n13;
        locals.var_t1_dn14 = assign31920_e42175_d_n14;

        let (assign31930_e42188, assign31930_e42188_d_n0, assign31930_e42188_d_n2, assign31930_e42188_d_n3, assign31930_e42188_d_n4, assign31930_e42188_d_n5, assign31930_e42188_d_n6, assign31930_e42188_d_n7, assign31930_e42188_d_n8, assign31930_e42188_d_n9, assign31930_e42188_d_n10, assign31930_e42188_d_n11, assign31930_e42188_d_n12, assign31930_e42188_d_n13, assign31930_e42188_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31930_e42180: f64 = (locals.var_t1 * locals.var_t1);
        let assign31930_e42183: f64 = (6.0 * locals.var_vgfbpd);
        let assign31930_e42184: f64 = (assign31930_e42180 + assign31930_e42183);
        let assign31930_e42185: f64 = (assign31930_e42184).sqrt();
        let assign31930_e42186: f64 = (locals.var_t1 + assign31930_e42185);
        (assign31930_e42186, (locals.var_t1_dn0 + ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (6.0 * locals.var_vgfbpd_dn0)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn2 + ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (6.0 * locals.var_vgfbpd_dn2)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn3 + ((((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) + (6.0 * locals.var_vgfbpd_dn3)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn4 + ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (6.0 * locals.var_vgfbpd_dn4)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn5 + ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (6.0 * locals.var_vgfbpd_dn5)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn6 + ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (6.0 * locals.var_vgfbpd_dn6)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn7 + ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (6.0 * locals.var_vgfbpd_dn7)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn8 + ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (6.0 * locals.var_vgfbpd_dn8)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn9 + ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (6.0 * locals.var_vgfbpd_dn9)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn10 + ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (6.0 * locals.var_vgfbpd_dn10)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn11 + ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (6.0 * locals.var_vgfbpd_dn11)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn12 + ((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + (6.0 * locals.var_vgfbpd_dn12)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn13 + ((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) + (6.0 * locals.var_vgfbpd_dn13)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn14 + ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + (6.0 * locals.var_vgfbpd_dn14)) / (2.0 * assign31930_e42185))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign31930_e42188;
        locals.var_t2_dn0 = assign31930_e42188_d_n0;
        locals.var_t2_dn2 = assign31930_e42188_d_n2;
        locals.var_t2_dn3 = assign31930_e42188_d_n3;
        locals.var_t2_dn4 = assign31930_e42188_d_n4;
        locals.var_t2_dn5 = assign31930_e42188_d_n5;
        locals.var_t2_dn6 = assign31930_e42188_d_n6;
        locals.var_t2_dn7 = assign31930_e42188_d_n7;
        locals.var_t2_dn8 = assign31930_e42188_d_n8;
        locals.var_t2_dn9 = assign31930_e42188_d_n9;
        locals.var_t2_dn10 = assign31930_e42188_d_n10;
        locals.var_t2_dn11 = assign31930_e42188_d_n11;
        locals.var_t2_dn12 = assign31930_e42188_d_n12;
        locals.var_t2_dn13 = assign31930_e42188_d_n13;
        locals.var_t2_dn14 = assign31930_e42188_d_n14;

        let assign31940_e42191: f64 = if locals.var_vgfbpd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard735 = assign31940_e42191;

        let (assign31950_e42201, assign31950_e42201_d_n0, assign31950_e42201_d_n2, assign31950_e42201_d_n3, assign31950_e42201_d_n4, assign31950_e42201_d_n5, assign31950_e42201_d_n6, assign31950_e42201_d_n7, assign31950_e42201_d_n8, assign31950_e42201_d_n9, assign31950_e42201_d_n10, assign31950_e42201_d_n11, assign31950_e42201_d_n12, assign31950_e42201_d_n13, assign31950_e42201_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 != 0.0)) {
        let assign31950_e42197: f64 = (locals.var_vgfbpd - locals.var_t2);
        let assign31950_e42199: f64 = (assign31950_e42197 / locals.var_gammapd);
        (assign31950_e42199, ((((locals.var_vgfbpd_dn0 - locals.var_t2_dn0) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn0)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn2 - locals.var_t2_dn2) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn2)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn3 - locals.var_t2_dn3) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn3)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn4 - locals.var_t2_dn4) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn4)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn5 - locals.var_t2_dn5) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn5)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn6 - locals.var_t2_dn6) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn6)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn7 - locals.var_t2_dn7) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn7)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn8 - locals.var_t2_dn8) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn8)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn9 - locals.var_t2_dn9) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn9)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn10 - locals.var_t2_dn10) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn10)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn11 - locals.var_t2_dn11) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn11)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn12 - locals.var_t2_dn12) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn12)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn13 - locals.var_t2_dn13) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn13)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn14 - locals.var_t2_dn14) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn14)) / (locals.var_gammapd * locals.var_gammapd)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign31950_e42201;
        locals.var_t3_dn0 = assign31950_e42201_d_n0;
        locals.var_t3_dn2 = assign31950_e42201_d_n2;
        locals.var_t3_dn3 = assign31950_e42201_d_n3;
        locals.var_t3_dn4 = assign31950_e42201_d_n4;
        locals.var_t3_dn5 = assign31950_e42201_d_n5;
        locals.var_t3_dn6 = assign31950_e42201_d_n6;
        locals.var_t3_dn7 = assign31950_e42201_d_n7;
        locals.var_t3_dn8 = assign31950_e42201_d_n8;
        locals.var_t3_dn9 = assign31950_e42201_d_n9;
        locals.var_t3_dn10 = assign31950_e42201_d_n10;
        locals.var_t3_dn11 = assign31950_e42201_d_n11;
        locals.var_t3_dn12 = assign31950_e42201_d_n12;
        locals.var_t3_dn13 = assign31950_e42201_d_n13;
        locals.var_t3_dn14 = assign31950_e42201_d_n14;

        let (assign31960_e42217, assign31960_e42217_d_n0, assign31960_e42217_d_n2, assign31960_e42217_d_n3, assign31960_e42217_d_n4, assign31960_e42217_d_n5, assign31960_e42217_d_n6, assign31960_e42217_d_n7, assign31960_e42217_d_n8, assign31960_e42217_d_n9, assign31960_e42217_d_n10, assign31960_e42217_d_n11, assign31960_e42217_d_n12, assign31960_e42217_d_n13, assign31960_e42217_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 != 0.0)) {
        let assign31960_e42207: f64 = (1.0 - locals.var_t2);
        let assign31960_e42210: f64 = (locals.var_t3 * locals.var_t3);
        let assign31960_e42211: f64 = (assign31960_e42207 + assign31960_e42210);
        let assign31960_e42213: f64 = (assign31960_e42211).max(1e-38);
        let assign31960_e42214: f64 = (assign31960_e42213).ln();
        let assign31960_e42215: f64 = (-assign31960_e42214);
        (assign31960_e42215, (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn0) + ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn2) + ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn3) + ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn4) + ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn5) + ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn6) + ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn7) + ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn8) + ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn9) + ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn10) + ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn11) + ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn12) + ((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn13) + ((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn14) + ((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14))) } else { 0.0 } / assign31960_e42213)),)
    } else {
        (locals.var_psip, locals.var_psip_dn0, locals.var_psip_dn2, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11, locals.var_psip_dn12, locals.var_psip_dn13, locals.var_psip_dn14,)
    }
};
        locals.var_psip = assign31960_e42217;
        locals.var_psip_dn0 = assign31960_e42217_d_n0;
        locals.var_psip_dn2 = assign31960_e42217_d_n2;
        locals.var_psip_dn3 = assign31960_e42217_d_n3;
        locals.var_psip_dn4 = assign31960_e42217_d_n4;
        locals.var_psip_dn5 = assign31960_e42217_d_n5;
        locals.var_psip_dn6 = assign31960_e42217_d_n6;
        locals.var_psip_dn7 = assign31960_e42217_d_n7;
        locals.var_psip_dn8 = assign31960_e42217_d_n8;
        locals.var_psip_dn9 = assign31960_e42217_d_n9;
        locals.var_psip_dn10 = assign31960_e42217_d_n10;
        locals.var_psip_dn11 = assign31960_e42217_d_n11;
        locals.var_psip_dn12 = assign31960_e42217_d_n12;
        locals.var_psip_dn13 = assign31960_e42217_d_n13;
        locals.var_psip_dn14 = assign31960_e42217_d_n14;

        let (assign31970_e42226, assign31970_e42226_d_n0, assign31970_e42226_d_n2, assign31970_e42226_d_n3, assign31970_e42226_d_n4, assign31970_e42226_d_n5, assign31970_e42226_d_n6, assign31970_e42226_d_n7, assign31970_e42226_d_n8, assign31970_e42226_d_n9, assign31970_e42226_d_n10, assign31970_e42226_d_n11, assign31970_e42226_d_n12, assign31970_e42226_d_n13, assign31970_e42226_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign31970_e42223: f64 = (-locals.var_t2);
        let assign31970_e42224: f64 = { let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign31970_e42224, ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn12)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign31970_e42226;
        locals.var_t3_dn0 = assign31970_e42226_d_n0;
        locals.var_t3_dn2 = assign31970_e42226_d_n2;
        locals.var_t3_dn3 = assign31970_e42226_d_n3;
        locals.var_t3_dn4 = assign31970_e42226_d_n4;
        locals.var_t3_dn5 = assign31970_e42226_d_n5;
        locals.var_t3_dn6 = assign31970_e42226_d_n6;
        locals.var_t3_dn7 = assign31970_e42226_d_n7;
        locals.var_t3_dn8 = assign31970_e42226_d_n8;
        locals.var_t3_dn9 = assign31970_e42226_d_n9;
        locals.var_t3_dn10 = assign31970_e42226_d_n10;
        locals.var_t3_dn11 = assign31970_e42226_d_n11;
        locals.var_t3_dn12 = assign31970_e42226_d_n12;
        locals.var_t3_dn13 = assign31970_e42226_d_n13;
        locals.var_t3_dn14 = assign31970_e42226_d_n14;

        let (assign31980_e42235, assign31980_e42235_d_n0, assign31980_e42235_d_n2, assign31980_e42235_d_n3, assign31980_e42235_d_n4, assign31980_e42235_d_n5, assign31980_e42235_d_n6, assign31980_e42235_d_n7, assign31980_e42235_d_n8, assign31980_e42235_d_n9, assign31980_e42235_d_n10, assign31980_e42235_d_n11, assign31980_e42235_d_n12, assign31980_e42235_d_n13, assign31980_e42235_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign31980_e42233: f64 = (0.5 * locals.var_gammapd);
        (assign31980_e42233, (0.5 * locals.var_gammapd_dn0), (0.5 * locals.var_gammapd_dn2), (0.5 * locals.var_gammapd_dn3), (0.5 * locals.var_gammapd_dn4), (0.5 * locals.var_gammapd_dn5), (0.5 * locals.var_gammapd_dn6), (0.5 * locals.var_gammapd_dn7), (0.5 * locals.var_gammapd_dn8), (0.5 * locals.var_gammapd_dn9), (0.5 * locals.var_gammapd_dn10), (0.5 * locals.var_gammapd_dn11), (0.5 * locals.var_gammapd_dn12), (0.5 * locals.var_gammapd_dn13), (0.5 * locals.var_gammapd_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31980_e42235;
        locals.var_t1_dn0 = assign31980_e42235_d_n0;
        locals.var_t1_dn2 = assign31980_e42235_d_n2;
        locals.var_t1_dn3 = assign31980_e42235_d_n3;
        locals.var_t1_dn4 = assign31980_e42235_d_n4;
        locals.var_t1_dn5 = assign31980_e42235_d_n5;
        locals.var_t1_dn6 = assign31980_e42235_d_n6;
        locals.var_t1_dn7 = assign31980_e42235_d_n7;
        locals.var_t1_dn8 = assign31980_e42235_d_n8;
        locals.var_t1_dn9 = assign31980_e42235_d_n9;
        locals.var_t1_dn10 = assign31980_e42235_d_n10;
        locals.var_t1_dn11 = assign31980_e42235_d_n11;
        locals.var_t1_dn12 = assign31980_e42235_d_n12;
        locals.var_t1_dn13 = assign31980_e42235_d_n13;
        locals.var_t1_dn14 = assign31980_e42235_d_n14;

        let (assign31990_e42253, assign31990_e42253_d_n0, assign31990_e42253_d_n2, assign31990_e42253_d_n3, assign31990_e42253_d_n4, assign31990_e42253_d_n5, assign31990_e42253_d_n6, assign31990_e42253_d_n7, assign31990_e42253_d_n8, assign31990_e42253_d_n9, assign31990_e42253_d_n10, assign31990_e42253_d_n11, assign31990_e42253_d_n12, assign31990_e42253_d_n13, assign31990_e42253_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign31990_e42242: f64 = (locals.var_vgfbpd - 1.0);
        let assign31990_e42244: f64 = (assign31990_e42242 + locals.var_t3);
        let assign31990_e42247: f64 = (locals.var_t1 * locals.var_t1);
        let assign31990_e42248: f64 = (assign31990_e42244 + assign31990_e42247);
        let assign31990_e42249: f64 = (assign31990_e42248).sqrt();
        let assign31990_e42251: f64 = (assign31990_e42249 - locals.var_t1);
        (assign31990_e42251, ((((locals.var_vgfbpd_dn0 + locals.var_t3_dn0) + ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn0), ((((locals.var_vgfbpd_dn2 + locals.var_t3_dn2) + ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn2), ((((locals.var_vgfbpd_dn3 + locals.var_t3_dn3) + ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn3), ((((locals.var_vgfbpd_dn4 + locals.var_t3_dn4) + ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn4), ((((locals.var_vgfbpd_dn5 + locals.var_t3_dn5) + ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn5), ((((locals.var_vgfbpd_dn6 + locals.var_t3_dn6) + ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn6), ((((locals.var_vgfbpd_dn7 + locals.var_t3_dn7) + ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn7), ((((locals.var_vgfbpd_dn8 + locals.var_t3_dn8) + ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn8), ((((locals.var_vgfbpd_dn9 + locals.var_t3_dn9) + ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn9), ((((locals.var_vgfbpd_dn10 + locals.var_t3_dn10) + ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn10), ((((locals.var_vgfbpd_dn11 + locals.var_t3_dn11) + ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn11), ((((locals.var_vgfbpd_dn12 + locals.var_t3_dn12) + ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn12), ((((locals.var_vgfbpd_dn13 + locals.var_t3_dn13) + ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn13), ((((locals.var_vgfbpd_dn14 + locals.var_t3_dn14) + ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign31990_e42253;
        locals.var_t2_dn0 = assign31990_e42253_d_n0;
        locals.var_t2_dn2 = assign31990_e42253_d_n2;
        locals.var_t2_dn3 = assign31990_e42253_d_n3;
        locals.var_t2_dn4 = assign31990_e42253_d_n4;
        locals.var_t2_dn5 = assign31990_e42253_d_n5;
        locals.var_t2_dn6 = assign31990_e42253_d_n6;
        locals.var_t2_dn7 = assign31990_e42253_d_n7;
        locals.var_t2_dn8 = assign31990_e42253_d_n8;
        locals.var_t2_dn9 = assign31990_e42253_d_n9;
        locals.var_t2_dn10 = assign31990_e42253_d_n10;
        locals.var_t2_dn11 = assign31990_e42253_d_n11;
        locals.var_t2_dn12 = assign31990_e42253_d_n12;
        locals.var_t2_dn13 = assign31990_e42253_d_n13;
        locals.var_t2_dn14 = assign31990_e42253_d_n14;

        let (assign32000_e42266, assign32000_e42266_d_n0, assign32000_e42266_d_n2, assign32000_e42266_d_n3, assign32000_e42266_d_n4, assign32000_e42266_d_n5, assign32000_e42266_d_n6, assign32000_e42266_d_n7, assign32000_e42266_d_n8, assign32000_e42266_d_n9, assign32000_e42266_d_n10, assign32000_e42266_d_n11, assign32000_e42266_d_n12, assign32000_e42266_d_n13, assign32000_e42266_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign32000_e42260: f64 = (locals.var_t2 * locals.var_t2);
        let assign32000_e42262: f64 = (assign32000_e42260 + 1.0);
        let assign32000_e42264: f64 = (assign32000_e42262 - locals.var_t3);
        (assign32000_e42264, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) - locals.var_t3_dn0), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) - locals.var_t3_dn2), (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) - locals.var_t3_dn3), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) - locals.var_t3_dn4), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) - locals.var_t3_dn5), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) - locals.var_t3_dn6), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) - locals.var_t3_dn7), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) - locals.var_t3_dn8), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) - locals.var_t3_dn9), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) - locals.var_t3_dn10), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) - locals.var_t3_dn11), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) - locals.var_t3_dn12), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) - locals.var_t3_dn13), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) - locals.var_t3_dn14),)
    } else {
        (locals.var_psip, locals.var_psip_dn0, locals.var_psip_dn2, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11, locals.var_psip_dn12, locals.var_psip_dn13, locals.var_psip_dn14,)
    }
};
        locals.var_psip = assign32000_e42266;
        locals.var_psip_dn0 = assign32000_e42266_d_n0;
        locals.var_psip_dn2 = assign32000_e42266_d_n2;
        locals.var_psip_dn3 = assign32000_e42266_d_n3;
        locals.var_psip_dn4 = assign32000_e42266_d_n4;
        locals.var_psip_dn5 = assign32000_e42266_d_n5;
        locals.var_psip_dn6 = assign32000_e42266_d_n6;
        locals.var_psip_dn7 = assign32000_e42266_d_n7;
        locals.var_psip_dn8 = assign32000_e42266_d_n8;
        locals.var_psip_dn9 = assign32000_e42266_d_n9;
        locals.var_psip_dn10 = assign32000_e42266_d_n10;
        locals.var_psip_dn11 = assign32000_e42266_d_n11;
        locals.var_psip_dn12 = assign32000_e42266_d_n12;
        locals.var_psip_dn13 = assign32000_e42266_d_n13;
        locals.var_psip_dn14 = assign32000_e42266_d_n14;

        let (assign32010_e42289, assign32010_e42289_d_n0, assign32010_e42289_d_n2, assign32010_e42289_d_n3, assign32010_e42289_d_n4, assign32010_e42289_d_n5, assign32010_e42289_d_n6, assign32010_e42289_d_n7, assign32010_e42289_d_n8, assign32010_e42289_d_n9, assign32010_e42289_d_n10, assign32010_e42289_d_n11, assign32010_e42289_d_n12, assign32010_e42289_d_n13, assign32010_e42289_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32010_e42271: f64 = (locals.var_psip + 1.0);
        let assign32010_e42274: f64 = (locals.var_psip - 1.0);
        let assign32010_e42277: f64 = (locals.var_psip - 1.0);
        let assign32010_e42278: f64 = (assign32010_e42274 * assign32010_e42277);
        let assign32010_e42281: f64 = (0.25 * 2.0);
        let assign32010_e42283: f64 = (assign32010_e42281 * 2.0);
        let assign32010_e42284: f64 = (assign32010_e42278 + assign32010_e42283);
        let assign32010_e42285: f64 = (assign32010_e42284).sqrt();
        let assign32010_e42286: f64 = (assign32010_e42271 + assign32010_e42285);
        let assign32010_e42287: f64 = (0.5 * assign32010_e42286);
        (assign32010_e42287, (0.5 * (locals.var_psip_dn0 + (((locals.var_psip_dn0 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn0)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn2 + (((locals.var_psip_dn2 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn2)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn3)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn4)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn5)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn6)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn7)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn8)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn9)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn10)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn11)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn12 + (((locals.var_psip_dn12 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn12)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn13 + (((locals.var_psip_dn13 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn13)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn14 + (((locals.var_psip_dn14 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn14)) / (2.0 * assign32010_e42285)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32010_e42289;
        locals.var_t8_dn0 = assign32010_e42289_d_n0;
        locals.var_t8_dn2 = assign32010_e42289_d_n2;
        locals.var_t8_dn3 = assign32010_e42289_d_n3;
        locals.var_t8_dn4 = assign32010_e42289_d_n4;
        locals.var_t8_dn5 = assign32010_e42289_d_n5;
        locals.var_t8_dn6 = assign32010_e42289_d_n6;
        locals.var_t8_dn7 = assign32010_e42289_d_n7;
        locals.var_t8_dn8 = assign32010_e42289_d_n8;
        locals.var_t8_dn9 = assign32010_e42289_d_n9;
        locals.var_t8_dn10 = assign32010_e42289_d_n10;
        locals.var_t8_dn11 = assign32010_e42289_d_n11;
        locals.var_t8_dn12 = assign32010_e42289_d_n12;
        locals.var_t8_dn13 = assign32010_e42289_d_n13;
        locals.var_t8_dn14 = assign32010_e42289_d_n14;

        let (assign32020_e42294, assign32020_e42294_d_n0, assign32020_e42294_d_n2, assign32020_e42294_d_n3, assign32020_e42294_d_n4, assign32020_e42294_d_n5, assign32020_e42294_d_n6, assign32020_e42294_d_n7, assign32020_e42294_d_n8, assign32020_e42294_d_n9, assign32020_e42294_d_n10, assign32020_e42294_d_n11, assign32020_e42294_d_n12, assign32020_e42294_d_n13, assign32020_e42294_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32020_e42292: f64 = (locals.var_t8).sqrt();
        (assign32020_e42292, (locals.var_t8_dn0 / (2.0 * assign32020_e42292)), (locals.var_t8_dn2 / (2.0 * assign32020_e42292)), (locals.var_t8_dn3 / (2.0 * assign32020_e42292)), (locals.var_t8_dn4 / (2.0 * assign32020_e42292)), (locals.var_t8_dn5 / (2.0 * assign32020_e42292)), (locals.var_t8_dn6 / (2.0 * assign32020_e42292)), (locals.var_t8_dn7 / (2.0 * assign32020_e42292)), (locals.var_t8_dn8 / (2.0 * assign32020_e42292)), (locals.var_t8_dn9 / (2.0 * assign32020_e42292)), (locals.var_t8_dn10 / (2.0 * assign32020_e42292)), (locals.var_t8_dn11 / (2.0 * assign32020_e42292)), (locals.var_t8_dn12 / (2.0 * assign32020_e42292)), (locals.var_t8_dn13 / (2.0 * assign32020_e42292)), (locals.var_t8_dn14 / (2.0 * assign32020_e42292)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    }
};
        locals.var_sqrtpsip = assign32020_e42294;
        locals.var_sqrtpsip_dn0 = assign32020_e42294_d_n0;
        locals.var_sqrtpsip_dn2 = assign32020_e42294_d_n2;
        locals.var_sqrtpsip_dn3 = assign32020_e42294_d_n3;
        locals.var_sqrtpsip_dn4 = assign32020_e42294_d_n4;
        locals.var_sqrtpsip_dn5 = assign32020_e42294_d_n5;
        locals.var_sqrtpsip_dn6 = assign32020_e42294_d_n6;
        locals.var_sqrtpsip_dn7 = assign32020_e42294_d_n7;
        locals.var_sqrtpsip_dn8 = assign32020_e42294_d_n8;
        locals.var_sqrtpsip_dn9 = assign32020_e42294_d_n9;
        locals.var_sqrtpsip_dn10 = assign32020_e42294_d_n10;
        locals.var_sqrtpsip_dn11 = assign32020_e42294_d_n11;
        locals.var_sqrtpsip_dn12 = assign32020_e42294_d_n12;
        locals.var_sqrtpsip_dn13 = assign32020_e42294_d_n13;
        locals.var_sqrtpsip_dn14 = assign32020_e42294_d_n14;

    }

    pub(super) fn stamp_transient_block_102(
        locals: &mut StampLocals,
    ) {
        let (assign32030_e42306, assign32030_e42306_d_n0, assign32030_e42306_d_n2, assign32030_e42306_d_n3, assign32030_e42306_d_n4, assign32030_e42306_d_n5, assign32030_e42306_d_n6, assign32030_e42306_d_n7, assign32030_e42306_d_n8, assign32030_e42306_d_n9, assign32030_e42306_d_n10, assign32030_e42306_d_n11, assign32030_e42306_d_n12, assign32030_e42306_d_n13, assign32030_e42306_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32030_e42300: f64 = (2.0 * locals.var_sqrtpsip);
        let assign32030_e42301: f64 = (locals.var_gam_edge / assign32030_e42300);
        let assign32030_e42302: f64 = (1.0 + assign32030_e42301);
        let assign32030_e42304: f64 = (assign32030_e42302 / locals.var_gam_edge);
        (assign32030_e42304, ((((((locals.var_gam_edge_dn0 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn0))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn0)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn2 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn2))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn2)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn3 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn3))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn3)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn4 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn4))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn4)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn5 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn5))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn5)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn6 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn6))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn6)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn7 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn7))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn7)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn8 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn8))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn8)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn9 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn9))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn9)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn10 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn10))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn10)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn11 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn11))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn11)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn12 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn12))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn12)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn13 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn13))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn13)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn14 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn14))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn14)) / (locals.var_gam_edge * locals.var_gam_edge)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32030_e42306;
        locals.var_t0_dn0 = assign32030_e42306_d_n0;
        locals.var_t0_dn2 = assign32030_e42306_d_n2;
        locals.var_t0_dn3 = assign32030_e42306_d_n3;
        locals.var_t0_dn4 = assign32030_e42306_d_n4;
        locals.var_t0_dn5 = assign32030_e42306_d_n5;
        locals.var_t0_dn6 = assign32030_e42306_d_n6;
        locals.var_t0_dn7 = assign32030_e42306_d_n7;
        locals.var_t0_dn8 = assign32030_e42306_d_n8;
        locals.var_t0_dn9 = assign32030_e42306_d_n9;
        locals.var_t0_dn10 = assign32030_e42306_d_n10;
        locals.var_t0_dn11 = assign32030_e42306_d_n11;
        locals.var_t0_dn12 = assign32030_e42306_d_n12;
        locals.var_t0_dn13 = assign32030_e42306_d_n13;
        locals.var_t0_dn14 = assign32030_e42306_d_n14;

        let (assign32040_e42316, assign32040_e42316_d_n0, assign32040_e42316_d_n2, assign32040_e42316_d_n3, assign32040_e42316_d_n4, assign32040_e42316_d_n5, assign32040_e42316_d_n6, assign32040_e42316_d_n7, assign32040_e42316_d_n8, assign32040_e42316_d_n9, assign32040_e42316_d_n10, assign32040_e42316_d_n11, assign32040_e42316_d_n12, assign32040_e42316_d_n13, assign32040_e42316_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32040_e42311: f64 = (2.0 * locals.var_phib_n_edge);
        let assign32040_e42312: f64 = (locals.var_psip - assign32040_e42311);
        let assign32040_e42314: f64 = (assign32040_e42312 - locals.var_vs_1);
        (assign32040_e42314, ((locals.var_psip_dn0 - (2.0 * locals.var_phib_n_edge_dn0)) - locals.var_vs_1_dn0), ((locals.var_psip_dn2 - (2.0 * locals.var_phib_n_edge_dn2)) - locals.var_vs_1_dn2), ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_edge_dn3)) - locals.var_vs_1_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_edge_dn4)) - locals.var_vs_1_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_edge_dn5)) - locals.var_vs_1_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_edge_dn6)) - locals.var_vs_1_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_edge_dn7)) - locals.var_vs_1_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_edge_dn8)) - locals.var_vs_1_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_edge_dn9)) - locals.var_vs_1_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_edge_dn10)) - locals.var_vs_1_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_edge_dn11)) - locals.var_vs_1_dn11), ((locals.var_psip_dn12 - (2.0 * locals.var_phib_n_edge_dn12)) - locals.var_vs_1_dn12), ((locals.var_psip_dn13 - (2.0 * locals.var_phib_n_edge_dn13)) - locals.var_vs_1_dn13), ((locals.var_psip_dn14 - (2.0 * locals.var_phib_n_edge_dn14)) - locals.var_vs_1_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32040_e42316;
        locals.var_t1_dn0 = assign32040_e42316_d_n0;
        locals.var_t1_dn2 = assign32040_e42316_d_n2;
        locals.var_t1_dn3 = assign32040_e42316_d_n3;
        locals.var_t1_dn4 = assign32040_e42316_d_n4;
        locals.var_t1_dn5 = assign32040_e42316_d_n5;
        locals.var_t1_dn6 = assign32040_e42316_d_n6;
        locals.var_t1_dn7 = assign32040_e42316_d_n7;
        locals.var_t1_dn8 = assign32040_e42316_d_n8;
        locals.var_t1_dn9 = assign32040_e42316_d_n9;
        locals.var_t1_dn10 = assign32040_e42316_d_n10;
        locals.var_t1_dn11 = assign32040_e42316_d_n11;
        locals.var_t1_dn12 = assign32040_e42316_d_n12;
        locals.var_t1_dn13 = assign32040_e42316_d_n13;
        locals.var_t1_dn14 = assign32040_e42316_d_n14;

        let (assign32050_e42331, assign32050_e42331_d_n0, assign32050_e42331_d_n2, assign32050_e42331_d_n3, assign32050_e42331_d_n4, assign32050_e42331_d_n5, assign32050_e42331_d_n6, assign32050_e42331_d_n7, assign32050_e42331_d_n8, assign32050_e42331_d_n9, assign32050_e42331_d_n10, assign32050_e42331_d_n11, assign32050_e42331_d_n12, assign32050_e42331_d_n13, assign32050_e42331_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32050_e42320: f64 = locals.var_t1;
        let assign32050_e42323: f64 = (4.0 * locals.var_t0);
        let assign32050_e42325: f64 = (assign32050_e42323 * locals.var_sqrtpsip);
        let assign32050_e42327: f64 = (assign32050_e42325).max(1e-38);
        let assign32050_e42328: f64 = (assign32050_e42327).ln();
        let assign32050_e42329: f64 = (assign32050_e42320 - assign32050_e42328);
        (assign32050_e42329, (locals.var_t1_dn0 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn0) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn0)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn2 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn2) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn2)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn3 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn4 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn5 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn6 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn7 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn8 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn9 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn10 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn11 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn12 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn12) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn12)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn13 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn13) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn13)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn14 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn14) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn14)) } else { 0.0 } / assign32050_e42327)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32050_e42331;
        locals.var_t2_dn0 = assign32050_e42331_d_n0;
        locals.var_t2_dn2 = assign32050_e42331_d_n2;
        locals.var_t2_dn3 = assign32050_e42331_d_n3;
        locals.var_t2_dn4 = assign32050_e42331_d_n4;
        locals.var_t2_dn5 = assign32050_e42331_d_n5;
        locals.var_t2_dn6 = assign32050_e42331_d_n6;
        locals.var_t2_dn7 = assign32050_e42331_d_n7;
        locals.var_t2_dn8 = assign32050_e42331_d_n8;
        locals.var_t2_dn9 = assign32050_e42331_d_n9;
        locals.var_t2_dn10 = assign32050_e42331_d_n10;
        locals.var_t2_dn11 = assign32050_e42331_d_n11;
        locals.var_t2_dn12 = assign32050_e42331_d_n12;
        locals.var_t2_dn13 = assign32050_e42331_d_n13;
        locals.var_t2_dn14 = assign32050_e42331_d_n14;

        let (assign32060_e42348, assign32060_e42348_d_n0, assign32060_e42348_d_n2, assign32060_e42348_d_n3, assign32060_e42348_d_n4, assign32060_e42348_d_n5, assign32060_e42348_d_n6, assign32060_e42348_d_n7, assign32060_e42348_d_n8, assign32060_e42348_d_n9, assign32060_e42348_d_n10, assign32060_e42348_d_n11, assign32060_e42348_d_n12, assign32060_e42348_d_n13, assign32060_e42348_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32060_e42336: f64 = (locals.var_t2 - 0.201491);
        let assign32060_e42340: f64 = (locals.var_t2 + 0.402982);
        let assign32060_e42341: f64 = (locals.var_t2 * assign32060_e42340);
        let assign32060_e42343: f64 = (assign32060_e42341 + 2.446562);
        let assign32060_e42344: f64 = (assign32060_e42343).sqrt();
        let assign32060_e42345: f64 = (assign32060_e42336 - assign32060_e42344);
        let assign32060_e42346: f64 = (0.5 * assign32060_e42345);
        (assign32060_e42346, (0.5 * (locals.var_t2_dn0 - (((locals.var_t2_dn0 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn2 - (((locals.var_t2_dn2 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn12 - (((locals.var_t2_dn12 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn13 - (((locals.var_t2_dn13 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn14 - (((locals.var_t2_dn14 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign32060_e42344)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32060_e42348;
        locals.var_t8_dn0 = assign32060_e42348_d_n0;
        locals.var_t8_dn2 = assign32060_e42348_d_n2;
        locals.var_t8_dn3 = assign32060_e42348_d_n3;
        locals.var_t8_dn4 = assign32060_e42348_d_n4;
        locals.var_t8_dn5 = assign32060_e42348_d_n5;
        locals.var_t8_dn6 = assign32060_e42348_d_n6;
        locals.var_t8_dn7 = assign32060_e42348_d_n7;
        locals.var_t8_dn8 = assign32060_e42348_d_n8;
        locals.var_t8_dn9 = assign32060_e42348_d_n9;
        locals.var_t8_dn10 = assign32060_e42348_d_n10;
        locals.var_t8_dn11 = assign32060_e42348_d_n11;
        locals.var_t8_dn12 = assign32060_e42348_d_n12;
        locals.var_t8_dn13 = assign32060_e42348_d_n13;
        locals.var_t8_dn14 = assign32060_e42348_d_n14;

        let (assign32070_e42352, assign32070_e42352_d_n0, assign32070_e42352_d_n2, assign32070_e42352_d_n3, assign32070_e42352_d_n4, assign32070_e42352_d_n5, assign32070_e42352_d_n6, assign32070_e42352_d_n7, assign32070_e42352_d_n8, assign32070_e42352_d_n9, assign32070_e42352_d_n10, assign32070_e42352_d_n11, assign32070_e42352_d_n12, assign32070_e42352_d_n13, assign32070_e42352_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn0, locals.var_sqrtpsisa_dn2, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11, locals.var_sqrtpsisa_dn12, locals.var_sqrtpsisa_dn13, locals.var_sqrtpsisa_dn14,)
    }
};
        locals.var_sqrtpsisa = assign32070_e42352;
        locals.var_sqrtpsisa_dn0 = assign32070_e42352_d_n0;
        locals.var_sqrtpsisa_dn2 = assign32070_e42352_d_n2;
        locals.var_sqrtpsisa_dn3 = assign32070_e42352_d_n3;
        locals.var_sqrtpsisa_dn4 = assign32070_e42352_d_n4;
        locals.var_sqrtpsisa_dn5 = assign32070_e42352_d_n5;
        locals.var_sqrtpsisa_dn6 = assign32070_e42352_d_n6;
        locals.var_sqrtpsisa_dn7 = assign32070_e42352_d_n7;
        locals.var_sqrtpsisa_dn8 = assign32070_e42352_d_n8;
        locals.var_sqrtpsisa_dn9 = assign32070_e42352_d_n9;
        locals.var_sqrtpsisa_dn10 = assign32070_e42352_d_n10;
        locals.var_sqrtpsisa_dn11 = assign32070_e42352_d_n11;
        locals.var_sqrtpsisa_dn12 = assign32070_e42352_d_n12;
        locals.var_sqrtpsisa_dn13 = assign32070_e42352_d_n13;
        locals.var_sqrtpsisa_dn14 = assign32070_e42352_d_n14;

        let assign32080_e42355: f64 = (-68.0);
        let assign32080_e42356: f64 = if locals.var_t8 <= assign32080_e42355 { 1.0 } else { 0.0 };
        locals.var_guard736 = assign32080_e42356;

        let (assign32090_e42363, assign32090_e42363_d_n0, assign32090_e42363_d_n2, assign32090_e42363_d_n3, assign32090_e42363_d_n4, assign32090_e42363_d_n5, assign32090_e42363_d_n6, assign32090_e42363_d_n7, assign32090_e42363_d_n8, assign32090_e42363_d_n9, assign32090_e42363_d_n10, assign32090_e42363_d_n11, assign32090_e42363_d_n12, assign32090_e42363_d_n13, assign32090_e42363_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) {
        let assign32090_e42361: f64 = (-100.0);
        (assign32090_e42361, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32090_e42363;
        locals.var_t4_dn0 = assign32090_e42363_d_n0;
        locals.var_t4_dn2 = assign32090_e42363_d_n2;
        locals.var_t4_dn3 = assign32090_e42363_d_n3;
        locals.var_t4_dn4 = assign32090_e42363_d_n4;
        locals.var_t4_dn5 = assign32090_e42363_d_n5;
        locals.var_t4_dn6 = assign32090_e42363_d_n6;
        locals.var_t4_dn7 = assign32090_e42363_d_n7;
        locals.var_t4_dn8 = assign32090_e42363_d_n8;
        locals.var_t4_dn9 = assign32090_e42363_d_n9;
        locals.var_t4_dn10 = assign32090_e42363_d_n10;
        locals.var_t4_dn11 = assign32090_e42363_d_n11;
        locals.var_t4_dn12 = assign32090_e42363_d_n12;
        locals.var_t4_dn13 = assign32090_e42363_d_n13;
        locals.var_t4_dn14 = assign32090_e42363_d_n14;

        let (assign32100_e42369, assign32100_e42369_d_n0, assign32100_e42369_d_n2, assign32100_e42369_d_n3, assign32100_e42369_d_n4, assign32100_e42369_d_n5, assign32100_e42369_d_n6, assign32100_e42369_d_n7, assign32100_e42369_d_n8, assign32100_e42369_d_n9, assign32100_e42369_d_n10, assign32100_e42369_d_n11, assign32100_e42369_d_n12, assign32100_e42369_d_n13, assign32100_e42369_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32100_e42369;
        locals.var_t5_dn0 = assign32100_e42369_d_n0;
        locals.var_t5_dn2 = assign32100_e42369_d_n2;
        locals.var_t5_dn3 = assign32100_e42369_d_n3;
        locals.var_t5_dn4 = assign32100_e42369_d_n4;
        locals.var_t5_dn5 = assign32100_e42369_d_n5;
        locals.var_t5_dn6 = assign32100_e42369_d_n6;
        locals.var_t5_dn7 = assign32100_e42369_d_n7;
        locals.var_t5_dn8 = assign32100_e42369_d_n8;
        locals.var_t5_dn9 = assign32100_e42369_d_n9;
        locals.var_t5_dn10 = assign32100_e42369_d_n10;
        locals.var_t5_dn11 = assign32100_e42369_d_n11;
        locals.var_t5_dn12 = assign32100_e42369_d_n12;
        locals.var_t5_dn13 = assign32100_e42369_d_n13;
        locals.var_t5_dn14 = assign32100_e42369_d_n14;

        let assign32110_e42374: f64 = (0.5 * locals.var_t5);
        let assign32110_e42375: f64 = (locals.var_t4 - assign32110_e42374);
        let assign32110_e42376: f64 = if locals.var_t8 < assign32110_e42375 { 1.0 } else { 0.0 };
        locals.var_guard737 = assign32110_e42376;

        let (assign32120_e42385, assign32120_e42385_d_n0, assign32120_e42385_d_n2, assign32120_e42385_d_n3, assign32120_e42385_d_n4, assign32120_e42385_d_n5, assign32120_e42385_d_n6, assign32120_e42385_d_n7, assign32120_e42385_d_n8, assign32120_e42385_d_n9, assign32120_e42385_d_n10, assign32120_e42385_d_n11, assign32120_e42385_d_n12, assign32120_e42385_d_n13, assign32120_e42385_d_n14,) = {
    if (((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 != 0.0)) {
        let assign32120_e42383: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32120_e42383, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn0), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn2), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn12), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn13), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32120_e42385;
        locals.var_t3_dn0 = assign32120_e42385_d_n0;
        locals.var_t3_dn2 = assign32120_e42385_d_n2;
        locals.var_t3_dn3 = assign32120_e42385_d_n3;
        locals.var_t3_dn4 = assign32120_e42385_d_n4;
        locals.var_t3_dn5 = assign32120_e42385_d_n5;
        locals.var_t3_dn6 = assign32120_e42385_d_n6;
        locals.var_t3_dn7 = assign32120_e42385_d_n7;
        locals.var_t3_dn8 = assign32120_e42385_d_n8;
        locals.var_t3_dn9 = assign32120_e42385_d_n9;
        locals.var_t3_dn10 = assign32120_e42385_d_n10;
        locals.var_t3_dn11 = assign32120_e42385_d_n11;
        locals.var_t3_dn12 = assign32120_e42385_d_n12;
        locals.var_t3_dn13 = assign32120_e42385_d_n13;
        locals.var_t3_dn14 = assign32120_e42385_d_n14;

        let assign32130_e42390: f64 = (0.5 * locals.var_t5);
        let assign32130_e42391: f64 = (locals.var_t4 + assign32130_e42390);
        let assign32130_e42392: f64 = if locals.var_t8 > assign32130_e42391 { 1.0 } else { 0.0 };
        locals.var_guard738 = assign32130_e42392;

        let (assign32140_e42404, assign32140_e42404_d_n0, assign32140_e42404_d_n2, assign32140_e42404_d_n3, assign32140_e42404_d_n4, assign32140_e42404_d_n5, assign32140_e42404_d_n6, assign32140_e42404_d_n7, assign32140_e42404_d_n8, assign32140_e42404_d_n9, assign32140_e42404_d_n10, assign32140_e42404_d_n11, assign32140_e42404_d_n12, assign32140_e42404_d_n13, assign32140_e42404_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign32140_e42402: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32140_e42402, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32140_e42404;
        locals.var_t3_dn0 = assign32140_e42404_d_n0;
        locals.var_t3_dn2 = assign32140_e42404_d_n2;
        locals.var_t3_dn3 = assign32140_e42404_d_n3;
        locals.var_t3_dn4 = assign32140_e42404_d_n4;
        locals.var_t3_dn5 = assign32140_e42404_d_n5;
        locals.var_t3_dn6 = assign32140_e42404_d_n6;
        locals.var_t3_dn7 = assign32140_e42404_d_n7;
        locals.var_t3_dn8 = assign32140_e42404_d_n8;
        locals.var_t3_dn9 = assign32140_e42404_d_n9;
        locals.var_t3_dn10 = assign32140_e42404_d_n10;
        locals.var_t3_dn11 = assign32140_e42404_d_n11;
        locals.var_t3_dn12 = assign32140_e42404_d_n12;
        locals.var_t3_dn13 = assign32140_e42404_d_n13;
        locals.var_t3_dn14 = assign32140_e42404_d_n14;

        let (assign32150_e42420, assign32150_e42420_d_n0, assign32150_e42420_d_n2, assign32150_e42420_d_n3, assign32150_e42420_d_n4, assign32150_e42420_d_n5, assign32150_e42420_d_n6, assign32150_e42420_d_n7, assign32150_e42420_d_n8, assign32150_e42420_d_n9, assign32150_e42420_d_n10, assign32150_e42420_d_n11, assign32150_e42420_d_n12, assign32150_e42420_d_n13, assign32150_e42420_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        let assign32150_e42416: f64 = (locals.var_t8 - locals.var_t4);
        let assign32150_e42418: f64 = (assign32150_e42416 / locals.var_t5);
        (assign32150_e42418, ((((locals.var_t8_dn0 - locals.var_t4_dn0) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn2 - locals.var_t4_dn2) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn12 - locals.var_t4_dn12) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn13 - locals.var_t4_dn13) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn14 - locals.var_t4_dn14) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32150_e42420;
        locals.var_t2_dn0 = assign32150_e42420_d_n0;
        locals.var_t2_dn2 = assign32150_e42420_d_n2;
        locals.var_t2_dn3 = assign32150_e42420_d_n3;
        locals.var_t2_dn4 = assign32150_e42420_d_n4;
        locals.var_t2_dn5 = assign32150_e42420_d_n5;
        locals.var_t2_dn6 = assign32150_e42420_d_n6;
        locals.var_t2_dn7 = assign32150_e42420_d_n7;
        locals.var_t2_dn8 = assign32150_e42420_d_n8;
        locals.var_t2_dn9 = assign32150_e42420_d_n9;
        locals.var_t2_dn10 = assign32150_e42420_d_n10;
        locals.var_t2_dn11 = assign32150_e42420_d_n11;
        locals.var_t2_dn12 = assign32150_e42420_d_n12;
        locals.var_t2_dn13 = assign32150_e42420_d_n13;
        locals.var_t2_dn14 = assign32150_e42420_d_n14;

        let (assign32160_e42434, assign32160_e42434_d_n0, assign32160_e42434_d_n2, assign32160_e42434_d_n3, assign32160_e42434_d_n4, assign32160_e42434_d_n5, assign32160_e42434_d_n6, assign32160_e42434_d_n7, assign32160_e42434_d_n8, assign32160_e42434_d_n9, assign32160_e42434_d_n10, assign32160_e42434_d_n11, assign32160_e42434_d_n12, assign32160_e42434_d_n13, assign32160_e42434_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        let assign32160_e42432: f64 = (locals.var_t2 * locals.var_t2);
        (assign32160_e42432, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32160_e42434;
        locals.var_t6_dn0 = assign32160_e42434_d_n0;
        locals.var_t6_dn2 = assign32160_e42434_d_n2;
        locals.var_t6_dn3 = assign32160_e42434_d_n3;
        locals.var_t6_dn4 = assign32160_e42434_d_n4;
        locals.var_t6_dn5 = assign32160_e42434_d_n5;
        locals.var_t6_dn6 = assign32160_e42434_d_n6;
        locals.var_t6_dn7 = assign32160_e42434_d_n7;
        locals.var_t6_dn8 = assign32160_e42434_d_n8;
        locals.var_t6_dn9 = assign32160_e42434_d_n9;
        locals.var_t6_dn10 = assign32160_e42434_d_n10;
        locals.var_t6_dn11 = assign32160_e42434_d_n11;
        locals.var_t6_dn12 = assign32160_e42434_d_n12;
        locals.var_t6_dn13 = assign32160_e42434_d_n13;
        locals.var_t6_dn14 = assign32160_e42434_d_n14;

        let (assign32170_e42469, assign32170_e42469_d_n0, assign32170_e42469_d_n2, assign32170_e42469_d_n3, assign32170_e42469_d_n4, assign32170_e42469_d_n5, assign32170_e42469_d_n6, assign32170_e42469_d_n7, assign32170_e42469_d_n8, assign32170_e42469_d_n9, assign32170_e42469_d_n10, assign32170_e42469_d_n11, assign32170_e42469_d_n12, assign32170_e42469_d_n13, assign32170_e42469_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        let assign32170_e42448: f64 = (5.0 / 64.0);
        let assign32170_e42451: f64 = (0.5 * locals.var_t2);
        let assign32170_e42452: f64 = (assign32170_e42448 + assign32170_e42451);
        let assign32170_e42456: f64 = (15.0 / 16.0);
        let assign32170_e42460: f64 = (1.25 - locals.var_t6);
        let assign32170_e42461: f64 = (locals.var_t6 * assign32170_e42460);
        let assign32170_e42462: f64 = (assign32170_e42456 - assign32170_e42461);
        let assign32170_e42463: f64 = (locals.var_t6 * assign32170_e42462);
        let assign32170_e42464: f64 = (assign32170_e42452 + assign32170_e42463);
        let assign32170_e42465: f64 = (locals.var_t5 * assign32170_e42464);
        let assign32170_e42466: f64 = (locals.var_t4 + assign32170_e42465);
        let assign32170_e42467: f64 = { let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32170_e42467, ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn0) + ((locals.var_t6_dn0 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn0 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn0))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn2) + ((locals.var_t6_dn2 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn2 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn2))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn12 + ((locals.var_t5_dn12 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn12) + ((locals.var_t6_dn12 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn12 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn12))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn13 + ((locals.var_t5_dn13 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn13) + ((locals.var_t6_dn13 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn13 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn13))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn14) + ((locals.var_t6_dn14 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn14 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn14))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32170_e42469;
        locals.var_t3_dn0 = assign32170_e42469_d_n0;
        locals.var_t3_dn2 = assign32170_e42469_d_n2;
        locals.var_t3_dn3 = assign32170_e42469_d_n3;
        locals.var_t3_dn4 = assign32170_e42469_d_n4;
        locals.var_t3_dn5 = assign32170_e42469_d_n5;
        locals.var_t3_dn6 = assign32170_e42469_d_n6;
        locals.var_t3_dn7 = assign32170_e42469_d_n7;
        locals.var_t3_dn8 = assign32170_e42469_d_n8;
        locals.var_t3_dn9 = assign32170_e42469_d_n9;
        locals.var_t3_dn10 = assign32170_e42469_d_n10;
        locals.var_t3_dn11 = assign32170_e42469_d_n11;
        locals.var_t3_dn12 = assign32170_e42469_d_n12;
        locals.var_t3_dn13 = assign32170_e42469_d_n13;
        locals.var_t3_dn14 = assign32170_e42469_d_n14;

        let (assign32180_e42502, assign32180_e42502_d_n0, assign32180_e42502_d_n2, assign32180_e42502_d_n3, assign32180_e42502_d_n4, assign32180_e42502_d_n5, assign32180_e42502_d_n6, assign32180_e42502_d_n7, assign32180_e42502_d_n8, assign32180_e42502_d_n9, assign32180_e42502_d_n10, assign32180_e42502_d_n11, assign32180_e42502_d_n12, assign32180_e42502_d_n13, assign32180_e42502_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) {
        let assign32180_e42476: f64 = (1.0 + locals.var_t1);
        let assign32180_e42479: f64 = locals.var_t8;
        let assign32180_e42480: f64 = (assign32180_e42476 - assign32180_e42479);
        let assign32180_e42484: f64 = (2.0 * locals.var_t0);
        let assign32180_e42487: f64 = (locals.var_t3 * 2.0);
        let assign32180_e42489: f64 = (assign32180_e42487 * locals.var_t0);
        let assign32180_e42492: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32180_e42493: f64 = (assign32180_e42489 + assign32180_e42492);
        let assign32180_e42494: f64 = (assign32180_e42484 * assign32180_e42493);
        let assign32180_e42496: f64 = (assign32180_e42494).max(1e-38);
        let assign32180_e42497: f64 = (assign32180_e42496).ln();
        let assign32180_e42498: f64 = assign32180_e42497;
        let assign32180_e42499: f64 = (assign32180_e42480 - assign32180_e42498);
        let assign32180_e42500: f64 = (locals.var_t3 * assign32180_e42499);
        (assign32180_e42500, ((locals.var_t3_dn0 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn0 - locals.var_t8_dn0) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn0) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn2 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn2 - locals.var_t8_dn2) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn2) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn3 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn4 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn5 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn6 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn7 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn8 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn9 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn10 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn11 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn12 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn12 - locals.var_t8_dn12) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn12) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn13 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn13 - locals.var_t8_dn13) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn13) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn14 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn14 - locals.var_t8_dn14) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn14) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32180_e42496)))),)
    } else {
        (locals.var_qs_edge, locals.var_qs_edge_dn0, locals.var_qs_edge_dn2, locals.var_qs_edge_dn3, locals.var_qs_edge_dn4, locals.var_qs_edge_dn5, locals.var_qs_edge_dn6, locals.var_qs_edge_dn7, locals.var_qs_edge_dn8, locals.var_qs_edge_dn9, locals.var_qs_edge_dn10, locals.var_qs_edge_dn11, locals.var_qs_edge_dn12, locals.var_qs_edge_dn13, locals.var_qs_edge_dn14,)
    }
};
        locals.var_qs_edge = assign32180_e42502;
        locals.var_qs_edge_dn0 = assign32180_e42502_d_n0;
        locals.var_qs_edge_dn2 = assign32180_e42502_d_n2;
        locals.var_qs_edge_dn3 = assign32180_e42502_d_n3;
        locals.var_qs_edge_dn4 = assign32180_e42502_d_n4;
        locals.var_qs_edge_dn5 = assign32180_e42502_d_n5;
        locals.var_qs_edge_dn6 = assign32180_e42502_d_n6;
        locals.var_qs_edge_dn7 = assign32180_e42502_d_n7;
        locals.var_qs_edge_dn8 = assign32180_e42502_d_n8;
        locals.var_qs_edge_dn9 = assign32180_e42502_d_n9;
        locals.var_qs_edge_dn10 = assign32180_e42502_d_n10;
        locals.var_qs_edge_dn11 = assign32180_e42502_d_n11;
        locals.var_qs_edge_dn12 = assign32180_e42502_d_n12;
        locals.var_qs_edge_dn13 = assign32180_e42502_d_n13;
        locals.var_qs_edge_dn14 = assign32180_e42502_d_n14;

        let (assign32190_e42510, assign32190_e42510_d_n0, assign32190_e42510_d_n2, assign32190_e42510_d_n3, assign32190_e42510_d_n4, assign32190_e42510_d_n5, assign32190_e42510_d_n6, assign32190_e42510_d_n7, assign32190_e42510_d_n8, assign32190_e42510_d_n9, assign32190_e42510_d_n10, assign32190_e42510_d_n11, assign32190_e42510_d_n12, assign32190_e42510_d_n13, assign32190_e42510_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32190_e42508: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32190_e42508, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32190_e42510;
        locals.var_t3_dn0 = assign32190_e42510_d_n0;
        locals.var_t3_dn2 = assign32190_e42510_d_n2;
        locals.var_t3_dn3 = assign32190_e42510_d_n3;
        locals.var_t3_dn4 = assign32190_e42510_d_n4;
        locals.var_t3_dn5 = assign32190_e42510_d_n5;
        locals.var_t3_dn6 = assign32190_e42510_d_n6;
        locals.var_t3_dn7 = assign32190_e42510_d_n7;
        locals.var_t3_dn8 = assign32190_e42510_d_n8;
        locals.var_t3_dn9 = assign32190_e42510_d_n9;
        locals.var_t3_dn10 = assign32190_e42510_d_n10;
        locals.var_t3_dn11 = assign32190_e42510_d_n11;
        locals.var_t3_dn12 = assign32190_e42510_d_n12;
        locals.var_t3_dn13 = assign32190_e42510_d_n13;
        locals.var_t3_dn14 = assign32190_e42510_d_n14;

        let (assign32200_e42519, assign32200_e42519_d_n0, assign32200_e42519_d_n2, assign32200_e42519_d_n3, assign32200_e42519_d_n4, assign32200_e42519_d_n5, assign32200_e42519_d_n6, assign32200_e42519_d_n7, assign32200_e42519_d_n8, assign32200_e42519_d_n9, assign32200_e42519_d_n10, assign32200_e42519_d_n11, assign32200_e42519_d_n12, assign32200_e42519_d_n13, assign32200_e42519_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32200_e42517: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign32200_e42517, (-(locals.var_sqrtpsisa_dn0 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn2 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn12 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn13 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn14 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn0, locals.var_sqrtpsisainv_dn2, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11, locals.var_sqrtpsisainv_dn12, locals.var_sqrtpsisainv_dn13, locals.var_sqrtpsisainv_dn14,)
    }
};
        locals.var_sqrtpsisainv = assign32200_e42519;
        locals.var_sqrtpsisainv_dn0 = assign32200_e42519_d_n0;
        locals.var_sqrtpsisainv_dn2 = assign32200_e42519_d_n2;
        locals.var_sqrtpsisainv_dn3 = assign32200_e42519_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign32200_e42519_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign32200_e42519_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign32200_e42519_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign32200_e42519_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign32200_e42519_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign32200_e42519_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign32200_e42519_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign32200_e42519_d_n11;
        locals.var_sqrtpsisainv_dn12 = assign32200_e42519_d_n12;
        locals.var_sqrtpsisainv_dn13 = assign32200_e42519_d_n13;
        locals.var_sqrtpsisainv_dn14 = assign32200_e42519_d_n14;

        let (assign32210_e42551, assign32210_e42551_d_n0, assign32210_e42551_d_n2, assign32210_e42551_d_n3, assign32210_e42551_d_n4, assign32210_e42551_d_n5, assign32210_e42551_d_n6, assign32210_e42551_d_n7, assign32210_e42551_d_n8, assign32210_e42551_d_n9, assign32210_e42551_d_n10, assign32210_e42551_d_n11, assign32210_e42551_d_n12, assign32210_e42551_d_n13, assign32210_e42551_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32210_e42526: f64 = (2.0 * locals.var_t3);
        let assign32210_e42530: f64 = (locals.var_t3 * 2.0);
        let assign32210_e42532: f64 = (assign32210_e42530 * locals.var_t0);
        let assign32210_e42535: f64 = (locals.var_t3 * 2.0);
        let assign32210_e42537: f64 = (assign32210_e42535 * locals.var_t0);
        let assign32210_e42540: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32210_e42541: f64 = (assign32210_e42537 + assign32210_e42540);
        let assign32210_e42542: f64 = (assign32210_e42532 * assign32210_e42541);
        let assign32210_e42544: f64 = (assign32210_e42542).max(1e-38);
        let assign32210_e42545: f64 = (assign32210_e42544).ln();
        let assign32210_e42546: f64 = assign32210_e42545;
        let assign32210_e42547: f64 = (assign32210_e42526 + assign32210_e42546);
        let assign32210_e42549: f64 = (assign32210_e42547 - locals.var_t1);
        (assign32210_e42549, (((2.0 * locals.var_t3_dn0) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn0)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn2)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn3)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn4)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn5)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn6)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn7)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn8)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn9)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn10)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn11)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn12)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn13)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn14)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32210_e42551;
        locals.var_t4_dn0 = assign32210_e42551_d_n0;
        locals.var_t4_dn2 = assign32210_e42551_d_n2;
        locals.var_t4_dn3 = assign32210_e42551_d_n3;
        locals.var_t4_dn4 = assign32210_e42551_d_n4;
        locals.var_t4_dn5 = assign32210_e42551_d_n5;
        locals.var_t4_dn6 = assign32210_e42551_d_n6;
        locals.var_t4_dn7 = assign32210_e42551_d_n7;
        locals.var_t4_dn8 = assign32210_e42551_d_n8;
        locals.var_t4_dn9 = assign32210_e42551_d_n9;
        locals.var_t4_dn10 = assign32210_e42551_d_n10;
        locals.var_t4_dn11 = assign32210_e42551_d_n11;
        locals.var_t4_dn12 = assign32210_e42551_d_n12;
        locals.var_t4_dn13 = assign32210_e42551_d_n13;
        locals.var_t4_dn14 = assign32210_e42551_d_n14;

        let (assign32220_e42576, assign32220_e42576_d_n0, assign32220_e42576_d_n2, assign32220_e42576_d_n3, assign32220_e42576_d_n4, assign32220_e42576_d_n5, assign32220_e42576_d_n6, assign32220_e42576_d_n7, assign32220_e42576_d_n8, assign32220_e42576_d_n9, assign32220_e42576_d_n10, assign32220_e42576_d_n11, assign32220_e42576_d_n12, assign32220_e42576_d_n13, assign32220_e42576_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32220_e42559: f64 = 1.0;
        let assign32220_e42561: f64 = (assign32220_e42559 / locals.var_t3);
        let assign32220_e42562: f64 = (2.0 + assign32220_e42561);
        let assign32220_e42566: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32220_e42567: f64 = assign32220_e42566;
        let assign32220_e42570: f64 = (locals.var_t0 * locals.var_t3);
        let assign32220_e42572: f64 = (assign32220_e42570 + locals.var_sqrtpsisa);
        let assign32220_e42573: f64 = (assign32220_e42567 / assign32220_e42572);
        let assign32220_e42574: f64 = (assign32220_e42562 + assign32220_e42573);
        (assign32220_e42574, ((-((assign32220_e42559 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32220_e42572 * assign32220_e42572))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32220_e42576;
        locals.var_t5_dn0 = assign32220_e42576_d_n0;
        locals.var_t5_dn2 = assign32220_e42576_d_n2;
        locals.var_t5_dn3 = assign32220_e42576_d_n3;
        locals.var_t5_dn4 = assign32220_e42576_d_n4;
        locals.var_t5_dn5 = assign32220_e42576_d_n5;
        locals.var_t5_dn6 = assign32220_e42576_d_n6;
        locals.var_t5_dn7 = assign32220_e42576_d_n7;
        locals.var_t5_dn8 = assign32220_e42576_d_n8;
        locals.var_t5_dn9 = assign32220_e42576_d_n9;
        locals.var_t5_dn10 = assign32220_e42576_d_n10;
        locals.var_t5_dn11 = assign32220_e42576_d_n11;
        locals.var_t5_dn12 = assign32220_e42576_d_n12;
        locals.var_t5_dn13 = assign32220_e42576_d_n13;
        locals.var_t5_dn14 = assign32220_e42576_d_n14;

        let (assign32230_e42587, assign32230_e42587_d_n0, assign32230_e42587_d_n2, assign32230_e42587_d_n3, assign32230_e42587_d_n4, assign32230_e42587_d_n5, assign32230_e42587_d_n6, assign32230_e42587_d_n7, assign32230_e42587_d_n8, assign32230_e42587_d_n9, assign32230_e42587_d_n10, assign32230_e42587_d_n11, assign32230_e42587_d_n12, assign32230_e42587_d_n13, assign32230_e42587_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32230_e42584: f64 = (locals.var_t4 / locals.var_t5);
        let assign32230_e42585: f64 = (locals.var_t3 - assign32230_e42584);
        (assign32230_e42585, (locals.var_t3_dn0 - (((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn2 - (((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn12 - (((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn13 - (((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn14 - (((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32230_e42587;
        locals.var_t3_dn0 = assign32230_e42587_d_n0;
        locals.var_t3_dn2 = assign32230_e42587_d_n2;
        locals.var_t3_dn3 = assign32230_e42587_d_n3;
        locals.var_t3_dn4 = assign32230_e42587_d_n4;
        locals.var_t3_dn5 = assign32230_e42587_d_n5;
        locals.var_t3_dn6 = assign32230_e42587_d_n6;
        locals.var_t3_dn7 = assign32230_e42587_d_n7;
        locals.var_t3_dn8 = assign32230_e42587_d_n8;
        locals.var_t3_dn9 = assign32230_e42587_d_n9;
        locals.var_t3_dn10 = assign32230_e42587_d_n10;
        locals.var_t3_dn11 = assign32230_e42587_d_n11;
        locals.var_t3_dn12 = assign32230_e42587_d_n12;
        locals.var_t3_dn13 = assign32230_e42587_d_n13;
        locals.var_t3_dn14 = assign32230_e42587_d_n14;

    }

    pub(super) fn stamp_transient_block_103(
        locals: &mut StampLocals,
    ) {
        let (assign32240_e42619, assign32240_e42619_d_n0, assign32240_e42619_d_n2, assign32240_e42619_d_n3, assign32240_e42619_d_n4, assign32240_e42619_d_n5, assign32240_e42619_d_n6, assign32240_e42619_d_n7, assign32240_e42619_d_n8, assign32240_e42619_d_n9, assign32240_e42619_d_n10, assign32240_e42619_d_n11, assign32240_e42619_d_n12, assign32240_e42619_d_n13, assign32240_e42619_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32240_e42594: f64 = (2.0 * locals.var_t3);
        let assign32240_e42598: f64 = (locals.var_t3 * 2.0);
        let assign32240_e42600: f64 = (assign32240_e42598 * locals.var_t0);
        let assign32240_e42603: f64 = (locals.var_t3 * 2.0);
        let assign32240_e42605: f64 = (assign32240_e42603 * locals.var_t0);
        let assign32240_e42608: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32240_e42609: f64 = (assign32240_e42605 + assign32240_e42608);
        let assign32240_e42610: f64 = (assign32240_e42600 * assign32240_e42609);
        let assign32240_e42612: f64 = (assign32240_e42610).max(1e-38);
        let assign32240_e42613: f64 = (assign32240_e42612).ln();
        let assign32240_e42614: f64 = assign32240_e42613;
        let assign32240_e42615: f64 = (assign32240_e42594 + assign32240_e42614);
        let assign32240_e42617: f64 = (assign32240_e42615 - locals.var_t1);
        (assign32240_e42617, (((2.0 * locals.var_t3_dn0) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn0)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn2)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn3)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn4)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn5)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn6)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn7)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn8)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn9)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn10)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn11)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn12)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn13)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn14)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32240_e42619;
        locals.var_t4_dn0 = assign32240_e42619_d_n0;
        locals.var_t4_dn2 = assign32240_e42619_d_n2;
        locals.var_t4_dn3 = assign32240_e42619_d_n3;
        locals.var_t4_dn4 = assign32240_e42619_d_n4;
        locals.var_t4_dn5 = assign32240_e42619_d_n5;
        locals.var_t4_dn6 = assign32240_e42619_d_n6;
        locals.var_t4_dn7 = assign32240_e42619_d_n7;
        locals.var_t4_dn8 = assign32240_e42619_d_n8;
        locals.var_t4_dn9 = assign32240_e42619_d_n9;
        locals.var_t4_dn10 = assign32240_e42619_d_n10;
        locals.var_t4_dn11 = assign32240_e42619_d_n11;
        locals.var_t4_dn12 = assign32240_e42619_d_n12;
        locals.var_t4_dn13 = assign32240_e42619_d_n13;
        locals.var_t4_dn14 = assign32240_e42619_d_n14;

        let (assign32250_e42644, assign32250_e42644_d_n0, assign32250_e42644_d_n2, assign32250_e42644_d_n3, assign32250_e42644_d_n4, assign32250_e42644_d_n5, assign32250_e42644_d_n6, assign32250_e42644_d_n7, assign32250_e42644_d_n8, assign32250_e42644_d_n9, assign32250_e42644_d_n10, assign32250_e42644_d_n11, assign32250_e42644_d_n12, assign32250_e42644_d_n13, assign32250_e42644_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32250_e42627: f64 = 1.0;
        let assign32250_e42629: f64 = (assign32250_e42627 / locals.var_t3);
        let assign32250_e42630: f64 = (2.0 + assign32250_e42629);
        let assign32250_e42634: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32250_e42635: f64 = assign32250_e42634;
        let assign32250_e42638: f64 = (locals.var_t0 * locals.var_t3);
        let assign32250_e42640: f64 = (assign32250_e42638 + locals.var_sqrtpsisa);
        let assign32250_e42641: f64 = (assign32250_e42635 / assign32250_e42640);
        let assign32250_e42642: f64 = (assign32250_e42630 + assign32250_e42641);
        (assign32250_e42642, ((-((assign32250_e42627 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32250_e42640 * assign32250_e42640))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32250_e42644;
        locals.var_t5_dn0 = assign32250_e42644_d_n0;
        locals.var_t5_dn2 = assign32250_e42644_d_n2;
        locals.var_t5_dn3 = assign32250_e42644_d_n3;
        locals.var_t5_dn4 = assign32250_e42644_d_n4;
        locals.var_t5_dn5 = assign32250_e42644_d_n5;
        locals.var_t5_dn6 = assign32250_e42644_d_n6;
        locals.var_t5_dn7 = assign32250_e42644_d_n7;
        locals.var_t5_dn8 = assign32250_e42644_d_n8;
        locals.var_t5_dn9 = assign32250_e42644_d_n9;
        locals.var_t5_dn10 = assign32250_e42644_d_n10;
        locals.var_t5_dn11 = assign32250_e42644_d_n11;
        locals.var_t5_dn12 = assign32250_e42644_d_n12;
        locals.var_t5_dn13 = assign32250_e42644_d_n13;
        locals.var_t5_dn14 = assign32250_e42644_d_n14;

        let (assign32260_e42671, assign32260_e42671_d_n0, assign32260_e42671_d_n2, assign32260_e42671_d_n3, assign32260_e42671_d_n4, assign32260_e42671_d_n5, assign32260_e42671_d_n6, assign32260_e42671_d_n7, assign32260_e42671_d_n8, assign32260_e42671_d_n9, assign32260_e42671_d_n10, assign32260_e42671_d_n11, assign32260_e42671_d_n12, assign32260_e42671_d_n13, assign32260_e42671_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32260_e42652: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32260_e42655: f64 = (locals.var_t0 * locals.var_t3);
        let assign32260_e42657: f64 = (assign32260_e42655 + locals.var_sqrtpsisa);
        let assign32260_e42658: f64 = (assign32260_e42652 / assign32260_e42657);
        let assign32260_e42659: f64 = assign32260_e42658;
        let assign32260_e42662: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32260_e42665: f64 = (locals.var_t0 * locals.var_t3);
        let assign32260_e42667: f64 = (assign32260_e42665 + locals.var_sqrtpsisa);
        let assign32260_e42668: f64 = (assign32260_e42662 / assign32260_e42667);
        let assign32260_e42669: f64 = (assign32260_e42659 * assign32260_e42668);
        (assign32260_e42669, ((((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32260_e42667 * assign32260_e42667)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32260_e42671;
        locals.var_t6_dn0 = assign32260_e42671_d_n0;
        locals.var_t6_dn2 = assign32260_e42671_d_n2;
        locals.var_t6_dn3 = assign32260_e42671_d_n3;
        locals.var_t6_dn4 = assign32260_e42671_d_n4;
        locals.var_t6_dn5 = assign32260_e42671_d_n5;
        locals.var_t6_dn6 = assign32260_e42671_d_n6;
        locals.var_t6_dn7 = assign32260_e42671_d_n7;
        locals.var_t6_dn8 = assign32260_e42671_d_n8;
        locals.var_t6_dn9 = assign32260_e42671_d_n9;
        locals.var_t6_dn10 = assign32260_e42671_d_n10;
        locals.var_t6_dn11 = assign32260_e42671_d_n11;
        locals.var_t6_dn12 = assign32260_e42671_d_n12;
        locals.var_t6_dn13 = assign32260_e42671_d_n13;
        locals.var_t6_dn14 = assign32260_e42671_d_n14;

        let (assign32270_e42705, assign32270_e42705_d_n0, assign32270_e42705_d_n2, assign32270_e42705_d_n3, assign32270_e42705_d_n4, assign32270_e42705_d_n5, assign32270_e42705_d_n6, assign32270_e42705_d_n7, assign32270_e42705_d_n8, assign32270_e42705_d_n9, assign32270_e42705_d_n10, assign32270_e42705_d_n11, assign32270_e42705_d_n12, assign32270_e42705_d_n13, assign32270_e42705_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32270_e42677: f64 = (-1.0);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign32270_e42680: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32270_e42683: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32270_e42684: f64 = (assign32270_e42680 * assign32270_e42683);
        let assign32270_e42685: f64 = (assign32270_e42677 * assign32270_e42684);
        let assign32270_e42688: f64 = 1.0;
        let assign32270_e42691: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign32270_e42693: f64 = (assign32270_e42691 * locals.var_sqrtpsisa);
        let assign32270_e42696: f64 = (locals.var_t0 * locals.var_t3);
        let assign32270_e42698: f64 = (assign32270_e42696 + locals.var_sqrtpsisa);
        let assign32270_e42699: f64 = (assign32270_e42693 * assign32270_e42698);
        let assign32270_e42700: f64 = (assign32270_e42688 / assign32270_e42699);
        let assign32270_e42701: f64 = (assign32270_e42685 - assign32270_e42700);
        let assign32270_e42703: f64 = (assign32270_e42701 - locals.var_t6);
        (assign32270_e42703, (((assign32270_e42677 * (((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn0 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn0)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn0)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn0), (((assign32270_e42677 * (((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn2 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn2)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn2)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn2), (((assign32270_e42677 * (((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn3)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn3), (((assign32270_e42677 * (((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn4)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn4), (((assign32270_e42677 * (((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn5)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn5), (((assign32270_e42677 * (((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn6)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn6), (((assign32270_e42677 * (((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn7)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn7), (((assign32270_e42677 * (((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn8)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn8), (((assign32270_e42677 * (((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn9)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn9), (((assign32270_e42677 * (((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn10)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn10), (((assign32270_e42677 * (((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn11)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn11), (((assign32270_e42677 * (((-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn12 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn12)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn12)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn12), (((assign32270_e42677 * (((-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn13 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn13)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn13)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn13), (((assign32270_e42677 * (((-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn14 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn14)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn14)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign32270_e42705;
        locals.var_t7_dn0 = assign32270_e42705_d_n0;
        locals.var_t7_dn2 = assign32270_e42705_d_n2;
        locals.var_t7_dn3 = assign32270_e42705_d_n3;
        locals.var_t7_dn4 = assign32270_e42705_d_n4;
        locals.var_t7_dn5 = assign32270_e42705_d_n5;
        locals.var_t7_dn6 = assign32270_e42705_d_n6;
        locals.var_t7_dn7 = assign32270_e42705_d_n7;
        locals.var_t7_dn8 = assign32270_e42705_d_n8;
        locals.var_t7_dn9 = assign32270_e42705_d_n9;
        locals.var_t7_dn10 = assign32270_e42705_d_n10;
        locals.var_t7_dn11 = assign32270_e42705_d_n11;
        locals.var_t7_dn12 = assign32270_e42705_d_n12;
        locals.var_t7_dn13 = assign32270_e42705_d_n13;
        locals.var_t7_dn14 = assign32270_e42705_d_n14;

        let (assign32280_e42728, assign32280_e42728_d_n0, assign32280_e42728_d_n2, assign32280_e42728_d_n3, assign32280_e42728_d_n4, assign32280_e42728_d_n5, assign32280_e42728_d_n6, assign32280_e42728_d_n7, assign32280_e42728_d_n8, assign32280_e42728_d_n9, assign32280_e42728_d_n10, assign32280_e42728_d_n11, assign32280_e42728_d_n12, assign32280_e42728_d_n13, assign32280_e42728_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32280_e42713: f64 = (locals.var_t4 / locals.var_t5);
        let assign32280_e42717: f64 = (locals.var_t4 * locals.var_t7);
        let assign32280_e42720: f64 = (2.0 * locals.var_t5);
        let assign32280_e42722: f64 = (assign32280_e42720 * locals.var_t5);
        let assign32280_e42723: f64 = (assign32280_e42717 / assign32280_e42722);
        let assign32280_e42724: f64 = (1.0 + assign32280_e42723);
        let assign32280_e42725: f64 = (assign32280_e42713 * assign32280_e42724);
        let assign32280_e42726: f64 = (locals.var_t3 - assign32280_e42725);
        (assign32280_e42726, (locals.var_t3_dn0 - (((((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn0 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn0)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn0) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn0)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn2 - (((((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn2 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn2)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn2) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn2)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn3)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn4)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn5)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn6)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn7)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn8)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn9)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn10)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn11)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn12 - (((((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn12 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn12)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn12) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn12)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn13 - (((((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn13 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn13)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn13) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn13)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn14 - (((((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn14 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn14)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn14) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn14)))) / (assign32280_e42722 * assign32280_e42722))))),)
    } else {
        (locals.var_qs_edge, locals.var_qs_edge_dn0, locals.var_qs_edge_dn2, locals.var_qs_edge_dn3, locals.var_qs_edge_dn4, locals.var_qs_edge_dn5, locals.var_qs_edge_dn6, locals.var_qs_edge_dn7, locals.var_qs_edge_dn8, locals.var_qs_edge_dn9, locals.var_qs_edge_dn10, locals.var_qs_edge_dn11, locals.var_qs_edge_dn12, locals.var_qs_edge_dn13, locals.var_qs_edge_dn14,)
    }
};
        locals.var_qs_edge = assign32280_e42728;
        locals.var_qs_edge_dn0 = assign32280_e42728_d_n0;
        locals.var_qs_edge_dn2 = assign32280_e42728_d_n2;
        locals.var_qs_edge_dn3 = assign32280_e42728_d_n3;
        locals.var_qs_edge_dn4 = assign32280_e42728_d_n4;
        locals.var_qs_edge_dn5 = assign32280_e42728_d_n5;
        locals.var_qs_edge_dn6 = assign32280_e42728_d_n6;
        locals.var_qs_edge_dn7 = assign32280_e42728_d_n7;
        locals.var_qs_edge_dn8 = assign32280_e42728_d_n8;
        locals.var_qs_edge_dn9 = assign32280_e42728_d_n9;
        locals.var_qs_edge_dn10 = assign32280_e42728_d_n10;
        locals.var_qs_edge_dn11 = assign32280_e42728_d_n11;
        locals.var_qs_edge_dn12 = assign32280_e42728_d_n12;
        locals.var_qs_edge_dn13 = assign32280_e42728_d_n13;
        locals.var_qs_edge_dn14 = assign32280_e42728_d_n14;

        let (assign32290_e42740, assign32290_e42740_d_n0, assign32290_e42740_d_n2, assign32290_e42740_d_n3, assign32290_e42740_d_n4, assign32290_e42740_d_n5, assign32290_e42740_d_n6, assign32290_e42740_d_n7, assign32290_e42740_d_n8, assign32290_e42740_d_n9, assign32290_e42740_d_n10, assign32290_e42740_d_n11, assign32290_e42740_d_n12, assign32290_e42740_d_n13, assign32290_e42740_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32290_e42732: f64 = (2.0 * locals.var_nvt);
        let assign32290_e42734: f64 = (assign32290_e42732 * locals.var_qs_edge);
        let assign32290_e42737: f64 = (2.0 * locals.var_nvt);
        let assign32290_e42738: f64 = (assign32290_e42734 + assign32290_e42737);
        (assign32290_e42738, ((((2.0 * locals.var_nvt_dn0) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn0)) + (2.0 * locals.var_nvt_dn0)), ((((2.0 * locals.var_nvt_dn2) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn2)) + (2.0 * locals.var_nvt_dn2)), ((((2.0 * locals.var_nvt_dn3) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn3)) + (2.0 * locals.var_nvt_dn3)), ((((2.0 * locals.var_nvt_dn4) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn4)) + (2.0 * locals.var_nvt_dn4)), ((((2.0 * locals.var_nvt_dn5) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn5)) + (2.0 * locals.var_nvt_dn5)), ((((2.0 * locals.var_nvt_dn6) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn6)) + (2.0 * locals.var_nvt_dn6)), ((((2.0 * locals.var_nvt_dn7) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn7)) + (2.0 * locals.var_nvt_dn7)), ((((2.0 * locals.var_nvt_dn8) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn8)) + (2.0 * locals.var_nvt_dn8)), ((((2.0 * locals.var_nvt_dn9) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn9)) + (2.0 * locals.var_nvt_dn9)), ((((2.0 * locals.var_nvt_dn10) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn10)) + (2.0 * locals.var_nvt_dn10)), ((((2.0 * locals.var_nvt_dn11) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn11)) + (2.0 * locals.var_nvt_dn11)), ((((2.0 * locals.var_nvt_dn12) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn12)) + (2.0 * locals.var_nvt_dn12)), ((((2.0 * locals.var_nvt_dn13) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn13)) + (2.0 * locals.var_nvt_dn13)), ((((2.0 * locals.var_nvt_dn14) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn14)) + (2.0 * locals.var_nvt_dn14)),)
    } else {
        (locals.var_vdsatedge, locals.var_vdsatedge_dn0, locals.var_vdsatedge_dn2, locals.var_vdsatedge_dn3, locals.var_vdsatedge_dn4, locals.var_vdsatedge_dn5, locals.var_vdsatedge_dn6, locals.var_vdsatedge_dn7, locals.var_vdsatedge_dn8, locals.var_vdsatedge_dn9, locals.var_vdsatedge_dn10, locals.var_vdsatedge_dn11, locals.var_vdsatedge_dn12, locals.var_vdsatedge_dn13, locals.var_vdsatedge_dn14,)
    }
};
        locals.var_vdsatedge = assign32290_e42740;
        locals.var_vdsatedge_dn0 = assign32290_e42740_d_n0;
        locals.var_vdsatedge_dn2 = assign32290_e42740_d_n2;
        locals.var_vdsatedge_dn3 = assign32290_e42740_d_n3;
        locals.var_vdsatedge_dn4 = assign32290_e42740_d_n4;
        locals.var_vdsatedge_dn5 = assign32290_e42740_d_n5;
        locals.var_vdsatedge_dn6 = assign32290_e42740_d_n6;
        locals.var_vdsatedge_dn7 = assign32290_e42740_d_n7;
        locals.var_vdsatedge_dn8 = assign32290_e42740_d_n8;
        locals.var_vdsatedge_dn9 = assign32290_e42740_d_n9;
        locals.var_vdsatedge_dn10 = assign32290_e42740_d_n10;
        locals.var_vdsatedge_dn11 = assign32290_e42740_d_n11;
        locals.var_vdsatedge_dn12 = assign32290_e42740_d_n12;
        locals.var_vdsatedge_dn13 = assign32290_e42740_d_n13;
        locals.var_vdsatedge_dn14 = assign32290_e42740_d_n14;

        let (assign32300_e42744, assign32300_e42744_d_n0, assign32300_e42744_d_n2, assign32300_e42744_d_n3, assign32300_e42744_d_n4, assign32300_e42744_d_n5, assign32300_e42744_d_n6, assign32300_e42744_d_n7, assign32300_e42744_d_n8, assign32300_e42744_d_n9, assign32300_e42744_d_n10, assign32300_e42744_d_n11, assign32300_e42744_d_n12, assign32300_e42744_d_n13, assign32300_e42744_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        (locals.var_vdsatedge, locals.var_vdsatedge_dn0, locals.var_vdsatedge_dn2, locals.var_vdsatedge_dn3, locals.var_vdsatedge_dn4, locals.var_vdsatedge_dn5, locals.var_vdsatedge_dn6, locals.var_vdsatedge_dn7, locals.var_vdsatedge_dn8, locals.var_vdsatedge_dn9, locals.var_vdsatedge_dn10, locals.var_vdsatedge_dn11, locals.var_vdsatedge_dn12, locals.var_vdsatedge_dn13, locals.var_vdsatedge_dn14,)
    } else {
        (locals.var_vdsatedge_1, locals.var_vdsatedge_1_dn0, locals.var_vdsatedge_1_dn2, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, locals.var_vdsatedge_1_dn6, locals.var_vdsatedge_1_dn7, locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, locals.var_vdsatedge_1_dn11, locals.var_vdsatedge_1_dn12, locals.var_vdsatedge_1_dn13, locals.var_vdsatedge_1_dn14,)
    }
};
        locals.var_vdsatedge_1 = assign32300_e42744;
        locals.var_vdsatedge_1_dn0 = assign32300_e42744_d_n0;
        locals.var_vdsatedge_1_dn2 = assign32300_e42744_d_n2;
        locals.var_vdsatedge_1_dn3 = assign32300_e42744_d_n3;
        locals.var_vdsatedge_1_dn4 = assign32300_e42744_d_n4;
        locals.var_vdsatedge_1_dn5 = assign32300_e42744_d_n5;
        locals.var_vdsatedge_1_dn6 = assign32300_e42744_d_n6;
        locals.var_vdsatedge_1_dn7 = assign32300_e42744_d_n7;
        locals.var_vdsatedge_1_dn8 = assign32300_e42744_d_n8;
        locals.var_vdsatedge_1_dn9 = assign32300_e42744_d_n9;
        locals.var_vdsatedge_1_dn10 = assign32300_e42744_d_n10;
        locals.var_vdsatedge_1_dn11 = assign32300_e42744_d_n11;
        locals.var_vdsatedge_1_dn12 = assign32300_e42744_d_n12;
        locals.var_vdsatedge_1_dn13 = assign32300_e42744_d_n13;
        locals.var_vdsatedge_1_dn14 = assign32300_e42744_d_n14;

        let (assign32310_e42750, assign32310_e42750_d_n0, assign32310_e42750_d_n2, assign32310_e42750_d_n3, assign32310_e42750_d_n4, assign32310_e42750_d_n5, assign32310_e42750_d_n6, assign32310_e42750_d_n7, assign32310_e42750_d_n8, assign32310_e42750_d_n9, assign32310_e42750_d_n10, assign32310_e42750_d_n11, assign32310_e42750_d_n12, assign32310_e42750_d_n13, assign32310_e42750_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32310_e42748: f64 = (locals.var_vdsatedge_1 + locals.var_vs);
        (assign32310_e42748, locals.var_vdsatedge_1_dn0, locals.var_vdsatedge_1_dn2, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, (locals.var_vdsatedge_1_dn5 + locals.var_vs_dn5), locals.var_vdsatedge_1_dn6, (locals.var_vdsatedge_1_dn7 + locals.var_vs_dn7), locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, (locals.var_vdsatedge_1_dn11 + locals.var_vs_dn11), locals.var_vdsatedge_1_dn12, locals.var_vdsatedge_1_dn13, locals.var_vdsatedge_1_dn14,)
    } else {
        (locals.var_vdsatedge_1, locals.var_vdsatedge_1_dn0, locals.var_vdsatedge_1_dn2, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, locals.var_vdsatedge_1_dn6, locals.var_vdsatedge_1_dn7, locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, locals.var_vdsatedge_1_dn11, locals.var_vdsatedge_1_dn12, locals.var_vdsatedge_1_dn13, locals.var_vdsatedge_1_dn14,)
    }
};
        locals.var_vdsatedge_1 = assign32310_e42750;
        locals.var_vdsatedge_1_dn0 = assign32310_e42750_d_n0;
        locals.var_vdsatedge_1_dn2 = assign32310_e42750_d_n2;
        locals.var_vdsatedge_1_dn3 = assign32310_e42750_d_n3;
        locals.var_vdsatedge_1_dn4 = assign32310_e42750_d_n4;
        locals.var_vdsatedge_1_dn5 = assign32310_e42750_d_n5;
        locals.var_vdsatedge_1_dn6 = assign32310_e42750_d_n6;
        locals.var_vdsatedge_1_dn7 = assign32310_e42750_d_n7;
        locals.var_vdsatedge_1_dn8 = assign32310_e42750_d_n8;
        locals.var_vdsatedge_1_dn9 = assign32310_e42750_d_n9;
        locals.var_vdsatedge_1_dn10 = assign32310_e42750_d_n10;
        locals.var_vdsatedge_1_dn11 = assign32310_e42750_d_n11;
        locals.var_vdsatedge_1_dn12 = assign32310_e42750_d_n12;
        locals.var_vdsatedge_1_dn13 = assign32310_e42750_d_n13;
        locals.var_vdsatedge_1_dn14 = assign32310_e42750_d_n14;

        let assign32320_e42756: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32320_e42758: f64 = (-2500.0);
        let assign32320_e42760: f64 = (assign32320_e42758 * 0.001);
        let assign32320_e42762: f64 = if ((0.0 == 0.0) && (assign32320_e42756 < assign32320_e42760)) { 1.0 } else { 0.0 };
        locals.var_guard739 = assign32320_e42762;

        let (assign32330_e42777, assign32330_e42777_d_n0, assign32330_e42777_d_n2, assign32330_e42777_d_n3, assign32330_e42777_d_n4, assign32330_e42777_d_n5, assign32330_e42777_d_n6, assign32330_e42777_d_n7, assign32330_e42777_d_n8, assign32330_e42777_d_n9, assign32330_e42777_d_n10, assign32330_e42777_d_n11, assign32330_e42777_d_n12, assign32330_e42777_d_n13, assign32330_e42777_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign32330_e42767: f64 = (-0.001);
        let assign32330_e42769: f64 = (assign32330_e42767 * 0.001);
        let assign32330_e42773: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32330_e42774: f64 = (16.0 * assign32330_e42773);
        let assign32330_e42775: f64 = (assign32330_e42769 / assign32330_e42774);
        (assign32330_e42775, (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn0)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn2)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn3)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn4)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * (locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5))) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn6)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * (locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7))) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn8)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn9)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn10)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * (locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11))) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn12)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn13)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn14)) / (assign32330_e42774 * assign32330_e42774))),)
    } else {
        (locals.var_vdssate, locals.var_vdssate_dn0, locals.var_vdssate_dn2, locals.var_vdssate_dn3, locals.var_vdssate_dn4, locals.var_vdssate_dn5, locals.var_vdssate_dn6, locals.var_vdssate_dn7, locals.var_vdssate_dn8, locals.var_vdssate_dn9, locals.var_vdssate_dn10, locals.var_vdssate_dn11, locals.var_vdssate_dn12, locals.var_vdssate_dn13, locals.var_vdssate_dn14,)
    }
};
        locals.var_vdssate = assign32330_e42777;
        locals.var_vdssate_dn0 = assign32330_e42777_d_n0;
        locals.var_vdssate_dn2 = assign32330_e42777_d_n2;
        locals.var_vdssate_dn3 = assign32330_e42777_d_n3;
        locals.var_vdssate_dn4 = assign32330_e42777_d_n4;
        locals.var_vdssate_dn5 = assign32330_e42777_d_n5;
        locals.var_vdssate_dn6 = assign32330_e42777_d_n6;
        locals.var_vdssate_dn7 = assign32330_e42777_d_n7;
        locals.var_vdssate_dn8 = assign32330_e42777_d_n8;
        locals.var_vdssate_dn9 = assign32330_e42777_d_n9;
        locals.var_vdssate_dn10 = assign32330_e42777_d_n10;
        locals.var_vdssate_dn11 = assign32330_e42777_d_n11;
        locals.var_vdssate_dn12 = assign32330_e42777_d_n12;
        locals.var_vdssate_dn13 = assign32330_e42777_d_n13;
        locals.var_vdssate_dn14 = assign32330_e42777_d_n14;

        let (assign32340_e42809, assign32340_e42809_d_n0, assign32340_e42809_d_n2, assign32340_e42809_d_n3, assign32340_e42809_d_n4, assign32340_e42809_d_n5, assign32340_e42809_d_n6, assign32340_e42809_d_n7, assign32340_e42809_d_n8, assign32340_e42809_d_n9, assign32340_e42809_d_n10, assign32340_e42809_d_n11, assign32340_e42809_d_n12, assign32340_e42809_d_n13, assign32340_e42809_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard739 == 0.0)) {
        let assign32340_e42785: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32340_e42787: f64 = assign32340_e42785;
        let assign32340_e42790: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32340_e42792: f64 = assign32340_e42790;
        let assign32340_e42795: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32340_e42797: f64 = assign32340_e42795;
        let assign32340_e42798: f64 = (assign32340_e42792 * assign32340_e42797);
        let assign32340_e42801: f64 = (0.25 * 0.001);
        let assign32340_e42803: f64 = (assign32340_e42801 * 0.001);
        let assign32340_e42804: f64 = (assign32340_e42798 + assign32340_e42803);
        let assign32340_e42805: f64 = (assign32340_e42804).sqrt();
        let assign32340_e42806: f64 = (assign32340_e42787 + assign32340_e42805);
        let assign32340_e42807: f64 = (0.5 * assign32340_e42806);
        (assign32340_e42807, (0.5 * (locals.var_vdsatedge_1_dn0 + (((locals.var_vdsatedge_1_dn0 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn0)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn2 + (((locals.var_vdsatedge_1_dn2 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn2)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn3 + (((locals.var_vdsatedge_1_dn3 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn3)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn4 + (((locals.var_vdsatedge_1_dn4 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn4)) / (2.0 * assign32340_e42805)))), (0.5 * ((locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5) + ((((locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5) * assign32340_e42797) + (assign32340_e42792 * (locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5))) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn6 + (((locals.var_vdsatedge_1_dn6 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn6)) / (2.0 * assign32340_e42805)))), (0.5 * ((locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7) + ((((locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7) * assign32340_e42797) + (assign32340_e42792 * (locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7))) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn8 + (((locals.var_vdsatedge_1_dn8 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn8)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn9 + (((locals.var_vdsatedge_1_dn9 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn9)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn10 + (((locals.var_vdsatedge_1_dn10 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn10)) / (2.0 * assign32340_e42805)))), (0.5 * ((locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11) + ((((locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11) * assign32340_e42797) + (assign32340_e42792 * (locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11))) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn12 + (((locals.var_vdsatedge_1_dn12 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn12)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn13 + (((locals.var_vdsatedge_1_dn13 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn13)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn14 + (((locals.var_vdsatedge_1_dn14 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn14)) / (2.0 * assign32340_e42805)))),)
    } else {
        (locals.var_vdssate, locals.var_vdssate_dn0, locals.var_vdssate_dn2, locals.var_vdssate_dn3, locals.var_vdssate_dn4, locals.var_vdssate_dn5, locals.var_vdssate_dn6, locals.var_vdssate_dn7, locals.var_vdssate_dn8, locals.var_vdssate_dn9, locals.var_vdssate_dn10, locals.var_vdssate_dn11, locals.var_vdssate_dn12, locals.var_vdssate_dn13, locals.var_vdssate_dn14,)
    }
};
        locals.var_vdssate = assign32340_e42809;
        locals.var_vdssate_dn0 = assign32340_e42809_d_n0;
        locals.var_vdssate_dn2 = assign32340_e42809_d_n2;
        locals.var_vdssate_dn3 = assign32340_e42809_d_n3;
        locals.var_vdssate_dn4 = assign32340_e42809_d_n4;
        locals.var_vdssate_dn5 = assign32340_e42809_d_n5;
        locals.var_vdssate_dn6 = assign32340_e42809_d_n6;
        locals.var_vdssate_dn7 = assign32340_e42809_d_n7;
        locals.var_vdssate_dn8 = assign32340_e42809_d_n8;
        locals.var_vdssate_dn9 = assign32340_e42809_d_n9;
        locals.var_vdssate_dn10 = assign32340_e42809_d_n10;
        locals.var_vdssate_dn11 = assign32340_e42809_d_n11;
        locals.var_vdssate_dn12 = assign32340_e42809_d_n12;
        locals.var_vdssate_dn13 = assign32340_e42809_d_n13;
        locals.var_vdssate_dn14 = assign32340_e42809_d_n14;

        let (assign32350_e42819, assign32350_e42819_d_n0, assign32350_e42819_d_n2, assign32350_e42819_d_n3, assign32350_e42819_d_n4, assign32350_e42819_d_n5, assign32350_e42819_d_n6, assign32350_e42819_d_n7, assign32350_e42819_d_n8, assign32350_e42819_d_n9, assign32350_e42819_d_n10, assign32350_e42819_d_n11, assign32350_e42819_d_n12, assign32350_e42819_d_n13, assign32350_e42819_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32350_e42813: f64 = (locals.var_vds / locals.var_vdssate);
        let assign32350_e42816: f64 = (1.0 / locals.var_delta_t);
        let assign32350_e42817: f64 = (assign32350_e42813).powf(assign32350_e42816);
        (assign32350_e42817, if (-(locals.var_delta_t_dn0 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn0) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn0 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn0) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn2 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn2) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn2 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn2) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn3) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn3) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn4) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn4) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (((locals.var_vds_dn5 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn5)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((((locals.var_vds_dn5 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn5)) / (locals.var_vdssate * locals.var_vdssate)) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn6) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn6) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (((locals.var_vds_dn7 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn7)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((((locals.var_vds_dn7 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn7)) / (locals.var_vdssate * locals.var_vdssate)) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn8) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn8) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn9) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn9) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn10) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn10) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (((locals.var_vds_dn11 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn11)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((((locals.var_vds_dn11 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn11)) / (locals.var_vdssate * locals.var_vdssate)) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn12 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn12) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn12 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn12) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn13 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn13) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn13 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn13) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn14 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn14) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn14 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn14) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) },)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign32350_e42819;
        locals.var_t7_dn0 = assign32350_e42819_d_n0;
        locals.var_t7_dn2 = assign32350_e42819_d_n2;
        locals.var_t7_dn3 = assign32350_e42819_d_n3;
        locals.var_t7_dn4 = assign32350_e42819_d_n4;
        locals.var_t7_dn5 = assign32350_e42819_d_n5;
        locals.var_t7_dn6 = assign32350_e42819_d_n6;
        locals.var_t7_dn7 = assign32350_e42819_d_n7;
        locals.var_t7_dn8 = assign32350_e42819_d_n8;
        locals.var_t7_dn9 = assign32350_e42819_d_n9;
        locals.var_t7_dn10 = assign32350_e42819_d_n10;
        locals.var_t7_dn11 = assign32350_e42819_d_n11;
        locals.var_t7_dn12 = assign32350_e42819_d_n12;
        locals.var_t7_dn13 = assign32350_e42819_d_n13;
        locals.var_t7_dn14 = assign32350_e42819_d_n14;

        let (assign32360_e42828, assign32360_e42828_d_n0, assign32360_e42828_d_n2, assign32360_e42828_d_n3, assign32360_e42828_d_n4, assign32360_e42828_d_n5, assign32360_e42828_d_n6, assign32360_e42828_d_n7, assign32360_e42828_d_n8, assign32360_e42828_d_n9, assign32360_e42828_d_n10, assign32360_e42828_d_n11, assign32360_e42828_d_n12, assign32360_e42828_d_n13, assign32360_e42828_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32360_e42823: f64 = (1.0 + locals.var_t7);
        let assign32360_e42825: f64 = (-locals.var_delta_t);
        let assign32360_e42826: f64 = (assign32360_e42823).powf(assign32360_e42825);
        (assign32360_e42826, if (-locals.var_delta_t_dn0) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn0)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn0) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn0 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn2) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn2)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn2) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn2 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn3) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn3)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn3) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn3 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn4) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn4)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn4) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn4 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn5) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn5)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn5) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn5 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn6) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn6)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn6) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn6 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn7) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn7)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn7) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn7 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn8) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn8)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn8) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn8 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn9) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn9)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn9) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn9 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn10) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn10)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn10) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn10 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn11) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn11)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn11) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn11 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn12) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn12)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn12) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn12 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn13) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn13)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn13) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn13 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn14) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn14)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn14) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn14 / assign32360_e42823)))) },)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32360_e42828;
        locals.var_t8_dn0 = assign32360_e42828_d_n0;
        locals.var_t8_dn2 = assign32360_e42828_d_n2;
        locals.var_t8_dn3 = assign32360_e42828_d_n3;
        locals.var_t8_dn4 = assign32360_e42828_d_n4;
        locals.var_t8_dn5 = assign32360_e42828_d_n5;
        locals.var_t8_dn6 = assign32360_e42828_d_n6;
        locals.var_t8_dn7 = assign32360_e42828_d_n7;
        locals.var_t8_dn8 = assign32360_e42828_d_n8;
        locals.var_t8_dn9 = assign32360_e42828_d_n9;
        locals.var_t8_dn10 = assign32360_e42828_d_n10;
        locals.var_t8_dn11 = assign32360_e42828_d_n11;
        locals.var_t8_dn12 = assign32360_e42828_d_n12;
        locals.var_t8_dn13 = assign32360_e42828_d_n13;
        locals.var_t8_dn14 = assign32360_e42828_d_n14;

        let (assign32370_e42834, assign32370_e42834_d_n0, assign32370_e42834_d_n2, assign32370_e42834_d_n3, assign32370_e42834_d_n4, assign32370_e42834_d_n5, assign32370_e42834_d_n6, assign32370_e42834_d_n7, assign32370_e42834_d_n8, assign32370_e42834_d_n9, assign32370_e42834_d_n10, assign32370_e42834_d_n11, assign32370_e42834_d_n12, assign32370_e42834_d_n13, assign32370_e42834_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32370_e42832: f64 = (locals.var_vds * locals.var_t8);
        (assign32370_e42832, (locals.var_vds * locals.var_t8_dn0), (locals.var_vds * locals.var_t8_dn2), (locals.var_vds * locals.var_t8_dn3), (locals.var_vds * locals.var_t8_dn4), ((locals.var_vds_dn5 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn5)), (locals.var_vds * locals.var_t8_dn6), ((locals.var_vds_dn7 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn7)), (locals.var_vds * locals.var_t8_dn8), (locals.var_vds * locals.var_t8_dn9), (locals.var_vds * locals.var_t8_dn10), ((locals.var_vds_dn11 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn11)), (locals.var_vds * locals.var_t8_dn12), (locals.var_vds * locals.var_t8_dn13), (locals.var_vds * locals.var_t8_dn14),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn3, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn13, locals.var_vdseff_dn14,)
    }
};
        locals.var_vdseff = assign32370_e42834;
        locals.var_vdseff_dn0 = assign32370_e42834_d_n0;
        locals.var_vdseff_dn2 = assign32370_e42834_d_n2;
        locals.var_vdseff_dn3 = assign32370_e42834_d_n3;
        locals.var_vdseff_dn4 = assign32370_e42834_d_n4;
        locals.var_vdseff_dn5 = assign32370_e42834_d_n5;
        locals.var_vdseff_dn6 = assign32370_e42834_d_n6;
        locals.var_vdseff_dn7 = assign32370_e42834_d_n7;
        locals.var_vdseff_dn8 = assign32370_e42834_d_n8;
        locals.var_vdseff_dn9 = assign32370_e42834_d_n9;
        locals.var_vdseff_dn10 = assign32370_e42834_d_n10;
        locals.var_vdseff_dn11 = assign32370_e42834_d_n11;
        locals.var_vdseff_dn12 = assign32370_e42834_d_n12;
        locals.var_vdseff_dn13 = assign32370_e42834_d_n13;
        locals.var_vdseff_dn14 = assign32370_e42834_d_n14;

        let (assign32380_e42842, assign32380_e42842_d_n0, assign32380_e42842_d_n2, assign32380_e42842_d_n3, assign32380_e42842_d_n4, assign32380_e42842_d_n5, assign32380_e42842_d_n6, assign32380_e42842_d_n7, assign32380_e42842_d_n8, assign32380_e42842_d_n9, assign32380_e42842_d_n10, assign32380_e42842_d_n11, assign32380_e42842_d_n12, assign32380_e42842_d_n13, assign32380_e42842_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32380_e42838: f64 = (locals.var_vdseff + locals.var_vs);
        let assign32380_e42840: f64 = (assign32380_e42838 * locals.var_inv_nvt);
        (assign32380_e42840, ((locals.var_vdseff_dn0 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn0)), ((locals.var_vdseff_dn2 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn2)), ((locals.var_vdseff_dn3 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn3)), ((locals.var_vdseff_dn4 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn4)), (((locals.var_vdseff_dn5 + locals.var_vs_dn5) * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn5)), ((locals.var_vdseff_dn6 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn6)), (((locals.var_vdseff_dn7 + locals.var_vs_dn7) * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn7)), ((locals.var_vdseff_dn8 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn8)), ((locals.var_vdseff_dn9 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn9)), ((locals.var_vdseff_dn10 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn10)), (((locals.var_vdseff_dn11 + locals.var_vs_dn11) * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn11)), ((locals.var_vdseff_dn12 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn12)), ((locals.var_vdseff_dn13 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn13)), ((locals.var_vdseff_dn14 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn14)),)
    } else {
        (locals.var_vdeff, locals.var_vdeff_dn0, locals.var_vdeff_dn2, locals.var_vdeff_dn3, locals.var_vdeff_dn4, locals.var_vdeff_dn5, locals.var_vdeff_dn6, locals.var_vdeff_dn7, locals.var_vdeff_dn8, locals.var_vdeff_dn9, locals.var_vdeff_dn10, locals.var_vdeff_dn11, locals.var_vdeff_dn12, locals.var_vdeff_dn13, locals.var_vdeff_dn14,)
    }
};
        locals.var_vdeff = assign32380_e42842;
        locals.var_vdeff_dn0 = assign32380_e42842_d_n0;
        locals.var_vdeff_dn2 = assign32380_e42842_d_n2;
        locals.var_vdeff_dn3 = assign32380_e42842_d_n3;
        locals.var_vdeff_dn4 = assign32380_e42842_d_n4;
        locals.var_vdeff_dn5 = assign32380_e42842_d_n5;
        locals.var_vdeff_dn6 = assign32380_e42842_d_n6;
        locals.var_vdeff_dn7 = assign32380_e42842_d_n7;
        locals.var_vdeff_dn8 = assign32380_e42842_d_n8;
        locals.var_vdeff_dn9 = assign32380_e42842_d_n9;
        locals.var_vdeff_dn10 = assign32380_e42842_d_n10;
        locals.var_vdeff_dn11 = assign32380_e42842_d_n11;
        locals.var_vdeff_dn12 = assign32380_e42842_d_n12;
        locals.var_vdeff_dn13 = assign32380_e42842_d_n13;
        locals.var_vdeff_dn14 = assign32380_e42842_d_n14;

        let (assign32390_e42865, assign32390_e42865_d_n0, assign32390_e42865_d_n2, assign32390_e42865_d_n3, assign32390_e42865_d_n4, assign32390_e42865_d_n5, assign32390_e42865_d_n6, assign32390_e42865_d_n7, assign32390_e42865_d_n8, assign32390_e42865_d_n9, assign32390_e42865_d_n10, assign32390_e42865_d_n11, assign32390_e42865_d_n12, assign32390_e42865_d_n13, assign32390_e42865_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32390_e42847: f64 = (locals.var_psip + 1.0);
        let assign32390_e42850: f64 = (locals.var_psip - 1.0);
        let assign32390_e42853: f64 = (locals.var_psip - 1.0);
        let assign32390_e42854: f64 = (assign32390_e42850 * assign32390_e42853);
        let assign32390_e42857: f64 = (0.25 * 2.0);
        let assign32390_e42859: f64 = (assign32390_e42857 * 2.0);
        let assign32390_e42860: f64 = (assign32390_e42854 + assign32390_e42859);
        let assign32390_e42861: f64 = (assign32390_e42860).sqrt();
        let assign32390_e42862: f64 = (assign32390_e42847 + assign32390_e42861);
        let assign32390_e42863: f64 = (0.5 * assign32390_e42862);
        (assign32390_e42863, (0.5 * (locals.var_psip_dn0 + (((locals.var_psip_dn0 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn0)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn2 + (((locals.var_psip_dn2 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn2)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn3)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn4)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn5)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn6)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn7)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn8)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn9)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn10)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn11)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn12 + (((locals.var_psip_dn12 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn12)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn13 + (((locals.var_psip_dn13 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn13)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn14 + (((locals.var_psip_dn14 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn14)) / (2.0 * assign32390_e42861)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32390_e42865;
        locals.var_t8_dn0 = assign32390_e42865_d_n0;
        locals.var_t8_dn2 = assign32390_e42865_d_n2;
        locals.var_t8_dn3 = assign32390_e42865_d_n3;
        locals.var_t8_dn4 = assign32390_e42865_d_n4;
        locals.var_t8_dn5 = assign32390_e42865_d_n5;
        locals.var_t8_dn6 = assign32390_e42865_d_n6;
        locals.var_t8_dn7 = assign32390_e42865_d_n7;
        locals.var_t8_dn8 = assign32390_e42865_d_n8;
        locals.var_t8_dn9 = assign32390_e42865_d_n9;
        locals.var_t8_dn10 = assign32390_e42865_d_n10;
        locals.var_t8_dn11 = assign32390_e42865_d_n11;
        locals.var_t8_dn12 = assign32390_e42865_d_n12;
        locals.var_t8_dn13 = assign32390_e42865_d_n13;
        locals.var_t8_dn14 = assign32390_e42865_d_n14;

        let (assign32400_e42870, assign32400_e42870_d_n0, assign32400_e42870_d_n2, assign32400_e42870_d_n3, assign32400_e42870_d_n4, assign32400_e42870_d_n5, assign32400_e42870_d_n6, assign32400_e42870_d_n7, assign32400_e42870_d_n8, assign32400_e42870_d_n9, assign32400_e42870_d_n10, assign32400_e42870_d_n11, assign32400_e42870_d_n12, assign32400_e42870_d_n13, assign32400_e42870_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32400_e42868: f64 = (locals.var_t8).sqrt();
        (assign32400_e42868, (locals.var_t8_dn0 / (2.0 * assign32400_e42868)), (locals.var_t8_dn2 / (2.0 * assign32400_e42868)), (locals.var_t8_dn3 / (2.0 * assign32400_e42868)), (locals.var_t8_dn4 / (2.0 * assign32400_e42868)), (locals.var_t8_dn5 / (2.0 * assign32400_e42868)), (locals.var_t8_dn6 / (2.0 * assign32400_e42868)), (locals.var_t8_dn7 / (2.0 * assign32400_e42868)), (locals.var_t8_dn8 / (2.0 * assign32400_e42868)), (locals.var_t8_dn9 / (2.0 * assign32400_e42868)), (locals.var_t8_dn10 / (2.0 * assign32400_e42868)), (locals.var_t8_dn11 / (2.0 * assign32400_e42868)), (locals.var_t8_dn12 / (2.0 * assign32400_e42868)), (locals.var_t8_dn13 / (2.0 * assign32400_e42868)), (locals.var_t8_dn14 / (2.0 * assign32400_e42868)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    }
};
        locals.var_sqrtpsip = assign32400_e42870;
        locals.var_sqrtpsip_dn0 = assign32400_e42870_d_n0;
        locals.var_sqrtpsip_dn2 = assign32400_e42870_d_n2;
        locals.var_sqrtpsip_dn3 = assign32400_e42870_d_n3;
        locals.var_sqrtpsip_dn4 = assign32400_e42870_d_n4;
        locals.var_sqrtpsip_dn5 = assign32400_e42870_d_n5;
        locals.var_sqrtpsip_dn6 = assign32400_e42870_d_n6;
        locals.var_sqrtpsip_dn7 = assign32400_e42870_d_n7;
        locals.var_sqrtpsip_dn8 = assign32400_e42870_d_n8;
        locals.var_sqrtpsip_dn9 = assign32400_e42870_d_n9;
        locals.var_sqrtpsip_dn10 = assign32400_e42870_d_n10;
        locals.var_sqrtpsip_dn11 = assign32400_e42870_d_n11;
        locals.var_sqrtpsip_dn12 = assign32400_e42870_d_n12;
        locals.var_sqrtpsip_dn13 = assign32400_e42870_d_n13;
        locals.var_sqrtpsip_dn14 = assign32400_e42870_d_n14;

        let (assign32410_e42882, assign32410_e42882_d_n0, assign32410_e42882_d_n2, assign32410_e42882_d_n3, assign32410_e42882_d_n4, assign32410_e42882_d_n5, assign32410_e42882_d_n6, assign32410_e42882_d_n7, assign32410_e42882_d_n8, assign32410_e42882_d_n9, assign32410_e42882_d_n10, assign32410_e42882_d_n11, assign32410_e42882_d_n12, assign32410_e42882_d_n13, assign32410_e42882_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32410_e42876: f64 = (2.0 * locals.var_sqrtpsip);
        let assign32410_e42877: f64 = (locals.var_gam_edge / assign32410_e42876);
        let assign32410_e42878: f64 = (1.0 + assign32410_e42877);
        let assign32410_e42880: f64 = (assign32410_e42878 / locals.var_gam_edge);
        (assign32410_e42880, ((((((locals.var_gam_edge_dn0 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn0))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn0)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn2 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn2))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn2)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn3 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn3))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn3)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn4 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn4))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn4)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn5 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn5))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn5)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn6 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn6))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn6)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn7 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn7))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn7)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn8 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn8))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn8)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn9 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn9))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn9)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn10 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn10))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn10)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn11 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn11))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn11)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn12 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn12))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn12)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn13 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn13))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn13)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn14 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn14))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn14)) / (locals.var_gam_edge * locals.var_gam_edge)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32410_e42882;
        locals.var_t0_dn0 = assign32410_e42882_d_n0;
        locals.var_t0_dn2 = assign32410_e42882_d_n2;
        locals.var_t0_dn3 = assign32410_e42882_d_n3;
        locals.var_t0_dn4 = assign32410_e42882_d_n4;
        locals.var_t0_dn5 = assign32410_e42882_d_n5;
        locals.var_t0_dn6 = assign32410_e42882_d_n6;
        locals.var_t0_dn7 = assign32410_e42882_d_n7;
        locals.var_t0_dn8 = assign32410_e42882_d_n8;
        locals.var_t0_dn9 = assign32410_e42882_d_n9;
        locals.var_t0_dn10 = assign32410_e42882_d_n10;
        locals.var_t0_dn11 = assign32410_e42882_d_n11;
        locals.var_t0_dn12 = assign32410_e42882_d_n12;
        locals.var_t0_dn13 = assign32410_e42882_d_n13;
        locals.var_t0_dn14 = assign32410_e42882_d_n14;

    }

    pub(super) fn stamp_transient_block_104(
        locals: &mut StampLocals,
    ) {
        let (assign32420_e42892, assign32420_e42892_d_n0, assign32420_e42892_d_n2, assign32420_e42892_d_n3, assign32420_e42892_d_n4, assign32420_e42892_d_n5, assign32420_e42892_d_n6, assign32420_e42892_d_n7, assign32420_e42892_d_n8, assign32420_e42892_d_n9, assign32420_e42892_d_n10, assign32420_e42892_d_n11, assign32420_e42892_d_n12, assign32420_e42892_d_n13, assign32420_e42892_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32420_e42887: f64 = (2.0 * locals.var_phib_n_edge);
        let assign32420_e42888: f64 = (locals.var_psip - assign32420_e42887);
        let assign32420_e42890: f64 = (assign32420_e42888 - locals.var_vdeff);
        (assign32420_e42890, ((locals.var_psip_dn0 - (2.0 * locals.var_phib_n_edge_dn0)) - locals.var_vdeff_dn0), ((locals.var_psip_dn2 - (2.0 * locals.var_phib_n_edge_dn2)) - locals.var_vdeff_dn2), ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_edge_dn3)) - locals.var_vdeff_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_edge_dn4)) - locals.var_vdeff_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_edge_dn5)) - locals.var_vdeff_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_edge_dn6)) - locals.var_vdeff_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_edge_dn7)) - locals.var_vdeff_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_edge_dn8)) - locals.var_vdeff_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_edge_dn9)) - locals.var_vdeff_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_edge_dn10)) - locals.var_vdeff_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_edge_dn11)) - locals.var_vdeff_dn11), ((locals.var_psip_dn12 - (2.0 * locals.var_phib_n_edge_dn12)) - locals.var_vdeff_dn12), ((locals.var_psip_dn13 - (2.0 * locals.var_phib_n_edge_dn13)) - locals.var_vdeff_dn13), ((locals.var_psip_dn14 - (2.0 * locals.var_phib_n_edge_dn14)) - locals.var_vdeff_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32420_e42892;
        locals.var_t1_dn0 = assign32420_e42892_d_n0;
        locals.var_t1_dn2 = assign32420_e42892_d_n2;
        locals.var_t1_dn3 = assign32420_e42892_d_n3;
        locals.var_t1_dn4 = assign32420_e42892_d_n4;
        locals.var_t1_dn5 = assign32420_e42892_d_n5;
        locals.var_t1_dn6 = assign32420_e42892_d_n6;
        locals.var_t1_dn7 = assign32420_e42892_d_n7;
        locals.var_t1_dn8 = assign32420_e42892_d_n8;
        locals.var_t1_dn9 = assign32420_e42892_d_n9;
        locals.var_t1_dn10 = assign32420_e42892_d_n10;
        locals.var_t1_dn11 = assign32420_e42892_d_n11;
        locals.var_t1_dn12 = assign32420_e42892_d_n12;
        locals.var_t1_dn13 = assign32420_e42892_d_n13;
        locals.var_t1_dn14 = assign32420_e42892_d_n14;

        let (assign32430_e42907, assign32430_e42907_d_n0, assign32430_e42907_d_n2, assign32430_e42907_d_n3, assign32430_e42907_d_n4, assign32430_e42907_d_n5, assign32430_e42907_d_n6, assign32430_e42907_d_n7, assign32430_e42907_d_n8, assign32430_e42907_d_n9, assign32430_e42907_d_n10, assign32430_e42907_d_n11, assign32430_e42907_d_n12, assign32430_e42907_d_n13, assign32430_e42907_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32430_e42896: f64 = locals.var_t1;
        let assign32430_e42899: f64 = (4.0 * locals.var_t0);
        let assign32430_e42901: f64 = (assign32430_e42899 * locals.var_sqrtpsip);
        let assign32430_e42903: f64 = (assign32430_e42901).max(1e-38);
        let assign32430_e42904: f64 = (assign32430_e42903).ln();
        let assign32430_e42905: f64 = (assign32430_e42896 - assign32430_e42904);
        (assign32430_e42905, (locals.var_t1_dn0 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn0) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn0)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn2 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn2) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn2)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn3 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn4 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn5 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn6 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn7 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn8 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn9 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn10 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn11 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn12 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn12) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn12)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn13 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn13) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn13)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn14 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn14) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn14)) } else { 0.0 } / assign32430_e42903)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32430_e42907;
        locals.var_t2_dn0 = assign32430_e42907_d_n0;
        locals.var_t2_dn2 = assign32430_e42907_d_n2;
        locals.var_t2_dn3 = assign32430_e42907_d_n3;
        locals.var_t2_dn4 = assign32430_e42907_d_n4;
        locals.var_t2_dn5 = assign32430_e42907_d_n5;
        locals.var_t2_dn6 = assign32430_e42907_d_n6;
        locals.var_t2_dn7 = assign32430_e42907_d_n7;
        locals.var_t2_dn8 = assign32430_e42907_d_n8;
        locals.var_t2_dn9 = assign32430_e42907_d_n9;
        locals.var_t2_dn10 = assign32430_e42907_d_n10;
        locals.var_t2_dn11 = assign32430_e42907_d_n11;
        locals.var_t2_dn12 = assign32430_e42907_d_n12;
        locals.var_t2_dn13 = assign32430_e42907_d_n13;
        locals.var_t2_dn14 = assign32430_e42907_d_n14;

        let (assign32440_e42924, assign32440_e42924_d_n0, assign32440_e42924_d_n2, assign32440_e42924_d_n3, assign32440_e42924_d_n4, assign32440_e42924_d_n5, assign32440_e42924_d_n6, assign32440_e42924_d_n7, assign32440_e42924_d_n8, assign32440_e42924_d_n9, assign32440_e42924_d_n10, assign32440_e42924_d_n11, assign32440_e42924_d_n12, assign32440_e42924_d_n13, assign32440_e42924_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32440_e42912: f64 = (locals.var_t2 - 0.201491);
        let assign32440_e42916: f64 = (locals.var_t2 + 0.402982);
        let assign32440_e42917: f64 = (locals.var_t2 * assign32440_e42916);
        let assign32440_e42919: f64 = (assign32440_e42917 + 2.446562);
        let assign32440_e42920: f64 = (assign32440_e42919).sqrt();
        let assign32440_e42921: f64 = (assign32440_e42912 - assign32440_e42920);
        let assign32440_e42922: f64 = (0.5 * assign32440_e42921);
        (assign32440_e42922, (0.5 * (locals.var_t2_dn0 - (((locals.var_t2_dn0 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn2 - (((locals.var_t2_dn2 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn12 - (((locals.var_t2_dn12 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn13 - (((locals.var_t2_dn13 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn14 - (((locals.var_t2_dn14 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign32440_e42920)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32440_e42924;
        locals.var_t8_dn0 = assign32440_e42924_d_n0;
        locals.var_t8_dn2 = assign32440_e42924_d_n2;
        locals.var_t8_dn3 = assign32440_e42924_d_n3;
        locals.var_t8_dn4 = assign32440_e42924_d_n4;
        locals.var_t8_dn5 = assign32440_e42924_d_n5;
        locals.var_t8_dn6 = assign32440_e42924_d_n6;
        locals.var_t8_dn7 = assign32440_e42924_d_n7;
        locals.var_t8_dn8 = assign32440_e42924_d_n8;
        locals.var_t8_dn9 = assign32440_e42924_d_n9;
        locals.var_t8_dn10 = assign32440_e42924_d_n10;
        locals.var_t8_dn11 = assign32440_e42924_d_n11;
        locals.var_t8_dn12 = assign32440_e42924_d_n12;
        locals.var_t8_dn13 = assign32440_e42924_d_n13;
        locals.var_t8_dn14 = assign32440_e42924_d_n14;

        let (assign32450_e42928, assign32450_e42928_d_n0, assign32450_e42928_d_n2, assign32450_e42928_d_n3, assign32450_e42928_d_n4, assign32450_e42928_d_n5, assign32450_e42928_d_n6, assign32450_e42928_d_n7, assign32450_e42928_d_n8, assign32450_e42928_d_n9, assign32450_e42928_d_n10, assign32450_e42928_d_n11, assign32450_e42928_d_n12, assign32450_e42928_d_n13, assign32450_e42928_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn0, locals.var_sqrtpsisa_dn2, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11, locals.var_sqrtpsisa_dn12, locals.var_sqrtpsisa_dn13, locals.var_sqrtpsisa_dn14,)
    }
};
        locals.var_sqrtpsisa = assign32450_e42928;
        locals.var_sqrtpsisa_dn0 = assign32450_e42928_d_n0;
        locals.var_sqrtpsisa_dn2 = assign32450_e42928_d_n2;
        locals.var_sqrtpsisa_dn3 = assign32450_e42928_d_n3;
        locals.var_sqrtpsisa_dn4 = assign32450_e42928_d_n4;
        locals.var_sqrtpsisa_dn5 = assign32450_e42928_d_n5;
        locals.var_sqrtpsisa_dn6 = assign32450_e42928_d_n6;
        locals.var_sqrtpsisa_dn7 = assign32450_e42928_d_n7;
        locals.var_sqrtpsisa_dn8 = assign32450_e42928_d_n8;
        locals.var_sqrtpsisa_dn9 = assign32450_e42928_d_n9;
        locals.var_sqrtpsisa_dn10 = assign32450_e42928_d_n10;
        locals.var_sqrtpsisa_dn11 = assign32450_e42928_d_n11;
        locals.var_sqrtpsisa_dn12 = assign32450_e42928_d_n12;
        locals.var_sqrtpsisa_dn13 = assign32450_e42928_d_n13;
        locals.var_sqrtpsisa_dn14 = assign32450_e42928_d_n14;

        let assign32460_e42931: f64 = (-68.0);
        let assign32460_e42932: f64 = if locals.var_t8 <= assign32460_e42931 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign32460_e42932;

        let (assign32470_e42939, assign32470_e42939_d_n0, assign32470_e42939_d_n2, assign32470_e42939_d_n3, assign32470_e42939_d_n4, assign32470_e42939_d_n5, assign32470_e42939_d_n6, assign32470_e42939_d_n7, assign32470_e42939_d_n8, assign32470_e42939_d_n9, assign32470_e42939_d_n10, assign32470_e42939_d_n11, assign32470_e42939_d_n12, assign32470_e42939_d_n13, assign32470_e42939_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        let assign32470_e42937: f64 = (-100.0);
        (assign32470_e42937, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32470_e42939;
        locals.var_t4_dn0 = assign32470_e42939_d_n0;
        locals.var_t4_dn2 = assign32470_e42939_d_n2;
        locals.var_t4_dn3 = assign32470_e42939_d_n3;
        locals.var_t4_dn4 = assign32470_e42939_d_n4;
        locals.var_t4_dn5 = assign32470_e42939_d_n5;
        locals.var_t4_dn6 = assign32470_e42939_d_n6;
        locals.var_t4_dn7 = assign32470_e42939_d_n7;
        locals.var_t4_dn8 = assign32470_e42939_d_n8;
        locals.var_t4_dn9 = assign32470_e42939_d_n9;
        locals.var_t4_dn10 = assign32470_e42939_d_n10;
        locals.var_t4_dn11 = assign32470_e42939_d_n11;
        locals.var_t4_dn12 = assign32470_e42939_d_n12;
        locals.var_t4_dn13 = assign32470_e42939_d_n13;
        locals.var_t4_dn14 = assign32470_e42939_d_n14;

        let (assign32480_e42945, assign32480_e42945_d_n0, assign32480_e42945_d_n2, assign32480_e42945_d_n3, assign32480_e42945_d_n4, assign32480_e42945_d_n5, assign32480_e42945_d_n6, assign32480_e42945_d_n7, assign32480_e42945_d_n8, assign32480_e42945_d_n9, assign32480_e42945_d_n10, assign32480_e42945_d_n11, assign32480_e42945_d_n12, assign32480_e42945_d_n13, assign32480_e42945_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32480_e42945;
        locals.var_t5_dn0 = assign32480_e42945_d_n0;
        locals.var_t5_dn2 = assign32480_e42945_d_n2;
        locals.var_t5_dn3 = assign32480_e42945_d_n3;
        locals.var_t5_dn4 = assign32480_e42945_d_n4;
        locals.var_t5_dn5 = assign32480_e42945_d_n5;
        locals.var_t5_dn6 = assign32480_e42945_d_n6;
        locals.var_t5_dn7 = assign32480_e42945_d_n7;
        locals.var_t5_dn8 = assign32480_e42945_d_n8;
        locals.var_t5_dn9 = assign32480_e42945_d_n9;
        locals.var_t5_dn10 = assign32480_e42945_d_n10;
        locals.var_t5_dn11 = assign32480_e42945_d_n11;
        locals.var_t5_dn12 = assign32480_e42945_d_n12;
        locals.var_t5_dn13 = assign32480_e42945_d_n13;
        locals.var_t5_dn14 = assign32480_e42945_d_n14;

        let assign32490_e42950: f64 = (0.5 * locals.var_t5);
        let assign32490_e42951: f64 = (locals.var_t4 - assign32490_e42950);
        let assign32490_e42952: f64 = if locals.var_t8 < assign32490_e42951 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign32490_e42952;

        let (assign32500_e42961, assign32500_e42961_d_n0, assign32500_e42961_d_n2, assign32500_e42961_d_n3, assign32500_e42961_d_n4, assign32500_e42961_d_n5, assign32500_e42961_d_n6, assign32500_e42961_d_n7, assign32500_e42961_d_n8, assign32500_e42961_d_n9, assign32500_e42961_d_n10, assign32500_e42961_d_n11, assign32500_e42961_d_n12, assign32500_e42961_d_n13, assign32500_e42961_d_n14,) = {
    if (((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 != 0.0)) {
        let assign32500_e42959: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32500_e42959, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn0), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn2), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn12), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn13), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32500_e42961;
        locals.var_t3_dn0 = assign32500_e42961_d_n0;
        locals.var_t3_dn2 = assign32500_e42961_d_n2;
        locals.var_t3_dn3 = assign32500_e42961_d_n3;
        locals.var_t3_dn4 = assign32500_e42961_d_n4;
        locals.var_t3_dn5 = assign32500_e42961_d_n5;
        locals.var_t3_dn6 = assign32500_e42961_d_n6;
        locals.var_t3_dn7 = assign32500_e42961_d_n7;
        locals.var_t3_dn8 = assign32500_e42961_d_n8;
        locals.var_t3_dn9 = assign32500_e42961_d_n9;
        locals.var_t3_dn10 = assign32500_e42961_d_n10;
        locals.var_t3_dn11 = assign32500_e42961_d_n11;
        locals.var_t3_dn12 = assign32500_e42961_d_n12;
        locals.var_t3_dn13 = assign32500_e42961_d_n13;
        locals.var_t3_dn14 = assign32500_e42961_d_n14;

        let assign32510_e42966: f64 = (0.5 * locals.var_t5);
        let assign32510_e42967: f64 = (locals.var_t4 + assign32510_e42966);
        let assign32510_e42968: f64 = if locals.var_t8 > assign32510_e42967 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign32510_e42968;

        let (assign32520_e42980, assign32520_e42980_d_n0, assign32520_e42980_d_n2, assign32520_e42980_d_n3, assign32520_e42980_d_n4, assign32520_e42980_d_n5, assign32520_e42980_d_n6, assign32520_e42980_d_n7, assign32520_e42980_d_n8, assign32520_e42980_d_n9, assign32520_e42980_d_n10, assign32520_e42980_d_n11, assign32520_e42980_d_n12, assign32520_e42980_d_n13, assign32520_e42980_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign32520_e42978: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32520_e42978, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32520_e42980;
        locals.var_t3_dn0 = assign32520_e42980_d_n0;
        locals.var_t3_dn2 = assign32520_e42980_d_n2;
        locals.var_t3_dn3 = assign32520_e42980_d_n3;
        locals.var_t3_dn4 = assign32520_e42980_d_n4;
        locals.var_t3_dn5 = assign32520_e42980_d_n5;
        locals.var_t3_dn6 = assign32520_e42980_d_n6;
        locals.var_t3_dn7 = assign32520_e42980_d_n7;
        locals.var_t3_dn8 = assign32520_e42980_d_n8;
        locals.var_t3_dn9 = assign32520_e42980_d_n9;
        locals.var_t3_dn10 = assign32520_e42980_d_n10;
        locals.var_t3_dn11 = assign32520_e42980_d_n11;
        locals.var_t3_dn12 = assign32520_e42980_d_n12;
        locals.var_t3_dn13 = assign32520_e42980_d_n13;
        locals.var_t3_dn14 = assign32520_e42980_d_n14;

        let (assign32530_e42996, assign32530_e42996_d_n0, assign32530_e42996_d_n2, assign32530_e42996_d_n3, assign32530_e42996_d_n4, assign32530_e42996_d_n5, assign32530_e42996_d_n6, assign32530_e42996_d_n7, assign32530_e42996_d_n8, assign32530_e42996_d_n9, assign32530_e42996_d_n10, assign32530_e42996_d_n11, assign32530_e42996_d_n12, assign32530_e42996_d_n13, assign32530_e42996_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32530_e42992: f64 = (locals.var_t8 - locals.var_t4);
        let assign32530_e42994: f64 = (assign32530_e42992 / locals.var_t5);
        (assign32530_e42994, ((((locals.var_t8_dn0 - locals.var_t4_dn0) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn2 - locals.var_t4_dn2) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn12 - locals.var_t4_dn12) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn13 - locals.var_t4_dn13) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn14 - locals.var_t4_dn14) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32530_e42996;
        locals.var_t2_dn0 = assign32530_e42996_d_n0;
        locals.var_t2_dn2 = assign32530_e42996_d_n2;
        locals.var_t2_dn3 = assign32530_e42996_d_n3;
        locals.var_t2_dn4 = assign32530_e42996_d_n4;
        locals.var_t2_dn5 = assign32530_e42996_d_n5;
        locals.var_t2_dn6 = assign32530_e42996_d_n6;
        locals.var_t2_dn7 = assign32530_e42996_d_n7;
        locals.var_t2_dn8 = assign32530_e42996_d_n8;
        locals.var_t2_dn9 = assign32530_e42996_d_n9;
        locals.var_t2_dn10 = assign32530_e42996_d_n10;
        locals.var_t2_dn11 = assign32530_e42996_d_n11;
        locals.var_t2_dn12 = assign32530_e42996_d_n12;
        locals.var_t2_dn13 = assign32530_e42996_d_n13;
        locals.var_t2_dn14 = assign32530_e42996_d_n14;

        let (assign32540_e43010, assign32540_e43010_d_n0, assign32540_e43010_d_n2, assign32540_e43010_d_n3, assign32540_e43010_d_n4, assign32540_e43010_d_n5, assign32540_e43010_d_n6, assign32540_e43010_d_n7, assign32540_e43010_d_n8, assign32540_e43010_d_n9, assign32540_e43010_d_n10, assign32540_e43010_d_n11, assign32540_e43010_d_n12, assign32540_e43010_d_n13, assign32540_e43010_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32540_e43008: f64 = (locals.var_t2 * locals.var_t2);
        (assign32540_e43008, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32540_e43010;
        locals.var_t6_dn0 = assign32540_e43010_d_n0;
        locals.var_t6_dn2 = assign32540_e43010_d_n2;
        locals.var_t6_dn3 = assign32540_e43010_d_n3;
        locals.var_t6_dn4 = assign32540_e43010_d_n4;
        locals.var_t6_dn5 = assign32540_e43010_d_n5;
        locals.var_t6_dn6 = assign32540_e43010_d_n6;
        locals.var_t6_dn7 = assign32540_e43010_d_n7;
        locals.var_t6_dn8 = assign32540_e43010_d_n8;
        locals.var_t6_dn9 = assign32540_e43010_d_n9;
        locals.var_t6_dn10 = assign32540_e43010_d_n10;
        locals.var_t6_dn11 = assign32540_e43010_d_n11;
        locals.var_t6_dn12 = assign32540_e43010_d_n12;
        locals.var_t6_dn13 = assign32540_e43010_d_n13;
        locals.var_t6_dn14 = assign32540_e43010_d_n14;

        let (assign32550_e43045, assign32550_e43045_d_n0, assign32550_e43045_d_n2, assign32550_e43045_d_n3, assign32550_e43045_d_n4, assign32550_e43045_d_n5, assign32550_e43045_d_n6, assign32550_e43045_d_n7, assign32550_e43045_d_n8, assign32550_e43045_d_n9, assign32550_e43045_d_n10, assign32550_e43045_d_n11, assign32550_e43045_d_n12, assign32550_e43045_d_n13, assign32550_e43045_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32550_e43024: f64 = (5.0 / 64.0);
        let assign32550_e43027: f64 = (0.5 * locals.var_t2);
        let assign32550_e43028: f64 = (assign32550_e43024 + assign32550_e43027);
        let assign32550_e43032: f64 = (15.0 / 16.0);
        let assign32550_e43036: f64 = (1.25 - locals.var_t6);
        let assign32550_e43037: f64 = (locals.var_t6 * assign32550_e43036);
        let assign32550_e43038: f64 = (assign32550_e43032 - assign32550_e43037);
        let assign32550_e43039: f64 = (locals.var_t6 * assign32550_e43038);
        let assign32550_e43040: f64 = (assign32550_e43028 + assign32550_e43039);
        let assign32550_e43041: f64 = (locals.var_t5 * assign32550_e43040);
        let assign32550_e43042: f64 = (locals.var_t4 + assign32550_e43041);
        let assign32550_e43043: f64 = { let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32550_e43043, ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn0) + ((locals.var_t6_dn0 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn0 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn0))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn2) + ((locals.var_t6_dn2 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn2 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn2))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn12 + ((locals.var_t5_dn12 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn12) + ((locals.var_t6_dn12 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn12 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn12))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn13 + ((locals.var_t5_dn13 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn13) + ((locals.var_t6_dn13 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn13 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn13))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn14) + ((locals.var_t6_dn14 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn14 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn14))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32550_e43045;
        locals.var_t3_dn0 = assign32550_e43045_d_n0;
        locals.var_t3_dn2 = assign32550_e43045_d_n2;
        locals.var_t3_dn3 = assign32550_e43045_d_n3;
        locals.var_t3_dn4 = assign32550_e43045_d_n4;
        locals.var_t3_dn5 = assign32550_e43045_d_n5;
        locals.var_t3_dn6 = assign32550_e43045_d_n6;
        locals.var_t3_dn7 = assign32550_e43045_d_n7;
        locals.var_t3_dn8 = assign32550_e43045_d_n8;
        locals.var_t3_dn9 = assign32550_e43045_d_n9;
        locals.var_t3_dn10 = assign32550_e43045_d_n10;
        locals.var_t3_dn11 = assign32550_e43045_d_n11;
        locals.var_t3_dn12 = assign32550_e43045_d_n12;
        locals.var_t3_dn13 = assign32550_e43045_d_n13;
        locals.var_t3_dn14 = assign32550_e43045_d_n14;

        let (assign32560_e43078, assign32560_e43078_d_n0, assign32560_e43078_d_n2, assign32560_e43078_d_n3, assign32560_e43078_d_n4, assign32560_e43078_d_n5, assign32560_e43078_d_n6, assign32560_e43078_d_n7, assign32560_e43078_d_n8, assign32560_e43078_d_n9, assign32560_e43078_d_n10, assign32560_e43078_d_n11, assign32560_e43078_d_n12, assign32560_e43078_d_n13, assign32560_e43078_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        let assign32560_e43052: f64 = (1.0 + locals.var_t1);
        let assign32560_e43055: f64 = locals.var_t8;
        let assign32560_e43056: f64 = (assign32560_e43052 - assign32560_e43055);
        let assign32560_e43060: f64 = (2.0 * locals.var_t0);
        let assign32560_e43063: f64 = (locals.var_t3 * 2.0);
        let assign32560_e43065: f64 = (assign32560_e43063 * locals.var_t0);
        let assign32560_e43068: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32560_e43069: f64 = (assign32560_e43065 + assign32560_e43068);
        let assign32560_e43070: f64 = (assign32560_e43060 * assign32560_e43069);
        let assign32560_e43072: f64 = (assign32560_e43070).max(1e-38);
        let assign32560_e43073: f64 = (assign32560_e43072).ln();
        let assign32560_e43074: f64 = assign32560_e43073;
        let assign32560_e43075: f64 = (assign32560_e43056 - assign32560_e43074);
        let assign32560_e43076: f64 = (locals.var_t3 * assign32560_e43075);
        (assign32560_e43076, ((locals.var_t3_dn0 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn0 - locals.var_t8_dn0) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn0) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn2 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn2 - locals.var_t8_dn2) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn2) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn3 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn4 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn5 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn6 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn7 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn8 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn9 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn10 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn11 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn12 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn12 - locals.var_t8_dn12) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn12) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn13 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn13 - locals.var_t8_dn13) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn13) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn14 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn14 - locals.var_t8_dn14) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn14) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32560_e43072)))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn0, locals.var_qdeff_edge_dn2, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11, locals.var_qdeff_edge_dn12, locals.var_qdeff_edge_dn13, locals.var_qdeff_edge_dn14,)
    }
};
        locals.var_qdeff_edge = assign32560_e43078;
        locals.var_qdeff_edge_dn0 = assign32560_e43078_d_n0;
        locals.var_qdeff_edge_dn2 = assign32560_e43078_d_n2;
        locals.var_qdeff_edge_dn3 = assign32560_e43078_d_n3;
        locals.var_qdeff_edge_dn4 = assign32560_e43078_d_n4;
        locals.var_qdeff_edge_dn5 = assign32560_e43078_d_n5;
        locals.var_qdeff_edge_dn6 = assign32560_e43078_d_n6;
        locals.var_qdeff_edge_dn7 = assign32560_e43078_d_n7;
        locals.var_qdeff_edge_dn8 = assign32560_e43078_d_n8;
        locals.var_qdeff_edge_dn9 = assign32560_e43078_d_n9;
        locals.var_qdeff_edge_dn10 = assign32560_e43078_d_n10;
        locals.var_qdeff_edge_dn11 = assign32560_e43078_d_n11;
        locals.var_qdeff_edge_dn12 = assign32560_e43078_d_n12;
        locals.var_qdeff_edge_dn13 = assign32560_e43078_d_n13;
        locals.var_qdeff_edge_dn14 = assign32560_e43078_d_n14;

        let (assign32570_e43086, assign32570_e43086_d_n0, assign32570_e43086_d_n2, assign32570_e43086_d_n3, assign32570_e43086_d_n4, assign32570_e43086_d_n5, assign32570_e43086_d_n6, assign32570_e43086_d_n7, assign32570_e43086_d_n8, assign32570_e43086_d_n9, assign32570_e43086_d_n10, assign32570_e43086_d_n11, assign32570_e43086_d_n12, assign32570_e43086_d_n13, assign32570_e43086_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32570_e43084: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32570_e43084, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32570_e43086;
        locals.var_t3_dn0 = assign32570_e43086_d_n0;
        locals.var_t3_dn2 = assign32570_e43086_d_n2;
        locals.var_t3_dn3 = assign32570_e43086_d_n3;
        locals.var_t3_dn4 = assign32570_e43086_d_n4;
        locals.var_t3_dn5 = assign32570_e43086_d_n5;
        locals.var_t3_dn6 = assign32570_e43086_d_n6;
        locals.var_t3_dn7 = assign32570_e43086_d_n7;
        locals.var_t3_dn8 = assign32570_e43086_d_n8;
        locals.var_t3_dn9 = assign32570_e43086_d_n9;
        locals.var_t3_dn10 = assign32570_e43086_d_n10;
        locals.var_t3_dn11 = assign32570_e43086_d_n11;
        locals.var_t3_dn12 = assign32570_e43086_d_n12;
        locals.var_t3_dn13 = assign32570_e43086_d_n13;
        locals.var_t3_dn14 = assign32570_e43086_d_n14;

        let (assign32580_e43095, assign32580_e43095_d_n0, assign32580_e43095_d_n2, assign32580_e43095_d_n3, assign32580_e43095_d_n4, assign32580_e43095_d_n5, assign32580_e43095_d_n6, assign32580_e43095_d_n7, assign32580_e43095_d_n8, assign32580_e43095_d_n9, assign32580_e43095_d_n10, assign32580_e43095_d_n11, assign32580_e43095_d_n12, assign32580_e43095_d_n13, assign32580_e43095_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32580_e43093: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign32580_e43093, (-(locals.var_sqrtpsisa_dn0 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn2 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn12 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn13 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn14 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn0, locals.var_sqrtpsisainv_dn2, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11, locals.var_sqrtpsisainv_dn12, locals.var_sqrtpsisainv_dn13, locals.var_sqrtpsisainv_dn14,)
    }
};
        locals.var_sqrtpsisainv = assign32580_e43095;
        locals.var_sqrtpsisainv_dn0 = assign32580_e43095_d_n0;
        locals.var_sqrtpsisainv_dn2 = assign32580_e43095_d_n2;
        locals.var_sqrtpsisainv_dn3 = assign32580_e43095_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign32580_e43095_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign32580_e43095_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign32580_e43095_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign32580_e43095_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign32580_e43095_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign32580_e43095_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign32580_e43095_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign32580_e43095_d_n11;
        locals.var_sqrtpsisainv_dn12 = assign32580_e43095_d_n12;
        locals.var_sqrtpsisainv_dn13 = assign32580_e43095_d_n13;
        locals.var_sqrtpsisainv_dn14 = assign32580_e43095_d_n14;

        let (assign32590_e43127, assign32590_e43127_d_n0, assign32590_e43127_d_n2, assign32590_e43127_d_n3, assign32590_e43127_d_n4, assign32590_e43127_d_n5, assign32590_e43127_d_n6, assign32590_e43127_d_n7, assign32590_e43127_d_n8, assign32590_e43127_d_n9, assign32590_e43127_d_n10, assign32590_e43127_d_n11, assign32590_e43127_d_n12, assign32590_e43127_d_n13, assign32590_e43127_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32590_e43102: f64 = (2.0 * locals.var_t3);
        let assign32590_e43106: f64 = (locals.var_t3 * 2.0);
        let assign32590_e43108: f64 = (assign32590_e43106 * locals.var_t0);
        let assign32590_e43111: f64 = (locals.var_t3 * 2.0);
        let assign32590_e43113: f64 = (assign32590_e43111 * locals.var_t0);
        let assign32590_e43116: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32590_e43117: f64 = (assign32590_e43113 + assign32590_e43116);
        let assign32590_e43118: f64 = (assign32590_e43108 * assign32590_e43117);
        let assign32590_e43120: f64 = (assign32590_e43118).max(1e-38);
        let assign32590_e43121: f64 = (assign32590_e43120).ln();
        let assign32590_e43122: f64 = assign32590_e43121;
        let assign32590_e43123: f64 = (assign32590_e43102 + assign32590_e43122);
        let assign32590_e43125: f64 = (assign32590_e43123 - locals.var_t1);
        (assign32590_e43125, (((2.0 * locals.var_t3_dn0) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn0)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn2)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn3)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn4)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn5)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn6)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn7)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn8)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn9)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn10)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn11)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn12)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn13)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn14)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32590_e43127;
        locals.var_t4_dn0 = assign32590_e43127_d_n0;
        locals.var_t4_dn2 = assign32590_e43127_d_n2;
        locals.var_t4_dn3 = assign32590_e43127_d_n3;
        locals.var_t4_dn4 = assign32590_e43127_d_n4;
        locals.var_t4_dn5 = assign32590_e43127_d_n5;
        locals.var_t4_dn6 = assign32590_e43127_d_n6;
        locals.var_t4_dn7 = assign32590_e43127_d_n7;
        locals.var_t4_dn8 = assign32590_e43127_d_n8;
        locals.var_t4_dn9 = assign32590_e43127_d_n9;
        locals.var_t4_dn10 = assign32590_e43127_d_n10;
        locals.var_t4_dn11 = assign32590_e43127_d_n11;
        locals.var_t4_dn12 = assign32590_e43127_d_n12;
        locals.var_t4_dn13 = assign32590_e43127_d_n13;
        locals.var_t4_dn14 = assign32590_e43127_d_n14;

        let (assign32600_e43152, assign32600_e43152_d_n0, assign32600_e43152_d_n2, assign32600_e43152_d_n3, assign32600_e43152_d_n4, assign32600_e43152_d_n5, assign32600_e43152_d_n6, assign32600_e43152_d_n7, assign32600_e43152_d_n8, assign32600_e43152_d_n9, assign32600_e43152_d_n10, assign32600_e43152_d_n11, assign32600_e43152_d_n12, assign32600_e43152_d_n13, assign32600_e43152_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32600_e43135: f64 = 1.0;
        let assign32600_e43137: f64 = (assign32600_e43135 / locals.var_t3);
        let assign32600_e43138: f64 = (2.0 + assign32600_e43137);
        let assign32600_e43142: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32600_e43143: f64 = assign32600_e43142;
        let assign32600_e43146: f64 = (locals.var_t0 * locals.var_t3);
        let assign32600_e43148: f64 = (assign32600_e43146 + locals.var_sqrtpsisa);
        let assign32600_e43149: f64 = (assign32600_e43143 / assign32600_e43148);
        let assign32600_e43150: f64 = (assign32600_e43138 + assign32600_e43149);
        (assign32600_e43150, ((-((assign32600_e43135 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32600_e43148 * assign32600_e43148))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32600_e43152;
        locals.var_t5_dn0 = assign32600_e43152_d_n0;
        locals.var_t5_dn2 = assign32600_e43152_d_n2;
        locals.var_t5_dn3 = assign32600_e43152_d_n3;
        locals.var_t5_dn4 = assign32600_e43152_d_n4;
        locals.var_t5_dn5 = assign32600_e43152_d_n5;
        locals.var_t5_dn6 = assign32600_e43152_d_n6;
        locals.var_t5_dn7 = assign32600_e43152_d_n7;
        locals.var_t5_dn8 = assign32600_e43152_d_n8;
        locals.var_t5_dn9 = assign32600_e43152_d_n9;
        locals.var_t5_dn10 = assign32600_e43152_d_n10;
        locals.var_t5_dn11 = assign32600_e43152_d_n11;
        locals.var_t5_dn12 = assign32600_e43152_d_n12;
        locals.var_t5_dn13 = assign32600_e43152_d_n13;
        locals.var_t5_dn14 = assign32600_e43152_d_n14;

        let (assign32610_e43163, assign32610_e43163_d_n0, assign32610_e43163_d_n2, assign32610_e43163_d_n3, assign32610_e43163_d_n4, assign32610_e43163_d_n5, assign32610_e43163_d_n6, assign32610_e43163_d_n7, assign32610_e43163_d_n8, assign32610_e43163_d_n9, assign32610_e43163_d_n10, assign32610_e43163_d_n11, assign32610_e43163_d_n12, assign32610_e43163_d_n13, assign32610_e43163_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32610_e43160: f64 = (locals.var_t4 / locals.var_t5);
        let assign32610_e43161: f64 = (locals.var_t3 - assign32610_e43160);
        (assign32610_e43161, (locals.var_t3_dn0 - (((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn2 - (((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn12 - (((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn13 - (((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn14 - (((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32610_e43163;
        locals.var_t3_dn0 = assign32610_e43163_d_n0;
        locals.var_t3_dn2 = assign32610_e43163_d_n2;
        locals.var_t3_dn3 = assign32610_e43163_d_n3;
        locals.var_t3_dn4 = assign32610_e43163_d_n4;
        locals.var_t3_dn5 = assign32610_e43163_d_n5;
        locals.var_t3_dn6 = assign32610_e43163_d_n6;
        locals.var_t3_dn7 = assign32610_e43163_d_n7;
        locals.var_t3_dn8 = assign32610_e43163_d_n8;
        locals.var_t3_dn9 = assign32610_e43163_d_n9;
        locals.var_t3_dn10 = assign32610_e43163_d_n10;
        locals.var_t3_dn11 = assign32610_e43163_d_n11;
        locals.var_t3_dn12 = assign32610_e43163_d_n12;
        locals.var_t3_dn13 = assign32610_e43163_d_n13;
        locals.var_t3_dn14 = assign32610_e43163_d_n14;

    }

    pub(super) fn stamp_transient_block_105(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32620_e43195, assign32620_e43195_d_n0, assign32620_e43195_d_n2, assign32620_e43195_d_n3, assign32620_e43195_d_n4, assign32620_e43195_d_n5, assign32620_e43195_d_n6, assign32620_e43195_d_n7, assign32620_e43195_d_n8, assign32620_e43195_d_n9, assign32620_e43195_d_n10, assign32620_e43195_d_n11, assign32620_e43195_d_n12, assign32620_e43195_d_n13, assign32620_e43195_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32620_e43170: f64 = (2.0 * locals.var_t3);
        let assign32620_e43174: f64 = (locals.var_t3 * 2.0);
        let assign32620_e43176: f64 = (assign32620_e43174 * locals.var_t0);
        let assign32620_e43179: f64 = (locals.var_t3 * 2.0);
        let assign32620_e43181: f64 = (assign32620_e43179 * locals.var_t0);
        let assign32620_e43184: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32620_e43185: f64 = (assign32620_e43181 + assign32620_e43184);
        let assign32620_e43186: f64 = (assign32620_e43176 * assign32620_e43185);
        let assign32620_e43188: f64 = (assign32620_e43186).max(1e-38);
        let assign32620_e43189: f64 = (assign32620_e43188).ln();
        let assign32620_e43190: f64 = assign32620_e43189;
        let assign32620_e43191: f64 = (assign32620_e43170 + assign32620_e43190);
        let assign32620_e43193: f64 = (assign32620_e43191 - locals.var_t1);
        (assign32620_e43193, (((2.0 * locals.var_t3_dn0) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn0)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn2)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn3)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn4)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn5)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn6)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn7)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn8)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn9)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn10)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn11)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn12)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn13)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn14)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32620_e43195;
        locals.var_t4_dn0 = assign32620_e43195_d_n0;
        locals.var_t4_dn2 = assign32620_e43195_d_n2;
        locals.var_t4_dn3 = assign32620_e43195_d_n3;
        locals.var_t4_dn4 = assign32620_e43195_d_n4;
        locals.var_t4_dn5 = assign32620_e43195_d_n5;
        locals.var_t4_dn6 = assign32620_e43195_d_n6;
        locals.var_t4_dn7 = assign32620_e43195_d_n7;
        locals.var_t4_dn8 = assign32620_e43195_d_n8;
        locals.var_t4_dn9 = assign32620_e43195_d_n9;
        locals.var_t4_dn10 = assign32620_e43195_d_n10;
        locals.var_t4_dn11 = assign32620_e43195_d_n11;
        locals.var_t4_dn12 = assign32620_e43195_d_n12;
        locals.var_t4_dn13 = assign32620_e43195_d_n13;
        locals.var_t4_dn14 = assign32620_e43195_d_n14;

        let (assign32630_e43220, assign32630_e43220_d_n0, assign32630_e43220_d_n2, assign32630_e43220_d_n3, assign32630_e43220_d_n4, assign32630_e43220_d_n5, assign32630_e43220_d_n6, assign32630_e43220_d_n7, assign32630_e43220_d_n8, assign32630_e43220_d_n9, assign32630_e43220_d_n10, assign32630_e43220_d_n11, assign32630_e43220_d_n12, assign32630_e43220_d_n13, assign32630_e43220_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32630_e43203: f64 = 1.0;
        let assign32630_e43205: f64 = (assign32630_e43203 / locals.var_t3);
        let assign32630_e43206: f64 = (2.0 + assign32630_e43205);
        let assign32630_e43210: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32630_e43211: f64 = assign32630_e43210;
        let assign32630_e43214: f64 = (locals.var_t0 * locals.var_t3);
        let assign32630_e43216: f64 = (assign32630_e43214 + locals.var_sqrtpsisa);
        let assign32630_e43217: f64 = (assign32630_e43211 / assign32630_e43216);
        let assign32630_e43218: f64 = (assign32630_e43206 + assign32630_e43217);
        (assign32630_e43218, ((-((assign32630_e43203 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32630_e43216 * assign32630_e43216))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32630_e43220;
        locals.var_t5_dn0 = assign32630_e43220_d_n0;
        locals.var_t5_dn2 = assign32630_e43220_d_n2;
        locals.var_t5_dn3 = assign32630_e43220_d_n3;
        locals.var_t5_dn4 = assign32630_e43220_d_n4;
        locals.var_t5_dn5 = assign32630_e43220_d_n5;
        locals.var_t5_dn6 = assign32630_e43220_d_n6;
        locals.var_t5_dn7 = assign32630_e43220_d_n7;
        locals.var_t5_dn8 = assign32630_e43220_d_n8;
        locals.var_t5_dn9 = assign32630_e43220_d_n9;
        locals.var_t5_dn10 = assign32630_e43220_d_n10;
        locals.var_t5_dn11 = assign32630_e43220_d_n11;
        locals.var_t5_dn12 = assign32630_e43220_d_n12;
        locals.var_t5_dn13 = assign32630_e43220_d_n13;
        locals.var_t5_dn14 = assign32630_e43220_d_n14;

        let (assign32640_e43247, assign32640_e43247_d_n0, assign32640_e43247_d_n2, assign32640_e43247_d_n3, assign32640_e43247_d_n4, assign32640_e43247_d_n5, assign32640_e43247_d_n6, assign32640_e43247_d_n7, assign32640_e43247_d_n8, assign32640_e43247_d_n9, assign32640_e43247_d_n10, assign32640_e43247_d_n11, assign32640_e43247_d_n12, assign32640_e43247_d_n13, assign32640_e43247_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32640_e43228: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32640_e43231: f64 = (locals.var_t0 * locals.var_t3);
        let assign32640_e43233: f64 = (assign32640_e43231 + locals.var_sqrtpsisa);
        let assign32640_e43234: f64 = (assign32640_e43228 / assign32640_e43233);
        let assign32640_e43235: f64 = assign32640_e43234;
        let assign32640_e43238: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32640_e43241: f64 = (locals.var_t0 * locals.var_t3);
        let assign32640_e43243: f64 = (assign32640_e43241 + locals.var_sqrtpsisa);
        let assign32640_e43244: f64 = (assign32640_e43238 / assign32640_e43243);
        let assign32640_e43245: f64 = (assign32640_e43235 * assign32640_e43244);
        (assign32640_e43245, ((((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32640_e43243 * assign32640_e43243)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32640_e43247;
        locals.var_t6_dn0 = assign32640_e43247_d_n0;
        locals.var_t6_dn2 = assign32640_e43247_d_n2;
        locals.var_t6_dn3 = assign32640_e43247_d_n3;
        locals.var_t6_dn4 = assign32640_e43247_d_n4;
        locals.var_t6_dn5 = assign32640_e43247_d_n5;
        locals.var_t6_dn6 = assign32640_e43247_d_n6;
        locals.var_t6_dn7 = assign32640_e43247_d_n7;
        locals.var_t6_dn8 = assign32640_e43247_d_n8;
        locals.var_t6_dn9 = assign32640_e43247_d_n9;
        locals.var_t6_dn10 = assign32640_e43247_d_n10;
        locals.var_t6_dn11 = assign32640_e43247_d_n11;
        locals.var_t6_dn12 = assign32640_e43247_d_n12;
        locals.var_t6_dn13 = assign32640_e43247_d_n13;
        locals.var_t6_dn14 = assign32640_e43247_d_n14;

        let (assign32650_e43281, assign32650_e43281_d_n0, assign32650_e43281_d_n2, assign32650_e43281_d_n3, assign32650_e43281_d_n4, assign32650_e43281_d_n5, assign32650_e43281_d_n6, assign32650_e43281_d_n7, assign32650_e43281_d_n8, assign32650_e43281_d_n9, assign32650_e43281_d_n10, assign32650_e43281_d_n11, assign32650_e43281_d_n12, assign32650_e43281_d_n13, assign32650_e43281_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32650_e43253: f64 = (-1.0);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign32650_e43256: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32650_e43259: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32650_e43260: f64 = (assign32650_e43256 * assign32650_e43259);
        let assign32650_e43261: f64 = (assign32650_e43253 * assign32650_e43260);
        let assign32650_e43264: f64 = 1.0;
        let assign32650_e43267: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign32650_e43269: f64 = (assign32650_e43267 * locals.var_sqrtpsisa);
        let assign32650_e43272: f64 = (locals.var_t0 * locals.var_t3);
        let assign32650_e43274: f64 = (assign32650_e43272 + locals.var_sqrtpsisa);
        let assign32650_e43275: f64 = (assign32650_e43269 * assign32650_e43274);
        let assign32650_e43276: f64 = (assign32650_e43264 / assign32650_e43275);
        let assign32650_e43277: f64 = (assign32650_e43261 - assign32650_e43276);
        let assign32650_e43279: f64 = (assign32650_e43277 - locals.var_t6);
        (assign32650_e43279, (((assign32650_e43253 * (((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn0 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn0)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn0)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn0), (((assign32650_e43253 * (((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn2 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn2)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn2)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn2), (((assign32650_e43253 * (((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn3)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn3), (((assign32650_e43253 * (((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn4)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn4), (((assign32650_e43253 * (((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn5)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn5), (((assign32650_e43253 * (((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn6)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn6), (((assign32650_e43253 * (((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn7)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn7), (((assign32650_e43253 * (((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn8)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn8), (((assign32650_e43253 * (((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn9)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn9), (((assign32650_e43253 * (((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn10)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn10), (((assign32650_e43253 * (((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn11)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn11), (((assign32650_e43253 * (((-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn12 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn12)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn12)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn12), (((assign32650_e43253 * (((-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn13 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn13)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn13)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn13), (((assign32650_e43253 * (((-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn14 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn14)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn14)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign32650_e43281;
        locals.var_t7_dn0 = assign32650_e43281_d_n0;
        locals.var_t7_dn2 = assign32650_e43281_d_n2;
        locals.var_t7_dn3 = assign32650_e43281_d_n3;
        locals.var_t7_dn4 = assign32650_e43281_d_n4;
        locals.var_t7_dn5 = assign32650_e43281_d_n5;
        locals.var_t7_dn6 = assign32650_e43281_d_n6;
        locals.var_t7_dn7 = assign32650_e43281_d_n7;
        locals.var_t7_dn8 = assign32650_e43281_d_n8;
        locals.var_t7_dn9 = assign32650_e43281_d_n9;
        locals.var_t7_dn10 = assign32650_e43281_d_n10;
        locals.var_t7_dn11 = assign32650_e43281_d_n11;
        locals.var_t7_dn12 = assign32650_e43281_d_n12;
        locals.var_t7_dn13 = assign32650_e43281_d_n13;
        locals.var_t7_dn14 = assign32650_e43281_d_n14;

        let (assign32660_e43304, assign32660_e43304_d_n0, assign32660_e43304_d_n2, assign32660_e43304_d_n3, assign32660_e43304_d_n4, assign32660_e43304_d_n5, assign32660_e43304_d_n6, assign32660_e43304_d_n7, assign32660_e43304_d_n8, assign32660_e43304_d_n9, assign32660_e43304_d_n10, assign32660_e43304_d_n11, assign32660_e43304_d_n12, assign32660_e43304_d_n13, assign32660_e43304_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32660_e43289: f64 = (locals.var_t4 / locals.var_t5);
        let assign32660_e43293: f64 = (locals.var_t4 * locals.var_t7);
        let assign32660_e43296: f64 = (2.0 * locals.var_t5);
        let assign32660_e43298: f64 = (assign32660_e43296 * locals.var_t5);
        let assign32660_e43299: f64 = (assign32660_e43293 / assign32660_e43298);
        let assign32660_e43300: f64 = (1.0 + assign32660_e43299);
        let assign32660_e43301: f64 = (assign32660_e43289 * assign32660_e43300);
        let assign32660_e43302: f64 = (locals.var_t3 - assign32660_e43301);
        (assign32660_e43302, (locals.var_t3_dn0 - (((((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn0 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn0)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn0) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn0)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn2 - (((((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn2 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn2)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn2) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn2)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn3)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn4)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn5)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn6)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn7)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn8)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn9)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn10)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn11)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn12 - (((((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn12 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn12)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn12) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn12)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn13 - (((((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn13 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn13)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn13) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn13)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn14 - (((((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn14 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn14)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn14) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn14)))) / (assign32660_e43298 * assign32660_e43298))))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn0, locals.var_qdeff_edge_dn2, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11, locals.var_qdeff_edge_dn12, locals.var_qdeff_edge_dn13, locals.var_qdeff_edge_dn14,)
    }
};
        locals.var_qdeff_edge = assign32660_e43304;
        locals.var_qdeff_edge_dn0 = assign32660_e43304_d_n0;
        locals.var_qdeff_edge_dn2 = assign32660_e43304_d_n2;
        locals.var_qdeff_edge_dn3 = assign32660_e43304_d_n3;
        locals.var_qdeff_edge_dn4 = assign32660_e43304_d_n4;
        locals.var_qdeff_edge_dn5 = assign32660_e43304_d_n5;
        locals.var_qdeff_edge_dn6 = assign32660_e43304_d_n6;
        locals.var_qdeff_edge_dn7 = assign32660_e43304_d_n7;
        locals.var_qdeff_edge_dn8 = assign32660_e43304_d_n8;
        locals.var_qdeff_edge_dn9 = assign32660_e43304_d_n9;
        locals.var_qdeff_edge_dn10 = assign32660_e43304_d_n10;
        locals.var_qdeff_edge_dn11 = assign32660_e43304_d_n11;
        locals.var_qdeff_edge_dn12 = assign32660_e43304_d_n12;
        locals.var_qdeff_edge_dn13 = assign32660_e43304_d_n13;
        locals.var_qdeff_edge_dn14 = assign32660_e43304_d_n14;

        let assign32670_e43310: f64 = (-2500.0);
        let assign32670_e43312: f64 = (assign32670_e43310 * 2.0);
        let assign32670_e43314: f64 = if ((1.0 == 0.0) && (locals.var_psip < assign32670_e43312)) { 1.0 } else { 0.0 };
        locals.var_guard743 = assign32670_e43314;

        let (assign32680_e43327, assign32680_e43327_d_n0, assign32680_e43327_d_n2, assign32680_e43327_d_n3, assign32680_e43327_d_n4, assign32680_e43327_d_n5, assign32680_e43327_d_n6, assign32680_e43327_d_n7, assign32680_e43327_d_n8, assign32680_e43327_d_n9, assign32680_e43327_d_n10, assign32680_e43327_d_n11, assign32680_e43327_d_n12, assign32680_e43327_d_n13, assign32680_e43327_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard743 != 0.0)) {
        let assign32680_e43319: f64 = (-2.0);
        let assign32680_e43321: f64 = (assign32680_e43319 * 2.0);
        let assign32680_e43324: f64 = (16.0 * locals.var_psip);
        let assign32680_e43325: f64 = (assign32680_e43321 / assign32680_e43324);
        (assign32680_e43325, (-((assign32680_e43321 * (16.0 * locals.var_psip_dn0)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn2)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn3)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn4)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn5)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn6)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn7)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn8)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn9)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn10)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn11)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn12)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn13)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn14)) / (assign32680_e43324 * assign32680_e43324))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn0, locals.var_psipclamp_dn2, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11, locals.var_psipclamp_dn12, locals.var_psipclamp_dn13, locals.var_psipclamp_dn14,)
    }
};
        locals.var_psipclamp = assign32680_e43327;
        locals.var_psipclamp_dn0 = assign32680_e43327_d_n0;
        locals.var_psipclamp_dn2 = assign32680_e43327_d_n2;
        locals.var_psipclamp_dn3 = assign32680_e43327_d_n3;
        locals.var_psipclamp_dn4 = assign32680_e43327_d_n4;
        locals.var_psipclamp_dn5 = assign32680_e43327_d_n5;
        locals.var_psipclamp_dn6 = assign32680_e43327_d_n6;
        locals.var_psipclamp_dn7 = assign32680_e43327_d_n7;
        locals.var_psipclamp_dn8 = assign32680_e43327_d_n8;
        locals.var_psipclamp_dn9 = assign32680_e43327_d_n9;
        locals.var_psipclamp_dn10 = assign32680_e43327_d_n10;
        locals.var_psipclamp_dn11 = assign32680_e43327_d_n11;
        locals.var_psipclamp_dn12 = assign32680_e43327_d_n12;
        locals.var_psipclamp_dn13 = assign32680_e43327_d_n13;
        locals.var_psipclamp_dn14 = assign32680_e43327_d_n14;

        let (assign32690_e43353, assign32690_e43353_d_n0, assign32690_e43353_d_n2, assign32690_e43353_d_n3, assign32690_e43353_d_n4, assign32690_e43353_d_n5, assign32690_e43353_d_n6, assign32690_e43353_d_n7, assign32690_e43353_d_n8, assign32690_e43353_d_n9, assign32690_e43353_d_n10, assign32690_e43353_d_n11, assign32690_e43353_d_n12, assign32690_e43353_d_n13, assign32690_e43353_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard743 == 0.0)) {
        let assign32690_e43335: f64 = (locals.var_psip + 1.0);
        let assign32690_e43338: f64 = (locals.var_psip - 1.0);
        let assign32690_e43341: f64 = (locals.var_psip - 1.0);
        let assign32690_e43342: f64 = (assign32690_e43338 * assign32690_e43341);
        let assign32690_e43345: f64 = (0.25 * 2.0);
        let assign32690_e43347: f64 = (assign32690_e43345 * 2.0);
        let assign32690_e43348: f64 = (assign32690_e43342 + assign32690_e43347);
        let assign32690_e43349: f64 = (assign32690_e43348).sqrt();
        let assign32690_e43350: f64 = (assign32690_e43335 + assign32690_e43349);
        let assign32690_e43351: f64 = (0.5 * assign32690_e43350);
        (assign32690_e43351, (0.5 * (locals.var_psip_dn0 + (((locals.var_psip_dn0 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn0)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn2 + (((locals.var_psip_dn2 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn2)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn3)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn4)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn5)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn6)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn7)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn8)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn9)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn10)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn11)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn12 + (((locals.var_psip_dn12 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn12)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn13 + (((locals.var_psip_dn13 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn13)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn14 + (((locals.var_psip_dn14 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn14)) / (2.0 * assign32690_e43349)))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn0, locals.var_psipclamp_dn2, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11, locals.var_psipclamp_dn12, locals.var_psipclamp_dn13, locals.var_psipclamp_dn14,)
    }
};
        locals.var_psipclamp = assign32690_e43353;
        locals.var_psipclamp_dn0 = assign32690_e43353_d_n0;
        locals.var_psipclamp_dn2 = assign32690_e43353_d_n2;
        locals.var_psipclamp_dn3 = assign32690_e43353_d_n3;
        locals.var_psipclamp_dn4 = assign32690_e43353_d_n4;
        locals.var_psipclamp_dn5 = assign32690_e43353_d_n5;
        locals.var_psipclamp_dn6 = assign32690_e43353_d_n6;
        locals.var_psipclamp_dn7 = assign32690_e43353_d_n7;
        locals.var_psipclamp_dn8 = assign32690_e43353_d_n8;
        locals.var_psipclamp_dn9 = assign32690_e43353_d_n9;
        locals.var_psipclamp_dn10 = assign32690_e43353_d_n10;
        locals.var_psipclamp_dn11 = assign32690_e43353_d_n11;
        locals.var_psipclamp_dn12 = assign32690_e43353_d_n12;
        locals.var_psipclamp_dn13 = assign32690_e43353_d_n13;
        locals.var_psipclamp_dn14 = assign32690_e43353_d_n14;

        let (assign32700_e43358, assign32700_e43358_d_n0, assign32700_e43358_d_n2, assign32700_e43358_d_n3, assign32700_e43358_d_n4, assign32700_e43358_d_n5, assign32700_e43358_d_n6, assign32700_e43358_d_n7, assign32700_e43358_d_n8, assign32700_e43358_d_n9, assign32700_e43358_d_n10, assign32700_e43358_d_n11, assign32700_e43358_d_n12, assign32700_e43358_d_n13, assign32700_e43358_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32700_e43356: f64 = (locals.var_psipclamp).sqrt();
        (assign32700_e43356, (locals.var_psipclamp_dn0 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn2 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn3 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn4 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn5 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn6 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn7 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn8 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn9 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn10 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn11 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn12 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn13 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn14 / (2.0 * assign32700_e43356)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    }
};
        locals.var_sqrtpsip = assign32700_e43358;
        locals.var_sqrtpsip_dn0 = assign32700_e43358_d_n0;
        locals.var_sqrtpsip_dn2 = assign32700_e43358_d_n2;
        locals.var_sqrtpsip_dn3 = assign32700_e43358_d_n3;
        locals.var_sqrtpsip_dn4 = assign32700_e43358_d_n4;
        locals.var_sqrtpsip_dn5 = assign32700_e43358_d_n5;
        locals.var_sqrtpsip_dn6 = assign32700_e43358_d_n6;
        locals.var_sqrtpsip_dn7 = assign32700_e43358_d_n7;
        locals.var_sqrtpsip_dn8 = assign32700_e43358_d_n8;
        locals.var_sqrtpsip_dn9 = assign32700_e43358_d_n9;
        locals.var_sqrtpsip_dn10 = assign32700_e43358_d_n10;
        locals.var_sqrtpsip_dn11 = assign32700_e43358_d_n11;
        locals.var_sqrtpsip_dn12 = assign32700_e43358_d_n12;
        locals.var_sqrtpsip_dn13 = assign32700_e43358_d_n13;
        locals.var_sqrtpsip_dn14 = assign32700_e43358_d_n14;

        let (assign32710_e43368, assign32710_e43368_d_n0, assign32710_e43368_d_n2, assign32710_e43368_d_n3, assign32710_e43368_d_n4, assign32710_e43368_d_n5, assign32710_e43368_d_n6, assign32710_e43368_d_n7, assign32710_e43368_d_n8, assign32710_e43368_d_n9, assign32710_e43368_d_n10, assign32710_e43368_d_n11, assign32710_e43368_d_n12, assign32710_e43368_d_n13, assign32710_e43368_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32710_e43362: f64 = (locals.var_psip - locals.var_qs_edge);
        let assign32710_e43364: f64 = (assign32710_e43362 - locals.var_qdeff_edge);
        let assign32710_e43366: f64 = (assign32710_e43364 - 1.0);
        (assign32710_e43366, ((locals.var_psip_dn0 - locals.var_qs_edge_dn0) - locals.var_qdeff_edge_dn0), ((locals.var_psip_dn2 - locals.var_qs_edge_dn2) - locals.var_qdeff_edge_dn2), ((locals.var_psip_dn3 - locals.var_qs_edge_dn3) - locals.var_qdeff_edge_dn3), ((locals.var_psip_dn4 - locals.var_qs_edge_dn4) - locals.var_qdeff_edge_dn4), ((locals.var_psip_dn5 - locals.var_qs_edge_dn5) - locals.var_qdeff_edge_dn5), ((locals.var_psip_dn6 - locals.var_qs_edge_dn6) - locals.var_qdeff_edge_dn6), ((locals.var_psip_dn7 - locals.var_qs_edge_dn7) - locals.var_qdeff_edge_dn7), ((locals.var_psip_dn8 - locals.var_qs_edge_dn8) - locals.var_qdeff_edge_dn8), ((locals.var_psip_dn9 - locals.var_qs_edge_dn9) - locals.var_qdeff_edge_dn9), ((locals.var_psip_dn10 - locals.var_qs_edge_dn10) - locals.var_qdeff_edge_dn10), ((locals.var_psip_dn11 - locals.var_qs_edge_dn11) - locals.var_qdeff_edge_dn11), ((locals.var_psip_dn12 - locals.var_qs_edge_dn12) - locals.var_qdeff_edge_dn12), ((locals.var_psip_dn13 - locals.var_qs_edge_dn13) - locals.var_qdeff_edge_dn13), ((locals.var_psip_dn14 - locals.var_qs_edge_dn14) - locals.var_qdeff_edge_dn14),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn0, locals.var_psiavg_dn2, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11, locals.var_psiavg_dn12, locals.var_psiavg_dn13, locals.var_psiavg_dn14,)
    }
};
        locals.var_psiavg = assign32710_e43368;
        locals.var_psiavg_dn0 = assign32710_e43368_d_n0;
        locals.var_psiavg_dn2 = assign32710_e43368_d_n2;
        locals.var_psiavg_dn3 = assign32710_e43368_d_n3;
        locals.var_psiavg_dn4 = assign32710_e43368_d_n4;
        locals.var_psiavg_dn5 = assign32710_e43368_d_n5;
        locals.var_psiavg_dn6 = assign32710_e43368_d_n6;
        locals.var_psiavg_dn7 = assign32710_e43368_d_n7;
        locals.var_psiavg_dn8 = assign32710_e43368_d_n8;
        locals.var_psiavg_dn9 = assign32710_e43368_d_n9;
        locals.var_psiavg_dn10 = assign32710_e43368_d_n10;
        locals.var_psiavg_dn11 = assign32710_e43368_d_n11;
        locals.var_psiavg_dn12 = assign32710_e43368_d_n12;
        locals.var_psiavg_dn13 = assign32710_e43368_d_n13;
        locals.var_psiavg_dn14 = assign32710_e43368_d_n14;

        let assign32720_e43374: f64 = (-2500.0);
        let assign32720_e43376: f64 = (assign32720_e43374 * 2.0);
        let assign32720_e43378: f64 = if ((1.0 == 0.0) && (locals.var_psiavg < assign32720_e43376)) { 1.0 } else { 0.0 };
        locals.var_guard744 = assign32720_e43378;

        let (assign32730_e43391, assign32730_e43391_d_n0, assign32730_e43391_d_n2, assign32730_e43391_d_n3, assign32730_e43391_d_n4, assign32730_e43391_d_n5, assign32730_e43391_d_n6, assign32730_e43391_d_n7, assign32730_e43391_d_n8, assign32730_e43391_d_n9, assign32730_e43391_d_n10, assign32730_e43391_d_n11, assign32730_e43391_d_n12, assign32730_e43391_d_n13, assign32730_e43391_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard744 != 0.0)) {
        let assign32730_e43383: f64 = (-2.0);
        let assign32730_e43385: f64 = (assign32730_e43383 * 2.0);
        let assign32730_e43388: f64 = (16.0 * locals.var_psiavg);
        let assign32730_e43389: f64 = (assign32730_e43385 / assign32730_e43388);
        (assign32730_e43389, (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn0)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn2)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn3)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn4)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn5)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn6)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn7)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn8)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn9)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn10)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn11)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn12)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn13)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn14)) / (assign32730_e43388 * assign32730_e43388))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32730_e43391;
        locals.var_t0_dn0 = assign32730_e43391_d_n0;
        locals.var_t0_dn2 = assign32730_e43391_d_n2;
        locals.var_t0_dn3 = assign32730_e43391_d_n3;
        locals.var_t0_dn4 = assign32730_e43391_d_n4;
        locals.var_t0_dn5 = assign32730_e43391_d_n5;
        locals.var_t0_dn6 = assign32730_e43391_d_n6;
        locals.var_t0_dn7 = assign32730_e43391_d_n7;
        locals.var_t0_dn8 = assign32730_e43391_d_n8;
        locals.var_t0_dn9 = assign32730_e43391_d_n9;
        locals.var_t0_dn10 = assign32730_e43391_d_n10;
        locals.var_t0_dn11 = assign32730_e43391_d_n11;
        locals.var_t0_dn12 = assign32730_e43391_d_n12;
        locals.var_t0_dn13 = assign32730_e43391_d_n13;
        locals.var_t0_dn14 = assign32730_e43391_d_n14;

        let (assign32740_e43417, assign32740_e43417_d_n0, assign32740_e43417_d_n2, assign32740_e43417_d_n3, assign32740_e43417_d_n4, assign32740_e43417_d_n5, assign32740_e43417_d_n6, assign32740_e43417_d_n7, assign32740_e43417_d_n8, assign32740_e43417_d_n9, assign32740_e43417_d_n10, assign32740_e43417_d_n11, assign32740_e43417_d_n12, assign32740_e43417_d_n13, assign32740_e43417_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard744 == 0.0)) {
        let assign32740_e43399: f64 = (locals.var_psiavg + 1.0);
        let assign32740_e43402: f64 = (locals.var_psiavg - 1.0);
        let assign32740_e43405: f64 = (locals.var_psiavg - 1.0);
        let assign32740_e43406: f64 = (assign32740_e43402 * assign32740_e43405);
        let assign32740_e43409: f64 = (0.25 * 2.0);
        let assign32740_e43411: f64 = (assign32740_e43409 * 2.0);
        let assign32740_e43412: f64 = (assign32740_e43406 + assign32740_e43411);
        let assign32740_e43413: f64 = (assign32740_e43412).sqrt();
        let assign32740_e43414: f64 = (assign32740_e43399 + assign32740_e43413);
        let assign32740_e43415: f64 = (0.5 * assign32740_e43414);
        (assign32740_e43415, (0.5 * (locals.var_psiavg_dn0 + (((locals.var_psiavg_dn0 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn0)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn2 + (((locals.var_psiavg_dn2 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn2)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn3)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn4)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn5)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn6)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn7)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn8)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn9)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn10)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn11)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn12 + (((locals.var_psiavg_dn12 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn12)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn13 + (((locals.var_psiavg_dn13 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn13)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn14 + (((locals.var_psiavg_dn14 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn14)) / (2.0 * assign32740_e43413)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32740_e43417;
        locals.var_t0_dn0 = assign32740_e43417_d_n0;
        locals.var_t0_dn2 = assign32740_e43417_d_n2;
        locals.var_t0_dn3 = assign32740_e43417_d_n3;
        locals.var_t0_dn4 = assign32740_e43417_d_n4;
        locals.var_t0_dn5 = assign32740_e43417_d_n5;
        locals.var_t0_dn6 = assign32740_e43417_d_n6;
        locals.var_t0_dn7 = assign32740_e43417_d_n7;
        locals.var_t0_dn8 = assign32740_e43417_d_n8;
        locals.var_t0_dn9 = assign32740_e43417_d_n9;
        locals.var_t0_dn10 = assign32740_e43417_d_n10;
        locals.var_t0_dn11 = assign32740_e43417_d_n11;
        locals.var_t0_dn12 = assign32740_e43417_d_n12;
        locals.var_t0_dn13 = assign32740_e43417_d_n13;
        locals.var_t0_dn14 = assign32740_e43417_d_n14;

        let (assign32750_e43422, assign32750_e43422_d_n0, assign32750_e43422_d_n2, assign32750_e43422_d_n3, assign32750_e43422_d_n4, assign32750_e43422_d_n5, assign32750_e43422_d_n6, assign32750_e43422_d_n7, assign32750_e43422_d_n8, assign32750_e43422_d_n9, assign32750_e43422_d_n10, assign32750_e43422_d_n11, assign32750_e43422_d_n12, assign32750_e43422_d_n13, assign32750_e43422_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32750_e43420: f64 = (locals.var_t0).sqrt();
        (assign32750_e43420, (locals.var_t0_dn0 / (2.0 * assign32750_e43420)), (locals.var_t0_dn2 / (2.0 * assign32750_e43420)), (locals.var_t0_dn3 / (2.0 * assign32750_e43420)), (locals.var_t0_dn4 / (2.0 * assign32750_e43420)), (locals.var_t0_dn5 / (2.0 * assign32750_e43420)), (locals.var_t0_dn6 / (2.0 * assign32750_e43420)), (locals.var_t0_dn7 / (2.0 * assign32750_e43420)), (locals.var_t0_dn8 / (2.0 * assign32750_e43420)), (locals.var_t0_dn9 / (2.0 * assign32750_e43420)), (locals.var_t0_dn10 / (2.0 * assign32750_e43420)), (locals.var_t0_dn11 / (2.0 * assign32750_e43420)), (locals.var_t0_dn12 / (2.0 * assign32750_e43420)), (locals.var_t0_dn13 / (2.0 * assign32750_e43420)), (locals.var_t0_dn14 / (2.0 * assign32750_e43420)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32750_e43422;
        locals.var_t2_dn0 = assign32750_e43422_d_n0;
        locals.var_t2_dn2 = assign32750_e43422_d_n2;
        locals.var_t2_dn3 = assign32750_e43422_d_n3;
        locals.var_t2_dn4 = assign32750_e43422_d_n4;
        locals.var_t2_dn5 = assign32750_e43422_d_n5;
        locals.var_t2_dn6 = assign32750_e43422_d_n6;
        locals.var_t2_dn7 = assign32750_e43422_d_n7;
        locals.var_t2_dn8 = assign32750_e43422_d_n8;
        locals.var_t2_dn9 = assign32750_e43422_d_n9;
        locals.var_t2_dn10 = assign32750_e43422_d_n10;
        locals.var_t2_dn11 = assign32750_e43422_d_n11;
        locals.var_t2_dn12 = assign32750_e43422_d_n12;
        locals.var_t2_dn13 = assign32750_e43422_d_n13;
        locals.var_t2_dn14 = assign32750_e43422_d_n14;

        let (assign32760_e43432, assign32760_e43432_d_n0, assign32760_e43432_d_n2, assign32760_e43432_d_n3, assign32760_e43432_d_n4, assign32760_e43432_d_n5, assign32760_e43432_d_n6, assign32760_e43432_d_n7, assign32760_e43432_d_n8, assign32760_e43432_d_n9, assign32760_e43432_d_n10, assign32760_e43432_d_n11, assign32760_e43432_d_n12, assign32760_e43432_d_n13, assign32760_e43432_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32760_e43428: f64 = (locals.var_sqrtpsip + locals.var_t2);
        let assign32760_e43429: f64 = (locals.var_gam_edge / assign32760_e43428);
        let assign32760_e43430: f64 = (1.0 + assign32760_e43429);
        (assign32760_e43430, (((locals.var_gam_edge_dn0 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn0 + locals.var_t2_dn0))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn2 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn2 + locals.var_t2_dn2))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn3 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn3 + locals.var_t2_dn3))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn4 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn4 + locals.var_t2_dn4))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn5 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn5 + locals.var_t2_dn5))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn6 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn6 + locals.var_t2_dn6))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn7 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn7 + locals.var_t2_dn7))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn8 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn8 + locals.var_t2_dn8))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn9 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn9 + locals.var_t2_dn9))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn10 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn10 + locals.var_t2_dn10))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn11 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn11 + locals.var_t2_dn11))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn12 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn12 + locals.var_t2_dn12))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn13 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn13 + locals.var_t2_dn13))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn14 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn14 + locals.var_t2_dn14))) / (assign32760_e43428 * assign32760_e43428)),)
    } else {
        (locals.var_nq_edge, locals.var_nq_edge_dn0, locals.var_nq_edge_dn2, locals.var_nq_edge_dn3, locals.var_nq_edge_dn4, locals.var_nq_edge_dn5, locals.var_nq_edge_dn6, locals.var_nq_edge_dn7, locals.var_nq_edge_dn8, locals.var_nq_edge_dn9, locals.var_nq_edge_dn10, locals.var_nq_edge_dn11, locals.var_nq_edge_dn12, locals.var_nq_edge_dn13, locals.var_nq_edge_dn14,)
    }
};
        locals.var_nq_edge = assign32760_e43432;
        locals.var_nq_edge_dn0 = assign32760_e43432_d_n0;
        locals.var_nq_edge_dn2 = assign32760_e43432_d_n2;
        locals.var_nq_edge_dn3 = assign32760_e43432_d_n3;
        locals.var_nq_edge_dn4 = assign32760_e43432_d_n4;
        locals.var_nq_edge_dn5 = assign32760_e43432_d_n5;
        locals.var_nq_edge_dn6 = assign32760_e43432_d_n6;
        locals.var_nq_edge_dn7 = assign32760_e43432_d_n7;
        locals.var_nq_edge_dn8 = assign32760_e43432_d_n8;
        locals.var_nq_edge_dn9 = assign32760_e43432_d_n9;
        locals.var_nq_edge_dn10 = assign32760_e43432_d_n10;
        locals.var_nq_edge_dn11 = assign32760_e43432_d_n11;
        locals.var_nq_edge_dn12 = assign32760_e43432_d_n12;
        locals.var_nq_edge_dn13 = assign32760_e43432_d_n13;
        locals.var_nq_edge_dn14 = assign32760_e43432_d_n14;

        let (assign32770_e43464, assign32770_e43464_d_n0, assign32770_e43464_d_n2, assign32770_e43464_d_n3, assign32770_e43464_d_n4, assign32770_e43464_d_n5, assign32770_e43464_d_n6, assign32770_e43464_d_n7, assign32770_e43464_d_n8, assign32770_e43464_d_n9, assign32770_e43464_d_n10, assign32770_e43464_d_n11, assign32770_e43464_d_n12, assign32770_e43464_d_n13, assign32770_e43464_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32770_e43436: f64 = (2.0 * p.p2);
        let assign32770_e43438: f64 = (assign32770_e43436 * locals.var_nq_edge);
        let assign32770_e43440: f64 = (assign32770_e43438 * locals.var_ueff);
        let assign32770_e43442: f64 = (assign32770_e43440 * p.p957);
        let assign32770_e43444: f64 = (assign32770_e43442 / locals.var_leff);
        let assign32770_e43446: f64 = (assign32770_e43444 * locals.var_cox);
        let assign32770_e43448: f64 = (assign32770_e43446 * locals.var_nvt);
        let assign32770_e43450: f64 = (assign32770_e43448 * locals.var_nvt);
        let assign32770_e43453: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign32770_e43456: f64 = (1.0 + locals.var_qs_edge);
        let assign32770_e43458: f64 = (assign32770_e43456 + locals.var_qdeff_edge);
        let assign32770_e43459: f64 = (assign32770_e43453 * assign32770_e43458);
        let assign32770_e43460: f64 = (assign32770_e43450 * assign32770_e43459);
        let assign32770_e43462: f64 = (assign32770_e43460 * locals.var_moc);
        (assign32770_e43462, ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn0) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn0)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn0)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn0)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn0 + locals.var_qdeff_edge_dn0))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn0)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn2) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn2)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn2)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn2)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn2 + locals.var_qdeff_edge_dn2))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn2)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn3) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn3)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn3)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn3)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn3 + locals.var_qdeff_edge_dn3))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn3)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn4) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn4)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn4)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn4)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn4 + locals.var_qdeff_edge_dn4))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn4)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn5) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn5)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn5)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn5)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn5 + locals.var_qdeff_edge_dn5))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn5)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn6) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn6)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn6)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn6)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn6 + locals.var_qdeff_edge_dn6))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn6)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn7) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn7)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn7)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn7)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn7 + locals.var_qdeff_edge_dn7))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn7)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn8) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn8)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn8)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn8)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn8 + locals.var_qdeff_edge_dn8))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn8)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn9) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn9)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn9)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn9)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn9 + locals.var_qdeff_edge_dn9))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn9)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn10) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn10)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn10)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn10)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn10 + locals.var_qdeff_edge_dn10))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn10)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn11) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn11)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn11)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn11)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn11 + locals.var_qdeff_edge_dn11))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn11)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn12) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn12)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn12)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn12)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn12 + locals.var_qdeff_edge_dn12))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn12)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn13) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn13)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn13)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn13)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn13 + locals.var_qdeff_edge_dn13))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn13)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn14) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn14)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn14)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn14)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn14 + locals.var_qdeff_edge_dn14))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn14)),)
    } else {
        (locals.var_ids_edge, locals.var_ids_edge_dn0, locals.var_ids_edge_dn2, locals.var_ids_edge_dn3, locals.var_ids_edge_dn4, locals.var_ids_edge_dn5, locals.var_ids_edge_dn6, locals.var_ids_edge_dn7, locals.var_ids_edge_dn8, locals.var_ids_edge_dn9, locals.var_ids_edge_dn10, locals.var_ids_edge_dn11, locals.var_ids_edge_dn12, locals.var_ids_edge_dn13, locals.var_ids_edge_dn14,)
    }
};
        locals.var_ids_edge = assign32770_e43464;
        locals.var_ids_edge_dn0 = assign32770_e43464_d_n0;
        locals.var_ids_edge_dn2 = assign32770_e43464_d_n2;
        locals.var_ids_edge_dn3 = assign32770_e43464_d_n3;
        locals.var_ids_edge_dn4 = assign32770_e43464_d_n4;
        locals.var_ids_edge_dn5 = assign32770_e43464_d_n5;
        locals.var_ids_edge_dn6 = assign32770_e43464_d_n6;
        locals.var_ids_edge_dn7 = assign32770_e43464_d_n7;
        locals.var_ids_edge_dn8 = assign32770_e43464_d_n8;
        locals.var_ids_edge_dn9 = assign32770_e43464_d_n9;
        locals.var_ids_edge_dn10 = assign32770_e43464_d_n10;
        locals.var_ids_edge_dn11 = assign32770_e43464_d_n11;
        locals.var_ids_edge_dn12 = assign32770_e43464_d_n12;
        locals.var_ids_edge_dn13 = assign32770_e43464_d_n13;
        locals.var_ids_edge_dn14 = assign32770_e43464_d_n14;

        let (assign32780_e43470, assign32780_e43470_d_n0, assign32780_e43470_d_n2, assign32780_e43470_d_n3, assign32780_e43470_d_n4, assign32780_e43470_d_n5, assign32780_e43470_d_n6, assign32780_e43470_d_n7, assign32780_e43470_d_n8, assign32780_e43470_d_n9, assign32780_e43470_d_n10, assign32780_e43470_d_n11, assign32780_e43470_d_n12, assign32780_e43470_d_n13, assign32780_e43470_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32780_e43468: f64 = (locals.var_ids_edge + locals.var_ids);
        (assign32780_e43468, (locals.var_ids_edge_dn0 + locals.var_ids_dn0), (locals.var_ids_edge_dn2 + locals.var_ids_dn2), (locals.var_ids_edge_dn3 + locals.var_ids_dn3), (locals.var_ids_edge_dn4 + locals.var_ids_dn4), (locals.var_ids_edge_dn5 + locals.var_ids_dn5), (locals.var_ids_edge_dn6 + locals.var_ids_dn6), (locals.var_ids_edge_dn7 + locals.var_ids_dn7), (locals.var_ids_edge_dn8 + locals.var_ids_dn8), (locals.var_ids_edge_dn9 + locals.var_ids_dn9), (locals.var_ids_edge_dn10 + locals.var_ids_dn10), (locals.var_ids_edge_dn11 + locals.var_ids_dn11), (locals.var_ids_edge_dn12 + locals.var_ids_dn12), (locals.var_ids_edge_dn13 + locals.var_ids_dn13), (locals.var_ids_edge_dn14 + locals.var_ids_dn14),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn13, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign32780_e43470;
        locals.var_ids_dn0 = assign32780_e43470_d_n0;
        locals.var_ids_dn2 = assign32780_e43470_d_n2;
        locals.var_ids_dn3 = assign32780_e43470_d_n3;
        locals.var_ids_dn4 = assign32780_e43470_d_n4;
        locals.var_ids_dn5 = assign32780_e43470_d_n5;
        locals.var_ids_dn6 = assign32780_e43470_d_n6;
        locals.var_ids_dn7 = assign32780_e43470_d_n7;
        locals.var_ids_dn8 = assign32780_e43470_d_n8;
        locals.var_ids_dn9 = assign32780_e43470_d_n9;
        locals.var_ids_dn10 = assign32780_e43470_d_n10;
        locals.var_ids_dn11 = assign32780_e43470_d_n11;
        locals.var_ids_dn12 = assign32780_e43470_d_n12;
        locals.var_ids_dn13 = assign32780_e43470_d_n13;
        locals.var_ids_dn14 = assign32780_e43470_d_n14;

        let (assign32790_e43476,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32790_e43474: f64 = (p.p785 * p.p1062);
        (assign32790_e43474,)
    } else {
        (locals.var_noia_edge,)
    }
};
        locals.var_noia_edge = assign32790_e43476;

        let (assign32800_e43482,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32800_e43480: f64 = (p.p799 * p.p1062);
        (assign32800_e43480,)
    } else {
        (locals.var_noib_edge,)
    }
};
        locals.var_noib_edge = assign32800_e43482;

        let (assign32810_e43488,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32810_e43486: f64 = (p.p800 * p.p1062);
        (assign32810_e43486,)
    } else {
        (locals.var_noic_edge,)
    }
};
        locals.var_noic_edge = assign32810_e43488;

        let (assign32820_e43496,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32820_e43493: f64 = (2.0 * locals.var_lintnoi_i);
        let assign32820_e43494: f64 = (locals.var_leff - assign32820_e43493);
        (assign32820_e43494,)
    } else {
        (locals.var_leffnoi_edge,)
    }
};
        locals.var_leffnoi_edge = assign32820_e43496;

        let (assign32830_e43502,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32830_e43500: f64 = (locals.var_leffnoi_edge * locals.var_leffnoi_edge);
        (assign32830_e43500,)
    } else {
        (locals.var_leffnoisq_edge,)
    }
};
        locals.var_leffnoisq_edge = assign32830_e43502;

    }

    pub(super) fn stamp_transient_block_106(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32840_e43514, assign32840_e43514_d_n0, assign32840_e43514_d_n2, assign32840_e43514_d_n3, assign32840_e43514_d_n4, assign32840_e43514_d_n5, assign32840_e43514_d_n6, assign32840_e43514_d_n7, assign32840_e43514_d_n8, assign32840_e43514_d_n9, assign32840_e43514_d_n10, assign32840_e43514_d_n11, assign32840_e43514_d_n12, assign32840_e43514_d_n13, assign32840_e43514_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32840_e43506: f64 = (locals.var_vt / 1.60219e-19);
        let assign32840_e43509: f64 = (locals.var_cox + locals.var_cdep);
        let assign32840_e43511: f64 = (assign32840_e43509 + locals.var_citedge_i);
        let assign32840_e43512: f64 = (assign32840_e43506 * assign32840_e43511);
        (assign32840_e43512, (assign32840_e43506 * locals.var_cdep_dn0), (assign32840_e43506 * locals.var_cdep_dn2), (assign32840_e43506 * locals.var_cdep_dn3), (((locals.var_vt_dn4 / 1.60219e-19) * assign32840_e43511) + (assign32840_e43506 * locals.var_cdep_dn4)), (assign32840_e43506 * locals.var_cdep_dn5), (assign32840_e43506 * locals.var_cdep_dn6), (assign32840_e43506 * locals.var_cdep_dn7), (assign32840_e43506 * locals.var_cdep_dn8), (assign32840_e43506 * locals.var_cdep_dn9), (assign32840_e43506 * locals.var_cdep_dn10), (assign32840_e43506 * locals.var_cdep_dn11), (assign32840_e43506 * locals.var_cdep_dn12), (assign32840_e43506 * locals.var_cdep_dn13), (assign32840_e43506 * locals.var_cdep_dn14),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn0, locals.var_nstar_dn2, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, locals.var_nstar_dn10, locals.var_nstar_dn11, locals.var_nstar_dn12, locals.var_nstar_dn13, locals.var_nstar_dn14,)
    }
};
        locals.var_nstar = assign32840_e43514;
        locals.var_nstar_dn0 = assign32840_e43514_d_n0;
        locals.var_nstar_dn2 = assign32840_e43514_d_n2;
        locals.var_nstar_dn3 = assign32840_e43514_d_n3;
        locals.var_nstar_dn4 = assign32840_e43514_d_n4;
        locals.var_nstar_dn5 = assign32840_e43514_d_n5;
        locals.var_nstar_dn6 = assign32840_e43514_d_n6;
        locals.var_nstar_dn7 = assign32840_e43514_d_n7;
        locals.var_nstar_dn8 = assign32840_e43514_d_n8;
        locals.var_nstar_dn9 = assign32840_e43514_d_n9;
        locals.var_nstar_dn10 = assign32840_e43514_d_n10;
        locals.var_nstar_dn11 = assign32840_e43514_d_n11;
        locals.var_nstar_dn12 = assign32840_e43514_d_n12;
        locals.var_nstar_dn13 = assign32840_e43514_d_n13;
        locals.var_nstar_dn14 = assign32840_e43514_d_n14;

        let (assign32850_e43528, assign32850_e43528_d_n0, assign32850_e43528_d_n2, assign32850_e43528_d_n3, assign32850_e43528_d_n4, assign32850_e43528_d_n5, assign32850_e43528_d_n6, assign32850_e43528_d_n7, assign32850_e43528_d_n8, assign32850_e43528_d_n9, assign32850_e43528_d_n10, assign32850_e43528_d_n11, assign32850_e43528_d_n12, assign32850_e43528_d_n13, assign32850_e43528_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32850_e43518: f64 = (2.0 * locals.var_nq_edge);
        let assign32850_e43520: f64 = (assign32850_e43518 * locals.var_cox);
        let assign32850_e43522: f64 = (assign32850_e43520 * locals.var_vt);
        let assign32850_e43524: f64 = (assign32850_e43522 * locals.var_qdeff_edge);
        let assign32850_e43526: f64 = (assign32850_e43524 / 1.60219e-19);
        (assign32850_e43526, ((((((2.0 * locals.var_nq_edge_dn0) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn0)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn2) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn2)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn3)) / 1.60219e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign32850_e43520 * locals.var_vt_dn4)) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn4)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn5)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn6)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn7)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn8)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn9)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn10)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn11)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn12) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn12)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn13) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn13)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn14) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn0, locals.var_nl_dn2, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8, locals.var_nl_dn9, locals.var_nl_dn10, locals.var_nl_dn11, locals.var_nl_dn12, locals.var_nl_dn13, locals.var_nl_dn14,)
    }
};
        locals.var_nl = assign32850_e43528;
        locals.var_nl_dn0 = assign32850_e43528_d_n0;
        locals.var_nl_dn2 = assign32850_e43528_d_n2;
        locals.var_nl_dn3 = assign32850_e43528_d_n3;
        locals.var_nl_dn4 = assign32850_e43528_d_n4;
        locals.var_nl_dn5 = assign32850_e43528_d_n5;
        locals.var_nl_dn6 = assign32850_e43528_d_n6;
        locals.var_nl_dn7 = assign32850_e43528_d_n7;
        locals.var_nl_dn8 = assign32850_e43528_d_n8;
        locals.var_nl_dn9 = assign32850_e43528_d_n9;
        locals.var_nl_dn10 = assign32850_e43528_d_n10;
        locals.var_nl_dn11 = assign32850_e43528_d_n11;
        locals.var_nl_dn12 = assign32850_e43528_d_n12;
        locals.var_nl_dn13 = assign32850_e43528_d_n13;
        locals.var_nl_dn14 = assign32850_e43528_d_n14;

        let (assign32860_e43543, assign32860_e43543_d_n0, assign32860_e43543_d_n2, assign32860_e43543_d_n3, assign32860_e43543_d_n4, assign32860_e43543_d_n5, assign32860_e43543_d_n6, assign32860_e43543_d_n7, assign32860_e43543_d_n8, assign32860_e43543_d_n9, assign32860_e43543_d_n10, assign32860_e43543_d_n11, assign32860_e43543_d_n12, assign32860_e43543_d_n13, assign32860_e43543_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32860_e43532: f64 = (1.60219e-19 * 1.60219e-19);
        let assign32860_e43534: f64 = (assign32860_e43532 * 1.60219e-19);
        let assign32860_e43536: f64 = (assign32860_e43534 * locals.var_vt);
        let assign32860_e43538: f64 = (locals.var_ids_edge).abs();
        let assign32860_e43539: f64 = (assign32860_e43536 * assign32860_e43538);
        let assign32860_e43541: f64 = (assign32860_e43539 * locals.var_ueff);
        (assign32860_e43541, (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn0 } else { (-locals.var_ids_edge_dn0) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn0)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn2 } else { (-locals.var_ids_edge_dn2) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn2)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn3 } else { (-locals.var_ids_edge_dn3) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn3)), (((((assign32860_e43534 * locals.var_vt_dn4) * assign32860_e43538) + (assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn4 } else { (-locals.var_ids_edge_dn4) })) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn4)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn5 } else { (-locals.var_ids_edge_dn5) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn5)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn6 } else { (-locals.var_ids_edge_dn6) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn6)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn7 } else { (-locals.var_ids_edge_dn7) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn7)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn8 } else { (-locals.var_ids_edge_dn8) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn8)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn9 } else { (-locals.var_ids_edge_dn9) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn9)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn10 } else { (-locals.var_ids_edge_dn10) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn10)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn11 } else { (-locals.var_ids_edge_dn11) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn11)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn12 } else { (-locals.var_ids_edge_dn12) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn12)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn13 } else { (-locals.var_ids_edge_dn13) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn13)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn14 } else { (-locals.var_ids_edge_dn14) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn14)),)
    } else {
        (locals.var_t0a, locals.var_t0a_dn0, locals.var_t0a_dn2, locals.var_t0a_dn3, locals.var_t0a_dn4, locals.var_t0a_dn5, locals.var_t0a_dn6, locals.var_t0a_dn7, locals.var_t0a_dn8, locals.var_t0a_dn9, locals.var_t0a_dn10, locals.var_t0a_dn11, locals.var_t0a_dn12, locals.var_t0a_dn13, locals.var_t0a_dn14,)
    }
};
        locals.var_t0a = assign32860_e43543;
        locals.var_t0a_dn0 = assign32860_e43543_d_n0;
        locals.var_t0a_dn2 = assign32860_e43543_d_n2;
        locals.var_t0a_dn3 = assign32860_e43543_d_n3;
        locals.var_t0a_dn4 = assign32860_e43543_d_n4;
        locals.var_t0a_dn5 = assign32860_e43543_d_n5;
        locals.var_t0a_dn6 = assign32860_e43543_d_n6;
        locals.var_t0a_dn7 = assign32860_e43543_d_n7;
        locals.var_t0a_dn8 = assign32860_e43543_d_n8;
        locals.var_t0a_dn9 = assign32860_e43543_d_n9;
        locals.var_t0a_dn10 = assign32860_e43543_d_n10;
        locals.var_t0a_dn11 = assign32860_e43543_d_n11;
        locals.var_t0a_dn12 = assign32860_e43543_d_n12;
        locals.var_t0a_dn13 = assign32860_e43543_d_n13;
        locals.var_t0a_dn14 = assign32860_e43543_d_n14;

        let (assign32870_e43553, assign32870_e43553_d_n0, assign32870_e43553_d_n2, assign32870_e43553_d_n3, assign32870_e43553_d_n4, assign32870_e43553_d_n5, assign32870_e43553_d_n6, assign32870_e43553_d_n7, assign32870_e43553_d_n8, assign32870_e43553_d_n9, assign32870_e43553_d_n10, assign32870_e43553_d_n11, assign32870_e43553_d_n12, assign32870_e43553_d_n13, assign32870_e43553_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32870_e43547: f64 = (1.60219e-19 * locals.var_vt);
        let assign32870_e43549: f64 = (assign32870_e43547 * locals.var_ids_edge);
        let assign32870_e43551: f64 = (assign32870_e43549 * locals.var_ids_edge);
        (assign32870_e43551, (((assign32870_e43547 * locals.var_ids_edge_dn0) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn0)), (((assign32870_e43547 * locals.var_ids_edge_dn2) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn2)), (((assign32870_e43547 * locals.var_ids_edge_dn3) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn3)), (((((1.60219e-19 * locals.var_vt_dn4) * locals.var_ids_edge) + (assign32870_e43547 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn4)), (((assign32870_e43547 * locals.var_ids_edge_dn5) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn5)), (((assign32870_e43547 * locals.var_ids_edge_dn6) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn6)), (((assign32870_e43547 * locals.var_ids_edge_dn7) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn7)), (((assign32870_e43547 * locals.var_ids_edge_dn8) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn8)), (((assign32870_e43547 * locals.var_ids_edge_dn9) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn9)), (((assign32870_e43547 * locals.var_ids_edge_dn10) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn10)), (((assign32870_e43547 * locals.var_ids_edge_dn11) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn11)), (((assign32870_e43547 * locals.var_ids_edge_dn12) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn12)), (((assign32870_e43547 * locals.var_ids_edge_dn13) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn13)), (((assign32870_e43547 * locals.var_ids_edge_dn14) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn14)),)
    } else {
        (locals.var_t0b, locals.var_t0b_dn0, locals.var_t0b_dn2, locals.var_t0b_dn3, locals.var_t0b_dn4, locals.var_t0b_dn5, locals.var_t0b_dn6, locals.var_t0b_dn7, locals.var_t0b_dn8, locals.var_t0b_dn9, locals.var_t0b_dn10, locals.var_t0b_dn11, locals.var_t0b_dn12, locals.var_t0b_dn13, locals.var_t0b_dn14,)
    }
};
        locals.var_t0b = assign32870_e43553;
        locals.var_t0b_dn0 = assign32870_e43553_d_n0;
        locals.var_t0b_dn2 = assign32870_e43553_d_n2;
        locals.var_t0b_dn3 = assign32870_e43553_d_n3;
        locals.var_t0b_dn4 = assign32870_e43553_d_n4;
        locals.var_t0b_dn5 = assign32870_e43553_d_n5;
        locals.var_t0b_dn6 = assign32870_e43553_d_n6;
        locals.var_t0b_dn7 = assign32870_e43553_d_n7;
        locals.var_t0b_dn8 = assign32870_e43553_d_n8;
        locals.var_t0b_dn9 = assign32870_e43553_d_n9;
        locals.var_t0b_dn10 = assign32870_e43553_d_n10;
        locals.var_t0b_dn11 = assign32870_e43553_d_n11;
        locals.var_t0b_dn12 = assign32870_e43553_d_n12;
        locals.var_t0b_dn13 = assign32870_e43553_d_n13;
        locals.var_t0b_dn14 = assign32870_e43553_d_n14;

        let (assign32880_e43567, assign32880_e43567_d_n0, assign32880_e43567_d_n2, assign32880_e43567_d_n3, assign32880_e43567_d_n4, assign32880_e43567_d_n5, assign32880_e43567_d_n6, assign32880_e43567_d_n7, assign32880_e43567_d_n8, assign32880_e43567_d_n9, assign32880_e43567_d_n10, assign32880_e43567_d_n11, assign32880_e43567_d_n12, assign32880_e43567_d_n13, assign32880_e43567_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32880_e43558: f64 = (locals.var_noib_edge * locals.var_nl);
        let assign32880_e43559: f64 = (locals.var_noia_edge + assign32880_e43558);
        let assign32880_e43562: f64 = (locals.var_noic_edge * locals.var_nl);
        let assign32880_e43564: f64 = (assign32880_e43562 * locals.var_nl);
        let assign32880_e43565: f64 = (assign32880_e43559 + assign32880_e43564);
        (assign32880_e43565, ((locals.var_noib_edge * locals.var_nl_dn0) + (((locals.var_noic_edge * locals.var_nl_dn0) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn0))), ((locals.var_noib_edge * locals.var_nl_dn2) + (((locals.var_noic_edge * locals.var_nl_dn2) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn2))), ((locals.var_noib_edge * locals.var_nl_dn3) + (((locals.var_noic_edge * locals.var_nl_dn3) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn3))), ((locals.var_noib_edge * locals.var_nl_dn4) + (((locals.var_noic_edge * locals.var_nl_dn4) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn4))), ((locals.var_noib_edge * locals.var_nl_dn5) + (((locals.var_noic_edge * locals.var_nl_dn5) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn5))), ((locals.var_noib_edge * locals.var_nl_dn6) + (((locals.var_noic_edge * locals.var_nl_dn6) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn6))), ((locals.var_noib_edge * locals.var_nl_dn7) + (((locals.var_noic_edge * locals.var_nl_dn7) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn7))), ((locals.var_noib_edge * locals.var_nl_dn8) + (((locals.var_noic_edge * locals.var_nl_dn8) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn8))), ((locals.var_noib_edge * locals.var_nl_dn9) + (((locals.var_noic_edge * locals.var_nl_dn9) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn9))), ((locals.var_noib_edge * locals.var_nl_dn10) + (((locals.var_noic_edge * locals.var_nl_dn10) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn10))), ((locals.var_noib_edge * locals.var_nl_dn11) + (((locals.var_noic_edge * locals.var_nl_dn11) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn11))), ((locals.var_noib_edge * locals.var_nl_dn12) + (((locals.var_noic_edge * locals.var_nl_dn12) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn12))), ((locals.var_noib_edge * locals.var_nl_dn13) + (((locals.var_noic_edge * locals.var_nl_dn13) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn13))), ((locals.var_noib_edge * locals.var_nl_dn14) + (((locals.var_noic_edge * locals.var_nl_dn14) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn14))),)
    } else {
        (locals.var_t0c, locals.var_t0c_dn0, locals.var_t0c_dn2, locals.var_t0c_dn3, locals.var_t0c_dn4, locals.var_t0c_dn5, locals.var_t0c_dn6, locals.var_t0c_dn7, locals.var_t0c_dn8, locals.var_t0c_dn9, locals.var_t0c_dn10, locals.var_t0c_dn11, locals.var_t0c_dn12, locals.var_t0c_dn13, locals.var_t0c_dn14,)
    }
};
        locals.var_t0c = assign32880_e43567;
        locals.var_t0c_dn0 = assign32880_e43567_d_n0;
        locals.var_t0c_dn2 = assign32880_e43567_d_n2;
        locals.var_t0c_dn3 = assign32880_e43567_d_n3;
        locals.var_t0c_dn4 = assign32880_e43567_d_n4;
        locals.var_t0c_dn5 = assign32880_e43567_d_n5;
        locals.var_t0c_dn6 = assign32880_e43567_d_n6;
        locals.var_t0c_dn7 = assign32880_e43567_d_n7;
        locals.var_t0c_dn8 = assign32880_e43567_d_n8;
        locals.var_t0c_dn9 = assign32880_e43567_d_n9;
        locals.var_t0c_dn10 = assign32880_e43567_d_n10;
        locals.var_t0c_dn11 = assign32880_e43567_d_n11;
        locals.var_t0c_dn12 = assign32880_e43567_d_n12;
        locals.var_t0c_dn13 = assign32880_e43567_d_n13;
        locals.var_t0c_dn14 = assign32880_e43567_d_n14;

        let (assign32890_e43577, assign32890_e43577_d_n0, assign32890_e43577_d_n2, assign32890_e43577_d_n3, assign32890_e43577_d_n4, assign32890_e43577_d_n5, assign32890_e43577_d_n6, assign32890_e43577_d_n7, assign32890_e43577_d_n8, assign32890_e43577_d_n9, assign32890_e43577_d_n10, assign32890_e43577_d_n11, assign32890_e43577_d_n12, assign32890_e43577_d_n13, assign32890_e43577_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32890_e43571: f64 = (locals.var_nl + locals.var_nstar);
        let assign32890_e43574: f64 = (locals.var_nl + locals.var_nstar);
        let assign32890_e43575: f64 = (assign32890_e43571 * assign32890_e43574);
        (assign32890_e43575, (((locals.var_nl_dn0 + locals.var_nstar_dn0) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn0 + locals.var_nstar_dn0))), (((locals.var_nl_dn2 + locals.var_nstar_dn2) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn2 + locals.var_nstar_dn2))), (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn8 + locals.var_nstar_dn8))), (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn9 + locals.var_nstar_dn9))), (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn10 + locals.var_nstar_dn10))), (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn11 + locals.var_nstar_dn11))), (((locals.var_nl_dn12 + locals.var_nstar_dn12) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn12 + locals.var_nstar_dn12))), (((locals.var_nl_dn13 + locals.var_nstar_dn13) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn13 + locals.var_nstar_dn13))), (((locals.var_nl_dn14 + locals.var_nstar_dn14) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn14 + locals.var_nstar_dn14))),)
    } else {
        (locals.var_t0d, locals.var_t0d_dn0, locals.var_t0d_dn2, locals.var_t0d_dn3, locals.var_t0d_dn4, locals.var_t0d_dn5, locals.var_t0d_dn6, locals.var_t0d_dn7, locals.var_t0d_dn8, locals.var_t0d_dn9, locals.var_t0d_dn10, locals.var_t0d_dn11, locals.var_t0d_dn12, locals.var_t0d_dn13, locals.var_t0d_dn14,)
    }
};
        locals.var_t0d = assign32890_e43577;
        locals.var_t0d_dn0 = assign32890_e43577_d_n0;
        locals.var_t0d_dn2 = assign32890_e43577_d_n2;
        locals.var_t0d_dn3 = assign32890_e43577_d_n3;
        locals.var_t0d_dn4 = assign32890_e43577_d_n4;
        locals.var_t0d_dn5 = assign32890_e43577_d_n5;
        locals.var_t0d_dn6 = assign32890_e43577_d_n6;
        locals.var_t0d_dn7 = assign32890_e43577_d_n7;
        locals.var_t0d_dn8 = assign32890_e43577_d_n8;
        locals.var_t0d_dn9 = assign32890_e43577_d_n9;
        locals.var_t0d_dn10 = assign32890_e43577_d_n10;
        locals.var_t0d_dn11 = assign32890_e43577_d_n11;
        locals.var_t0d_dn12 = assign32890_e43577_d_n12;
        locals.var_t0d_dn13 = assign32890_e43577_d_n13;
        locals.var_t0d_dn14 = assign32890_e43577_d_n14;

        let (assign32900_e43585, assign32900_e43585_d_n0, assign32900_e43585_d_n2, assign32900_e43585_d_n3, assign32900_e43585_d_n4, assign32900_e43585_d_n5, assign32900_e43585_d_n6, assign32900_e43585_d_n7, assign32900_e43585_d_n8, assign32900_e43585_d_n9, assign32900_e43585_d_n10, assign32900_e43585_d_n11, assign32900_e43585_d_n12, assign32900_e43585_d_n13, assign32900_e43585_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32900_e43581: f64 = (locals.var_noia_edge * 1.60219e-19);
        let assign32900_e43583: f64 = (assign32900_e43581 * locals.var_vt);
        (assign32900_e43583, 0.0, 0.0, 0.0, (assign32900_e43581 * locals.var_vt_dn4), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0e, locals.var_t0e_dn0, locals.var_t0e_dn2, locals.var_t0e_dn3, locals.var_t0e_dn4, locals.var_t0e_dn5, locals.var_t0e_dn6, locals.var_t0e_dn7, locals.var_t0e_dn8, locals.var_t0e_dn9, locals.var_t0e_dn10, locals.var_t0e_dn11, locals.var_t0e_dn12, locals.var_t0e_dn13, locals.var_t0e_dn14,)
    }
};
        locals.var_t0e = assign32900_e43585;
        locals.var_t0e_dn0 = assign32900_e43585_d_n0;
        locals.var_t0e_dn2 = assign32900_e43585_d_n2;
        locals.var_t0e_dn3 = assign32900_e43585_d_n3;
        locals.var_t0e_dn4 = assign32900_e43585_d_n4;
        locals.var_t0e_dn5 = assign32900_e43585_d_n5;
        locals.var_t0e_dn6 = assign32900_e43585_d_n6;
        locals.var_t0e_dn7 = assign32900_e43585_d_n7;
        locals.var_t0e_dn8 = assign32900_e43585_d_n8;
        locals.var_t0e_dn9 = assign32900_e43585_d_n9;
        locals.var_t0e_dn10 = assign32900_e43585_d_n10;
        locals.var_t0e_dn11 = assign32900_e43585_d_n11;
        locals.var_t0e_dn12 = assign32900_e43585_d_n12;
        locals.var_t0e_dn13 = assign32900_e43585_d_n13;
        locals.var_t0e_dn14 = assign32900_e43585_d_n14;

        let (assign32910_e43599, assign32910_e43599_d_n0, assign32910_e43599_d_n2, assign32910_e43599_d_n3, assign32910_e43599_d_n4, assign32910_e43599_d_n5, assign32910_e43599_d_n6, assign32910_e43599_d_n7, assign32910_e43599_d_n8, assign32910_e43599_d_n9, assign32910_e43599_d_n10, assign32910_e43599_d_n11, assign32910_e43599_d_n12, assign32910_e43599_d_n13, assign32910_e43599_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32910_e43589: f64 = (2.0 * locals.var_nq_edge);
        let assign32910_e43591: f64 = (assign32910_e43589 * locals.var_cox);
        let assign32910_e43593: f64 = (assign32910_e43591 * locals.var_vt);
        let assign32910_e43595: f64 = (assign32910_e43593 * locals.var_qs_edge);
        let assign32910_e43597: f64 = (assign32910_e43595 / 1.60219e-19);
        (assign32910_e43597, ((((((2.0 * locals.var_nq_edge_dn0) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn0)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn2) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn2)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn3)) / 1.60219e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign32910_e43591 * locals.var_vt_dn4)) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn4)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn5)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn6)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn7)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn8)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn9)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn10)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn11)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn12) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn12)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn13) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn13)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn14) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn0, locals.var_n0_dn2, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11, locals.var_n0_dn12, locals.var_n0_dn13, locals.var_n0_dn14,)
    }
};
        locals.var_n0 = assign32910_e43599;
        locals.var_n0_dn0 = assign32910_e43599_d_n0;
        locals.var_n0_dn2 = assign32910_e43599_d_n2;
        locals.var_n0_dn3 = assign32910_e43599_d_n3;
        locals.var_n0_dn4 = assign32910_e43599_d_n4;
        locals.var_n0_dn5 = assign32910_e43599_d_n5;
        locals.var_n0_dn6 = assign32910_e43599_d_n6;
        locals.var_n0_dn7 = assign32910_e43599_d_n7;
        locals.var_n0_dn8 = assign32910_e43599_d_n8;
        locals.var_n0_dn9 = assign32910_e43599_d_n9;
        locals.var_n0_dn10 = assign32910_e43599_d_n10;
        locals.var_n0_dn11 = assign32910_e43599_d_n11;
        locals.var_n0_dn12 = assign32910_e43599_d_n12;
        locals.var_n0_dn13 = assign32910_e43599_d_n13;
        locals.var_n0_dn14 = assign32910_e43599_d_n14;

        let (assign32920_e43614, assign32920_e43614_d_n0, assign32920_e43614_d_n2, assign32920_e43614_d_n3, assign32920_e43614_d_n4, assign32920_e43614_d_n5, assign32920_e43614_d_n6, assign32920_e43614_d_n7, assign32920_e43614_d_n8, assign32920_e43614_d_n9, assign32920_e43614_d_n10, assign32920_e43614_d_n11, assign32920_e43614_d_n12, assign32920_e43614_d_n13, assign32920_e43614_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32920_e43604: f64 = (locals.var_n0 + locals.var_nstar);
        let assign32920_e43607: f64 = (locals.var_nl + locals.var_nstar);
        let assign32920_e43608: f64 = (assign32920_e43604 / assign32920_e43607);
        let assign32920_e43610: f64 = (assign32920_e43608).max(1e-38);
        let assign32920_e43611: f64 = (assign32920_e43610).ln();
        let assign32920_e43612: f64 = (locals.var_noia_edge * assign32920_e43611);
        (assign32920_e43612, (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn0 + locals.var_nstar_dn0) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn0 + locals.var_nstar_dn0))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn2 + locals.var_nstar_dn2) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn2 + locals.var_nstar_dn2))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn9 + locals.var_nstar_dn9) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn10 + locals.var_nstar_dn10) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn11 + locals.var_nstar_dn11) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn12 + locals.var_nstar_dn12) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn12 + locals.var_nstar_dn12))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn13 + locals.var_nstar_dn13) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn13 + locals.var_nstar_dn13))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn14 + locals.var_nstar_dn14) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn14 + locals.var_nstar_dn14))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32920_e43614;
        locals.var_t1_dn0 = assign32920_e43614_d_n0;
        locals.var_t1_dn2 = assign32920_e43614_d_n2;
        locals.var_t1_dn3 = assign32920_e43614_d_n3;
        locals.var_t1_dn4 = assign32920_e43614_d_n4;
        locals.var_t1_dn5 = assign32920_e43614_d_n5;
        locals.var_t1_dn6 = assign32920_e43614_d_n6;
        locals.var_t1_dn7 = assign32920_e43614_d_n7;
        locals.var_t1_dn8 = assign32920_e43614_d_n8;
        locals.var_t1_dn9 = assign32920_e43614_d_n9;
        locals.var_t1_dn10 = assign32920_e43614_d_n10;
        locals.var_t1_dn11 = assign32920_e43614_d_n11;
        locals.var_t1_dn12 = assign32920_e43614_d_n12;
        locals.var_t1_dn13 = assign32920_e43614_d_n13;
        locals.var_t1_dn14 = assign32920_e43614_d_n14;

        let (assign32930_e43622, assign32930_e43622_d_n0, assign32930_e43622_d_n2, assign32930_e43622_d_n3, assign32930_e43622_d_n4, assign32930_e43622_d_n5, assign32930_e43622_d_n6, assign32930_e43622_d_n7, assign32930_e43622_d_n8, assign32930_e43622_d_n9, assign32930_e43622_d_n10, assign32930_e43622_d_n11, assign32930_e43622_d_n12, assign32930_e43622_d_n13, assign32930_e43622_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32930_e43619: f64 = (locals.var_n0 - locals.var_nl);
        let assign32930_e43620: f64 = (locals.var_noib_edge * assign32930_e43619);
        (assign32930_e43620, (locals.var_noib_edge * (locals.var_n0_dn0 - locals.var_nl_dn0)), (locals.var_noib_edge * (locals.var_n0_dn2 - locals.var_nl_dn2)), (locals.var_noib_edge * (locals.var_n0_dn3 - locals.var_nl_dn3)), (locals.var_noib_edge * (locals.var_n0_dn4 - locals.var_nl_dn4)), (locals.var_noib_edge * (locals.var_n0_dn5 - locals.var_nl_dn5)), (locals.var_noib_edge * (locals.var_n0_dn6 - locals.var_nl_dn6)), (locals.var_noib_edge * (locals.var_n0_dn7 - locals.var_nl_dn7)), (locals.var_noib_edge * (locals.var_n0_dn8 - locals.var_nl_dn8)), (locals.var_noib_edge * (locals.var_n0_dn9 - locals.var_nl_dn9)), (locals.var_noib_edge * (locals.var_n0_dn10 - locals.var_nl_dn10)), (locals.var_noib_edge * (locals.var_n0_dn11 - locals.var_nl_dn11)), (locals.var_noib_edge * (locals.var_n0_dn12 - locals.var_nl_dn12)), (locals.var_noib_edge * (locals.var_n0_dn13 - locals.var_nl_dn13)), (locals.var_noib_edge * (locals.var_n0_dn14 - locals.var_nl_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32930_e43622;
        locals.var_t2_dn0 = assign32930_e43622_d_n0;
        locals.var_t2_dn2 = assign32930_e43622_d_n2;
        locals.var_t2_dn3 = assign32930_e43622_d_n3;
        locals.var_t2_dn4 = assign32930_e43622_d_n4;
        locals.var_t2_dn5 = assign32930_e43622_d_n5;
        locals.var_t2_dn6 = assign32930_e43622_d_n6;
        locals.var_t2_dn7 = assign32930_e43622_d_n7;
        locals.var_t2_dn8 = assign32930_e43622_d_n8;
        locals.var_t2_dn9 = assign32930_e43622_d_n9;
        locals.var_t2_dn10 = assign32930_e43622_d_n10;
        locals.var_t2_dn11 = assign32930_e43622_d_n11;
        locals.var_t2_dn12 = assign32930_e43622_d_n12;
        locals.var_t2_dn13 = assign32930_e43622_d_n13;
        locals.var_t2_dn14 = assign32930_e43622_d_n14;

        let (assign32940_e43636, assign32940_e43636_d_n0, assign32940_e43636_d_n2, assign32940_e43636_d_n3, assign32940_e43636_d_n4, assign32940_e43636_d_n5, assign32940_e43636_d_n6, assign32940_e43636_d_n7, assign32940_e43636_d_n8, assign32940_e43636_d_n9, assign32940_e43636_d_n10, assign32940_e43636_d_n11, assign32940_e43636_d_n12, assign32940_e43636_d_n13, assign32940_e43636_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32940_e43626: f64 = (0.5 * locals.var_noic_edge);
        let assign32940_e43629: f64 = (locals.var_n0 * locals.var_n0);
        let assign32940_e43632: f64 = (locals.var_nl * locals.var_nl);
        let assign32940_e43633: f64 = (assign32940_e43629 - assign32940_e43632);
        let assign32940_e43634: f64 = (assign32940_e43626 * assign32940_e43633);
        (assign32940_e43634, (assign32940_e43626 * (((locals.var_n0_dn0 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn0)) - ((locals.var_nl_dn0 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn0)))), (assign32940_e43626 * (((locals.var_n0_dn2 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn2)) - ((locals.var_nl_dn2 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn2)))), (assign32940_e43626 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign32940_e43626 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign32940_e43626 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign32940_e43626 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign32940_e43626 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign32940_e43626 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign32940_e43626 * (((locals.var_n0_dn9 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign32940_e43626 * (((locals.var_n0_dn10 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign32940_e43626 * (((locals.var_n0_dn11 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))), (assign32940_e43626 * (((locals.var_n0_dn12 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn12)) - ((locals.var_nl_dn12 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn12)))), (assign32940_e43626 * (((locals.var_n0_dn13 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn13)) - ((locals.var_nl_dn13 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn13)))), (assign32940_e43626 * (((locals.var_n0_dn14 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn14)) - ((locals.var_nl_dn14 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32940_e43636;
        locals.var_t3_dn0 = assign32940_e43636_d_n0;
        locals.var_t3_dn2 = assign32940_e43636_d_n2;
        locals.var_t3_dn3 = assign32940_e43636_d_n3;
        locals.var_t3_dn4 = assign32940_e43636_d_n4;
        locals.var_t3_dn5 = assign32940_e43636_d_n5;
        locals.var_t3_dn6 = assign32940_e43636_d_n6;
        locals.var_t3_dn7 = assign32940_e43636_d_n7;
        locals.var_t3_dn8 = assign32940_e43636_d_n8;
        locals.var_t3_dn9 = assign32940_e43636_d_n9;
        locals.var_t3_dn10 = assign32940_e43636_d_n10;
        locals.var_t3_dn11 = assign32940_e43636_d_n11;
        locals.var_t3_dn12 = assign32940_e43636_d_n12;
        locals.var_t3_dn13 = assign32940_e43636_d_n13;
        locals.var_t3_dn14 = assign32940_e43636_d_n14;

        let (assign32950_e43646, assign32950_e43646_d_n0, assign32950_e43646_d_n2, assign32950_e43646_d_n3, assign32950_e43646_d_n4, assign32950_e43646_d_n5, assign32950_e43646_d_n6, assign32950_e43646_d_n7, assign32950_e43646_d_n8, assign32950_e43646_d_n9, assign32950_e43646_d_n10, assign32950_e43646_d_n11, assign32950_e43646_d_n12, assign32950_e43646_d_n13, assign32950_e43646_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32950_e43640: f64 = (10000000000.0 * locals.var_leffnoisq_edge);
        let assign32950_e43642: f64 = (assign32950_e43640 * p.p957);
        let assign32950_e43644: f64 = (assign32950_e43642 * p.p2);
        (assign32950_e43644, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32950_e43646;
        locals.var_t4_dn0 = assign32950_e43646_d_n0;
        locals.var_t4_dn2 = assign32950_e43646_d_n2;
        locals.var_t4_dn3 = assign32950_e43646_d_n3;
        locals.var_t4_dn4 = assign32950_e43646_d_n4;
        locals.var_t4_dn5 = assign32950_e43646_d_n5;
        locals.var_t4_dn6 = assign32950_e43646_d_n6;
        locals.var_t4_dn7 = assign32950_e43646_d_n7;
        locals.var_t4_dn8 = assign32950_e43646_d_n8;
        locals.var_t4_dn9 = assign32950_e43646_d_n9;
        locals.var_t4_dn10 = assign32950_e43646_d_n10;
        locals.var_t4_dn11 = assign32950_e43646_d_n11;
        locals.var_t4_dn12 = assign32950_e43646_d_n12;
        locals.var_t4_dn13 = assign32950_e43646_d_n13;
        locals.var_t4_dn14 = assign32950_e43646_d_n14;

        let (assign32960_e43668, assign32960_e43668_d_n0, assign32960_e43668_d_n2, assign32960_e43668_d_n3, assign32960_e43668_d_n4, assign32960_e43668_d_n5, assign32960_e43668_d_n6, assign32960_e43668_d_n7, assign32960_e43668_d_n8, assign32960_e43668_d_n9, assign32960_e43668_d_n10, assign32960_e43668_d_n11, assign32960_e43668_d_n12, assign32960_e43668_d_n13, assign32960_e43668_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32960_e43650: f64 = (locals.var_t0a / locals.var_t0);
        let assign32960_e43653: f64 = (locals.var_t1 + locals.var_t2);
        let assign32960_e43655: f64 = (assign32960_e43653 + locals.var_t3);
        let assign32960_e43656: f64 = (assign32960_e43650 * assign32960_e43655);
        let assign32960_e43659: f64 = (locals.var_t0b / locals.var_t4);
        let assign32960_e43661: f64 = (assign32960_e43659 * locals.var_delclm);
        let assign32960_e43663: f64 = (assign32960_e43661 * locals.var_t0c);
        let assign32960_e43665: f64 = (assign32960_e43663 / locals.var_t0d);
        let assign32960_e43666: f64 = (assign32960_e43656 + assign32960_e43665);
        (assign32960_e43666, ((((((locals.var_t0a_dn0 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn0 + locals.var_t2_dn0) + locals.var_t3_dn0))) + ((((((((((locals.var_t0b_dn0 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn0)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn0)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn0)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn2 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn2 + locals.var_t2_dn2) + locals.var_t3_dn2))) + ((((((((((locals.var_t0b_dn2 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn2)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn2)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn2)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn3 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn3 + locals.var_t2_dn3) + locals.var_t3_dn3))) + ((((((((((locals.var_t0b_dn3 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn3)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn3)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn3)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn4 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn4 + locals.var_t2_dn4) + locals.var_t3_dn4))) + ((((((((((locals.var_t0b_dn4 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn4)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn4)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn4)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn5 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn5 + locals.var_t2_dn5) + locals.var_t3_dn5))) + ((((((((((locals.var_t0b_dn5 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn5)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn5)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn5)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn6 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn6 + locals.var_t2_dn6) + locals.var_t3_dn6))) + ((((((((((locals.var_t0b_dn6 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn6)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn6)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn6)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn7 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn7 + locals.var_t2_dn7) + locals.var_t3_dn7))) + ((((((((((locals.var_t0b_dn7 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn7)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn7)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn7)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn8 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn8 + locals.var_t2_dn8) + locals.var_t3_dn8))) + ((((((((((locals.var_t0b_dn8 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn8)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn8)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn8)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn9 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn9 + locals.var_t2_dn9) + locals.var_t3_dn9))) + ((((((((((locals.var_t0b_dn9 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn9)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn9)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn9)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn10 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn10 + locals.var_t2_dn10) + locals.var_t3_dn10))) + ((((((((((locals.var_t0b_dn10 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn10)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn10)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn10)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn11 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn11 + locals.var_t2_dn11) + locals.var_t3_dn11))) + ((((((((((locals.var_t0b_dn11 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn11)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn11)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn11)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn12 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn12 + locals.var_t2_dn12) + locals.var_t3_dn12))) + ((((((((((locals.var_t0b_dn12 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn12)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn12)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn12)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn13 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn13 + locals.var_t2_dn13) + locals.var_t3_dn13))) + ((((((((((locals.var_t0b_dn13 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn13)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn13)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn13)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn14 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn14 + locals.var_t2_dn14) + locals.var_t3_dn14))) + ((((((((((locals.var_t0b_dn14 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn14)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn14)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn14)) / (locals.var_t0d * locals.var_t0d))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn0, locals.var_ssi_dn2, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11, locals.var_ssi_dn12, locals.var_ssi_dn13, locals.var_ssi_dn14,)
    }
};
        locals.var_ssi = assign32960_e43668;
        locals.var_ssi_dn0 = assign32960_e43668_d_n0;
        locals.var_ssi_dn2 = assign32960_e43668_d_n2;
        locals.var_ssi_dn3 = assign32960_e43668_d_n3;
        locals.var_ssi_dn4 = assign32960_e43668_d_n4;
        locals.var_ssi_dn5 = assign32960_e43668_d_n5;
        locals.var_ssi_dn6 = assign32960_e43668_d_n6;
        locals.var_ssi_dn7 = assign32960_e43668_d_n7;
        locals.var_ssi_dn8 = assign32960_e43668_d_n8;
        locals.var_ssi_dn9 = assign32960_e43668_d_n9;
        locals.var_ssi_dn10 = assign32960_e43668_d_n10;
        locals.var_ssi_dn11 = assign32960_e43668_d_n11;
        locals.var_ssi_dn12 = assign32960_e43668_d_n12;
        locals.var_ssi_dn13 = assign32960_e43668_d_n13;
        locals.var_ssi_dn14 = assign32960_e43668_d_n14;

        let (assign32970_e43682, assign32970_e43682_d_n0, assign32970_e43682_d_n2, assign32970_e43682_d_n3, assign32970_e43682_d_n4, assign32970_e43682_d_n5, assign32970_e43682_d_n6, assign32970_e43682_d_n7, assign32970_e43682_d_n8, assign32970_e43682_d_n9, assign32970_e43682_d_n10, assign32970_e43682_d_n11, assign32970_e43682_d_n12, assign32970_e43682_d_n13, assign32970_e43682_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32970_e43672: f64 = (p.p957 * p.p2);
        let assign32970_e43674: f64 = (assign32970_e43672 * locals.var_leffnoi_edge);
        let assign32970_e43676: f64 = (assign32970_e43674 * 10000000000.0);
        let assign32970_e43678: f64 = (assign32970_e43676 * locals.var_nstar);
        let assign32970_e43680: f64 = (assign32970_e43678 * locals.var_nstar);
        (assign32970_e43680, (((assign32970_e43676 * locals.var_nstar_dn0) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn0)), (((assign32970_e43676 * locals.var_nstar_dn2) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn2)), (((assign32970_e43676 * locals.var_nstar_dn3) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn3)), (((assign32970_e43676 * locals.var_nstar_dn4) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn4)), (((assign32970_e43676 * locals.var_nstar_dn5) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn5)), (((assign32970_e43676 * locals.var_nstar_dn6) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn6)), (((assign32970_e43676 * locals.var_nstar_dn7) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn7)), (((assign32970_e43676 * locals.var_nstar_dn8) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn8)), (((assign32970_e43676 * locals.var_nstar_dn9) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn9)), (((assign32970_e43676 * locals.var_nstar_dn10) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn10)), (((assign32970_e43676 * locals.var_nstar_dn11) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn11)), (((assign32970_e43676 * locals.var_nstar_dn12) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn12)), (((assign32970_e43676 * locals.var_nstar_dn13) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn13)), (((assign32970_e43676 * locals.var_nstar_dn14) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32970_e43682;
        locals.var_t5_dn0 = assign32970_e43682_d_n0;
        locals.var_t5_dn2 = assign32970_e43682_d_n2;
        locals.var_t5_dn3 = assign32970_e43682_d_n3;
        locals.var_t5_dn4 = assign32970_e43682_d_n4;
        locals.var_t5_dn5 = assign32970_e43682_d_n5;
        locals.var_t5_dn6 = assign32970_e43682_d_n6;
        locals.var_t5_dn7 = assign32970_e43682_d_n7;
        locals.var_t5_dn8 = assign32970_e43682_d_n8;
        locals.var_t5_dn9 = assign32970_e43682_d_n9;
        locals.var_t5_dn10 = assign32970_e43682_d_n10;
        locals.var_t5_dn11 = assign32970_e43682_d_n11;
        locals.var_t5_dn12 = assign32970_e43682_d_n12;
        locals.var_t5_dn13 = assign32970_e43682_d_n13;
        locals.var_t5_dn14 = assign32970_e43682_d_n14;

        let (assign32980_e43692, assign32980_e43692_d_n0, assign32980_e43692_d_n2, assign32980_e43692_d_n3, assign32980_e43692_d_n4, assign32980_e43692_d_n5, assign32980_e43692_d_n6, assign32980_e43692_d_n7, assign32980_e43692_d_n8, assign32980_e43692_d_n9, assign32980_e43692_d_n10, assign32980_e43692_d_n11, assign32980_e43692_d_n12, assign32980_e43692_d_n13, assign32980_e43692_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32980_e43686: f64 = (locals.var_t0e / locals.var_t5);
        let assign32980_e43688: f64 = (assign32980_e43686 * locals.var_ids_edge);
        let assign32980_e43690: f64 = (assign32980_e43688 * locals.var_ids_edge);
        (assign32980_e43690, (((((((locals.var_t0e_dn0 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn0)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn0)), (((((((locals.var_t0e_dn2 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn2)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn2)), (((((((locals.var_t0e_dn3 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn3)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn3)), (((((((locals.var_t0e_dn4 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn4)), (((((((locals.var_t0e_dn5 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn5)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn5)), (((((((locals.var_t0e_dn6 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn6)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn6)), (((((((locals.var_t0e_dn7 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn7)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn7)), (((((((locals.var_t0e_dn8 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn8)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn8)), (((((((locals.var_t0e_dn9 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn9)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn9)), (((((((locals.var_t0e_dn10 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn10)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn10)), (((((((locals.var_t0e_dn11 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn11)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn11)), (((((((locals.var_t0e_dn12 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn12)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn12)), (((((((locals.var_t0e_dn13 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn13)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn13)), (((((((locals.var_t0e_dn14 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn14)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn14)),)
    } else {
        (locals.var_swi, locals.var_swi_dn0, locals.var_swi_dn2, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11, locals.var_swi_dn12, locals.var_swi_dn13, locals.var_swi_dn14,)
    }
};
        locals.var_swi = assign32980_e43692;
        locals.var_swi_dn0 = assign32980_e43692_d_n0;
        locals.var_swi_dn2 = assign32980_e43692_d_n2;
        locals.var_swi_dn3 = assign32980_e43692_d_n3;
        locals.var_swi_dn4 = assign32980_e43692_d_n4;
        locals.var_swi_dn5 = assign32980_e43692_d_n5;
        locals.var_swi_dn6 = assign32980_e43692_d_n6;
        locals.var_swi_dn7 = assign32980_e43692_d_n7;
        locals.var_swi_dn8 = assign32980_e43692_d_n8;
        locals.var_swi_dn9 = assign32980_e43692_d_n9;
        locals.var_swi_dn10 = assign32980_e43692_d_n10;
        locals.var_swi_dn11 = assign32980_e43692_d_n11;
        locals.var_swi_dn12 = assign32980_e43692_d_n12;
        locals.var_swi_dn13 = assign32980_e43692_d_n13;
        locals.var_swi_dn14 = assign32980_e43692_d_n14;

        let (assign32990_e43698, assign32990_e43698_d_n0, assign32990_e43698_d_n2, assign32990_e43698_d_n3, assign32990_e43698_d_n4, assign32990_e43698_d_n5, assign32990_e43698_d_n6, assign32990_e43698_d_n7, assign32990_e43698_d_n8, assign32990_e43698_d_n9, assign32990_e43698_d_n10, assign32990_e43698_d_n11, assign32990_e43698_d_n12, assign32990_e43698_d_n13, assign32990_e43698_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32990_e43696: f64 = (locals.var_swi + locals.var_ssi);
        (assign32990_e43696, (locals.var_swi_dn0 + locals.var_ssi_dn0), (locals.var_swi_dn2 + locals.var_ssi_dn2), (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11), (locals.var_swi_dn12 + locals.var_ssi_dn12), (locals.var_swi_dn13 + locals.var_ssi_dn13), (locals.var_swi_dn14 + locals.var_ssi_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32990_e43698;
        locals.var_t6_dn0 = assign32990_e43698_d_n0;
        locals.var_t6_dn2 = assign32990_e43698_d_n2;
        locals.var_t6_dn3 = assign32990_e43698_d_n3;
        locals.var_t6_dn4 = assign32990_e43698_d_n4;
        locals.var_t6_dn5 = assign32990_e43698_d_n5;
        locals.var_t6_dn6 = assign32990_e43698_d_n6;
        locals.var_t6_dn7 = assign32990_e43698_d_n7;
        locals.var_t6_dn8 = assign32990_e43698_d_n8;
        locals.var_t6_dn9 = assign32990_e43698_d_n9;
        locals.var_t6_dn10 = assign32990_e43698_d_n10;
        locals.var_t6_dn11 = assign32990_e43698_d_n11;
        locals.var_t6_dn12 = assign32990_e43698_d_n12;
        locals.var_t6_dn13 = assign32990_e43698_d_n13;
        locals.var_t6_dn14 = assign32990_e43698_d_n14;

        let assign33000_e43701: f64 = if locals.var_t6 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign33000_e43701;

        let (assign33010_e43711, assign33010_e43711_d_n0, assign33010_e43711_d_n2, assign33010_e43711_d_n3, assign33010_e43711_d_n4, assign33010_e43711_d_n5, assign33010_e43711_d_n6, assign33010_e43711_d_n7, assign33010_e43711_d_n8, assign33010_e43711_d_n9, assign33010_e43711_d_n10, assign33010_e43711_d_n11, assign33010_e43711_d_n12, assign33010_e43711_d_n13, assign33010_e43711_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard745 != 0.0)) {
        let assign33010_e43707: f64 = (locals.var_ssi * locals.var_swi);
        let assign33010_e43709: f64 = (assign33010_e43707 / locals.var_t6);
        (assign33010_e43709, (((((locals.var_ssi_dn0 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn0)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn2 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn2)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn3 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn3)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn3)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn4 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn4)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn5 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn5)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn6 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn6)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn7 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn7)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn8 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn8)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn9 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn9)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn10 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn10)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn11 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn11)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn12 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn12)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn12)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn13 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn13)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn13)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn14 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn14)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign33010_e43711;
        locals.var_t7_dn0 = assign33010_e43711_d_n0;
        locals.var_t7_dn2 = assign33010_e43711_d_n2;
        locals.var_t7_dn3 = assign33010_e43711_d_n3;
        locals.var_t7_dn4 = assign33010_e43711_d_n4;
        locals.var_t7_dn5 = assign33010_e43711_d_n5;
        locals.var_t7_dn6 = assign33010_e43711_d_n6;
        locals.var_t7_dn7 = assign33010_e43711_d_n7;
        locals.var_t7_dn8 = assign33010_e43711_d_n8;
        locals.var_t7_dn9 = assign33010_e43711_d_n9;
        locals.var_t7_dn10 = assign33010_e43711_d_n10;
        locals.var_t7_dn11 = assign33010_e43711_d_n11;
        locals.var_t7_dn12 = assign33010_e43711_d_n12;
        locals.var_t7_dn13 = assign33010_e43711_d_n13;
        locals.var_t7_dn14 = assign33010_e43711_d_n14;

        let (assign33020_e43725, assign33020_e43725_d_n0, assign33020_e43725_d_n2, assign33020_e43725_d_n3, assign33020_e43725_d_n4, assign33020_e43725_d_n5, assign33020_e43725_d_n6, assign33020_e43725_d_n7, assign33020_e43725_d_n8, assign33020_e43725_d_n9, assign33020_e43725_d_n10, assign33020_e43725_d_n11, assign33020_e43725_d_n12, assign33020_e43725_d_n13, assign33020_e43725_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard745 != 0.0)) {
        let assign33020_e43719: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign33020_e43721: f64 = (assign33020_e43719).powf(p.p1064);
        let assign33020_e43722: f64 = (p.p1063 * assign33020_e43721);
        let assign33020_e43723: f64 = (1.0 + assign33020_e43722);
        (assign33020_e43723, (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14) / assign33020_e43719))) }),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign33020_e43725;
        locals.var_t8_dn0 = assign33020_e43725_d_n0;
        locals.var_t8_dn2 = assign33020_e43725_d_n2;
        locals.var_t8_dn3 = assign33020_e43725_d_n3;
        locals.var_t8_dn4 = assign33020_e43725_d_n4;
        locals.var_t8_dn5 = assign33020_e43725_d_n5;
        locals.var_t8_dn6 = assign33020_e43725_d_n6;
        locals.var_t8_dn7 = assign33020_e43725_d_n7;
        locals.var_t8_dn8 = assign33020_e43725_d_n8;
        locals.var_t8_dn9 = assign33020_e43725_d_n9;
        locals.var_t8_dn10 = assign33020_e43725_d_n10;
        locals.var_t8_dn11 = assign33020_e43725_d_n11;
        locals.var_t8_dn12 = assign33020_e43725_d_n12;
        locals.var_t8_dn13 = assign33020_e43725_d_n13;
        locals.var_t8_dn14 = assign33020_e43725_d_n14;

        let assign33060_e43756: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign33060_e43756;

    }

    pub(super) fn stamp_transient_block_107(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (assign33070_e43764, assign33070_e43764_d_n0, assign33070_e43764_d_n2, assign33070_e43764_d_n3, assign33070_e43764_d_n4, assign33070_e43764_d_n5, assign33070_e43764_d_n6, assign33070_e43764_d_n7, assign33070_e43764_d_n8, assign33070_e43764_d_n9, assign33070_e43764_d_n10, assign33070_e43764_d_n11, assign33070_e43764_d_n12, assign33070_e43764_d_n13, assign33070_e43764_d_n14,) = {
    if (locals.var_guard746 != 0.0) {
        let assign33070_e43760: f64 = (locals.var_devsign * p.p29);
        let assign33070_e43762: f64 = (assign33070_e43760 * locals.var_qsi);
        (assign33070_e43762, (assign33070_e43760 * locals.var_qsi_dn0), (assign33070_e43760 * locals.var_qsi_dn2), (assign33070_e43760 * locals.var_qsi_dn3), (assign33070_e43760 * locals.var_qsi_dn4), (assign33070_e43760 * locals.var_qsi_dn5), (assign33070_e43760 * locals.var_qsi_dn6), (assign33070_e43760 * locals.var_qsi_dn7), (assign33070_e43760 * locals.var_qsi_dn8), (assign33070_e43760 * locals.var_qsi_dn9), (assign33070_e43760 * locals.var_qsi_dn10), (assign33070_e43760 * locals.var_qsi_dn11), (assign33070_e43760 * locals.var_qsi_dn12), (assign33070_e43760 * locals.var_qsi_dn13), (assign33070_e43760 * locals.var_qsi_dn14),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn0, locals.var_qsi_1_dn2, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11, locals.var_qsi_1_dn12, locals.var_qsi_1_dn13, locals.var_qsi_1_dn14,)
    }
};
        locals.var_qsi_1 = assign33070_e43764;
        locals.var_qsi_1_dn0 = assign33070_e43764_d_n0;
        locals.var_qsi_1_dn2 = assign33070_e43764_d_n2;
        locals.var_qsi_1_dn3 = assign33070_e43764_d_n3;
        locals.var_qsi_1_dn4 = assign33070_e43764_d_n4;
        locals.var_qsi_1_dn5 = assign33070_e43764_d_n5;
        locals.var_qsi_1_dn6 = assign33070_e43764_d_n6;
        locals.var_qsi_1_dn7 = assign33070_e43764_d_n7;
        locals.var_qsi_1_dn8 = assign33070_e43764_d_n8;
        locals.var_qsi_1_dn9 = assign33070_e43764_d_n9;
        locals.var_qsi_1_dn10 = assign33070_e43764_d_n10;
        locals.var_qsi_1_dn11 = assign33070_e43764_d_n11;
        locals.var_qsi_1_dn12 = assign33070_e43764_d_n12;
        locals.var_qsi_1_dn13 = assign33070_e43764_d_n13;
        locals.var_qsi_1_dn14 = assign33070_e43764_d_n14;

        let (assign33080_e43772, assign33080_e43772_d_n0, assign33080_e43772_d_n2, assign33080_e43772_d_n3, assign33080_e43772_d_n4, assign33080_e43772_d_n5, assign33080_e43772_d_n6, assign33080_e43772_d_n7, assign33080_e43772_d_n8, assign33080_e43772_d_n9, assign33080_e43772_d_n10, assign33080_e43772_d_n11, assign33080_e43772_d_n12, assign33080_e43772_d_n13, assign33080_e43772_d_n14,) = {
    if (locals.var_guard746 != 0.0) {
        let assign33080_e43768: f64 = (locals.var_devsign * p.p29);
        let assign33080_e43770: f64 = (assign33080_e43768 * locals.var_qdi);
        (assign33080_e43770, (assign33080_e43768 * locals.var_qdi_dn0), (assign33080_e43768 * locals.var_qdi_dn2), (assign33080_e43768 * locals.var_qdi_dn3), (assign33080_e43768 * locals.var_qdi_dn4), (assign33080_e43768 * locals.var_qdi_dn5), (assign33080_e43768 * locals.var_qdi_dn6), (assign33080_e43768 * locals.var_qdi_dn7), (assign33080_e43768 * locals.var_qdi_dn8), (assign33080_e43768 * locals.var_qdi_dn9), (assign33080_e43768 * locals.var_qdi_dn10), (assign33080_e43768 * locals.var_qdi_dn11), (assign33080_e43768 * locals.var_qdi_dn12), (assign33080_e43768 * locals.var_qdi_dn13), (assign33080_e43768 * locals.var_qdi_dn14),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn0, locals.var_qdi_1_dn2, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11, locals.var_qdi_1_dn12, locals.var_qdi_1_dn13, locals.var_qdi_1_dn14,)
    }
};
        locals.var_qdi_1 = assign33080_e43772;
        locals.var_qdi_1_dn0 = assign33080_e43772_d_n0;
        locals.var_qdi_1_dn2 = assign33080_e43772_d_n2;
        locals.var_qdi_1_dn3 = assign33080_e43772_d_n3;
        locals.var_qdi_1_dn4 = assign33080_e43772_d_n4;
        locals.var_qdi_1_dn5 = assign33080_e43772_d_n5;
        locals.var_qdi_1_dn6 = assign33080_e43772_d_n6;
        locals.var_qdi_1_dn7 = assign33080_e43772_d_n7;
        locals.var_qdi_1_dn8 = assign33080_e43772_d_n8;
        locals.var_qdi_1_dn9 = assign33080_e43772_d_n9;
        locals.var_qdi_1_dn10 = assign33080_e43772_d_n10;
        locals.var_qdi_1_dn11 = assign33080_e43772_d_n11;
        locals.var_qdi_1_dn12 = assign33080_e43772_d_n12;
        locals.var_qdi_1_dn13 = assign33080_e43772_d_n13;
        locals.var_qdi_1_dn14 = assign33080_e43772_d_n14;

        let (assign33110_e43807, assign33110_e43807_d_n0, assign33110_e43807_d_n2, assign33110_e43807_d_n3, assign33110_e43807_d_n4, assign33110_e43807_d_n5, assign33110_e43807_d_n6, assign33110_e43807_d_n7, assign33110_e43807_d_n8, assign33110_e43807_d_n9, assign33110_e43807_d_n10, assign33110_e43807_d_n11, assign33110_e43807_d_n12, assign33110_e43807_d_n13, assign33110_e43807_d_n14,) = {
    if (locals.var_guard746 == 0.0) {
        let assign33110_e43803: f64 = (locals.var_devsign * p.p29);
        let assign33110_e43805: f64 = (assign33110_e43803 * locals.var_qdi);
        (assign33110_e43805, (assign33110_e43803 * locals.var_qdi_dn0), (assign33110_e43803 * locals.var_qdi_dn2), (assign33110_e43803 * locals.var_qdi_dn3), (assign33110_e43803 * locals.var_qdi_dn4), (assign33110_e43803 * locals.var_qdi_dn5), (assign33110_e43803 * locals.var_qdi_dn6), (assign33110_e43803 * locals.var_qdi_dn7), (assign33110_e43803 * locals.var_qdi_dn8), (assign33110_e43803 * locals.var_qdi_dn9), (assign33110_e43803 * locals.var_qdi_dn10), (assign33110_e43803 * locals.var_qdi_dn11), (assign33110_e43803 * locals.var_qdi_dn12), (assign33110_e43803 * locals.var_qdi_dn13), (assign33110_e43803 * locals.var_qdi_dn14),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn0, locals.var_qsi_1_dn2, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11, locals.var_qsi_1_dn12, locals.var_qsi_1_dn13, locals.var_qsi_1_dn14,)
    }
};
        locals.var_qsi_1 = assign33110_e43807;
        locals.var_qsi_1_dn0 = assign33110_e43807_d_n0;
        locals.var_qsi_1_dn2 = assign33110_e43807_d_n2;
        locals.var_qsi_1_dn3 = assign33110_e43807_d_n3;
        locals.var_qsi_1_dn4 = assign33110_e43807_d_n4;
        locals.var_qsi_1_dn5 = assign33110_e43807_d_n5;
        locals.var_qsi_1_dn6 = assign33110_e43807_d_n6;
        locals.var_qsi_1_dn7 = assign33110_e43807_d_n7;
        locals.var_qsi_1_dn8 = assign33110_e43807_d_n8;
        locals.var_qsi_1_dn9 = assign33110_e43807_d_n9;
        locals.var_qsi_1_dn10 = assign33110_e43807_d_n10;
        locals.var_qsi_1_dn11 = assign33110_e43807_d_n11;
        locals.var_qsi_1_dn12 = assign33110_e43807_d_n12;
        locals.var_qsi_1_dn13 = assign33110_e43807_d_n13;
        locals.var_qsi_1_dn14 = assign33110_e43807_d_n14;

        let (assign33120_e43816, assign33120_e43816_d_n0, assign33120_e43816_d_n2, assign33120_e43816_d_n3, assign33120_e43816_d_n4, assign33120_e43816_d_n5, assign33120_e43816_d_n6, assign33120_e43816_d_n7, assign33120_e43816_d_n8, assign33120_e43816_d_n9, assign33120_e43816_d_n10, assign33120_e43816_d_n11, assign33120_e43816_d_n12, assign33120_e43816_d_n13, assign33120_e43816_d_n14,) = {
    if (locals.var_guard746 == 0.0) {
        let assign33120_e43812: f64 = (locals.var_devsign * p.p29);
        let assign33120_e43814: f64 = (assign33120_e43812 * locals.var_qsi);
        (assign33120_e43814, (assign33120_e43812 * locals.var_qsi_dn0), (assign33120_e43812 * locals.var_qsi_dn2), (assign33120_e43812 * locals.var_qsi_dn3), (assign33120_e43812 * locals.var_qsi_dn4), (assign33120_e43812 * locals.var_qsi_dn5), (assign33120_e43812 * locals.var_qsi_dn6), (assign33120_e43812 * locals.var_qsi_dn7), (assign33120_e43812 * locals.var_qsi_dn8), (assign33120_e43812 * locals.var_qsi_dn9), (assign33120_e43812 * locals.var_qsi_dn10), (assign33120_e43812 * locals.var_qsi_dn11), (assign33120_e43812 * locals.var_qsi_dn12), (assign33120_e43812 * locals.var_qsi_dn13), (assign33120_e43812 * locals.var_qsi_dn14),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn0, locals.var_qdi_1_dn2, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11, locals.var_qdi_1_dn12, locals.var_qdi_1_dn13, locals.var_qdi_1_dn14,)
    }
};
        locals.var_qdi_1 = assign33120_e43816;
        locals.var_qdi_1_dn0 = assign33120_e43816_d_n0;
        locals.var_qdi_1_dn2 = assign33120_e43816_d_n2;
        locals.var_qdi_1_dn3 = assign33120_e43816_d_n3;
        locals.var_qdi_1_dn4 = assign33120_e43816_d_n4;
        locals.var_qdi_1_dn5 = assign33120_e43816_d_n5;
        locals.var_qdi_1_dn6 = assign33120_e43816_d_n6;
        locals.var_qdi_1_dn7 = assign33120_e43816_d_n7;
        locals.var_qdi_1_dn8 = assign33120_e43816_d_n8;
        locals.var_qdi_1_dn9 = assign33120_e43816_d_n9;
        locals.var_qdi_1_dn10 = assign33120_e43816_d_n10;
        locals.var_qdi_1_dn11 = assign33120_e43816_d_n11;
        locals.var_qdi_1_dn12 = assign33120_e43816_d_n12;
        locals.var_qdi_1_dn13 = assign33120_e43816_d_n13;
        locals.var_qdi_1_dn14 = assign33120_e43816_d_n14;

        let assign33160_e43858: f64 = if ((p.p1094 == 1.0) && (p.p1095 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard747 = assign33160_e43858;

        let (assign33170_e43864, assign33170_e43864_d_n0, assign33170_e43864_d_n2, assign33170_e43864_d_n3, assign33170_e43864_d_n4, assign33170_e43864_d_n5, assign33170_e43864_d_n6, assign33170_e43864_d_n7, assign33170_e43864_d_n8, assign33170_e43864_d_n9, assign33170_e43864_d_n10, assign33170_e43864_d_n11, assign33170_e43864_d_n12, assign33170_e43864_d_n13, assign33170_e43864_d_n14,) = {
    if (locals.var_guard747 != 0.0) {
        let assign33170_e43862: f64 = (locals.var_qovb + locals.var_qiov);
        (assign33170_e43862, (locals.var_qovb_dn0 + locals.var_qiov_dn0), (locals.var_qovb_dn2 + locals.var_qiov_dn2), (locals.var_qovb_dn3 + locals.var_qiov_dn3), (locals.var_qovb_dn4 + locals.var_qiov_dn4), (locals.var_qovb_dn5 + locals.var_qiov_dn5), (locals.var_qovb_dn6 + locals.var_qiov_dn6), (locals.var_qovb_dn7 + locals.var_qiov_dn7), (locals.var_qovb_dn8 + locals.var_qiov_dn8), (locals.var_qovb_dn9 + locals.var_qiov_dn9), (locals.var_qovb_dn10 + locals.var_qiov_dn10), (locals.var_qovb_dn11 + locals.var_qiov_dn11), (locals.var_qovb_dn12 + locals.var_qiov_dn12), (locals.var_qovb_dn13 + locals.var_qiov_dn13), (locals.var_qovb_dn14 + locals.var_qiov_dn14),)
    } else {
        (locals.var_qovb, locals.var_qovb_dn0, locals.var_qovb_dn2, locals.var_qovb_dn3, locals.var_qovb_dn4, locals.var_qovb_dn5, locals.var_qovb_dn6, locals.var_qovb_dn7, locals.var_qovb_dn8, locals.var_qovb_dn9, locals.var_qovb_dn10, locals.var_qovb_dn11, locals.var_qovb_dn12, locals.var_qovb_dn13, locals.var_qovb_dn14,)
    }
};
        locals.var_qovb = assign33170_e43864;
        locals.var_qovb_dn0 = assign33170_e43864_d_n0;
        locals.var_qovb_dn2 = assign33170_e43864_d_n2;
        locals.var_qovb_dn3 = assign33170_e43864_d_n3;
        locals.var_qovb_dn4 = assign33170_e43864_d_n4;
        locals.var_qovb_dn5 = assign33170_e43864_d_n5;
        locals.var_qovb_dn6 = assign33170_e43864_d_n6;
        locals.var_qovb_dn7 = assign33170_e43864_d_n7;
        locals.var_qovb_dn8 = assign33170_e43864_d_n8;
        locals.var_qovb_dn9 = assign33170_e43864_d_n9;
        locals.var_qovb_dn10 = assign33170_e43864_d_n10;
        locals.var_qovb_dn11 = assign33170_e43864_d_n11;
        locals.var_qovb_dn12 = assign33170_e43864_d_n12;
        locals.var_qovb_dn13 = assign33170_e43864_d_n13;
        locals.var_qovb_dn14 = assign33170_e43864_d_n14;

        let (assign33180_e43870, assign33180_e43870_d_n0, assign33180_e43870_d_n2, assign33180_e43870_d_n3, assign33180_e43870_d_n4, assign33180_e43870_d_n5, assign33180_e43870_d_n6, assign33180_e43870_d_n7, assign33180_e43870_d_n8, assign33180_e43870_d_n9, assign33180_e43870_d_n10, assign33180_e43870_d_n11, assign33180_e43870_d_n12, assign33180_e43870_d_n13, assign33180_e43870_d_n14,) = {
    if (locals.var_guard747 != 0.0) {
        let assign33180_e43868: f64 = (locals.var_qovd + locals.var_qbov);
        (assign33180_e43868, (locals.var_qovd_dn0 + locals.var_qbov_dn0), (locals.var_qovd_dn2 + locals.var_qbov_dn2), (locals.var_qovd_dn3 + locals.var_qbov_dn3), (locals.var_qovd_dn4 + locals.var_qbov_dn4), (locals.var_qovd_dn5 + locals.var_qbov_dn5), (locals.var_qovd_dn6 + locals.var_qbov_dn6), (locals.var_qovd_dn7 + locals.var_qbov_dn7), (locals.var_qovd_dn8 + locals.var_qbov_dn8), (locals.var_qovd_dn9 + locals.var_qbov_dn9), (locals.var_qovd_dn10 + locals.var_qbov_dn10), (locals.var_qovd_dn11 + locals.var_qbov_dn11), (locals.var_qovd_dn12 + locals.var_qbov_dn12), (locals.var_qovd_dn13 + locals.var_qbov_dn13), (locals.var_qovd_dn14 + locals.var_qbov_dn14),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn13, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign33180_e43870;
        locals.var_qovd_dn0 = assign33180_e43870_d_n0;
        locals.var_qovd_dn2 = assign33180_e43870_d_n2;
        locals.var_qovd_dn3 = assign33180_e43870_d_n3;
        locals.var_qovd_dn4 = assign33180_e43870_d_n4;
        locals.var_qovd_dn5 = assign33180_e43870_d_n5;
        locals.var_qovd_dn6 = assign33180_e43870_d_n6;
        locals.var_qovd_dn7 = assign33180_e43870_d_n7;
        locals.var_qovd_dn8 = assign33180_e43870_d_n8;
        locals.var_qovd_dn9 = assign33180_e43870_d_n9;
        locals.var_qovd_dn10 = assign33180_e43870_d_n10;
        locals.var_qovd_dn11 = assign33180_e43870_d_n11;
        locals.var_qovd_dn12 = assign33180_e43870_d_n12;
        locals.var_qovd_dn13 = assign33180_e43870_d_n13;
        locals.var_qovd_dn14 = assign33180_e43870_d_n14;

        let assign33190_e43873: f64 = if p.p1096 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard748 = assign33190_e43873;

        let (assign33200_e43881, assign33200_e43881_d_n0, assign33200_e43881_d_n2, assign33200_e43881_d_n3, assign33200_e43881_d_n4, assign33200_e43881_d_n5, assign33200_e43881_d_n6, assign33200_e43881_d_n7, assign33200_e43881_d_n8, assign33200_e43881_d_n9, assign33200_e43881_d_n10, assign33200_e43881_d_n11, assign33200_e43881_d_n12, assign33200_e43881_d_n13, assign33200_e43881_d_n14,) = {
    if ((locals.var_guard747 != 0.0) && (locals.var_guard748 != 0.0)) {
        let assign33200_e43879: f64 = (locals.var_qovb + locals.var_qiovs);
        (assign33200_e43879, (locals.var_qovb_dn0 + locals.var_qiovs_dn0), (locals.var_qovb_dn2 + locals.var_qiovs_dn2), (locals.var_qovb_dn3 + locals.var_qiovs_dn3), (locals.var_qovb_dn4 + locals.var_qiovs_dn4), (locals.var_qovb_dn5 + locals.var_qiovs_dn5), (locals.var_qovb_dn6 + locals.var_qiovs_dn6), (locals.var_qovb_dn7 + locals.var_qiovs_dn7), (locals.var_qovb_dn8 + locals.var_qiovs_dn8), (locals.var_qovb_dn9 + locals.var_qiovs_dn9), (locals.var_qovb_dn10 + locals.var_qiovs_dn10), (locals.var_qovb_dn11 + locals.var_qiovs_dn11), (locals.var_qovb_dn12 + locals.var_qiovs_dn12), (locals.var_qovb_dn13 + locals.var_qiovs_dn13), (locals.var_qovb_dn14 + locals.var_qiovs_dn14),)
    } else {
        (locals.var_qovb, locals.var_qovb_dn0, locals.var_qovb_dn2, locals.var_qovb_dn3, locals.var_qovb_dn4, locals.var_qovb_dn5, locals.var_qovb_dn6, locals.var_qovb_dn7, locals.var_qovb_dn8, locals.var_qovb_dn9, locals.var_qovb_dn10, locals.var_qovb_dn11, locals.var_qovb_dn12, locals.var_qovb_dn13, locals.var_qovb_dn14,)
    }
};
        locals.var_qovb = assign33200_e43881;
        locals.var_qovb_dn0 = assign33200_e43881_d_n0;
        locals.var_qovb_dn2 = assign33200_e43881_d_n2;
        locals.var_qovb_dn3 = assign33200_e43881_d_n3;
        locals.var_qovb_dn4 = assign33200_e43881_d_n4;
        locals.var_qovb_dn5 = assign33200_e43881_d_n5;
        locals.var_qovb_dn6 = assign33200_e43881_d_n6;
        locals.var_qovb_dn7 = assign33200_e43881_d_n7;
        locals.var_qovb_dn8 = assign33200_e43881_d_n8;
        locals.var_qovb_dn9 = assign33200_e43881_d_n9;
        locals.var_qovb_dn10 = assign33200_e43881_d_n10;
        locals.var_qovb_dn11 = assign33200_e43881_d_n11;
        locals.var_qovb_dn12 = assign33200_e43881_d_n12;
        locals.var_qovb_dn13 = assign33200_e43881_d_n13;
        locals.var_qovb_dn14 = assign33200_e43881_d_n14;

        let (assign33210_e43889, assign33210_e43889_d_n0, assign33210_e43889_d_n2, assign33210_e43889_d_n3, assign33210_e43889_d_n4, assign33210_e43889_d_n5, assign33210_e43889_d_n6, assign33210_e43889_d_n7, assign33210_e43889_d_n8, assign33210_e43889_d_n9, assign33210_e43889_d_n10, assign33210_e43889_d_n11, assign33210_e43889_d_n12, assign33210_e43889_d_n13, assign33210_e43889_d_n14,) = {
    if ((locals.var_guard747 != 0.0) && (locals.var_guard748 != 0.0)) {
        let assign33210_e43887: f64 = (locals.var_qovs + locals.var_qbovs);
        (assign33210_e43887, (locals.var_qovs_dn0 + locals.var_qbovs_dn0), (locals.var_qovs_dn2 + locals.var_qbovs_dn2), (locals.var_qovs_dn3 + locals.var_qbovs_dn3), (locals.var_qovs_dn4 + locals.var_qbovs_dn4), (locals.var_qovs_dn5 + locals.var_qbovs_dn5), (locals.var_qovs_dn6 + locals.var_qbovs_dn6), (locals.var_qovs_dn7 + locals.var_qbovs_dn7), (locals.var_qovs_dn8 + locals.var_qbovs_dn8), (locals.var_qovs_dn9 + locals.var_qbovs_dn9), (locals.var_qovs_dn10 + locals.var_qbovs_dn10), (locals.var_qovs_dn11 + locals.var_qbovs_dn11), (locals.var_qovs_dn12 + locals.var_qbovs_dn12), (locals.var_qovs_dn13 + locals.var_qbovs_dn13), (locals.var_qovs_dn14 + locals.var_qbovs_dn14),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn13, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign33210_e43889;
        locals.var_qovs_dn0 = assign33210_e43889_d_n0;
        locals.var_qovs_dn2 = assign33210_e43889_d_n2;
        locals.var_qovs_dn3 = assign33210_e43889_d_n3;
        locals.var_qovs_dn4 = assign33210_e43889_d_n4;
        locals.var_qovs_dn5 = assign33210_e43889_d_n5;
        locals.var_qovs_dn6 = assign33210_e43889_d_n6;
        locals.var_qovs_dn7 = assign33210_e43889_d_n7;
        locals.var_qovs_dn8 = assign33210_e43889_d_n8;
        locals.var_qovs_dn9 = assign33210_e43889_d_n9;
        locals.var_qovs_dn10 = assign33210_e43889_d_n10;
        locals.var_qovs_dn11 = assign33210_e43889_d_n11;
        locals.var_qovs_dn12 = assign33210_e43889_d_n12;
        locals.var_qovs_dn13 = assign33210_e43889_d_n13;
        locals.var_qovs_dn14 = assign33210_e43889_d_n14;

        let assign33230_e43897: f64 = (locals.var_devsign * p.p29);
        let assign33230_e43899: f64 = (assign33230_e43897 * locals.var_qgi);
        locals.var_qgi_1 = assign33230_e43899;
        locals.var_qgi_1_dn0 = (assign33230_e43897 * locals.var_qgi_dn0);
        locals.var_qgi_1_dn2 = (assign33230_e43897 * locals.var_qgi_dn2);
        locals.var_qgi_1_dn3 = (assign33230_e43897 * locals.var_qgi_dn3);
        locals.var_qgi_1_dn4 = (assign33230_e43897 * locals.var_qgi_dn4);
        locals.var_qgi_1_dn5 = (assign33230_e43897 * locals.var_qgi_dn5);
        locals.var_qgi_1_dn6 = (assign33230_e43897 * locals.var_qgi_dn6);
        locals.var_qgi_1_dn7 = (assign33230_e43897 * locals.var_qgi_dn7);
        locals.var_qgi_1_dn8 = (assign33230_e43897 * locals.var_qgi_dn8);
        locals.var_qgi_1_dn9 = (assign33230_e43897 * locals.var_qgi_dn9);
        locals.var_qgi_1_dn10 = (assign33230_e43897 * locals.var_qgi_dn10);
        locals.var_qgi_1_dn11 = (assign33230_e43897 * locals.var_qgi_dn11);
        locals.var_qgi_1_dn12 = (assign33230_e43897 * locals.var_qgi_dn12);
        locals.var_qgi_1_dn13 = (assign33230_e43897 * locals.var_qgi_dn13);
        locals.var_qgi_1_dn14 = (assign33230_e43897 * locals.var_qgi_dn14);

        let assign33840_e44240: f64 = if p.p47 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard751 = assign33840_e44240;

        let assign33850_e44243: f64 = if p.p46 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard752 = assign33850_e44243;

        let assign33860_e44246: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard753 = assign33860_e44246;

        let assign33870_e44253: f64 = if ((p.p42 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard754 = assign33870_e44253;

        let (assign33880_e44259, assign33880_e44259_d_n0, assign33880_e44259_d_n2, assign33880_e44259_d_n3, assign33880_e44259_d_n4, assign33880_e44259_d_n5, assign33880_e44259_d_n6, assign33880_e44259_d_n7, assign33880_e44259_d_n8, assign33880_e44259_d_n9, assign33880_e44259_d_n10, assign33880_e44259_d_n11, assign33880_e44259_d_n12, assign33880_e44259_d_n13, assign33880_e44259_d_n14,) = {
    if (locals.var_guard754 != 0.0) {
        let assign33880_e44257: f64 = (1.0 / locals.var_rdrain);
        (assign33880_e44257, (-(locals.var_rdrain_dn0 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn2 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn3 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn4 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn5 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn6 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn7 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn8 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn9 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn10 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn11 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn12 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn13 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn14 / (locals.var_rdrain * locals.var_rdrain))),)
    } else {
        (locals.var_gdpr, locals.var_gdpr_dn0, locals.var_gdpr_dn2, locals.var_gdpr_dn3, locals.var_gdpr_dn4, locals.var_gdpr_dn5, locals.var_gdpr_dn6, locals.var_gdpr_dn7, locals.var_gdpr_dn8, locals.var_gdpr_dn9, locals.var_gdpr_dn10, locals.var_gdpr_dn11, locals.var_gdpr_dn12, locals.var_gdpr_dn13, locals.var_gdpr_dn14,)
    }
};
        locals.var_gdpr = assign33880_e44259;
        locals.var_gdpr_dn0 = assign33880_e44259_d_n0;
        locals.var_gdpr_dn2 = assign33880_e44259_d_n2;
        locals.var_gdpr_dn3 = assign33880_e44259_d_n3;
        locals.var_gdpr_dn4 = assign33880_e44259_d_n4;
        locals.var_gdpr_dn5 = assign33880_e44259_d_n5;
        locals.var_gdpr_dn6 = assign33880_e44259_d_n6;
        locals.var_gdpr_dn7 = assign33880_e44259_d_n7;
        locals.var_gdpr_dn8 = assign33880_e44259_d_n8;
        locals.var_gdpr_dn9 = assign33880_e44259_d_n9;
        locals.var_gdpr_dn10 = assign33880_e44259_d_n10;
        locals.var_gdpr_dn11 = assign33880_e44259_d_n11;
        locals.var_gdpr_dn12 = assign33880_e44259_d_n12;
        locals.var_gdpr_dn13 = assign33880_e44259_d_n13;
        locals.var_gdpr_dn14 = assign33880_e44259_d_n14;

        let assign33890_e44270: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard755 = assign33890_e44270;

        let (assign33900_e44278, assign33900_e44278_d_n0, assign33900_e44278_d_n2, assign33900_e44278_d_n3, assign33900_e44278_d_n4, assign33900_e44278_d_n5, assign33900_e44278_d_n6, assign33900_e44278_d_n7, assign33900_e44278_d_n8, assign33900_e44278_d_n9, assign33900_e44278_d_n10, assign33900_e44278_d_n11, assign33900_e44278_d_n12, assign33900_e44278_d_n13, assign33900_e44278_d_n14,) = {
    if ((locals.var_guard754 != 0.0) && (locals.var_guard755 != 0.0)) {
        let assign33900_e44276: f64 = (1.0 / locals.var_rdrift_d);
        (assign33900_e44276, (-(locals.var_rdrift_d_dn0 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn2 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn3 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn4 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn5 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn6 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn7 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn8 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn9 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn10 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn11 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn12 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn13 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn14 / (locals.var_rdrift_d * locals.var_rdrift_d))),)
    } else {
        (locals.var_gdrift_d, locals.var_gdrift_d_dn0, locals.var_gdrift_d_dn2, locals.var_gdrift_d_dn3, locals.var_gdrift_d_dn4, locals.var_gdrift_d_dn5, locals.var_gdrift_d_dn6, locals.var_gdrift_d_dn7, locals.var_gdrift_d_dn8, locals.var_gdrift_d_dn9, locals.var_gdrift_d_dn10, locals.var_gdrift_d_dn11, locals.var_gdrift_d_dn12, locals.var_gdrift_d_dn13, locals.var_gdrift_d_dn14,)
    }
};
        locals.var_gdrift_d = assign33900_e44278;
        locals.var_gdrift_d_dn0 = assign33900_e44278_d_n0;
        locals.var_gdrift_d_dn2 = assign33900_e44278_d_n2;
        locals.var_gdrift_d_dn3 = assign33900_e44278_d_n3;
        locals.var_gdrift_d_dn4 = assign33900_e44278_d_n4;
        locals.var_gdrift_d_dn5 = assign33900_e44278_d_n5;
        locals.var_gdrift_d_dn6 = assign33900_e44278_d_n6;
        locals.var_gdrift_d_dn7 = assign33900_e44278_d_n7;
        locals.var_gdrift_d_dn8 = assign33900_e44278_d_n8;
        locals.var_gdrift_d_dn9 = assign33900_e44278_d_n9;
        locals.var_gdrift_d_dn10 = assign33900_e44278_d_n10;
        locals.var_gdrift_d_dn11 = assign33900_e44278_d_n11;
        locals.var_gdrift_d_dn12 = assign33900_e44278_d_n12;
        locals.var_gdrift_d_dn13 = assign33900_e44278_d_n13;
        locals.var_gdrift_d_dn14 = assign33900_e44278_d_n14;

        let assign33910_e44285: f64 = if ((p.p42 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard756 = assign33910_e44285;

        let (assign33920_e44291, assign33920_e44291_d_n0, assign33920_e44291_d_n2, assign33920_e44291_d_n3, assign33920_e44291_d_n4, assign33920_e44291_d_n5, assign33920_e44291_d_n6, assign33920_e44291_d_n7, assign33920_e44291_d_n8, assign33920_e44291_d_n9, assign33920_e44291_d_n10, assign33920_e44291_d_n11, assign33920_e44291_d_n12, assign33920_e44291_d_n13, assign33920_e44291_d_n14,) = {
    if (locals.var_guard756 != 0.0) {
        let assign33920_e44289: f64 = (1.0 / locals.var_rsource);
        (assign33920_e44289, (-(locals.var_rsource_dn0 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn2 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn3 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn4 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn5 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn6 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn7 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn8 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn9 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn10 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn11 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn12 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn13 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn14 / (locals.var_rsource * locals.var_rsource))),)
    } else {
        (locals.var_gspr, locals.var_gspr_dn0, locals.var_gspr_dn2, locals.var_gspr_dn3, locals.var_gspr_dn4, locals.var_gspr_dn5, locals.var_gspr_dn6, locals.var_gspr_dn7, locals.var_gspr_dn8, locals.var_gspr_dn9, locals.var_gspr_dn10, locals.var_gspr_dn11, locals.var_gspr_dn12, locals.var_gspr_dn13, locals.var_gspr_dn14,)
    }
};
        locals.var_gspr = assign33920_e44291;
        locals.var_gspr_dn0 = assign33920_e44291_d_n0;
        locals.var_gspr_dn2 = assign33920_e44291_d_n2;
        locals.var_gspr_dn3 = assign33920_e44291_d_n3;
        locals.var_gspr_dn4 = assign33920_e44291_d_n4;
        locals.var_gspr_dn5 = assign33920_e44291_d_n5;
        locals.var_gspr_dn6 = assign33920_e44291_d_n6;
        locals.var_gspr_dn7 = assign33920_e44291_d_n7;
        locals.var_gspr_dn8 = assign33920_e44291_d_n8;
        locals.var_gspr_dn9 = assign33920_e44291_d_n9;
        locals.var_gspr_dn10 = assign33920_e44291_d_n10;
        locals.var_gspr_dn11 = assign33920_e44291_d_n11;
        locals.var_gspr_dn12 = assign33920_e44291_d_n12;
        locals.var_gspr_dn13 = assign33920_e44291_d_n13;
        locals.var_gspr_dn14 = assign33920_e44291_d_n14;

        let assign33930_e44302: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard757 = assign33930_e44302;

        let (assign33940_e44310, assign33940_e44310_d_n0, assign33940_e44310_d_n2, assign33940_e44310_d_n3, assign33940_e44310_d_n4, assign33940_e44310_d_n5, assign33940_e44310_d_n6, assign33940_e44310_d_n7, assign33940_e44310_d_n8, assign33940_e44310_d_n9, assign33940_e44310_d_n10, assign33940_e44310_d_n11, assign33940_e44310_d_n12, assign33940_e44310_d_n13, assign33940_e44310_d_n14,) = {
    if ((locals.var_guard756 != 0.0) && (locals.var_guard757 != 0.0)) {
        let assign33940_e44308: f64 = (1.0 / locals.var_rdrift_s);
        (assign33940_e44308, (-(locals.var_rdrift_s_dn0 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn2 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn3 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn4 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn5 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn6 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn7 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn8 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn9 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn10 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn11 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn12 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn13 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn14 / (locals.var_rdrift_s * locals.var_rdrift_s))),)
    } else {
        (locals.var_gdrift_s, locals.var_gdrift_s_dn0, locals.var_gdrift_s_dn2, locals.var_gdrift_s_dn3, locals.var_gdrift_s_dn4, locals.var_gdrift_s_dn5, locals.var_gdrift_s_dn6, locals.var_gdrift_s_dn7, locals.var_gdrift_s_dn8, locals.var_gdrift_s_dn9, locals.var_gdrift_s_dn10, locals.var_gdrift_s_dn11, locals.var_gdrift_s_dn12, locals.var_gdrift_s_dn13, locals.var_gdrift_s_dn14,)
    }
};
        locals.var_gdrift_s = assign33940_e44310;
        locals.var_gdrift_s_dn0 = assign33940_e44310_d_n0;
        locals.var_gdrift_s_dn2 = assign33940_e44310_d_n2;
        locals.var_gdrift_s_dn3 = assign33940_e44310_d_n3;
        locals.var_gdrift_s_dn4 = assign33940_e44310_d_n4;
        locals.var_gdrift_s_dn5 = assign33940_e44310_d_n5;
        locals.var_gdrift_s_dn6 = assign33940_e44310_d_n6;
        locals.var_gdrift_s_dn7 = assign33940_e44310_d_n7;
        locals.var_gdrift_s_dn8 = assign33940_e44310_d_n8;
        locals.var_gdrift_s_dn9 = assign33940_e44310_d_n9;
        locals.var_gdrift_s_dn10 = assign33940_e44310_d_n10;
        locals.var_gdrift_s_dn11 = assign33940_e44310_d_n11;
        locals.var_gdrift_s_dn12 = assign33940_e44310_d_n12;
        locals.var_gdrift_s_dn13 = assign33940_e44310_d_n13;
        locals.var_gdrift_s_dn14 = assign33940_e44310_d_n14;

        let assign33950_e44313: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard758 = assign33950_e44313;

        let assign33960_e44316: f64 = if p.p7 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard761 = assign33960_e44316;

        let (assign33970_e44323, assign33970_e44323_d_n0, assign33970_e44323_d_n2, assign33970_e44323_d_n3, assign33970_e44323_d_n4, assign33970_e44323_d_n5, assign33970_e44323_d_n6, assign33970_e44323_d_n7, assign33970_e44323_d_n8, assign33970_e44323_d_n9, assign33970_e44323_d_n10, assign33970_e44323_d_n11, assign33970_e44323_d_n12, assign33970_e44323_d_n13, assign33970_e44323_d_n14,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard761 != 0.0)) {
        (locals.var_gcrg, locals.var_gcrg_dn0, locals.var_gcrg_dn2, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11, locals.var_gcrg_dn12, locals.var_gcrg_dn13, locals.var_gcrg_dn14,)
    } else {
        (locals.var_ggate, locals.var_ggate_dn0, locals.var_ggate_dn2, locals.var_ggate_dn3, locals.var_ggate_dn4, locals.var_ggate_dn5, locals.var_ggate_dn6, locals.var_ggate_dn7, locals.var_ggate_dn8, locals.var_ggate_dn9, locals.var_ggate_dn10, locals.var_ggate_dn11, locals.var_ggate_dn12, locals.var_ggate_dn13, locals.var_ggate_dn14,)
    }
};
        locals.var_ggate = assign33970_e44323;
        locals.var_ggate_dn0 = assign33970_e44323_d_n0;
        locals.var_ggate_dn2 = assign33970_e44323_d_n2;
        locals.var_ggate_dn3 = assign33970_e44323_d_n3;
        locals.var_ggate_dn4 = assign33970_e44323_d_n4;
        locals.var_ggate_dn5 = assign33970_e44323_d_n5;
        locals.var_ggate_dn6 = assign33970_e44323_d_n6;
        locals.var_ggate_dn7 = assign33970_e44323_d_n7;
        locals.var_ggate_dn8 = assign33970_e44323_d_n8;
        locals.var_ggate_dn9 = assign33970_e44323_d_n9;
        locals.var_ggate_dn10 = assign33970_e44323_d_n10;
        locals.var_ggate_dn11 = assign33970_e44323_d_n11;
        locals.var_ggate_dn12 = assign33970_e44323_d_n12;
        locals.var_ggate_dn13 = assign33970_e44323_d_n13;
        locals.var_ggate_dn14 = assign33970_e44323_d_n14;

        let (assign33990_e44342, assign33990_e44342_d_n0, assign33990_e44342_d_n2, assign33990_e44342_d_n3, assign33990_e44342_d_n4, assign33990_e44342_d_n5, assign33990_e44342_d_n6, assign33990_e44342_d_n7, assign33990_e44342_d_n8, assign33990_e44342_d_n9, assign33990_e44342_d_n10, assign33990_e44342_d_n11, assign33990_e44342_d_n12, assign33990_e44342_d_n13, assign33990_e44342_d_n14,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard761 == 0.0)) {
        (locals.var_grgeltd, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ggate, locals.var_ggate_dn0, locals.var_ggate_dn2, locals.var_ggate_dn3, locals.var_ggate_dn4, locals.var_ggate_dn5, locals.var_ggate_dn6, locals.var_ggate_dn7, locals.var_ggate_dn8, locals.var_ggate_dn9, locals.var_ggate_dn10, locals.var_ggate_dn11, locals.var_ggate_dn12, locals.var_ggate_dn13, locals.var_ggate_dn14,)
    }
};
        locals.var_ggate = assign33990_e44342;
        locals.var_ggate_dn0 = assign33990_e44342_d_n0;
        locals.var_ggate_dn2 = assign33990_e44342_d_n2;
        locals.var_ggate_dn3 = assign33990_e44342_d_n3;
        locals.var_ggate_dn4 = assign33990_e44342_d_n4;
        locals.var_ggate_dn5 = assign33990_e44342_d_n5;
        locals.var_ggate_dn6 = assign33990_e44342_d_n6;
        locals.var_ggate_dn7 = assign33990_e44342_d_n7;
        locals.var_ggate_dn8 = assign33990_e44342_d_n8;
        locals.var_ggate_dn9 = assign33990_e44342_d_n9;
        locals.var_ggate_dn10 = assign33990_e44342_d_n10;
        locals.var_ggate_dn11 = assign33990_e44342_d_n11;
        locals.var_ggate_dn12 = assign33990_e44342_d_n12;
        locals.var_ggate_dn13 = assign33990_e44342_d_n13;
        locals.var_ggate_dn14 = assign33990_e44342_d_n14;

        let assign34010_e44353: f64 = if p.p7 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard762 = assign34010_e44353;

        let assign34020_e44360: f64 = if ((p.p49 != 0.0) && (p.p909 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard763 = assign34020_e44360;

        let (assign34030_e44370, assign34030_e44370_d_n0, assign34030_e44370_d_n2, assign34030_e44370_d_n3, assign34030_e44370_d_n4, assign34030_e44370_d_n5, assign34030_e44370_d_n6, assign34030_e44370_d_n7, assign34030_e44370_d_n8, assign34030_e44370_d_n9, assign34030_e44370_d_n10, assign34030_e44370_d_n11, assign34030_e44370_d_n12, assign34030_e44370_d_n13, assign34030_e44370_d_n14,) = {
    if (locals.var_guard763 != 0.0) {
        let assign34030_e44364: f64 = (locals.var_devsign * locals.var_sigvds);
        let assign34030_e44366: f64 = (assign34030_e44364 * locals.var_ids);
        let assign34030_e44368: f64 = (assign34030_e44366 * (nv5 - nv7));
        (assign34030_e44368, ((assign34030_e44364 * locals.var_ids_dn0) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn2) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn3) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn4) * (nv5 - nv7)), (((assign34030_e44364 * locals.var_ids_dn5) * (nv5 - nv7)) + assign34030_e44366), ((assign34030_e44364 * locals.var_ids_dn6) * (nv5 - nv7)), (((assign34030_e44364 * locals.var_ids_dn7) * (nv5 - nv7)) + (-assign34030_e44366)), ((assign34030_e44364 * locals.var_ids_dn8) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn9) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn10) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn11) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn12) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn13) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn14) * (nv5 - nv7)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34030_e44370;
        locals.var_pdiss_dn0 = assign34030_e44370_d_n0;
        locals.var_pdiss_dn2 = assign34030_e44370_d_n2;
        locals.var_pdiss_dn3 = assign34030_e44370_d_n3;
        locals.var_pdiss_dn4 = assign34030_e44370_d_n4;
        locals.var_pdiss_dn5 = assign34030_e44370_d_n5;
        locals.var_pdiss_dn6 = assign34030_e44370_d_n6;
        locals.var_pdiss_dn7 = assign34030_e44370_d_n7;
        locals.var_pdiss_dn8 = assign34030_e44370_d_n8;
        locals.var_pdiss_dn9 = assign34030_e44370_d_n9;
        locals.var_pdiss_dn10 = assign34030_e44370_d_n10;
        locals.var_pdiss_dn11 = assign34030_e44370_d_n11;
        locals.var_pdiss_dn12 = assign34030_e44370_d_n12;
        locals.var_pdiss_dn13 = assign34030_e44370_d_n13;
        locals.var_pdiss_dn14 = assign34030_e44370_d_n14;

        let assign34040_e44377: f64 = if ((p.p42 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard764 = assign34040_e44377;

        let assign34050_e44388: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard765 = assign34050_e44388;

        let (assign34060_e44408, assign34060_e44408_d_n0, assign34060_e44408_d_n2, assign34060_e44408_d_n3, assign34060_e44408_d_n4, assign34060_e44408_d_n5, assign34060_e44408_d_n6, assign34060_e44408_d_n7, assign34060_e44408_d_n8, assign34060_e44408_d_n9, assign34060_e44408_d_n10, assign34060_e44408_d_n11, assign34060_e44408_d_n12, assign34060_e44408_d_n13, assign34060_e44408_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard764 != 0.0)) && (locals.var_guard765 != 0.0)) {
        let assign34060_e44397: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign34060_e44399: f64 = (assign34060_e44397 * locals.var_gdpr);
        let assign34060_e44400: f64 = (locals.var_pdiss + assign34060_e44399);
        let assign34060_e44403: f64 = ((nv6 - nv5) * (nv6 - nv5));
        let assign34060_e44405: f64 = (assign34060_e44403 * locals.var_gdrift_d);
        let assign34060_e44406: f64 = (assign34060_e44400 + assign34060_e44405);
        (assign34060_e44406, ((locals.var_pdiss_dn0 + ((((nv0 - nv6) + (nv0 - nv6)) * locals.var_gdpr) + (assign34060_e44397 * locals.var_gdpr_dn0))) + (assign34060_e44403 * locals.var_gdrift_d_dn0)), ((locals.var_pdiss_dn2 + (assign34060_e44397 * locals.var_gdpr_dn2)) + (assign34060_e44403 * locals.var_gdrift_d_dn2)), ((locals.var_pdiss_dn3 + (assign34060_e44397 * locals.var_gdpr_dn3)) + (assign34060_e44403 * locals.var_gdrift_d_dn3)), ((locals.var_pdiss_dn4 + (assign34060_e44397 * locals.var_gdpr_dn4)) + (assign34060_e44403 * locals.var_gdrift_d_dn4)), ((locals.var_pdiss_dn5 + (assign34060_e44397 * locals.var_gdpr_dn5)) + ((((-(nv6 - nv5)) + (-(nv6 - nv5))) * locals.var_gdrift_d) + (assign34060_e44403 * locals.var_gdrift_d_dn5))), ((locals.var_pdiss_dn6 + ((((-(nv0 - nv6)) + (-(nv0 - nv6))) * locals.var_gdpr) + (assign34060_e44397 * locals.var_gdpr_dn6))) + ((((nv6 - nv5) + (nv6 - nv5)) * locals.var_gdrift_d) + (assign34060_e44403 * locals.var_gdrift_d_dn6))), ((locals.var_pdiss_dn7 + (assign34060_e44397 * locals.var_gdpr_dn7)) + (assign34060_e44403 * locals.var_gdrift_d_dn7)), ((locals.var_pdiss_dn8 + (assign34060_e44397 * locals.var_gdpr_dn8)) + (assign34060_e44403 * locals.var_gdrift_d_dn8)), ((locals.var_pdiss_dn9 + (assign34060_e44397 * locals.var_gdpr_dn9)) + (assign34060_e44403 * locals.var_gdrift_d_dn9)), ((locals.var_pdiss_dn10 + (assign34060_e44397 * locals.var_gdpr_dn10)) + (assign34060_e44403 * locals.var_gdrift_d_dn10)), ((locals.var_pdiss_dn11 + (assign34060_e44397 * locals.var_gdpr_dn11)) + (assign34060_e44403 * locals.var_gdrift_d_dn11)), ((locals.var_pdiss_dn12 + (assign34060_e44397 * locals.var_gdpr_dn12)) + (assign34060_e44403 * locals.var_gdrift_d_dn12)), ((locals.var_pdiss_dn13 + (assign34060_e44397 * locals.var_gdpr_dn13)) + (assign34060_e44403 * locals.var_gdrift_d_dn13)), ((locals.var_pdiss_dn14 + (assign34060_e44397 * locals.var_gdpr_dn14)) + (assign34060_e44403 * locals.var_gdrift_d_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34060_e44408;
        locals.var_pdiss_dn0 = assign34060_e44408_d_n0;
        locals.var_pdiss_dn2 = assign34060_e44408_d_n2;
        locals.var_pdiss_dn3 = assign34060_e44408_d_n3;
        locals.var_pdiss_dn4 = assign34060_e44408_d_n4;
        locals.var_pdiss_dn5 = assign34060_e44408_d_n5;
        locals.var_pdiss_dn6 = assign34060_e44408_d_n6;
        locals.var_pdiss_dn7 = assign34060_e44408_d_n7;
        locals.var_pdiss_dn8 = assign34060_e44408_d_n8;
        locals.var_pdiss_dn9 = assign34060_e44408_d_n9;
        locals.var_pdiss_dn10 = assign34060_e44408_d_n10;
        locals.var_pdiss_dn11 = assign34060_e44408_d_n11;
        locals.var_pdiss_dn12 = assign34060_e44408_d_n12;
        locals.var_pdiss_dn13 = assign34060_e44408_d_n13;
        locals.var_pdiss_dn14 = assign34060_e44408_d_n14;

        let (assign34070_e44423, assign34070_e44423_d_n0, assign34070_e44423_d_n2, assign34070_e44423_d_n3, assign34070_e44423_d_n4, assign34070_e44423_d_n5, assign34070_e44423_d_n6, assign34070_e44423_d_n7, assign34070_e44423_d_n8, assign34070_e44423_d_n9, assign34070_e44423_d_n10, assign34070_e44423_d_n11, assign34070_e44423_d_n12, assign34070_e44423_d_n13, assign34070_e44423_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard764 != 0.0)) && (locals.var_guard765 == 0.0)) {
        let assign34070_e44418: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign34070_e44420: f64 = (assign34070_e44418 * locals.var_gdpr);
        let assign34070_e44421: f64 = (locals.var_pdiss + assign34070_e44420);
        (assign34070_e44421, (locals.var_pdiss_dn0 + ((((nv0 - nv6) + (nv0 - nv6)) * locals.var_gdpr) + (assign34070_e44418 * locals.var_gdpr_dn0))), (locals.var_pdiss_dn2 + (assign34070_e44418 * locals.var_gdpr_dn2)), (locals.var_pdiss_dn3 + (assign34070_e44418 * locals.var_gdpr_dn3)), (locals.var_pdiss_dn4 + (assign34070_e44418 * locals.var_gdpr_dn4)), (locals.var_pdiss_dn5 + (assign34070_e44418 * locals.var_gdpr_dn5)), (locals.var_pdiss_dn6 + ((((-(nv0 - nv6)) + (-(nv0 - nv6))) * locals.var_gdpr) + (assign34070_e44418 * locals.var_gdpr_dn6))), (locals.var_pdiss_dn7 + (assign34070_e44418 * locals.var_gdpr_dn7)), (locals.var_pdiss_dn8 + (assign34070_e44418 * locals.var_gdpr_dn8)), (locals.var_pdiss_dn9 + (assign34070_e44418 * locals.var_gdpr_dn9)), (locals.var_pdiss_dn10 + (assign34070_e44418 * locals.var_gdpr_dn10)), (locals.var_pdiss_dn11 + (assign34070_e44418 * locals.var_gdpr_dn11)), (locals.var_pdiss_dn12 + (assign34070_e44418 * locals.var_gdpr_dn12)), (locals.var_pdiss_dn13 + (assign34070_e44418 * locals.var_gdpr_dn13)), (locals.var_pdiss_dn14 + (assign34070_e44418 * locals.var_gdpr_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34070_e44423;
        locals.var_pdiss_dn0 = assign34070_e44423_d_n0;
        locals.var_pdiss_dn2 = assign34070_e44423_d_n2;
        locals.var_pdiss_dn3 = assign34070_e44423_d_n3;
        locals.var_pdiss_dn4 = assign34070_e44423_d_n4;
        locals.var_pdiss_dn5 = assign34070_e44423_d_n5;
        locals.var_pdiss_dn6 = assign34070_e44423_d_n6;
        locals.var_pdiss_dn7 = assign34070_e44423_d_n7;
        locals.var_pdiss_dn8 = assign34070_e44423_d_n8;
        locals.var_pdiss_dn9 = assign34070_e44423_d_n9;
        locals.var_pdiss_dn10 = assign34070_e44423_d_n10;
        locals.var_pdiss_dn11 = assign34070_e44423_d_n11;
        locals.var_pdiss_dn12 = assign34070_e44423_d_n12;
        locals.var_pdiss_dn13 = assign34070_e44423_d_n13;
        locals.var_pdiss_dn14 = assign34070_e44423_d_n14;

        let assign34080_e44430: f64 = if ((p.p42 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard766 = assign34080_e44430;

        let assign34090_e44441: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard767 = assign34090_e44441;

    }

    pub(super) fn stamp_transient_block_108(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (assign34100_e44461, assign34100_e44461_d_n0, assign34100_e44461_d_n2, assign34100_e44461_d_n3, assign34100_e44461_d_n4, assign34100_e44461_d_n5, assign34100_e44461_d_n6, assign34100_e44461_d_n7, assign34100_e44461_d_n8, assign34100_e44461_d_n9, assign34100_e44461_d_n10, assign34100_e44461_d_n11, assign34100_e44461_d_n12, assign34100_e44461_d_n13, assign34100_e44461_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard766 != 0.0)) && (locals.var_guard767 != 0.0)) {
        let assign34100_e44450: f64 = ((nv2 - nv8) * (nv2 - nv8));
        let assign34100_e44452: f64 = (assign34100_e44450 * locals.var_gspr);
        let assign34100_e44453: f64 = (locals.var_pdiss + assign34100_e44452);
        let assign34100_e44456: f64 = ((nv8 - nv7) * (nv8 - nv7));
        let assign34100_e44458: f64 = (assign34100_e44456 * locals.var_gdrift_s);
        let assign34100_e44459: f64 = (assign34100_e44453 + assign34100_e44458);
        (assign34100_e44459, ((locals.var_pdiss_dn0 + (assign34100_e44450 * locals.var_gspr_dn0)) + (assign34100_e44456 * locals.var_gdrift_s_dn0)), ((locals.var_pdiss_dn2 + ((((nv2 - nv8) + (nv2 - nv8)) * locals.var_gspr) + (assign34100_e44450 * locals.var_gspr_dn2))) + (assign34100_e44456 * locals.var_gdrift_s_dn2)), ((locals.var_pdiss_dn3 + (assign34100_e44450 * locals.var_gspr_dn3)) + (assign34100_e44456 * locals.var_gdrift_s_dn3)), ((locals.var_pdiss_dn4 + (assign34100_e44450 * locals.var_gspr_dn4)) + (assign34100_e44456 * locals.var_gdrift_s_dn4)), ((locals.var_pdiss_dn5 + (assign34100_e44450 * locals.var_gspr_dn5)) + (assign34100_e44456 * locals.var_gdrift_s_dn5)), ((locals.var_pdiss_dn6 + (assign34100_e44450 * locals.var_gspr_dn6)) + (assign34100_e44456 * locals.var_gdrift_s_dn6)), ((locals.var_pdiss_dn7 + (assign34100_e44450 * locals.var_gspr_dn7)) + ((((-(nv8 - nv7)) + (-(nv8 - nv7))) * locals.var_gdrift_s) + (assign34100_e44456 * locals.var_gdrift_s_dn7))), ((locals.var_pdiss_dn8 + ((((-(nv2 - nv8)) + (-(nv2 - nv8))) * locals.var_gspr) + (assign34100_e44450 * locals.var_gspr_dn8))) + ((((nv8 - nv7) + (nv8 - nv7)) * locals.var_gdrift_s) + (assign34100_e44456 * locals.var_gdrift_s_dn8))), ((locals.var_pdiss_dn9 + (assign34100_e44450 * locals.var_gspr_dn9)) + (assign34100_e44456 * locals.var_gdrift_s_dn9)), ((locals.var_pdiss_dn10 + (assign34100_e44450 * locals.var_gspr_dn10)) + (assign34100_e44456 * locals.var_gdrift_s_dn10)), ((locals.var_pdiss_dn11 + (assign34100_e44450 * locals.var_gspr_dn11)) + (assign34100_e44456 * locals.var_gdrift_s_dn11)), ((locals.var_pdiss_dn12 + (assign34100_e44450 * locals.var_gspr_dn12)) + (assign34100_e44456 * locals.var_gdrift_s_dn12)), ((locals.var_pdiss_dn13 + (assign34100_e44450 * locals.var_gspr_dn13)) + (assign34100_e44456 * locals.var_gdrift_s_dn13)), ((locals.var_pdiss_dn14 + (assign34100_e44450 * locals.var_gspr_dn14)) + (assign34100_e44456 * locals.var_gdrift_s_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34100_e44461;
        locals.var_pdiss_dn0 = assign34100_e44461_d_n0;
        locals.var_pdiss_dn2 = assign34100_e44461_d_n2;
        locals.var_pdiss_dn3 = assign34100_e44461_d_n3;
        locals.var_pdiss_dn4 = assign34100_e44461_d_n4;
        locals.var_pdiss_dn5 = assign34100_e44461_d_n5;
        locals.var_pdiss_dn6 = assign34100_e44461_d_n6;
        locals.var_pdiss_dn7 = assign34100_e44461_d_n7;
        locals.var_pdiss_dn8 = assign34100_e44461_d_n8;
        locals.var_pdiss_dn9 = assign34100_e44461_d_n9;
        locals.var_pdiss_dn10 = assign34100_e44461_d_n10;
        locals.var_pdiss_dn11 = assign34100_e44461_d_n11;
        locals.var_pdiss_dn12 = assign34100_e44461_d_n12;
        locals.var_pdiss_dn13 = assign34100_e44461_d_n13;
        locals.var_pdiss_dn14 = assign34100_e44461_d_n14;

        let (assign34110_e44476, assign34110_e44476_d_n0, assign34110_e44476_d_n2, assign34110_e44476_d_n3, assign34110_e44476_d_n4, assign34110_e44476_d_n5, assign34110_e44476_d_n6, assign34110_e44476_d_n7, assign34110_e44476_d_n8, assign34110_e44476_d_n9, assign34110_e44476_d_n10, assign34110_e44476_d_n11, assign34110_e44476_d_n12, assign34110_e44476_d_n13, assign34110_e44476_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard766 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign34110_e44471: f64 = ((nv2 - nv8) * (nv2 - nv8));
        let assign34110_e44473: f64 = (assign34110_e44471 * locals.var_gspr);
        let assign34110_e44474: f64 = (locals.var_pdiss + assign34110_e44473);
        (assign34110_e44474, (locals.var_pdiss_dn0 + (assign34110_e44471 * locals.var_gspr_dn0)), (locals.var_pdiss_dn2 + ((((nv2 - nv8) + (nv2 - nv8)) * locals.var_gspr) + (assign34110_e44471 * locals.var_gspr_dn2))), (locals.var_pdiss_dn3 + (assign34110_e44471 * locals.var_gspr_dn3)), (locals.var_pdiss_dn4 + (assign34110_e44471 * locals.var_gspr_dn4)), (locals.var_pdiss_dn5 + (assign34110_e44471 * locals.var_gspr_dn5)), (locals.var_pdiss_dn6 + (assign34110_e44471 * locals.var_gspr_dn6)), (locals.var_pdiss_dn7 + (assign34110_e44471 * locals.var_gspr_dn7)), (locals.var_pdiss_dn8 + ((((-(nv2 - nv8)) + (-(nv2 - nv8))) * locals.var_gspr) + (assign34110_e44471 * locals.var_gspr_dn8))), (locals.var_pdiss_dn9 + (assign34110_e44471 * locals.var_gspr_dn9)), (locals.var_pdiss_dn10 + (assign34110_e44471 * locals.var_gspr_dn10)), (locals.var_pdiss_dn11 + (assign34110_e44471 * locals.var_gspr_dn11)), (locals.var_pdiss_dn12 + (assign34110_e44471 * locals.var_gspr_dn12)), (locals.var_pdiss_dn13 + (assign34110_e44471 * locals.var_gspr_dn13)), (locals.var_pdiss_dn14 + (assign34110_e44471 * locals.var_gspr_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34110_e44476;
        locals.var_pdiss_dn0 = assign34110_e44476_d_n0;
        locals.var_pdiss_dn2 = assign34110_e44476_d_n2;
        locals.var_pdiss_dn3 = assign34110_e44476_d_n3;
        locals.var_pdiss_dn4 = assign34110_e44476_d_n4;
        locals.var_pdiss_dn5 = assign34110_e44476_d_n5;
        locals.var_pdiss_dn6 = assign34110_e44476_d_n6;
        locals.var_pdiss_dn7 = assign34110_e44476_d_n7;
        locals.var_pdiss_dn8 = assign34110_e44476_d_n8;
        locals.var_pdiss_dn9 = assign34110_e44476_d_n9;
        locals.var_pdiss_dn10 = assign34110_e44476_d_n10;
        locals.var_pdiss_dn11 = assign34110_e44476_d_n11;
        locals.var_pdiss_dn12 = assign34110_e44476_d_n12;
        locals.var_pdiss_dn13 = assign34110_e44476_d_n13;
        locals.var_pdiss_dn14 = assign34110_e44476_d_n14;

        let assign34130_e44482: f64 = if p.p8 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard769 = assign34130_e44482;

        let assign34140_e44485: f64 = if p.p1097 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign34140_e44485;

        let assign34160_e44499: f64 = if ((p.p8 != 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard772 = assign34160_e44499;

    }

    pub(super) fn stamp_reactive_block_0(
        locals: &mut StampLocals,
    ) {
        locals.var_cdscdr_i = 0.0;
        locals.var_cdscdr_i_dn0 = 0.0;
        locals.var_cdscdr_i_dn2 = 0.0;
        locals.var_cdscdr_i_dn3 = 0.0;
        locals.var_cdscdr_i_dn4 = 0.0;
        locals.var_cdscdr_i_dn5 = 0.0;
        locals.var_cdscdr_i_dn6 = 0.0;
        locals.var_cdscdr_i_dn7 = 0.0;
        locals.var_cdscdr_i_dn8 = 0.0;
        locals.var_cdscdr_i_dn9 = 0.0;
        locals.var_cdscdr_i_dn10 = 0.0;
        locals.var_cdscdr_i_dn11 = 0.0;
        locals.var_cdscdr_i_dn12 = 0.0;
        locals.var_cdscdr_i_dn13 = 0.0;
        locals.var_cdscdr_i_dn14 = 0.0;
        locals.var_cdscdr_i_rv = 0.0;

        locals.var_l_wln1 = 0.0;
        locals.var_l_wln1_rv = 0.0;

        locals.var_ptwgr_i = 0.0;
        locals.var_ptwgr_i_dn0 = 0.0;
        locals.var_ptwgr_i_dn2 = 0.0;
        locals.var_ptwgr_i_dn3 = 0.0;
        locals.var_ptwgr_i_dn4 = 0.0;
        locals.var_ptwgr_i_dn5 = 0.0;
        locals.var_ptwgr_i_dn6 = 0.0;
        locals.var_ptwgr_i_dn7 = 0.0;
        locals.var_ptwgr_i_dn8 = 0.0;
        locals.var_ptwgr_i_dn9 = 0.0;
        locals.var_ptwgr_i_dn10 = 0.0;
        locals.var_ptwgr_i_dn11 = 0.0;
        locals.var_ptwgr_i_dn12 = 0.0;
        locals.var_ptwgr_i_dn13 = 0.0;
        locals.var_ptwgr_i_dn14 = 0.0;
        locals.var_ptwgr_i_rv = 0.0;

        locals.var_uar_i = 0.0;
        locals.var_uar_i_dn0 = 0.0;
        locals.var_uar_i_dn2 = 0.0;
        locals.var_uar_i_dn3 = 0.0;
        locals.var_uar_i_dn4 = 0.0;
        locals.var_uar_i_dn5 = 0.0;
        locals.var_uar_i_dn6 = 0.0;
        locals.var_uar_i_dn7 = 0.0;
        locals.var_uar_i_dn8 = 0.0;
        locals.var_uar_i_dn9 = 0.0;
        locals.var_uar_i_dn10 = 0.0;
        locals.var_uar_i_dn11 = 0.0;
        locals.var_uar_i_dn12 = 0.0;
        locals.var_uar_i_dn13 = 0.0;
        locals.var_uar_i_dn14 = 0.0;
        locals.var_uar_i_rv = 0.0;

        locals.var_ucsr_i = 0.0;
        locals.var_ucsr_i_rv = 0.0;

        locals.var_ud_a = 0.0;
        locals.var_ud_a_dn0 = 0.0;
        locals.var_ud_a_dn2 = 0.0;
        locals.var_ud_a_dn3 = 0.0;
        locals.var_ud_a_dn4 = 0.0;
        locals.var_ud_a_dn5 = 0.0;
        locals.var_ud_a_dn6 = 0.0;
        locals.var_ud_a_dn7 = 0.0;
        locals.var_ud_a_dn8 = 0.0;
        locals.var_ud_a_dn9 = 0.0;
        locals.var_ud_a_dn10 = 0.0;
        locals.var_ud_a_dn11 = 0.0;
        locals.var_ud_a_dn12 = 0.0;
        locals.var_ud_a_dn13 = 0.0;
        locals.var_ud_a_dn14 = 0.0;
        locals.var_ud_a_rv = 0.0;

        locals.var_w_wwn1 = 0.0;
        locals.var_w_wwn1_rv = 0.0;

        locals.var_inv_sa = 0.0;
        locals.var_inv_sa_dn0 = 0.0;
        locals.var_inv_sa_dn2 = 0.0;
        locals.var_inv_sa_dn3 = 0.0;
        locals.var_inv_sa_dn4 = 0.0;
        locals.var_inv_sa_dn5 = 0.0;
        locals.var_inv_sa_dn6 = 0.0;
        locals.var_inv_sa_dn7 = 0.0;
        locals.var_inv_sa_dn8 = 0.0;
        locals.var_inv_sa_dn9 = 0.0;
        locals.var_inv_sa_dn10 = 0.0;
        locals.var_inv_sa_dn11 = 0.0;
        locals.var_inv_sa_dn12 = 0.0;
        locals.var_inv_sa_dn13 = 0.0;
        locals.var_inv_sa_dn14 = 0.0;
        locals.var_inv_sa_rv = 0.0;

        locals.var_eta_stress = 0.0;
        locals.var_eta_stress_dn0 = 0.0;
        locals.var_eta_stress_dn2 = 0.0;
        locals.var_eta_stress_dn3 = 0.0;
        locals.var_eta_stress_dn4 = 0.0;
        locals.var_eta_stress_dn5 = 0.0;
        locals.var_eta_stress_dn6 = 0.0;
        locals.var_eta_stress_dn7 = 0.0;
        locals.var_eta_stress_dn8 = 0.0;
        locals.var_eta_stress_dn9 = 0.0;
        locals.var_eta_stress_dn10 = 0.0;
        locals.var_eta_stress_dn11 = 0.0;
        locals.var_eta_stress_dn12 = 0.0;
        locals.var_eta_stress_dn13 = 0.0;
        locals.var_eta_stress_dn14 = 0.0;
        locals.var_eta_stress_rv = 0.0;

        locals.var_local_sca = 0.0;
        locals.var_local_sca_dn0 = 0.0;
        locals.var_local_sca_dn2 = 0.0;
        locals.var_local_sca_dn3 = 0.0;
        locals.var_local_sca_dn4 = 0.0;
        locals.var_local_sca_dn5 = 0.0;
        locals.var_local_sca_dn6 = 0.0;
        locals.var_local_sca_dn7 = 0.0;
        locals.var_local_sca_dn8 = 0.0;
        locals.var_local_sca_dn9 = 0.0;
        locals.var_local_sca_dn10 = 0.0;
        locals.var_local_sca_dn11 = 0.0;
        locals.var_local_sca_dn12 = 0.0;
        locals.var_local_sca_dn13 = 0.0;
        locals.var_local_sca_dn14 = 0.0;
        locals.var_local_sca_rv = 0.0;

        locals.var_m0_i = 0.0;
        locals.var_m0_i_rv = 0.0;

        locals.var_m0_t = 0.0;
        locals.var_m0_t_dn4 = 0.0;
        locals.var_m0_t_rv = 0.0;

        locals.var_eta0edge_i = 0.0;
        locals.var_eta0edge_i_dn0 = 0.0;
        locals.var_eta0edge_i_dn2 = 0.0;
        locals.var_eta0edge_i_dn3 = 0.0;
        locals.var_eta0edge_i_dn4 = 0.0;
        locals.var_eta0edge_i_dn5 = 0.0;
        locals.var_eta0edge_i_dn6 = 0.0;
        locals.var_eta0edge_i_dn7 = 0.0;
        locals.var_eta0edge_i_dn8 = 0.0;
        locals.var_eta0edge_i_dn9 = 0.0;
        locals.var_eta0edge_i_dn10 = 0.0;
        locals.var_eta0edge_i_dn11 = 0.0;
        locals.var_eta0edge_i_dn12 = 0.0;
        locals.var_eta0edge_i_dn13 = 0.0;
        locals.var_eta0edge_i_dn14 = 0.0;
        locals.var_eta0edge_i_rv = 0.0;

        locals.var_kt2edge_i = 0.0;
        locals.var_kt2edge_i_rv = 0.0;

        locals.var_k2edge_i = 0.0;
        locals.var_k2edge_i_dn0 = 0.0;
        locals.var_k2edge_i_dn2 = 0.0;
        locals.var_k2edge_i_dn3 = 0.0;
        locals.var_k2edge_i_dn4 = 0.0;
        locals.var_k2edge_i_dn5 = 0.0;
        locals.var_k2edge_i_dn6 = 0.0;
        locals.var_k2edge_i_dn7 = 0.0;
        locals.var_k2edge_i_dn8 = 0.0;
        locals.var_k2edge_i_dn9 = 0.0;
        locals.var_k2edge_i_dn10 = 0.0;
        locals.var_k2edge_i_dn11 = 0.0;
        locals.var_k2edge_i_dn12 = 0.0;
        locals.var_k2edge_i_dn13 = 0.0;
        locals.var_k2edge_i_dn14 = 0.0;
        locals.var_k2edge_i_rv = 0.0;

        locals.var_mnud1 = 0.0;
        locals.var_mnud1_dn0 = 0.0;
        locals.var_mnud1_dn2 = 0.0;
        locals.var_mnud1_dn3 = 0.0;
        locals.var_mnud1_dn4 = 0.0;
        locals.var_mnud1_dn5 = 0.0;
        locals.var_mnud1_dn6 = 0.0;
        locals.var_mnud1_dn7 = 0.0;
        locals.var_mnud1_dn8 = 0.0;
        locals.var_mnud1_dn9 = 0.0;
        locals.var_mnud1_dn10 = 0.0;
        locals.var_mnud1_dn11 = 0.0;
        locals.var_mnud1_dn12 = 0.0;
        locals.var_mnud1_dn13 = 0.0;
        locals.var_mnud1_dn14 = 0.0;
        locals.var_mnud1_rv = 0.0;

        locals.var_c0si_i = 0.0;
        locals.var_c0si_i_rv = 0.0;

        locals.var_c0sisat1_i = 0.0;
        locals.var_c0sisat1_i_rv = 0.0;

        locals.var_eta0r_i = 0.0;
        locals.var_eta0r_i_dn0 = 0.0;
        locals.var_eta0r_i_dn2 = 0.0;
        locals.var_eta0r_i_dn3 = 0.0;
        locals.var_eta0r_i_dn4 = 0.0;
        locals.var_eta0r_i_dn5 = 0.0;
        locals.var_eta0r_i_dn6 = 0.0;
        locals.var_eta0r_i_dn7 = 0.0;
        locals.var_eta0r_i_dn8 = 0.0;
        locals.var_eta0r_i_dn9 = 0.0;
        locals.var_eta0r_i_dn10 = 0.0;
        locals.var_eta0r_i_dn11 = 0.0;
        locals.var_eta0r_i_dn12 = 0.0;
        locals.var_eta0r_i_dn13 = 0.0;
        locals.var_eta0r_i_dn14 = 0.0;
        locals.var_eta0r_i_rv = 0.0;

        locals.var_pclmr_i = 0.0;
        locals.var_pclmr_i_dn0 = 0.0;
        locals.var_pclmr_i_dn2 = 0.0;
        locals.var_pclmr_i_dn3 = 0.0;
        locals.var_pclmr_i_dn4 = 0.0;
        locals.var_pclmr_i_dn5 = 0.0;
        locals.var_pclmr_i_dn6 = 0.0;
        locals.var_pclmr_i_dn7 = 0.0;
        locals.var_pclmr_i_dn8 = 0.0;
        locals.var_pclmr_i_dn9 = 0.0;
        locals.var_pclmr_i_dn10 = 0.0;
        locals.var_pclmr_i_dn11 = 0.0;
        locals.var_pclmr_i_dn12 = 0.0;
        locals.var_pclmr_i_dn13 = 0.0;
        locals.var_pclmr_i_dn14 = 0.0;
        locals.var_pclmr_i_rv = 0.0;

        locals.var_ptwgr_t = 0.0;
        locals.var_ptwgr_t_dn0 = 0.0;
        locals.var_ptwgr_t_dn2 = 0.0;
        locals.var_ptwgr_t_dn3 = 0.0;
        locals.var_ptwgr_t_dn4 = 0.0;
        locals.var_ptwgr_t_dn5 = 0.0;
        locals.var_ptwgr_t_dn6 = 0.0;
        locals.var_ptwgr_t_dn7 = 0.0;
        locals.var_ptwgr_t_dn8 = 0.0;
        locals.var_ptwgr_t_dn9 = 0.0;
        locals.var_ptwgr_t_dn10 = 0.0;
        locals.var_ptwgr_t_dn11 = 0.0;
        locals.var_ptwgr_t_dn12 = 0.0;
        locals.var_ptwgr_t_dn13 = 0.0;
        locals.var_ptwgr_t_dn14 = 0.0;
        locals.var_ptwgr_t_rv = 0.0;

        locals.var_uar_t = 0.0;
        locals.var_uar_t_dn0 = 0.0;
        locals.var_uar_t_dn2 = 0.0;
        locals.var_uar_t_dn3 = 0.0;
        locals.var_uar_t_dn4 = 0.0;
        locals.var_uar_t_dn5 = 0.0;
        locals.var_uar_t_dn6 = 0.0;
        locals.var_uar_t_dn7 = 0.0;
        locals.var_uar_t_dn8 = 0.0;
        locals.var_uar_t_dn9 = 0.0;
        locals.var_uar_t_dn10 = 0.0;
        locals.var_uar_t_dn11 = 0.0;
        locals.var_uar_t_dn12 = 0.0;
        locals.var_uar_t_dn13 = 0.0;
        locals.var_uar_t_dn14 = 0.0;
        locals.var_uar_t_rv = 0.0;

        locals.var_ucsr_t = 0.0;
        locals.var_ucsr_t_dn4 = 0.0;
        locals.var_ucsr_t_rv = 0.0;

        locals.var_vsatr_i = 0.0;
        locals.var_vsatr_i_dn0 = 0.0;
        locals.var_vsatr_i_dn2 = 0.0;
        locals.var_vsatr_i_dn3 = 0.0;
        locals.var_vsatr_i_dn4 = 0.0;
        locals.var_vsatr_i_dn5 = 0.0;
        locals.var_vsatr_i_dn6 = 0.0;
        locals.var_vsatr_i_dn7 = 0.0;
        locals.var_vsatr_i_dn8 = 0.0;
        locals.var_vsatr_i_dn9 = 0.0;
        locals.var_vsatr_i_dn10 = 0.0;
        locals.var_vsatr_i_dn11 = 0.0;
        locals.var_vsatr_i_dn12 = 0.0;
        locals.var_vsatr_i_dn13 = 0.0;
        locals.var_vsatr_i_dn14 = 0.0;
        locals.var_vsatr_i_rv = 0.0;

        locals.var_inv_sb = 0.0;
        locals.var_inv_sb_dn0 = 0.0;
        locals.var_inv_sb_dn2 = 0.0;
        locals.var_inv_sb_dn3 = 0.0;
        locals.var_inv_sb_dn4 = 0.0;
        locals.var_inv_sb_dn5 = 0.0;
        locals.var_inv_sb_dn6 = 0.0;
        locals.var_inv_sb_dn7 = 0.0;
        locals.var_inv_sb_dn8 = 0.0;
        locals.var_inv_sb_dn9 = 0.0;
        locals.var_inv_sb_dn10 = 0.0;
        locals.var_inv_sb_dn11 = 0.0;
        locals.var_inv_sb_dn12 = 0.0;
        locals.var_inv_sb_dn13 = 0.0;
        locals.var_inv_sb_dn14 = 0.0;
        locals.var_inv_sb_rv = 0.0;

        locals.var_local_scb = 0.0;
        locals.var_local_scb_dn0 = 0.0;
        locals.var_local_scb_dn2 = 0.0;
        locals.var_local_scb_dn3 = 0.0;
        locals.var_local_scb_dn4 = 0.0;
        locals.var_local_scb_dn5 = 0.0;
        locals.var_local_scb_dn6 = 0.0;
        locals.var_local_scb_dn7 = 0.0;
        locals.var_local_scb_dn8 = 0.0;
        locals.var_local_scb_dn9 = 0.0;
        locals.var_local_scb_dn10 = 0.0;
        locals.var_local_scb_dn11 = 0.0;
        locals.var_local_scb_dn12 = 0.0;
        locals.var_local_scb_dn13 = 0.0;
        locals.var_local_scb_dn14 = 0.0;
        locals.var_local_scb_rv = 0.0;

        locals.var_k01_i = 0.0;
        locals.var_k01_i_rv = 0.0;

        locals.var_citedge_i = 0.0;
        locals.var_citedge_i_rv = 0.0;

        locals.var_etabedge_i = 0.0;
        locals.var_etabedge_i_rv = 0.0;

        locals.var_kt1expedge_i = 0.0;
        locals.var_kt1expedge_i_rv = 0.0;

        locals.var_kvth0edge_i = 0.0;
        locals.var_kvth0edge_i_rv = 0.0;

        locals.var_c0_i = 0.0;
        locals.var_c0_i_rv = 0.0;

        locals.var_c0si1_i = 0.0;
        locals.var_c0si1_i_rv = 0.0;

        locals.var_c0sisat_t = 0.0;
        locals.var_c0sisat_t_dn4 = 0.0;
        locals.var_c0sisat_t_rv = 0.0;

        locals.var_rdstemphv = 1.0;
        locals.var_rdstemphv_dn4 = 0.0;
        locals.var_rdstemphv_rv = 0.0;

        locals.var_eta0r_t = 0.0;
        locals.var_eta0r_t_dn0 = 0.0;
        locals.var_eta0r_t_dn2 = 0.0;
        locals.var_eta0r_t_dn3 = 0.0;
        locals.var_eta0r_t_dn4 = 0.0;
        locals.var_eta0r_t_dn5 = 0.0;
        locals.var_eta0r_t_dn6 = 0.0;
        locals.var_eta0r_t_dn7 = 0.0;
        locals.var_eta0r_t_dn8 = 0.0;
        locals.var_eta0r_t_dn9 = 0.0;
        locals.var_eta0r_t_dn10 = 0.0;
        locals.var_eta0r_t_dn11 = 0.0;
        locals.var_eta0r_t_dn12 = 0.0;
        locals.var_eta0r_t_dn13 = 0.0;
        locals.var_eta0r_t_dn14 = 0.0;
        locals.var_eta0r_t_rv = 0.0;

        locals.var_pdiblcr_i = 0.0;
        locals.var_pdiblcr_i_dn0 = 0.0;
        locals.var_pdiblcr_i_dn2 = 0.0;
        locals.var_pdiblcr_i_dn3 = 0.0;
        locals.var_pdiblcr_i_dn4 = 0.0;
        locals.var_pdiblcr_i_dn5 = 0.0;
        locals.var_pdiblcr_i_dn6 = 0.0;
        locals.var_pdiblcr_i_dn7 = 0.0;
        locals.var_pdiblcr_i_dn8 = 0.0;
        locals.var_pdiblcr_i_dn9 = 0.0;
        locals.var_pdiblcr_i_dn10 = 0.0;
        locals.var_pdiblcr_i_dn11 = 0.0;
        locals.var_pdiblcr_i_dn12 = 0.0;
        locals.var_pdiblcr_i_dn13 = 0.0;
        locals.var_pdiblcr_i_dn14 = 0.0;
        locals.var_pdiblcr_i_rv = 0.0;

        locals.var_u0r_i = 0.0;
        locals.var_u0r_i_rv = 0.0;

        locals.var_ucr_i = 0.0;
        locals.var_ucr_i_dn0 = 0.0;
        locals.var_ucr_i_dn2 = 0.0;
        locals.var_ucr_i_dn3 = 0.0;
        locals.var_ucr_i_dn4 = 0.0;
        locals.var_ucr_i_dn5 = 0.0;
        locals.var_ucr_i_dn6 = 0.0;
        locals.var_ucr_i_dn7 = 0.0;
        locals.var_ucr_i_dn8 = 0.0;
        locals.var_ucr_i_dn9 = 0.0;
        locals.var_ucr_i_dn10 = 0.0;
        locals.var_ucr_i_dn11 = 0.0;
        locals.var_ucr_i_dn12 = 0.0;
        locals.var_ucr_i_dn13 = 0.0;
        locals.var_ucr_i_dn14 = 0.0;
        locals.var_ucr_i_rv = 0.0;

        locals.var_udr_i = 0.0;
        locals.var_udr_i_dn0 = 0.0;
        locals.var_udr_i_dn2 = 0.0;
        locals.var_udr_i_dn3 = 0.0;
        locals.var_udr_i_dn4 = 0.0;
        locals.var_udr_i_dn5 = 0.0;
        locals.var_udr_i_dn6 = 0.0;
        locals.var_udr_i_dn7 = 0.0;
        locals.var_udr_i_dn8 = 0.0;
        locals.var_udr_i_dn9 = 0.0;
        locals.var_udr_i_dn10 = 0.0;
        locals.var_udr_i_dn11 = 0.0;
        locals.var_udr_i_dn12 = 0.0;
        locals.var_udr_i_dn13 = 0.0;
        locals.var_udr_i_dn14 = 0.0;
        locals.var_udr_i_rv = 0.0;

        locals.var_vsatr_t = 0.0;
        locals.var_vsatr_t_dn0 = 0.0;
        locals.var_vsatr_t_dn2 = 0.0;
        locals.var_vsatr_t_dn3 = 0.0;
        locals.var_vsatr_t_dn4 = 0.0;
        locals.var_vsatr_t_dn5 = 0.0;
        locals.var_vsatr_t_dn6 = 0.0;
        locals.var_vsatr_t_dn7 = 0.0;
        locals.var_vsatr_t_dn8 = 0.0;
        locals.var_vsatr_t_dn9 = 0.0;
        locals.var_vsatr_t_dn10 = 0.0;
        locals.var_vsatr_t_dn11 = 0.0;
        locals.var_vsatr_t_dn12 = 0.0;
        locals.var_vsatr_t_dn13 = 0.0;
        locals.var_vsatr_t_dn14 = 0.0;
        locals.var_vsatr_t_rv = 0.0;

        locals.var_vth0_stress_edge = 0.0;
        locals.var_vth0_stress_edge_dn0 = 0.0;
        locals.var_vth0_stress_edge_dn2 = 0.0;
        locals.var_vth0_stress_edge_dn3 = 0.0;
        locals.var_vth0_stress_edge_dn4 = 0.0;
        locals.var_vth0_stress_edge_dn5 = 0.0;
        locals.var_vth0_stress_edge_dn6 = 0.0;
        locals.var_vth0_stress_edge_dn7 = 0.0;
        locals.var_vth0_stress_edge_dn8 = 0.0;
        locals.var_vth0_stress_edge_dn9 = 0.0;
        locals.var_vth0_stress_edge_dn10 = 0.0;
        locals.var_vth0_stress_edge_dn11 = 0.0;
        locals.var_vth0_stress_edge_dn12 = 0.0;
        locals.var_vth0_stress_edge_dn13 = 0.0;
        locals.var_vth0_stress_edge_dn14 = 0.0;
        locals.var_vth0_stress_edge_rv = 0.0;

        locals.var_eta_stress_edge = 0.0;
        locals.var_eta_stress_edge_dn0 = 0.0;
        locals.var_eta_stress_edge_dn2 = 0.0;
        locals.var_eta_stress_edge_dn3 = 0.0;
        locals.var_eta_stress_edge_dn4 = 0.0;
        locals.var_eta_stress_edge_dn5 = 0.0;
        locals.var_eta_stress_edge_dn6 = 0.0;
        locals.var_eta_stress_edge_dn7 = 0.0;
        locals.var_eta_stress_edge_dn8 = 0.0;
        locals.var_eta_stress_edge_dn9 = 0.0;
        locals.var_eta_stress_edge_dn10 = 0.0;
        locals.var_eta_stress_edge_dn11 = 0.0;
        locals.var_eta_stress_edge_dn12 = 0.0;
        locals.var_eta_stress_edge_dn13 = 0.0;
        locals.var_eta_stress_edge_dn14 = 0.0;
        locals.var_eta_stress_edge_rv = 0.0;

        locals.var_local_scc = 0.0;
        locals.var_local_scc_dn0 = 0.0;
        locals.var_local_scc_dn2 = 0.0;
        locals.var_local_scc_dn3 = 0.0;
        locals.var_local_scc_dn4 = 0.0;
        locals.var_local_scc_dn5 = 0.0;
        locals.var_local_scc_dn6 = 0.0;
        locals.var_local_scc_dn7 = 0.0;
        locals.var_local_scc_dn8 = 0.0;
        locals.var_local_scc_dn9 = 0.0;
        locals.var_local_scc_dn10 = 0.0;
        locals.var_local_scc_dn11 = 0.0;
        locals.var_local_scc_dn12 = 0.0;
        locals.var_local_scc_dn13 = 0.0;
        locals.var_local_scc_dn14 = 0.0;
        locals.var_local_scc_rv = 0.0;

        locals.var_m01_i = 0.0;
        locals.var_m01_i_rv = 0.0;

        locals.var_cdscdedge_i = 0.0;
        locals.var_cdscdedge_i_rv = 0.0;

        locals.var_kt1edge_i = 0.0;
        locals.var_kt1edge_i_rv = 0.0;

        locals.var_tnfactoredge_i = 0.0;
        locals.var_tnfactoredge_i_rv = 0.0;

        locals.var_stk2edge_i = 0.0;
        locals.var_stk2edge_i_rv = 0.0;

        locals.var_c01_i = 0.0;
        locals.var_c01_i_rv = 0.0;

        locals.var_c0si_t = 0.0;
        locals.var_c0si_t_dn4 = 0.0;
        locals.var_c0si_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        locals.var_rdrift_d = 0.0;
        locals.var_rdrift_d_dn0 = 0.0;
        locals.var_rdrift_d_dn2 = 0.0;
        locals.var_rdrift_d_dn3 = 0.0;
        locals.var_rdrift_d_dn4 = 0.0;
        locals.var_rdrift_d_dn5 = 0.0;
        locals.var_rdrift_d_dn6 = 0.0;
        locals.var_rdrift_d_dn7 = 0.0;
        locals.var_rdrift_d_dn8 = 0.0;
        locals.var_rdrift_d_dn9 = 0.0;
        locals.var_rdrift_d_dn10 = 0.0;
        locals.var_rdrift_d_dn11 = 0.0;
        locals.var_rdrift_d_dn12 = 0.0;
        locals.var_rdrift_d_dn13 = 0.0;
        locals.var_rdrift_d_dn14 = 0.0;
        locals.var_rdrift_d_rv = 0.0;

        locals.var_vdrift_t = 1.0;
        locals.var_vdrift_t_dn4 = 0.0;
        locals.var_vdrift_t_rv = 0.0;

        locals.var_l_lln1 = 0.0;
        locals.var_l_lln1_rv = 0.0;

        locals.var_psatr_i = 0.0;
        locals.var_psatr_i_rv = 0.0;

        locals.var_u0r_t = 0.0;
        locals.var_u0r_t_dn4 = 0.0;
        locals.var_u0r_t_rv = 0.0;

        locals.var_ucr_t = 0.0;
        locals.var_ucr_t_dn0 = 0.0;
        locals.var_ucr_t_dn2 = 0.0;
        locals.var_ucr_t_dn3 = 0.0;
        locals.var_ucr_t_dn4 = 0.0;
        locals.var_ucr_t_dn5 = 0.0;
        locals.var_ucr_t_dn6 = 0.0;
        locals.var_ucr_t_dn7 = 0.0;
        locals.var_ucr_t_dn8 = 0.0;
        locals.var_ucr_t_dn9 = 0.0;
        locals.var_ucr_t_dn10 = 0.0;
        locals.var_ucr_t_dn11 = 0.0;
        locals.var_ucr_t_dn12 = 0.0;
        locals.var_ucr_t_dn13 = 0.0;
        locals.var_ucr_t_dn14 = 0.0;
        locals.var_ucr_t_rv = 0.0;

        locals.var_udr_t = 0.0;
        locals.var_udr_t_dn0 = 0.0;
        locals.var_udr_t_dn2 = 0.0;
        locals.var_udr_t_dn3 = 0.0;
        locals.var_udr_t_dn4 = 0.0;
        locals.var_udr_t_dn5 = 0.0;
        locals.var_udr_t_dn6 = 0.0;
        locals.var_udr_t_dn7 = 0.0;
        locals.var_udr_t_dn8 = 0.0;
        locals.var_udr_t_dn9 = 0.0;
        locals.var_udr_t_dn10 = 0.0;
        locals.var_udr_t_dn11 = 0.0;
        locals.var_udr_t_dn12 = 0.0;
        locals.var_udr_t_dn13 = 0.0;
        locals.var_udr_t_dn14 = 0.0;
        locals.var_udr_t_rv = 0.0;

        locals.var_w_lwn1 = 0.0;
        locals.var_w_lwn1_rv = 0.0;

        locals.var_k2_stress_edge = 0.0;
        locals.var_k2_stress_edge_dn0 = 0.0;
        locals.var_k2_stress_edge_dn2 = 0.0;
        locals.var_k2_stress_edge_dn3 = 0.0;
        locals.var_k2_stress_edge_dn4 = 0.0;
        locals.var_k2_stress_edge_dn5 = 0.0;
        locals.var_k2_stress_edge_dn6 = 0.0;
        locals.var_k2_stress_edge_dn7 = 0.0;
        locals.var_k2_stress_edge_dn8 = 0.0;
        locals.var_k2_stress_edge_dn9 = 0.0;
        locals.var_k2_stress_edge_dn10 = 0.0;
        locals.var_k2_stress_edge_dn11 = 0.0;
        locals.var_k2_stress_edge_dn12 = 0.0;
        locals.var_k2_stress_edge_dn13 = 0.0;
        locals.var_k2_stress_edge_dn14 = 0.0;
        locals.var_k2_stress_edge_rv = 0.0;

        locals.var_k0_i = 0.0;
        locals.var_k0_i_rv = 0.0;

        locals.var_k0_t = 0.0;
        locals.var_k0_t_dn4 = 0.0;
        locals.var_k0_t_rv = 0.0;

        locals.var_cdscbedge_i = 0.0;
        locals.var_cdscbedge_i_rv = 0.0;

        locals.var_kt1ledge_i = 0.0;
        locals.var_kt1ledge_i_rv = 0.0;

        locals.var_teta0edge_i = 0.0;
        locals.var_teta0edge_i_rv = 0.0;

        locals.var_steta0edge_i = 0.0;
        locals.var_steta0edge_i_rv = 0.0;

        locals.var_c0_t = 0.0;
        locals.var_c0_t_dn4 = 0.0;
        locals.var_c0_t_rv = 0.0;

        locals.var_c0sisat_i = 0.0;
        locals.var_c0sisat_i_rv = 0.0;

        locals.var_rdrift_s = 0.0;
        locals.var_rdrift_s_dn0 = 0.0;
        locals.var_rdrift_s_dn2 = 0.0;
        locals.var_rdrift_s_dn3 = 0.0;
        locals.var_rdrift_s_dn4 = 0.0;
        locals.var_rdrift_s_dn5 = 0.0;
        locals.var_rdrift_s_dn6 = 0.0;
        locals.var_rdrift_s_dn7 = 0.0;
        locals.var_rdrift_s_dn8 = 0.0;
        locals.var_rdrift_s_dn9 = 0.0;
        locals.var_rdrift_s_dn10 = 0.0;
        locals.var_rdrift_s_dn11 = 0.0;
        locals.var_rdrift_s_dn12 = 0.0;
        locals.var_rdrift_s_dn13 = 0.0;
        locals.var_rdrift_s_dn14 = 0.0;
        locals.var_rdrift_s_rv = 0.0;

        locals.var_k2edgewe_i = 0.0;
        locals.var_k2edgewe_i_rv = 0.0;

        locals.var_kvth0edgewe_i = 0.0;
        locals.var_kvth0edgewe_i_rv = 0.0;

        locals.var_temp_adeff = 0.0;
        locals.var_temp_adeff_dn0 = 0.0;
        locals.var_temp_adeff_dn2 = 0.0;
        locals.var_temp_adeff_dn3 = 0.0;
        locals.var_temp_adeff_dn4 = 0.0;
        locals.var_temp_adeff_dn5 = 0.0;
        locals.var_temp_adeff_dn6 = 0.0;
        locals.var_temp_adeff_dn7 = 0.0;
        locals.var_temp_adeff_dn8 = 0.0;
        locals.var_temp_adeff_dn9 = 0.0;
        locals.var_temp_adeff_dn10 = 0.0;
        locals.var_temp_adeff_dn11 = 0.0;
        locals.var_temp_adeff_dn12 = 0.0;
        locals.var_temp_adeff_dn13 = 0.0;
        locals.var_temp_adeff_dn14 = 0.0;
        locals.var_temp_adeff_rv = 0.0;

        locals.var_temp_aseff = 0.0;
        locals.var_temp_aseff_dn0 = 0.0;
        locals.var_temp_aseff_dn2 = 0.0;
        locals.var_temp_aseff_dn3 = 0.0;
        locals.var_temp_aseff_dn4 = 0.0;
        locals.var_temp_aseff_dn5 = 0.0;
        locals.var_temp_aseff_dn6 = 0.0;
        locals.var_temp_aseff_dn7 = 0.0;
        locals.var_temp_aseff_dn8 = 0.0;
        locals.var_temp_aseff_dn9 = 0.0;
        locals.var_temp_aseff_dn10 = 0.0;
        locals.var_temp_aseff_dn11 = 0.0;
        locals.var_temp_aseff_dn12 = 0.0;
        locals.var_temp_aseff_dn13 = 0.0;
        locals.var_temp_aseff_dn14 = 0.0;
        locals.var_temp_aseff_rv = 0.0;

        locals.var_temp_pdeff = 0.0;
        locals.var_temp_pdeff_dn0 = 0.0;
        locals.var_temp_pdeff_dn2 = 0.0;
        locals.var_temp_pdeff_dn3 = 0.0;
        locals.var_temp_pdeff_dn4 = 0.0;
        locals.var_temp_pdeff_dn5 = 0.0;
        locals.var_temp_pdeff_dn6 = 0.0;
        locals.var_temp_pdeff_dn7 = 0.0;
        locals.var_temp_pdeff_dn8 = 0.0;
        locals.var_temp_pdeff_dn9 = 0.0;
        locals.var_temp_pdeff_dn10 = 0.0;
        locals.var_temp_pdeff_dn11 = 0.0;
        locals.var_temp_pdeff_dn12 = 0.0;
        locals.var_temp_pdeff_dn13 = 0.0;
        locals.var_temp_pdeff_dn14 = 0.0;
        locals.var_temp_pdeff_rv = 0.0;

        locals.var_temp_pseff = 0.0;
        locals.var_temp_pseff_dn0 = 0.0;
        locals.var_temp_pseff_dn2 = 0.0;
        locals.var_temp_pseff_dn3 = 0.0;
        locals.var_temp_pseff_dn4 = 0.0;
        locals.var_temp_pseff_dn5 = 0.0;
        locals.var_temp_pseff_dn6 = 0.0;
        locals.var_temp_pseff_dn7 = 0.0;
        locals.var_temp_pseff_dn8 = 0.0;
        locals.var_temp_pseff_dn9 = 0.0;
        locals.var_temp_pseff_dn10 = 0.0;
        locals.var_temp_pseff_dn11 = 0.0;
        locals.var_temp_pseff_dn12 = 0.0;
        locals.var_temp_pseff_dn13 = 0.0;
        locals.var_temp_pseff_dn14 = 0.0;
        locals.var_temp_pseff_rv = 0.0;

        locals.var_abulkiv = 1.0;
        locals.var_abulkiv_dn0 = 0.0;
        locals.var_abulkiv_dn2 = 0.0;
        locals.var_abulkiv_dn3 = 0.0;
        locals.var_abulkiv_dn4 = 0.0;
        locals.var_abulkiv_dn5 = 0.0;
        locals.var_abulkiv_dn6 = 0.0;
        locals.var_abulkiv_dn7 = 0.0;
        locals.var_abulkiv_dn8 = 0.0;
        locals.var_abulkiv_dn9 = 0.0;
        locals.var_abulkiv_dn10 = 0.0;
        locals.var_abulkiv_dn11 = 0.0;
        locals.var_abulkiv_dn12 = 0.0;
        locals.var_abulkiv_dn13 = 0.0;
        locals.var_abulkiv_dn14 = 0.0;
        locals.var_abulkiv_rv = 0.0;

        locals.var_abulkcv = 1.0;
        locals.var_abulkcv_dn0 = 0.0;
        locals.var_abulkcv_dn2 = 0.0;
        locals.var_abulkcv_dn3 = 0.0;
        locals.var_abulkcv_dn4 = 0.0;
        locals.var_abulkcv_dn5 = 0.0;
        locals.var_abulkcv_dn6 = 0.0;
        locals.var_abulkcv_dn7 = 0.0;
        locals.var_abulkcv_dn8 = 0.0;
        locals.var_abulkcv_dn9 = 0.0;
        locals.var_abulkcv_dn10 = 0.0;
        locals.var_abulkcv_dn11 = 0.0;
        locals.var_abulkcv_dn12 = 0.0;
        locals.var_abulkcv_dn13 = 0.0;
        locals.var_abulkcv_dn14 = 0.0;
        locals.var_abulkcv_rv = 0.0;

        locals.var_gdpr = 0.0;
        locals.var_gdpr_dn0 = 0.0;
        locals.var_gdpr_dn2 = 0.0;
        locals.var_gdpr_dn3 = 0.0;
        locals.var_gdpr_dn4 = 0.0;
        locals.var_gdpr_dn5 = 0.0;
        locals.var_gdpr_dn6 = 0.0;
        locals.var_gdpr_dn7 = 0.0;
        locals.var_gdpr_dn8 = 0.0;
        locals.var_gdpr_dn9 = 0.0;
        locals.var_gdpr_dn10 = 0.0;
        locals.var_gdpr_dn11 = 0.0;
        locals.var_gdpr_dn12 = 0.0;
        locals.var_gdpr_dn13 = 0.0;
        locals.var_gdpr_dn14 = 0.0;
        locals.var_gdpr_rv = 0.0;

        locals.var_gspr = 0.0;
        locals.var_gspr_dn0 = 0.0;
        locals.var_gspr_dn2 = 0.0;
        locals.var_gspr_dn3 = 0.0;
        locals.var_gspr_dn4 = 0.0;
        locals.var_gspr_dn5 = 0.0;
        locals.var_gspr_dn6 = 0.0;
        locals.var_gspr_dn7 = 0.0;
        locals.var_gspr_dn8 = 0.0;
        locals.var_gspr_dn9 = 0.0;
        locals.var_gspr_dn10 = 0.0;
        locals.var_gspr_dn11 = 0.0;
        locals.var_gspr_dn12 = 0.0;
        locals.var_gspr_dn13 = 0.0;
        locals.var_gspr_dn14 = 0.0;
        locals.var_gspr_rv = 0.0;

        locals.var_gdrift_d = 0.0;
        locals.var_gdrift_d_dn0 = 0.0;
        locals.var_gdrift_d_dn2 = 0.0;
        locals.var_gdrift_d_dn3 = 0.0;
        locals.var_gdrift_d_dn4 = 0.0;
        locals.var_gdrift_d_dn5 = 0.0;
        locals.var_gdrift_d_dn6 = 0.0;
        locals.var_gdrift_d_dn7 = 0.0;
        locals.var_gdrift_d_dn8 = 0.0;
        locals.var_gdrift_d_dn9 = 0.0;
        locals.var_gdrift_d_dn10 = 0.0;
        locals.var_gdrift_d_dn11 = 0.0;
        locals.var_gdrift_d_dn12 = 0.0;
        locals.var_gdrift_d_dn13 = 0.0;
        locals.var_gdrift_d_dn14 = 0.0;
        locals.var_gdrift_d_rv = 0.0;

        locals.var_gdrift_s = 0.0;
        locals.var_gdrift_s_dn0 = 0.0;
        locals.var_gdrift_s_dn2 = 0.0;
        locals.var_gdrift_s_dn3 = 0.0;
        locals.var_gdrift_s_dn4 = 0.0;
        locals.var_gdrift_s_dn5 = 0.0;
        locals.var_gdrift_s_dn6 = 0.0;
        locals.var_gdrift_s_dn7 = 0.0;
        locals.var_gdrift_s_dn8 = 0.0;
        locals.var_gdrift_s_dn9 = 0.0;
        locals.var_gdrift_s_dn10 = 0.0;
        locals.var_gdrift_s_dn11 = 0.0;
        locals.var_gdrift_s_dn12 = 0.0;
        locals.var_gdrift_s_dn13 = 0.0;
        locals.var_gdrift_s_dn14 = 0.0;
        locals.var_gdrift_s_rv = 0.0;

        locals.var_vd1 = 0.0;
        locals.var_vd1_dn6 = 0.0;
        locals.var_vd1_dn11 = 0.0;
        locals.var_vd1_rv = 0.0;

        locals.var_vs1 = 0.0;
        locals.var_vs1_dn8 = 0.0;
        locals.var_vs1_dn11 = 0.0;
        locals.var_vs1_rv = 0.0;

        locals.var_idrift_sat_d = 0.0;
        locals.var_idrift_sat_d_dn0 = 0.0;
        locals.var_idrift_sat_d_dn2 = 0.0;
        locals.var_idrift_sat_d_dn3 = 0.0;
        locals.var_idrift_sat_d_dn4 = 0.0;
        locals.var_idrift_sat_d_dn5 = 0.0;
        locals.var_idrift_sat_d_dn6 = 0.0;
        locals.var_idrift_sat_d_dn7 = 0.0;
        locals.var_idrift_sat_d_dn8 = 0.0;
        locals.var_idrift_sat_d_dn9 = 0.0;
        locals.var_idrift_sat_d_dn10 = 0.0;
        locals.var_idrift_sat_d_dn11 = 0.0;
        locals.var_idrift_sat_d_dn12 = 0.0;
        locals.var_idrift_sat_d_dn13 = 0.0;
        locals.var_idrift_sat_d_dn14 = 0.0;
        locals.var_idrift_sat_d_rv = 0.0;

        locals.var_ln_t1_t2 = 0.0;
        locals.var_ln_t1_t2_dn0 = 0.0;
        locals.var_ln_t1_t2_dn2 = 0.0;
        locals.var_ln_t1_t2_dn3 = 0.0;
        locals.var_ln_t1_t2_dn4 = 0.0;
        locals.var_ln_t1_t2_dn5 = 0.0;
        locals.var_ln_t1_t2_dn6 = 0.0;
        locals.var_ln_t1_t2_dn7 = 0.0;
        locals.var_ln_t1_t2_dn8 = 0.0;
        locals.var_ln_t1_t2_dn9 = 0.0;
        locals.var_ln_t1_t2_dn10 = 0.0;
        locals.var_ln_t1_t2_dn11 = 0.0;
        locals.var_ln_t1_t2_dn12 = 0.0;
        locals.var_ln_t1_t2_dn13 = 0.0;
        locals.var_ln_t1_t2_dn14 = 0.0;
        locals.var_ln_t1_t2_rv = 0.0;

        locals.var_vdseffii = 0.0;
        locals.var_vdseffii_dn0 = 0.0;
        locals.var_vdseffii_dn2 = 0.0;
        locals.var_vdseffii_dn3 = 0.0;
        locals.var_vdseffii_dn4 = 0.0;
        locals.var_vdseffii_dn5 = 0.0;
        locals.var_vdseffii_dn6 = 0.0;
        locals.var_vdseffii_dn7 = 0.0;
        locals.var_vdseffii_dn8 = 0.0;
        locals.var_vdseffii_dn9 = 0.0;
        locals.var_vdseffii_dn10 = 0.0;
        locals.var_vdseffii_dn11 = 0.0;
        locals.var_vdseffii_dn12 = 0.0;
        locals.var_vdseffii_dn13 = 0.0;
        locals.var_vdseffii_dn14 = 0.0;
        locals.var_vdseffii_rv = 0.0;

        locals.var_beta0r_t = 0.0;
        locals.var_beta0r_t_dn4 = 0.0;
        locals.var_beta0r_t_rv = 0.0;

        locals.var_alpha0r_i = 0.0;
        locals.var_alpha0r_i_dn0 = 0.0;
        locals.var_alpha0r_i_dn2 = 0.0;
        locals.var_alpha0r_i_dn3 = 0.0;
        locals.var_alpha0r_i_dn4 = 0.0;
        locals.var_alpha0r_i_dn5 = 0.0;
        locals.var_alpha0r_i_dn6 = 0.0;
        locals.var_alpha0r_i_dn7 = 0.0;
        locals.var_alpha0r_i_dn8 = 0.0;
        locals.var_alpha0r_i_dn9 = 0.0;
        locals.var_alpha0r_i_dn10 = 0.0;
        locals.var_alpha0r_i_dn11 = 0.0;
        locals.var_alpha0r_i_dn12 = 0.0;
        locals.var_alpha0r_i_dn13 = 0.0;
        locals.var_alpha0r_i_dn14 = 0.0;
        locals.var_alpha0r_i_rv = 0.0;

        locals.var_beta0r_i = 0.0;
        locals.var_beta0r_i_rv = 0.0;

        locals.var_vb_cm = 0.0;
        locals.var_vb_cm_dn3 = 0.0;
        locals.var_vb_cm_dn11 = 0.0;
        locals.var_vb_cm_rv = 0.0;

        let assign940_e2092: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign940_e2092;
        locals.var_guard1_rv = 0.0;

        let (assign950_e2096,) = {
    if (locals.var_guard1 != 0.0) {
        (1.0,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign950_e2096;
        locals.var_devsign_rv = 0.0;

        let (assign960_e2102,) = {
    if (locals.var_guard1 == 0.0) {
        let assign960_e2100: f64 = (-1.0);
        (assign960_e2100,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign960_e2102;
        locals.var_devsign_rv = 0.0;

        let assign970_e2105: f64 = (p.p110 * 8.85418e-12);
        locals.var_epssi = assign970_e2105;
        locals.var_epssi_rv = 0.0;

        let assign980_e2108: f64 = (p.p111 * 8.85418e-12);
        locals.var_epsox = assign980_e2108;
        locals.var_epsox_rv = 0.0;

        let assign990_e2111: f64 = (p.p111 * 8.85418e-12);
        let assign990_e2113: f64 = (assign990_e2111 / p.p77);
        locals.var_cox = assign990_e2113;
        locals.var_cox_rv = 0.0;

        let assign1000_e2116: f64 = (p.p110 / p.p111);
        locals.var_epsratio = assign1000_e2116;
        locals.var_epsratio_rv = 0.0;

        let assign1010_e2119: f64 = if (!param_given[78]) { 1.0 } else { 0.0 };
        locals.var_guard2 = assign1010_e2119;
        locals.var_guard2_rv = 0.0;

        let (assign1020_e2129,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1020_e2123: f64 = (p.p77 * p.p111);
        let assign1020_e2125: f64 = (assign1020_e2123 / 3.9);
        let assign1020_e2127: f64 = (assign1020_e2125 - p.p79);
        (assign1020_e2127,)
    } else {
        (locals.var_bsimbulktoxp,)
    }
};
        locals.var_bsimbulktoxp = assign1020_e2129;
        locals.var_bsimbulktoxp_rv = 0.0;

        let (assign1030_e2134,) = {
    if (locals.var_guard2 == 0.0) {
        (p.p78,)
    } else {
        (locals.var_bsimbulktoxp,)
    }
};
        locals.var_bsimbulktoxp = assign1030_e2134;
        locals.var_bsimbulktoxp_rv = 0.0;

        let assign1040_e2137: f64 = (p.p0 * p.p52);
        locals.var_l_mult = assign1040_e2137;
        locals.var_l_mult_rv = 0.0;

        let assign1050_e2140: f64 = (p.p1 * p.p53);
        locals.var_w_mult = assign1050_e2140;
        locals.var_w_mult_rv = 0.0;

        let assign1060_e2143: f64 = (locals.var_l_mult + p.p54);
        locals.var_lnew = assign1060_e2143;
        locals.var_lnew_rv = 0.0;

        let assign1080_e2149: f64 = (locals.var_w_mult / p.p2);
        locals.var_w_by_nf = assign1080_e2149;
        locals.var_w_by_nf_rv = 0.0;

        let assign1090_e2152: f64 = (locals.var_w_by_nf + p.p56);
        locals.var_wnew = assign1090_e2152;
        locals.var_wnew_rv = 0.0;

        let assign1110_e2158: f64 = (-p.p61);
        let assign1110_e2159: f64 = (locals.var_lnew).powf(assign1110_e2158);
        locals.var_l_lln = assign1110_e2159;
        locals.var_l_lln_rv = 0.0;

        let assign1120_e2162: f64 = (-p.p62);
        let assign1120_e2163: f64 = (locals.var_wnew).powf(assign1120_e2162);
        locals.var_w_lwn = assign1120_e2163;
        locals.var_w_lwn_rv = 0.0;

        let assign1130_e2166: f64 = (locals.var_l_lln * locals.var_w_lwn);
        locals.var_lw_lln_lwn = assign1130_e2166;
        locals.var_lw_lln_lwn_rv = 0.0;

        let assign1140_e2170: f64 = (p.p58 * locals.var_l_lln);
        let assign1140_e2171: f64 = (p.p57 + assign1140_e2170);
        let assign1140_e2174: f64 = (p.p59 * locals.var_w_lwn);
        let assign1140_e2175: f64 = (assign1140_e2171 + assign1140_e2174);
        let assign1140_e2178: f64 = (p.p60 * locals.var_lw_lln_lwn);
        let assign1140_e2179: f64 = (assign1140_e2175 + assign1140_e2178);
        locals.var_dliv = assign1140_e2179;
        locals.var_dliv_rv = 0.0;

        let assign1150_e2182: f64 = (-p.p67);
        let assign1150_e2183: f64 = (locals.var_lnew).powf(assign1150_e2182);
        locals.var_l_wln = assign1150_e2183;
        locals.var_l_wln_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1160_e2186: f64 = (-p.p68);
        let assign1160_e2187: f64 = (locals.var_wnew).powf(assign1160_e2186);
        locals.var_w_wwn = assign1160_e2187;
        locals.var_w_wwn_rv = 0.0;

        let assign1170_e2190: f64 = (locals.var_l_wln * locals.var_w_wwn);
        locals.var_lw_wln_wwn = assign1170_e2190;
        locals.var_lw_wln_wwn_rv = 0.0;

        let assign1180_e2194: f64 = (p.p64 * locals.var_l_wln);
        let assign1180_e2195: f64 = (p.p63 + assign1180_e2194);
        let assign1180_e2198: f64 = (p.p65 * locals.var_w_wwn);
        let assign1180_e2199: f64 = (assign1180_e2195 + assign1180_e2198);
        let assign1180_e2202: f64 = (p.p66 * locals.var_lw_wln_wwn);
        let assign1180_e2203: f64 = (assign1180_e2199 + assign1180_e2202);
        locals.var_dwiv = assign1180_e2203;
        locals.var_dwiv_rv = 0.0;

        let assign1190_e2207: f64 = (2.0 * locals.var_dliv);
        let assign1190_e2208: f64 = (locals.var_lnew - assign1190_e2207);
        locals.var_leff = assign1190_e2208;
        locals.var_leff_rv = 0.0;

        let assign1220_e2218: f64 = (2.0 * locals.var_dwiv);
        let assign1220_e2219: f64 = (locals.var_wnew - assign1220_e2218);
        locals.var_weff = assign1220_e2219;
        locals.var_weff_rv = 0.0;

        let assign1250_e2229: f64 = (p.p70 * locals.var_l_lln);
        let assign1250_e2230: f64 = (p.p69 + assign1250_e2229);
        let assign1250_e2233: f64 = (p.p71 * locals.var_w_lwn);
        let assign1250_e2234: f64 = (assign1250_e2230 + assign1250_e2233);
        let assign1250_e2237: f64 = (p.p72 * locals.var_lw_lln_lwn);
        let assign1250_e2238: f64 = (assign1250_e2234 + assign1250_e2237);
        locals.var_dlcv = assign1250_e2238;
        locals.var_dlcv_rv = 0.0;

        let assign1260_e2242: f64 = (p.p74 * locals.var_l_wln);
        let assign1260_e2243: f64 = (p.p73 + assign1260_e2242);
        let assign1260_e2246: f64 = (p.p75 * locals.var_w_wwn);
        let assign1260_e2247: f64 = (assign1260_e2243 + assign1260_e2246);
        let assign1260_e2250: f64 = (p.p76 * locals.var_lw_wln_wwn);
        let assign1260_e2251: f64 = (assign1260_e2247 + assign1260_e2250);
        locals.var_dwcv = assign1260_e2251;
        locals.var_dwcv_rv = 0.0;

        let assign1270_e2255: f64 = (2.0 * locals.var_dlcv);
        let assign1270_e2256: f64 = (locals.var_lnew - assign1270_e2255);
        locals.var_lact = assign1270_e2256;
        locals.var_lact_rv = 0.0;

        let assign1300_e2266: f64 = (2.0 * locals.var_dwcv);
        let assign1300_e2267: f64 = (locals.var_wnew - assign1300_e2266);
        locals.var_wact = assign1300_e2267;
        locals.var_wact_rv = 0.0;

        let assign1330_e2278: f64 = (locals.var_lnew).powf(p.p67);
        let assign1330_e2279: f64 = (p.p74 / assign1330_e2278);
        let assign1330_e2280: f64 = (p.p138 + assign1330_e2279);
        let assign1330_e2284: f64 = (locals.var_wnew).powf(p.p68);
        let assign1330_e2285: f64 = (p.p75 / assign1330_e2284);
        let assign1330_e2286: f64 = (assign1330_e2280 + assign1330_e2285);
        let assign1330_e2290: f64 = (locals.var_lnew).powf(p.p67);
        let assign1330_e2291: f64 = (p.p76 / assign1330_e2290);
        let assign1330_e2294: f64 = (locals.var_wnew).powf(p.p68);
        let assign1330_e2295: f64 = (assign1330_e2291 / assign1330_e2294);
        let assign1330_e2296: f64 = (assign1330_e2286 + assign1330_e2295);
        locals.var_dwj = assign1330_e2296;
        locals.var_dwj_rv = 0.0;

        let assign1340_e2300: f64 = (2.0 * locals.var_dwj);
        let assign1340_e2301: f64 = (locals.var_wnew - assign1340_e2300);
        locals.var_weffcj = assign1340_e2301;
        locals.var_weffcj_rv = 0.0;

        let assign1360_e2307: f64 = (1e-6 / locals.var_leff);
        locals.var_inv_l = assign1360_e2307;
        locals.var_inv_l_rv = 0.0;

        let assign1370_e2310: f64 = (1e-6 / locals.var_weff);
        locals.var_inv_w = assign1370_e2310;
        locals.var_inv_w_rv = 0.0;

        let assign1380_e2313: f64 = (1e-6 / locals.var_lact);
        locals.var_inv_lact = assign1380_e2313;
        locals.var_inv_lact_rv = 0.0;

        let assign1390_e2316: f64 = (1e-6 / locals.var_wact);
        locals.var_inv_wact = assign1390_e2316;
        locals.var_inv_wact_rv = 0.0;

        let assign1400_e2319: f64 = (1e-6 / p.p51);
        locals.var_inv_llong = assign1400_e2319;
        locals.var_inv_llong_rv = 0.0;

        let assign1410_e2322: f64 = (1e-6 / p.p55);
        locals.var_inv_wwide = assign1410_e2322;
        locals.var_inv_wwide_rv = 0.0;

        let assign1420_e2325: f64 = (locals.var_inv_l * locals.var_inv_w);
        locals.var_inv_wl = assign1420_e2325;
        locals.var_inv_wl_rv = 0.0;

        locals.var_l_lln1 = locals.var_l_lln;
        locals.var_l_lln1_rv = 0.0;

        locals.var_l_wln1 = locals.var_l_wln;
        locals.var_l_wln1_rv = 0.0;

        let assign1450_e2330: f64 = if p.p818 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1450_e2330;
        locals.var_guard14_rv = 0.0;

        let assign1460_e2333: f64 = (-locals.var_lnew);
        let assign1460_e2334: f64 = if p.p818 <= assign1460_e2333 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1460_e2334;
        locals.var_guard15_rv = 0.0;

        let (assign1470_e2346,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign1470_e2341: f64 = (locals.var_lnew + p.p818);
        let assign1470_e2343: f64 = (-p.p61);
        let assign1470_e2344: f64 = (assign1470_e2341).powf(assign1470_e2343);
        (assign1470_e2344,)
    } else {
        (locals.var_l_lln1,)
    }
};
        locals.var_l_lln1 = assign1470_e2346;
        locals.var_l_lln1_rv = 0.0;

        let (assign1480_e2358,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign1480_e2353: f64 = (locals.var_lnew + p.p818);
        let assign1480_e2355: f64 = (-p.p67);
        let assign1480_e2356: f64 = (assign1480_e2353).powf(assign1480_e2355);
        (assign1480_e2356,)
    } else {
        (locals.var_l_wln1,)
    }
};
        locals.var_l_wln1 = assign1480_e2358;
        locals.var_l_wln1_rv = 0.0;

        locals.var_w_lwn1 = locals.var_w_lwn;
        locals.var_w_lwn1_rv = 0.0;

        locals.var_w_wwn1 = locals.var_w_wwn;
        locals.var_w_wwn1_rv = 0.0;

        let assign1510_e2363: f64 = if p.p819 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1510_e2363;
        locals.var_guard16_rv = 0.0;

        let assign1520_e2366: f64 = (-locals.var_wnew);
        let assign1520_e2367: f64 = if p.p819 <= assign1520_e2366 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1520_e2367;
        locals.var_guard17_rv = 0.0;

        let (assign1530_e2379,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1530_e2374: f64 = (locals.var_wnew + p.p819);
        let assign1530_e2376: f64 = (-p.p62);
        let assign1530_e2377: f64 = (assign1530_e2374).powf(assign1530_e2376);
        (assign1530_e2377,)
    } else {
        (locals.var_w_lwn1,)
    }
};
        locals.var_w_lwn1 = assign1530_e2379;
        locals.var_w_lwn1_rv = 0.0;

        let (assign1540_e2391,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1540_e2386: f64 = (locals.var_wnew + p.p819);
        let assign1540_e2388: f64 = (-p.p68);
        let assign1540_e2389: f64 = (assign1540_e2386).powf(assign1540_e2388);
        (assign1540_e2389,)
    } else {
        (locals.var_w_wwn1,)
    }
};
        locals.var_w_wwn1 = assign1540_e2391;
        locals.var_w_wwn1_rv = 0.0;

        let assign1550_e2394: f64 = (locals.var_l_lln1 * locals.var_w_lwn1);
        locals.var_lw_lln_lwn1 = assign1550_e2394;
        locals.var_lw_lln_lwn1_rv = 0.0;

        let assign1560_e2398: f64 = (p.p58 * locals.var_l_lln1);
        let assign1560_e2399: f64 = (p.p57 + assign1560_e2398);
        let assign1560_e2402: f64 = (p.p59 * locals.var_w_lwn1);
        let assign1560_e2403: f64 = (assign1560_e2399 + assign1560_e2402);
        let assign1560_e2406: f64 = (p.p60 * locals.var_lw_lln_lwn1);
        let assign1560_e2407: f64 = (assign1560_e2403 + assign1560_e2406);
        locals.var_dlb = assign1560_e2407;
        locals.var_dlb_rv = 0.0;

        let assign1570_e2410: f64 = (locals.var_l_wln1 * locals.var_w_wwn1);
        locals.var_lw_wln_wwn1 = assign1570_e2410;
        locals.var_lw_wln_wwn1_rv = 0.0;

        let assign1580_e2414: f64 = (p.p64 * locals.var_l_wln1);
        let assign1580_e2415: f64 = (p.p63 + assign1580_e2414);
        let assign1580_e2418: f64 = (p.p65 * locals.var_w_wwn1);
        let assign1580_e2419: f64 = (assign1580_e2415 + assign1580_e2418);
        let assign1580_e2422: f64 = (p.p66 * locals.var_lw_wln_wwn1);
        let assign1580_e2423: f64 = (assign1580_e2419 + assign1580_e2422);
        locals.var_dwb = assign1580_e2423;
        locals.var_dwb_rv = 0.0;

        let assign1590_e2427: f64 = (2.0 * locals.var_dlb);
        let assign1590_e2428: f64 = (locals.var_lnew - assign1590_e2427);
        let assign1590_e2430: f64 = (assign1590_e2428 + p.p818);
        locals.var_leff1 = assign1590_e2430;
        locals.var_leff1_rv = 0.0;

        let assign1610_e2437: f64 = (2.0 * locals.var_dwb);
        let assign1610_e2438: f64 = (locals.var_wnew - assign1610_e2437);
        let assign1610_e2440: f64 = (assign1610_e2438 + p.p819);
        locals.var_weff1 = assign1610_e2440;
        locals.var_weff1_rv = 0.0;

        let assign1630_e2446: f64 = if p.p817 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1630_e2446;
        locals.var_guard20_rv = 0.0;

        let (assign1640_e2452,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1640_e2450: f64 = (1e-6 / locals.var_leff1);
        (assign1640_e2450,)
    } else {
        (locals.var_bin_l,)
    }
};
        locals.var_bin_l = assign1640_e2452;
        locals.var_bin_l_rv = 0.0;

        let (assign1650_e2458,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1650_e2456: f64 = (1e-6 / locals.var_weff1);
        (assign1650_e2456,)
    } else {
        (locals.var_bin_w,)
    }
};
        locals.var_bin_w = assign1650_e2458;
        locals.var_bin_w_rv = 0.0;

        let (assign1660_e2465,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1660_e2463: f64 = (1.0 / locals.var_leff1);
        (assign1660_e2463,)
    } else {
        (locals.var_bin_l,)
    }
};
        locals.var_bin_l = assign1660_e2465;
        locals.var_bin_l_rv = 0.0;

        let (assign1670_e2472,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1670_e2470: f64 = (1.0 / locals.var_weff1);
        (assign1670_e2470,)
    } else {
        (locals.var_bin_w,)
    }
};
        locals.var_bin_w = assign1670_e2472;
        locals.var_bin_w_rv = 0.0;

        let assign1680_e2475: f64 = (locals.var_bin_l * locals.var_bin_w);
        locals.var_bin_wl = assign1680_e2475;
        locals.var_bin_wl_rv = 0.0;

        let assign1690_e2479: f64 = (locals.var_bin_l * p.p117);
        let assign1690_e2480: f64 = (p.p116 + assign1690_e2479);
        let assign1690_e2483: f64 = (locals.var_bin_w * p.p118);
        let assign1690_e2484: f64 = (assign1690_e2480 + assign1690_e2483);
        let assign1690_e2487: f64 = (locals.var_bin_wl * p.p119);
        let assign1690_e2488: f64 = (assign1690_e2484 + assign1690_e2487);
        locals.var_vfb_i = assign1690_e2488;
        locals.var_vfb_i_dn0 = 0.0;
        locals.var_vfb_i_dn2 = 0.0;
        locals.var_vfb_i_dn3 = 0.0;
        locals.var_vfb_i_dn4 = 0.0;
        locals.var_vfb_i_dn5 = 0.0;
        locals.var_vfb_i_dn6 = 0.0;
        locals.var_vfb_i_dn7 = 0.0;
        locals.var_vfb_i_dn8 = 0.0;
        locals.var_vfb_i_dn9 = 0.0;
        locals.var_vfb_i_dn10 = 0.0;
        locals.var_vfb_i_dn11 = 0.0;
        locals.var_vfb_i_dn12 = 0.0;
        locals.var_vfb_i_dn13 = 0.0;
        locals.var_vfb_i_dn14 = 0.0;
        locals.var_vfb_i_rv = 0.0;

        let assign1700_e2492: f64 = (locals.var_bin_l * p.p127);
        let assign1700_e2493: f64 = (p.p126 + assign1700_e2492);
        let assign1700_e2496: f64 = (locals.var_bin_w * p.p128);
        let assign1700_e2497: f64 = (assign1700_e2493 + assign1700_e2496);
        let assign1700_e2500: f64 = (locals.var_bin_wl * p.p129);
        let assign1700_e2501: f64 = (assign1700_e2497 + assign1700_e2500);
        locals.var_vfbcv_i = assign1700_e2501;
        locals.var_vfbcv_i_dn0 = 0.0;
        locals.var_vfbcv_i_dn2 = 0.0;
        locals.var_vfbcv_i_dn3 = 0.0;
        locals.var_vfbcv_i_dn4 = 0.0;
        locals.var_vfbcv_i_dn5 = 0.0;
        locals.var_vfbcv_i_dn6 = 0.0;
        locals.var_vfbcv_i_dn7 = 0.0;
        locals.var_vfbcv_i_dn8 = 0.0;
        locals.var_vfbcv_i_dn9 = 0.0;
        locals.var_vfbcv_i_dn10 = 0.0;
        locals.var_vfbcv_i_dn11 = 0.0;
        locals.var_vfbcv_i_dn12 = 0.0;
        locals.var_vfbcv_i_dn13 = 0.0;
        locals.var_vfbcv_i_dn14 = 0.0;
        locals.var_vfbcv_i_rv = 0.0;

        let assign1710_e2505: f64 = (locals.var_bin_l * p.p140);
        let assign1710_e2506: f64 = (p.p139 + assign1710_e2505);
        let assign1710_e2509: f64 = (locals.var_bin_w * p.p141);
        let assign1710_e2510: f64 = (assign1710_e2506 + assign1710_e2509);
        let assign1710_e2513: f64 = (locals.var_bin_wl * p.p142);
        let assign1710_e2514: f64 = (assign1710_e2510 + assign1710_e2513);
        locals.var_nsd_i = assign1710_e2514;
        locals.var_nsd_i_rv = 0.0;

        let assign1720_e2518: f64 = (locals.var_bin_l * p.p89);
        let assign1720_e2519: f64 = (p.p80 + assign1720_e2518);
        let assign1720_e2522: f64 = (locals.var_bin_w * p.p90);
        let assign1720_e2523: f64 = (assign1720_e2519 + assign1720_e2522);
        let assign1720_e2526: f64 = (locals.var_bin_wl * p.p91);
        let assign1720_e2527: f64 = (assign1720_e2523 + assign1720_e2526);
        locals.var_ndep_i = assign1720_e2527;
        locals.var_ndep_i_dn0 = 0.0;
        locals.var_ndep_i_dn2 = 0.0;
        locals.var_ndep_i_dn3 = 0.0;
        locals.var_ndep_i_dn4 = 0.0;
        locals.var_ndep_i_dn5 = 0.0;
        locals.var_ndep_i_dn6 = 0.0;
        locals.var_ndep_i_dn7 = 0.0;
        locals.var_ndep_i_dn8 = 0.0;
        locals.var_ndep_i_dn9 = 0.0;
        locals.var_ndep_i_dn10 = 0.0;
        locals.var_ndep_i_dn11 = 0.0;
        locals.var_ndep_i_dn12 = 0.0;
        locals.var_ndep_i_dn13 = 0.0;
        locals.var_ndep_i_dn14 = 0.0;
        locals.var_ndep_i_rv = 0.0;

        let assign1730_e2531: f64 = (locals.var_bin_l * p.p101);
        let assign1730_e2532: f64 = (p.p92 + assign1730_e2531);
        let assign1730_e2535: f64 = (locals.var_bin_w * p.p102);
        let assign1730_e2536: f64 = (assign1730_e2532 + assign1730_e2535);
        let assign1730_e2539: f64 = (locals.var_bin_wl * p.p103);
        let assign1730_e2540: f64 = (assign1730_e2536 + assign1730_e2539);
        locals.var_ndepcv_i = assign1730_e2540;
        locals.var_ndepcv_i_dn0 = 0.0;
        locals.var_ndepcv_i_dn2 = 0.0;
        locals.var_ndepcv_i_dn3 = 0.0;
        locals.var_ndepcv_i_dn4 = 0.0;
        locals.var_ndepcv_i_dn5 = 0.0;
        locals.var_ndepcv_i_dn6 = 0.0;
        locals.var_ndepcv_i_dn7 = 0.0;
        locals.var_ndepcv_i_dn8 = 0.0;
        locals.var_ndepcv_i_dn9 = 0.0;
        locals.var_ndepcv_i_dn10 = 0.0;
        locals.var_ndepcv_i_dn11 = 0.0;
        locals.var_ndepcv_i_dn12 = 0.0;
        locals.var_ndepcv_i_dn13 = 0.0;
        locals.var_ndepcv_i_dn14 = 0.0;
        locals.var_ndepcv_i_rv = 0.0;

        let assign1740_e2544: f64 = (locals.var_bin_l * p.p105);
        let assign1740_e2545: f64 = (p.p104 + assign1740_e2544);
        let assign1740_e2548: f64 = (locals.var_bin_w * p.p106);
        let assign1740_e2549: f64 = (assign1740_e2545 + assign1740_e2548);
        let assign1740_e2552: f64 = (locals.var_bin_wl * p.p107);
        let assign1740_e2553: f64 = (assign1740_e2549 + assign1740_e2552);
        locals.var_ngate_i = assign1740_e2553;
        locals.var_ngate_i_rv = 0.0;

        let assign1750_e2557: f64 = (locals.var_bin_l * p.p210);
        let assign1750_e2558: f64 = (p.p209 + assign1750_e2557);
        let assign1750_e2561: f64 = (locals.var_bin_w * p.p211);
        let assign1750_e2562: f64 = (assign1750_e2558 + assign1750_e2561);
        let assign1750_e2565: f64 = (locals.var_bin_wl * p.p212);
        let assign1750_e2566: f64 = (assign1750_e2562 + assign1750_e2565);
        locals.var_cit_i = assign1750_e2566;
        locals.var_cit_i_rv = 0.0;

        let assign1760_e2570: f64 = (locals.var_bin_l * p.p220);
        let assign1760_e2571: f64 = (p.p213 + assign1760_e2570);
        let assign1760_e2574: f64 = (locals.var_bin_w * p.p221);
        let assign1760_e2575: f64 = (assign1760_e2571 + assign1760_e2574);
        let assign1760_e2578: f64 = (locals.var_bin_wl * p.p222);
        let assign1760_e2579: f64 = (assign1760_e2575 + assign1760_e2578);
        locals.var_nfactor_i = assign1760_e2579;
        locals.var_nfactor_i_dn0 = 0.0;
        locals.var_nfactor_i_dn2 = 0.0;
        locals.var_nfactor_i_dn3 = 0.0;
        locals.var_nfactor_i_dn4 = 0.0;
        locals.var_nfactor_i_dn5 = 0.0;
        locals.var_nfactor_i_dn6 = 0.0;
        locals.var_nfactor_i_dn7 = 0.0;
        locals.var_nfactor_i_dn8 = 0.0;
        locals.var_nfactor_i_dn9 = 0.0;
        locals.var_nfactor_i_dn10 = 0.0;
        locals.var_nfactor_i_dn11 = 0.0;
        locals.var_nfactor_i_dn12 = 0.0;
        locals.var_nfactor_i_dn13 = 0.0;
        locals.var_nfactor_i_dn14 = 0.0;
        locals.var_nfactor_i_rv = 0.0;

        let assign1770_e2583: f64 = (locals.var_bin_l * p.p226);
        let assign1770_e2584: f64 = (p.p223 + assign1770_e2583);
        let assign1770_e2587: f64 = (locals.var_bin_w * p.p227);
        let assign1770_e2588: f64 = (assign1770_e2584 + assign1770_e2587);
        let assign1770_e2591: f64 = (locals.var_bin_wl * p.p228);
        let assign1770_e2592: f64 = (assign1770_e2588 + assign1770_e2591);
        locals.var_cdscd_i = assign1770_e2592;
        locals.var_cdscd_i_dn0 = 0.0;
        locals.var_cdscd_i_dn2 = 0.0;
        locals.var_cdscd_i_dn3 = 0.0;
        locals.var_cdscd_i_dn4 = 0.0;
        locals.var_cdscd_i_dn5 = 0.0;
        locals.var_cdscd_i_dn6 = 0.0;
        locals.var_cdscd_i_dn7 = 0.0;
        locals.var_cdscd_i_dn8 = 0.0;
        locals.var_cdscd_i_dn9 = 0.0;
        locals.var_cdscd_i_dn10 = 0.0;
        locals.var_cdscd_i_dn11 = 0.0;
        locals.var_cdscd_i_dn12 = 0.0;
        locals.var_cdscd_i_dn13 = 0.0;
        locals.var_cdscd_i_dn14 = 0.0;
        locals.var_cdscd_i_rv = 0.0;

        let assign1780_e2596: f64 = (locals.var_bin_l * p.p236);
        let assign1780_e2597: f64 = (p.p233 + assign1780_e2596);
        let assign1780_e2600: f64 = (locals.var_bin_w * p.p237);
        let assign1780_e2601: f64 = (assign1780_e2597 + assign1780_e2600);
        let assign1780_e2604: f64 = (locals.var_bin_wl * p.p238);
        let assign1780_e2605: f64 = (assign1780_e2601 + assign1780_e2604);
        locals.var_cdscb_i = assign1780_e2605;
        locals.var_cdscb_i_rv = 0.0;

        let assign1790_e2609: f64 = (locals.var_bin_l * p.p144);
        let assign1790_e2610: f64 = (p.p143 + assign1790_e2609);
        let assign1790_e2613: f64 = (locals.var_bin_w * p.p145);
        let assign1790_e2614: f64 = (assign1790_e2610 + assign1790_e2613);
        let assign1790_e2617: f64 = (locals.var_bin_wl * p.p146);
        let assign1790_e2618: f64 = (assign1790_e2614 + assign1790_e2617);
        locals.var_dvtp0_i = assign1790_e2618;
        locals.var_dvtp0_i_rv = 0.0;

        let assign1800_e2622: f64 = (locals.var_bin_l * p.p148);
        let assign1800_e2623: f64 = (p.p147 + assign1800_e2622);
        let assign1800_e2626: f64 = (locals.var_bin_w * p.p149);
        let assign1800_e2627: f64 = (assign1800_e2623 + assign1800_e2626);
        let assign1800_e2630: f64 = (locals.var_bin_wl * p.p150);
        let assign1800_e2631: f64 = (assign1800_e2627 + assign1800_e2630);
        locals.var_dvtp1_i = assign1800_e2631;
        locals.var_dvtp1_i_rv = 0.0;

        let assign1810_e2635: f64 = (locals.var_bin_l * p.p152);
        let assign1810_e2636: f64 = (p.p151 + assign1810_e2635);
        let assign1810_e2639: f64 = (locals.var_bin_w * p.p153);
        let assign1810_e2640: f64 = (assign1810_e2636 + assign1810_e2639);
        let assign1810_e2643: f64 = (locals.var_bin_wl * p.p154);
        let assign1810_e2644: f64 = (assign1810_e2640 + assign1810_e2643);
        locals.var_dvtp2_i = assign1810_e2644;
        locals.var_dvtp2_i_rv = 0.0;

        let assign1820_e2648: f64 = (locals.var_bin_l * p.p156);
        let assign1820_e2649: f64 = (p.p155 + assign1820_e2648);
        let assign1820_e2652: f64 = (locals.var_bin_w * p.p157);
        let assign1820_e2653: f64 = (assign1820_e2649 + assign1820_e2652);
        let assign1820_e2656: f64 = (locals.var_bin_wl * p.p158);
        let assign1820_e2657: f64 = (assign1820_e2653 + assign1820_e2656);
        locals.var_dvtp3_i = assign1820_e2657;
        locals.var_dvtp3_i_rv = 0.0;

        let assign1830_e2661: f64 = (locals.var_bin_l * p.p160);
        let assign1830_e2662: f64 = (p.p159 + assign1830_e2661);
        let assign1830_e2665: f64 = (locals.var_bin_w * p.p161);
        let assign1830_e2666: f64 = (assign1830_e2662 + assign1830_e2665);
        let assign1830_e2669: f64 = (locals.var_bin_wl * p.p162);
        let assign1830_e2670: f64 = (assign1830_e2666 + assign1830_e2669);
        locals.var_dvtp4_i = assign1830_e2670;
        locals.var_dvtp4_i_rv = 0.0;

        let assign1840_e2674: f64 = (locals.var_bin_l * p.p164);
        let assign1840_e2675: f64 = (p.p163 + assign1840_e2674);
        let assign1840_e2678: f64 = (locals.var_bin_w * p.p165);
        let assign1840_e2679: f64 = (assign1840_e2675 + assign1840_e2678);
        let assign1840_e2682: f64 = (locals.var_bin_wl * p.p166);
        let assign1840_e2683: f64 = (assign1840_e2679 + assign1840_e2682);
        locals.var_dvtp5_i = assign1840_e2683;
        locals.var_dvtp5_i_rv = 0.0;

    }
}

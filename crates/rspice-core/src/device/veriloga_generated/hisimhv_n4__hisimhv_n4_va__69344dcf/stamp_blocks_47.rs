#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_375(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign100850_e152989, assign100850_e152989_d_n0, assign100850_e152989_d_n2, assign100850_e152989_d_n4, assign100850_e152989_d_n5, assign100850_e152989_d_n6, assign100850_e152989_d_n7, assign100850_e152989_d_n8, assign100850_e152989_d_n9, assign100850_e152989_d_n10, assign100850_e152989_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100850_e152987: f64 = (locals.var_t6).sqrt();
        (assign100850_e152987, (locals.var_t6_dn0 / (2.0 * assign100850_e152987)), (locals.var_t6_dn2 / (2.0 * assign100850_e152987)), (locals.var_t6_dn4 / (2.0 * assign100850_e152987)), (locals.var_t6_dn5 / (2.0 * assign100850_e152987)), (locals.var_t6_dn6 / (2.0 * assign100850_e152987)), (locals.var_t6_dn7 / (2.0 * assign100850_e152987)), (locals.var_t6_dn8 / (2.0 * assign100850_e152987)), (locals.var_t6_dn9 / (2.0 * assign100850_e152987)), (locals.var_t6_dn10 / (2.0 * assign100850_e152987)), (locals.var_t6_dn13 / (2.0 * assign100850_e152987)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign100850_e152989;
        locals.var_t6_dn0 = assign100850_e152989_d_n0;
        locals.var_t6_dn2 = assign100850_e152989_d_n2;
        locals.var_t6_dn4 = assign100850_e152989_d_n4;
        locals.var_t6_dn5 = assign100850_e152989_d_n5;
        locals.var_t6_dn6 = assign100850_e152989_d_n6;
        locals.var_t6_dn7 = assign100850_e152989_d_n7;
        locals.var_t6_dn8 = assign100850_e152989_d_n8;
        locals.var_t6_dn9 = assign100850_e152989_d_n9;
        locals.var_t6_dn10 = assign100850_e152989_d_n10;
        locals.var_t6_dn13 = assign100850_e152989_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign100860_e153001, assign100860_e153001_d_n0, assign100860_e153001_d_n2, assign100860_e153001_d_n4, assign100860_e153001_d_n5, assign100860_e153001_d_n6, assign100860_e153001_d_n7, assign100860_e153001_d_n8, assign100860_e153001_d_n9, assign100860_e153001_d_n10, assign100860_e153001_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100860_e152997: f64 = (1.0 - locals.var_t6);
        let assign100860_e152998: f64 = (locals.var_t3 * assign100860_e152997);
        let assign100860_e152999: f64 = (locals.var_t1 + assign100860_e152998);
        (assign100860_e152999, (locals.var_t1_dn0 + ((locals.var_t3_dn0 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn0)))), (locals.var_t1_dn2 + ((locals.var_t3_dn2 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn2)))), (locals.var_t1_dn4 + ((locals.var_t3_dn4 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn4)))), (locals.var_t1_dn5 + ((locals.var_t3_dn5 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn5)))), (locals.var_t1_dn6 + ((locals.var_t3_dn6 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn6)))), (locals.var_t1_dn7 + ((locals.var_t3_dn7 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn7)))), (locals.var_t1_dn8 + ((locals.var_t3_dn8 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn8)))), (locals.var_t1_dn9 + ((locals.var_t3_dn9 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn9)))), (locals.var_t1_dn10 + ((locals.var_t3_dn10 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn10)))), (locals.var_t1_dn13 + ((locals.var_t3_dn13 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn13)))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn4, locals.var_psislsat_dn5, locals.var_psislsat_dn6, locals.var_psislsat_dn7, locals.var_psislsat_dn8, locals.var_psislsat_dn9, locals.var_psislsat_dn10, locals.var_psislsat_dn13,)
    }
};
        locals.var_psislsat = assign100860_e153001;
        locals.var_psislsat_dn0 = assign100860_e153001_d_n0;
        locals.var_psislsat_dn2 = assign100860_e153001_d_n2;
        locals.var_psislsat_dn4 = assign100860_e153001_d_n4;
        locals.var_psislsat_dn5 = assign100860_e153001_d_n5;
        locals.var_psislsat_dn6 = assign100860_e153001_d_n6;
        locals.var_psislsat_dn7 = assign100860_e153001_d_n7;
        locals.var_psislsat_dn8 = assign100860_e153001_d_n8;
        locals.var_psislsat_dn9 = assign100860_e153001_d_n9;
        locals.var_psislsat_dn10 = assign100860_e153001_d_n10;
        locals.var_psislsat_dn13 = assign100860_e153001_d_n13;
        locals.var_psislsat_rv = 0.0;

        let (assign100870_e153011, assign100870_e153011_d_n0, assign100870_e153011_d_n2, assign100870_e153011_d_n4, assign100870_e153011_d_n5, assign100870_e153011_d_n6, assign100870_e153011_d_n7, assign100870_e153011_d_n8, assign100870_e153011_d_n9, assign100870_e153011_d_n10, assign100870_e153011_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100870_e153008: f64 = (locals.var_xgate_1 + locals.var_lgate);
        let assign100870_e153009: f64 = (locals.var_lgate / assign100870_e153008);
        (assign100870_e153009, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign100870_e153011;
        locals.var_t2_dn0 = assign100870_e153011_d_n0;
        locals.var_t2_dn2 = assign100870_e153011_d_n2;
        locals.var_t2_dn4 = assign100870_e153011_d_n4;
        locals.var_t2_dn5 = assign100870_e153011_d_n5;
        locals.var_t2_dn6 = assign100870_e153011_d_n6;
        locals.var_t2_dn7 = assign100870_e153011_d_n7;
        locals.var_t2_dn8 = assign100870_e153011_d_n8;
        locals.var_t2_dn9 = assign100870_e153011_d_n9;
        locals.var_t2_dn10 = assign100870_e153011_d_n10;
        locals.var_t2_dn13 = assign100870_e153011_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign100880_e153025, assign100880_e153025_d_n0, assign100880_e153025_d_n2, assign100880_e153025_d_n4, assign100880_e153025_d_n5, assign100880_e153025_d_n6, assign100880_e153025_d_n7, assign100880_e153025_d_n8, assign100880_e153025_d_n9, assign100880_e153025_d_n10, assign100880_e153025_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100880_e153017: f64 = (locals.var_uc_svdssnp * locals.var_vdsz__blk439);
        let assign100880_e153019: f64 = (assign100880_e153017 + locals.var_ps0z);
        let assign100880_e153022: f64 = (locals.var_t2 * locals.var_psislsat);
        let assign100880_e153023: f64 = (assign100880_e153019 - assign100880_e153022);
        (assign100880_e153023, (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn0) + locals.var_ps0z_dn0) - ((locals.var_t2_dn0 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn0))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn2) + locals.var_ps0z_dn2) - ((locals.var_t2_dn2 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn2))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn4) + locals.var_ps0z_dn4) - ((locals.var_t2_dn4 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn4))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn5) + locals.var_ps0z_dn5) - ((locals.var_t2_dn5 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn5))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn6) + locals.var_ps0z_dn6) - ((locals.var_t2_dn6 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn6))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn7) + locals.var_ps0z_dn7) - ((locals.var_t2_dn7 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn7))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn8) + locals.var_ps0z_dn8) - ((locals.var_t2_dn8 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn8))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn9) + locals.var_ps0z_dn9) - ((locals.var_t2_dn9 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn9))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn10) + locals.var_ps0z_dn10) - ((locals.var_t2_dn10 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn10))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn13) + locals.var_ps0z_dn13) - ((locals.var_t2_dn13 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn13))),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign100880_e153025;
        locals.var_psisubsat_dn0 = assign100880_e153025_d_n0;
        locals.var_psisubsat_dn2 = assign100880_e153025_d_n2;
        locals.var_psisubsat_dn4 = assign100880_e153025_d_n4;
        locals.var_psisubsat_dn5 = assign100880_e153025_d_n5;
        locals.var_psisubsat_dn6 = assign100880_e153025_d_n6;
        locals.var_psisubsat_dn7 = assign100880_e153025_d_n7;
        locals.var_psisubsat_dn8 = assign100880_e153025_d_n8;
        locals.var_psisubsat_dn9 = assign100880_e153025_d_n9;
        locals.var_psisubsat_dn10 = assign100880_e153025_d_n10;
        locals.var_psisubsat_dn13 = assign100880_e153025_d_n13;
        locals.var_psisubsat_rv = 0.0;

        let (assign100890_e153040, assign100890_e153040_d_n0, assign100890_e153040_d_n2, assign100890_e153040_d_n4, assign100890_e153040_d_n5, assign100890_e153040_d_n6, assign100890_e153040_d_n7, assign100890_e153040_d_n8, assign100890_e153040_d_n9, assign100890_e153040_d_n10, assign100890_e153040_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100890_e153031: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign100890_e153034: f64 = (4.0 * 0.001);
        let assign100890_e153036: f64 = (assign100890_e153034 * 0.001);
        let assign100890_e153037: f64 = (assign100890_e153031 + assign100890_e153036);
        let assign100890_e153038: f64 = (assign100890_e153037).sqrt();
        (assign100890_e153038, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn4 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn4)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn5 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn5)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn7 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn7)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn8 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn8)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn9 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn9)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn13 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn13)) / (2.0 * assign100890_e153038)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign100890_e153040;
        locals.var_tmf2_dn0 = assign100890_e153040_d_n0;
        locals.var_tmf2_dn2 = assign100890_e153040_d_n2;
        locals.var_tmf2_dn4 = assign100890_e153040_d_n4;
        locals.var_tmf2_dn5 = assign100890_e153040_d_n5;
        locals.var_tmf2_dn6 = assign100890_e153040_d_n6;
        locals.var_tmf2_dn7 = assign100890_e153040_d_n7;
        locals.var_tmf2_dn8 = assign100890_e153040_d_n8;
        locals.var_tmf2_dn9 = assign100890_e153040_d_n9;
        locals.var_tmf2_dn10 = assign100890_e153040_d_n10;
        locals.var_tmf2_dn13 = assign100890_e153040_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign100900_e153052, assign100900_e153052_d_n0, assign100900_e153052_d_n2, assign100900_e153052_d_n4, assign100900_e153052_d_n5, assign100900_e153052_d_n6, assign100900_e153052_d_n7, assign100900_e153052_d_n8, assign100900_e153052_d_n9, assign100900_e153052_d_n10, assign100900_e153052_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100900_e153048: f64 = (locals.var_psisubsat / locals.var_tmf2);
        let assign100900_e153049: f64 = (1.0 + assign100900_e153048);
        let assign100900_e153050: f64 = (0.5 * assign100900_e153049);
        (assign100900_e153050, (0.5 * (((locals.var_psisubsat_dn0 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn2 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn4 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn5 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn6 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn7 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn8 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn9 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn10 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn13 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign100900_e153052;
        locals.var_t9_dn0 = assign100900_e153052_d_n0;
        locals.var_t9_dn2 = assign100900_e153052_d_n2;
        locals.var_t9_dn4 = assign100900_e153052_d_n4;
        locals.var_t9_dn5 = assign100900_e153052_d_n5;
        locals.var_t9_dn6 = assign100900_e153052_d_n6;
        locals.var_t9_dn7 = assign100900_e153052_d_n7;
        locals.var_t9_dn8 = assign100900_e153052_d_n8;
        locals.var_t9_dn9 = assign100900_e153052_d_n9;
        locals.var_t9_dn10 = assign100900_e153052_d_n10;
        locals.var_t9_dn13 = assign100900_e153052_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign100910_e153062, assign100910_e153062_d_n0, assign100910_e153062_d_n2, assign100910_e153062_d_n4, assign100910_e153062_d_n5, assign100910_e153062_d_n6, assign100910_e153062_d_n7, assign100910_e153062_d_n8, assign100910_e153062_d_n9, assign100910_e153062_d_n10, assign100910_e153062_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100910_e153059: f64 = (locals.var_psisubsat + locals.var_tmf2);
        let assign100910_e153060: f64 = (0.5 * assign100910_e153059);
        (assign100910_e153060, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_psisubsat_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_psisubsat_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_psisubsat_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_psisubsat_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_psisubsat_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_psisubsat_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign100910_e153062;
        locals.var_psisubsat_dn0 = assign100910_e153062_d_n0;
        locals.var_psisubsat_dn2 = assign100910_e153062_d_n2;
        locals.var_psisubsat_dn4 = assign100910_e153062_d_n4;
        locals.var_psisubsat_dn5 = assign100910_e153062_d_n5;
        locals.var_psisubsat_dn6 = assign100910_e153062_d_n6;
        locals.var_psisubsat_dn7 = assign100910_e153062_d_n7;
        locals.var_psisubsat_dn8 = assign100910_e153062_d_n8;
        locals.var_psisubsat_dn9 = assign100910_e153062_d_n9;
        locals.var_psisubsat_dn10 = assign100910_e153062_d_n10;
        locals.var_psisubsat_dn13 = assign100910_e153062_d_n13;
        locals.var_psisubsat_rv = 0.0;

        let assign100920_e153065: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2319 = assign100920_e153065;
        locals.var_guard2319_rv = 0.0;

        let (assign100930_e153073, assign100930_e153073_d_n0, assign100930_e153073_d_n2, assign100930_e153073_d_n4, assign100930_e153073_d_n5, assign100930_e153073_d_n6, assign100930_e153073_d_n7, assign100930_e153073_d_n8, assign100930_e153073_d_n9, assign100930_e153073_d_n10, assign100930_e153073_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2319 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign100930_e153073;
        locals.var_psisubsat_dn0 = assign100930_e153073_d_n0;
        locals.var_psisubsat_dn2 = assign100930_e153073_d_n2;
        locals.var_psisubsat_dn4 = assign100930_e153073_d_n4;
        locals.var_psisubsat_dn5 = assign100930_e153073_d_n5;
        locals.var_psisubsat_dn6 = assign100930_e153073_d_n6;
        locals.var_psisubsat_dn7 = assign100930_e153073_d_n7;
        locals.var_psisubsat_dn8 = assign100930_e153073_d_n8;
        locals.var_psisubsat_dn9 = assign100930_e153073_d_n9;
        locals.var_psisubsat_dn10 = assign100930_e153073_d_n10;
        locals.var_psisubsat_dn13 = assign100930_e153073_d_n13;
        locals.var_psisubsat_rv = 0.0;

        let (assign100940_e153081, assign100940_e153081_d_n0, assign100940_e153081_d_n2, assign100940_e153081_d_n4, assign100940_e153081_d_n5, assign100940_e153081_d_n6, assign100940_e153081_d_n7, assign100940_e153081_d_n8, assign100940_e153081_d_n9, assign100940_e153081_d_n10, assign100940_e153081_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2319 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign100940_e153081;
        locals.var_t9_dn0 = assign100940_e153081_d_n0;
        locals.var_t9_dn2 = assign100940_e153081_d_n2;
        locals.var_t9_dn4 = assign100940_e153081_d_n4;
        locals.var_t9_dn5 = assign100940_e153081_d_n5;
        locals.var_t9_dn6 = assign100940_e153081_d_n6;
        locals.var_t9_dn7 = assign100940_e153081_d_n7;
        locals.var_t9_dn8 = assign100940_e153081_d_n8;
        locals.var_t9_dn9 = assign100940_e153081_d_n9;
        locals.var_t9_dn10 = assign100940_e153081_d_n10;
        locals.var_t9_dn13 = assign100940_e153081_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign100950_e153089, assign100950_e153089_d_n0, assign100950_e153089_d_n2, assign100950_e153089_d_n4, assign100950_e153089_d_n5, assign100950_e153089_d_n6, assign100950_e153089_d_n7, assign100950_e153089_d_n8, assign100950_e153089_d_n9, assign100950_e153089_d_n10, assign100950_e153089_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100950_e153087: f64 = (locals.var_psisubsat + 1e-25);
        (assign100950_e153087, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign100950_e153089;
        locals.var_psisubsat_dn0 = assign100950_e153089_d_n0;
        locals.var_psisubsat_dn2 = assign100950_e153089_d_n2;
        locals.var_psisubsat_dn4 = assign100950_e153089_d_n4;
        locals.var_psisubsat_dn5 = assign100950_e153089_d_n5;
        locals.var_psisubsat_dn6 = assign100950_e153089_d_n6;
        locals.var_psisubsat_dn7 = assign100950_e153089_d_n7;
        locals.var_psisubsat_dn8 = assign100950_e153089_d_n8;
        locals.var_psisubsat_dn9 = assign100950_e153089_d_n9;
        locals.var_psisubsat_dn10 = assign100950_e153089_d_n10;
        locals.var_psisubsat_dn13 = assign100950_e153089_d_n13;
        locals.var_psisubsat_rv = 0.0;

        let (assign100960_e153101, assign100960_e153101_d_n0, assign100960_e153101_d_n2, assign100960_e153101_d_n4, assign100960_e153101_d_n5, assign100960_e153101_d_n6, assign100960_e153101_d_n7, assign100960_e153101_d_n8, assign100960_e153101_d_n9, assign100960_e153101_d_n10, assign100960_e153101_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100960_e153097: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign100960_e153098: f64 = (locals.var_uc_subtmp * assign100960_e153097);
        let assign100960_e153099: f64 = (1.0 + assign100960_e153098);
        (assign100960_e153099, (locals.var_uc_subtmp * locals.var_ttemp_dn0), (locals.var_uc_subtmp * locals.var_ttemp_dn2), (locals.var_uc_subtmp * locals.var_ttemp_dn4), (locals.var_uc_subtmp * locals.var_ttemp_dn5), (locals.var_uc_subtmp * locals.var_ttemp_dn6), (locals.var_uc_subtmp * locals.var_ttemp_dn7), (locals.var_uc_subtmp * locals.var_ttemp_dn8), (locals.var_uc_subtmp * locals.var_ttemp_dn9), (locals.var_uc_subtmp * locals.var_ttemp_dn10), (locals.var_uc_subtmp * locals.var_ttemp_dn13),)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
    }
};
        locals.var_xsubtmp = assign100960_e153101;
        locals.var_xsubtmp_dn0 = assign100960_e153101_d_n0;
        locals.var_xsubtmp_dn2 = assign100960_e153101_d_n2;
        locals.var_xsubtmp_dn4 = assign100960_e153101_d_n4;
        locals.var_xsubtmp_dn5 = assign100960_e153101_d_n5;
        locals.var_xsubtmp_dn6 = assign100960_e153101_d_n6;
        locals.var_xsubtmp_dn7 = assign100960_e153101_d_n7;
        locals.var_xsubtmp_dn8 = assign100960_e153101_d_n8;
        locals.var_xsubtmp_dn9 = assign100960_e153101_d_n9;
        locals.var_xsubtmp_dn10 = assign100960_e153101_d_n10;
        locals.var_xsubtmp_dn13 = assign100960_e153101_d_n13;
        locals.var_xsubtmp_rv = 0.0;

        let (assign100970_e153112, assign100970_e153112_d_n0, assign100970_e153112_d_n2, assign100970_e153112_d_n4, assign100970_e153112_d_n5, assign100970_e153112_d_n6, assign100970_e153112_d_n7, assign100970_e153112_d_n8, assign100970_e153112_d_n9, assign100970_e153112_d_n10, assign100970_e153112_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let (assign100970_e153110, assign100970_e153110_d_n0, assign100970_e153110_d_n2, assign100970_e153110_d_n4, assign100970_e153110_d_n5, assign100970_e153110_d_n6, assign100970_e153110_d_n7, assign100970_e153110_d_n8, assign100970_e153110_d_n9, assign100970_e153110_d_n10, assign100970_e153110_d_n13,) = {
            if (locals.var_xsubtmp <= 0.001) {
                (0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
            }
        };
        (assign100970_e153110, assign100970_e153110_d_n0, assign100970_e153110_d_n2, assign100970_e153110_d_n4, assign100970_e153110_d_n5, assign100970_e153110_d_n6, assign100970_e153110_d_n7, assign100970_e153110_d_n8, assign100970_e153110_d_n9, assign100970_e153110_d_n10, assign100970_e153110_d_n13,)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
    }
};
        locals.var_xsubtmp = assign100970_e153112;
        locals.var_xsubtmp_dn0 = assign100970_e153112_d_n0;
        locals.var_xsubtmp_dn2 = assign100970_e153112_d_n2;
        locals.var_xsubtmp_dn4 = assign100970_e153112_d_n4;
        locals.var_xsubtmp_dn5 = assign100970_e153112_d_n5;
        locals.var_xsubtmp_dn6 = assign100970_e153112_d_n6;
        locals.var_xsubtmp_dn7 = assign100970_e153112_d_n7;
        locals.var_xsubtmp_dn8 = assign100970_e153112_d_n8;
        locals.var_xsubtmp_dn9 = assign100970_e153112_d_n9;
        locals.var_xsubtmp_dn10 = assign100970_e153112_d_n10;
        locals.var_xsubtmp_dn13 = assign100970_e153112_d_n13;
        locals.var_xsubtmp_rv = 0.0;

        let (assign100980_e153120, assign100980_e153120_d_n0, assign100980_e153120_d_n2, assign100980_e153120_d_n4, assign100980_e153120_d_n5, assign100980_e153120_d_n6, assign100980_e153120_d_n7, assign100980_e153120_d_n8, assign100980_e153120_d_n9, assign100980_e153120_d_n10, assign100980_e153120_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100980_e153118: f64 = (locals.var_xsub1_1 / locals.var_xsubtmp);
        (assign100980_e153118, (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn0) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn2) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn4) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn5) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn6) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn7) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn8) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn9) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn10) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn13) / (locals.var_xsubtmp * locals.var_xsubtmp))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign100980_e153120;
        locals.var_t5_dn0 = assign100980_e153120_d_n0;
        locals.var_t5_dn2 = assign100980_e153120_d_n2;
        locals.var_t5_dn4 = assign100980_e153120_d_n4;
        locals.var_t5_dn5 = assign100980_e153120_d_n5;
        locals.var_t5_dn6 = assign100980_e153120_d_n6;
        locals.var_t5_dn7 = assign100980_e153120_d_n7;
        locals.var_t5_dn8 = assign100980_e153120_d_n8;
        locals.var_t5_dn9 = assign100980_e153120_d_n9;
        locals.var_t5_dn10 = assign100980_e153120_d_n10;
        locals.var_t5_dn13 = assign100980_e153120_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign100990_e153128, assign100990_e153128_d_n0, assign100990_e153128_d_n2, assign100990_e153128_d_n4, assign100990_e153128_d_n5, assign100990_e153128_d_n6, assign100990_e153128_d_n7, assign100990_e153128_d_n8, assign100990_e153128_d_n9, assign100990_e153128_d_n10, assign100990_e153128_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100990_e153126: f64 = (locals.var_xsub2_1 * locals.var_xsubtmp);
        (assign100990_e153126, (locals.var_xsub2_1 * locals.var_xsubtmp_dn0), (locals.var_xsub2_1 * locals.var_xsubtmp_dn2), (locals.var_xsub2_1 * locals.var_xsubtmp_dn4), (locals.var_xsub2_1 * locals.var_xsubtmp_dn5), (locals.var_xsub2_1 * locals.var_xsubtmp_dn6), (locals.var_xsub2_1 * locals.var_xsubtmp_dn7), (locals.var_xsub2_1 * locals.var_xsubtmp_dn8), (locals.var_xsub2_1 * locals.var_xsubtmp_dn9), (locals.var_xsub2_1 * locals.var_xsubtmp_dn10), (locals.var_xsub2_1 * locals.var_xsubtmp_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign100990_e153128;
        locals.var_t6_dn0 = assign100990_e153128_d_n0;
        locals.var_t6_dn2 = assign100990_e153128_d_n2;
        locals.var_t6_dn4 = assign100990_e153128_d_n4;
        locals.var_t6_dn5 = assign100990_e153128_d_n5;
        locals.var_t6_dn6 = assign100990_e153128_d_n6;
        locals.var_t6_dn7 = assign100990_e153128_d_n7;
        locals.var_t6_dn8 = assign100990_e153128_d_n8;
        locals.var_t6_dn9 = assign100990_e153128_d_n9;
        locals.var_t6_dn10 = assign100990_e153128_d_n10;
        locals.var_t6_dn13 = assign100990_e153128_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign101000_e153138, assign101000_e153138_d_n0, assign101000_e153138_d_n2, assign101000_e153138_d_n4, assign101000_e153138_d_n5, assign101000_e153138_d_n6, assign101000_e153138_d_n7, assign101000_e153138_d_n8, assign101000_e153138_d_n9, assign101000_e153138_d_n10, assign101000_e153138_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign101000_e153133: f64 = (-locals.var_t6);
        let assign101000_e153135: f64 = (assign101000_e153133 / locals.var_psisubsat);
        let assign101000_e153136: f64 = (assign101000_e153135).exp();
        (assign101000_e153136, (assign101000_e153136 * ((((-locals.var_t6_dn0) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn0)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn2) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn2)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn4) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn4)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn5) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn5)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn6) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn6)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn7) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn7)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn8) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn8)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn9) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn9)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn10) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn10)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn13) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn13)) / (locals.var_psisubsat * locals.var_psisubsat))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign101000_e153138;
        locals.var_t2_dn0 = assign101000_e153138_d_n0;
        locals.var_t2_dn2 = assign101000_e153138_d_n2;
        locals.var_t2_dn4 = assign101000_e153138_d_n4;
        locals.var_t2_dn5 = assign101000_e153138_d_n5;
        locals.var_t2_dn6 = assign101000_e153138_d_n6;
        locals.var_t2_dn7 = assign101000_e153138_d_n7;
        locals.var_t2_dn8 = assign101000_e153138_d_n8;
        locals.var_t2_dn9 = assign101000_e153138_d_n9;
        locals.var_t2_dn10 = assign101000_e153138_d_n10;
        locals.var_t2_dn13 = assign101000_e153138_d_n13;
        locals.var_t2_rv = 0.0;

        let assign101050_e153179: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2321 = assign101050_e153179;
        locals.var_guard2321_rv = 0.0;

        let (assign101060_e153185, assign101060_e153185_d_n0, assign101060_e153185_d_n2, assign101060_e153185_d_n4, assign101060_e153185_d_n5, assign101060_e153185_d_n6, assign101060_e153185_d_n7, assign101060_e153185_d_n8, assign101060_e153185_d_n9, assign101060_e153185_d_n10, assign101060_e153185_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 != 0.0)) {
        (p.p270, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn13,)
    }
};
        locals.var_t12 = assign101060_e153185;
        locals.var_t12_dn0 = assign101060_e153185_d_n0;
        locals.var_t12_dn2 = assign101060_e153185_d_n2;
        locals.var_t12_dn4 = assign101060_e153185_d_n4;
        locals.var_t12_dn5 = assign101060_e153185_d_n5;
        locals.var_t12_dn6 = assign101060_e153185_d_n6;
        locals.var_t12_dn7 = assign101060_e153185_d_n7;
        locals.var_t12_dn8 = assign101060_e153185_d_n8;
        locals.var_t12_dn9 = assign101060_e153185_d_n9;
        locals.var_t12_dn10 = assign101060_e153185_d_n10;
        locals.var_t12_dn13 = assign101060_e153185_d_n13;
        locals.var_t12_rv = 0.0;

        let (assign101070_e153191, assign101070_e153191_d_n0, assign101070_e153191_d_n2, assign101070_e153191_d_n4, assign101070_e153191_d_n5, assign101070_e153191_d_n6, assign101070_e153191_d_n7, assign101070_e153191_d_n8, assign101070_e153191_d_n9, assign101070_e153191_d_n10, assign101070_e153191_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 != 0.0)) {
        (p.p271, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign101070_e153191;
        locals.var_t10_dn0 = assign101070_e153191_d_n0;
        locals.var_t10_dn2 = assign101070_e153191_d_n2;
        locals.var_t10_dn4 = assign101070_e153191_d_n4;
        locals.var_t10_dn5 = assign101070_e153191_d_n5;
        locals.var_t10_dn6 = assign101070_e153191_d_n6;
        locals.var_t10_dn7 = assign101070_e153191_d_n7;
        locals.var_t10_dn8 = assign101070_e153191_d_n8;
        locals.var_t10_dn9 = assign101070_e153191_d_n9;
        locals.var_t10_dn10 = assign101070_e153191_d_n10;
        locals.var_t10_dn13 = assign101070_e153191_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign101080_e153197, assign101080_e153197_d_n0, assign101080_e153197_d_n2, assign101080_e153197_d_n4, assign101080_e153197_d_n5, assign101080_e153197_d_n6, assign101080_e153197_d_n7, assign101080_e153197_d_n8, assign101080_e153197_d_n9, assign101080_e153197_d_n10, assign101080_e153197_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 != 0.0)) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign101080_e153197;
        locals.var_t3_dn0 = assign101080_e153197_d_n0;
        locals.var_t3_dn2 = assign101080_e153197_d_n2;
        locals.var_t3_dn4 = assign101080_e153197_d_n4;
        locals.var_t3_dn5 = assign101080_e153197_d_n5;
        locals.var_t3_dn6 = assign101080_e153197_d_n6;
        locals.var_t3_dn7 = assign101080_e153197_d_n7;
        locals.var_t3_dn8 = assign101080_e153197_d_n8;
        locals.var_t3_dn9 = assign101080_e153197_d_n9;
        locals.var_t3_dn10 = assign101080_e153197_d_n10;
        locals.var_t3_dn13 = assign101080_e153197_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign101090_e153209, assign101090_e153209_d_n0, assign101090_e153209_d_n2, assign101090_e153209_d_n4, assign101090_e153209_d_n5, assign101090_e153209_d_n6, assign101090_e153209_d_n7, assign101090_e153209_d_n8, assign101090_e153209_d_n9, assign101090_e153209_d_n10, assign101090_e153209_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 != 0.0)) {
        let assign101090_e153203: f64 = (locals.var_t12 * locals.var_t10);
        let assign101090_e153205: f64 = (assign101090_e153203 * locals.var_t3);
        let assign101090_e153207: f64 = (assign101090_e153205 * locals.var_t3);
        (assign101090_e153207, ((((((locals.var_t12_dn0 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn0)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn0)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn0)), ((((((locals.var_t12_dn2 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn2)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn2)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn2)), ((((((locals.var_t12_dn4 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn4)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn4)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn4)), ((((((locals.var_t12_dn5 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn5)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn5)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn5)), ((((((locals.var_t12_dn6 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn6)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn6)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn6)), ((((((locals.var_t12_dn7 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn7)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn7)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn7)), ((((((locals.var_t12_dn8 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn8)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn8)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn8)), ((((((locals.var_t12_dn9 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn9)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn9)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn9)), ((((((locals.var_t12_dn10 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn10)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn10)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn10)), ((((((locals.var_t12_dn13 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn13)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn13)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign101090_e153209;
        locals.var_t1_dn0 = assign101090_e153209_d_n0;
        locals.var_t1_dn2 = assign101090_e153209_d_n2;
        locals.var_t1_dn4 = assign101090_e153209_d_n4;
        locals.var_t1_dn5 = assign101090_e153209_d_n5;
        locals.var_t1_dn6 = assign101090_e153209_d_n6;
        locals.var_t1_dn7 = assign101090_e153209_d_n7;
        locals.var_t1_dn8 = assign101090_e153209_d_n8;
        locals.var_t1_dn9 = assign101090_e153209_d_n9;
        locals.var_t1_dn10 = assign101090_e153209_d_n10;
        locals.var_t1_dn13 = assign101090_e153209_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign101100_e153227, assign101100_e153227_d_n0, assign101100_e153227_d_n2, assign101100_e153227_d_n4, assign101100_e153227_d_n5, assign101100_e153227_d_n6, assign101100_e153227_d_n7, assign101100_e153227_d_n8, assign101100_e153227_d_n9, assign101100_e153227_d_n10, assign101100_e153227_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 != 0.0)) {
        let assign101100_e153215: f64 = (locals.var_mu * locals.var_vgvt);
        let assign101100_e153217: f64 = (assign101100_e153215 * locals.var_t12);
        let assign101100_e153220: f64 = (locals.var_t10 * locals.var_t3);
        let assign101100_e153222: f64 = (assign101100_e153220 * locals.var_t3);
        let assign101100_e153223: f64 = (assign101100_e153217 + assign101100_e153222);
        let assign101100_e153225: f64 = (assign101100_e153223 + 1e-25);
        (assign101100_e153225, (((((locals.var_mu_dn0 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn0)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn0)) + ((((locals.var_t10_dn0 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn0)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn0))), (((((locals.var_mu_dn2 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn2)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn2)) + ((((locals.var_t10_dn2 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn2)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn2))), (((((locals.var_mu_dn4 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn4)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn4)) + ((((locals.var_t10_dn4 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn4)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn4))), (((((locals.var_mu_dn5 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn5)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn5)) + ((((locals.var_t10_dn5 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn5)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn5))), (((((locals.var_mu_dn6 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn6)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn6)) + ((((locals.var_t10_dn6 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn6)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn6))), (((((locals.var_mu_dn7 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn7)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn7)) + ((((locals.var_t10_dn7 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn7)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn7))), (((((locals.var_mu_dn8 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn8)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn8)) + ((((locals.var_t10_dn8 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn8)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn8))), (((((locals.var_mu_dn9 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn9)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn9)) + ((((locals.var_t10_dn9 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn9)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn9))), (((((locals.var_mu_dn10 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn10)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn10)) + ((((locals.var_t10_dn10 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn10)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn10))), (((((locals.var_mu_dn13 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn13)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn13)) + ((((locals.var_t10_dn13 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn13)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn13))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign101100_e153227;
        locals.var_t2_dn0 = assign101100_e153227_d_n0;
        locals.var_t2_dn2 = assign101100_e153227_d_n2;
        locals.var_t2_dn4 = assign101100_e153227_d_n4;
        locals.var_t2_dn5 = assign101100_e153227_d_n5;
        locals.var_t2_dn6 = assign101100_e153227_d_n6;
        locals.var_t2_dn7 = assign101100_e153227_d_n7;
        locals.var_t2_dn8 = assign101100_e153227_d_n8;
        locals.var_t2_dn9 = assign101100_e153227_d_n9;
        locals.var_t2_dn10 = assign101100_e153227_d_n10;
        locals.var_t2_dn13 = assign101100_e153227_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign101130_e153246, assign101130_e153246_d_n0, assign101130_e153246_d_n2, assign101130_e153246_d_n4, assign101130_e153246_d_n5, assign101130_e153246_d_n6, assign101130_e153246_d_n7, assign101130_e153246_d_n8, assign101130_e153246_d_n9, assign101130_e153246_d_n10, assign101130_e153246_d_n13,) = {
    if (locals.var_flg_nqs != 0.0) {
        (locals.var_mks_dly3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign101130_e153246;
        locals.var_t2_dn0 = assign101130_e153246_d_n0;
        locals.var_t2_dn2 = assign101130_e153246_d_n2;
        locals.var_t2_dn4 = assign101130_e153246_d_n4;
        locals.var_t2_dn5 = assign101130_e153246_d_n5;
        locals.var_t2_dn6 = assign101130_e153246_d_n6;
        locals.var_t2_dn7 = assign101130_e153246_d_n7;
        locals.var_t2_dn8 = assign101130_e153246_d_n8;
        locals.var_t2_dn9 = assign101130_e153246_d_n9;
        locals.var_t2_dn10 = assign101130_e153246_d_n10;
        locals.var_t2_dn13 = assign101130_e153246_d_n13;
        locals.var_t2_rv = 0.0;

        let assign101150_e153258: f64 = if ((p.p26 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2322 = assign101150_e153258;
        locals.var_guard2322_rv = 0.0;

        let (assign101160_e153262,) = {
    if (locals.var_guard2322 != 0.0) {
        (locals.var_uc_nfalp,)
    } else {
        (locals.var_nfalpe,)
    }
};
        locals.var_nfalpe = assign101160_e153262;
        locals.var_nfalpe_rv = 0.0;

        let (assign101180_e153270,) = {
    if (locals.var_guard2322 != 0.0) {
        (locals.var_mks_cit,)
    } else {
        (locals.var_cite,)
    }
};
        locals.var_cite = assign101180_e153270;
        locals.var_cite_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_376(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101190_e153276, assign101190_e153276_d_n0, assign101190_e153276_d_n2, assign101190_e153276_d_n4, assign101190_e153276_d_n5, assign101190_e153276_d_n6, assign101190_e153276_d_n7, assign101190_e153276_d_n8, assign101190_e153276_d_n9, assign101190_e153276_d_n10, assign101190_e153276_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101190_e153274: f64 = (locals.var_qn0 / 1.6021918e-19);
        (assign101190_e153274, (locals.var_qn0_dn0 / 1.6021918e-19), (locals.var_qn0_dn2 / 1.6021918e-19), (locals.var_qn0_dn4 / 1.6021918e-19), (locals.var_qn0_dn5 / 1.6021918e-19), (locals.var_qn0_dn6 / 1.6021918e-19), (locals.var_qn0_dn7 / 1.6021918e-19), (locals.var_qn0_dn8 / 1.6021918e-19), (locals.var_qn0_dn9 / 1.6021918e-19), (locals.var_qn0_dn10 / 1.6021918e-19), (locals.var_qn0_dn13 / 1.6021918e-19),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign101190_e153276;
        locals.var_t1_dn0 = assign101190_e153276_d_n0;
        locals.var_t1_dn2 = assign101190_e153276_d_n2;
        locals.var_t1_dn4 = assign101190_e153276_d_n4;
        locals.var_t1_dn5 = assign101190_e153276_d_n5;
        locals.var_t1_dn6 = assign101190_e153276_d_n6;
        locals.var_t1_dn7 = assign101190_e153276_d_n7;
        locals.var_t1_dn8 = assign101190_e153276_d_n8;
        locals.var_t1_dn9 = assign101190_e153276_d_n9;
        locals.var_t1_dn10 = assign101190_e153276_d_n10;
        locals.var_t1_dn13 = assign101190_e153276_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign101200_e153293, assign101200_e153293_d_n0, assign101200_e153293_d_n2, assign101200_e153293_d_n4, assign101200_e153293_d_n5, assign101200_e153293_d_n6, assign101200_e153293_d_n7, assign101200_e153293_d_n8, assign101200_e153293_d_n9, assign101200_e153293_d_n10, assign101200_e153293_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101200_e153280: f64 = (locals.var_ps0 - locals.var_vbscl__blk435);
        let assign101200_e153283: f64 = (locals.var_ps0 - locals.var_vbscl__blk435);
        let assign101200_e153284: f64 = (assign101200_e153280 * assign101200_e153283);
        let assign101200_e153287: f64 = (4.0 * 0.001);
        let assign101200_e153289: f64 = (assign101200_e153287 * 0.001);
        let assign101200_e153290: f64 = (assign101200_e153284 + assign101200_e153289);
        let assign101200_e153291: f64 = (assign101200_e153290).sqrt();
        (assign101200_e153291, ((((locals.var_ps0_dn0 - locals.var_vbscl__blk435_dn0) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn0 - locals.var_vbscl__blk435_dn0))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn2 - locals.var_vbscl__blk435_dn2) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn2 - locals.var_vbscl__blk435_dn2))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn4 - locals.var_vbscl__blk435_dn4) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn4 - locals.var_vbscl__blk435_dn4))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn5 - locals.var_vbscl__blk435_dn5) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn5 - locals.var_vbscl__blk435_dn5))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn6 - locals.var_vbscl__blk435_dn6) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn6 - locals.var_vbscl__blk435_dn6))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn7 - locals.var_vbscl__blk435_dn7) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn7 - locals.var_vbscl__blk435_dn7))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn8 - locals.var_vbscl__blk435_dn8) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn8 - locals.var_vbscl__blk435_dn8))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn9 - locals.var_vbscl__blk435_dn9) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn9 - locals.var_vbscl__blk435_dn9))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn10 - locals.var_vbscl__blk435_dn10) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn10 - locals.var_vbscl__blk435_dn10))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn13 - locals.var_vbscl__blk435_dn13) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn13 - locals.var_vbscl__blk435_dn13))) / (2.0 * assign101200_e153291)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign101200_e153293;
        locals.var_tmf2_dn0 = assign101200_e153293_d_n0;
        locals.var_tmf2_dn2 = assign101200_e153293_d_n2;
        locals.var_tmf2_dn4 = assign101200_e153293_d_n4;
        locals.var_tmf2_dn5 = assign101200_e153293_d_n5;
        locals.var_tmf2_dn6 = assign101200_e153293_d_n6;
        locals.var_tmf2_dn7 = assign101200_e153293_d_n7;
        locals.var_tmf2_dn8 = assign101200_e153293_d_n8;
        locals.var_tmf2_dn9 = assign101200_e153293_d_n9;
        locals.var_tmf2_dn10 = assign101200_e153293_d_n10;
        locals.var_tmf2_dn13 = assign101200_e153293_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign101210_e153305, assign101210_e153305_d_n0, assign101210_e153305_d_n2, assign101210_e153305_d_n4, assign101210_e153305_d_n5, assign101210_e153305_d_n6, assign101210_e153305_d_n7, assign101210_e153305_d_n8, assign101210_e153305_d_n9, assign101210_e153305_d_n10, assign101210_e153305_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101210_e153299: f64 = (locals.var_ps0 - locals.var_vbscl__blk435);
        let assign101210_e153301: f64 = (assign101210_e153299 / locals.var_tmf2);
        let assign101210_e153302: f64 = (1.0 + assign101210_e153301);
        let assign101210_e153303: f64 = (0.5 * assign101210_e153302);
        (assign101210_e153303, (0.5 * ((((locals.var_ps0_dn0 - locals.var_vbscl__blk435_dn0) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn2 - locals.var_vbscl__blk435_dn2) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn4 - locals.var_vbscl__blk435_dn4) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn5 - locals.var_vbscl__blk435_dn5) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn6 - locals.var_vbscl__blk435_dn6) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn7 - locals.var_vbscl__blk435_dn7) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn8 - locals.var_vbscl__blk435_dn8) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn9 - locals.var_vbscl__blk435_dn9) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn10 - locals.var_vbscl__blk435_dn10) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn13 - locals.var_vbscl__blk435_dn13) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign101210_e153305;
        locals.var_t0_dn0 = assign101210_e153305_d_n0;
        locals.var_t0_dn2 = assign101210_e153305_d_n2;
        locals.var_t0_dn4 = assign101210_e153305_d_n4;
        locals.var_t0_dn5 = assign101210_e153305_d_n5;
        locals.var_t0_dn6 = assign101210_e153305_d_n6;
        locals.var_t0_dn7 = assign101210_e153305_d_n7;
        locals.var_t0_dn8 = assign101210_e153305_d_n8;
        locals.var_t0_dn9 = assign101210_e153305_d_n9;
        locals.var_t0_dn10 = assign101210_e153305_d_n10;
        locals.var_t0_dn13 = assign101210_e153305_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign101220_e153315, assign101220_e153315_d_n0, assign101220_e153315_d_n2, assign101220_e153315_d_n4, assign101220_e153315_d_n5, assign101220_e153315_d_n6, assign101220_e153315_d_n7, assign101220_e153315_d_n8, assign101220_e153315_d_n9, assign101220_e153315_d_n10, assign101220_e153315_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101220_e153310: f64 = (locals.var_ps0 - locals.var_vbscl__blk435);
        let assign101220_e153312: f64 = (assign101220_e153310 + locals.var_tmf2);
        let assign101220_e153313: f64 = (0.5 * assign101220_e153312);
        (assign101220_e153313, (0.5 * ((locals.var_ps0_dn0 - locals.var_vbscl__blk435_dn0) + locals.var_tmf2_dn0)), (0.5 * ((locals.var_ps0_dn2 - locals.var_vbscl__blk435_dn2) + locals.var_tmf2_dn2)), (0.5 * ((locals.var_ps0_dn4 - locals.var_vbscl__blk435_dn4) + locals.var_tmf2_dn4)), (0.5 * ((locals.var_ps0_dn5 - locals.var_vbscl__blk435_dn5) + locals.var_tmf2_dn5)), (0.5 * ((locals.var_ps0_dn6 - locals.var_vbscl__blk435_dn6) + locals.var_tmf2_dn6)), (0.5 * ((locals.var_ps0_dn7 - locals.var_vbscl__blk435_dn7) + locals.var_tmf2_dn7)), (0.5 * ((locals.var_ps0_dn8 - locals.var_vbscl__blk435_dn8) + locals.var_tmf2_dn8)), (0.5 * ((locals.var_ps0_dn9 - locals.var_vbscl__blk435_dn9) + locals.var_tmf2_dn9)), (0.5 * ((locals.var_ps0_dn10 - locals.var_vbscl__blk435_dn10) + locals.var_tmf2_dn10)), (0.5 * ((locals.var_ps0_dn13 - locals.var_vbscl__blk435_dn13) + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign101220_e153315;
        locals.var_t5_dn0 = assign101220_e153315_d_n0;
        locals.var_t5_dn2 = assign101220_e153315_d_n2;
        locals.var_t5_dn4 = assign101220_e153315_d_n4;
        locals.var_t5_dn5 = assign101220_e153315_d_n5;
        locals.var_t5_dn6 = assign101220_e153315_d_n6;
        locals.var_t5_dn7 = assign101220_e153315_d_n7;
        locals.var_t5_dn8 = assign101220_e153315_d_n8;
        locals.var_t5_dn9 = assign101220_e153315_d_n9;
        locals.var_t5_dn10 = assign101220_e153315_d_n10;
        locals.var_t5_dn13 = assign101220_e153315_d_n13;
        locals.var_t5_rv = 0.0;

        let assign101230_e153318: f64 = if locals.var_t5 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2323 = assign101230_e153318;
        locals.var_guard2323_rv = 0.0;

        let (assign101240_e153324, assign101240_e153324_d_n0, assign101240_e153324_d_n2, assign101240_e153324_d_n4, assign101240_e153324_d_n5, assign101240_e153324_d_n6, assign101240_e153324_d_n7, assign101240_e153324_d_n8, assign101240_e153324_d_n9, assign101240_e153324_d_n10, assign101240_e153324_d_n13,) = {
    if ((locals.var_guard2322 != 0.0) && (locals.var_guard2323 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign101240_e153324;
        locals.var_t5_dn0 = assign101240_e153324_d_n0;
        locals.var_t5_dn2 = assign101240_e153324_d_n2;
        locals.var_t5_dn4 = assign101240_e153324_d_n4;
        locals.var_t5_dn5 = assign101240_e153324_d_n5;
        locals.var_t5_dn6 = assign101240_e153324_d_n6;
        locals.var_t5_dn7 = assign101240_e153324_d_n7;
        locals.var_t5_dn8 = assign101240_e153324_d_n8;
        locals.var_t5_dn9 = assign101240_e153324_d_n9;
        locals.var_t5_dn10 = assign101240_e153324_d_n10;
        locals.var_t5_dn13 = assign101240_e153324_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign101250_e153330, assign101250_e153330_d_n0, assign101250_e153330_d_n2, assign101250_e153330_d_n4, assign101250_e153330_d_n5, assign101250_e153330_d_n6, assign101250_e153330_d_n7, assign101250_e153330_d_n8, assign101250_e153330_d_n9, assign101250_e153330_d_n10, assign101250_e153330_d_n13,) = {
    if ((locals.var_guard2322 != 0.0) && (locals.var_guard2323 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign101250_e153330;
        locals.var_t0_dn0 = assign101250_e153330_d_n0;
        locals.var_t0_dn2 = assign101250_e153330_d_n2;
        locals.var_t0_dn4 = assign101250_e153330_d_n4;
        locals.var_t0_dn5 = assign101250_e153330_d_n5;
        locals.var_t0_dn6 = assign101250_e153330_d_n6;
        locals.var_t0_dn7 = assign101250_e153330_d_n7;
        locals.var_t0_dn8 = assign101250_e153330_d_n8;
        locals.var_t0_dn9 = assign101250_e153330_d_n9;
        locals.var_t0_dn10 = assign101250_e153330_d_n10;
        locals.var_t0_dn13 = assign101250_e153330_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign101260_e153344, assign101260_e153344_d_n0, assign101260_e153344_d_n2, assign101260_e153344_d_n4, assign101260_e153344_d_n5, assign101260_e153344_d_n6, assign101260_e153344_d_n7, assign101260_e153344_d_n8, assign101260_e153344_d_n9, assign101260_e153344_d_n10, assign101260_e153344_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101260_e153335: f64 = (locals.var_qn0 / locals.var_t5);
        let assign101260_e153336: f64 = (locals.var_cox + assign101260_e153335);
        let assign101260_e153338: f64 = (assign101260_e153336 + locals.var_cite);
        let assign101260_e153340: f64 = (assign101260_e153338 * locals.var_beta_inv);
        let assign101260_e153342: f64 = (assign101260_e153340 / 1.6021918e-19);
        (assign101260_e153342, ((((locals.var_cox_dn0 + (((locals.var_qn0_dn0 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn0)) / 1.6021918e-19), ((((locals.var_cox_dn2 + (((locals.var_qn0_dn2 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn2)) / 1.6021918e-19), ((((locals.var_cox_dn4 + (((locals.var_qn0_dn4 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn4)) / 1.6021918e-19), ((((locals.var_cox_dn5 + (((locals.var_qn0_dn5 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn5)) / 1.6021918e-19), ((((locals.var_cox_dn6 + (((locals.var_qn0_dn6 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn6)) / 1.6021918e-19), ((((locals.var_cox_dn7 + (((locals.var_qn0_dn7 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn7)) / 1.6021918e-19), ((((locals.var_cox_dn8 + (((locals.var_qn0_dn8 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn8)) / 1.6021918e-19), ((((locals.var_cox_dn9 + (((locals.var_qn0_dn9 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn9)) / 1.6021918e-19), ((((locals.var_cox_dn10 + (((locals.var_qn0_dn10 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn10)) / 1.6021918e-19), ((((locals.var_cox_dn13 + (((locals.var_qn0_dn13 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn13)) / 1.6021918e-19),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign101260_e153344;
        locals.var_t2_dn0 = assign101260_e153344_d_n0;
        locals.var_t2_dn2 = assign101260_e153344_d_n2;
        locals.var_t2_dn4 = assign101260_e153344_d_n4;
        locals.var_t2_dn5 = assign101260_e153344_d_n5;
        locals.var_t2_dn6 = assign101260_e153344_d_n6;
        locals.var_t2_dn7 = assign101260_e153344_d_n7;
        locals.var_t2_dn8 = assign101260_e153344_d_n8;
        locals.var_t2_dn9 = assign101260_e153344_d_n9;
        locals.var_t2_dn10 = assign101260_e153344_d_n10;
        locals.var_t2_dn13 = assign101260_e153344_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign101270_e153359, assign101270_e153359_d_n0, assign101270_e153359_d_n2, assign101270_e153359_d_n4, assign101270_e153359_d_n5, assign101270_e153359_d_n6, assign101270_e153359_d_n7, assign101270_e153359_d_n8, assign101270_e153359_d_n9, assign101270_e153359_d_n10, assign101270_e153359_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101270_e153347: f64 = (-2.0);
        let assign101270_e153349: f64 = (assign101270_e153347 * locals.var_qi_noi);
        let assign101270_e153351: f64 = (assign101270_e153349 / 1.6021918e-19);
        let assign101270_e153353: f64 = (assign101270_e153351 / locals.var_lch);
        let assign101270_e153355: f64 = (assign101270_e153353 / locals.var_weffcv_nf);
        let assign101270_e153357: f64 = (assign101270_e153355 - locals.var_t1);
        (assign101270_e153357, (((((((assign101270_e153347 * locals.var_qi_noi_dn0) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn0), (((((((assign101270_e153347 * locals.var_qi_noi_dn2) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn2), (((((((assign101270_e153347 * locals.var_qi_noi_dn4) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn4), (((((((assign101270_e153347 * locals.var_qi_noi_dn5) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn5), (((((((assign101270_e153347 * locals.var_qi_noi_dn6) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn6), (((((((assign101270_e153347 * locals.var_qi_noi_dn7) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn7), (((((((assign101270_e153347 * locals.var_qi_noi_dn8) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn8), (((((((assign101270_e153347 * locals.var_qi_noi_dn9) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn9), (((((((assign101270_e153347 * locals.var_qi_noi_dn10) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn10), (((((((assign101270_e153347 * locals.var_qi_noi_dn13) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn13),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign101270_e153359;
        locals.var_t3_dn0 = assign101270_e153359_d_n0;
        locals.var_t3_dn2 = assign101270_e153359_d_n2;
        locals.var_t3_dn4 = assign101270_e153359_d_n4;
        locals.var_t3_dn5 = assign101270_e153359_d_n5;
        locals.var_t3_dn6 = assign101270_e153359_d_n6;
        locals.var_t3_dn7 = assign101270_e153359_d_n7;
        locals.var_t3_dn8 = assign101270_e153359_d_n8;
        locals.var_t3_dn9 = assign101270_e153359_d_n9;
        locals.var_t3_dn10 = assign101270_e153359_d_n10;
        locals.var_t3_dn13 = assign101270_e153359_d_n13;
        locals.var_t3_rv = 0.0;

        let assign101280_e153362: f64 = (locals.var_t3 - locals.var_t1);
        let assign101280_e153363: f64 = (assign101280_e153362).abs();
        let assign101280_e153366: f64 = (10.0 * 2.220446049250313e-16);
        let assign101280_e153367: f64 = if assign101280_e153363 > assign101280_e153366 { 1.0 } else { 0.0 };
        locals.var_guard2324 = assign101280_e153367;
        locals.var_guard2324_rv = 0.0;

        let (assign101290_e153414, assign101290_e153414_d_n0, assign101290_e153414_d_n2, assign101290_e153414_d_n4, assign101290_e153414_d_n5, assign101290_e153414_d_n6, assign101290_e153414_d_n7, assign101290_e153414_d_n8, assign101290_e153414_d_n9, assign101290_e153414_d_n10, assign101290_e153414_d_n13,) = {
    if ((locals.var_guard2322 != 0.0) && (locals.var_guard2324 != 0.0)) {
        let assign101290_e153374: f64 = (locals.var_t1 + locals.var_t2);
        let assign101290_e153375: f64 = (1.0 / assign101290_e153374);
        let assign101290_e153378: f64 = (locals.var_t3 + locals.var_t2);
        let assign101290_e153379: f64 = (assign101290_e153375 / assign101290_e153378);
        let assign101290_e153382: f64 = (2.0 * locals.var_nfalpe);
        let assign101290_e153384: f64 = (assign101290_e153382 * locals.var_ey);
        let assign101290_e153386: f64 = (assign101290_e153384 * locals.var_mu);
        let assign101290_e153389: f64 = (locals.var_t3 - locals.var_t1);
        let assign101290_e153390: f64 = (assign101290_e153386 / assign101290_e153389);
        let assign101290_e153393: f64 = (locals.var_t3 + locals.var_t2);
        let assign101290_e153396: f64 = (locals.var_t1 + locals.var_t2);
        let assign101290_e153397: f64 = (assign101290_e153393 / assign101290_e153396);
        let assign101290_e153398: f64 = (assign101290_e153397).ln();
        let assign101290_e153399: f64 = (assign101290_e153390 * assign101290_e153398);
        let assign101290_e153400: f64 = (assign101290_e153379 + assign101290_e153399);
        let assign101290_e153403: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101290_e153405: f64 = (assign101290_e153403 * locals.var_mu);
        let assign101290_e153407: f64 = (assign101290_e153405 * locals.var_nfalpe);
        let assign101290_e153409: f64 = (assign101290_e153407 * locals.var_ey);
        let assign101290_e153411: f64 = (assign101290_e153409 * locals.var_mu);
        let assign101290_e153412: f64 = (assign101290_e153400 + assign101290_e153411);
        (assign101290_e153412, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn0) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn0)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn0 - locals.var_t1_dn0))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn0 + locals.var_t2_dn0) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn0)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn2) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn2)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn2 - locals.var_t1_dn2))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn2 + locals.var_t2_dn2) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn2)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn4) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn4)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn4 - locals.var_t1_dn4))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn4 + locals.var_t2_dn4) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn4)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn5) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn5)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn5 - locals.var_t1_dn5))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn5 + locals.var_t2_dn5) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn5)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn6) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn6)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn6 - locals.var_t1_dn6))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn6 + locals.var_t2_dn6) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn6)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn7) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn7)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn7 - locals.var_t1_dn7))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn7 + locals.var_t2_dn7) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn7)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn8) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn8)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn8 - locals.var_t1_dn8))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn8 + locals.var_t2_dn8) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn8)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn9) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn9)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn9 - locals.var_t1_dn9))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn9 + locals.var_t2_dn9) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn9)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn10) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn10)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn10 - locals.var_t1_dn10))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn10 + locals.var_t2_dn10) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn10)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn13 + locals.var_t2_dn13) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn13 + locals.var_t2_dn13))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn13) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn13)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn13 - locals.var_t1_dn13))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn13 + locals.var_t2_dn13) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn13 + locals.var_t2_dn13))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn13) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn13)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn13)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign101290_e153414;
        locals.var_t4_dn0 = assign101290_e153414_d_n0;
        locals.var_t4_dn2 = assign101290_e153414_d_n2;
        locals.var_t4_dn4 = assign101290_e153414_d_n4;
        locals.var_t4_dn5 = assign101290_e153414_d_n5;
        locals.var_t4_dn6 = assign101290_e153414_d_n6;
        locals.var_t4_dn7 = assign101290_e153414_d_n7;
        locals.var_t4_dn8 = assign101290_e153414_d_n8;
        locals.var_t4_dn9 = assign101290_e153414_d_n9;
        locals.var_t4_dn10 = assign101290_e153414_d_n10;
        locals.var_t4_dn13 = assign101290_e153414_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign101300_e153453, assign101300_e153453_d_n0, assign101300_e153453_d_n2, assign101300_e153453_d_n4, assign101300_e153453_d_n5, assign101300_e153453_d_n6, assign101300_e153453_d_n7, assign101300_e153453_d_n8, assign101300_e153453_d_n9, assign101300_e153453_d_n10, assign101300_e153453_d_n13,) = {
    if ((locals.var_guard2322 != 0.0) && (locals.var_guard2324 == 0.0)) {
        let assign101300_e153422: f64 = (locals.var_t1 + locals.var_t2);
        let assign101300_e153423: f64 = (1.0 / assign101300_e153422);
        let assign101300_e153426: f64 = (locals.var_t3 + locals.var_t2);
        let assign101300_e153427: f64 = (assign101300_e153423 / assign101300_e153426);
        let assign101300_e153430: f64 = (2.0 * locals.var_nfalpe);
        let assign101300_e153432: f64 = (assign101300_e153430 * locals.var_ey);
        let assign101300_e153434: f64 = (assign101300_e153432 * locals.var_mu);
        let assign101300_e153437: f64 = (locals.var_t1 + locals.var_t2);
        let assign101300_e153438: f64 = (assign101300_e153434 / assign101300_e153437);
        let assign101300_e153439: f64 = (assign101300_e153427 + assign101300_e153438);
        let assign101300_e153442: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101300_e153444: f64 = (assign101300_e153442 * locals.var_mu);
        let assign101300_e153446: f64 = (assign101300_e153444 * locals.var_nfalpe);
        let assign101300_e153448: f64 = (assign101300_e153446 * locals.var_ey);
        let assign101300_e153450: f64 = (assign101300_e153448 * locals.var_mu);
        let assign101300_e153451: f64 = (assign101300_e153439 + assign101300_e153450);
        (assign101300_e153451, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn0) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn0)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn0)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn2) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn2)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn2)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn4) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn4)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn4)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn5) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn5)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn5)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn6) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn6)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn6)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn7) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn7)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn7)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn8) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn8)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn8)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn9) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn9)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn9)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn10) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn10)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn10)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn13 + locals.var_t2_dn13) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn13 + locals.var_t2_dn13))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn13) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn13)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn13 + locals.var_t2_dn13))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn13) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn13)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn13)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign101300_e153453;
        locals.var_t4_dn0 = assign101300_e153453_d_n0;
        locals.var_t4_dn2 = assign101300_e153453_d_n2;
        locals.var_t4_dn4 = assign101300_e153453_d_n4;
        locals.var_t4_dn5 = assign101300_e153453_d_n5;
        locals.var_t4_dn6 = assign101300_e153453_d_n6;
        locals.var_t4_dn7 = assign101300_e153453_d_n7;
        locals.var_t4_dn8 = assign101300_e153453_d_n8;
        locals.var_t4_dn9 = assign101300_e153453_d_n9;
        locals.var_t4_dn10 = assign101300_e153453_d_n10;
        locals.var_t4_dn13 = assign101300_e153453_d_n13;
        locals.var_t4_rv = 0.0;

        let assign101330_e153484: f64 = if (((p.p30 != 0.0) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2325 = assign101330_e153484;
        locals.var_guard2325_rv = 0.0;

        let (assign101340_e153496, assign101340_e153496_d_n0, assign101340_e153496_d_n2, assign101340_e153496_d_n4, assign101340_e153496_d_n5, assign101340_e153496_d_n6, assign101340_e153496_d_n7, assign101340_e153496_d_n8, assign101340_e153496_d_n9, assign101340_e153496_d_n10, assign101340_e153496_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101340_e153488: f64 = (locals.var_psdl - locals.var_ps0);
        let assign101340_e153491: f64 = (10.0 * 2.220446049250313e-16);
        let assign101340_e153492: f64 = (assign101340_e153488 + assign101340_e153491);
        let assign101340_e153494: f64 = (assign101340_e153492 / locals.var_lch);
        (assign101340_e153494, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn4 - locals.var_ps0_dn4) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn5 - locals.var_ps0_dn5) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn8 - locals.var_ps0_dn8) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn9 - locals.var_ps0_dn9) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn13 - locals.var_ps0_dn13) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn13,)
    }
};
        locals.var_eyd = assign101340_e153496;
        locals.var_eyd_dn0 = assign101340_e153496_d_n0;
        locals.var_eyd_dn2 = assign101340_e153496_d_n2;
        locals.var_eyd_dn4 = assign101340_e153496_d_n4;
        locals.var_eyd_dn5 = assign101340_e153496_d_n5;
        locals.var_eyd_dn6 = assign101340_e153496_d_n6;
        locals.var_eyd_dn7 = assign101340_e153496_d_n7;
        locals.var_eyd_dn8 = assign101340_e153496_d_n8;
        locals.var_eyd_dn9 = assign101340_e153496_d_n9;
        locals.var_eyd_dn10 = assign101340_e153496_d_n10;
        locals.var_eyd_dn13 = assign101340_e153496_d_n13;
        locals.var_eyd_rv = 0.0;

        let (assign101350_e153505, assign101350_e153505_d_n0, assign101350_e153505_d_n2, assign101350_e153505_d_n4, assign101350_e153505_d_n5, assign101350_e153505_d_n6, assign101350_e153505_d_n7, assign101350_e153505_d_n8, assign101350_e153505_d_n9, assign101350_e153505_d_n10, assign101350_e153505_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let (assign101350_e153503, assign101350_e153503_d_n0, assign101350_e153503_d_n2, assign101350_e153503_d_n4, assign101350_e153503_d_n5, assign101350_e153503_d_n6, assign101350_e153503_d_n7, assign101350_e153503_d_n8, assign101350_e153503_d_n9, assign101350_e153503_d_n10, assign101350_e153503_d_n13,) = {
            if (locals.var_eyd >= 0.0) {
                (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign101350_e153503, assign101350_e153503_d_n0, assign101350_e153503_d_n2, assign101350_e153503_d_n4, assign101350_e153503_d_n5, assign101350_e153503_d_n6, assign101350_e153503_d_n7, assign101350_e153503_d_n8, assign101350_e153503_d_n9, assign101350_e153503_d_n10, assign101350_e153503_d_n13,)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn13,)
    }
};
        locals.var_eyd = assign101350_e153505;
        locals.var_eyd_dn0 = assign101350_e153505_d_n0;
        locals.var_eyd_dn2 = assign101350_e153505_d_n2;
        locals.var_eyd_dn4 = assign101350_e153505_d_n4;
        locals.var_eyd_dn5 = assign101350_e153505_d_n5;
        locals.var_eyd_dn6 = assign101350_e153505_d_n6;
        locals.var_eyd_dn7 = assign101350_e153505_d_n7;
        locals.var_eyd_dn8 = assign101350_e153505_d_n8;
        locals.var_eyd_dn9 = assign101350_e153505_d_n9;
        locals.var_eyd_dn10 = assign101350_e153505_d_n10;
        locals.var_eyd_dn13 = assign101350_e153505_d_n13;
        locals.var_eyd_rv = 0.0;

        let (assign101360_e153513, assign101360_e153513_d_n0, assign101360_e153513_d_n2, assign101360_e153513_d_n4, assign101360_e153513_d_n5, assign101360_e153513_d_n6, assign101360_e153513_d_n7, assign101360_e153513_d_n8, assign101360_e153513_d_n9, assign101360_e153513_d_n10, assign101360_e153513_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101360_e153509: f64 = (locals.var_muun * locals.var_eyd);
        let assign101360_e153511: f64 = (assign101360_e153509 / 10000000.0);
        (assign101360_e153511, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 10000000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 10000000.0), (((locals.var_muun_dn4 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn4)) / 10000000.0), (((locals.var_muun_dn5 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn5)) / 10000000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 10000000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 10000000.0), (((locals.var_muun_dn8 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn8)) / 10000000.0), (((locals.var_muun_dn9 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn9)) / 10000000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 10000000.0), (((locals.var_muun_dn13 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn13)) / 10000000.0),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn13,)
    }
};
        locals.var_t12 = assign101360_e153513;
        locals.var_t12_dn0 = assign101360_e153513_d_n0;
        locals.var_t12_dn2 = assign101360_e153513_d_n2;
        locals.var_t12_dn4 = assign101360_e153513_d_n4;
        locals.var_t12_dn5 = assign101360_e153513_d_n5;
        locals.var_t12_dn6 = assign101360_e153513_d_n6;
        locals.var_t12_dn7 = assign101360_e153513_d_n7;
        locals.var_t12_dn8 = assign101360_e153513_d_n8;
        locals.var_t12_dn9 = assign101360_e153513_d_n9;
        locals.var_t12_dn10 = assign101360_e153513_d_n10;
        locals.var_t12_dn13 = assign101360_e153513_d_n13;
        locals.var_t12_rv = 0.0;

        let assign101370_e153517: f64 = (10.0 * 2.220446049250313e-16);
        let assign101370_e153518: f64 = (1.0 - assign101370_e153517);
        let assign101370_e153525: f64 = (10.0 * 2.220446049250313e-16);
        let assign101370_e153526: f64 = (1.0 + assign101370_e153525);
        let assign101370_e153528: f64 = if ((assign101370_e153518 <= p.p178) && (p.p178 <= assign101370_e153526)) { 1.0 } else { 0.0 };
        locals.var_guard2326 = assign101370_e153528;
        locals.var_guard2326_rv = 0.0;

        let (assign101380_e153534, assign101380_e153534_d_n0, assign101380_e153534_d_n2, assign101380_e153534_d_n4, assign101380_e153534_d_n5, assign101380_e153534_d_n6, assign101380_e153534_d_n7, assign101380_e153534_d_n8, assign101380_e153534_d_n9, assign101380_e153534_d_n10, assign101380_e153534_d_n13,) = {
    if ((locals.var_guard2325 != 0.0) && (locals.var_guard2326 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign101380_e153534;
        locals.var_t7_dn0 = assign101380_e153534_d_n0;
        locals.var_t7_dn2 = assign101380_e153534_d_n2;
        locals.var_t7_dn4 = assign101380_e153534_d_n4;
        locals.var_t7_dn5 = assign101380_e153534_d_n5;
        locals.var_t7_dn6 = assign101380_e153534_d_n6;
        locals.var_t7_dn7 = assign101380_e153534_d_n7;
        locals.var_t7_dn8 = assign101380_e153534_d_n8;
        locals.var_t7_dn9 = assign101380_e153534_d_n9;
        locals.var_t7_dn10 = assign101380_e153534_d_n10;
        locals.var_t7_dn13 = assign101380_e153534_d_n13;
        locals.var_t7_rv = 0.0;

        let assign101390_e153538: f64 = (10.0 * 2.220446049250313e-16);
        let assign101390_e153539: f64 = (2.0 - assign101390_e153538);
        let assign101390_e153546: f64 = (10.0 * 2.220446049250313e-16);
        let assign101390_e153547: f64 = (2.0 + assign101390_e153546);
        let assign101390_e153549: f64 = if ((assign101390_e153539 <= p.p178) && (p.p178 <= assign101390_e153547)) { 1.0 } else { 0.0 };
        locals.var_guard2327 = assign101390_e153549;
        locals.var_guard2327_rv = 0.0;

        let (assign101400_e153558, assign101400_e153558_d_n0, assign101400_e153558_d_n2, assign101400_e153558_d_n4, assign101400_e153558_d_n5, assign101400_e153558_d_n6, assign101400_e153558_d_n7, assign101400_e153558_d_n8, assign101400_e153558_d_n9, assign101400_e153558_d_n10, assign101400_e153558_d_n13,) = {
    if (((locals.var_guard2325 != 0.0) && (locals.var_guard2326 == 0.0)) && (locals.var_guard2327 != 0.0)) {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign101400_e153558;
        locals.var_t7_dn0 = assign101400_e153558_d_n0;
        locals.var_t7_dn2 = assign101400_e153558_d_n2;
        locals.var_t7_dn4 = assign101400_e153558_d_n4;
        locals.var_t7_dn5 = assign101400_e153558_d_n5;
        locals.var_t7_dn6 = assign101400_e153558_d_n6;
        locals.var_t7_dn7 = assign101400_e153558_d_n7;
        locals.var_t7_dn8 = assign101400_e153558_d_n8;
        locals.var_t7_dn9 = assign101400_e153558_d_n9;
        locals.var_t7_dn10 = assign101400_e153558_d_n10;
        locals.var_t7_dn13 = assign101400_e153558_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign101410_e153577, assign101410_e153577_d_n0, assign101410_e153577_d_n2, assign101410_e153577_d_n4, assign101410_e153577_d_n5, assign101410_e153577_d_n6, assign101410_e153577_d_n7, assign101410_e153577_d_n8, assign101410_e153577_d_n9, assign101410_e153577_d_n10, assign101410_e153577_d_n13,) = {
    if (((locals.var_guard2325 != 0.0) && (locals.var_guard2326 == 0.0)) && (locals.var_guard2327 == 0.0)) {
        let (assign101410_e153575, assign101410_e153575_d_n0, assign101410_e153575_d_n2, assign101410_e153575_d_n4, assign101410_e153575_d_n5, assign101410_e153575_d_n6, assign101410_e153575_d_n7, assign101410_e153575_d_n8, assign101410_e153575_d_n9, assign101410_e153575_d_n10, assign101410_e153575_d_n13,) = {
            if (locals.var_eyd == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101410_e153573: f64 = (p.p178 - 1.0);
                let assign101410_e153574: f64 = (locals.var_eyd).powf(assign101410_e153573);
                (assign101410_e153574, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn0)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn0 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn2)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn2 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn4)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn4 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn5)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn5 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn6)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn6 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn7)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn7 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn8)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn8 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn9)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn9 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn10)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn10 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn13)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn13 / locals.var_eyd))) },)
            }
        };
        (assign101410_e153575, assign101410_e153575_d_n0, assign101410_e153575_d_n2, assign101410_e153575_d_n4, assign101410_e153575_d_n5, assign101410_e153575_d_n6, assign101410_e153575_d_n7, assign101410_e153575_d_n8, assign101410_e153575_d_n9, assign101410_e153575_d_n10, assign101410_e153575_d_n13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign101410_e153577;
        locals.var_t7_dn0 = assign101410_e153577_d_n0;
        locals.var_t7_dn2 = assign101410_e153577_d_n2;
        locals.var_t7_dn4 = assign101410_e153577_d_n4;
        locals.var_t7_dn5 = assign101410_e153577_d_n5;
        locals.var_t7_dn6 = assign101410_e153577_d_n6;
        locals.var_t7_dn7 = assign101410_e153577_d_n7;
        locals.var_t7_dn8 = assign101410_e153577_d_n8;
        locals.var_t7_dn9 = assign101410_e153577_d_n9;
        locals.var_t7_dn10 = assign101410_e153577_d_n10;
        locals.var_t7_dn13 = assign101410_e153577_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign101420_e153583, assign101420_e153583_d_n0, assign101420_e153583_d_n2, assign101420_e153583_d_n4, assign101420_e153583_d_n5, assign101420_e153583_d_n6, assign101420_e153583_d_n7, assign101420_e153583_d_n8, assign101420_e153583_d_n9, assign101420_e153583_d_n10, assign101420_e153583_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101420_e153581: f64 = (locals.var_t12 * locals.var_t7);
        (assign101420_e153581, ((locals.var_t12_dn0 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn0)), ((locals.var_t12_dn2 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn2)), ((locals.var_t12_dn4 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn4)), ((locals.var_t12_dn5 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn5)), ((locals.var_t12_dn6 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn6)), ((locals.var_t12_dn7 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn7)), ((locals.var_t12_dn8 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn8)), ((locals.var_t12_dn9 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn9)), ((locals.var_t12_dn10 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn10)), ((locals.var_t12_dn13 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign101420_e153583;
        locals.var_t8_dn0 = assign101420_e153583_d_n0;
        locals.var_t8_dn2 = assign101420_e153583_d_n2;
        locals.var_t8_dn4 = assign101420_e153583_d_n4;
        locals.var_t8_dn5 = assign101420_e153583_d_n5;
        locals.var_t8_dn6 = assign101420_e153583_d_n6;
        locals.var_t8_dn7 = assign101420_e153583_d_n7;
        locals.var_t8_dn8 = assign101420_e153583_d_n8;
        locals.var_t8_dn9 = assign101420_e153583_d_n9;
        locals.var_t8_dn10 = assign101420_e153583_d_n10;
        locals.var_t8_dn13 = assign101420_e153583_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign101430_e153589, assign101430_e153589_d_n0, assign101430_e153589_d_n2, assign101430_e153589_d_n4, assign101430_e153589_d_n5, assign101430_e153589_d_n6, assign101430_e153589_d_n7, assign101430_e153589_d_n8, assign101430_e153589_d_n9, assign101430_e153589_d_n10, assign101430_e153589_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101430_e153587: f64 = (1.0 + locals.var_t8);
        (assign101430_e153587, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign101430_e153589;
        locals.var_t9_dn0 = assign101430_e153589_d_n0;
        locals.var_t9_dn2 = assign101430_e153589_d_n2;
        locals.var_t9_dn4 = assign101430_e153589_d_n4;
        locals.var_t9_dn5 = assign101430_e153589_d_n5;
        locals.var_t9_dn6 = assign101430_e153589_d_n6;
        locals.var_t9_dn7 = assign101430_e153589_d_n7;
        locals.var_t9_dn8 = assign101430_e153589_d_n8;
        locals.var_t9_dn9 = assign101430_e153589_d_n9;
        locals.var_t9_dn10 = assign101430_e153589_d_n10;
        locals.var_t9_dn13 = assign101430_e153589_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign101440_e153605, assign101440_e153605_d_n0, assign101440_e153605_d_n2, assign101440_e153605_d_n4, assign101440_e153605_d_n5, assign101440_e153605_d_n6, assign101440_e153605_d_n7, assign101440_e153605_d_n8, assign101440_e153605_d_n9, assign101440_e153605_d_n10, assign101440_e153605_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let (assign101440_e153603, assign101440_e153603_d_n0, assign101440_e153603_d_n2, assign101440_e153603_d_n4, assign101440_e153603_d_n5, assign101440_e153603_d_n6, assign101440_e153603_d_n7, assign101440_e153603_d_n8, assign101440_e153603_d_n9, assign101440_e153603_d_n10, assign101440_e153603_d_n13,) = {
            if (locals.var_t9 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101440_e153597: f64 = (-1.0);
                let assign101440_e153599: f64 = (assign101440_e153597 / p.p178);
                let assign101440_e153601: f64 = (assign101440_e153599 - 1.0);
                let assign101440_e153602: f64 = (locals.var_t9).powf(assign101440_e153601);
                (assign101440_e153602, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn0)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn0 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn2)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn2 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn4)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn4 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn5)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn5 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn6)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn6 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn7)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn7 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn8)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn8 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn9)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn9 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn10)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn10 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn13)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn13 / locals.var_t9))) },)
            }
        };
        (assign101440_e153603, assign101440_e153603_d_n0, assign101440_e153603_d_n2, assign101440_e153603_d_n4, assign101440_e153603_d_n5, assign101440_e153603_d_n6, assign101440_e153603_d_n7, assign101440_e153603_d_n8, assign101440_e153603_d_n9, assign101440_e153603_d_n10, assign101440_e153603_d_n13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign101440_e153605;
        locals.var_t10_dn0 = assign101440_e153605_d_n0;
        locals.var_t10_dn2 = assign101440_e153605_d_n2;
        locals.var_t10_dn4 = assign101440_e153605_d_n4;
        locals.var_t10_dn5 = assign101440_e153605_d_n5;
        locals.var_t10_dn6 = assign101440_e153605_d_n6;
        locals.var_t10_dn7 = assign101440_e153605_d_n7;
        locals.var_t10_dn8 = assign101440_e153605_d_n8;
        locals.var_t10_dn9 = assign101440_e153605_d_n9;
        locals.var_t10_dn10 = assign101440_e153605_d_n10;
        locals.var_t10_dn13 = assign101440_e153605_d_n13;
        locals.var_t10_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_377(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101450_e153611, assign101450_e153611_d_n0, assign101450_e153611_d_n2, assign101450_e153611_d_n4, assign101450_e153611_d_n5, assign101450_e153611_d_n6, assign101450_e153611_d_n7, assign101450_e153611_d_n8, assign101450_e153611_d_n9, assign101450_e153611_d_n10, assign101450_e153611_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101450_e153609: f64 = (locals.var_t9 * locals.var_t10);
        (assign101450_e153609, ((locals.var_t9_dn0 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn0)), ((locals.var_t9_dn2 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn2)), ((locals.var_t9_dn4 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn4)), ((locals.var_t9_dn5 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn5)), ((locals.var_t9_dn6 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn6)), ((locals.var_t9_dn7 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn7)), ((locals.var_t9_dn8 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn8)), ((locals.var_t9_dn9 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn9)), ((locals.var_t9_dn10 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn10)), ((locals.var_t9_dn13 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign101450_e153611;
        locals.var_t11_dn0 = assign101450_e153611_d_n0;
        locals.var_t11_dn2 = assign101450_e153611_d_n2;
        locals.var_t11_dn4 = assign101450_e153611_d_n4;
        locals.var_t11_dn5 = assign101450_e153611_d_n5;
        locals.var_t11_dn6 = assign101450_e153611_d_n6;
        locals.var_t11_dn7 = assign101450_e153611_d_n7;
        locals.var_t11_dn8 = assign101450_e153611_d_n8;
        locals.var_t11_dn9 = assign101450_e153611_d_n9;
        locals.var_t11_dn10 = assign101450_e153611_d_n10;
        locals.var_t11_dn13 = assign101450_e153611_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign101460_e153617, assign101460_e153617_d_n0, assign101460_e153617_d_n2, assign101460_e153617_d_n4, assign101460_e153617_d_n5, assign101460_e153617_d_n6, assign101460_e153617_d_n7, assign101460_e153617_d_n8, assign101460_e153617_d_n9, assign101460_e153617_d_n10, assign101460_e153617_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101460_e153615: f64 = (locals.var_muun * locals.var_t11);
        (assign101460_e153615, ((locals.var_muun_dn0 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn0)), ((locals.var_muun_dn2 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn2)), ((locals.var_muun_dn4 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn4)), ((locals.var_muun_dn5 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn5)), ((locals.var_muun_dn6 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn6)), ((locals.var_muun_dn7 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn7)), ((locals.var_muun_dn8 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn8)), ((locals.var_muun_dn9 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn9)), ((locals.var_muun_dn10 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn10)), ((locals.var_muun_dn13 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn13)),)
    } else {
        (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn4, locals.var_mud_hoso_dn5, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn8, locals.var_mud_hoso_dn9, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn13,)
    }
};
        locals.var_mud_hoso = assign101460_e153617;
        locals.var_mud_hoso_dn0 = assign101460_e153617_d_n0;
        locals.var_mud_hoso_dn2 = assign101460_e153617_d_n2;
        locals.var_mud_hoso_dn4 = assign101460_e153617_d_n4;
        locals.var_mud_hoso_dn5 = assign101460_e153617_d_n5;
        locals.var_mud_hoso_dn6 = assign101460_e153617_d_n6;
        locals.var_mud_hoso_dn7 = assign101460_e153617_d_n7;
        locals.var_mud_hoso_dn8 = assign101460_e153617_d_n8;
        locals.var_mud_hoso_dn9 = assign101460_e153617_d_n9;
        locals.var_mud_hoso_dn10 = assign101460_e153617_d_n10;
        locals.var_mud_hoso_dn13 = assign101460_e153617_d_n13;
        locals.var_mud_hoso_rv = 0.0;

        let (assign101470_e153625, assign101470_e153625_d_n0, assign101470_e153625_d_n2, assign101470_e153625_d_n4, assign101470_e153625_d_n5, assign101470_e153625_d_n6, assign101470_e153625_d_n7, assign101470_e153625_d_n8, assign101470_e153625_d_n9, assign101470_e153625_d_n10, assign101470_e153625_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101470_e153621: f64 = (locals.var_mu + locals.var_mud_hoso);
        let assign101470_e153623: f64 = (assign101470_e153621 / 2.0);
        (assign101470_e153623, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn4 + locals.var_mud_hoso_dn4) / 2.0), ((locals.var_mu_dn5 + locals.var_mud_hoso_dn5) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn8 + locals.var_mud_hoso_dn8) / 2.0), ((locals.var_mu_dn9 + locals.var_mud_hoso_dn9) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn13 + locals.var_mud_hoso_dn13) / 2.0),)
    } else {
        (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn4, locals.var_mu_ave_dn5, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn8, locals.var_mu_ave_dn9, locals.var_mu_ave_dn10, locals.var_mu_ave_dn13,)
    }
};
        locals.var_mu_ave = assign101470_e153625;
        locals.var_mu_ave_dn0 = assign101470_e153625_d_n0;
        locals.var_mu_ave_dn2 = assign101470_e153625_d_n2;
        locals.var_mu_ave_dn4 = assign101470_e153625_d_n4;
        locals.var_mu_ave_dn5 = assign101470_e153625_d_n5;
        locals.var_mu_ave_dn6 = assign101470_e153625_d_n6;
        locals.var_mu_ave_dn7 = assign101470_e153625_d_n7;
        locals.var_mu_ave_dn8 = assign101470_e153625_d_n8;
        locals.var_mu_ave_dn9 = assign101470_e153625_d_n9;
        locals.var_mu_ave_dn10 = assign101470_e153625_d_n10;
        locals.var_mu_ave_dn13 = assign101470_e153625_d_n13;
        locals.var_mu_ave_rv = 0.0;

        let (assign101480_e153631, assign101480_e153631_d_n0, assign101480_e153631_d_n2, assign101480_e153631_d_n4, assign101480_e153631_d_n5, assign101480_e153631_d_n6, assign101480_e153631_d_n7, assign101480_e153631_d_n8, assign101480_e153631_d_n9, assign101480_e153631_d_n10, assign101480_e153631_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101480_e153629: f64 = (locals.var_alpha * locals.var_alpha);
        (assign101480_e153629, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn4 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn4)), ((locals.var_alpha_dn5 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn5)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn8 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn8)), ((locals.var_alpha_dn9 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn9)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn13 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign101480_e153631;
        locals.var_t0_dn0 = assign101480_e153631_d_n0;
        locals.var_t0_dn2 = assign101480_e153631_d_n2;
        locals.var_t0_dn4 = assign101480_e153631_d_n4;
        locals.var_t0_dn5 = assign101480_e153631_d_n5;
        locals.var_t0_dn6 = assign101480_e153631_d_n6;
        locals.var_t0_dn7 = assign101480_e153631_d_n7;
        locals.var_t0_dn8 = assign101480_e153631_d_n8;
        locals.var_t0_dn9 = assign101480_e153631_d_n9;
        locals.var_t0_dn10 = assign101480_e153631_d_n10;
        locals.var_t0_dn13 = assign101480_e153631_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign101490_e153693, assign101490_e153693_d_n0, assign101490_e153693_d_n2, assign101490_e153693_d_n4, assign101490_e153693_d_n5, assign101490_e153693_d_n6, assign101490_e153693_d_n7, assign101490_e153693_d_n8, assign101490_e153693_d_n9, assign101490_e153693_d_n10, assign101490_e153693_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101490_e153635: f64 = (locals.var_weff_nf * locals.var_cox);
        let assign101490_e153637: f64 = (assign101490_e153635 * locals.var_vgvt);
        let assign101490_e153639: f64 = (assign101490_e153637 * locals.var_mu);
        let assign101490_e153643: f64 = (3.0 * locals.var_alpha);
        let assign101490_e153644: f64 = (1.0 + assign101490_e153643);
        let assign101490_e153647: f64 = (6.0 * locals.var_t0);
        let assign101490_e153648: f64 = (assign101490_e153644 + assign101490_e153647);
        let assign101490_e153650: f64 = (assign101490_e153648 * locals.var_mud_hoso);
        let assign101490_e153652: f64 = (assign101490_e153650 * locals.var_mud_hoso);
        let assign101490_e153656: f64 = (4.0 * locals.var_alpha);
        let assign101490_e153657: f64 = (3.0 + assign101490_e153656);
        let assign101490_e153660: f64 = (3.0 * locals.var_t0);
        let assign101490_e153661: f64 = (assign101490_e153657 + assign101490_e153660);
        let assign101490_e153663: f64 = (assign101490_e153661 * locals.var_mud_hoso);
        let assign101490_e153665: f64 = (assign101490_e153663 * locals.var_mu);
        let assign101490_e153666: f64 = (assign101490_e153652 + assign101490_e153665);
        let assign101490_e153670: f64 = (3.0 * locals.var_alpha);
        let assign101490_e153671: f64 = (6.0 + assign101490_e153670);
        let assign101490_e153673: f64 = (assign101490_e153671 + locals.var_t0);
        let assign101490_e153675: f64 = (assign101490_e153673 * locals.var_mu);
        let assign101490_e153677: f64 = (assign101490_e153675 * locals.var_mu);
        let assign101490_e153678: f64 = (assign101490_e153666 + assign101490_e153677);
        let assign101490_e153679: f64 = (assign101490_e153639 * assign101490_e153678);
        let assign101490_e153682: f64 = (15.0 * locals.var_lch);
        let assign101490_e153685: f64 = (1.0 + locals.var_alpha);
        let assign101490_e153686: f64 = (assign101490_e153682 * assign101490_e153685);
        let assign101490_e153688: f64 = (assign101490_e153686 * locals.var_mu_ave);
        let assign101490_e153690: f64 = (assign101490_e153688 * locals.var_mu_ave);
        let assign101490_e153691: f64 = (assign101490_e153679 / assign101490_e153690);
        (assign101490_e153691, ((((((((((locals.var_weff_nf * locals.var_cox_dn0) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn0)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0_dn0) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn0)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn0))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn0) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn0)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn2) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn2)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0_dn2) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn2)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn2))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn2) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn2)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn4) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn4)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn4)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn4) + (6.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn4)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn4)) + ((((((4.0 * locals.var_alpha_dn4) + (3.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn4)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn4))) + ((((((3.0 * locals.var_alpha_dn4) + locals.var_t0_dn4) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn4)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn4))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn4) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn4)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn4)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn4)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn5) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn5)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn5)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn5) + (6.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn5)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn5)) + ((((((4.0 * locals.var_alpha_dn5) + (3.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn5)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn5))) + ((((((3.0 * locals.var_alpha_dn5) + locals.var_t0_dn5) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn5)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn5))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn5) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn5)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn5)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn5)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn6) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn6)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0_dn6) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn6)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn6))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn6) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn6)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn7) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn7)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0_dn7) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn7)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn7))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn7) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn7)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn8) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn8)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn8)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn8) + (6.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn8)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn8)) + ((((((4.0 * locals.var_alpha_dn8) + (3.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn8)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn8))) + ((((((3.0 * locals.var_alpha_dn8) + locals.var_t0_dn8) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn8)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn8))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn8) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn8)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn8)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn8)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn9) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn9)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn9)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn9) + (6.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn9)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn9)) + ((((((4.0 * locals.var_alpha_dn9) + (3.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn9)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn9))) + ((((((3.0 * locals.var_alpha_dn9) + locals.var_t0_dn9) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn9)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn9))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn9) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn9)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn9)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn9)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn10) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn10)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0_dn10) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn10)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn10))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn10) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn10)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn13) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn13)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn13)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn13) + (6.0 * locals.var_t0_dn13)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn13)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn13)) + ((((((4.0 * locals.var_alpha_dn13) + (3.0 * locals.var_t0_dn13)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn13)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn13))) + ((((((3.0 * locals.var_alpha_dn13) + locals.var_t0_dn13) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn13)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn13))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn13) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn13)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn13)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn13)))) / (assign101490_e153690 * assign101490_e153690)),)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn13,)
    }
};
        locals.var_nthrml = assign101490_e153693;
        locals.var_nthrml_dn0 = assign101490_e153693_d_n0;
        locals.var_nthrml_dn2 = assign101490_e153693_d_n2;
        locals.var_nthrml_dn4 = assign101490_e153693_d_n4;
        locals.var_nthrml_dn5 = assign101490_e153693_d_n5;
        locals.var_nthrml_dn6 = assign101490_e153693_d_n6;
        locals.var_nthrml_dn7 = assign101490_e153693_d_n7;
        locals.var_nthrml_dn8 = assign101490_e153693_d_n8;
        locals.var_nthrml_dn9 = assign101490_e153693_d_n9;
        locals.var_nthrml_dn10 = assign101490_e153693_d_n10;
        locals.var_nthrml_dn13 = assign101490_e153693_d_n13;
        locals.var_nthrml_rv = 0.0;

        let (assign101500_e153698, assign101500_e153698_d_n0, assign101500_e153698_d_n2, assign101500_e153698_d_n4, assign101500_e153698_d_n5, assign101500_e153698_d_n6, assign101500_e153698_d_n7, assign101500_e153698_d_n8, assign101500_e153698_d_n9, assign101500_e153698_d_n10, assign101500_e153698_d_n13,) = {
    if (locals.var_guard2325 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn13,)
    }
};
        locals.var_nthrml = assign101500_e153698;
        locals.var_nthrml_dn0 = assign101500_e153698_d_n0;
        locals.var_nthrml_dn2 = assign101500_e153698_d_n2;
        locals.var_nthrml_dn4 = assign101500_e153698_d_n4;
        locals.var_nthrml_dn5 = assign101500_e153698_d_n5;
        locals.var_nthrml_dn6 = assign101500_e153698_d_n6;
        locals.var_nthrml_dn7 = assign101500_e153698_d_n7;
        locals.var_nthrml_dn8 = assign101500_e153698_d_n8;
        locals.var_nthrml_dn9 = assign101500_e153698_d_n9;
        locals.var_nthrml_dn10 = assign101500_e153698_d_n10;
        locals.var_nthrml_dn13 = assign101500_e153698_d_n13;
        locals.var_nthrml_rv = 0.0;

        let assign101510_e153716: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2328 = assign101510_e153716;
        locals.var_guard2328_rv = 0.0;

        let (assign101520_e153721, assign101520_e153721_d_n0, assign101520_e153721_d_n2, assign101520_e153721_d_n4, assign101520_e153721_d_n5, assign101520_e153721_d_n6, assign101520_e153721_d_n7, assign101520_e153721_d_n8, assign101520_e153721_d_n9, assign101520_e153721_d_n10, assign101520_e153721_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101520_e153719: f64 = (locals.var_kusail).sqrt();
        (assign101520_e153719, (locals.var_kusail_dn0 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn2 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn4 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn5 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn6 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn7 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn8 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn9 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn10 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn13 / (2.0 * assign101520_e153719)),)
    } else {
        (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn4, locals.var_sqrtkusail_dn5, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn8, locals.var_sqrtkusail_dn9, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn13,)
    }
};
        locals.var_sqrtkusail = assign101520_e153721;
        locals.var_sqrtkusail_dn0 = assign101520_e153721_d_n0;
        locals.var_sqrtkusail_dn2 = assign101520_e153721_d_n2;
        locals.var_sqrtkusail_dn4 = assign101520_e153721_d_n4;
        locals.var_sqrtkusail_dn5 = assign101520_e153721_d_n5;
        locals.var_sqrtkusail_dn6 = assign101520_e153721_d_n6;
        locals.var_sqrtkusail_dn7 = assign101520_e153721_d_n7;
        locals.var_sqrtkusail_dn8 = assign101520_e153721_d_n8;
        locals.var_sqrtkusail_dn9 = assign101520_e153721_d_n9;
        locals.var_sqrtkusail_dn10 = assign101520_e153721_d_n10;
        locals.var_sqrtkusail_dn13 = assign101520_e153721_d_n13;
        locals.var_sqrtkusail_rv = 0.0;

        let (assign101530_e153727, assign101530_e153727_d_n0, assign101530_e153727_d_n2, assign101530_e153727_d_n4, assign101530_e153727_d_n5, assign101530_e153727_d_n6, assign101530_e153727_d_n7, assign101530_e153727_d_n8, assign101530_e153727_d_n9, assign101530_e153727_d_n10, assign101530_e153727_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101530_e153725: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        (assign101530_e153725, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4), (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8), (locals.var_vgvt_dn9 + locals.var_sqrtkusail_dn9), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn13 + locals.var_sqrtkusail_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign101530_e153727;
        locals.var_t2_dn0 = assign101530_e153727_d_n0;
        locals.var_t2_dn2 = assign101530_e153727_d_n2;
        locals.var_t2_dn4 = assign101530_e153727_d_n4;
        locals.var_t2_dn5 = assign101530_e153727_d_n5;
        locals.var_t2_dn6 = assign101530_e153727_d_n6;
        locals.var_t2_dn7 = assign101530_e153727_d_n7;
        locals.var_t2_dn8 = assign101530_e153727_d_n8;
        locals.var_t2_dn9 = assign101530_e153727_d_n9;
        locals.var_t2_dn10 = assign101530_e153727_d_n10;
        locals.var_t2_dn13 = assign101530_e153727_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign101540_e153733, assign101540_e153733_d_n0, assign101540_e153733_d_n2, assign101540_e153733_d_n4, assign101540_e153733_d_n5, assign101540_e153733_d_n6, assign101540_e153733_d_n7, assign101540_e153733_d_n8, assign101540_e153733_d_n9, assign101540_e153733_d_n10, assign101540_e153733_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101540_e153731: f64 = (locals.var_kusai00 * locals.var_kusai00);
        (assign101540_e153731, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)), ((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)), ((locals.var_kusai00_dn9 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn9)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn13 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign101540_e153733;
        locals.var_t3_dn0 = assign101540_e153733_d_n0;
        locals.var_t3_dn2 = assign101540_e153733_d_n2;
        locals.var_t3_dn4 = assign101540_e153733_d_n4;
        locals.var_t3_dn5 = assign101540_e153733_d_n5;
        locals.var_t3_dn6 = assign101540_e153733_d_n6;
        locals.var_t3_dn7 = assign101540_e153733_d_n7;
        locals.var_t3_dn8 = assign101540_e153733_d_n8;
        locals.var_t3_dn9 = assign101540_e153733_d_n9;
        locals.var_t3_dn10 = assign101540_e153733_d_n10;
        locals.var_t3_dn13 = assign101540_e153733_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign101550_e153739, assign101550_e153739_d_n0, assign101550_e153739_d_n2, assign101550_e153739_d_n4, assign101550_e153739_d_n5, assign101550_e153739_d_n6, assign101550_e153739_d_n7, assign101550_e153739_d_n8, assign101550_e153739_d_n9, assign101550_e153739_d_n10, assign101550_e153739_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101550_e153737: f64 = (locals.var_kusail * locals.var_kusail);
        (assign101550_e153737, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)), ((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)), ((locals.var_kusail_dn9 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn9)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn13 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign101550_e153739;
        locals.var_t4_dn0 = assign101550_e153739_d_n0;
        locals.var_t4_dn2 = assign101550_e153739_d_n2;
        locals.var_t4_dn4 = assign101550_e153739_d_n4;
        locals.var_t4_dn5 = assign101550_e153739_d_n5;
        locals.var_t4_dn6 = assign101550_e153739_d_n6;
        locals.var_t4_dn7 = assign101550_e153739_d_n7;
        locals.var_t4_dn8 = assign101550_e153739_d_n8;
        locals.var_t4_dn9 = assign101550_e153739_d_n9;
        locals.var_t4_dn10 = assign101550_e153739_d_n10;
        locals.var_t4_dn13 = assign101550_e153739_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign101560_e153747, assign101560_e153747_d_n0, assign101560_e153747_d_n2, assign101560_e153747_d_n4, assign101560_e153747_d_n5, assign101560_e153747_d_n6, assign101560_e153747_d_n7, assign101560_e153747_d_n8, assign101560_e153747_d_n9, assign101560_e153747_d_n10, assign101560_e153747_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101560_e153743: f64 = (42.0 * locals.var_kusai00);
        let assign101560_e153745: f64 = (assign101560_e153743 * locals.var_kusail);
        (assign101560_e153745, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn4) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn4)), (((42.0 * locals.var_kusai00_dn5) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn5)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn8) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn8)), (((42.0 * locals.var_kusai00_dn9) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn9)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn13) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign101560_e153747;
        locals.var_t5_dn0 = assign101560_e153747_d_n0;
        locals.var_t5_dn2 = assign101560_e153747_d_n2;
        locals.var_t5_dn4 = assign101560_e153747_d_n4;
        locals.var_t5_dn5 = assign101560_e153747_d_n5;
        locals.var_t5_dn6 = assign101560_e153747_d_n6;
        locals.var_t5_dn7 = assign101560_e153747_d_n7;
        locals.var_t5_dn8 = assign101560_e153747_d_n8;
        locals.var_t5_dn9 = assign101560_e153747_d_n9;
        locals.var_t5_dn10 = assign101560_e153747_d_n10;
        locals.var_t5_dn13 = assign101560_e153747_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign101570_e153757, assign101570_e153757_d_n0, assign101570_e153757_d_n2, assign101570_e153757_d_n4, assign101570_e153757_d_n5, assign101570_e153757_d_n6, assign101570_e153757_d_n7, assign101570_e153757_d_n8, assign101570_e153757_d_n9, assign101570_e153757_d_n10, assign101570_e153757_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101570_e153753: f64 = (locals.var_t3 + locals.var_t4);
        let assign101570_e153754: f64 = (4.0 * assign101570_e153753);
        let assign101570_e153755: f64 = (locals.var_t5 + assign101570_e153754);
        (assign101570_e153755, (locals.var_t5_dn0 + (4.0 * (locals.var_t3_dn0 + locals.var_t4_dn0))), (locals.var_t5_dn2 + (4.0 * (locals.var_t3_dn2 + locals.var_t4_dn2))), (locals.var_t5_dn4 + (4.0 * (locals.var_t3_dn4 + locals.var_t4_dn4))), (locals.var_t5_dn5 + (4.0 * (locals.var_t3_dn5 + locals.var_t4_dn5))), (locals.var_t5_dn6 + (4.0 * (locals.var_t3_dn6 + locals.var_t4_dn6))), (locals.var_t5_dn7 + (4.0 * (locals.var_t3_dn7 + locals.var_t4_dn7))), (locals.var_t5_dn8 + (4.0 * (locals.var_t3_dn8 + locals.var_t4_dn8))), (locals.var_t5_dn9 + (4.0 * (locals.var_t3_dn9 + locals.var_t4_dn9))), (locals.var_t5_dn10 + (4.0 * (locals.var_t3_dn10 + locals.var_t4_dn10))), (locals.var_t5_dn13 + (4.0 * (locals.var_t3_dn13 + locals.var_t4_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign101570_e153757;
        locals.var_t5_dn0 = assign101570_e153757_d_n0;
        locals.var_t5_dn2 = assign101570_e153757_d_n2;
        locals.var_t5_dn4 = assign101570_e153757_d_n4;
        locals.var_t5_dn5 = assign101570_e153757_d_n5;
        locals.var_t5_dn6 = assign101570_e153757_d_n6;
        locals.var_t5_dn7 = assign101570_e153757_d_n7;
        locals.var_t5_dn8 = assign101570_e153757_d_n8;
        locals.var_t5_dn9 = assign101570_e153757_d_n9;
        locals.var_t5_dn10 = assign101570_e153757_d_n10;
        locals.var_t5_dn13 = assign101570_e153757_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign101580_e153771, assign101580_e153771_d_n0, assign101580_e153771_d_n2, assign101580_e153771_d_n4, assign101580_e153771_d_n5, assign101580_e153771_d_n6, assign101580_e153771_d_n7, assign101580_e153771_d_n8, assign101580_e153771_d_n9, assign101580_e153771_d_n10, assign101580_e153771_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101580_e153762: f64 = (20.0 * locals.var_sqrtkusail);
        let assign101580_e153764: f64 = (assign101580_e153762 * locals.var_vgvt);
        let assign101580_e153767: f64 = (locals.var_kusai00 + locals.var_kusail);
        let assign101580_e153768: f64 = (assign101580_e153764 * assign101580_e153767);
        let assign101580_e153769: f64 = (locals.var_t5 + assign101580_e153768);
        (assign101580_e153769, (locals.var_t5_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn0)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn2)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5_dn4 + (((((20.0 * locals.var_sqrtkusail_dn4) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn4)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn4 + locals.var_kusail_dn4)))), (locals.var_t5_dn5 + (((((20.0 * locals.var_sqrtkusail_dn5) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn5)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn5 + locals.var_kusail_dn5)))), (locals.var_t5_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn6)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn7)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5_dn8 + (((((20.0 * locals.var_sqrtkusail_dn8) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn8)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn8 + locals.var_kusail_dn8)))), (locals.var_t5_dn9 + (((((20.0 * locals.var_sqrtkusail_dn9) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn9)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn9 + locals.var_kusail_dn9)))), (locals.var_t5_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn10)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5_dn13 + (((((20.0 * locals.var_sqrtkusail_dn13) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn13)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn13 + locals.var_kusail_dn13)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign101580_e153771;
        locals.var_t5_dn0 = assign101580_e153771_d_n0;
        locals.var_t5_dn2 = assign101580_e153771_d_n2;
        locals.var_t5_dn4 = assign101580_e153771_d_n4;
        locals.var_t5_dn5 = assign101580_e153771_d_n5;
        locals.var_t5_dn6 = assign101580_e153771_d_n6;
        locals.var_t5_dn7 = assign101580_e153771_d_n7;
        locals.var_t5_dn8 = assign101580_e153771_d_n8;
        locals.var_t5_dn9 = assign101580_e153771_d_n9;
        locals.var_t5_dn10 = assign101580_e153771_d_n10;
        locals.var_t5_dn13 = assign101580_e153771_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign101590_e153777, assign101590_e153777_d_n0, assign101590_e153777_d_n2, assign101590_e153777_d_n4, assign101590_e153777_d_n5, assign101590_e153777_d_n6, assign101590_e153777_d_n7, assign101590_e153777_d_n8, assign101590_e153777_d_n9, assign101590_e153777_d_n10, assign101590_e153777_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101590_e153775: f64 = (locals.var_t2 * locals.var_t2);
        (assign101590_e153775, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign101590_e153777;
        locals.var_t10_dn0 = assign101590_e153777_d_n0;
        locals.var_t10_dn2 = assign101590_e153777_d_n2;
        locals.var_t10_dn4 = assign101590_e153777_d_n4;
        locals.var_t10_dn5 = assign101590_e153777_d_n5;
        locals.var_t10_dn6 = assign101590_e153777_d_n6;
        locals.var_t10_dn7 = assign101590_e153777_d_n7;
        locals.var_t10_dn8 = assign101590_e153777_d_n8;
        locals.var_t10_dn9 = assign101590_e153777_d_n9;
        locals.var_t10_dn10 = assign101590_e153777_d_n10;
        locals.var_t10_dn13 = assign101590_e153777_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign101600_e153783, assign101600_e153783_d_n0, assign101600_e153783_d_n2, assign101600_e153783_d_n4, assign101600_e153783_d_n5, assign101600_e153783_d_n6, assign101600_e153783_d_n7, assign101600_e153783_d_n8, assign101600_e153783_d_n9, assign101600_e153783_d_n10, assign101600_e153783_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101600_e153781: f64 = (locals.var_t10 * locals.var_t10);
        (assign101600_e153781, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn13 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign101600_e153783;
        locals.var_t10_dn0 = assign101600_e153783_d_n0;
        locals.var_t10_dn2 = assign101600_e153783_d_n2;
        locals.var_t10_dn4 = assign101600_e153783_d_n4;
        locals.var_t10_dn5 = assign101600_e153783_d_n5;
        locals.var_t10_dn6 = assign101600_e153783_d_n6;
        locals.var_t10_dn7 = assign101600_e153783_d_n7;
        locals.var_t10_dn8 = assign101600_e153783_d_n8;
        locals.var_t10_dn9 = assign101600_e153783_d_n9;
        locals.var_t10_dn10 = assign101600_e153783_d_n10;
        locals.var_t10_dn13 = assign101600_e153783_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign101610_e153791, assign101610_e153791_d_n0, assign101610_e153791_d_n2, assign101610_e153791_d_n4, assign101610_e153791_d_n5, assign101610_e153791_d_n6, assign101610_e153791_d_n7, assign101610_e153791_d_n8, assign101610_e153791_d_n9, assign101610_e153791_d_n10, assign101610_e153791_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101610_e153788: f64 = (locals.var_t10 * locals.var_t2);
        let assign101610_e153789: f64 = (locals.var_t5 / assign101610_e153788);
        (assign101610_e153789, (((locals.var_t5_dn0 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn0 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn0)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn2 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn2 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn2)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn4 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn4 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn4)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn5 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn5 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn5)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn6 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn6 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn6)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn7 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn7 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn7)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn8 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn8 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn8)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn9 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn9 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn9)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn10 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn10 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn10)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn13 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn13 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn13)))) / (assign101610_e153788 * assign101610_e153788)),)
    } else {
        (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn4, locals.var_kusai_ig_dn5, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn8, locals.var_kusai_ig_dn9, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn13,)
    }
};
        locals.var_kusai_ig = assign101610_e153791;
        locals.var_kusai_ig_dn0 = assign101610_e153791_d_n0;
        locals.var_kusai_ig_dn2 = assign101610_e153791_d_n2;
        locals.var_kusai_ig_dn4 = assign101610_e153791_d_n4;
        locals.var_kusai_ig_dn5 = assign101610_e153791_d_n5;
        locals.var_kusai_ig_dn6 = assign101610_e153791_d_n6;
        locals.var_kusai_ig_dn7 = assign101610_e153791_d_n7;
        locals.var_kusai_ig_dn8 = assign101610_e153791_d_n8;
        locals.var_kusai_ig_dn9 = assign101610_e153791_d_n9;
        locals.var_kusai_ig_dn10 = assign101610_e153791_d_n10;
        locals.var_kusai_ig_dn13 = assign101610_e153791_d_n13;
        locals.var_kusai_ig_rv = 0.0;

        let (assign101620_e153801, assign101620_e153801_d_n0, assign101620_e153801_d_n2, assign101620_e153801_d_n4, assign101620_e153801_d_n5, assign101620_e153801_d_n6, assign101620_e153801_d_n7, assign101620_e153801_d_n8, assign101620_e153801_d_n9, assign101620_e153801_d_n10, assign101620_e153801_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101620_e153795: f64 = (locals.var_weff_nf / locals.var_lch);
        let assign101620_e153797: f64 = (assign101620_e153795 * locals.var_mu);
        let assign101620_e153799: f64 = (assign101620_e153797 * locals.var_cox);
        (assign101620_e153799, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn0)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn2)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn4) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn4)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn4)), (((((-((locals.var_weff_nf * locals.var_lch_dn5) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn5)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn5)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn6)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn7)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn8) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn8)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn8)), (((((-((locals.var_weff_nf * locals.var_lch_dn9) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn9)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn9)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn10)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn13) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn13)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn13)),)
    } else {
        (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn4, locals.var_gds0_ign_dn5, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn8, locals.var_gds0_ign_dn9, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn13,)
    }
};
        locals.var_gds0_ign = assign101620_e153801;
        locals.var_gds0_ign_dn0 = assign101620_e153801_d_n0;
        locals.var_gds0_ign_dn2 = assign101620_e153801_d_n2;
        locals.var_gds0_ign_dn4 = assign101620_e153801_d_n4;
        locals.var_gds0_ign_dn5 = assign101620_e153801_d_n5;
        locals.var_gds0_ign_dn6 = assign101620_e153801_d_n6;
        locals.var_gds0_ign_dn7 = assign101620_e153801_d_n7;
        locals.var_gds0_ign_dn8 = assign101620_e153801_d_n8;
        locals.var_gds0_ign_dn9 = assign101620_e153801_d_n9;
        locals.var_gds0_ign_dn10 = assign101620_e153801_d_n10;
        locals.var_gds0_ign_dn13 = assign101620_e153801_d_n13;
        locals.var_gds0_ign_rv = 0.0;

        let (assign101650_e153825, assign101650_e153825_d_n0, assign101650_e153825_d_n2, assign101650_e153825_d_n4, assign101650_e153825_d_n5, assign101650_e153825_d_n6, assign101650_e153825_d_n7, assign101650_e153825_d_n8, assign101650_e153825_d_n9, assign101650_e153825_d_n10, assign101650_e153825_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101650_e153818: f64 = (4.0 * locals.var_vgvt);
        let assign101650_e153820: f64 = (assign101650_e153818 * locals.var_sqrtkusail);
        let assign101650_e153821: f64 = (locals.var_kusai00 + assign101650_e153820);
        let assign101650_e153823: f64 = (assign101650_e153821 + locals.var_kusail);
        (assign101650_e153823, ((locals.var_kusai00_dn0 + (((4.0 * locals.var_vgvt_dn0) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0), ((locals.var_kusai00_dn2 + (((4.0 * locals.var_vgvt_dn2) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2), ((locals.var_kusai00_dn4 + (((4.0 * locals.var_vgvt_dn4) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4), ((locals.var_kusai00_dn5 + (((4.0 * locals.var_vgvt_dn5) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5), ((locals.var_kusai00_dn6 + (((4.0 * locals.var_vgvt_dn6) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6), ((locals.var_kusai00_dn7 + (((4.0 * locals.var_vgvt_dn7) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7), ((locals.var_kusai00_dn8 + (((4.0 * locals.var_vgvt_dn8) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8), ((locals.var_kusai00_dn9 + (((4.0 * locals.var_vgvt_dn9) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn9))) + locals.var_kusail_dn9), ((locals.var_kusai00_dn10 + (((4.0 * locals.var_vgvt_dn10) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10), ((locals.var_kusai00_dn13 + (((4.0 * locals.var_vgvt_dn13) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn13))) + locals.var_kusail_dn13),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign101650_e153825;
        locals.var_t7_dn0 = assign101650_e153825_d_n0;
        locals.var_t7_dn2 = assign101650_e153825_d_n2;
        locals.var_t7_dn4 = assign101650_e153825_d_n4;
        locals.var_t7_dn5 = assign101650_e153825_d_n5;
        locals.var_t7_dn6 = assign101650_e153825_d_n6;
        locals.var_t7_dn7 = assign101650_e153825_d_n7;
        locals.var_t7_dn8 = assign101650_e153825_d_n8;
        locals.var_t7_dn9 = assign101650_e153825_d_n9;
        locals.var_t7_dn10 = assign101650_e153825_d_n10;
        locals.var_t7_dn13 = assign101650_e153825_d_n13;
        locals.var_t7_rv = 0.0;

        let assign101670_e153849: f64 = (locals.var_mfactor * locals.var_ids);
        locals.var_idse = assign101670_e153849;
        locals.var_idse_dn0 = (locals.var_mfactor * locals.var_ids_dn0);
        locals.var_idse_dn2 = (locals.var_mfactor * locals.var_ids_dn2);
        locals.var_idse_dn4 = (locals.var_mfactor * locals.var_ids_dn4);
        locals.var_idse_dn5 = (locals.var_mfactor * locals.var_ids_dn5);
        locals.var_idse_dn6 = (locals.var_mfactor * locals.var_ids_dn6);
        locals.var_idse_dn7 = (locals.var_mfactor * locals.var_ids_dn7);
        locals.var_idse_dn8 = (locals.var_mfactor * locals.var_ids_dn8);
        locals.var_idse_dn9 = (locals.var_mfactor * locals.var_ids_dn9);
        locals.var_idse_dn10 = (locals.var_mfactor * locals.var_ids_dn10);
        locals.var_idse_dn13 = (locals.var_mfactor * locals.var_ids_dn13);
        locals.var_idse_rv = 0.0;

        let assign101710_e153861: f64 = (locals.var_mfactor * locals.var_idsibpc);
        locals.var_idsibpce = assign101710_e153861;
        locals.var_idsibpce_dn0 = (locals.var_mfactor * locals.var_idsibpc_dn0);
        locals.var_idsibpce_dn2 = (locals.var_mfactor * locals.var_idsibpc_dn2);
        locals.var_idsibpce_dn4 = (locals.var_mfactor * locals.var_idsibpc_dn4);
        locals.var_idsibpce_dn5 = (locals.var_mfactor * locals.var_idsibpc_dn5);
        locals.var_idsibpce_dn6 = (locals.var_mfactor * locals.var_idsibpc_dn6);
        locals.var_idsibpce_dn7 = (locals.var_mfactor * locals.var_idsibpc_dn7);
        locals.var_idsibpce_dn8 = (locals.var_mfactor * locals.var_idsibpc_dn8);
        locals.var_idsibpce_dn9 = (locals.var_mfactor * locals.var_idsibpc_dn9);
        locals.var_idsibpce_dn10 = (locals.var_mfactor * locals.var_idsibpc_dn10);
        locals.var_idsibpce_dn13 = (locals.var_mfactor * locals.var_idsibpc_dn13);
        locals.var_idsibpce_rv = 0.0;

        locals.var_qgexte = 0.0;
        locals.var_qgexte_dn0 = 0.0;
        locals.var_qgexte_dn2 = 0.0;
        locals.var_qgexte_dn4 = 0.0;
        locals.var_qgexte_dn5 = 0.0;
        locals.var_qgexte_dn6 = 0.0;
        locals.var_qgexte_dn7 = 0.0;
        locals.var_qgexte_dn8 = 0.0;
        locals.var_qgexte_dn9 = 0.0;
        locals.var_qgexte_dn10 = 0.0;
        locals.var_qgexte_dn13 = 0.0;
        locals.var_qgexte_rv = 0.0;

        locals.var_qdexte = 0.0;
        locals.var_qdexte_dn0 = 0.0;
        locals.var_qdexte_dn2 = 0.0;
        locals.var_qdexte_dn4 = 0.0;
        locals.var_qdexte_dn5 = 0.0;
        locals.var_qdexte_dn6 = 0.0;
        locals.var_qdexte_dn7 = 0.0;
        locals.var_qdexte_dn8 = 0.0;
        locals.var_qdexte_dn9 = 0.0;
        locals.var_qdexte_dn10 = 0.0;
        locals.var_qdexte_dn13 = 0.0;
        locals.var_qdexte_rv = 0.0;

        locals.var_qsexte = 0.0;
        locals.var_qsexte_dn0 = 0.0;
        locals.var_qsexte_dn2 = 0.0;
        locals.var_qsexte_dn4 = 0.0;
        locals.var_qsexte_dn5 = 0.0;
        locals.var_qsexte_dn6 = 0.0;
        locals.var_qsexte_dn7 = 0.0;
        locals.var_qsexte_dn8 = 0.0;
        locals.var_qsexte_dn9 = 0.0;
        locals.var_qsexte_dn10 = 0.0;
        locals.var_qsexte_dn13 = 0.0;
        locals.var_qsexte_rv = 0.0;

        locals.var_qgov = 0.0;
        locals.var_qgov_dn0 = 0.0;
        locals.var_qgov_dn2 = 0.0;
        locals.var_qgov_dn4 = 0.0;
        locals.var_qgov_dn5 = 0.0;
        locals.var_qgov_dn6 = 0.0;
        locals.var_qgov_dn7 = 0.0;
        locals.var_qgov_dn8 = 0.0;
        locals.var_qgov_dn9 = 0.0;
        locals.var_qgov_dn10 = 0.0;
        locals.var_qgov_dn13 = 0.0;
        locals.var_qgov_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_378(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        locals.var_qdov = 0.0;
        locals.var_qdov_dn0 = 0.0;
        locals.var_qdov_dn2 = 0.0;
        locals.var_qdov_dn4 = 0.0;
        locals.var_qdov_dn5 = 0.0;
        locals.var_qdov_dn6 = 0.0;
        locals.var_qdov_dn7 = 0.0;
        locals.var_qdov_dn8 = 0.0;
        locals.var_qdov_dn9 = 0.0;
        locals.var_qdov_dn10 = 0.0;
        locals.var_qdov_dn13 = 0.0;
        locals.var_qdov_rv = 0.0;

        locals.var_qsov = 0.0;
        locals.var_qsov_dn0 = 0.0;
        locals.var_qsov_dn2 = 0.0;
        locals.var_qsov_dn4 = 0.0;
        locals.var_qsov_dn5 = 0.0;
        locals.var_qsov_dn6 = 0.0;
        locals.var_qsov_dn7 = 0.0;
        locals.var_qsov_dn8 = 0.0;
        locals.var_qsov_dn9 = 0.0;
        locals.var_qsov_dn10 = 0.0;
        locals.var_qsov_dn13 = 0.0;
        locals.var_qsov_rv = 0.0;

        locals.var_qdp = 0.0;
        locals.var_qdp_dn0 = 0.0;
        locals.var_qdp_dn2 = 0.0;
        locals.var_qdp_dn6 = 0.0;
        locals.var_qdp_rv = 0.0;

        locals.var_qsp = 0.0;
        locals.var_qsp_dn2 = 0.0;
        locals.var_qsp_dn6 = 0.0;
        locals.var_qsp_rv = 0.0;

        let assign101810_e153875: f64 = if ((locals.var_flg_nqs != 0.0) || (p.p22 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard2329 = assign101810_e153875;
        locals.var_guard2329_rv = 0.0;

        let (assign101820_e153879, assign101820_e153879_d_n0, assign101820_e153879_d_n2, assign101820_e153879_d_n4, assign101820_e153879_d_n5, assign101820_e153879_d_n6, assign101820_e153879_d_n7, assign101820_e153879_d_n8, assign101820_e153879_d_n9, assign101820_e153879_d_n10, assign101820_e153879_d_n13,) = {
    if (locals.var_guard2329 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn13,)
    }
};
        locals.var_qge = assign101820_e153879;
        locals.var_qge_dn0 = assign101820_e153879_d_n0;
        locals.var_qge_dn2 = assign101820_e153879_d_n2;
        locals.var_qge_dn4 = assign101820_e153879_d_n4;
        locals.var_qge_dn5 = assign101820_e153879_d_n5;
        locals.var_qge_dn6 = assign101820_e153879_d_n6;
        locals.var_qge_dn7 = assign101820_e153879_d_n7;
        locals.var_qge_dn8 = assign101820_e153879_d_n8;
        locals.var_qge_dn9 = assign101820_e153879_d_n9;
        locals.var_qge_dn10 = assign101820_e153879_d_n10;
        locals.var_qge_dn13 = assign101820_e153879_d_n13;
        locals.var_qge_rv = 0.0;

        let (assign101830_e153883, assign101830_e153883_d_n0, assign101830_e153883_d_n2, assign101830_e153883_d_n4, assign101830_e153883_d_n5, assign101830_e153883_d_n6, assign101830_e153883_d_n7, assign101830_e153883_d_n8, assign101830_e153883_d_n9, assign101830_e153883_d_n10, assign101830_e153883_d_n13,) = {
    if (locals.var_guard2329 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn13,)
    }
};
        locals.var_qde = assign101830_e153883;
        locals.var_qde_dn0 = assign101830_e153883_d_n0;
        locals.var_qde_dn2 = assign101830_e153883_d_n2;
        locals.var_qde_dn4 = assign101830_e153883_d_n4;
        locals.var_qde_dn5 = assign101830_e153883_d_n5;
        locals.var_qde_dn6 = assign101830_e153883_d_n6;
        locals.var_qde_dn7 = assign101830_e153883_d_n7;
        locals.var_qde_dn8 = assign101830_e153883_d_n8;
        locals.var_qde_dn9 = assign101830_e153883_d_n9;
        locals.var_qde_dn10 = assign101830_e153883_d_n10;
        locals.var_qde_dn13 = assign101830_e153883_d_n13;
        locals.var_qde_rv = 0.0;

        let (assign101840_e153887, assign101840_e153887_d_n0, assign101840_e153887_d_n2, assign101840_e153887_d_n4, assign101840_e153887_d_n5, assign101840_e153887_d_n6, assign101840_e153887_d_n7, assign101840_e153887_d_n8, assign101840_e153887_d_n9, assign101840_e153887_d_n10, assign101840_e153887_d_n13,) = {
    if (locals.var_guard2329 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn13,)
    }
};
        locals.var_qse = assign101840_e153887;
        locals.var_qse_dn0 = assign101840_e153887_d_n0;
        locals.var_qse_dn2 = assign101840_e153887_d_n2;
        locals.var_qse_dn4 = assign101840_e153887_d_n4;
        locals.var_qse_dn5 = assign101840_e153887_d_n5;
        locals.var_qse_dn6 = assign101840_e153887_d_n6;
        locals.var_qse_dn7 = assign101840_e153887_d_n7;
        locals.var_qse_dn8 = assign101840_e153887_d_n8;
        locals.var_qse_dn9 = assign101840_e153887_d_n9;
        locals.var_qse_dn10 = assign101840_e153887_d_n10;
        locals.var_qse_dn13 = assign101840_e153887_d_n13;
        locals.var_qse_rv = 0.0;

        let (assign101850_e153891, assign101850_e153891_d_n0, assign101850_e153891_d_n2, assign101850_e153891_d_n4, assign101850_e153891_d_n5, assign101850_e153891_d_n6, assign101850_e153891_d_n7, assign101850_e153891_d_n8, assign101850_e153891_d_n9, assign101850_e153891_d_n10, assign101850_e153891_d_n13,) = {
    if (locals.var_guard2329 != 0.0) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn13,)
    }
};
        locals.var_xd = assign101850_e153891;
        locals.var_xd_dn0 = assign101850_e153891_d_n0;
        locals.var_xd_dn2 = assign101850_e153891_d_n2;
        locals.var_xd_dn4 = assign101850_e153891_d_n4;
        locals.var_xd_dn5 = assign101850_e153891_d_n5;
        locals.var_xd_dn6 = assign101850_e153891_d_n6;
        locals.var_xd_dn7 = assign101850_e153891_d_n7;
        locals.var_xd_dn8 = assign101850_e153891_d_n8;
        locals.var_xd_dn9 = assign101850_e153891_d_n9;
        locals.var_xd_dn10 = assign101850_e153891_d_n10;
        locals.var_xd_dn13 = assign101850_e153891_d_n13;
        locals.var_xd_rv = 0.0;

        let (assign101870_e153903, assign101870_e153903_d_n0, assign101870_e153903_d_n2, assign101870_e153903_d_n4, assign101870_e153903_d_n5, assign101870_e153903_d_n6, assign101870_e153903_d_n7, assign101870_e153903_d_n8, assign101870_e153903_d_n9, assign101870_e153903_d_n10, assign101870_e153903_d_n13,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101870_e153901: f64 = (locals.var_mfactor * locals.var_qi);
        (assign101870_e153901, (locals.var_mfactor * locals.var_qi_dn0), (locals.var_mfactor * locals.var_qi_dn2), (locals.var_mfactor * locals.var_qi_dn4), (locals.var_mfactor * locals.var_qi_dn5), (locals.var_mfactor * locals.var_qi_dn6), (locals.var_mfactor * locals.var_qi_dn7), (locals.var_mfactor * locals.var_qi_dn8), (locals.var_mfactor * locals.var_qi_dn9), (locals.var_mfactor * locals.var_qi_dn10), (locals.var_mfactor * locals.var_qi_dn13),)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn4, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, locals.var_qi_dn9, locals.var_qi_dn10, locals.var_qi_dn13,)
    }
};
        locals.var_qi = assign101870_e153903;
        locals.var_qi_dn0 = assign101870_e153903_d_n0;
        locals.var_qi_dn2 = assign101870_e153903_d_n2;
        locals.var_qi_dn4 = assign101870_e153903_d_n4;
        locals.var_qi_dn5 = assign101870_e153903_d_n5;
        locals.var_qi_dn6 = assign101870_e153903_d_n6;
        locals.var_qi_dn7 = assign101870_e153903_d_n7;
        locals.var_qi_dn8 = assign101870_e153903_d_n8;
        locals.var_qi_dn9 = assign101870_e153903_d_n9;
        locals.var_qi_dn10 = assign101870_e153903_d_n10;
        locals.var_qi_dn13 = assign101870_e153903_d_n13;
        locals.var_qi_rv = 0.0;

        let (assign101880_e153913, assign101880_e153913_d_n0, assign101880_e153913_d_n2, assign101880_e153913_d_n4, assign101880_e153913_d_n5, assign101880_e153913_d_n6, assign101880_e153913_d_n7, assign101880_e153913_d_n8, assign101880_e153913_d_n9, assign101880_e153913_d_n10, assign101880_e153913_d_n13,) = {
    if (locals.var_guard2329 == 0.0) {
        let assign101880_e153909: f64 = (locals.var_qb + locals.var_qi);
        let assign101880_e153910: f64 = (-assign101880_e153909);
        let assign101880_e153911: f64 = (locals.var_mfactor * assign101880_e153910);
        (assign101880_e153911, (locals.var_mfactor * (-(locals.var_qb_dn0 + locals.var_qi_dn0))), (locals.var_mfactor * (-(locals.var_qb_dn2 + locals.var_qi_dn2))), (locals.var_mfactor * (-(locals.var_qb_dn4 + locals.var_qi_dn4))), (locals.var_mfactor * (-(locals.var_qb_dn5 + locals.var_qi_dn5))), (locals.var_mfactor * (-(locals.var_qb_dn6 + locals.var_qi_dn6))), (locals.var_mfactor * (-(locals.var_qb_dn7 + locals.var_qi_dn7))), (locals.var_mfactor * (-(locals.var_qb_dn8 + locals.var_qi_dn8))), (locals.var_mfactor * (-(locals.var_qb_dn9 + locals.var_qi_dn9))), (locals.var_mfactor * (-(locals.var_qb_dn10 + locals.var_qi_dn10))), (locals.var_mfactor * (-(locals.var_qb_dn13 + locals.var_qi_dn13))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn13,)
    }
};
        locals.var_qge = assign101880_e153913;
        locals.var_qge_dn0 = assign101880_e153913_d_n0;
        locals.var_qge_dn2 = assign101880_e153913_d_n2;
        locals.var_qge_dn4 = assign101880_e153913_d_n4;
        locals.var_qge_dn5 = assign101880_e153913_d_n5;
        locals.var_qge_dn6 = assign101880_e153913_d_n6;
        locals.var_qge_dn7 = assign101880_e153913_d_n7;
        locals.var_qge_dn8 = assign101880_e153913_d_n8;
        locals.var_qge_dn9 = assign101880_e153913_d_n9;
        locals.var_qge_dn10 = assign101880_e153913_d_n10;
        locals.var_qge_dn13 = assign101880_e153913_d_n13;
        locals.var_qge_rv = 0.0;

        let (assign101890_e153920, assign101890_e153920_d_n0, assign101890_e153920_d_n2, assign101890_e153920_d_n4, assign101890_e153920_d_n5, assign101890_e153920_d_n6, assign101890_e153920_d_n7, assign101890_e153920_d_n8, assign101890_e153920_d_n9, assign101890_e153920_d_n10, assign101890_e153920_d_n13,) = {
    if (locals.var_guard2329 == 0.0) {
        let assign101890_e153918: f64 = (locals.var_mfactor * locals.var_qd);
        (assign101890_e153918, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn4), (locals.var_mfactor * locals.var_qd_dn5), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn8), (locals.var_mfactor * locals.var_qd_dn9), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn13),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn13,)
    }
};
        locals.var_qde = assign101890_e153920;
        locals.var_qde_dn0 = assign101890_e153920_d_n0;
        locals.var_qde_dn2 = assign101890_e153920_d_n2;
        locals.var_qde_dn4 = assign101890_e153920_d_n4;
        locals.var_qde_dn5 = assign101890_e153920_d_n5;
        locals.var_qde_dn6 = assign101890_e153920_d_n6;
        locals.var_qde_dn7 = assign101890_e153920_d_n7;
        locals.var_qde_dn8 = assign101890_e153920_d_n8;
        locals.var_qde_dn9 = assign101890_e153920_d_n9;
        locals.var_qde_dn10 = assign101890_e153920_d_n10;
        locals.var_qde_dn13 = assign101890_e153920_d_n13;
        locals.var_qde_rv = 0.0;

        let (assign101900_e153929, assign101900_e153929_d_n0, assign101900_e153929_d_n2, assign101900_e153929_d_n4, assign101900_e153929_d_n5, assign101900_e153929_d_n6, assign101900_e153929_d_n7, assign101900_e153929_d_n8, assign101900_e153929_d_n9, assign101900_e153929_d_n10, assign101900_e153929_d_n13,) = {
    if (locals.var_guard2329 == 0.0) {
        let assign101900_e153926: f64 = (locals.var_qi - locals.var_qd);
        let assign101900_e153927: f64 = (locals.var_mfactor * assign101900_e153926);
        (assign101900_e153927, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn4 - locals.var_qd_dn4)), (locals.var_mfactor * (locals.var_qi_dn5 - locals.var_qd_dn5)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn8 - locals.var_qd_dn8)), (locals.var_mfactor * (locals.var_qi_dn9 - locals.var_qd_dn9)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn13 - locals.var_qd_dn13)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn13,)
    }
};
        locals.var_qse = assign101900_e153929;
        locals.var_qse_dn0 = assign101900_e153929_d_n0;
        locals.var_qse_dn2 = assign101900_e153929_d_n2;
        locals.var_qse_dn4 = assign101900_e153929_d_n4;
        locals.var_qse_dn5 = assign101900_e153929_d_n5;
        locals.var_qse_dn6 = assign101900_e153929_d_n6;
        locals.var_qse_dn7 = assign101900_e153929_d_n7;
        locals.var_qse_dn8 = assign101900_e153929_d_n8;
        locals.var_qse_dn9 = assign101900_e153929_d_n9;
        locals.var_qse_dn10 = assign101900_e153929_d_n10;
        locals.var_qse_dn13 = assign101900_e153929_d_n13;
        locals.var_qse_rv = 0.0;

        let (assign101910_e153935, assign101910_e153935_d_n0, assign101910_e153935_d_n2, assign101910_e153935_d_n4, assign101910_e153935_d_n5, assign101910_e153935_d_n6, assign101910_e153935_d_n7, assign101910_e153935_d_n8, assign101910_e153935_d_n9, assign101910_e153935_d_n10, assign101910_e153935_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign101910_e153933: f64 = (locals.var_mks_dlyov * locals.var_psl);
        (assign101910_e153933, ((locals.var_mks_dlyov_dn0 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn0)), ((locals.var_mks_dlyov_dn2 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn2)), ((locals.var_mks_dlyov_dn4 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn4)), ((locals.var_mks_dlyov_dn5 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn5)), ((locals.var_mks_dlyov_dn6 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn6)), ((locals.var_mks_dlyov_dn7 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn7)), ((locals.var_mks_dlyov_dn8 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn8)), ((locals.var_mks_dlyov_dn9 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn9)), ((locals.var_mks_dlyov_dn10 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn10)), ((locals.var_mks_dlyov_dn13 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn13)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn13,)
    }
};
        locals.var_mks_dlyov = assign101910_e153935;
        locals.var_mks_dlyov_dn0 = assign101910_e153935_d_n0;
        locals.var_mks_dlyov_dn2 = assign101910_e153935_d_n2;
        locals.var_mks_dlyov_dn4 = assign101910_e153935_d_n4;
        locals.var_mks_dlyov_dn5 = assign101910_e153935_d_n5;
        locals.var_mks_dlyov_dn6 = assign101910_e153935_d_n6;
        locals.var_mks_dlyov_dn7 = assign101910_e153935_d_n7;
        locals.var_mks_dlyov_dn8 = assign101910_e153935_d_n8;
        locals.var_mks_dlyov_dn9 = assign101910_e153935_d_n9;
        locals.var_mks_dlyov_dn10 = assign101910_e153935_d_n10;
        locals.var_mks_dlyov_dn13 = assign101910_e153935_d_n13;
        locals.var_mks_dlyov_rv = 0.0;

        let (assign101920_e153948, assign101920_e153948_d_n0, assign101920_e153948_d_n2, assign101920_e153948_d_n4, assign101920_e153948_d_n5, assign101920_e153948_d_n6, assign101920_e153948_d_n7, assign101920_e153948_d_n8, assign101920_e153948_d_n9, assign101920_e153948_d_n10, assign101920_e153948_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign101920_e153939: f64 = (locals.var_mks_dlyov * locals.var_mks_dlyov);
        let assign101920_e153942: f64 = (4.0 * 1e-12);
        let assign101920_e153944: f64 = (assign101920_e153942 * 1e-12);
        let assign101920_e153945: f64 = (assign101920_e153939 + assign101920_e153944);
        let assign101920_e153946: f64 = (assign101920_e153945).sqrt();
        (assign101920_e153946, (((locals.var_mks_dlyov_dn0 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn0)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn2 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn2)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn4 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn4)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn5 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn5)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn6 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn6)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn7 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn7)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn8 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn8)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn9 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn9)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn10 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn10)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn13 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn13)) / (2.0 * assign101920_e153946)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign101920_e153948;
        locals.var_tmf2_dn0 = assign101920_e153948_d_n0;
        locals.var_tmf2_dn2 = assign101920_e153948_d_n2;
        locals.var_tmf2_dn4 = assign101920_e153948_d_n4;
        locals.var_tmf2_dn5 = assign101920_e153948_d_n5;
        locals.var_tmf2_dn6 = assign101920_e153948_d_n6;
        locals.var_tmf2_dn7 = assign101920_e153948_d_n7;
        locals.var_tmf2_dn8 = assign101920_e153948_d_n8;
        locals.var_tmf2_dn9 = assign101920_e153948_d_n9;
        locals.var_tmf2_dn10 = assign101920_e153948_d_n10;
        locals.var_tmf2_dn13 = assign101920_e153948_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign101930_e153958, assign101930_e153958_d_n0, assign101930_e153958_d_n2, assign101930_e153958_d_n4, assign101930_e153958_d_n5, assign101930_e153958_d_n6, assign101930_e153958_d_n7, assign101930_e153958_d_n8, assign101930_e153958_d_n9, assign101930_e153958_d_n10, assign101930_e153958_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign101930_e153954: f64 = (locals.var_mks_dlyov / locals.var_tmf2);
        let assign101930_e153955: f64 = (1.0 + assign101930_e153954);
        let assign101930_e153956: f64 = (0.5 * assign101930_e153955);
        (assign101930_e153956, (0.5 * (((locals.var_mks_dlyov_dn0 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn2 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn4 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn5 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn6 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn7 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn8 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn9 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn10 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn13 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign101930_e153958;
        locals.var_t0_dn0 = assign101930_e153958_d_n0;
        locals.var_t0_dn2 = assign101930_e153958_d_n2;
        locals.var_t0_dn4 = assign101930_e153958_d_n4;
        locals.var_t0_dn5 = assign101930_e153958_d_n5;
        locals.var_t0_dn6 = assign101930_e153958_d_n6;
        locals.var_t0_dn7 = assign101930_e153958_d_n7;
        locals.var_t0_dn8 = assign101930_e153958_d_n8;
        locals.var_t0_dn9 = assign101930_e153958_d_n9;
        locals.var_t0_dn10 = assign101930_e153958_d_n10;
        locals.var_t0_dn13 = assign101930_e153958_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign101940_e153966, assign101940_e153966_d_n0, assign101940_e153966_d_n2, assign101940_e153966_d_n4, assign101940_e153966_d_n5, assign101940_e153966_d_n6, assign101940_e153966_d_n7, assign101940_e153966_d_n8, assign101940_e153966_d_n9, assign101940_e153966_d_n10, assign101940_e153966_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign101940_e153963: f64 = (locals.var_mks_dlyov + locals.var_tmf2);
        let assign101940_e153964: f64 = (0.5 * assign101940_e153963);
        (assign101940_e153964, (0.5 * (locals.var_mks_dlyov_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_mks_dlyov_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_mks_dlyov_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_mks_dlyov_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_mks_dlyov_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_mks_dlyov_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_mks_dlyov_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_mks_dlyov_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_mks_dlyov_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_mks_dlyov_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn13,)
    }
};
        locals.var_mks_dlyov = assign101940_e153966;
        locals.var_mks_dlyov_dn0 = assign101940_e153966_d_n0;
        locals.var_mks_dlyov_dn2 = assign101940_e153966_d_n2;
        locals.var_mks_dlyov_dn4 = assign101940_e153966_d_n4;
        locals.var_mks_dlyov_dn5 = assign101940_e153966_d_n5;
        locals.var_mks_dlyov_dn6 = assign101940_e153966_d_n6;
        locals.var_mks_dlyov_dn7 = assign101940_e153966_d_n7;
        locals.var_mks_dlyov_dn8 = assign101940_e153966_d_n8;
        locals.var_mks_dlyov_dn9 = assign101940_e153966_d_n9;
        locals.var_mks_dlyov_dn10 = assign101940_e153966_d_n10;
        locals.var_mks_dlyov_dn13 = assign101940_e153966_d_n13;
        locals.var_mks_dlyov_rv = 0.0;

        let assign101950_e153969: f64 = if locals.var_mks_dlyov < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2330 = assign101950_e153969;
        locals.var_guard2330_rv = 0.0;

        let (assign101960_e153975, assign101960_e153975_d_n0, assign101960_e153975_d_n2, assign101960_e153975_d_n4, assign101960_e153975_d_n5, assign101960_e153975_d_n6, assign101960_e153975_d_n7, assign101960_e153975_d_n8, assign101960_e153975_d_n9, assign101960_e153975_d_n10, assign101960_e153975_d_n13,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2330 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn13,)
    }
};
        locals.var_mks_dlyov = assign101960_e153975;
        locals.var_mks_dlyov_dn0 = assign101960_e153975_d_n0;
        locals.var_mks_dlyov_dn2 = assign101960_e153975_d_n2;
        locals.var_mks_dlyov_dn4 = assign101960_e153975_d_n4;
        locals.var_mks_dlyov_dn5 = assign101960_e153975_d_n5;
        locals.var_mks_dlyov_dn6 = assign101960_e153975_d_n6;
        locals.var_mks_dlyov_dn7 = assign101960_e153975_d_n7;
        locals.var_mks_dlyov_dn8 = assign101960_e153975_d_n8;
        locals.var_mks_dlyov_dn9 = assign101960_e153975_d_n9;
        locals.var_mks_dlyov_dn10 = assign101960_e153975_d_n10;
        locals.var_mks_dlyov_dn13 = assign101960_e153975_d_n13;
        locals.var_mks_dlyov_rv = 0.0;

        let (assign101970_e153981, assign101970_e153981_d_n0, assign101970_e153981_d_n2, assign101970_e153981_d_n4, assign101970_e153981_d_n5, assign101970_e153981_d_n6, assign101970_e153981_d_n7, assign101970_e153981_d_n8, assign101970_e153981_d_n9, assign101970_e153981_d_n10, assign101970_e153981_d_n13,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2330 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign101970_e153981;
        locals.var_t0_dn0 = assign101970_e153981_d_n0;
        locals.var_t0_dn2 = assign101970_e153981_d_n2;
        locals.var_t0_dn4 = assign101970_e153981_d_n4;
        locals.var_t0_dn5 = assign101970_e153981_d_n5;
        locals.var_t0_dn6 = assign101970_e153981_d_n6;
        locals.var_t0_dn7 = assign101970_e153981_d_n7;
        locals.var_t0_dn8 = assign101970_e153981_d_n8;
        locals.var_t0_dn9 = assign101970_e153981_d_n9;
        locals.var_t0_dn10 = assign101970_e153981_d_n10;
        locals.var_t0_dn13 = assign101970_e153981_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign101990_e153991, assign101990_e153991_d_n0, assign101990_e153991_d_n2, assign101990_e153991_d_n4, assign101990_e153991_d_n5, assign101990_e153991_d_n6, assign101990_e153991_d_n7, assign101990_e153991_d_n8, assign101990_e153991_d_n9, assign101990_e153991_d_n10, assign101990_e153991_d_n13,) = {
    if (p.p29 != 0.0) {
        ((nv13 - 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn13,)
    }
};
        locals.var_qbd_nqs = assign101990_e153991;
        locals.var_qbd_nqs_dn0 = assign101990_e153991_d_n0;
        locals.var_qbd_nqs_dn2 = assign101990_e153991_d_n2;
        locals.var_qbd_nqs_dn4 = assign101990_e153991_d_n4;
        locals.var_qbd_nqs_dn5 = assign101990_e153991_d_n5;
        locals.var_qbd_nqs_dn6 = assign101990_e153991_d_n6;
        locals.var_qbd_nqs_dn7 = assign101990_e153991_d_n7;
        locals.var_qbd_nqs_dn8 = assign101990_e153991_d_n8;
        locals.var_qbd_nqs_dn9 = assign101990_e153991_d_n9;
        locals.var_qbd_nqs_dn10 = assign101990_e153991_d_n10;
        locals.var_qbd_nqs_dn13 = assign101990_e153991_d_n13;
        locals.var_qbd_nqs_rv = 0.0;

        let (assign102010_e154007, assign102010_e154007_d_n0, assign102010_e154007_d_n2, assign102010_e154007_d_n4, assign102010_e154007_d_n5, assign102010_e154007_d_n6, assign102010_e154007_d_n7, assign102010_e154007_d_n8, assign102010_e154007_d_n9, assign102010_e154007_d_n10, assign102010_e154007_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign102010_e154004: f64 = (locals.var_qbd_qs - locals.var_qbd_nqs);
        let assign102010_e154005: f64 = (locals.var_qovd - assign102010_e154004);
        (assign102010_e154005, (locals.var_qovd_dn0 - (locals.var_qbd_qs_dn0 - locals.var_qbd_nqs_dn0)), (locals.var_qovd_dn2 - (locals.var_qbd_qs_dn2 - locals.var_qbd_nqs_dn2)), (locals.var_qovd_dn4 - (locals.var_qbd_qs_dn4 - locals.var_qbd_nqs_dn4)), (locals.var_qovd_dn5 - (locals.var_qbd_qs_dn5 - locals.var_qbd_nqs_dn5)), (locals.var_qovd_dn6 - (locals.var_qbd_qs_dn6 - locals.var_qbd_nqs_dn6)), (locals.var_qovd_dn7 - (locals.var_qbd_qs_dn7 - locals.var_qbd_nqs_dn7)), (locals.var_qovd_dn8 - (locals.var_qbd_qs_dn8 - locals.var_qbd_nqs_dn8)), (locals.var_qovd_dn9 - (locals.var_qbd_qs_dn9 - locals.var_qbd_nqs_dn9)), (locals.var_qovd_dn10 - (locals.var_qbd_qs_dn10 - locals.var_qbd_nqs_dn10)), (locals.var_qovd_dn13 - (locals.var_qbd_qs_dn13 - locals.var_qbd_nqs_dn13)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn13,)
    }
};
        locals.var_qovd = assign102010_e154007;
        locals.var_qovd_dn0 = assign102010_e154007_d_n0;
        locals.var_qovd_dn2 = assign102010_e154007_d_n2;
        locals.var_qovd_dn4 = assign102010_e154007_d_n4;
        locals.var_qovd_dn5 = assign102010_e154007_d_n5;
        locals.var_qovd_dn6 = assign102010_e154007_d_n6;
        locals.var_qovd_dn7 = assign102010_e154007_d_n7;
        locals.var_qovd_dn8 = assign102010_e154007_d_n8;
        locals.var_qovd_dn9 = assign102010_e154007_d_n9;
        locals.var_qovd_dn10 = assign102010_e154007_d_n10;
        locals.var_qovd_dn13 = assign102010_e154007_d_n13;
        locals.var_qovd_rv = 0.0;

        let (assign102020_e154011, assign102020_e154011_d_n0, assign102020_e154011_d_n2, assign102020_e154011_d_n4, assign102020_e154011_d_n5, assign102020_e154011_d_n6, assign102020_e154011_d_n7, assign102020_e154011_d_n8, assign102020_e154011_d_n9, assign102020_e154011_d_n10, assign102020_e154011_d_n13,) = {
    if (p.p29 != 0.0) {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn13,)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    }
};
        locals.var_qbdld = assign102020_e154011;
        locals.var_qbdld_dn0 = assign102020_e154011_d_n0;
        locals.var_qbdld_dn2 = assign102020_e154011_d_n2;
        locals.var_qbdld_dn4 = assign102020_e154011_d_n4;
        locals.var_qbdld_dn5 = assign102020_e154011_d_n5;
        locals.var_qbdld_dn6 = assign102020_e154011_d_n6;
        locals.var_qbdld_dn7 = assign102020_e154011_d_n7;
        locals.var_qbdld_dn8 = assign102020_e154011_d_n8;
        locals.var_qbdld_dn9 = assign102020_e154011_d_n9;
        locals.var_qbdld_dn10 = assign102020_e154011_d_n10;
        locals.var_qbdld_dn13 = assign102020_e154011_d_n13;
        locals.var_qbdld_rv = 0.0;

        let (assign102030_e154016, assign102030_e154016_d_n0, assign102030_e154016_d_n2, assign102030_e154016_d_n4, assign102030_e154016_d_n5, assign102030_e154016_d_n6, assign102030_e154016_d_n7, assign102030_e154016_d_n8, assign102030_e154016_d_n9, assign102030_e154016_d_n10, assign102030_e154016_d_n13,) = {
    if (p.p29 == 0.0) {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn13,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn13,)
    }
};
        locals.var_qbd_nqs = assign102030_e154016;
        locals.var_qbd_nqs_dn0 = assign102030_e154016_d_n0;
        locals.var_qbd_nqs_dn2 = assign102030_e154016_d_n2;
        locals.var_qbd_nqs_dn4 = assign102030_e154016_d_n4;
        locals.var_qbd_nqs_dn5 = assign102030_e154016_d_n5;
        locals.var_qbd_nqs_dn6 = assign102030_e154016_d_n6;
        locals.var_qbd_nqs_dn7 = assign102030_e154016_d_n7;
        locals.var_qbd_nqs_dn8 = assign102030_e154016_d_n8;
        locals.var_qbd_nqs_dn9 = assign102030_e154016_d_n9;
        locals.var_qbd_nqs_dn10 = assign102030_e154016_d_n10;
        locals.var_qbd_nqs_dn13 = assign102030_e154016_d_n13;
        locals.var_qbd_nqs_rv = 0.0;

        let assign102040_e154019: f64 = if p.p22 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2331 = assign102040_e154019;
        locals.var_guard2331_rv = 0.0;

        let (assign102050_e154033, assign102050_e154033_d_n0, assign102050_e154033_d_n2, assign102050_e154033_d_n4, assign102050_e154033_d_n5, assign102050_e154033_d_n6, assign102050_e154033_d_n7, assign102050_e154033_d_n8, assign102050_e154033_d_n9, assign102050_e154033_d_n10, assign102050_e154033_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102050_e154024: f64 = (locals.var_qgbo - locals.var_qovd);
        let assign102050_e154026: f64 = (assign102050_e154024 - locals.var_qovs);
        let assign102050_e154028: f64 = (assign102050_e154026 + locals.var_qgos);
        let assign102050_e154030: f64 = (assign102050_e154028 + locals.var_qgod);
        let assign102050_e154031: f64 = (locals.var_mfactor * assign102050_e154030);
        (assign102050_e154031, (locals.var_mfactor * ((((-locals.var_qovd_dn0) - locals.var_qovs_dn0) + locals.var_qgos_dn0) + locals.var_qgod_dn0)), (locals.var_mfactor * ((((-locals.var_qovd_dn2) - locals.var_qovs_dn2) + locals.var_qgos_dn2) + locals.var_qgod_dn2)), (locals.var_mfactor * ((((-locals.var_qovd_dn4) - locals.var_qovs_dn4) + locals.var_qgos_dn4) + locals.var_qgod_dn4)), (locals.var_mfactor * ((((-locals.var_qovd_dn5) - locals.var_qovs_dn5) + locals.var_qgos_dn5) + locals.var_qgod_dn5)), (locals.var_mfactor * ((((locals.var_qgbo_dn6 - locals.var_qovd_dn6) - locals.var_qovs_dn6) + locals.var_qgos_dn6) + locals.var_qgod_dn6)), (locals.var_mfactor * ((((locals.var_qgbo_dn7 - locals.var_qovd_dn7) - locals.var_qovs_dn7) + locals.var_qgos_dn7) + locals.var_qgod_dn7)), (locals.var_mfactor * ((((locals.var_qgbo_dn8 - locals.var_qovd_dn8) - locals.var_qovs_dn8) + locals.var_qgos_dn8) + locals.var_qgod_dn8)), (locals.var_mfactor * ((((-locals.var_qovd_dn9) - locals.var_qovs_dn9) + locals.var_qgos_dn9) + locals.var_qgod_dn9)), (locals.var_mfactor * ((((-locals.var_qovd_dn10) - locals.var_qovs_dn10) + locals.var_qgos_dn10) + locals.var_qgod_dn10)), (locals.var_mfactor * ((((-locals.var_qovd_dn13) - locals.var_qovs_dn13) + locals.var_qgos_dn13) + locals.var_qgod_dn13)),)
    } else {
        (locals.var_qgov, locals.var_qgov_dn0, locals.var_qgov_dn2, locals.var_qgov_dn4, locals.var_qgov_dn5, locals.var_qgov_dn6, locals.var_qgov_dn7, locals.var_qgov_dn8, locals.var_qgov_dn9, locals.var_qgov_dn10, locals.var_qgov_dn13,)
    }
};
        locals.var_qgov = assign102050_e154033;
        locals.var_qgov_dn0 = assign102050_e154033_d_n0;
        locals.var_qgov_dn2 = assign102050_e154033_d_n2;
        locals.var_qgov_dn4 = assign102050_e154033_d_n4;
        locals.var_qgov_dn5 = assign102050_e154033_d_n5;
        locals.var_qgov_dn6 = assign102050_e154033_d_n6;
        locals.var_qgov_dn7 = assign102050_e154033_d_n7;
        locals.var_qgov_dn8 = assign102050_e154033_d_n8;
        locals.var_qgov_dn9 = assign102050_e154033_d_n9;
        locals.var_qgov_dn10 = assign102050_e154033_d_n10;
        locals.var_qgov_dn13 = assign102050_e154033_d_n13;
        locals.var_qgov_rv = 0.0;

        let (assign102060_e154042, assign102060_e154042_d_n0, assign102060_e154042_d_n2, assign102060_e154042_d_n4, assign102060_e154042_d_n5, assign102060_e154042_d_n6, assign102060_e154042_d_n7, assign102060_e154042_d_n8, assign102060_e154042_d_n9, assign102060_e154042_d_n10, assign102060_e154042_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102060_e154037: f64 = locals.var_qbdld;
        let assign102060_e154039: f64 = (assign102060_e154037 - locals.var_qgod);
        let assign102060_e154040: f64 = (locals.var_mfactor * assign102060_e154039);
        (assign102060_e154040, (locals.var_mfactor * (locals.var_qbdld_dn0 - locals.var_qgod_dn0)), (locals.var_mfactor * (locals.var_qbdld_dn2 - locals.var_qgod_dn2)), (locals.var_mfactor * (locals.var_qbdld_dn4 - locals.var_qgod_dn4)), (locals.var_mfactor * (locals.var_qbdld_dn5 - locals.var_qgod_dn5)), (locals.var_mfactor * (locals.var_qbdld_dn6 - locals.var_qgod_dn6)), (locals.var_mfactor * (locals.var_qbdld_dn7 - locals.var_qgod_dn7)), (locals.var_mfactor * (locals.var_qbdld_dn8 - locals.var_qgod_dn8)), (locals.var_mfactor * (locals.var_qbdld_dn9 - locals.var_qgod_dn9)), (locals.var_mfactor * (locals.var_qbdld_dn10 - locals.var_qgod_dn10)), (locals.var_mfactor * (locals.var_qbdld_dn13 - locals.var_qgod_dn13)),)
    } else {
        (locals.var_qdov, locals.var_qdov_dn0, locals.var_qdov_dn2, locals.var_qdov_dn4, locals.var_qdov_dn5, locals.var_qdov_dn6, locals.var_qdov_dn7, locals.var_qdov_dn8, locals.var_qdov_dn9, locals.var_qdov_dn10, locals.var_qdov_dn13,)
    }
};
        locals.var_qdov = assign102060_e154042;
        locals.var_qdov_dn0 = assign102060_e154042_d_n0;
        locals.var_qdov_dn2 = assign102060_e154042_d_n2;
        locals.var_qdov_dn4 = assign102060_e154042_d_n4;
        locals.var_qdov_dn5 = assign102060_e154042_d_n5;
        locals.var_qdov_dn6 = assign102060_e154042_d_n6;
        locals.var_qdov_dn7 = assign102060_e154042_d_n7;
        locals.var_qdov_dn8 = assign102060_e154042_d_n8;
        locals.var_qdov_dn9 = assign102060_e154042_d_n9;
        locals.var_qdov_dn10 = assign102060_e154042_d_n10;
        locals.var_qdov_dn13 = assign102060_e154042_d_n13;
        locals.var_qdov_rv = 0.0;

        let (assign102070_e154051, assign102070_e154051_d_n0, assign102070_e154051_d_n2, assign102070_e154051_d_n4, assign102070_e154051_d_n5, assign102070_e154051_d_n6, assign102070_e154051_d_n7, assign102070_e154051_d_n8, assign102070_e154051_d_n9, assign102070_e154051_d_n10, assign102070_e154051_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102070_e154046: f64 = locals.var_qbsld;
        let assign102070_e154048: f64 = (assign102070_e154046 - locals.var_qgos);
        let assign102070_e154049: f64 = (locals.var_mfactor * assign102070_e154048);
        (assign102070_e154049, (locals.var_mfactor * (locals.var_qbsld_dn0 - locals.var_qgos_dn0)), (locals.var_mfactor * (locals.var_qbsld_dn2 - locals.var_qgos_dn2)), (locals.var_mfactor * (locals.var_qbsld_dn4 - locals.var_qgos_dn4)), (locals.var_mfactor * (locals.var_qbsld_dn5 - locals.var_qgos_dn5)), (locals.var_mfactor * (locals.var_qbsld_dn6 - locals.var_qgos_dn6)), (locals.var_mfactor * (locals.var_qbsld_dn7 - locals.var_qgos_dn7)), (locals.var_mfactor * (locals.var_qbsld_dn8 - locals.var_qgos_dn8)), (locals.var_mfactor * (locals.var_qbsld_dn9 - locals.var_qgos_dn9)), (locals.var_mfactor * (locals.var_qbsld_dn10 - locals.var_qgos_dn10)), (locals.var_mfactor * (locals.var_qbsld_dn13 - locals.var_qgos_dn13)),)
    } else {
        (locals.var_qsov, locals.var_qsov_dn0, locals.var_qsov_dn2, locals.var_qsov_dn4, locals.var_qsov_dn5, locals.var_qsov_dn6, locals.var_qsov_dn7, locals.var_qsov_dn8, locals.var_qsov_dn9, locals.var_qsov_dn10, locals.var_qsov_dn13,)
    }
};
        locals.var_qsov = assign102070_e154051;
        locals.var_qsov_dn0 = assign102070_e154051_d_n0;
        locals.var_qsov_dn2 = assign102070_e154051_d_n2;
        locals.var_qsov_dn4 = assign102070_e154051_d_n4;
        locals.var_qsov_dn5 = assign102070_e154051_d_n5;
        locals.var_qsov_dn6 = assign102070_e154051_d_n6;
        locals.var_qsov_dn7 = assign102070_e154051_d_n7;
        locals.var_qsov_dn8 = assign102070_e154051_d_n8;
        locals.var_qsov_dn9 = assign102070_e154051_d_n9;
        locals.var_qsov_dn10 = assign102070_e154051_d_n10;
        locals.var_qsov_dn13 = assign102070_e154051_d_n13;
        locals.var_qsov_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_379(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign102080_e154064, assign102080_e154064_d_n0, assign102080_e154064_d_n2, assign102080_e154064_d_n4, assign102080_e154064_d_n5, assign102080_e154064_d_n6, assign102080_e154064_d_n7, assign102080_e154064_d_n8, assign102080_e154064_d_n9, assign102080_e154064_d_n10, assign102080_e154064_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102080_e154056: f64 = locals.var_qy;
        let assign102080_e154058: f64 = (assign102080_e154056 - locals.var_qovd_add);
        let assign102080_e154060: f64 = (assign102080_e154058 - locals.var_qovs_add);
        let assign102080_e154061: f64 = (locals.var_mfactor * assign102080_e154060);
        let assign102080_e154062: f64 = (locals.var_qge + assign102080_e154061);
        (assign102080_e154062, (locals.var_qge_dn0 + (locals.var_mfactor * ((locals.var_qy_dn0 - locals.var_qovd_add_dn0) - locals.var_qovs_add_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((locals.var_qy_dn2 - locals.var_qovd_add_dn2) - locals.var_qovs_add_dn2))), (locals.var_qge_dn4 + (locals.var_mfactor * ((locals.var_qy_dn4 - locals.var_qovd_add_dn4) - locals.var_qovs_add_dn4))), (locals.var_qge_dn5 + (locals.var_mfactor * ((locals.var_qy_dn5 - locals.var_qovd_add_dn5) - locals.var_qovs_add_dn5))), (locals.var_qge_dn6 + (locals.var_mfactor * ((locals.var_qy_dn6 - locals.var_qovd_add_dn6) - locals.var_qovs_add_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((locals.var_qy_dn7 - locals.var_qovd_add_dn7) - locals.var_qovs_add_dn7))), (locals.var_qge_dn8 + (locals.var_mfactor * ((locals.var_qy_dn8 - locals.var_qovd_add_dn8) - locals.var_qovs_add_dn8))), (locals.var_qge_dn9 + (locals.var_mfactor * ((locals.var_qy_dn9 - locals.var_qovd_add_dn9) - locals.var_qovs_add_dn9))), (locals.var_qge_dn10 + (locals.var_mfactor * ((locals.var_qy_dn10 - locals.var_qovd_add_dn10) - locals.var_qovs_add_dn10))), (locals.var_qge_dn13 + (locals.var_mfactor * ((locals.var_qy_dn13 - locals.var_qovd_add_dn13) - locals.var_qovs_add_dn13))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn13,)
    }
};
        locals.var_qge = assign102080_e154064;
        locals.var_qge_dn0 = assign102080_e154064_d_n0;
        locals.var_qge_dn2 = assign102080_e154064_d_n2;
        locals.var_qge_dn4 = assign102080_e154064_d_n4;
        locals.var_qge_dn5 = assign102080_e154064_d_n5;
        locals.var_qge_dn6 = assign102080_e154064_d_n6;
        locals.var_qge_dn7 = assign102080_e154064_d_n7;
        locals.var_qge_dn8 = assign102080_e154064_d_n8;
        locals.var_qge_dn9 = assign102080_e154064_d_n9;
        locals.var_qge_dn10 = assign102080_e154064_d_n10;
        locals.var_qge_dn13 = assign102080_e154064_d_n13;
        locals.var_qge_rv = 0.0;

        let (assign102090_e154075, assign102090_e154075_d_n0, assign102090_e154075_d_n2, assign102090_e154075_d_n4, assign102090_e154075_d_n5, assign102090_e154075_d_n6, assign102090_e154075_d_n7, assign102090_e154075_d_n8, assign102090_e154075_d_n9, assign102090_e154075_d_n10, assign102090_e154075_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102090_e154069: f64 = (-locals.var_qy);
        let assign102090_e154071: f64 = (assign102090_e154069 + locals.var_qbdld_add);
        let assign102090_e154072: f64 = (locals.var_mfactor * assign102090_e154071);
        let assign102090_e154073: f64 = (locals.var_qde + assign102090_e154072);
        (assign102090_e154073, (locals.var_qde_dn0 + (locals.var_mfactor * ((-locals.var_qy_dn0) + locals.var_qbdld_add_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((-locals.var_qy_dn2) + locals.var_qbdld_add_dn2))), (locals.var_qde_dn4 + (locals.var_mfactor * ((-locals.var_qy_dn4) + locals.var_qbdld_add_dn4))), (locals.var_qde_dn5 + (locals.var_mfactor * ((-locals.var_qy_dn5) + locals.var_qbdld_add_dn5))), (locals.var_qde_dn6 + (locals.var_mfactor * ((-locals.var_qy_dn6) + locals.var_qbdld_add_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((-locals.var_qy_dn7) + locals.var_qbdld_add_dn7))), (locals.var_qde_dn8 + (locals.var_mfactor * ((-locals.var_qy_dn8) + locals.var_qbdld_add_dn8))), (locals.var_qde_dn9 + (locals.var_mfactor * ((-locals.var_qy_dn9) + locals.var_qbdld_add_dn9))), (locals.var_qde_dn10 + (locals.var_mfactor * ((-locals.var_qy_dn10) + locals.var_qbdld_add_dn10))), (locals.var_qde_dn13 + (locals.var_mfactor * ((-locals.var_qy_dn13) + locals.var_qbdld_add_dn13))),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn13,)
    }
};
        locals.var_qde = assign102090_e154075;
        locals.var_qde_dn0 = assign102090_e154075_d_n0;
        locals.var_qde_dn2 = assign102090_e154075_d_n2;
        locals.var_qde_dn4 = assign102090_e154075_d_n4;
        locals.var_qde_dn5 = assign102090_e154075_d_n5;
        locals.var_qde_dn6 = assign102090_e154075_d_n6;
        locals.var_qde_dn7 = assign102090_e154075_d_n7;
        locals.var_qde_dn8 = assign102090_e154075_d_n8;
        locals.var_qde_dn9 = assign102090_e154075_d_n9;
        locals.var_qde_dn10 = assign102090_e154075_d_n10;
        locals.var_qde_dn13 = assign102090_e154075_d_n13;
        locals.var_qde_rv = 0.0;

        let (assign102100_e154084, assign102100_e154084_d_n0, assign102100_e154084_d_n2, assign102100_e154084_d_n4, assign102100_e154084_d_n5, assign102100_e154084_d_n6, assign102100_e154084_d_n7, assign102100_e154084_d_n8, assign102100_e154084_d_n9, assign102100_e154084_d_n10, assign102100_e154084_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102100_e154080: f64 = locals.var_qbsld_add;
        let assign102100_e154081: f64 = (locals.var_mfactor * assign102100_e154080);
        let assign102100_e154082: f64 = (locals.var_qse + assign102100_e154081);
        (assign102100_e154082, (locals.var_qse_dn0 + (locals.var_mfactor * locals.var_qbsld_add_dn0)), (locals.var_qse_dn2 + (locals.var_mfactor * locals.var_qbsld_add_dn2)), (locals.var_qse_dn4 + (locals.var_mfactor * locals.var_qbsld_add_dn4)), (locals.var_qse_dn5 + (locals.var_mfactor * locals.var_qbsld_add_dn5)), (locals.var_qse_dn6 + (locals.var_mfactor * locals.var_qbsld_add_dn6)), (locals.var_qse_dn7 + (locals.var_mfactor * locals.var_qbsld_add_dn7)), (locals.var_qse_dn8 + (locals.var_mfactor * locals.var_qbsld_add_dn8)), (locals.var_qse_dn9 + (locals.var_mfactor * locals.var_qbsld_add_dn9)), (locals.var_qse_dn10 + (locals.var_mfactor * locals.var_qbsld_add_dn10)), (locals.var_qse_dn13 + (locals.var_mfactor * locals.var_qbsld_add_dn13)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn13,)
    }
};
        locals.var_qse = assign102100_e154084;
        locals.var_qse_dn0 = assign102100_e154084_d_n0;
        locals.var_qse_dn2 = assign102100_e154084_d_n2;
        locals.var_qse_dn4 = assign102100_e154084_d_n4;
        locals.var_qse_dn5 = assign102100_e154084_d_n5;
        locals.var_qse_dn6 = assign102100_e154084_d_n6;
        locals.var_qse_dn7 = assign102100_e154084_d_n7;
        locals.var_qse_dn8 = assign102100_e154084_d_n8;
        locals.var_qse_dn9 = assign102100_e154084_d_n9;
        locals.var_qse_dn10 = assign102100_e154084_d_n10;
        locals.var_qse_dn13 = assign102100_e154084_d_n13;
        locals.var_qse_rv = 0.0;

        let (assign102110_e154093, assign102110_e154093_d_n0, assign102110_e154093_d_n2, assign102110_e154093_d_n4, assign102110_e154093_d_n5, assign102110_e154093_d_n6, assign102110_e154093_d_n7, assign102110_e154093_d_n8, assign102110_e154093_d_n9, assign102110_e154093_d_n10, assign102110_e154093_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102110_e154088: f64 = (-locals.var_qovdext);
        let assign102110_e154090: f64 = (assign102110_e154088 - locals.var_qovsext);
        let assign102110_e154091: f64 = (locals.var_mfactor * assign102110_e154090);
        (assign102110_e154091, (locals.var_mfactor * ((-locals.var_qovdext_dn0) - locals.var_qovsext_dn0)), (locals.var_mfactor * ((-locals.var_qovdext_dn2) - locals.var_qovsext_dn2)), (locals.var_mfactor * ((-locals.var_qovdext_dn4) - locals.var_qovsext_dn4)), (locals.var_mfactor * ((-locals.var_qovdext_dn5) - locals.var_qovsext_dn5)), (locals.var_mfactor * ((-locals.var_qovdext_dn6) - locals.var_qovsext_dn6)), (locals.var_mfactor * ((-locals.var_qovdext_dn7) - locals.var_qovsext_dn7)), (locals.var_mfactor * ((-locals.var_qovdext_dn8) - locals.var_qovsext_dn8)), (locals.var_mfactor * ((-locals.var_qovdext_dn9) - locals.var_qovsext_dn9)), (locals.var_mfactor * ((-locals.var_qovdext_dn10) - locals.var_qovsext_dn10)), (locals.var_mfactor * ((-locals.var_qovdext_dn13) - locals.var_qovsext_dn13)),)
    } else {
        (locals.var_qgexte, locals.var_qgexte_dn0, locals.var_qgexte_dn2, locals.var_qgexte_dn4, locals.var_qgexte_dn5, locals.var_qgexte_dn6, locals.var_qgexte_dn7, locals.var_qgexte_dn8, locals.var_qgexte_dn9, locals.var_qgexte_dn10, locals.var_qgexte_dn13,)
    }
};
        locals.var_qgexte = assign102110_e154093;
        locals.var_qgexte_dn0 = assign102110_e154093_d_n0;
        locals.var_qgexte_dn2 = assign102110_e154093_d_n2;
        locals.var_qgexte_dn4 = assign102110_e154093_d_n4;
        locals.var_qgexte_dn5 = assign102110_e154093_d_n5;
        locals.var_qgexte_dn6 = assign102110_e154093_d_n6;
        locals.var_qgexte_dn7 = assign102110_e154093_d_n7;
        locals.var_qgexte_dn8 = assign102110_e154093_d_n8;
        locals.var_qgexte_dn9 = assign102110_e154093_d_n9;
        locals.var_qgexte_dn10 = assign102110_e154093_d_n10;
        locals.var_qgexte_dn13 = assign102110_e154093_d_n13;
        locals.var_qgexte_rv = 0.0;

        let (assign102120_e154099, assign102120_e154099_d_n0, assign102120_e154099_d_n2, assign102120_e154099_d_n4, assign102120_e154099_d_n5, assign102120_e154099_d_n6, assign102120_e154099_d_n7, assign102120_e154099_d_n8, assign102120_e154099_d_n9, assign102120_e154099_d_n10, assign102120_e154099_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102120_e154097: f64 = (locals.var_mfactor * locals.var_qbdldext);
        (assign102120_e154097, (locals.var_mfactor * locals.var_qbdldext_dn0), (locals.var_mfactor * locals.var_qbdldext_dn2), (locals.var_mfactor * locals.var_qbdldext_dn4), (locals.var_mfactor * locals.var_qbdldext_dn5), (locals.var_mfactor * locals.var_qbdldext_dn6), (locals.var_mfactor * locals.var_qbdldext_dn7), (locals.var_mfactor * locals.var_qbdldext_dn8), (locals.var_mfactor * locals.var_qbdldext_dn9), (locals.var_mfactor * locals.var_qbdldext_dn10), (locals.var_mfactor * locals.var_qbdldext_dn13),)
    } else {
        (locals.var_qdexte, locals.var_qdexte_dn0, locals.var_qdexte_dn2, locals.var_qdexte_dn4, locals.var_qdexte_dn5, locals.var_qdexte_dn6, locals.var_qdexte_dn7, locals.var_qdexte_dn8, locals.var_qdexte_dn9, locals.var_qdexte_dn10, locals.var_qdexte_dn13,)
    }
};
        locals.var_qdexte = assign102120_e154099;
        locals.var_qdexte_dn0 = assign102120_e154099_d_n0;
        locals.var_qdexte_dn2 = assign102120_e154099_d_n2;
        locals.var_qdexte_dn4 = assign102120_e154099_d_n4;
        locals.var_qdexte_dn5 = assign102120_e154099_d_n5;
        locals.var_qdexte_dn6 = assign102120_e154099_d_n6;
        locals.var_qdexte_dn7 = assign102120_e154099_d_n7;
        locals.var_qdexte_dn8 = assign102120_e154099_d_n8;
        locals.var_qdexte_dn9 = assign102120_e154099_d_n9;
        locals.var_qdexte_dn10 = assign102120_e154099_d_n10;
        locals.var_qdexte_dn13 = assign102120_e154099_d_n13;
        locals.var_qdexte_rv = 0.0;

        let (assign102130_e154105, assign102130_e154105_d_n0, assign102130_e154105_d_n2, assign102130_e154105_d_n4, assign102130_e154105_d_n5, assign102130_e154105_d_n6, assign102130_e154105_d_n7, assign102130_e154105_d_n8, assign102130_e154105_d_n9, assign102130_e154105_d_n10, assign102130_e154105_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102130_e154103: f64 = (locals.var_mfactor * locals.var_qbsldext);
        (assign102130_e154103, (locals.var_mfactor * locals.var_qbsldext_dn0), (locals.var_mfactor * locals.var_qbsldext_dn2), (locals.var_mfactor * locals.var_qbsldext_dn4), (locals.var_mfactor * locals.var_qbsldext_dn5), (locals.var_mfactor * locals.var_qbsldext_dn6), (locals.var_mfactor * locals.var_qbsldext_dn7), (locals.var_mfactor * locals.var_qbsldext_dn8), (locals.var_mfactor * locals.var_qbsldext_dn9), (locals.var_mfactor * locals.var_qbsldext_dn10), (locals.var_mfactor * locals.var_qbsldext_dn13),)
    } else {
        (locals.var_qsexte, locals.var_qsexte_dn0, locals.var_qsexte_dn2, locals.var_qsexte_dn4, locals.var_qsexte_dn5, locals.var_qsexte_dn6, locals.var_qsexte_dn7, locals.var_qsexte_dn8, locals.var_qsexte_dn9, locals.var_qsexte_dn10, locals.var_qsexte_dn13,)
    }
};
        locals.var_qsexte = assign102130_e154105;
        locals.var_qsexte_dn0 = assign102130_e154105_d_n0;
        locals.var_qsexte_dn2 = assign102130_e154105_d_n2;
        locals.var_qsexte_dn4 = assign102130_e154105_d_n4;
        locals.var_qsexte_dn5 = assign102130_e154105_d_n5;
        locals.var_qsexte_dn6 = assign102130_e154105_d_n6;
        locals.var_qsexte_dn7 = assign102130_e154105_d_n7;
        locals.var_qsexte_dn8 = assign102130_e154105_d_n8;
        locals.var_qsexte_dn9 = assign102130_e154105_d_n9;
        locals.var_qsexte_dn10 = assign102130_e154105_d_n10;
        locals.var_qsexte_dn13 = assign102130_e154105_d_n13;
        locals.var_qsexte_rv = 0.0;

        let (assign102140_e154116, assign102140_e154116_d_n0, assign102140_e154116_d_n2, assign102140_e154116_d_n6,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102140_e154110: f64 = (-locals.var_qfd);
        let assign102140_e154112: f64 = (assign102140_e154110 - locals.var_qgdo);
        let assign102140_e154113: f64 = (locals.var_mfactor * assign102140_e154112);
        let assign102140_e154114: f64 = (locals.var_qdp + assign102140_e154113);
        (assign102140_e154114, (locals.var_qdp_dn0 + (locals.var_mfactor * ((-locals.var_qfd_dn0) - locals.var_qgdo_dn0))), (locals.var_qdp_dn2 + (locals.var_mfactor * ((-locals.var_qfd_dn2) - locals.var_qgdo_dn2))), (locals.var_qdp_dn6 + (locals.var_mfactor * ((-locals.var_qfd_dn6) - locals.var_qgdo_dn6))),)
    } else {
        (locals.var_qdp, locals.var_qdp_dn0, locals.var_qdp_dn2, locals.var_qdp_dn6,)
    }
};
        locals.var_qdp = assign102140_e154116;
        locals.var_qdp_dn0 = assign102140_e154116_d_n0;
        locals.var_qdp_dn2 = assign102140_e154116_d_n2;
        locals.var_qdp_dn6 = assign102140_e154116_d_n6;
        locals.var_qdp_rv = 0.0;

        let (assign102150_e154127, assign102150_e154127_d_n2, assign102150_e154127_d_n6,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102150_e154121: f64 = (-locals.var_qfs);
        let assign102150_e154123: f64 = (assign102150_e154121 - locals.var_qgso);
        let assign102150_e154124: f64 = (locals.var_mfactor * assign102150_e154123);
        let assign102150_e154125: f64 = (locals.var_qsp + assign102150_e154124);
        (assign102150_e154125, (locals.var_qsp_dn2 + (locals.var_mfactor * ((-locals.var_qfs_dn2) - locals.var_qgso_dn2))), (locals.var_qsp_dn6 + (locals.var_mfactor * ((-locals.var_qfs_dn6) - locals.var_qgso_dn6))),)
    } else {
        (locals.var_qsp, locals.var_qsp_dn2, locals.var_qsp_dn6,)
    }
};
        locals.var_qsp = assign102150_e154127;
        locals.var_qsp_dn2 = assign102150_e154127_d_n2;
        locals.var_qsp_dn6 = assign102150_e154127_d_n6;
        locals.var_qsp_rv = 0.0;

        let assign102160_e154131: f64 = (locals.var_isub + locals.var_isubibpc);
        let assign102160_e154132: f64 = (locals.var_mfactor * assign102160_e154131);
        locals.var_isube = assign102160_e154132;
        locals.var_isube_dn0 = (locals.var_mfactor * (locals.var_isub_dn0 + locals.var_isubibpc_dn0));
        locals.var_isube_dn2 = (locals.var_mfactor * (locals.var_isub_dn2 + locals.var_isubibpc_dn2));
        locals.var_isube_dn4 = (locals.var_mfactor * (locals.var_isub_dn4 + locals.var_isubibpc_dn4));
        locals.var_isube_dn5 = (locals.var_mfactor * (locals.var_isub_dn5 + locals.var_isubibpc_dn5));
        locals.var_isube_dn6 = (locals.var_mfactor * (locals.var_isub_dn6 + locals.var_isubibpc_dn6));
        locals.var_isube_dn7 = (locals.var_mfactor * (locals.var_isub_dn7 + locals.var_isubibpc_dn7));
        locals.var_isube_dn8 = (locals.var_mfactor * (locals.var_isub_dn8 + locals.var_isubibpc_dn8));
        locals.var_isube_dn9 = (locals.var_mfactor * (locals.var_isub_dn9 + locals.var_isubibpc_dn9));
        locals.var_isube_dn10 = (locals.var_mfactor * (locals.var_isub_dn10 + locals.var_isubibpc_dn10));
        locals.var_isube_dn13 = (locals.var_mfactor * (locals.var_isub_dn13 + locals.var_isubibpc_dn13));
        locals.var_isube_rv = 0.0;

        let assign102170_e154135: f64 = (locals.var_mfactor * locals.var_isubld);
        locals.var_isublde = assign102170_e154135;
        locals.var_isublde_dn0 = (locals.var_mfactor * locals.var_isubld_dn0);
        locals.var_isublde_dn2 = (locals.var_mfactor * locals.var_isubld_dn2);
        locals.var_isublde_dn4 = (locals.var_mfactor * locals.var_isubld_dn4);
        locals.var_isublde_dn5 = (locals.var_mfactor * locals.var_isubld_dn5);
        locals.var_isublde_dn6 = (locals.var_mfactor * locals.var_isubld_dn6);
        locals.var_isublde_dn7 = (locals.var_mfactor * locals.var_isubld_dn7);
        locals.var_isublde_dn8 = (locals.var_mfactor * locals.var_isubld_dn8);
        locals.var_isublde_dn9 = (locals.var_mfactor * locals.var_isubld_dn9);
        locals.var_isublde_dn10 = (locals.var_mfactor * locals.var_isubld_dn10);
        locals.var_isublde_dn13 = (locals.var_mfactor * locals.var_isubld_dn13);
        locals.var_isublde_rv = 0.0;

        let assign102290_e154202: f64 = (4.0 * 1.3806226e-23);
        let assign102290_e154204: f64 = (assign102290_e154202 * locals.var_ttemp);
        let assign102290_e154206: f64 = assign102290_e154204;
        locals.var_whi_noise = assign102290_e154206;
        locals.var_whi_noise_dn0 = (assign102290_e154202 * locals.var_ttemp_dn0);
        locals.var_whi_noise_dn2 = (assign102290_e154202 * locals.var_ttemp_dn2);
        locals.var_whi_noise_dn4 = (assign102290_e154202 * locals.var_ttemp_dn4);
        locals.var_whi_noise_dn5 = (assign102290_e154202 * locals.var_ttemp_dn5);
        locals.var_whi_noise_dn6 = (assign102290_e154202 * locals.var_ttemp_dn6);
        locals.var_whi_noise_dn7 = (assign102290_e154202 * locals.var_ttemp_dn7);
        locals.var_whi_noise_dn8 = (assign102290_e154202 * locals.var_ttemp_dn8);
        locals.var_whi_noise_dn9 = (assign102290_e154202 * locals.var_ttemp_dn9);
        locals.var_whi_noise_dn10 = (assign102290_e154202 * locals.var_ttemp_dn10);
        locals.var_whi_noise_dn13 = (assign102290_e154202 * locals.var_ttemp_dn13);
        locals.var_whi_noise_rv = 0.0;

        let assign102310_e154212: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign102310_e154212;
        locals.var_noithrml_dn0 = (locals.var_mfactor * locals.var_nthrml_dn0);
        locals.var_noithrml_dn2 = (locals.var_mfactor * locals.var_nthrml_dn2);
        locals.var_noithrml_dn4 = (locals.var_mfactor * locals.var_nthrml_dn4);
        locals.var_noithrml_dn5 = (locals.var_mfactor * locals.var_nthrml_dn5);
        locals.var_noithrml_dn6 = (locals.var_mfactor * locals.var_nthrml_dn6);
        locals.var_noithrml_dn7 = (locals.var_mfactor * locals.var_nthrml_dn7);
        locals.var_noithrml_dn8 = (locals.var_mfactor * locals.var_nthrml_dn8);
        locals.var_noithrml_dn9 = (locals.var_mfactor * locals.var_nthrml_dn9);
        locals.var_noithrml_dn10 = (locals.var_mfactor * locals.var_nthrml_dn10);
        locals.var_noithrml_dn13 = (locals.var_mfactor * locals.var_nthrml_dn13);
        locals.var_noithrml_rv = 0.0;

        let assign102320_e154215: f64 = locals.var_qge_dn5;
        locals.var_cgdbd = assign102320_e154215;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn4 = 0.0;
        locals.var_cgdbd_dn5 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn8 = 0.0;
        locals.var_cgdbd_dn9 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn13 = 0.0;
        locals.var_cgdbd_rv = 0.0;

        let assign102330_e154218: f64 = (p.p87 * locals.var_cgdbd);
        locals.var_cgdbd = assign102330_e154218;
        locals.var_cgdbd_dn0 = (p.p87 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p87 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn4 = (p.p87 * locals.var_cgdbd_dn4);
        locals.var_cgdbd_dn5 = (p.p87 * locals.var_cgdbd_dn5);
        locals.var_cgdbd_dn6 = (p.p87 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p87 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn8 = (p.p87 * locals.var_cgdbd_dn8);
        locals.var_cgdbd_dn9 = (p.p87 * locals.var_cgdbd_dn9);
        locals.var_cgdbd_dn10 = (p.p87 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn13 = (p.p87 * locals.var_cgdbd_dn13);
        locals.var_cgdbd_rv = 0.0;

        let assign102340_e154221: f64 = locals.var_qge_dn7;
        locals.var_cgsbd = assign102340_e154221;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn4 = 0.0;
        locals.var_cgsbd_dn5 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn8 = 0.0;
        locals.var_cgsbd_dn9 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn13 = 0.0;
        locals.var_cgsbd_rv = 0.0;

        let assign102350_e154224: f64 = (p.p87 * locals.var_cgsbd);
        locals.var_cgsbd = assign102350_e154224;
        locals.var_cgsbd_dn0 = (p.p87 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p87 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn4 = (p.p87 * locals.var_cgsbd_dn4);
        locals.var_cgsbd_dn5 = (p.p87 * locals.var_cgsbd_dn5);
        locals.var_cgsbd_dn6 = (p.p87 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p87 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn8 = (p.p87 * locals.var_cgsbd_dn8);
        locals.var_cgsbd_dn9 = (p.p87 * locals.var_cgsbd_dn9);
        locals.var_cgsbd_dn10 = (p.p87 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn13 = (p.p87 * locals.var_cgsbd_dn13);
        locals.var_cgsbd_rv = 0.0;

        let (assign102360_e154230, assign102360_e154230_d_n0, assign102360_e154230_d_n2, assign102360_e154230_d_n4, assign102360_e154230_d_n5, assign102360_e154230_d_n6, assign102360_e154230_d_n7, assign102360_e154230_d_n8, assign102360_e154230_d_n9, assign102360_e154230_d_n10, assign102360_e154230_d_n13,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn13,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn13,)
    }
};
        locals.var_cgsb = assign102360_e154230;
        locals.var_cgsb_dn0 = assign102360_e154230_d_n0;
        locals.var_cgsb_dn2 = assign102360_e154230_d_n2;
        locals.var_cgsb_dn4 = assign102360_e154230_d_n4;
        locals.var_cgsb_dn5 = assign102360_e154230_d_n5;
        locals.var_cgsb_dn6 = assign102360_e154230_d_n6;
        locals.var_cgsb_dn7 = assign102360_e154230_d_n7;
        locals.var_cgsb_dn8 = assign102360_e154230_d_n8;
        locals.var_cgsb_dn9 = assign102360_e154230_d_n9;
        locals.var_cgsb_dn10 = assign102360_e154230_d_n10;
        locals.var_cgsb_dn13 = assign102360_e154230_d_n13;
        locals.var_cgsb_rv = 0.0;

        locals.var_noiigate = 0.0;
        locals.var_noiigate_dn0 = 0.0;
        locals.var_noiigate_dn2 = 0.0;
        locals.var_noiigate_dn4 = 0.0;
        locals.var_noiigate_dn5 = 0.0;
        locals.var_noiigate_dn6 = 0.0;
        locals.var_noiigate_dn7 = 0.0;
        locals.var_noiigate_dn8 = 0.0;
        locals.var_noiigate_dn9 = 0.0;
        locals.var_noiigate_dn10 = 0.0;
        locals.var_noiigate_dn13 = 0.0;
        locals.var_noiigate_rv = 0.0;

        let assign102390_e154250: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2334 = assign102390_e154250;
        locals.var_guard2334_rv = 0.0;

        let (assign102400_e154260, assign102400_e154260_d_n0, assign102400_e154260_d_n2, assign102400_e154260_d_n4, assign102400_e154260_d_n5, assign102400_e154260_d_n6, assign102400_e154260_d_n7, assign102400_e154260_d_n8, assign102400_e154260_d_n9, assign102400_e154260_d_n10, assign102400_e154260_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let assign102400_e154254: f64 = (1e-6 * locals.var_cox);
        let assign102400_e154256: f64 = (assign102400_e154254 * locals.var_weffcv_nf);
        let assign102400_e154258: f64 = (assign102400_e154256 * locals.var_leff);
        (assign102400_e154258, (((1e-6 * locals.var_cox_dn0) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn2) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn4) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn5) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn6) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn7) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn8) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn9) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn10) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn13) * locals.var_weffcv_nf) * locals.var_leff),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign102400_e154260;
        locals.var_t0_dn0 = assign102400_e154260_d_n0;
        locals.var_t0_dn2 = assign102400_e154260_d_n2;
        locals.var_t0_dn4 = assign102400_e154260_d_n4;
        locals.var_t0_dn5 = assign102400_e154260_d_n5;
        locals.var_t0_dn6 = assign102400_e154260_d_n6;
        locals.var_t0_dn7 = assign102400_e154260_d_n7;
        locals.var_t0_dn8 = assign102400_e154260_d_n8;
        locals.var_t0_dn9 = assign102400_e154260_d_n9;
        locals.var_t0_dn10 = assign102400_e154260_d_n10;
        locals.var_t0_dn13 = assign102400_e154260_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign102410_e154266, assign102410_e154266_d_n0, assign102410_e154266_d_n2, assign102410_e154266_d_n4, assign102410_e154266_d_n5, assign102410_e154266_d_n6, assign102410_e154266_d_n7, assign102410_e154266_d_n8, assign102410_e154266_d_n9, assign102410_e154266_d_n10, assign102410_e154266_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let assign102410_e154264: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign102410_e154264, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn4 / locals.var_mfactor), (locals.var_cgsb_dn5 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn8 / locals.var_mfactor), (locals.var_cgsb_dn9 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn13 / locals.var_mfactor),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign102410_e154266;
        locals.var_t10_dn0 = assign102410_e154266_d_n0;
        locals.var_t10_dn2 = assign102410_e154266_d_n2;
        locals.var_t10_dn4 = assign102410_e154266_d_n4;
        locals.var_t10_dn5 = assign102410_e154266_d_n5;
        locals.var_t10_dn6 = assign102410_e154266_d_n6;
        locals.var_t10_dn7 = assign102410_e154266_d_n7;
        locals.var_t10_dn8 = assign102410_e154266_d_n8;
        locals.var_t10_dn9 = assign102410_e154266_d_n9;
        locals.var_t10_dn10 = assign102410_e154266_d_n10;
        locals.var_t10_dn13 = assign102410_e154266_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign102420_e154280, assign102420_e154280_d_n0, assign102420_e154280_d_n2, assign102420_e154280_d_n4, assign102420_e154280_d_n5, assign102420_e154280_d_n6, assign102420_e154280_d_n7, assign102420_e154280_d_n8, assign102420_e154280_d_n9, assign102420_e154280_d_n10, assign102420_e154280_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let assign102420_e154270: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign102420_e154272: f64 = (assign102420_e154270 * locals.var_beta_inv);
        let assign102420_e154274: f64 = (assign102420_e154272 * locals.var_t10);
        let assign102420_e154276: f64 = (assign102420_e154274 * locals.var_t10);
        let assign102420_e154278: f64 = (assign102420_e154276 / locals.var_gds0_ign);
        (assign102420_e154278, ((((((((assign102420_e154270 * locals.var_beta_inv_dn0) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn0)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn0)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn2) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn2)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn2)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn4) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn4)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn4)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn4)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn5) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn5)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn5)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn5)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn6) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn6)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn6)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn7) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn7)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn7)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn8) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn8)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn8)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn8)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn9) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn9)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn9)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn9)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn10) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn10)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn10)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn13) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn13)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn13)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn13)) / (locals.var_gds0_ign * locals.var_gds0_ign)),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn4, locals.var_nign0_dn5, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn8, locals.var_nign0_dn9, locals.var_nign0_dn10, locals.var_nign0_dn13,)
    }
};
        locals.var_nign0 = assign102420_e154280;
        locals.var_nign0_dn0 = assign102420_e154280_d_n0;
        locals.var_nign0_dn2 = assign102420_e154280_d_n2;
        locals.var_nign0_dn4 = assign102420_e154280_d_n4;
        locals.var_nign0_dn5 = assign102420_e154280_d_n5;
        locals.var_nign0_dn6 = assign102420_e154280_d_n6;
        locals.var_nign0_dn7 = assign102420_e154280_d_n7;
        locals.var_nign0_dn8 = assign102420_e154280_d_n8;
        locals.var_nign0_dn9 = assign102420_e154280_d_n9;
        locals.var_nign0_dn10 = assign102420_e154280_d_n10;
        locals.var_nign0_dn13 = assign102420_e154280_d_n13;
        locals.var_nign0_rv = 0.0;

        let assign102430_e154284: f64 = (10.0 * 2.220446049250313e-16);
        let assign102430_e154289: f64 = (10.0 * 2.220446049250313e-16);
        let assign102430_e154291: f64 = if ((locals.var_kusai00l > assign102430_e154284) && (locals.var_vds > assign102430_e154289)) { 1.0 } else { 0.0 };
        locals.var_guard2335 = assign102430_e154291;
        locals.var_guard2335_rv = 0.0;

        let (assign102440_e154299, assign102440_e154299_d_n0, assign102440_e154299_d_n2, assign102440_e154299_d_n4, assign102440_e154299_d_n5, assign102440_e154299_d_n6, assign102440_e154299_d_n7, assign102440_e154299_d_n8, assign102440_e154299_d_n9, assign102440_e154299_d_n10, assign102440_e154299_d_n13,) = {
    if ((locals.var_guard2334 != 0.0) && (locals.var_guard2335 != 0.0)) {
        let assign102440_e154297: f64 = (locals.var_muun / locals.var_mu);
        (assign102440_e154297, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn4 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn4)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn5 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn5)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn8 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn8)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn9 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn9)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn13 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn13)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn4, locals.var_mumoda_dn5, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn8, locals.var_mumoda_dn9, locals.var_mumoda_dn10, locals.var_mumoda_dn13,)
    }
};
        locals.var_mumoda = assign102440_e154299;
        locals.var_mumoda_dn0 = assign102440_e154299_d_n0;
        locals.var_mumoda_dn2 = assign102440_e154299_d_n2;
        locals.var_mumoda_dn4 = assign102440_e154299_d_n4;
        locals.var_mumoda_dn5 = assign102440_e154299_d_n5;
        locals.var_mumoda_dn6 = assign102440_e154299_d_n6;
        locals.var_mumoda_dn7 = assign102440_e154299_d_n7;
        locals.var_mumoda_dn8 = assign102440_e154299_d_n8;
        locals.var_mumoda_dn9 = assign102440_e154299_d_n9;
        locals.var_mumoda_dn10 = assign102440_e154299_d_n10;
        locals.var_mumoda_dn13 = assign102440_e154299_d_n13;
        locals.var_mumoda_rv = 0.0;

        let (assign102450_e154311, assign102450_e154311_d_n0, assign102450_e154311_d_n2, assign102450_e154311_d_n4, assign102450_e154311_d_n5, assign102450_e154311_d_n6, assign102450_e154311_d_n7, assign102450_e154311_d_n8, assign102450_e154311_d_n9, assign102450_e154311_d_n10, assign102450_e154311_d_n13,) = {
    if ((locals.var_guard2334 != 0.0) && (locals.var_guard2335 != 0.0)) {
        let assign102450_e154305: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign102450_e154307: f64 = (assign102450_e154305 - locals.var_mumoda);
        let assign102450_e154309: f64 = (assign102450_e154307 / locals.var_vds);
        (assign102450_e154309, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn4) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn4)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn5) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn5)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn8) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn8)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn9) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn9)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn13 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn13)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn13) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn13)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn4, locals.var_mumodb_dn5, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn8, locals.var_mumodb_dn9, locals.var_mumodb_dn10, locals.var_mumodb_dn13,)
    }
};
        locals.var_mumodb = assign102450_e154311;
        locals.var_mumodb_dn0 = assign102450_e154311_d_n0;
        locals.var_mumodb_dn2 = assign102450_e154311_d_n2;
        locals.var_mumodb_dn4 = assign102450_e154311_d_n4;
        locals.var_mumodb_dn5 = assign102450_e154311_d_n5;
        locals.var_mumodb_dn6 = assign102450_e154311_d_n6;
        locals.var_mumodb_dn7 = assign102450_e154311_d_n7;
        locals.var_mumodb_dn8 = assign102450_e154311_d_n8;
        locals.var_mumodb_dn9 = assign102450_e154311_d_n9;
        locals.var_mumodb_dn10 = assign102450_e154311_d_n10;
        locals.var_mumodb_dn13 = assign102450_e154311_d_n13;
        locals.var_mumodb_rv = 0.0;

        let (assign102460_e154333, assign102460_e154333_d_n0, assign102460_e154333_d_n2, assign102460_e154333_d_n4, assign102460_e154333_d_n5, assign102460_e154333_d_n6, assign102460_e154333_d_n7, assign102460_e154333_d_n8, assign102460_e154333_d_n9, assign102460_e154333_d_n10, assign102460_e154333_d_n13,) = {
    if ((locals.var_guard2334 != 0.0) && (locals.var_guard2335 != 0.0)) {
        let assign102460_e154318: f64 = (0.6666666666666667 * locals.var_mumodb);
        let assign102460_e154322: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
        let assign102460_e154323: f64 = (locals.var_kusai00 + assign102460_e154322);
        let assign102460_e154325: f64 = (assign102460_e154323 + locals.var_kusail);
        let assign102460_e154326: f64 = (assign102460_e154318 * assign102460_e154325);
        let assign102460_e154329: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        let assign102460_e154330: f64 = (assign102460_e154326 / assign102460_e154329);
        let assign102460_e154331: f64 = (locals.var_mumoda + assign102460_e154330);
        (assign102460_e154331, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn4 + ((((((0.6666666666666667 * locals.var_mumodb_dn4) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn4 + ((locals.var_vgvt_dn4 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn5 + ((((((0.6666666666666667 * locals.var_mumodb_dn5) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn5 + ((locals.var_vgvt_dn5 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn7 + ((((((0.6666666666666667 * locals.var_mumodb_dn7) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn7 + ((locals.var_vgvt_dn7 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn8 + ((((((0.6666666666666667 * locals.var_mumodb_dn8) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn8 + ((locals.var_vgvt_dn8 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn9 + ((((((0.6666666666666667 * locals.var_mumodb_dn9) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn9 + ((locals.var_vgvt_dn9 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn9))) + locals.var_kusail_dn9))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn9 + locals.var_sqrtkusail_dn9))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn13 + ((((((0.6666666666666667 * locals.var_mumodb_dn13) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn13 + ((locals.var_vgvt_dn13 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn13))) + locals.var_kusail_dn13))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn13 + locals.var_sqrtkusail_dn13))) / (assign102460_e154329 * assign102460_e154329))),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn8, locals.var_correct_w1_dn9, locals.var_correct_w1_dn10, locals.var_correct_w1_dn13,)
    }
};
        locals.var_correct_w1 = assign102460_e154333;
        locals.var_correct_w1_dn0 = assign102460_e154333_d_n0;
        locals.var_correct_w1_dn2 = assign102460_e154333_d_n2;
        locals.var_correct_w1_dn4 = assign102460_e154333_d_n4;
        locals.var_correct_w1_dn5 = assign102460_e154333_d_n5;
        locals.var_correct_w1_dn6 = assign102460_e154333_d_n6;
        locals.var_correct_w1_dn7 = assign102460_e154333_d_n7;
        locals.var_correct_w1_dn8 = assign102460_e154333_d_n8;
        locals.var_correct_w1_dn9 = assign102460_e154333_d_n9;
        locals.var_correct_w1_dn10 = assign102460_e154333_d_n10;
        locals.var_correct_w1_dn13 = assign102460_e154333_d_n13;
        locals.var_correct_w1_rv = 0.0;

        let (assign102470_e154342, assign102470_e154342_d_n0, assign102470_e154342_d_n2, assign102470_e154342_d_n4, assign102470_e154342_d_n5, assign102470_e154342_d_n6, assign102470_e154342_d_n7, assign102470_e154342_d_n8, assign102470_e154342_d_n9, assign102470_e154342_d_n10, assign102470_e154342_d_n13,) = {
    if ((locals.var_guard2334 != 0.0) && (locals.var_guard2335 == 0.0)) {
        let assign102470_e154340: f64 = (locals.var_muun / locals.var_mud_hoso);
        (assign102470_e154340, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn13 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn13)) / (locals.var_mud_hoso * locals.var_mud_hoso)),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn8, locals.var_correct_w1_dn9, locals.var_correct_w1_dn10, locals.var_correct_w1_dn13,)
    }
};
        locals.var_correct_w1 = assign102470_e154342;
        locals.var_correct_w1_dn0 = assign102470_e154342_d_n0;
        locals.var_correct_w1_dn2 = assign102470_e154342_d_n2;
        locals.var_correct_w1_dn4 = assign102470_e154342_d_n4;
        locals.var_correct_w1_dn5 = assign102470_e154342_d_n5;
        locals.var_correct_w1_dn6 = assign102470_e154342_d_n6;
        locals.var_correct_w1_dn7 = assign102470_e154342_d_n7;
        locals.var_correct_w1_dn8 = assign102470_e154342_d_n8;
        locals.var_correct_w1_dn9 = assign102470_e154342_d_n9;
        locals.var_correct_w1_dn10 = assign102470_e154342_d_n10;
        locals.var_correct_w1_dn13 = assign102470_e154342_d_n13;
        locals.var_correct_w1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_380(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign102480_e154352, assign102480_e154352_d_n0, assign102480_e154352_d_n2, assign102480_e154352_d_n4, assign102480_e154352_d_n5, assign102480_e154352_d_n6, assign102480_e154352_d_n7, assign102480_e154352_d_n8, assign102480_e154352_d_n9, assign102480_e154352_d_n10, assign102480_e154352_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let assign102480_e154346: f64 = (locals.var_mfactor * locals.var_nign0);
        let assign102480_e154348: f64 = (assign102480_e154346 * locals.var_kusai_ig);
        let assign102480_e154350: f64 = (assign102480_e154348 * locals.var_correct_w1);
        (assign102480_e154350, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn4) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn4)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn4)), (((((locals.var_mfactor * locals.var_nign0_dn5) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn5)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn5)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn7) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn7)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn7)), (((((locals.var_mfactor * locals.var_nign0_dn8) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn8)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn8)), (((((locals.var_mfactor * locals.var_nign0_dn9) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn9)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn9)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn13) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn13)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn13)),)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn13,)
    }
};
        locals.var_noiigate = assign102480_e154352;
        locals.var_noiigate_dn0 = assign102480_e154352_d_n0;
        locals.var_noiigate_dn2 = assign102480_e154352_d_n2;
        locals.var_noiigate_dn4 = assign102480_e154352_d_n4;
        locals.var_noiigate_dn5 = assign102480_e154352_d_n5;
        locals.var_noiigate_dn6 = assign102480_e154352_d_n6;
        locals.var_noiigate_dn7 = assign102480_e154352_d_n7;
        locals.var_noiigate_dn8 = assign102480_e154352_d_n8;
        locals.var_noiigate_dn9 = assign102480_e154352_d_n9;
        locals.var_noiigate_dn10 = assign102480_e154352_d_n10;
        locals.var_noiigate_dn13 = assign102480_e154352_d_n13;
        locals.var_noiigate_rv = 0.0;

        let (assign102500_e154365, assign102500_e154365_d_n0, assign102500_e154365_d_n2, assign102500_e154365_d_n4, assign102500_e154365_d_n5, assign102500_e154365_d_n6, assign102500_e154365_d_n7, assign102500_e154365_d_n8, assign102500_e154365_d_n9, assign102500_e154365_d_n10, assign102500_e154365_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let (assign102500_e154363, assign102500_e154363_d_n0, assign102500_e154363_d_n2, assign102500_e154363_d_n4, assign102500_e154363_d_n5, assign102500_e154363_d_n6, assign102500_e154363_d_n7, assign102500_e154363_d_n8, assign102500_e154363_d_n9, assign102500_e154363_d_n10, assign102500_e154363_d_n13,) = {
            if (locals.var_noiigate < 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn13,)
            }
        };
        (assign102500_e154363, assign102500_e154363_d_n0, assign102500_e154363_d_n2, assign102500_e154363_d_n4, assign102500_e154363_d_n5, assign102500_e154363_d_n6, assign102500_e154363_d_n7, assign102500_e154363_d_n8, assign102500_e154363_d_n9, assign102500_e154363_d_n10, assign102500_e154363_d_n13,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn13,)
    }
};
        locals.var_noiigate = assign102500_e154365;
        locals.var_noiigate_dn0 = assign102500_e154365_d_n0;
        locals.var_noiigate_dn2 = assign102500_e154365_d_n2;
        locals.var_noiigate_dn4 = assign102500_e154365_d_n4;
        locals.var_noiigate_dn5 = assign102500_e154365_d_n5;
        locals.var_noiigate_dn6 = assign102500_e154365_d_n6;
        locals.var_noiigate_dn7 = assign102500_e154365_d_n7;
        locals.var_noiigate_dn8 = assign102500_e154365_d_n8;
        locals.var_noiigate_dn9 = assign102500_e154365_d_n9;
        locals.var_noiigate_dn10 = assign102500_e154365_d_n10;
        locals.var_noiigate_dn13 = assign102500_e154365_d_n13;
        locals.var_noiigate_rv = 0.0;

        let (assign102510_e154375, assign102510_e154375_d_n0, assign102510_e154375_d_n2, assign102510_e154375_d_n4, assign102510_e154375_d_n5, assign102510_e154375_d_n6, assign102510_e154375_d_n7, assign102510_e154375_d_n8, assign102510_e154375_d_n9, assign102510_e154375_d_n10, assign102510_e154375_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let assign102510_e154368: f64 = (-locals.var_t10);
        let (assign102510_e154373, assign102510_e154373_d_n0, assign102510_e154373_d_n2, assign102510_e154373_d_n4, assign102510_e154373_d_n5, assign102510_e154373_d_n6, assign102510_e154373_d_n7, assign102510_e154373_d_n8, assign102510_e154373_d_n9, assign102510_e154373_d_n10, assign102510_e154373_d_n13,) = {
            if (assign102510_e154368 > locals.var_t0) {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign102510_e154373, assign102510_e154373_d_n0, assign102510_e154373_d_n2, assign102510_e154373_d_n4, assign102510_e154373_d_n5, assign102510_e154373_d_n6, assign102510_e154373_d_n7, assign102510_e154373_d_n8, assign102510_e154373_d_n9, assign102510_e154373_d_n10, assign102510_e154373_d_n13,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn13,)
    }
};
        locals.var_noiigate = assign102510_e154375;
        locals.var_noiigate_dn0 = assign102510_e154375_d_n0;
        locals.var_noiigate_dn2 = assign102510_e154375_d_n2;
        locals.var_noiigate_dn4 = assign102510_e154375_d_n4;
        locals.var_noiigate_dn5 = assign102510_e154375_d_n5;
        locals.var_noiigate_dn6 = assign102510_e154375_d_n6;
        locals.var_noiigate_dn7 = assign102510_e154375_d_n7;
        locals.var_noiigate_dn8 = assign102510_e154375_d_n8;
        locals.var_noiigate_dn9 = assign102510_e154375_d_n9;
        locals.var_noiigate_dn10 = assign102510_e154375_d_n10;
        locals.var_noiigate_dn13 = assign102510_e154375_d_n13;
        locals.var_noiigate_rv = 0.0;

        let assign102530_e154388: f64 = (locals.var_whi_noise * locals.var_noithrml);
        locals.var_sid = assign102530_e154388;
        locals.var_sid_dn0 = ((locals.var_whi_noise_dn0 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn0));
        locals.var_sid_dn2 = ((locals.var_whi_noise_dn2 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn2));
        locals.var_sid_dn4 = ((locals.var_whi_noise_dn4 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn4));
        locals.var_sid_dn5 = ((locals.var_whi_noise_dn5 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn5));
        locals.var_sid_dn6 = ((locals.var_whi_noise_dn6 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn6));
        locals.var_sid_dn7 = ((locals.var_whi_noise_dn7 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn7));
        locals.var_sid_dn8 = ((locals.var_whi_noise_dn8 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn8));
        locals.var_sid_dn9 = ((locals.var_whi_noise_dn9 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn9));
        locals.var_sid_dn10 = ((locals.var_whi_noise_dn10 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn10));
        locals.var_sid_dn13 = ((locals.var_whi_noise_dn13 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn13));
        locals.var_sid_rv = 0.0;

        let (assign102550_e154402, assign102550_e154402_d_n0, assign102550_e154402_d_n2, assign102550_e154402_d_n4, assign102550_e154402_d_n5, assign102550_e154402_d_n6, assign102550_e154402_d_n7, assign102550_e154402_d_n8, assign102550_e154402_d_n9, assign102550_e154402_d_n10, assign102550_e154402_d_n13,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign102550_e154399: f64 = (locals.var_noiigate / locals.var_sid);
        let assign102550_e154400: f64 = (assign102550_e154399).sqrt();
        (assign102550_e154400, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn4 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn4)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn5 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn5)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn8 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn8)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn9 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn9)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn13 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn13)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_sigrat = assign102550_e154402;
        locals.var_sigrat_dn0 = assign102550_e154402_d_n0;
        locals.var_sigrat_dn2 = assign102550_e154402_d_n2;
        locals.var_sigrat_dn4 = assign102550_e154402_d_n4;
        locals.var_sigrat_dn5 = assign102550_e154402_d_n5;
        locals.var_sigrat_dn6 = assign102550_e154402_d_n6;
        locals.var_sigrat_dn7 = assign102550_e154402_d_n7;
        locals.var_sigrat_dn8 = assign102550_e154402_d_n8;
        locals.var_sigrat_dn9 = assign102550_e154402_d_n9;
        locals.var_sigrat_dn10 = assign102550_e154402_d_n10;
        locals.var_sigrat_dn13 = assign102550_e154402_d_n13;
        locals.var_sigrat_rv = 0.0;

        let (assign102560_e154414, assign102560_e154414_d_n0, assign102560_e154414_d_n2, assign102560_e154414_d_n4, assign102560_e154414_d_n5, assign102560_e154414_d_n6, assign102560_e154414_d_n7, assign102560_e154414_d_n8, assign102560_e154414_d_n9, assign102560_e154414_d_n10, assign102560_e154414_d_n13,) = {
    if (locals.var_mode > 0.0) {
        let assign102560_e154409: f64 = (1.0 - locals.var_qdrat);
        let assign102560_e154410: f64 = (locals.var_sigrat * assign102560_e154409);
        (assign102560_e154410, ((locals.var_sigrat_dn0 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn4 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn4))), ((locals.var_sigrat_dn5 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn5))), ((locals.var_sigrat_dn6 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn8 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn8))), ((locals.var_sigrat_dn9 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn9))), ((locals.var_sigrat_dn10 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn13 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn13))),)
    } else {
        let assign102560_e154413: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign102560_e154413, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn4 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn4)), ((locals.var_sigrat_dn5 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn5)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn8 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn8)), ((locals.var_sigrat_dn9 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn9)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn13 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn13)),)
    }
};
        locals.var_sigrat_s = assign102560_e154414;
        locals.var_sigrat_s_dn0 = assign102560_e154414_d_n0;
        locals.var_sigrat_s_dn2 = assign102560_e154414_d_n2;
        locals.var_sigrat_s_dn4 = assign102560_e154414_d_n4;
        locals.var_sigrat_s_dn5 = assign102560_e154414_d_n5;
        locals.var_sigrat_s_dn6 = assign102560_e154414_d_n6;
        locals.var_sigrat_s_dn7 = assign102560_e154414_d_n7;
        locals.var_sigrat_s_dn8 = assign102560_e154414_d_n8;
        locals.var_sigrat_s_dn9 = assign102560_e154414_d_n9;
        locals.var_sigrat_s_dn10 = assign102560_e154414_d_n10;
        locals.var_sigrat_s_dn13 = assign102560_e154414_d_n13;
        locals.var_sigrat_s_rv = 0.0;

        let (assign102570_e154426, assign102570_e154426_d_n0, assign102570_e154426_d_n2, assign102570_e154426_d_n4, assign102570_e154426_d_n5, assign102570_e154426_d_n6, assign102570_e154426_d_n7, assign102570_e154426_d_n8, assign102570_e154426_d_n9, assign102570_e154426_d_n10, assign102570_e154426_d_n13,) = {
    if (locals.var_mode > 0.0) {
        let assign102570_e154420: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign102570_e154420, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn4 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn4)), ((locals.var_sigrat_dn5 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn5)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn8 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn8)), ((locals.var_sigrat_dn9 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn9)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn13 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn13)),)
    } else {
        let assign102570_e154424: f64 = (1.0 - locals.var_qdrat);
        let assign102570_e154425: f64 = (locals.var_sigrat * assign102570_e154424);
        (assign102570_e154425, ((locals.var_sigrat_dn0 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn4 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn4))), ((locals.var_sigrat_dn5 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn5))), ((locals.var_sigrat_dn6 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn8 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn8))), ((locals.var_sigrat_dn9 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn9))), ((locals.var_sigrat_dn10 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn13 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn13))),)
    }
};
        locals.var_sigrat_d = assign102570_e154426;
        locals.var_sigrat_d_dn0 = assign102570_e154426_d_n0;
        locals.var_sigrat_d_dn2 = assign102570_e154426_d_n2;
        locals.var_sigrat_d_dn4 = assign102570_e154426_d_n4;
        locals.var_sigrat_d_dn5 = assign102570_e154426_d_n5;
        locals.var_sigrat_d_dn6 = assign102570_e154426_d_n6;
        locals.var_sigrat_d_dn7 = assign102570_e154426_d_n7;
        locals.var_sigrat_d_dn8 = assign102570_e154426_d_n8;
        locals.var_sigrat_d_dn9 = assign102570_e154426_d_n9;
        locals.var_sigrat_d_dn10 = assign102570_e154426_d_n10;
        locals.var_sigrat_d_dn13 = assign102570_e154426_d_n13;
        locals.var_sigrat_d_rv = 0.0;

        locals.var_rsde = 0.0;
        locals.var_rsde_dn0 = 0.0;
        locals.var_rsde_dn2 = 0.0;
        locals.var_rsde_dn4 = 0.0;
        locals.var_rsde_dn5 = 0.0;
        locals.var_rsde_dn6 = 0.0;
        locals.var_rsde_dn7 = 0.0;
        locals.var_rsde_dn8 = 0.0;
        locals.var_rsde_dn9 = 0.0;
        locals.var_rsde_dn10 = 0.0;
        locals.var_rsde_dn13 = 0.0;
        locals.var_rsde_rv = 0.0;

        locals.var_rdde = 0.0;
        locals.var_rdde_dn0 = 0.0;
        locals.var_rdde_dn2 = 0.0;
        locals.var_rdde_dn4 = 0.0;
        locals.var_rdde_dn5 = 0.0;
        locals.var_rdde_dn6 = 0.0;
        locals.var_rdde_dn7 = 0.0;
        locals.var_rdde_dn8 = 0.0;
        locals.var_rdde_dn9 = 0.0;
        locals.var_rdde_dn10 = 0.0;
        locals.var_rdde_dn13 = 0.0;
        locals.var_rdde_rv = 0.0;

        let assign102600_e154431: f64 = if locals.var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2336 = assign102600_e154431;
        locals.var_guard2336_rv = 0.0;

        let assign102610_e154434: f64 = if locals.var_flg_rs == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2337 = assign102610_e154434;
        locals.var_guard2337_rv = 0.0;

        let assign102620_e154441: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2338 = assign102620_e154441;
        locals.var_guard2338_rv = 0.0;

        let (assign102630_e154457, assign102630_e154457_d_n0, assign102630_e154457_d_n2, assign102630_e154457_d_n4, assign102630_e154457_d_n5, assign102630_e154457_d_n6, assign102630_e154457_d_n7, assign102630_e154457_d_n8, assign102630_e154457_d_n9, assign102630_e154457_d_n10, assign102630_e154457_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 != 0.0)) {
        let (assign102630_e154455, assign102630_e154455_d_n0, assign102630_e154455_d_n2, assign102630_e154455_d_n4, assign102630_e154455_d_n5, assign102630_e154455_d_n6, assign102630_e154455_d_n7, assign102630_e154455_d_n8, assign102630_e154455_d_n9, assign102630_e154455_d_n10, assign102630_e154455_d_n13,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign102630_e154454: f64 = (locals.var_tratio).powf(p.p416);
                (assign102630_e154454, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn0)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn2)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn4)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn5)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn6)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn7)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn8)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn9)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn10)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn13)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn13 / locals.var_tratio))) },)
            }
        };
        (assign102630_e154455, assign102630_e154455_d_n0, assign102630_e154455_d_n2, assign102630_e154455_d_n4, assign102630_e154455_d_n5, assign102630_e154455_d_n6, assign102630_e154455_d_n7, assign102630_e154455_d_n8, assign102630_e154455_d_n9, assign102630_e154455_d_n10, assign102630_e154455_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign102630_e154457;
        locals.var_t1_dn0 = assign102630_e154457_d_n0;
        locals.var_t1_dn2 = assign102630_e154457_d_n2;
        locals.var_t1_dn4 = assign102630_e154457_d_n4;
        locals.var_t1_dn5 = assign102630_e154457_d_n5;
        locals.var_t1_dn6 = assign102630_e154457_d_n6;
        locals.var_t1_dn7 = assign102630_e154457_d_n7;
        locals.var_t1_dn8 = assign102630_e154457_d_n8;
        locals.var_t1_dn9 = assign102630_e154457_d_n9;
        locals.var_t1_dn10 = assign102630_e154457_d_n10;
        locals.var_t1_dn13 = assign102630_e154457_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign102640_e154468, assign102640_e154468_d_n0, assign102640_e154468_d_n2, assign102640_e154468_d_n4, assign102640_e154468_d_n5, assign102640_e154468_d_n6, assign102640_e154468_d_n7, assign102640_e154468_d_n8, assign102640_e154468_d_n9, assign102640_e154468_d_n10, assign102640_e154468_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 != 0.0)) {
        let assign102640_e154466: f64 = (locals.var_mks_rdrmues / locals.var_t1);
        (assign102640_e154466, (-((locals.var_mks_rdrmues * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmues, locals.var_rrdrmues_dn0, locals.var_rrdrmues_dn2, locals.var_rrdrmues_dn4, locals.var_rrdrmues_dn5, locals.var_rrdrmues_dn6, locals.var_rrdrmues_dn7, locals.var_rrdrmues_dn8, locals.var_rrdrmues_dn9, locals.var_rrdrmues_dn10, locals.var_rrdrmues_dn13,)
    }
};
        locals.var_rrdrmues = assign102640_e154468;
        locals.var_rrdrmues_dn0 = assign102640_e154468_d_n0;
        locals.var_rrdrmues_dn2 = assign102640_e154468_d_n2;
        locals.var_rrdrmues_dn4 = assign102640_e154468_d_n4;
        locals.var_rrdrmues_dn5 = assign102640_e154468_d_n5;
        locals.var_rrdrmues_dn6 = assign102640_e154468_d_n6;
        locals.var_rrdrmues_dn7 = assign102640_e154468_d_n7;
        locals.var_rrdrmues_dn8 = assign102640_e154468_d_n8;
        locals.var_rrdrmues_dn9 = assign102640_e154468_d_n9;
        locals.var_rrdrmues_dn10 = assign102640_e154468_d_n10;
        locals.var_rrdrmues_dn13 = assign102640_e154468_d_n13;
        locals.var_rrdrmues_rv = 0.0;

        let (assign102650_e154493, assign102650_e154493_d_n0, assign102650_e154493_d_n2, assign102650_e154493_d_n4, assign102650_e154493_d_n5, assign102650_e154493_d_n6, assign102650_e154493_d_n7, assign102650_e154493_d_n8, assign102650_e154493_d_n9, assign102650_e154493_d_n10, assign102650_e154493_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 != 0.0)) {
        let assign102650_e154478: f64 = (0.4 * locals.var_tratio);
        let assign102650_e154479: f64 = (1.8 + assign102650_e154478);
        let assign102650_e154482: f64 = (0.1 * locals.var_tratio);
        let assign102650_e154484: f64 = (assign102650_e154482 * locals.var_tratio);
        let assign102650_e154485: f64 = (assign102650_e154479 + assign102650_e154484);
        let assign102650_e154489: f64 = (1.0 - locals.var_tratio);
        let assign102650_e154490: f64 = (p.p418 * assign102650_e154489);
        let assign102650_e154491: f64 = (assign102650_e154485 - assign102650_e154490);
        (assign102650_e154491, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn0))) - (p.p418 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn2))) - (p.p418 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn4))) - (p.p418 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn5))) - (p.p418 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn6))) - (p.p418 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn7))) - (p.p418 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn8))) - (p.p418 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn9))) - (p.p418 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn10))) - (p.p418 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn13))) - (p.p418 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign102650_e154493;
        locals.var_t0_dn0 = assign102650_e154493_d_n0;
        locals.var_t0_dn2 = assign102650_e154493_d_n2;
        locals.var_t0_dn4 = assign102650_e154493_d_n4;
        locals.var_t0_dn5 = assign102650_e154493_d_n5;
        locals.var_t0_dn6 = assign102650_e154493_d_n6;
        locals.var_t0_dn7 = assign102650_e154493_d_n7;
        locals.var_t0_dn8 = assign102650_e154493_d_n8;
        locals.var_t0_dn9 = assign102650_e154493_d_n9;
        locals.var_t0_dn10 = assign102650_e154493_d_n10;
        locals.var_t0_dn13 = assign102650_e154493_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign102660_e154504, assign102660_e154504_d_n0, assign102660_e154504_d_n2, assign102660_e154504_d_n4, assign102660_e154504_d_n5, assign102660_e154504_d_n6, assign102660_e154504_d_n7, assign102660_e154504_d_n8, assign102660_e154504_d_n9, assign102660_e154504_d_n10, assign102660_e154504_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 != 0.0)) {
        let assign102660_e154502: f64 = (locals.var_mks_rdrvmaxs / locals.var_t0);
        (assign102660_e154502, (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmaxs, locals.var_rrdrvmaxs_dn0, locals.var_rrdrvmaxs_dn2, locals.var_rrdrvmaxs_dn4, locals.var_rrdrvmaxs_dn5, locals.var_rrdrvmaxs_dn6, locals.var_rrdrvmaxs_dn7, locals.var_rrdrvmaxs_dn8, locals.var_rrdrvmaxs_dn9, locals.var_rrdrvmaxs_dn10, locals.var_rrdrvmaxs_dn13,)
    }
};
        locals.var_rrdrvmaxs = assign102660_e154504;
        locals.var_rrdrvmaxs_dn0 = assign102660_e154504_d_n0;
        locals.var_rrdrvmaxs_dn2 = assign102660_e154504_d_n2;
        locals.var_rrdrvmaxs_dn4 = assign102660_e154504_d_n4;
        locals.var_rrdrvmaxs_dn5 = assign102660_e154504_d_n5;
        locals.var_rrdrvmaxs_dn6 = assign102660_e154504_d_n6;
        locals.var_rrdrvmaxs_dn7 = assign102660_e154504_d_n7;
        locals.var_rrdrvmaxs_dn8 = assign102660_e154504_d_n8;
        locals.var_rrdrvmaxs_dn9 = assign102660_e154504_d_n9;
        locals.var_rrdrvmaxs_dn10 = assign102660_e154504_d_n10;
        locals.var_rrdrvmaxs_dn13 = assign102660_e154504_d_n13;
        locals.var_rrdrvmaxs_rv = 0.0;

        let (assign102670_e154519, assign102670_e154519_d_n0, assign102670_e154519_d_n2, assign102670_e154519_d_n4, assign102670_e154519_d_n5, assign102670_e154519_d_n6, assign102670_e154519_d_n7, assign102670_e154519_d_n8, assign102670_e154519_d_n9, assign102670_e154519_d_n10, assign102670_e154519_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 != 0.0)) {
        let assign102670_e154515: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign102670_e154516: f64 = (p.p439 * assign102670_e154515);
        let assign102670_e154517: f64 = (locals.var_uc_rdrbb_s + assign102670_e154516);
        (assign102670_e154517, (locals.var_uc_rdrbb_s_dn0 + (p.p439 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_s_dn2 + (p.p439 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_s_dn4 + (p.p439 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_s_dn5 + (p.p439 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_s_dn6 + (p.p439 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_s_dn7 + (p.p439 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_s_dn8 + (p.p439 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_s_dn9 + (p.p439 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_s_dn10 + (p.p439 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_s_dn13 + (p.p439 * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_uc_rdrbb_s, locals.var_uc_rdrbb_s_dn0, locals.var_uc_rdrbb_s_dn2, locals.var_uc_rdrbb_s_dn4, locals.var_uc_rdrbb_s_dn5, locals.var_uc_rdrbb_s_dn6, locals.var_uc_rdrbb_s_dn7, locals.var_uc_rdrbb_s_dn8, locals.var_uc_rdrbb_s_dn9, locals.var_uc_rdrbb_s_dn10, locals.var_uc_rdrbb_s_dn13,)
    }
};
        locals.var_uc_rdrbb_s = assign102670_e154519;
        locals.var_uc_rdrbb_s_dn0 = assign102670_e154519_d_n0;
        locals.var_uc_rdrbb_s_dn2 = assign102670_e154519_d_n2;
        locals.var_uc_rdrbb_s_dn4 = assign102670_e154519_d_n4;
        locals.var_uc_rdrbb_s_dn5 = assign102670_e154519_d_n5;
        locals.var_uc_rdrbb_s_dn6 = assign102670_e154519_d_n6;
        locals.var_uc_rdrbb_s_dn7 = assign102670_e154519_d_n7;
        locals.var_uc_rdrbb_s_dn8 = assign102670_e154519_d_n8;
        locals.var_uc_rdrbb_s_dn9 = assign102670_e154519_d_n9;
        locals.var_uc_rdrbb_s_dn10 = assign102670_e154519_d_n10;
        locals.var_uc_rdrbb_s_dn13 = assign102670_e154519_d_n13;
        locals.var_uc_rdrbb_s_rv = 0.0;

        let (assign102680_e154531, assign102680_e154531_d_n0, assign102680_e154531_d_n2, assign102680_e154531_d_n4, assign102680_e154531_d_n5, assign102680_e154531_d_n6, assign102680_e154531_d_n7, assign102680_e154531_d_n8, assign102680_e154531_d_n9, assign102680_e154531_d_n10, assign102680_e154531_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 == 0.0)) {
        let assign102680_e154527: f64 = ctx_temp;
        let assign102680_e154529: f64 = (assign102680_e154527 + p.p11);
        (assign102680_e154529, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign102680_e154531;
        locals.var_ttemp_dn0 = assign102680_e154531_d_n0;
        locals.var_ttemp_dn2 = assign102680_e154531_d_n2;
        locals.var_ttemp_dn4 = assign102680_e154531_d_n4;
        locals.var_ttemp_dn5 = assign102680_e154531_d_n5;
        locals.var_ttemp_dn6 = assign102680_e154531_d_n6;
        locals.var_ttemp_dn7 = assign102680_e154531_d_n7;
        locals.var_ttemp_dn8 = assign102680_e154531_d_n8;
        locals.var_ttemp_dn9 = assign102680_e154531_d_n9;
        locals.var_ttemp_dn10 = assign102680_e154531_d_n10;
        locals.var_ttemp_dn13 = assign102680_e154531_d_n13;
        locals.var_ttemp_rv = 0.0;

        let (assign102690_e154540,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102690_e154538: f64 = (locals.var_weff_ld * p.p7);
        (assign102690_e154538,)
    } else {
        (locals.var_weffld_nf,)
    }
};
        locals.var_weffld_nf = assign102690_e154540;
        locals.var_weffld_nf_rv = 0.0;

        let (assign102700_e154547,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        (p.p71,)
    } else {
        (locals.var_ldrifte_s,)
    }
};
        locals.var_ldrifte_s = assign102700_e154547;
        locals.var_ldrifte_s_rv = 0.0;

        let (assign102710_e154554,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_novers,)
    }
};
        locals.var_novers = assign102710_e154554;
        locals.var_novers_rv = 0.0;

        let (assign102720_e154563, assign102720_e154563_d_n0, assign102720_e154563_d_n2, assign102720_e154563_d_n4, assign102720_e154563_d_n5, assign102720_e154563_d_n6, assign102720_e154563_d_n7, assign102720_e154563_d_n8, assign102720_e154563_d_n9, assign102720_e154563_d_n10, assign102720_e154563_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102720_e154561: f64 = (locals.var_rrdrmues * locals.var_rdrmuele);
        (assign102720_e154561, (locals.var_rrdrmues_dn0 * locals.var_rdrmuele), (locals.var_rrdrmues_dn2 * locals.var_rdrmuele), (locals.var_rrdrmues_dn4 * locals.var_rdrmuele), (locals.var_rrdrmues_dn5 * locals.var_rdrmuele), (locals.var_rrdrmues_dn6 * locals.var_rdrmuele), (locals.var_rrdrmues_dn7 * locals.var_rdrmuele), (locals.var_rrdrmues_dn8 * locals.var_rdrmuele), (locals.var_rrdrmues_dn9 * locals.var_rdrmuele), (locals.var_rrdrmues_dn10 * locals.var_rdrmuele), (locals.var_rrdrmues_dn13 * locals.var_rdrmuele),)
    } else {
        (locals.var_mu0_s, locals.var_mu0_s_dn0, locals.var_mu0_s_dn2, locals.var_mu0_s_dn4, locals.var_mu0_s_dn5, locals.var_mu0_s_dn6, locals.var_mu0_s_dn7, locals.var_mu0_s_dn8, locals.var_mu0_s_dn9, locals.var_mu0_s_dn10, locals.var_mu0_s_dn13,)
    }
};
        locals.var_mu0_s = assign102720_e154563;
        locals.var_mu0_s_dn0 = assign102720_e154563_d_n0;
        locals.var_mu0_s_dn2 = assign102720_e154563_d_n2;
        locals.var_mu0_s_dn4 = assign102720_e154563_d_n4;
        locals.var_mu0_s_dn5 = assign102720_e154563_d_n5;
        locals.var_mu0_s_dn6 = assign102720_e154563_d_n6;
        locals.var_mu0_s_dn7 = assign102720_e154563_d_n7;
        locals.var_mu0_s_dn8 = assign102720_e154563_d_n8;
        locals.var_mu0_s_dn9 = assign102720_e154563_d_n9;
        locals.var_mu0_s_dn10 = assign102720_e154563_d_n10;
        locals.var_mu0_s_dn13 = assign102720_e154563_d_n13;
        locals.var_mu0_s_rv = 0.0;

        let (assign102730_e154576, assign102730_e154576_d_n0, assign102730_e154576_d_n2, assign102730_e154576_d_n4, assign102730_e154576_d_n5, assign102730_e154576_d_n6, assign102730_e154576_d_n7, assign102730_e154576_d_n8, assign102730_e154576_d_n9, assign102730_e154576_d_n10, assign102730_e154576_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102730_e154570: f64 = (locals.var_rrdrvmaxs * locals.var_rdrvmaxwe);
        let assign102730_e154572: f64 = (assign102730_e154570 * locals.var_rdrvmaxle);
        let assign102730_e154574: f64 = (assign102730_e154572 + 1e-25);
        (assign102730_e154574, ((locals.var_rrdrvmaxs_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn9 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn13 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe_s, locals.var_vmaxe_s_dn0, locals.var_vmaxe_s_dn2, locals.var_vmaxe_s_dn4, locals.var_vmaxe_s_dn5, locals.var_vmaxe_s_dn6, locals.var_vmaxe_s_dn7, locals.var_vmaxe_s_dn8, locals.var_vmaxe_s_dn9, locals.var_vmaxe_s_dn10, locals.var_vmaxe_s_dn13,)
    }
};
        locals.var_vmaxe_s = assign102730_e154576;
        locals.var_vmaxe_s_dn0 = assign102730_e154576_d_n0;
        locals.var_vmaxe_s_dn2 = assign102730_e154576_d_n2;
        locals.var_vmaxe_s_dn4 = assign102730_e154576_d_n4;
        locals.var_vmaxe_s_dn5 = assign102730_e154576_d_n5;
        locals.var_vmaxe_s_dn6 = assign102730_e154576_d_n6;
        locals.var_vmaxe_s_dn7 = assign102730_e154576_d_n7;
        locals.var_vmaxe_s_dn8 = assign102730_e154576_d_n8;
        locals.var_vmaxe_s_dn9 = assign102730_e154576_d_n9;
        locals.var_vmaxe_s_dn10 = assign102730_e154576_d_n10;
        locals.var_vmaxe_s_dn13 = assign102730_e154576_d_n13;
        locals.var_vmaxe_s_rv = 0.0;

        let (assign102740_e154585, assign102740_e154585_d_n2, assign102740_e154585_d_n7,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102740_e154583: f64 = (locals.var_vsps / locals.var_ldrifte_s);
        (assign102740_e154583, (locals.var_vsps_dn2 / locals.var_ldrifte_s), (locals.var_vsps_dn7 / locals.var_ldrifte_s),)
    } else {
        (locals.var_edri_s, locals.var_edri_s_dn2, locals.var_edri_s_dn7,)
    }
};
        locals.var_edri_s = assign102740_e154585;
        locals.var_edri_s_dn2 = assign102740_e154585_d_n2;
        locals.var_edri_s_dn7 = assign102740_e154585_d_n7;
        locals.var_edri_s_rv = 0.0;

        let (assign102750_e154594, assign102750_e154594_d_n0, assign102750_e154594_d_n2, assign102750_e154594_d_n4, assign102750_e154594_d_n5, assign102750_e154594_d_n6, assign102750_e154594_d_n7, assign102750_e154594_d_n8, assign102750_e154594_d_n9, assign102750_e154594_d_n10, assign102750_e154594_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102750_e154592: f64 = (locals.var_mu0_s * locals.var_edri_s);
        (assign102750_e154592, (locals.var_mu0_s_dn0 * locals.var_edri_s), ((locals.var_mu0_s_dn2 * locals.var_edri_s) + (locals.var_mu0_s * locals.var_edri_s_dn2)), (locals.var_mu0_s_dn4 * locals.var_edri_s), (locals.var_mu0_s_dn5 * locals.var_edri_s), (locals.var_mu0_s_dn6 * locals.var_edri_s), ((locals.var_mu0_s_dn7 * locals.var_edri_s) + (locals.var_mu0_s * locals.var_edri_s_dn7)), (locals.var_mu0_s_dn8 * locals.var_edri_s), (locals.var_mu0_s_dn9 * locals.var_edri_s), (locals.var_mu0_s_dn10 * locals.var_edri_s), (locals.var_mu0_s_dn13 * locals.var_edri_s),)
    } else {
        (locals.var_vdri_s, locals.var_vdri_s_dn0, locals.var_vdri_s_dn2, locals.var_vdri_s_dn4, locals.var_vdri_s_dn5, locals.var_vdri_s_dn6, locals.var_vdri_s_dn7, locals.var_vdri_s_dn8, locals.var_vdri_s_dn9, locals.var_vdri_s_dn10, locals.var_vdri_s_dn13,)
    }
};
        locals.var_vdri_s = assign102750_e154594;
        locals.var_vdri_s_dn0 = assign102750_e154594_d_n0;
        locals.var_vdri_s_dn2 = assign102750_e154594_d_n2;
        locals.var_vdri_s_dn4 = assign102750_e154594_d_n4;
        locals.var_vdri_s_dn5 = assign102750_e154594_d_n5;
        locals.var_vdri_s_dn6 = assign102750_e154594_d_n6;
        locals.var_vdri_s_dn7 = assign102750_e154594_d_n7;
        locals.var_vdri_s_dn8 = assign102750_e154594_d_n8;
        locals.var_vdri_s_dn9 = assign102750_e154594_d_n9;
        locals.var_vdri_s_dn10 = assign102750_e154594_d_n10;
        locals.var_vdri_s_dn13 = assign102750_e154594_d_n13;
        locals.var_vdri_s_rv = 0.0;

        let assign102760_e154597: f64 = if locals.var_vsps >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2339 = assign102760_e154597;
        locals.var_guard2339_rv = 0.0;

        let (assign102770_e154608, assign102770_e154608_d_n0, assign102770_e154608_d_n2, assign102770_e154608_d_n4, assign102770_e154608_d_n5, assign102770_e154608_d_n6, assign102770_e154608_d_n7, assign102770_e154608_d_n8, assign102770_e154608_d_n9, assign102770_e154608_d_n10, assign102770_e154608_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2339 != 0.0)) {
        let assign102770_e154606: f64 = (locals.var_vdri_s / locals.var_vmaxe_s);
        (assign102770_e154606, (((locals.var_vdri_s_dn0 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn0)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn2 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn2)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn4 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn4)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn5 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn5)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn6 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn6)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn7 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn7)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn8 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn8)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn9 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn9)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn10 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn10)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn13 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn13)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign102770_e154608;
        locals.var_t1_dn0 = assign102770_e154608_d_n0;
        locals.var_t1_dn2 = assign102770_e154608_d_n2;
        locals.var_t1_dn4 = assign102770_e154608_d_n4;
        locals.var_t1_dn5 = assign102770_e154608_d_n5;
        locals.var_t1_dn6 = assign102770_e154608_d_n6;
        locals.var_t1_dn7 = assign102770_e154608_d_n7;
        locals.var_t1_dn8 = assign102770_e154608_d_n8;
        locals.var_t1_dn9 = assign102770_e154608_d_n9;
        locals.var_t1_dn10 = assign102770_e154608_d_n10;
        locals.var_t1_dn13 = assign102770_e154608_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign102780_e154621, assign102780_e154621_d_n0, assign102780_e154621_d_n2, assign102780_e154621_d_n4, assign102780_e154621_d_n5, assign102780_e154621_d_n6, assign102780_e154621_d_n7, assign102780_e154621_d_n8, assign102780_e154621_d_n9, assign102780_e154621_d_n10, assign102780_e154621_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2339 == 0.0)) {
        let assign102780_e154617: f64 = (-locals.var_vdri_s);
        let assign102780_e154619: f64 = (assign102780_e154617 / locals.var_vmaxe_s);
        (assign102780_e154619, ((((-locals.var_vdri_s_dn0) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn0)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn2) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn2)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn4) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn4)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn5) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn5)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn6) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn6)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn7) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn7)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn8) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn8)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn9) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn9)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn10) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn10)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn13) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn13)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign102780_e154621;
        locals.var_t1_dn0 = assign102780_e154621_d_n0;
        locals.var_t1_dn2 = assign102780_e154621_d_n2;
        locals.var_t1_dn4 = assign102780_e154621_d_n4;
        locals.var_t1_dn5 = assign102780_e154621_d_n5;
        locals.var_t1_dn6 = assign102780_e154621_d_n6;
        locals.var_t1_dn7 = assign102780_e154621_d_n7;
        locals.var_t1_dn8 = assign102780_e154621_d_n8;
        locals.var_t1_dn9 = assign102780_e154621_d_n9;
        locals.var_t1_dn10 = assign102780_e154621_d_n10;
        locals.var_t1_dn13 = assign102780_e154621_d_n13;
        locals.var_t1_rv = 0.0;

        let assign102790_e154625: f64 = (10.0 * 2.220446049250313e-16);
        let assign102790_e154626: f64 = (1.0 - assign102790_e154625);
        let assign102790_e154633: f64 = (10.0 * 2.220446049250313e-16);
        let assign102790_e154634: f64 = (1.0 + assign102790_e154633);
        let assign102790_e154636: f64 = if ((assign102790_e154626 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102790_e154634)) { 1.0 } else { 0.0 };
        locals.var_guard2340 = assign102790_e154636;
        locals.var_guard2340_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_381(
        locals: &mut StampLocals,
    ) {
        let (assign102800_e154645, assign102800_e154645_d_n0, assign102800_e154645_d_n2, assign102800_e154645_d_n4, assign102800_e154645_d_n5, assign102800_e154645_d_n6, assign102800_e154645_d_n7, assign102800_e154645_d_n8, assign102800_e154645_d_n9, assign102800_e154645_d_n10, assign102800_e154645_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2340 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign102800_e154645;
        locals.var_t3_dn0 = assign102800_e154645_d_n0;
        locals.var_t3_dn2 = assign102800_e154645_d_n2;
        locals.var_t3_dn4 = assign102800_e154645_d_n4;
        locals.var_t3_dn5 = assign102800_e154645_d_n5;
        locals.var_t3_dn6 = assign102800_e154645_d_n6;
        locals.var_t3_dn7 = assign102800_e154645_d_n7;
        locals.var_t3_dn8 = assign102800_e154645_d_n8;
        locals.var_t3_dn9 = assign102800_e154645_d_n9;
        locals.var_t3_dn10 = assign102800_e154645_d_n10;
        locals.var_t3_dn13 = assign102800_e154645_d_n13;
        locals.var_t3_rv = 0.0;

        let assign102810_e154649: f64 = (10.0 * 2.220446049250313e-16);
        let assign102810_e154650: f64 = (2.0 - assign102810_e154649);
        let assign102810_e154657: f64 = (10.0 * 2.220446049250313e-16);
        let assign102810_e154658: f64 = (2.0 + assign102810_e154657);
        let assign102810_e154660: f64 = if ((assign102810_e154650 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102810_e154658)) { 1.0 } else { 0.0 };
        locals.var_guard2341 = assign102810_e154660;
        locals.var_guard2341_rv = 0.0;

        let (assign102820_e154672, assign102820_e154672_d_n0, assign102820_e154672_d_n2, assign102820_e154672_d_n4, assign102820_e154672_d_n5, assign102820_e154672_d_n6, assign102820_e154672_d_n7, assign102820_e154672_d_n8, assign102820_e154672_d_n9, assign102820_e154672_d_n10, assign102820_e154672_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2340 == 0.0)) && (locals.var_guard2341 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign102820_e154672;
        locals.var_t3_dn0 = assign102820_e154672_d_n0;
        locals.var_t3_dn2 = assign102820_e154672_d_n2;
        locals.var_t3_dn4 = assign102820_e154672_d_n4;
        locals.var_t3_dn5 = assign102820_e154672_d_n5;
        locals.var_t3_dn6 = assign102820_e154672_d_n6;
        locals.var_t3_dn7 = assign102820_e154672_d_n7;
        locals.var_t3_dn8 = assign102820_e154672_d_n8;
        locals.var_t3_dn9 = assign102820_e154672_d_n9;
        locals.var_t3_dn10 = assign102820_e154672_d_n10;
        locals.var_t3_dn13 = assign102820_e154672_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign102830_e154689, assign102830_e154689_d_n0, assign102830_e154689_d_n2, assign102830_e154689_d_n4, assign102830_e154689_d_n5, assign102830_e154689_d_n6, assign102830_e154689_d_n7, assign102830_e154689_d_n8, assign102830_e154689_d_n9, assign102830_e154689_d_n10, assign102830_e154689_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2340 == 0.0)) && (locals.var_guard2341 == 0.0)) {
        let assign102830_e154686: f64 = (locals.var_uc_rdrbb_s - 1.0);
        let assign102830_e154687: f64 = (locals.var_t1).powf(assign102830_e154686);
        (assign102830_e154687, if locals.var_uc_rdrbb_s_dn0 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn0)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn0 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn2 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn2)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn2 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn4 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn4)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn4 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn5 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn5)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn5 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn6 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn6)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn6 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn7 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn7)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn7 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn8 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn8)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn8 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn9 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn9)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn9 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn10 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn10)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn10 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn13 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn13)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn13 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn13 / locals.var_t1)))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign102830_e154689;
        locals.var_t3_dn0 = assign102830_e154689_d_n0;
        locals.var_t3_dn2 = assign102830_e154689_d_n2;
        locals.var_t3_dn4 = assign102830_e154689_d_n4;
        locals.var_t3_dn5 = assign102830_e154689_d_n5;
        locals.var_t3_dn6 = assign102830_e154689_d_n6;
        locals.var_t3_dn7 = assign102830_e154689_d_n7;
        locals.var_t3_dn8 = assign102830_e154689_d_n8;
        locals.var_t3_dn9 = assign102830_e154689_d_n9;
        locals.var_t3_dn10 = assign102830_e154689_d_n10;
        locals.var_t3_dn13 = assign102830_e154689_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign102840_e154698, assign102840_e154698_d_n0, assign102840_e154698_d_n2, assign102840_e154698_d_n4, assign102840_e154698_d_n5, assign102840_e154698_d_n6, assign102840_e154698_d_n7, assign102840_e154698_d_n8, assign102840_e154698_d_n9, assign102840_e154698_d_n10, assign102840_e154698_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102840_e154696: f64 = (locals.var_t1 * locals.var_t3);
        (assign102840_e154696, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign102840_e154698;
        locals.var_t2_dn0 = assign102840_e154698_d_n0;
        locals.var_t2_dn2 = assign102840_e154698_d_n2;
        locals.var_t2_dn4 = assign102840_e154698_d_n4;
        locals.var_t2_dn5 = assign102840_e154698_d_n5;
        locals.var_t2_dn6 = assign102840_e154698_d_n6;
        locals.var_t2_dn7 = assign102840_e154698_d_n7;
        locals.var_t2_dn8 = assign102840_e154698_d_n8;
        locals.var_t2_dn9 = assign102840_e154698_d_n9;
        locals.var_t2_dn10 = assign102840_e154698_d_n10;
        locals.var_t2_dn13 = assign102840_e154698_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign102850_e154707, assign102850_e154707_d_n0, assign102850_e154707_d_n2, assign102850_e154707_d_n4, assign102850_e154707_d_n5, assign102850_e154707_d_n6, assign102850_e154707_d_n7, assign102850_e154707_d_n8, assign102850_e154707_d_n9, assign102850_e154707_d_n10, assign102850_e154707_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102850_e154705: f64 = (1.0 + locals.var_t2);
        (assign102850_e154705, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign102850_e154707;
        locals.var_t4_dn0 = assign102850_e154707_d_n0;
        locals.var_t4_dn2 = assign102850_e154707_d_n2;
        locals.var_t4_dn4 = assign102850_e154707_d_n4;
        locals.var_t4_dn5 = assign102850_e154707_d_n5;
        locals.var_t4_dn6 = assign102850_e154707_d_n6;
        locals.var_t4_dn7 = assign102850_e154707_d_n7;
        locals.var_t4_dn8 = assign102850_e154707_d_n8;
        locals.var_t4_dn9 = assign102850_e154707_d_n9;
        locals.var_t4_dn10 = assign102850_e154707_d_n10;
        locals.var_t4_dn13 = assign102850_e154707_d_n13;
        locals.var_t4_rv = 0.0;

        let assign102860_e154711: f64 = (10.0 * 2.220446049250313e-16);
        let assign102860_e154712: f64 = (1.0 - assign102860_e154711);
        let assign102860_e154719: f64 = (10.0 * 2.220446049250313e-16);
        let assign102860_e154720: f64 = (1.0 + assign102860_e154719);
        let assign102860_e154722: f64 = if ((assign102860_e154712 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102860_e154720)) { 1.0 } else { 0.0 };
        locals.var_guard2342 = assign102860_e154722;
        locals.var_guard2342_rv = 0.0;

        let (assign102870_e154733, assign102870_e154733_d_n0, assign102870_e154733_d_n2, assign102870_e154733_d_n4, assign102870_e154733_d_n5, assign102870_e154733_d_n6, assign102870_e154733_d_n7, assign102870_e154733_d_n8, assign102870_e154733_d_n9, assign102870_e154733_d_n10, assign102870_e154733_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let assign102870_e154731: f64 = (1.0 / locals.var_t4);
        (assign102870_e154731, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign102870_e154733;
        locals.var_t5_dn0 = assign102870_e154733_d_n0;
        locals.var_t5_dn2 = assign102870_e154733_d_n2;
        locals.var_t5_dn4 = assign102870_e154733_d_n4;
        locals.var_t5_dn5 = assign102870_e154733_d_n5;
        locals.var_t5_dn6 = assign102870_e154733_d_n6;
        locals.var_t5_dn7 = assign102870_e154733_d_n7;
        locals.var_t5_dn8 = assign102870_e154733_d_n8;
        locals.var_t5_dn9 = assign102870_e154733_d_n9;
        locals.var_t5_dn10 = assign102870_e154733_d_n10;
        locals.var_t5_dn13 = assign102870_e154733_d_n13;
        locals.var_t5_rv = 0.0;

        let assign102880_e154737: f64 = (10.0 * 2.220446049250313e-16);
        let assign102880_e154738: f64 = (2.0 - assign102880_e154737);
        let assign102880_e154745: f64 = (10.0 * 2.220446049250313e-16);
        let assign102880_e154746: f64 = (2.0 + assign102880_e154745);
        let assign102880_e154748: f64 = if ((assign102880_e154738 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102880_e154746)) { 1.0 } else { 0.0 };
        locals.var_guard2343 = assign102880_e154748;
        locals.var_guard2343_rv = 0.0;

        let (assign102890_e154763, assign102890_e154763_d_n0, assign102890_e154763_d_n2, assign102890_e154763_d_n4, assign102890_e154763_d_n5, assign102890_e154763_d_n6, assign102890_e154763_d_n7, assign102890_e154763_d_n8, assign102890_e154763_d_n9, assign102890_e154763_d_n10, assign102890_e154763_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2342 == 0.0)) && (locals.var_guard2343 != 0.0)) {
        let assign102890_e154760: f64 = (locals.var_t4).sqrt();
        let assign102890_e154761: f64 = (1.0 / assign102890_e154760);
        (assign102890_e154761, (-((locals.var_t4_dn0 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn2 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn4 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn5 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn6 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn7 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn8 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn9 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn10 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn13 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign102890_e154763;
        locals.var_t5_dn0 = assign102890_e154763_d_n0;
        locals.var_t5_dn2 = assign102890_e154763_d_n2;
        locals.var_t5_dn4 = assign102890_e154763_d_n4;
        locals.var_t5_dn5 = assign102890_e154763_d_n5;
        locals.var_t5_dn6 = assign102890_e154763_d_n6;
        locals.var_t5_dn7 = assign102890_e154763_d_n7;
        locals.var_t5_dn8 = assign102890_e154763_d_n8;
        locals.var_t5_dn9 = assign102890_e154763_d_n9;
        locals.var_t5_dn10 = assign102890_e154763_d_n10;
        locals.var_t5_dn13 = assign102890_e154763_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign102900_e154788, assign102900_e154788_d_n0, assign102900_e154788_d_n2, assign102900_e154788_d_n4, assign102900_e154788_d_n5, assign102900_e154788_d_n6, assign102900_e154788_d_n7, assign102900_e154788_d_n8, assign102900_e154788_d_n9, assign102900_e154788_d_n10, assign102900_e154788_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2342 == 0.0)) && (locals.var_guard2343 == 0.0)) {
        let (assign102900_e154786, assign102900_e154786_d_n0, assign102900_e154786_d_n2, assign102900_e154786_d_n4, assign102900_e154786_d_n5, assign102900_e154786_d_n6, assign102900_e154786_d_n7, assign102900_e154786_d_n8, assign102900_e154786_d_n9, assign102900_e154786_d_n10, assign102900_e154786_d_n13,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign102900_e154780: f64 = (-1.0);
                let assign102900_e154782: f64 = (assign102900_e154780 / locals.var_uc_rdrbb_s);
                let assign102900_e154784: f64 = (assign102900_e154782 - 1.0);
                let assign102900_e154785: f64 = (locals.var_t4).powf(assign102900_e154784);
                (assign102900_e154785, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn0) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn0)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn0) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn0 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn2) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn2)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn2) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn2 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn4) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn4)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn4) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn4 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn5) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn5)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn5) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn5 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn6) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn6)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn6) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn6 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn7) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn7)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn7) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn7 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn8) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn8)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn8) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn8 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn9) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn9)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn9) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn9 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn10) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn10)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn10) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn10 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn13) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn13)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn13) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn13 / locals.var_t4)))) },)
            }
        };
        (assign102900_e154786, assign102900_e154786_d_n0, assign102900_e154786_d_n2, assign102900_e154786_d_n4, assign102900_e154786_d_n5, assign102900_e154786_d_n6, assign102900_e154786_d_n7, assign102900_e154786_d_n8, assign102900_e154786_d_n9, assign102900_e154786_d_n10, assign102900_e154786_d_n13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign102900_e154788;
        locals.var_t6_dn0 = assign102900_e154788_d_n0;
        locals.var_t6_dn2 = assign102900_e154788_d_n2;
        locals.var_t6_dn4 = assign102900_e154788_d_n4;
        locals.var_t6_dn5 = assign102900_e154788_d_n5;
        locals.var_t6_dn6 = assign102900_e154788_d_n6;
        locals.var_t6_dn7 = assign102900_e154788_d_n7;
        locals.var_t6_dn8 = assign102900_e154788_d_n8;
        locals.var_t6_dn9 = assign102900_e154788_d_n9;
        locals.var_t6_dn10 = assign102900_e154788_d_n10;
        locals.var_t6_dn13 = assign102900_e154788_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign102910_e154803, assign102910_e154803_d_n0, assign102910_e154803_d_n2, assign102910_e154803_d_n4, assign102910_e154803_d_n5, assign102910_e154803_d_n6, assign102910_e154803_d_n7, assign102910_e154803_d_n8, assign102910_e154803_d_n9, assign102910_e154803_d_n10, assign102910_e154803_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2342 == 0.0)) && (locals.var_guard2343 == 0.0)) {
        let assign102910_e154801: f64 = (locals.var_t4 * locals.var_t6);
        (assign102910_e154801, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn13 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign102910_e154803;
        locals.var_t5_dn0 = assign102910_e154803_d_n0;
        locals.var_t5_dn2 = assign102910_e154803_d_n2;
        locals.var_t5_dn4 = assign102910_e154803_d_n4;
        locals.var_t5_dn5 = assign102910_e154803_d_n5;
        locals.var_t5_dn6 = assign102910_e154803_d_n6;
        locals.var_t5_dn7 = assign102910_e154803_d_n7;
        locals.var_t5_dn8 = assign102910_e154803_d_n8;
        locals.var_t5_dn9 = assign102910_e154803_d_n9;
        locals.var_t5_dn10 = assign102910_e154803_d_n10;
        locals.var_t5_dn13 = assign102910_e154803_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign102920_e154812, assign102920_e154812_d_n0, assign102920_e154812_d_n2, assign102920_e154812_d_n4, assign102920_e154812_d_n5, assign102920_e154812_d_n6, assign102920_e154812_d_n7, assign102920_e154812_d_n8, assign102920_e154812_d_n9, assign102920_e154812_d_n10, assign102920_e154812_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102920_e154810: f64 = (locals.var_mu0_s * locals.var_t5);
        (assign102920_e154810, ((locals.var_mu0_s_dn0 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn0)), ((locals.var_mu0_s_dn2 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn2)), ((locals.var_mu0_s_dn4 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn4)), ((locals.var_mu0_s_dn5 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn5)), ((locals.var_mu0_s_dn6 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn6)), ((locals.var_mu0_s_dn7 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn7)), ((locals.var_mu0_s_dn8 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn8)), ((locals.var_mu0_s_dn9 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn9)), ((locals.var_mu0_s_dn10 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn10)), ((locals.var_mu0_s_dn13 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn13)),)
    } else {
        (locals.var_mu_s, locals.var_mu_s_dn0, locals.var_mu_s_dn2, locals.var_mu_s_dn4, locals.var_mu_s_dn5, locals.var_mu_s_dn6, locals.var_mu_s_dn7, locals.var_mu_s_dn8, locals.var_mu_s_dn9, locals.var_mu_s_dn10, locals.var_mu_s_dn13,)
    }
};
        locals.var_mu_s = assign102920_e154812;
        locals.var_mu_s_dn0 = assign102920_e154812_d_n0;
        locals.var_mu_s_dn2 = assign102920_e154812_d_n2;
        locals.var_mu_s_dn4 = assign102920_e154812_d_n4;
        locals.var_mu_s_dn5 = assign102920_e154812_d_n5;
        locals.var_mu_s_dn6 = assign102920_e154812_d_n6;
        locals.var_mu_s_dn7 = assign102920_e154812_d_n7;
        locals.var_mu_s_dn8 = assign102920_e154812_d_n8;
        locals.var_mu_s_dn9 = assign102920_e154812_d_n9;
        locals.var_mu_s_dn10 = assign102920_e154812_d_n10;
        locals.var_mu_s_dn13 = assign102920_e154812_d_n13;
        locals.var_mu_s_rv = 0.0;

        let (assign102930_e154819,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        (locals.var_novers,)
    } else {
        (locals.var_carr_s,)
    }
};
        locals.var_carr_s = assign102930_e154819;
        locals.var_carr_s_rv = 0.0;

        let (assign102940_e154826,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        (locals.var_xmax_s,)
    } else {
        (locals.var_xov_s,)
    }
};
        locals.var_xov_s = assign102940_e154826;
        locals.var_xov_s_rv = 0.0;

        let (assign102950_e154835, assign102950_e154835_d_n0, assign102950_e154835_d_n2, assign102950_e154835_d_n4, assign102950_e154835_d_n5, assign102950_e154835_d_n6, assign102950_e154835_d_n7, assign102950_e154835_d_n8, assign102950_e154835_d_n9, assign102950_e154835_d_n10, assign102950_e154835_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102950_e154833: f64 = (1.6021918e-19 / locals.var_ldrifte_s);
        (assign102950_e154833, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign102950_e154835;
        locals.var_t1_dn0 = assign102950_e154835_d_n0;
        locals.var_t1_dn2 = assign102950_e154835_d_n2;
        locals.var_t1_dn4 = assign102950_e154835_d_n4;
        locals.var_t1_dn5 = assign102950_e154835_d_n5;
        locals.var_t1_dn6 = assign102950_e154835_d_n6;
        locals.var_t1_dn7 = assign102950_e154835_d_n7;
        locals.var_t1_dn8 = assign102950_e154835_d_n8;
        locals.var_t1_dn9 = assign102950_e154835_d_n9;
        locals.var_t1_dn10 = assign102950_e154835_d_n10;
        locals.var_t1_dn13 = assign102950_e154835_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign102960_e154848, assign102960_e154848_d_n0, assign102960_e154848_d_n2, assign102960_e154848_d_n4, assign102960_e154848_d_n5, assign102960_e154848_d_n6, assign102960_e154848_d_n7, assign102960_e154848_d_n8, assign102960_e154848_d_n9, assign102960_e154848_d_n10, assign102960_e154848_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102960_e154842: f64 = (locals.var_t1 * locals.var_xov_s);
        let assign102960_e154844: f64 = (assign102960_e154842 * locals.var_mu_s);
        let assign102960_e154846: f64 = (assign102960_e154844 * locals.var_carr_s);
        (assign102960_e154846, ((((locals.var_t1_dn0 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn0)) * locals.var_carr_s), ((((locals.var_t1_dn2 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn2)) * locals.var_carr_s), ((((locals.var_t1_dn4 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn4)) * locals.var_carr_s), ((((locals.var_t1_dn5 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn5)) * locals.var_carr_s), ((((locals.var_t1_dn6 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn6)) * locals.var_carr_s), ((((locals.var_t1_dn7 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn7)) * locals.var_carr_s), ((((locals.var_t1_dn8 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn8)) * locals.var_carr_s), ((((locals.var_t1_dn9 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn9)) * locals.var_carr_s), ((((locals.var_t1_dn10 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn10)) * locals.var_carr_s), ((((locals.var_t1_dn13 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn13)) * locals.var_carr_s),)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn13,)
    }
};
        locals.var_gd_s = assign102960_e154848;
        locals.var_gd_s_dn0 = assign102960_e154848_d_n0;
        locals.var_gd_s_dn2 = assign102960_e154848_d_n2;
        locals.var_gd_s_dn4 = assign102960_e154848_d_n4;
        locals.var_gd_s_dn5 = assign102960_e154848_d_n5;
        locals.var_gd_s_dn6 = assign102960_e154848_d_n6;
        locals.var_gd_s_dn7 = assign102960_e154848_d_n7;
        locals.var_gd_s_dn8 = assign102960_e154848_d_n8;
        locals.var_gd_s_dn9 = assign102960_e154848_d_n9;
        locals.var_gd_s_dn10 = assign102960_e154848_d_n10;
        locals.var_gd_s_dn13 = assign102960_e154848_d_n13;
        locals.var_gd_s_rv = 0.0;

        let assign102970_e154852: f64 = 1e-25;
        let assign102970_e154857: f64 = if ((locals.var_gd_s < assign102970_e154852) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2344 = assign102970_e154857;
        locals.var_guard2344_rv = 0.0;

        let (assign102980_e154870, assign102980_e154870_d_n0, assign102980_e154870_d_n2, assign102980_e154870_d_n4, assign102980_e154870_d_n5, assign102980_e154870_d_n6, assign102980_e154870_d_n7, assign102980_e154870_d_n8, assign102980_e154870_d_n9, assign102980_e154870_d_n10, assign102980_e154870_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign102980_e154866: f64 = 1e-25;
        let assign102980_e154868: f64 = (assign102980_e154866 - locals.var_gd_s);
        (assign102980_e154868, (-locals.var_gd_s_dn0), (-locals.var_gd_s_dn2), (-locals.var_gd_s_dn4), (-locals.var_gd_s_dn5), (-locals.var_gd_s_dn6), (-locals.var_gd_s_dn7), (-locals.var_gd_s_dn8), (-locals.var_gd_s_dn9), (-locals.var_gd_s_dn10), (-locals.var_gd_s_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign102980_e154870;
        locals.var_tmf1_dn0 = assign102980_e154870_d_n0;
        locals.var_tmf1_dn2 = assign102980_e154870_d_n2;
        locals.var_tmf1_dn4 = assign102980_e154870_d_n4;
        locals.var_tmf1_dn5 = assign102980_e154870_d_n5;
        locals.var_tmf1_dn6 = assign102980_e154870_d_n6;
        locals.var_tmf1_dn7 = assign102980_e154870_d_n7;
        locals.var_tmf1_dn8 = assign102980_e154870_d_n8;
        locals.var_tmf1_dn9 = assign102980_e154870_d_n9;
        locals.var_tmf1_dn10 = assign102980_e154870_d_n10;
        locals.var_tmf1_dn13 = assign102980_e154870_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign102990_e154881, assign102990_e154881_d_n0, assign102990_e154881_d_n2, assign102990_e154881_d_n4, assign102990_e154881_d_n5, assign102990_e154881_d_n6, assign102990_e154881_d_n7, assign102990_e154881_d_n8, assign102990_e154881_d_n9, assign102990_e154881_d_n10, assign102990_e154881_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign102990_e154879: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign102990_e154879, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign102990_e154881;
        locals.var_x2_dn0 = assign102990_e154881_d_n0;
        locals.var_x2_dn2 = assign102990_e154881_d_n2;
        locals.var_x2_dn4 = assign102990_e154881_d_n4;
        locals.var_x2_dn5 = assign102990_e154881_d_n5;
        locals.var_x2_dn6 = assign102990_e154881_d_n6;
        locals.var_x2_dn7 = assign102990_e154881_d_n7;
        locals.var_x2_dn8 = assign102990_e154881_d_n8;
        locals.var_x2_dn9 = assign102990_e154881_d_n9;
        locals.var_x2_dn10 = assign102990_e154881_d_n10;
        locals.var_x2_dn13 = assign102990_e154881_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign103000_e154892, assign103000_e154892_d_n0, assign103000_e154892_d_n2, assign103000_e154892_d_n4, assign103000_e154892_d_n5, assign103000_e154892_d_n6, assign103000_e154892_d_n7, assign103000_e154892_d_n8, assign103000_e154892_d_n9, assign103000_e154892_d_n10, assign103000_e154892_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103000_e154890: f64 = (1e-25 * 1e-25);
        (assign103000_e154890, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign103000_e154892;
        locals.var_xmax2_dn0 = assign103000_e154892_d_n0;
        locals.var_xmax2_dn2 = assign103000_e154892_d_n2;
        locals.var_xmax2_dn4 = assign103000_e154892_d_n4;
        locals.var_xmax2_dn5 = assign103000_e154892_d_n5;
        locals.var_xmax2_dn6 = assign103000_e154892_d_n6;
        locals.var_xmax2_dn7 = assign103000_e154892_d_n7;
        locals.var_xmax2_dn8 = assign103000_e154892_d_n8;
        locals.var_xmax2_dn9 = assign103000_e154892_d_n9;
        locals.var_xmax2_dn10 = assign103000_e154892_d_n10;
        locals.var_xmax2_dn13 = assign103000_e154892_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign103010_e154901, assign103010_e154901_d_n0, assign103010_e154901_d_n2, assign103010_e154901_d_n4, assign103010_e154901_d_n5, assign103010_e154901_d_n6, assign103010_e154901_d_n7, assign103010_e154901_d_n8, assign103010_e154901_d_n9, assign103010_e154901_d_n10, assign103010_e154901_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign103010_e154901;
        locals.var_xp_dn0 = assign103010_e154901_d_n0;
        locals.var_xp_dn2 = assign103010_e154901_d_n2;
        locals.var_xp_dn4 = assign103010_e154901_d_n4;
        locals.var_xp_dn5 = assign103010_e154901_d_n5;
        locals.var_xp_dn6 = assign103010_e154901_d_n6;
        locals.var_xp_dn7 = assign103010_e154901_d_n7;
        locals.var_xp_dn8 = assign103010_e154901_d_n8;
        locals.var_xp_dn9 = assign103010_e154901_d_n9;
        locals.var_xp_dn10 = assign103010_e154901_d_n10;
        locals.var_xp_dn13 = assign103010_e154901_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign103020_e154910, assign103020_e154910_d_n0, assign103020_e154910_d_n2, assign103020_e154910_d_n4, assign103020_e154910_d_n5, assign103020_e154910_d_n6, assign103020_e154910_d_n7, assign103020_e154910_d_n8, assign103020_e154910_d_n9, assign103020_e154910_d_n10, assign103020_e154910_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign103020_e154910;
        locals.var_xmp_dn0 = assign103020_e154910_d_n0;
        locals.var_xmp_dn2 = assign103020_e154910_d_n2;
        locals.var_xmp_dn4 = assign103020_e154910_d_n4;
        locals.var_xmp_dn5 = assign103020_e154910_d_n5;
        locals.var_xmp_dn6 = assign103020_e154910_d_n6;
        locals.var_xmp_dn7 = assign103020_e154910_d_n7;
        locals.var_xmp_dn8 = assign103020_e154910_d_n8;
        locals.var_xmp_dn9 = assign103020_e154910_d_n9;
        locals.var_xmp_dn10 = assign103020_e154910_d_n10;
        locals.var_xmp_dn13 = assign103020_e154910_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign103030_e154919,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign103030_e154919;
        locals.var_m0_rv = 0.0;

        let (assign103040_e154928,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103040_e154928;
        locals.var_mm_rv = 0.0;

        let (assign103050_e154937, assign103050_e154937_d_n0, assign103050_e154937_d_n2, assign103050_e154937_d_n4, assign103050_e154937_d_n5, assign103050_e154937_d_n6, assign103050_e154937_d_n7, assign103050_e154937_d_n8, assign103050_e154937_d_n9, assign103050_e154937_d_n10, assign103050_e154937_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign103050_e154937;
        locals.var_arg_dn0 = assign103050_e154937_d_n0;
        locals.var_arg_dn2 = assign103050_e154937_d_n2;
        locals.var_arg_dn4 = assign103050_e154937_d_n4;
        locals.var_arg_dn5 = assign103050_e154937_d_n5;
        locals.var_arg_dn6 = assign103050_e154937_d_n6;
        locals.var_arg_dn7 = assign103050_e154937_d_n7;
        locals.var_arg_dn8 = assign103050_e154937_d_n8;
        locals.var_arg_dn9 = assign103050_e154937_d_n9;
        locals.var_arg_dn10 = assign103050_e154937_d_n10;
        locals.var_arg_dn13 = assign103050_e154937_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign103060_e154946, assign103060_e154946_d_n0, assign103060_e154946_d_n2, assign103060_e154946_d_n4, assign103060_e154946_d_n5, assign103060_e154946_d_n6, assign103060_e154946_d_n7, assign103060_e154946_d_n8, assign103060_e154946_d_n9, assign103060_e154946_d_n10, assign103060_e154946_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign103060_e154946;
        locals.var_dnm_dn0 = assign103060_e154946_d_n0;
        locals.var_dnm_dn2 = assign103060_e154946_d_n2;
        locals.var_dnm_dn4 = assign103060_e154946_d_n4;
        locals.var_dnm_dn5 = assign103060_e154946_d_n5;
        locals.var_dnm_dn6 = assign103060_e154946_d_n6;
        locals.var_dnm_dn7 = assign103060_e154946_d_n7;
        locals.var_dnm_dn8 = assign103060_e154946_d_n8;
        locals.var_dnm_dn9 = assign103060_e154946_d_n9;
        locals.var_dnm_dn10 = assign103060_e154946_d_n10;
        locals.var_dnm_dn13 = assign103060_e154946_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign103070_e154957, assign103070_e154957_d_n0, assign103070_e154957_d_n2, assign103070_e154957_d_n4, assign103070_e154957_d_n5, assign103070_e154957_d_n6, assign103070_e154957_d_n7, assign103070_e154957_d_n8, assign103070_e154957_d_n9, assign103070_e154957_d_n10, assign103070_e154957_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103070_e154955: f64 = (locals.var_xp * locals.var_x2);
        (assign103070_e154955, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign103070_e154957;
        locals.var_xp_dn0 = assign103070_e154957_d_n0;
        locals.var_xp_dn2 = assign103070_e154957_d_n2;
        locals.var_xp_dn4 = assign103070_e154957_d_n4;
        locals.var_xp_dn5 = assign103070_e154957_d_n5;
        locals.var_xp_dn6 = assign103070_e154957_d_n6;
        locals.var_xp_dn7 = assign103070_e154957_d_n7;
        locals.var_xp_dn8 = assign103070_e154957_d_n8;
        locals.var_xp_dn9 = assign103070_e154957_d_n9;
        locals.var_xp_dn10 = assign103070_e154957_d_n10;
        locals.var_xp_dn13 = assign103070_e154957_d_n13;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_382(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign103080_e154968, assign103080_e154968_d_n0, assign103080_e154968_d_n2, assign103080_e154968_d_n4, assign103080_e154968_d_n5, assign103080_e154968_d_n6, assign103080_e154968_d_n7, assign103080_e154968_d_n8, assign103080_e154968_d_n9, assign103080_e154968_d_n10, assign103080_e154968_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103080_e154966: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign103080_e154966, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign103080_e154968;
        locals.var_xmp_dn0 = assign103080_e154968_d_n0;
        locals.var_xmp_dn2 = assign103080_e154968_d_n2;
        locals.var_xmp_dn4 = assign103080_e154968_d_n4;
        locals.var_xmp_dn5 = assign103080_e154968_d_n5;
        locals.var_xmp_dn6 = assign103080_e154968_d_n6;
        locals.var_xmp_dn7 = assign103080_e154968_d_n7;
        locals.var_xmp_dn8 = assign103080_e154968_d_n8;
        locals.var_xmp_dn9 = assign103080_e154968_d_n9;
        locals.var_xmp_dn10 = assign103080_e154968_d_n10;
        locals.var_xmp_dn13 = assign103080_e154968_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign103090_e154979, assign103090_e154979_d_n0, assign103090_e154979_d_n2, assign103090_e154979_d_n4, assign103090_e154979_d_n5, assign103090_e154979_d_n6, assign103090_e154979_d_n7, assign103090_e154979_d_n8, assign103090_e154979_d_n9, assign103090_e154979_d_n10, assign103090_e154979_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103090_e154977: f64 = (locals.var_xp * locals.var_x2);
        (assign103090_e154977, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign103090_e154979;
        locals.var_xp_dn0 = assign103090_e154979_d_n0;
        locals.var_xp_dn2 = assign103090_e154979_d_n2;
        locals.var_xp_dn4 = assign103090_e154979_d_n4;
        locals.var_xp_dn5 = assign103090_e154979_d_n5;
        locals.var_xp_dn6 = assign103090_e154979_d_n6;
        locals.var_xp_dn7 = assign103090_e154979_d_n7;
        locals.var_xp_dn8 = assign103090_e154979_d_n8;
        locals.var_xp_dn9 = assign103090_e154979_d_n9;
        locals.var_xp_dn10 = assign103090_e154979_d_n10;
        locals.var_xp_dn13 = assign103090_e154979_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign103100_e154990, assign103100_e154990_d_n0, assign103100_e154990_d_n2, assign103100_e154990_d_n4, assign103100_e154990_d_n5, assign103100_e154990_d_n6, assign103100_e154990_d_n7, assign103100_e154990_d_n8, assign103100_e154990_d_n9, assign103100_e154990_d_n10, assign103100_e154990_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103100_e154988: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign103100_e154988, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign103100_e154990;
        locals.var_xmp_dn0 = assign103100_e154990_d_n0;
        locals.var_xmp_dn2 = assign103100_e154990_d_n2;
        locals.var_xmp_dn4 = assign103100_e154990_d_n4;
        locals.var_xmp_dn5 = assign103100_e154990_d_n5;
        locals.var_xmp_dn6 = assign103100_e154990_d_n6;
        locals.var_xmp_dn7 = assign103100_e154990_d_n7;
        locals.var_xmp_dn8 = assign103100_e154990_d_n8;
        locals.var_xmp_dn9 = assign103100_e154990_d_n9;
        locals.var_xmp_dn10 = assign103100_e154990_d_n10;
        locals.var_xmp_dn13 = assign103100_e154990_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign103110_e155001, assign103110_e155001_d_n0, assign103110_e155001_d_n2, assign103110_e155001_d_n4, assign103110_e155001_d_n5, assign103110_e155001_d_n6, assign103110_e155001_d_n7, assign103110_e155001_d_n8, assign103110_e155001_d_n9, assign103110_e155001_d_n10, assign103110_e155001_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103110_e154999: f64 = (locals.var_xp + locals.var_xmp);
        (assign103110_e154999, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign103110_e155001;
        locals.var_arg_dn0 = assign103110_e155001_d_n0;
        locals.var_arg_dn2 = assign103110_e155001_d_n2;
        locals.var_arg_dn4 = assign103110_e155001_d_n4;
        locals.var_arg_dn5 = assign103110_e155001_d_n5;
        locals.var_arg_dn6 = assign103110_e155001_d_n6;
        locals.var_arg_dn7 = assign103110_e155001_d_n7;
        locals.var_arg_dn8 = assign103110_e155001_d_n8;
        locals.var_arg_dn9 = assign103110_e155001_d_n9;
        locals.var_arg_dn10 = assign103110_e155001_d_n10;
        locals.var_arg_dn13 = assign103110_e155001_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign103120_e155010, assign103120_e155010_d_n0, assign103120_e155010_d_n2, assign103120_e155010_d_n4, assign103120_e155010_d_n5, assign103120_e155010_d_n6, assign103120_e155010_d_n7, assign103120_e155010_d_n8, assign103120_e155010_d_n9, assign103120_e155010_d_n10, assign103120_e155010_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign103120_e155010;
        locals.var_dnm_dn0 = assign103120_e155010_d_n0;
        locals.var_dnm_dn2 = assign103120_e155010_d_n2;
        locals.var_dnm_dn4 = assign103120_e155010_d_n4;
        locals.var_dnm_dn5 = assign103120_e155010_d_n5;
        locals.var_dnm_dn6 = assign103120_e155010_d_n6;
        locals.var_dnm_dn7 = assign103120_e155010_d_n7;
        locals.var_dnm_dn8 = assign103120_e155010_d_n8;
        locals.var_dnm_dn9 = assign103120_e155010_d_n9;
        locals.var_dnm_dn10 = assign103120_e155010_d_n10;
        locals.var_dnm_dn13 = assign103120_e155010_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign103130_e155025: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2345 = assign103130_e155025;
        locals.var_guard2345_rv = 0.0;

        let assign103140_e155028: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2346 = assign103140_e155028;
        locals.var_guard2346_rv = 0.0;

        let (assign103150_e155041,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) && (locals.var_guard2346 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103150_e155041;
        locals.var_mm_rv = 0.0;

        let assign103160_e155044: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2347 = assign103160_e155044;
        locals.var_guard2347_rv = 0.0;

        let (assign103170_e155060,) = {
    if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103170_e155060;
        locals.var_mm_rv = 0.0;

        let assign103180_e155063: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2348 = assign103180_e155063;
        locals.var_guard2348_rv = 0.0;

        let (assign103190_e155082,) = {
    if (((((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103190_e155082;
        locals.var_mm_rv = 0.0;

        let assign103200_e155085: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2349 = assign103200_e155085;
        locals.var_guard2349_rv = 0.0;

        let (assign103210_e155107,) = {
    if ((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 == 0.0)) && (locals.var_guard2348 == 0.0)) && (locals.var_guard2349 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103210_e155107;
        locals.var_mm_rv = 0.0;

        let (assign103220_e155118,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign103220_e155118;
        locals.var_m0_rv = 0.0;

        let mut assign103230_loop_guard: usize = 0;
        while {
            let assign103230_cond_e155130: f64 = if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign103230_cond_e155130 != 0.0
        } {
            assign103230_loop_guard += 1;
            assert!(assign103230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign103230_body0_e155142, assign103230_body0_e155142_d_n0, assign103230_body0_e155142_d_n2, assign103230_body0_e155142_d_n4, assign103230_body0_e155142_d_n5, assign103230_body0_e155142_d_n6, assign103230_body0_e155142_d_n7, assign103230_body0_e155142_d_n8, assign103230_body0_e155142_d_n9, assign103230_body0_e155142_d_n10, assign103230_body0_e155142_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) {
        let assign103230_body0_e155140: f64 = (locals.var_dnm).sqrt();
        (assign103230_body0_e155140, (locals.var_dnm_dn0 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn2 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn4 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn5 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn6 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn7 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn8 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn9 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn10 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn13 / (2.0 * assign103230_body0_e155140)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign103230_body0_e155142;
            locals.var_dnm_dn0 = assign103230_body0_e155142_d_n0;
            locals.var_dnm_dn2 = assign103230_body0_e155142_d_n2;
            locals.var_dnm_dn4 = assign103230_body0_e155142_d_n4;
            locals.var_dnm_dn5 = assign103230_body0_e155142_d_n5;
            locals.var_dnm_dn6 = assign103230_body0_e155142_d_n6;
            locals.var_dnm_dn7 = assign103230_body0_e155142_d_n7;
            locals.var_dnm_dn8 = assign103230_body0_e155142_d_n8;
            locals.var_dnm_dn9 = assign103230_body0_e155142_d_n9;
            locals.var_dnm_dn10 = assign103230_body0_e155142_d_n10;
            locals.var_dnm_dn13 = assign103230_body0_e155142_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign103230_body1_e155155,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) {
        let assign103230_body1_e155153: f64 = (locals.var_m0 + 1.0);
        (assign103230_body1_e155153,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign103230_body1_e155155;
            locals.var_m0_rv = 0.0;
        }

        let (assign103240_e155178, assign103240_e155178_d_n0, assign103240_e155178_d_n2, assign103240_e155178_d_n4, assign103240_e155178_d_n5, assign103240_e155178_d_n6, assign103240_e155178_d_n7, assign103240_e155178_d_n8, assign103240_e155178_d_n9, assign103240_e155178_d_n10, assign103240_e155178_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 == 0.0)) {
        let (assign103240_e155176, assign103240_e155176_d_n0, assign103240_e155176_d_n2, assign103240_e155176_d_n4, assign103240_e155176_d_n5, assign103240_e155176_d_n6, assign103240_e155176_d_n7, assign103240_e155176_d_n8, assign103240_e155176_d_n9, assign103240_e155176_d_n10, assign103240_e155176_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign103240_e155173: f64 = (2.0 * 2.0);
                let assign103240_e155174: f64 = (1.0 / assign103240_e155173);
                let assign103240_e155175: f64 = (locals.var_dnm).powf(assign103240_e155174);
                (assign103240_e155175, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn0)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn2)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn4)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn5)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn6)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn7)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn8)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn9)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn10)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn13)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign103240_e155176, assign103240_e155176_d_n0, assign103240_e155176_d_n2, assign103240_e155176_d_n4, assign103240_e155176_d_n5, assign103240_e155176_d_n6, assign103240_e155176_d_n7, assign103240_e155176_d_n8, assign103240_e155176_d_n9, assign103240_e155176_d_n10, assign103240_e155176_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign103240_e155178;
        locals.var_dnm_dn0 = assign103240_e155178_d_n0;
        locals.var_dnm_dn2 = assign103240_e155178_d_n2;
        locals.var_dnm_dn4 = assign103240_e155178_d_n4;
        locals.var_dnm_dn5 = assign103240_e155178_d_n5;
        locals.var_dnm_dn6 = assign103240_e155178_d_n6;
        locals.var_dnm_dn7 = assign103240_e155178_d_n7;
        locals.var_dnm_dn8 = assign103240_e155178_d_n8;
        locals.var_dnm_dn9 = assign103240_e155178_d_n9;
        locals.var_dnm_dn10 = assign103240_e155178_d_n10;
        locals.var_dnm_dn13 = assign103240_e155178_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign103250_e155189, assign103250_e155189_d_n0, assign103250_e155189_d_n2, assign103250_e155189_d_n4, assign103250_e155189_d_n5, assign103250_e155189_d_n6, assign103250_e155189_d_n7, assign103250_e155189_d_n8, assign103250_e155189_d_n9, assign103250_e155189_d_n10, assign103250_e155189_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103250_e155187: f64 = (1.0 / locals.var_dnm);
        (assign103250_e155187, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign103250_e155189;
        locals.var_dnm_dn0 = assign103250_e155189_d_n0;
        locals.var_dnm_dn2 = assign103250_e155189_d_n2;
        locals.var_dnm_dn4 = assign103250_e155189_d_n4;
        locals.var_dnm_dn5 = assign103250_e155189_d_n5;
        locals.var_dnm_dn6 = assign103250_e155189_d_n6;
        locals.var_dnm_dn7 = assign103250_e155189_d_n7;
        locals.var_dnm_dn8 = assign103250_e155189_d_n8;
        locals.var_dnm_dn9 = assign103250_e155189_d_n9;
        locals.var_dnm_dn10 = assign103250_e155189_d_n10;
        locals.var_dnm_dn13 = assign103250_e155189_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign103260_e155202, assign103260_e155202_d_n0, assign103260_e155202_d_n2, assign103260_e155202_d_n4, assign103260_e155202_d_n5, assign103260_e155202_d_n6, assign103260_e155202_d_n7, assign103260_e155202_d_n8, assign103260_e155202_d_n9, assign103260_e155202_d_n10, assign103260_e155202_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103260_e155198: f64 = (locals.var_tmf1 * 1e-25);
        let assign103260_e155200: f64 = (assign103260_e155198 * locals.var_dnm);
        (assign103260_e155200, (((locals.var_tmf1_dn0 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign103260_e155202;
        locals.var_tmf0_dn0 = assign103260_e155202_d_n0;
        locals.var_tmf0_dn2 = assign103260_e155202_d_n2;
        locals.var_tmf0_dn4 = assign103260_e155202_d_n4;
        locals.var_tmf0_dn5 = assign103260_e155202_d_n5;
        locals.var_tmf0_dn6 = assign103260_e155202_d_n6;
        locals.var_tmf0_dn7 = assign103260_e155202_d_n7;
        locals.var_tmf0_dn8 = assign103260_e155202_d_n8;
        locals.var_tmf0_dn9 = assign103260_e155202_d_n9;
        locals.var_tmf0_dn10 = assign103260_e155202_d_n10;
        locals.var_tmf0_dn13 = assign103260_e155202_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign103270_e155217, assign103270_e155217_d_n0, assign103270_e155217_d_n2, assign103270_e155217_d_n4, assign103270_e155217_d_n5, assign103270_e155217_d_n6, assign103270_e155217_d_n7, assign103270_e155217_d_n8, assign103270_e155217_d_n9, assign103270_e155217_d_n10, assign103270_e155217_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103270_e155211: f64 = (1e-25 * locals.var_xmp);
        let assign103270_e155213: f64 = (assign103270_e155211 * locals.var_dnm);
        let assign103270_e155215: f64 = (assign103270_e155213 / locals.var_arg);
        (assign103270_e155215, ((((((1e-25 * locals.var_xmp_dn0) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn0)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn2) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn2)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn4) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn4)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn5) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn5)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn6) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn6)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn7) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn7)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn8) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn8)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn9) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn9)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn10) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn10)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn13) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn13)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign103270_e155217;
        locals.var_t0_dn0 = assign103270_e155217_d_n0;
        locals.var_t0_dn2 = assign103270_e155217_d_n2;
        locals.var_t0_dn4 = assign103270_e155217_d_n4;
        locals.var_t0_dn5 = assign103270_e155217_d_n5;
        locals.var_t0_dn6 = assign103270_e155217_d_n6;
        locals.var_t0_dn7 = assign103270_e155217_d_n7;
        locals.var_t0_dn8 = assign103270_e155217_d_n8;
        locals.var_t0_dn9 = assign103270_e155217_d_n9;
        locals.var_t0_dn10 = assign103270_e155217_d_n10;
        locals.var_t0_dn13 = assign103270_e155217_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign103280_e155230, assign103280_e155230_d_n0, assign103280_e155230_d_n2, assign103280_e155230_d_n4, assign103280_e155230_d_n5, assign103280_e155230_d_n6, assign103280_e155230_d_n7, assign103280_e155230_d_n8, assign103280_e155230_d_n9, assign103280_e155230_d_n10, assign103280_e155230_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103280_e155226: f64 = 1e-25;
        let assign103280_e155228: f64 = (assign103280_e155226 - locals.var_tmf0);
        (assign103280_e155228, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn13,)
    }
};
        locals.var_gd_s = assign103280_e155230;
        locals.var_gd_s_dn0 = assign103280_e155230_d_n0;
        locals.var_gd_s_dn2 = assign103280_e155230_d_n2;
        locals.var_gd_s_dn4 = assign103280_e155230_d_n4;
        locals.var_gd_s_dn5 = assign103280_e155230_d_n5;
        locals.var_gd_s_dn6 = assign103280_e155230_d_n6;
        locals.var_gd_s_dn7 = assign103280_e155230_d_n7;
        locals.var_gd_s_dn8 = assign103280_e155230_d_n8;
        locals.var_gd_s_dn9 = assign103280_e155230_d_n9;
        locals.var_gd_s_dn10 = assign103280_e155230_d_n10;
        locals.var_gd_s_dn13 = assign103280_e155230_d_n13;
        locals.var_gd_s_rv = 0.0;

        let (assign103290_e155239, assign103290_e155239_d_n0, assign103290_e155239_d_n2, assign103290_e155239_d_n4, assign103290_e155239_d_n5, assign103290_e155239_d_n6, assign103290_e155239_d_n7, assign103290_e155239_d_n8, assign103290_e155239_d_n9, assign103290_e155239_d_n10, assign103290_e155239_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign103290_e155239;
        locals.var_t0_dn0 = assign103290_e155239_d_n0;
        locals.var_t0_dn2 = assign103290_e155239_d_n2;
        locals.var_t0_dn4 = assign103290_e155239_d_n4;
        locals.var_t0_dn5 = assign103290_e155239_d_n5;
        locals.var_t0_dn6 = assign103290_e155239_d_n6;
        locals.var_t0_dn7 = assign103290_e155239_d_n7;
        locals.var_t0_dn8 = assign103290_e155239_d_n8;
        locals.var_t0_dn9 = assign103290_e155239_d_n9;
        locals.var_t0_dn10 = assign103290_e155239_d_n10;
        locals.var_t0_dn13 = assign103290_e155239_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign103300_e155249, assign103300_e155249_d_n0, assign103300_e155249_d_n2, assign103300_e155249_d_n4, assign103300_e155249_d_n5, assign103300_e155249_d_n6, assign103300_e155249_d_n7, assign103300_e155249_d_n8, assign103300_e155249_d_n9, assign103300_e155249_d_n10, assign103300_e155249_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 == 0.0)) {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn13,)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn13,)
    }
};
        locals.var_gd_s = assign103300_e155249;
        locals.var_gd_s_dn0 = assign103300_e155249_d_n0;
        locals.var_gd_s_dn2 = assign103300_e155249_d_n2;
        locals.var_gd_s_dn4 = assign103300_e155249_d_n4;
        locals.var_gd_s_dn5 = assign103300_e155249_d_n5;
        locals.var_gd_s_dn6 = assign103300_e155249_d_n6;
        locals.var_gd_s_dn7 = assign103300_e155249_d_n7;
        locals.var_gd_s_dn8 = assign103300_e155249_d_n8;
        locals.var_gd_s_dn9 = assign103300_e155249_d_n9;
        locals.var_gd_s_dn10 = assign103300_e155249_d_n10;
        locals.var_gd_s_dn13 = assign103300_e155249_d_n13;
        locals.var_gd_s_rv = 0.0;

        let (assign103310_e155259, assign103310_e155259_d_n0, assign103310_e155259_d_n2, assign103310_e155259_d_n4, assign103310_e155259_d_n5, assign103310_e155259_d_n6, assign103310_e155259_d_n7, assign103310_e155259_d_n8, assign103310_e155259_d_n9, assign103310_e155259_d_n10, assign103310_e155259_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign103310_e155259;
        locals.var_t0_dn0 = assign103310_e155259_d_n0;
        locals.var_t0_dn2 = assign103310_e155259_d_n2;
        locals.var_t0_dn4 = assign103310_e155259_d_n4;
        locals.var_t0_dn5 = assign103310_e155259_d_n5;
        locals.var_t0_dn6 = assign103310_e155259_d_n6;
        locals.var_t0_dn7 = assign103310_e155259_d_n7;
        locals.var_t0_dn8 = assign103310_e155259_d_n8;
        locals.var_t0_dn9 = assign103310_e155259_d_n9;
        locals.var_t0_dn10 = assign103310_e155259_d_n10;
        locals.var_t0_dn13 = assign103310_e155259_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign103320_e155268, assign103320_e155268_d_n0, assign103320_e155268_d_n2, assign103320_e155268_d_n4, assign103320_e155268_d_n5, assign103320_e155268_d_n6, assign103320_e155268_d_n7, assign103320_e155268_d_n8, assign103320_e155268_d_n9, assign103320_e155268_d_n10, assign103320_e155268_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign103320_e155266: f64 = (1.0 / locals.var_gd_s);
        (assign103320_e155266, (-(locals.var_gd_s_dn0 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn2 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn4 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn5 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn6 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn7 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn8 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn9 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn10 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn13 / (locals.var_gd_s * locals.var_gd_s))),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13,)
    }
};
        locals.var_rsd = assign103320_e155268;
        locals.var_rsd_dn0 = assign103320_e155268_d_n0;
        locals.var_rsd_dn2 = assign103320_e155268_d_n2;
        locals.var_rsd_dn4 = assign103320_e155268_d_n4;
        locals.var_rsd_dn5 = assign103320_e155268_d_n5;
        locals.var_rsd_dn6 = assign103320_e155268_d_n6;
        locals.var_rsd_dn7 = assign103320_e155268_d_n7;
        locals.var_rsd_dn8 = assign103320_e155268_d_n8;
        locals.var_rsd_dn9 = assign103320_e155268_d_n9;
        locals.var_rsd_dn10 = assign103320_e155268_d_n10;
        locals.var_rsd_dn13 = assign103320_e155268_d_n13;
        locals.var_rsd_rv = 0.0;

        let (assign103330_e155277, assign103330_e155277_d_n0, assign103330_e155277_d_n2, assign103330_e155277_d_n4, assign103330_e155277_d_n5, assign103330_e155277_d_n6, assign103330_e155277_d_n7, assign103330_e155277_d_n8, assign103330_e155277_d_n9, assign103330_e155277_d_n10, assign103330_e155277_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign103330_e155275: f64 = (locals.var_rsd / locals.var_weffld_nf);
        (assign103330_e155275, (locals.var_rsd_dn0 / locals.var_weffld_nf), (locals.var_rsd_dn2 / locals.var_weffld_nf), (locals.var_rsd_dn4 / locals.var_weffld_nf), (locals.var_rsd_dn5 / locals.var_weffld_nf), (locals.var_rsd_dn6 / locals.var_weffld_nf), (locals.var_rsd_dn7 / locals.var_weffld_nf), (locals.var_rsd_dn8 / locals.var_weffld_nf), (locals.var_rsd_dn9 / locals.var_weffld_nf), (locals.var_rsd_dn10 / locals.var_weffld_nf), (locals.var_rsd_dn13 / locals.var_weffld_nf),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13,)
    }
};
        locals.var_rsd = assign103330_e155277;
        locals.var_rsd_dn0 = assign103330_e155277_d_n0;
        locals.var_rsd_dn2 = assign103330_e155277_d_n2;
        locals.var_rsd_dn4 = assign103330_e155277_d_n4;
        locals.var_rsd_dn5 = assign103330_e155277_d_n5;
        locals.var_rsd_dn6 = assign103330_e155277_d_n6;
        locals.var_rsd_dn7 = assign103330_e155277_d_n7;
        locals.var_rsd_dn8 = assign103330_e155277_d_n8;
        locals.var_rsd_dn9 = assign103330_e155277_d_n9;
        locals.var_rsd_dn10 = assign103330_e155277_d_n10;
        locals.var_rsd_dn13 = assign103330_e155277_d_n13;
        locals.var_rsd_rv = 0.0;

        let (assign103340_e155286, assign103340_e155286_d_n0, assign103340_e155286_d_n2, assign103340_e155286_d_n4, assign103340_e155286_d_n5, assign103340_e155286_d_n6, assign103340_e155286_d_n7, assign103340_e155286_d_n8, assign103340_e155286_d_n9, assign103340_e155286_d_n10, assign103340_e155286_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign103340_e155284: f64 = (locals.var_rsd + locals.var_rs0);
        (assign103340_e155284, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13,)
    }
};
        locals.var_rsd = assign103340_e155286;
        locals.var_rsd_dn0 = assign103340_e155286_d_n0;
        locals.var_rsd_dn2 = assign103340_e155286_d_n2;
        locals.var_rsd_dn4 = assign103340_e155286_d_n4;
        locals.var_rsd_dn5 = assign103340_e155286_d_n5;
        locals.var_rsd_dn6 = assign103340_e155286_d_n6;
        locals.var_rsd_dn7 = assign103340_e155286_d_n7;
        locals.var_rsd_dn8 = assign103340_e155286_d_n8;
        locals.var_rsd_dn9 = assign103340_e155286_d_n9;
        locals.var_rsd_dn10 = assign103340_e155286_d_n10;
        locals.var_rsd_dn13 = assign103340_e155286_d_n13;
        locals.var_rsd_rv = 0.0;

        let assign103380_e155317: f64 = if locals.var_rsd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2351 = assign103380_e155317;
        locals.var_guard2351_rv = 0.0;

        let (assign103390_e155326, assign103390_e155326_d_n0, assign103390_e155326_d_n2, assign103390_e155326_d_n4, assign103390_e155326_d_n5, assign103390_e155326_d_n6, assign103390_e155326_d_n7, assign103390_e155326_d_n8, assign103390_e155326_d_n9, assign103390_e155326_d_n10, assign103390_e155326_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2351 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13,)
    }
};
        locals.var_rsd = assign103390_e155326;
        locals.var_rsd_dn0 = assign103390_e155326_d_n0;
        locals.var_rsd_dn2 = assign103390_e155326_d_n2;
        locals.var_rsd_dn4 = assign103390_e155326_d_n4;
        locals.var_rsd_dn5 = assign103390_e155326_d_n5;
        locals.var_rsd_dn6 = assign103390_e155326_d_n6;
        locals.var_rsd_dn7 = assign103390_e155326_d_n7;
        locals.var_rsd_dn8 = assign103390_e155326_d_n8;
        locals.var_rsd_dn9 = assign103390_e155326_d_n9;
        locals.var_rsd_dn10 = assign103390_e155326_d_n10;
        locals.var_rsd_dn13 = assign103390_e155326_d_n13;
        locals.var_rsd_rv = 0.0;

        let (assign103400_e155335, assign103400_e155335_d_n0, assign103400_e155335_d_n2, assign103400_e155335_d_n4, assign103400_e155335_d_n5, assign103400_e155335_d_n6, assign103400_e155335_d_n7, assign103400_e155335_d_n8, assign103400_e155335_d_n9, assign103400_e155335_d_n10, assign103400_e155335_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign103400_e155333: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign103400_e155333, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn13 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn13,)
    }
};
        locals.var_rsde = assign103400_e155335;
        locals.var_rsde_dn0 = assign103400_e155335_d_n0;
        locals.var_rsde_dn2 = assign103400_e155335_d_n2;
        locals.var_rsde_dn4 = assign103400_e155335_d_n4;
        locals.var_rsde_dn5 = assign103400_e155335_d_n5;
        locals.var_rsde_dn6 = assign103400_e155335_d_n6;
        locals.var_rsde_dn7 = assign103400_e155335_d_n7;
        locals.var_rsde_dn8 = assign103400_e155335_d_n8;
        locals.var_rsde_dn9 = assign103400_e155335_d_n9;
        locals.var_rsde_dn10 = assign103400_e155335_d_n10;
        locals.var_rsde_dn13 = assign103400_e155335_d_n13;
        locals.var_rsde_rv = 0.0;

        let assign103410_e155338: f64 = if locals.var_flg_rd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2356 = assign103410_e155338;
        locals.var_guard2356_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_383(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign103420_e155345, assign103420_e155345_d_n5, assign103420_e155345_d_n7,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        (locals.var_vdsi, locals.var_vdsi_dn5, locals.var_vdsi_dn7,)
    } else {
        (locals.var_vds__blk2352, locals.var_vds__blk2352_dn5, locals.var_vds__blk2352_dn7,)
    }
};
        locals.var_vds__blk2352 = assign103420_e155345;
        locals.var_vds__blk2352_dn5 = assign103420_e155345_d_n5;
        locals.var_vds__blk2352_dn7 = assign103420_e155345_d_n7;
        locals.var_vds__blk2352_rv = 0.0;

        let (assign103430_e155352, assign103430_e155352_d_n7, assign103430_e155352_d_n8,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        (locals.var_vbsi, locals.var_vbsi_dn7, locals.var_vbsi_dn8,)
    } else {
        (locals.var_vbs__blk2353, locals.var_vbs__blk2353_dn7, locals.var_vbs__blk2353_dn8,)
    }
};
        locals.var_vbs__blk2353 = assign103430_e155352;
        locals.var_vbs__blk2353_dn7 = assign103430_e155352_d_n7;
        locals.var_vbs__blk2353_dn8 = assign103430_e155352_d_n8;
        locals.var_vbs__blk2353_rv = 0.0;

        let assign103440_e155359: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2357 = assign103440_e155359;
        locals.var_guard2357_rv = 0.0;

        let (assign103450_e155375, assign103450_e155375_d_n0, assign103450_e155375_d_n2, assign103450_e155375_d_n4, assign103450_e155375_d_n5, assign103450_e155375_d_n6, assign103450_e155375_d_n7, assign103450_e155375_d_n8, assign103450_e155375_d_n9, assign103450_e155375_d_n10, assign103450_e155375_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) {
        let (assign103450_e155373, assign103450_e155373_d_n0, assign103450_e155373_d_n2, assign103450_e155373_d_n4, assign103450_e155373_d_n5, assign103450_e155373_d_n6, assign103450_e155373_d_n7, assign103450_e155373_d_n8, assign103450_e155373_d_n9, assign103450_e155373_d_n10, assign103450_e155373_d_n13,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign103450_e155372: f64 = (locals.var_tratio).powf(p.p415);
                (assign103450_e155372, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn0)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn2)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn4)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn5)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn6)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn7)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn8)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn9)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn10)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn13)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn13 / locals.var_tratio))) },)
            }
        };
        (assign103450_e155373, assign103450_e155373_d_n0, assign103450_e155373_d_n2, assign103450_e155373_d_n4, assign103450_e155373_d_n5, assign103450_e155373_d_n6, assign103450_e155373_d_n7, assign103450_e155373_d_n8, assign103450_e155373_d_n9, assign103450_e155373_d_n10, assign103450_e155373_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign103450_e155375;
        locals.var_t1_dn0 = assign103450_e155375_d_n0;
        locals.var_t1_dn2 = assign103450_e155375_d_n2;
        locals.var_t1_dn4 = assign103450_e155375_d_n4;
        locals.var_t1_dn5 = assign103450_e155375_d_n5;
        locals.var_t1_dn6 = assign103450_e155375_d_n6;
        locals.var_t1_dn7 = assign103450_e155375_d_n7;
        locals.var_t1_dn8 = assign103450_e155375_d_n8;
        locals.var_t1_dn9 = assign103450_e155375_d_n9;
        locals.var_t1_dn10 = assign103450_e155375_d_n10;
        locals.var_t1_dn13 = assign103450_e155375_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign103460_e155386, assign103460_e155386_d_n0, assign103460_e155386_d_n2, assign103460_e155386_d_n4, assign103460_e155386_d_n5, assign103460_e155386_d_n6, assign103460_e155386_d_n7, assign103460_e155386_d_n8, assign103460_e155386_d_n9, assign103460_e155386_d_n10, assign103460_e155386_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) {
        let assign103460_e155384: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign103460_e155384, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmue, locals.var_rrdrmue_dn0, locals.var_rrdrmue_dn2, locals.var_rrdrmue_dn4, locals.var_rrdrmue_dn5, locals.var_rrdrmue_dn6, locals.var_rrdrmue_dn7, locals.var_rrdrmue_dn8, locals.var_rrdrmue_dn9, locals.var_rrdrmue_dn10, locals.var_rrdrmue_dn13,)
    }
};
        locals.var_rrdrmue = assign103460_e155386;
        locals.var_rrdrmue_dn0 = assign103460_e155386_d_n0;
        locals.var_rrdrmue_dn2 = assign103460_e155386_d_n2;
        locals.var_rrdrmue_dn4 = assign103460_e155386_d_n4;
        locals.var_rrdrmue_dn5 = assign103460_e155386_d_n5;
        locals.var_rrdrmue_dn6 = assign103460_e155386_d_n6;
        locals.var_rrdrmue_dn7 = assign103460_e155386_d_n7;
        locals.var_rrdrmue_dn8 = assign103460_e155386_d_n8;
        locals.var_rrdrmue_dn9 = assign103460_e155386_d_n9;
        locals.var_rrdrmue_dn10 = assign103460_e155386_d_n10;
        locals.var_rrdrmue_dn13 = assign103460_e155386_d_n13;
        locals.var_rrdrmue_rv = 0.0;

        let (assign103470_e155411, assign103470_e155411_d_n0, assign103470_e155411_d_n2, assign103470_e155411_d_n4, assign103470_e155411_d_n5, assign103470_e155411_d_n6, assign103470_e155411_d_n7, assign103470_e155411_d_n8, assign103470_e155411_d_n9, assign103470_e155411_d_n10, assign103470_e155411_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) {
        let assign103470_e155396: f64 = (0.4 * locals.var_tratio);
        let assign103470_e155397: f64 = (1.8 + assign103470_e155396);
        let assign103470_e155400: f64 = (0.1 * locals.var_tratio);
        let assign103470_e155402: f64 = (assign103470_e155400 * locals.var_tratio);
        let assign103470_e155403: f64 = (assign103470_e155397 + assign103470_e155402);
        let assign103470_e155407: f64 = (1.0 - locals.var_tratio);
        let assign103470_e155408: f64 = (p.p417 * assign103470_e155407);
        let assign103470_e155409: f64 = (assign103470_e155403 - assign103470_e155408);
        (assign103470_e155409, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn0))) - (p.p417 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn2))) - (p.p417 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn4))) - (p.p417 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn5))) - (p.p417 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn6))) - (p.p417 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn7))) - (p.p417 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn8))) - (p.p417 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn9))) - (p.p417 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn10))) - (p.p417 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn13))) - (p.p417 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign103470_e155411;
        locals.var_t0_dn0 = assign103470_e155411_d_n0;
        locals.var_t0_dn2 = assign103470_e155411_d_n2;
        locals.var_t0_dn4 = assign103470_e155411_d_n4;
        locals.var_t0_dn5 = assign103470_e155411_d_n5;
        locals.var_t0_dn6 = assign103470_e155411_d_n6;
        locals.var_t0_dn7 = assign103470_e155411_d_n7;
        locals.var_t0_dn8 = assign103470_e155411_d_n8;
        locals.var_t0_dn9 = assign103470_e155411_d_n9;
        locals.var_t0_dn10 = assign103470_e155411_d_n10;
        locals.var_t0_dn13 = assign103470_e155411_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign103480_e155422, assign103480_e155422_d_n0, assign103480_e155422_d_n2, assign103480_e155422_d_n4, assign103480_e155422_d_n5, assign103480_e155422_d_n6, assign103480_e155422_d_n7, assign103480_e155422_d_n8, assign103480_e155422_d_n9, assign103480_e155422_d_n10, assign103480_e155422_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) {
        let assign103480_e155420: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign103480_e155420, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmax, locals.var_rrdrvmax_dn0, locals.var_rrdrvmax_dn2, locals.var_rrdrvmax_dn4, locals.var_rrdrvmax_dn5, locals.var_rrdrvmax_dn6, locals.var_rrdrvmax_dn7, locals.var_rrdrvmax_dn8, locals.var_rrdrvmax_dn9, locals.var_rrdrvmax_dn10, locals.var_rrdrvmax_dn13,)
    }
};
        locals.var_rrdrvmax = assign103480_e155422;
        locals.var_rrdrvmax_dn0 = assign103480_e155422_d_n0;
        locals.var_rrdrvmax_dn2 = assign103480_e155422_d_n2;
        locals.var_rrdrvmax_dn4 = assign103480_e155422_d_n4;
        locals.var_rrdrvmax_dn5 = assign103480_e155422_d_n5;
        locals.var_rrdrvmax_dn6 = assign103480_e155422_d_n6;
        locals.var_rrdrvmax_dn7 = assign103480_e155422_d_n7;
        locals.var_rrdrvmax_dn8 = assign103480_e155422_d_n8;
        locals.var_rrdrvmax_dn9 = assign103480_e155422_d_n9;
        locals.var_rrdrvmax_dn10 = assign103480_e155422_d_n10;
        locals.var_rrdrvmax_dn13 = assign103480_e155422_d_n13;
        locals.var_rrdrvmax_rv = 0.0;

        let (assign103490_e155437, assign103490_e155437_d_n0, assign103490_e155437_d_n2, assign103490_e155437_d_n4, assign103490_e155437_d_n5, assign103490_e155437_d_n6, assign103490_e155437_d_n7, assign103490_e155437_d_n8, assign103490_e155437_d_n9, assign103490_e155437_d_n10, assign103490_e155437_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) {
        let assign103490_e155433: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign103490_e155434: f64 = (p.p438 * assign103490_e155433);
        let assign103490_e155435: f64 = (locals.var_uc_rdrbb + assign103490_e155434);
        (assign103490_e155435, (locals.var_uc_rdrbb_dn0 + (p.p438 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_dn2 + (p.p438 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_dn4 + (p.p438 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_dn5 + (p.p438 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_dn6 + (p.p438 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_dn7 + (p.p438 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_dn8 + (p.p438 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_dn9 + (p.p438 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_dn10 + (p.p438 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_dn13 + (p.p438 * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn13,)
    }
};
        locals.var_uc_rdrbb = assign103490_e155437;
        locals.var_uc_rdrbb_dn0 = assign103490_e155437_d_n0;
        locals.var_uc_rdrbb_dn2 = assign103490_e155437_d_n2;
        locals.var_uc_rdrbb_dn4 = assign103490_e155437_d_n4;
        locals.var_uc_rdrbb_dn5 = assign103490_e155437_d_n5;
        locals.var_uc_rdrbb_dn6 = assign103490_e155437_d_n6;
        locals.var_uc_rdrbb_dn7 = assign103490_e155437_d_n7;
        locals.var_uc_rdrbb_dn8 = assign103490_e155437_d_n8;
        locals.var_uc_rdrbb_dn9 = assign103490_e155437_d_n9;
        locals.var_uc_rdrbb_dn10 = assign103490_e155437_d_n10;
        locals.var_uc_rdrbb_dn13 = assign103490_e155437_d_n13;
        locals.var_uc_rdrbb_rv = 0.0;

        let assign103510_e155445: f64 = if locals.var_uc_rdrbb < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard2359 = assign103510_e155445;
        locals.var_guard2359_rv = 0.0;

        let (assign103520_e155456, assign103520_e155456_d_n0, assign103520_e155456_d_n2, assign103520_e155456_d_n4, assign103520_e155456_d_n5, assign103520_e155456_d_n6, assign103520_e155456_d_n7, assign103520_e155456_d_n8, assign103520_e155456_d_n9, assign103520_e155456_d_n10, assign103520_e155456_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) && (locals.var_guard2359 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn13,)
    }
};
        locals.var_uc_rdrbb = assign103520_e155456;
        locals.var_uc_rdrbb_dn0 = assign103520_e155456_d_n0;
        locals.var_uc_rdrbb_dn2 = assign103520_e155456_d_n2;
        locals.var_uc_rdrbb_dn4 = assign103520_e155456_d_n4;
        locals.var_uc_rdrbb_dn5 = assign103520_e155456_d_n5;
        locals.var_uc_rdrbb_dn6 = assign103520_e155456_d_n6;
        locals.var_uc_rdrbb_dn7 = assign103520_e155456_d_n7;
        locals.var_uc_rdrbb_dn8 = assign103520_e155456_d_n8;
        locals.var_uc_rdrbb_dn9 = assign103520_e155456_d_n9;
        locals.var_uc_rdrbb_dn10 = assign103520_e155456_d_n10;
        locals.var_uc_rdrbb_dn13 = assign103520_e155456_d_n13;
        locals.var_uc_rdrbb_rv = 0.0;

        let (assign103530_e155468, assign103530_e155468_d_n0, assign103530_e155468_d_n2, assign103530_e155468_d_n4, assign103530_e155468_d_n5, assign103530_e155468_d_n6, assign103530_e155468_d_n7, assign103530_e155468_d_n8, assign103530_e155468_d_n9, assign103530_e155468_d_n10, assign103530_e155468_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 == 0.0)) {
        let assign103530_e155464: f64 = ctx_temp;
        let assign103530_e155466: f64 = (assign103530_e155464 + p.p11);
        (assign103530_e155466, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign103530_e155468;
        locals.var_ttemp_dn0 = assign103530_e155468_d_n0;
        locals.var_ttemp_dn2 = assign103530_e155468_d_n2;
        locals.var_ttemp_dn4 = assign103530_e155468_d_n4;
        locals.var_ttemp_dn5 = assign103530_e155468_d_n5;
        locals.var_ttemp_dn6 = assign103530_e155468_d_n6;
        locals.var_ttemp_dn7 = assign103530_e155468_d_n7;
        locals.var_ttemp_dn8 = assign103530_e155468_d_n8;
        locals.var_ttemp_dn9 = assign103530_e155468_d_n9;
        locals.var_ttemp_dn10 = assign103530_e155468_d_n10;
        locals.var_ttemp_dn13 = assign103530_e155468_d_n13;
        locals.var_ttemp_rv = 0.0;

        let (assign103540_e155477,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103540_e155475: f64 = (locals.var_weff_ld * p.p7);
        (assign103540_e155475,)
    } else {
        (locals.var_weffld_nf,)
    }
};
        locals.var_weffld_nf = assign103540_e155477;
        locals.var_weffld_nf_rv = 0.0;

        let (assign103550_e155486,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103550_e155484: f64 = (p.p67 + p.p68);
        (assign103550_e155484,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign103550_e155486;
        locals.var_ldrifte_rv = 0.0;

        let (assign103560_e155495,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103560_e155493: f64 = (locals.var_uc_xldld + 1e-12);
        (assign103560_e155493,)
    } else {
        (locals.var_rd_xldld,)
    }
};
        locals.var_rd_xldld = assign103560_e155495;
        locals.var_rd_xldld_rv = 0.0;

        let (assign103570_e155502,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_noverd,)
    }
};
        locals.var_noverd = assign103570_e155502;
        locals.var_noverd_rv = 0.0;

        let (assign103580_e155517, assign103580_e155517_d_n0, assign103580_e155517_d_n2, assign103580_e155517_d_n4, assign103580_e155517_d_n5, assign103580_e155517_d_n6, assign103580_e155517_d_n7, assign103580_e155517_d_n8, assign103580_e155517_d_n9, assign103580_e155517_d_n10, assign103580_e155517_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103580_e155512: f64 = (p.p411 * locals.var_vbs__blk2353);
        let assign103580_e155513: f64 = (p.p410 - assign103580_e155512);
        let assign103580_e155514: f64 = (locals.var_vbs__blk2353 * assign103580_e155513);
        let assign103580_e155515: f64 = (1.0 + assign103580_e155514);
        (assign103580_e155515, 0.0, 0.0, 0.0, 0.0, 0.0, ((locals.var_vbs__blk2353_dn7 * assign103580_e155513) + (locals.var_vbs__blk2353 * (-(p.p411 * locals.var_vbs__blk2353_dn7)))), ((locals.var_vbs__blk2353_dn8 * assign103580_e155513) + (locals.var_vbs__blk2353 * (-(p.p411 * locals.var_vbs__blk2353_dn8)))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign103580_e155517;
        locals.var_t1_dn0 = assign103580_e155517_d_n0;
        locals.var_t1_dn2 = assign103580_e155517_d_n2;
        locals.var_t1_dn4 = assign103580_e155517_d_n4;
        locals.var_t1_dn5 = assign103580_e155517_d_n5;
        locals.var_t1_dn6 = assign103580_e155517_d_n6;
        locals.var_t1_dn7 = assign103580_e155517_d_n7;
        locals.var_t1_dn8 = assign103580_e155517_d_n8;
        locals.var_t1_dn9 = assign103580_e155517_d_n9;
        locals.var_t1_dn10 = assign103580_e155517_d_n10;
        locals.var_t1_dn13 = assign103580_e155517_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign103590_e155533, assign103590_e155533_d_n0, assign103590_e155533_d_n2, assign103590_e155533_d_n4, assign103590_e155533_d_n5, assign103590_e155533_d_n6, assign103590_e155533_d_n7, assign103590_e155533_d_n8, assign103590_e155533_d_n9, assign103590_e155533_d_n10, assign103590_e155533_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103590_e155524: f64 = (locals.var_t1 * locals.var_t1);
        let assign103590_e155527: f64 = (4.0 * 0.1);
        let assign103590_e155529: f64 = (assign103590_e155527 * 0.1);
        let assign103590_e155530: f64 = (assign103590_e155524 + assign103590_e155529);
        let assign103590_e155531: f64 = (assign103590_e155530).sqrt();
        (assign103590_e155531, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign103590_e155531)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign103590_e155533;
        locals.var_tmf2_dn0 = assign103590_e155533_d_n0;
        locals.var_tmf2_dn2 = assign103590_e155533_d_n2;
        locals.var_tmf2_dn4 = assign103590_e155533_d_n4;
        locals.var_tmf2_dn5 = assign103590_e155533_d_n5;
        locals.var_tmf2_dn6 = assign103590_e155533_d_n6;
        locals.var_tmf2_dn7 = assign103590_e155533_d_n7;
        locals.var_tmf2_dn8 = assign103590_e155533_d_n8;
        locals.var_tmf2_dn9 = assign103590_e155533_d_n9;
        locals.var_tmf2_dn10 = assign103590_e155533_d_n10;
        locals.var_tmf2_dn13 = assign103590_e155533_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign103600_e155546, assign103600_e155546_d_n0, assign103600_e155546_d_n2, assign103600_e155546_d_n4, assign103600_e155546_d_n5, assign103600_e155546_d_n6, assign103600_e155546_d_n7, assign103600_e155546_d_n8, assign103600_e155546_d_n9, assign103600_e155546_d_n10, assign103600_e155546_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103600_e155542: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign103600_e155543: f64 = (1.0 + assign103600_e155542);
        let assign103600_e155544: f64 = (0.5 * assign103600_e155543);
        (assign103600_e155544, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign103600_e155546;
        locals.var_t2_dn0 = assign103600_e155546_d_n0;
        locals.var_t2_dn2 = assign103600_e155546_d_n2;
        locals.var_t2_dn4 = assign103600_e155546_d_n4;
        locals.var_t2_dn5 = assign103600_e155546_d_n5;
        locals.var_t2_dn6 = assign103600_e155546_d_n6;
        locals.var_t2_dn7 = assign103600_e155546_d_n7;
        locals.var_t2_dn8 = assign103600_e155546_d_n8;
        locals.var_t2_dn9 = assign103600_e155546_d_n9;
        locals.var_t2_dn10 = assign103600_e155546_d_n10;
        locals.var_t2_dn13 = assign103600_e155546_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign103610_e155557, assign103610_e155557_d_n0, assign103610_e155557_d_n2, assign103610_e155557_d_n4, assign103610_e155557_d_n5, assign103610_e155557_d_n6, assign103610_e155557_d_n7, assign103610_e155557_d_n8, assign103610_e155557_d_n9, assign103610_e155557_d_n10, assign103610_e155557_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103610_e155554: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign103610_e155555: f64 = (0.5 * assign103610_e155554);
        (assign103610_e155555, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn13,)
    }
};
        locals.var_rdrmuevbs = assign103610_e155557;
        locals.var_rdrmuevbs_dn0 = assign103610_e155557_d_n0;
        locals.var_rdrmuevbs_dn2 = assign103610_e155557_d_n2;
        locals.var_rdrmuevbs_dn4 = assign103610_e155557_d_n4;
        locals.var_rdrmuevbs_dn5 = assign103610_e155557_d_n5;
        locals.var_rdrmuevbs_dn6 = assign103610_e155557_d_n6;
        locals.var_rdrmuevbs_dn7 = assign103610_e155557_d_n7;
        locals.var_rdrmuevbs_dn8 = assign103610_e155557_d_n8;
        locals.var_rdrmuevbs_dn9 = assign103610_e155557_d_n9;
        locals.var_rdrmuevbs_dn10 = assign103610_e155557_d_n10;
        locals.var_rdrmuevbs_dn13 = assign103610_e155557_d_n13;
        locals.var_rdrmuevbs_rv = 0.0;

        let assign103620_e155560: f64 = if locals.var_rdrmuevbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2360 = assign103620_e155560;
        locals.var_guard2360_rv = 0.0;

        let (assign103630_e155569, assign103630_e155569_d_n0, assign103630_e155569_d_n2, assign103630_e155569_d_n4, assign103630_e155569_d_n5, assign103630_e155569_d_n6, assign103630_e155569_d_n7, assign103630_e155569_d_n8, assign103630_e155569_d_n9, assign103630_e155569_d_n10, assign103630_e155569_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2360 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn13,)
    }
};
        locals.var_rdrmuevbs = assign103630_e155569;
        locals.var_rdrmuevbs_dn0 = assign103630_e155569_d_n0;
        locals.var_rdrmuevbs_dn2 = assign103630_e155569_d_n2;
        locals.var_rdrmuevbs_dn4 = assign103630_e155569_d_n4;
        locals.var_rdrmuevbs_dn5 = assign103630_e155569_d_n5;
        locals.var_rdrmuevbs_dn6 = assign103630_e155569_d_n6;
        locals.var_rdrmuevbs_dn7 = assign103630_e155569_d_n7;
        locals.var_rdrmuevbs_dn8 = assign103630_e155569_d_n8;
        locals.var_rdrmuevbs_dn9 = assign103630_e155569_d_n9;
        locals.var_rdrmuevbs_dn10 = assign103630_e155569_d_n10;
        locals.var_rdrmuevbs_dn13 = assign103630_e155569_d_n13;
        locals.var_rdrmuevbs_rv = 0.0;

        let (assign103640_e155578, assign103640_e155578_d_n0, assign103640_e155578_d_n2, assign103640_e155578_d_n4, assign103640_e155578_d_n5, assign103640_e155578_d_n6, assign103640_e155578_d_n7, assign103640_e155578_d_n8, assign103640_e155578_d_n9, assign103640_e155578_d_n10, assign103640_e155578_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2360 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign103640_e155578;
        locals.var_t2_dn0 = assign103640_e155578_d_n0;
        locals.var_t2_dn2 = assign103640_e155578_d_n2;
        locals.var_t2_dn4 = assign103640_e155578_d_n4;
        locals.var_t2_dn5 = assign103640_e155578_d_n5;
        locals.var_t2_dn6 = assign103640_e155578_d_n6;
        locals.var_t2_dn7 = assign103640_e155578_d_n7;
        locals.var_t2_dn8 = assign103640_e155578_d_n8;
        locals.var_t2_dn9 = assign103640_e155578_d_n9;
        locals.var_t2_dn10 = assign103640_e155578_d_n10;
        locals.var_t2_dn13 = assign103640_e155578_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign103650_e155589, assign103650_e155589_d_n0, assign103650_e155589_d_n2, assign103650_e155589_d_n4, assign103650_e155589_d_n5, assign103650_e155589_d_n6, assign103650_e155589_d_n7, assign103650_e155589_d_n8, assign103650_e155589_d_n9, assign103650_e155589_d_n10, assign103650_e155589_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103650_e155585: f64 = (locals.var_rrdrmue * locals.var_rdrmuele);
        let assign103650_e155587: f64 = (assign103650_e155585 * locals.var_rdrmuevbs);
        (assign103650_e155587, (((locals.var_rrdrmue_dn0 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn0)), (((locals.var_rrdrmue_dn2 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn2)), (((locals.var_rrdrmue_dn4 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn4)), (((locals.var_rrdrmue_dn5 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn5)), (((locals.var_rrdrmue_dn6 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn6)), (((locals.var_rrdrmue_dn7 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn7)), (((locals.var_rrdrmue_dn8 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn8)), (((locals.var_rrdrmue_dn9 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn9)), (((locals.var_rrdrmue_dn10 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn10)), (((locals.var_rrdrmue_dn13 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn13)),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn4, locals.var_mu0_dn5, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn8, locals.var_mu0_dn9, locals.var_mu0_dn10, locals.var_mu0_dn13,)
    }
};
        locals.var_mu0 = assign103650_e155589;
        locals.var_mu0_dn0 = assign103650_e155589_d_n0;
        locals.var_mu0_dn2 = assign103650_e155589_d_n2;
        locals.var_mu0_dn4 = assign103650_e155589_d_n4;
        locals.var_mu0_dn5 = assign103650_e155589_d_n5;
        locals.var_mu0_dn6 = assign103650_e155589_d_n6;
        locals.var_mu0_dn7 = assign103650_e155589_d_n7;
        locals.var_mu0_dn8 = assign103650_e155589_d_n8;
        locals.var_mu0_dn9 = assign103650_e155589_d_n9;
        locals.var_mu0_dn10 = assign103650_e155589_d_n10;
        locals.var_mu0_dn13 = assign103650_e155589_d_n13;
        locals.var_mu0_rv = 0.0;

        let (assign103660_e155602, assign103660_e155602_d_n0, assign103660_e155602_d_n2, assign103660_e155602_d_n4, assign103660_e155602_d_n5, assign103660_e155602_d_n6, assign103660_e155602_d_n7, assign103660_e155602_d_n8, assign103660_e155602_d_n9, assign103660_e155602_d_n10, assign103660_e155602_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103660_e155596: f64 = (locals.var_rrdrvmax * locals.var_rdrvmaxwe);
        let assign103660_e155598: f64 = (assign103660_e155596 * locals.var_rdrvmaxle);
        let assign103660_e155600: f64 = (assign103660_e155598 + 1e-25);
        (assign103660_e155600, ((locals.var_rrdrvmax_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn9 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn13 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe__blk2355, locals.var_vmaxe__blk2355_dn0, locals.var_vmaxe__blk2355_dn2, locals.var_vmaxe__blk2355_dn4, locals.var_vmaxe__blk2355_dn5, locals.var_vmaxe__blk2355_dn6, locals.var_vmaxe__blk2355_dn7, locals.var_vmaxe__blk2355_dn8, locals.var_vmaxe__blk2355_dn9, locals.var_vmaxe__blk2355_dn10, locals.var_vmaxe__blk2355_dn13,)
    }
};
        locals.var_vmaxe__blk2355 = assign103660_e155602;
        locals.var_vmaxe__blk2355_dn0 = assign103660_e155602_d_n0;
        locals.var_vmaxe__blk2355_dn2 = assign103660_e155602_d_n2;
        locals.var_vmaxe__blk2355_dn4 = assign103660_e155602_d_n4;
        locals.var_vmaxe__blk2355_dn5 = assign103660_e155602_d_n5;
        locals.var_vmaxe__blk2355_dn6 = assign103660_e155602_d_n6;
        locals.var_vmaxe__blk2355_dn7 = assign103660_e155602_d_n7;
        locals.var_vmaxe__blk2355_dn8 = assign103660_e155602_d_n8;
        locals.var_vmaxe__blk2355_dn9 = assign103660_e155602_d_n9;
        locals.var_vmaxe__blk2355_dn10 = assign103660_e155602_d_n10;
        locals.var_vmaxe__blk2355_dn13 = assign103660_e155602_d_n13;
        locals.var_vmaxe__blk2355_rv = 0.0;

        let (assign103670_e155609,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        (locals.var_uc_rdrcx,)
    } else {
        (locals.var_cx,)
    }
};
        locals.var_cx = assign103670_e155609;
        locals.var_cx_rv = 0.0;

        let (assign103680_e155616,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        (p.p421,)
    } else {
        (locals.var_car,)
    }
};
        locals.var_car = assign103680_e155616;
        locals.var_car_rv = 0.0;

        let (assign103690_e155625, assign103690_e155625_d_n0, assign103690_e155625_d_n2, assign103690_e155625_d_n4, assign103690_e155625_d_n5, assign103690_e155625_d_n6, assign103690_e155625_d_n7, assign103690_e155625_d_n8, assign103690_e155625_d_n9, assign103690_e155625_d_n10, assign103690_e155625_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103690_e155623: f64 = (locals.var_mu0 * 10000.0);
        (assign103690_e155623, (locals.var_mu0_dn0 * 10000.0), (locals.var_mu0_dn2 * 10000.0), (locals.var_mu0_dn4 * 10000.0), (locals.var_mu0_dn5 * 10000.0), (locals.var_mu0_dn6 * 10000.0), (locals.var_mu0_dn7 * 10000.0), (locals.var_mu0_dn8 * 10000.0), (locals.var_mu0_dn9 * 10000.0), (locals.var_mu0_dn10 * 10000.0), (locals.var_mu0_dn13 * 10000.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign103690_e155625;
        locals.var_t1_dn0 = assign103690_e155625_d_n0;
        locals.var_t1_dn2 = assign103690_e155625_d_n2;
        locals.var_t1_dn4 = assign103690_e155625_d_n4;
        locals.var_t1_dn5 = assign103690_e155625_d_n5;
        locals.var_t1_dn6 = assign103690_e155625_d_n6;
        locals.var_t1_dn7 = assign103690_e155625_d_n7;
        locals.var_t1_dn8 = assign103690_e155625_d_n8;
        locals.var_t1_dn9 = assign103690_e155625_d_n9;
        locals.var_t1_dn10 = assign103690_e155625_d_n10;
        locals.var_t1_dn13 = assign103690_e155625_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign103700_e155634, assign103700_e155634_d_n0, assign103700_e155634_d_n2, assign103700_e155634_d_n4, assign103700_e155634_d_n5, assign103700_e155634_d_n6, assign103700_e155634_d_n7, assign103700_e155634_d_n8, assign103700_e155634_d_n9, assign103700_e155634_d_n10, assign103700_e155634_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103700_e155632: f64 = (locals.var_vmaxe__blk2355 * 100.0);
        (assign103700_e155632, (locals.var_vmaxe__blk2355_dn0 * 100.0), (locals.var_vmaxe__blk2355_dn2 * 100.0), (locals.var_vmaxe__blk2355_dn4 * 100.0), (locals.var_vmaxe__blk2355_dn5 * 100.0), (locals.var_vmaxe__blk2355_dn6 * 100.0), (locals.var_vmaxe__blk2355_dn7 * 100.0), (locals.var_vmaxe__blk2355_dn8 * 100.0), (locals.var_vmaxe__blk2355_dn9 * 100.0), (locals.var_vmaxe__blk2355_dn10 * 100.0), (locals.var_vmaxe__blk2355_dn13 * 100.0),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign103700_e155634;
        locals.var_t2_dn0 = assign103700_e155634_d_n0;
        locals.var_t2_dn2 = assign103700_e155634_d_n2;
        locals.var_t2_dn4 = assign103700_e155634_d_n4;
        locals.var_t2_dn5 = assign103700_e155634_d_n5;
        locals.var_t2_dn6 = assign103700_e155634_d_n6;
        locals.var_t2_dn7 = assign103700_e155634_d_n7;
        locals.var_t2_dn8 = assign103700_e155634_d_n8;
        locals.var_t2_dn9 = assign103700_e155634_d_n9;
        locals.var_t2_dn10 = assign103700_e155634_d_n10;
        locals.var_t2_dn13 = assign103700_e155634_d_n13;
        locals.var_t2_rv = 0.0;

        let assign103730_e155655: f64 = if locals.var_vddp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2363 = assign103730_e155655;
        locals.var_guard2363_rv = 0.0;

        let (assign103740_e155671, assign103740_e155671_d_n0, assign103740_e155671_d_n2, assign103740_e155671_d_n4, assign103740_e155671_d_n5, assign103740_e155671_d_n6, assign103740_e155671_d_n7, assign103740_e155671_d_n8, assign103740_e155671_d_n9, assign103740_e155671_d_n10, assign103740_e155671_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103740_e155664: f64 = (-locals.var_vddp);
        let assign103740_e155666: f64 = (assign103740_e155664 / 2.0);
        let assign103740_e155667: f64 = (2.0 * assign103740_e155666);
        let assign103740_e155669: f64 = (assign103740_e155667 / p.p262);
        (assign103740_e155669, ((2.0 * ((-locals.var_vddp_dn0) / 2.0)) / p.p262), 0.0, 0.0, ((2.0 * ((-locals.var_vddp_dn5) / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign103740_e155671;
        locals.var_tmf1_dn0 = assign103740_e155671_d_n0;
        locals.var_tmf1_dn2 = assign103740_e155671_d_n2;
        locals.var_tmf1_dn4 = assign103740_e155671_d_n4;
        locals.var_tmf1_dn5 = assign103740_e155671_d_n5;
        locals.var_tmf1_dn6 = assign103740_e155671_d_n6;
        locals.var_tmf1_dn7 = assign103740_e155671_d_n7;
        locals.var_tmf1_dn8 = assign103740_e155671_d_n8;
        locals.var_tmf1_dn9 = assign103740_e155671_d_n9;
        locals.var_tmf1_dn10 = assign103740_e155671_d_n10;
        locals.var_tmf1_dn13 = assign103740_e155671_d_n13;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_384(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign103750_e155716, assign103750_e155716_d_n0, assign103750_e155716_d_n2, assign103750_e155716_d_n4, assign103750_e155716_d_n5, assign103750_e155716_d_n6, assign103750_e155716_d_n7, assign103750_e155716_d_n8, assign103750_e155716_d_n9, assign103750_e155716_d_n10, assign103750_e155716_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103750_e155682: f64 = (1.0 / 2.0);
        let assign103750_e155686: f64 = (1.0 / 6.0);
        let assign103750_e155690: f64 = (1.0 / 24.0);
        let assign103750_e155694: f64 = (1.0 / 120.0);
        let assign103750_e155698: f64 = (1.0 / 720.0);
        let assign103750_e155702: f64 = (1.0 / 5040.0);
        let assign103750_e155703: f64 = (locals.var_tmf1 * assign103750_e155702);
        let assign103750_e155704: f64 = (assign103750_e155698 + assign103750_e155703);
        let assign103750_e155705: f64 = (locals.var_tmf1 * assign103750_e155704);
        let assign103750_e155706: f64 = (assign103750_e155694 + assign103750_e155705);
        let assign103750_e155707: f64 = (locals.var_tmf1 * assign103750_e155706);
        let assign103750_e155708: f64 = (assign103750_e155690 + assign103750_e155707);
        let assign103750_e155709: f64 = (locals.var_tmf1 * assign103750_e155708);
        let assign103750_e155710: f64 = (assign103750_e155686 + assign103750_e155709);
        let assign103750_e155711: f64 = (locals.var_tmf1 * assign103750_e155710);
        let assign103750_e155712: f64 = (assign103750_e155682 + assign103750_e155711);
        let assign103750_e155713: f64 = (locals.var_tmf1 * assign103750_e155712);
        let assign103750_e155714: f64 = (1.0 + assign103750_e155713);
        (assign103750_e155714, ((locals.var_tmf1_dn0 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn2 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn4 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn5 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn6 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn7 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn8 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn9 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn10 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn13 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign103750_e155702))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign103750_e155716;
        locals.var_tmf2_dn0 = assign103750_e155716_d_n0;
        locals.var_tmf2_dn2 = assign103750_e155716_d_n2;
        locals.var_tmf2_dn4 = assign103750_e155716_d_n4;
        locals.var_tmf2_dn5 = assign103750_e155716_d_n5;
        locals.var_tmf2_dn6 = assign103750_e155716_d_n6;
        locals.var_tmf2_dn7 = assign103750_e155716_d_n7;
        locals.var_tmf2_dn8 = assign103750_e155716_d_n8;
        locals.var_tmf2_dn9 = assign103750_e155716_d_n9;
        locals.var_tmf2_dn10 = assign103750_e155716_d_n10;
        locals.var_tmf2_dn13 = assign103750_e155716_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign103760_e155757, assign103760_e155757_d_n0, assign103760_e155757_d_n2, assign103760_e155757_d_n4, assign103760_e155757_d_n5, assign103760_e155757_d_n6, assign103760_e155757_d_n7, assign103760_e155757_d_n8, assign103760_e155757_d_n9, assign103760_e155757_d_n10, assign103760_e155757_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103760_e155725: f64 = (1.0 / 2.0);
        let assign103760_e155729: f64 = (1.0 / 3.0);
        let assign103760_e155733: f64 = (1.0 / 8.0);
        let assign103760_e155737: f64 = (1.0 / 30.0);
        let assign103760_e155741: f64 = (1.0 / 144.0);
        let assign103760_e155745: f64 = (1.0 / 840.0);
        let assign103760_e155746: f64 = (locals.var_tmf1 * assign103760_e155745);
        let assign103760_e155747: f64 = (assign103760_e155741 + assign103760_e155746);
        let assign103760_e155748: f64 = (locals.var_tmf1 * assign103760_e155747);
        let assign103760_e155749: f64 = (assign103760_e155737 + assign103760_e155748);
        let assign103760_e155750: f64 = (locals.var_tmf1 * assign103760_e155749);
        let assign103760_e155751: f64 = (assign103760_e155733 + assign103760_e155750);
        let assign103760_e155752: f64 = (locals.var_tmf1 * assign103760_e155751);
        let assign103760_e155753: f64 = (assign103760_e155729 + assign103760_e155752);
        let assign103760_e155754: f64 = (locals.var_tmf1 * assign103760_e155753);
        let assign103760_e155755: f64 = (assign103760_e155725 + assign103760_e155754);
        (assign103760_e155755, ((locals.var_tmf1_dn0 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103760_e155745))))))))), ((locals.var_tmf1_dn2 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103760_e155745))))))))), ((locals.var_tmf1_dn4 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103760_e155745))))))))), ((locals.var_tmf1_dn5 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103760_e155745))))))))), ((locals.var_tmf1_dn6 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103760_e155745))))))))), ((locals.var_tmf1_dn7 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103760_e155745))))))))), ((locals.var_tmf1_dn8 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103760_e155745))))))))), ((locals.var_tmf1_dn9 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103760_e155745))))))))), ((locals.var_tmf1_dn10 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103760_e155745))))))))), ((locals.var_tmf1_dn13 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign103760_e155745))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign103760_e155757;
        locals.var_tmf3_dn0 = assign103760_e155757_d_n0;
        locals.var_tmf3_dn2 = assign103760_e155757_d_n2;
        locals.var_tmf3_dn4 = assign103760_e155757_d_n4;
        locals.var_tmf3_dn5 = assign103760_e155757_d_n5;
        locals.var_tmf3_dn6 = assign103760_e155757_d_n6;
        locals.var_tmf3_dn7 = assign103760_e155757_d_n7;
        locals.var_tmf3_dn8 = assign103760_e155757_d_n8;
        locals.var_tmf3_dn9 = assign103760_e155757_d_n9;
        locals.var_tmf3_dn10 = assign103760_e155757_d_n10;
        locals.var_tmf3_dn13 = assign103760_e155757_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign103770_e155768, assign103770_e155768_d_n0, assign103770_e155768_d_n2, assign103770_e155768_d_n4, assign103770_e155768_d_n5, assign103770_e155768_d_n6, assign103770_e155768_d_n7, assign103770_e155768_d_n8, assign103770_e155768_d_n9, assign103770_e155768_d_n10, assign103770_e155768_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103770_e155766: f64 = (p.p262 / locals.var_tmf2);
        (assign103770_e155766, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign103770_e155768;
        locals.var_vzadd_dn0 = assign103770_e155768_d_n0;
        locals.var_vzadd_dn2 = assign103770_e155768_d_n2;
        locals.var_vzadd_dn4 = assign103770_e155768_d_n4;
        locals.var_vzadd_dn5 = assign103770_e155768_d_n5;
        locals.var_vzadd_dn6 = assign103770_e155768_d_n6;
        locals.var_vzadd_dn7 = assign103770_e155768_d_n7;
        locals.var_vzadd_dn8 = assign103770_e155768_d_n8;
        locals.var_vzadd_dn9 = assign103770_e155768_d_n9;
        locals.var_vzadd_dn10 = assign103770_e155768_d_n10;
        locals.var_vzadd_dn13 = assign103770_e155768_d_n13;
        locals.var_vzadd_rv = 0.0;

        let (assign103780_e155784, assign103780_e155784_d_n0, assign103780_e155784_d_n2, assign103780_e155784_d_n4, assign103780_e155784_d_n5, assign103780_e155784_d_n6, assign103780_e155784_d_n7, assign103780_e155784_d_n8, assign103780_e155784_d_n9, assign103780_e155784_d_n10, assign103780_e155784_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103780_e155776: f64 = (-2.0);
        let assign103780_e155778: f64 = (assign103780_e155776 * locals.var_tmf3);
        let assign103780_e155781: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign103780_e155782: f64 = (assign103780_e155778 / assign103780_e155781);
        (assign103780_e155782, ((((assign103780_e155776 * locals.var_tmf3_dn0) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn2) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn4) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn5) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn6) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn7) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn8) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn9) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn10) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn13) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign103780_e155781 * assign103780_e155781)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign103780_e155784;
        locals.var_t2_dn0 = assign103780_e155784_d_n0;
        locals.var_t2_dn2 = assign103780_e155784_d_n2;
        locals.var_t2_dn4 = assign103780_e155784_d_n4;
        locals.var_t2_dn5 = assign103780_e155784_d_n5;
        locals.var_t2_dn6 = assign103780_e155784_d_n6;
        locals.var_t2_dn7 = assign103780_e155784_d_n7;
        locals.var_t2_dn8 = assign103780_e155784_d_n8;
        locals.var_t2_dn9 = assign103780_e155784_d_n9;
        locals.var_t2_dn10 = assign103780_e155784_d_n10;
        locals.var_t2_dn13 = assign103780_e155784_d_n13;
        locals.var_t2_rv = 0.0;

        let assign103790_e155787: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard2364 = assign103790_e155787;
        locals.var_guard2364_rv = 0.0;

        let (assign103800_e155798, assign103800_e155798_d_n0, assign103800_e155798_d_n2, assign103800_e155798_d_n4, assign103800_e155798_d_n5, assign103800_e155798_d_n6, assign103800_e155798_d_n7, assign103800_e155798_d_n8, assign103800_e155798_d_n9, assign103800_e155798_d_n10, assign103800_e155798_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) && (locals.var_guard2364 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign103800_e155798;
        locals.var_vzadd_dn0 = assign103800_e155798_d_n0;
        locals.var_vzadd_dn2 = assign103800_e155798_d_n2;
        locals.var_vzadd_dn4 = assign103800_e155798_d_n4;
        locals.var_vzadd_dn5 = assign103800_e155798_d_n5;
        locals.var_vzadd_dn6 = assign103800_e155798_d_n6;
        locals.var_vzadd_dn7 = assign103800_e155798_d_n7;
        locals.var_vzadd_dn8 = assign103800_e155798_d_n8;
        locals.var_vzadd_dn9 = assign103800_e155798_d_n9;
        locals.var_vzadd_dn10 = assign103800_e155798_d_n10;
        locals.var_vzadd_dn13 = assign103800_e155798_d_n13;
        locals.var_vzadd_rv = 0.0;

        let (assign103810_e155811, assign103810_e155811_d_n0, assign103810_e155811_d_n2, assign103810_e155811_d_n4, assign103810_e155811_d_n5, assign103810_e155811_d_n6, assign103810_e155811_d_n7, assign103810_e155811_d_n8, assign103810_e155811_d_n9, assign103810_e155811_d_n10, assign103810_e155811_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103810_e155808: f64 = (2.0 * locals.var_vzadd);
        let assign103810_e155809: f64 = (locals.var_vddp - assign103810_e155808);
        (assign103810_e155809, (locals.var_vddp_dn0 - (2.0 * locals.var_vzadd_dn0)), (-(2.0 * locals.var_vzadd_dn2)), (-(2.0 * locals.var_vzadd_dn4)), (locals.var_vddp_dn5 - (2.0 * locals.var_vzadd_dn5)), (-(2.0 * locals.var_vzadd_dn6)), (-(2.0 * locals.var_vzadd_dn7)), (-(2.0 * locals.var_vzadd_dn8)), (-(2.0 * locals.var_vzadd_dn9)), (-(2.0 * locals.var_vzadd_dn10)), (-(2.0 * locals.var_vzadd_dn13)),)
    } else {
        (locals.var_vddpz, locals.var_vddpz_dn0, locals.var_vddpz_dn2, locals.var_vddpz_dn4, locals.var_vddpz_dn5, locals.var_vddpz_dn6, locals.var_vddpz_dn7, locals.var_vddpz_dn8, locals.var_vddpz_dn9, locals.var_vddpz_dn10, locals.var_vddpz_dn13,)
    }
};
        locals.var_vddpz = assign103810_e155811;
        locals.var_vddpz_dn0 = assign103810_e155811_d_n0;
        locals.var_vddpz_dn2 = assign103810_e155811_d_n2;
        locals.var_vddpz_dn4 = assign103810_e155811_d_n4;
        locals.var_vddpz_dn5 = assign103810_e155811_d_n5;
        locals.var_vddpz_dn6 = assign103810_e155811_d_n6;
        locals.var_vddpz_dn7 = assign103810_e155811_d_n7;
        locals.var_vddpz_dn8 = assign103810_e155811_d_n8;
        locals.var_vddpz_dn9 = assign103810_e155811_d_n9;
        locals.var_vddpz_dn10 = assign103810_e155811_d_n10;
        locals.var_vddpz_dn13 = assign103810_e155811_d_n13;
        locals.var_vddpz_rv = 0.0;

        let (assign103820_e155827, assign103820_e155827_d_n0, assign103820_e155827_d_n2, assign103820_e155827_d_n4, assign103820_e155827_d_n5, assign103820_e155827_d_n6, assign103820_e155827_d_n7, assign103820_e155827_d_n8, assign103820_e155827_d_n9, assign103820_e155827_d_n10, assign103820_e155827_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103820_e155822: f64 = (locals.var_vddp / 2.0);
        let assign103820_e155823: f64 = (2.0 * assign103820_e155822);
        let assign103820_e155825: f64 = (assign103820_e155823 / p.p262);
        (assign103820_e155825, ((2.0 * (locals.var_vddp_dn0 / 2.0)) / p.p262), 0.0, 0.0, ((2.0 * (locals.var_vddp_dn5 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign103820_e155827;
        locals.var_tmf1_dn0 = assign103820_e155827_d_n0;
        locals.var_tmf1_dn2 = assign103820_e155827_d_n2;
        locals.var_tmf1_dn4 = assign103820_e155827_d_n4;
        locals.var_tmf1_dn5 = assign103820_e155827_d_n5;
        locals.var_tmf1_dn6 = assign103820_e155827_d_n6;
        locals.var_tmf1_dn7 = assign103820_e155827_d_n7;
        locals.var_tmf1_dn8 = assign103820_e155827_d_n8;
        locals.var_tmf1_dn9 = assign103820_e155827_d_n9;
        locals.var_tmf1_dn10 = assign103820_e155827_d_n10;
        locals.var_tmf1_dn13 = assign103820_e155827_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign103830_e155873, assign103830_e155873_d_n0, assign103830_e155873_d_n2, assign103830_e155873_d_n4, assign103830_e155873_d_n5, assign103830_e155873_d_n6, assign103830_e155873_d_n7, assign103830_e155873_d_n8, assign103830_e155873_d_n9, assign103830_e155873_d_n10, assign103830_e155873_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103830_e155839: f64 = (1.0 / 2.0);
        let assign103830_e155843: f64 = (1.0 / 6.0);
        let assign103830_e155847: f64 = (1.0 / 24.0);
        let assign103830_e155851: f64 = (1.0 / 120.0);
        let assign103830_e155855: f64 = (1.0 / 720.0);
        let assign103830_e155859: f64 = (1.0 / 5040.0);
        let assign103830_e155860: f64 = (locals.var_tmf1 * assign103830_e155859);
        let assign103830_e155861: f64 = (assign103830_e155855 + assign103830_e155860);
        let assign103830_e155862: f64 = (locals.var_tmf1 * assign103830_e155861);
        let assign103830_e155863: f64 = (assign103830_e155851 + assign103830_e155862);
        let assign103830_e155864: f64 = (locals.var_tmf1 * assign103830_e155863);
        let assign103830_e155865: f64 = (assign103830_e155847 + assign103830_e155864);
        let assign103830_e155866: f64 = (locals.var_tmf1 * assign103830_e155865);
        let assign103830_e155867: f64 = (assign103830_e155843 + assign103830_e155866);
        let assign103830_e155868: f64 = (locals.var_tmf1 * assign103830_e155867);
        let assign103830_e155869: f64 = (assign103830_e155839 + assign103830_e155868);
        let assign103830_e155870: f64 = (locals.var_tmf1 * assign103830_e155869);
        let assign103830_e155871: f64 = (1.0 + assign103830_e155870);
        (assign103830_e155871, ((locals.var_tmf1_dn0 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn2 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn4 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn5 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn6 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn7 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn8 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn9 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn10 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn13 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign103830_e155859))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign103830_e155873;
        locals.var_tmf2_dn0 = assign103830_e155873_d_n0;
        locals.var_tmf2_dn2 = assign103830_e155873_d_n2;
        locals.var_tmf2_dn4 = assign103830_e155873_d_n4;
        locals.var_tmf2_dn5 = assign103830_e155873_d_n5;
        locals.var_tmf2_dn6 = assign103830_e155873_d_n6;
        locals.var_tmf2_dn7 = assign103830_e155873_d_n7;
        locals.var_tmf2_dn8 = assign103830_e155873_d_n8;
        locals.var_tmf2_dn9 = assign103830_e155873_d_n9;
        locals.var_tmf2_dn10 = assign103830_e155873_d_n10;
        locals.var_tmf2_dn13 = assign103830_e155873_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign103840_e155915, assign103840_e155915_d_n0, assign103840_e155915_d_n2, assign103840_e155915_d_n4, assign103840_e155915_d_n5, assign103840_e155915_d_n6, assign103840_e155915_d_n7, assign103840_e155915_d_n8, assign103840_e155915_d_n9, assign103840_e155915_d_n10, assign103840_e155915_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103840_e155883: f64 = (1.0 / 2.0);
        let assign103840_e155887: f64 = (1.0 / 3.0);
        let assign103840_e155891: f64 = (1.0 / 8.0);
        let assign103840_e155895: f64 = (1.0 / 30.0);
        let assign103840_e155899: f64 = (1.0 / 144.0);
        let assign103840_e155903: f64 = (1.0 / 840.0);
        let assign103840_e155904: f64 = (locals.var_tmf1 * assign103840_e155903);
        let assign103840_e155905: f64 = (assign103840_e155899 + assign103840_e155904);
        let assign103840_e155906: f64 = (locals.var_tmf1 * assign103840_e155905);
        let assign103840_e155907: f64 = (assign103840_e155895 + assign103840_e155906);
        let assign103840_e155908: f64 = (locals.var_tmf1 * assign103840_e155907);
        let assign103840_e155909: f64 = (assign103840_e155891 + assign103840_e155908);
        let assign103840_e155910: f64 = (locals.var_tmf1 * assign103840_e155909);
        let assign103840_e155911: f64 = (assign103840_e155887 + assign103840_e155910);
        let assign103840_e155912: f64 = (locals.var_tmf1 * assign103840_e155911);
        let assign103840_e155913: f64 = (assign103840_e155883 + assign103840_e155912);
        (assign103840_e155913, ((locals.var_tmf1_dn0 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103840_e155903))))))))), ((locals.var_tmf1_dn2 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103840_e155903))))))))), ((locals.var_tmf1_dn4 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103840_e155903))))))))), ((locals.var_tmf1_dn5 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103840_e155903))))))))), ((locals.var_tmf1_dn6 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103840_e155903))))))))), ((locals.var_tmf1_dn7 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103840_e155903))))))))), ((locals.var_tmf1_dn8 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103840_e155903))))))))), ((locals.var_tmf1_dn9 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103840_e155903))))))))), ((locals.var_tmf1_dn10 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103840_e155903))))))))), ((locals.var_tmf1_dn13 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign103840_e155903))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign103840_e155915;
        locals.var_tmf3_dn0 = assign103840_e155915_d_n0;
        locals.var_tmf3_dn2 = assign103840_e155915_d_n2;
        locals.var_tmf3_dn4 = assign103840_e155915_d_n4;
        locals.var_tmf3_dn5 = assign103840_e155915_d_n5;
        locals.var_tmf3_dn6 = assign103840_e155915_d_n6;
        locals.var_tmf3_dn7 = assign103840_e155915_d_n7;
        locals.var_tmf3_dn8 = assign103840_e155915_d_n8;
        locals.var_tmf3_dn9 = assign103840_e155915_d_n9;
        locals.var_tmf3_dn10 = assign103840_e155915_d_n10;
        locals.var_tmf3_dn13 = assign103840_e155915_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign103850_e155927, assign103850_e155927_d_n0, assign103850_e155927_d_n2, assign103850_e155927_d_n4, assign103850_e155927_d_n5, assign103850_e155927_d_n6, assign103850_e155927_d_n7, assign103850_e155927_d_n8, assign103850_e155927_d_n9, assign103850_e155927_d_n10, assign103850_e155927_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103850_e155925: f64 = (p.p262 / locals.var_tmf2);
        (assign103850_e155925, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign103850_e155927;
        locals.var_vzadd_dn0 = assign103850_e155927_d_n0;
        locals.var_vzadd_dn2 = assign103850_e155927_d_n2;
        locals.var_vzadd_dn4 = assign103850_e155927_d_n4;
        locals.var_vzadd_dn5 = assign103850_e155927_d_n5;
        locals.var_vzadd_dn6 = assign103850_e155927_d_n6;
        locals.var_vzadd_dn7 = assign103850_e155927_d_n7;
        locals.var_vzadd_dn8 = assign103850_e155927_d_n8;
        locals.var_vzadd_dn9 = assign103850_e155927_d_n9;
        locals.var_vzadd_dn10 = assign103850_e155927_d_n10;
        locals.var_vzadd_dn13 = assign103850_e155927_d_n13;
        locals.var_vzadd_rv = 0.0;

        let (assign103860_e155944, assign103860_e155944_d_n0, assign103860_e155944_d_n2, assign103860_e155944_d_n4, assign103860_e155944_d_n5, assign103860_e155944_d_n6, assign103860_e155944_d_n7, assign103860_e155944_d_n8, assign103860_e155944_d_n9, assign103860_e155944_d_n10, assign103860_e155944_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103860_e155936: f64 = (-2.0);
        let assign103860_e155938: f64 = (assign103860_e155936 * locals.var_tmf3);
        let assign103860_e155941: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign103860_e155942: f64 = (assign103860_e155938 / assign103860_e155941);
        (assign103860_e155942, ((((assign103860_e155936 * locals.var_tmf3_dn0) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn2) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn4) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn5) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn6) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn7) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn8) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn9) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn10) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn13) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign103860_e155941 * assign103860_e155941)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign103860_e155944;
        locals.var_t2_dn0 = assign103860_e155944_d_n0;
        locals.var_t2_dn2 = assign103860_e155944_d_n2;
        locals.var_t2_dn4 = assign103860_e155944_d_n4;
        locals.var_t2_dn5 = assign103860_e155944_d_n5;
        locals.var_t2_dn6 = assign103860_e155944_d_n6;
        locals.var_t2_dn7 = assign103860_e155944_d_n7;
        locals.var_t2_dn8 = assign103860_e155944_d_n8;
        locals.var_t2_dn9 = assign103860_e155944_d_n9;
        locals.var_t2_dn10 = assign103860_e155944_d_n10;
        locals.var_t2_dn13 = assign103860_e155944_d_n13;
        locals.var_t2_rv = 0.0;

        let assign103870_e155947: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard2365 = assign103870_e155947;
        locals.var_guard2365_rv = 0.0;

        let (assign103880_e155959, assign103880_e155959_d_n0, assign103880_e155959_d_n2, assign103880_e155959_d_n4, assign103880_e155959_d_n5, assign103880_e155959_d_n6, assign103880_e155959_d_n7, assign103880_e155959_d_n8, assign103880_e155959_d_n9, assign103880_e155959_d_n10, assign103880_e155959_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) && (locals.var_guard2365 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign103880_e155959;
        locals.var_vzadd_dn0 = assign103880_e155959_d_n0;
        locals.var_vzadd_dn2 = assign103880_e155959_d_n2;
        locals.var_vzadd_dn4 = assign103880_e155959_d_n4;
        locals.var_vzadd_dn5 = assign103880_e155959_d_n5;
        locals.var_vzadd_dn6 = assign103880_e155959_d_n6;
        locals.var_vzadd_dn7 = assign103880_e155959_d_n7;
        locals.var_vzadd_dn8 = assign103880_e155959_d_n8;
        locals.var_vzadd_dn9 = assign103880_e155959_d_n9;
        locals.var_vzadd_dn10 = assign103880_e155959_d_n10;
        locals.var_vzadd_dn13 = assign103880_e155959_d_n13;
        locals.var_vzadd_rv = 0.0;

        let (assign103890_e155973, assign103890_e155973_d_n0, assign103890_e155973_d_n2, assign103890_e155973_d_n4, assign103890_e155973_d_n5, assign103890_e155973_d_n6, assign103890_e155973_d_n7, assign103890_e155973_d_n8, assign103890_e155973_d_n9, assign103890_e155973_d_n10, assign103890_e155973_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103890_e155970: f64 = (2.0 * locals.var_vzadd);
        let assign103890_e155971: f64 = (locals.var_vddp + assign103890_e155970);
        (assign103890_e155971, (locals.var_vddp_dn0 + (2.0 * locals.var_vzadd_dn0)), (2.0 * locals.var_vzadd_dn2), (2.0 * locals.var_vzadd_dn4), (locals.var_vddp_dn5 + (2.0 * locals.var_vzadd_dn5)), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn13),)
    } else {
        (locals.var_vddpz, locals.var_vddpz_dn0, locals.var_vddpz_dn2, locals.var_vddpz_dn4, locals.var_vddpz_dn5, locals.var_vddpz_dn6, locals.var_vddpz_dn7, locals.var_vddpz_dn8, locals.var_vddpz_dn9, locals.var_vddpz_dn10, locals.var_vddpz_dn13,)
    }
};
        locals.var_vddpz = assign103890_e155973;
        locals.var_vddpz_dn0 = assign103890_e155973_d_n0;
        locals.var_vddpz_dn2 = assign103890_e155973_d_n2;
        locals.var_vddpz_dn4 = assign103890_e155973_d_n4;
        locals.var_vddpz_dn5 = assign103890_e155973_d_n5;
        locals.var_vddpz_dn6 = assign103890_e155973_d_n6;
        locals.var_vddpz_dn7 = assign103890_e155973_d_n7;
        locals.var_vddpz_dn8 = assign103890_e155973_d_n8;
        locals.var_vddpz_dn9 = assign103890_e155973_d_n9;
        locals.var_vddpz_dn10 = assign103890_e155973_d_n10;
        locals.var_vddpz_dn13 = assign103890_e155973_d_n13;
        locals.var_vddpz_rv = 0.0;

        let (assign103900_e155982, assign103900_e155982_d_n0, assign103900_e155982_d_n2, assign103900_e155982_d_n4, assign103900_e155982_d_n5, assign103900_e155982_d_n6, assign103900_e155982_d_n7, assign103900_e155982_d_n8, assign103900_e155982_d_n9, assign103900_e155982_d_n10, assign103900_e155982_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103900_e155980: f64 = (locals.var_vddpz / locals.var_ldrifte);
        (assign103900_e155980, (locals.var_vddpz_dn0 / locals.var_ldrifte), (locals.var_vddpz_dn2 / locals.var_ldrifte), (locals.var_vddpz_dn4 / locals.var_ldrifte), (locals.var_vddpz_dn5 / locals.var_ldrifte), (locals.var_vddpz_dn6 / locals.var_ldrifte), (locals.var_vddpz_dn7 / locals.var_ldrifte), (locals.var_vddpz_dn8 / locals.var_ldrifte), (locals.var_vddpz_dn9 / locals.var_ldrifte), (locals.var_vddpz_dn10 / locals.var_ldrifte), (locals.var_vddpz_dn13 / locals.var_ldrifte),)
    } else {
        (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn4, locals.var_edri_dn5, locals.var_edri_dn6, locals.var_edri_dn7, locals.var_edri_dn8, locals.var_edri_dn9, locals.var_edri_dn10, locals.var_edri_dn13,)
    }
};
        locals.var_edri = assign103900_e155982;
        locals.var_edri_dn0 = assign103900_e155982_d_n0;
        locals.var_edri_dn2 = assign103900_e155982_d_n2;
        locals.var_edri_dn4 = assign103900_e155982_d_n4;
        locals.var_edri_dn5 = assign103900_e155982_d_n5;
        locals.var_edri_dn6 = assign103900_e155982_d_n6;
        locals.var_edri_dn7 = assign103900_e155982_d_n7;
        locals.var_edri_dn8 = assign103900_e155982_d_n8;
        locals.var_edri_dn9 = assign103900_e155982_d_n9;
        locals.var_edri_dn10 = assign103900_e155982_d_n10;
        locals.var_edri_dn13 = assign103900_e155982_d_n13;
        locals.var_edri_rv = 0.0;

        let (assign103910_e155991, assign103910_e155991_d_n0, assign103910_e155991_d_n2, assign103910_e155991_d_n4, assign103910_e155991_d_n5, assign103910_e155991_d_n6, assign103910_e155991_d_n7, assign103910_e155991_d_n8, assign103910_e155991_d_n9, assign103910_e155991_d_n10, assign103910_e155991_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103910_e155989: f64 = (locals.var_mu0 * locals.var_edri);
        (assign103910_e155989, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn4 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn4)), ((locals.var_mu0_dn5 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn5)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), ((locals.var_mu0_dn8 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn8)), ((locals.var_mu0_dn9 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn9)), ((locals.var_mu0_dn10 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn10)), ((locals.var_mu0_dn13 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn13)),)
    } else {
        (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn4, locals.var_vdri_dn5, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn8, locals.var_vdri_dn9, locals.var_vdri_dn10, locals.var_vdri_dn13,)
    }
};
        locals.var_vdri = assign103910_e155991;
        locals.var_vdri_dn0 = assign103910_e155991_d_n0;
        locals.var_vdri_dn2 = assign103910_e155991_d_n2;
        locals.var_vdri_dn4 = assign103910_e155991_d_n4;
        locals.var_vdri_dn5 = assign103910_e155991_d_n5;
        locals.var_vdri_dn6 = assign103910_e155991_d_n6;
        locals.var_vdri_dn7 = assign103910_e155991_d_n7;
        locals.var_vdri_dn8 = assign103910_e155991_d_n8;
        locals.var_vdri_dn9 = assign103910_e155991_d_n9;
        locals.var_vdri_dn10 = assign103910_e155991_d_n10;
        locals.var_vdri_dn13 = assign103910_e155991_d_n13;
        locals.var_vdri_rv = 0.0;

        let assign103920_e155994: f64 = if locals.var_vddp >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2366 = assign103920_e155994;
        locals.var_guard2366_rv = 0.0;

        let (assign103930_e156005, assign103930_e156005_d_n0, assign103930_e156005_d_n2, assign103930_e156005_d_n4, assign103930_e156005_d_n5, assign103930_e156005_d_n6, assign103930_e156005_d_n7, assign103930_e156005_d_n8, assign103930_e156005_d_n9, assign103930_e156005_d_n10, assign103930_e156005_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2366 != 0.0)) {
        let assign103930_e156003: f64 = (locals.var_vdri / locals.var_vmaxe__blk2355);
        (assign103930_e156003, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn0)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn2)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn4 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn4)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn5 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn5)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn6)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn7)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn8 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn8)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn9 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn9)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn10)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn13 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn13)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign103930_e156005;
        locals.var_t1_dn0 = assign103930_e156005_d_n0;
        locals.var_t1_dn2 = assign103930_e156005_d_n2;
        locals.var_t1_dn4 = assign103930_e156005_d_n4;
        locals.var_t1_dn5 = assign103930_e156005_d_n5;
        locals.var_t1_dn6 = assign103930_e156005_d_n6;
        locals.var_t1_dn7 = assign103930_e156005_d_n7;
        locals.var_t1_dn8 = assign103930_e156005_d_n8;
        locals.var_t1_dn9 = assign103930_e156005_d_n9;
        locals.var_t1_dn10 = assign103930_e156005_d_n10;
        locals.var_t1_dn13 = assign103930_e156005_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign103940_e156018, assign103940_e156018_d_n0, assign103940_e156018_d_n2, assign103940_e156018_d_n4, assign103940_e156018_d_n5, assign103940_e156018_d_n6, assign103940_e156018_d_n7, assign103940_e156018_d_n8, assign103940_e156018_d_n9, assign103940_e156018_d_n10, assign103940_e156018_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2366 == 0.0)) {
        let assign103940_e156014: f64 = (-locals.var_vdri);
        let assign103940_e156016: f64 = (assign103940_e156014 / locals.var_vmaxe__blk2355);
        (assign103940_e156016, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn0)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn2)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn4) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn4)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn5) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn5)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn6)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn7)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn8) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn8)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn9) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn9)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn10)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn13) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn13)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign103940_e156018;
        locals.var_t1_dn0 = assign103940_e156018_d_n0;
        locals.var_t1_dn2 = assign103940_e156018_d_n2;
        locals.var_t1_dn4 = assign103940_e156018_d_n4;
        locals.var_t1_dn5 = assign103940_e156018_d_n5;
        locals.var_t1_dn6 = assign103940_e156018_d_n6;
        locals.var_t1_dn7 = assign103940_e156018_d_n7;
        locals.var_t1_dn8 = assign103940_e156018_d_n8;
        locals.var_t1_dn9 = assign103940_e156018_d_n9;
        locals.var_t1_dn10 = assign103940_e156018_d_n10;
        locals.var_t1_dn13 = assign103940_e156018_d_n13;
        locals.var_t1_rv = 0.0;

        let assign103950_e156022: f64 = (10.0 * 2.220446049250313e-16);
        let assign103950_e156023: f64 = (1.0 - assign103950_e156022);
        let assign103950_e156030: f64 = (10.0 * 2.220446049250313e-16);
        let assign103950_e156031: f64 = (1.0 + assign103950_e156030);
        let assign103950_e156033: f64 = if ((assign103950_e156023 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign103950_e156031)) { 1.0 } else { 0.0 };
        locals.var_guard2367 = assign103950_e156033;
        locals.var_guard2367_rv = 0.0;

        let (assign103960_e156042, assign103960_e156042_d_n0, assign103960_e156042_d_n2, assign103960_e156042_d_n4, assign103960_e156042_d_n5, assign103960_e156042_d_n6, assign103960_e156042_d_n7, assign103960_e156042_d_n8, assign103960_e156042_d_n9, assign103960_e156042_d_n10, assign103960_e156042_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign103960_e156042;
        locals.var_t3_dn0 = assign103960_e156042_d_n0;
        locals.var_t3_dn2 = assign103960_e156042_d_n2;
        locals.var_t3_dn4 = assign103960_e156042_d_n4;
        locals.var_t3_dn5 = assign103960_e156042_d_n5;
        locals.var_t3_dn6 = assign103960_e156042_d_n6;
        locals.var_t3_dn7 = assign103960_e156042_d_n7;
        locals.var_t3_dn8 = assign103960_e156042_d_n8;
        locals.var_t3_dn9 = assign103960_e156042_d_n9;
        locals.var_t3_dn10 = assign103960_e156042_d_n10;
        locals.var_t3_dn13 = assign103960_e156042_d_n13;
        locals.var_t3_rv = 0.0;

        let assign103970_e156046: f64 = (10.0 * 2.220446049250313e-16);
        let assign103970_e156047: f64 = (2.0 - assign103970_e156046);
        let assign103970_e156054: f64 = (10.0 * 2.220446049250313e-16);
        let assign103970_e156055: f64 = (2.0 + assign103970_e156054);
        let assign103970_e156057: f64 = if ((assign103970_e156047 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign103970_e156055)) { 1.0 } else { 0.0 };
        locals.var_guard2368 = assign103970_e156057;
        locals.var_guard2368_rv = 0.0;

        let (assign103980_e156069, assign103980_e156069_d_n0, assign103980_e156069_d_n2, assign103980_e156069_d_n4, assign103980_e156069_d_n5, assign103980_e156069_d_n6, assign103980_e156069_d_n7, assign103980_e156069_d_n8, assign103980_e156069_d_n9, assign103980_e156069_d_n10, assign103980_e156069_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2367 == 0.0)) && (locals.var_guard2368 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign103980_e156069;
        locals.var_t3_dn0 = assign103980_e156069_d_n0;
        locals.var_t3_dn2 = assign103980_e156069_d_n2;
        locals.var_t3_dn4 = assign103980_e156069_d_n4;
        locals.var_t3_dn5 = assign103980_e156069_d_n5;
        locals.var_t3_dn6 = assign103980_e156069_d_n6;
        locals.var_t3_dn7 = assign103980_e156069_d_n7;
        locals.var_t3_dn8 = assign103980_e156069_d_n8;
        locals.var_t3_dn9 = assign103980_e156069_d_n9;
        locals.var_t3_dn10 = assign103980_e156069_d_n10;
        locals.var_t3_dn13 = assign103980_e156069_d_n13;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_385(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign103990_e156086, assign103990_e156086_d_n0, assign103990_e156086_d_n2, assign103990_e156086_d_n4, assign103990_e156086_d_n5, assign103990_e156086_d_n6, assign103990_e156086_d_n7, assign103990_e156086_d_n8, assign103990_e156086_d_n9, assign103990_e156086_d_n10, assign103990_e156086_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2367 == 0.0)) && (locals.var_guard2368 == 0.0)) {
        let assign103990_e156083: f64 = (locals.var_uc_rdrbb - 1.0);
        let assign103990_e156084: f64 = (locals.var_t1).powf(assign103990_e156083);
        (assign103990_e156084, if locals.var_uc_rdrbb_dn0 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn0)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn0 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn2 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn2)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn2 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn4 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn4)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn4 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn5 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn5)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn5 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn6 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn6)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn6 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn7 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn7)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn7 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn8 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn8)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn8 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn9 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn9)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn9 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn10 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn10)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn10 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn13 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn13)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn13 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn13 / locals.var_t1)))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign103990_e156086;
        locals.var_t3_dn0 = assign103990_e156086_d_n0;
        locals.var_t3_dn2 = assign103990_e156086_d_n2;
        locals.var_t3_dn4 = assign103990_e156086_d_n4;
        locals.var_t3_dn5 = assign103990_e156086_d_n5;
        locals.var_t3_dn6 = assign103990_e156086_d_n6;
        locals.var_t3_dn7 = assign103990_e156086_d_n7;
        locals.var_t3_dn8 = assign103990_e156086_d_n8;
        locals.var_t3_dn9 = assign103990_e156086_d_n9;
        locals.var_t3_dn10 = assign103990_e156086_d_n10;
        locals.var_t3_dn13 = assign103990_e156086_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign104000_e156095, assign104000_e156095_d_n0, assign104000_e156095_d_n2, assign104000_e156095_d_n4, assign104000_e156095_d_n5, assign104000_e156095_d_n6, assign104000_e156095_d_n7, assign104000_e156095_d_n8, assign104000_e156095_d_n9, assign104000_e156095_d_n10, assign104000_e156095_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104000_e156093: f64 = (locals.var_t1 * locals.var_t3);
        (assign104000_e156093, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104000_e156095;
        locals.var_t2_dn0 = assign104000_e156095_d_n0;
        locals.var_t2_dn2 = assign104000_e156095_d_n2;
        locals.var_t2_dn4 = assign104000_e156095_d_n4;
        locals.var_t2_dn5 = assign104000_e156095_d_n5;
        locals.var_t2_dn6 = assign104000_e156095_d_n6;
        locals.var_t2_dn7 = assign104000_e156095_d_n7;
        locals.var_t2_dn8 = assign104000_e156095_d_n8;
        locals.var_t2_dn9 = assign104000_e156095_d_n9;
        locals.var_t2_dn10 = assign104000_e156095_d_n10;
        locals.var_t2_dn13 = assign104000_e156095_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign104010_e156104, assign104010_e156104_d_n0, assign104010_e156104_d_n2, assign104010_e156104_d_n4, assign104010_e156104_d_n5, assign104010_e156104_d_n6, assign104010_e156104_d_n7, assign104010_e156104_d_n8, assign104010_e156104_d_n9, assign104010_e156104_d_n10, assign104010_e156104_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104010_e156102: f64 = (1.0 + locals.var_t2);
        (assign104010_e156102, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign104010_e156104;
        locals.var_t4_dn0 = assign104010_e156104_d_n0;
        locals.var_t4_dn2 = assign104010_e156104_d_n2;
        locals.var_t4_dn4 = assign104010_e156104_d_n4;
        locals.var_t4_dn5 = assign104010_e156104_d_n5;
        locals.var_t4_dn6 = assign104010_e156104_d_n6;
        locals.var_t4_dn7 = assign104010_e156104_d_n7;
        locals.var_t4_dn8 = assign104010_e156104_d_n8;
        locals.var_t4_dn9 = assign104010_e156104_d_n9;
        locals.var_t4_dn10 = assign104010_e156104_d_n10;
        locals.var_t4_dn13 = assign104010_e156104_d_n13;
        locals.var_t4_rv = 0.0;

        let assign104020_e156108: f64 = (10.0 * 2.220446049250313e-16);
        let assign104020_e156109: f64 = (1.0 - assign104020_e156108);
        let assign104020_e156116: f64 = (10.0 * 2.220446049250313e-16);
        let assign104020_e156117: f64 = (1.0 + assign104020_e156116);
        let assign104020_e156119: f64 = if ((assign104020_e156109 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104020_e156117)) { 1.0 } else { 0.0 };
        locals.var_guard2369 = assign104020_e156119;
        locals.var_guard2369_rv = 0.0;

        let (assign104030_e156130, assign104030_e156130_d_n0, assign104030_e156130_d_n2, assign104030_e156130_d_n4, assign104030_e156130_d_n5, assign104030_e156130_d_n6, assign104030_e156130_d_n7, assign104030_e156130_d_n8, assign104030_e156130_d_n9, assign104030_e156130_d_n10, assign104030_e156130_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2369 != 0.0)) {
        let assign104030_e156128: f64 = (1.0 / locals.var_t4);
        (assign104030_e156128, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign104030_e156130;
        locals.var_t5_dn0 = assign104030_e156130_d_n0;
        locals.var_t5_dn2 = assign104030_e156130_d_n2;
        locals.var_t5_dn4 = assign104030_e156130_d_n4;
        locals.var_t5_dn5 = assign104030_e156130_d_n5;
        locals.var_t5_dn6 = assign104030_e156130_d_n6;
        locals.var_t5_dn7 = assign104030_e156130_d_n7;
        locals.var_t5_dn8 = assign104030_e156130_d_n8;
        locals.var_t5_dn9 = assign104030_e156130_d_n9;
        locals.var_t5_dn10 = assign104030_e156130_d_n10;
        locals.var_t5_dn13 = assign104030_e156130_d_n13;
        locals.var_t5_rv = 0.0;

        let assign104040_e156134: f64 = (10.0 * 2.220446049250313e-16);
        let assign104040_e156135: f64 = (2.0 - assign104040_e156134);
        let assign104040_e156142: f64 = (10.0 * 2.220446049250313e-16);
        let assign104040_e156143: f64 = (2.0 + assign104040_e156142);
        let assign104040_e156145: f64 = if ((assign104040_e156135 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104040_e156143)) { 1.0 } else { 0.0 };
        locals.var_guard2370 = assign104040_e156145;
        locals.var_guard2370_rv = 0.0;

        let (assign104050_e156160, assign104050_e156160_d_n0, assign104050_e156160_d_n2, assign104050_e156160_d_n4, assign104050_e156160_d_n5, assign104050_e156160_d_n6, assign104050_e156160_d_n7, assign104050_e156160_d_n8, assign104050_e156160_d_n9, assign104050_e156160_d_n10, assign104050_e156160_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2369 == 0.0)) && (locals.var_guard2370 != 0.0)) {
        let assign104050_e156157: f64 = (locals.var_t4).sqrt();
        let assign104050_e156158: f64 = (1.0 / assign104050_e156157);
        (assign104050_e156158, (-((locals.var_t4_dn0 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn2 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn4 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn5 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn6 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn7 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn8 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn9 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn10 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn13 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign104050_e156160;
        locals.var_t5_dn0 = assign104050_e156160_d_n0;
        locals.var_t5_dn2 = assign104050_e156160_d_n2;
        locals.var_t5_dn4 = assign104050_e156160_d_n4;
        locals.var_t5_dn5 = assign104050_e156160_d_n5;
        locals.var_t5_dn6 = assign104050_e156160_d_n6;
        locals.var_t5_dn7 = assign104050_e156160_d_n7;
        locals.var_t5_dn8 = assign104050_e156160_d_n8;
        locals.var_t5_dn9 = assign104050_e156160_d_n9;
        locals.var_t5_dn10 = assign104050_e156160_d_n10;
        locals.var_t5_dn13 = assign104050_e156160_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign104060_e156185, assign104060_e156185_d_n0, assign104060_e156185_d_n2, assign104060_e156185_d_n4, assign104060_e156185_d_n5, assign104060_e156185_d_n6, assign104060_e156185_d_n7, assign104060_e156185_d_n8, assign104060_e156185_d_n9, assign104060_e156185_d_n10, assign104060_e156185_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2369 == 0.0)) && (locals.var_guard2370 == 0.0)) {
        let (assign104060_e156183, assign104060_e156183_d_n0, assign104060_e156183_d_n2, assign104060_e156183_d_n4, assign104060_e156183_d_n5, assign104060_e156183_d_n6, assign104060_e156183_d_n7, assign104060_e156183_d_n8, assign104060_e156183_d_n9, assign104060_e156183_d_n10, assign104060_e156183_d_n13,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign104060_e156177: f64 = (-1.0);
                let assign104060_e156179: f64 = (assign104060_e156177 / locals.var_uc_rdrbb);
                let assign104060_e156181: f64 = (assign104060_e156179 - 1.0);
                let assign104060_e156182: f64 = (locals.var_t4).powf(assign104060_e156181);
                (assign104060_e156182, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn0) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn0)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn0) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn0 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn2) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn2)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn2) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn2 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn4) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn4)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn4) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn4 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn5) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn5)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn5) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn5 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn6) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn6)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn6) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn6 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn7) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn7)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn7) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn7 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn8) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn8)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn8) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn8 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn9) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn9)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn9) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn9 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn10) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn10)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn10) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn10 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn13) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn13)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn13) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn13 / locals.var_t4)))) },)
            }
        };
        (assign104060_e156183, assign104060_e156183_d_n0, assign104060_e156183_d_n2, assign104060_e156183_d_n4, assign104060_e156183_d_n5, assign104060_e156183_d_n6, assign104060_e156183_d_n7, assign104060_e156183_d_n8, assign104060_e156183_d_n9, assign104060_e156183_d_n10, assign104060_e156183_d_n13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign104060_e156185;
        locals.var_t6_dn0 = assign104060_e156185_d_n0;
        locals.var_t6_dn2 = assign104060_e156185_d_n2;
        locals.var_t6_dn4 = assign104060_e156185_d_n4;
        locals.var_t6_dn5 = assign104060_e156185_d_n5;
        locals.var_t6_dn6 = assign104060_e156185_d_n6;
        locals.var_t6_dn7 = assign104060_e156185_d_n7;
        locals.var_t6_dn8 = assign104060_e156185_d_n8;
        locals.var_t6_dn9 = assign104060_e156185_d_n9;
        locals.var_t6_dn10 = assign104060_e156185_d_n10;
        locals.var_t6_dn13 = assign104060_e156185_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign104070_e156200, assign104070_e156200_d_n0, assign104070_e156200_d_n2, assign104070_e156200_d_n4, assign104070_e156200_d_n5, assign104070_e156200_d_n6, assign104070_e156200_d_n7, assign104070_e156200_d_n8, assign104070_e156200_d_n9, assign104070_e156200_d_n10, assign104070_e156200_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2369 == 0.0)) && (locals.var_guard2370 == 0.0)) {
        let assign104070_e156198: f64 = (locals.var_t4 * locals.var_t6);
        (assign104070_e156198, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn13 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign104070_e156200;
        locals.var_t5_dn0 = assign104070_e156200_d_n0;
        locals.var_t5_dn2 = assign104070_e156200_d_n2;
        locals.var_t5_dn4 = assign104070_e156200_d_n4;
        locals.var_t5_dn5 = assign104070_e156200_d_n5;
        locals.var_t5_dn6 = assign104070_e156200_d_n6;
        locals.var_t5_dn7 = assign104070_e156200_d_n7;
        locals.var_t5_dn8 = assign104070_e156200_d_n8;
        locals.var_t5_dn9 = assign104070_e156200_d_n9;
        locals.var_t5_dn10 = assign104070_e156200_d_n10;
        locals.var_t5_dn13 = assign104070_e156200_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign104080_e156209, assign104080_e156209_d_n0, assign104080_e156209_d_n2, assign104080_e156209_d_n4, assign104080_e156209_d_n5, assign104080_e156209_d_n6, assign104080_e156209_d_n7, assign104080_e156209_d_n8, assign104080_e156209_d_n9, assign104080_e156209_d_n10, assign104080_e156209_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104080_e156207: f64 = (locals.var_mu0 * locals.var_t5);
        (assign104080_e156207, ((locals.var_mu0_dn0 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn0)), ((locals.var_mu0_dn2 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn2)), ((locals.var_mu0_dn4 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn4)), ((locals.var_mu0_dn5 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn5)), ((locals.var_mu0_dn6 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn6)), ((locals.var_mu0_dn7 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn7)), ((locals.var_mu0_dn8 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn8)), ((locals.var_mu0_dn9 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn9)), ((locals.var_mu0_dn10 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn10)), ((locals.var_mu0_dn13 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn13)),)
    } else {
        (locals.var_mu__blk2354, locals.var_mu__blk2354_dn0, locals.var_mu__blk2354_dn2, locals.var_mu__blk2354_dn4, locals.var_mu__blk2354_dn5, locals.var_mu__blk2354_dn6, locals.var_mu__blk2354_dn7, locals.var_mu__blk2354_dn8, locals.var_mu__blk2354_dn9, locals.var_mu__blk2354_dn10, locals.var_mu__blk2354_dn13,)
    }
};
        locals.var_mu__blk2354 = assign104080_e156209;
        locals.var_mu__blk2354_dn0 = assign104080_e156209_d_n0;
        locals.var_mu__blk2354_dn2 = assign104080_e156209_d_n2;
        locals.var_mu__blk2354_dn4 = assign104080_e156209_d_n4;
        locals.var_mu__blk2354_dn5 = assign104080_e156209_d_n5;
        locals.var_mu__blk2354_dn6 = assign104080_e156209_d_n6;
        locals.var_mu__blk2354_dn7 = assign104080_e156209_d_n7;
        locals.var_mu__blk2354_dn8 = assign104080_e156209_d_n8;
        locals.var_mu__blk2354_dn9 = assign104080_e156209_d_n9;
        locals.var_mu__blk2354_dn10 = assign104080_e156209_d_n10;
        locals.var_mu__blk2354_dn13 = assign104080_e156209_d_n13;
        locals.var_mu__blk2354_rv = 0.0;

        let (assign104090_e156218, assign104090_e156218_d_n0, assign104090_e156218_d_n2, assign104090_e156218_d_n4, assign104090_e156218_d_n5, assign104090_e156218_d_n6, assign104090_e156218_d_n7, assign104090_e156218_d_n8, assign104090_e156218_d_n9, assign104090_e156218_d_n10, assign104090_e156218_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104090_e156216: f64 = (1.0 + locals.var_t1);
        (assign104090_e156216, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign104090_e156218;
        locals.var_t4_dn0 = assign104090_e156218_d_n0;
        locals.var_t4_dn2 = assign104090_e156218_d_n2;
        locals.var_t4_dn4 = assign104090_e156218_d_n4;
        locals.var_t4_dn5 = assign104090_e156218_d_n5;
        locals.var_t4_dn6 = assign104090_e156218_d_n6;
        locals.var_t4_dn7 = assign104090_e156218_d_n7;
        locals.var_t4_dn8 = assign104090_e156218_d_n8;
        locals.var_t4_dn9 = assign104090_e156218_d_n9;
        locals.var_t4_dn10 = assign104090_e156218_d_n10;
        locals.var_t4_dn13 = assign104090_e156218_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign104100_e156227, assign104100_e156227_d_n0, assign104100_e156227_d_n2, assign104100_e156227_d_n4, assign104100_e156227_d_n5, assign104100_e156227_d_n6, assign104100_e156227_d_n7, assign104100_e156227_d_n8, assign104100_e156227_d_n9, assign104100_e156227_d_n10, assign104100_e156227_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104100_e156225: f64 = (1.0 / locals.var_t4);
        (assign104100_e156225, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign104100_e156227;
        locals.var_t5_dn0 = assign104100_e156227_d_n0;
        locals.var_t5_dn2 = assign104100_e156227_d_n2;
        locals.var_t5_dn4 = assign104100_e156227_d_n4;
        locals.var_t5_dn5 = assign104100_e156227_d_n5;
        locals.var_t5_dn6 = assign104100_e156227_d_n6;
        locals.var_t5_dn7 = assign104100_e156227_d_n7;
        locals.var_t5_dn8 = assign104100_e156227_d_n8;
        locals.var_t5_dn9 = assign104100_e156227_d_n9;
        locals.var_t5_dn10 = assign104100_e156227_d_n10;
        locals.var_t5_dn13 = assign104100_e156227_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign104110_e156246, assign104110_e156246_d_n0, assign104110_e156246_d_n2, assign104110_e156246_d_n4, assign104110_e156246_d_n5, assign104110_e156246_d_n6, assign104110_e156246_d_n7, assign104110_e156246_d_n8, assign104110_e156246_d_n9, assign104110_e156246_d_n10, assign104110_e156246_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104110_e156236: f64 = (1.0 - locals.var_t5);
        let assign104110_e156237: f64 = (locals.var_car * assign104110_e156236);
        let assign104110_e156239: f64 = (assign104110_e156237 * locals.var_vddpz);
        let assign104110_e156242: f64 = (locals.var_ldrifte - p.p423);
        let assign104110_e156243: f64 = (assign104110_e156239 / assign104110_e156242);
        let assign104110_e156244: f64 = (1.0 + assign104110_e156243);
        (assign104110_e156244, ((((locals.var_car * (-locals.var_t5_dn0)) * locals.var_vddpz) + (assign104110_e156237 * locals.var_vddpz_dn0)) / assign104110_e156242), ((((locals.var_car * (-locals.var_t5_dn2)) * locals.var_vddpz) + (assign104110_e156237 * locals.var_vddpz_dn2)) / assign104110_e156242), ((((locals.var_car * (-locals.var_t5_dn4)) * locals.var_vddpz) + (assign104110_e156237 * locals.var_vddpz_dn4)) / assign104110_e156242), ((((locals.var_car * (-locals.var_t5_dn5)) * locals.var_vddpz) + (assign104110_e156237 * locals.var_vddpz_dn5)) / assign104110_e156242), ((((locals.var_car * (-locals.var_t5_dn6)) * locals.var_vddpz) + (assign104110_e156237 * locals.var_vddpz_dn6)) / assign104110_e156242), ((((locals.var_car * (-locals.var_t5_dn7)) * locals.var_vddpz) + (assign104110_e156237 * locals.var_vddpz_dn7)) / assign104110_e156242), ((((locals.var_car * (-locals.var_t5_dn8)) * locals.var_vddpz) + (assign104110_e156237 * locals.var_vddpz_dn8)) / assign104110_e156242), ((((locals.var_car * (-locals.var_t5_dn9)) * locals.var_vddpz) + (assign104110_e156237 * locals.var_vddpz_dn9)) / assign104110_e156242), ((((locals.var_car * (-locals.var_t5_dn10)) * locals.var_vddpz) + (assign104110_e156237 * locals.var_vddpz_dn10)) / assign104110_e156242), ((((locals.var_car * (-locals.var_t5_dn13)) * locals.var_vddpz) + (assign104110_e156237 * locals.var_vddpz_dn13)) / assign104110_e156242),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign104110_e156246;
        locals.var_t4_dn0 = assign104110_e156246_d_n0;
        locals.var_t4_dn2 = assign104110_e156246_d_n2;
        locals.var_t4_dn4 = assign104110_e156246_d_n4;
        locals.var_t4_dn5 = assign104110_e156246_d_n5;
        locals.var_t4_dn6 = assign104110_e156246_d_n6;
        locals.var_t4_dn7 = assign104110_e156246_d_n7;
        locals.var_t4_dn8 = assign104110_e156246_d_n8;
        locals.var_t4_dn9 = assign104110_e156246_d_n9;
        locals.var_t4_dn10 = assign104110_e156246_d_n10;
        locals.var_t4_dn13 = assign104110_e156246_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign104120_e156257, assign104120_e156257_d_n0, assign104120_e156257_d_n2, assign104120_e156257_d_n4, assign104120_e156257_d_n5, assign104120_e156257_d_n6, assign104120_e156257_d_n7, assign104120_e156257_d_n8, assign104120_e156257_d_n9, assign104120_e156257_d_n10, assign104120_e156257_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104120_e156253: f64 = locals.var_t4;
        let assign104120_e156255: f64 = (assign104120_e156253 - 0.001);
        (assign104120_e156255, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign104120_e156257;
        locals.var_tmf1_dn0 = assign104120_e156257_d_n0;
        locals.var_tmf1_dn2 = assign104120_e156257_d_n2;
        locals.var_tmf1_dn4 = assign104120_e156257_d_n4;
        locals.var_tmf1_dn5 = assign104120_e156257_d_n5;
        locals.var_tmf1_dn6 = assign104120_e156257_d_n6;
        locals.var_tmf1_dn7 = assign104120_e156257_d_n7;
        locals.var_tmf1_dn8 = assign104120_e156257_d_n8;
        locals.var_tmf1_dn9 = assign104120_e156257_d_n9;
        locals.var_tmf1_dn10 = assign104120_e156257_d_n10;
        locals.var_tmf1_dn13 = assign104120_e156257_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign104130_e156268, assign104130_e156268_d_n0, assign104130_e156268_d_n2, assign104130_e156268_d_n4, assign104130_e156268_d_n5, assign104130_e156268_d_n6, assign104130_e156268_d_n7, assign104130_e156268_d_n8, assign104130_e156268_d_n9, assign104130_e156268_d_n10, assign104130_e156268_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104130_e156268;
        locals.var_tmf2_dn0 = assign104130_e156268_d_n0;
        locals.var_tmf2_dn2 = assign104130_e156268_d_n2;
        locals.var_tmf2_dn4 = assign104130_e156268_d_n4;
        locals.var_tmf2_dn5 = assign104130_e156268_d_n5;
        locals.var_tmf2_dn6 = assign104130_e156268_d_n6;
        locals.var_tmf2_dn7 = assign104130_e156268_d_n7;
        locals.var_tmf2_dn8 = assign104130_e156268_d_n8;
        locals.var_tmf2_dn9 = assign104130_e156268_d_n9;
        locals.var_tmf2_dn10 = assign104130_e156268_d_n10;
        locals.var_tmf2_dn13 = assign104130_e156268_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign104140_e156281, assign104140_e156281_d_n0, assign104140_e156281_d_n2, assign104140_e156281_d_n4, assign104140_e156281_d_n5, assign104140_e156281_d_n6, assign104140_e156281_d_n7, assign104140_e156281_d_n8, assign104140_e156281_d_n9, assign104140_e156281_d_n10, assign104140_e156281_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let (assign104140_e156279, assign104140_e156279_d_n0, assign104140_e156279_d_n2, assign104140_e156279_d_n4, assign104140_e156279_d_n5, assign104140_e156279_d_n6, assign104140_e156279_d_n7, assign104140_e156279_d_n8, assign104140_e156279_d_n9, assign104140_e156279_d_n10, assign104140_e156279_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign104140_e156278: f64 = (-locals.var_tmf2);
                (assign104140_e156278, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign104140_e156279, assign104140_e156279_d_n0, assign104140_e156279_d_n2, assign104140_e156279_d_n4, assign104140_e156279_d_n5, assign104140_e156279_d_n6, assign104140_e156279_d_n7, assign104140_e156279_d_n8, assign104140_e156279_d_n9, assign104140_e156279_d_n10, assign104140_e156279_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104140_e156281;
        locals.var_tmf2_dn0 = assign104140_e156281_d_n0;
        locals.var_tmf2_dn2 = assign104140_e156281_d_n2;
        locals.var_tmf2_dn4 = assign104140_e156281_d_n4;
        locals.var_tmf2_dn5 = assign104140_e156281_d_n5;
        locals.var_tmf2_dn6 = assign104140_e156281_d_n6;
        locals.var_tmf2_dn7 = assign104140_e156281_d_n7;
        locals.var_tmf2_dn8 = assign104140_e156281_d_n8;
        locals.var_tmf2_dn9 = assign104140_e156281_d_n9;
        locals.var_tmf2_dn10 = assign104140_e156281_d_n10;
        locals.var_tmf2_dn13 = assign104140_e156281_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign104150_e156293, assign104150_e156293_d_n0, assign104150_e156293_d_n2, assign104150_e156293_d_n4, assign104150_e156293_d_n5, assign104150_e156293_d_n6, assign104150_e156293_d_n7, assign104150_e156293_d_n8, assign104150_e156293_d_n9, assign104150_e156293_d_n10, assign104150_e156293_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104150_e156288: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104150_e156290: f64 = (assign104150_e156288 + locals.var_tmf2);
        let assign104150_e156291: f64 = (assign104150_e156290).sqrt();
        (assign104150_e156291, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104150_e156291)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104150_e156291)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104150_e156291)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104150_e156291)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104150_e156291)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104150_e156291)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104150_e156291)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104150_e156291)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104150_e156291)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign104150_e156291)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104150_e156293;
        locals.var_tmf2_dn0 = assign104150_e156293_d_n0;
        locals.var_tmf2_dn2 = assign104150_e156293_d_n2;
        locals.var_tmf2_dn4 = assign104150_e156293_d_n4;
        locals.var_tmf2_dn5 = assign104150_e156293_d_n5;
        locals.var_tmf2_dn6 = assign104150_e156293_d_n6;
        locals.var_tmf2_dn7 = assign104150_e156293_d_n7;
        locals.var_tmf2_dn8 = assign104150_e156293_d_n8;
        locals.var_tmf2_dn9 = assign104150_e156293_d_n9;
        locals.var_tmf2_dn10 = assign104150_e156293_d_n10;
        locals.var_tmf2_dn13 = assign104150_e156293_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign104160_e156306, assign104160_e156306_d_n0, assign104160_e156306_d_n2, assign104160_e156306_d_n4, assign104160_e156306_d_n5, assign104160_e156306_d_n6, assign104160_e156306_d_n7, assign104160_e156306_d_n8, assign104160_e156306_d_n9, assign104160_e156306_d_n10, assign104160_e156306_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104160_e156302: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104160_e156303: f64 = (1.0 + assign104160_e156302);
        let assign104160_e156304: f64 = (0.5 * assign104160_e156303);
        (assign104160_e156304, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104160_e156306;
        locals.var_t0_dn0 = assign104160_e156306_d_n0;
        locals.var_t0_dn2 = assign104160_e156306_d_n2;
        locals.var_t0_dn4 = assign104160_e156306_d_n4;
        locals.var_t0_dn5 = assign104160_e156306_d_n5;
        locals.var_t0_dn6 = assign104160_e156306_d_n6;
        locals.var_t0_dn7 = assign104160_e156306_d_n7;
        locals.var_t0_dn8 = assign104160_e156306_d_n8;
        locals.var_t0_dn9 = assign104160_e156306_d_n9;
        locals.var_t0_dn10 = assign104160_e156306_d_n10;
        locals.var_t0_dn13 = assign104160_e156306_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign104170_e156319, assign104170_e156319_d_n0, assign104170_e156319_d_n2, assign104170_e156319_d_n4, assign104170_e156319_d_n5, assign104170_e156319_d_n6, assign104170_e156319_d_n7, assign104170_e156319_d_n8, assign104170_e156319_d_n9, assign104170_e156319_d_n10, assign104170_e156319_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104170_e156315: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104170_e156316: f64 = (0.5 * assign104170_e156315);
        let assign104170_e156317: f64 = assign104170_e156316;
        (assign104170_e156317, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign104170_e156319;
        locals.var_t5_dn0 = assign104170_e156319_d_n0;
        locals.var_t5_dn2 = assign104170_e156319_d_n2;
        locals.var_t5_dn4 = assign104170_e156319_d_n4;
        locals.var_t5_dn5 = assign104170_e156319_d_n5;
        locals.var_t5_dn6 = assign104170_e156319_d_n6;
        locals.var_t5_dn7 = assign104170_e156319_d_n7;
        locals.var_t5_dn8 = assign104170_e156319_d_n8;
        locals.var_t5_dn9 = assign104170_e156319_d_n9;
        locals.var_t5_dn10 = assign104170_e156319_d_n10;
        locals.var_t5_dn13 = assign104170_e156319_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign104180_e156328, assign104180_e156328_d_n0, assign104180_e156328_d_n2, assign104180_e156328_d_n4, assign104180_e156328_d_n5, assign104180_e156328_d_n6, assign104180_e156328_d_n7, assign104180_e156328_d_n8, assign104180_e156328_d_n9, assign104180_e156328_d_n10, assign104180_e156328_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104180_e156326: f64 = (locals.var_noverd * locals.var_t5);
        (assign104180_e156326, (locals.var_noverd * locals.var_t5_dn0), (locals.var_noverd * locals.var_t5_dn2), (locals.var_noverd * locals.var_t5_dn4), (locals.var_noverd * locals.var_t5_dn5), (locals.var_noverd * locals.var_t5_dn6), (locals.var_noverd * locals.var_t5_dn7), (locals.var_noverd * locals.var_t5_dn8), (locals.var_noverd * locals.var_t5_dn9), (locals.var_noverd * locals.var_t5_dn10), (locals.var_noverd * locals.var_t5_dn13),)
    } else {
        (locals.var_carr1, locals.var_carr1_dn0, locals.var_carr1_dn2, locals.var_carr1_dn4, locals.var_carr1_dn5, locals.var_carr1_dn6, locals.var_carr1_dn7, locals.var_carr1_dn8, locals.var_carr1_dn9, locals.var_carr1_dn10, locals.var_carr1_dn13,)
    }
};
        locals.var_carr1 = assign104180_e156328;
        locals.var_carr1_dn0 = assign104180_e156328_d_n0;
        locals.var_carr1_dn2 = assign104180_e156328_d_n2;
        locals.var_carr1_dn4 = assign104180_e156328_d_n4;
        locals.var_carr1_dn5 = assign104180_e156328_d_n5;
        locals.var_carr1_dn6 = assign104180_e156328_d_n6;
        locals.var_carr1_dn7 = assign104180_e156328_d_n7;
        locals.var_carr1_dn8 = assign104180_e156328_d_n8;
        locals.var_carr1_dn9 = assign104180_e156328_d_n9;
        locals.var_carr1_dn10 = assign104180_e156328_d_n10;
        locals.var_carr1_dn13 = assign104180_e156328_d_n13;
        locals.var_carr1_rv = 0.0;

        let (assign104190_e156339, assign104190_e156339_d_n0, assign104190_e156339_d_n2, assign104190_e156339_d_n4, assign104190_e156339_d_n5, assign104190_e156339_d_n6, assign104190_e156339_d_n7, assign104190_e156339_d_n8, assign104190_e156339_d_n9, assign104190_e156339_d_n10, assign104190_e156339_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104190_e156335: f64 = (locals.var_rd_qbuld / 1.6021918e-19);
        let assign104190_e156337: f64 = (assign104190_e156335 * p.p430);
        (assign104190_e156337, ((locals.var_rd_qbuld_dn0 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn2 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn4 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn5 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn6 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn7 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn8 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn9 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn10 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn13 / 1.6021918e-19) * p.p430),)
    } else {
        (locals.var_carr2, locals.var_carr2_dn0, locals.var_carr2_dn2, locals.var_carr2_dn4, locals.var_carr2_dn5, locals.var_carr2_dn6, locals.var_carr2_dn7, locals.var_carr2_dn8, locals.var_carr2_dn9, locals.var_carr2_dn10, locals.var_carr2_dn13,)
    }
};
        locals.var_carr2 = assign104190_e156339;
        locals.var_carr2_dn0 = assign104190_e156339_d_n0;
        locals.var_carr2_dn2 = assign104190_e156339_d_n2;
        locals.var_carr2_dn4 = assign104190_e156339_d_n4;
        locals.var_carr2_dn5 = assign104190_e156339_d_n5;
        locals.var_carr2_dn6 = assign104190_e156339_d_n6;
        locals.var_carr2_dn7 = assign104190_e156339_d_n7;
        locals.var_carr2_dn8 = assign104190_e156339_d_n8;
        locals.var_carr2_dn9 = assign104190_e156339_d_n9;
        locals.var_carr2_dn10 = assign104190_e156339_d_n10;
        locals.var_carr2_dn13 = assign104190_e156339_d_n13;
        locals.var_carr2_rv = 0.0;

        let (assign104200_e156352, assign104200_e156352_d_n0, assign104200_e156352_d_n2, assign104200_e156352_d_n4, assign104200_e156352_d_n5, assign104200_e156352_d_n6, assign104200_e156352_d_n7, assign104200_e156352_d_n8, assign104200_e156352_d_n9, assign104200_e156352_d_n10, assign104200_e156352_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104200_e156346: f64 = (locals.var_carr1 - locals.var_carr2);
        let assign104200_e156349: f64 = (locals.var_carr1 * 0.001);
        let assign104200_e156350: f64 = (assign104200_e156346 - assign104200_e156349);
        (assign104200_e156350, ((locals.var_carr1_dn0 - locals.var_carr2_dn0) - (locals.var_carr1_dn0 * 0.001)), ((locals.var_carr1_dn2 - locals.var_carr2_dn2) - (locals.var_carr1_dn2 * 0.001)), ((locals.var_carr1_dn4 - locals.var_carr2_dn4) - (locals.var_carr1_dn4 * 0.001)), ((locals.var_carr1_dn5 - locals.var_carr2_dn5) - (locals.var_carr1_dn5 * 0.001)), ((locals.var_carr1_dn6 - locals.var_carr2_dn6) - (locals.var_carr1_dn6 * 0.001)), ((locals.var_carr1_dn7 - locals.var_carr2_dn7) - (locals.var_carr1_dn7 * 0.001)), ((locals.var_carr1_dn8 - locals.var_carr2_dn8) - (locals.var_carr1_dn8 * 0.001)), ((locals.var_carr1_dn9 - locals.var_carr2_dn9) - (locals.var_carr1_dn9 * 0.001)), ((locals.var_carr1_dn10 - locals.var_carr2_dn10) - (locals.var_carr1_dn10 * 0.001)), ((locals.var_carr1_dn13 - locals.var_carr2_dn13) - (locals.var_carr1_dn13 * 0.001)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign104200_e156352;
        locals.var_tmf1_dn0 = assign104200_e156352_d_n0;
        locals.var_tmf1_dn2 = assign104200_e156352_d_n2;
        locals.var_tmf1_dn4 = assign104200_e156352_d_n4;
        locals.var_tmf1_dn5 = assign104200_e156352_d_n5;
        locals.var_tmf1_dn6 = assign104200_e156352_d_n6;
        locals.var_tmf1_dn7 = assign104200_e156352_d_n7;
        locals.var_tmf1_dn8 = assign104200_e156352_d_n8;
        locals.var_tmf1_dn9 = assign104200_e156352_d_n9;
        locals.var_tmf1_dn10 = assign104200_e156352_d_n10;
        locals.var_tmf1_dn13 = assign104200_e156352_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign104210_e156365, assign104210_e156365_d_n0, assign104210_e156365_d_n2, assign104210_e156365_d_n4, assign104210_e156365_d_n5, assign104210_e156365_d_n6, assign104210_e156365_d_n7, assign104210_e156365_d_n8, assign104210_e156365_d_n9, assign104210_e156365_d_n10, assign104210_e156365_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104210_e156359: f64 = (4.0 * locals.var_carr1);
        let assign104210_e156362: f64 = (locals.var_carr1 * 0.001);
        let assign104210_e156363: f64 = (assign104210_e156359 * assign104210_e156362);
        (assign104210_e156363, (((4.0 * locals.var_carr1_dn0) * assign104210_e156362) + (assign104210_e156359 * (locals.var_carr1_dn0 * 0.001))), (((4.0 * locals.var_carr1_dn2) * assign104210_e156362) + (assign104210_e156359 * (locals.var_carr1_dn2 * 0.001))), (((4.0 * locals.var_carr1_dn4) * assign104210_e156362) + (assign104210_e156359 * (locals.var_carr1_dn4 * 0.001))), (((4.0 * locals.var_carr1_dn5) * assign104210_e156362) + (assign104210_e156359 * (locals.var_carr1_dn5 * 0.001))), (((4.0 * locals.var_carr1_dn6) * assign104210_e156362) + (assign104210_e156359 * (locals.var_carr1_dn6 * 0.001))), (((4.0 * locals.var_carr1_dn7) * assign104210_e156362) + (assign104210_e156359 * (locals.var_carr1_dn7 * 0.001))), (((4.0 * locals.var_carr1_dn8) * assign104210_e156362) + (assign104210_e156359 * (locals.var_carr1_dn8 * 0.001))), (((4.0 * locals.var_carr1_dn9) * assign104210_e156362) + (assign104210_e156359 * (locals.var_carr1_dn9 * 0.001))), (((4.0 * locals.var_carr1_dn10) * assign104210_e156362) + (assign104210_e156359 * (locals.var_carr1_dn10 * 0.001))), (((4.0 * locals.var_carr1_dn13) * assign104210_e156362) + (assign104210_e156359 * (locals.var_carr1_dn13 * 0.001))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104210_e156365;
        locals.var_tmf2_dn0 = assign104210_e156365_d_n0;
        locals.var_tmf2_dn2 = assign104210_e156365_d_n2;
        locals.var_tmf2_dn4 = assign104210_e156365_d_n4;
        locals.var_tmf2_dn5 = assign104210_e156365_d_n5;
        locals.var_tmf2_dn6 = assign104210_e156365_d_n6;
        locals.var_tmf2_dn7 = assign104210_e156365_d_n7;
        locals.var_tmf2_dn8 = assign104210_e156365_d_n8;
        locals.var_tmf2_dn9 = assign104210_e156365_d_n9;
        locals.var_tmf2_dn10 = assign104210_e156365_d_n10;
        locals.var_tmf2_dn13 = assign104210_e156365_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_386(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104220_e156378, assign104220_e156378_d_n0, assign104220_e156378_d_n2, assign104220_e156378_d_n4, assign104220_e156378_d_n5, assign104220_e156378_d_n6, assign104220_e156378_d_n7, assign104220_e156378_d_n8, assign104220_e156378_d_n9, assign104220_e156378_d_n10, assign104220_e156378_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let (assign104220_e156376, assign104220_e156376_d_n0, assign104220_e156376_d_n2, assign104220_e156376_d_n4, assign104220_e156376_d_n5, assign104220_e156376_d_n6, assign104220_e156376_d_n7, assign104220_e156376_d_n8, assign104220_e156376_d_n9, assign104220_e156376_d_n10, assign104220_e156376_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign104220_e156375: f64 = (-locals.var_tmf2);
                (assign104220_e156375, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign104220_e156376, assign104220_e156376_d_n0, assign104220_e156376_d_n2, assign104220_e156376_d_n4, assign104220_e156376_d_n5, assign104220_e156376_d_n6, assign104220_e156376_d_n7, assign104220_e156376_d_n8, assign104220_e156376_d_n9, assign104220_e156376_d_n10, assign104220_e156376_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104220_e156378;
        locals.var_tmf2_dn0 = assign104220_e156378_d_n0;
        locals.var_tmf2_dn2 = assign104220_e156378_d_n2;
        locals.var_tmf2_dn4 = assign104220_e156378_d_n4;
        locals.var_tmf2_dn5 = assign104220_e156378_d_n5;
        locals.var_tmf2_dn6 = assign104220_e156378_d_n6;
        locals.var_tmf2_dn7 = assign104220_e156378_d_n7;
        locals.var_tmf2_dn8 = assign104220_e156378_d_n8;
        locals.var_tmf2_dn9 = assign104220_e156378_d_n9;
        locals.var_tmf2_dn10 = assign104220_e156378_d_n10;
        locals.var_tmf2_dn13 = assign104220_e156378_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign104230_e156390, assign104230_e156390_d_n0, assign104230_e156390_d_n2, assign104230_e156390_d_n4, assign104230_e156390_d_n5, assign104230_e156390_d_n6, assign104230_e156390_d_n7, assign104230_e156390_d_n8, assign104230_e156390_d_n9, assign104230_e156390_d_n10, assign104230_e156390_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104230_e156385: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104230_e156387: f64 = (assign104230_e156385 + locals.var_tmf2);
        let assign104230_e156388: f64 = (assign104230_e156387).sqrt();
        (assign104230_e156388, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104230_e156388)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104230_e156388)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104230_e156388)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104230_e156388)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104230_e156388)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104230_e156388)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104230_e156388)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104230_e156388)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104230_e156388)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign104230_e156388)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104230_e156390;
        locals.var_tmf2_dn0 = assign104230_e156390_d_n0;
        locals.var_tmf2_dn2 = assign104230_e156390_d_n2;
        locals.var_tmf2_dn4 = assign104230_e156390_d_n4;
        locals.var_tmf2_dn5 = assign104230_e156390_d_n5;
        locals.var_tmf2_dn6 = assign104230_e156390_d_n6;
        locals.var_tmf2_dn7 = assign104230_e156390_d_n7;
        locals.var_tmf2_dn8 = assign104230_e156390_d_n8;
        locals.var_tmf2_dn9 = assign104230_e156390_d_n9;
        locals.var_tmf2_dn10 = assign104230_e156390_d_n10;
        locals.var_tmf2_dn13 = assign104230_e156390_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign104240_e156403, assign104240_e156403_d_n0, assign104240_e156403_d_n2, assign104240_e156403_d_n4, assign104240_e156403_d_n5, assign104240_e156403_d_n6, assign104240_e156403_d_n7, assign104240_e156403_d_n8, assign104240_e156403_d_n9, assign104240_e156403_d_n10, assign104240_e156403_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104240_e156399: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104240_e156400: f64 = (1.0 + assign104240_e156399);
        let assign104240_e156401: f64 = (0.5 * assign104240_e156400);
        (assign104240_e156401, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104240_e156403;
        locals.var_t0_dn0 = assign104240_e156403_d_n0;
        locals.var_t0_dn2 = assign104240_e156403_d_n2;
        locals.var_t0_dn4 = assign104240_e156403_d_n4;
        locals.var_t0_dn5 = assign104240_e156403_d_n5;
        locals.var_t0_dn6 = assign104240_e156403_d_n6;
        locals.var_t0_dn7 = assign104240_e156403_d_n7;
        locals.var_t0_dn8 = assign104240_e156403_d_n8;
        locals.var_t0_dn9 = assign104240_e156403_d_n9;
        locals.var_t0_dn10 = assign104240_e156403_d_n10;
        locals.var_t0_dn13 = assign104240_e156403_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign104250_e156416, assign104250_e156416_d_n0, assign104250_e156416_d_n2, assign104250_e156416_d_n4, assign104250_e156416_d_n5, assign104250_e156416_d_n6, assign104250_e156416_d_n7, assign104250_e156416_d_n8, assign104250_e156416_d_n9, assign104250_e156416_d_n10, assign104250_e156416_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104250_e156412: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104250_e156413: f64 = (0.5 * assign104250_e156412);
        let assign104250_e156414: f64 = (locals.var_carr1 - assign104250_e156413);
        (assign104250_e156414, (locals.var_carr1_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_carr1_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_carr1_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_carr1_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_carr1_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_carr1_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_carr1_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_carr1_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_carr1_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_carr1_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_carr2, locals.var_carr2_dn0, locals.var_carr2_dn2, locals.var_carr2_dn4, locals.var_carr2_dn5, locals.var_carr2_dn6, locals.var_carr2_dn7, locals.var_carr2_dn8, locals.var_carr2_dn9, locals.var_carr2_dn10, locals.var_carr2_dn13,)
    }
};
        locals.var_carr2 = assign104250_e156416;
        locals.var_carr2_dn0 = assign104250_e156416_d_n0;
        locals.var_carr2_dn2 = assign104250_e156416_d_n2;
        locals.var_carr2_dn4 = assign104250_e156416_d_n4;
        locals.var_carr2_dn5 = assign104250_e156416_d_n5;
        locals.var_carr2_dn6 = assign104250_e156416_d_n6;
        locals.var_carr2_dn7 = assign104250_e156416_d_n7;
        locals.var_carr2_dn8 = assign104250_e156416_d_n8;
        locals.var_carr2_dn9 = assign104250_e156416_d_n9;
        locals.var_carr2_dn10 = assign104250_e156416_d_n10;
        locals.var_carr2_dn13 = assign104250_e156416_d_n13;
        locals.var_carr2_rv = 0.0;

        let (assign104260_e156425, assign104260_e156425_d_n0, assign104260_e156425_d_n2, assign104260_e156425_d_n4, assign104260_e156425_d_n5, assign104260_e156425_d_n6, assign104260_e156425_d_n7, assign104260_e156425_d_n8, assign104260_e156425_d_n9, assign104260_e156425_d_n10, assign104260_e156425_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104260_e156423: f64 = (locals.var_carr1 - locals.var_carr2);
        (assign104260_e156423, (locals.var_carr1_dn0 - locals.var_carr2_dn0), (locals.var_carr1_dn2 - locals.var_carr2_dn2), (locals.var_carr1_dn4 - locals.var_carr2_dn4), (locals.var_carr1_dn5 - locals.var_carr2_dn5), (locals.var_carr1_dn6 - locals.var_carr2_dn6), (locals.var_carr1_dn7 - locals.var_carr2_dn7), (locals.var_carr1_dn8 - locals.var_carr2_dn8), (locals.var_carr1_dn9 - locals.var_carr2_dn9), (locals.var_carr1_dn10 - locals.var_carr2_dn10), (locals.var_carr1_dn13 - locals.var_carr2_dn13),)
    } else {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn13,)
    }
};
        locals.var_carr = assign104260_e156425;
        locals.var_carr_dn0 = assign104260_e156425_d_n0;
        locals.var_carr_dn2 = assign104260_e156425_d_n2;
        locals.var_carr_dn4 = assign104260_e156425_d_n4;
        locals.var_carr_dn5 = assign104260_e156425_d_n5;
        locals.var_carr_dn6 = assign104260_e156425_d_n6;
        locals.var_carr_dn7 = assign104260_e156425_d_n7;
        locals.var_carr_dn8 = assign104260_e156425_d_n8;
        locals.var_carr_dn9 = assign104260_e156425_d_n9;
        locals.var_carr_dn10 = assign104260_e156425_d_n10;
        locals.var_carr_dn13 = assign104260_e156425_d_n13;
        locals.var_carr_rv = 0.0;

        let assign104270_e156432: f64 = if ((p.p441 > 0.0) && (p.p440 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard2371 = assign104270_e156432;
        locals.var_guard2371_rv = 0.0;

        let assign104280_e156436: f64 = (locals.var_noverd * p.p440);
        let assign104280_e156439: f64 = (locals.var_noverd * p.p441);
        let assign104280_e156440: f64 = (assign104280_e156436 - assign104280_e156439);
        let assign104280_e156444: f64 = (locals.var_noverd * p.p441);
        let assign104280_e156447: f64 = if ((locals.var_carr > assign104280_e156440) && (assign104280_e156444 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2372 = assign104280_e156447;
        locals.var_guard2372_rv = 0.0;

        let (assign104290_e156466, assign104290_e156466_d_n0, assign104290_e156466_d_n2, assign104290_e156466_d_n4, assign104290_e156466_d_n5, assign104290_e156466_d_n6, assign104290_e156466_d_n7, assign104290_e156466_d_n8, assign104290_e156466_d_n9, assign104290_e156466_d_n10, assign104290_e156466_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104290_e156459: f64 = (locals.var_noverd * p.p440);
        let assign104290_e156460: f64 = (locals.var_carr - assign104290_e156459);
        let assign104290_e156463: f64 = (locals.var_noverd * p.p441);
        let assign104290_e156464: f64 = (assign104290_e156460 + assign104290_e156463);
        (assign104290_e156464, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign104290_e156466;
        locals.var_tmf1_dn0 = assign104290_e156466_d_n0;
        locals.var_tmf1_dn2 = assign104290_e156466_d_n2;
        locals.var_tmf1_dn4 = assign104290_e156466_d_n4;
        locals.var_tmf1_dn5 = assign104290_e156466_d_n5;
        locals.var_tmf1_dn6 = assign104290_e156466_d_n6;
        locals.var_tmf1_dn7 = assign104290_e156466_d_n7;
        locals.var_tmf1_dn8 = assign104290_e156466_d_n8;
        locals.var_tmf1_dn9 = assign104290_e156466_d_n9;
        locals.var_tmf1_dn10 = assign104290_e156466_d_n10;
        locals.var_tmf1_dn13 = assign104290_e156466_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign104300_e156479, assign104300_e156479_d_n0, assign104300_e156479_d_n2, assign104300_e156479_d_n4, assign104300_e156479_d_n5, assign104300_e156479_d_n6, assign104300_e156479_d_n7, assign104300_e156479_d_n8, assign104300_e156479_d_n9, assign104300_e156479_d_n10, assign104300_e156479_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104300_e156477: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign104300_e156477, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign104300_e156479;
        locals.var_x2_dn0 = assign104300_e156479_d_n0;
        locals.var_x2_dn2 = assign104300_e156479_d_n2;
        locals.var_x2_dn4 = assign104300_e156479_d_n4;
        locals.var_x2_dn5 = assign104300_e156479_d_n5;
        locals.var_x2_dn6 = assign104300_e156479_d_n6;
        locals.var_x2_dn7 = assign104300_e156479_d_n7;
        locals.var_x2_dn8 = assign104300_e156479_d_n8;
        locals.var_x2_dn9 = assign104300_e156479_d_n9;
        locals.var_x2_dn10 = assign104300_e156479_d_n10;
        locals.var_x2_dn13 = assign104300_e156479_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign104310_e156496, assign104310_e156496_d_n0, assign104310_e156496_d_n2, assign104310_e156496_d_n4, assign104310_e156496_d_n5, assign104310_e156496_d_n6, assign104310_e156496_d_n7, assign104310_e156496_d_n8, assign104310_e156496_d_n9, assign104310_e156496_d_n10, assign104310_e156496_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104310_e156490: f64 = (locals.var_noverd * p.p441);
        let assign104310_e156493: f64 = (locals.var_noverd * p.p441);
        let assign104310_e156494: f64 = (assign104310_e156490 * assign104310_e156493);
        (assign104310_e156494, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign104310_e156496;
        locals.var_xmax2_dn0 = assign104310_e156496_d_n0;
        locals.var_xmax2_dn2 = assign104310_e156496_d_n2;
        locals.var_xmax2_dn4 = assign104310_e156496_d_n4;
        locals.var_xmax2_dn5 = assign104310_e156496_d_n5;
        locals.var_xmax2_dn6 = assign104310_e156496_d_n6;
        locals.var_xmax2_dn7 = assign104310_e156496_d_n7;
        locals.var_xmax2_dn8 = assign104310_e156496_d_n8;
        locals.var_xmax2_dn9 = assign104310_e156496_d_n9;
        locals.var_xmax2_dn10 = assign104310_e156496_d_n10;
        locals.var_xmax2_dn13 = assign104310_e156496_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign104320_e156507, assign104320_e156507_d_n0, assign104320_e156507_d_n2, assign104320_e156507_d_n4, assign104320_e156507_d_n5, assign104320_e156507_d_n6, assign104320_e156507_d_n7, assign104320_e156507_d_n8, assign104320_e156507_d_n9, assign104320_e156507_d_n10, assign104320_e156507_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign104320_e156507;
        locals.var_xp_dn0 = assign104320_e156507_d_n0;
        locals.var_xp_dn2 = assign104320_e156507_d_n2;
        locals.var_xp_dn4 = assign104320_e156507_d_n4;
        locals.var_xp_dn5 = assign104320_e156507_d_n5;
        locals.var_xp_dn6 = assign104320_e156507_d_n6;
        locals.var_xp_dn7 = assign104320_e156507_d_n7;
        locals.var_xp_dn8 = assign104320_e156507_d_n8;
        locals.var_xp_dn9 = assign104320_e156507_d_n9;
        locals.var_xp_dn10 = assign104320_e156507_d_n10;
        locals.var_xp_dn13 = assign104320_e156507_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign104330_e156518, assign104330_e156518_d_n0, assign104330_e156518_d_n2, assign104330_e156518_d_n4, assign104330_e156518_d_n5, assign104330_e156518_d_n6, assign104330_e156518_d_n7, assign104330_e156518_d_n8, assign104330_e156518_d_n9, assign104330_e156518_d_n10, assign104330_e156518_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign104330_e156518;
        locals.var_xmp_dn0 = assign104330_e156518_d_n0;
        locals.var_xmp_dn2 = assign104330_e156518_d_n2;
        locals.var_xmp_dn4 = assign104330_e156518_d_n4;
        locals.var_xmp_dn5 = assign104330_e156518_d_n5;
        locals.var_xmp_dn6 = assign104330_e156518_d_n6;
        locals.var_xmp_dn7 = assign104330_e156518_d_n7;
        locals.var_xmp_dn8 = assign104330_e156518_d_n8;
        locals.var_xmp_dn9 = assign104330_e156518_d_n9;
        locals.var_xmp_dn10 = assign104330_e156518_d_n10;
        locals.var_xmp_dn13 = assign104330_e156518_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign104340_e156529,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104340_e156529;
        locals.var_m0_rv = 0.0;

        let (assign104350_e156540,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104350_e156540;
        locals.var_mm_rv = 0.0;

        let (assign104360_e156551, assign104360_e156551_d_n0, assign104360_e156551_d_n2, assign104360_e156551_d_n4, assign104360_e156551_d_n5, assign104360_e156551_d_n6, assign104360_e156551_d_n7, assign104360_e156551_d_n8, assign104360_e156551_d_n9, assign104360_e156551_d_n10, assign104360_e156551_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign104360_e156551;
        locals.var_arg_dn0 = assign104360_e156551_d_n0;
        locals.var_arg_dn2 = assign104360_e156551_d_n2;
        locals.var_arg_dn4 = assign104360_e156551_d_n4;
        locals.var_arg_dn5 = assign104360_e156551_d_n5;
        locals.var_arg_dn6 = assign104360_e156551_d_n6;
        locals.var_arg_dn7 = assign104360_e156551_d_n7;
        locals.var_arg_dn8 = assign104360_e156551_d_n8;
        locals.var_arg_dn9 = assign104360_e156551_d_n9;
        locals.var_arg_dn10 = assign104360_e156551_d_n10;
        locals.var_arg_dn13 = assign104360_e156551_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign104370_e156562, assign104370_e156562_d_n0, assign104370_e156562_d_n2, assign104370_e156562_d_n4, assign104370_e156562_d_n5, assign104370_e156562_d_n6, assign104370_e156562_d_n7, assign104370_e156562_d_n8, assign104370_e156562_d_n9, assign104370_e156562_d_n10, assign104370_e156562_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign104370_e156562;
        locals.var_dnm_dn0 = assign104370_e156562_d_n0;
        locals.var_dnm_dn2 = assign104370_e156562_d_n2;
        locals.var_dnm_dn4 = assign104370_e156562_d_n4;
        locals.var_dnm_dn5 = assign104370_e156562_d_n5;
        locals.var_dnm_dn6 = assign104370_e156562_d_n6;
        locals.var_dnm_dn7 = assign104370_e156562_d_n7;
        locals.var_dnm_dn8 = assign104370_e156562_d_n8;
        locals.var_dnm_dn9 = assign104370_e156562_d_n9;
        locals.var_dnm_dn10 = assign104370_e156562_d_n10;
        locals.var_dnm_dn13 = assign104370_e156562_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign104380_e156573,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104380_e156573;
        locals.var_m0_rv = 0.0;

        let mut assign104390_loop_guard: usize = 0;
        while {
            let assign104390_cond_e156585: f64 = if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_m0 < p.p442)) { 1.0 } else { 0.0 };
            assign104390_cond_e156585 != 0.0
        } {
            assign104390_loop_guard += 1;
            assert!(assign104390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104390_body0_e156598, assign104390_body0_e156598_d_n0, assign104390_body0_e156598_d_n2, assign104390_body0_e156598_d_n4, assign104390_body0_e156598_d_n5, assign104390_body0_e156598_d_n6, assign104390_body0_e156598_d_n7, assign104390_body0_e156598_d_n8, assign104390_body0_e156598_d_n9, assign104390_body0_e156598_d_n10, assign104390_body0_e156598_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104390_body0_e156596: f64 = (locals.var_xp * locals.var_x2);
        (assign104390_body0_e156596, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign104390_body0_e156598;
            locals.var_xp_dn0 = assign104390_body0_e156598_d_n0;
            locals.var_xp_dn2 = assign104390_body0_e156598_d_n2;
            locals.var_xp_dn4 = assign104390_body0_e156598_d_n4;
            locals.var_xp_dn5 = assign104390_body0_e156598_d_n5;
            locals.var_xp_dn6 = assign104390_body0_e156598_d_n6;
            locals.var_xp_dn7 = assign104390_body0_e156598_d_n7;
            locals.var_xp_dn8 = assign104390_body0_e156598_d_n8;
            locals.var_xp_dn9 = assign104390_body0_e156598_d_n9;
            locals.var_xp_dn10 = assign104390_body0_e156598_d_n10;
            locals.var_xp_dn13 = assign104390_body0_e156598_d_n13;
            locals.var_xp_rv = 0.0;
            let (assign104390_body1_e156611, assign104390_body1_e156611_d_n0, assign104390_body1_e156611_d_n2, assign104390_body1_e156611_d_n4, assign104390_body1_e156611_d_n5, assign104390_body1_e156611_d_n6, assign104390_body1_e156611_d_n7, assign104390_body1_e156611_d_n8, assign104390_body1_e156611_d_n9, assign104390_body1_e156611_d_n10, assign104390_body1_e156611_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104390_body1_e156609: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign104390_body1_e156609, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign104390_body1_e156611;
            locals.var_xmp_dn0 = assign104390_body1_e156611_d_n0;
            locals.var_xmp_dn2 = assign104390_body1_e156611_d_n2;
            locals.var_xmp_dn4 = assign104390_body1_e156611_d_n4;
            locals.var_xmp_dn5 = assign104390_body1_e156611_d_n5;
            locals.var_xmp_dn6 = assign104390_body1_e156611_d_n6;
            locals.var_xmp_dn7 = assign104390_body1_e156611_d_n7;
            locals.var_xmp_dn8 = assign104390_body1_e156611_d_n8;
            locals.var_xmp_dn9 = assign104390_body1_e156611_d_n9;
            locals.var_xmp_dn10 = assign104390_body1_e156611_d_n10;
            locals.var_xmp_dn13 = assign104390_body1_e156611_d_n13;
            locals.var_xmp_rv = 0.0;
            let (assign104390_body2_e156624,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104390_body2_e156622: f64 = (locals.var_m0 + 1.0);
        (assign104390_body2_e156622,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign104390_body2_e156624;
            locals.var_m0_rv = 0.0;
        }

        let (assign104400_e156637, assign104400_e156637_d_n0, assign104400_e156637_d_n2, assign104400_e156637_d_n4, assign104400_e156637_d_n5, assign104400_e156637_d_n6, assign104400_e156637_d_n7, assign104400_e156637_d_n8, assign104400_e156637_d_n9, assign104400_e156637_d_n10, assign104400_e156637_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104400_e156635: f64 = (locals.var_xp + locals.var_xmp);
        (assign104400_e156635, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign104400_e156637;
        locals.var_arg_dn0 = assign104400_e156637_d_n0;
        locals.var_arg_dn2 = assign104400_e156637_d_n2;
        locals.var_arg_dn4 = assign104400_e156637_d_n4;
        locals.var_arg_dn5 = assign104400_e156637_d_n5;
        locals.var_arg_dn6 = assign104400_e156637_d_n6;
        locals.var_arg_dn7 = assign104400_e156637_d_n7;
        locals.var_arg_dn8 = assign104400_e156637_d_n8;
        locals.var_arg_dn9 = assign104400_e156637_d_n9;
        locals.var_arg_dn10 = assign104400_e156637_d_n10;
        locals.var_arg_dn13 = assign104400_e156637_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign104410_e156648, assign104410_e156648_d_n0, assign104410_e156648_d_n2, assign104410_e156648_d_n4, assign104410_e156648_d_n5, assign104410_e156648_d_n6, assign104410_e156648_d_n7, assign104410_e156648_d_n8, assign104410_e156648_d_n9, assign104410_e156648_d_n10, assign104410_e156648_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign104410_e156648;
        locals.var_dnm_dn0 = assign104410_e156648_d_n0;
        locals.var_dnm_dn2 = assign104410_e156648_d_n2;
        locals.var_dnm_dn4 = assign104410_e156648_d_n4;
        locals.var_dnm_dn5 = assign104410_e156648_d_n5;
        locals.var_dnm_dn6 = assign104410_e156648_d_n6;
        locals.var_dnm_dn7 = assign104410_e156648_d_n7;
        locals.var_dnm_dn8 = assign104410_e156648_d_n8;
        locals.var_dnm_dn9 = assign104410_e156648_d_n9;
        locals.var_dnm_dn10 = assign104410_e156648_d_n10;
        locals.var_dnm_dn13 = assign104410_e156648_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign104420_e156663: f64 = if ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2373 = assign104420_e156663;
        locals.var_guard2373_rv = 0.0;

        let assign104430_e156666: f64 = if p.p442 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2374 = assign104430_e156666;
        locals.var_guard2374_rv = 0.0;

        let (assign104440_e156681,) = {
    if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104440_e156681;
        locals.var_mm_rv = 0.0;

        let assign104450_e156684: f64 = if p.p442 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2375 = assign104450_e156684;
        locals.var_guard2375_rv = 0.0;

        let (assign104460_e156702,) = {
    if (((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 == 0.0)) && (locals.var_guard2375 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104460_e156702;
        locals.var_mm_rv = 0.0;

        let assign104470_e156705: f64 = if p.p442 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2376 = assign104470_e156705;
        locals.var_guard2376_rv = 0.0;

        let (assign104480_e156726,) = {
    if ((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 == 0.0)) && (locals.var_guard2375 == 0.0)) && (locals.var_guard2376 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104480_e156726;
        locals.var_mm_rv = 0.0;

        let assign104490_e156729: f64 = if p.p442 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2377 = assign104490_e156729;
        locals.var_guard2377_rv = 0.0;

        let (assign104500_e156753,) = {
    if (((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 == 0.0)) && (locals.var_guard2375 == 0.0)) && (locals.var_guard2376 == 0.0)) && (locals.var_guard2377 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104500_e156753;
        locals.var_mm_rv = 0.0;

        let (assign104510_e156766,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104510_e156766;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_387(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign104520_loop_guard: usize = 0;
        while {
            let assign104520_cond_e156780: f64 = if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign104520_cond_e156780 != 0.0
        } {
            assign104520_loop_guard += 1;
            assert!(assign104520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104520_body0_e156794, assign104520_body0_e156794_d_n0, assign104520_body0_e156794_d_n2, assign104520_body0_e156794_d_n4, assign104520_body0_e156794_d_n5, assign104520_body0_e156794_d_n6, assign104520_body0_e156794_d_n7, assign104520_body0_e156794_d_n8, assign104520_body0_e156794_d_n9, assign104520_body0_e156794_d_n10, assign104520_body0_e156794_d_n13,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) {
        let assign104520_body0_e156792: f64 = (locals.var_dnm).sqrt();
        (assign104520_body0_e156792, (locals.var_dnm_dn0 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn2 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn4 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn5 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn6 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn7 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn8 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn9 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn10 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn13 / (2.0 * assign104520_body0_e156792)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign104520_body0_e156794;
            locals.var_dnm_dn0 = assign104520_body0_e156794_d_n0;
            locals.var_dnm_dn2 = assign104520_body0_e156794_d_n2;
            locals.var_dnm_dn4 = assign104520_body0_e156794_d_n4;
            locals.var_dnm_dn5 = assign104520_body0_e156794_d_n5;
            locals.var_dnm_dn6 = assign104520_body0_e156794_d_n6;
            locals.var_dnm_dn7 = assign104520_body0_e156794_d_n7;
            locals.var_dnm_dn8 = assign104520_body0_e156794_d_n8;
            locals.var_dnm_dn9 = assign104520_body0_e156794_d_n9;
            locals.var_dnm_dn10 = assign104520_body0_e156794_d_n10;
            locals.var_dnm_dn13 = assign104520_body0_e156794_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign104520_body1_e156809,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) {
        let assign104520_body1_e156807: f64 = (locals.var_m0 + 1.0);
        (assign104520_body1_e156807,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign104520_body1_e156809;
            locals.var_m0_rv = 0.0;
        }

        let (assign104530_e156834, assign104530_e156834_d_n0, assign104530_e156834_d_n2, assign104530_e156834_d_n4, assign104530_e156834_d_n5, assign104530_e156834_d_n6, assign104530_e156834_d_n7, assign104530_e156834_d_n8, assign104530_e156834_d_n9, assign104530_e156834_d_n10, assign104530_e156834_d_n13,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 == 0.0)) {
        let (assign104530_e156832, assign104530_e156832_d_n0, assign104530_e156832_d_n2, assign104530_e156832_d_n4, assign104530_e156832_d_n5, assign104530_e156832_d_n6, assign104530_e156832_d_n7, assign104530_e156832_d_n8, assign104530_e156832_d_n9, assign104530_e156832_d_n10, assign104530_e156832_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign104530_e156829: f64 = (2.0 * p.p442);
                let assign104530_e156830: f64 = (1.0 / assign104530_e156829);
                let assign104530_e156831: f64 = (locals.var_dnm).powf(assign104530_e156830);
                (assign104530_e156831, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn0)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn2)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn4)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn5)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn6)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn7)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn8)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn9)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn10)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn13)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign104530_e156832, assign104530_e156832_d_n0, assign104530_e156832_d_n2, assign104530_e156832_d_n4, assign104530_e156832_d_n5, assign104530_e156832_d_n6, assign104530_e156832_d_n7, assign104530_e156832_d_n8, assign104530_e156832_d_n9, assign104530_e156832_d_n10, assign104530_e156832_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign104530_e156834;
        locals.var_dnm_dn0 = assign104530_e156834_d_n0;
        locals.var_dnm_dn2 = assign104530_e156834_d_n2;
        locals.var_dnm_dn4 = assign104530_e156834_d_n4;
        locals.var_dnm_dn5 = assign104530_e156834_d_n5;
        locals.var_dnm_dn6 = assign104530_e156834_d_n6;
        locals.var_dnm_dn7 = assign104530_e156834_d_n7;
        locals.var_dnm_dn8 = assign104530_e156834_d_n8;
        locals.var_dnm_dn9 = assign104530_e156834_d_n9;
        locals.var_dnm_dn10 = assign104530_e156834_d_n10;
        locals.var_dnm_dn13 = assign104530_e156834_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign104540_e156847, assign104540_e156847_d_n0, assign104540_e156847_d_n2, assign104540_e156847_d_n4, assign104540_e156847_d_n5, assign104540_e156847_d_n6, assign104540_e156847_d_n7, assign104540_e156847_d_n8, assign104540_e156847_d_n9, assign104540_e156847_d_n10, assign104540_e156847_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104540_e156845: f64 = (1.0 / locals.var_dnm);
        (assign104540_e156845, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign104540_e156847;
        locals.var_dnm_dn0 = assign104540_e156847_d_n0;
        locals.var_dnm_dn2 = assign104540_e156847_d_n2;
        locals.var_dnm_dn4 = assign104540_e156847_d_n4;
        locals.var_dnm_dn5 = assign104540_e156847_d_n5;
        locals.var_dnm_dn6 = assign104540_e156847_d_n6;
        locals.var_dnm_dn7 = assign104540_e156847_d_n7;
        locals.var_dnm_dn8 = assign104540_e156847_d_n8;
        locals.var_dnm_dn9 = assign104540_e156847_d_n9;
        locals.var_dnm_dn10 = assign104540_e156847_d_n10;
        locals.var_dnm_dn13 = assign104540_e156847_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign104550_e156864, assign104550_e156864_d_n0, assign104550_e156864_d_n2, assign104550_e156864_d_n4, assign104550_e156864_d_n5, assign104550_e156864_d_n6, assign104550_e156864_d_n7, assign104550_e156864_d_n8, assign104550_e156864_d_n9, assign104550_e156864_d_n10, assign104550_e156864_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104550_e156859: f64 = (locals.var_noverd * p.p441);
        let assign104550_e156860: f64 = (locals.var_tmf1 * assign104550_e156859);
        let assign104550_e156862: f64 = (assign104550_e156860 * locals.var_dnm);
        (assign104550_e156862, (((locals.var_tmf1_dn0 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign104550_e156864;
        locals.var_tmf0_dn0 = assign104550_e156864_d_n0;
        locals.var_tmf0_dn2 = assign104550_e156864_d_n2;
        locals.var_tmf0_dn4 = assign104550_e156864_d_n4;
        locals.var_tmf0_dn5 = assign104550_e156864_d_n5;
        locals.var_tmf0_dn6 = assign104550_e156864_d_n6;
        locals.var_tmf0_dn7 = assign104550_e156864_d_n7;
        locals.var_tmf0_dn8 = assign104550_e156864_d_n8;
        locals.var_tmf0_dn9 = assign104550_e156864_d_n9;
        locals.var_tmf0_dn10 = assign104550_e156864_d_n10;
        locals.var_tmf0_dn13 = assign104550_e156864_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign104560_e156883, assign104560_e156883_d_n0, assign104560_e156883_d_n2, assign104560_e156883_d_n4, assign104560_e156883_d_n5, assign104560_e156883_d_n6, assign104560_e156883_d_n7, assign104560_e156883_d_n8, assign104560_e156883_d_n9, assign104560_e156883_d_n10, assign104560_e156883_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104560_e156875: f64 = (locals.var_noverd * p.p441);
        let assign104560_e156877: f64 = (assign104560_e156875 * locals.var_xmp);
        let assign104560_e156879: f64 = (assign104560_e156877 * locals.var_dnm);
        let assign104560_e156881: f64 = (assign104560_e156879 / locals.var_arg);
        (assign104560_e156881, ((((((assign104560_e156875 * locals.var_xmp_dn0) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn0)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn2) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn2)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn4) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn4)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn5) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn5)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn6) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn6)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn7) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn7)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn8) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn8)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn9) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn9)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn10) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn10)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn13) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn13)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104560_e156883;
        locals.var_t0_dn0 = assign104560_e156883_d_n0;
        locals.var_t0_dn2 = assign104560_e156883_d_n2;
        locals.var_t0_dn4 = assign104560_e156883_d_n4;
        locals.var_t0_dn5 = assign104560_e156883_d_n5;
        locals.var_t0_dn6 = assign104560_e156883_d_n6;
        locals.var_t0_dn7 = assign104560_e156883_d_n7;
        locals.var_t0_dn8 = assign104560_e156883_d_n8;
        locals.var_t0_dn9 = assign104560_e156883_d_n9;
        locals.var_t0_dn10 = assign104560_e156883_d_n10;
        locals.var_t0_dn13 = assign104560_e156883_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign104570_e156902, assign104570_e156902_d_n0, assign104570_e156902_d_n2, assign104570_e156902_d_n4, assign104570_e156902_d_n5, assign104570_e156902_d_n6, assign104570_e156902_d_n7, assign104570_e156902_d_n8, assign104570_e156902_d_n9, assign104570_e156902_d_n10, assign104570_e156902_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104570_e156894: f64 = (locals.var_noverd * p.p440);
        let assign104570_e156897: f64 = (locals.var_noverd * p.p441);
        let assign104570_e156898: f64 = (assign104570_e156894 - assign104570_e156897);
        let assign104570_e156900: f64 = (assign104570_e156898 + locals.var_tmf0);
        (assign104570_e156900, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104570_e156902;
        locals.var_t2_dn0 = assign104570_e156902_d_n0;
        locals.var_t2_dn2 = assign104570_e156902_d_n2;
        locals.var_t2_dn4 = assign104570_e156902_d_n4;
        locals.var_t2_dn5 = assign104570_e156902_d_n5;
        locals.var_t2_dn6 = assign104570_e156902_d_n6;
        locals.var_t2_dn7 = assign104570_e156902_d_n7;
        locals.var_t2_dn8 = assign104570_e156902_d_n8;
        locals.var_t2_dn9 = assign104570_e156902_d_n9;
        locals.var_t2_dn10 = assign104570_e156902_d_n10;
        locals.var_t2_dn13 = assign104570_e156902_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign104580_e156913, assign104580_e156913_d_n0, assign104580_e156913_d_n2, assign104580_e156913_d_n4, assign104580_e156913_d_n5, assign104580_e156913_d_n6, assign104580_e156913_d_n7, assign104580_e156913_d_n8, assign104580_e156913_d_n9, assign104580_e156913_d_n10, assign104580_e156913_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104580_e156913;
        locals.var_t0_dn0 = assign104580_e156913_d_n0;
        locals.var_t0_dn2 = assign104580_e156913_d_n2;
        locals.var_t0_dn4 = assign104580_e156913_d_n4;
        locals.var_t0_dn5 = assign104580_e156913_d_n5;
        locals.var_t0_dn6 = assign104580_e156913_d_n6;
        locals.var_t0_dn7 = assign104580_e156913_d_n7;
        locals.var_t0_dn8 = assign104580_e156913_d_n8;
        locals.var_t0_dn9 = assign104580_e156913_d_n9;
        locals.var_t0_dn10 = assign104580_e156913_d_n10;
        locals.var_t0_dn13 = assign104580_e156913_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign104590_e156925, assign104590_e156925_d_n0, assign104590_e156925_d_n2, assign104590_e156925_d_n4, assign104590_e156925_d_n5, assign104590_e156925_d_n6, assign104590_e156925_d_n7, assign104590_e156925_d_n8, assign104590_e156925_d_n9, assign104590_e156925_d_n10, assign104590_e156925_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 == 0.0)) {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104590_e156925;
        locals.var_t2_dn0 = assign104590_e156925_d_n0;
        locals.var_t2_dn2 = assign104590_e156925_d_n2;
        locals.var_t2_dn4 = assign104590_e156925_d_n4;
        locals.var_t2_dn5 = assign104590_e156925_d_n5;
        locals.var_t2_dn6 = assign104590_e156925_d_n6;
        locals.var_t2_dn7 = assign104590_e156925_d_n7;
        locals.var_t2_dn8 = assign104590_e156925_d_n8;
        locals.var_t2_dn9 = assign104590_e156925_d_n9;
        locals.var_t2_dn10 = assign104590_e156925_d_n10;
        locals.var_t2_dn13 = assign104590_e156925_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign104600_e156937, assign104600_e156937_d_n0, assign104600_e156937_d_n2, assign104600_e156937_d_n4, assign104600_e156937_d_n5, assign104600_e156937_d_n6, assign104600_e156937_d_n7, assign104600_e156937_d_n8, assign104600_e156937_d_n9, assign104600_e156937_d_n10, assign104600_e156937_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104600_e156937;
        locals.var_t0_dn0 = assign104600_e156937_d_n0;
        locals.var_t0_dn2 = assign104600_e156937_d_n2;
        locals.var_t0_dn4 = assign104600_e156937_d_n4;
        locals.var_t0_dn5 = assign104600_e156937_d_n5;
        locals.var_t0_dn6 = assign104600_e156937_d_n6;
        locals.var_t0_dn7 = assign104600_e156937_d_n7;
        locals.var_t0_dn8 = assign104600_e156937_d_n8;
        locals.var_t0_dn9 = assign104600_e156937_d_n9;
        locals.var_t0_dn10 = assign104600_e156937_d_n10;
        locals.var_t0_dn13 = assign104600_e156937_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign104610_e156946, assign104610_e156946_d_n0, assign104610_e156946_d_n2, assign104610_e156946_d_n4, assign104610_e156946_d_n5, assign104610_e156946_d_n6, assign104610_e156946_d_n7, assign104610_e156946_d_n8, assign104610_e156946_d_n9, assign104610_e156946_d_n10, assign104610_e156946_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn13,)
    }
};
        locals.var_carr = assign104610_e156946;
        locals.var_carr_dn0 = assign104610_e156946_d_n0;
        locals.var_carr_dn2 = assign104610_e156946_d_n2;
        locals.var_carr_dn4 = assign104610_e156946_d_n4;
        locals.var_carr_dn5 = assign104610_e156946_d_n5;
        locals.var_carr_dn6 = assign104610_e156946_d_n6;
        locals.var_carr_dn7 = assign104610_e156946_d_n7;
        locals.var_carr_dn8 = assign104610_e156946_d_n8;
        locals.var_carr_dn9 = assign104610_e156946_d_n9;
        locals.var_carr_dn10 = assign104610_e156946_d_n10;
        locals.var_carr_dn13 = assign104610_e156946_d_n13;
        locals.var_carr_rv = 0.0;

        let (assign104620_e156954, assign104620_e156954_d_n0, assign104620_e156954_d_n2, assign104620_e156954_d_n4, assign104620_e156954_d_n5, assign104620_e156954_d_n6, assign104620_e156954_d_n7, assign104620_e156954_d_n8, assign104620_e156954_d_n9, assign104620_e156954_d_n10, assign104620_e156954_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104620_e156952: f64 = (-locals.var_rd_ps0ld);
        (assign104620_e156952, (-locals.var_rd_ps0ld_dn0), (-locals.var_rd_ps0ld_dn2), (-locals.var_rd_ps0ld_dn4), (-locals.var_rd_ps0ld_dn5), (-locals.var_rd_ps0ld_dn6), (-locals.var_rd_ps0ld_dn7), (-locals.var_rd_ps0ld_dn8), (-locals.var_rd_ps0ld_dn9), (-locals.var_rd_ps0ld_dn10), (-locals.var_rd_ps0ld_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104620_e156954;
        locals.var_t0_dn0 = assign104620_e156954_d_n0;
        locals.var_t0_dn2 = assign104620_e156954_d_n2;
        locals.var_t0_dn4 = assign104620_e156954_d_n4;
        locals.var_t0_dn5 = assign104620_e156954_d_n5;
        locals.var_t0_dn6 = assign104620_e156954_d_n6;
        locals.var_t0_dn7 = assign104620_e156954_d_n7;
        locals.var_t0_dn8 = assign104620_e156954_d_n8;
        locals.var_t0_dn9 = assign104620_e156954_d_n9;
        locals.var_t0_dn10 = assign104620_e156954_d_n10;
        locals.var_t0_dn13 = assign104620_e156954_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign104630_e156970, assign104630_e156970_d_n0, assign104630_e156970_d_n2, assign104630_e156970_d_n4, assign104630_e156970_d_n5, assign104630_e156970_d_n6, assign104630_e156970_d_n7, assign104630_e156970_d_n8, assign104630_e156970_d_n9, assign104630_e156970_d_n10, assign104630_e156970_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104630_e156961: f64 = (locals.var_t0 * locals.var_t0);
        let assign104630_e156964: f64 = (4.0 * 0.01);
        let assign104630_e156966: f64 = (assign104630_e156964 * 0.01);
        let assign104630_e156967: f64 = (assign104630_e156961 + assign104630_e156966);
        let assign104630_e156968: f64 = (assign104630_e156967).sqrt();
        (assign104630_e156968, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign104630_e156968)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104630_e156970;
        locals.var_tmf2_dn0 = assign104630_e156970_d_n0;
        locals.var_tmf2_dn2 = assign104630_e156970_d_n2;
        locals.var_tmf2_dn4 = assign104630_e156970_d_n4;
        locals.var_tmf2_dn5 = assign104630_e156970_d_n5;
        locals.var_tmf2_dn6 = assign104630_e156970_d_n6;
        locals.var_tmf2_dn7 = assign104630_e156970_d_n7;
        locals.var_tmf2_dn8 = assign104630_e156970_d_n8;
        locals.var_tmf2_dn9 = assign104630_e156970_d_n9;
        locals.var_tmf2_dn10 = assign104630_e156970_d_n10;
        locals.var_tmf2_dn13 = assign104630_e156970_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign104640_e156983, assign104640_e156983_d_n0, assign104640_e156983_d_n2, assign104640_e156983_d_n4, assign104640_e156983_d_n5, assign104640_e156983_d_n6, assign104640_e156983_d_n7, assign104640_e156983_d_n8, assign104640_e156983_d_n9, assign104640_e156983_d_n10, assign104640_e156983_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104640_e156979: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign104640_e156980: f64 = (1.0 + assign104640_e156979);
        let assign104640_e156981: f64 = (0.5 * assign104640_e156980);
        (assign104640_e156981, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn13 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104640_e156983;
        locals.var_t9_dn0 = assign104640_e156983_d_n0;
        locals.var_t9_dn2 = assign104640_e156983_d_n2;
        locals.var_t9_dn4 = assign104640_e156983_d_n4;
        locals.var_t9_dn5 = assign104640_e156983_d_n5;
        locals.var_t9_dn6 = assign104640_e156983_d_n6;
        locals.var_t9_dn7 = assign104640_e156983_d_n7;
        locals.var_t9_dn8 = assign104640_e156983_d_n8;
        locals.var_t9_dn9 = assign104640_e156983_d_n9;
        locals.var_t9_dn10 = assign104640_e156983_d_n10;
        locals.var_t9_dn13 = assign104640_e156983_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign104650_e156994, assign104650_e156994_d_n0, assign104650_e156994_d_n2, assign104650_e156994_d_n4, assign104650_e156994_d_n5, assign104650_e156994_d_n6, assign104650_e156994_d_n7, assign104650_e156994_d_n8, assign104650_e156994_d_n9, assign104650_e156994_d_n10, assign104650_e156994_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104650_e156991: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign104650_e156992: f64 = (0.5 * assign104650_e156991);
        (assign104650_e156992, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104650_e156994;
        locals.var_t0_dn0 = assign104650_e156994_d_n0;
        locals.var_t0_dn2 = assign104650_e156994_d_n2;
        locals.var_t0_dn4 = assign104650_e156994_d_n4;
        locals.var_t0_dn5 = assign104650_e156994_d_n5;
        locals.var_t0_dn6 = assign104650_e156994_d_n6;
        locals.var_t0_dn7 = assign104650_e156994_d_n7;
        locals.var_t0_dn8 = assign104650_e156994_d_n8;
        locals.var_t0_dn9 = assign104650_e156994_d_n9;
        locals.var_t0_dn10 = assign104650_e156994_d_n10;
        locals.var_t0_dn13 = assign104650_e156994_d_n13;
        locals.var_t0_rv = 0.0;

        let assign104660_e156997: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2378 = assign104660_e156997;
        locals.var_guard2378_rv = 0.0;

        let (assign104670_e157006, assign104670_e157006_d_n0, assign104670_e157006_d_n2, assign104670_e157006_d_n4, assign104670_e157006_d_n5, assign104670_e157006_d_n6, assign104670_e157006_d_n7, assign104670_e157006_d_n8, assign104670_e157006_d_n9, assign104670_e157006_d_n10, assign104670_e157006_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2378 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104670_e157006;
        locals.var_t0_dn0 = assign104670_e157006_d_n0;
        locals.var_t0_dn2 = assign104670_e157006_d_n2;
        locals.var_t0_dn4 = assign104670_e157006_d_n4;
        locals.var_t0_dn5 = assign104670_e157006_d_n5;
        locals.var_t0_dn6 = assign104670_e157006_d_n6;
        locals.var_t0_dn7 = assign104670_e157006_d_n7;
        locals.var_t0_dn8 = assign104670_e157006_d_n8;
        locals.var_t0_dn9 = assign104670_e157006_d_n9;
        locals.var_t0_dn10 = assign104670_e157006_d_n10;
        locals.var_t0_dn13 = assign104670_e157006_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign104680_e157015, assign104680_e157015_d_n0, assign104680_e157015_d_n2, assign104680_e157015_d_n4, assign104680_e157015_d_n5, assign104680_e157015_d_n6, assign104680_e157015_d_n7, assign104680_e157015_d_n8, assign104680_e157015_d_n9, assign104680_e157015_d_n10, assign104680_e157015_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2378 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104680_e157015;
        locals.var_t9_dn0 = assign104680_e157015_d_n0;
        locals.var_t9_dn2 = assign104680_e157015_d_n2;
        locals.var_t9_dn4 = assign104680_e157015_d_n4;
        locals.var_t9_dn5 = assign104680_e157015_d_n5;
        locals.var_t9_dn6 = assign104680_e157015_d_n6;
        locals.var_t9_dn7 = assign104680_e157015_d_n7;
        locals.var_t9_dn8 = assign104680_e157015_d_n8;
        locals.var_t9_dn9 = assign104680_e157015_d_n9;
        locals.var_t9_dn10 = assign104680_e157015_d_n10;
        locals.var_t9_dn13 = assign104680_e157015_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign104690_e157026, assign104690_e157026_d_n0, assign104690_e157026_d_n2, assign104690_e157026_d_n4, assign104690_e157026_d_n5, assign104690_e157026_d_n6, assign104690_e157026_d_n7, assign104690_e157026_d_n8, assign104690_e157026_d_n9, assign104690_e157026_d_n10, assign104690_e157026_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104690_e157023: f64 = (10.0 * 2.220446049250313e-16);
        let assign104690_e157024: f64 = (locals.var_t0 + assign104690_e157023);
        (assign104690_e157024, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104690_e157026;
        locals.var_t0_dn0 = assign104690_e157026_d_n0;
        locals.var_t0_dn2 = assign104690_e157026_d_n2;
        locals.var_t0_dn4 = assign104690_e157026_d_n4;
        locals.var_t0_dn5 = assign104690_e157026_d_n5;
        locals.var_t0_dn6 = assign104690_e157026_d_n6;
        locals.var_t0_dn7 = assign104690_e157026_d_n7;
        locals.var_t0_dn8 = assign104690_e157026_d_n8;
        locals.var_t0_dn9 = assign104690_e157026_d_n9;
        locals.var_t0_dn10 = assign104690_e157026_d_n10;
        locals.var_t0_dn13 = assign104690_e157026_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign104700_e157036, assign104700_e157036_d_n0, assign104700_e157036_d_n2, assign104700_e157036_d_n4, assign104700_e157036_d_n5, assign104700_e157036_d_n6, assign104700_e157036_d_n7, assign104700_e157036_d_n8, assign104700_e157036_d_n9, assign104700_e157036_d_n10, assign104700_e157036_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104700_e157033: f64 = (locals.var_kdep * locals.var_t0);
        let assign104700_e157034: f64 = (assign104700_e157033).sqrt();
        (assign104700_e157034, ((locals.var_kdep * locals.var_t0_dn0) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn2) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn4) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn5) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn6) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn7) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn8) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn9) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn10) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn13) / (2.0 * assign104700_e157034)),)
    } else {
        (locals.var_wdepl, locals.var_wdepl_dn0, locals.var_wdepl_dn2, locals.var_wdepl_dn4, locals.var_wdepl_dn5, locals.var_wdepl_dn6, locals.var_wdepl_dn7, locals.var_wdepl_dn8, locals.var_wdepl_dn9, locals.var_wdepl_dn10, locals.var_wdepl_dn13,)
    }
};
        locals.var_wdepl = assign104700_e157036;
        locals.var_wdepl_dn0 = assign104700_e157036_d_n0;
        locals.var_wdepl_dn2 = assign104700_e157036_d_n2;
        locals.var_wdepl_dn4 = assign104700_e157036_d_n4;
        locals.var_wdepl_dn5 = assign104700_e157036_d_n5;
        locals.var_wdepl_dn6 = assign104700_e157036_d_n6;
        locals.var_wdepl_dn7 = assign104700_e157036_d_n7;
        locals.var_wdepl_dn8 = assign104700_e157036_d_n8;
        locals.var_wdepl_dn9 = assign104700_e157036_d_n9;
        locals.var_wdepl_dn10 = assign104700_e157036_d_n10;
        locals.var_wdepl_dn13 = assign104700_e157036_d_n13;
        locals.var_wdepl_rv = 0.0;

        let (assign104710_e157047, assign104710_e157047_d_n0, assign104710_e157047_d_n2, assign104710_e157047_d_n4, assign104710_e157047_d_n5, assign104710_e157047_d_n6, assign104710_e157047_d_n7, assign104710_e157047_d_n8, assign104710_e157047_d_n9, assign104710_e157047_d_n10, assign104710_e157047_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104710_e157043: f64 = (locals.var_vds__blk2352 - locals.var_vbs__blk2353);
        let assign104710_e157045: f64 = (assign104710_e157043 + p.p137);
        (assign104710_e157045, 0.0, 0.0, 0.0, locals.var_vds__blk2352_dn5, 0.0, (locals.var_vds__blk2352_dn7 - locals.var_vbs__blk2353_dn7), (-locals.var_vbs__blk2353_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104710_e157047;
        locals.var_t2_dn0 = assign104710_e157047_d_n0;
        locals.var_t2_dn2 = assign104710_e157047_d_n2;
        locals.var_t2_dn4 = assign104710_e157047_d_n4;
        locals.var_t2_dn5 = assign104710_e157047_d_n5;
        locals.var_t2_dn6 = assign104710_e157047_d_n6;
        locals.var_t2_dn7 = assign104710_e157047_d_n7;
        locals.var_t2_dn8 = assign104710_e157047_d_n8;
        locals.var_t2_dn9 = assign104710_e157047_d_n9;
        locals.var_t2_dn10 = assign104710_e157047_d_n10;
        locals.var_t2_dn13 = assign104710_e157047_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign104720_e157063, assign104720_e157063_d_n0, assign104720_e157063_d_n2, assign104720_e157063_d_n4, assign104720_e157063_d_n5, assign104720_e157063_d_n6, assign104720_e157063_d_n7, assign104720_e157063_d_n8, assign104720_e157063_d_n9, assign104720_e157063_d_n10, assign104720_e157063_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104720_e157054: f64 = (locals.var_t2 * locals.var_t2);
        let assign104720_e157057: f64 = (4.0 * 0.01);
        let assign104720_e157059: f64 = (assign104720_e157057 * 0.01);
        let assign104720_e157060: f64 = (assign104720_e157054 + assign104720_e157059);
        let assign104720_e157061: f64 = (assign104720_e157060).sqrt();
        (assign104720_e157061, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign104720_e157061)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104720_e157063;
        locals.var_tmf2_dn0 = assign104720_e157063_d_n0;
        locals.var_tmf2_dn2 = assign104720_e157063_d_n2;
        locals.var_tmf2_dn4 = assign104720_e157063_d_n4;
        locals.var_tmf2_dn5 = assign104720_e157063_d_n5;
        locals.var_tmf2_dn6 = assign104720_e157063_d_n6;
        locals.var_tmf2_dn7 = assign104720_e157063_d_n7;
        locals.var_tmf2_dn8 = assign104720_e157063_d_n8;
        locals.var_tmf2_dn9 = assign104720_e157063_d_n9;
        locals.var_tmf2_dn10 = assign104720_e157063_d_n10;
        locals.var_tmf2_dn13 = assign104720_e157063_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign104730_e157076, assign104730_e157076_d_n0, assign104730_e157076_d_n2, assign104730_e157076_d_n4, assign104730_e157076_d_n5, assign104730_e157076_d_n6, assign104730_e157076_d_n7, assign104730_e157076_d_n8, assign104730_e157076_d_n9, assign104730_e157076_d_n10, assign104730_e157076_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104730_e157072: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign104730_e157073: f64 = (1.0 + assign104730_e157072);
        let assign104730_e157074: f64 = (0.5 * assign104730_e157073);
        (assign104730_e157074, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104730_e157076;
        locals.var_t9_dn0 = assign104730_e157076_d_n0;
        locals.var_t9_dn2 = assign104730_e157076_d_n2;
        locals.var_t9_dn4 = assign104730_e157076_d_n4;
        locals.var_t9_dn5 = assign104730_e157076_d_n5;
        locals.var_t9_dn6 = assign104730_e157076_d_n6;
        locals.var_t9_dn7 = assign104730_e157076_d_n7;
        locals.var_t9_dn8 = assign104730_e157076_d_n8;
        locals.var_t9_dn9 = assign104730_e157076_d_n9;
        locals.var_t9_dn10 = assign104730_e157076_d_n10;
        locals.var_t9_dn13 = assign104730_e157076_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign104740_e157087, assign104740_e157087_d_n0, assign104740_e157087_d_n2, assign104740_e157087_d_n4, assign104740_e157087_d_n5, assign104740_e157087_d_n6, assign104740_e157087_d_n7, assign104740_e157087_d_n8, assign104740_e157087_d_n9, assign104740_e157087_d_n10, assign104740_e157087_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104740_e157084: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign104740_e157085: f64 = (0.5 * assign104740_e157084);
        (assign104740_e157085, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104740_e157087;
        locals.var_t2_dn0 = assign104740_e157087_d_n0;
        locals.var_t2_dn2 = assign104740_e157087_d_n2;
        locals.var_t2_dn4 = assign104740_e157087_d_n4;
        locals.var_t2_dn5 = assign104740_e157087_d_n5;
        locals.var_t2_dn6 = assign104740_e157087_d_n6;
        locals.var_t2_dn7 = assign104740_e157087_d_n7;
        locals.var_t2_dn8 = assign104740_e157087_d_n8;
        locals.var_t2_dn9 = assign104740_e157087_d_n9;
        locals.var_t2_dn10 = assign104740_e157087_d_n10;
        locals.var_t2_dn13 = assign104740_e157087_d_n13;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_388(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign104750_e157090: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2379 = assign104750_e157090;
        locals.var_guard2379_rv = 0.0;

        let (assign104760_e157099, assign104760_e157099_d_n0, assign104760_e157099_d_n2, assign104760_e157099_d_n4, assign104760_e157099_d_n5, assign104760_e157099_d_n6, assign104760_e157099_d_n7, assign104760_e157099_d_n8, assign104760_e157099_d_n9, assign104760_e157099_d_n10, assign104760_e157099_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2379 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104760_e157099;
        locals.var_t2_dn0 = assign104760_e157099_d_n0;
        locals.var_t2_dn2 = assign104760_e157099_d_n2;
        locals.var_t2_dn4 = assign104760_e157099_d_n4;
        locals.var_t2_dn5 = assign104760_e157099_d_n5;
        locals.var_t2_dn6 = assign104760_e157099_d_n6;
        locals.var_t2_dn7 = assign104760_e157099_d_n7;
        locals.var_t2_dn8 = assign104760_e157099_d_n8;
        locals.var_t2_dn9 = assign104760_e157099_d_n9;
        locals.var_t2_dn10 = assign104760_e157099_d_n10;
        locals.var_t2_dn13 = assign104760_e157099_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign104770_e157108, assign104770_e157108_d_n0, assign104770_e157108_d_n2, assign104770_e157108_d_n4, assign104770_e157108_d_n5, assign104770_e157108_d_n6, assign104770_e157108_d_n7, assign104770_e157108_d_n8, assign104770_e157108_d_n9, assign104770_e157108_d_n10, assign104770_e157108_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2379 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104770_e157108;
        locals.var_t9_dn0 = assign104770_e157108_d_n0;
        locals.var_t9_dn2 = assign104770_e157108_d_n2;
        locals.var_t9_dn4 = assign104770_e157108_d_n4;
        locals.var_t9_dn5 = assign104770_e157108_d_n5;
        locals.var_t9_dn6 = assign104770_e157108_d_n6;
        locals.var_t9_dn7 = assign104770_e157108_d_n7;
        locals.var_t9_dn8 = assign104770_e157108_d_n8;
        locals.var_t9_dn9 = assign104770_e157108_d_n9;
        locals.var_t9_dn10 = assign104770_e157108_d_n10;
        locals.var_t9_dn13 = assign104770_e157108_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign104780_e157119, assign104780_e157119_d_n0, assign104780_e157119_d_n2, assign104780_e157119_d_n4, assign104780_e157119_d_n5, assign104780_e157119_d_n6, assign104780_e157119_d_n7, assign104780_e157119_d_n8, assign104780_e157119_d_n9, assign104780_e157119_d_n10, assign104780_e157119_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104780_e157116: f64 = (10.0 * 2.220446049250313e-16);
        let assign104780_e157117: f64 = (locals.var_t2 + assign104780_e157116);
        (assign104780_e157117, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104780_e157119;
        locals.var_t2_dn0 = assign104780_e157119_d_n0;
        locals.var_t2_dn2 = assign104780_e157119_d_n2;
        locals.var_t2_dn4 = assign104780_e157119_d_n4;
        locals.var_t2_dn5 = assign104780_e157119_d_n5;
        locals.var_t2_dn6 = assign104780_e157119_d_n6;
        locals.var_t2_dn7 = assign104780_e157119_d_n7;
        locals.var_t2_dn8 = assign104780_e157119_d_n8;
        locals.var_t2_dn9 = assign104780_e157119_d_n9;
        locals.var_t2_dn10 = assign104780_e157119_d_n10;
        locals.var_t2_dn13 = assign104780_e157119_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign104790_e157129, assign104790_e157129_d_n0, assign104790_e157129_d_n2, assign104790_e157129_d_n4, assign104790_e157129_d_n5, assign104790_e157129_d_n6, assign104790_e157129_d_n7, assign104790_e157129_d_n8, assign104790_e157129_d_n9, assign104790_e157129_d_n10, assign104790_e157129_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104790_e157126: f64 = (locals.var_kjunc * locals.var_t2);
        let assign104790_e157127: f64 = (assign104790_e157126).sqrt();
        (assign104790_e157127, (((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign104790_e157127)),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign104790_e157129;
        locals.var_wjunc0_dn0 = assign104790_e157129_d_n0;
        locals.var_wjunc0_dn2 = assign104790_e157129_d_n2;
        locals.var_wjunc0_dn4 = assign104790_e157129_d_n4;
        locals.var_wjunc0_dn5 = assign104790_e157129_d_n5;
        locals.var_wjunc0_dn6 = assign104790_e157129_d_n6;
        locals.var_wjunc0_dn7 = assign104790_e157129_d_n7;
        locals.var_wjunc0_dn8 = assign104790_e157129_d_n8;
        locals.var_wjunc0_dn9 = assign104790_e157129_d_n9;
        locals.var_wjunc0_dn10 = assign104790_e157129_d_n10;
        locals.var_wjunc0_dn13 = assign104790_e157129_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign104800_e157142, assign104800_e157142_d_n0, assign104800_e157142_d_n2, assign104800_e157142_d_n4, assign104800_e157142_d_n5, assign104800_e157142_d_n6, assign104800_e157142_d_n7, assign104800_e157142_d_n8, assign104800_e157142_d_n9, assign104800_e157142_d_n10, assign104800_e157142_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104800_e157136: f64 = (locals.var_rd_xldld - locals.var_wjunc0);
        let assign104800_e157139: f64 = (0.01 * locals.var_rd_xldld);
        let assign104800_e157140: f64 = (assign104800_e157136 - assign104800_e157139);
        (assign104800_e157140, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign104800_e157142;
        locals.var_tmf1_dn0 = assign104800_e157142_d_n0;
        locals.var_tmf1_dn2 = assign104800_e157142_d_n2;
        locals.var_tmf1_dn4 = assign104800_e157142_d_n4;
        locals.var_tmf1_dn5 = assign104800_e157142_d_n5;
        locals.var_tmf1_dn6 = assign104800_e157142_d_n6;
        locals.var_tmf1_dn7 = assign104800_e157142_d_n7;
        locals.var_tmf1_dn8 = assign104800_e157142_d_n8;
        locals.var_tmf1_dn9 = assign104800_e157142_d_n9;
        locals.var_tmf1_dn10 = assign104800_e157142_d_n10;
        locals.var_tmf1_dn13 = assign104800_e157142_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign104810_e157155, assign104810_e157155_d_n0, assign104810_e157155_d_n2, assign104810_e157155_d_n4, assign104810_e157155_d_n5, assign104810_e157155_d_n6, assign104810_e157155_d_n7, assign104810_e157155_d_n8, assign104810_e157155_d_n9, assign104810_e157155_d_n10, assign104810_e157155_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104810_e157149: f64 = (4.0 * locals.var_rd_xldld);
        let assign104810_e157152: f64 = (0.01 * locals.var_rd_xldld);
        let assign104810_e157153: f64 = (assign104810_e157149 * assign104810_e157152);
        (assign104810_e157153, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104810_e157155;
        locals.var_tmf2_dn0 = assign104810_e157155_d_n0;
        locals.var_tmf2_dn2 = assign104810_e157155_d_n2;
        locals.var_tmf2_dn4 = assign104810_e157155_d_n4;
        locals.var_tmf2_dn5 = assign104810_e157155_d_n5;
        locals.var_tmf2_dn6 = assign104810_e157155_d_n6;
        locals.var_tmf2_dn7 = assign104810_e157155_d_n7;
        locals.var_tmf2_dn8 = assign104810_e157155_d_n8;
        locals.var_tmf2_dn9 = assign104810_e157155_d_n9;
        locals.var_tmf2_dn10 = assign104810_e157155_d_n10;
        locals.var_tmf2_dn13 = assign104810_e157155_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign104820_e157168, assign104820_e157168_d_n0, assign104820_e157168_d_n2, assign104820_e157168_d_n4, assign104820_e157168_d_n5, assign104820_e157168_d_n6, assign104820_e157168_d_n7, assign104820_e157168_d_n8, assign104820_e157168_d_n9, assign104820_e157168_d_n10, assign104820_e157168_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let (assign104820_e157166, assign104820_e157166_d_n0, assign104820_e157166_d_n2, assign104820_e157166_d_n4, assign104820_e157166_d_n5, assign104820_e157166_d_n6, assign104820_e157166_d_n7, assign104820_e157166_d_n8, assign104820_e157166_d_n9, assign104820_e157166_d_n10, assign104820_e157166_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign104820_e157165: f64 = (-locals.var_tmf2);
                (assign104820_e157165, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign104820_e157166, assign104820_e157166_d_n0, assign104820_e157166_d_n2, assign104820_e157166_d_n4, assign104820_e157166_d_n5, assign104820_e157166_d_n6, assign104820_e157166_d_n7, assign104820_e157166_d_n8, assign104820_e157166_d_n9, assign104820_e157166_d_n10, assign104820_e157166_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104820_e157168;
        locals.var_tmf2_dn0 = assign104820_e157168_d_n0;
        locals.var_tmf2_dn2 = assign104820_e157168_d_n2;
        locals.var_tmf2_dn4 = assign104820_e157168_d_n4;
        locals.var_tmf2_dn5 = assign104820_e157168_d_n5;
        locals.var_tmf2_dn6 = assign104820_e157168_d_n6;
        locals.var_tmf2_dn7 = assign104820_e157168_d_n7;
        locals.var_tmf2_dn8 = assign104820_e157168_d_n8;
        locals.var_tmf2_dn9 = assign104820_e157168_d_n9;
        locals.var_tmf2_dn10 = assign104820_e157168_d_n10;
        locals.var_tmf2_dn13 = assign104820_e157168_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign104830_e157180, assign104830_e157180_d_n0, assign104830_e157180_d_n2, assign104830_e157180_d_n4, assign104830_e157180_d_n5, assign104830_e157180_d_n6, assign104830_e157180_d_n7, assign104830_e157180_d_n8, assign104830_e157180_d_n9, assign104830_e157180_d_n10, assign104830_e157180_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104830_e157175: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104830_e157177: f64 = (assign104830_e157175 + locals.var_tmf2);
        let assign104830_e157178: f64 = (assign104830_e157177).sqrt();
        (assign104830_e157178, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign104830_e157178)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104830_e157180;
        locals.var_tmf2_dn0 = assign104830_e157180_d_n0;
        locals.var_tmf2_dn2 = assign104830_e157180_d_n2;
        locals.var_tmf2_dn4 = assign104830_e157180_d_n4;
        locals.var_tmf2_dn5 = assign104830_e157180_d_n5;
        locals.var_tmf2_dn6 = assign104830_e157180_d_n6;
        locals.var_tmf2_dn7 = assign104830_e157180_d_n7;
        locals.var_tmf2_dn8 = assign104830_e157180_d_n8;
        locals.var_tmf2_dn9 = assign104830_e157180_d_n9;
        locals.var_tmf2_dn10 = assign104830_e157180_d_n10;
        locals.var_tmf2_dn13 = assign104830_e157180_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign104840_e157193, assign104840_e157193_d_n0, assign104840_e157193_d_n2, assign104840_e157193_d_n4, assign104840_e157193_d_n5, assign104840_e157193_d_n6, assign104840_e157193_d_n7, assign104840_e157193_d_n8, assign104840_e157193_d_n9, assign104840_e157193_d_n10, assign104840_e157193_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104840_e157189: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104840_e157190: f64 = (1.0 + assign104840_e157189);
        let assign104840_e157191: f64 = (0.5 * assign104840_e157190);
        (assign104840_e157191, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104840_e157193;
        locals.var_t0_dn0 = assign104840_e157193_d_n0;
        locals.var_t0_dn2 = assign104840_e157193_d_n2;
        locals.var_t0_dn4 = assign104840_e157193_d_n4;
        locals.var_t0_dn5 = assign104840_e157193_d_n5;
        locals.var_t0_dn6 = assign104840_e157193_d_n6;
        locals.var_t0_dn7 = assign104840_e157193_d_n7;
        locals.var_t0_dn8 = assign104840_e157193_d_n8;
        locals.var_t0_dn9 = assign104840_e157193_d_n9;
        locals.var_t0_dn10 = assign104840_e157193_d_n10;
        locals.var_t0_dn13 = assign104840_e157193_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign104850_e157206, assign104850_e157206_d_n0, assign104850_e157206_d_n2, assign104850_e157206_d_n4, assign104850_e157206_d_n5, assign104850_e157206_d_n6, assign104850_e157206_d_n7, assign104850_e157206_d_n8, assign104850_e157206_d_n9, assign104850_e157206_d_n10, assign104850_e157206_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104850_e157202: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104850_e157203: f64 = (0.5 * assign104850_e157202);
        let assign104850_e157204: f64 = (locals.var_rd_xldld - assign104850_e157203);
        (assign104850_e157204, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wjunc, locals.var_wjunc_dn0, locals.var_wjunc_dn2, locals.var_wjunc_dn4, locals.var_wjunc_dn5, locals.var_wjunc_dn6, locals.var_wjunc_dn7, locals.var_wjunc_dn8, locals.var_wjunc_dn9, locals.var_wjunc_dn10, locals.var_wjunc_dn13,)
    }
};
        locals.var_wjunc = assign104850_e157206;
        locals.var_wjunc_dn0 = assign104850_e157206_d_n0;
        locals.var_wjunc_dn2 = assign104850_e157206_d_n2;
        locals.var_wjunc_dn4 = assign104850_e157206_d_n4;
        locals.var_wjunc_dn5 = assign104850_e157206_d_n5;
        locals.var_wjunc_dn6 = assign104850_e157206_d_n6;
        locals.var_wjunc_dn7 = assign104850_e157206_d_n7;
        locals.var_wjunc_dn8 = assign104850_e157206_d_n8;
        locals.var_wjunc_dn9 = assign104850_e157206_d_n9;
        locals.var_wjunc_dn10 = assign104850_e157206_d_n10;
        locals.var_wjunc_dn13 = assign104850_e157206_d_n13;
        locals.var_wjunc_rv = 0.0;

        let (assign104860_e157215,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104860_e157213: f64 = (p.p419 + 1e-25);
        (assign104860_e157213,)
    } else {
        (locals.var_wrdrdjunc,)
    }
};
        locals.var_wrdrdjunc = assign104860_e157215;
        locals.var_wrdrdjunc_rv = 0.0;

        let (assign104870_e157234, assign104870_e157234_d_n0, assign104870_e157234_d_n2, assign104870_e157234_d_n4, assign104870_e157234_d_n5, assign104870_e157234_d_n6, assign104870_e157234_d_n7, assign104870_e157234_d_n8, assign104870_e157234_d_n9, assign104870_e157234_d_n10, assign104870_e157234_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104870_e157225: f64 = (locals.var_wdepl / locals.var_wrdrdjunc);
        let assign104870_e157228: f64 = (locals.var_wjunc / locals.var_rd_xldld);
        let assign104870_e157229: f64 = (assign104870_e157225 + assign104870_e157228);
        let assign104870_e157230: f64 = (locals.var_cx * assign104870_e157229);
        let assign104870_e157231: f64 = (1.0 - assign104870_e157230);
        let assign104870_e157232: f64 = (locals.var_xmax * assign104870_e157231);
        (assign104870_e157232, (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn0 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn0 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn2 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn2 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn4 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn4 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn5 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn5 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn6 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn6 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn7 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn7 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn8 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn8 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn9 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn9 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn10 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn10 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn13 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn13 / locals.var_rd_xldld))))),)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn13,)
    }
};
        locals.var_xov = assign104870_e157234;
        locals.var_xov_dn0 = assign104870_e157234_d_n0;
        locals.var_xov_dn2 = assign104870_e157234_d_n2;
        locals.var_xov_dn4 = assign104870_e157234_d_n4;
        locals.var_xov_dn5 = assign104870_e157234_d_n5;
        locals.var_xov_dn6 = assign104870_e157234_d_n6;
        locals.var_xov_dn7 = assign104870_e157234_d_n7;
        locals.var_xov_dn8 = assign104870_e157234_d_n8;
        locals.var_xov_dn9 = assign104870_e157234_d_n9;
        locals.var_xov_dn10 = assign104870_e157234_d_n10;
        locals.var_xov_dn13 = assign104870_e157234_d_n13;
        locals.var_xov_rv = 0.0;

        let (assign104880_e157262, assign104880_e157262_d_n0, assign104880_e157262_d_n2, assign104880_e157262_d_n4, assign104880_e157262_d_n5, assign104880_e157262_d_n6, assign104880_e157262_d_n7, assign104880_e157262_d_n8, assign104880_e157262_d_n9, assign104880_e157262_d_n10, assign104880_e157262_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104880_e157241: f64 = (locals.var_xov * locals.var_xov);
        let assign104880_e157245: f64 = (1.0 - locals.var_uc_rdrcx);
        let assign104880_e157247: f64 = (assign104880_e157245 * locals.var_xmax);
        let assign104880_e157249: f64 = (assign104880_e157247 / 100.0);
        let assign104880_e157250: f64 = (4.0 * assign104880_e157249);
        let assign104880_e157253: f64 = (1.0 - locals.var_uc_rdrcx);
        let assign104880_e157255: f64 = (assign104880_e157253 * locals.var_xmax);
        let assign104880_e157257: f64 = (assign104880_e157255 / 100.0);
        let assign104880_e157258: f64 = (assign104880_e157250 * assign104880_e157257);
        let assign104880_e157259: f64 = (assign104880_e157241 + assign104880_e157258);
        let assign104880_e157260: f64 = (assign104880_e157259).sqrt();
        (assign104880_e157260, (((locals.var_xov_dn0 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn0)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn2 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn2)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn4 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn4)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn5 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn5)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn6 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn6)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn7 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn7)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn8 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn8)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn9 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn9)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn10 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn10)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn13 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn13)) / (2.0 * assign104880_e157260)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104880_e157262;
        locals.var_tmf2_dn0 = assign104880_e157262_d_n0;
        locals.var_tmf2_dn2 = assign104880_e157262_d_n2;
        locals.var_tmf2_dn4 = assign104880_e157262_d_n4;
        locals.var_tmf2_dn5 = assign104880_e157262_d_n5;
        locals.var_tmf2_dn6 = assign104880_e157262_d_n6;
        locals.var_tmf2_dn7 = assign104880_e157262_d_n7;
        locals.var_tmf2_dn8 = assign104880_e157262_d_n8;
        locals.var_tmf2_dn9 = assign104880_e157262_d_n9;
        locals.var_tmf2_dn10 = assign104880_e157262_d_n10;
        locals.var_tmf2_dn13 = assign104880_e157262_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign104890_e157275, assign104890_e157275_d_n0, assign104890_e157275_d_n2, assign104890_e157275_d_n4, assign104890_e157275_d_n5, assign104890_e157275_d_n6, assign104890_e157275_d_n7, assign104890_e157275_d_n8, assign104890_e157275_d_n9, assign104890_e157275_d_n10, assign104890_e157275_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104890_e157271: f64 = (locals.var_xov / locals.var_tmf2);
        let assign104890_e157272: f64 = (1.0 + assign104890_e157271);
        let assign104890_e157273: f64 = (0.5 * assign104890_e157272);
        (assign104890_e157273, (0.5 * (((locals.var_xov_dn0 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn2 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn4 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn5 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn6 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn7 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn8 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn9 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn10 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn13 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104890_e157275;
        locals.var_t9_dn0 = assign104890_e157275_d_n0;
        locals.var_t9_dn2 = assign104890_e157275_d_n2;
        locals.var_t9_dn4 = assign104890_e157275_d_n4;
        locals.var_t9_dn5 = assign104890_e157275_d_n5;
        locals.var_t9_dn6 = assign104890_e157275_d_n6;
        locals.var_t9_dn7 = assign104890_e157275_d_n7;
        locals.var_t9_dn8 = assign104890_e157275_d_n8;
        locals.var_t9_dn9 = assign104890_e157275_d_n9;
        locals.var_t9_dn10 = assign104890_e157275_d_n10;
        locals.var_t9_dn13 = assign104890_e157275_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign104900_e157286, assign104900_e157286_d_n0, assign104900_e157286_d_n2, assign104900_e157286_d_n4, assign104900_e157286_d_n5, assign104900_e157286_d_n6, assign104900_e157286_d_n7, assign104900_e157286_d_n8, assign104900_e157286_d_n9, assign104900_e157286_d_n10, assign104900_e157286_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104900_e157283: f64 = (locals.var_xov + locals.var_tmf2);
        let assign104900_e157284: f64 = (0.5 * assign104900_e157283);
        (assign104900_e157284, (0.5 * (locals.var_xov_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_xov_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_xov_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_xov_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_xov_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_xov_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_xov_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_xov_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_xov_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_xov_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn13,)
    }
};
        locals.var_xov = assign104900_e157286;
        locals.var_xov_dn0 = assign104900_e157286_d_n0;
        locals.var_xov_dn2 = assign104900_e157286_d_n2;
        locals.var_xov_dn4 = assign104900_e157286_d_n4;
        locals.var_xov_dn5 = assign104900_e157286_d_n5;
        locals.var_xov_dn6 = assign104900_e157286_d_n6;
        locals.var_xov_dn7 = assign104900_e157286_d_n7;
        locals.var_xov_dn8 = assign104900_e157286_d_n8;
        locals.var_xov_dn9 = assign104900_e157286_d_n9;
        locals.var_xov_dn10 = assign104900_e157286_d_n10;
        locals.var_xov_dn13 = assign104900_e157286_d_n13;
        locals.var_xov_rv = 0.0;

        let assign104910_e157289: f64 = if locals.var_xov < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2380 = assign104910_e157289;
        locals.var_guard2380_rv = 0.0;

        let (assign104920_e157298, assign104920_e157298_d_n0, assign104920_e157298_d_n2, assign104920_e157298_d_n4, assign104920_e157298_d_n5, assign104920_e157298_d_n6, assign104920_e157298_d_n7, assign104920_e157298_d_n8, assign104920_e157298_d_n9, assign104920_e157298_d_n10, assign104920_e157298_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2380 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn13,)
    }
};
        locals.var_xov = assign104920_e157298;
        locals.var_xov_dn0 = assign104920_e157298_d_n0;
        locals.var_xov_dn2 = assign104920_e157298_d_n2;
        locals.var_xov_dn4 = assign104920_e157298_d_n4;
        locals.var_xov_dn5 = assign104920_e157298_d_n5;
        locals.var_xov_dn6 = assign104920_e157298_d_n6;
        locals.var_xov_dn7 = assign104920_e157298_d_n7;
        locals.var_xov_dn8 = assign104920_e157298_d_n8;
        locals.var_xov_dn9 = assign104920_e157298_d_n9;
        locals.var_xov_dn10 = assign104920_e157298_d_n10;
        locals.var_xov_dn13 = assign104920_e157298_d_n13;
        locals.var_xov_rv = 0.0;

        let (assign104930_e157307, assign104930_e157307_d_n0, assign104930_e157307_d_n2, assign104930_e157307_d_n4, assign104930_e157307_d_n5, assign104930_e157307_d_n6, assign104930_e157307_d_n7, assign104930_e157307_d_n8, assign104930_e157307_d_n9, assign104930_e157307_d_n10, assign104930_e157307_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2380 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104930_e157307;
        locals.var_t9_dn0 = assign104930_e157307_d_n0;
        locals.var_t9_dn2 = assign104930_e157307_d_n2;
        locals.var_t9_dn4 = assign104930_e157307_d_n4;
        locals.var_t9_dn5 = assign104930_e157307_d_n5;
        locals.var_t9_dn6 = assign104930_e157307_d_n6;
        locals.var_t9_dn7 = assign104930_e157307_d_n7;
        locals.var_t9_dn8 = assign104930_e157307_d_n8;
        locals.var_t9_dn9 = assign104930_e157307_d_n9;
        locals.var_t9_dn10 = assign104930_e157307_d_n10;
        locals.var_t9_dn13 = assign104930_e157307_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign104940_e157318, assign104940_e157318_d_n0, assign104940_e157318_d_n2, assign104940_e157318_d_n4, assign104940_e157318_d_n5, assign104940_e157318_d_n6, assign104940_e157318_d_n7, assign104940_e157318_d_n8, assign104940_e157318_d_n9, assign104940_e157318_d_n10, assign104940_e157318_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104940_e157315: f64 = (locals.var_ldrifte + p.p422);
        let assign104940_e157316: f64 = (1.6021918e-19 / assign104940_e157315);
        (assign104940_e157316, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign104940_e157318;
        locals.var_t1_dn0 = assign104940_e157318_d_n0;
        locals.var_t1_dn2 = assign104940_e157318_d_n2;
        locals.var_t1_dn4 = assign104940_e157318_d_n4;
        locals.var_t1_dn5 = assign104940_e157318_d_n5;
        locals.var_t1_dn6 = assign104940_e157318_d_n6;
        locals.var_t1_dn7 = assign104940_e157318_d_n7;
        locals.var_t1_dn8 = assign104940_e157318_d_n8;
        locals.var_t1_dn9 = assign104940_e157318_d_n9;
        locals.var_t1_dn10 = assign104940_e157318_d_n10;
        locals.var_t1_dn13 = assign104940_e157318_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign104950_e157331, assign104950_e157331_d_n0, assign104950_e157331_d_n2, assign104950_e157331_d_n4, assign104950_e157331_d_n5, assign104950_e157331_d_n6, assign104950_e157331_d_n7, assign104950_e157331_d_n8, assign104950_e157331_d_n9, assign104950_e157331_d_n10, assign104950_e157331_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104950_e157325: f64 = (locals.var_t1 * locals.var_xov);
        let assign104950_e157327: f64 = (assign104950_e157325 * locals.var_mu__blk2354);
        let assign104950_e157329: f64 = (assign104950_e157327 * locals.var_carr);
        (assign104950_e157329, ((((((locals.var_t1_dn0 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn0)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn0)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn0)), ((((((locals.var_t1_dn2 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn2)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn2)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn2)), ((((((locals.var_t1_dn4 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn4)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn4)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn4)), ((((((locals.var_t1_dn5 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn5)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn5)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn5)), ((((((locals.var_t1_dn6 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn6)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn6)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn6)), ((((((locals.var_t1_dn7 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn7)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn7)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn7)), ((((((locals.var_t1_dn8 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn8)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn8)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn8)), ((((((locals.var_t1_dn9 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn9)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn9)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn9)), ((((((locals.var_t1_dn10 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn10)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn10)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn10)), ((((((locals.var_t1_dn13 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn13)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn13)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn13)),)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn13,)
    }
};
        locals.var_gd = assign104950_e157331;
        locals.var_gd_dn0 = assign104950_e157331_d_n0;
        locals.var_gd_dn2 = assign104950_e157331_d_n2;
        locals.var_gd_dn4 = assign104950_e157331_d_n4;
        locals.var_gd_dn5 = assign104950_e157331_d_n5;
        locals.var_gd_dn6 = assign104950_e157331_d_n6;
        locals.var_gd_dn7 = assign104950_e157331_d_n7;
        locals.var_gd_dn8 = assign104950_e157331_d_n8;
        locals.var_gd_dn9 = assign104950_e157331_d_n9;
        locals.var_gd_dn10 = assign104950_e157331_d_n10;
        locals.var_gd_dn13 = assign104950_e157331_d_n13;
        locals.var_gd_rv = 0.0;

        let assign104960_e157335: f64 = 1e-25;
        let assign104960_e157340: f64 = if ((locals.var_gd < assign104960_e157335) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2381 = assign104960_e157340;
        locals.var_guard2381_rv = 0.0;

        let (assign104970_e157353, assign104970_e157353_d_n0, assign104970_e157353_d_n2, assign104970_e157353_d_n4, assign104970_e157353_d_n5, assign104970_e157353_d_n6, assign104970_e157353_d_n7, assign104970_e157353_d_n8, assign104970_e157353_d_n9, assign104970_e157353_d_n10, assign104970_e157353_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign104970_e157349: f64 = 1e-25;
        let assign104970_e157351: f64 = (assign104970_e157349 - locals.var_gd);
        (assign104970_e157351, (-locals.var_gd_dn0), (-locals.var_gd_dn2), (-locals.var_gd_dn4), (-locals.var_gd_dn5), (-locals.var_gd_dn6), (-locals.var_gd_dn7), (-locals.var_gd_dn8), (-locals.var_gd_dn9), (-locals.var_gd_dn10), (-locals.var_gd_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign104970_e157353;
        locals.var_tmf1_dn0 = assign104970_e157353_d_n0;
        locals.var_tmf1_dn2 = assign104970_e157353_d_n2;
        locals.var_tmf1_dn4 = assign104970_e157353_d_n4;
        locals.var_tmf1_dn5 = assign104970_e157353_d_n5;
        locals.var_tmf1_dn6 = assign104970_e157353_d_n6;
        locals.var_tmf1_dn7 = assign104970_e157353_d_n7;
        locals.var_tmf1_dn8 = assign104970_e157353_d_n8;
        locals.var_tmf1_dn9 = assign104970_e157353_d_n9;
        locals.var_tmf1_dn10 = assign104970_e157353_d_n10;
        locals.var_tmf1_dn13 = assign104970_e157353_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign104980_e157364, assign104980_e157364_d_n0, assign104980_e157364_d_n2, assign104980_e157364_d_n4, assign104980_e157364_d_n5, assign104980_e157364_d_n6, assign104980_e157364_d_n7, assign104980_e157364_d_n8, assign104980_e157364_d_n9, assign104980_e157364_d_n10, assign104980_e157364_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign104980_e157362: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign104980_e157362, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign104980_e157364;
        locals.var_x2_dn0 = assign104980_e157364_d_n0;
        locals.var_x2_dn2 = assign104980_e157364_d_n2;
        locals.var_x2_dn4 = assign104980_e157364_d_n4;
        locals.var_x2_dn5 = assign104980_e157364_d_n5;
        locals.var_x2_dn6 = assign104980_e157364_d_n6;
        locals.var_x2_dn7 = assign104980_e157364_d_n7;
        locals.var_x2_dn8 = assign104980_e157364_d_n8;
        locals.var_x2_dn9 = assign104980_e157364_d_n9;
        locals.var_x2_dn10 = assign104980_e157364_d_n10;
        locals.var_x2_dn13 = assign104980_e157364_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign104990_e157375, assign104990_e157375_d_n0, assign104990_e157375_d_n2, assign104990_e157375_d_n4, assign104990_e157375_d_n5, assign104990_e157375_d_n6, assign104990_e157375_d_n7, assign104990_e157375_d_n8, assign104990_e157375_d_n9, assign104990_e157375_d_n10, assign104990_e157375_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign104990_e157373: f64 = (1e-25 * 1e-25);
        (assign104990_e157373, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign104990_e157375;
        locals.var_xmax2_dn0 = assign104990_e157375_d_n0;
        locals.var_xmax2_dn2 = assign104990_e157375_d_n2;
        locals.var_xmax2_dn4 = assign104990_e157375_d_n4;
        locals.var_xmax2_dn5 = assign104990_e157375_d_n5;
        locals.var_xmax2_dn6 = assign104990_e157375_d_n6;
        locals.var_xmax2_dn7 = assign104990_e157375_d_n7;
        locals.var_xmax2_dn8 = assign104990_e157375_d_n8;
        locals.var_xmax2_dn9 = assign104990_e157375_d_n9;
        locals.var_xmax2_dn10 = assign104990_e157375_d_n10;
        locals.var_xmax2_dn13 = assign104990_e157375_d_n13;
        locals.var_xmax2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_389(
        locals: &mut StampLocals,
    ) {
        let (assign105000_e157384, assign105000_e157384_d_n0, assign105000_e157384_d_n2, assign105000_e157384_d_n4, assign105000_e157384_d_n5, assign105000_e157384_d_n6, assign105000_e157384_d_n7, assign105000_e157384_d_n8, assign105000_e157384_d_n9, assign105000_e157384_d_n10, assign105000_e157384_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105000_e157384;
        locals.var_xp_dn0 = assign105000_e157384_d_n0;
        locals.var_xp_dn2 = assign105000_e157384_d_n2;
        locals.var_xp_dn4 = assign105000_e157384_d_n4;
        locals.var_xp_dn5 = assign105000_e157384_d_n5;
        locals.var_xp_dn6 = assign105000_e157384_d_n6;
        locals.var_xp_dn7 = assign105000_e157384_d_n7;
        locals.var_xp_dn8 = assign105000_e157384_d_n8;
        locals.var_xp_dn9 = assign105000_e157384_d_n9;
        locals.var_xp_dn10 = assign105000_e157384_d_n10;
        locals.var_xp_dn13 = assign105000_e157384_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign105010_e157393, assign105010_e157393_d_n0, assign105010_e157393_d_n2, assign105010_e157393_d_n4, assign105010_e157393_d_n5, assign105010_e157393_d_n6, assign105010_e157393_d_n7, assign105010_e157393_d_n8, assign105010_e157393_d_n9, assign105010_e157393_d_n10, assign105010_e157393_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105010_e157393;
        locals.var_xmp_dn0 = assign105010_e157393_d_n0;
        locals.var_xmp_dn2 = assign105010_e157393_d_n2;
        locals.var_xmp_dn4 = assign105010_e157393_d_n4;
        locals.var_xmp_dn5 = assign105010_e157393_d_n5;
        locals.var_xmp_dn6 = assign105010_e157393_d_n6;
        locals.var_xmp_dn7 = assign105010_e157393_d_n7;
        locals.var_xmp_dn8 = assign105010_e157393_d_n8;
        locals.var_xmp_dn9 = assign105010_e157393_d_n9;
        locals.var_xmp_dn10 = assign105010_e157393_d_n10;
        locals.var_xmp_dn13 = assign105010_e157393_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign105020_e157402,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105020_e157402;
        locals.var_m0_rv = 0.0;

        let (assign105030_e157411,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105030_e157411;
        locals.var_mm_rv = 0.0;

        let (assign105040_e157420, assign105040_e157420_d_n0, assign105040_e157420_d_n2, assign105040_e157420_d_n4, assign105040_e157420_d_n5, assign105040_e157420_d_n6, assign105040_e157420_d_n7, assign105040_e157420_d_n8, assign105040_e157420_d_n9, assign105040_e157420_d_n10, assign105040_e157420_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign105040_e157420;
        locals.var_arg_dn0 = assign105040_e157420_d_n0;
        locals.var_arg_dn2 = assign105040_e157420_d_n2;
        locals.var_arg_dn4 = assign105040_e157420_d_n4;
        locals.var_arg_dn5 = assign105040_e157420_d_n5;
        locals.var_arg_dn6 = assign105040_e157420_d_n6;
        locals.var_arg_dn7 = assign105040_e157420_d_n7;
        locals.var_arg_dn8 = assign105040_e157420_d_n8;
        locals.var_arg_dn9 = assign105040_e157420_d_n9;
        locals.var_arg_dn10 = assign105040_e157420_d_n10;
        locals.var_arg_dn13 = assign105040_e157420_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign105050_e157429, assign105050_e157429_d_n0, assign105050_e157429_d_n2, assign105050_e157429_d_n4, assign105050_e157429_d_n5, assign105050_e157429_d_n6, assign105050_e157429_d_n7, assign105050_e157429_d_n8, assign105050_e157429_d_n9, assign105050_e157429_d_n10, assign105050_e157429_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105050_e157429;
        locals.var_dnm_dn0 = assign105050_e157429_d_n0;
        locals.var_dnm_dn2 = assign105050_e157429_d_n2;
        locals.var_dnm_dn4 = assign105050_e157429_d_n4;
        locals.var_dnm_dn5 = assign105050_e157429_d_n5;
        locals.var_dnm_dn6 = assign105050_e157429_d_n6;
        locals.var_dnm_dn7 = assign105050_e157429_d_n7;
        locals.var_dnm_dn8 = assign105050_e157429_d_n8;
        locals.var_dnm_dn9 = assign105050_e157429_d_n9;
        locals.var_dnm_dn10 = assign105050_e157429_d_n10;
        locals.var_dnm_dn13 = assign105050_e157429_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign105060_e157440, assign105060_e157440_d_n0, assign105060_e157440_d_n2, assign105060_e157440_d_n4, assign105060_e157440_d_n5, assign105060_e157440_d_n6, assign105060_e157440_d_n7, assign105060_e157440_d_n8, assign105060_e157440_d_n9, assign105060_e157440_d_n10, assign105060_e157440_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105060_e157438: f64 = (locals.var_xp * locals.var_x2);
        (assign105060_e157438, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105060_e157440;
        locals.var_xp_dn0 = assign105060_e157440_d_n0;
        locals.var_xp_dn2 = assign105060_e157440_d_n2;
        locals.var_xp_dn4 = assign105060_e157440_d_n4;
        locals.var_xp_dn5 = assign105060_e157440_d_n5;
        locals.var_xp_dn6 = assign105060_e157440_d_n6;
        locals.var_xp_dn7 = assign105060_e157440_d_n7;
        locals.var_xp_dn8 = assign105060_e157440_d_n8;
        locals.var_xp_dn9 = assign105060_e157440_d_n9;
        locals.var_xp_dn10 = assign105060_e157440_d_n10;
        locals.var_xp_dn13 = assign105060_e157440_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign105070_e157451, assign105070_e157451_d_n0, assign105070_e157451_d_n2, assign105070_e157451_d_n4, assign105070_e157451_d_n5, assign105070_e157451_d_n6, assign105070_e157451_d_n7, assign105070_e157451_d_n8, assign105070_e157451_d_n9, assign105070_e157451_d_n10, assign105070_e157451_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105070_e157449: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105070_e157449, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105070_e157451;
        locals.var_xmp_dn0 = assign105070_e157451_d_n0;
        locals.var_xmp_dn2 = assign105070_e157451_d_n2;
        locals.var_xmp_dn4 = assign105070_e157451_d_n4;
        locals.var_xmp_dn5 = assign105070_e157451_d_n5;
        locals.var_xmp_dn6 = assign105070_e157451_d_n6;
        locals.var_xmp_dn7 = assign105070_e157451_d_n7;
        locals.var_xmp_dn8 = assign105070_e157451_d_n8;
        locals.var_xmp_dn9 = assign105070_e157451_d_n9;
        locals.var_xmp_dn10 = assign105070_e157451_d_n10;
        locals.var_xmp_dn13 = assign105070_e157451_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign105080_e157462, assign105080_e157462_d_n0, assign105080_e157462_d_n2, assign105080_e157462_d_n4, assign105080_e157462_d_n5, assign105080_e157462_d_n6, assign105080_e157462_d_n7, assign105080_e157462_d_n8, assign105080_e157462_d_n9, assign105080_e157462_d_n10, assign105080_e157462_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105080_e157460: f64 = (locals.var_xp * locals.var_x2);
        (assign105080_e157460, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105080_e157462;
        locals.var_xp_dn0 = assign105080_e157462_d_n0;
        locals.var_xp_dn2 = assign105080_e157462_d_n2;
        locals.var_xp_dn4 = assign105080_e157462_d_n4;
        locals.var_xp_dn5 = assign105080_e157462_d_n5;
        locals.var_xp_dn6 = assign105080_e157462_d_n6;
        locals.var_xp_dn7 = assign105080_e157462_d_n7;
        locals.var_xp_dn8 = assign105080_e157462_d_n8;
        locals.var_xp_dn9 = assign105080_e157462_d_n9;
        locals.var_xp_dn10 = assign105080_e157462_d_n10;
        locals.var_xp_dn13 = assign105080_e157462_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign105090_e157473, assign105090_e157473_d_n0, assign105090_e157473_d_n2, assign105090_e157473_d_n4, assign105090_e157473_d_n5, assign105090_e157473_d_n6, assign105090_e157473_d_n7, assign105090_e157473_d_n8, assign105090_e157473_d_n9, assign105090_e157473_d_n10, assign105090_e157473_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105090_e157471: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105090_e157471, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105090_e157473;
        locals.var_xmp_dn0 = assign105090_e157473_d_n0;
        locals.var_xmp_dn2 = assign105090_e157473_d_n2;
        locals.var_xmp_dn4 = assign105090_e157473_d_n4;
        locals.var_xmp_dn5 = assign105090_e157473_d_n5;
        locals.var_xmp_dn6 = assign105090_e157473_d_n6;
        locals.var_xmp_dn7 = assign105090_e157473_d_n7;
        locals.var_xmp_dn8 = assign105090_e157473_d_n8;
        locals.var_xmp_dn9 = assign105090_e157473_d_n9;
        locals.var_xmp_dn10 = assign105090_e157473_d_n10;
        locals.var_xmp_dn13 = assign105090_e157473_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign105100_e157484, assign105100_e157484_d_n0, assign105100_e157484_d_n2, assign105100_e157484_d_n4, assign105100_e157484_d_n5, assign105100_e157484_d_n6, assign105100_e157484_d_n7, assign105100_e157484_d_n8, assign105100_e157484_d_n9, assign105100_e157484_d_n10, assign105100_e157484_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105100_e157482: f64 = (locals.var_xp + locals.var_xmp);
        (assign105100_e157482, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign105100_e157484;
        locals.var_arg_dn0 = assign105100_e157484_d_n0;
        locals.var_arg_dn2 = assign105100_e157484_d_n2;
        locals.var_arg_dn4 = assign105100_e157484_d_n4;
        locals.var_arg_dn5 = assign105100_e157484_d_n5;
        locals.var_arg_dn6 = assign105100_e157484_d_n6;
        locals.var_arg_dn7 = assign105100_e157484_d_n7;
        locals.var_arg_dn8 = assign105100_e157484_d_n8;
        locals.var_arg_dn9 = assign105100_e157484_d_n9;
        locals.var_arg_dn10 = assign105100_e157484_d_n10;
        locals.var_arg_dn13 = assign105100_e157484_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign105110_e157493, assign105110_e157493_d_n0, assign105110_e157493_d_n2, assign105110_e157493_d_n4, assign105110_e157493_d_n5, assign105110_e157493_d_n6, assign105110_e157493_d_n7, assign105110_e157493_d_n8, assign105110_e157493_d_n9, assign105110_e157493_d_n10, assign105110_e157493_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105110_e157493;
        locals.var_dnm_dn0 = assign105110_e157493_d_n0;
        locals.var_dnm_dn2 = assign105110_e157493_d_n2;
        locals.var_dnm_dn4 = assign105110_e157493_d_n4;
        locals.var_dnm_dn5 = assign105110_e157493_d_n5;
        locals.var_dnm_dn6 = assign105110_e157493_d_n6;
        locals.var_dnm_dn7 = assign105110_e157493_d_n7;
        locals.var_dnm_dn8 = assign105110_e157493_d_n8;
        locals.var_dnm_dn9 = assign105110_e157493_d_n9;
        locals.var_dnm_dn10 = assign105110_e157493_d_n10;
        locals.var_dnm_dn13 = assign105110_e157493_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign105120_e157508: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2382 = assign105120_e157508;
        locals.var_guard2382_rv = 0.0;

        let assign105130_e157511: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2383 = assign105130_e157511;
        locals.var_guard2383_rv = 0.0;

        let (assign105140_e157524,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105140_e157524;
        locals.var_mm_rv = 0.0;

        let assign105150_e157527: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2384 = assign105150_e157527;
        locals.var_guard2384_rv = 0.0;

        let (assign105160_e157543,) = {
    if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 == 0.0)) && (locals.var_guard2384 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105160_e157543;
        locals.var_mm_rv = 0.0;

        let assign105170_e157546: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2385 = assign105170_e157546;
        locals.var_guard2385_rv = 0.0;

        let (assign105180_e157565,) = {
    if (((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 == 0.0)) && (locals.var_guard2384 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105180_e157565;
        locals.var_mm_rv = 0.0;

        let assign105190_e157568: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2386 = assign105190_e157568;
        locals.var_guard2386_rv = 0.0;

        let (assign105200_e157590,) = {
    if ((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 == 0.0)) && (locals.var_guard2384 == 0.0)) && (locals.var_guard2385 == 0.0)) && (locals.var_guard2386 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105200_e157590;
        locals.var_mm_rv = 0.0;

        let (assign105210_e157601,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105210_e157601;
        locals.var_m0_rv = 0.0;

        let mut assign105220_loop_guard: usize = 0;
        while {
            let assign105220_cond_e157613: f64 = if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105220_cond_e157613 != 0.0
        } {
            assign105220_loop_guard += 1;
            assert!(assign105220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105220_body0_e157625, assign105220_body0_e157625_d_n0, assign105220_body0_e157625_d_n2, assign105220_body0_e157625_d_n4, assign105220_body0_e157625_d_n5, assign105220_body0_e157625_d_n6, assign105220_body0_e157625_d_n7, assign105220_body0_e157625_d_n8, assign105220_body0_e157625_d_n9, assign105220_body0_e157625_d_n10, assign105220_body0_e157625_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) {
        let assign105220_body0_e157623: f64 = (locals.var_dnm).sqrt();
        (assign105220_body0_e157623, (locals.var_dnm_dn0 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn2 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn4 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn5 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn6 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn7 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn8 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn9 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn10 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn13 / (2.0 * assign105220_body0_e157623)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign105220_body0_e157625;
            locals.var_dnm_dn0 = assign105220_body0_e157625_d_n0;
            locals.var_dnm_dn2 = assign105220_body0_e157625_d_n2;
            locals.var_dnm_dn4 = assign105220_body0_e157625_d_n4;
            locals.var_dnm_dn5 = assign105220_body0_e157625_d_n5;
            locals.var_dnm_dn6 = assign105220_body0_e157625_d_n6;
            locals.var_dnm_dn7 = assign105220_body0_e157625_d_n7;
            locals.var_dnm_dn8 = assign105220_body0_e157625_d_n8;
            locals.var_dnm_dn9 = assign105220_body0_e157625_d_n9;
            locals.var_dnm_dn10 = assign105220_body0_e157625_d_n10;
            locals.var_dnm_dn13 = assign105220_body0_e157625_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign105220_body1_e157638,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) {
        let assign105220_body1_e157636: f64 = (locals.var_m0 + 1.0);
        (assign105220_body1_e157636,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign105220_body1_e157638;
            locals.var_m0_rv = 0.0;
        }

        let (assign105230_e157661, assign105230_e157661_d_n0, assign105230_e157661_d_n2, assign105230_e157661_d_n4, assign105230_e157661_d_n5, assign105230_e157661_d_n6, assign105230_e157661_d_n7, assign105230_e157661_d_n8, assign105230_e157661_d_n9, assign105230_e157661_d_n10, assign105230_e157661_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 == 0.0)) {
        let (assign105230_e157659, assign105230_e157659_d_n0, assign105230_e157659_d_n2, assign105230_e157659_d_n4, assign105230_e157659_d_n5, assign105230_e157659_d_n6, assign105230_e157659_d_n7, assign105230_e157659_d_n8, assign105230_e157659_d_n9, assign105230_e157659_d_n10, assign105230_e157659_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105230_e157656: f64 = (2.0 * 2.0);
                let assign105230_e157657: f64 = (1.0 / assign105230_e157656);
                let assign105230_e157658: f64 = (locals.var_dnm).powf(assign105230_e157657);
                (assign105230_e157658, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn13)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign105230_e157659, assign105230_e157659_d_n0, assign105230_e157659_d_n2, assign105230_e157659_d_n4, assign105230_e157659_d_n5, assign105230_e157659_d_n6, assign105230_e157659_d_n7, assign105230_e157659_d_n8, assign105230_e157659_d_n9, assign105230_e157659_d_n10, assign105230_e157659_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105230_e157661;
        locals.var_dnm_dn0 = assign105230_e157661_d_n0;
        locals.var_dnm_dn2 = assign105230_e157661_d_n2;
        locals.var_dnm_dn4 = assign105230_e157661_d_n4;
        locals.var_dnm_dn5 = assign105230_e157661_d_n5;
        locals.var_dnm_dn6 = assign105230_e157661_d_n6;
        locals.var_dnm_dn7 = assign105230_e157661_d_n7;
        locals.var_dnm_dn8 = assign105230_e157661_d_n8;
        locals.var_dnm_dn9 = assign105230_e157661_d_n9;
        locals.var_dnm_dn10 = assign105230_e157661_d_n10;
        locals.var_dnm_dn13 = assign105230_e157661_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign105240_e157672, assign105240_e157672_d_n0, assign105240_e157672_d_n2, assign105240_e157672_d_n4, assign105240_e157672_d_n5, assign105240_e157672_d_n6, assign105240_e157672_d_n7, assign105240_e157672_d_n8, assign105240_e157672_d_n9, assign105240_e157672_d_n10, assign105240_e157672_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105240_e157670: f64 = (1.0 / locals.var_dnm);
        (assign105240_e157670, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105240_e157672;
        locals.var_dnm_dn0 = assign105240_e157672_d_n0;
        locals.var_dnm_dn2 = assign105240_e157672_d_n2;
        locals.var_dnm_dn4 = assign105240_e157672_d_n4;
        locals.var_dnm_dn5 = assign105240_e157672_d_n5;
        locals.var_dnm_dn6 = assign105240_e157672_d_n6;
        locals.var_dnm_dn7 = assign105240_e157672_d_n7;
        locals.var_dnm_dn8 = assign105240_e157672_d_n8;
        locals.var_dnm_dn9 = assign105240_e157672_d_n9;
        locals.var_dnm_dn10 = assign105240_e157672_d_n10;
        locals.var_dnm_dn13 = assign105240_e157672_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign105250_e157685, assign105250_e157685_d_n0, assign105250_e157685_d_n2, assign105250_e157685_d_n4, assign105250_e157685_d_n5, assign105250_e157685_d_n6, assign105250_e157685_d_n7, assign105250_e157685_d_n8, assign105250_e157685_d_n9, assign105250_e157685_d_n10, assign105250_e157685_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105250_e157681: f64 = (locals.var_tmf1 * 1e-25);
        let assign105250_e157683: f64 = (assign105250_e157681 * locals.var_dnm);
        (assign105250_e157683, (((locals.var_tmf1_dn0 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign105250_e157685;
        locals.var_tmf0_dn0 = assign105250_e157685_d_n0;
        locals.var_tmf0_dn2 = assign105250_e157685_d_n2;
        locals.var_tmf0_dn4 = assign105250_e157685_d_n4;
        locals.var_tmf0_dn5 = assign105250_e157685_d_n5;
        locals.var_tmf0_dn6 = assign105250_e157685_d_n6;
        locals.var_tmf0_dn7 = assign105250_e157685_d_n7;
        locals.var_tmf0_dn8 = assign105250_e157685_d_n8;
        locals.var_tmf0_dn9 = assign105250_e157685_d_n9;
        locals.var_tmf0_dn10 = assign105250_e157685_d_n10;
        locals.var_tmf0_dn13 = assign105250_e157685_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign105260_e157700, assign105260_e157700_d_n0, assign105260_e157700_d_n2, assign105260_e157700_d_n4, assign105260_e157700_d_n5, assign105260_e157700_d_n6, assign105260_e157700_d_n7, assign105260_e157700_d_n8, assign105260_e157700_d_n9, assign105260_e157700_d_n10, assign105260_e157700_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105260_e157694: f64 = (1e-25 * locals.var_xmp);
        let assign105260_e157696: f64 = (assign105260_e157694 * locals.var_dnm);
        let assign105260_e157698: f64 = (assign105260_e157696 / locals.var_arg);
        (assign105260_e157698, ((((((1e-25 * locals.var_xmp_dn0) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn0)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn2) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn2)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn4) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn4)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn5) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn5)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn6) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn6)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn7) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn7)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn8) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn8)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn9) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn9)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn10) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn10)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn13) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn13)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign105260_e157700;
        locals.var_t0_dn0 = assign105260_e157700_d_n0;
        locals.var_t0_dn2 = assign105260_e157700_d_n2;
        locals.var_t0_dn4 = assign105260_e157700_d_n4;
        locals.var_t0_dn5 = assign105260_e157700_d_n5;
        locals.var_t0_dn6 = assign105260_e157700_d_n6;
        locals.var_t0_dn7 = assign105260_e157700_d_n7;
        locals.var_t0_dn8 = assign105260_e157700_d_n8;
        locals.var_t0_dn9 = assign105260_e157700_d_n9;
        locals.var_t0_dn10 = assign105260_e157700_d_n10;
        locals.var_t0_dn13 = assign105260_e157700_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign105270_e157713, assign105270_e157713_d_n0, assign105270_e157713_d_n2, assign105270_e157713_d_n4, assign105270_e157713_d_n5, assign105270_e157713_d_n6, assign105270_e157713_d_n7, assign105270_e157713_d_n8, assign105270_e157713_d_n9, assign105270_e157713_d_n10, assign105270_e157713_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105270_e157709: f64 = 1e-25;
        let assign105270_e157711: f64 = (assign105270_e157709 - locals.var_tmf0);
        (assign105270_e157711, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn13,)
    }
};
        locals.var_gd = assign105270_e157713;
        locals.var_gd_dn0 = assign105270_e157713_d_n0;
        locals.var_gd_dn2 = assign105270_e157713_d_n2;
        locals.var_gd_dn4 = assign105270_e157713_d_n4;
        locals.var_gd_dn5 = assign105270_e157713_d_n5;
        locals.var_gd_dn6 = assign105270_e157713_d_n6;
        locals.var_gd_dn7 = assign105270_e157713_d_n7;
        locals.var_gd_dn8 = assign105270_e157713_d_n8;
        locals.var_gd_dn9 = assign105270_e157713_d_n9;
        locals.var_gd_dn10 = assign105270_e157713_d_n10;
        locals.var_gd_dn13 = assign105270_e157713_d_n13;
        locals.var_gd_rv = 0.0;

        let (assign105280_e157722, assign105280_e157722_d_n0, assign105280_e157722_d_n2, assign105280_e157722_d_n4, assign105280_e157722_d_n5, assign105280_e157722_d_n6, assign105280_e157722_d_n7, assign105280_e157722_d_n8, assign105280_e157722_d_n9, assign105280_e157722_d_n10, assign105280_e157722_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign105280_e157722;
        locals.var_t0_dn0 = assign105280_e157722_d_n0;
        locals.var_t0_dn2 = assign105280_e157722_d_n2;
        locals.var_t0_dn4 = assign105280_e157722_d_n4;
        locals.var_t0_dn5 = assign105280_e157722_d_n5;
        locals.var_t0_dn6 = assign105280_e157722_d_n6;
        locals.var_t0_dn7 = assign105280_e157722_d_n7;
        locals.var_t0_dn8 = assign105280_e157722_d_n8;
        locals.var_t0_dn9 = assign105280_e157722_d_n9;
        locals.var_t0_dn10 = assign105280_e157722_d_n10;
        locals.var_t0_dn13 = assign105280_e157722_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign105290_e157732, assign105290_e157732_d_n0, assign105290_e157732_d_n2, assign105290_e157732_d_n4, assign105290_e157732_d_n5, assign105290_e157732_d_n6, assign105290_e157732_d_n7, assign105290_e157732_d_n8, assign105290_e157732_d_n9, assign105290_e157732_d_n10, assign105290_e157732_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 == 0.0)) {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn13,)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn13,)
    }
};
        locals.var_gd = assign105290_e157732;
        locals.var_gd_dn0 = assign105290_e157732_d_n0;
        locals.var_gd_dn2 = assign105290_e157732_d_n2;
        locals.var_gd_dn4 = assign105290_e157732_d_n4;
        locals.var_gd_dn5 = assign105290_e157732_d_n5;
        locals.var_gd_dn6 = assign105290_e157732_d_n6;
        locals.var_gd_dn7 = assign105290_e157732_d_n7;
        locals.var_gd_dn8 = assign105290_e157732_d_n8;
        locals.var_gd_dn9 = assign105290_e157732_d_n9;
        locals.var_gd_dn10 = assign105290_e157732_d_n10;
        locals.var_gd_dn13 = assign105290_e157732_d_n13;
        locals.var_gd_rv = 0.0;

        let (assign105300_e157742, assign105300_e157742_d_n0, assign105300_e157742_d_n2, assign105300_e157742_d_n4, assign105300_e157742_d_n5, assign105300_e157742_d_n6, assign105300_e157742_d_n7, assign105300_e157742_d_n8, assign105300_e157742_d_n9, assign105300_e157742_d_n10, assign105300_e157742_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign105300_e157742;
        locals.var_t0_dn0 = assign105300_e157742_d_n0;
        locals.var_t0_dn2 = assign105300_e157742_d_n2;
        locals.var_t0_dn4 = assign105300_e157742_d_n4;
        locals.var_t0_dn5 = assign105300_e157742_d_n5;
        locals.var_t0_dn6 = assign105300_e157742_d_n6;
        locals.var_t0_dn7 = assign105300_e157742_d_n7;
        locals.var_t0_dn8 = assign105300_e157742_d_n8;
        locals.var_t0_dn9 = assign105300_e157742_d_n9;
        locals.var_t0_dn10 = assign105300_e157742_d_n10;
        locals.var_t0_dn13 = assign105300_e157742_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_390(
        locals: &mut StampLocals,
    ) {
        let (assign105310_e157751, assign105310_e157751_d_n0, assign105310_e157751_d_n2, assign105310_e157751_d_n4, assign105310_e157751_d_n5, assign105310_e157751_d_n6, assign105310_e157751_d_n7, assign105310_e157751_d_n8, assign105310_e157751_d_n9, assign105310_e157751_d_n10, assign105310_e157751_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign105310_e157749: f64 = (1.0 / locals.var_gd);
        (assign105310_e157749, (-(locals.var_gd_dn0 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn2 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn4 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn5 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn6 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn7 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn8 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn9 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn10 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn13 / (locals.var_gd * locals.var_gd))),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    }
};
        locals.var_rdd = assign105310_e157751;
        locals.var_rdd_dn0 = assign105310_e157751_d_n0;
        locals.var_rdd_dn2 = assign105310_e157751_d_n2;
        locals.var_rdd_dn4 = assign105310_e157751_d_n4;
        locals.var_rdd_dn5 = assign105310_e157751_d_n5;
        locals.var_rdd_dn6 = assign105310_e157751_d_n6;
        locals.var_rdd_dn7 = assign105310_e157751_d_n7;
        locals.var_rdd_dn8 = assign105310_e157751_d_n8;
        locals.var_rdd_dn9 = assign105310_e157751_d_n9;
        locals.var_rdd_dn10 = assign105310_e157751_d_n10;
        locals.var_rdd_dn13 = assign105310_e157751_d_n13;
        locals.var_rdd_rv = 0.0;

        let (assign105320_e157760, assign105320_e157760_d_n0, assign105320_e157760_d_n2, assign105320_e157760_d_n4, assign105320_e157760_d_n5, assign105320_e157760_d_n6, assign105320_e157760_d_n7, assign105320_e157760_d_n8, assign105320_e157760_d_n9, assign105320_e157760_d_n10, assign105320_e157760_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign105320_e157758: f64 = (locals.var_rdd / locals.var_weffld_nf);
        (assign105320_e157758, (locals.var_rdd_dn0 / locals.var_weffld_nf), (locals.var_rdd_dn2 / locals.var_weffld_nf), (locals.var_rdd_dn4 / locals.var_weffld_nf), (locals.var_rdd_dn5 / locals.var_weffld_nf), (locals.var_rdd_dn6 / locals.var_weffld_nf), (locals.var_rdd_dn7 / locals.var_weffld_nf), (locals.var_rdd_dn8 / locals.var_weffld_nf), (locals.var_rdd_dn9 / locals.var_weffld_nf), (locals.var_rdd_dn10 / locals.var_weffld_nf), (locals.var_rdd_dn13 / locals.var_weffld_nf),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    }
};
        locals.var_rdd = assign105320_e157760;
        locals.var_rdd_dn0 = assign105320_e157760_d_n0;
        locals.var_rdd_dn2 = assign105320_e157760_d_n2;
        locals.var_rdd_dn4 = assign105320_e157760_d_n4;
        locals.var_rdd_dn5 = assign105320_e157760_d_n5;
        locals.var_rdd_dn6 = assign105320_e157760_d_n6;
        locals.var_rdd_dn7 = assign105320_e157760_d_n7;
        locals.var_rdd_dn8 = assign105320_e157760_d_n8;
        locals.var_rdd_dn9 = assign105320_e157760_d_n9;
        locals.var_rdd_dn10 = assign105320_e157760_d_n10;
        locals.var_rdd_dn13 = assign105320_e157760_d_n13;
        locals.var_rdd_rv = 0.0;

        let assign105330_e157764: f64 = (1000000.0 - 1000.0);
        let assign105330_e157769: f64 = if ((locals.var_rdd > assign105330_e157764) && (1000.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2387 = assign105330_e157769;
        locals.var_guard2387_rv = 0.0;

        let (assign105340_e157782, assign105340_e157782_d_n0, assign105340_e157782_d_n2, assign105340_e157782_d_n4, assign105340_e157782_d_n5, assign105340_e157782_d_n6, assign105340_e157782_d_n7, assign105340_e157782_d_n8, assign105340_e157782_d_n9, assign105340_e157782_d_n10, assign105340_e157782_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105340_e157778: f64 = (locals.var_rdd - 1000000.0);
        let assign105340_e157780: f64 = (assign105340_e157778 + 1000.0);
        (assign105340_e157780, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign105340_e157782;
        locals.var_tmf1_dn0 = assign105340_e157782_d_n0;
        locals.var_tmf1_dn2 = assign105340_e157782_d_n2;
        locals.var_tmf1_dn4 = assign105340_e157782_d_n4;
        locals.var_tmf1_dn5 = assign105340_e157782_d_n5;
        locals.var_tmf1_dn6 = assign105340_e157782_d_n6;
        locals.var_tmf1_dn7 = assign105340_e157782_d_n7;
        locals.var_tmf1_dn8 = assign105340_e157782_d_n8;
        locals.var_tmf1_dn9 = assign105340_e157782_d_n9;
        locals.var_tmf1_dn10 = assign105340_e157782_d_n10;
        locals.var_tmf1_dn13 = assign105340_e157782_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign105350_e157793, assign105350_e157793_d_n0, assign105350_e157793_d_n2, assign105350_e157793_d_n4, assign105350_e157793_d_n5, assign105350_e157793_d_n6, assign105350_e157793_d_n7, assign105350_e157793_d_n8, assign105350_e157793_d_n9, assign105350_e157793_d_n10, assign105350_e157793_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105350_e157791: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign105350_e157791, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign105350_e157793;
        locals.var_x2_dn0 = assign105350_e157793_d_n0;
        locals.var_x2_dn2 = assign105350_e157793_d_n2;
        locals.var_x2_dn4 = assign105350_e157793_d_n4;
        locals.var_x2_dn5 = assign105350_e157793_d_n5;
        locals.var_x2_dn6 = assign105350_e157793_d_n6;
        locals.var_x2_dn7 = assign105350_e157793_d_n7;
        locals.var_x2_dn8 = assign105350_e157793_d_n8;
        locals.var_x2_dn9 = assign105350_e157793_d_n9;
        locals.var_x2_dn10 = assign105350_e157793_d_n10;
        locals.var_x2_dn13 = assign105350_e157793_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign105360_e157804, assign105360_e157804_d_n0, assign105360_e157804_d_n2, assign105360_e157804_d_n4, assign105360_e157804_d_n5, assign105360_e157804_d_n6, assign105360_e157804_d_n7, assign105360_e157804_d_n8, assign105360_e157804_d_n9, assign105360_e157804_d_n10, assign105360_e157804_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105360_e157802: f64 = (1000.0 * 1000.0);
        (assign105360_e157802, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign105360_e157804;
        locals.var_xmax2_dn0 = assign105360_e157804_d_n0;
        locals.var_xmax2_dn2 = assign105360_e157804_d_n2;
        locals.var_xmax2_dn4 = assign105360_e157804_d_n4;
        locals.var_xmax2_dn5 = assign105360_e157804_d_n5;
        locals.var_xmax2_dn6 = assign105360_e157804_d_n6;
        locals.var_xmax2_dn7 = assign105360_e157804_d_n7;
        locals.var_xmax2_dn8 = assign105360_e157804_d_n8;
        locals.var_xmax2_dn9 = assign105360_e157804_d_n9;
        locals.var_xmax2_dn10 = assign105360_e157804_d_n10;
        locals.var_xmax2_dn13 = assign105360_e157804_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign105370_e157813, assign105370_e157813_d_n0, assign105370_e157813_d_n2, assign105370_e157813_d_n4, assign105370_e157813_d_n5, assign105370_e157813_d_n6, assign105370_e157813_d_n7, assign105370_e157813_d_n8, assign105370_e157813_d_n9, assign105370_e157813_d_n10, assign105370_e157813_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105370_e157813;
        locals.var_xp_dn0 = assign105370_e157813_d_n0;
        locals.var_xp_dn2 = assign105370_e157813_d_n2;
        locals.var_xp_dn4 = assign105370_e157813_d_n4;
        locals.var_xp_dn5 = assign105370_e157813_d_n5;
        locals.var_xp_dn6 = assign105370_e157813_d_n6;
        locals.var_xp_dn7 = assign105370_e157813_d_n7;
        locals.var_xp_dn8 = assign105370_e157813_d_n8;
        locals.var_xp_dn9 = assign105370_e157813_d_n9;
        locals.var_xp_dn10 = assign105370_e157813_d_n10;
        locals.var_xp_dn13 = assign105370_e157813_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign105380_e157822, assign105380_e157822_d_n0, assign105380_e157822_d_n2, assign105380_e157822_d_n4, assign105380_e157822_d_n5, assign105380_e157822_d_n6, assign105380_e157822_d_n7, assign105380_e157822_d_n8, assign105380_e157822_d_n9, assign105380_e157822_d_n10, assign105380_e157822_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105380_e157822;
        locals.var_xmp_dn0 = assign105380_e157822_d_n0;
        locals.var_xmp_dn2 = assign105380_e157822_d_n2;
        locals.var_xmp_dn4 = assign105380_e157822_d_n4;
        locals.var_xmp_dn5 = assign105380_e157822_d_n5;
        locals.var_xmp_dn6 = assign105380_e157822_d_n6;
        locals.var_xmp_dn7 = assign105380_e157822_d_n7;
        locals.var_xmp_dn8 = assign105380_e157822_d_n8;
        locals.var_xmp_dn9 = assign105380_e157822_d_n9;
        locals.var_xmp_dn10 = assign105380_e157822_d_n10;
        locals.var_xmp_dn13 = assign105380_e157822_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign105390_e157831,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105390_e157831;
        locals.var_m0_rv = 0.0;

        let (assign105400_e157840,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105400_e157840;
        locals.var_mm_rv = 0.0;

        let (assign105410_e157849, assign105410_e157849_d_n0, assign105410_e157849_d_n2, assign105410_e157849_d_n4, assign105410_e157849_d_n5, assign105410_e157849_d_n6, assign105410_e157849_d_n7, assign105410_e157849_d_n8, assign105410_e157849_d_n9, assign105410_e157849_d_n10, assign105410_e157849_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign105410_e157849;
        locals.var_arg_dn0 = assign105410_e157849_d_n0;
        locals.var_arg_dn2 = assign105410_e157849_d_n2;
        locals.var_arg_dn4 = assign105410_e157849_d_n4;
        locals.var_arg_dn5 = assign105410_e157849_d_n5;
        locals.var_arg_dn6 = assign105410_e157849_d_n6;
        locals.var_arg_dn7 = assign105410_e157849_d_n7;
        locals.var_arg_dn8 = assign105410_e157849_d_n8;
        locals.var_arg_dn9 = assign105410_e157849_d_n9;
        locals.var_arg_dn10 = assign105410_e157849_d_n10;
        locals.var_arg_dn13 = assign105410_e157849_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign105420_e157858, assign105420_e157858_d_n0, assign105420_e157858_d_n2, assign105420_e157858_d_n4, assign105420_e157858_d_n5, assign105420_e157858_d_n6, assign105420_e157858_d_n7, assign105420_e157858_d_n8, assign105420_e157858_d_n9, assign105420_e157858_d_n10, assign105420_e157858_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105420_e157858;
        locals.var_dnm_dn0 = assign105420_e157858_d_n0;
        locals.var_dnm_dn2 = assign105420_e157858_d_n2;
        locals.var_dnm_dn4 = assign105420_e157858_d_n4;
        locals.var_dnm_dn5 = assign105420_e157858_d_n5;
        locals.var_dnm_dn6 = assign105420_e157858_d_n6;
        locals.var_dnm_dn7 = assign105420_e157858_d_n7;
        locals.var_dnm_dn8 = assign105420_e157858_d_n8;
        locals.var_dnm_dn9 = assign105420_e157858_d_n9;
        locals.var_dnm_dn10 = assign105420_e157858_d_n10;
        locals.var_dnm_dn13 = assign105420_e157858_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign105430_e157869, assign105430_e157869_d_n0, assign105430_e157869_d_n2, assign105430_e157869_d_n4, assign105430_e157869_d_n5, assign105430_e157869_d_n6, assign105430_e157869_d_n7, assign105430_e157869_d_n8, assign105430_e157869_d_n9, assign105430_e157869_d_n10, assign105430_e157869_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105430_e157867: f64 = (locals.var_xp * locals.var_x2);
        (assign105430_e157867, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105430_e157869;
        locals.var_xp_dn0 = assign105430_e157869_d_n0;
        locals.var_xp_dn2 = assign105430_e157869_d_n2;
        locals.var_xp_dn4 = assign105430_e157869_d_n4;
        locals.var_xp_dn5 = assign105430_e157869_d_n5;
        locals.var_xp_dn6 = assign105430_e157869_d_n6;
        locals.var_xp_dn7 = assign105430_e157869_d_n7;
        locals.var_xp_dn8 = assign105430_e157869_d_n8;
        locals.var_xp_dn9 = assign105430_e157869_d_n9;
        locals.var_xp_dn10 = assign105430_e157869_d_n10;
        locals.var_xp_dn13 = assign105430_e157869_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign105440_e157880, assign105440_e157880_d_n0, assign105440_e157880_d_n2, assign105440_e157880_d_n4, assign105440_e157880_d_n5, assign105440_e157880_d_n6, assign105440_e157880_d_n7, assign105440_e157880_d_n8, assign105440_e157880_d_n9, assign105440_e157880_d_n10, assign105440_e157880_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105440_e157878: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105440_e157878, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105440_e157880;
        locals.var_xmp_dn0 = assign105440_e157880_d_n0;
        locals.var_xmp_dn2 = assign105440_e157880_d_n2;
        locals.var_xmp_dn4 = assign105440_e157880_d_n4;
        locals.var_xmp_dn5 = assign105440_e157880_d_n5;
        locals.var_xmp_dn6 = assign105440_e157880_d_n6;
        locals.var_xmp_dn7 = assign105440_e157880_d_n7;
        locals.var_xmp_dn8 = assign105440_e157880_d_n8;
        locals.var_xmp_dn9 = assign105440_e157880_d_n9;
        locals.var_xmp_dn10 = assign105440_e157880_d_n10;
        locals.var_xmp_dn13 = assign105440_e157880_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign105450_e157891, assign105450_e157891_d_n0, assign105450_e157891_d_n2, assign105450_e157891_d_n4, assign105450_e157891_d_n5, assign105450_e157891_d_n6, assign105450_e157891_d_n7, assign105450_e157891_d_n8, assign105450_e157891_d_n9, assign105450_e157891_d_n10, assign105450_e157891_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105450_e157889: f64 = (locals.var_xp * locals.var_x2);
        (assign105450_e157889, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105450_e157891;
        locals.var_xp_dn0 = assign105450_e157891_d_n0;
        locals.var_xp_dn2 = assign105450_e157891_d_n2;
        locals.var_xp_dn4 = assign105450_e157891_d_n4;
        locals.var_xp_dn5 = assign105450_e157891_d_n5;
        locals.var_xp_dn6 = assign105450_e157891_d_n6;
        locals.var_xp_dn7 = assign105450_e157891_d_n7;
        locals.var_xp_dn8 = assign105450_e157891_d_n8;
        locals.var_xp_dn9 = assign105450_e157891_d_n9;
        locals.var_xp_dn10 = assign105450_e157891_d_n10;
        locals.var_xp_dn13 = assign105450_e157891_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign105460_e157902, assign105460_e157902_d_n0, assign105460_e157902_d_n2, assign105460_e157902_d_n4, assign105460_e157902_d_n5, assign105460_e157902_d_n6, assign105460_e157902_d_n7, assign105460_e157902_d_n8, assign105460_e157902_d_n9, assign105460_e157902_d_n10, assign105460_e157902_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105460_e157900: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105460_e157900, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105460_e157902;
        locals.var_xmp_dn0 = assign105460_e157902_d_n0;
        locals.var_xmp_dn2 = assign105460_e157902_d_n2;
        locals.var_xmp_dn4 = assign105460_e157902_d_n4;
        locals.var_xmp_dn5 = assign105460_e157902_d_n5;
        locals.var_xmp_dn6 = assign105460_e157902_d_n6;
        locals.var_xmp_dn7 = assign105460_e157902_d_n7;
        locals.var_xmp_dn8 = assign105460_e157902_d_n8;
        locals.var_xmp_dn9 = assign105460_e157902_d_n9;
        locals.var_xmp_dn10 = assign105460_e157902_d_n10;
        locals.var_xmp_dn13 = assign105460_e157902_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign105470_e157913, assign105470_e157913_d_n0, assign105470_e157913_d_n2, assign105470_e157913_d_n4, assign105470_e157913_d_n5, assign105470_e157913_d_n6, assign105470_e157913_d_n7, assign105470_e157913_d_n8, assign105470_e157913_d_n9, assign105470_e157913_d_n10, assign105470_e157913_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105470_e157911: f64 = (locals.var_xp + locals.var_xmp);
        (assign105470_e157911, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign105470_e157913;
        locals.var_arg_dn0 = assign105470_e157913_d_n0;
        locals.var_arg_dn2 = assign105470_e157913_d_n2;
        locals.var_arg_dn4 = assign105470_e157913_d_n4;
        locals.var_arg_dn5 = assign105470_e157913_d_n5;
        locals.var_arg_dn6 = assign105470_e157913_d_n6;
        locals.var_arg_dn7 = assign105470_e157913_d_n7;
        locals.var_arg_dn8 = assign105470_e157913_d_n8;
        locals.var_arg_dn9 = assign105470_e157913_d_n9;
        locals.var_arg_dn10 = assign105470_e157913_d_n10;
        locals.var_arg_dn13 = assign105470_e157913_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign105480_e157922, assign105480_e157922_d_n0, assign105480_e157922_d_n2, assign105480_e157922_d_n4, assign105480_e157922_d_n5, assign105480_e157922_d_n6, assign105480_e157922_d_n7, assign105480_e157922_d_n8, assign105480_e157922_d_n9, assign105480_e157922_d_n10, assign105480_e157922_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105480_e157922;
        locals.var_dnm_dn0 = assign105480_e157922_d_n0;
        locals.var_dnm_dn2 = assign105480_e157922_d_n2;
        locals.var_dnm_dn4 = assign105480_e157922_d_n4;
        locals.var_dnm_dn5 = assign105480_e157922_d_n5;
        locals.var_dnm_dn6 = assign105480_e157922_d_n6;
        locals.var_dnm_dn7 = assign105480_e157922_d_n7;
        locals.var_dnm_dn8 = assign105480_e157922_d_n8;
        locals.var_dnm_dn9 = assign105480_e157922_d_n9;
        locals.var_dnm_dn10 = assign105480_e157922_d_n10;
        locals.var_dnm_dn13 = assign105480_e157922_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign105490_e157937: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2388 = assign105490_e157937;
        locals.var_guard2388_rv = 0.0;

        let assign105500_e157940: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2389 = assign105500_e157940;
        locals.var_guard2389_rv = 0.0;

        let (assign105510_e157953,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105510_e157953;
        locals.var_mm_rv = 0.0;

        let assign105520_e157956: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2390 = assign105520_e157956;
        locals.var_guard2390_rv = 0.0;

        let (assign105530_e157972,) = {
    if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 == 0.0)) && (locals.var_guard2390 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105530_e157972;
        locals.var_mm_rv = 0.0;

        let assign105540_e157975: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2391 = assign105540_e157975;
        locals.var_guard2391_rv = 0.0;

        let (assign105550_e157994,) = {
    if (((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 == 0.0)) && (locals.var_guard2390 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105550_e157994;
        locals.var_mm_rv = 0.0;

        let assign105560_e157997: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2392 = assign105560_e157997;
        locals.var_guard2392_rv = 0.0;

        let (assign105570_e158019,) = {
    if ((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 == 0.0)) && (locals.var_guard2390 == 0.0)) && (locals.var_guard2391 == 0.0)) && (locals.var_guard2392 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105570_e158019;
        locals.var_mm_rv = 0.0;

        let (assign105580_e158030,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105580_e158030;
        locals.var_m0_rv = 0.0;

        let mut assign105590_loop_guard: usize = 0;
        while {
            let assign105590_cond_e158042: f64 = if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105590_cond_e158042 != 0.0
        } {
            assign105590_loop_guard += 1;
            assert!(assign105590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105590_body0_e158054, assign105590_body0_e158054_d_n0, assign105590_body0_e158054_d_n2, assign105590_body0_e158054_d_n4, assign105590_body0_e158054_d_n5, assign105590_body0_e158054_d_n6, assign105590_body0_e158054_d_n7, assign105590_body0_e158054_d_n8, assign105590_body0_e158054_d_n9, assign105590_body0_e158054_d_n10, assign105590_body0_e158054_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) {
        let assign105590_body0_e158052: f64 = (locals.var_dnm).sqrt();
        (assign105590_body0_e158052, (locals.var_dnm_dn0 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn2 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn4 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn5 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn6 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn7 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn8 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn9 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn10 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn13 / (2.0 * assign105590_body0_e158052)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign105590_body0_e158054;
            locals.var_dnm_dn0 = assign105590_body0_e158054_d_n0;
            locals.var_dnm_dn2 = assign105590_body0_e158054_d_n2;
            locals.var_dnm_dn4 = assign105590_body0_e158054_d_n4;
            locals.var_dnm_dn5 = assign105590_body0_e158054_d_n5;
            locals.var_dnm_dn6 = assign105590_body0_e158054_d_n6;
            locals.var_dnm_dn7 = assign105590_body0_e158054_d_n7;
            locals.var_dnm_dn8 = assign105590_body0_e158054_d_n8;
            locals.var_dnm_dn9 = assign105590_body0_e158054_d_n9;
            locals.var_dnm_dn10 = assign105590_body0_e158054_d_n10;
            locals.var_dnm_dn13 = assign105590_body0_e158054_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign105590_body1_e158067,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) {
        let assign105590_body1_e158065: f64 = (locals.var_m0 + 1.0);
        (assign105590_body1_e158065,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign105590_body1_e158067;
            locals.var_m0_rv = 0.0;
        }

        let (assign105600_e158090, assign105600_e158090_d_n0, assign105600_e158090_d_n2, assign105600_e158090_d_n4, assign105600_e158090_d_n5, assign105600_e158090_d_n6, assign105600_e158090_d_n7, assign105600_e158090_d_n8, assign105600_e158090_d_n9, assign105600_e158090_d_n10, assign105600_e158090_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 == 0.0)) {
        let (assign105600_e158088, assign105600_e158088_d_n0, assign105600_e158088_d_n2, assign105600_e158088_d_n4, assign105600_e158088_d_n5, assign105600_e158088_d_n6, assign105600_e158088_d_n7, assign105600_e158088_d_n8, assign105600_e158088_d_n9, assign105600_e158088_d_n10, assign105600_e158088_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105600_e158085: f64 = (2.0 * 2.0);
                let assign105600_e158086: f64 = (1.0 / assign105600_e158085);
                let assign105600_e158087: f64 = (locals.var_dnm).powf(assign105600_e158086);
                (assign105600_e158087, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn13)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign105600_e158088, assign105600_e158088_d_n0, assign105600_e158088_d_n2, assign105600_e158088_d_n4, assign105600_e158088_d_n5, assign105600_e158088_d_n6, assign105600_e158088_d_n7, assign105600_e158088_d_n8, assign105600_e158088_d_n9, assign105600_e158088_d_n10, assign105600_e158088_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105600_e158090;
        locals.var_dnm_dn0 = assign105600_e158090_d_n0;
        locals.var_dnm_dn2 = assign105600_e158090_d_n2;
        locals.var_dnm_dn4 = assign105600_e158090_d_n4;
        locals.var_dnm_dn5 = assign105600_e158090_d_n5;
        locals.var_dnm_dn6 = assign105600_e158090_d_n6;
        locals.var_dnm_dn7 = assign105600_e158090_d_n7;
        locals.var_dnm_dn8 = assign105600_e158090_d_n8;
        locals.var_dnm_dn9 = assign105600_e158090_d_n9;
        locals.var_dnm_dn10 = assign105600_e158090_d_n10;
        locals.var_dnm_dn13 = assign105600_e158090_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign105610_e158101, assign105610_e158101_d_n0, assign105610_e158101_d_n2, assign105610_e158101_d_n4, assign105610_e158101_d_n5, assign105610_e158101_d_n6, assign105610_e158101_d_n7, assign105610_e158101_d_n8, assign105610_e158101_d_n9, assign105610_e158101_d_n10, assign105610_e158101_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105610_e158099: f64 = (1.0 / locals.var_dnm);
        (assign105610_e158099, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105610_e158101;
        locals.var_dnm_dn0 = assign105610_e158101_d_n0;
        locals.var_dnm_dn2 = assign105610_e158101_d_n2;
        locals.var_dnm_dn4 = assign105610_e158101_d_n4;
        locals.var_dnm_dn5 = assign105610_e158101_d_n5;
        locals.var_dnm_dn6 = assign105610_e158101_d_n6;
        locals.var_dnm_dn7 = assign105610_e158101_d_n7;
        locals.var_dnm_dn8 = assign105610_e158101_d_n8;
        locals.var_dnm_dn9 = assign105610_e158101_d_n9;
        locals.var_dnm_dn10 = assign105610_e158101_d_n10;
        locals.var_dnm_dn13 = assign105610_e158101_d_n13;
        locals.var_dnm_rv = 0.0;

    }
}

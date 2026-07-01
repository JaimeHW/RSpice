#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_249(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign68970_e105536, assign68970_e105536_d_n0, assign68970_e105536_d_n2, assign68970_e105536_d_n4, assign68970_e105536_d_n5, assign68970_e105536_d_n6, assign68970_e105536_d_n7, assign68970_e105536_d_n8, assign68970_e105536_d_n9, assign68970_e105536_d_n10, assign68970_e105536_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign68970_e105534: f64 = (locals.var_t1 * locals.var_t2);
        (assign68970_e105534, ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)), ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)), ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)), ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)), ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)), ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)), ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)), ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)), ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)), ((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13)),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, locals.var_e1_dn10, locals.var_e1_dn13,)
    }
};
        locals.var_e1 = assign68970_e105536;
        locals.var_e1_dn0 = assign68970_e105536_d_n0;
        locals.var_e1_dn2 = assign68970_e105536_d_n2;
        locals.var_e1_dn4 = assign68970_e105536_d_n4;
        locals.var_e1_dn5 = assign68970_e105536_d_n5;
        locals.var_e1_dn6 = assign68970_e105536_d_n6;
        locals.var_e1_dn7 = assign68970_e105536_d_n7;
        locals.var_e1_dn8 = assign68970_e105536_d_n8;
        locals.var_e1_dn9 = assign68970_e105536_d_n9;
        locals.var_e1_dn10 = assign68970_e105536_d_n10;
        locals.var_e1_dn13 = assign68970_e105536_d_n13;
        locals.var_e1_rv = 0.0;

        let (assign68980_e105554, assign68980_e105554_d_n0, assign68980_e105554_d_n2, assign68980_e105554_d_n4, assign68980_e105554_d_n5, assign68980_e105554_d_n6, assign68980_e105554_d_n7, assign68980_e105554_d_n8, assign68980_e105554_d_n9, assign68980_e105554_d_n10, assign68980_e105554_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign68980_e105541: f64 = (locals.var_e1 * locals.var_e1);
        let assign68980_e105545: f64 = (0.01 / 0.01);
        let assign68980_e105546: f64 = (4.0 * assign68980_e105545);
        let assign68980_e105549: f64 = (0.01 / 0.01);
        let assign68980_e105550: f64 = (assign68980_e105546 * assign68980_e105549);
        let assign68980_e105551: f64 = (assign68980_e105541 + assign68980_e105550);
        let assign68980_e105552: f64 = (assign68980_e105551).sqrt();
        (assign68980_e105552, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn4 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn4)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn5 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn5)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn7 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn7)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn8 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn8)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn9 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn9)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn13 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn13)) / (2.0 * assign68980_e105552)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign68980_e105554;
        locals.var_tmf2_dn0 = assign68980_e105554_d_n0;
        locals.var_tmf2_dn2 = assign68980_e105554_d_n2;
        locals.var_tmf2_dn4 = assign68980_e105554_d_n4;
        locals.var_tmf2_dn5 = assign68980_e105554_d_n5;
        locals.var_tmf2_dn6 = assign68980_e105554_d_n6;
        locals.var_tmf2_dn7 = assign68980_e105554_d_n7;
        locals.var_tmf2_dn8 = assign68980_e105554_d_n8;
        locals.var_tmf2_dn9 = assign68980_e105554_d_n9;
        locals.var_tmf2_dn10 = assign68980_e105554_d_n10;
        locals.var_tmf2_dn13 = assign68980_e105554_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign68990_e105565, assign68990_e105565_d_n0, assign68990_e105565_d_n2, assign68990_e105565_d_n4, assign68990_e105565_d_n5, assign68990_e105565_d_n6, assign68990_e105565_d_n7, assign68990_e105565_d_n8, assign68990_e105565_d_n9, assign68990_e105565_d_n10, assign68990_e105565_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign68990_e105561: f64 = (locals.var_e1 / locals.var_tmf2);
        let assign68990_e105562: f64 = (1.0 + assign68990_e105561);
        let assign68990_e105563: f64 = (0.5 * assign68990_e105562);
        (assign68990_e105563, (0.5 * (((locals.var_e1_dn0 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn2 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn4 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn5 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn6 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn7 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn8 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn9 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn10 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn13 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign68990_e105565;
        locals.var_t5_dn0 = assign68990_e105565_d_n0;
        locals.var_t5_dn2 = assign68990_e105565_d_n2;
        locals.var_t5_dn4 = assign68990_e105565_d_n4;
        locals.var_t5_dn5 = assign68990_e105565_d_n5;
        locals.var_t5_dn6 = assign68990_e105565_d_n6;
        locals.var_t5_dn7 = assign68990_e105565_d_n7;
        locals.var_t5_dn8 = assign68990_e105565_d_n8;
        locals.var_t5_dn9 = assign68990_e105565_d_n9;
        locals.var_t5_dn10 = assign68990_e105565_d_n10;
        locals.var_t5_dn13 = assign68990_e105565_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign69000_e105574, assign69000_e105574_d_n0, assign69000_e105574_d_n2, assign69000_e105574_d_n4, assign69000_e105574_d_n5, assign69000_e105574_d_n6, assign69000_e105574_d_n7, assign69000_e105574_d_n8, assign69000_e105574_d_n9, assign69000_e105574_d_n10, assign69000_e105574_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign69000_e105571: f64 = (locals.var_e1 + locals.var_tmf2);
        let assign69000_e105572: f64 = (0.5 * assign69000_e105571);
        (assign69000_e105572, (0.5 * (locals.var_e1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_e1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_e1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_e1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_e1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_e1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_e1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn4, locals.var_egidl_dn5, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn8, locals.var_egidl_dn9, locals.var_egidl_dn10, locals.var_egidl_dn13,)
    }
};
        locals.var_egidl = assign69000_e105574;
        locals.var_egidl_dn0 = assign69000_e105574_d_n0;
        locals.var_egidl_dn2 = assign69000_e105574_d_n2;
        locals.var_egidl_dn4 = assign69000_e105574_d_n4;
        locals.var_egidl_dn5 = assign69000_e105574_d_n5;
        locals.var_egidl_dn6 = assign69000_e105574_d_n6;
        locals.var_egidl_dn7 = assign69000_e105574_d_n7;
        locals.var_egidl_dn8 = assign69000_e105574_d_n8;
        locals.var_egidl_dn9 = assign69000_e105574_d_n9;
        locals.var_egidl_dn10 = assign69000_e105574_d_n10;
        locals.var_egidl_dn13 = assign69000_e105574_d_n13;
        locals.var_egidl_rv = 0.0;

        let assign69010_e105577: f64 = if locals.var_egidl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1616 = assign69010_e105577;
        locals.var_guard1616_rv = 0.0;

        let (assign69020_e105584, assign69020_e105584_d_n0, assign69020_e105584_d_n2, assign69020_e105584_d_n4, assign69020_e105584_d_n5, assign69020_e105584_d_n6, assign69020_e105584_d_n7, assign69020_e105584_d_n8, assign69020_e105584_d_n9, assign69020_e105584_d_n10, assign69020_e105584_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1616 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn4, locals.var_egidl_dn5, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn8, locals.var_egidl_dn9, locals.var_egidl_dn10, locals.var_egidl_dn13,)
    }
};
        locals.var_egidl = assign69020_e105584;
        locals.var_egidl_dn0 = assign69020_e105584_d_n0;
        locals.var_egidl_dn2 = assign69020_e105584_d_n2;
        locals.var_egidl_dn4 = assign69020_e105584_d_n4;
        locals.var_egidl_dn5 = assign69020_e105584_d_n5;
        locals.var_egidl_dn6 = assign69020_e105584_d_n6;
        locals.var_egidl_dn7 = assign69020_e105584_d_n7;
        locals.var_egidl_dn8 = assign69020_e105584_d_n8;
        locals.var_egidl_dn9 = assign69020_e105584_d_n9;
        locals.var_egidl_dn10 = assign69020_e105584_d_n10;
        locals.var_egidl_dn13 = assign69020_e105584_d_n13;
        locals.var_egidl_rv = 0.0;

        let (assign69030_e105591, assign69030_e105591_d_n0, assign69030_e105591_d_n2, assign69030_e105591_d_n4, assign69030_e105591_d_n5, assign69030_e105591_d_n6, assign69030_e105591_d_n7, assign69030_e105591_d_n8, assign69030_e105591_d_n9, assign69030_e105591_d_n10, assign69030_e105591_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1616 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign69030_e105591;
        locals.var_t5_dn0 = assign69030_e105591_d_n0;
        locals.var_t5_dn2 = assign69030_e105591_d_n2;
        locals.var_t5_dn4 = assign69030_e105591_d_n4;
        locals.var_t5_dn5 = assign69030_e105591_d_n5;
        locals.var_t5_dn6 = assign69030_e105591_d_n6;
        locals.var_t5_dn7 = assign69030_e105591_d_n7;
        locals.var_t5_dn8 = assign69030_e105591_d_n8;
        locals.var_t5_dn9 = assign69030_e105591_d_n9;
        locals.var_t5_dn10 = assign69030_e105591_d_n10;
        locals.var_t5_dn13 = assign69030_e105591_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign69040_e105600, assign69040_e105600_d_n0, assign69040_e105600_d_n2, assign69040_e105600_d_n4, assign69040_e105600_d_n5, assign69040_e105600_d_n6, assign69040_e105600_d_n7, assign69040_e105600_d_n8, assign69040_e105600_d_n9, assign69040_e105600_d_n10, assign69040_e105600_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign69040_e105597: f64 = (locals.var_egidl + 1e-25);
        let assign69040_e105598: f64 = (1.0 / assign69040_e105597);
        (assign69040_e105598, (-(locals.var_egidl_dn0 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn2 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn4 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn5 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn6 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn7 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn8 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn9 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn10 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn13 / (assign69040_e105597 * assign69040_e105597))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign69040_e105600;
        locals.var_t3_dn0 = assign69040_e105600_d_n0;
        locals.var_t3_dn2 = assign69040_e105600_d_n2;
        locals.var_t3_dn4 = assign69040_e105600_d_n4;
        locals.var_t3_dn5 = assign69040_e105600_d_n5;
        locals.var_t3_dn6 = assign69040_e105600_d_n6;
        locals.var_t3_dn7 = assign69040_e105600_d_n7;
        locals.var_t3_dn8 = assign69040_e105600_d_n8;
        locals.var_t3_dn9 = assign69040_e105600_d_n9;
        locals.var_t3_dn10 = assign69040_e105600_d_n10;
        locals.var_t3_dn13 = assign69040_e105600_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign69050_e105610, assign69050_e105610_d_n0, assign69050_e105610_d_n2, assign69050_e105610_d_n4, assign69050_e105610_d_n5, assign69050_e105610_d_n6, assign69050_e105610_d_n7, assign69050_e105610_d_n8, assign69050_e105610_d_n9, assign69050_e105610_d_n10, assign69050_e105610_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign69050_e105604: f64 = (-locals.var_uc_gidl2);
        let assign69050_e105606: f64 = (assign69050_e105604 * locals.var_egp32);
        let assign69050_e105608: f64 = (assign69050_e105606 * locals.var_t3);
        (assign69050_e105608, (((assign69050_e105604 * locals.var_egp32_dn0) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn0)), (((assign69050_e105604 * locals.var_egp32_dn2) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn2)), (((assign69050_e105604 * locals.var_egp32_dn4) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn4)), (((assign69050_e105604 * locals.var_egp32_dn5) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn5)), (((assign69050_e105604 * locals.var_egp32_dn6) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn6)), (((assign69050_e105604 * locals.var_egp32_dn7) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn7)), (((assign69050_e105604 * locals.var_egp32_dn8) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn8)), (((assign69050_e105604 * locals.var_egp32_dn9) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn9)), (((assign69050_e105604 * locals.var_egp32_dn10) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn10)), (((assign69050_e105604 * locals.var_egp32_dn13) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign69050_e105610;
        locals.var_t0_dn0 = assign69050_e105610_d_n0;
        locals.var_t0_dn2 = assign69050_e105610_d_n2;
        locals.var_t0_dn4 = assign69050_e105610_d_n4;
        locals.var_t0_dn5 = assign69050_e105610_d_n5;
        locals.var_t0_dn6 = assign69050_e105610_d_n6;
        locals.var_t0_dn7 = assign69050_e105610_d_n7;
        locals.var_t0_dn8 = assign69050_e105610_d_n8;
        locals.var_t0_dn9 = assign69050_e105610_d_n9;
        locals.var_t0_dn10 = assign69050_e105610_d_n10;
        locals.var_t0_dn13 = assign69050_e105610_d_n13;
        locals.var_t0_rv = 0.0;

        let assign69060_e105613: f64 = (-34.0);
        let assign69060_e105614: f64 = if locals.var_t0 < assign69060_e105613 { 1.0 } else { 0.0 };
        locals.var_guard1617 = assign69060_e105614;
        locals.var_guard1617_rv = 0.0;

        let (assign69080_e105630, assign69080_e105630_d_n0, assign69080_e105630_d_n2, assign69080_e105630_d_n4, assign69080_e105630_d_n5, assign69080_e105630_d_n6, assign69080_e105630_d_n7, assign69080_e105630_d_n8, assign69080_e105630_d_n9, assign69080_e105630_d_n10, assign69080_e105630_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1617 == 0.0)) {
        let assign69080_e105628: f64 = (locals.var_t0).exp();
        (assign69080_e105628, (assign69080_e105628 * locals.var_t0_dn0), (assign69080_e105628 * locals.var_t0_dn2), (assign69080_e105628 * locals.var_t0_dn4), (assign69080_e105628 * locals.var_t0_dn5), (assign69080_e105628 * locals.var_t0_dn6), (assign69080_e105628 * locals.var_t0_dn7), (assign69080_e105628 * locals.var_t0_dn8), (assign69080_e105628 * locals.var_t0_dn9), (assign69080_e105628 * locals.var_t0_dn10), (assign69080_e105628 * locals.var_t0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign69080_e105630;
        locals.var_t1_dn0 = assign69080_e105630_d_n0;
        locals.var_t1_dn2 = assign69080_e105630_d_n2;
        locals.var_t1_dn4 = assign69080_e105630_d_n4;
        locals.var_t1_dn5 = assign69080_e105630_d_n5;
        locals.var_t1_dn6 = assign69080_e105630_d_n6;
        locals.var_t1_dn7 = assign69080_e105630_d_n7;
        locals.var_t1_dn8 = assign69080_e105630_d_n8;
        locals.var_t1_dn9 = assign69080_e105630_d_n9;
        locals.var_t1_dn10 = assign69080_e105630_d_n10;
        locals.var_t1_dn13 = assign69080_e105630_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign69090_e105644, assign69090_e105644_d_n0, assign69090_e105644_d_n2, assign69090_e105644_d_n4, assign69090_e105644_d_n5, assign69090_e105644_d_n6, assign69090_e105644_d_n7, assign69090_e105644_d_n8, assign69090_e105644_d_n9, assign69090_e105644_d_n10, assign69090_e105644_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1617 == 0.0)) {
        let assign69090_e105638: f64 = (locals.var_uc_gidl1 / locals.var_egp12);
        let assign69090_e105640: f64 = (assign69090_e105638 * 1.6021918e-19);
        let assign69090_e105642: f64 = (assign69090_e105640 * locals.var_weff_nf);
        (assign69090_e105642, (((-((locals.var_uc_gidl1 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn4) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn5) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn8) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn9) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn13) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69090_e105644;
        locals.var_t2_dn0 = assign69090_e105644_d_n0;
        locals.var_t2_dn2 = assign69090_e105644_d_n2;
        locals.var_t2_dn4 = assign69090_e105644_d_n4;
        locals.var_t2_dn5 = assign69090_e105644_d_n5;
        locals.var_t2_dn6 = assign69090_e105644_d_n6;
        locals.var_t2_dn7 = assign69090_e105644_d_n7;
        locals.var_t2_dn8 = assign69090_e105644_d_n8;
        locals.var_t2_dn9 = assign69090_e105644_d_n9;
        locals.var_t2_dn10 = assign69090_e105644_d_n10;
        locals.var_t2_dn13 = assign69090_e105644_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign69110_e105665, assign69110_e105665_d_n0, assign69110_e105665_d_n2, assign69110_e105665_d_n4, assign69110_e105665_d_n5, assign69110_e105665_d_n6, assign69110_e105665_d_n7, assign69110_e105665_d_n8, assign69110_e105665_d_n9, assign69110_e105665_d_n10, assign69110_e105665_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign69110_e105663: f64 = (locals.var_vds - locals.var_vbs);
        (assign69110_e105663, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, (locals.var_vds_dn5 - locals.var_vbs_dn5), locals.var_vds_dn6, (locals.var_vds_dn7 - locals.var_vbs_dn7), (locals.var_vds_dn8 - locals.var_vbs_dn8), locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    } else {
        (locals.var_vdb, locals.var_vdb_dn0, locals.var_vdb_dn2, locals.var_vdb_dn4, locals.var_vdb_dn5, locals.var_vdb_dn6, locals.var_vdb_dn7, locals.var_vdb_dn8, locals.var_vdb_dn9, locals.var_vdb_dn10, locals.var_vdb_dn13,)
    }
};
        locals.var_vdb = assign69110_e105665;
        locals.var_vdb_dn0 = assign69110_e105665_d_n0;
        locals.var_vdb_dn2 = assign69110_e105665_d_n2;
        locals.var_vdb_dn4 = assign69110_e105665_d_n4;
        locals.var_vdb_dn5 = assign69110_e105665_d_n5;
        locals.var_vdb_dn6 = assign69110_e105665_d_n6;
        locals.var_vdb_dn7 = assign69110_e105665_d_n7;
        locals.var_vdb_dn8 = assign69110_e105665_d_n8;
        locals.var_vdb_dn9 = assign69110_e105665_d_n9;
        locals.var_vdb_dn10 = assign69110_e105665_d_n10;
        locals.var_vdb_dn13 = assign69110_e105665_d_n13;
        locals.var_vdb_rv = 0.0;

        let assign69120_e105668: f64 = if locals.var_vdb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1618 = assign69120_e105668;
        locals.var_guard1618_rv = 0.0;

        let (assign69130_e105677, assign69130_e105677_d_n0, assign69130_e105677_d_n2, assign69130_e105677_d_n4, assign69130_e105677_d_n5, assign69130_e105677_d_n6, assign69130_e105677_d_n7, assign69130_e105677_d_n8, assign69130_e105677_d_n9, assign69130_e105677_d_n10, assign69130_e105677_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 != 0.0)) {
        let assign69130_e105675: f64 = (locals.var_vdb * locals.var_vdb);
        (assign69130_e105675, ((locals.var_vdb_dn0 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn0)), ((locals.var_vdb_dn2 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn2)), ((locals.var_vdb_dn4 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn4)), ((locals.var_vdb_dn5 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn5)), ((locals.var_vdb_dn6 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn6)), ((locals.var_vdb_dn7 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn7)), ((locals.var_vdb_dn8 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn8)), ((locals.var_vdb_dn9 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn9)), ((locals.var_vdb_dn10 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn10)), ((locals.var_vdb_dn13 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69130_e105677;
        locals.var_t2_dn0 = assign69130_e105677_d_n0;
        locals.var_t2_dn2 = assign69130_e105677_d_n2;
        locals.var_t2_dn4 = assign69130_e105677_d_n4;
        locals.var_t2_dn5 = assign69130_e105677_d_n5;
        locals.var_t2_dn6 = assign69130_e105677_d_n6;
        locals.var_t2_dn7 = assign69130_e105677_d_n7;
        locals.var_t2_dn8 = assign69130_e105677_d_n8;
        locals.var_t2_dn9 = assign69130_e105677_d_n9;
        locals.var_t2_dn10 = assign69130_e105677_d_n10;
        locals.var_t2_dn13 = assign69130_e105677_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign69140_e105686, assign69140_e105686_d_n0, assign69140_e105686_d_n2, assign69140_e105686_d_n4, assign69140_e105686_d_n5, assign69140_e105686_d_n6, assign69140_e105686_d_n7, assign69140_e105686_d_n8, assign69140_e105686_d_n9, assign69140_e105686_d_n10, assign69140_e105686_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 != 0.0)) {
        let assign69140_e105684: f64 = (locals.var_t2 * locals.var_vdb);
        (assign69140_e105684, ((locals.var_t2_dn0 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn0)), ((locals.var_t2_dn2 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn2)), ((locals.var_t2_dn4 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn4)), ((locals.var_t2_dn5 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn5)), ((locals.var_t2_dn6 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn6)), ((locals.var_t2_dn7 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn7)), ((locals.var_t2_dn8 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn8)), ((locals.var_t2_dn9 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn9)), ((locals.var_t2_dn10 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn10)), ((locals.var_t2_dn13 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign69140_e105686;
        locals.var_t4_dn0 = assign69140_e105686_d_n0;
        locals.var_t4_dn2 = assign69140_e105686_d_n2;
        locals.var_t4_dn4 = assign69140_e105686_d_n4;
        locals.var_t4_dn5 = assign69140_e105686_d_n5;
        locals.var_t4_dn6 = assign69140_e105686_d_n6;
        locals.var_t4_dn7 = assign69140_e105686_d_n7;
        locals.var_t4_dn8 = assign69140_e105686_d_n8;
        locals.var_t4_dn9 = assign69140_e105686_d_n9;
        locals.var_t4_dn10 = assign69140_e105686_d_n10;
        locals.var_t4_dn13 = assign69140_e105686_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign69150_e105695, assign69150_e105695_d_n0, assign69150_e105695_d_n2, assign69150_e105695_d_n4, assign69150_e105695_d_n5, assign69150_e105695_d_n6, assign69150_e105695_d_n7, assign69150_e105695_d_n8, assign69150_e105695_d_n9, assign69150_e105695_d_n10, assign69150_e105695_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 != 0.0)) {
        let assign69150_e105693: f64 = (locals.var_t4 + 0.5);
        (assign69150_e105693, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign69150_e105695;
        locals.var_t0_dn0 = assign69150_e105695_d_n0;
        locals.var_t0_dn2 = assign69150_e105695_d_n2;
        locals.var_t0_dn4 = assign69150_e105695_d_n4;
        locals.var_t0_dn5 = assign69150_e105695_d_n5;
        locals.var_t0_dn6 = assign69150_e105695_d_n6;
        locals.var_t0_dn7 = assign69150_e105695_d_n7;
        locals.var_t0_dn8 = assign69150_e105695_d_n8;
        locals.var_t0_dn9 = assign69150_e105695_d_n9;
        locals.var_t0_dn10 = assign69150_e105695_d_n10;
        locals.var_t0_dn13 = assign69150_e105695_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign69160_e105704, assign69160_e105704_d_n0, assign69160_e105704_d_n2, assign69160_e105704_d_n4, assign69160_e105704_d_n5, assign69160_e105704_d_n6, assign69160_e105704_d_n7, assign69160_e105704_d_n8, assign69160_e105704_d_n9, assign69160_e105704_d_n10, assign69160_e105704_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 != 0.0)) {
        let assign69160_e105702: f64 = (locals.var_t4 / locals.var_t0);
        (assign69160_e105702, (((locals.var_t4_dn0 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn2 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn4 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn5 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn6 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn7 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn8 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn9 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn10 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn13 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign69160_e105704;
        locals.var_t5_dn0 = assign69160_e105704_d_n0;
        locals.var_t5_dn2 = assign69160_e105704_d_n2;
        locals.var_t5_dn4 = assign69160_e105704_d_n4;
        locals.var_t5_dn5 = assign69160_e105704_d_n5;
        locals.var_t5_dn6 = assign69160_e105704_d_n6;
        locals.var_t5_dn7 = assign69160_e105704_d_n7;
        locals.var_t5_dn8 = assign69160_e105704_d_n8;
        locals.var_t5_dn9 = assign69160_e105704_d_n9;
        locals.var_t5_dn10 = assign69160_e105704_d_n10;
        locals.var_t5_dn13 = assign69160_e105704_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign69170_e105725, assign69170_e105725_d_n0, assign69170_e105725_d_n2, assign69170_e105725_d_n4, assign69170_e105725_d_n5, assign69170_e105725_d_n6, assign69170_e105725_d_n7, assign69170_e105725_d_n8, assign69170_e105725_d_n9, assign69170_e105725_d_n10, assign69170_e105725_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 != 0.0)) {
        let assign69170_e105711: f64 = (3.0 * locals.var_t2);
        let assign69170_e105713: f64 = (assign69170_e105711 * locals.var_t0);
        let assign69170_e105716: f64 = (locals.var_t4 * 3.0);
        let assign69170_e105718: f64 = (assign69170_e105716 * locals.var_t2);
        let assign69170_e105719: f64 = (assign69170_e105713 - assign69170_e105718);
        let assign69170_e105722: f64 = (locals.var_t0 * locals.var_t0);
        let assign69170_e105723: f64 = (assign69170_e105719 / assign69170_e105722);
        (assign69170_e105723, (((((((3.0 * locals.var_t2_dn0) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn0)) - (((locals.var_t4_dn0 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn0))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn2) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn2)) - (((locals.var_t4_dn2 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn2))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn4) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn4)) - (((locals.var_t4_dn4 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn4))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn5) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn5)) - (((locals.var_t4_dn5 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn5))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn6) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn6)) - (((locals.var_t4_dn6 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn6))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn7) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn7)) - (((locals.var_t4_dn7 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn7))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn8) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn8)) - (((locals.var_t4_dn8 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn8))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn9) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn9)) - (((locals.var_t4_dn9 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn9))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn10) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn10)) - (((locals.var_t4_dn10 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn10))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn13) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn13)) - (((locals.var_t4_dn13 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn13))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)))) / (assign69170_e105722 * assign69170_e105722)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign69170_e105725;
        locals.var_t7_dn0 = assign69170_e105725_d_n0;
        locals.var_t7_dn2 = assign69170_e105725_d_n2;
        locals.var_t7_dn4 = assign69170_e105725_d_n4;
        locals.var_t7_dn5 = assign69170_e105725_d_n5;
        locals.var_t7_dn6 = assign69170_e105725_d_n6;
        locals.var_t7_dn7 = assign69170_e105725_d_n7;
        locals.var_t7_dn8 = assign69170_e105725_d_n8;
        locals.var_t7_dn9 = assign69170_e105725_d_n9;
        locals.var_t7_dn10 = assign69170_e105725_d_n10;
        locals.var_t7_dn13 = assign69170_e105725_d_n13;
        locals.var_t7_rv = 0.0;

        let assign69200_e105745: f64 = if p.p25 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1619 = assign69200_e105745;
        locals.var_guard1619_rv = 0.0;

        let (assign69220_e105769, assign69220_e105769_d_n0, assign69220_e105769_d_n2, assign69220_e105769_d_n4, assign69220_e105769_d_n5, assign69220_e105769_d_n6, assign69220_e105769_d_n7, assign69220_e105769_d_n8, assign69220_e105769_d_n9, assign69220_e105769_d_n10, assign69220_e105769_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69220_e105754: f64 = (-locals.var_vdsp);
        let assign69220_e105756: f64 = (assign69220_e105754 + p.p243);
        let assign69220_e105757: f64 = (p.p242 * assign69220_e105756);
        let assign69220_e105760: f64 = (locals.var_vgs - locals.var_vdsp);
        let assign69220_e105761: f64 = (assign69220_e105757 - assign69220_e105760);
        let assign69220_e105764: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign69220_e105766: f64 = (assign69220_e105764 * p.p244);
        let assign69220_e105767: f64 = (assign69220_e105761 + assign69220_e105766);
        (assign69220_e105767, (((p.p242 * (-locals.var_vdsp_dn0)) - (-locals.var_vdsp_dn0)) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn2)) - (-locals.var_vdsp_dn2)) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn4)) - (-locals.var_vdsp_dn4)) + ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn5)) - (locals.var_vgs_dn5 - locals.var_vdsp_dn5)) + ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn6)) - (locals.var_vgs_dn6 - locals.var_vdsp_dn6)) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn7)) - (locals.var_vgs_dn7 - locals.var_vdsp_dn7)) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn8)) - (-locals.var_vdsp_dn8)) + ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn9)) - (-locals.var_vdsp_dn9)) + ((locals.var_dvthsc_dn9 + locals.var_dvthlp_dn9) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn10)) - (-locals.var_vdsp_dn10)) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn13)) - (-locals.var_vdsp_dn13)) + ((locals.var_dvthsc_dn13 + locals.var_dvthlp_dn13) * p.p244)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign69220_e105769;
        locals.var_t1_dn0 = assign69220_e105769_d_n0;
        locals.var_t1_dn2 = assign69220_e105769_d_n2;
        locals.var_t1_dn4 = assign69220_e105769_d_n4;
        locals.var_t1_dn5 = assign69220_e105769_d_n5;
        locals.var_t1_dn6 = assign69220_e105769_d_n6;
        locals.var_t1_dn7 = assign69220_e105769_d_n7;
        locals.var_t1_dn8 = assign69220_e105769_d_n8;
        locals.var_t1_dn9 = assign69220_e105769_d_n9;
        locals.var_t1_dn10 = assign69220_e105769_d_n10;
        locals.var_t1_dn13 = assign69220_e105769_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign69230_e105776, assign69230_e105776_d_n0, assign69230_e105776_d_n2, assign69230_e105776_d_n4, assign69230_e105776_d_n5, assign69230_e105776_d_n6, assign69230_e105776_d_n7, assign69230_e105776_d_n8, assign69230_e105776_d_n9, assign69230_e105776_d_n10, assign69230_e105776_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69230_e105774: f64 = (1.0 / locals.var_tox0);
        (assign69230_e105774, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69230_e105776;
        locals.var_t2_dn0 = assign69230_e105776_d_n0;
        locals.var_t2_dn2 = assign69230_e105776_d_n2;
        locals.var_t2_dn4 = assign69230_e105776_d_n4;
        locals.var_t2_dn5 = assign69230_e105776_d_n5;
        locals.var_t2_dn6 = assign69230_e105776_d_n6;
        locals.var_t2_dn7 = assign69230_e105776_d_n7;
        locals.var_t2_dn8 = assign69230_e105776_d_n8;
        locals.var_t2_dn9 = assign69230_e105776_d_n9;
        locals.var_t2_dn10 = assign69230_e105776_d_n10;
        locals.var_t2_dn13 = assign69230_e105776_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign69240_e105783, assign69240_e105783_d_n0, assign69240_e105783_d_n2, assign69240_e105783_d_n4, assign69240_e105783_d_n5, assign69240_e105783_d_n6, assign69240_e105783_d_n7, assign69240_e105783_d_n8, assign69240_e105783_d_n9, assign69240_e105783_d_n10, assign69240_e105783_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69240_e105781: f64 = (locals.var_t1 * locals.var_t2);
        (assign69240_e105781, ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)), ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)), ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)), ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)), ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)), ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)), ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)), ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)), ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)), ((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13)),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, locals.var_e1_dn10, locals.var_e1_dn13,)
    }
};
        locals.var_e1 = assign69240_e105783;
        locals.var_e1_dn0 = assign69240_e105783_d_n0;
        locals.var_e1_dn2 = assign69240_e105783_d_n2;
        locals.var_e1_dn4 = assign69240_e105783_d_n4;
        locals.var_e1_dn5 = assign69240_e105783_d_n5;
        locals.var_e1_dn6 = assign69240_e105783_d_n6;
        locals.var_e1_dn7 = assign69240_e105783_d_n7;
        locals.var_e1_dn8 = assign69240_e105783_d_n8;
        locals.var_e1_dn9 = assign69240_e105783_d_n9;
        locals.var_e1_dn10 = assign69240_e105783_d_n10;
        locals.var_e1_dn13 = assign69240_e105783_d_n13;
        locals.var_e1_rv = 0.0;

        let (assign69250_e105801, assign69250_e105801_d_n0, assign69250_e105801_d_n2, assign69250_e105801_d_n4, assign69250_e105801_d_n5, assign69250_e105801_d_n6, assign69250_e105801_d_n7, assign69250_e105801_d_n8, assign69250_e105801_d_n9, assign69250_e105801_d_n10, assign69250_e105801_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69250_e105788: f64 = (locals.var_e1 * locals.var_e1);
        let assign69250_e105792: f64 = (0.01 / 0.01);
        let assign69250_e105793: f64 = (4.0 * assign69250_e105792);
        let assign69250_e105796: f64 = (0.01 / 0.01);
        let assign69250_e105797: f64 = (assign69250_e105793 * assign69250_e105796);
        let assign69250_e105798: f64 = (assign69250_e105788 + assign69250_e105797);
        let assign69250_e105799: f64 = (assign69250_e105798).sqrt();
        (assign69250_e105799, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn4 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn4)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn5 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn5)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn7 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn7)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn8 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn8)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn9 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn9)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn13 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn13)) / (2.0 * assign69250_e105799)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign69250_e105801;
        locals.var_tmf2_dn0 = assign69250_e105801_d_n0;
        locals.var_tmf2_dn2 = assign69250_e105801_d_n2;
        locals.var_tmf2_dn4 = assign69250_e105801_d_n4;
        locals.var_tmf2_dn5 = assign69250_e105801_d_n5;
        locals.var_tmf2_dn6 = assign69250_e105801_d_n6;
        locals.var_tmf2_dn7 = assign69250_e105801_d_n7;
        locals.var_tmf2_dn8 = assign69250_e105801_d_n8;
        locals.var_tmf2_dn9 = assign69250_e105801_d_n9;
        locals.var_tmf2_dn10 = assign69250_e105801_d_n10;
        locals.var_tmf2_dn13 = assign69250_e105801_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign69260_e105812, assign69260_e105812_d_n0, assign69260_e105812_d_n2, assign69260_e105812_d_n4, assign69260_e105812_d_n5, assign69260_e105812_d_n6, assign69260_e105812_d_n7, assign69260_e105812_d_n8, assign69260_e105812_d_n9, assign69260_e105812_d_n10, assign69260_e105812_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69260_e105808: f64 = (locals.var_e1 / locals.var_tmf2);
        let assign69260_e105809: f64 = (1.0 + assign69260_e105808);
        let assign69260_e105810: f64 = (0.5 * assign69260_e105809);
        (assign69260_e105810, (0.5 * (((locals.var_e1_dn0 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn2 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn4 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn5 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn6 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn7 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn8 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn9 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn10 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn13 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign69260_e105812;
        locals.var_t5_dn0 = assign69260_e105812_d_n0;
        locals.var_t5_dn2 = assign69260_e105812_d_n2;
        locals.var_t5_dn4 = assign69260_e105812_d_n4;
        locals.var_t5_dn5 = assign69260_e105812_d_n5;
        locals.var_t5_dn6 = assign69260_e105812_d_n6;
        locals.var_t5_dn7 = assign69260_e105812_d_n7;
        locals.var_t5_dn8 = assign69260_e105812_d_n8;
        locals.var_t5_dn9 = assign69260_e105812_d_n9;
        locals.var_t5_dn10 = assign69260_e105812_d_n10;
        locals.var_t5_dn13 = assign69260_e105812_d_n13;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_250(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign69270_e105821, assign69270_e105821_d_n0, assign69270_e105821_d_n2, assign69270_e105821_d_n4, assign69270_e105821_d_n5, assign69270_e105821_d_n6, assign69270_e105821_d_n7, assign69270_e105821_d_n8, assign69270_e105821_d_n9, assign69270_e105821_d_n10, assign69270_e105821_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69270_e105818: f64 = (locals.var_e1 + locals.var_tmf2);
        let assign69270_e105819: f64 = (0.5 * assign69270_e105818);
        (assign69270_e105819, (0.5 * (locals.var_e1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_e1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_e1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_e1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_e1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_e1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_e1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn4, locals.var_egisl_dn5, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn8, locals.var_egisl_dn9, locals.var_egisl_dn10, locals.var_egisl_dn13,)
    }
};
        locals.var_egisl = assign69270_e105821;
        locals.var_egisl_dn0 = assign69270_e105821_d_n0;
        locals.var_egisl_dn2 = assign69270_e105821_d_n2;
        locals.var_egisl_dn4 = assign69270_e105821_d_n4;
        locals.var_egisl_dn5 = assign69270_e105821_d_n5;
        locals.var_egisl_dn6 = assign69270_e105821_d_n6;
        locals.var_egisl_dn7 = assign69270_e105821_d_n7;
        locals.var_egisl_dn8 = assign69270_e105821_d_n8;
        locals.var_egisl_dn9 = assign69270_e105821_d_n9;
        locals.var_egisl_dn10 = assign69270_e105821_d_n10;
        locals.var_egisl_dn13 = assign69270_e105821_d_n13;
        locals.var_egisl_rv = 0.0;

        let assign69280_e105824: f64 = if locals.var_egisl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1620 = assign69280_e105824;
        locals.var_guard1620_rv = 0.0;

        let (assign69290_e105831, assign69290_e105831_d_n0, assign69290_e105831_d_n2, assign69290_e105831_d_n4, assign69290_e105831_d_n5, assign69290_e105831_d_n6, assign69290_e105831_d_n7, assign69290_e105831_d_n8, assign69290_e105831_d_n9, assign69290_e105831_d_n10, assign69290_e105831_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1620 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn4, locals.var_egisl_dn5, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn8, locals.var_egisl_dn9, locals.var_egisl_dn10, locals.var_egisl_dn13,)
    }
};
        locals.var_egisl = assign69290_e105831;
        locals.var_egisl_dn0 = assign69290_e105831_d_n0;
        locals.var_egisl_dn2 = assign69290_e105831_d_n2;
        locals.var_egisl_dn4 = assign69290_e105831_d_n4;
        locals.var_egisl_dn5 = assign69290_e105831_d_n5;
        locals.var_egisl_dn6 = assign69290_e105831_d_n6;
        locals.var_egisl_dn7 = assign69290_e105831_d_n7;
        locals.var_egisl_dn8 = assign69290_e105831_d_n8;
        locals.var_egisl_dn9 = assign69290_e105831_d_n9;
        locals.var_egisl_dn10 = assign69290_e105831_d_n10;
        locals.var_egisl_dn13 = assign69290_e105831_d_n13;
        locals.var_egisl_rv = 0.0;

        let (assign69300_e105838, assign69300_e105838_d_n0, assign69300_e105838_d_n2, assign69300_e105838_d_n4, assign69300_e105838_d_n5, assign69300_e105838_d_n6, assign69300_e105838_d_n7, assign69300_e105838_d_n8, assign69300_e105838_d_n9, assign69300_e105838_d_n10, assign69300_e105838_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1620 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign69300_e105838;
        locals.var_t5_dn0 = assign69300_e105838_d_n0;
        locals.var_t5_dn2 = assign69300_e105838_d_n2;
        locals.var_t5_dn4 = assign69300_e105838_d_n4;
        locals.var_t5_dn5 = assign69300_e105838_d_n5;
        locals.var_t5_dn6 = assign69300_e105838_d_n6;
        locals.var_t5_dn7 = assign69300_e105838_d_n7;
        locals.var_t5_dn8 = assign69300_e105838_d_n8;
        locals.var_t5_dn9 = assign69300_e105838_d_n9;
        locals.var_t5_dn10 = assign69300_e105838_d_n10;
        locals.var_t5_dn13 = assign69300_e105838_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign69310_e105847, assign69310_e105847_d_n0, assign69310_e105847_d_n2, assign69310_e105847_d_n4, assign69310_e105847_d_n5, assign69310_e105847_d_n6, assign69310_e105847_d_n7, assign69310_e105847_d_n8, assign69310_e105847_d_n9, assign69310_e105847_d_n10, assign69310_e105847_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69310_e105844: f64 = (locals.var_egisl + 1e-25);
        let assign69310_e105845: f64 = (1.0 / assign69310_e105844);
        (assign69310_e105845, (-(locals.var_egisl_dn0 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn2 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn4 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn5 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn6 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn7 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn8 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn9 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn10 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn13 / (assign69310_e105844 * assign69310_e105844))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign69310_e105847;
        locals.var_t3_dn0 = assign69310_e105847_d_n0;
        locals.var_t3_dn2 = assign69310_e105847_d_n2;
        locals.var_t3_dn4 = assign69310_e105847_d_n4;
        locals.var_t3_dn5 = assign69310_e105847_d_n5;
        locals.var_t3_dn6 = assign69310_e105847_d_n6;
        locals.var_t3_dn7 = assign69310_e105847_d_n7;
        locals.var_t3_dn8 = assign69310_e105847_d_n8;
        locals.var_t3_dn9 = assign69310_e105847_d_n9;
        locals.var_t3_dn10 = assign69310_e105847_d_n10;
        locals.var_t3_dn13 = assign69310_e105847_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign69320_e105857, assign69320_e105857_d_n0, assign69320_e105857_d_n2, assign69320_e105857_d_n4, assign69320_e105857_d_n5, assign69320_e105857_d_n6, assign69320_e105857_d_n7, assign69320_e105857_d_n8, assign69320_e105857_d_n9, assign69320_e105857_d_n10, assign69320_e105857_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69320_e105851: f64 = (-locals.var_uc_gidl2);
        let assign69320_e105853: f64 = (assign69320_e105851 * locals.var_egp32);
        let assign69320_e105855: f64 = (assign69320_e105853 * locals.var_t3);
        (assign69320_e105855, (((assign69320_e105851 * locals.var_egp32_dn0) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn0)), (((assign69320_e105851 * locals.var_egp32_dn2) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn2)), (((assign69320_e105851 * locals.var_egp32_dn4) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn4)), (((assign69320_e105851 * locals.var_egp32_dn5) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn5)), (((assign69320_e105851 * locals.var_egp32_dn6) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn6)), (((assign69320_e105851 * locals.var_egp32_dn7) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn7)), (((assign69320_e105851 * locals.var_egp32_dn8) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn8)), (((assign69320_e105851 * locals.var_egp32_dn9) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn9)), (((assign69320_e105851 * locals.var_egp32_dn10) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn10)), (((assign69320_e105851 * locals.var_egp32_dn13) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign69320_e105857;
        locals.var_t0_dn0 = assign69320_e105857_d_n0;
        locals.var_t0_dn2 = assign69320_e105857_d_n2;
        locals.var_t0_dn4 = assign69320_e105857_d_n4;
        locals.var_t0_dn5 = assign69320_e105857_d_n5;
        locals.var_t0_dn6 = assign69320_e105857_d_n6;
        locals.var_t0_dn7 = assign69320_e105857_d_n7;
        locals.var_t0_dn8 = assign69320_e105857_d_n8;
        locals.var_t0_dn9 = assign69320_e105857_d_n9;
        locals.var_t0_dn10 = assign69320_e105857_d_n10;
        locals.var_t0_dn13 = assign69320_e105857_d_n13;
        locals.var_t0_rv = 0.0;

        let assign69330_e105860: f64 = (-34.0);
        let assign69330_e105861: f64 = if locals.var_t0 < assign69330_e105860 { 1.0 } else { 0.0 };
        locals.var_guard1621 = assign69330_e105861;
        locals.var_guard1621_rv = 0.0;

        let (assign69350_e105877, assign69350_e105877_d_n0, assign69350_e105877_d_n2, assign69350_e105877_d_n4, assign69350_e105877_d_n5, assign69350_e105877_d_n6, assign69350_e105877_d_n7, assign69350_e105877_d_n8, assign69350_e105877_d_n9, assign69350_e105877_d_n10, assign69350_e105877_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 == 0.0)) {
        let assign69350_e105875: f64 = (locals.var_t0).exp();
        (assign69350_e105875, (assign69350_e105875 * locals.var_t0_dn0), (assign69350_e105875 * locals.var_t0_dn2), (assign69350_e105875 * locals.var_t0_dn4), (assign69350_e105875 * locals.var_t0_dn5), (assign69350_e105875 * locals.var_t0_dn6), (assign69350_e105875 * locals.var_t0_dn7), (assign69350_e105875 * locals.var_t0_dn8), (assign69350_e105875 * locals.var_t0_dn9), (assign69350_e105875 * locals.var_t0_dn10), (assign69350_e105875 * locals.var_t0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign69350_e105877;
        locals.var_t1_dn0 = assign69350_e105877_d_n0;
        locals.var_t1_dn2 = assign69350_e105877_d_n2;
        locals.var_t1_dn4 = assign69350_e105877_d_n4;
        locals.var_t1_dn5 = assign69350_e105877_d_n5;
        locals.var_t1_dn6 = assign69350_e105877_d_n6;
        locals.var_t1_dn7 = assign69350_e105877_d_n7;
        locals.var_t1_dn8 = assign69350_e105877_d_n8;
        locals.var_t1_dn9 = assign69350_e105877_d_n9;
        locals.var_t1_dn10 = assign69350_e105877_d_n10;
        locals.var_t1_dn13 = assign69350_e105877_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign69360_e105887, assign69360_e105887_d_n0, assign69360_e105887_d_n2, assign69360_e105887_d_n4, assign69360_e105887_d_n5, assign69360_e105887_d_n6, assign69360_e105887_d_n7, assign69360_e105887_d_n8, assign69360_e105887_d_n9, assign69360_e105887_d_n10, assign69360_e105887_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 == 0.0)) {
        let assign69360_e105885: f64 = (1.0 / locals.var_egp12);
        (assign69360_e105885, (-(locals.var_egp12_dn0 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn2 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn4 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn5 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn6 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn7 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn8 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn9 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn10 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn13 / (locals.var_egp12 * locals.var_egp12))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign69360_e105887;
        locals.var_t3_dn0 = assign69360_e105887_d_n0;
        locals.var_t3_dn2 = assign69360_e105887_d_n2;
        locals.var_t3_dn4 = assign69360_e105887_d_n4;
        locals.var_t3_dn5 = assign69360_e105887_d_n5;
        locals.var_t3_dn6 = assign69360_e105887_d_n6;
        locals.var_t3_dn7 = assign69360_e105887_d_n7;
        locals.var_t3_dn8 = assign69360_e105887_d_n8;
        locals.var_t3_dn9 = assign69360_e105887_d_n9;
        locals.var_t3_dn10 = assign69360_e105887_d_n10;
        locals.var_t3_dn13 = assign69360_e105887_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign69370_e105901, assign69370_e105901_d_n0, assign69370_e105901_d_n2, assign69370_e105901_d_n4, assign69370_e105901_d_n5, assign69370_e105901_d_n6, assign69370_e105901_d_n7, assign69370_e105901_d_n8, assign69370_e105901_d_n9, assign69370_e105901_d_n10, assign69370_e105901_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 == 0.0)) {
        let assign69370_e105895: f64 = (locals.var_uc_gidl1 * locals.var_t3);
        let assign69370_e105897: f64 = (assign69370_e105895 * 1.6021918e-19);
        let assign69370_e105899: f64 = (assign69370_e105897 * locals.var_weff_nf);
        (assign69370_e105899, (((locals.var_uc_gidl1 * locals.var_t3_dn0) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn2) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn4) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn5) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn6) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn7) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn8) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn9) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn10) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn13) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69370_e105901;
        locals.var_t2_dn0 = assign69370_e105901_d_n0;
        locals.var_t2_dn2 = assign69370_e105901_d_n2;
        locals.var_t2_dn4 = assign69370_e105901_d_n4;
        locals.var_t2_dn5 = assign69370_e105901_d_n5;
        locals.var_t2_dn6 = assign69370_e105901_d_n6;
        locals.var_t2_dn7 = assign69370_e105901_d_n7;
        locals.var_t2_dn8 = assign69370_e105901_d_n8;
        locals.var_t2_dn9 = assign69370_e105901_d_n9;
        locals.var_t2_dn10 = assign69370_e105901_d_n10;
        locals.var_t2_dn13 = assign69370_e105901_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign69390_e105921, assign69390_e105921_d_n5, assign69390_e105921_d_n7, assign69390_e105921_d_n8,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69390_e105919: f64 = (-locals.var_vbs);
        (assign69390_e105919, (-locals.var_vbs_dn5), (-locals.var_vbs_dn7), (-locals.var_vbs_dn8),)
    } else {
        (locals.var_vsb, locals.var_vsb_dn5, locals.var_vsb_dn7, locals.var_vsb_dn8,)
    }
};
        locals.var_vsb = assign69390_e105921;
        locals.var_vsb_dn5 = assign69390_e105921_d_n5;
        locals.var_vsb_dn7 = assign69390_e105921_d_n7;
        locals.var_vsb_dn8 = assign69390_e105921_d_n8;
        locals.var_vsb_rv = 0.0;

        let assign69400_e105924: f64 = if locals.var_vsb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1622 = assign69400_e105924;
        locals.var_guard1622_rv = 0.0;

        let (assign69410_e105933, assign69410_e105933_d_n0, assign69410_e105933_d_n2, assign69410_e105933_d_n4, assign69410_e105933_d_n5, assign69410_e105933_d_n6, assign69410_e105933_d_n7, assign69410_e105933_d_n8, assign69410_e105933_d_n9, assign69410_e105933_d_n10, assign69410_e105933_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69410_e105931: f64 = (locals.var_vsb * locals.var_vsb);
        (assign69410_e105931, 0.0, 0.0, 0.0, ((locals.var_vsb_dn5 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn5)), 0.0, ((locals.var_vsb_dn7 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn7)), ((locals.var_vsb_dn8 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn8)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69410_e105933;
        locals.var_t2_dn0 = assign69410_e105933_d_n0;
        locals.var_t2_dn2 = assign69410_e105933_d_n2;
        locals.var_t2_dn4 = assign69410_e105933_d_n4;
        locals.var_t2_dn5 = assign69410_e105933_d_n5;
        locals.var_t2_dn6 = assign69410_e105933_d_n6;
        locals.var_t2_dn7 = assign69410_e105933_d_n7;
        locals.var_t2_dn8 = assign69410_e105933_d_n8;
        locals.var_t2_dn9 = assign69410_e105933_d_n9;
        locals.var_t2_dn10 = assign69410_e105933_d_n10;
        locals.var_t2_dn13 = assign69410_e105933_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign69420_e105942, assign69420_e105942_d_n0, assign69420_e105942_d_n2, assign69420_e105942_d_n4, assign69420_e105942_d_n5, assign69420_e105942_d_n6, assign69420_e105942_d_n7, assign69420_e105942_d_n8, assign69420_e105942_d_n9, assign69420_e105942_d_n10, assign69420_e105942_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69420_e105940: f64 = (locals.var_t2 * locals.var_vsb);
        (assign69420_e105940, (locals.var_t2_dn0 * locals.var_vsb), (locals.var_t2_dn2 * locals.var_vsb), (locals.var_t2_dn4 * locals.var_vsb), ((locals.var_t2_dn5 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn5)), (locals.var_t2_dn6 * locals.var_vsb), ((locals.var_t2_dn7 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn7)), ((locals.var_t2_dn8 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn8)), (locals.var_t2_dn9 * locals.var_vsb), (locals.var_t2_dn10 * locals.var_vsb), (locals.var_t2_dn13 * locals.var_vsb),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign69420_e105942;
        locals.var_t4_dn0 = assign69420_e105942_d_n0;
        locals.var_t4_dn2 = assign69420_e105942_d_n2;
        locals.var_t4_dn4 = assign69420_e105942_d_n4;
        locals.var_t4_dn5 = assign69420_e105942_d_n5;
        locals.var_t4_dn6 = assign69420_e105942_d_n6;
        locals.var_t4_dn7 = assign69420_e105942_d_n7;
        locals.var_t4_dn8 = assign69420_e105942_d_n8;
        locals.var_t4_dn9 = assign69420_e105942_d_n9;
        locals.var_t4_dn10 = assign69420_e105942_d_n10;
        locals.var_t4_dn13 = assign69420_e105942_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign69430_e105951, assign69430_e105951_d_n0, assign69430_e105951_d_n2, assign69430_e105951_d_n4, assign69430_e105951_d_n5, assign69430_e105951_d_n6, assign69430_e105951_d_n7, assign69430_e105951_d_n8, assign69430_e105951_d_n9, assign69430_e105951_d_n10, assign69430_e105951_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69430_e105949: f64 = (locals.var_t4 + 0.5);
        (assign69430_e105949, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign69430_e105951;
        locals.var_t0_dn0 = assign69430_e105951_d_n0;
        locals.var_t0_dn2 = assign69430_e105951_d_n2;
        locals.var_t0_dn4 = assign69430_e105951_d_n4;
        locals.var_t0_dn5 = assign69430_e105951_d_n5;
        locals.var_t0_dn6 = assign69430_e105951_d_n6;
        locals.var_t0_dn7 = assign69430_e105951_d_n7;
        locals.var_t0_dn8 = assign69430_e105951_d_n8;
        locals.var_t0_dn9 = assign69430_e105951_d_n9;
        locals.var_t0_dn10 = assign69430_e105951_d_n10;
        locals.var_t0_dn13 = assign69430_e105951_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign69440_e105960, assign69440_e105960_d_n0, assign69440_e105960_d_n2, assign69440_e105960_d_n4, assign69440_e105960_d_n5, assign69440_e105960_d_n6, assign69440_e105960_d_n7, assign69440_e105960_d_n8, assign69440_e105960_d_n9, assign69440_e105960_d_n10, assign69440_e105960_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69440_e105958: f64 = (locals.var_t4 / locals.var_t0);
        (assign69440_e105958, (((locals.var_t4_dn0 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn2 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn4 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn5 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn6 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn7 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn8 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn9 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn10 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn13 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign69440_e105960;
        locals.var_t5_dn0 = assign69440_e105960_d_n0;
        locals.var_t5_dn2 = assign69440_e105960_d_n2;
        locals.var_t5_dn4 = assign69440_e105960_d_n4;
        locals.var_t5_dn5 = assign69440_e105960_d_n5;
        locals.var_t5_dn6 = assign69440_e105960_d_n6;
        locals.var_t5_dn7 = assign69440_e105960_d_n7;
        locals.var_t5_dn8 = assign69440_e105960_d_n8;
        locals.var_t5_dn9 = assign69440_e105960_d_n9;
        locals.var_t5_dn10 = assign69440_e105960_d_n10;
        locals.var_t5_dn13 = assign69440_e105960_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign69450_e105981, assign69450_e105981_d_n0, assign69450_e105981_d_n2, assign69450_e105981_d_n4, assign69450_e105981_d_n5, assign69450_e105981_d_n6, assign69450_e105981_d_n7, assign69450_e105981_d_n8, assign69450_e105981_d_n9, assign69450_e105981_d_n10, assign69450_e105981_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69450_e105967: f64 = (3.0 * locals.var_t2);
        let assign69450_e105969: f64 = (assign69450_e105967 * locals.var_t0);
        let assign69450_e105972: f64 = (locals.var_t4 * 3.0);
        let assign69450_e105974: f64 = (assign69450_e105972 * locals.var_t2);
        let assign69450_e105975: f64 = (assign69450_e105969 - assign69450_e105974);
        let assign69450_e105978: f64 = (locals.var_t0 * locals.var_t0);
        let assign69450_e105979: f64 = (assign69450_e105975 / assign69450_e105978);
        (assign69450_e105979, (((((((3.0 * locals.var_t2_dn0) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn0)) - (((locals.var_t4_dn0 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn0))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn2) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn2)) - (((locals.var_t4_dn2 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn2))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn4) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn4)) - (((locals.var_t4_dn4 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn4))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn5) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn5)) - (((locals.var_t4_dn5 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn5))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn6) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn6)) - (((locals.var_t4_dn6 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn6))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn7) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn7)) - (((locals.var_t4_dn7 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn7))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn8) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn8)) - (((locals.var_t4_dn8 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn8))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn9) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn9)) - (((locals.var_t4_dn9 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn9))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn10) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn10)) - (((locals.var_t4_dn10 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn10))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn13) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn13)) - (((locals.var_t4_dn13 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn13))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)))) / (assign69450_e105978 * assign69450_e105978)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign69450_e105981;
        locals.var_t7_dn0 = assign69450_e105981_d_n0;
        locals.var_t7_dn2 = assign69450_e105981_d_n2;
        locals.var_t7_dn4 = assign69450_e105981_d_n4;
        locals.var_t7_dn5 = assign69450_e105981_d_n5;
        locals.var_t7_dn6 = assign69450_e105981_d_n6;
        locals.var_t7_dn7 = assign69450_e105981_d_n7;
        locals.var_t7_dn8 = assign69450_e105981_d_n8;
        locals.var_t7_dn9 = assign69450_e105981_d_n9;
        locals.var_t7_dn10 = assign69450_e105981_d_n10;
        locals.var_t7_dn13 = assign69450_e105981_d_n13;
        locals.var_t7_rv = 0.0;

        locals.var_flg_coovlps = 0.0;
        locals.var_flg_coovlps_rv = 0.0;

        locals.var_flg_coovlp = 0.0;
        locals.var_flg_coovlp_rv = 0.0;

        locals.var_flg_calcqover = 0.0;
        locals.var_flg_calcqover_rv = 0.0;

        locals.var_flg_never_reach_vfbover = 0.0;
        locals.var_flg_never_reach_vfbover_rv = 0.0;

        locals.var_flg_calcqover = 0.0;
        locals.var_flg_calcqover_rv = 0.0;

        let assign69540_e106007: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1627 = assign69540_e106007;
        locals.var_guard1627_rv = 0.0;

        let assign69550_e106010: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1628 = assign69550_e106010;
        locals.var_guard1628_rv = 0.0;

        let assign69560_e106013: f64 = if 1.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1629 = assign69560_e106013;
        locals.var_guard1629_rv = 0.0;

        let assign69570_e106016: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1630 = assign69570_e106016;
        locals.var_guard1630_rv = 0.0;

        let assign69580_e106027: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1631 = assign69580_e106027;
        locals.var_guard1631_rv = 0.0;

        let (assign69590_e106033,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69590_e106033;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign69600_e106039,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign69600_e106039;
        locals.var_flg_coovlps_rv = 0.0;

        let (assign69610_e106047, assign69610_e106047_d_n2, assign69610_e106047_d_n6, assign69610_e106047_d_n7, assign69610_e106047_d_n8,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        let assign69610_e106045: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign69610_e106045, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign69610_e106047;
        locals.var_vgbgmt_dn2 = assign69610_e106047_d_n2;
        locals.var_vgbgmt_dn6 = assign69610_e106047_d_n6;
        locals.var_vgbgmt_dn7 = assign69610_e106047_d_n7;
        locals.var_vgbgmt_dn8 = assign69610_e106047_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign69620_e106054, assign69620_e106054_d_n0, assign69620_e106054_d_n2, assign69620_e106054_d_n4, assign69620_e106054_d_n5, assign69620_e106054_d_n6, assign69620_e106054_d_n7, assign69620_e106054_d_n8, assign69620_e106054_d_n9, assign69620_e106054_d_n10, assign69620_e106054_d_n13,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        let assign69620_e106052: f64 = (-locals.var_vbsi);
        (assign69620_e106052, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign69620_e106054;
        locals.var_vxbgmt_dn0 = assign69620_e106054_d_n0;
        locals.var_vxbgmt_dn2 = assign69620_e106054_d_n2;
        locals.var_vxbgmt_dn4 = assign69620_e106054_d_n4;
        locals.var_vxbgmt_dn5 = assign69620_e106054_d_n5;
        locals.var_vxbgmt_dn6 = assign69620_e106054_d_n6;
        locals.var_vxbgmt_dn7 = assign69620_e106054_d_n7;
        locals.var_vxbgmt_dn8 = assign69620_e106054_d_n8;
        locals.var_vxbgmt_dn9 = assign69620_e106054_d_n9;
        locals.var_vxbgmt_dn10 = assign69620_e106054_d_n10;
        locals.var_vxbgmt_dn13 = assign69620_e106054_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign69630_e106060,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign69630_e106060;
        locals.var_nover_func_rv = 0.0;

        let (assign69640_e106066, assign69640_e106066_d_n0, assign69640_e106066_d_n2, assign69640_e106066_d_n4, assign69640_e106066_d_n5, assign69640_e106066_d_n6, assign69640_e106066_d_n7, assign69640_e106066_d_n8, assign69640_e106066_d_n9, assign69640_e106066_d_n10, assign69640_e106066_d_n13,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign69640_e106066;
        locals.var_lover_func_dn0 = assign69640_e106066_d_n0;
        locals.var_lover_func_dn2 = assign69640_e106066_d_n2;
        locals.var_lover_func_dn4 = assign69640_e106066_d_n4;
        locals.var_lover_func_dn5 = assign69640_e106066_d_n5;
        locals.var_lover_func_dn6 = assign69640_e106066_d_n6;
        locals.var_lover_func_dn7 = assign69640_e106066_d_n7;
        locals.var_lover_func_dn8 = assign69640_e106066_d_n8;
        locals.var_lover_func_dn9 = assign69640_e106066_d_n9;
        locals.var_lover_func_dn10 = assign69640_e106066_d_n10;
        locals.var_lover_func_dn13 = assign69640_e106066_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign69650_e106072, assign69650_e106072_d_n0, assign69650_e106072_d_n2, assign69650_e106072_d_n4, assign69650_e106072_d_n5, assign69650_e106072_d_n6, assign69650_e106072_d_n7, assign69650_e106072_d_n8, assign69650_e106072_d_n9, assign69650_e106072_d_n10, assign69650_e106072_d_n13,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign69650_e106072;
        locals.var_wdep_func_dn0 = assign69650_e106072_d_n0;
        locals.var_wdep_func_dn2 = assign69650_e106072_d_n2;
        locals.var_wdep_func_dn4 = assign69650_e106072_d_n4;
        locals.var_wdep_func_dn5 = assign69650_e106072_d_n5;
        locals.var_wdep_func_dn6 = assign69650_e106072_d_n6;
        locals.var_wdep_func_dn7 = assign69650_e106072_d_n7;
        locals.var_wdep_func_dn8 = assign69650_e106072_d_n8;
        locals.var_wdep_func_dn9 = assign69650_e106072_d_n9;
        locals.var_wdep_func_dn10 = assign69650_e106072_d_n10;
        locals.var_wdep_func_dn13 = assign69650_e106072_d_n13;
        locals.var_wdep_func_rv = 0.0;

        let (assign69660_e106078, assign69660_e106078_d_n0, assign69660_e106078_d_n2, assign69660_e106078_d_n4, assign69660_e106078_d_n5, assign69660_e106078_d_n6, assign69660_e106078_d_n7, assign69660_e106078_d_n8, assign69660_e106078_d_n9, assign69660_e106078_d_n10, assign69660_e106078_d_n13,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign69660_e106078;
        locals.var_cnst0over_func_dn0 = assign69660_e106078_d_n0;
        locals.var_cnst0over_func_dn2 = assign69660_e106078_d_n2;
        locals.var_cnst0over_func_dn4 = assign69660_e106078_d_n4;
        locals.var_cnst0over_func_dn5 = assign69660_e106078_d_n5;
        locals.var_cnst0over_func_dn6 = assign69660_e106078_d_n6;
        locals.var_cnst0over_func_dn7 = assign69660_e106078_d_n7;
        locals.var_cnst0over_func_dn8 = assign69660_e106078_d_n8;
        locals.var_cnst0over_func_dn9 = assign69660_e106078_d_n9;
        locals.var_cnst0over_func_dn10 = assign69660_e106078_d_n10;
        locals.var_cnst0over_func_dn13 = assign69660_e106078_d_n13;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign69670_e106084,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign69670_e106084;
        locals.var_cox0_func_rv = 0.0;

        let assign69680_e106103: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1632 = assign69680_e106103;
        locals.var_guard1632_rv = 0.0;

        let (assign69690_e106112,) = {
    if (((locals.var_guard1628 != 0.0) && (locals.var_guard1627 == 0.0)) && (locals.var_guard1632 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69690_e106112;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign69700_e106123, assign69700_e106123_d_n2, assign69700_e106123_d_n6, assign69700_e106123_d_n7, assign69700_e106123_d_n8,) = {
    if (((locals.var_guard1628 != 0.0) && (locals.var_guard1627 == 0.0)) && (locals.var_guard1632 != 0.0)) {
        let assign69700_e106121: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign69700_e106121, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign69700_e106123;
        locals.var_vgbgmt_dn2 = assign69700_e106123_d_n2;
        locals.var_vgbgmt_dn6 = assign69700_e106123_d_n6;
        locals.var_vgbgmt_dn7 = assign69700_e106123_d_n7;
        locals.var_vgbgmt_dn8 = assign69700_e106123_d_n8;
        locals.var_vgbgmt_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_251(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign69710_e106133, assign69710_e106133_d_n0, assign69710_e106133_d_n2, assign69710_e106133_d_n4, assign69710_e106133_d_n5, assign69710_e106133_d_n6, assign69710_e106133_d_n7, assign69710_e106133_d_n8, assign69710_e106133_d_n9, assign69710_e106133_d_n10, assign69710_e106133_d_n13,) = {
    if (((locals.var_guard1628 != 0.0) && (locals.var_guard1627 == 0.0)) && (locals.var_guard1632 != 0.0)) {
        let assign69710_e106131: f64 = (-locals.var_vbsei);
        (assign69710_e106131, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign69710_e106133;
        locals.var_vxbgmt_dn0 = assign69710_e106133_d_n0;
        locals.var_vxbgmt_dn2 = assign69710_e106133_d_n2;
        locals.var_vxbgmt_dn4 = assign69710_e106133_d_n4;
        locals.var_vxbgmt_dn5 = assign69710_e106133_d_n5;
        locals.var_vxbgmt_dn6 = assign69710_e106133_d_n6;
        locals.var_vxbgmt_dn7 = assign69710_e106133_d_n7;
        locals.var_vxbgmt_dn8 = assign69710_e106133_d_n8;
        locals.var_vxbgmt_dn9 = assign69710_e106133_d_n9;
        locals.var_vxbgmt_dn10 = assign69710_e106133_d_n10;
        locals.var_vxbgmt_dn13 = assign69710_e106133_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let assign69720_e106144: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1633 = assign69720_e106144;
        locals.var_guard1633_rv = 0.0;

        let (assign69730_e106155,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69730_e106155;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign69740_e106166,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign69740_e106166;
        locals.var_flg_coovlp_rv = 0.0;

        let (assign69750_e106179, assign69750_e106179_d_n2, assign69750_e106179_d_n6, assign69750_e106179_d_n7, assign69750_e106179_d_n8,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        let assign69750_e106177: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign69750_e106177, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign69750_e106179;
        locals.var_vgbgmt_dn2 = assign69750_e106179_d_n2;
        locals.var_vgbgmt_dn6 = assign69750_e106179_d_n6;
        locals.var_vgbgmt_dn7 = assign69750_e106179_d_n7;
        locals.var_vgbgmt_dn8 = assign69750_e106179_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign69760_e106192, assign69760_e106192_d_n0, assign69760_e106192_d_n2, assign69760_e106192_d_n4, assign69760_e106192_d_n5, assign69760_e106192_d_n6, assign69760_e106192_d_n7, assign69760_e106192_d_n8, assign69760_e106192_d_n9, assign69760_e106192_d_n10, assign69760_e106192_d_n13,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        let assign69760_e106190: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign69760_e106190, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, (locals.var_vdsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign69760_e106192;
        locals.var_vxbgmt_dn0 = assign69760_e106192_d_n0;
        locals.var_vxbgmt_dn2 = assign69760_e106192_d_n2;
        locals.var_vxbgmt_dn4 = assign69760_e106192_d_n4;
        locals.var_vxbgmt_dn5 = assign69760_e106192_d_n5;
        locals.var_vxbgmt_dn6 = assign69760_e106192_d_n6;
        locals.var_vxbgmt_dn7 = assign69760_e106192_d_n7;
        locals.var_vxbgmt_dn8 = assign69760_e106192_d_n8;
        locals.var_vxbgmt_dn9 = assign69760_e106192_d_n9;
        locals.var_vxbgmt_dn10 = assign69760_e106192_d_n10;
        locals.var_vxbgmt_dn13 = assign69760_e106192_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign69770_e106203,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign69770_e106203;
        locals.var_nover_func_rv = 0.0;

        let (assign69780_e106218, assign69780_e106218_d_n0, assign69780_e106218_d_n2, assign69780_e106218_d_n4, assign69780_e106218_d_n5, assign69780_e106218_d_n6, assign69780_e106218_d_n7, assign69780_e106218_d_n8, assign69780_e106218_d_n9, assign69780_e106218_d_n10, assign69780_e106218_d_n13,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        let assign69780_e106215: f64 = (p.p64 * p.p55);
        let assign69780_e106216: f64 = (p.p63 + assign69780_e106215);
        (assign69780_e106216, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign69780_e106218;
        locals.var_lover_func_dn0 = assign69780_e106218_d_n0;
        locals.var_lover_func_dn2 = assign69780_e106218_d_n2;
        locals.var_lover_func_dn4 = assign69780_e106218_d_n4;
        locals.var_lover_func_dn5 = assign69780_e106218_d_n5;
        locals.var_lover_func_dn6 = assign69780_e106218_d_n6;
        locals.var_lover_func_dn7 = assign69780_e106218_d_n7;
        locals.var_lover_func_dn8 = assign69780_e106218_d_n8;
        locals.var_lover_func_dn9 = assign69780_e106218_d_n9;
        locals.var_lover_func_dn10 = assign69780_e106218_d_n10;
        locals.var_lover_func_dn13 = assign69780_e106218_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign69790_e106229, assign69790_e106229_d_n0, assign69790_e106229_d_n2, assign69790_e106229_d_n4, assign69790_e106229_d_n5, assign69790_e106229_d_n6, assign69790_e106229_d_n7, assign69790_e106229_d_n8, assign69790_e106229_d_n9, assign69790_e106229_d_n10, assign69790_e106229_d_n13,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign69790_e106229;
        locals.var_wdep_func_dn0 = assign69790_e106229_d_n0;
        locals.var_wdep_func_dn2 = assign69790_e106229_d_n2;
        locals.var_wdep_func_dn4 = assign69790_e106229_d_n4;
        locals.var_wdep_func_dn5 = assign69790_e106229_d_n5;
        locals.var_wdep_func_dn6 = assign69790_e106229_d_n6;
        locals.var_wdep_func_dn7 = assign69790_e106229_d_n7;
        locals.var_wdep_func_dn8 = assign69790_e106229_d_n8;
        locals.var_wdep_func_dn9 = assign69790_e106229_d_n9;
        locals.var_wdep_func_dn10 = assign69790_e106229_d_n10;
        locals.var_wdep_func_dn13 = assign69790_e106229_d_n13;
        locals.var_wdep_func_rv = 0.0;

        let (assign69800_e106240, assign69800_e106240_d_n0, assign69800_e106240_d_n2, assign69800_e106240_d_n4, assign69800_e106240_d_n5, assign69800_e106240_d_n6, assign69800_e106240_d_n7, assign69800_e106240_d_n8, assign69800_e106240_d_n9, assign69800_e106240_d_n10, assign69800_e106240_d_n13,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign69800_e106240;
        locals.var_cnst0over_func_dn0 = assign69800_e106240_d_n0;
        locals.var_cnst0over_func_dn2 = assign69800_e106240_d_n2;
        locals.var_cnst0over_func_dn4 = assign69800_e106240_d_n4;
        locals.var_cnst0over_func_dn5 = assign69800_e106240_d_n5;
        locals.var_cnst0over_func_dn6 = assign69800_e106240_d_n6;
        locals.var_cnst0over_func_dn7 = assign69800_e106240_d_n7;
        locals.var_cnst0over_func_dn8 = assign69800_e106240_d_n8;
        locals.var_cnst0over_func_dn9 = assign69800_e106240_d_n9;
        locals.var_cnst0over_func_dn10 = assign69800_e106240_d_n10;
        locals.var_cnst0over_func_dn13 = assign69800_e106240_d_n13;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign69810_e106251,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign69810_e106251;
        locals.var_cox0_func_rv = 0.0;

        let (assign69820_e106263, assign69820_e106263_d_n0, assign69820_e106263_d_n2, assign69820_e106263_d_n4, assign69820_e106263_d_n5, assign69820_e106263_d_n6, assign69820_e106263_d_n7, assign69820_e106263_d_n8, assign69820_e106263_d_n9, assign69820_e106263_d_n10, assign69820_e106263_d_n13,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        let assign69820_e106261: f64 = (-locals.var_lover_func);
        (assign69820_e106261, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign69820_e106263;
        locals.var_lover_func_dn0 = assign69820_e106263_d_n0;
        locals.var_lover_func_dn2 = assign69820_e106263_d_n2;
        locals.var_lover_func_dn4 = assign69820_e106263_d_n4;
        locals.var_lover_func_dn5 = assign69820_e106263_d_n5;
        locals.var_lover_func_dn6 = assign69820_e106263_d_n6;
        locals.var_lover_func_dn7 = assign69820_e106263_d_n7;
        locals.var_lover_func_dn8 = assign69820_e106263_d_n8;
        locals.var_lover_func_dn9 = assign69820_e106263_d_n9;
        locals.var_lover_func_dn10 = assign69820_e106263_d_n10;
        locals.var_lover_func_dn13 = assign69820_e106263_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign69830_e106274: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1634 = assign69830_e106274;
        locals.var_guard1634_rv = 0.0;

        let (assign69840_e106288, assign69840_e106288_d_n0, assign69840_e106288_d_n2, assign69840_e106288_d_n4, assign69840_e106288_d_n5, assign69840_e106288_d_n6, assign69840_e106288_d_n7, assign69840_e106288_d_n8, assign69840_e106288_d_n9, assign69840_e106288_d_n10, assign69840_e106288_d_n13,) = {
    if ((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) {
        let assign69840_e106286: f64 = (-locals.var_lover_func);
        (assign69840_e106286, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign69840_e106288;
        locals.var_lover_func_dn0 = assign69840_e106288_d_n0;
        locals.var_lover_func_dn2 = assign69840_e106288_d_n2;
        locals.var_lover_func_dn4 = assign69840_e106288_d_n4;
        locals.var_lover_func_dn5 = assign69840_e106288_d_n5;
        locals.var_lover_func_dn6 = assign69840_e106288_d_n6;
        locals.var_lover_func_dn7 = assign69840_e106288_d_n7;
        locals.var_lover_func_dn8 = assign69840_e106288_d_n8;
        locals.var_lover_func_dn9 = assign69840_e106288_d_n9;
        locals.var_lover_func_dn10 = assign69840_e106288_d_n10;
        locals.var_lover_func_dn13 = assign69840_e106288_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign69850_e106301, assign69850_e106301_d_n0, assign69850_e106301_d_n2, assign69850_e106301_d_n4, assign69850_e106301_d_n5, assign69850_e106301_d_n6, assign69850_e106301_d_n7, assign69850_e106301_d_n8, assign69850_e106301_d_n9, assign69850_e106301_d_n10, assign69850_e106301_d_n13,) = {
    if ((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign69850_e106301;
        locals.var_t1_dn0 = assign69850_e106301_d_n0;
        locals.var_t1_dn2 = assign69850_e106301_d_n2;
        locals.var_t1_dn4 = assign69850_e106301_d_n4;
        locals.var_t1_dn5 = assign69850_e106301_d_n5;
        locals.var_t1_dn6 = assign69850_e106301_d_n6;
        locals.var_t1_dn7 = assign69850_e106301_d_n7;
        locals.var_t1_dn8 = assign69850_e106301_d_n8;
        locals.var_t1_dn9 = assign69850_e106301_d_n9;
        locals.var_t1_dn10 = assign69850_e106301_d_n10;
        locals.var_t1_dn13 = assign69850_e106301_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign69860_e106320, assign69860_e106320_d_n0, assign69860_e106320_d_n2, assign69860_e106320_d_n4, assign69860_e106320_d_n5, assign69860_e106320_d_n6, assign69860_e106320_d_n7, assign69860_e106320_d_n8, assign69860_e106320_d_n9, assign69860_e106320_d_n10, assign69860_e106320_d_n13,) = {
    if ((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) {
        let assign69860_e106314: f64 = (locals.var_t1 * locals.var_t1);
        let assign69860_e106316: f64 = (assign69860_e106314 / locals.var_kjunc);
        let assign69860_e106318: f64 = (assign69860_e106316 - p.p137);
        (assign69860_e106318, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn13)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn13,)
    }
};
        locals.var_vxb_lim = assign69860_e106320;
        locals.var_vxb_lim_dn0 = assign69860_e106320_d_n0;
        locals.var_vxb_lim_dn2 = assign69860_e106320_d_n2;
        locals.var_vxb_lim_dn4 = assign69860_e106320_d_n4;
        locals.var_vxb_lim_dn5 = assign69860_e106320_d_n5;
        locals.var_vxb_lim_dn6 = assign69860_e106320_d_n6;
        locals.var_vxb_lim_dn7 = assign69860_e106320_d_n7;
        locals.var_vxb_lim_dn8 = assign69860_e106320_d_n8;
        locals.var_vxb_lim_dn9 = assign69860_e106320_d_n9;
        locals.var_vxb_lim_dn10 = assign69860_e106320_d_n10;
        locals.var_vxb_lim_dn13 = assign69860_e106320_d_n13;
        locals.var_vxb_lim_rv = 0.0;

        let assign69870_e106323: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1635 = assign69870_e106323;
        locals.var_guard1635_rv = 0.0;

        let assign69880_e106330: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1636 = assign69880_e106330;
        locals.var_guard1636_rv = 0.0;

        let (assign69890_e106347, assign69890_e106347_d_n0, assign69890_e106347_d_n2, assign69890_e106347_d_n4, assign69890_e106347_d_n5, assign69890_e106347_d_n6, assign69890_e106347_d_n7, assign69890_e106347_d_n8, assign69890_e106347_d_n9, assign69890_e106347_d_n10, assign69890_e106347_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign69890_e106347;
        locals.var_vxbgmt_dn0 = assign69890_e106347_d_n0;
        locals.var_vxbgmt_dn2 = assign69890_e106347_d_n2;
        locals.var_vxbgmt_dn4 = assign69890_e106347_d_n4;
        locals.var_vxbgmt_dn5 = assign69890_e106347_d_n5;
        locals.var_vxbgmt_dn6 = assign69890_e106347_d_n6;
        locals.var_vxbgmt_dn7 = assign69890_e106347_d_n7;
        locals.var_vxbgmt_dn8 = assign69890_e106347_d_n8;
        locals.var_vxbgmt_dn9 = assign69890_e106347_d_n9;
        locals.var_vxbgmt_dn10 = assign69890_e106347_d_n10;
        locals.var_vxbgmt_dn13 = assign69890_e106347_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign69900_e106371, assign69900_e106371_d_n0, assign69900_e106371_d_n2, assign69900_e106371_d_n4, assign69900_e106371_d_n5, assign69900_e106371_d_n6, assign69900_e106371_d_n7, assign69900_e106371_d_n8, assign69900_e106371_d_n9, assign69900_e106371_d_n10, assign69900_e106371_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 == 0.0)) {
        let (assign69900_e106369,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign69900_e106367: f64 = (-1.0);
                (assign69900_e106367,)
            } else {
                (1.0,)
            }
        };
        (assign69900_e106369, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign69900_e106371;
        locals.var_tmf3_dn0 = assign69900_e106371_d_n0;
        locals.var_tmf3_dn2 = assign69900_e106371_d_n2;
        locals.var_tmf3_dn4 = assign69900_e106371_d_n4;
        locals.var_tmf3_dn5 = assign69900_e106371_d_n5;
        locals.var_tmf3_dn6 = assign69900_e106371_d_n6;
        locals.var_tmf3_dn7 = assign69900_e106371_d_n7;
        locals.var_tmf3_dn8 = assign69900_e106371_d_n8;
        locals.var_tmf3_dn9 = assign69900_e106371_d_n9;
        locals.var_tmf3_dn10 = assign69900_e106371_d_n10;
        locals.var_tmf3_dn13 = assign69900_e106371_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign69910_e106391, assign69910_e106391_d_n0, assign69910_e106391_d_n2, assign69910_e106391_d_n4, assign69910_e106391_d_n5, assign69910_e106391_d_n6, assign69910_e106391_d_n7, assign69910_e106391_d_n8, assign69910_e106391_d_n9, assign69910_e106391_d_n10, assign69910_e106391_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 == 0.0)) {
        let assign69910_e106389: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign69910_e106389, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn13 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign69910_e106391;
        locals.var_tmf4_dn0 = assign69910_e106391_d_n0;
        locals.var_tmf4_dn2 = assign69910_e106391_d_n2;
        locals.var_tmf4_dn4 = assign69910_e106391_d_n4;
        locals.var_tmf4_dn5 = assign69910_e106391_d_n5;
        locals.var_tmf4_dn6 = assign69910_e106391_d_n6;
        locals.var_tmf4_dn7 = assign69910_e106391_d_n7;
        locals.var_tmf4_dn8 = assign69910_e106391_d_n8;
        locals.var_tmf4_dn9 = assign69910_e106391_d_n9;
        locals.var_tmf4_dn10 = assign69910_e106391_d_n10;
        locals.var_tmf4_dn13 = assign69910_e106391_d_n13;
        locals.var_tmf4_rv = 0.0;

        let (assign69920_e106415, assign69920_e106415_d_n0, assign69920_e106415_d_n2, assign69920_e106415_d_n4, assign69920_e106415_d_n5, assign69920_e106415_d_n6, assign69920_e106415_d_n7, assign69920_e106415_d_n8, assign69920_e106415_d_n9, assign69920_e106415_d_n10, assign69920_e106415_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 == 0.0)) {
        let assign69920_e106410: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign69920_e106412: f64 = (assign69920_e106410).powf(p.p113);
        let assign69920_e106413: f64 = (1.0 + assign69920_e106412);
        (assign69920_e106413, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign69920_e106415;
        locals.var_tmf1_dn0 = assign69920_e106415_d_n0;
        locals.var_tmf1_dn2 = assign69920_e106415_d_n2;
        locals.var_tmf1_dn4 = assign69920_e106415_d_n4;
        locals.var_tmf1_dn5 = assign69920_e106415_d_n5;
        locals.var_tmf1_dn6 = assign69920_e106415_d_n6;
        locals.var_tmf1_dn7 = assign69920_e106415_d_n7;
        locals.var_tmf1_dn8 = assign69920_e106415_d_n8;
        locals.var_tmf1_dn9 = assign69920_e106415_d_n9;
        locals.var_tmf1_dn10 = assign69920_e106415_d_n10;
        locals.var_tmf1_dn13 = assign69920_e106415_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign69930_e106437, assign69930_e106437_d_n0, assign69930_e106437_d_n2, assign69930_e106437_d_n4, assign69930_e106437_d_n5, assign69930_e106437_d_n6, assign69930_e106437_d_n7, assign69930_e106437_d_n8, assign69930_e106437_d_n9, assign69930_e106437_d_n10, assign69930_e106437_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 == 0.0)) {
        let assign69930_e106434: f64 = (1.0 / p.p113);
        let assign69930_e106435: f64 = (locals.var_tmf1).powf(assign69930_e106434);
        (assign69930_e106435, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn13)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn13 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign69930_e106437;
        locals.var_tmf2_dn0 = assign69930_e106437_d_n0;
        locals.var_tmf2_dn2 = assign69930_e106437_d_n2;
        locals.var_tmf2_dn4 = assign69930_e106437_d_n4;
        locals.var_tmf2_dn5 = assign69930_e106437_d_n5;
        locals.var_tmf2_dn6 = assign69930_e106437_d_n6;
        locals.var_tmf2_dn7 = assign69930_e106437_d_n7;
        locals.var_tmf2_dn8 = assign69930_e106437_d_n8;
        locals.var_tmf2_dn9 = assign69930_e106437_d_n9;
        locals.var_tmf2_dn10 = assign69930_e106437_d_n10;
        locals.var_tmf2_dn13 = assign69930_e106437_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign69940_e106459, assign69940_e106459_d_n0, assign69940_e106459_d_n2, assign69940_e106459_d_n4, assign69940_e106459_d_n5, assign69940_e106459_d_n6, assign69940_e106459_d_n7, assign69940_e106459_d_n8, assign69940_e106459_d_n9, assign69940_e106459_d_n10, assign69940_e106459_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 == 0.0)) {
        let assign69940_e106455: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign69940_e106457: f64 = (assign69940_e106455 / locals.var_tmf2);
        (assign69940_e106457, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn13 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn13)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign69940_e106459;
        locals.var_vxbgmt_dn0 = assign69940_e106459_d_n0;
        locals.var_vxbgmt_dn2 = assign69940_e106459_d_n2;
        locals.var_vxbgmt_dn4 = assign69940_e106459_d_n4;
        locals.var_vxbgmt_dn5 = assign69940_e106459_d_n5;
        locals.var_vxbgmt_dn6 = assign69940_e106459_d_n6;
        locals.var_vxbgmt_dn7 = assign69940_e106459_d_n7;
        locals.var_vxbgmt_dn8 = assign69940_e106459_d_n8;
        locals.var_vxbgmt_dn9 = assign69940_e106459_d_n9;
        locals.var_vxbgmt_dn10 = assign69940_e106459_d_n10;
        locals.var_vxbgmt_dn13 = assign69940_e106459_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign69950_e106487, assign69950_e106487_d_n0, assign69950_e106487_d_n2, assign69950_e106487_d_n4, assign69950_e106487_d_n5, assign69950_e106487_d_n6, assign69950_e106487_d_n7, assign69950_e106487_d_n8, assign69950_e106487_d_n9, assign69950_e106487_d_n10, assign69950_e106487_d_n13,) = {
    if (((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) {
        let assign69950_e106474: f64 = (locals.var_vxbgmt + p.p137);
        let assign69950_e106477: f64 = (locals.var_vxbgmt + p.p137);
        let assign69950_e106478: f64 = (assign69950_e106474 * assign69950_e106477);
        let assign69950_e106481: f64 = (4.0 * 0.1);
        let assign69950_e106483: f64 = (assign69950_e106481 * 0.1);
        let assign69950_e106484: f64 = (assign69950_e106478 + assign69950_e106483);
        let assign69950_e106485: f64 = (assign69950_e106484).sqrt();
        (assign69950_e106485, (((locals.var_vxbgmt_dn0 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn0)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn2 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn2)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn4 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn4)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn5 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn5)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn6 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn6)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn7 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn7)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn8 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn8)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn9 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn9)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn10 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn10)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn13 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn13)) / (2.0 * assign69950_e106485)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign69950_e106487;
        locals.var_tmf2_dn0 = assign69950_e106487_d_n0;
        locals.var_tmf2_dn2 = assign69950_e106487_d_n2;
        locals.var_tmf2_dn4 = assign69950_e106487_d_n4;
        locals.var_tmf2_dn5 = assign69950_e106487_d_n5;
        locals.var_tmf2_dn6 = assign69950_e106487_d_n6;
        locals.var_tmf2_dn7 = assign69950_e106487_d_n7;
        locals.var_tmf2_dn8 = assign69950_e106487_d_n8;
        locals.var_tmf2_dn9 = assign69950_e106487_d_n9;
        locals.var_tmf2_dn10 = assign69950_e106487_d_n10;
        locals.var_tmf2_dn13 = assign69950_e106487_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign69960_e106510, assign69960_e106510_d_n0, assign69960_e106510_d_n2, assign69960_e106510_d_n4, assign69960_e106510_d_n5, assign69960_e106510_d_n6, assign69960_e106510_d_n7, assign69960_e106510_d_n8, assign69960_e106510_d_n9, assign69960_e106510_d_n10, assign69960_e106510_d_n13,) = {
    if (((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) {
        let assign69960_e106504: f64 = (locals.var_vxbgmt + p.p137);
        let assign69960_e106506: f64 = (assign69960_e106504 / locals.var_tmf2);
        let assign69960_e106507: f64 = (1.0 + assign69960_e106506);
        let assign69960_e106508: f64 = (0.5 * assign69960_e106507);
        (assign69960_e106508, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn13 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign69960_e106510;
        locals.var_t9_dn0 = assign69960_e106510_d_n0;
        locals.var_t9_dn2 = assign69960_e106510_d_n2;
        locals.var_t9_dn4 = assign69960_e106510_d_n4;
        locals.var_t9_dn5 = assign69960_e106510_d_n5;
        locals.var_t9_dn6 = assign69960_e106510_d_n6;
        locals.var_t9_dn7 = assign69960_e106510_d_n7;
        locals.var_t9_dn8 = assign69960_e106510_d_n8;
        locals.var_t9_dn9 = assign69960_e106510_d_n9;
        locals.var_t9_dn10 = assign69960_e106510_d_n10;
        locals.var_t9_dn13 = assign69960_e106510_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign69970_e106531, assign69970_e106531_d_n0, assign69970_e106531_d_n2, assign69970_e106531_d_n4, assign69970_e106531_d_n5, assign69970_e106531_d_n6, assign69970_e106531_d_n7, assign69970_e106531_d_n8, assign69970_e106531_d_n9, assign69970_e106531_d_n10, assign69970_e106531_d_n13,) = {
    if (((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) {
        let assign69970_e106526: f64 = (locals.var_vxbgmt + p.p137);
        let assign69970_e106528: f64 = (assign69970_e106526 + locals.var_tmf2);
        let assign69970_e106529: f64 = (0.5 * assign69970_e106528);
        (assign69970_e106529, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69970_e106531;
        locals.var_t2_dn0 = assign69970_e106531_d_n0;
        locals.var_t2_dn2 = assign69970_e106531_d_n2;
        locals.var_t2_dn4 = assign69970_e106531_d_n4;
        locals.var_t2_dn5 = assign69970_e106531_d_n5;
        locals.var_t2_dn6 = assign69970_e106531_d_n6;
        locals.var_t2_dn7 = assign69970_e106531_d_n7;
        locals.var_t2_dn8 = assign69970_e106531_d_n8;
        locals.var_t2_dn9 = assign69970_e106531_d_n9;
        locals.var_t2_dn10 = assign69970_e106531_d_n10;
        locals.var_t2_dn13 = assign69970_e106531_d_n13;
        locals.var_t2_rv = 0.0;

        let assign69980_e106534: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1637 = assign69980_e106534;
        locals.var_guard1637_rv = 0.0;

        let (assign69990_e106551, assign69990_e106551_d_n0, assign69990_e106551_d_n2, assign69990_e106551_d_n4, assign69990_e106551_d_n5, assign69990_e106551_d_n6, assign69990_e106551_d_n7, assign69990_e106551_d_n8, assign69990_e106551_d_n9, assign69990_e106551_d_n10, assign69990_e106551_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1637 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69990_e106551;
        locals.var_t2_dn0 = assign69990_e106551_d_n0;
        locals.var_t2_dn2 = assign69990_e106551_d_n2;
        locals.var_t2_dn4 = assign69990_e106551_d_n4;
        locals.var_t2_dn5 = assign69990_e106551_d_n5;
        locals.var_t2_dn6 = assign69990_e106551_d_n6;
        locals.var_t2_dn7 = assign69990_e106551_d_n7;
        locals.var_t2_dn8 = assign69990_e106551_d_n8;
        locals.var_t2_dn9 = assign69990_e106551_d_n9;
        locals.var_t2_dn10 = assign69990_e106551_d_n10;
        locals.var_t2_dn13 = assign69990_e106551_d_n13;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_252(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign70000_e106568, assign70000_e106568_d_n0, assign70000_e106568_d_n2, assign70000_e106568_d_n4, assign70000_e106568_d_n5, assign70000_e106568_d_n6, assign70000_e106568_d_n7, assign70000_e106568_d_n8, assign70000_e106568_d_n9, assign70000_e106568_d_n10, assign70000_e106568_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1637 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign70000_e106568;
        locals.var_t9_dn0 = assign70000_e106568_d_n0;
        locals.var_t9_dn2 = assign70000_e106568_d_n2;
        locals.var_t9_dn4 = assign70000_e106568_d_n4;
        locals.var_t9_dn5 = assign70000_e106568_d_n5;
        locals.var_t9_dn6 = assign70000_e106568_d_n6;
        locals.var_t9_dn7 = assign70000_e106568_d_n7;
        locals.var_t9_dn8 = assign70000_e106568_d_n8;
        locals.var_t9_dn9 = assign70000_e106568_d_n9;
        locals.var_t9_dn10 = assign70000_e106568_d_n10;
        locals.var_t9_dn13 = assign70000_e106568_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign70010_e106588, assign70010_e106588_d_n0, assign70010_e106588_d_n2, assign70010_e106588_d_n4, assign70010_e106588_d_n5, assign70010_e106588_d_n6, assign70010_e106588_d_n7, assign70010_e106588_d_n8, assign70010_e106588_d_n9, assign70010_e106588_d_n10, assign70010_e106588_d_n13,) = {
    if (((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) {
        let assign70010_e106583: f64 = (locals.var_kjunc * locals.var_t2);
        let assign70010_e106584: f64 = (assign70010_e106583).sqrt();
        let assign70010_e106586: f64 = (assign70010_e106584 * p.p432);
        (assign70010_e106586, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign70010_e106584)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign70010_e106588;
        locals.var_wjunc0_dn0 = assign70010_e106588_d_n0;
        locals.var_wjunc0_dn2 = assign70010_e106588_d_n2;
        locals.var_wjunc0_dn4 = assign70010_e106588_d_n4;
        locals.var_wjunc0_dn5 = assign70010_e106588_d_n5;
        locals.var_wjunc0_dn6 = assign70010_e106588_d_n6;
        locals.var_wjunc0_dn7 = assign70010_e106588_d_n7;
        locals.var_wjunc0_dn8 = assign70010_e106588_d_n8;
        locals.var_wjunc0_dn9 = assign70010_e106588_d_n9;
        locals.var_wjunc0_dn10 = assign70010_e106588_d_n10;
        locals.var_wjunc0_dn13 = assign70010_e106588_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign70020_e106605, assign70020_e106605_d_n0, assign70020_e106605_d_n2, assign70020_e106605_d_n4, assign70020_e106605_d_n5, assign70020_e106605_d_n6, assign70020_e106605_d_n7, assign70020_e106605_d_n8, assign70020_e106605_d_n9, assign70020_e106605_d_n10, assign70020_e106605_d_n13,) = {
    if (((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) {
        let assign70020_e106603: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign70020_e106603, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn13 - locals.var_wjunc0_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign70020_e106605;
        locals.var_lover_func_dn0 = assign70020_e106605_d_n0;
        locals.var_lover_func_dn2 = assign70020_e106605_d_n2;
        locals.var_lover_func_dn4 = assign70020_e106605_d_n4;
        locals.var_lover_func_dn5 = assign70020_e106605_d_n5;
        locals.var_lover_func_dn6 = assign70020_e106605_d_n6;
        locals.var_lover_func_dn7 = assign70020_e106605_d_n7;
        locals.var_lover_func_dn8 = assign70020_e106605_d_n8;
        locals.var_lover_func_dn9 = assign70020_e106605_d_n9;
        locals.var_lover_func_dn10 = assign70020_e106605_d_n10;
        locals.var_lover_func_dn13 = assign70020_e106605_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign70030_e106624: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1638 = assign70030_e106624;
        locals.var_guard1638_rv = 0.0;

        let (assign70040_e106637,) = {
    if (((locals.var_guard1630 != 0.0) && (!(((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)) || (locals.var_guard1629 != 0.0)))) && (locals.var_guard1638 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign70040_e106637;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign70050_e106652, assign70050_e106652_d_n2, assign70050_e106652_d_n6, assign70050_e106652_d_n7, assign70050_e106652_d_n8,) = {
    if (((locals.var_guard1630 != 0.0) && (!(((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)) || (locals.var_guard1629 != 0.0)))) && (locals.var_guard1638 != 0.0)) {
        let assign70050_e106650: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign70050_e106650, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign70050_e106652;
        locals.var_vgbgmt_dn2 = assign70050_e106652_d_n2;
        locals.var_vgbgmt_dn6 = assign70050_e106652_d_n6;
        locals.var_vgbgmt_dn7 = assign70050_e106652_d_n7;
        locals.var_vgbgmt_dn8 = assign70050_e106652_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign70060_e106667, assign70060_e106667_d_n0, assign70060_e106667_d_n2, assign70060_e106667_d_n4, assign70060_e106667_d_n5, assign70060_e106667_d_n6, assign70060_e106667_d_n7, assign70060_e106667_d_n8, assign70060_e106667_d_n9, assign70060_e106667_d_n10, assign70060_e106667_d_n13,) = {
    if (((locals.var_guard1630 != 0.0) && (!(((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)) || (locals.var_guard1629 != 0.0)))) && (locals.var_guard1638 != 0.0)) {
        let assign70060_e106665: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign70060_e106665, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign70060_e106667;
        locals.var_vxbgmt_dn0 = assign70060_e106667_d_n0;
        locals.var_vxbgmt_dn2 = assign70060_e106667_d_n2;
        locals.var_vxbgmt_dn4 = assign70060_e106667_d_n4;
        locals.var_vxbgmt_dn5 = assign70060_e106667_d_n5;
        locals.var_vxbgmt_dn6 = assign70060_e106667_d_n6;
        locals.var_vxbgmt_dn7 = assign70060_e106667_d_n7;
        locals.var_vxbgmt_dn8 = assign70060_e106667_d_n8;
        locals.var_vxbgmt_dn9 = assign70060_e106667_d_n9;
        locals.var_vxbgmt_dn10 = assign70060_e106667_d_n10;
        locals.var_vxbgmt_dn13 = assign70060_e106667_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign70070_e106671, assign70070_e106671_d_n0, assign70070_e106671_d_n2, assign70070_e106671_d_n4, assign70070_e106671_d_n5, assign70070_e106671_d_n6, assign70070_e106671_d_n7, assign70070_e106671_d_n8, assign70070_e106671_d_n9, assign70070_e106671_d_n10, assign70070_e106671_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn13,)
    }
};
        locals.var_vbs_bnd_over = assign70070_e106671;
        locals.var_vbs_bnd_over_dn0 = assign70070_e106671_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70070_e106671_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70070_e106671_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70070_e106671_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70070_e106671_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70070_e106671_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70070_e106671_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70070_e106671_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70070_e106671_d_n10;
        locals.var_vbs_bnd_over_dn13 = assign70070_e106671_d_n13;
        locals.var_vbs_bnd_over_rv = 0.0;

        let (assign70090_e106679,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign70090_e106679;
        locals.var_flg_fd_mode_rv = 0.0;

        let (assign70100_e106683, assign70100_e106683_d_n0, assign70100_e106683_d_n2, assign70100_e106683_d_n4, assign70100_e106683_d_n5, assign70100_e106683_d_n6, assign70100_e106683_d_n7, assign70100_e106683_d_n8, assign70100_e106683_d_n9, assign70100_e106683_d_n10, assign70100_e106683_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign70100_e106683;
        locals.var_fb_dn0 = assign70100_e106683_d_n0;
        locals.var_fb_dn2 = assign70100_e106683_d_n2;
        locals.var_fb_dn4 = assign70100_e106683_d_n4;
        locals.var_fb_dn5 = assign70100_e106683_d_n5;
        locals.var_fb_dn6 = assign70100_e106683_d_n6;
        locals.var_fb_dn7 = assign70100_e106683_d_n7;
        locals.var_fb_dn8 = assign70100_e106683_d_n8;
        locals.var_fb_dn9 = assign70100_e106683_d_n9;
        locals.var_fb_dn10 = assign70100_e106683_d_n10;
        locals.var_fb_dn13 = assign70100_e106683_d_n13;
        locals.var_fb_rv = 0.0;

        let (assign70110_e106687, assign70110_e106687_d_n0, assign70110_e106687_d_n2, assign70110_e106687_d_n4, assign70110_e106687_d_n5, assign70110_e106687_d_n6, assign70110_e106687_d_n7, assign70110_e106687_d_n8, assign70110_e106687_d_n9, assign70110_e106687_d_n10, assign70110_e106687_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
        locals.var_fs01 = assign70110_e106687;
        locals.var_fs01_dn0 = assign70110_e106687_d_n0;
        locals.var_fs01_dn2 = assign70110_e106687_d_n2;
        locals.var_fs01_dn4 = assign70110_e106687_d_n4;
        locals.var_fs01_dn5 = assign70110_e106687_d_n5;
        locals.var_fs01_dn6 = assign70110_e106687_d_n6;
        locals.var_fs01_dn7 = assign70110_e106687_d_n7;
        locals.var_fs01_dn8 = assign70110_e106687_d_n8;
        locals.var_fs01_dn9 = assign70110_e106687_d_n9;
        locals.var_fs01_dn10 = assign70110_e106687_d_n10;
        locals.var_fs01_dn13 = assign70110_e106687_d_n13;
        locals.var_fs01_rv = 0.0;

        let (assign70120_e106691, assign70120_e106691_d_n0, assign70120_e106691_d_n2, assign70120_e106691_d_n4, assign70120_e106691_d_n5, assign70120_e106691_d_n6, assign70120_e106691_d_n7, assign70120_e106691_d_n8, assign70120_e106691_d_n9, assign70120_e106691_d_n10, assign70120_e106691_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
        locals.var_fs02 = assign70120_e106691;
        locals.var_fs02_dn0 = assign70120_e106691_d_n0;
        locals.var_fs02_dn2 = assign70120_e106691_d_n2;
        locals.var_fs02_dn4 = assign70120_e106691_d_n4;
        locals.var_fs02_dn5 = assign70120_e106691_d_n5;
        locals.var_fs02_dn6 = assign70120_e106691_d_n6;
        locals.var_fs02_dn7 = assign70120_e106691_d_n7;
        locals.var_fs02_dn8 = assign70120_e106691_d_n8;
        locals.var_fs02_dn9 = assign70120_e106691_d_n9;
        locals.var_fs02_dn10 = assign70120_e106691_d_n10;
        locals.var_fs02_dn13 = assign70120_e106691_d_n13;
        locals.var_fs02_rv = 0.0;

        let (assign70130_e106695, assign70130_e106695_d_n0, assign70130_e106695_d_n2, assign70130_e106695_d_n4, assign70130_e106695_d_n5, assign70130_e106695_d_n6, assign70130_e106695_d_n7, assign70130_e106695_d_n8, assign70130_e106695_d_n9, assign70130_e106695_d_n10, assign70130_e106695_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
        locals.var_fs0 = assign70130_e106695;
        locals.var_fs0_dn0 = assign70130_e106695_d_n0;
        locals.var_fs0_dn2 = assign70130_e106695_d_n2;
        locals.var_fs0_dn4 = assign70130_e106695_d_n4;
        locals.var_fs0_dn5 = assign70130_e106695_d_n5;
        locals.var_fs0_dn6 = assign70130_e106695_d_n6;
        locals.var_fs0_dn7 = assign70130_e106695_d_n7;
        locals.var_fs0_dn8 = assign70130_e106695_d_n8;
        locals.var_fs0_dn9 = assign70130_e106695_d_n9;
        locals.var_fs0_dn10 = assign70130_e106695_d_n10;
        locals.var_fs0_dn13 = assign70130_e106695_d_n13;
        locals.var_fs0_rv = 0.0;

        let (assign70140_e106699, assign70140_e106699_d_n0, assign70140_e106699_d_n2, assign70140_e106699_d_n4, assign70140_e106699_d_n5, assign70140_e106699_d_n6, assign70140_e106699_d_n7, assign70140_e106699_d_n8, assign70140_e106699_d_n9, assign70140_e106699_d_n10, assign70140_e106699_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
        locals.var_dps0 = assign70140_e106699;
        locals.var_dps0_dn0 = assign70140_e106699_d_n0;
        locals.var_dps0_dn2 = assign70140_e106699_d_n2;
        locals.var_dps0_dn4 = assign70140_e106699_d_n4;
        locals.var_dps0_dn5 = assign70140_e106699_d_n5;
        locals.var_dps0_dn6 = assign70140_e106699_d_n6;
        locals.var_dps0_dn7 = assign70140_e106699_d_n7;
        locals.var_dps0_dn8 = assign70140_e106699_d_n8;
        locals.var_dps0_dn9 = assign70140_e106699_d_n9;
        locals.var_dps0_dn10 = assign70140_e106699_d_n10;
        locals.var_dps0_dn13 = assign70140_e106699_d_n13;
        locals.var_dps0_rv = 0.0;

        let (assign70150_e106703, assign70150_e106703_d_n0, assign70150_e106703_d_n2, assign70150_e106703_d_n4, assign70150_e106703_d_n5, assign70150_e106703_d_n6, assign70150_e106703_d_n7, assign70150_e106703_d_n8, assign70150_e106703_d_n9, assign70150_e106703_d_n10, assign70150_e106703_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
        locals.var_fs0_dps0 = assign70150_e106703;
        locals.var_fs0_dps0_dn0 = assign70150_e106703_d_n0;
        locals.var_fs0_dps0_dn2 = assign70150_e106703_d_n2;
        locals.var_fs0_dps0_dn4 = assign70150_e106703_d_n4;
        locals.var_fs0_dps0_dn5 = assign70150_e106703_d_n5;
        locals.var_fs0_dps0_dn6 = assign70150_e106703_d_n6;
        locals.var_fs0_dps0_dn7 = assign70150_e106703_d_n7;
        locals.var_fs0_dps0_dn8 = assign70150_e106703_d_n8;
        locals.var_fs0_dps0_dn9 = assign70150_e106703_d_n9;
        locals.var_fs0_dps0_dn10 = assign70150_e106703_d_n10;
        locals.var_fs0_dps0_dn13 = assign70150_e106703_d_n13;
        locals.var_fs0_dps0_rv = 0.0;

        let (assign70160_e106707, assign70160_e106707_d_n0, assign70160_e106707_d_n2, assign70160_e106707_d_n4, assign70160_e106707_d_n5, assign70160_e106707_d_n6, assign70160_e106707_d_n7, assign70160_e106707_d_n8, assign70160_e106707_d_n9, assign70160_e106707_d_n10, assign70160_e106707_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
        locals.var_fs02_dps0 = assign70160_e106707;
        locals.var_fs02_dps0_dn0 = assign70160_e106707_d_n0;
        locals.var_fs02_dps0_dn2 = assign70160_e106707_d_n2;
        locals.var_fs02_dps0_dn4 = assign70160_e106707_d_n4;
        locals.var_fs02_dps0_dn5 = assign70160_e106707_d_n5;
        locals.var_fs02_dps0_dn6 = assign70160_e106707_d_n6;
        locals.var_fs02_dps0_dn7 = assign70160_e106707_d_n7;
        locals.var_fs02_dps0_dn8 = assign70160_e106707_d_n8;
        locals.var_fs02_dps0_dn9 = assign70160_e106707_d_n9;
        locals.var_fs02_dps0_dn10 = assign70160_e106707_d_n10;
        locals.var_fs02_dps0_dn13 = assign70160_e106707_d_n13;
        locals.var_fs02_dps0_rv = 0.0;

        let (assign70170_e106711, assign70170_e106711_d_n0, assign70170_e106711_d_n2, assign70170_e106711_d_n4, assign70170_e106711_d_n5, assign70170_e106711_d_n6, assign70170_e106711_d_n7, assign70170_e106711_d_n8, assign70170_e106711_d_n9, assign70170_e106711_d_n10, assign70170_e106711_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
        locals.var_fb_dpss = assign70170_e106711;
        locals.var_fb_dpss_dn0 = assign70170_e106711_d_n0;
        locals.var_fb_dpss_dn2 = assign70170_e106711_d_n2;
        locals.var_fb_dpss_dn4 = assign70170_e106711_d_n4;
        locals.var_fb_dpss_dn5 = assign70170_e106711_d_n5;
        locals.var_fb_dpss_dn6 = assign70170_e106711_d_n6;
        locals.var_fb_dpss_dn7 = assign70170_e106711_d_n7;
        locals.var_fb_dpss_dn8 = assign70170_e106711_d_n8;
        locals.var_fb_dpss_dn9 = assign70170_e106711_d_n9;
        locals.var_fb_dpss_dn10 = assign70170_e106711_d_n10;
        locals.var_fb_dpss_dn13 = assign70170_e106711_d_n13;
        locals.var_fb_dpss_rv = 0.0;

        let (assign70180_e106715, assign70180_e106715_d_n0, assign70180_e106715_d_n2, assign70180_e106715_d_n4, assign70180_e106715_d_n5, assign70180_e106715_d_n6, assign70180_e106715_d_n7, assign70180_e106715_d_n8, assign70180_e106715_d_n9, assign70180_e106715_d_n10, assign70180_e106715_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
        locals.var_fs01_dps0 = assign70180_e106715;
        locals.var_fs01_dps0_dn0 = assign70180_e106715_d_n0;
        locals.var_fs01_dps0_dn2 = assign70180_e106715_d_n2;
        locals.var_fs01_dps0_dn4 = assign70180_e106715_d_n4;
        locals.var_fs01_dps0_dn5 = assign70180_e106715_d_n5;
        locals.var_fs01_dps0_dn6 = assign70180_e106715_d_n6;
        locals.var_fs01_dps0_dn7 = assign70180_e106715_d_n7;
        locals.var_fs01_dps0_dn8 = assign70180_e106715_d_n8;
        locals.var_fs01_dps0_dn9 = assign70180_e106715_d_n9;
        locals.var_fs01_dps0_dn10 = assign70180_e106715_d_n10;
        locals.var_fs01_dps0_dn13 = assign70180_e106715_d_n13;
        locals.var_fs01_dps0_rv = 0.0;

        let (assign70190_e106719, assign70190_e106719_d_n0, assign70190_e106719_d_n2, assign70190_e106719_d_n4, assign70190_e106719_d_n5, assign70190_e106719_d_n6, assign70190_e106719_d_n7, assign70190_e106719_d_n8, assign70190_e106719_d_n9, assign70190_e106719_d_n10, assign70190_e106719_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign70190_e106719;
        locals.var_chi_1_dn0 = assign70190_e106719_d_n0;
        locals.var_chi_1_dn2 = assign70190_e106719_d_n2;
        locals.var_chi_1_dn4 = assign70190_e106719_d_n4;
        locals.var_chi_1_dn5 = assign70190_e106719_d_n5;
        locals.var_chi_1_dn6 = assign70190_e106719_d_n6;
        locals.var_chi_1_dn7 = assign70190_e106719_d_n7;
        locals.var_chi_1_dn8 = assign70190_e106719_d_n8;
        locals.var_chi_1_dn9 = assign70190_e106719_d_n9;
        locals.var_chi_1_dn10 = assign70190_e106719_d_n10;
        locals.var_chi_1_dn13 = assign70190_e106719_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign70200_e106723, assign70200_e106723_d_n0, assign70200_e106723_d_n2, assign70200_e106723_d_n4, assign70200_e106723_d_n5, assign70200_e106723_d_n6, assign70200_e106723_d_n7, assign70200_e106723_d_n8, assign70200_e106723_d_n9, assign70200_e106723_d_n10, assign70200_e106723_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign70200_e106723;
        locals.var_chi_a_dn0 = assign70200_e106723_d_n0;
        locals.var_chi_a_dn2 = assign70200_e106723_d_n2;
        locals.var_chi_a_dn4 = assign70200_e106723_d_n4;
        locals.var_chi_a_dn5 = assign70200_e106723_d_n5;
        locals.var_chi_a_dn6 = assign70200_e106723_d_n6;
        locals.var_chi_a_dn7 = assign70200_e106723_d_n7;
        locals.var_chi_a_dn8 = assign70200_e106723_d_n8;
        locals.var_chi_a_dn9 = assign70200_e106723_d_n9;
        locals.var_chi_a_dn10 = assign70200_e106723_d_n10;
        locals.var_chi_a_dn13 = assign70200_e106723_d_n13;
        locals.var_chi_a_rv = 0.0;

        let (assign70210_e106727, assign70210_e106727_d_n0, assign70210_e106727_d_n2, assign70210_e106727_d_n4, assign70210_e106727_d_n5, assign70210_e106727_d_n6, assign70210_e106727_d_n7, assign70210_e106727_d_n8, assign70210_e106727_d_n9, assign70210_e106727_d_n10, assign70210_e106727_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign70210_e106727;
        locals.var_chi_b_dn0 = assign70210_e106727_d_n0;
        locals.var_chi_b_dn2 = assign70210_e106727_d_n2;
        locals.var_chi_b_dn4 = assign70210_e106727_d_n4;
        locals.var_chi_b_dn5 = assign70210_e106727_d_n5;
        locals.var_chi_b_dn6 = assign70210_e106727_d_n6;
        locals.var_chi_b_dn7 = assign70210_e106727_d_n7;
        locals.var_chi_b_dn8 = assign70210_e106727_d_n8;
        locals.var_chi_b_dn9 = assign70210_e106727_d_n9;
        locals.var_chi_b_dn10 = assign70210_e106727_d_n10;
        locals.var_chi_b_dn13 = assign70210_e106727_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign70220_e106732,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70220_e106730: f64 = (-1.0);
        (assign70220_e106730,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign70220_e106732;
        locals.var_flg_conv_rv = 0.0;

        let (assign70230_e106736, assign70230_e106736_d_n0, assign70230_e106736_d_n2, assign70230_e106736_d_n4, assign70230_e106736_d_n5, assign70230_e106736_d_n6, assign70230_e106736_d_n7, assign70230_e106736_d_n8, assign70230_e106736_d_n9, assign70230_e106736_d_n10, assign70230_e106736_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn13,)
    }
};
        locals.var_ps0ld_ini = assign70230_e106736;
        locals.var_ps0ld_ini_dn0 = assign70230_e106736_d_n0;
        locals.var_ps0ld_ini_dn2 = assign70230_e106736_d_n2;
        locals.var_ps0ld_ini_dn4 = assign70230_e106736_d_n4;
        locals.var_ps0ld_ini_dn5 = assign70230_e106736_d_n5;
        locals.var_ps0ld_ini_dn6 = assign70230_e106736_d_n6;
        locals.var_ps0ld_ini_dn7 = assign70230_e106736_d_n7;
        locals.var_ps0ld_ini_dn8 = assign70230_e106736_d_n8;
        locals.var_ps0ld_ini_dn9 = assign70230_e106736_d_n9;
        locals.var_ps0ld_ini_dn10 = assign70230_e106736_d_n10;
        locals.var_ps0ld_ini_dn13 = assign70230_e106736_d_n13;
        locals.var_ps0ld_ini_rv = 0.0;

        let (assign70240_e106740, assign70240_e106740_d_n0, assign70240_e106740_d_n2, assign70240_e106740_d_n4, assign70240_e106740_d_n5, assign70240_e106740_d_n6, assign70240_e106740_d_n7, assign70240_e106740_d_n8, assign70240_e106740_d_n9, assign70240_e106740_d_n10, assign70240_e106740_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq, locals.var_fbsq_dn0, locals.var_fbsq_dn2, locals.var_fbsq_dn4, locals.var_fbsq_dn5, locals.var_fbsq_dn6, locals.var_fbsq_dn7, locals.var_fbsq_dn8, locals.var_fbsq_dn9, locals.var_fbsq_dn10, locals.var_fbsq_dn13,)
    }
};
        locals.var_fbsq = assign70240_e106740;
        locals.var_fbsq_dn0 = assign70240_e106740_d_n0;
        locals.var_fbsq_dn2 = assign70240_e106740_d_n2;
        locals.var_fbsq_dn4 = assign70240_e106740_d_n4;
        locals.var_fbsq_dn5 = assign70240_e106740_d_n5;
        locals.var_fbsq_dn6 = assign70240_e106740_d_n6;
        locals.var_fbsq_dn7 = assign70240_e106740_d_n7;
        locals.var_fbsq_dn8 = assign70240_e106740_d_n8;
        locals.var_fbsq_dn9 = assign70240_e106740_d_n9;
        locals.var_fbsq_dn10 = assign70240_e106740_d_n10;
        locals.var_fbsq_dn13 = assign70240_e106740_d_n13;
        locals.var_fbsq_rv = 0.0;

        let (assign70250_e106751, assign70250_e106751_d_n0, assign70250_e106751_d_n2, assign70250_e106751_d_n4, assign70250_e106751_d_n5, assign70250_e106751_d_n6, assign70250_e106751_d_n7, assign70250_e106751_d_n8, assign70250_e106751_d_n9, assign70250_e106751_d_n10, assign70250_e106751_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70250_e106744: f64 = (2.0 * locals.var_beta_inv);
        let assign70250_e106747: f64 = (locals.var_nover_func / locals.var_nin);
        let assign70250_e106748: f64 = (assign70250_e106747).ln();
        let assign70250_e106749: f64 = (assign70250_e106744 * assign70250_e106748);
        (assign70250_e106749, (((2.0 * locals.var_beta_inv_dn0) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn2) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn4) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn5) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn6) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn7) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn8) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn9) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn10) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn13) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn4, locals.var_pb2over_dn5, locals.var_pb2over_dn6, locals.var_pb2over_dn7, locals.var_pb2over_dn8, locals.var_pb2over_dn9, locals.var_pb2over_dn10, locals.var_pb2over_dn13,)
    }
};
        locals.var_pb2over = assign70250_e106751;
        locals.var_pb2over_dn0 = assign70250_e106751_d_n0;
        locals.var_pb2over_dn2 = assign70250_e106751_d_n2;
        locals.var_pb2over_dn4 = assign70250_e106751_d_n4;
        locals.var_pb2over_dn5 = assign70250_e106751_d_n5;
        locals.var_pb2over_dn6 = assign70250_e106751_d_n6;
        locals.var_pb2over_dn7 = assign70250_e106751_d_n7;
        locals.var_pb2over_dn8 = assign70250_e106751_d_n8;
        locals.var_pb2over_dn9 = assign70250_e106751_d_n9;
        locals.var_pb2over_dn10 = assign70250_e106751_d_n10;
        locals.var_pb2over_dn13 = assign70250_e106751_d_n13;
        locals.var_pb2over_rv = 0.0;

        let (assign70260_e106759, assign70260_e106759_d_n0, assign70260_e106759_d_n2, assign70260_e106759_d_n4, assign70260_e106759_d_n5, assign70260_e106759_d_n6, assign70260_e106759_d_n7, assign70260_e106759_d_n8, assign70260_e106759_d_n9, assign70260_e106759_d_n10, assign70260_e106759_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70260_e106755: f64 = (0.8 - locals.var_pb2over);
        let assign70260_e106757: f64 = (assign70260_e106755 - 0.1);
        (assign70260_e106757, (-locals.var_pb2over_dn0), (-locals.var_pb2over_dn2), (-locals.var_pb2over_dn4), (-locals.var_pb2over_dn5), (-locals.var_pb2over_dn6), (-locals.var_pb2over_dn7), (-locals.var_pb2over_dn8), (-locals.var_pb2over_dn9), (-locals.var_pb2over_dn10), (-locals.var_pb2over_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign70260_e106759;
        locals.var_tmf1_dn0 = assign70260_e106759_d_n0;
        locals.var_tmf1_dn2 = assign70260_e106759_d_n2;
        locals.var_tmf1_dn4 = assign70260_e106759_d_n4;
        locals.var_tmf1_dn5 = assign70260_e106759_d_n5;
        locals.var_tmf1_dn6 = assign70260_e106759_d_n6;
        locals.var_tmf1_dn7 = assign70260_e106759_d_n7;
        locals.var_tmf1_dn8 = assign70260_e106759_d_n8;
        locals.var_tmf1_dn9 = assign70260_e106759_d_n9;
        locals.var_tmf1_dn10 = assign70260_e106759_d_n10;
        locals.var_tmf1_dn13 = assign70260_e106759_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign70270_e106767, assign70270_e106767_d_n0, assign70270_e106767_d_n2, assign70270_e106767_d_n4, assign70270_e106767_d_n5, assign70270_e106767_d_n6, assign70270_e106767_d_n7, assign70270_e106767_d_n8, assign70270_e106767_d_n9, assign70270_e106767_d_n10, assign70270_e106767_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70270_e106763: f64 = (4.0 * 0.8);
        let assign70270_e106765: f64 = (assign70270_e106763 * 0.1);
        (assign70270_e106765, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign70270_e106767;
        locals.var_tmf2_dn0 = assign70270_e106767_d_n0;
        locals.var_tmf2_dn2 = assign70270_e106767_d_n2;
        locals.var_tmf2_dn4 = assign70270_e106767_d_n4;
        locals.var_tmf2_dn5 = assign70270_e106767_d_n5;
        locals.var_tmf2_dn6 = assign70270_e106767_d_n6;
        locals.var_tmf2_dn7 = assign70270_e106767_d_n7;
        locals.var_tmf2_dn8 = assign70270_e106767_d_n8;
        locals.var_tmf2_dn9 = assign70270_e106767_d_n9;
        locals.var_tmf2_dn10 = assign70270_e106767_d_n10;
        locals.var_tmf2_dn13 = assign70270_e106767_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_253(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign70280_e106777, assign70280_e106777_d_n0, assign70280_e106777_d_n2, assign70280_e106777_d_n4, assign70280_e106777_d_n5, assign70280_e106777_d_n6, assign70280_e106777_d_n7, assign70280_e106777_d_n8, assign70280_e106777_d_n9, assign70280_e106777_d_n10, assign70280_e106777_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign70280_e106775, assign70280_e106775_d_n0, assign70280_e106775_d_n2, assign70280_e106775_d_n4, assign70280_e106775_d_n5, assign70280_e106775_d_n6, assign70280_e106775_d_n7, assign70280_e106775_d_n8, assign70280_e106775_d_n9, assign70280_e106775_d_n10, assign70280_e106775_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign70280_e106774: f64 = (-locals.var_tmf2);
                (assign70280_e106774, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign70280_e106775, assign70280_e106775_d_n0, assign70280_e106775_d_n2, assign70280_e106775_d_n4, assign70280_e106775_d_n5, assign70280_e106775_d_n6, assign70280_e106775_d_n7, assign70280_e106775_d_n8, assign70280_e106775_d_n9, assign70280_e106775_d_n10, assign70280_e106775_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign70280_e106777;
        locals.var_tmf2_dn0 = assign70280_e106777_d_n0;
        locals.var_tmf2_dn2 = assign70280_e106777_d_n2;
        locals.var_tmf2_dn4 = assign70280_e106777_d_n4;
        locals.var_tmf2_dn5 = assign70280_e106777_d_n5;
        locals.var_tmf2_dn6 = assign70280_e106777_d_n6;
        locals.var_tmf2_dn7 = assign70280_e106777_d_n7;
        locals.var_tmf2_dn8 = assign70280_e106777_d_n8;
        locals.var_tmf2_dn9 = assign70280_e106777_d_n9;
        locals.var_tmf2_dn10 = assign70280_e106777_d_n10;
        locals.var_tmf2_dn13 = assign70280_e106777_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign70290_e106786, assign70290_e106786_d_n0, assign70290_e106786_d_n2, assign70290_e106786_d_n4, assign70290_e106786_d_n5, assign70290_e106786_d_n6, assign70290_e106786_d_n7, assign70290_e106786_d_n8, assign70290_e106786_d_n9, assign70290_e106786_d_n10, assign70290_e106786_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70290_e106781: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign70290_e106783: f64 = (assign70290_e106781 + locals.var_tmf2);
        let assign70290_e106784: f64 = (assign70290_e106783).sqrt();
        (assign70290_e106784, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign70290_e106784)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign70290_e106786;
        locals.var_tmf2_dn0 = assign70290_e106786_d_n0;
        locals.var_tmf2_dn2 = assign70290_e106786_d_n2;
        locals.var_tmf2_dn4 = assign70290_e106786_d_n4;
        locals.var_tmf2_dn5 = assign70290_e106786_d_n5;
        locals.var_tmf2_dn6 = assign70290_e106786_d_n6;
        locals.var_tmf2_dn7 = assign70290_e106786_d_n7;
        locals.var_tmf2_dn8 = assign70290_e106786_d_n8;
        locals.var_tmf2_dn9 = assign70290_e106786_d_n9;
        locals.var_tmf2_dn10 = assign70290_e106786_d_n10;
        locals.var_tmf2_dn13 = assign70290_e106786_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign70300_e106796, assign70300_e106796_d_n0, assign70300_e106796_d_n2, assign70300_e106796_d_n4, assign70300_e106796_d_n5, assign70300_e106796_d_n6, assign70300_e106796_d_n7, assign70300_e106796_d_n8, assign70300_e106796_d_n9, assign70300_e106796_d_n10, assign70300_e106796_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70300_e106792: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign70300_e106793: f64 = (1.0 + assign70300_e106792);
        let assign70300_e106794: f64 = (0.5 * assign70300_e106793);
        (assign70300_e106794, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign70300_e106796;
        locals.var_t0_dn0 = assign70300_e106796_d_n0;
        locals.var_t0_dn2 = assign70300_e106796_d_n2;
        locals.var_t0_dn4 = assign70300_e106796_d_n4;
        locals.var_t0_dn5 = assign70300_e106796_d_n5;
        locals.var_t0_dn6 = assign70300_e106796_d_n6;
        locals.var_t0_dn7 = assign70300_e106796_d_n7;
        locals.var_t0_dn8 = assign70300_e106796_d_n8;
        locals.var_t0_dn9 = assign70300_e106796_d_n9;
        locals.var_t0_dn10 = assign70300_e106796_d_n10;
        locals.var_t0_dn13 = assign70300_e106796_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign70310_e106806, assign70310_e106806_d_n0, assign70310_e106806_d_n2, assign70310_e106806_d_n4, assign70310_e106806_d_n5, assign70310_e106806_d_n6, assign70310_e106806_d_n7, assign70310_e106806_d_n8, assign70310_e106806_d_n9, assign70310_e106806_d_n10, assign70310_e106806_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70310_e106802: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign70310_e106803: f64 = (0.5 * assign70310_e106802);
        let assign70310_e106804: f64 = (0.8 - assign70310_e106803);
        (assign70310_e106804, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_vbs_max_over, locals.var_vbs_max_over_dn0, locals.var_vbs_max_over_dn2, locals.var_vbs_max_over_dn4, locals.var_vbs_max_over_dn5, locals.var_vbs_max_over_dn6, locals.var_vbs_max_over_dn7, locals.var_vbs_max_over_dn8, locals.var_vbs_max_over_dn9, locals.var_vbs_max_over_dn10, locals.var_vbs_max_over_dn13,)
    }
};
        locals.var_vbs_max_over = assign70310_e106806;
        locals.var_vbs_max_over_dn0 = assign70310_e106806_d_n0;
        locals.var_vbs_max_over_dn2 = assign70310_e106806_d_n2;
        locals.var_vbs_max_over_dn4 = assign70310_e106806_d_n4;
        locals.var_vbs_max_over_dn5 = assign70310_e106806_d_n5;
        locals.var_vbs_max_over_dn6 = assign70310_e106806_d_n6;
        locals.var_vbs_max_over_dn7 = assign70310_e106806_d_n7;
        locals.var_vbs_max_over_dn8 = assign70310_e106806_d_n8;
        locals.var_vbs_max_over_dn9 = assign70310_e106806_d_n9;
        locals.var_vbs_max_over_dn10 = assign70310_e106806_d_n10;
        locals.var_vbs_max_over_dn13 = assign70310_e106806_d_n13;
        locals.var_vbs_max_over_rv = 0.0;

        let assign70320_e106810: f64 = (locals.var_vbs_max_over * 0.5);
        let assign70320_e106811: f64 = if locals.var_vbs_bnd_over > assign70320_e106810 { 1.0 } else { 0.0 };
        locals.var_guard1651 = assign70320_e106811;
        locals.var_guard1651_rv = 0.0;

        let (assign70330_e106819, assign70330_e106819_d_n0, assign70330_e106819_d_n2, assign70330_e106819_d_n4, assign70330_e106819_d_n5, assign70330_e106819_d_n6, assign70330_e106819_d_n7, assign70330_e106819_d_n8, assign70330_e106819_d_n9, assign70330_e106819_d_n10, assign70330_e106819_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1651 != 0.0)) {
        let assign70330_e106817: f64 = (0.5 * locals.var_vbs_max_over);
        (assign70330_e106817, (0.5 * locals.var_vbs_max_over_dn0), (0.5 * locals.var_vbs_max_over_dn2), (0.5 * locals.var_vbs_max_over_dn4), (0.5 * locals.var_vbs_max_over_dn5), (0.5 * locals.var_vbs_max_over_dn6), (0.5 * locals.var_vbs_max_over_dn7), (0.5 * locals.var_vbs_max_over_dn8), (0.5 * locals.var_vbs_max_over_dn9), (0.5 * locals.var_vbs_max_over_dn10), (0.5 * locals.var_vbs_max_over_dn13),)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn13,)
    }
};
        locals.var_vbs_bnd_over = assign70330_e106819;
        locals.var_vbs_bnd_over_dn0 = assign70330_e106819_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70330_e106819_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70330_e106819_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70330_e106819_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70330_e106819_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70330_e106819_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70330_e106819_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70330_e106819_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70330_e106819_d_n10;
        locals.var_vbs_bnd_over_dn13 = assign70330_e106819_d_n13;
        locals.var_vbs_bnd_over_rv = 0.0;

        let assign70340_e106821: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1652 = assign70340_e106821;
        locals.var_guard1652_rv = 0.0;

        let (assign70350_e106827, assign70350_e106827_d_n0, assign70350_e106827_d_n2, assign70350_e106827_d_n4, assign70350_e106827_d_n5, assign70350_e106827_d_n6, assign70350_e106827_d_n7, assign70350_e106827_d_n8, assign70350_e106827_d_n9, assign70350_e106827_d_n10, assign70350_e106827_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1652 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over, locals.var_vbs_max_over_dn0, locals.var_vbs_max_over_dn2, locals.var_vbs_max_over_dn4, locals.var_vbs_max_over_dn5, locals.var_vbs_max_over_dn6, locals.var_vbs_max_over_dn7, locals.var_vbs_max_over_dn8, locals.var_vbs_max_over_dn9, locals.var_vbs_max_over_dn10, locals.var_vbs_max_over_dn13,)
    }
};
        locals.var_vbs_max_over = assign70350_e106827;
        locals.var_vbs_max_over_dn0 = assign70350_e106827_d_n0;
        locals.var_vbs_max_over_dn2 = assign70350_e106827_d_n2;
        locals.var_vbs_max_over_dn4 = assign70350_e106827_d_n4;
        locals.var_vbs_max_over_dn5 = assign70350_e106827_d_n5;
        locals.var_vbs_max_over_dn6 = assign70350_e106827_d_n6;
        locals.var_vbs_max_over_dn7 = assign70350_e106827_d_n7;
        locals.var_vbs_max_over_dn8 = assign70350_e106827_d_n8;
        locals.var_vbs_max_over_dn9 = assign70350_e106827_d_n9;
        locals.var_vbs_max_over_dn10 = assign70350_e106827_d_n10;
        locals.var_vbs_max_over_dn13 = assign70350_e106827_d_n13;
        locals.var_vbs_max_over_rv = 0.0;

        let assign70360_e106829: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard1653 = assign70360_e106829;
        locals.var_guard1653_rv = 0.0;

        let (assign70370_e106835, assign70370_e106835_d_n0, assign70370_e106835_d_n2, assign70370_e106835_d_n4, assign70370_e106835_d_n5, assign70370_e106835_d_n6, assign70370_e106835_d_n7, assign70370_e106835_d_n8, assign70370_e106835_d_n9, assign70370_e106835_d_n10, assign70370_e106835_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1653 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn13,)
    }
};
        locals.var_vbs_bnd_over = assign70370_e106835;
        locals.var_vbs_bnd_over_dn0 = assign70370_e106835_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70370_e106835_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70370_e106835_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70370_e106835_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70370_e106835_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70370_e106835_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70370_e106835_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70370_e106835_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70370_e106835_d_n10;
        locals.var_vbs_bnd_over_dn13 = assign70370_e106835_d_n13;
        locals.var_vbs_bnd_over_rv = 0.0;

        let assign70380_e106837: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1654 = assign70380_e106837;
        locals.var_guard1654_rv = 0.0;

        let (assign70390_e106848, assign70390_e106848_d_n0, assign70390_e106848_d_n2, assign70390_e106848_d_n4, assign70390_e106848_d_n5, assign70390_e106848_d_n6, assign70390_e106848_d_n7, assign70390_e106848_d_n8, assign70390_e106848_d_n9, assign70390_e106848_d_n10, assign70390_e106848_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1653 == 0.0)) && (locals.var_guard1654 != 0.0)) {
        let assign70390_e106846: f64 = (0.5 * locals.var_vbs_max_over);
        (assign70390_e106846, (0.5 * locals.var_vbs_max_over_dn0), (0.5 * locals.var_vbs_max_over_dn2), (0.5 * locals.var_vbs_max_over_dn4), (0.5 * locals.var_vbs_max_over_dn5), (0.5 * locals.var_vbs_max_over_dn6), (0.5 * locals.var_vbs_max_over_dn7), (0.5 * locals.var_vbs_max_over_dn8), (0.5 * locals.var_vbs_max_over_dn9), (0.5 * locals.var_vbs_max_over_dn10), (0.5 * locals.var_vbs_max_over_dn13),)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn13,)
    }
};
        locals.var_vbs_bnd_over = assign70390_e106848;
        locals.var_vbs_bnd_over_dn0 = assign70390_e106848_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70390_e106848_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70390_e106848_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70390_e106848_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70390_e106848_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70390_e106848_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70390_e106848_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70390_e106848_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70390_e106848_d_n10;
        locals.var_vbs_bnd_over_dn13 = assign70390_e106848_d_n13;
        locals.var_vbs_bnd_over_rv = 0.0;

        let assign70400_e106852: f64 = (locals.var_vbs_max_over * 0.5);
        let assign70400_e106853: f64 = if locals.var_vbs_bnd_over > assign70400_e106852 { 1.0 } else { 0.0 };
        locals.var_guard1655 = assign70400_e106853;
        locals.var_guard1655_rv = 0.0;

        let (assign70410_e106861, assign70410_e106861_d_n0, assign70410_e106861_d_n2, assign70410_e106861_d_n4, assign70410_e106861_d_n5, assign70410_e106861_d_n6, assign70410_e106861_d_n7, assign70410_e106861_d_n8, assign70410_e106861_d_n9, assign70410_e106861_d_n10, assign70410_e106861_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1655 != 0.0)) {
        let assign70410_e106859: f64 = (0.5 * locals.var_vbs_max_over);
        (assign70410_e106859, (0.5 * locals.var_vbs_max_over_dn0), (0.5 * locals.var_vbs_max_over_dn2), (0.5 * locals.var_vbs_max_over_dn4), (0.5 * locals.var_vbs_max_over_dn5), (0.5 * locals.var_vbs_max_over_dn6), (0.5 * locals.var_vbs_max_over_dn7), (0.5 * locals.var_vbs_max_over_dn8), (0.5 * locals.var_vbs_max_over_dn9), (0.5 * locals.var_vbs_max_over_dn10), (0.5 * locals.var_vbs_max_over_dn13),)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn13,)
    }
};
        locals.var_vbs_bnd_over = assign70410_e106861;
        locals.var_vbs_bnd_over_dn0 = assign70410_e106861_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70410_e106861_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70410_e106861_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70410_e106861_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70410_e106861_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70410_e106861_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70410_e106861_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70410_e106861_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70410_e106861_d_n10;
        locals.var_vbs_bnd_over_dn13 = assign70410_e106861_d_n13;
        locals.var_vbs_bnd_over_rv = 0.0;

        let assign70420_e106864: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1656 = assign70420_e106864;
        locals.var_guard1656_rv = 0.0;

        let (assign70430_e106871, assign70430_e106871_d_n0, assign70430_e106871_d_n2, assign70430_e106871_d_n4, assign70430_e106871_d_n5, assign70430_e106871_d_n6, assign70430_e106871_d_n7, assign70430_e106871_d_n8, assign70430_e106871_d_n9, assign70430_e106871_d_n10, assign70430_e106871_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) {
        let assign70430_e106869: f64 = (-locals.var_vxbgmt);
        (assign70430_e106869, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign70430_e106871;
        locals.var_t0_dn0 = assign70430_e106871_d_n0;
        locals.var_t0_dn2 = assign70430_e106871_d_n2;
        locals.var_t0_dn4 = assign70430_e106871_d_n4;
        locals.var_t0_dn5 = assign70430_e106871_d_n5;
        locals.var_t0_dn6 = assign70430_e106871_d_n6;
        locals.var_t0_dn7 = assign70430_e106871_d_n7;
        locals.var_t0_dn8 = assign70430_e106871_d_n8;
        locals.var_t0_dn9 = assign70430_e106871_d_n9;
        locals.var_t0_dn10 = assign70430_e106871_d_n10;
        locals.var_t0_dn13 = assign70430_e106871_d_n13;
        locals.var_t0_rv = 0.0;

        let assign70440_e106874: f64 = if locals.var_t0 > locals.var_vbs_bnd_over { 1.0 } else { 0.0 };
        locals.var_guard1657 = assign70440_e106874;
        locals.var_guard1657_rv = 0.0;

        let (assign70450_e106884, assign70450_e106884_d_n0, assign70450_e106884_d_n2, assign70450_e106884_d_n4, assign70450_e106884_d_n5, assign70450_e106884_d_n6, assign70450_e106884_d_n7, assign70450_e106884_d_n8, assign70450_e106884_d_n9, assign70450_e106884_d_n10, assign70450_e106884_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70450_e106882: f64 = (locals.var_t0 - locals.var_vbs_bnd_over);
        (assign70450_e106882, (locals.var_t0_dn0 - locals.var_vbs_bnd_over_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over_dn10), (locals.var_t0_dn13 - locals.var_vbs_bnd_over_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign70450_e106884;
        locals.var_t1_dn0 = assign70450_e106884_d_n0;
        locals.var_t1_dn2 = assign70450_e106884_d_n2;
        locals.var_t1_dn4 = assign70450_e106884_d_n4;
        locals.var_t1_dn5 = assign70450_e106884_d_n5;
        locals.var_t1_dn6 = assign70450_e106884_d_n6;
        locals.var_t1_dn7 = assign70450_e106884_d_n7;
        locals.var_t1_dn8 = assign70450_e106884_d_n8;
        locals.var_t1_dn9 = assign70450_e106884_d_n9;
        locals.var_t1_dn10 = assign70450_e106884_d_n10;
        locals.var_t1_dn13 = assign70450_e106884_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign70460_e106894, assign70460_e106894_d_n0, assign70460_e106894_d_n2, assign70460_e106894_d_n4, assign70460_e106894_d_n5, assign70460_e106894_d_n6, assign70460_e106894_d_n7, assign70460_e106894_d_n8, assign70460_e106894_d_n9, assign70460_e106894_d_n10, assign70460_e106894_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70460_e106892: f64 = (locals.var_vbs_max_over - locals.var_vbs_bnd_over);
        (assign70460_e106892, (locals.var_vbs_max_over_dn0 - locals.var_vbs_bnd_over_dn0), (locals.var_vbs_max_over_dn2 - locals.var_vbs_bnd_over_dn2), (locals.var_vbs_max_over_dn4 - locals.var_vbs_bnd_over_dn4), (locals.var_vbs_max_over_dn5 - locals.var_vbs_bnd_over_dn5), (locals.var_vbs_max_over_dn6 - locals.var_vbs_bnd_over_dn6), (locals.var_vbs_max_over_dn7 - locals.var_vbs_bnd_over_dn7), (locals.var_vbs_max_over_dn8 - locals.var_vbs_bnd_over_dn8), (locals.var_vbs_max_over_dn9 - locals.var_vbs_bnd_over_dn9), (locals.var_vbs_max_over_dn10 - locals.var_vbs_bnd_over_dn10), (locals.var_vbs_max_over_dn13 - locals.var_vbs_bnd_over_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign70460_e106894;
        locals.var_t2_dn0 = assign70460_e106894_d_n0;
        locals.var_t2_dn2 = assign70460_e106894_d_n2;
        locals.var_t2_dn4 = assign70460_e106894_d_n4;
        locals.var_t2_dn5 = assign70460_e106894_d_n5;
        locals.var_t2_dn6 = assign70460_e106894_d_n6;
        locals.var_t2_dn7 = assign70460_e106894_d_n7;
        locals.var_t2_dn8 = assign70460_e106894_d_n8;
        locals.var_t2_dn9 = assign70460_e106894_d_n9;
        locals.var_t2_dn10 = assign70460_e106894_d_n10;
        locals.var_t2_dn13 = assign70460_e106894_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign70470_e106904, assign70470_e106904_d_n0, assign70470_e106904_d_n2, assign70470_e106904_d_n4, assign70470_e106904_d_n5, assign70470_e106904_d_n6, assign70470_e106904_d_n7, assign70470_e106904_d_n8, assign70470_e106904_d_n9, assign70470_e106904_d_n10, assign70470_e106904_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70470_e106902: f64 = (locals.var_t1 / locals.var_t2);
        (assign70470_e106902, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign70470_e106904;
        locals.var_tmf1_dn0 = assign70470_e106904_d_n0;
        locals.var_tmf1_dn2 = assign70470_e106904_d_n2;
        locals.var_tmf1_dn4 = assign70470_e106904_d_n4;
        locals.var_tmf1_dn5 = assign70470_e106904_d_n5;
        locals.var_tmf1_dn6 = assign70470_e106904_d_n6;
        locals.var_tmf1_dn7 = assign70470_e106904_d_n7;
        locals.var_tmf1_dn8 = assign70470_e106904_d_n8;
        locals.var_tmf1_dn9 = assign70470_e106904_d_n9;
        locals.var_tmf1_dn10 = assign70470_e106904_d_n10;
        locals.var_tmf1_dn13 = assign70470_e106904_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign70480_e106914, assign70480_e106914_d_n0, assign70480_e106914_d_n2, assign70480_e106914_d_n4, assign70480_e106914_d_n5, assign70480_e106914_d_n6, assign70480_e106914_d_n7, assign70480_e106914_d_n8, assign70480_e106914_d_n9, assign70480_e106914_d_n10, assign70480_e106914_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70480_e106912: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign70480_e106912, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign70480_e106914;
        locals.var_tmf2_dn0 = assign70480_e106914_d_n0;
        locals.var_tmf2_dn2 = assign70480_e106914_d_n2;
        locals.var_tmf2_dn4 = assign70480_e106914_d_n4;
        locals.var_tmf2_dn5 = assign70480_e106914_d_n5;
        locals.var_tmf2_dn6 = assign70480_e106914_d_n6;
        locals.var_tmf2_dn7 = assign70480_e106914_d_n7;
        locals.var_tmf2_dn8 = assign70480_e106914_d_n8;
        locals.var_tmf2_dn9 = assign70480_e106914_d_n9;
        locals.var_tmf2_dn10 = assign70480_e106914_d_n10;
        locals.var_tmf2_dn13 = assign70480_e106914_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign70490_e106924, assign70490_e106924_d_n0, assign70490_e106924_d_n2, assign70490_e106924_d_n4, assign70490_e106924_d_n5, assign70490_e106924_d_n6, assign70490_e106924_d_n7, assign70490_e106924_d_n8, assign70490_e106924_d_n9, assign70490_e106924_d_n10, assign70490_e106924_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70490_e106922: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign70490_e106922, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign70490_e106924;
        locals.var_tmf3_dn0 = assign70490_e106924_d_n0;
        locals.var_tmf3_dn2 = assign70490_e106924_d_n2;
        locals.var_tmf3_dn4 = assign70490_e106924_d_n4;
        locals.var_tmf3_dn5 = assign70490_e106924_d_n5;
        locals.var_tmf3_dn6 = assign70490_e106924_d_n6;
        locals.var_tmf3_dn7 = assign70490_e106924_d_n7;
        locals.var_tmf3_dn8 = assign70490_e106924_d_n8;
        locals.var_tmf3_dn9 = assign70490_e106924_d_n9;
        locals.var_tmf3_dn10 = assign70490_e106924_d_n10;
        locals.var_tmf3_dn13 = assign70490_e106924_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign70500_e106934, assign70500_e106934_d_n0, assign70500_e106934_d_n2, assign70500_e106934_d_n4, assign70500_e106934_d_n5, assign70500_e106934_d_n6, assign70500_e106934_d_n7, assign70500_e106934_d_n8, assign70500_e106934_d_n9, assign70500_e106934_d_n10, assign70500_e106934_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70500_e106932: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign70500_e106932, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign70500_e106934;
        locals.var_tmf4_dn0 = assign70500_e106934_d_n0;
        locals.var_tmf4_dn2 = assign70500_e106934_d_n2;
        locals.var_tmf4_dn4 = assign70500_e106934_d_n4;
        locals.var_tmf4_dn5 = assign70500_e106934_d_n5;
        locals.var_tmf4_dn6 = assign70500_e106934_d_n6;
        locals.var_tmf4_dn7 = assign70500_e106934_d_n7;
        locals.var_tmf4_dn8 = assign70500_e106934_d_n8;
        locals.var_tmf4_dn9 = assign70500_e106934_d_n9;
        locals.var_tmf4_dn10 = assign70500_e106934_d_n10;
        locals.var_tmf4_dn13 = assign70500_e106934_d_n13;
        locals.var_tmf4_rv = 0.0;

        let (assign70510_e106952, assign70510_e106952_d_n0, assign70510_e106952_d_n2, assign70510_e106952_d_n4, assign70510_e106952_d_n5, assign70510_e106952_d_n6, assign70510_e106952_d_n7, assign70510_e106952_d_n8, assign70510_e106952_d_n9, assign70510_e106952_d_n10, assign70510_e106952_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70510_e106943: f64 = (1.0 + locals.var_tmf1);
        let assign70510_e106945: f64 = (assign70510_e106943 + locals.var_tmf2);
        let assign70510_e106947: f64 = (assign70510_e106945 + locals.var_tmf3);
        let assign70510_e106949: f64 = (assign70510_e106947 + locals.var_tmf4);
        let assign70510_e106950: f64 = (1.0 / assign70510_e106949);
        (assign70510_e106950, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign70510_e106949 * assign70510_e106949))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign70510_e106949 * assign70510_e106949))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign70510_e106949 * assign70510_e106949))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign70510_e106949 * assign70510_e106949))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign70510_e106949 * assign70510_e106949))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign70510_e106949 * assign70510_e106949))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign70510_e106949 * assign70510_e106949))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign70510_e106949 * assign70510_e106949))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign70510_e106949 * assign70510_e106949))), (-((((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) + locals.var_tmf3_dn13) + locals.var_tmf4_dn13) / (assign70510_e106949 * assign70510_e106949))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign70510_e106952;
        locals.var_tmf0_dn0 = assign70510_e106952_d_n0;
        locals.var_tmf0_dn2 = assign70510_e106952_d_n2;
        locals.var_tmf0_dn4 = assign70510_e106952_d_n4;
        locals.var_tmf0_dn5 = assign70510_e106952_d_n5;
        locals.var_tmf0_dn6 = assign70510_e106952_d_n6;
        locals.var_tmf0_dn7 = assign70510_e106952_d_n7;
        locals.var_tmf0_dn8 = assign70510_e106952_d_n8;
        locals.var_tmf0_dn9 = assign70510_e106952_d_n9;
        locals.var_tmf0_dn10 = assign70510_e106952_d_n10;
        locals.var_tmf0_dn13 = assign70510_e106952_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign70520_e106977, assign70520_e106977_d_n0, assign70520_e106977_d_n2, assign70520_e106977_d_n4, assign70520_e106977_d_n5, assign70520_e106977_d_n6, assign70520_e106977_d_n7, assign70520_e106977_d_n8, assign70520_e106977_d_n9, assign70520_e106977_d_n10, assign70520_e106977_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70520_e106961: f64 = (2.0 * locals.var_tmf1);
        let assign70520_e106962: f64 = (1.0 + assign70520_e106961);
        let assign70520_e106965: f64 = (3.0 * locals.var_tmf2);
        let assign70520_e106966: f64 = (assign70520_e106962 + assign70520_e106965);
        let assign70520_e106969: f64 = (4.0 * locals.var_tmf3);
        let assign70520_e106970: f64 = (assign70520_e106966 + assign70520_e106969);
        let assign70520_e106971: f64 = (-assign70520_e106970);
        let assign70520_e106973: f64 = (assign70520_e106971 * locals.var_tmf0);
        let assign70520_e106975: f64 = (assign70520_e106973 * locals.var_tmf0);
        (assign70520_e106975, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign70520_e106971 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign70520_e106973 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign70520_e106971 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign70520_e106973 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign70520_e106971 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign70520_e106973 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign70520_e106971 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign70520_e106973 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign70520_e106971 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign70520_e106973 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign70520_e106971 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign70520_e106973 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign70520_e106971 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign70520_e106973 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign70520_e106971 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign70520_e106973 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign70520_e106971 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign70520_e106973 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn13) + (3.0 * locals.var_tmf2_dn13)) + (4.0 * locals.var_tmf3_dn13))) * locals.var_tmf0) + (assign70520_e106971 * locals.var_tmf0_dn13)) * locals.var_tmf0) + (assign70520_e106973 * locals.var_tmf0_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign70520_e106977;
        locals.var_t11_dn0 = assign70520_e106977_d_n0;
        locals.var_t11_dn2 = assign70520_e106977_d_n2;
        locals.var_t11_dn4 = assign70520_e106977_d_n4;
        locals.var_t11_dn5 = assign70520_e106977_d_n5;
        locals.var_t11_dn6 = assign70520_e106977_d_n6;
        locals.var_t11_dn7 = assign70520_e106977_d_n7;
        locals.var_t11_dn8 = assign70520_e106977_d_n8;
        locals.var_t11_dn9 = assign70520_e106977_d_n9;
        locals.var_t11_dn10 = assign70520_e106977_d_n10;
        locals.var_t11_dn13 = assign70520_e106977_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign70530_e106989, assign70530_e106989_d_n0, assign70530_e106989_d_n2, assign70530_e106989_d_n4, assign70530_e106989_d_n5, assign70530_e106989_d_n6, assign70530_e106989_d_n7, assign70530_e106989_d_n8, assign70530_e106989_d_n9, assign70530_e106989_d_n10, assign70530_e106989_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70530_e106986: f64 = (1.0 - locals.var_tmf0);
        let assign70530_e106987: f64 = (locals.var_t2 * assign70530_e106986);
        (assign70530_e106987, ((locals.var_t2_dn0 * assign70530_e106986) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign70530_e106986) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign70530_e106986) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign70530_e106986) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign70530_e106986) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign70530_e106986) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign70530_e106986) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign70530_e106986) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign70530_e106986) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn13 * assign70530_e106986) + (locals.var_t2 * (-locals.var_tmf0_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign70530_e106989;
        locals.var_ty_dn0 = assign70530_e106989_d_n0;
        locals.var_ty_dn2 = assign70530_e106989_d_n2;
        locals.var_ty_dn4 = assign70530_e106989_d_n4;
        locals.var_ty_dn5 = assign70530_e106989_d_n5;
        locals.var_ty_dn6 = assign70530_e106989_d_n6;
        locals.var_ty_dn7 = assign70530_e106989_d_n7;
        locals.var_ty_dn8 = assign70530_e106989_d_n8;
        locals.var_ty_dn9 = assign70530_e106989_d_n9;
        locals.var_ty_dn10 = assign70530_e106989_d_n10;
        locals.var_ty_dn13 = assign70530_e106989_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign70540_e107003, assign70540_e107003_d_n0, assign70540_e107003_d_n2, assign70540_e107003_d_n4, assign70540_e107003_d_n5, assign70540_e107003_d_n6, assign70540_e107003_d_n7, assign70540_e107003_d_n8, assign70540_e107003_d_n9, assign70540_e107003_d_n10, assign70540_e107003_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70540_e106997: f64 = (1.0 - locals.var_tmf0);
        let assign70540_e107000: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign70540_e107001: f64 = (assign70540_e106997 + assign70540_e107000);
        (assign70540_e107001, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn13) + ((locals.var_tmf1_dn13 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign70540_e107003;
        locals.var_t0_dn0 = assign70540_e107003_d_n0;
        locals.var_t0_dn2 = assign70540_e107003_d_n2;
        locals.var_t0_dn4 = assign70540_e107003_d_n4;
        locals.var_t0_dn5 = assign70540_e107003_d_n5;
        locals.var_t0_dn6 = assign70540_e107003_d_n6;
        locals.var_t0_dn7 = assign70540_e107003_d_n7;
        locals.var_t0_dn8 = assign70540_e107003_d_n8;
        locals.var_t0_dn9 = assign70540_e107003_d_n9;
        locals.var_t0_dn10 = assign70540_e107003_d_n10;
        locals.var_t0_dn13 = assign70540_e107003_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign70550_e107012, assign70550_e107012_d_n0, assign70550_e107012_d_n2, assign70550_e107012_d_n4, assign70550_e107012_d_n5, assign70550_e107012_d_n6, assign70550_e107012_d_n7, assign70550_e107012_d_n8, assign70550_e107012_d_n9, assign70550_e107012_d_n10, assign70550_e107012_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70550_e107010: f64 = (-locals.var_t11);
        (assign70550_e107010, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn13),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign70550_e107012;
        locals.var_t11_dn0 = assign70550_e107012_d_n0;
        locals.var_t11_dn2 = assign70550_e107012_d_n2;
        locals.var_t11_dn4 = assign70550_e107012_d_n4;
        locals.var_t11_dn5 = assign70550_e107012_d_n5;
        locals.var_t11_dn6 = assign70550_e107012_d_n6;
        locals.var_t11_dn7 = assign70550_e107012_d_n7;
        locals.var_t11_dn8 = assign70550_e107012_d_n8;
        locals.var_t11_dn9 = assign70550_e107012_d_n9;
        locals.var_t11_dn10 = assign70550_e107012_d_n10;
        locals.var_t11_dn13 = assign70550_e107012_d_n13;
        locals.var_t11_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_254(
        locals: &mut StampLocals,
    ) {
        let (assign70560_e107022, assign70560_e107022_d_n0, assign70560_e107022_d_n2, assign70560_e107022_d_n4, assign70560_e107022_d_n5, assign70560_e107022_d_n6, assign70560_e107022_d_n7, assign70560_e107022_d_n8, assign70560_e107022_d_n9, assign70560_e107022_d_n10, assign70560_e107022_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 != 0.0)) {
        let assign70560_e107020: f64 = (locals.var_vbs_bnd_over + locals.var_ty);
        (assign70560_e107020, (locals.var_vbs_bnd_over_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over_dn13 + locals.var_ty_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign70560_e107022;
        locals.var_t10_dn0 = assign70560_e107022_d_n0;
        locals.var_t10_dn2 = assign70560_e107022_d_n2;
        locals.var_t10_dn4 = assign70560_e107022_d_n4;
        locals.var_t10_dn5 = assign70560_e107022_d_n5;
        locals.var_t10_dn6 = assign70560_e107022_d_n6;
        locals.var_t10_dn7 = assign70560_e107022_d_n7;
        locals.var_t10_dn8 = assign70560_e107022_d_n8;
        locals.var_t10_dn9 = assign70560_e107022_d_n9;
        locals.var_t10_dn10 = assign70560_e107022_d_n10;
        locals.var_t10_dn13 = assign70560_e107022_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign70570_e107031, assign70570_e107031_d_n0, assign70570_e107031_d_n2, assign70570_e107031_d_n4, assign70570_e107031_d_n5, assign70570_e107031_d_n6, assign70570_e107031_d_n7, assign70570_e107031_d_n8, assign70570_e107031_d_n9, assign70570_e107031_d_n10, assign70570_e107031_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) && (locals.var_guard1657 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign70570_e107031;
        locals.var_t10_dn0 = assign70570_e107031_d_n0;
        locals.var_t10_dn2 = assign70570_e107031_d_n2;
        locals.var_t10_dn4 = assign70570_e107031_d_n4;
        locals.var_t10_dn5 = assign70570_e107031_d_n5;
        locals.var_t10_dn6 = assign70570_e107031_d_n6;
        locals.var_t10_dn7 = assign70570_e107031_d_n7;
        locals.var_t10_dn8 = assign70570_e107031_d_n8;
        locals.var_t10_dn9 = assign70570_e107031_d_n9;
        locals.var_t10_dn10 = assign70570_e107031_d_n10;
        locals.var_t10_dn13 = assign70570_e107031_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign70580_e107038, assign70580_e107038_d_n0, assign70580_e107038_d_n2, assign70580_e107038_d_n4, assign70580_e107038_d_n5, assign70580_e107038_d_n6, assign70580_e107038_d_n7, assign70580_e107038_d_n8, assign70580_e107038_d_n9, assign70580_e107038_d_n10, assign70580_e107038_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) {
        let assign70580_e107036: f64 = (-locals.var_t10);
        (assign70580_e107036, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn13),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign70580_e107038;
        locals.var_vxbgmtcl_dn0 = assign70580_e107038_d_n0;
        locals.var_vxbgmtcl_dn2 = assign70580_e107038_d_n2;
        locals.var_vxbgmtcl_dn4 = assign70580_e107038_d_n4;
        locals.var_vxbgmtcl_dn5 = assign70580_e107038_d_n5;
        locals.var_vxbgmtcl_dn6 = assign70580_e107038_d_n6;
        locals.var_vxbgmtcl_dn7 = assign70580_e107038_d_n7;
        locals.var_vxbgmtcl_dn8 = assign70580_e107038_d_n8;
        locals.var_vxbgmtcl_dn9 = assign70580_e107038_d_n9;
        locals.var_vxbgmtcl_dn10 = assign70580_e107038_d_n10;
        locals.var_vxbgmtcl_dn13 = assign70580_e107038_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign70590_e107045, assign70590_e107045_d_n0, assign70590_e107045_d_n2, assign70590_e107045_d_n4, assign70590_e107045_d_n5, assign70590_e107045_d_n6, assign70590_e107045_d_n7, assign70590_e107045_d_n8, assign70590_e107045_d_n9, assign70590_e107045_d_n10, assign70590_e107045_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign70590_e107045;
        locals.var_vxbgmtcl_dn0 = assign70590_e107045_d_n0;
        locals.var_vxbgmtcl_dn2 = assign70590_e107045_d_n2;
        locals.var_vxbgmtcl_dn4 = assign70590_e107045_d_n4;
        locals.var_vxbgmtcl_dn5 = assign70590_e107045_d_n5;
        locals.var_vxbgmtcl_dn6 = assign70590_e107045_d_n6;
        locals.var_vxbgmtcl_dn7 = assign70590_e107045_d_n7;
        locals.var_vxbgmtcl_dn8 = assign70590_e107045_d_n8;
        locals.var_vxbgmtcl_dn9 = assign70590_e107045_d_n9;
        locals.var_vxbgmtcl_dn10 = assign70590_e107045_d_n10;
        locals.var_vxbgmtcl_dn13 = assign70590_e107045_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign70600_e107051, assign70600_e107051_d_n0, assign70600_e107051_d_n2, assign70600_e107051_d_n4, assign70600_e107051_d_n5, assign70600_e107051_d_n6, assign70600_e107051_d_n7, assign70600_e107051_d_n8, assign70600_e107051_d_n9, assign70600_e107051_d_n10, assign70600_e107051_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70600_e107049: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign70600_e107049, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn13 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn13,)
    }
};
        locals.var_fac1 = assign70600_e107051;
        locals.var_fac1_dn0 = assign70600_e107051_d_n0;
        locals.var_fac1_dn2 = assign70600_e107051_d_n2;
        locals.var_fac1_dn4 = assign70600_e107051_d_n4;
        locals.var_fac1_dn5 = assign70600_e107051_d_n5;
        locals.var_fac1_dn6 = assign70600_e107051_d_n6;
        locals.var_fac1_dn7 = assign70600_e107051_d_n7;
        locals.var_fac1_dn8 = assign70600_e107051_d_n8;
        locals.var_fac1_dn9 = assign70600_e107051_d_n9;
        locals.var_fac1_dn10 = assign70600_e107051_d_n10;
        locals.var_fac1_dn13 = assign70600_e107051_d_n13;
        locals.var_fac1_rv = 0.0;

        let (assign70610_e107057, assign70610_e107057_d_n0, assign70610_e107057_d_n2, assign70610_e107057_d_n4, assign70610_e107057_d_n5, assign70610_e107057_d_n6, assign70610_e107057_d_n7, assign70610_e107057_d_n8, assign70610_e107057_d_n9, assign70610_e107057_d_n10, assign70610_e107057_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70610_e107055: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign70610_e107055, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn13 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn13)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn13,)
    }
};
        locals.var_fac1p2 = assign70610_e107057;
        locals.var_fac1p2_dn0 = assign70610_e107057_d_n0;
        locals.var_fac1p2_dn2 = assign70610_e107057_d_n2;
        locals.var_fac1p2_dn4 = assign70610_e107057_d_n4;
        locals.var_fac1p2_dn5 = assign70610_e107057_d_n5;
        locals.var_fac1p2_dn6 = assign70610_e107057_d_n6;
        locals.var_fac1p2_dn7 = assign70610_e107057_d_n7;
        locals.var_fac1p2_dn8 = assign70610_e107057_d_n8;
        locals.var_fac1p2_dn9 = assign70610_e107057_d_n9;
        locals.var_fac1p2_dn10 = assign70610_e107057_d_n10;
        locals.var_fac1p2_dn13 = assign70610_e107057_d_n13;
        locals.var_fac1p2_rv = 0.0;

        let (assign70620_e107064, assign70620_e107064_d_n2, assign70620_e107064_d_n6, assign70620_e107064_d_n7, assign70620_e107064_d_n8,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70620_e107060: f64 = (-locals.var_vgbgmt);
        let assign70620_e107062: f64 = (assign70620_e107060 + locals.var_uc_vfbover);
        (assign70620_e107062, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn6), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn8,)
    }
};
        locals.var_vgpld = assign70620_e107064;
        locals.var_vgpld_dn2 = assign70620_e107064_d_n2;
        locals.var_vgpld_dn6 = assign70620_e107064_d_n6;
        locals.var_vgpld_dn7 = assign70620_e107064_d_n7;
        locals.var_vgpld_dn8 = assign70620_e107064_d_n8;
        locals.var_vgpld_rv = 0.0;

        let (assign70630_e107073, assign70630_e107073_d_n0, assign70630_e107073_d_n2, assign70630_e107073_d_n4, assign70630_e107073_d_n5, assign70630_e107073_d_n6, assign70630_e107073_d_n7, assign70630_e107073_d_n8, assign70630_e107073_d_n9, assign70630_e107073_d_n10, assign70630_e107073_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70630_e107067: f64 = (-locals.var_vxbgmtcl);
        let assign70630_e107070: f64 = (10.0 * 2.220446049250313e-16);
        let assign70630_e107071: f64 = (assign70630_e107067 + assign70630_e107070);
        (assign70630_e107071, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn13,)
    }
};
        locals.var_vgb_fb_ld = assign70630_e107073;
        locals.var_vgb_fb_ld_dn0 = assign70630_e107073_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign70630_e107073_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign70630_e107073_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign70630_e107073_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign70630_e107073_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign70630_e107073_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign70630_e107073_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign70630_e107073_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign70630_e107073_d_n10;
        locals.var_vgb_fb_ld_dn13 = assign70630_e107073_d_n13;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign70640_e107077, assign70640_e107077_d_n0, assign70640_e107077_d_n2, assign70640_e107077_d_n4, assign70640_e107077_d_n5, assign70640_e107077_d_n6, assign70640_e107077_d_n7, assign70640_e107077_d_n8, assign70640_e107077_d_n9, assign70640_e107077_d_n10, assign70640_e107077_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld, locals.var_q_dep_ld_dn0, locals.var_q_dep_ld_dn2, locals.var_q_dep_ld_dn4, locals.var_q_dep_ld_dn5, locals.var_q_dep_ld_dn6, locals.var_q_dep_ld_dn7, locals.var_q_dep_ld_dn8, locals.var_q_dep_ld_dn9, locals.var_q_dep_ld_dn10, locals.var_q_dep_ld_dn13,)
    }
};
        locals.var_q_dep_ld = assign70640_e107077;
        locals.var_q_dep_ld_dn0 = assign70640_e107077_d_n0;
        locals.var_q_dep_ld_dn2 = assign70640_e107077_d_n2;
        locals.var_q_dep_ld_dn4 = assign70640_e107077_d_n4;
        locals.var_q_dep_ld_dn5 = assign70640_e107077_d_n5;
        locals.var_q_dep_ld_dn6 = assign70640_e107077_d_n6;
        locals.var_q_dep_ld_dn7 = assign70640_e107077_d_n7;
        locals.var_q_dep_ld_dn8 = assign70640_e107077_d_n8;
        locals.var_q_dep_ld_dn9 = assign70640_e107077_d_n9;
        locals.var_q_dep_ld_dn10 = assign70640_e107077_d_n10;
        locals.var_q_dep_ld_dn13 = assign70640_e107077_d_n13;
        locals.var_q_dep_ld_rv = 0.0;

        let (assign70650_e107083,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70650_e107081: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign70650_e107081,)
    } else {
        (locals.var_q_nsubld,)
    }
};
        locals.var_q_nsubld = assign70650_e107083;
        locals.var_q_nsubld_rv = 0.0;

        let (assign70660_e107089, assign70660_e107089_d_n0, assign70660_e107089_d_n2, assign70660_e107089_d_n4, assign70660_e107089_d_n5, assign70660_e107089_d_n6, assign70660_e107089_d_n7, assign70660_e107089_d_n8, assign70660_e107089_d_n9, assign70660_e107089_d_n10, assign70660_e107089_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70660_e107087: f64 = (locals.var_nin / locals.var_nover_func);
        (assign70660_e107087, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign70660_e107089;
        locals.var_t0_dn0 = assign70660_e107089_d_n0;
        locals.var_t0_dn2 = assign70660_e107089_d_n2;
        locals.var_t0_dn4 = assign70660_e107089_d_n4;
        locals.var_t0_dn5 = assign70660_e107089_d_n5;
        locals.var_t0_dn6 = assign70660_e107089_d_n6;
        locals.var_t0_dn7 = assign70660_e107089_d_n7;
        locals.var_t0_dn8 = assign70660_e107089_d_n8;
        locals.var_t0_dn9 = assign70660_e107089_d_n9;
        locals.var_t0_dn10 = assign70660_e107089_d_n10;
        locals.var_t0_dn13 = assign70660_e107089_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign70670_e107095, assign70670_e107095_d_n0, assign70670_e107095_d_n2, assign70670_e107095_d_n4, assign70670_e107095_d_n5, assign70670_e107095_d_n6, assign70670_e107095_d_n7, assign70670_e107095_d_n8, assign70670_e107095_d_n9, assign70670_e107095_d_n10, assign70670_e107095_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70670_e107093: f64 = (locals.var_t0 * locals.var_t0);
        (assign70670_e107093, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign70670_e107095;
        locals.var_cnst1over_dn0 = assign70670_e107095_d_n0;
        locals.var_cnst1over_dn2 = assign70670_e107095_d_n2;
        locals.var_cnst1over_dn4 = assign70670_e107095_d_n4;
        locals.var_cnst1over_dn5 = assign70670_e107095_d_n5;
        locals.var_cnst1over_dn6 = assign70670_e107095_d_n6;
        locals.var_cnst1over_dn7 = assign70670_e107095_d_n7;
        locals.var_cnst1over_dn8 = assign70670_e107095_d_n8;
        locals.var_cnst1over_dn9 = assign70670_e107095_d_n9;
        locals.var_cnst1over_dn10 = assign70670_e107095_d_n10;
        locals.var_cnst1over_dn13 = assign70670_e107095_d_n13;
        locals.var_cnst1over_rv = 0.0;

        let assign70680_e107098: f64 = (-locals.var_vxbgmtcl);
        let assign70680_e107099: f64 = (locals.var_beta * assign70680_e107098);
        let assign70680_e107101: f64 = if assign70680_e107099 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1658 = assign70680_e107101;
        locals.var_guard1658_rv = 0.0;

        let (assign70690_e107116, assign70690_e107116_d_n0, assign70690_e107116_d_n2, assign70690_e107116_d_n4, assign70690_e107116_d_n5, assign70690_e107116_d_n6, assign70690_e107116_d_n7, assign70690_e107116_d_n8, assign70690_e107116_d_n9, assign70690_e107116_d_n10, assign70690_e107116_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) {
        let assign70690_e107109: f64 = (-locals.var_vxbgmtcl);
        let assign70690_e107110: f64 = (locals.var_beta * assign70690_e107109);
        let assign70690_e107111: f64 = (1.0 + assign70690_e107110);
        let assign70690_e107113: f64 = (assign70690_e107111 - 500.0);
        let assign70690_e107114: f64 = (1.403592217853e217 * assign70690_e107113);
        (assign70690_e107114, (1.403592217853e217 * ((locals.var_beta_dn0 * assign70690_e107109) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign70690_e107109) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign70690_e107109) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign70690_e107109) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign70690_e107109) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign70690_e107109) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign70690_e107109) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign70690_e107109) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign70690_e107109) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn13 * assign70690_e107109) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign70690_e107116;
        locals.var_exp_bvbs_dn0 = assign70690_e107116_d_n0;
        locals.var_exp_bvbs_dn2 = assign70690_e107116_d_n2;
        locals.var_exp_bvbs_dn4 = assign70690_e107116_d_n4;
        locals.var_exp_bvbs_dn5 = assign70690_e107116_d_n5;
        locals.var_exp_bvbs_dn6 = assign70690_e107116_d_n6;
        locals.var_exp_bvbs_dn7 = assign70690_e107116_d_n7;
        locals.var_exp_bvbs_dn8 = assign70690_e107116_d_n8;
        locals.var_exp_bvbs_dn9 = assign70690_e107116_d_n9;
        locals.var_exp_bvbs_dn10 = assign70690_e107116_d_n10;
        locals.var_exp_bvbs_dn13 = assign70690_e107116_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign70700_e107122, assign70700_e107122_d_n0, assign70700_e107122_d_n2, assign70700_e107122_d_n4, assign70700_e107122_d_n5, assign70700_e107122_d_n6, assign70700_e107122_d_n7, assign70700_e107122_d_n8, assign70700_e107122_d_n9, assign70700_e107122_d_n10, assign70700_e107122_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign70700_e107122;
        locals.var_t0_dn0 = assign70700_e107122_d_n0;
        locals.var_t0_dn2 = assign70700_e107122_d_n2;
        locals.var_t0_dn4 = assign70700_e107122_d_n4;
        locals.var_t0_dn5 = assign70700_e107122_d_n5;
        locals.var_t0_dn6 = assign70700_e107122_d_n6;
        locals.var_t0_dn7 = assign70700_e107122_d_n7;
        locals.var_t0_dn8 = assign70700_e107122_d_n8;
        locals.var_t0_dn9 = assign70700_e107122_d_n9;
        locals.var_t0_dn10 = assign70700_e107122_d_n10;
        locals.var_t0_dn13 = assign70700_e107122_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign70710_e107132, assign70710_e107132_d_n0, assign70710_e107132_d_n2, assign70710_e107132_d_n4, assign70710_e107132_d_n5, assign70710_e107132_d_n6, assign70710_e107132_d_n7, assign70710_e107132_d_n8, assign70710_e107132_d_n9, assign70710_e107132_d_n10, assign70710_e107132_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 == 0.0)) {
        let assign70710_e107129: f64 = (-locals.var_vxbgmtcl);
        let assign70710_e107130: f64 = (locals.var_beta * assign70710_e107129);
        (assign70710_e107130, ((locals.var_beta_dn0 * assign70710_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign70710_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign70710_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign70710_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign70710_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign70710_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign70710_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign70710_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign70710_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign70710_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign70710_e107132;
        locals.var_tmf1_dn0 = assign70710_e107132_d_n0;
        locals.var_tmf1_dn2 = assign70710_e107132_d_n2;
        locals.var_tmf1_dn4 = assign70710_e107132_d_n4;
        locals.var_tmf1_dn5 = assign70710_e107132_d_n5;
        locals.var_tmf1_dn6 = assign70710_e107132_d_n6;
        locals.var_tmf1_dn7 = assign70710_e107132_d_n7;
        locals.var_tmf1_dn8 = assign70710_e107132_d_n8;
        locals.var_tmf1_dn9 = assign70710_e107132_d_n9;
        locals.var_tmf1_dn10 = assign70710_e107132_d_n10;
        locals.var_tmf1_dn13 = assign70710_e107132_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign70720_e107139, assign70720_e107139_d_n0, assign70720_e107139_d_n2, assign70720_e107139_d_n4, assign70720_e107139_d_n5, assign70720_e107139_d_n6, assign70720_e107139_d_n7, assign70720_e107139_d_n8, assign70720_e107139_d_n9, assign70720_e107139_d_n10, assign70720_e107139_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign70720_e107139;
        locals.var_exp_bvbs_dn0 = assign70720_e107139_d_n0;
        locals.var_exp_bvbs_dn2 = assign70720_e107139_d_n2;
        locals.var_exp_bvbs_dn4 = assign70720_e107139_d_n4;
        locals.var_exp_bvbs_dn5 = assign70720_e107139_d_n5;
        locals.var_exp_bvbs_dn6 = assign70720_e107139_d_n6;
        locals.var_exp_bvbs_dn7 = assign70720_e107139_d_n7;
        locals.var_exp_bvbs_dn8 = assign70720_e107139_d_n8;
        locals.var_exp_bvbs_dn9 = assign70720_e107139_d_n9;
        locals.var_exp_bvbs_dn10 = assign70720_e107139_d_n10;
        locals.var_exp_bvbs_dn13 = assign70720_e107139_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let mut assign70730_loop_guard: usize = 0;
        while {
            let assign70730_cond_e107147: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign70730_cond_e107147 != 0.0
        } {
            assign70730_loop_guard += 1;
            assert!(assign70730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign70730_body0_e107156, assign70730_body0_e107156_d_n0, assign70730_body0_e107156_d_n2, assign70730_body0_e107156_d_n4, assign70730_body0_e107156_d_n5, assign70730_body0_e107156_d_n6, assign70730_body0_e107156_d_n7, assign70730_body0_e107156_d_n8, assign70730_body0_e107156_d_n9, assign70730_body0_e107156_d_n10, assign70730_body0_e107156_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 == 0.0)) {
        let assign70730_body0_e107154: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign70730_body0_e107154, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn13 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
            locals.var_exp_bvbs = assign70730_body0_e107156;
            locals.var_exp_bvbs_dn0 = assign70730_body0_e107156_d_n0;
            locals.var_exp_bvbs_dn2 = assign70730_body0_e107156_d_n2;
            locals.var_exp_bvbs_dn4 = assign70730_body0_e107156_d_n4;
            locals.var_exp_bvbs_dn5 = assign70730_body0_e107156_d_n5;
            locals.var_exp_bvbs_dn6 = assign70730_body0_e107156_d_n6;
            locals.var_exp_bvbs_dn7 = assign70730_body0_e107156_d_n7;
            locals.var_exp_bvbs_dn8 = assign70730_body0_e107156_d_n8;
            locals.var_exp_bvbs_dn9 = assign70730_body0_e107156_d_n9;
            locals.var_exp_bvbs_dn10 = assign70730_body0_e107156_d_n10;
            locals.var_exp_bvbs_dn13 = assign70730_body0_e107156_d_n13;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign70730_body1_e107165, assign70730_body1_e107165_d_n0, assign70730_body1_e107165_d_n2, assign70730_body1_e107165_d_n4, assign70730_body1_e107165_d_n5, assign70730_body1_e107165_d_n6, assign70730_body1_e107165_d_n7, assign70730_body1_e107165_d_n8, assign70730_body1_e107165_d_n9, assign70730_body1_e107165_d_n10, assign70730_body1_e107165_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 == 0.0)) {
        let assign70730_body1_e107163: f64 = (locals.var_tmf1 - 60.0);
        (assign70730_body1_e107163, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
            locals.var_tmf1 = assign70730_body1_e107165;
            locals.var_tmf1_dn0 = assign70730_body1_e107165_d_n0;
            locals.var_tmf1_dn2 = assign70730_body1_e107165_d_n2;
            locals.var_tmf1_dn4 = assign70730_body1_e107165_d_n4;
            locals.var_tmf1_dn5 = assign70730_body1_e107165_d_n5;
            locals.var_tmf1_dn6 = assign70730_body1_e107165_d_n6;
            locals.var_tmf1_dn7 = assign70730_body1_e107165_d_n7;
            locals.var_tmf1_dn8 = assign70730_body1_e107165_d_n8;
            locals.var_tmf1_dn9 = assign70730_body1_e107165_d_n9;
            locals.var_tmf1_dn10 = assign70730_body1_e107165_d_n10;
            locals.var_tmf1_dn13 = assign70730_body1_e107165_d_n13;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign70740_e107175, assign70740_e107175_d_n0, assign70740_e107175_d_n2, assign70740_e107175_d_n4, assign70740_e107175_d_n5, assign70740_e107175_d_n6, assign70740_e107175_d_n7, assign70740_e107175_d_n8, assign70740_e107175_d_n9, assign70740_e107175_d_n10, assign70740_e107175_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 == 0.0)) {
        let assign70740_e107172: f64 = (locals.var_tmf1).exp();
        let assign70740_e107173: f64 = (locals.var_exp_bvbs * assign70740_e107172);
        (assign70740_e107173, ((locals.var_exp_bvbs_dn0 * assign70740_e107172) + (locals.var_exp_bvbs * (assign70740_e107172 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign70740_e107172) + (locals.var_exp_bvbs * (assign70740_e107172 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign70740_e107172) + (locals.var_exp_bvbs * (assign70740_e107172 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign70740_e107172) + (locals.var_exp_bvbs * (assign70740_e107172 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign70740_e107172) + (locals.var_exp_bvbs * (assign70740_e107172 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign70740_e107172) + (locals.var_exp_bvbs * (assign70740_e107172 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign70740_e107172) + (locals.var_exp_bvbs * (assign70740_e107172 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign70740_e107172) + (locals.var_exp_bvbs * (assign70740_e107172 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign70740_e107172) + (locals.var_exp_bvbs * (assign70740_e107172 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn13 * assign70740_e107172) + (locals.var_exp_bvbs * (assign70740_e107172 * locals.var_tmf1_dn13))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign70740_e107175;
        locals.var_exp_bvbs_dn0 = assign70740_e107175_d_n0;
        locals.var_exp_bvbs_dn2 = assign70740_e107175_d_n2;
        locals.var_exp_bvbs_dn4 = assign70740_e107175_d_n4;
        locals.var_exp_bvbs_dn5 = assign70740_e107175_d_n5;
        locals.var_exp_bvbs_dn6 = assign70740_e107175_d_n6;
        locals.var_exp_bvbs_dn7 = assign70740_e107175_d_n7;
        locals.var_exp_bvbs_dn8 = assign70740_e107175_d_n8;
        locals.var_exp_bvbs_dn9 = assign70740_e107175_d_n9;
        locals.var_exp_bvbs_dn10 = assign70740_e107175_d_n10;
        locals.var_exp_bvbs_dn13 = assign70740_e107175_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign70750_e107182, assign70750_e107182_d_n0, assign70750_e107182_d_n2, assign70750_e107182_d_n4, assign70750_e107182_d_n5, assign70750_e107182_d_n6, assign70750_e107182_d_n7, assign70750_e107182_d_n8, assign70750_e107182_d_n9, assign70750_e107182_d_n10, assign70750_e107182_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign70750_e107182;
        locals.var_t0_dn0 = assign70750_e107182_d_n0;
        locals.var_t0_dn2 = assign70750_e107182_d_n2;
        locals.var_t0_dn4 = assign70750_e107182_d_n4;
        locals.var_t0_dn5 = assign70750_e107182_d_n5;
        locals.var_t0_dn6 = assign70750_e107182_d_n6;
        locals.var_t0_dn7 = assign70750_e107182_d_n7;
        locals.var_t0_dn8 = assign70750_e107182_d_n8;
        locals.var_t0_dn9 = assign70750_e107182_d_n9;
        locals.var_t0_dn10 = assign70750_e107182_d_n10;
        locals.var_t0_dn13 = assign70750_e107182_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign70760_e107195, assign70760_e107195_d_n0, assign70760_e107195_d_n2, assign70760_e107195_d_n4, assign70760_e107195_d_n5, assign70760_e107195_d_n6, assign70760_e107195_d_n7, assign70760_e107195_d_n8, assign70760_e107195_d_n9, assign70760_e107195_d_n10, assign70760_e107195_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70760_e107187: f64 = (-locals.var_vgpld);
        let assign70760_e107189: f64 = (assign70760_e107187 * 0.5);
        let assign70760_e107191: f64 = (assign70760_e107189 - 0.5);
        let assign70760_e107193: f64 = (assign70760_e107191 - 1.0);
        (assign70760_e107193, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, ((-locals.var_vgpld_dn6) * 0.5), ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign70760_e107195;
        locals.var_tmf1_dn0 = assign70760_e107195_d_n0;
        locals.var_tmf1_dn2 = assign70760_e107195_d_n2;
        locals.var_tmf1_dn4 = assign70760_e107195_d_n4;
        locals.var_tmf1_dn5 = assign70760_e107195_d_n5;
        locals.var_tmf1_dn6 = assign70760_e107195_d_n6;
        locals.var_tmf1_dn7 = assign70760_e107195_d_n7;
        locals.var_tmf1_dn8 = assign70760_e107195_d_n8;
        locals.var_tmf1_dn9 = assign70760_e107195_d_n9;
        locals.var_tmf1_dn10 = assign70760_e107195_d_n10;
        locals.var_tmf1_dn13 = assign70760_e107195_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign70770_e107205, assign70770_e107205_d_n0, assign70770_e107205_d_n2, assign70770_e107205_d_n4, assign70770_e107205_d_n5, assign70770_e107205_d_n6, assign70770_e107205_d_n7, assign70770_e107205_d_n8, assign70770_e107205_d_n9, assign70770_e107205_d_n10, assign70770_e107205_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70770_e107201: f64 = (4.0 * 0.5);
        let assign70770_e107203: f64 = assign70770_e107201;
        (assign70770_e107203, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign70770_e107205;
        locals.var_tmf2_dn0 = assign70770_e107205_d_n0;
        locals.var_tmf2_dn2 = assign70770_e107205_d_n2;
        locals.var_tmf2_dn4 = assign70770_e107205_d_n4;
        locals.var_tmf2_dn5 = assign70770_e107205_d_n5;
        locals.var_tmf2_dn6 = assign70770_e107205_d_n6;
        locals.var_tmf2_dn7 = assign70770_e107205_d_n7;
        locals.var_tmf2_dn8 = assign70770_e107205_d_n8;
        locals.var_tmf2_dn9 = assign70770_e107205_d_n9;
        locals.var_tmf2_dn10 = assign70770_e107205_d_n10;
        locals.var_tmf2_dn13 = assign70770_e107205_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign70780_e107217, assign70780_e107217_d_n0, assign70780_e107217_d_n2, assign70780_e107217_d_n4, assign70780_e107217_d_n5, assign70780_e107217_d_n6, assign70780_e107217_d_n7, assign70780_e107217_d_n8, assign70780_e107217_d_n9, assign70780_e107217_d_n10, assign70780_e107217_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign70780_e107215, assign70780_e107215_d_n0, assign70780_e107215_d_n2, assign70780_e107215_d_n4, assign70780_e107215_d_n5, assign70780_e107215_d_n6, assign70780_e107215_d_n7, assign70780_e107215_d_n8, assign70780_e107215_d_n9, assign70780_e107215_d_n10, assign70780_e107215_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign70780_e107214: f64 = (-locals.var_tmf2);
                (assign70780_e107214, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign70780_e107215, assign70780_e107215_d_n0, assign70780_e107215_d_n2, assign70780_e107215_d_n4, assign70780_e107215_d_n5, assign70780_e107215_d_n6, assign70780_e107215_d_n7, assign70780_e107215_d_n8, assign70780_e107215_d_n9, assign70780_e107215_d_n10, assign70780_e107215_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign70780_e107217;
        locals.var_tmf2_dn0 = assign70780_e107217_d_n0;
        locals.var_tmf2_dn2 = assign70780_e107217_d_n2;
        locals.var_tmf2_dn4 = assign70780_e107217_d_n4;
        locals.var_tmf2_dn5 = assign70780_e107217_d_n5;
        locals.var_tmf2_dn6 = assign70780_e107217_d_n6;
        locals.var_tmf2_dn7 = assign70780_e107217_d_n7;
        locals.var_tmf2_dn8 = assign70780_e107217_d_n8;
        locals.var_tmf2_dn9 = assign70780_e107217_d_n9;
        locals.var_tmf2_dn10 = assign70780_e107217_d_n10;
        locals.var_tmf2_dn13 = assign70780_e107217_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_255(
        locals: &mut StampLocals,
    ) {
        let (assign70790_e107228, assign70790_e107228_d_n0, assign70790_e107228_d_n2, assign70790_e107228_d_n4, assign70790_e107228_d_n5, assign70790_e107228_d_n6, assign70790_e107228_d_n7, assign70790_e107228_d_n8, assign70790_e107228_d_n9, assign70790_e107228_d_n10, assign70790_e107228_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70790_e107223: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign70790_e107225: f64 = (assign70790_e107223 + locals.var_tmf2);
        let assign70790_e107226: f64 = (assign70790_e107225).sqrt();
        (assign70790_e107226, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign70790_e107226)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign70790_e107226)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign70790_e107226)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign70790_e107226)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign70790_e107226)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign70790_e107226)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign70790_e107226)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign70790_e107226)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign70790_e107226)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign70790_e107226)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign70790_e107228;
        locals.var_tmf2_dn0 = assign70790_e107228_d_n0;
        locals.var_tmf2_dn2 = assign70790_e107228_d_n2;
        locals.var_tmf2_dn4 = assign70790_e107228_d_n4;
        locals.var_tmf2_dn5 = assign70790_e107228_d_n5;
        locals.var_tmf2_dn6 = assign70790_e107228_d_n6;
        locals.var_tmf2_dn7 = assign70790_e107228_d_n7;
        locals.var_tmf2_dn8 = assign70790_e107228_d_n8;
        locals.var_tmf2_dn9 = assign70790_e107228_d_n9;
        locals.var_tmf2_dn10 = assign70790_e107228_d_n10;
        locals.var_tmf2_dn13 = assign70790_e107228_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign70800_e107240, assign70800_e107240_d_n0, assign70800_e107240_d_n2, assign70800_e107240_d_n4, assign70800_e107240_d_n5, assign70800_e107240_d_n6, assign70800_e107240_d_n7, assign70800_e107240_d_n8, assign70800_e107240_d_n9, assign70800_e107240_d_n10, assign70800_e107240_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70800_e107236: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign70800_e107237: f64 = (1.0 + assign70800_e107236);
        let assign70800_e107238: f64 = (0.5 * assign70800_e107237);
        (assign70800_e107238, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign70800_e107240;
        locals.var_t0_dn0 = assign70800_e107240_d_n0;
        locals.var_t0_dn2 = assign70800_e107240_d_n2;
        locals.var_t0_dn4 = assign70800_e107240_d_n4;
        locals.var_t0_dn5 = assign70800_e107240_d_n5;
        locals.var_t0_dn6 = assign70800_e107240_d_n6;
        locals.var_t0_dn7 = assign70800_e107240_d_n7;
        locals.var_t0_dn8 = assign70800_e107240_d_n8;
        locals.var_t0_dn9 = assign70800_e107240_d_n9;
        locals.var_t0_dn10 = assign70800_e107240_d_n10;
        locals.var_t0_dn13 = assign70800_e107240_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign70810_e107252, assign70810_e107252_d_n0, assign70810_e107252_d_n2, assign70810_e107252_d_n4, assign70810_e107252_d_n5, assign70810_e107252_d_n6, assign70810_e107252_d_n7, assign70810_e107252_d_n8, assign70810_e107252_d_n9, assign70810_e107252_d_n10, assign70810_e107252_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70810_e107248: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign70810_e107249: f64 = (0.5 * assign70810_e107248);
        let assign70810_e107250: f64 = (0.5 + assign70810_e107249);
        (assign70810_e107250, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign70810_e107252;
        locals.var_t1_dn0 = assign70810_e107252_d_n0;
        locals.var_t1_dn2 = assign70810_e107252_d_n2;
        locals.var_t1_dn4 = assign70810_e107252_d_n4;
        locals.var_t1_dn5 = assign70810_e107252_d_n5;
        locals.var_t1_dn6 = assign70810_e107252_d_n6;
        locals.var_t1_dn7 = assign70810_e107252_d_n7;
        locals.var_t1_dn8 = assign70810_e107252_d_n8;
        locals.var_t1_dn9 = assign70810_e107252_d_n9;
        locals.var_t1_dn10 = assign70810_e107252_d_n10;
        locals.var_t1_dn13 = assign70810_e107252_d_n13;
        locals.var_t1_rv = 0.0;

        let assign70820_e107255: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign70820_e107258: f64 = (-locals.var_t1);
        let assign70820_e107263: f64 = if ((assign70820_e107255 > assign70820_e107258) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1659 = assign70820_e107263;
        locals.var_guard1659_rv = 0.0;

        let (assign70830_e107277, assign70830_e107277_d_n0, assign70830_e107277_d_n2, assign70830_e107277_d_n4, assign70830_e107277_d_n5, assign70830_e107277_d_n6, assign70830_e107277_d_n7, assign70830_e107277_d_n8, assign70830_e107277_d_n9, assign70830_e107277_d_n10, assign70830_e107277_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70830_e107271: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign70830_e107273: f64 = assign70830_e107271;
        let assign70830_e107275: f64 = (assign70830_e107273 + locals.var_t1);
        (assign70830_e107275, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), ((locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6) + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), (locals.var_vxbgmtcl_dn9 + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn13 + locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign70830_e107277;
        locals.var_tmf1_dn0 = assign70830_e107277_d_n0;
        locals.var_tmf1_dn2 = assign70830_e107277_d_n2;
        locals.var_tmf1_dn4 = assign70830_e107277_d_n4;
        locals.var_tmf1_dn5 = assign70830_e107277_d_n5;
        locals.var_tmf1_dn6 = assign70830_e107277_d_n6;
        locals.var_tmf1_dn7 = assign70830_e107277_d_n7;
        locals.var_tmf1_dn8 = assign70830_e107277_d_n8;
        locals.var_tmf1_dn9 = assign70830_e107277_d_n9;
        locals.var_tmf1_dn10 = assign70830_e107277_d_n10;
        locals.var_tmf1_dn13 = assign70830_e107277_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign70840_e107287, assign70840_e107287_d_n0, assign70840_e107287_d_n2, assign70840_e107287_d_n4, assign70840_e107287_d_n5, assign70840_e107287_d_n6, assign70840_e107287_d_n7, assign70840_e107287_d_n8, assign70840_e107287_d_n9, assign70840_e107287_d_n10, assign70840_e107287_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70840_e107285: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign70840_e107285, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign70840_e107287;
        locals.var_x2_dn0 = assign70840_e107287_d_n0;
        locals.var_x2_dn2 = assign70840_e107287_d_n2;
        locals.var_x2_dn4 = assign70840_e107287_d_n4;
        locals.var_x2_dn5 = assign70840_e107287_d_n5;
        locals.var_x2_dn6 = assign70840_e107287_d_n6;
        locals.var_x2_dn7 = assign70840_e107287_d_n7;
        locals.var_x2_dn8 = assign70840_e107287_d_n8;
        locals.var_x2_dn9 = assign70840_e107287_d_n9;
        locals.var_x2_dn10 = assign70840_e107287_d_n10;
        locals.var_x2_dn13 = assign70840_e107287_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign70850_e107297, assign70850_e107297_d_n0, assign70850_e107297_d_n2, assign70850_e107297_d_n4, assign70850_e107297_d_n5, assign70850_e107297_d_n6, assign70850_e107297_d_n7, assign70850_e107297_d_n8, assign70850_e107297_d_n9, assign70850_e107297_d_n10, assign70850_e107297_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70850_e107295: f64 = (locals.var_t1 * locals.var_t1);
        (assign70850_e107295, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign70850_e107297;
        locals.var_xmax2_dn0 = assign70850_e107297_d_n0;
        locals.var_xmax2_dn2 = assign70850_e107297_d_n2;
        locals.var_xmax2_dn4 = assign70850_e107297_d_n4;
        locals.var_xmax2_dn5 = assign70850_e107297_d_n5;
        locals.var_xmax2_dn6 = assign70850_e107297_d_n6;
        locals.var_xmax2_dn7 = assign70850_e107297_d_n7;
        locals.var_xmax2_dn8 = assign70850_e107297_d_n8;
        locals.var_xmax2_dn9 = assign70850_e107297_d_n9;
        locals.var_xmax2_dn10 = assign70850_e107297_d_n10;
        locals.var_xmax2_dn13 = assign70850_e107297_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign70860_e107305, assign70860_e107305_d_n0, assign70860_e107305_d_n2, assign70860_e107305_d_n4, assign70860_e107305_d_n5, assign70860_e107305_d_n6, assign70860_e107305_d_n7, assign70860_e107305_d_n8, assign70860_e107305_d_n9, assign70860_e107305_d_n10, assign70860_e107305_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign70860_e107305;
        locals.var_xp_dn0 = assign70860_e107305_d_n0;
        locals.var_xp_dn2 = assign70860_e107305_d_n2;
        locals.var_xp_dn4 = assign70860_e107305_d_n4;
        locals.var_xp_dn5 = assign70860_e107305_d_n5;
        locals.var_xp_dn6 = assign70860_e107305_d_n6;
        locals.var_xp_dn7 = assign70860_e107305_d_n7;
        locals.var_xp_dn8 = assign70860_e107305_d_n8;
        locals.var_xp_dn9 = assign70860_e107305_d_n9;
        locals.var_xp_dn10 = assign70860_e107305_d_n10;
        locals.var_xp_dn13 = assign70860_e107305_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign70870_e107313, assign70870_e107313_d_n0, assign70870_e107313_d_n2, assign70870_e107313_d_n4, assign70870_e107313_d_n5, assign70870_e107313_d_n6, assign70870_e107313_d_n7, assign70870_e107313_d_n8, assign70870_e107313_d_n9, assign70870_e107313_d_n10, assign70870_e107313_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign70870_e107313;
        locals.var_xmp_dn0 = assign70870_e107313_d_n0;
        locals.var_xmp_dn2 = assign70870_e107313_d_n2;
        locals.var_xmp_dn4 = assign70870_e107313_d_n4;
        locals.var_xmp_dn5 = assign70870_e107313_d_n5;
        locals.var_xmp_dn6 = assign70870_e107313_d_n6;
        locals.var_xmp_dn7 = assign70870_e107313_d_n7;
        locals.var_xmp_dn8 = assign70870_e107313_d_n8;
        locals.var_xmp_dn9 = assign70870_e107313_d_n9;
        locals.var_xmp_dn10 = assign70870_e107313_d_n10;
        locals.var_xmp_dn13 = assign70870_e107313_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign70880_e107321,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign70880_e107321;
        locals.var_m0_rv = 0.0;

        let (assign70890_e107329,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign70890_e107329;
        locals.var_mm_rv = 0.0;

        let (assign70900_e107337, assign70900_e107337_d_n0, assign70900_e107337_d_n2, assign70900_e107337_d_n4, assign70900_e107337_d_n5, assign70900_e107337_d_n6, assign70900_e107337_d_n7, assign70900_e107337_d_n8, assign70900_e107337_d_n9, assign70900_e107337_d_n10, assign70900_e107337_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign70900_e107337;
        locals.var_arg_dn0 = assign70900_e107337_d_n0;
        locals.var_arg_dn2 = assign70900_e107337_d_n2;
        locals.var_arg_dn4 = assign70900_e107337_d_n4;
        locals.var_arg_dn5 = assign70900_e107337_d_n5;
        locals.var_arg_dn6 = assign70900_e107337_d_n6;
        locals.var_arg_dn7 = assign70900_e107337_d_n7;
        locals.var_arg_dn8 = assign70900_e107337_d_n8;
        locals.var_arg_dn9 = assign70900_e107337_d_n9;
        locals.var_arg_dn10 = assign70900_e107337_d_n10;
        locals.var_arg_dn13 = assign70900_e107337_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign70910_e107345, assign70910_e107345_d_n0, assign70910_e107345_d_n2, assign70910_e107345_d_n4, assign70910_e107345_d_n5, assign70910_e107345_d_n6, assign70910_e107345_d_n7, assign70910_e107345_d_n8, assign70910_e107345_d_n9, assign70910_e107345_d_n10, assign70910_e107345_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign70910_e107345;
        locals.var_dnm_dn0 = assign70910_e107345_d_n0;
        locals.var_dnm_dn2 = assign70910_e107345_d_n2;
        locals.var_dnm_dn4 = assign70910_e107345_d_n4;
        locals.var_dnm_dn5 = assign70910_e107345_d_n5;
        locals.var_dnm_dn6 = assign70910_e107345_d_n6;
        locals.var_dnm_dn7 = assign70910_e107345_d_n7;
        locals.var_dnm_dn8 = assign70910_e107345_d_n8;
        locals.var_dnm_dn9 = assign70910_e107345_d_n9;
        locals.var_dnm_dn10 = assign70910_e107345_d_n10;
        locals.var_dnm_dn13 = assign70910_e107345_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign70920_e107355, assign70920_e107355_d_n0, assign70920_e107355_d_n2, assign70920_e107355_d_n4, assign70920_e107355_d_n5, assign70920_e107355_d_n6, assign70920_e107355_d_n7, assign70920_e107355_d_n8, assign70920_e107355_d_n9, assign70920_e107355_d_n10, assign70920_e107355_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70920_e107353: f64 = (locals.var_xp * locals.var_x2);
        (assign70920_e107353, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign70920_e107355;
        locals.var_xp_dn0 = assign70920_e107355_d_n0;
        locals.var_xp_dn2 = assign70920_e107355_d_n2;
        locals.var_xp_dn4 = assign70920_e107355_d_n4;
        locals.var_xp_dn5 = assign70920_e107355_d_n5;
        locals.var_xp_dn6 = assign70920_e107355_d_n6;
        locals.var_xp_dn7 = assign70920_e107355_d_n7;
        locals.var_xp_dn8 = assign70920_e107355_d_n8;
        locals.var_xp_dn9 = assign70920_e107355_d_n9;
        locals.var_xp_dn10 = assign70920_e107355_d_n10;
        locals.var_xp_dn13 = assign70920_e107355_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign70930_e107365, assign70930_e107365_d_n0, assign70930_e107365_d_n2, assign70930_e107365_d_n4, assign70930_e107365_d_n5, assign70930_e107365_d_n6, assign70930_e107365_d_n7, assign70930_e107365_d_n8, assign70930_e107365_d_n9, assign70930_e107365_d_n10, assign70930_e107365_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70930_e107363: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign70930_e107363, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign70930_e107365;
        locals.var_xmp_dn0 = assign70930_e107365_d_n0;
        locals.var_xmp_dn2 = assign70930_e107365_d_n2;
        locals.var_xmp_dn4 = assign70930_e107365_d_n4;
        locals.var_xmp_dn5 = assign70930_e107365_d_n5;
        locals.var_xmp_dn6 = assign70930_e107365_d_n6;
        locals.var_xmp_dn7 = assign70930_e107365_d_n7;
        locals.var_xmp_dn8 = assign70930_e107365_d_n8;
        locals.var_xmp_dn9 = assign70930_e107365_d_n9;
        locals.var_xmp_dn10 = assign70930_e107365_d_n10;
        locals.var_xmp_dn13 = assign70930_e107365_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign70940_e107375, assign70940_e107375_d_n0, assign70940_e107375_d_n2, assign70940_e107375_d_n4, assign70940_e107375_d_n5, assign70940_e107375_d_n6, assign70940_e107375_d_n7, assign70940_e107375_d_n8, assign70940_e107375_d_n9, assign70940_e107375_d_n10, assign70940_e107375_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70940_e107373: f64 = (locals.var_xp + locals.var_xmp);
        (assign70940_e107373, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign70940_e107375;
        locals.var_arg_dn0 = assign70940_e107375_d_n0;
        locals.var_arg_dn2 = assign70940_e107375_d_n2;
        locals.var_arg_dn4 = assign70940_e107375_d_n4;
        locals.var_arg_dn5 = assign70940_e107375_d_n5;
        locals.var_arg_dn6 = assign70940_e107375_d_n6;
        locals.var_arg_dn7 = assign70940_e107375_d_n7;
        locals.var_arg_dn8 = assign70940_e107375_d_n8;
        locals.var_arg_dn9 = assign70940_e107375_d_n9;
        locals.var_arg_dn10 = assign70940_e107375_d_n10;
        locals.var_arg_dn13 = assign70940_e107375_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign70950_e107383, assign70950_e107383_d_n0, assign70950_e107383_d_n2, assign70950_e107383_d_n4, assign70950_e107383_d_n5, assign70950_e107383_d_n6, assign70950_e107383_d_n7, assign70950_e107383_d_n8, assign70950_e107383_d_n9, assign70950_e107383_d_n10, assign70950_e107383_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign70950_e107383;
        locals.var_dnm_dn0 = assign70950_e107383_d_n0;
        locals.var_dnm_dn2 = assign70950_e107383_d_n2;
        locals.var_dnm_dn4 = assign70950_e107383_d_n4;
        locals.var_dnm_dn5 = assign70950_e107383_d_n5;
        locals.var_dnm_dn6 = assign70950_e107383_d_n6;
        locals.var_dnm_dn7 = assign70950_e107383_d_n7;
        locals.var_dnm_dn8 = assign70950_e107383_d_n8;
        locals.var_dnm_dn9 = assign70950_e107383_d_n9;
        locals.var_dnm_dn10 = assign70950_e107383_d_n10;
        locals.var_dnm_dn13 = assign70950_e107383_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign70960_e107398: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1660 = assign70960_e107398;
        locals.var_guard1660_rv = 0.0;

        let assign70970_e107401: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1661 = assign70970_e107401;
        locals.var_guard1661_rv = 0.0;

        let (assign70980_e107413,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign70980_e107413;
        locals.var_mm_rv = 0.0;

        let assign70990_e107416: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1662 = assign70990_e107416;
        locals.var_guard1662_rv = 0.0;

        let (assign71000_e107431,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 == 0.0)) && (locals.var_guard1662 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71000_e107431;
        locals.var_mm_rv = 0.0;

        let assign71010_e107434: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1663 = assign71010_e107434;
        locals.var_guard1663_rv = 0.0;

        let (assign71020_e107452,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 == 0.0)) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1663 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71020_e107452;
        locals.var_mm_rv = 0.0;

        let assign71030_e107455: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1664 = assign71030_e107455;
        locals.var_guard1664_rv = 0.0;

        let (assign71040_e107476,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 == 0.0)) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1664 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71040_e107476;
        locals.var_mm_rv = 0.0;

        let (assign71050_e107486,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) && (locals.var_guard1660 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign71050_e107486;
        locals.var_m0_rv = 0.0;

        let mut assign71060_loop_guard: usize = 0;
        while {
            let assign71060_cond_e107497: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) && (locals.var_guard1660 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign71060_cond_e107497 != 0.0
        } {
            assign71060_loop_guard += 1;
            assert!(assign71060_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign71060_body0_e107508, assign71060_body0_e107508_d_n0, assign71060_body0_e107508_d_n2, assign71060_body0_e107508_d_n4, assign71060_body0_e107508_d_n5, assign71060_body0_e107508_d_n6, assign71060_body0_e107508_d_n7, assign71060_body0_e107508_d_n8, assign71060_body0_e107508_d_n9, assign71060_body0_e107508_d_n10, assign71060_body0_e107508_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) && (locals.var_guard1660 != 0.0)) {
        let assign71060_body0_e107506: f64 = (locals.var_dnm).sqrt();
        (assign71060_body0_e107506, (locals.var_dnm_dn0 / (2.0 * assign71060_body0_e107506)), (locals.var_dnm_dn2 / (2.0 * assign71060_body0_e107506)), (locals.var_dnm_dn4 / (2.0 * assign71060_body0_e107506)), (locals.var_dnm_dn5 / (2.0 * assign71060_body0_e107506)), (locals.var_dnm_dn6 / (2.0 * assign71060_body0_e107506)), (locals.var_dnm_dn7 / (2.0 * assign71060_body0_e107506)), (locals.var_dnm_dn8 / (2.0 * assign71060_body0_e107506)), (locals.var_dnm_dn9 / (2.0 * assign71060_body0_e107506)), (locals.var_dnm_dn10 / (2.0 * assign71060_body0_e107506)), (locals.var_dnm_dn13 / (2.0 * assign71060_body0_e107506)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign71060_body0_e107508;
            locals.var_dnm_dn0 = assign71060_body0_e107508_d_n0;
            locals.var_dnm_dn2 = assign71060_body0_e107508_d_n2;
            locals.var_dnm_dn4 = assign71060_body0_e107508_d_n4;
            locals.var_dnm_dn5 = assign71060_body0_e107508_d_n5;
            locals.var_dnm_dn6 = assign71060_body0_e107508_d_n6;
            locals.var_dnm_dn7 = assign71060_body0_e107508_d_n7;
            locals.var_dnm_dn8 = assign71060_body0_e107508_d_n8;
            locals.var_dnm_dn9 = assign71060_body0_e107508_d_n9;
            locals.var_dnm_dn10 = assign71060_body0_e107508_d_n10;
            locals.var_dnm_dn13 = assign71060_body0_e107508_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign71060_body1_e107520,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) && (locals.var_guard1660 != 0.0)) {
        let assign71060_body1_e107518: f64 = (locals.var_m0 + 1.0);
        (assign71060_body1_e107518,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign71060_body1_e107520;
            locals.var_m0_rv = 0.0;
        }

        let (assign71070_e107542, assign71070_e107542_d_n0, assign71070_e107542_d_n2, assign71070_e107542_d_n4, assign71070_e107542_d_n5, assign71070_e107542_d_n6, assign71070_e107542_d_n7, assign71070_e107542_d_n8, assign71070_e107542_d_n9, assign71070_e107542_d_n10, assign71070_e107542_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) && (locals.var_guard1660 == 0.0)) {
        let (assign71070_e107540, assign71070_e107540_d_n0, assign71070_e107540_d_n2, assign71070_e107540_d_n4, assign71070_e107540_d_n5, assign71070_e107540_d_n6, assign71070_e107540_d_n7, assign71070_e107540_d_n8, assign71070_e107540_d_n9, assign71070_e107540_d_n10, assign71070_e107540_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign71070_e107537: f64 = 2.0;
                let assign71070_e107538: f64 = (1.0 / assign71070_e107537);
                let assign71070_e107539: f64 = (locals.var_dnm).powf(assign71070_e107538);
                (assign71070_e107539, if 0.0 == 0.0 && ((assign71070_e107538) as f64).is_finite() && ((assign71070_e107538) as f64).fract() == 0.0 { if assign71070_e107538 == 0.0 { 0.0 } else { (assign71070_e107538 * ((locals.var_dnm).powf(assign71070_e107538 - 1.0) * locals.var_dnm_dn0)) } } else { (assign71070_e107539 * (assign71070_e107538 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71070_e107538) as f64).is_finite() && ((assign71070_e107538) as f64).fract() == 0.0 { if assign71070_e107538 == 0.0 { 0.0 } else { (assign71070_e107538 * ((locals.var_dnm).powf(assign71070_e107538 - 1.0) * locals.var_dnm_dn2)) } } else { (assign71070_e107539 * (assign71070_e107538 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71070_e107538) as f64).is_finite() && ((assign71070_e107538) as f64).fract() == 0.0 { if assign71070_e107538 == 0.0 { 0.0 } else { (assign71070_e107538 * ((locals.var_dnm).powf(assign71070_e107538 - 1.0) * locals.var_dnm_dn4)) } } else { (assign71070_e107539 * (assign71070_e107538 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71070_e107538) as f64).is_finite() && ((assign71070_e107538) as f64).fract() == 0.0 { if assign71070_e107538 == 0.0 { 0.0 } else { (assign71070_e107538 * ((locals.var_dnm).powf(assign71070_e107538 - 1.0) * locals.var_dnm_dn5)) } } else { (assign71070_e107539 * (assign71070_e107538 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71070_e107538) as f64).is_finite() && ((assign71070_e107538) as f64).fract() == 0.0 { if assign71070_e107538 == 0.0 { 0.0 } else { (assign71070_e107538 * ((locals.var_dnm).powf(assign71070_e107538 - 1.0) * locals.var_dnm_dn6)) } } else { (assign71070_e107539 * (assign71070_e107538 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71070_e107538) as f64).is_finite() && ((assign71070_e107538) as f64).fract() == 0.0 { if assign71070_e107538 == 0.0 { 0.0 } else { (assign71070_e107538 * ((locals.var_dnm).powf(assign71070_e107538 - 1.0) * locals.var_dnm_dn7)) } } else { (assign71070_e107539 * (assign71070_e107538 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71070_e107538) as f64).is_finite() && ((assign71070_e107538) as f64).fract() == 0.0 { if assign71070_e107538 == 0.0 { 0.0 } else { (assign71070_e107538 * ((locals.var_dnm).powf(assign71070_e107538 - 1.0) * locals.var_dnm_dn8)) } } else { (assign71070_e107539 * (assign71070_e107538 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71070_e107538) as f64).is_finite() && ((assign71070_e107538) as f64).fract() == 0.0 { if assign71070_e107538 == 0.0 { 0.0 } else { (assign71070_e107538 * ((locals.var_dnm).powf(assign71070_e107538 - 1.0) * locals.var_dnm_dn9)) } } else { (assign71070_e107539 * (assign71070_e107538 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71070_e107538) as f64).is_finite() && ((assign71070_e107538) as f64).fract() == 0.0 { if assign71070_e107538 == 0.0 { 0.0 } else { (assign71070_e107538 * ((locals.var_dnm).powf(assign71070_e107538 - 1.0) * locals.var_dnm_dn10)) } } else { (assign71070_e107539 * (assign71070_e107538 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71070_e107538) as f64).is_finite() && ((assign71070_e107538) as f64).fract() == 0.0 { if assign71070_e107538 == 0.0 { 0.0 } else { (assign71070_e107538 * ((locals.var_dnm).powf(assign71070_e107538 - 1.0) * locals.var_dnm_dn13)) } } else { (assign71070_e107539 * (assign71070_e107538 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign71070_e107540, assign71070_e107540_d_n0, assign71070_e107540_d_n2, assign71070_e107540_d_n4, assign71070_e107540_d_n5, assign71070_e107540_d_n6, assign71070_e107540_d_n7, assign71070_e107540_d_n8, assign71070_e107540_d_n9, assign71070_e107540_d_n10, assign71070_e107540_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign71070_e107542;
        locals.var_dnm_dn0 = assign71070_e107542_d_n0;
        locals.var_dnm_dn2 = assign71070_e107542_d_n2;
        locals.var_dnm_dn4 = assign71070_e107542_d_n4;
        locals.var_dnm_dn5 = assign71070_e107542_d_n5;
        locals.var_dnm_dn6 = assign71070_e107542_d_n6;
        locals.var_dnm_dn7 = assign71070_e107542_d_n7;
        locals.var_dnm_dn8 = assign71070_e107542_d_n8;
        locals.var_dnm_dn9 = assign71070_e107542_d_n9;
        locals.var_dnm_dn10 = assign71070_e107542_d_n10;
        locals.var_dnm_dn13 = assign71070_e107542_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign71080_e107552, assign71080_e107552_d_n0, assign71080_e107552_d_n2, assign71080_e107552_d_n4, assign71080_e107552_d_n5, assign71080_e107552_d_n6, assign71080_e107552_d_n7, assign71080_e107552_d_n8, assign71080_e107552_d_n9, assign71080_e107552_d_n10, assign71080_e107552_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign71080_e107550: f64 = (1.0 / locals.var_dnm);
        (assign71080_e107550, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign71080_e107552;
        locals.var_dnm_dn0 = assign71080_e107552_d_n0;
        locals.var_dnm_dn2 = assign71080_e107552_d_n2;
        locals.var_dnm_dn4 = assign71080_e107552_d_n4;
        locals.var_dnm_dn5 = assign71080_e107552_d_n5;
        locals.var_dnm_dn6 = assign71080_e107552_d_n6;
        locals.var_dnm_dn7 = assign71080_e107552_d_n7;
        locals.var_dnm_dn8 = assign71080_e107552_d_n8;
        locals.var_dnm_dn9 = assign71080_e107552_d_n9;
        locals.var_dnm_dn10 = assign71080_e107552_d_n10;
        locals.var_dnm_dn13 = assign71080_e107552_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign71090_e107564, assign71090_e107564_d_n0, assign71090_e107564_d_n2, assign71090_e107564_d_n4, assign71090_e107564_d_n5, assign71090_e107564_d_n6, assign71090_e107564_d_n7, assign71090_e107564_d_n8, assign71090_e107564_d_n9, assign71090_e107564_d_n10, assign71090_e107564_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign71090_e107560: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign71090_e107562: f64 = (assign71090_e107560 * locals.var_dnm);
        (assign71090_e107562, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign71090_e107560 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign71090_e107560 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign71090_e107560 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign71090_e107560 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign71090_e107560 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign71090_e107560 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign71090_e107560 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign71090_e107560 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign71090_e107560 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn13)) * locals.var_dnm) + (assign71090_e107560 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign71090_e107564;
        locals.var_tmf0_dn0 = assign71090_e107564_d_n0;
        locals.var_tmf0_dn2 = assign71090_e107564_d_n2;
        locals.var_tmf0_dn4 = assign71090_e107564_d_n4;
        locals.var_tmf0_dn5 = assign71090_e107564_d_n5;
        locals.var_tmf0_dn6 = assign71090_e107564_d_n6;
        locals.var_tmf0_dn7 = assign71090_e107564_d_n7;
        locals.var_tmf0_dn8 = assign71090_e107564_d_n8;
        locals.var_tmf0_dn9 = assign71090_e107564_d_n9;
        locals.var_tmf0_dn10 = assign71090_e107564_d_n10;
        locals.var_tmf0_dn13 = assign71090_e107564_d_n13;
        locals.var_tmf0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_256(
        locals: &mut StampLocals,
    ) {
        let (assign71100_e107578, assign71100_e107578_d_n0, assign71100_e107578_d_n2, assign71100_e107578_d_n4, assign71100_e107578_d_n5, assign71100_e107578_d_n6, assign71100_e107578_d_n7, assign71100_e107578_d_n8, assign71100_e107578_d_n9, assign71100_e107578_d_n10, assign71100_e107578_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign71100_e107572: f64 = (locals.var_t1 * locals.var_xmp);
        let assign71100_e107574: f64 = (assign71100_e107572 * locals.var_dnm);
        let assign71100_e107576: f64 = (assign71100_e107574 / locals.var_arg);
        (assign71100_e107576, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign71100_e107572 * locals.var_dnm_dn0)) * locals.var_arg) - (assign71100_e107574 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign71100_e107572 * locals.var_dnm_dn2)) * locals.var_arg) - (assign71100_e107574 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign71100_e107572 * locals.var_dnm_dn4)) * locals.var_arg) - (assign71100_e107574 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign71100_e107572 * locals.var_dnm_dn5)) * locals.var_arg) - (assign71100_e107574 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign71100_e107572 * locals.var_dnm_dn6)) * locals.var_arg) - (assign71100_e107574 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign71100_e107572 * locals.var_dnm_dn7)) * locals.var_arg) - (assign71100_e107574 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign71100_e107572 * locals.var_dnm_dn8)) * locals.var_arg) - (assign71100_e107574 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign71100_e107572 * locals.var_dnm_dn9)) * locals.var_arg) - (assign71100_e107574 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign71100_e107572 * locals.var_dnm_dn10)) * locals.var_arg) - (assign71100_e107574 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn13 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign71100_e107572 * locals.var_dnm_dn13)) * locals.var_arg) - (assign71100_e107574 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign71100_e107578;
        locals.var_t0_dn0 = assign71100_e107578_d_n0;
        locals.var_t0_dn2 = assign71100_e107578_d_n2;
        locals.var_t0_dn4 = assign71100_e107578_d_n4;
        locals.var_t0_dn5 = assign71100_e107578_d_n5;
        locals.var_t0_dn6 = assign71100_e107578_d_n6;
        locals.var_t0_dn7 = assign71100_e107578_d_n7;
        locals.var_t0_dn8 = assign71100_e107578_d_n8;
        locals.var_t0_dn9 = assign71100_e107578_d_n9;
        locals.var_t0_dn10 = assign71100_e107578_d_n10;
        locals.var_t0_dn13 = assign71100_e107578_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign71110_e107590, assign71110_e107590_d_n0, assign71110_e107590_d_n2, assign71110_e107590_d_n4, assign71110_e107590_d_n5, assign71110_e107590_d_n6, assign71110_e107590_d_n7, assign71110_e107590_d_n8, assign71110_e107590_d_n9, assign71110_e107590_d_n10, assign71110_e107590_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign71110_e107586: f64 = (-locals.var_t1);
        let assign71110_e107588: f64 = (assign71110_e107586 + locals.var_tmf0);
        (assign71110_e107588, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign71110_e107590;
        locals.var_t1_dn0 = assign71110_e107590_d_n0;
        locals.var_t1_dn2 = assign71110_e107590_d_n2;
        locals.var_t1_dn4 = assign71110_e107590_d_n4;
        locals.var_t1_dn5 = assign71110_e107590_d_n5;
        locals.var_t1_dn6 = assign71110_e107590_d_n6;
        locals.var_t1_dn7 = assign71110_e107590_d_n7;
        locals.var_t1_dn8 = assign71110_e107590_d_n8;
        locals.var_t1_dn9 = assign71110_e107590_d_n9;
        locals.var_t1_dn10 = assign71110_e107590_d_n10;
        locals.var_t1_dn13 = assign71110_e107590_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign71120_e107598, assign71120_e107598_d_n0, assign71120_e107598_d_n2, assign71120_e107598_d_n4, assign71120_e107598_d_n5, assign71120_e107598_d_n6, assign71120_e107598_d_n7, assign71120_e107598_d_n8, assign71120_e107598_d_n9, assign71120_e107598_d_n10, assign71120_e107598_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign71120_e107598;
        locals.var_t0_dn0 = assign71120_e107598_d_n0;
        locals.var_t0_dn2 = assign71120_e107598_d_n2;
        locals.var_t0_dn4 = assign71120_e107598_d_n4;
        locals.var_t0_dn5 = assign71120_e107598_d_n5;
        locals.var_t0_dn6 = assign71120_e107598_d_n6;
        locals.var_t0_dn7 = assign71120_e107598_d_n7;
        locals.var_t0_dn8 = assign71120_e107598_d_n8;
        locals.var_t0_dn9 = assign71120_e107598_d_n9;
        locals.var_t0_dn10 = assign71120_e107598_d_n10;
        locals.var_t0_dn13 = assign71120_e107598_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign71130_e107609, assign71130_e107609_d_n0, assign71130_e107609_d_n2, assign71130_e107609_d_n4, assign71130_e107609_d_n5, assign71130_e107609_d_n6, assign71130_e107609_d_n7, assign71130_e107609_d_n8, assign71130_e107609_d_n9, assign71130_e107609_d_n10, assign71130_e107609_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 == 0.0)) {
        let assign71130_e107607: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign71130_e107607, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign71130_e107609;
        locals.var_t1_dn0 = assign71130_e107609_d_n0;
        locals.var_t1_dn2 = assign71130_e107609_d_n2;
        locals.var_t1_dn4 = assign71130_e107609_d_n4;
        locals.var_t1_dn5 = assign71130_e107609_d_n5;
        locals.var_t1_dn6 = assign71130_e107609_d_n6;
        locals.var_t1_dn7 = assign71130_e107609_d_n7;
        locals.var_t1_dn8 = assign71130_e107609_d_n8;
        locals.var_t1_dn9 = assign71130_e107609_d_n9;
        locals.var_t1_dn10 = assign71130_e107609_d_n10;
        locals.var_t1_dn13 = assign71130_e107609_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign71140_e107618, assign71140_e107618_d_n0, assign71140_e107618_d_n2, assign71140_e107618_d_n4, assign71140_e107618_d_n5, assign71140_e107618_d_n6, assign71140_e107618_d_n7, assign71140_e107618_d_n8, assign71140_e107618_d_n9, assign71140_e107618_d_n10, assign71140_e107618_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1659 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign71140_e107618;
        locals.var_t0_dn0 = assign71140_e107618_d_n0;
        locals.var_t0_dn2 = assign71140_e107618_d_n2;
        locals.var_t0_dn4 = assign71140_e107618_d_n4;
        locals.var_t0_dn5 = assign71140_e107618_d_n5;
        locals.var_t0_dn6 = assign71140_e107618_d_n6;
        locals.var_t0_dn7 = assign71140_e107618_d_n7;
        locals.var_t0_dn8 = assign71140_e107618_d_n8;
        locals.var_t0_dn9 = assign71140_e107618_d_n9;
        locals.var_t0_dn10 = assign71140_e107618_d_n10;
        locals.var_t0_dn13 = assign71140_e107618_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign71150_e107626, assign71150_e107626_d_n0, assign71150_e107626_d_n2, assign71150_e107626_d_n4, assign71150_e107626_d_n5, assign71150_e107626_d_n6, assign71150_e107626_d_n7, assign71150_e107626_d_n8, assign71150_e107626_d_n9, assign71150_e107626_d_n10, assign71150_e107626_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign71150_e107624: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign71150_e107624, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, (locals.var_t1_dn6 - locals.var_vgpld_dn6), (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign71150_e107626;
        locals.var_vxbgmtcl_dn0 = assign71150_e107626_d_n0;
        locals.var_vxbgmtcl_dn2 = assign71150_e107626_d_n2;
        locals.var_vxbgmtcl_dn4 = assign71150_e107626_d_n4;
        locals.var_vxbgmtcl_dn5 = assign71150_e107626_d_n5;
        locals.var_vxbgmtcl_dn6 = assign71150_e107626_d_n6;
        locals.var_vxbgmtcl_dn7 = assign71150_e107626_d_n7;
        locals.var_vxbgmtcl_dn8 = assign71150_e107626_d_n8;
        locals.var_vxbgmtcl_dn9 = assign71150_e107626_d_n9;
        locals.var_vxbgmtcl_dn10 = assign71150_e107626_d_n10;
        locals.var_vxbgmtcl_dn13 = assign71150_e107626_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign71160_e107637, assign71160_e107637_d_n0, assign71160_e107637_d_n2, assign71160_e107637_d_n4, assign71160_e107637_d_n5, assign71160_e107637_d_n6, assign71160_e107637_d_n7, assign71160_e107637_d_n8, assign71160_e107637_d_n9, assign71160_e107637_d_n10, assign71160_e107637_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign71160_e107631: f64 = (-locals.var_vxbgmtcl);
        let assign71160_e107634: f64 = (10.0 * 2.220446049250313e-16);
        let assign71160_e107635: f64 = (assign71160_e107631 + assign71160_e107634);
        (assign71160_e107635, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn13,)
    }
};
        locals.var_vgb_fb_ld = assign71160_e107637;
        locals.var_vgb_fb_ld_dn0 = assign71160_e107637_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign71160_e107637_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign71160_e107637_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign71160_e107637_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign71160_e107637_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign71160_e107637_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign71160_e107637_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign71160_e107637_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign71160_e107637_d_n10;
        locals.var_vgb_fb_ld_dn13 = assign71160_e107637_d_n13;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign71170_e107640: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard1665 = assign71170_e107640;
        locals.var_guard1665_rv = 0.0;

        let (assign71190_e107661, assign71190_e107661_d_n0, assign71190_e107661_d_n2, assign71190_e107661_d_n4, assign71190_e107661_d_n5, assign71190_e107661_d_n6, assign71190_e107661_d_n7, assign71190_e107661_d_n8, assign71190_e107661_d_n9, assign71190_e107661_d_n10, assign71190_e107661_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71190_e107653: f64 = (2.0 * locals.var_beta_inv);
        let assign71190_e107655: f64 = (-locals.var_vgs_min);
        let assign71190_e107657: f64 = (assign71190_e107655 / locals.var_fac1);
        let assign71190_e107658: f64 = (assign71190_e107657).ln();
        let assign71190_e107659: f64 = (assign71190_e107653 * assign71190_e107658);
        (assign71190_e107659, (((2.0 * locals.var_beta_inv_dn0) * assign71190_e107658) + (assign71190_e107653 * ((-((assign71190_e107655 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign71190_e107657))), (((2.0 * locals.var_beta_inv_dn2) * assign71190_e107658) + (assign71190_e107653 * ((-((assign71190_e107655 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign71190_e107657))), (((2.0 * locals.var_beta_inv_dn4) * assign71190_e107658) + (assign71190_e107653 * ((-((assign71190_e107655 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign71190_e107657))), (((2.0 * locals.var_beta_inv_dn5) * assign71190_e107658) + (assign71190_e107653 * ((-((assign71190_e107655 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign71190_e107657))), (((2.0 * locals.var_beta_inv_dn6) * assign71190_e107658) + (assign71190_e107653 * ((-((assign71190_e107655 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign71190_e107657))), (((2.0 * locals.var_beta_inv_dn7) * assign71190_e107658) + (assign71190_e107653 * ((-((assign71190_e107655 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign71190_e107657))), (((2.0 * locals.var_beta_inv_dn8) * assign71190_e107658) + (assign71190_e107653 * ((-((assign71190_e107655 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign71190_e107657))), (((2.0 * locals.var_beta_inv_dn9) * assign71190_e107658) + (assign71190_e107653 * ((-((assign71190_e107655 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign71190_e107657))), (((2.0 * locals.var_beta_inv_dn10) * assign71190_e107658) + (assign71190_e107653 * ((-((assign71190_e107655 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign71190_e107657))), (((2.0 * locals.var_beta_inv_dn13) * assign71190_e107658) + (assign71190_e107653 * ((-((assign71190_e107655 * locals.var_fac1_dn13) / (locals.var_fac1 * locals.var_fac1))) / assign71190_e107657))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn13,)
    }
};
        locals.var_ps0_min = assign71190_e107661;
        locals.var_ps0_min_dn0 = assign71190_e107661_d_n0;
        locals.var_ps0_min_dn2 = assign71190_e107661_d_n2;
        locals.var_ps0_min_dn4 = assign71190_e107661_d_n4;
        locals.var_ps0_min_dn5 = assign71190_e107661_d_n5;
        locals.var_ps0_min_dn6 = assign71190_e107661_d_n6;
        locals.var_ps0_min_dn7 = assign71190_e107661_d_n7;
        locals.var_ps0_min_dn8 = assign71190_e107661_d_n8;
        locals.var_ps0_min_dn9 = assign71190_e107661_d_n9;
        locals.var_ps0_min_dn10 = assign71190_e107661_d_n10;
        locals.var_ps0_min_dn13 = assign71190_e107661_d_n13;
        locals.var_ps0_min_rv = 0.0;

        let (assign71200_e107671, assign71200_e107671_d_n0, assign71200_e107671_d_n2, assign71200_e107671_d_n4, assign71200_e107671_d_n5, assign71200_e107671_d_n6, assign71200_e107671_d_n7, assign71200_e107671_d_n8, assign71200_e107671_d_n9, assign71200_e107671_d_n10, assign71200_e107671_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71200_e107668: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71200_e107669: f64 = (locals.var_beta * assign71200_e107668);
        (assign71200_e107669, ((locals.var_beta_dn0 * assign71200_e107668) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign71200_e107668) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71200_e107668) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign71200_e107668) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign71200_e107668) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign71200_e107668) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71200_e107668) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71200_e107668) + (locals.var_beta * locals.var_vxbgmtcl_dn9)), ((locals.var_beta_dn10 * assign71200_e107668) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn13 * assign71200_e107668) + (locals.var_beta * locals.var_vxbgmtcl_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign71200_e107671;
        locals.var_tx_dn0 = assign71200_e107671_d_n0;
        locals.var_tx_dn2 = assign71200_e107671_d_n2;
        locals.var_tx_dn4 = assign71200_e107671_d_n4;
        locals.var_tx_dn5 = assign71200_e107671_d_n5;
        locals.var_tx_dn6 = assign71200_e107671_d_n6;
        locals.var_tx_dn7 = assign71200_e107671_d_n7;
        locals.var_tx_dn8 = assign71200_e107671_d_n8;
        locals.var_tx_dn9 = assign71200_e107671_d_n9;
        locals.var_tx_dn10 = assign71200_e107671_d_n10;
        locals.var_tx_dn13 = assign71200_e107671_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign71210_e107681, assign71210_e107681_d_n0, assign71210_e107681_d_n2, assign71210_e107681_d_n4, assign71210_e107681_d_n5, assign71210_e107681_d_n6, assign71210_e107681_d_n7, assign71210_e107681_d_n8, assign71210_e107681_d_n9, assign71210_e107681_d_n10, assign71210_e107681_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71210_e107678: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign71210_e107679: f64 = (1.0 / assign71210_e107678);
        (assign71210_e107679, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign71210_e107678 * assign71210_e107678))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign71210_e107678 * assign71210_e107678))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign71210_e107678 * assign71210_e107678))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign71210_e107678 * assign71210_e107678))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign71210_e107678 * assign71210_e107678))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign71210_e107678 * assign71210_e107678))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign71210_e107678 * assign71210_e107678))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign71210_e107678 * assign71210_e107678))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign71210_e107678 * assign71210_e107678))), (-(((locals.var_beta_dn13 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn13)) / (assign71210_e107678 * assign71210_e107678))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign71210_e107681;
        locals.var_t1_dn0 = assign71210_e107681_d_n0;
        locals.var_t1_dn2 = assign71210_e107681_d_n2;
        locals.var_t1_dn4 = assign71210_e107681_d_n4;
        locals.var_t1_dn5 = assign71210_e107681_d_n5;
        locals.var_t1_dn6 = assign71210_e107681_d_n6;
        locals.var_t1_dn7 = assign71210_e107681_d_n7;
        locals.var_t1_dn8 = assign71210_e107681_d_n8;
        locals.var_t1_dn9 = assign71210_e107681_d_n9;
        locals.var_t1_dn10 = assign71210_e107681_d_n10;
        locals.var_t1_dn13 = assign71210_e107681_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign71220_e107689, assign71220_e107689_d_n0, assign71220_e107689_d_n2, assign71220_e107689_d_n4, assign71220_e107689_d_n5, assign71220_e107689_d_n6, assign71220_e107689_d_n7, assign71220_e107689_d_n8, assign71220_e107689_d_n9, assign71220_e107689_d_n10, assign71220_e107689_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71220_e107687: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign71220_e107687, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn13 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign71220_e107689;
        locals.var_ty_dn0 = assign71220_e107689_d_n0;
        locals.var_ty_dn2 = assign71220_e107689_d_n2;
        locals.var_ty_dn4 = assign71220_e107689_d_n4;
        locals.var_ty_dn5 = assign71220_e107689_d_n5;
        locals.var_ty_dn6 = assign71220_e107689_d_n6;
        locals.var_ty_dn7 = assign71220_e107689_d_n7;
        locals.var_ty_dn8 = assign71220_e107689_d_n8;
        locals.var_ty_dn9 = assign71220_e107689_d_n9;
        locals.var_ty_dn10 = assign71220_e107689_d_n10;
        locals.var_ty_dn13 = assign71220_e107689_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign71230_e107701, assign71230_e107701_d_n0, assign71230_e107701_d_n2, assign71230_e107701_d_n4, assign71230_e107701_d_n5, assign71230_e107701_d_n6, assign71230_e107701_d_n7, assign71230_e107701_d_n8, assign71230_e107701_d_n9, assign71230_e107701_d_n10, assign71230_e107701_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71230_e107696: f64 = (3.0 * 1.414213562373095);
        let assign71230_e107698: f64 = (assign71230_e107696 * locals.var_ty);
        let assign71230_e107699: f64 = (2.0 + assign71230_e107698);
        (assign71230_e107699, (assign71230_e107696 * locals.var_ty_dn0), (assign71230_e107696 * locals.var_ty_dn2), (assign71230_e107696 * locals.var_ty_dn4), (assign71230_e107696 * locals.var_ty_dn5), (assign71230_e107696 * locals.var_ty_dn6), (assign71230_e107696 * locals.var_ty_dn7), (assign71230_e107696 * locals.var_ty_dn8), (assign71230_e107696 * locals.var_ty_dn9), (assign71230_e107696 * locals.var_ty_dn10), (assign71230_e107696 * locals.var_ty_dn13),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn13,)
    }
};
        locals.var_ac41 = assign71230_e107701;
        locals.var_ac41_dn0 = assign71230_e107701_d_n0;
        locals.var_ac41_dn2 = assign71230_e107701_d_n2;
        locals.var_ac41_dn4 = assign71230_e107701_d_n4;
        locals.var_ac41_dn5 = assign71230_e107701_d_n5;
        locals.var_ac41_dn6 = assign71230_e107701_d_n6;
        locals.var_ac41_dn7 = assign71230_e107701_d_n7;
        locals.var_ac41_dn8 = assign71230_e107701_d_n8;
        locals.var_ac41_dn9 = assign71230_e107701_d_n9;
        locals.var_ac41_dn10 = assign71230_e107701_d_n10;
        locals.var_ac41_dn13 = assign71230_e107701_d_n13;
        locals.var_ac41_rv = 0.0;

        let (assign71240_e107713, assign71240_e107713_d_n0, assign71240_e107713_d_n2, assign71240_e107713_d_n4, assign71240_e107713_d_n5, assign71240_e107713_d_n6, assign71240_e107713_d_n7, assign71240_e107713_d_n8, assign71240_e107713_d_n9, assign71240_e107713_d_n10, assign71240_e107713_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71240_e107707: f64 = (8.0 * locals.var_ac41);
        let assign71240_e107709: f64 = (assign71240_e107707 * locals.var_ac41);
        let assign71240_e107711: f64 = (assign71240_e107709 * locals.var_ac41);
        (assign71240_e107711, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign71240_e107707 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign71240_e107709 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign71240_e107707 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign71240_e107709 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign71240_e107707 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign71240_e107709 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign71240_e107707 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign71240_e107709 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign71240_e107707 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign71240_e107709 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign71240_e107707 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign71240_e107709 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign71240_e107707 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign71240_e107709 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign71240_e107707 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign71240_e107709 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign71240_e107707 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign71240_e107709 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn13) * locals.var_ac41) + (assign71240_e107707 * locals.var_ac41_dn13)) * locals.var_ac41) + (assign71240_e107709 * locals.var_ac41_dn13)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn13,)
    }
};
        locals.var_ac4 = assign71240_e107713;
        locals.var_ac4_dn0 = assign71240_e107713_d_n0;
        locals.var_ac4_dn2 = assign71240_e107713_d_n2;
        locals.var_ac4_dn4 = assign71240_e107713_d_n4;
        locals.var_ac4_dn5 = assign71240_e107713_d_n5;
        locals.var_ac4_dn6 = assign71240_e107713_d_n6;
        locals.var_ac4_dn7 = assign71240_e107713_d_n7;
        locals.var_ac4_dn8 = assign71240_e107713_d_n8;
        locals.var_ac4_dn9 = assign71240_e107713_d_n9;
        locals.var_ac4_dn10 = assign71240_e107713_d_n10;
        locals.var_ac4_dn13 = assign71240_e107713_d_n13;
        locals.var_ac4_rv = 0.0;

        let (assign71250_e107729, assign71250_e107729_d_n0, assign71250_e107729_d_n2, assign71250_e107729_d_n4, assign71250_e107729_d_n5, assign71250_e107729_d_n6, assign71250_e107729_d_n7, assign71250_e107729_d_n8, assign71250_e107729_d_n9, assign71250_e107729_d_n10, assign71250_e107729_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71250_e107719: f64 = (7.0 * 1.414213562373095);
        let assign71250_e107722: f64 = (9.0 * locals.var_ty);
        let assign71250_e107725: f64 = (locals.var_tx - 2.0);
        let assign71250_e107726: f64 = (assign71250_e107722 * assign71250_e107725);
        let assign71250_e107727: f64 = (assign71250_e107719 - assign71250_e107726);
        (assign71250_e107727, (-(((9.0 * locals.var_ty_dn0) * assign71250_e107725) + (assign71250_e107722 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign71250_e107725) + (assign71250_e107722 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign71250_e107725) + (assign71250_e107722 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign71250_e107725) + (assign71250_e107722 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign71250_e107725) + (assign71250_e107722 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign71250_e107725) + (assign71250_e107722 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign71250_e107725) + (assign71250_e107722 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign71250_e107725) + (assign71250_e107722 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign71250_e107725) + (assign71250_e107722 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn13) * assign71250_e107725) + (assign71250_e107722 * locals.var_tx_dn13))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn13,)
    }
};
        locals.var_ac31 = assign71250_e107729;
        locals.var_ac31_dn0 = assign71250_e107729_d_n0;
        locals.var_ac31_dn2 = assign71250_e107729_d_n2;
        locals.var_ac31_dn4 = assign71250_e107729_d_n4;
        locals.var_ac31_dn5 = assign71250_e107729_d_n5;
        locals.var_ac31_dn6 = assign71250_e107729_d_n6;
        locals.var_ac31_dn7 = assign71250_e107729_d_n7;
        locals.var_ac31_dn8 = assign71250_e107729_d_n8;
        locals.var_ac31_dn9 = assign71250_e107729_d_n9;
        locals.var_ac31_dn10 = assign71250_e107729_d_n10;
        locals.var_ac31_dn13 = assign71250_e107729_d_n13;
        locals.var_ac31_rv = 0.0;

        let (assign71260_e107737, assign71260_e107737_d_n0, assign71260_e107737_d_n2, assign71260_e107737_d_n4, assign71260_e107737_d_n5, assign71260_e107737_d_n6, assign71260_e107737_d_n7, assign71260_e107737_d_n8, assign71260_e107737_d_n9, assign71260_e107737_d_n10, assign71260_e107737_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71260_e107735: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign71260_e107735, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn13 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn13)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn13,)
    }
};
        locals.var_ac3 = assign71260_e107737;
        locals.var_ac3_dn0 = assign71260_e107737_d_n0;
        locals.var_ac3_dn2 = assign71260_e107737_d_n2;
        locals.var_ac3_dn4 = assign71260_e107737_d_n4;
        locals.var_ac3_dn5 = assign71260_e107737_d_n5;
        locals.var_ac3_dn6 = assign71260_e107737_d_n6;
        locals.var_ac3_dn7 = assign71260_e107737_d_n7;
        locals.var_ac3_dn8 = assign71260_e107737_d_n8;
        locals.var_ac3_dn9 = assign71260_e107737_d_n9;
        locals.var_ac3_dn10 = assign71260_e107737_d_n10;
        locals.var_ac3_dn13 = assign71260_e107737_d_n13;
        locals.var_ac3_rv = 0.0;

        let assign71270_e107741: f64 = (locals.var_ac3 * 1e-8);
        let assign71270_e107742: f64 = if locals.var_ac4 < assign71270_e107741 { 1.0 } else { 0.0 };
        locals.var_guard1666 = assign71270_e107742;
        locals.var_guard1666_rv = 0.0;

        let (assign71290_e107763, assign71290_e107763_d_n0, assign71290_e107763_d_n2, assign71290_e107763_d_n4, assign71290_e107763_d_n5, assign71290_e107763_d_n6, assign71290_e107763_d_n7, assign71290_e107763_d_n8, assign71290_e107763_d_n9, assign71290_e107763_d_n10, assign71290_e107763_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) && (locals.var_guard1666 != 0.0)) {
        let assign71290_e107759: f64 = (0.5 * locals.var_ac4);
        let assign71290_e107761: f64 = (assign71290_e107759 / locals.var_ac31);
        (assign71290_e107761, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign71290_e107759 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign71290_e107759 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign71290_e107759 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign71290_e107759 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign71290_e107759 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign71290_e107759 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign71290_e107759 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign71290_e107759 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign71290_e107759 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn13) * locals.var_ac31) - (assign71290_e107759 * locals.var_ac31_dn13)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign71290_e107763;
        locals.var_ac1_dn0 = assign71290_e107763_d_n0;
        locals.var_ac1_dn2 = assign71290_e107763_d_n2;
        locals.var_ac1_dn4 = assign71290_e107763_d_n4;
        locals.var_ac1_dn5 = assign71290_e107763_d_n5;
        locals.var_ac1_dn6 = assign71290_e107763_d_n6;
        locals.var_ac1_dn7 = assign71290_e107763_d_n7;
        locals.var_ac1_dn8 = assign71290_e107763_d_n8;
        locals.var_ac1_dn9 = assign71290_e107763_d_n9;
        locals.var_ac1_dn10 = assign71290_e107763_d_n10;
        locals.var_ac1_dn13 = assign71290_e107763_d_n13;
        locals.var_ac1_rv = 0.0;

        let (assign71300_e107775, assign71300_e107775_d_n0, assign71300_e107775_d_n2, assign71300_e107775_d_n4, assign71300_e107775_d_n5, assign71300_e107775_d_n6, assign71300_e107775_d_n7, assign71300_e107775_d_n8, assign71300_e107775_d_n9, assign71300_e107775_d_n10, assign71300_e107775_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) && (locals.var_guard1666 == 0.0)) {
        let assign71300_e107772: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign71300_e107773: f64 = (assign71300_e107772).sqrt();
        (assign71300_e107773, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign71300_e107773)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign71300_e107773)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign71300_e107773)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign71300_e107773)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign71300_e107773)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign71300_e107773)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign71300_e107773)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign71300_e107773)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign71300_e107773)), ((locals.var_ac4_dn13 + locals.var_ac3_dn13) / (2.0 * assign71300_e107773)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn13,)
    }
};
        locals.var_ac2 = assign71300_e107775;
        locals.var_ac2_dn0 = assign71300_e107775_d_n0;
        locals.var_ac2_dn2 = assign71300_e107775_d_n2;
        locals.var_ac2_dn4 = assign71300_e107775_d_n4;
        locals.var_ac2_dn5 = assign71300_e107775_d_n5;
        locals.var_ac2_dn6 = assign71300_e107775_d_n6;
        locals.var_ac2_dn7 = assign71300_e107775_d_n7;
        locals.var_ac2_dn8 = assign71300_e107775_d_n8;
        locals.var_ac2_dn9 = assign71300_e107775_d_n9;
        locals.var_ac2_dn10 = assign71300_e107775_d_n10;
        locals.var_ac2_dn13 = assign71300_e107775_d_n13;
        locals.var_ac2_rv = 0.0;

        let (assign71310_e107787, assign71310_e107787_d_n0, assign71310_e107787_d_n2, assign71310_e107787_d_n4, assign71310_e107787_d_n5, assign71310_e107787_d_n6, assign71310_e107787_d_n7, assign71310_e107787_d_n8, assign71310_e107787_d_n9, assign71310_e107787_d_n10, assign71310_e107787_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) && (locals.var_guard1666 == 0.0)) {
        let assign71310_e107783: f64 = (-locals.var_ac31);
        let assign71310_e107785: f64 = (assign71310_e107783 + locals.var_ac2);
        (assign71310_e107785, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn13) + locals.var_ac2_dn13),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign71310_e107787;
        locals.var_ac1_dn0 = assign71310_e107787_d_n0;
        locals.var_ac1_dn2 = assign71310_e107787_d_n2;
        locals.var_ac1_dn4 = assign71310_e107787_d_n4;
        locals.var_ac1_dn5 = assign71310_e107787_d_n5;
        locals.var_ac1_dn6 = assign71310_e107787_d_n6;
        locals.var_ac1_dn7 = assign71310_e107787_d_n7;
        locals.var_ac1_dn8 = assign71310_e107787_d_n8;
        locals.var_ac1_dn9 = assign71310_e107787_d_n9;
        locals.var_ac1_dn10 = assign71310_e107787_d_n10;
        locals.var_ac1_dn13 = assign71310_e107787_d_n13;
        locals.var_ac1_rv = 0.0;

        let (assign71320_e107795, assign71320_e107795_d_n0, assign71320_e107795_d_n2, assign71320_e107795_d_n4, assign71320_e107795_d_n5, assign71320_e107795_d_n6, assign71320_e107795_d_n7, assign71320_e107795_d_n8, assign71320_e107795_d_n9, assign71320_e107795_d_n10, assign71320_e107795_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71320_e107793: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign71320_e107793, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign71320_e107793 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign71320_e107793 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign71320_e107793 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign71320_e107793 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign71320_e107793 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign71320_e107793 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign71320_e107793 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign71320_e107793 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign71320_e107793 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn13)) } } else { (assign71320_e107793 * (0.3333333333333333 * (locals.var_ac1_dn13 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn13,)
    }
};
        locals.var_acd = assign71320_e107795;
        locals.var_acd_dn0 = assign71320_e107795_d_n0;
        locals.var_acd_dn2 = assign71320_e107795_d_n2;
        locals.var_acd_dn4 = assign71320_e107795_d_n4;
        locals.var_acd_dn5 = assign71320_e107795_d_n5;
        locals.var_acd_dn6 = assign71320_e107795_d_n6;
        locals.var_acd_dn7 = assign71320_e107795_d_n7;
        locals.var_acd_dn8 = assign71320_e107795_d_n8;
        locals.var_acd_dn9 = assign71320_e107795_d_n9;
        locals.var_acd_dn10 = assign71320_e107795_d_n10;
        locals.var_acd_dn13 = assign71320_e107795_d_n13;
        locals.var_acd_rv = 0.0;

        let (assign71330_e107818, assign71330_e107818_d_n0, assign71330_e107818_d_n2, assign71330_e107818_d_n4, assign71330_e107818_d_n5, assign71330_e107818_d_n6, assign71330_e107818_d_n7, assign71330_e107818_d_n8, assign71330_e107818_d_n9, assign71330_e107818_d_n10, assign71330_e107818_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71330_e107800: f64 = (-4.0);
        let assign71330_e107802: f64 = (assign71330_e107800 * 1.414213562373095);
        let assign71330_e107805: f64 = (12.0 * locals.var_ty);
        let assign71330_e107806: f64 = (assign71330_e107802 - assign71330_e107805);
        let assign71330_e107809: f64 = (2.0 * locals.var_acd);
        let assign71330_e107810: f64 = (assign71330_e107806 + assign71330_e107809);
        let assign71330_e107813: f64 = (1.414213562373095 * locals.var_acd);
        let assign71330_e107815: f64 = (assign71330_e107813 * locals.var_acd);
        let assign71330_e107816: f64 = (assign71330_e107810 + assign71330_e107815);
        (assign71330_e107816, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign71330_e107813 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign71330_e107813 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign71330_e107813 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign71330_e107813 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign71330_e107813 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign71330_e107813 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign71330_e107813 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign71330_e107813 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign71330_e107813 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn13)) + (2.0 * locals.var_acd_dn13)) + (((1.414213562373095 * locals.var_acd_dn13) * locals.var_acd) + (assign71330_e107813 * locals.var_acd_dn13))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn13,)
    }
};
        locals.var_acn = assign71330_e107818;
        locals.var_acn_dn0 = assign71330_e107818_d_n0;
        locals.var_acn_dn2 = assign71330_e107818_d_n2;
        locals.var_acn_dn4 = assign71330_e107818_d_n4;
        locals.var_acn_dn5 = assign71330_e107818_d_n5;
        locals.var_acn_dn6 = assign71330_e107818_d_n6;
        locals.var_acn_dn7 = assign71330_e107818_d_n7;
        locals.var_acn_dn8 = assign71330_e107818_d_n8;
        locals.var_acn_dn9 = assign71330_e107818_d_n9;
        locals.var_acn_dn10 = assign71330_e107818_d_n10;
        locals.var_acn_dn13 = assign71330_e107818_d_n13;
        locals.var_acn_rv = 0.0;

        let (assign71340_e107826, assign71340_e107826_d_n0, assign71340_e107826_d_n2, assign71340_e107826_d_n4, assign71340_e107826_d_n5, assign71340_e107826_d_n6, assign71340_e107826_d_n7, assign71340_e107826_d_n8, assign71340_e107826_d_n9, assign71340_e107826_d_n10, assign71340_e107826_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71340_e107824: f64 = (locals.var_acn / locals.var_acd);
        (assign71340_e107824, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn13 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn13)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign71340_e107826;
        locals.var_chi_dn0 = assign71340_e107826_d_n0;
        locals.var_chi_dn2 = assign71340_e107826_d_n2;
        locals.var_chi_dn4 = assign71340_e107826_d_n4;
        locals.var_chi_dn5 = assign71340_e107826_d_n5;
        locals.var_chi_dn6 = assign71340_e107826_d_n6;
        locals.var_chi_dn7 = assign71340_e107826_d_n7;
        locals.var_chi_dn8 = assign71340_e107826_d_n8;
        locals.var_chi_dn9 = assign71340_e107826_d_n9;
        locals.var_chi_dn10 = assign71340_e107826_d_n10;
        locals.var_chi_dn13 = assign71340_e107826_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign71350_e107834, assign71350_e107834_d_n0, assign71350_e107834_d_n2, assign71350_e107834_d_n4, assign71350_e107834_d_n5, assign71350_e107834_d_n6, assign71350_e107834_d_n7, assign71350_e107834_d_n8, assign71350_e107834_d_n9, assign71350_e107834_d_n10, assign71350_e107834_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71350_e107832: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign71350_e107832, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign71350_e107834;
        locals.var_t1_dn0 = assign71350_e107834_d_n0;
        locals.var_t1_dn2 = assign71350_e107834_d_n2;
        locals.var_t1_dn4 = assign71350_e107834_d_n4;
        locals.var_t1_dn5 = assign71350_e107834_d_n5;
        locals.var_t1_dn6 = assign71350_e107834_d_n6;
        locals.var_t1_dn7 = assign71350_e107834_d_n7;
        locals.var_t1_dn8 = assign71350_e107834_d_n8;
        locals.var_t1_dn9 = assign71350_e107834_d_n9;
        locals.var_t1_dn10 = assign71350_e107834_d_n10;
        locals.var_t1_dn13 = assign71350_e107834_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_257(
        locals: &mut StampLocals,
    ) {
        let (assign71360_e107842, assign71360_e107842_d_n0, assign71360_e107842_d_n2, assign71360_e107842_d_n4, assign71360_e107842_d_n5, assign71360_e107842_d_n6, assign71360_e107842_d_n7, assign71360_e107842_d_n8, assign71360_e107842_d_n9, assign71360_e107842_d_n10, assign71360_e107842_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71360_e107840: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign71360_e107840, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn13 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn13)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign71360_e107842;
        locals.var_t2_dn0 = assign71360_e107842_d_n0;
        locals.var_t2_dn2 = assign71360_e107842_d_n2;
        locals.var_t2_dn4 = assign71360_e107842_d_n4;
        locals.var_t2_dn5 = assign71360_e107842_d_n5;
        locals.var_t2_dn6 = assign71360_e107842_d_n6;
        locals.var_t2_dn7 = assign71360_e107842_d_n7;
        locals.var_t2_dn8 = assign71360_e107842_d_n8;
        locals.var_t2_dn9 = assign71360_e107842_d_n9;
        locals.var_t2_dn10 = assign71360_e107842_d_n10;
        locals.var_t2_dn13 = assign71360_e107842_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign71370_e107853, assign71370_e107853_d_n0, assign71370_e107853_d_n2, assign71370_e107853_d_n4, assign71370_e107853_d_n5, assign71370_e107853_d_n6, assign71370_e107853_d_n7, assign71370_e107853_d_n8, assign71370_e107853_d_n9, assign71370_e107853_d_n10, assign71370_e107853_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71370_e107849: f64 = (locals.var_t2 * locals.var_t2);
        let assign71370_e107850: f64 = (1.0 + assign71370_e107849);
        let assign71370_e107851: f64 = (assign71370_e107850).sqrt();
        (assign71370_e107851, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign71370_e107851)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign71370_e107851)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign71370_e107851)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign71370_e107851)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign71370_e107851)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign71370_e107851)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign71370_e107851)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign71370_e107851)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign71370_e107851)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign71370_e107851)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign71370_e107853;
        locals.var_t3_dn0 = assign71370_e107853_d_n0;
        locals.var_t3_dn2 = assign71370_e107853_d_n2;
        locals.var_t3_dn4 = assign71370_e107853_d_n4;
        locals.var_t3_dn5 = assign71370_e107853_d_n5;
        locals.var_t3_dn6 = assign71370_e107853_d_n6;
        locals.var_t3_dn7 = assign71370_e107853_d_n7;
        locals.var_t3_dn8 = assign71370_e107853_d_n8;
        locals.var_t3_dn9 = assign71370_e107853_d_n9;
        locals.var_t3_dn10 = assign71370_e107853_d_n10;
        locals.var_t3_dn13 = assign71370_e107853_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign71380_e107863, assign71380_e107863_d_n0, assign71380_e107863_d_n2, assign71380_e107863_d_n4, assign71380_e107863_d_n5, assign71380_e107863_d_n6, assign71380_e107863_d_n7, assign71380_e107863_d_n8, assign71380_e107863_d_n9, assign71380_e107863_d_n10, assign71380_e107863_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71380_e107859: f64 = (locals.var_t1 / locals.var_t3);
        let assign71380_e107861: f64 = (assign71380_e107859 - locals.var_vxbgmtcl);
        (assign71380_e107861, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn13 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign71380_e107863;
        locals.var_ps0ld_dn0 = assign71380_e107863_d_n0;
        locals.var_ps0ld_dn2 = assign71380_e107863_d_n2;
        locals.var_ps0ld_dn4 = assign71380_e107863_d_n4;
        locals.var_ps0ld_dn5 = assign71380_e107863_d_n5;
        locals.var_ps0ld_dn6 = assign71380_e107863_d_n6;
        locals.var_ps0ld_dn7 = assign71380_e107863_d_n7;
        locals.var_ps0ld_dn8 = assign71380_e107863_d_n8;
        locals.var_ps0ld_dn9 = assign71380_e107863_d_n9;
        locals.var_ps0ld_dn10 = assign71380_e107863_d_n10;
        locals.var_ps0ld_dn13 = assign71380_e107863_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign71390_e107871, assign71390_e107871_d_n0, assign71390_e107871_d_n2, assign71390_e107871_d_n4, assign71390_e107871_d_n5, assign71390_e107871_d_n6, assign71390_e107871_d_n7, assign71390_e107871_d_n8, assign71390_e107871_d_n9, assign71390_e107871_d_n10, assign71390_e107871_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71390_e107869: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign71390_e107869, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign71390_e107871;
        locals.var_t2_dn0 = assign71390_e107871_d_n0;
        locals.var_t2_dn2 = assign71390_e107871_d_n2;
        locals.var_t2_dn4 = assign71390_e107871_d_n4;
        locals.var_t2_dn5 = assign71390_e107871_d_n5;
        locals.var_t2_dn6 = assign71390_e107871_d_n6;
        locals.var_t2_dn7 = assign71390_e107871_d_n7;
        locals.var_t2_dn8 = assign71390_e107871_d_n8;
        locals.var_t2_dn9 = assign71390_e107871_d_n9;
        locals.var_t2_dn10 = assign71390_e107871_d_n10;
        locals.var_t2_dn13 = assign71390_e107871_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign71400_e107879, assign71400_e107879_d_n0, assign71400_e107879_d_n2, assign71400_e107879_d_n4, assign71400_e107879_d_n5, assign71400_e107879_d_n6, assign71400_e107879_d_n7, assign71400_e107879_d_n8, assign71400_e107879_d_n9, assign71400_e107879_d_n10, assign71400_e107879_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        let assign71400_e107877: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign71400_e107877, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign71400_e107879;
        locals.var_qsuld_dn0 = assign71400_e107879_d_n0;
        locals.var_qsuld_dn2 = assign71400_e107879_d_n2;
        locals.var_qsuld_dn4 = assign71400_e107879_d_n4;
        locals.var_qsuld_dn5 = assign71400_e107879_d_n5;
        locals.var_qsuld_dn6 = assign71400_e107879_d_n6;
        locals.var_qsuld_dn7 = assign71400_e107879_d_n7;
        locals.var_qsuld_dn8 = assign71400_e107879_d_n8;
        locals.var_qsuld_dn9 = assign71400_e107879_d_n9;
        locals.var_qsuld_dn10 = assign71400_e107879_d_n10;
        locals.var_qsuld_dn13 = assign71400_e107879_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign71410_e107885, assign71410_e107885_d_n0, assign71410_e107885_d_n2, assign71410_e107885_d_n4, assign71410_e107885_d_n5, assign71410_e107885_d_n6, assign71410_e107885_d_n7, assign71410_e107885_d_n8, assign71410_e107885_d_n9, assign71410_e107885_d_n10, assign71410_e107885_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign71410_e107885;
        locals.var_qbuld_dn0 = assign71410_e107885_d_n0;
        locals.var_qbuld_dn2 = assign71410_e107885_d_n2;
        locals.var_qbuld_dn4 = assign71410_e107885_d_n4;
        locals.var_qbuld_dn5 = assign71410_e107885_d_n5;
        locals.var_qbuld_dn6 = assign71410_e107885_d_n6;
        locals.var_qbuld_dn7 = assign71410_e107885_d_n7;
        locals.var_qbuld_dn8 = assign71410_e107885_d_n8;
        locals.var_qbuld_dn9 = assign71410_e107885_d_n9;
        locals.var_qbuld_dn10 = assign71410_e107885_d_n10;
        locals.var_qbuld_dn13 = assign71410_e107885_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign71420_e107891, assign71420_e107891_d_n0, assign71420_e107891_d_n2, assign71420_e107891_d_n4, assign71420_e107891_d_n5, assign71420_e107891_d_n6, assign71420_e107891_d_n7, assign71420_e107891_d_n8, assign71420_e107891_d_n9, assign71420_e107891_d_n10, assign71420_e107891_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn13,)
    }
};
        locals.var_ps0ld_ini = assign71420_e107891;
        locals.var_ps0ld_ini_dn0 = assign71420_e107891_d_n0;
        locals.var_ps0ld_ini_dn2 = assign71420_e107891_d_n2;
        locals.var_ps0ld_ini_dn4 = assign71420_e107891_d_n4;
        locals.var_ps0ld_ini_dn5 = assign71420_e107891_d_n5;
        locals.var_ps0ld_ini_dn6 = assign71420_e107891_d_n6;
        locals.var_ps0ld_ini_dn7 = assign71420_e107891_d_n7;
        locals.var_ps0ld_ini_dn8 = assign71420_e107891_d_n8;
        locals.var_ps0ld_ini_dn9 = assign71420_e107891_d_n9;
        locals.var_ps0ld_ini_dn10 = assign71420_e107891_d_n10;
        locals.var_ps0ld_ini_dn13 = assign71420_e107891_d_n13;
        locals.var_ps0ld_ini_rv = 0.0;

        let assign71430_e107895: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71430_e107896: f64 = (locals.var_beta * assign71430_e107895);
        let assign71430_e107900: f64 = (10.0 * 2.220446049250313e-16);
        let assign71430_e107902: f64 = (assign71430_e107900 - 1.0);
        let assign71430_e107904: f64 = (assign71430_e107902 * locals.var_fac1p2);
        let assign71430_e107906: f64 = (assign71430_e107904 * locals.var_beta2);
        let assign71430_e107908: f64 = (assign71430_e107906 / 4.0);
        let assign71430_e107909: f64 = (1.0 + assign71430_e107908);
        let assign71430_e107910: f64 = if assign71430_e107896 < assign71430_e107909 { 1.0 } else { 0.0 };
        locals.var_guard1667 = assign71430_e107910;
        locals.var_guard1667_rv = 0.0;

        let (assign71440_e107925, assign71440_e107925_d_n0, assign71440_e107925_d_n2, assign71440_e107925_d_n4, assign71440_e107925_d_n5, assign71440_e107925_d_n6, assign71440_e107925_d_n7, assign71440_e107925_d_n8, assign71440_e107925_d_n9, assign71440_e107925_d_n10, assign71440_e107925_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1667 != 0.0)) {
        let assign71440_e107920: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71440_e107922: f64 = (assign71440_e107920 / 2.0);
        let assign71440_e107923: f64 = (locals.var_vgpld + assign71440_e107922);
        (assign71440_e107923, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (locals.var_vgpld_dn6 + (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0)), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign71440_e107925;
        locals.var_ps0_inia_dn0 = assign71440_e107925_d_n0;
        locals.var_ps0_inia_dn2 = assign71440_e107925_d_n2;
        locals.var_ps0_inia_dn4 = assign71440_e107925_d_n4;
        locals.var_ps0_inia_dn5 = assign71440_e107925_d_n5;
        locals.var_ps0_inia_dn6 = assign71440_e107925_d_n6;
        locals.var_ps0_inia_dn7 = assign71440_e107925_d_n7;
        locals.var_ps0_inia_dn8 = assign71440_e107925_d_n8;
        locals.var_ps0_inia_dn9 = assign71440_e107925_d_n9;
        locals.var_ps0_inia_dn10 = assign71440_e107925_d_n10;
        locals.var_ps0_inia_dn13 = assign71440_e107925_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign71450_e107949, assign71450_e107949_d_n0, assign71450_e107949_d_n2, assign71450_e107949_d_n4, assign71450_e107949_d_n5, assign71450_e107949_d_n6, assign71450_e107949_d_n7, assign71450_e107949_d_n8, assign71450_e107949_d_n9, assign71450_e107949_d_n10, assign71450_e107949_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1667 == 0.0)) {
        let assign71450_e107938: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71450_e107939: f64 = (locals.var_beta * assign71450_e107938);
        let assign71450_e107941: f64 = (assign71450_e107939 - 1.0);
        let assign71450_e107942: f64 = (4.0 * assign71450_e107941);
        let assign71450_e107945: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign71450_e107946: f64 = (assign71450_e107942 / assign71450_e107945);
        let assign71450_e107947: f64 = (1.0 + assign71450_e107946);
        (assign71450_e107947, ((((4.0 * ((locals.var_beta_dn0 * assign71450_e107938) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign71450_e107945) - (assign71450_e107942 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign71450_e107945 * assign71450_e107945)), ((((4.0 * ((locals.var_beta_dn2 * assign71450_e107938) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign71450_e107945) - (assign71450_e107942 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign71450_e107945 * assign71450_e107945)), ((((4.0 * ((locals.var_beta_dn4 * assign71450_e107938) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign71450_e107945) - (assign71450_e107942 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign71450_e107945 * assign71450_e107945)), ((((4.0 * ((locals.var_beta_dn5 * assign71450_e107938) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign71450_e107945) - (assign71450_e107942 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign71450_e107945 * assign71450_e107945)), ((((4.0 * ((locals.var_beta_dn6 * assign71450_e107938) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)))) * assign71450_e107945) - (assign71450_e107942 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign71450_e107945 * assign71450_e107945)), ((((4.0 * ((locals.var_beta_dn7 * assign71450_e107938) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign71450_e107945) - (assign71450_e107942 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign71450_e107945 * assign71450_e107945)), ((((4.0 * ((locals.var_beta_dn8 * assign71450_e107938) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign71450_e107945) - (assign71450_e107942 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign71450_e107945 * assign71450_e107945)), ((((4.0 * ((locals.var_beta_dn9 * assign71450_e107938) + (locals.var_beta * locals.var_vxbgmtcl_dn9))) * assign71450_e107945) - (assign71450_e107942 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign71450_e107945 * assign71450_e107945)), ((((4.0 * ((locals.var_beta_dn10 * assign71450_e107938) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign71450_e107945) - (assign71450_e107942 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign71450_e107945 * assign71450_e107945)), ((((4.0 * ((locals.var_beta_dn13 * assign71450_e107938) + (locals.var_beta * locals.var_vxbgmtcl_dn13))) * assign71450_e107945) - (assign71450_e107942 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign71450_e107945 * assign71450_e107945)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign71450_e107949;
        locals.var_tx_dn0 = assign71450_e107949_d_n0;
        locals.var_tx_dn2 = assign71450_e107949_d_n2;
        locals.var_tx_dn4 = assign71450_e107949_d_n4;
        locals.var_tx_dn5 = assign71450_e107949_d_n5;
        locals.var_tx_dn6 = assign71450_e107949_d_n6;
        locals.var_tx_dn7 = assign71450_e107949_d_n7;
        locals.var_tx_dn8 = assign71450_e107949_d_n8;
        locals.var_tx_dn9 = assign71450_e107949_d_n9;
        locals.var_tx_dn10 = assign71450_e107949_d_n10;
        locals.var_tx_dn13 = assign71450_e107949_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign71460_e107970, assign71460_e107970_d_n0, assign71460_e107970_d_n2, assign71460_e107970_d_n4, assign71460_e107970_d_n5, assign71460_e107970_d_n6, assign71460_e107970_d_n7, assign71460_e107970_d_n8, assign71460_e107970_d_n9, assign71460_e107970_d_n10, assign71460_e107970_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1667 == 0.0)) {
        let assign71460_e107960: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71460_e107962: f64 = (assign71460_e107960 / 2.0);
        let assign71460_e107965: f64 = (locals.var_tx).sqrt();
        let assign71460_e107966: f64 = (1.0 - assign71460_e107965);
        let assign71460_e107967: f64 = (assign71460_e107962 * assign71460_e107966);
        let assign71460_e107968: f64 = (locals.var_vgpld + assign71460_e107967);
        (assign71460_e107968, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign71460_e107966) + (assign71460_e107962 * (-(locals.var_tx_dn0 / (2.0 * assign71460_e107965))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign71460_e107966) + (assign71460_e107962 * (-(locals.var_tx_dn2 / (2.0 * assign71460_e107965)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign71460_e107966) + (assign71460_e107962 * (-(locals.var_tx_dn4 / (2.0 * assign71460_e107965))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign71460_e107966) + (assign71460_e107962 * (-(locals.var_tx_dn5 / (2.0 * assign71460_e107965))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign71460_e107966) + (assign71460_e107962 * (-(locals.var_tx_dn6 / (2.0 * assign71460_e107965)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign71460_e107966) + (assign71460_e107962 * (-(locals.var_tx_dn7 / (2.0 * assign71460_e107965)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign71460_e107966) + (assign71460_e107962 * (-(locals.var_tx_dn8 / (2.0 * assign71460_e107965)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign71460_e107966) + (assign71460_e107962 * (-(locals.var_tx_dn9 / (2.0 * assign71460_e107965))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign71460_e107966) + (assign71460_e107962 * (-(locals.var_tx_dn10 / (2.0 * assign71460_e107965))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign71460_e107966) + (assign71460_e107962 * (-(locals.var_tx_dn13 / (2.0 * assign71460_e107965))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign71460_e107970;
        locals.var_ps0_inia_dn0 = assign71460_e107970_d_n0;
        locals.var_ps0_inia_dn2 = assign71460_e107970_d_n2;
        locals.var_ps0_inia_dn4 = assign71460_e107970_d_n4;
        locals.var_ps0_inia_dn5 = assign71460_e107970_d_n5;
        locals.var_ps0_inia_dn6 = assign71460_e107970_d_n6;
        locals.var_ps0_inia_dn7 = assign71460_e107970_d_n7;
        locals.var_ps0_inia_dn8 = assign71460_e107970_d_n8;
        locals.var_ps0_inia_dn9 = assign71460_e107970_d_n9;
        locals.var_ps0_inia_dn10 = assign71460_e107970_d_n10;
        locals.var_ps0_inia_dn13 = assign71460_e107970_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign71470_e107981, assign71470_e107981_d_n0, assign71470_e107981_d_n2, assign71470_e107981_d_n4, assign71470_e107981_d_n5, assign71470_e107981_d_n6, assign71470_e107981_d_n7, assign71470_e107981_d_n8, assign71470_e107981_d_n9, assign71470_e107981_d_n10, assign71470_e107981_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) {
        let assign71470_e107978: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign71470_e107979: f64 = (locals.var_beta * assign71470_e107978);
        (assign71470_e107979, ((locals.var_beta_dn0 * assign71470_e107978) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign71470_e107978) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71470_e107978) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign71470_e107978) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign71470_e107978) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign71470_e107978) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71470_e107978) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71470_e107978) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign71470_e107978) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign71470_e107978) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign71470_e107981;
        locals.var_chi_dn0 = assign71470_e107981_d_n0;
        locals.var_chi_dn2 = assign71470_e107981_d_n2;
        locals.var_chi_dn4 = assign71470_e107981_d_n4;
        locals.var_chi_dn5 = assign71470_e107981_d_n5;
        locals.var_chi_dn6 = assign71470_e107981_d_n6;
        locals.var_chi_dn7 = assign71470_e107981_d_n7;
        locals.var_chi_dn8 = assign71470_e107981_d_n8;
        locals.var_chi_dn9 = assign71470_e107981_d_n9;
        locals.var_chi_dn10 = assign71470_e107981_d_n10;
        locals.var_chi_dn13 = assign71470_e107981_d_n13;
        locals.var_chi_rv = 0.0;

        let assign71480_e107984: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1668 = assign71480_e107984;
        locals.var_guard1668_rv = 0.0;

        let (assign71500_e108004, assign71500_e108004_d_n0, assign71500_e108004_d_n2, assign71500_e108004_d_n4, assign71500_e108004_d_n5, assign71500_e108004_d_n6, assign71500_e108004_d_n7, assign71500_e108004_d_n8, assign71500_e108004_d_n9, assign71500_e108004_d_n10, assign71500_e108004_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 != 0.0)) {
        let assign71500_e108001: f64 = (-locals.var_chi);
        let assign71500_e108002: f64 = (assign71500_e108001).exp();
        (assign71500_e108002, (assign71500_e108002 * (-locals.var_chi_dn0)), (assign71500_e108002 * (-locals.var_chi_dn2)), (assign71500_e108002 * (-locals.var_chi_dn4)), (assign71500_e108002 * (-locals.var_chi_dn5)), (assign71500_e108002 * (-locals.var_chi_dn6)), (assign71500_e108002 * (-locals.var_chi_dn7)), (assign71500_e108002 * (-locals.var_chi_dn8)), (assign71500_e108002 * (-locals.var_chi_dn9)), (assign71500_e108002 * (-locals.var_chi_dn10)), (assign71500_e108002 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign71500_e108004;
        locals.var_ty_dn0 = assign71500_e108004_d_n0;
        locals.var_ty_dn2 = assign71500_e108004_d_n2;
        locals.var_ty_dn4 = assign71500_e108004_d_n4;
        locals.var_ty_dn5 = assign71500_e108004_d_n5;
        locals.var_ty_dn6 = assign71500_e108004_d_n6;
        locals.var_ty_dn7 = assign71500_e108004_d_n7;
        locals.var_ty_dn8 = assign71500_e108004_d_n8;
        locals.var_ty_dn9 = assign71500_e108004_d_n9;
        locals.var_ty_dn10 = assign71500_e108004_d_n10;
        locals.var_ty_dn13 = assign71500_e108004_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign71510_e108029, assign71510_e108029_d_n0, assign71510_e108029_d_n2, assign71510_e108029_d_n4, assign71510_e108029_d_n5, assign71510_e108029_d_n6, assign71510_e108029_d_n7, assign71510_e108029_d_n8, assign71510_e108029_d_n9, assign71510_e108029_d_n10, assign71510_e108029_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 != 0.0)) {
        let assign71510_e108016: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71510_e108017: f64 = (locals.var_beta * assign71510_e108016);
        let assign71510_e108019: f64 = (assign71510_e108017 - 1.0);
        let assign71510_e108021: f64 = (assign71510_e108019 + locals.var_ty);
        let assign71510_e108022: f64 = (4.0 * assign71510_e108021);
        let assign71510_e108025: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign71510_e108026: f64 = (assign71510_e108022 / assign71510_e108025);
        let assign71510_e108027: f64 = (1.0 + assign71510_e108026);
        (assign71510_e108027, ((((4.0 * (((locals.var_beta_dn0 * assign71510_e108016) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign71510_e108025) - (assign71510_e108022 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign71510_e108025 * assign71510_e108025)), ((((4.0 * (((locals.var_beta_dn2 * assign71510_e108016) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign71510_e108025) - (assign71510_e108022 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign71510_e108025 * assign71510_e108025)), ((((4.0 * (((locals.var_beta_dn4 * assign71510_e108016) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign71510_e108025) - (assign71510_e108022 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign71510_e108025 * assign71510_e108025)), ((((4.0 * (((locals.var_beta_dn5 * assign71510_e108016) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign71510_e108025) - (assign71510_e108022 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign71510_e108025 * assign71510_e108025)), ((((4.0 * (((locals.var_beta_dn6 * assign71510_e108016) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))) + locals.var_ty_dn6)) * assign71510_e108025) - (assign71510_e108022 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign71510_e108025 * assign71510_e108025)), ((((4.0 * (((locals.var_beta_dn7 * assign71510_e108016) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign71510_e108025) - (assign71510_e108022 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign71510_e108025 * assign71510_e108025)), ((((4.0 * (((locals.var_beta_dn8 * assign71510_e108016) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign71510_e108025) - (assign71510_e108022 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign71510_e108025 * assign71510_e108025)), ((((4.0 * (((locals.var_beta_dn9 * assign71510_e108016) + (locals.var_beta * locals.var_vxbgmtcl_dn9)) + locals.var_ty_dn9)) * assign71510_e108025) - (assign71510_e108022 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign71510_e108025 * assign71510_e108025)), ((((4.0 * (((locals.var_beta_dn10 * assign71510_e108016) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign71510_e108025) - (assign71510_e108022 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign71510_e108025 * assign71510_e108025)), ((((4.0 * (((locals.var_beta_dn13 * assign71510_e108016) + (locals.var_beta * locals.var_vxbgmtcl_dn13)) + locals.var_ty_dn13)) * assign71510_e108025) - (assign71510_e108022 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign71510_e108025 * assign71510_e108025)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign71510_e108029;
        locals.var_tx_dn0 = assign71510_e108029_d_n0;
        locals.var_tx_dn2 = assign71510_e108029_d_n2;
        locals.var_tx_dn4 = assign71510_e108029_d_n4;
        locals.var_tx_dn5 = assign71510_e108029_d_n5;
        locals.var_tx_dn6 = assign71510_e108029_d_n6;
        locals.var_tx_dn7 = assign71510_e108029_d_n7;
        locals.var_tx_dn8 = assign71510_e108029_d_n8;
        locals.var_tx_dn9 = assign71510_e108029_d_n9;
        locals.var_tx_dn10 = assign71510_e108029_d_n10;
        locals.var_tx_dn13 = assign71510_e108029_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign71520_e108049, assign71520_e108049_d_n0, assign71520_e108049_d_n2, assign71520_e108049_d_n4, assign71520_e108049_d_n5, assign71520_e108049_d_n6, assign71520_e108049_d_n7, assign71520_e108049_d_n8, assign71520_e108049_d_n9, assign71520_e108049_d_n10, assign71520_e108049_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 != 0.0)) {
        let assign71520_e108039: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71520_e108041: f64 = (assign71520_e108039 / 2.0);
        let assign71520_e108044: f64 = (locals.var_tx).sqrt();
        let assign71520_e108045: f64 = (1.0 - assign71520_e108044);
        let assign71520_e108046: f64 = (assign71520_e108041 * assign71520_e108045);
        let assign71520_e108047: f64 = (locals.var_vgpld + assign71520_e108046);
        (assign71520_e108047, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign71520_e108045) + (assign71520_e108041 * (-(locals.var_tx_dn0 / (2.0 * assign71520_e108044))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign71520_e108045) + (assign71520_e108041 * (-(locals.var_tx_dn2 / (2.0 * assign71520_e108044)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign71520_e108045) + (assign71520_e108041 * (-(locals.var_tx_dn4 / (2.0 * assign71520_e108044))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign71520_e108045) + (assign71520_e108041 * (-(locals.var_tx_dn5 / (2.0 * assign71520_e108044))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign71520_e108045) + (assign71520_e108041 * (-(locals.var_tx_dn6 / (2.0 * assign71520_e108044)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign71520_e108045) + (assign71520_e108041 * (-(locals.var_tx_dn7 / (2.0 * assign71520_e108044)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign71520_e108045) + (assign71520_e108041 * (-(locals.var_tx_dn8 / (2.0 * assign71520_e108044)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign71520_e108045) + (assign71520_e108041 * (-(locals.var_tx_dn9 / (2.0 * assign71520_e108044))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign71520_e108045) + (assign71520_e108041 * (-(locals.var_tx_dn10 / (2.0 * assign71520_e108044))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign71520_e108045) + (assign71520_e108041 * (-(locals.var_tx_dn13 / (2.0 * assign71520_e108044))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign71520_e108049;
        locals.var_ps0_inia_dn0 = assign71520_e108049_d_n0;
        locals.var_ps0_inia_dn2 = assign71520_e108049_d_n2;
        locals.var_ps0_inia_dn4 = assign71520_e108049_d_n4;
        locals.var_ps0_inia_dn5 = assign71520_e108049_d_n5;
        locals.var_ps0_inia_dn6 = assign71520_e108049_d_n6;
        locals.var_ps0_inia_dn7 = assign71520_e108049_d_n7;
        locals.var_ps0_inia_dn8 = assign71520_e108049_d_n8;
        locals.var_ps0_inia_dn9 = assign71520_e108049_d_n9;
        locals.var_ps0_inia_dn10 = assign71520_e108049_d_n10;
        locals.var_ps0_inia_dn13 = assign71520_e108049_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign71530_e108062, assign71530_e108062_d_n0, assign71530_e108062_d_n2, assign71530_e108062_d_n4, assign71530_e108062_d_n5, assign71530_e108062_d_n6, assign71530_e108062_d_n7, assign71530_e108062_d_n8, assign71530_e108062_d_n9, assign71530_e108062_d_n10, assign71530_e108062_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 != 0.0)) {
        let assign71530_e108059: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign71530_e108060: f64 = (locals.var_beta * assign71530_e108059);
        (assign71530_e108060, ((locals.var_beta_dn0 * assign71530_e108059) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign71530_e108059) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71530_e108059) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign71530_e108059) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign71530_e108059) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign71530_e108059) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71530_e108059) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71530_e108059) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign71530_e108059) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign71530_e108059) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign71530_e108062;
        locals.var_chi_dn0 = assign71530_e108062_d_n0;
        locals.var_chi_dn2 = assign71530_e108062_d_n2;
        locals.var_chi_dn4 = assign71530_e108062_d_n4;
        locals.var_chi_dn5 = assign71530_e108062_d_n5;
        locals.var_chi_dn6 = assign71530_e108062_d_n6;
        locals.var_chi_dn7 = assign71530_e108062_d_n7;
        locals.var_chi_dn8 = assign71530_e108062_d_n8;
        locals.var_chi_dn9 = assign71530_e108062_d_n9;
        locals.var_chi_dn10 = assign71530_e108062_d_n10;
        locals.var_chi_dn13 = assign71530_e108062_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign71540_e108073, assign71540_e108073_d_n0, assign71540_e108073_d_n2, assign71540_e108073_d_n4, assign71540_e108073_d_n5, assign71540_e108073_d_n6, assign71540_e108073_d_n7, assign71540_e108073_d_n8, assign71540_e108073_d_n9, assign71540_e108073_d_n10, assign71540_e108073_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 != 0.0)) {
        let assign71540_e108070: f64 = (-locals.var_chi);
        let assign71540_e108071: f64 = (assign71540_e108070).exp();
        (assign71540_e108071, (assign71540_e108071 * (-locals.var_chi_dn0)), (assign71540_e108071 * (-locals.var_chi_dn2)), (assign71540_e108071 * (-locals.var_chi_dn4)), (assign71540_e108071 * (-locals.var_chi_dn5)), (assign71540_e108071 * (-locals.var_chi_dn6)), (assign71540_e108071 * (-locals.var_chi_dn7)), (assign71540_e108071 * (-locals.var_chi_dn8)), (assign71540_e108071 * (-locals.var_chi_dn9)), (assign71540_e108071 * (-locals.var_chi_dn10)), (assign71540_e108071 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign71540_e108073;
        locals.var_ty_dn0 = assign71540_e108073_d_n0;
        locals.var_ty_dn2 = assign71540_e108073_d_n2;
        locals.var_ty_dn4 = assign71540_e108073_d_n4;
        locals.var_ty_dn5 = assign71540_e108073_d_n5;
        locals.var_ty_dn6 = assign71540_e108073_d_n6;
        locals.var_ty_dn7 = assign71540_e108073_d_n7;
        locals.var_ty_dn8 = assign71540_e108073_d_n8;
        locals.var_ty_dn9 = assign71540_e108073_d_n9;
        locals.var_ty_dn10 = assign71540_e108073_d_n10;
        locals.var_ty_dn13 = assign71540_e108073_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign71550_e108098, assign71550_e108098_d_n0, assign71550_e108098_d_n2, assign71550_e108098_d_n4, assign71550_e108098_d_n5, assign71550_e108098_d_n6, assign71550_e108098_d_n7, assign71550_e108098_d_n8, assign71550_e108098_d_n9, assign71550_e108098_d_n10, assign71550_e108098_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 != 0.0)) {
        let assign71550_e108085: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71550_e108086: f64 = (locals.var_beta * assign71550_e108085);
        let assign71550_e108088: f64 = (assign71550_e108086 - 1.0);
        let assign71550_e108090: f64 = (assign71550_e108088 + locals.var_ty);
        let assign71550_e108091: f64 = (4.0 * assign71550_e108090);
        let assign71550_e108094: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign71550_e108095: f64 = (assign71550_e108091 / assign71550_e108094);
        let assign71550_e108096: f64 = (1.0 + assign71550_e108095);
        (assign71550_e108096, ((((4.0 * (((locals.var_beta_dn0 * assign71550_e108085) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign71550_e108094) - (assign71550_e108091 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign71550_e108094 * assign71550_e108094)), ((((4.0 * (((locals.var_beta_dn2 * assign71550_e108085) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign71550_e108094) - (assign71550_e108091 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign71550_e108094 * assign71550_e108094)), ((((4.0 * (((locals.var_beta_dn4 * assign71550_e108085) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign71550_e108094) - (assign71550_e108091 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign71550_e108094 * assign71550_e108094)), ((((4.0 * (((locals.var_beta_dn5 * assign71550_e108085) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign71550_e108094) - (assign71550_e108091 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign71550_e108094 * assign71550_e108094)), ((((4.0 * (((locals.var_beta_dn6 * assign71550_e108085) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))) + locals.var_ty_dn6)) * assign71550_e108094) - (assign71550_e108091 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign71550_e108094 * assign71550_e108094)), ((((4.0 * (((locals.var_beta_dn7 * assign71550_e108085) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign71550_e108094) - (assign71550_e108091 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign71550_e108094 * assign71550_e108094)), ((((4.0 * (((locals.var_beta_dn8 * assign71550_e108085) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign71550_e108094) - (assign71550_e108091 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign71550_e108094 * assign71550_e108094)), ((((4.0 * (((locals.var_beta_dn9 * assign71550_e108085) + (locals.var_beta * locals.var_vxbgmtcl_dn9)) + locals.var_ty_dn9)) * assign71550_e108094) - (assign71550_e108091 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign71550_e108094 * assign71550_e108094)), ((((4.0 * (((locals.var_beta_dn10 * assign71550_e108085) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign71550_e108094) - (assign71550_e108091 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign71550_e108094 * assign71550_e108094)), ((((4.0 * (((locals.var_beta_dn13 * assign71550_e108085) + (locals.var_beta * locals.var_vxbgmtcl_dn13)) + locals.var_ty_dn13)) * assign71550_e108094) - (assign71550_e108091 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign71550_e108094 * assign71550_e108094)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign71550_e108098;
        locals.var_tx_dn0 = assign71550_e108098_d_n0;
        locals.var_tx_dn2 = assign71550_e108098_d_n2;
        locals.var_tx_dn4 = assign71550_e108098_d_n4;
        locals.var_tx_dn5 = assign71550_e108098_d_n5;
        locals.var_tx_dn6 = assign71550_e108098_d_n6;
        locals.var_tx_dn7 = assign71550_e108098_d_n7;
        locals.var_tx_dn8 = assign71550_e108098_d_n8;
        locals.var_tx_dn9 = assign71550_e108098_d_n9;
        locals.var_tx_dn10 = assign71550_e108098_d_n10;
        locals.var_tx_dn13 = assign71550_e108098_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign71560_e108118, assign71560_e108118_d_n0, assign71560_e108118_d_n2, assign71560_e108118_d_n4, assign71560_e108118_d_n5, assign71560_e108118_d_n6, assign71560_e108118_d_n7, assign71560_e108118_d_n8, assign71560_e108118_d_n9, assign71560_e108118_d_n10, assign71560_e108118_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 != 0.0)) {
        let assign71560_e108108: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71560_e108110: f64 = (assign71560_e108108 / 2.0);
        let assign71560_e108113: f64 = (locals.var_tx).sqrt();
        let assign71560_e108114: f64 = (1.0 - assign71560_e108113);
        let assign71560_e108115: f64 = (assign71560_e108110 * assign71560_e108114);
        let assign71560_e108116: f64 = (locals.var_vgpld + assign71560_e108115);
        (assign71560_e108116, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign71560_e108114) + (assign71560_e108110 * (-(locals.var_tx_dn0 / (2.0 * assign71560_e108113))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign71560_e108114) + (assign71560_e108110 * (-(locals.var_tx_dn2 / (2.0 * assign71560_e108113)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign71560_e108114) + (assign71560_e108110 * (-(locals.var_tx_dn4 / (2.0 * assign71560_e108113))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign71560_e108114) + (assign71560_e108110 * (-(locals.var_tx_dn5 / (2.0 * assign71560_e108113))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign71560_e108114) + (assign71560_e108110 * (-(locals.var_tx_dn6 / (2.0 * assign71560_e108113)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign71560_e108114) + (assign71560_e108110 * (-(locals.var_tx_dn7 / (2.0 * assign71560_e108113)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign71560_e108114) + (assign71560_e108110 * (-(locals.var_tx_dn8 / (2.0 * assign71560_e108113)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign71560_e108114) + (assign71560_e108110 * (-(locals.var_tx_dn9 / (2.0 * assign71560_e108113))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign71560_e108114) + (assign71560_e108110 * (-(locals.var_tx_dn10 / (2.0 * assign71560_e108113))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign71560_e108114) + (assign71560_e108110 * (-(locals.var_tx_dn13 / (2.0 * assign71560_e108113))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign71560_e108118;
        locals.var_ps0_inia_dn0 = assign71560_e108118_d_n0;
        locals.var_ps0_inia_dn2 = assign71560_e108118_d_n2;
        locals.var_ps0_inia_dn4 = assign71560_e108118_d_n4;
        locals.var_ps0_inia_dn5 = assign71560_e108118_d_n5;
        locals.var_ps0_inia_dn6 = assign71560_e108118_d_n6;
        locals.var_ps0_inia_dn7 = assign71560_e108118_d_n7;
        locals.var_ps0_inia_dn8 = assign71560_e108118_d_n8;
        locals.var_ps0_inia_dn9 = assign71560_e108118_d_n9;
        locals.var_ps0_inia_dn10 = assign71560_e108118_d_n10;
        locals.var_ps0_inia_dn13 = assign71560_e108118_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign71570_e108131, assign71570_e108131_d_n0, assign71570_e108131_d_n2, assign71570_e108131_d_n4, assign71570_e108131_d_n5, assign71570_e108131_d_n6, assign71570_e108131_d_n7, assign71570_e108131_d_n8, assign71570_e108131_d_n9, assign71570_e108131_d_n10, assign71570_e108131_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 != 0.0)) {
        let assign71570_e108128: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign71570_e108129: f64 = (locals.var_beta * assign71570_e108128);
        (assign71570_e108129, ((locals.var_beta_dn0 * assign71570_e108128) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign71570_e108128) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71570_e108128) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign71570_e108128) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign71570_e108128) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign71570_e108128) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71570_e108128) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71570_e108128) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign71570_e108128) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign71570_e108128) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign71570_e108131;
        locals.var_chi_dn0 = assign71570_e108131_d_n0;
        locals.var_chi_dn2 = assign71570_e108131_d_n2;
        locals.var_chi_dn4 = assign71570_e108131_d_n4;
        locals.var_chi_dn5 = assign71570_e108131_d_n5;
        locals.var_chi_dn6 = assign71570_e108131_d_n6;
        locals.var_chi_dn7 = assign71570_e108131_d_n7;
        locals.var_chi_dn8 = assign71570_e108131_d_n8;
        locals.var_chi_dn9 = assign71570_e108131_d_n9;
        locals.var_chi_dn10 = assign71570_e108131_d_n10;
        locals.var_chi_dn13 = assign71570_e108131_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign71590_e108173,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71590_e108152: f64 = (2.0_f64).sqrt();
        let assign71590_e108153: f64 = (9.0 * assign71590_e108152);
        let assign71590_e108154: f64 = (1.0 / assign71590_e108153);
        let assign71590_e108158: f64 = (-3.0);
        let assign71590_e108159: f64 = (assign71590_e108158).exp();
        let assign71590_e108160: f64 = (7.0 * assign71590_e108159);
        let assign71590_e108161: f64 = (5.0 + assign71590_e108160);
        let assign71590_e108165: f64 = (-3.0);
        let assign71590_e108166: f64 = (assign71590_e108165).exp();
        let assign71590_e108167: f64 = (2.0 + assign71590_e108166);
        let assign71590_e108168: f64 = (assign71590_e108167).sqrt();
        let assign71590_e108169: f64 = (54.0 * assign71590_e108168);
        let assign71590_e108170: f64 = (assign71590_e108161 / assign71590_e108169);
        let assign71590_e108171: f64 = (assign71590_e108154 - assign71590_e108170);
        (assign71590_e108171,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign71590_e108173;
        locals.var_ta_rv = 0.0;

        let (assign71600_e108201,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71600_e108183: f64 = (-3.0);
        let assign71600_e108184: f64 = (assign71600_e108183).exp();
        let assign71600_e108185: f64 = (1.0 + assign71600_e108184);
        let assign71600_e108189: f64 = (-3.0);
        let assign71600_e108190: f64 = (assign71600_e108189).exp();
        let assign71600_e108191: f64 = (2.0 + assign71600_e108190);
        let assign71600_e108192: f64 = (assign71600_e108191).sqrt();
        let assign71600_e108193: f64 = (2.0 * assign71600_e108192);
        let assign71600_e108194: f64 = (assign71600_e108185 / assign71600_e108193);
        let assign71600_e108196: f64 = (2.0_f64).sqrt();
        let assign71600_e108198: f64 = (assign71600_e108196 / 3.0);
        let assign71600_e108199: f64 = (assign71600_e108194 - assign71600_e108198);
        (assign71600_e108199,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign71600_e108201;
        locals.var_tb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_258(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign71610_e108220, assign71610_e108220_d_n0, assign71610_e108220_d_n2, assign71610_e108220_d_n4, assign71610_e108220_d_n5, assign71610_e108220_d_n6, assign71610_e108220_d_n7, assign71610_e108220_d_n8, assign71610_e108220_d_n9, assign71610_e108220_d_n10, assign71610_e108220_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71610_e108211: f64 = (2.0_f64).sqrt();
        let assign71610_e108212: f64 = (1.0 / assign71610_e108211);
        let assign71610_e108216: f64 = (locals.var_beta * locals.var_fac1);
        let assign71610_e108217: f64 = (1.0 / assign71610_e108216);
        let assign71610_e108218: f64 = (assign71610_e108212 + assign71610_e108217);
        (assign71610_e108218, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign71610_e108216 * assign71610_e108216))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign71610_e108216 * assign71610_e108216))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign71610_e108216 * assign71610_e108216))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign71610_e108216 * assign71610_e108216))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign71610_e108216 * assign71610_e108216))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign71610_e108216 * assign71610_e108216))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign71610_e108216 * assign71610_e108216))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign71610_e108216 * assign71610_e108216))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign71610_e108216 * assign71610_e108216))), (-(((locals.var_beta_dn13 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn13)) / (assign71610_e108216 * assign71610_e108216))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn13,)
    }
};
        locals.var_tc = assign71610_e108220;
        locals.var_tc_dn0 = assign71610_e108220_d_n0;
        locals.var_tc_dn2 = assign71610_e108220_d_n2;
        locals.var_tc_dn4 = assign71610_e108220_d_n4;
        locals.var_tc_dn5 = assign71610_e108220_d_n5;
        locals.var_tc_dn6 = assign71610_e108220_d_n6;
        locals.var_tc_dn7 = assign71610_e108220_d_n7;
        locals.var_tc_dn8 = assign71610_e108220_d_n8;
        locals.var_tc_dn9 = assign71610_e108220_d_n9;
        locals.var_tc_dn10 = assign71610_e108220_d_n10;
        locals.var_tc_dn13 = assign71610_e108220_d_n13;
        locals.var_tc_rv = 0.0;

        let (assign71620_e108235, assign71620_e108235_d_n0, assign71620_e108235_d_n2, assign71620_e108235_d_n4, assign71620_e108235_d_n5, assign71620_e108235_d_n6, assign71620_e108235_d_n7, assign71620_e108235_d_n8, assign71620_e108235_d_n9, assign71620_e108235_d_n10, assign71620_e108235_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71620_e108230: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71620_e108231: f64 = (-assign71620_e108230);
        let assign71620_e108233: f64 = (assign71620_e108231 / locals.var_fac1);
        (assign71620_e108233, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign71620_e108231 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign71620_e108231 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign71620_e108231 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign71620_e108231 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1) - (assign71620_e108231 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign71620_e108231 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign71620_e108231 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn9) * locals.var_fac1) - (assign71620_e108231 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign71620_e108231 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn13) * locals.var_fac1) - (assign71620_e108231 * locals.var_fac1_dn13)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn13,)
    }
};
        locals.var_td = assign71620_e108235;
        locals.var_td_dn0 = assign71620_e108235_d_n0;
        locals.var_td_dn2 = assign71620_e108235_d_n2;
        locals.var_td_dn4 = assign71620_e108235_d_n4;
        locals.var_td_dn5 = assign71620_e108235_d_n5;
        locals.var_td_dn6 = assign71620_e108235_d_n6;
        locals.var_td_dn7 = assign71620_e108235_d_n7;
        locals.var_td_dn8 = assign71620_e108235_d_n8;
        locals.var_td_dn9 = assign71620_e108235_d_n9;
        locals.var_td_dn10 = assign71620_e108235_d_n10;
        locals.var_td_dn13 = assign71620_e108235_d_n13;
        locals.var_td_rv = 0.0;

        let (assign71630_e108273, assign71630_e108273_d_n0, assign71630_e108273_d_n2, assign71630_e108273_d_n4, assign71630_e108273_d_n5, assign71630_e108273_d_n6, assign71630_e108273_d_n7, assign71630_e108273_d_n8, assign71630_e108273_d_n9, assign71630_e108273_d_n10, assign71630_e108273_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71630_e108245: f64 = (locals.var_tb * locals.var_tb);
        let assign71630_e108247: f64 = (assign71630_e108245 * locals.var_tb);
        let assign71630_e108250: f64 = (27.0 * locals.var_ta);
        let assign71630_e108252: f64 = (assign71630_e108250 * locals.var_ta);
        let assign71630_e108254: f64 = (assign71630_e108252 * locals.var_ta);
        let assign71630_e108255: f64 = (assign71630_e108247 / assign71630_e108254);
        let assign71630_e108258: f64 = (locals.var_tb * locals.var_tc);
        let assign71630_e108261: f64 = (6.0 * locals.var_ta);
        let assign71630_e108263: f64 = (assign71630_e108261 * locals.var_ta);
        let assign71630_e108264: f64 = (assign71630_e108258 / assign71630_e108263);
        let assign71630_e108265: f64 = (assign71630_e108255 - assign71630_e108264);
        let assign71630_e108269: f64 = (2.0 * locals.var_ta);
        let assign71630_e108270: f64 = (locals.var_td / assign71630_e108269);
        let assign71630_e108271: f64 = (assign71630_e108265 + assign71630_e108270);
        (assign71630_e108271, ((-((locals.var_tb * locals.var_tc_dn0) / assign71630_e108263)) + (locals.var_td_dn0 / assign71630_e108269)), ((-((locals.var_tb * locals.var_tc_dn2) / assign71630_e108263)) + (locals.var_td_dn2 / assign71630_e108269)), ((-((locals.var_tb * locals.var_tc_dn4) / assign71630_e108263)) + (locals.var_td_dn4 / assign71630_e108269)), ((-((locals.var_tb * locals.var_tc_dn5) / assign71630_e108263)) + (locals.var_td_dn5 / assign71630_e108269)), ((-((locals.var_tb * locals.var_tc_dn6) / assign71630_e108263)) + (locals.var_td_dn6 / assign71630_e108269)), ((-((locals.var_tb * locals.var_tc_dn7) / assign71630_e108263)) + (locals.var_td_dn7 / assign71630_e108269)), ((-((locals.var_tb * locals.var_tc_dn8) / assign71630_e108263)) + (locals.var_td_dn8 / assign71630_e108269)), ((-((locals.var_tb * locals.var_tc_dn9) / assign71630_e108263)) + (locals.var_td_dn9 / assign71630_e108269)), ((-((locals.var_tb * locals.var_tc_dn10) / assign71630_e108263)) + (locals.var_td_dn10 / assign71630_e108269)), ((-((locals.var_tb * locals.var_tc_dn13) / assign71630_e108263)) + (locals.var_td_dn13 / assign71630_e108269)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn13,)
    }
};
        locals.var_tq = assign71630_e108273;
        locals.var_tq_dn0 = assign71630_e108273_d_n0;
        locals.var_tq_dn2 = assign71630_e108273_d_n2;
        locals.var_tq_dn4 = assign71630_e108273_d_n4;
        locals.var_tq_dn5 = assign71630_e108273_d_n5;
        locals.var_tq_dn6 = assign71630_e108273_d_n6;
        locals.var_tq_dn7 = assign71630_e108273_d_n7;
        locals.var_tq_dn8 = assign71630_e108273_d_n8;
        locals.var_tq_dn9 = assign71630_e108273_d_n9;
        locals.var_tq_dn10 = assign71630_e108273_d_n10;
        locals.var_tq_dn13 = assign71630_e108273_d_n13;
        locals.var_tq_rv = 0.0;

        let (assign71640_e108297, assign71640_e108297_d_n0, assign71640_e108297_d_n2, assign71640_e108297_d_n4, assign71640_e108297_d_n5, assign71640_e108297_d_n6, assign71640_e108297_d_n7, assign71640_e108297_d_n8, assign71640_e108297_d_n9, assign71640_e108297_d_n10, assign71640_e108297_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71640_e108283: f64 = (3.0 * locals.var_ta);
        let assign71640_e108285: f64 = (assign71640_e108283 * locals.var_tc);
        let assign71640_e108288: f64 = (locals.var_tb * locals.var_tb);
        let assign71640_e108289: f64 = (assign71640_e108285 - assign71640_e108288);
        let assign71640_e108292: f64 = (9.0 * locals.var_ta);
        let assign71640_e108294: f64 = (assign71640_e108292 * locals.var_ta);
        let assign71640_e108295: f64 = (assign71640_e108289 / assign71640_e108294);
        (assign71640_e108295, ((assign71640_e108283 * locals.var_tc_dn0) / assign71640_e108294), ((assign71640_e108283 * locals.var_tc_dn2) / assign71640_e108294), ((assign71640_e108283 * locals.var_tc_dn4) / assign71640_e108294), ((assign71640_e108283 * locals.var_tc_dn5) / assign71640_e108294), ((assign71640_e108283 * locals.var_tc_dn6) / assign71640_e108294), ((assign71640_e108283 * locals.var_tc_dn7) / assign71640_e108294), ((assign71640_e108283 * locals.var_tc_dn8) / assign71640_e108294), ((assign71640_e108283 * locals.var_tc_dn9) / assign71640_e108294), ((assign71640_e108283 * locals.var_tc_dn10) / assign71640_e108294), ((assign71640_e108283 * locals.var_tc_dn13) / assign71640_e108294),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn13,)
    }
};
        locals.var_tp = assign71640_e108297;
        locals.var_tp_dn0 = assign71640_e108297_d_n0;
        locals.var_tp_dn2 = assign71640_e108297_d_n2;
        locals.var_tp_dn4 = assign71640_e108297_d_n4;
        locals.var_tp_dn5 = assign71640_e108297_d_n5;
        locals.var_tp_dn6 = assign71640_e108297_d_n6;
        locals.var_tp_dn7 = assign71640_e108297_d_n7;
        locals.var_tp_dn8 = assign71640_e108297_d_n8;
        locals.var_tp_dn9 = assign71640_e108297_d_n9;
        locals.var_tp_dn10 = assign71640_e108297_d_n10;
        locals.var_tp_dn13 = assign71640_e108297_d_n13;
        locals.var_tp_rv = 0.0;

        let (assign71650_e108316, assign71650_e108316_d_n0, assign71650_e108316_d_n2, assign71650_e108316_d_n4, assign71650_e108316_d_n5, assign71650_e108316_d_n6, assign71650_e108316_d_n7, assign71650_e108316_d_n8, assign71650_e108316_d_n9, assign71650_e108316_d_n10, assign71650_e108316_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71650_e108307: f64 = (locals.var_tq * locals.var_tq);
        let assign71650_e108310: f64 = (locals.var_tp * locals.var_tp);
        let assign71650_e108312: f64 = (assign71650_e108310 * locals.var_tp);
        let assign71650_e108313: f64 = (assign71650_e108307 + assign71650_e108312);
        let assign71650_e108314: f64 = (assign71650_e108313).sqrt();
        (assign71650_e108314, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign71650_e108310 * locals.var_tp_dn0))) / (2.0 * assign71650_e108314)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign71650_e108310 * locals.var_tp_dn2))) / (2.0 * assign71650_e108314)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign71650_e108310 * locals.var_tp_dn4))) / (2.0 * assign71650_e108314)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign71650_e108310 * locals.var_tp_dn5))) / (2.0 * assign71650_e108314)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign71650_e108310 * locals.var_tp_dn6))) / (2.0 * assign71650_e108314)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign71650_e108310 * locals.var_tp_dn7))) / (2.0 * assign71650_e108314)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign71650_e108310 * locals.var_tp_dn8))) / (2.0 * assign71650_e108314)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign71650_e108310 * locals.var_tp_dn9))) / (2.0 * assign71650_e108314)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign71650_e108310 * locals.var_tp_dn10))) / (2.0 * assign71650_e108314)), ((((locals.var_tq_dn13 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn13)) + ((((locals.var_tp_dn13 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn13)) * locals.var_tp) + (assign71650_e108310 * locals.var_tp_dn13))) / (2.0 * assign71650_e108314)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign71650_e108316;
        locals.var_t5_dn0 = assign71650_e108316_d_n0;
        locals.var_t5_dn2 = assign71650_e108316_d_n2;
        locals.var_t5_dn4 = assign71650_e108316_d_n4;
        locals.var_t5_dn5 = assign71650_e108316_d_n5;
        locals.var_t5_dn6 = assign71650_e108316_d_n6;
        locals.var_t5_dn7 = assign71650_e108316_d_n7;
        locals.var_t5_dn8 = assign71650_e108316_d_n8;
        locals.var_t5_dn9 = assign71650_e108316_d_n9;
        locals.var_t5_dn10 = assign71650_e108316_d_n10;
        locals.var_t5_dn13 = assign71650_e108316_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign71660_e108331, assign71660_e108331_d_n0, assign71660_e108331_d_n2, assign71660_e108331_d_n4, assign71660_e108331_d_n5, assign71660_e108331_d_n6, assign71660_e108331_d_n7, assign71660_e108331_d_n8, assign71660_e108331_d_n9, assign71660_e108331_d_n10, assign71660_e108331_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71660_e108325: f64 = (-locals.var_tq);
        let assign71660_e108327: f64 = (assign71660_e108325 + locals.var_t5);
        let assign71660_e108329: f64 = (assign71660_e108327).powf(0.3333333333333333);
        (assign71660_e108329, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71660_e108327).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign71660_e108329 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign71660_e108327))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71660_e108327).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign71660_e108329 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign71660_e108327))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71660_e108327).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign71660_e108329 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign71660_e108327))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71660_e108327).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign71660_e108329 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign71660_e108327))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71660_e108327).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign71660_e108329 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign71660_e108327))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71660_e108327).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign71660_e108329 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign71660_e108327))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71660_e108327).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign71660_e108329 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign71660_e108327))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71660_e108327).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign71660_e108329 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign71660_e108327))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71660_e108327).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign71660_e108329 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign71660_e108327))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71660_e108327).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn13) + locals.var_t5_dn13))) } } else { (assign71660_e108329 * (0.3333333333333333 * (((-locals.var_tq_dn13) + locals.var_t5_dn13) / assign71660_e108327))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn13,)
    }
};
        locals.var_tu = assign71660_e108331;
        locals.var_tu_dn0 = assign71660_e108331_d_n0;
        locals.var_tu_dn2 = assign71660_e108331_d_n2;
        locals.var_tu_dn4 = assign71660_e108331_d_n4;
        locals.var_tu_dn5 = assign71660_e108331_d_n5;
        locals.var_tu_dn6 = assign71660_e108331_d_n6;
        locals.var_tu_dn7 = assign71660_e108331_d_n7;
        locals.var_tu_dn8 = assign71660_e108331_d_n8;
        locals.var_tu_dn9 = assign71660_e108331_d_n9;
        locals.var_tu_dn10 = assign71660_e108331_d_n10;
        locals.var_tu_dn13 = assign71660_e108331_d_n13;
        locals.var_tu_rv = 0.0;

        let (assign71670_e108346, assign71670_e108346_d_n0, assign71670_e108346_d_n2, assign71670_e108346_d_n4, assign71670_e108346_d_n5, assign71670_e108346_d_n6, assign71670_e108346_d_n7, assign71670_e108346_d_n8, assign71670_e108346_d_n9, assign71670_e108346_d_n10, assign71670_e108346_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71670_e108341: f64 = (locals.var_tq + locals.var_t5);
        let assign71670_e108343: f64 = (assign71670_e108341).powf(0.3333333333333333);
        let assign71670_e108344: f64 = (-assign71670_e108343);
        (assign71670_e108344, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71670_e108341).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign71670_e108343 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign71670_e108341))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71670_e108341).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign71670_e108343 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign71670_e108341))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71670_e108341).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign71670_e108343 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign71670_e108341))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71670_e108341).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign71670_e108343 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign71670_e108341))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71670_e108341).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign71670_e108343 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign71670_e108341))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71670_e108341).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign71670_e108343 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign71670_e108341))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71670_e108341).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign71670_e108343 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign71670_e108341))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71670_e108341).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign71670_e108343 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign71670_e108341))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71670_e108341).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign71670_e108343 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign71670_e108341))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71670_e108341).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn13 + locals.var_t5_dn13))) } } else { (assign71670_e108343 * (0.3333333333333333 * ((locals.var_tq_dn13 + locals.var_t5_dn13) / assign71670_e108341))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn13,)
    }
};
        locals.var_tv = assign71670_e108346;
        locals.var_tv_dn0 = assign71670_e108346_d_n0;
        locals.var_tv_dn2 = assign71670_e108346_d_n2;
        locals.var_tv_dn4 = assign71670_e108346_d_n4;
        locals.var_tv_dn5 = assign71670_e108346_d_n5;
        locals.var_tv_dn6 = assign71670_e108346_d_n6;
        locals.var_tv_dn7 = assign71670_e108346_d_n7;
        locals.var_tv_dn8 = assign71670_e108346_d_n8;
        locals.var_tv_dn9 = assign71670_e108346_d_n9;
        locals.var_tv_dn10 = assign71670_e108346_d_n10;
        locals.var_tv_dn13 = assign71670_e108346_d_n13;
        locals.var_tv_rv = 0.0;

        let (assign71680_e108364, assign71680_e108364_d_n0, assign71680_e108364_d_n2, assign71680_e108364_d_n4, assign71680_e108364_d_n5, assign71680_e108364_d_n6, assign71680_e108364_d_n7, assign71680_e108364_d_n8, assign71680_e108364_d_n9, assign71680_e108364_d_n10, assign71680_e108364_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71680_e108356: f64 = (locals.var_tu + locals.var_tv);
        let assign71680_e108360: f64 = (3.0 * locals.var_ta);
        let assign71680_e108361: f64 = (locals.var_tb / assign71680_e108360);
        let assign71680_e108362: f64 = (assign71680_e108356 - assign71680_e108361);
        (assign71680_e108362, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn13 + locals.var_tv_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign71680_e108364;
        locals.var_chi_dn0 = assign71680_e108364_d_n0;
        locals.var_chi_dn2 = assign71680_e108364_d_n2;
        locals.var_chi_dn4 = assign71680_e108364_d_n4;
        locals.var_chi_dn5 = assign71680_e108364_d_n5;
        locals.var_chi_dn6 = assign71680_e108364_d_n6;
        locals.var_chi_dn7 = assign71680_e108364_d_n7;
        locals.var_chi_dn8 = assign71680_e108364_d_n8;
        locals.var_chi_dn9 = assign71680_e108364_d_n9;
        locals.var_chi_dn10 = assign71680_e108364_d_n10;
        locals.var_chi_dn13 = assign71680_e108364_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign71690_e108378, assign71690_e108378_d_n0, assign71690_e108378_d_n2, assign71690_e108378_d_n4, assign71690_e108378_d_n5, assign71690_e108378_d_n6, assign71690_e108378_d_n7, assign71690_e108378_d_n8, assign71690_e108378_d_n9, assign71690_e108378_d_n10, assign71690_e108378_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71690_e108374: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign71690_e108376: f64 = (assign71690_e108374 - locals.var_vxbgmtcl);
        (assign71690_e108376, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign71690_e108378;
        locals.var_ps0_inia_dn0 = assign71690_e108378_d_n0;
        locals.var_ps0_inia_dn2 = assign71690_e108378_d_n2;
        locals.var_ps0_inia_dn4 = assign71690_e108378_d_n4;
        locals.var_ps0_inia_dn5 = assign71690_e108378_d_n5;
        locals.var_ps0_inia_dn6 = assign71690_e108378_d_n6;
        locals.var_ps0_inia_dn7 = assign71690_e108378_d_n7;
        locals.var_ps0_inia_dn8 = assign71690_e108378_d_n8;
        locals.var_ps0_inia_dn9 = assign71690_e108378_d_n9;
        locals.var_ps0_inia_dn10 = assign71690_e108378_d_n10;
        locals.var_ps0_inia_dn13 = assign71690_e108378_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let assign71700_e108381: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1669 = assign71700_e108381;
        locals.var_guard1669_rv = 0.0;

        let (assign71710_e108394, assign71710_e108394_d_n0, assign71710_e108394_d_n2, assign71710_e108394_d_n4, assign71710_e108394_d_n5, assign71710_e108394_d_n6, assign71710_e108394_d_n7, assign71710_e108394_d_n8, assign71710_e108394_d_n9, assign71710_e108394_d_n10, assign71710_e108394_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let assign71710_e108390: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71710_e108392: f64 = (assign71710_e108390 + 0.1);
        (assign71710_e108392, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn13,)
    }
};
        locals.var_vgpld_shift = assign71710_e108394;
        locals.var_vgpld_shift_dn0 = assign71710_e108394_d_n0;
        locals.var_vgpld_shift_dn2 = assign71710_e108394_d_n2;
        locals.var_vgpld_shift_dn4 = assign71710_e108394_d_n4;
        locals.var_vgpld_shift_dn5 = assign71710_e108394_d_n5;
        locals.var_vgpld_shift_dn6 = assign71710_e108394_d_n6;
        locals.var_vgpld_shift_dn7 = assign71710_e108394_d_n7;
        locals.var_vgpld_shift_dn8 = assign71710_e108394_d_n8;
        locals.var_vgpld_shift_dn9 = assign71710_e108394_d_n9;
        locals.var_vgpld_shift_dn10 = assign71710_e108394_d_n10;
        locals.var_vgpld_shift_dn13 = assign71710_e108394_d_n13;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign71720_e108405, assign71720_e108405_d_n0, assign71720_e108405_d_n2, assign71720_e108405_d_n4, assign71720_e108405_d_n5, assign71720_e108405_d_n6, assign71720_e108405_d_n7, assign71720_e108405_d_n8, assign71720_e108405_d_n9, assign71720_e108405_d_n10, assign71720_e108405_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let assign71720_e108403: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign71720_e108403, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign71720_e108405;
        locals.var_cfs1_dn0 = assign71720_e108405_d_n0;
        locals.var_cfs1_dn2 = assign71720_e108405_d_n2;
        locals.var_cfs1_dn4 = assign71720_e108405_d_n4;
        locals.var_cfs1_dn5 = assign71720_e108405_d_n5;
        locals.var_cfs1_dn6 = assign71720_e108405_d_n6;
        locals.var_cfs1_dn7 = assign71720_e108405_d_n7;
        locals.var_cfs1_dn8 = assign71720_e108405_d_n8;
        locals.var_cfs1_dn9 = assign71720_e108405_d_n9;
        locals.var_cfs1_dn10 = assign71720_e108405_d_n10;
        locals.var_cfs1_dn13 = assign71720_e108405_d_n13;
        locals.var_cfs1_rv = 0.0;

        let (assign71730_e108416, assign71730_e108416_d_n0, assign71730_e108416_d_n2, assign71730_e108416_d_n4, assign71730_e108416_d_n5, assign71730_e108416_d_n6, assign71730_e108416_d_n7, assign71730_e108416_d_n8, assign71730_e108416_d_n9, assign71730_e108416_d_n10, assign71730_e108416_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let assign71730_e108414: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign71730_e108414, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn13,)
    }
};
        locals.var_gammachi = assign71730_e108416;
        locals.var_gammachi_dn0 = assign71730_e108416_d_n0;
        locals.var_gammachi_dn2 = assign71730_e108416_d_n2;
        locals.var_gammachi_dn4 = assign71730_e108416_d_n4;
        locals.var_gammachi_dn5 = assign71730_e108416_d_n5;
        locals.var_gammachi_dn6 = assign71730_e108416_d_n6;
        locals.var_gammachi_dn7 = assign71730_e108416_d_n7;
        locals.var_gammachi_dn8 = assign71730_e108416_d_n8;
        locals.var_gammachi_dn9 = assign71730_e108416_d_n9;
        locals.var_gammachi_dn10 = assign71730_e108416_d_n10;
        locals.var_gammachi_dn13 = assign71730_e108416_d_n13;
        locals.var_gammachi_rv = 0.0;

        let (assign71740_e108427, assign71740_e108427_d_n0, assign71740_e108427_d_n2, assign71740_e108427_d_n4, assign71740_e108427_d_n5, assign71740_e108427_d_n6, assign71740_e108427_d_n7, assign71740_e108427_d_n8, assign71740_e108427_d_n9, assign71740_e108427_d_n10, assign71740_e108427_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let assign71740_e108425: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign71740_e108425, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn13 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign71740_e108427;
        locals.var_t0_dn0 = assign71740_e108427_d_n0;
        locals.var_t0_dn2 = assign71740_e108427_d_n2;
        locals.var_t0_dn4 = assign71740_e108427_d_n4;
        locals.var_t0_dn5 = assign71740_e108427_d_n5;
        locals.var_t0_dn6 = assign71740_e108427_d_n6;
        locals.var_t0_dn7 = assign71740_e108427_d_n7;
        locals.var_t0_dn8 = assign71740_e108427_d_n8;
        locals.var_t0_dn9 = assign71740_e108427_d_n9;
        locals.var_t0_dn10 = assign71740_e108427_d_n10;
        locals.var_t0_dn13 = assign71740_e108427_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign71750_e108438, assign71750_e108438_d_n0, assign71750_e108438_d_n2, assign71750_e108438_d_n4, assign71750_e108438_d_n5, assign71750_e108438_d_n6, assign71750_e108438_d_n7, assign71750_e108438_d_n8, assign71750_e108438_d_n9, assign71750_e108438_d_n10, assign71750_e108438_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let assign71750_e108436: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign71750_e108436, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn13 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn13)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign71750_e108438;
        locals.var_psi_dn0 = assign71750_e108438_d_n0;
        locals.var_psi_dn2 = assign71750_e108438_d_n2;
        locals.var_psi_dn4 = assign71750_e108438_d_n4;
        locals.var_psi_dn5 = assign71750_e108438_d_n5;
        locals.var_psi_dn6 = assign71750_e108438_d_n6;
        locals.var_psi_dn7 = assign71750_e108438_d_n7;
        locals.var_psi_dn8 = assign71750_e108438_d_n8;
        locals.var_psi_dn9 = assign71750_e108438_d_n9;
        locals.var_psi_dn10 = assign71750_e108438_d_n10;
        locals.var_psi_dn13 = assign71750_e108438_d_n13;
        locals.var_psi_rv = 0.0;

        let (assign71760_e108463, assign71760_e108463_d_n0, assign71760_e108463_d_n2, assign71760_e108463_d_n4, assign71760_e108463_d_n5, assign71760_e108463_d_n6, assign71760_e108463_d_n7, assign71760_e108463_d_n8, assign71760_e108463_d_n9, assign71760_e108463_d_n10, assign71760_e108463_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let assign71760_e108447: f64 = (locals.var_gammachi * locals.var_t0);
        let assign71760_e108450: f64 = (locals.var_psi * locals.var_psi);
        let assign71760_e108451: f64 = (assign71760_e108447 + assign71760_e108450);
        let assign71760_e108452: f64 = (assign71760_e108451).ln();
        let assign71760_e108455: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign71760_e108456: f64 = (assign71760_e108455).ln();
        let assign71760_e108457: f64 = (assign71760_e108452 - assign71760_e108456);
        let assign71760_e108460: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign71760_e108461: f64 = (assign71760_e108457 + assign71760_e108460);
        (assign71760_e108461, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign71760_e108451) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign71760_e108455)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign71760_e108451) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign71760_e108455)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign71760_e108451) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign71760_e108455)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign71760_e108451) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign71760_e108455)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign71760_e108451) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign71760_e108455)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign71760_e108451) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign71760_e108455)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign71760_e108451) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign71760_e108455)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign71760_e108451) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign71760_e108455)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign71760_e108451) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign71760_e108455)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign71760_e108451) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign71760_e108455)) + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign71760_e108463;
        locals.var_chi_1_dn0 = assign71760_e108463_d_n0;
        locals.var_chi_1_dn2 = assign71760_e108463_d_n2;
        locals.var_chi_1_dn4 = assign71760_e108463_d_n4;
        locals.var_chi_1_dn5 = assign71760_e108463_d_n5;
        locals.var_chi_1_dn6 = assign71760_e108463_d_n6;
        locals.var_chi_1_dn7 = assign71760_e108463_d_n7;
        locals.var_chi_1_dn8 = assign71760_e108463_d_n8;
        locals.var_chi_1_dn9 = assign71760_e108463_d_n9;
        locals.var_chi_1_dn10 = assign71760_e108463_d_n10;
        locals.var_chi_1_dn13 = assign71760_e108463_d_n13;
        locals.var_chi_1_rv = 0.0;

        let assign71770_e108466: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1670 = assign71770_e108466;
        locals.var_guard1670_rv = 0.0;

        let (assign71780_e108481, assign71780_e108481_d_n0, assign71780_e108481_d_n2, assign71780_e108481_d_n4, assign71780_e108481_d_n5, assign71780_e108481_d_n6, assign71780_e108481_d_n7, assign71780_e108481_d_n8, assign71780_e108481_d_n9, assign71780_e108481_d_n10, assign71780_e108481_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71780_e108477: f64 = (locals.var_psi - locals.var_chi_1);
        let assign71780_e108479: f64 = (assign71780_e108477 - 1.0);
        (assign71780_e108479, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign71780_e108481;
        locals.var_tmf1_dn0 = assign71780_e108481_d_n0;
        locals.var_tmf1_dn2 = assign71780_e108481_d_n2;
        locals.var_tmf1_dn4 = assign71780_e108481_d_n4;
        locals.var_tmf1_dn5 = assign71780_e108481_d_n5;
        locals.var_tmf1_dn6 = assign71780_e108481_d_n6;
        locals.var_tmf1_dn7 = assign71780_e108481_d_n7;
        locals.var_tmf1_dn8 = assign71780_e108481_d_n8;
        locals.var_tmf1_dn9 = assign71780_e108481_d_n9;
        locals.var_tmf1_dn10 = assign71780_e108481_d_n10;
        locals.var_tmf1_dn13 = assign71780_e108481_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign71790_e108496, assign71790_e108496_d_n0, assign71790_e108496_d_n2, assign71790_e108496_d_n4, assign71790_e108496_d_n5, assign71790_e108496_d_n6, assign71790_e108496_d_n7, assign71790_e108496_d_n8, assign71790_e108496_d_n9, assign71790_e108496_d_n10, assign71790_e108496_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71790_e108492: f64 = (4.0 * locals.var_psi);
        let assign71790_e108494: f64 = assign71790_e108492;
        (assign71790_e108494, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn13),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign71790_e108496;
        locals.var_tmf2_dn0 = assign71790_e108496_d_n0;
        locals.var_tmf2_dn2 = assign71790_e108496_d_n2;
        locals.var_tmf2_dn4 = assign71790_e108496_d_n4;
        locals.var_tmf2_dn5 = assign71790_e108496_d_n5;
        locals.var_tmf2_dn6 = assign71790_e108496_d_n6;
        locals.var_tmf2_dn7 = assign71790_e108496_d_n7;
        locals.var_tmf2_dn8 = assign71790_e108496_d_n8;
        locals.var_tmf2_dn9 = assign71790_e108496_d_n9;
        locals.var_tmf2_dn10 = assign71790_e108496_d_n10;
        locals.var_tmf2_dn13 = assign71790_e108496_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign71800_e108513, assign71800_e108513_d_n0, assign71800_e108513_d_n2, assign71800_e108513_d_n4, assign71800_e108513_d_n5, assign71800_e108513_d_n6, assign71800_e108513_d_n7, assign71800_e108513_d_n8, assign71800_e108513_d_n9, assign71800_e108513_d_n10, assign71800_e108513_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1670 != 0.0)) {
        let (assign71800_e108511, assign71800_e108511_d_n0, assign71800_e108511_d_n2, assign71800_e108511_d_n4, assign71800_e108511_d_n5, assign71800_e108511_d_n6, assign71800_e108511_d_n7, assign71800_e108511_d_n8, assign71800_e108511_d_n9, assign71800_e108511_d_n10, assign71800_e108511_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign71800_e108510: f64 = (-locals.var_tmf2);
                (assign71800_e108510, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign71800_e108511, assign71800_e108511_d_n0, assign71800_e108511_d_n2, assign71800_e108511_d_n4, assign71800_e108511_d_n5, assign71800_e108511_d_n6, assign71800_e108511_d_n7, assign71800_e108511_d_n8, assign71800_e108511_d_n9, assign71800_e108511_d_n10, assign71800_e108511_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign71800_e108513;
        locals.var_tmf2_dn0 = assign71800_e108513_d_n0;
        locals.var_tmf2_dn2 = assign71800_e108513_d_n2;
        locals.var_tmf2_dn4 = assign71800_e108513_d_n4;
        locals.var_tmf2_dn5 = assign71800_e108513_d_n5;
        locals.var_tmf2_dn6 = assign71800_e108513_d_n6;
        locals.var_tmf2_dn7 = assign71800_e108513_d_n7;
        locals.var_tmf2_dn8 = assign71800_e108513_d_n8;
        locals.var_tmf2_dn9 = assign71800_e108513_d_n9;
        locals.var_tmf2_dn10 = assign71800_e108513_d_n10;
        locals.var_tmf2_dn13 = assign71800_e108513_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign71810_e108529, assign71810_e108529_d_n0, assign71810_e108529_d_n2, assign71810_e108529_d_n4, assign71810_e108529_d_n5, assign71810_e108529_d_n6, assign71810_e108529_d_n7, assign71810_e108529_d_n8, assign71810_e108529_d_n9, assign71810_e108529_d_n10, assign71810_e108529_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71810_e108524: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign71810_e108526: f64 = (assign71810_e108524 + locals.var_tmf2);
        let assign71810_e108527: f64 = (assign71810_e108526).sqrt();
        (assign71810_e108527, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign71810_e108527)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign71810_e108527)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign71810_e108527)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign71810_e108527)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign71810_e108527)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign71810_e108527)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign71810_e108527)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign71810_e108527)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign71810_e108527)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign71810_e108527)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign71810_e108529;
        locals.var_tmf2_dn0 = assign71810_e108529_d_n0;
        locals.var_tmf2_dn2 = assign71810_e108529_d_n2;
        locals.var_tmf2_dn4 = assign71810_e108529_d_n4;
        locals.var_tmf2_dn5 = assign71810_e108529_d_n5;
        locals.var_tmf2_dn6 = assign71810_e108529_d_n6;
        locals.var_tmf2_dn7 = assign71810_e108529_d_n7;
        locals.var_tmf2_dn8 = assign71810_e108529_d_n8;
        locals.var_tmf2_dn9 = assign71810_e108529_d_n9;
        locals.var_tmf2_dn10 = assign71810_e108529_d_n10;
        locals.var_tmf2_dn13 = assign71810_e108529_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign71820_e108546, assign71820_e108546_d_n0, assign71820_e108546_d_n2, assign71820_e108546_d_n4, assign71820_e108546_d_n5, assign71820_e108546_d_n6, assign71820_e108546_d_n7, assign71820_e108546_d_n8, assign71820_e108546_d_n9, assign71820_e108546_d_n10, assign71820_e108546_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71820_e108542: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign71820_e108543: f64 = (1.0 + assign71820_e108542);
        let assign71820_e108544: f64 = (0.5 * assign71820_e108543);
        (assign71820_e108544, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign71820_e108546;
        locals.var_t1_dn0 = assign71820_e108546_d_n0;
        locals.var_t1_dn2 = assign71820_e108546_d_n2;
        locals.var_t1_dn4 = assign71820_e108546_d_n4;
        locals.var_t1_dn5 = assign71820_e108546_d_n5;
        locals.var_t1_dn6 = assign71820_e108546_d_n6;
        locals.var_t1_dn7 = assign71820_e108546_d_n7;
        locals.var_t1_dn8 = assign71820_e108546_d_n8;
        locals.var_t1_dn9 = assign71820_e108546_d_n9;
        locals.var_t1_dn10 = assign71820_e108546_d_n10;
        locals.var_t1_dn13 = assign71820_e108546_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign71830_e108563, assign71830_e108563_d_n0, assign71830_e108563_d_n2, assign71830_e108563_d_n4, assign71830_e108563_d_n5, assign71830_e108563_d_n6, assign71830_e108563_d_n7, assign71830_e108563_d_n8, assign71830_e108563_d_n9, assign71830_e108563_d_n10, assign71830_e108563_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71830_e108559: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign71830_e108560: f64 = (0.5 * assign71830_e108559);
        let assign71830_e108561: f64 = (locals.var_psi - assign71830_e108560);
        (assign71830_e108561, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign71830_e108563;
        locals.var_chi_1_dn0 = assign71830_e108563_d_n0;
        locals.var_chi_1_dn2 = assign71830_e108563_d_n2;
        locals.var_chi_1_dn4 = assign71830_e108563_d_n4;
        locals.var_chi_1_dn5 = assign71830_e108563_d_n5;
        locals.var_chi_1_dn6 = assign71830_e108563_d_n6;
        locals.var_chi_1_dn7 = assign71830_e108563_d_n7;
        locals.var_chi_1_dn8 = assign71830_e108563_d_n8;
        locals.var_chi_1_dn9 = assign71830_e108563_d_n9;
        locals.var_chi_1_dn10 = assign71830_e108563_d_n10;
        locals.var_chi_1_dn13 = assign71830_e108563_d_n13;
        locals.var_chi_1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_259(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign71840_e108580, assign71840_e108580_d_n0, assign71840_e108580_d_n2, assign71840_e108580_d_n4, assign71840_e108580_d_n5, assign71840_e108580_d_n6, assign71840_e108580_d_n7, assign71840_e108580_d_n8, assign71840_e108580_d_n9, assign71840_e108580_d_n10, assign71840_e108580_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1670 == 0.0)) {
        let (assign71840_e108578, assign71840_e108578_d_n0, assign71840_e108578_d_n2, assign71840_e108578_d_n4, assign71840_e108578_d_n5, assign71840_e108578_d_n6, assign71840_e108578_d_n7, assign71840_e108578_d_n8, assign71840_e108578_d_n9, assign71840_e108578_d_n10, assign71840_e108578_d_n13,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
            }
        };
        (assign71840_e108578, assign71840_e108578_d_n0, assign71840_e108578_d_n2, assign71840_e108578_d_n4, assign71840_e108578_d_n5, assign71840_e108578_d_n6, assign71840_e108578_d_n7, assign71840_e108578_d_n8, assign71840_e108578_d_n9, assign71840_e108578_d_n10, assign71840_e108578_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign71840_e108580;
        locals.var_chi_1_dn0 = assign71840_e108580_d_n0;
        locals.var_chi_1_dn2 = assign71840_e108580_d_n2;
        locals.var_chi_1_dn4 = assign71840_e108580_d_n4;
        locals.var_chi_1_dn5 = assign71840_e108580_d_n5;
        locals.var_chi_1_dn6 = assign71840_e108580_d_n6;
        locals.var_chi_1_dn7 = assign71840_e108580_d_n7;
        locals.var_chi_1_dn8 = assign71840_e108580_d_n8;
        locals.var_chi_1_dn9 = assign71840_e108580_d_n9;
        locals.var_chi_1_dn10 = assign71840_e108580_d_n10;
        locals.var_chi_1_dn13 = assign71840_e108580_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign71850_e108594, assign71850_e108594_d_n0, assign71850_e108594_d_n2, assign71850_e108594_d_n4, assign71850_e108594_d_n5, assign71850_e108594_d_n6, assign71850_e108594_d_n7, assign71850_e108594_d_n8, assign71850_e108594_d_n9, assign71850_e108594_d_n10, assign71850_e108594_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let (assign71850_e108592, assign71850_e108592_d_n0, assign71850_e108592_d_n2, assign71850_e108592_d_n4, assign71850_e108592_d_n5, assign71850_e108592_d_n6, assign71850_e108592_d_n7, assign71850_e108592_d_n8, assign71850_e108592_d_n9, assign71850_e108592_d_n10, assign71850_e108592_d_n13,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign71850_e108592, assign71850_e108592_d_n0, assign71850_e108592_d_n2, assign71850_e108592_d_n4, assign71850_e108592_d_n5, assign71850_e108592_d_n6, assign71850_e108592_d_n7, assign71850_e108592_d_n8, assign71850_e108592_d_n9, assign71850_e108592_d_n10, assign71850_e108592_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign71850_e108594;
        locals.var_chi_1_dn0 = assign71850_e108594_d_n0;
        locals.var_chi_1_dn2 = assign71850_e108594_d_n2;
        locals.var_chi_1_dn4 = assign71850_e108594_d_n4;
        locals.var_chi_1_dn5 = assign71850_e108594_d_n5;
        locals.var_chi_1_dn6 = assign71850_e108594_d_n6;
        locals.var_chi_1_dn7 = assign71850_e108594_d_n7;
        locals.var_chi_1_dn8 = assign71850_e108594_d_n8;
        locals.var_chi_1_dn9 = assign71850_e108594_d_n9;
        locals.var_chi_1_dn10 = assign71850_e108594_d_n10;
        locals.var_chi_1_dn13 = assign71850_e108594_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign71860_e108605, assign71860_e108605_d_n0, assign71860_e108605_d_n2, assign71860_e108605_d_n4, assign71860_e108605_d_n5, assign71860_e108605_d_n6, assign71860_e108605_d_n7, assign71860_e108605_d_n8, assign71860_e108605_d_n9, assign71860_e108605_d_n10, assign71860_e108605_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let assign71860_e108603: f64 = (locals.var_psi - locals.var_chi_1);
        (assign71860_e108603, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign71860_e108605;
        locals.var_psi_dn0 = assign71860_e108605_d_n0;
        locals.var_psi_dn2 = assign71860_e108605_d_n2;
        locals.var_psi_dn4 = assign71860_e108605_d_n4;
        locals.var_psi_dn5 = assign71860_e108605_d_n5;
        locals.var_psi_dn6 = assign71860_e108605_d_n6;
        locals.var_psi_dn7 = assign71860_e108605_d_n7;
        locals.var_psi_dn8 = assign71860_e108605_d_n8;
        locals.var_psi_dn9 = assign71860_e108605_d_n9;
        locals.var_psi_dn10 = assign71860_e108605_d_n10;
        locals.var_psi_dn13 = assign71860_e108605_d_n13;
        locals.var_psi_rv = 0.0;

        let (assign71870_e108618, assign71870_e108618_d_n0, assign71870_e108618_d_n2, assign71870_e108618_d_n4, assign71870_e108618_d_n5, assign71870_e108618_d_n6, assign71870_e108618_d_n7, assign71870_e108618_d_n8, assign71870_e108618_d_n9, assign71870_e108618_d_n10, assign71870_e108618_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let assign71870_e108615: f64 = (locals.var_beta * 0.1);
        let assign71870_e108616: f64 = (locals.var_psi + assign71870_e108615);
        (assign71870_e108616, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn13 + (locals.var_beta_dn13 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign71870_e108618;
        locals.var_psi_dn0 = assign71870_e108618_d_n0;
        locals.var_psi_dn2 = assign71870_e108618_d_n2;
        locals.var_psi_dn4 = assign71870_e108618_d_n4;
        locals.var_psi_dn5 = assign71870_e108618_d_n5;
        locals.var_psi_dn6 = assign71870_e108618_d_n6;
        locals.var_psi_dn7 = assign71870_e108618_d_n7;
        locals.var_psi_dn8 = assign71870_e108618_d_n8;
        locals.var_psi_dn9 = assign71870_e108618_d_n9;
        locals.var_psi_dn10 = assign71870_e108618_d_n10;
        locals.var_psi_dn13 = assign71870_e108618_d_n13;
        locals.var_psi_rv = 0.0;

        let (assign71880_e108639, assign71880_e108639_d_n0, assign71880_e108639_d_n2, assign71880_e108639_d_n4, assign71880_e108639_d_n5, assign71880_e108639_d_n6, assign71880_e108639_d_n7, assign71880_e108639_d_n8, assign71880_e108639_d_n9, assign71880_e108639_d_n10, assign71880_e108639_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let assign71880_e108627: f64 = (locals.var_gammachi * locals.var_t0);
        let assign71880_e108630: f64 = (locals.var_psi * locals.var_psi);
        let assign71880_e108631: f64 = (assign71880_e108627 + assign71880_e108630);
        let assign71880_e108632: f64 = (assign71880_e108631).ln();
        let assign71880_e108635: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign71880_e108636: f64 = (assign71880_e108635).ln();
        let assign71880_e108637: f64 = (assign71880_e108632 - assign71880_e108636);
        (assign71880_e108637, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign71880_e108631) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign71880_e108635)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign71880_e108631) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign71880_e108635)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign71880_e108631) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign71880_e108635)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign71880_e108631) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign71880_e108635)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign71880_e108631) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign71880_e108635)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign71880_e108631) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign71880_e108635)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign71880_e108631) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign71880_e108635)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign71880_e108631) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign71880_e108635)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign71880_e108631) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign71880_e108635)), (((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign71880_e108631) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign71880_e108635)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign71880_e108639;
        locals.var_t1_dn0 = assign71880_e108639_d_n0;
        locals.var_t1_dn2 = assign71880_e108639_d_n2;
        locals.var_t1_dn4 = assign71880_e108639_d_n4;
        locals.var_t1_dn5 = assign71880_e108639_d_n5;
        locals.var_t1_dn6 = assign71880_e108639_d_n6;
        locals.var_t1_dn7 = assign71880_e108639_d_n7;
        locals.var_t1_dn8 = assign71880_e108639_d_n8;
        locals.var_t1_dn9 = assign71880_e108639_d_n9;
        locals.var_t1_dn10 = assign71880_e108639_d_n10;
        locals.var_t1_dn13 = assign71880_e108639_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign71890_e108652, assign71890_e108652_d_n0, assign71890_e108652_d_n2, assign71890_e108652_d_n4, assign71890_e108652_d_n5, assign71890_e108652_d_n6, assign71890_e108652_d_n7, assign71890_e108652_d_n8, assign71890_e108652_d_n9, assign71890_e108652_d_n10, assign71890_e108652_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let assign71890_e108649: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign71890_e108650: f64 = (locals.var_t1 + assign71890_e108649);
        (assign71890_e108650, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn13 + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign71890_e108652;
        locals.var_chi_b_dn0 = assign71890_e108652_d_n0;
        locals.var_chi_b_dn2 = assign71890_e108652_d_n2;
        locals.var_chi_b_dn4 = assign71890_e108652_d_n4;
        locals.var_chi_b_dn5 = assign71890_e108652_d_n5;
        locals.var_chi_b_dn6 = assign71890_e108652_d_n6;
        locals.var_chi_b_dn7 = assign71890_e108652_d_n7;
        locals.var_chi_b_dn8 = assign71890_e108652_d_n8;
        locals.var_chi_b_dn9 = assign71890_e108652_d_n9;
        locals.var_chi_b_dn10 = assign71890_e108652_d_n10;
        locals.var_chi_b_dn13 = assign71890_e108652_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign71900_e108666, assign71900_e108666_d_n0, assign71900_e108666_d_n2, assign71900_e108666_d_n4, assign71900_e108666_d_n5, assign71900_e108666_d_n6, assign71900_e108666_d_n7, assign71900_e108666_d_n8, assign71900_e108666_d_n9, assign71900_e108666_d_n10, assign71900_e108666_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let (assign71900_e108664, assign71900_e108664_d_n0, assign71900_e108664_d_n2, assign71900_e108664_d_n4, assign71900_e108664_d_n5, assign71900_e108664_d_n6, assign71900_e108664_d_n7, assign71900_e108664_d_n8, assign71900_e108664_d_n9, assign71900_e108664_d_n10, assign71900_e108664_d_n13,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign71900_e108664, assign71900_e108664_d_n0, assign71900_e108664_d_n2, assign71900_e108664_d_n4, assign71900_e108664_d_n5, assign71900_e108664_d_n6, assign71900_e108664_d_n7, assign71900_e108664_d_n8, assign71900_e108664_d_n9, assign71900_e108664_d_n10, assign71900_e108664_d_n13,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign71900_e108666;
        locals.var_chi_b_dn0 = assign71900_e108666_d_n0;
        locals.var_chi_b_dn2 = assign71900_e108666_d_n2;
        locals.var_chi_b_dn4 = assign71900_e108666_d_n4;
        locals.var_chi_b_dn5 = assign71900_e108666_d_n5;
        locals.var_chi_b_dn6 = assign71900_e108666_d_n6;
        locals.var_chi_b_dn7 = assign71900_e108666_d_n7;
        locals.var_chi_b_dn8 = assign71900_e108666_d_n8;
        locals.var_chi_b_dn9 = assign71900_e108666_d_n9;
        locals.var_chi_b_dn10 = assign71900_e108666_d_n10;
        locals.var_chi_b_dn13 = assign71900_e108666_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign71910_e108675, assign71910_e108675_d_n0, assign71910_e108675_d_n2, assign71910_e108675_d_n4, assign71910_e108675_d_n5, assign71910_e108675_d_n6, assign71910_e108675_d_n7, assign71910_e108675_d_n8, assign71910_e108675_d_n9, assign71910_e108675_d_n10, assign71910_e108675_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign71910_e108675;
        locals.var_chi_a_dn0 = assign71910_e108675_d_n0;
        locals.var_chi_a_dn2 = assign71910_e108675_d_n2;
        locals.var_chi_a_dn4 = assign71910_e108675_d_n4;
        locals.var_chi_a_dn5 = assign71910_e108675_d_n5;
        locals.var_chi_a_dn6 = assign71910_e108675_d_n6;
        locals.var_chi_a_dn7 = assign71910_e108675_d_n7;
        locals.var_chi_a_dn8 = assign71910_e108675_d_n8;
        locals.var_chi_a_dn9 = assign71910_e108675_d_n9;
        locals.var_chi_a_dn10 = assign71910_e108675_d_n10;
        locals.var_chi_a_dn13 = assign71910_e108675_d_n13;
        locals.var_chi_a_rv = 0.0;

        let assign71920_e108678: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1671 = assign71920_e108678;
        locals.var_guard1671_rv = 0.0;

        let assign71930_e108683: f64 = (0.2 * locals.var_chi_b);
        let assign71930_e108684: f64 = (locals.var_chi_b - assign71930_e108683);
        let assign71930_e108688: f64 = (0.2 * locals.var_chi_b);
        let assign71930_e108691: f64 = if ((locals.var_chi_a > assign71930_e108684) && (assign71930_e108688 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1672 = assign71930_e108691;
        locals.var_guard1672_rv = 0.0;

        let (assign71940_e108710, assign71940_e108710_d_n0, assign71940_e108710_d_n2, assign71940_e108710_d_n4, assign71940_e108710_d_n5, assign71940_e108710_d_n6, assign71940_e108710_d_n7, assign71940_e108710_d_n8, assign71940_e108710_d_n9, assign71940_e108710_d_n10, assign71940_e108710_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71940_e108704: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign71940_e108707: f64 = (0.2 * locals.var_chi_b);
        let assign71940_e108708: f64 = (assign71940_e108704 + assign71940_e108707);
        (assign71940_e108708, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn13 - locals.var_chi_b_dn13) + (0.2 * locals.var_chi_b_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign71940_e108710;
        locals.var_tmf1_dn0 = assign71940_e108710_d_n0;
        locals.var_tmf1_dn2 = assign71940_e108710_d_n2;
        locals.var_tmf1_dn4 = assign71940_e108710_d_n4;
        locals.var_tmf1_dn5 = assign71940_e108710_d_n5;
        locals.var_tmf1_dn6 = assign71940_e108710_d_n6;
        locals.var_tmf1_dn7 = assign71940_e108710_d_n7;
        locals.var_tmf1_dn8 = assign71940_e108710_d_n8;
        locals.var_tmf1_dn9 = assign71940_e108710_d_n9;
        locals.var_tmf1_dn10 = assign71940_e108710_d_n10;
        locals.var_tmf1_dn13 = assign71940_e108710_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign71950_e108725, assign71950_e108725_d_n0, assign71950_e108725_d_n2, assign71950_e108725_d_n4, assign71950_e108725_d_n5, assign71950_e108725_d_n6, assign71950_e108725_d_n7, assign71950_e108725_d_n8, assign71950_e108725_d_n9, assign71950_e108725_d_n10, assign71950_e108725_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71950_e108723: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign71950_e108723, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign71950_e108725;
        locals.var_x2_dn0 = assign71950_e108725_d_n0;
        locals.var_x2_dn2 = assign71950_e108725_d_n2;
        locals.var_x2_dn4 = assign71950_e108725_d_n4;
        locals.var_x2_dn5 = assign71950_e108725_d_n5;
        locals.var_x2_dn6 = assign71950_e108725_d_n6;
        locals.var_x2_dn7 = assign71950_e108725_d_n7;
        locals.var_x2_dn8 = assign71950_e108725_d_n8;
        locals.var_x2_dn9 = assign71950_e108725_d_n9;
        locals.var_x2_dn10 = assign71950_e108725_d_n10;
        locals.var_x2_dn13 = assign71950_e108725_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign71960_e108744, assign71960_e108744_d_n0, assign71960_e108744_d_n2, assign71960_e108744_d_n4, assign71960_e108744_d_n5, assign71960_e108744_d_n6, assign71960_e108744_d_n7, assign71960_e108744_d_n8, assign71960_e108744_d_n9, assign71960_e108744_d_n10, assign71960_e108744_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71960_e108738: f64 = (0.2 * locals.var_chi_b);
        let assign71960_e108741: f64 = (0.2 * locals.var_chi_b);
        let assign71960_e108742: f64 = (assign71960_e108738 * assign71960_e108741);
        (assign71960_e108742, (((0.2 * locals.var_chi_b_dn0) * assign71960_e108741) + (assign71960_e108738 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign71960_e108741) + (assign71960_e108738 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign71960_e108741) + (assign71960_e108738 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign71960_e108741) + (assign71960_e108738 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign71960_e108741) + (assign71960_e108738 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign71960_e108741) + (assign71960_e108738 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign71960_e108741) + (assign71960_e108738 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign71960_e108741) + (assign71960_e108738 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign71960_e108741) + (assign71960_e108738 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn13) * assign71960_e108741) + (assign71960_e108738 * (0.2 * locals.var_chi_b_dn13))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign71960_e108744;
        locals.var_xmax2_dn0 = assign71960_e108744_d_n0;
        locals.var_xmax2_dn2 = assign71960_e108744_d_n2;
        locals.var_xmax2_dn4 = assign71960_e108744_d_n4;
        locals.var_xmax2_dn5 = assign71960_e108744_d_n5;
        locals.var_xmax2_dn6 = assign71960_e108744_d_n6;
        locals.var_xmax2_dn7 = assign71960_e108744_d_n7;
        locals.var_xmax2_dn8 = assign71960_e108744_d_n8;
        locals.var_xmax2_dn9 = assign71960_e108744_d_n9;
        locals.var_xmax2_dn10 = assign71960_e108744_d_n10;
        locals.var_xmax2_dn13 = assign71960_e108744_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign71970_e108757, assign71970_e108757_d_n0, assign71970_e108757_d_n2, assign71970_e108757_d_n4, assign71970_e108757_d_n5, assign71970_e108757_d_n6, assign71970_e108757_d_n7, assign71970_e108757_d_n8, assign71970_e108757_d_n9, assign71970_e108757_d_n10, assign71970_e108757_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign71970_e108757;
        locals.var_xp_dn0 = assign71970_e108757_d_n0;
        locals.var_xp_dn2 = assign71970_e108757_d_n2;
        locals.var_xp_dn4 = assign71970_e108757_d_n4;
        locals.var_xp_dn5 = assign71970_e108757_d_n5;
        locals.var_xp_dn6 = assign71970_e108757_d_n6;
        locals.var_xp_dn7 = assign71970_e108757_d_n7;
        locals.var_xp_dn8 = assign71970_e108757_d_n8;
        locals.var_xp_dn9 = assign71970_e108757_d_n9;
        locals.var_xp_dn10 = assign71970_e108757_d_n10;
        locals.var_xp_dn13 = assign71970_e108757_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign71980_e108770, assign71980_e108770_d_n0, assign71980_e108770_d_n2, assign71980_e108770_d_n4, assign71980_e108770_d_n5, assign71980_e108770_d_n6, assign71980_e108770_d_n7, assign71980_e108770_d_n8, assign71980_e108770_d_n9, assign71980_e108770_d_n10, assign71980_e108770_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign71980_e108770;
        locals.var_xmp_dn0 = assign71980_e108770_d_n0;
        locals.var_xmp_dn2 = assign71980_e108770_d_n2;
        locals.var_xmp_dn4 = assign71980_e108770_d_n4;
        locals.var_xmp_dn5 = assign71980_e108770_d_n5;
        locals.var_xmp_dn6 = assign71980_e108770_d_n6;
        locals.var_xmp_dn7 = assign71980_e108770_d_n7;
        locals.var_xmp_dn8 = assign71980_e108770_d_n8;
        locals.var_xmp_dn9 = assign71980_e108770_d_n9;
        locals.var_xmp_dn10 = assign71980_e108770_d_n10;
        locals.var_xmp_dn13 = assign71980_e108770_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign71990_e108783,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign71990_e108783;
        locals.var_m0_rv = 0.0;

        let (assign72000_e108796,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72000_e108796;
        locals.var_mm_rv = 0.0;

        let (assign72010_e108809, assign72010_e108809_d_n0, assign72010_e108809_d_n2, assign72010_e108809_d_n4, assign72010_e108809_d_n5, assign72010_e108809_d_n6, assign72010_e108809_d_n7, assign72010_e108809_d_n8, assign72010_e108809_d_n9, assign72010_e108809_d_n10, assign72010_e108809_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign72010_e108809;
        locals.var_arg_dn0 = assign72010_e108809_d_n0;
        locals.var_arg_dn2 = assign72010_e108809_d_n2;
        locals.var_arg_dn4 = assign72010_e108809_d_n4;
        locals.var_arg_dn5 = assign72010_e108809_d_n5;
        locals.var_arg_dn6 = assign72010_e108809_d_n6;
        locals.var_arg_dn7 = assign72010_e108809_d_n7;
        locals.var_arg_dn8 = assign72010_e108809_d_n8;
        locals.var_arg_dn9 = assign72010_e108809_d_n9;
        locals.var_arg_dn10 = assign72010_e108809_d_n10;
        locals.var_arg_dn13 = assign72010_e108809_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign72020_e108822, assign72020_e108822_d_n0, assign72020_e108822_d_n2, assign72020_e108822_d_n4, assign72020_e108822_d_n5, assign72020_e108822_d_n6, assign72020_e108822_d_n7, assign72020_e108822_d_n8, assign72020_e108822_d_n9, assign72020_e108822_d_n10, assign72020_e108822_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign72020_e108822;
        locals.var_dnm_dn0 = assign72020_e108822_d_n0;
        locals.var_dnm_dn2 = assign72020_e108822_d_n2;
        locals.var_dnm_dn4 = assign72020_e108822_d_n4;
        locals.var_dnm_dn5 = assign72020_e108822_d_n5;
        locals.var_dnm_dn6 = assign72020_e108822_d_n6;
        locals.var_dnm_dn7 = assign72020_e108822_d_n7;
        locals.var_dnm_dn8 = assign72020_e108822_d_n8;
        locals.var_dnm_dn9 = assign72020_e108822_d_n9;
        locals.var_dnm_dn10 = assign72020_e108822_d_n10;
        locals.var_dnm_dn13 = assign72020_e108822_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign72030_e108837, assign72030_e108837_d_n0, assign72030_e108837_d_n2, assign72030_e108837_d_n4, assign72030_e108837_d_n5, assign72030_e108837_d_n6, assign72030_e108837_d_n7, assign72030_e108837_d_n8, assign72030_e108837_d_n9, assign72030_e108837_d_n10, assign72030_e108837_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign72030_e108835: f64 = (locals.var_xp * locals.var_x2);
        (assign72030_e108835, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign72030_e108837;
        locals.var_xp_dn0 = assign72030_e108837_d_n0;
        locals.var_xp_dn2 = assign72030_e108837_d_n2;
        locals.var_xp_dn4 = assign72030_e108837_d_n4;
        locals.var_xp_dn5 = assign72030_e108837_d_n5;
        locals.var_xp_dn6 = assign72030_e108837_d_n6;
        locals.var_xp_dn7 = assign72030_e108837_d_n7;
        locals.var_xp_dn8 = assign72030_e108837_d_n8;
        locals.var_xp_dn9 = assign72030_e108837_d_n9;
        locals.var_xp_dn10 = assign72030_e108837_d_n10;
        locals.var_xp_dn13 = assign72030_e108837_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign72040_e108852, assign72040_e108852_d_n0, assign72040_e108852_d_n2, assign72040_e108852_d_n4, assign72040_e108852_d_n5, assign72040_e108852_d_n6, assign72040_e108852_d_n7, assign72040_e108852_d_n8, assign72040_e108852_d_n9, assign72040_e108852_d_n10, assign72040_e108852_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign72040_e108850: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign72040_e108850, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign72040_e108852;
        locals.var_xmp_dn0 = assign72040_e108852_d_n0;
        locals.var_xmp_dn2 = assign72040_e108852_d_n2;
        locals.var_xmp_dn4 = assign72040_e108852_d_n4;
        locals.var_xmp_dn5 = assign72040_e108852_d_n5;
        locals.var_xmp_dn6 = assign72040_e108852_d_n6;
        locals.var_xmp_dn7 = assign72040_e108852_d_n7;
        locals.var_xmp_dn8 = assign72040_e108852_d_n8;
        locals.var_xmp_dn9 = assign72040_e108852_d_n9;
        locals.var_xmp_dn10 = assign72040_e108852_d_n10;
        locals.var_xmp_dn13 = assign72040_e108852_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign72050_e108867, assign72050_e108867_d_n0, assign72050_e108867_d_n2, assign72050_e108867_d_n4, assign72050_e108867_d_n5, assign72050_e108867_d_n6, assign72050_e108867_d_n7, assign72050_e108867_d_n8, assign72050_e108867_d_n9, assign72050_e108867_d_n10, assign72050_e108867_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign72050_e108865: f64 = (locals.var_xp * locals.var_x2);
        (assign72050_e108865, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign72050_e108867;
        locals.var_xp_dn0 = assign72050_e108867_d_n0;
        locals.var_xp_dn2 = assign72050_e108867_d_n2;
        locals.var_xp_dn4 = assign72050_e108867_d_n4;
        locals.var_xp_dn5 = assign72050_e108867_d_n5;
        locals.var_xp_dn6 = assign72050_e108867_d_n6;
        locals.var_xp_dn7 = assign72050_e108867_d_n7;
        locals.var_xp_dn8 = assign72050_e108867_d_n8;
        locals.var_xp_dn9 = assign72050_e108867_d_n9;
        locals.var_xp_dn10 = assign72050_e108867_d_n10;
        locals.var_xp_dn13 = assign72050_e108867_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign72060_e108882, assign72060_e108882_d_n0, assign72060_e108882_d_n2, assign72060_e108882_d_n4, assign72060_e108882_d_n5, assign72060_e108882_d_n6, assign72060_e108882_d_n7, assign72060_e108882_d_n8, assign72060_e108882_d_n9, assign72060_e108882_d_n10, assign72060_e108882_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign72060_e108880: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign72060_e108880, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign72060_e108882;
        locals.var_xmp_dn0 = assign72060_e108882_d_n0;
        locals.var_xmp_dn2 = assign72060_e108882_d_n2;
        locals.var_xmp_dn4 = assign72060_e108882_d_n4;
        locals.var_xmp_dn5 = assign72060_e108882_d_n5;
        locals.var_xmp_dn6 = assign72060_e108882_d_n6;
        locals.var_xmp_dn7 = assign72060_e108882_d_n7;
        locals.var_xmp_dn8 = assign72060_e108882_d_n8;
        locals.var_xmp_dn9 = assign72060_e108882_d_n9;
        locals.var_xmp_dn10 = assign72060_e108882_d_n10;
        locals.var_xmp_dn13 = assign72060_e108882_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign72070_e108897, assign72070_e108897_d_n0, assign72070_e108897_d_n2, assign72070_e108897_d_n4, assign72070_e108897_d_n5, assign72070_e108897_d_n6, assign72070_e108897_d_n7, assign72070_e108897_d_n8, assign72070_e108897_d_n9, assign72070_e108897_d_n10, assign72070_e108897_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign72070_e108895: f64 = (locals.var_xp + locals.var_xmp);
        (assign72070_e108895, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign72070_e108897;
        locals.var_arg_dn0 = assign72070_e108897_d_n0;
        locals.var_arg_dn2 = assign72070_e108897_d_n2;
        locals.var_arg_dn4 = assign72070_e108897_d_n4;
        locals.var_arg_dn5 = assign72070_e108897_d_n5;
        locals.var_arg_dn6 = assign72070_e108897_d_n6;
        locals.var_arg_dn7 = assign72070_e108897_d_n7;
        locals.var_arg_dn8 = assign72070_e108897_d_n8;
        locals.var_arg_dn9 = assign72070_e108897_d_n9;
        locals.var_arg_dn10 = assign72070_e108897_d_n10;
        locals.var_arg_dn13 = assign72070_e108897_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign72080_e108910, assign72080_e108910_d_n0, assign72080_e108910_d_n2, assign72080_e108910_d_n4, assign72080_e108910_d_n5, assign72080_e108910_d_n6, assign72080_e108910_d_n7, assign72080_e108910_d_n8, assign72080_e108910_d_n9, assign72080_e108910_d_n10, assign72080_e108910_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign72080_e108910;
        locals.var_dnm_dn0 = assign72080_e108910_d_n0;
        locals.var_dnm_dn2 = assign72080_e108910_d_n2;
        locals.var_dnm_dn4 = assign72080_e108910_d_n4;
        locals.var_dnm_dn5 = assign72080_e108910_d_n5;
        locals.var_dnm_dn6 = assign72080_e108910_d_n6;
        locals.var_dnm_dn7 = assign72080_e108910_d_n7;
        locals.var_dnm_dn8 = assign72080_e108910_d_n8;
        locals.var_dnm_dn9 = assign72080_e108910_d_n9;
        locals.var_dnm_dn10 = assign72080_e108910_d_n10;
        locals.var_dnm_dn13 = assign72080_e108910_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign72090_e108925: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1673 = assign72090_e108925;
        locals.var_guard1673_rv = 0.0;

        let assign72100_e108928: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1674 = assign72100_e108928;
        locals.var_guard1674_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_260(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign72110_e108945,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72110_e108945;
        locals.var_mm_rv = 0.0;

        let assign72120_e108948: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1675 = assign72120_e108948;
        locals.var_guard1675_rv = 0.0;

        let (assign72130_e108968,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 == 0.0)) && (locals.var_guard1675 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72130_e108968;
        locals.var_mm_rv = 0.0;

        let assign72140_e108971: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1676 = assign72140_e108971;
        locals.var_guard1676_rv = 0.0;

        let (assign72150_e108994,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 == 0.0)) && (locals.var_guard1675 == 0.0)) && (locals.var_guard1676 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72150_e108994;
        locals.var_mm_rv = 0.0;

        let assign72160_e108997: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1677 = assign72160_e108997;
        locals.var_guard1677_rv = 0.0;

        let (assign72170_e109023,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 == 0.0)) && (locals.var_guard1675 == 0.0)) && (locals.var_guard1676 == 0.0)) && (locals.var_guard1677 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72170_e109023;
        locals.var_mm_rv = 0.0;

        let (assign72180_e109038,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) && (locals.var_guard1673 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign72180_e109038;
        locals.var_m0_rv = 0.0;

        let mut assign72190_loop_guard: usize = 0;
        while {
            let assign72190_cond_e109054: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign72190_cond_e109054 != 0.0
        } {
            assign72190_loop_guard += 1;
            assert!(assign72190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign72190_body0_e109070, assign72190_body0_e109070_d_n0, assign72190_body0_e109070_d_n2, assign72190_body0_e109070_d_n4, assign72190_body0_e109070_d_n5, assign72190_body0_e109070_d_n6, assign72190_body0_e109070_d_n7, assign72190_body0_e109070_d_n8, assign72190_body0_e109070_d_n9, assign72190_body0_e109070_d_n10, assign72190_body0_e109070_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) && (locals.var_guard1673 != 0.0)) {
        let assign72190_body0_e109068: f64 = (locals.var_dnm).sqrt();
        (assign72190_body0_e109068, (locals.var_dnm_dn0 / (2.0 * assign72190_body0_e109068)), (locals.var_dnm_dn2 / (2.0 * assign72190_body0_e109068)), (locals.var_dnm_dn4 / (2.0 * assign72190_body0_e109068)), (locals.var_dnm_dn5 / (2.0 * assign72190_body0_e109068)), (locals.var_dnm_dn6 / (2.0 * assign72190_body0_e109068)), (locals.var_dnm_dn7 / (2.0 * assign72190_body0_e109068)), (locals.var_dnm_dn8 / (2.0 * assign72190_body0_e109068)), (locals.var_dnm_dn9 / (2.0 * assign72190_body0_e109068)), (locals.var_dnm_dn10 / (2.0 * assign72190_body0_e109068)), (locals.var_dnm_dn13 / (2.0 * assign72190_body0_e109068)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign72190_body0_e109070;
            locals.var_dnm_dn0 = assign72190_body0_e109070_d_n0;
            locals.var_dnm_dn2 = assign72190_body0_e109070_d_n2;
            locals.var_dnm_dn4 = assign72190_body0_e109070_d_n4;
            locals.var_dnm_dn5 = assign72190_body0_e109070_d_n5;
            locals.var_dnm_dn6 = assign72190_body0_e109070_d_n6;
            locals.var_dnm_dn7 = assign72190_body0_e109070_d_n7;
            locals.var_dnm_dn8 = assign72190_body0_e109070_d_n8;
            locals.var_dnm_dn9 = assign72190_body0_e109070_d_n9;
            locals.var_dnm_dn10 = assign72190_body0_e109070_d_n10;
            locals.var_dnm_dn13 = assign72190_body0_e109070_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign72190_body1_e109087,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) && (locals.var_guard1673 != 0.0)) {
        let assign72190_body1_e109085: f64 = (locals.var_m0 + 1.0);
        (assign72190_body1_e109085,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign72190_body1_e109087;
            locals.var_m0_rv = 0.0;
        }

        let (assign72200_e109114, assign72200_e109114_d_n0, assign72200_e109114_d_n2, assign72200_e109114_d_n4, assign72200_e109114_d_n5, assign72200_e109114_d_n6, assign72200_e109114_d_n7, assign72200_e109114_d_n8, assign72200_e109114_d_n9, assign72200_e109114_d_n10, assign72200_e109114_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) && (locals.var_guard1673 == 0.0)) {
        let (assign72200_e109112, assign72200_e109112_d_n0, assign72200_e109112_d_n2, assign72200_e109112_d_n4, assign72200_e109112_d_n5, assign72200_e109112_d_n6, assign72200_e109112_d_n7, assign72200_e109112_d_n8, assign72200_e109112_d_n9, assign72200_e109112_d_n10, assign72200_e109112_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign72200_e109109: f64 = (2.0 * 2.0);
                let assign72200_e109110: f64 = (1.0 / assign72200_e109109);
                let assign72200_e109111: f64 = (locals.var_dnm).powf(assign72200_e109110);
                (assign72200_e109111, if 0.0 == 0.0 && ((assign72200_e109110) as f64).is_finite() && ((assign72200_e109110) as f64).fract() == 0.0 { if assign72200_e109110 == 0.0 { 0.0 } else { (assign72200_e109110 * ((locals.var_dnm).powf(assign72200_e109110 - 1.0) * locals.var_dnm_dn0)) } } else { (assign72200_e109111 * (assign72200_e109110 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72200_e109110) as f64).is_finite() && ((assign72200_e109110) as f64).fract() == 0.0 { if assign72200_e109110 == 0.0 { 0.0 } else { (assign72200_e109110 * ((locals.var_dnm).powf(assign72200_e109110 - 1.0) * locals.var_dnm_dn2)) } } else { (assign72200_e109111 * (assign72200_e109110 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72200_e109110) as f64).is_finite() && ((assign72200_e109110) as f64).fract() == 0.0 { if assign72200_e109110 == 0.0 { 0.0 } else { (assign72200_e109110 * ((locals.var_dnm).powf(assign72200_e109110 - 1.0) * locals.var_dnm_dn4)) } } else { (assign72200_e109111 * (assign72200_e109110 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72200_e109110) as f64).is_finite() && ((assign72200_e109110) as f64).fract() == 0.0 { if assign72200_e109110 == 0.0 { 0.0 } else { (assign72200_e109110 * ((locals.var_dnm).powf(assign72200_e109110 - 1.0) * locals.var_dnm_dn5)) } } else { (assign72200_e109111 * (assign72200_e109110 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72200_e109110) as f64).is_finite() && ((assign72200_e109110) as f64).fract() == 0.0 { if assign72200_e109110 == 0.0 { 0.0 } else { (assign72200_e109110 * ((locals.var_dnm).powf(assign72200_e109110 - 1.0) * locals.var_dnm_dn6)) } } else { (assign72200_e109111 * (assign72200_e109110 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72200_e109110) as f64).is_finite() && ((assign72200_e109110) as f64).fract() == 0.0 { if assign72200_e109110 == 0.0 { 0.0 } else { (assign72200_e109110 * ((locals.var_dnm).powf(assign72200_e109110 - 1.0) * locals.var_dnm_dn7)) } } else { (assign72200_e109111 * (assign72200_e109110 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72200_e109110) as f64).is_finite() && ((assign72200_e109110) as f64).fract() == 0.0 { if assign72200_e109110 == 0.0 { 0.0 } else { (assign72200_e109110 * ((locals.var_dnm).powf(assign72200_e109110 - 1.0) * locals.var_dnm_dn8)) } } else { (assign72200_e109111 * (assign72200_e109110 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72200_e109110) as f64).is_finite() && ((assign72200_e109110) as f64).fract() == 0.0 { if assign72200_e109110 == 0.0 { 0.0 } else { (assign72200_e109110 * ((locals.var_dnm).powf(assign72200_e109110 - 1.0) * locals.var_dnm_dn9)) } } else { (assign72200_e109111 * (assign72200_e109110 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72200_e109110) as f64).is_finite() && ((assign72200_e109110) as f64).fract() == 0.0 { if assign72200_e109110 == 0.0 { 0.0 } else { (assign72200_e109110 * ((locals.var_dnm).powf(assign72200_e109110 - 1.0) * locals.var_dnm_dn10)) } } else { (assign72200_e109111 * (assign72200_e109110 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72200_e109110) as f64).is_finite() && ((assign72200_e109110) as f64).fract() == 0.0 { if assign72200_e109110 == 0.0 { 0.0 } else { (assign72200_e109110 * ((locals.var_dnm).powf(assign72200_e109110 - 1.0) * locals.var_dnm_dn13)) } } else { (assign72200_e109111 * (assign72200_e109110 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign72200_e109112, assign72200_e109112_d_n0, assign72200_e109112_d_n2, assign72200_e109112_d_n4, assign72200_e109112_d_n5, assign72200_e109112_d_n6, assign72200_e109112_d_n7, assign72200_e109112_d_n8, assign72200_e109112_d_n9, assign72200_e109112_d_n10, assign72200_e109112_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign72200_e109114;
        locals.var_dnm_dn0 = assign72200_e109114_d_n0;
        locals.var_dnm_dn2 = assign72200_e109114_d_n2;
        locals.var_dnm_dn4 = assign72200_e109114_d_n4;
        locals.var_dnm_dn5 = assign72200_e109114_d_n5;
        locals.var_dnm_dn6 = assign72200_e109114_d_n6;
        locals.var_dnm_dn7 = assign72200_e109114_d_n7;
        locals.var_dnm_dn8 = assign72200_e109114_d_n8;
        locals.var_dnm_dn9 = assign72200_e109114_d_n9;
        locals.var_dnm_dn10 = assign72200_e109114_d_n10;
        locals.var_dnm_dn13 = assign72200_e109114_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign72210_e109129, assign72210_e109129_d_n0, assign72210_e109129_d_n2, assign72210_e109129_d_n4, assign72210_e109129_d_n5, assign72210_e109129_d_n6, assign72210_e109129_d_n7, assign72210_e109129_d_n8, assign72210_e109129_d_n9, assign72210_e109129_d_n10, assign72210_e109129_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign72210_e109127: f64 = (1.0 / locals.var_dnm);
        (assign72210_e109127, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign72210_e109129;
        locals.var_dnm_dn0 = assign72210_e109129_d_n0;
        locals.var_dnm_dn2 = assign72210_e109129_d_n2;
        locals.var_dnm_dn4 = assign72210_e109129_d_n4;
        locals.var_dnm_dn5 = assign72210_e109129_d_n5;
        locals.var_dnm_dn6 = assign72210_e109129_d_n6;
        locals.var_dnm_dn7 = assign72210_e109129_d_n7;
        locals.var_dnm_dn8 = assign72210_e109129_d_n8;
        locals.var_dnm_dn9 = assign72210_e109129_d_n9;
        locals.var_dnm_dn10 = assign72210_e109129_d_n10;
        locals.var_dnm_dn13 = assign72210_e109129_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign72220_e109148, assign72220_e109148_d_n0, assign72220_e109148_d_n2, assign72220_e109148_d_n4, assign72220_e109148_d_n5, assign72220_e109148_d_n6, assign72220_e109148_d_n7, assign72220_e109148_d_n8, assign72220_e109148_d_n9, assign72220_e109148_d_n10, assign72220_e109148_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign72220_e109143: f64 = (0.2 * locals.var_chi_b);
        let assign72220_e109144: f64 = (locals.var_tmf1 * assign72220_e109143);
        let assign72220_e109146: f64 = (assign72220_e109144 * locals.var_dnm);
        (assign72220_e109146, ((((locals.var_tmf1_dn0 * assign72220_e109143) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign72220_e109144 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign72220_e109143) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign72220_e109144 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign72220_e109143) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign72220_e109144 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign72220_e109143) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign72220_e109144 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign72220_e109143) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign72220_e109144 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign72220_e109143) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign72220_e109144 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign72220_e109143) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign72220_e109144 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign72220_e109143) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign72220_e109144 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign72220_e109143) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign72220_e109144 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign72220_e109143) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn13))) * locals.var_dnm) + (assign72220_e109144 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign72220_e109148;
        locals.var_tmf0_dn0 = assign72220_e109148_d_n0;
        locals.var_tmf0_dn2 = assign72220_e109148_d_n2;
        locals.var_tmf0_dn4 = assign72220_e109148_d_n4;
        locals.var_tmf0_dn5 = assign72220_e109148_d_n5;
        locals.var_tmf0_dn6 = assign72220_e109148_d_n6;
        locals.var_tmf0_dn7 = assign72220_e109148_d_n7;
        locals.var_tmf0_dn8 = assign72220_e109148_d_n8;
        locals.var_tmf0_dn9 = assign72220_e109148_d_n9;
        locals.var_tmf0_dn10 = assign72220_e109148_d_n10;
        locals.var_tmf0_dn13 = assign72220_e109148_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign72230_e109169, assign72230_e109169_d_n0, assign72230_e109169_d_n2, assign72230_e109169_d_n4, assign72230_e109169_d_n5, assign72230_e109169_d_n6, assign72230_e109169_d_n7, assign72230_e109169_d_n8, assign72230_e109169_d_n9, assign72230_e109169_d_n10, assign72230_e109169_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign72230_e109161: f64 = (0.2 * locals.var_chi_b);
        let assign72230_e109163: f64 = (assign72230_e109161 * locals.var_xmp);
        let assign72230_e109165: f64 = (assign72230_e109163 * locals.var_dnm);
        let assign72230_e109167: f64 = (assign72230_e109165 / locals.var_arg);
        (assign72230_e109167, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign72230_e109161 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign72230_e109163 * locals.var_dnm_dn0)) * locals.var_arg) - (assign72230_e109165 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign72230_e109161 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign72230_e109163 * locals.var_dnm_dn2)) * locals.var_arg) - (assign72230_e109165 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign72230_e109161 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign72230_e109163 * locals.var_dnm_dn4)) * locals.var_arg) - (assign72230_e109165 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign72230_e109161 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign72230_e109163 * locals.var_dnm_dn5)) * locals.var_arg) - (assign72230_e109165 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign72230_e109161 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign72230_e109163 * locals.var_dnm_dn6)) * locals.var_arg) - (assign72230_e109165 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign72230_e109161 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign72230_e109163 * locals.var_dnm_dn7)) * locals.var_arg) - (assign72230_e109165 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign72230_e109161 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign72230_e109163 * locals.var_dnm_dn8)) * locals.var_arg) - (assign72230_e109165 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign72230_e109161 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign72230_e109163 * locals.var_dnm_dn9)) * locals.var_arg) - (assign72230_e109165 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign72230_e109161 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign72230_e109163 * locals.var_dnm_dn10)) * locals.var_arg) - (assign72230_e109165 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn13) * locals.var_xmp) + (assign72230_e109161 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign72230_e109163 * locals.var_dnm_dn13)) * locals.var_arg) - (assign72230_e109165 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign72230_e109169;
        locals.var_t1_dn0 = assign72230_e109169_d_n0;
        locals.var_t1_dn2 = assign72230_e109169_d_n2;
        locals.var_t1_dn4 = assign72230_e109169_d_n4;
        locals.var_t1_dn5 = assign72230_e109169_d_n5;
        locals.var_t1_dn6 = assign72230_e109169_d_n6;
        locals.var_t1_dn7 = assign72230_e109169_d_n7;
        locals.var_t1_dn8 = assign72230_e109169_d_n8;
        locals.var_t1_dn9 = assign72230_e109169_d_n9;
        locals.var_t1_dn10 = assign72230_e109169_d_n10;
        locals.var_t1_dn13 = assign72230_e109169_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign72240_e109188, assign72240_e109188_d_n0, assign72240_e109188_d_n2, assign72240_e109188_d_n4, assign72240_e109188_d_n5, assign72240_e109188_d_n6, assign72240_e109188_d_n7, assign72240_e109188_d_n8, assign72240_e109188_d_n9, assign72240_e109188_d_n10, assign72240_e109188_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign72240_e109183: f64 = (0.2 * locals.var_chi_b);
        let assign72240_e109184: f64 = (locals.var_chi_b - assign72240_e109183);
        let assign72240_e109186: f64 = (assign72240_e109184 + locals.var_tmf0);
        (assign72240_e109186, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn13 - (0.2 * locals.var_chi_b_dn13)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign72240_e109188;
        locals.var_chi_dn0 = assign72240_e109188_d_n0;
        locals.var_chi_dn2 = assign72240_e109188_d_n2;
        locals.var_chi_dn4 = assign72240_e109188_d_n4;
        locals.var_chi_dn5 = assign72240_e109188_d_n5;
        locals.var_chi_dn6 = assign72240_e109188_d_n6;
        locals.var_chi_dn7 = assign72240_e109188_d_n7;
        locals.var_chi_dn8 = assign72240_e109188_d_n8;
        locals.var_chi_dn9 = assign72240_e109188_d_n9;
        locals.var_chi_dn10 = assign72240_e109188_d_n10;
        locals.var_chi_dn13 = assign72240_e109188_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign72250_e109201, assign72250_e109201_d_n0, assign72250_e109201_d_n2, assign72250_e109201_d_n4, assign72250_e109201_d_n5, assign72250_e109201_d_n6, assign72250_e109201_d_n7, assign72250_e109201_d_n8, assign72250_e109201_d_n9, assign72250_e109201_d_n10, assign72250_e109201_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign72250_e109201;
        locals.var_t1_dn0 = assign72250_e109201_d_n0;
        locals.var_t1_dn2 = assign72250_e109201_d_n2;
        locals.var_t1_dn4 = assign72250_e109201_d_n4;
        locals.var_t1_dn5 = assign72250_e109201_d_n5;
        locals.var_t1_dn6 = assign72250_e109201_d_n6;
        locals.var_t1_dn7 = assign72250_e109201_d_n7;
        locals.var_t1_dn8 = assign72250_e109201_d_n8;
        locals.var_t1_dn9 = assign72250_e109201_d_n9;
        locals.var_t1_dn10 = assign72250_e109201_d_n10;
        locals.var_t1_dn13 = assign72250_e109201_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign72260_e109215, assign72260_e109215_d_n0, assign72260_e109215_d_n2, assign72260_e109215_d_n4, assign72260_e109215_d_n5, assign72260_e109215_d_n6, assign72260_e109215_d_n7, assign72260_e109215_d_n8, assign72260_e109215_d_n9, assign72260_e109215_d_n10, assign72260_e109215_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign72260_e109215;
        locals.var_chi_dn0 = assign72260_e109215_d_n0;
        locals.var_chi_dn2 = assign72260_e109215_d_n2;
        locals.var_chi_dn4 = assign72260_e109215_d_n4;
        locals.var_chi_dn5 = assign72260_e109215_d_n5;
        locals.var_chi_dn6 = assign72260_e109215_d_n6;
        locals.var_chi_dn7 = assign72260_e109215_d_n7;
        locals.var_chi_dn8 = assign72260_e109215_d_n8;
        locals.var_chi_dn9 = assign72260_e109215_d_n9;
        locals.var_chi_dn10 = assign72260_e109215_d_n10;
        locals.var_chi_dn13 = assign72260_e109215_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign72270_e109229, assign72270_e109229_d_n0, assign72270_e109229_d_n2, assign72270_e109229_d_n4, assign72270_e109229_d_n5, assign72270_e109229_d_n6, assign72270_e109229_d_n7, assign72270_e109229_d_n8, assign72270_e109229_d_n9, assign72270_e109229_d_n10, assign72270_e109229_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign72270_e109229;
        locals.var_t1_dn0 = assign72270_e109229_d_n0;
        locals.var_t1_dn2 = assign72270_e109229_d_n2;
        locals.var_t1_dn4 = assign72270_e109229_d_n4;
        locals.var_t1_dn5 = assign72270_e109229_d_n5;
        locals.var_t1_dn6 = assign72270_e109229_d_n6;
        locals.var_t1_dn7 = assign72270_e109229_d_n7;
        locals.var_t1_dn8 = assign72270_e109229_d_n8;
        locals.var_t1_dn9 = assign72270_e109229_d_n9;
        locals.var_t1_dn10 = assign72270_e109229_d_n10;
        locals.var_t1_dn13 = assign72270_e109229_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign72280_e109246, assign72280_e109246_d_n0, assign72280_e109246_d_n2, assign72280_e109246_d_n4, assign72280_e109246_d_n5, assign72280_e109246_d_n6, assign72280_e109246_d_n7, assign72280_e109246_d_n8, assign72280_e109246_d_n9, assign72280_e109246_d_n10, assign72280_e109246_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1671 == 0.0)) {
        let (assign72280_e109244, assign72280_e109244_d_n0, assign72280_e109244_d_n2, assign72280_e109244_d_n4, assign72280_e109244_d_n5, assign72280_e109244_d_n6, assign72280_e109244_d_n7, assign72280_e109244_d_n8, assign72280_e109244_d_n9, assign72280_e109244_d_n10, assign72280_e109244_d_n13,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            }
        };
        (assign72280_e109244, assign72280_e109244_d_n0, assign72280_e109244_d_n2, assign72280_e109244_d_n4, assign72280_e109244_d_n5, assign72280_e109244_d_n6, assign72280_e109244_d_n7, assign72280_e109244_d_n8, assign72280_e109244_d_n9, assign72280_e109244_d_n10, assign72280_e109244_d_n13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign72280_e109246;
        locals.var_chi_dn0 = assign72280_e109246_d_n0;
        locals.var_chi_dn2 = assign72280_e109246_d_n2;
        locals.var_chi_dn4 = assign72280_e109246_d_n4;
        locals.var_chi_dn5 = assign72280_e109246_d_n5;
        locals.var_chi_dn6 = assign72280_e109246_d_n6;
        locals.var_chi_dn7 = assign72280_e109246_d_n7;
        locals.var_chi_dn8 = assign72280_e109246_d_n8;
        locals.var_chi_dn9 = assign72280_e109246_d_n9;
        locals.var_chi_dn10 = assign72280_e109246_d_n10;
        locals.var_chi_dn13 = assign72280_e109246_d_n13;
        locals.var_chi_rv = 0.0;

        let assign72290_e109249: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1678 = assign72290_e109249;
        locals.var_guard1678_rv = 0.0;

        let (assign72300_e109262, assign72300_e109262_d_n0, assign72300_e109262_d_n2, assign72300_e109262_d_n4, assign72300_e109262_d_n5, assign72300_e109262_d_n6, assign72300_e109262_d_n7, assign72300_e109262_d_n8, assign72300_e109262_d_n9, assign72300_e109262_d_n10, assign72300_e109262_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign72300_e109258: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign72300_e109260: f64 = (assign72300_e109258 - locals.var_vxbgmtcl);
        (assign72300_e109260, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign72300_e109262;
        locals.var_ps0ld_dn0 = assign72300_e109262_d_n0;
        locals.var_ps0ld_dn2 = assign72300_e109262_d_n2;
        locals.var_ps0ld_dn4 = assign72300_e109262_d_n4;
        locals.var_ps0ld_dn5 = assign72300_e109262_d_n5;
        locals.var_ps0ld_dn6 = assign72300_e109262_d_n6;
        locals.var_ps0ld_dn7 = assign72300_e109262_d_n7;
        locals.var_ps0ld_dn8 = assign72300_e109262_d_n8;
        locals.var_ps0ld_dn9 = assign72300_e109262_d_n9;
        locals.var_ps0ld_dn10 = assign72300_e109262_d_n10;
        locals.var_ps0ld_dn13 = assign72300_e109262_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign72310_e109265: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1679 = assign72310_e109265;
        locals.var_guard1679_rv = 0.0;

        let (assign72320_e109278, assign72320_e109278_d_n0, assign72320_e109278_d_n2, assign72320_e109278_d_n4, assign72320_e109278_d_n5, assign72320_e109278_d_n6, assign72320_e109278_d_n7, assign72320_e109278_d_n8, assign72320_e109278_d_n9, assign72320_e109278_d_n10, assign72320_e109278_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1679 != 0.0)) {
        let assign72320_e109276: f64 = (p.p334 - locals.var_wdep_func);
        (assign72320_e109276, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign72320_e109278;
        locals.var_t2_dn0 = assign72320_e109278_d_n0;
        locals.var_t2_dn2 = assign72320_e109278_d_n2;
        locals.var_t2_dn4 = assign72320_e109278_d_n4;
        locals.var_t2_dn5 = assign72320_e109278_d_n5;
        locals.var_t2_dn6 = assign72320_e109278_d_n6;
        locals.var_t2_dn7 = assign72320_e109278_d_n7;
        locals.var_t2_dn8 = assign72320_e109278_d_n8;
        locals.var_t2_dn9 = assign72320_e109278_d_n9;
        locals.var_t2_dn10 = assign72320_e109278_d_n10;
        locals.var_t2_dn13 = assign72320_e109278_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign72330_e109303, assign72330_e109303_d_n0, assign72330_e109303_d_n2, assign72330_e109303_d_n4, assign72330_e109303_d_n5, assign72330_e109303_d_n6, assign72330_e109303_d_n7, assign72330_e109303_d_n8, assign72330_e109303_d_n9, assign72330_e109303_d_n10, assign72330_e109303_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1679 == 0.0)) {
        let assign72330_e109290: f64 = (locals.var_vdsi + p.p137);
        let assign72330_e109293: f64 = (locals.var_vdsi + p.p137);
        let assign72330_e109294: f64 = (assign72330_e109290 * assign72330_e109293);
        let assign72330_e109297: f64 = (4.0 * 0.1);
        let assign72330_e109299: f64 = (assign72330_e109297 * 0.1);
        let assign72330_e109300: f64 = (assign72330_e109294 + assign72330_e109299);
        let assign72330_e109301: f64 = (assign72330_e109300).sqrt();
        (assign72330_e109301, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign72330_e109293) + (assign72330_e109290 * locals.var_vdsi_dn5)) / (2.0 * assign72330_e109301)), 0.0, (((locals.var_vdsi_dn7 * assign72330_e109293) + (assign72330_e109290 * locals.var_vdsi_dn7)) / (2.0 * assign72330_e109301)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign72330_e109303;
        locals.var_tmf2_dn0 = assign72330_e109303_d_n0;
        locals.var_tmf2_dn2 = assign72330_e109303_d_n2;
        locals.var_tmf2_dn4 = assign72330_e109303_d_n4;
        locals.var_tmf2_dn5 = assign72330_e109303_d_n5;
        locals.var_tmf2_dn6 = assign72330_e109303_d_n6;
        locals.var_tmf2_dn7 = assign72330_e109303_d_n7;
        locals.var_tmf2_dn8 = assign72330_e109303_d_n8;
        locals.var_tmf2_dn9 = assign72330_e109303_d_n9;
        locals.var_tmf2_dn10 = assign72330_e109303_d_n10;
        locals.var_tmf2_dn13 = assign72330_e109303_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign72340_e109323, assign72340_e109323_d_n0, assign72340_e109323_d_n2, assign72340_e109323_d_n4, assign72340_e109323_d_n5, assign72340_e109323_d_n6, assign72340_e109323_d_n7, assign72340_e109323_d_n8, assign72340_e109323_d_n9, assign72340_e109323_d_n10, assign72340_e109323_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1679 == 0.0)) {
        let assign72340_e109317: f64 = (locals.var_vdsi + p.p137);
        let assign72340_e109319: f64 = (assign72340_e109317 / locals.var_tmf2);
        let assign72340_e109320: f64 = (1.0 + assign72340_e109319);
        let assign72340_e109321: f64 = (0.5 * assign72340_e109320);
        (assign72340_e109321, (0.5 * (-((assign72340_e109317 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign72340_e109317 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign72340_e109317 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign72340_e109317 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign72340_e109317 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign72340_e109317 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign72340_e109317 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign72340_e109317 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign72340_e109317 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign72340_e109317 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign72340_e109323;
        locals.var_t9_dn0 = assign72340_e109323_d_n0;
        locals.var_t9_dn2 = assign72340_e109323_d_n2;
        locals.var_t9_dn4 = assign72340_e109323_d_n4;
        locals.var_t9_dn5 = assign72340_e109323_d_n5;
        locals.var_t9_dn6 = assign72340_e109323_d_n6;
        locals.var_t9_dn7 = assign72340_e109323_d_n7;
        locals.var_t9_dn8 = assign72340_e109323_d_n8;
        locals.var_t9_dn9 = assign72340_e109323_d_n9;
        locals.var_t9_dn10 = assign72340_e109323_d_n10;
        locals.var_t9_dn13 = assign72340_e109323_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign72350_e109341, assign72350_e109341_d_n0, assign72350_e109341_d_n2, assign72350_e109341_d_n4, assign72350_e109341_d_n5, assign72350_e109341_d_n6, assign72350_e109341_d_n7, assign72350_e109341_d_n8, assign72350_e109341_d_n9, assign72350_e109341_d_n10, assign72350_e109341_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1679 == 0.0)) {
        let assign72350_e109336: f64 = (locals.var_vdsi + p.p137);
        let assign72350_e109338: f64 = (assign72350_e109336 + locals.var_tmf2);
        let assign72350_e109339: f64 = (0.5 * assign72350_e109338);
        (assign72350_e109339, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign72350_e109341;
        locals.var_t2_dn0 = assign72350_e109341_d_n0;
        locals.var_t2_dn2 = assign72350_e109341_d_n2;
        locals.var_t2_dn4 = assign72350_e109341_d_n4;
        locals.var_t2_dn5 = assign72350_e109341_d_n5;
        locals.var_t2_dn6 = assign72350_e109341_d_n6;
        locals.var_t2_dn7 = assign72350_e109341_d_n7;
        locals.var_t2_dn8 = assign72350_e109341_d_n8;
        locals.var_t2_dn9 = assign72350_e109341_d_n9;
        locals.var_t2_dn10 = assign72350_e109341_d_n10;
        locals.var_t2_dn13 = assign72350_e109341_d_n13;
        locals.var_t2_rv = 0.0;

        let assign72360_e109344: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1680 = assign72360_e109344;
        locals.var_guard1680_rv = 0.0;

        let (assign72370_e109358, assign72370_e109358_d_n0, assign72370_e109358_d_n2, assign72370_e109358_d_n4, assign72370_e109358_d_n5, assign72370_e109358_d_n6, assign72370_e109358_d_n7, assign72370_e109358_d_n8, assign72370_e109358_d_n9, assign72370_e109358_d_n10, assign72370_e109358_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1679 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign72370_e109358;
        locals.var_t2_dn0 = assign72370_e109358_d_n0;
        locals.var_t2_dn2 = assign72370_e109358_d_n2;
        locals.var_t2_dn4 = assign72370_e109358_d_n4;
        locals.var_t2_dn5 = assign72370_e109358_d_n5;
        locals.var_t2_dn6 = assign72370_e109358_d_n6;
        locals.var_t2_dn7 = assign72370_e109358_d_n7;
        locals.var_t2_dn8 = assign72370_e109358_d_n8;
        locals.var_t2_dn9 = assign72370_e109358_d_n9;
        locals.var_t2_dn10 = assign72370_e109358_d_n10;
        locals.var_t2_dn13 = assign72370_e109358_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign72380_e109372, assign72380_e109372_d_n0, assign72380_e109372_d_n2, assign72380_e109372_d_n4, assign72380_e109372_d_n5, assign72380_e109372_d_n6, assign72380_e109372_d_n7, assign72380_e109372_d_n8, assign72380_e109372_d_n9, assign72380_e109372_d_n10, assign72380_e109372_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1679 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign72380_e109372;
        locals.var_t9_dn0 = assign72380_e109372_d_n0;
        locals.var_t9_dn2 = assign72380_e109372_d_n2;
        locals.var_t9_dn4 = assign72380_e109372_d_n4;
        locals.var_t9_dn5 = assign72380_e109372_d_n5;
        locals.var_t9_dn6 = assign72380_e109372_d_n6;
        locals.var_t9_dn7 = assign72380_e109372_d_n7;
        locals.var_t9_dn8 = assign72380_e109372_d_n8;
        locals.var_t9_dn9 = assign72380_e109372_d_n9;
        locals.var_t9_dn10 = assign72380_e109372_d_n10;
        locals.var_t9_dn13 = assign72380_e109372_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign72390_e109389, assign72390_e109389_d_n0, assign72390_e109389_d_n2, assign72390_e109389_d_n4, assign72390_e109389_d_n5, assign72390_e109389_d_n6, assign72390_e109389_d_n7, assign72390_e109389_d_n8, assign72390_e109389_d_n9, assign72390_e109389_d_n10, assign72390_e109389_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1679 == 0.0)) {
        let assign72390_e109384: f64 = (locals.var_kjunc * locals.var_t2);
        let assign72390_e109385: f64 = (assign72390_e109384).sqrt();
        let assign72390_e109387: f64 = (assign72390_e109385 * p.p432);
        (assign72390_e109387, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign72390_e109385)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign72390_e109385)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign72390_e109385)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign72390_e109385)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign72390_e109385)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign72390_e109385)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign72390_e109385)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign72390_e109385)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign72390_e109385)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign72390_e109385)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign72390_e109389;
        locals.var_wjunc0_dn0 = assign72390_e109389_d_n0;
        locals.var_wjunc0_dn2 = assign72390_e109389_d_n2;
        locals.var_wjunc0_dn4 = assign72390_e109389_d_n4;
        locals.var_wjunc0_dn5 = assign72390_e109389_d_n5;
        locals.var_wjunc0_dn6 = assign72390_e109389_d_n6;
        locals.var_wjunc0_dn7 = assign72390_e109389_d_n7;
        locals.var_wjunc0_dn8 = assign72390_e109389_d_n8;
        locals.var_wjunc0_dn9 = assign72390_e109389_d_n9;
        locals.var_wjunc0_dn10 = assign72390_e109389_d_n10;
        locals.var_wjunc0_dn13 = assign72390_e109389_d_n13;
        locals.var_wjunc0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_261(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign72400_e109403, assign72400_e109403_d_n0, assign72400_e109403_d_n2, assign72400_e109403_d_n4, assign72400_e109403_d_n5, assign72400_e109403_d_n6, assign72400_e109403_d_n7, assign72400_e109403_d_n8, assign72400_e109403_d_n9, assign72400_e109403_d_n10, assign72400_e109403_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1679 == 0.0)) {
        let assign72400_e109401: f64 = (p.p334 - locals.var_wjunc0);
        (assign72400_e109401, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign72400_e109403;
        locals.var_t2_dn0 = assign72400_e109403_d_n0;
        locals.var_t2_dn2 = assign72400_e109403_d_n2;
        locals.var_t2_dn4 = assign72400_e109403_d_n4;
        locals.var_t2_dn5 = assign72400_e109403_d_n5;
        locals.var_t2_dn6 = assign72400_e109403_d_n6;
        locals.var_t2_dn7 = assign72400_e109403_d_n7;
        locals.var_t2_dn8 = assign72400_e109403_d_n8;
        locals.var_t2_dn9 = assign72400_e109403_d_n9;
        locals.var_t2_dn10 = assign72400_e109403_d_n10;
        locals.var_t2_dn13 = assign72400_e109403_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign72410_e109425, assign72410_e109425_d_n0, assign72410_e109425_d_n2, assign72410_e109425_d_n4, assign72410_e109425_d_n5, assign72410_e109425_d_n6, assign72410_e109425_d_n7, assign72410_e109425_d_n8, assign72410_e109425_d_n9, assign72410_e109425_d_n10, assign72410_e109425_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign72410_e109412: f64 = (locals.var_t2 * locals.var_t2);
        let assign72410_e109416: f64 = (p.p334 * 0.01);
        let assign72410_e109417: f64 = (4.0 * assign72410_e109416);
        let assign72410_e109420: f64 = (p.p334 * 0.01);
        let assign72410_e109421: f64 = (assign72410_e109417 * assign72410_e109420);
        let assign72410_e109422: f64 = (assign72410_e109412 + assign72410_e109421);
        let assign72410_e109423: f64 = (assign72410_e109422).sqrt();
        (assign72410_e109423, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign72410_e109423)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign72410_e109423)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign72410_e109423)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign72410_e109423)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign72410_e109423)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign72410_e109423)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign72410_e109423)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign72410_e109423)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign72410_e109423)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign72410_e109423)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign72410_e109425;
        locals.var_tmf2_dn0 = assign72410_e109425_d_n0;
        locals.var_tmf2_dn2 = assign72410_e109425_d_n2;
        locals.var_tmf2_dn4 = assign72410_e109425_d_n4;
        locals.var_tmf2_dn5 = assign72410_e109425_d_n5;
        locals.var_tmf2_dn6 = assign72410_e109425_d_n6;
        locals.var_tmf2_dn7 = assign72410_e109425_d_n7;
        locals.var_tmf2_dn8 = assign72410_e109425_d_n8;
        locals.var_tmf2_dn9 = assign72410_e109425_d_n9;
        locals.var_tmf2_dn10 = assign72410_e109425_d_n10;
        locals.var_tmf2_dn13 = assign72410_e109425_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign72420_e109440, assign72420_e109440_d_n0, assign72420_e109440_d_n2, assign72420_e109440_d_n4, assign72420_e109440_d_n5, assign72420_e109440_d_n6, assign72420_e109440_d_n7, assign72420_e109440_d_n8, assign72420_e109440_d_n9, assign72420_e109440_d_n10, assign72420_e109440_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign72420_e109436: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign72420_e109437: f64 = (1.0 + assign72420_e109436);
        let assign72420_e109438: f64 = (0.5 * assign72420_e109437);
        (assign72420_e109438, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign72420_e109440;
        locals.var_t9_dn0 = assign72420_e109440_d_n0;
        locals.var_t9_dn2 = assign72420_e109440_d_n2;
        locals.var_t9_dn4 = assign72420_e109440_d_n4;
        locals.var_t9_dn5 = assign72420_e109440_d_n5;
        locals.var_t9_dn6 = assign72420_e109440_d_n6;
        locals.var_t9_dn7 = assign72420_e109440_d_n7;
        locals.var_t9_dn8 = assign72420_e109440_d_n8;
        locals.var_t9_dn9 = assign72420_e109440_d_n9;
        locals.var_t9_dn10 = assign72420_e109440_d_n10;
        locals.var_t9_dn13 = assign72420_e109440_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign72430_e109453, assign72430_e109453_d_n0, assign72430_e109453_d_n2, assign72430_e109453_d_n4, assign72430_e109453_d_n5, assign72430_e109453_d_n6, assign72430_e109453_d_n7, assign72430_e109453_d_n8, assign72430_e109453_d_n9, assign72430_e109453_d_n10, assign72430_e109453_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign72430_e109450: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign72430_e109451: f64 = (0.5 * assign72430_e109450);
        (assign72430_e109451, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign72430_e109453;
        locals.var_t2_dn0 = assign72430_e109453_d_n0;
        locals.var_t2_dn2 = assign72430_e109453_d_n2;
        locals.var_t2_dn4 = assign72430_e109453_d_n4;
        locals.var_t2_dn5 = assign72430_e109453_d_n5;
        locals.var_t2_dn6 = assign72430_e109453_d_n6;
        locals.var_t2_dn7 = assign72430_e109453_d_n7;
        locals.var_t2_dn8 = assign72430_e109453_d_n8;
        locals.var_t2_dn9 = assign72430_e109453_d_n9;
        locals.var_t2_dn10 = assign72430_e109453_d_n10;
        locals.var_t2_dn13 = assign72430_e109453_d_n13;
        locals.var_t2_rv = 0.0;

        let assign72440_e109456: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1681 = assign72440_e109456;
        locals.var_guard1681_rv = 0.0;

        let (assign72450_e109467, assign72450_e109467_d_n0, assign72450_e109467_d_n2, assign72450_e109467_d_n4, assign72450_e109467_d_n5, assign72450_e109467_d_n6, assign72450_e109467_d_n7, assign72450_e109467_d_n8, assign72450_e109467_d_n9, assign72450_e109467_d_n10, assign72450_e109467_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1681 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign72450_e109467;
        locals.var_t2_dn0 = assign72450_e109467_d_n0;
        locals.var_t2_dn2 = assign72450_e109467_d_n2;
        locals.var_t2_dn4 = assign72450_e109467_d_n4;
        locals.var_t2_dn5 = assign72450_e109467_d_n5;
        locals.var_t2_dn6 = assign72450_e109467_d_n6;
        locals.var_t2_dn7 = assign72450_e109467_d_n7;
        locals.var_t2_dn8 = assign72450_e109467_d_n8;
        locals.var_t2_dn9 = assign72450_e109467_d_n9;
        locals.var_t2_dn10 = assign72450_e109467_d_n10;
        locals.var_t2_dn13 = assign72450_e109467_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign72460_e109478, assign72460_e109478_d_n0, assign72460_e109478_d_n2, assign72460_e109478_d_n4, assign72460_e109478_d_n5, assign72460_e109478_d_n6, assign72460_e109478_d_n7, assign72460_e109478_d_n8, assign72460_e109478_d_n9, assign72460_e109478_d_n10, assign72460_e109478_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1681 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign72460_e109478;
        locals.var_t9_dn0 = assign72460_e109478_d_n0;
        locals.var_t9_dn2 = assign72460_e109478_d_n2;
        locals.var_t9_dn4 = assign72460_e109478_d_n4;
        locals.var_t9_dn5 = assign72460_e109478_d_n5;
        locals.var_t9_dn6 = assign72460_e109478_d_n6;
        locals.var_t9_dn7 = assign72460_e109478_d_n7;
        locals.var_t9_dn8 = assign72460_e109478_d_n8;
        locals.var_t9_dn9 = assign72460_e109478_d_n9;
        locals.var_t9_dn10 = assign72460_e109478_d_n10;
        locals.var_t9_dn13 = assign72460_e109478_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign72470_e109487, assign72470_e109487_d_n0, assign72470_e109487_d_n2, assign72470_e109487_d_n4, assign72470_e109487_d_n5, assign72470_e109487_d_n6, assign72470_e109487_d_n7, assign72470_e109487_d_n8, assign72470_e109487_d_n9, assign72470_e109487_d_n10, assign72470_e109487_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign72470_e109487;
        locals.var_ddriftldc_dn0 = assign72470_e109487_d_n0;
        locals.var_ddriftldc_dn2 = assign72470_e109487_d_n2;
        locals.var_ddriftldc_dn4 = assign72470_e109487_d_n4;
        locals.var_ddriftldc_dn5 = assign72470_e109487_d_n5;
        locals.var_ddriftldc_dn6 = assign72470_e109487_d_n6;
        locals.var_ddriftldc_dn7 = assign72470_e109487_d_n7;
        locals.var_ddriftldc_dn8 = assign72470_e109487_d_n8;
        locals.var_ddriftldc_dn9 = assign72470_e109487_d_n9;
        locals.var_ddriftldc_dn10 = assign72470_e109487_d_n10;
        locals.var_ddriftldc_dn13 = assign72470_e109487_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign72480_e109504, assign72480_e109504_d_n0, assign72480_e109504_d_n2, assign72480_e109504_d_n4, assign72480_e109504_d_n5, assign72480_e109504_d_n6, assign72480_e109504_d_n7, assign72480_e109504_d_n8, assign72480_e109504_d_n9, assign72480_e109504_d_n10, assign72480_e109504_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign72480_e109496: f64 = (locals.var_q_nsubld * locals.var_ddriftldc);
        let assign72480_e109498: f64 = (assign72480_e109496 * locals.var_ddriftldc);
        let assign72480_e109500: f64 = (assign72480_e109498 / 2.0);
        let assign72480_e109502: f64 = (assign72480_e109500 / 1.034943e-10);
        (assign72480_e109502, (((((locals.var_q_nsubld * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign72480_e109496 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign72480_e109496 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign72480_e109496 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign72480_e109496 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign72480_e109496 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign72480_e109496 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign72480_e109496 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign72480_e109496 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign72480_e109496 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign72480_e109496 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign72480_e109504;
        locals.var_dphi_sb_dn0 = assign72480_e109504_d_n0;
        locals.var_dphi_sb_dn2 = assign72480_e109504_d_n2;
        locals.var_dphi_sb_dn4 = assign72480_e109504_d_n4;
        locals.var_dphi_sb_dn5 = assign72480_e109504_d_n5;
        locals.var_dphi_sb_dn6 = assign72480_e109504_d_n6;
        locals.var_dphi_sb_dn7 = assign72480_e109504_d_n7;
        locals.var_dphi_sb_dn8 = assign72480_e109504_d_n8;
        locals.var_dphi_sb_dn9 = assign72480_e109504_d_n9;
        locals.var_dphi_sb_dn10 = assign72480_e109504_d_n10;
        locals.var_dphi_sb_dn13 = assign72480_e109504_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign72490_e109518, assign72490_e109518_d_n0, assign72490_e109518_d_n2, assign72490_e109518_d_n4, assign72490_e109518_d_n5, assign72490_e109518_d_n6, assign72490_e109518_d_n7, assign72490_e109518_d_n8, assign72490_e109518_d_n9, assign72490_e109518_d_n10, assign72490_e109518_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign72490_e109513: f64 = (2.0 * locals.var_beta);
        let assign72490_e109515: f64 = (assign72490_e109513 * locals.var_dphi_sb);
        let assign72490_e109516: f64 = (assign72490_e109515).sqrt();
        (assign72490_e109516, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign72490_e109513 * locals.var_dphi_sb_dn0)) / (2.0 * assign72490_e109516)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign72490_e109513 * locals.var_dphi_sb_dn2)) / (2.0 * assign72490_e109516)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign72490_e109513 * locals.var_dphi_sb_dn4)) / (2.0 * assign72490_e109516)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign72490_e109513 * locals.var_dphi_sb_dn5)) / (2.0 * assign72490_e109516)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign72490_e109513 * locals.var_dphi_sb_dn6)) / (2.0 * assign72490_e109516)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign72490_e109513 * locals.var_dphi_sb_dn7)) / (2.0 * assign72490_e109516)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign72490_e109513 * locals.var_dphi_sb_dn8)) / (2.0 * assign72490_e109516)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign72490_e109513 * locals.var_dphi_sb_dn9)) / (2.0 * assign72490_e109516)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign72490_e109513 * locals.var_dphi_sb_dn10)) / (2.0 * assign72490_e109516)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign72490_e109513 * locals.var_dphi_sb_dn13)) / (2.0 * assign72490_e109516)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign72490_e109518;
        locals.var_t0_dn0 = assign72490_e109518_d_n0;
        locals.var_t0_dn2 = assign72490_e109518_d_n2;
        locals.var_t0_dn4 = assign72490_e109518_d_n4;
        locals.var_t0_dn5 = assign72490_e109518_d_n5;
        locals.var_t0_dn6 = assign72490_e109518_d_n6;
        locals.var_t0_dn7 = assign72490_e109518_d_n7;
        locals.var_t0_dn8 = assign72490_e109518_d_n8;
        locals.var_t0_dn9 = assign72490_e109518_d_n9;
        locals.var_t0_dn10 = assign72490_e109518_d_n10;
        locals.var_t0_dn13 = assign72490_e109518_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign72500_e109534, assign72500_e109534_d_n0, assign72500_e109534_d_n2, assign72500_e109534_d_n4, assign72500_e109534_d_n5, assign72500_e109534_d_n6, assign72500_e109534_d_n7, assign72500_e109534_d_n8, assign72500_e109534_d_n9, assign72500_e109534_d_n10, assign72500_e109534_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign72500_e109526: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign72500_e109528: f64 = (-locals.var_t0);
        let assign72500_e109529: f64 = { let limited_exp_arg = assign72500_e109528; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign72500_e109530: f64 = (assign72500_e109526 + assign72500_e109529);
        let assign72500_e109532: f64 = (assign72500_e109530 / 2.0);
        (assign72500_e109532, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign72500_e109528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign72500_e109528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign72500_e109528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign72500_e109528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign72500_e109528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign72500_e109528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign72500_e109528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign72500_e109528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign72500_e109528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign72500_e109528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign72500_e109534;
        locals.var_t1_dn0 = assign72500_e109534_d_n0;
        locals.var_t1_dn2 = assign72500_e109534_d_n2;
        locals.var_t1_dn4 = assign72500_e109534_d_n4;
        locals.var_t1_dn5 = assign72500_e109534_d_n5;
        locals.var_t1_dn6 = assign72500_e109534_d_n6;
        locals.var_t1_dn7 = assign72500_e109534_d_n7;
        locals.var_t1_dn8 = assign72500_e109534_d_n8;
        locals.var_t1_dn9 = assign72500_e109534_d_n9;
        locals.var_t1_dn10 = assign72500_e109534_d_n10;
        locals.var_t1_dn13 = assign72500_e109534_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign72510_e109546, assign72510_e109546_d_n0, assign72510_e109546_d_n2, assign72510_e109546_d_n4, assign72510_e109546_d_n5, assign72510_e109546_d_n6, assign72510_e109546_d_n7, assign72510_e109546_d_n8, assign72510_e109546_d_n9, assign72510_e109546_d_n10, assign72510_e109546_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign72510_e109542: f64 = (locals.var_t1).ln();
        let assign72510_e109544: f64 = (assign72510_e109542 / locals.var_dphi_sb);
        (assign72510_e109544, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign72510_e109542 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign72510_e109542 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign72510_e109542 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign72510_e109542 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign72510_e109542 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign72510_e109542 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign72510_e109542 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign72510_e109542 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign72510_e109542 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign72510_e109542 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign72510_e109546;
        locals.var_c_sb_dn0 = assign72510_e109546_d_n0;
        locals.var_c_sb_dn2 = assign72510_e109546_d_n2;
        locals.var_c_sb_dn4 = assign72510_e109546_d_n4;
        locals.var_c_sb_dn5 = assign72510_e109546_d_n5;
        locals.var_c_sb_dn6 = assign72510_e109546_d_n6;
        locals.var_c_sb_dn7 = assign72510_e109546_d_n7;
        locals.var_c_sb_dn8 = assign72510_e109546_d_n8;
        locals.var_c_sb_dn9 = assign72510_e109546_d_n9;
        locals.var_c_sb_dn10 = assign72510_e109546_d_n10;
        locals.var_c_sb_dn13 = assign72510_e109546_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign72520_e109557, assign72520_e109557_d_n0, assign72520_e109557_d_n2, assign72520_e109557_d_n4, assign72520_e109557_d_n5, assign72520_e109557_d_n6, assign72520_e109557_d_n7, assign72520_e109557_d_n8, assign72520_e109557_d_n9, assign72520_e109557_d_n10, assign72520_e109557_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign72520_e109555: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign72520_e109555, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
        locals.var_ps0ld_vxb = assign72520_e109557;
        locals.var_ps0ld_vxb_dn0 = assign72520_e109557_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign72520_e109557_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign72520_e109557_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign72520_e109557_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign72520_e109557_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign72520_e109557_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign72520_e109557_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign72520_e109557_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign72520_e109557_d_n10;
        locals.var_ps0ld_vxb_dn13 = assign72520_e109557_d_n13;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign72530_e109570, assign72530_e109570_d_n0, assign72530_e109570_d_n2, assign72530_e109570_d_n4, assign72530_e109570_d_n5, assign72530_e109570_d_n6, assign72530_e109570_d_n7, assign72530_e109570_d_n8, assign72530_e109570_d_n9, assign72530_e109570_d_n10, assign72530_e109570_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign72530_e109567: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign72530_e109568: f64 = (locals.var_c_sb * assign72530_e109567);
        (assign72530_e109568, ((locals.var_c_sb_dn0 * assign72530_e109567) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign72530_e109567) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign72530_e109567) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign72530_e109567) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign72530_e109567) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign72530_e109567) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign72530_e109567) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign72530_e109567) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign72530_e109567) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign72530_e109567) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign72530_e109570;
        locals.var_ty_dn0 = assign72530_e109570_d_n0;
        locals.var_ty_dn2 = assign72530_e109570_d_n2;
        locals.var_ty_dn4 = assign72530_e109570_d_n4;
        locals.var_ty_dn5 = assign72530_e109570_d_n5;
        locals.var_ty_dn6 = assign72530_e109570_d_n6;
        locals.var_ty_dn7 = assign72530_e109570_d_n7;
        locals.var_ty_dn8 = assign72530_e109570_d_n8;
        locals.var_ty_dn9 = assign72530_e109570_d_n9;
        locals.var_ty_dn10 = assign72530_e109570_d_n10;
        locals.var_ty_dn13 = assign72530_e109570_d_n13;
        locals.var_ty_rv = 0.0;

        let assign72540_e109573: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard1682 = assign72540_e109573;
        locals.var_guard1682_rv = 0.0;

        let (assign72550_e109585, assign72550_e109585_d_n0, assign72550_e109585_d_n2, assign72550_e109585_d_n4, assign72550_e109585_d_n5, assign72550_e109585_d_n6, assign72550_e109585_d_n7, assign72550_e109585_d_n8, assign72550_e109585_d_n9, assign72550_e109585_d_n10, assign72550_e109585_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1682 != 0.0)) {
        let assign72550_e109583: f64 = (locals.var_ty).exp();
        (assign72550_e109583, (assign72550_e109583 * locals.var_ty_dn0), (assign72550_e109583 * locals.var_ty_dn2), (assign72550_e109583 * locals.var_ty_dn4), (assign72550_e109583 * locals.var_ty_dn5), (assign72550_e109583 * locals.var_ty_dn6), (assign72550_e109583 * locals.var_ty_dn7), (assign72550_e109583 * locals.var_ty_dn8), (assign72550_e109583 * locals.var_ty_dn9), (assign72550_e109583 * locals.var_ty_dn10), (assign72550_e109583 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign72550_e109585;
        locals.var_t1_dn0 = assign72550_e109585_d_n0;
        locals.var_t1_dn2 = assign72550_e109585_d_n2;
        locals.var_t1_dn4 = assign72550_e109585_d_n4;
        locals.var_t1_dn5 = assign72550_e109585_d_n5;
        locals.var_t1_dn6 = assign72550_e109585_d_n6;
        locals.var_t1_dn7 = assign72550_e109585_d_n7;
        locals.var_t1_dn8 = assign72550_e109585_d_n8;
        locals.var_t1_dn9 = assign72550_e109585_d_n9;
        locals.var_t1_dn10 = assign72550_e109585_d_n10;
        locals.var_t1_dn13 = assign72550_e109585_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign72560_e109600, assign72560_e109600_d_n0, assign72560_e109600_d_n2, assign72560_e109600_d_n4, assign72560_e109600_d_n5, assign72560_e109600_d_n6, assign72560_e109600_d_n7, assign72560_e109600_d_n8, assign72560_e109600_d_n9, assign72560_e109600_d_n10, assign72560_e109600_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1682 != 0.0)) {
        let assign72560_e109595: f64 = (-locals.var_c_sb);
        let assign72560_e109597: f64 = (assign72560_e109595 * locals.var_dphi_sb);
        let assign72560_e109598: f64 = (assign72560_e109597).exp();
        (assign72560_e109598, (assign72560_e109598 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign72560_e109595 * locals.var_dphi_sb_dn0))), (assign72560_e109598 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign72560_e109595 * locals.var_dphi_sb_dn2))), (assign72560_e109598 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign72560_e109595 * locals.var_dphi_sb_dn4))), (assign72560_e109598 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign72560_e109595 * locals.var_dphi_sb_dn5))), (assign72560_e109598 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign72560_e109595 * locals.var_dphi_sb_dn6))), (assign72560_e109598 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign72560_e109595 * locals.var_dphi_sb_dn7))), (assign72560_e109598 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign72560_e109595 * locals.var_dphi_sb_dn8))), (assign72560_e109598 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign72560_e109595 * locals.var_dphi_sb_dn9))), (assign72560_e109598 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign72560_e109595 * locals.var_dphi_sb_dn10))), (assign72560_e109598 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign72560_e109595 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign72560_e109600;
        locals.var_t0_dn0 = assign72560_e109600_d_n0;
        locals.var_t0_dn2 = assign72560_e109600_d_n2;
        locals.var_t0_dn4 = assign72560_e109600_d_n4;
        locals.var_t0_dn5 = assign72560_e109600_d_n5;
        locals.var_t0_dn6 = assign72560_e109600_d_n6;
        locals.var_t0_dn7 = assign72560_e109600_d_n7;
        locals.var_t0_dn8 = assign72560_e109600_d_n8;
        locals.var_t0_dn9 = assign72560_e109600_d_n9;
        locals.var_t0_dn10 = assign72560_e109600_d_n10;
        locals.var_t0_dn13 = assign72560_e109600_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign72570_e109613, assign72570_e109613_d_n0, assign72570_e109613_d_n2, assign72570_e109613_d_n4, assign72570_e109613_d_n5, assign72570_e109613_d_n6, assign72570_e109613_d_n7, assign72570_e109613_d_n8, assign72570_e109613_d_n9, assign72570_e109613_d_n10, assign72570_e109613_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1682 != 0.0)) {
        let assign72570_e109611: f64 = (locals.var_t1 - locals.var_t0);
        (assign72570_e109611, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign72570_e109613;
        locals.var_t2_dn0 = assign72570_e109613_d_n0;
        locals.var_t2_dn2 = assign72570_e109613_d_n2;
        locals.var_t2_dn4 = assign72570_e109613_d_n4;
        locals.var_t2_dn5 = assign72570_e109613_d_n5;
        locals.var_t2_dn6 = assign72570_e109613_d_n6;
        locals.var_t2_dn7 = assign72570_e109613_d_n7;
        locals.var_t2_dn8 = assign72570_e109613_d_n8;
        locals.var_t2_dn9 = assign72570_e109613_d_n9;
        locals.var_t2_dn10 = assign72570_e109613_d_n10;
        locals.var_t2_dn13 = assign72570_e109613_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign72580_e109629, assign72580_e109629_d_n0, assign72580_e109629_d_n2, assign72580_e109629_d_n4, assign72580_e109629_d_n5, assign72580_e109629_d_n6, assign72580_e109629_d_n7, assign72580_e109629_d_n8, assign72580_e109629_d_n9, assign72580_e109629_d_n10, assign72580_e109629_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1682 != 0.0)) {
        let assign72580_e109624: f64 = (1.0 + locals.var_t2);
        let assign72580_e109625: f64 = (assign72580_e109624).ln();
        let assign72580_e109627: f64 = (assign72580_e109625 / locals.var_c_sb);
        (assign72580_e109627, ((((locals.var_t2_dn0 / assign72580_e109624) * locals.var_c_sb) - (assign72580_e109625 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign72580_e109624) * locals.var_c_sb) - (assign72580_e109625 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign72580_e109624) * locals.var_c_sb) - (assign72580_e109625 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign72580_e109624) * locals.var_c_sb) - (assign72580_e109625 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign72580_e109624) * locals.var_c_sb) - (assign72580_e109625 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign72580_e109624) * locals.var_c_sb) - (assign72580_e109625 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign72580_e109624) * locals.var_c_sb) - (assign72580_e109625 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign72580_e109624) * locals.var_c_sb) - (assign72580_e109625 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign72580_e109624) * locals.var_c_sb) - (assign72580_e109625 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign72580_e109624) * locals.var_c_sb) - (assign72580_e109625 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign72580_e109629;
        locals.var_phi_b_dn0 = assign72580_e109629_d_n0;
        locals.var_phi_b_dn2 = assign72580_e109629_d_n2;
        locals.var_phi_b_dn4 = assign72580_e109629_d_n4;
        locals.var_phi_b_dn5 = assign72580_e109629_d_n5;
        locals.var_phi_b_dn6 = assign72580_e109629_d_n6;
        locals.var_phi_b_dn7 = assign72580_e109629_d_n7;
        locals.var_phi_b_dn8 = assign72580_e109629_d_n8;
        locals.var_phi_b_dn9 = assign72580_e109629_d_n9;
        locals.var_phi_b_dn10 = assign72580_e109629_d_n10;
        locals.var_phi_b_dn13 = assign72580_e109629_d_n13;
        locals.var_phi_b_rv = 0.0;

        let (assign72590_e109643, assign72590_e109643_d_n0, assign72590_e109643_d_n2, assign72590_e109643_d_n4, assign72590_e109643_d_n5, assign72590_e109643_d_n6, assign72590_e109643_d_n7, assign72590_e109643_d_n8, assign72590_e109643_d_n9, assign72590_e109643_d_n10, assign72590_e109643_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1682 == 0.0)) {
        let assign72590_e109641: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign72590_e109641, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign72590_e109643;
        locals.var_phi_b_dn0 = assign72590_e109643_d_n0;
        locals.var_phi_b_dn2 = assign72590_e109643_d_n2;
        locals.var_phi_b_dn4 = assign72590_e109643_d_n4;
        locals.var_phi_b_dn5 = assign72590_e109643_d_n5;
        locals.var_phi_b_dn6 = assign72590_e109643_d_n6;
        locals.var_phi_b_dn7 = assign72590_e109643_d_n7;
        locals.var_phi_b_dn8 = assign72590_e109643_d_n8;
        locals.var_phi_b_dn9 = assign72590_e109643_d_n9;
        locals.var_phi_b_dn10 = assign72590_e109643_d_n10;
        locals.var_phi_b_dn13 = assign72590_e109643_d_n13;
        locals.var_phi_b_rv = 0.0;

        let (assign72600_e109654, assign72600_e109654_d_n0, assign72600_e109654_d_n2, assign72600_e109654_d_n4, assign72600_e109654_d_n5, assign72600_e109654_d_n6, assign72600_e109654_d_n7, assign72600_e109654_d_n8, assign72600_e109654_d_n9, assign72600_e109654_d_n10, assign72600_e109654_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign72600_e109652: f64 = (locals.var_beta * locals.var_phi_b);
        (assign72600_e109652, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
        locals.var_chib = assign72600_e109654;
        locals.var_chib_dn0 = assign72600_e109654_d_n0;
        locals.var_chib_dn2 = assign72600_e109654_d_n2;
        locals.var_chib_dn4 = assign72600_e109654_d_n4;
        locals.var_chib_dn5 = assign72600_e109654_d_n5;
        locals.var_chib_dn6 = assign72600_e109654_d_n6;
        locals.var_chib_dn7 = assign72600_e109654_d_n7;
        locals.var_chib_dn8 = assign72600_e109654_d_n8;
        locals.var_chib_dn9 = assign72600_e109654_d_n9;
        locals.var_chib_dn10 = assign72600_e109654_d_n10;
        locals.var_chib_dn13 = assign72600_e109654_d_n13;
        locals.var_chib_rv = 0.0;

        let assign72610_e109658: f64 = (locals.var_chi / 100.0);
        let assign72610_e109663: f64 = if ((locals.var_chib > assign72610_e109658) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1683 = assign72610_e109663;
        locals.var_guard1683_rv = 0.0;

        let (assign72620_e109676,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1683 != 0.0)) {
        let assign72620_e109674: f64 = (locals.var_flg_fd_mode + 1.0);
        (assign72620_e109674,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign72620_e109676;
        locals.var_flg_fd_mode_rv = 0.0;

        let (assign72630_e109687, assign72630_e109687_d_n0, assign72630_e109687_d_n2, assign72630_e109687_d_n4, assign72630_e109687_d_n5, assign72630_e109687_d_n6, assign72630_e109687_d_n7, assign72630_e109687_d_n8, assign72630_e109687_d_n9, assign72630_e109687_d_n10, assign72630_e109687_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1683 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign72630_e109687;
        locals.var_chi_dn0 = assign72630_e109687_d_n0;
        locals.var_chi_dn2 = assign72630_e109687_d_n2;
        locals.var_chi_dn4 = assign72630_e109687_d_n4;
        locals.var_chi_dn5 = assign72630_e109687_d_n5;
        locals.var_chi_dn6 = assign72630_e109687_d_n6;
        locals.var_chi_dn7 = assign72630_e109687_d_n7;
        locals.var_chi_dn8 = assign72630_e109687_d_n8;
        locals.var_chi_dn9 = assign72630_e109687_d_n9;
        locals.var_chi_dn10 = assign72630_e109687_d_n10;
        locals.var_chi_dn13 = assign72630_e109687_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign72640_e109698, assign72640_e109698_d_n0, assign72640_e109698_d_n2, assign72640_e109698_d_n4, assign72640_e109698_d_n5, assign72640_e109698_d_n6, assign72640_e109698_d_n7, assign72640_e109698_d_n8, assign72640_e109698_d_n9, assign72640_e109698_d_n10, assign72640_e109698_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) {
        let assign72640_e109694: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign72640_e109696: f64 = (assign72640_e109694 - locals.var_vxbgmtcl);
        (assign72640_e109696, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign72640_e109698;
        locals.var_ps0ld_dn0 = assign72640_e109698_d_n0;
        locals.var_ps0ld_dn2 = assign72640_e109698_d_n2;
        locals.var_ps0ld_dn4 = assign72640_e109698_d_n4;
        locals.var_ps0ld_dn5 = assign72640_e109698_d_n5;
        locals.var_ps0ld_dn6 = assign72640_e109698_d_n6;
        locals.var_ps0ld_dn7 = assign72640_e109698_d_n7;
        locals.var_ps0ld_dn8 = assign72640_e109698_d_n8;
        locals.var_ps0ld_dn9 = assign72640_e109698_d_n9;
        locals.var_ps0ld_dn10 = assign72640_e109698_d_n10;
        locals.var_ps0ld_dn13 = assign72640_e109698_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign72650_e109700: f64 = (locals.var_chi).abs();
        let assign72650_e109702: f64 = if assign72650_e109700 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1684 = assign72650_e109702;
        locals.var_guard1684_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_262(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign72660_e109717, assign72660_e109717_d_n0, assign72660_e109717_d_n2, assign72660_e109717_d_n4, assign72660_e109717_d_n5, assign72660_e109717_d_n6, assign72660_e109717_d_n7, assign72660_e109717_d_n8, assign72660_e109717_d_n9, assign72660_e109717_d_n10, assign72660_e109717_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1684 != 0.0)) {
        let assign72660_e109711: f64 = (locals.var_chi - 1.0);
        let assign72660_e109713: f64 = (-locals.var_chi);
        let assign72660_e109714: f64 = (assign72660_e109713).exp();
        let assign72660_e109715: f64 = (assign72660_e109711 + assign72660_e109714);
        (assign72660_e109715, (locals.var_chi_dn0 + (assign72660_e109714 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign72660_e109714 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign72660_e109714 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign72660_e109714 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign72660_e109714 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign72660_e109714 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign72660_e109714 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign72660_e109714 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign72660_e109714 * (-locals.var_chi_dn10))), (locals.var_chi_dn13 + (assign72660_e109714 * (-locals.var_chi_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign72660_e109717;
        locals.var_t1_dn0 = assign72660_e109717_d_n0;
        locals.var_t1_dn2 = assign72660_e109717_d_n2;
        locals.var_t1_dn4 = assign72660_e109717_d_n4;
        locals.var_t1_dn5 = assign72660_e109717_d_n5;
        locals.var_t1_dn6 = assign72660_e109717_d_n6;
        locals.var_t1_dn7 = assign72660_e109717_d_n7;
        locals.var_t1_dn8 = assign72660_e109717_d_n8;
        locals.var_t1_dn9 = assign72660_e109717_d_n9;
        locals.var_t1_dn10 = assign72660_e109717_d_n10;
        locals.var_t1_dn13 = assign72660_e109717_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign72670_e109727, assign72670_e109727_d_n0, assign72670_e109727_d_n2, assign72670_e109727_d_n4, assign72670_e109727_d_n5, assign72670_e109727_d_n6, assign72670_e109727_d_n7, assign72670_e109727_d_n8, assign72670_e109727_d_n9, assign72670_e109727_d_n10, assign72670_e109727_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1684 != 0.0)) {
        let assign72670_e109725: f64 = (locals.var_t1).sqrt();
        (assign72670_e109725, (locals.var_t1_dn0 / (2.0 * assign72670_e109725)), (locals.var_t1_dn2 / (2.0 * assign72670_e109725)), (locals.var_t1_dn4 / (2.0 * assign72670_e109725)), (locals.var_t1_dn5 / (2.0 * assign72670_e109725)), (locals.var_t1_dn6 / (2.0 * assign72670_e109725)), (locals.var_t1_dn7 / (2.0 * assign72670_e109725)), (locals.var_t1_dn8 / (2.0 * assign72670_e109725)), (locals.var_t1_dn9 / (2.0 * assign72670_e109725)), (locals.var_t1_dn10 / (2.0 * assign72670_e109725)), (locals.var_t1_dn13 / (2.0 * assign72670_e109725)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign72670_e109727;
        locals.var_t2_dn0 = assign72670_e109727_d_n0;
        locals.var_t2_dn2 = assign72670_e109727_d_n2;
        locals.var_t2_dn4 = assign72670_e109727_d_n4;
        locals.var_t2_dn5 = assign72670_e109727_d_n5;
        locals.var_t2_dn6 = assign72670_e109727_d_n6;
        locals.var_t2_dn7 = assign72670_e109727_d_n7;
        locals.var_t2_dn8 = assign72670_e109727_d_n8;
        locals.var_t2_dn9 = assign72670_e109727_d_n9;
        locals.var_t2_dn10 = assign72670_e109727_d_n10;
        locals.var_t2_dn13 = assign72670_e109727_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign72690_e109758, assign72690_e109758_d_n0, assign72690_e109758_d_n2, assign72690_e109758_d_n4, assign72690_e109758_d_n5, assign72690_e109758_d_n6, assign72690_e109758_d_n7, assign72690_e109758_d_n8, assign72690_e109758_d_n9, assign72690_e109758_d_n10, assign72690_e109758_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1684 == 0.0)) {
        let assign72690_e109749: f64 = (0.7071067811865475 * locals.var_chi);
        let assign72690_e109753: f64 = (locals.var_chi * 0.3333333333333333);
        let assign72690_e109754: f64 = (1.0 - assign72690_e109753);
        let assign72690_e109755: f64 = (assign72690_e109754).sqrt();
        let assign72690_e109756: f64 = (assign72690_e109749 * assign72690_e109755);
        (assign72690_e109756, (((0.7071067811865475 * locals.var_chi_dn0) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn13) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn13 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign72690_e109758;
        locals.var_t2_dn0 = assign72690_e109758_d_n0;
        locals.var_t2_dn2 = assign72690_e109758_d_n2;
        locals.var_t2_dn4 = assign72690_e109758_d_n4;
        locals.var_t2_dn5 = assign72690_e109758_d_n5;
        locals.var_t2_dn6 = assign72690_e109758_d_n6;
        locals.var_t2_dn7 = assign72690_e109758_d_n7;
        locals.var_t2_dn8 = assign72690_e109758_d_n8;
        locals.var_t2_dn9 = assign72690_e109758_d_n9;
        locals.var_t2_dn10 = assign72690_e109758_d_n10;
        locals.var_t2_dn13 = assign72690_e109758_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign72700_e109767, assign72700_e109767_d_n0, assign72700_e109767_d_n2, assign72700_e109767_d_n4, assign72700_e109767_d_n5, assign72700_e109767_d_n6, assign72700_e109767_d_n7, assign72700_e109767_d_n8, assign72700_e109767_d_n9, assign72700_e109767_d_n10, assign72700_e109767_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) {
        let assign72700_e109765: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign72700_e109765, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign72700_e109767;
        locals.var_qbuld_dn0 = assign72700_e109767_d_n0;
        locals.var_qbuld_dn2 = assign72700_e109767_d_n2;
        locals.var_qbuld_dn4 = assign72700_e109767_d_n4;
        locals.var_qbuld_dn5 = assign72700_e109767_d_n5;
        locals.var_qbuld_dn6 = assign72700_e109767_d_n6;
        locals.var_qbuld_dn7 = assign72700_e109767_d_n7;
        locals.var_qbuld_dn8 = assign72700_e109767_d_n8;
        locals.var_qbuld_dn9 = assign72700_e109767_d_n9;
        locals.var_qbuld_dn10 = assign72700_e109767_d_n10;
        locals.var_qbuld_dn13 = assign72700_e109767_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign72710_e109778, assign72710_e109778_d_n0, assign72710_e109778_d_n2, assign72710_e109778_d_n4, assign72710_e109778_d_n5, assign72710_e109778_d_n6, assign72710_e109778_d_n7, assign72710_e109778_d_n8, assign72710_e109778_d_n9, assign72710_e109778_d_n10, assign72710_e109778_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) {
        let assign72710_e109775: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign72710_e109776: f64 = (locals.var_cox0_func * assign72710_e109775);
        (assign72710_e109776, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (-locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn13)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign72710_e109778;
        locals.var_qsuld_dn0 = assign72710_e109778_d_n0;
        locals.var_qsuld_dn2 = assign72710_e109778_d_n2;
        locals.var_qsuld_dn4 = assign72710_e109778_d_n4;
        locals.var_qsuld_dn5 = assign72710_e109778_d_n5;
        locals.var_qsuld_dn6 = assign72710_e109778_d_n6;
        locals.var_qsuld_dn7 = assign72710_e109778_d_n7;
        locals.var_qsuld_dn8 = assign72710_e109778_d_n8;
        locals.var_qsuld_dn9 = assign72710_e109778_d_n9;
        locals.var_qsuld_dn10 = assign72710_e109778_d_n10;
        locals.var_qsuld_dn13 = assign72710_e109778_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign72720_e109787, assign72720_e109787_d_n0, assign72720_e109787_d_n2, assign72720_e109787_d_n4, assign72720_e109787_d_n5, assign72720_e109787_d_n6, assign72720_e109787_d_n7, assign72720_e109787_d_n8, assign72720_e109787_d_n9, assign72720_e109787_d_n10, assign72720_e109787_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) {
        let assign72720_e109785: f64 = (locals.var_qbuld / locals.var_q_nsubld);
        (assign72720_e109785, (locals.var_qbuld_dn0 / locals.var_q_nsubld), (locals.var_qbuld_dn2 / locals.var_q_nsubld), (locals.var_qbuld_dn4 / locals.var_q_nsubld), (locals.var_qbuld_dn5 / locals.var_q_nsubld), (locals.var_qbuld_dn6 / locals.var_q_nsubld), (locals.var_qbuld_dn7 / locals.var_q_nsubld), (locals.var_qbuld_dn8 / locals.var_q_nsubld), (locals.var_qbuld_dn9 / locals.var_q_nsubld), (locals.var_qbuld_dn10 / locals.var_q_nsubld), (locals.var_qbuld_dn13 / locals.var_q_nsubld),)
    } else {
        (locals.var_wdld0, locals.var_wdld0_dn0, locals.var_wdld0_dn2, locals.var_wdld0_dn4, locals.var_wdld0_dn5, locals.var_wdld0_dn6, locals.var_wdld0_dn7, locals.var_wdld0_dn8, locals.var_wdld0_dn9, locals.var_wdld0_dn10, locals.var_wdld0_dn13,)
    }
};
        locals.var_wdld0 = assign72720_e109787;
        locals.var_wdld0_dn0 = assign72720_e109787_d_n0;
        locals.var_wdld0_dn2 = assign72720_e109787_d_n2;
        locals.var_wdld0_dn4 = assign72720_e109787_d_n4;
        locals.var_wdld0_dn5 = assign72720_e109787_d_n5;
        locals.var_wdld0_dn6 = assign72720_e109787_d_n6;
        locals.var_wdld0_dn7 = assign72720_e109787_d_n7;
        locals.var_wdld0_dn8 = assign72720_e109787_d_n8;
        locals.var_wdld0_dn9 = assign72720_e109787_d_n9;
        locals.var_wdld0_dn10 = assign72720_e109787_d_n10;
        locals.var_wdld0_dn13 = assign72720_e109787_d_n13;
        locals.var_wdld0_rv = 0.0;

        let assign72730_e109790: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1687 = assign72730_e109790;
        locals.var_guard1687_rv = 0.0;

        let assign72740_e109795: f64 = (locals.var_ddriftldc * 0.1);
        let assign72740_e109796: f64 = (locals.var_ddriftldc - assign72740_e109795);
        let assign72740_e109800: f64 = (locals.var_ddriftldc * 0.1);
        let assign72740_e109803: f64 = if ((locals.var_wdld0 > assign72740_e109796) && (assign72740_e109800 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1688 = assign72740_e109803;
        locals.var_guard1688_rv = 0.0;

        let (assign72750_e109820, assign72750_e109820_d_n0, assign72750_e109820_d_n2, assign72750_e109820_d_n4, assign72750_e109820_d_n5, assign72750_e109820_d_n6, assign72750_e109820_d_n7, assign72750_e109820_d_n8, assign72750_e109820_d_n9, assign72750_e109820_d_n10, assign72750_e109820_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72750_e109814: f64 = (locals.var_wdld0 - locals.var_ddriftldc);
        let assign72750_e109817: f64 = (locals.var_ddriftldc * 0.1);
        let assign72750_e109818: f64 = (assign72750_e109814 + assign72750_e109817);
        (assign72750_e109818, ((locals.var_wdld0_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0_dn13 - locals.var_ddriftldc_dn13) + (locals.var_ddriftldc_dn13 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign72750_e109820;
        locals.var_tmf1_dn0 = assign72750_e109820_d_n0;
        locals.var_tmf1_dn2 = assign72750_e109820_d_n2;
        locals.var_tmf1_dn4 = assign72750_e109820_d_n4;
        locals.var_tmf1_dn5 = assign72750_e109820_d_n5;
        locals.var_tmf1_dn6 = assign72750_e109820_d_n6;
        locals.var_tmf1_dn7 = assign72750_e109820_d_n7;
        locals.var_tmf1_dn8 = assign72750_e109820_d_n8;
        locals.var_tmf1_dn9 = assign72750_e109820_d_n9;
        locals.var_tmf1_dn10 = assign72750_e109820_d_n10;
        locals.var_tmf1_dn13 = assign72750_e109820_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign72760_e109833, assign72760_e109833_d_n0, assign72760_e109833_d_n2, assign72760_e109833_d_n4, assign72760_e109833_d_n5, assign72760_e109833_d_n6, assign72760_e109833_d_n7, assign72760_e109833_d_n8, assign72760_e109833_d_n9, assign72760_e109833_d_n10, assign72760_e109833_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72760_e109831: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign72760_e109831, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign72760_e109833;
        locals.var_x2_dn0 = assign72760_e109833_d_n0;
        locals.var_x2_dn2 = assign72760_e109833_d_n2;
        locals.var_x2_dn4 = assign72760_e109833_d_n4;
        locals.var_x2_dn5 = assign72760_e109833_d_n5;
        locals.var_x2_dn6 = assign72760_e109833_d_n6;
        locals.var_x2_dn7 = assign72760_e109833_d_n7;
        locals.var_x2_dn8 = assign72760_e109833_d_n8;
        locals.var_x2_dn9 = assign72760_e109833_d_n9;
        locals.var_x2_dn10 = assign72760_e109833_d_n10;
        locals.var_x2_dn13 = assign72760_e109833_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign72770_e109850, assign72770_e109850_d_n0, assign72770_e109850_d_n2, assign72770_e109850_d_n4, assign72770_e109850_d_n5, assign72770_e109850_d_n6, assign72770_e109850_d_n7, assign72770_e109850_d_n8, assign72770_e109850_d_n9, assign72770_e109850_d_n10, assign72770_e109850_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72770_e109844: f64 = (locals.var_ddriftldc * 0.1);
        let assign72770_e109847: f64 = (locals.var_ddriftldc * 0.1);
        let assign72770_e109848: f64 = (assign72770_e109844 * assign72770_e109847);
        (assign72770_e109848, (((locals.var_ddriftldc_dn0 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn13 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn13 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign72770_e109850;
        locals.var_xmax2_dn0 = assign72770_e109850_d_n0;
        locals.var_xmax2_dn2 = assign72770_e109850_d_n2;
        locals.var_xmax2_dn4 = assign72770_e109850_d_n4;
        locals.var_xmax2_dn5 = assign72770_e109850_d_n5;
        locals.var_xmax2_dn6 = assign72770_e109850_d_n6;
        locals.var_xmax2_dn7 = assign72770_e109850_d_n7;
        locals.var_xmax2_dn8 = assign72770_e109850_d_n8;
        locals.var_xmax2_dn9 = assign72770_e109850_d_n9;
        locals.var_xmax2_dn10 = assign72770_e109850_d_n10;
        locals.var_xmax2_dn13 = assign72770_e109850_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign72780_e109861, assign72780_e109861_d_n0, assign72780_e109861_d_n2, assign72780_e109861_d_n4, assign72780_e109861_d_n5, assign72780_e109861_d_n6, assign72780_e109861_d_n7, assign72780_e109861_d_n8, assign72780_e109861_d_n9, assign72780_e109861_d_n10, assign72780_e109861_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign72780_e109861;
        locals.var_xp_dn0 = assign72780_e109861_d_n0;
        locals.var_xp_dn2 = assign72780_e109861_d_n2;
        locals.var_xp_dn4 = assign72780_e109861_d_n4;
        locals.var_xp_dn5 = assign72780_e109861_d_n5;
        locals.var_xp_dn6 = assign72780_e109861_d_n6;
        locals.var_xp_dn7 = assign72780_e109861_d_n7;
        locals.var_xp_dn8 = assign72780_e109861_d_n8;
        locals.var_xp_dn9 = assign72780_e109861_d_n9;
        locals.var_xp_dn10 = assign72780_e109861_d_n10;
        locals.var_xp_dn13 = assign72780_e109861_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign72790_e109872, assign72790_e109872_d_n0, assign72790_e109872_d_n2, assign72790_e109872_d_n4, assign72790_e109872_d_n5, assign72790_e109872_d_n6, assign72790_e109872_d_n7, assign72790_e109872_d_n8, assign72790_e109872_d_n9, assign72790_e109872_d_n10, assign72790_e109872_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign72790_e109872;
        locals.var_xmp_dn0 = assign72790_e109872_d_n0;
        locals.var_xmp_dn2 = assign72790_e109872_d_n2;
        locals.var_xmp_dn4 = assign72790_e109872_d_n4;
        locals.var_xmp_dn5 = assign72790_e109872_d_n5;
        locals.var_xmp_dn6 = assign72790_e109872_d_n6;
        locals.var_xmp_dn7 = assign72790_e109872_d_n7;
        locals.var_xmp_dn8 = assign72790_e109872_d_n8;
        locals.var_xmp_dn9 = assign72790_e109872_d_n9;
        locals.var_xmp_dn10 = assign72790_e109872_d_n10;
        locals.var_xmp_dn13 = assign72790_e109872_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign72800_e109883,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign72800_e109883;
        locals.var_m0_rv = 0.0;

        let (assign72810_e109894,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72810_e109894;
        locals.var_mm_rv = 0.0;

        let (assign72820_e109905, assign72820_e109905_d_n0, assign72820_e109905_d_n2, assign72820_e109905_d_n4, assign72820_e109905_d_n5, assign72820_e109905_d_n6, assign72820_e109905_d_n7, assign72820_e109905_d_n8, assign72820_e109905_d_n9, assign72820_e109905_d_n10, assign72820_e109905_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign72820_e109905;
        locals.var_arg_dn0 = assign72820_e109905_d_n0;
        locals.var_arg_dn2 = assign72820_e109905_d_n2;
        locals.var_arg_dn4 = assign72820_e109905_d_n4;
        locals.var_arg_dn5 = assign72820_e109905_d_n5;
        locals.var_arg_dn6 = assign72820_e109905_d_n6;
        locals.var_arg_dn7 = assign72820_e109905_d_n7;
        locals.var_arg_dn8 = assign72820_e109905_d_n8;
        locals.var_arg_dn9 = assign72820_e109905_d_n9;
        locals.var_arg_dn10 = assign72820_e109905_d_n10;
        locals.var_arg_dn13 = assign72820_e109905_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign72830_e109916, assign72830_e109916_d_n0, assign72830_e109916_d_n2, assign72830_e109916_d_n4, assign72830_e109916_d_n5, assign72830_e109916_d_n6, assign72830_e109916_d_n7, assign72830_e109916_d_n8, assign72830_e109916_d_n9, assign72830_e109916_d_n10, assign72830_e109916_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign72830_e109916;
        locals.var_dnm_dn0 = assign72830_e109916_d_n0;
        locals.var_dnm_dn2 = assign72830_e109916_d_n2;
        locals.var_dnm_dn4 = assign72830_e109916_d_n4;
        locals.var_dnm_dn5 = assign72830_e109916_d_n5;
        locals.var_dnm_dn6 = assign72830_e109916_d_n6;
        locals.var_dnm_dn7 = assign72830_e109916_d_n7;
        locals.var_dnm_dn8 = assign72830_e109916_d_n8;
        locals.var_dnm_dn9 = assign72830_e109916_d_n9;
        locals.var_dnm_dn10 = assign72830_e109916_d_n10;
        locals.var_dnm_dn13 = assign72830_e109916_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign72840_e109929, assign72840_e109929_d_n0, assign72840_e109929_d_n2, assign72840_e109929_d_n4, assign72840_e109929_d_n5, assign72840_e109929_d_n6, assign72840_e109929_d_n7, assign72840_e109929_d_n8, assign72840_e109929_d_n9, assign72840_e109929_d_n10, assign72840_e109929_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72840_e109927: f64 = (locals.var_xp * locals.var_x2);
        (assign72840_e109927, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign72840_e109929;
        locals.var_xp_dn0 = assign72840_e109929_d_n0;
        locals.var_xp_dn2 = assign72840_e109929_d_n2;
        locals.var_xp_dn4 = assign72840_e109929_d_n4;
        locals.var_xp_dn5 = assign72840_e109929_d_n5;
        locals.var_xp_dn6 = assign72840_e109929_d_n6;
        locals.var_xp_dn7 = assign72840_e109929_d_n7;
        locals.var_xp_dn8 = assign72840_e109929_d_n8;
        locals.var_xp_dn9 = assign72840_e109929_d_n9;
        locals.var_xp_dn10 = assign72840_e109929_d_n10;
        locals.var_xp_dn13 = assign72840_e109929_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign72850_e109942, assign72850_e109942_d_n0, assign72850_e109942_d_n2, assign72850_e109942_d_n4, assign72850_e109942_d_n5, assign72850_e109942_d_n6, assign72850_e109942_d_n7, assign72850_e109942_d_n8, assign72850_e109942_d_n9, assign72850_e109942_d_n10, assign72850_e109942_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72850_e109940: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign72850_e109940, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign72850_e109942;
        locals.var_xmp_dn0 = assign72850_e109942_d_n0;
        locals.var_xmp_dn2 = assign72850_e109942_d_n2;
        locals.var_xmp_dn4 = assign72850_e109942_d_n4;
        locals.var_xmp_dn5 = assign72850_e109942_d_n5;
        locals.var_xmp_dn6 = assign72850_e109942_d_n6;
        locals.var_xmp_dn7 = assign72850_e109942_d_n7;
        locals.var_xmp_dn8 = assign72850_e109942_d_n8;
        locals.var_xmp_dn9 = assign72850_e109942_d_n9;
        locals.var_xmp_dn10 = assign72850_e109942_d_n10;
        locals.var_xmp_dn13 = assign72850_e109942_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign72860_e109955, assign72860_e109955_d_n0, assign72860_e109955_d_n2, assign72860_e109955_d_n4, assign72860_e109955_d_n5, assign72860_e109955_d_n6, assign72860_e109955_d_n7, assign72860_e109955_d_n8, assign72860_e109955_d_n9, assign72860_e109955_d_n10, assign72860_e109955_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72860_e109953: f64 = (locals.var_xp * locals.var_x2);
        (assign72860_e109953, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign72860_e109955;
        locals.var_xp_dn0 = assign72860_e109955_d_n0;
        locals.var_xp_dn2 = assign72860_e109955_d_n2;
        locals.var_xp_dn4 = assign72860_e109955_d_n4;
        locals.var_xp_dn5 = assign72860_e109955_d_n5;
        locals.var_xp_dn6 = assign72860_e109955_d_n6;
        locals.var_xp_dn7 = assign72860_e109955_d_n7;
        locals.var_xp_dn8 = assign72860_e109955_d_n8;
        locals.var_xp_dn9 = assign72860_e109955_d_n9;
        locals.var_xp_dn10 = assign72860_e109955_d_n10;
        locals.var_xp_dn13 = assign72860_e109955_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign72870_e109968, assign72870_e109968_d_n0, assign72870_e109968_d_n2, assign72870_e109968_d_n4, assign72870_e109968_d_n5, assign72870_e109968_d_n6, assign72870_e109968_d_n7, assign72870_e109968_d_n8, assign72870_e109968_d_n9, assign72870_e109968_d_n10, assign72870_e109968_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72870_e109966: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign72870_e109966, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign72870_e109968;
        locals.var_xmp_dn0 = assign72870_e109968_d_n0;
        locals.var_xmp_dn2 = assign72870_e109968_d_n2;
        locals.var_xmp_dn4 = assign72870_e109968_d_n4;
        locals.var_xmp_dn5 = assign72870_e109968_d_n5;
        locals.var_xmp_dn6 = assign72870_e109968_d_n6;
        locals.var_xmp_dn7 = assign72870_e109968_d_n7;
        locals.var_xmp_dn8 = assign72870_e109968_d_n8;
        locals.var_xmp_dn9 = assign72870_e109968_d_n9;
        locals.var_xmp_dn10 = assign72870_e109968_d_n10;
        locals.var_xmp_dn13 = assign72870_e109968_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign72880_e109981, assign72880_e109981_d_n0, assign72880_e109981_d_n2, assign72880_e109981_d_n4, assign72880_e109981_d_n5, assign72880_e109981_d_n6, assign72880_e109981_d_n7, assign72880_e109981_d_n8, assign72880_e109981_d_n9, assign72880_e109981_d_n10, assign72880_e109981_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72880_e109979: f64 = (locals.var_xp + locals.var_xmp);
        (assign72880_e109979, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign72880_e109981;
        locals.var_arg_dn0 = assign72880_e109981_d_n0;
        locals.var_arg_dn2 = assign72880_e109981_d_n2;
        locals.var_arg_dn4 = assign72880_e109981_d_n4;
        locals.var_arg_dn5 = assign72880_e109981_d_n5;
        locals.var_arg_dn6 = assign72880_e109981_d_n6;
        locals.var_arg_dn7 = assign72880_e109981_d_n7;
        locals.var_arg_dn8 = assign72880_e109981_d_n8;
        locals.var_arg_dn9 = assign72880_e109981_d_n9;
        locals.var_arg_dn10 = assign72880_e109981_d_n10;
        locals.var_arg_dn13 = assign72880_e109981_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign72890_e109992, assign72890_e109992_d_n0, assign72890_e109992_d_n2, assign72890_e109992_d_n4, assign72890_e109992_d_n5, assign72890_e109992_d_n6, assign72890_e109992_d_n7, assign72890_e109992_d_n8, assign72890_e109992_d_n9, assign72890_e109992_d_n10, assign72890_e109992_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign72890_e109992;
        locals.var_dnm_dn0 = assign72890_e109992_d_n0;
        locals.var_dnm_dn2 = assign72890_e109992_d_n2;
        locals.var_dnm_dn4 = assign72890_e109992_d_n4;
        locals.var_dnm_dn5 = assign72890_e109992_d_n5;
        locals.var_dnm_dn6 = assign72890_e109992_d_n6;
        locals.var_dnm_dn7 = assign72890_e109992_d_n7;
        locals.var_dnm_dn8 = assign72890_e109992_d_n8;
        locals.var_dnm_dn9 = assign72890_e109992_d_n9;
        locals.var_dnm_dn10 = assign72890_e109992_d_n10;
        locals.var_dnm_dn13 = assign72890_e109992_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign72900_e110007: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1689 = assign72900_e110007;
        locals.var_guard1689_rv = 0.0;

        let assign72910_e110010: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1690 = assign72910_e110010;
        locals.var_guard1690_rv = 0.0;

        let (assign72920_e110025,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72920_e110025;
        locals.var_mm_rv = 0.0;

        let assign72930_e110028: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1691 = assign72930_e110028;
        locals.var_guard1691_rv = 0.0;

        let (assign72940_e110046,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1691 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72940_e110046;
        locals.var_mm_rv = 0.0;

        let assign72950_e110049: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1692 = assign72950_e110049;
        locals.var_guard1692_rv = 0.0;

        let (assign72960_e110070,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1691 == 0.0)) && (locals.var_guard1692 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72960_e110070;
        locals.var_mm_rv = 0.0;

        let assign72970_e110073: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1693 = assign72970_e110073;
        locals.var_guard1693_rv = 0.0;

        let (assign72980_e110097,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1691 == 0.0)) && (locals.var_guard1692 == 0.0)) && (locals.var_guard1693 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72980_e110097;
        locals.var_mm_rv = 0.0;

        let (assign72990_e110110,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign72990_e110110;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_263(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign73000_loop_guard: usize = 0;
        while {
            let assign73000_cond_e110124: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign73000_cond_e110124 != 0.0
        } {
            assign73000_loop_guard += 1;
            assert!(assign73000_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign73000_body0_e110138, assign73000_body0_e110138_d_n0, assign73000_body0_e110138_d_n2, assign73000_body0_e110138_d_n4, assign73000_body0_e110138_d_n5, assign73000_body0_e110138_d_n6, assign73000_body0_e110138_d_n7, assign73000_body0_e110138_d_n8, assign73000_body0_e110138_d_n9, assign73000_body0_e110138_d_n10, assign73000_body0_e110138_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) {
        let assign73000_body0_e110136: f64 = (locals.var_dnm).sqrt();
        (assign73000_body0_e110136, (locals.var_dnm_dn0 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn2 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn4 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn5 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn6 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn7 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn8 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn9 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn10 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn13 / (2.0 * assign73000_body0_e110136)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign73000_body0_e110138;
            locals.var_dnm_dn0 = assign73000_body0_e110138_d_n0;
            locals.var_dnm_dn2 = assign73000_body0_e110138_d_n2;
            locals.var_dnm_dn4 = assign73000_body0_e110138_d_n4;
            locals.var_dnm_dn5 = assign73000_body0_e110138_d_n5;
            locals.var_dnm_dn6 = assign73000_body0_e110138_d_n6;
            locals.var_dnm_dn7 = assign73000_body0_e110138_d_n7;
            locals.var_dnm_dn8 = assign73000_body0_e110138_d_n8;
            locals.var_dnm_dn9 = assign73000_body0_e110138_d_n9;
            locals.var_dnm_dn10 = assign73000_body0_e110138_d_n10;
            locals.var_dnm_dn13 = assign73000_body0_e110138_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign73000_body1_e110153,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) {
        let assign73000_body1_e110151: f64 = (locals.var_m0 + 1.0);
        (assign73000_body1_e110151,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign73000_body1_e110153;
            locals.var_m0_rv = 0.0;
        }

        let (assign73010_e110178, assign73010_e110178_d_n0, assign73010_e110178_d_n2, assign73010_e110178_d_n4, assign73010_e110178_d_n5, assign73010_e110178_d_n6, assign73010_e110178_d_n7, assign73010_e110178_d_n8, assign73010_e110178_d_n9, assign73010_e110178_d_n10, assign73010_e110178_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 == 0.0)) {
        let (assign73010_e110176, assign73010_e110176_d_n0, assign73010_e110176_d_n2, assign73010_e110176_d_n4, assign73010_e110176_d_n5, assign73010_e110176_d_n6, assign73010_e110176_d_n7, assign73010_e110176_d_n8, assign73010_e110176_d_n9, assign73010_e110176_d_n10, assign73010_e110176_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign73010_e110173: f64 = (2.0 * 2.0);
                let assign73010_e110174: f64 = (1.0 / assign73010_e110173);
                let assign73010_e110175: f64 = (locals.var_dnm).powf(assign73010_e110174);
                (assign73010_e110175, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn0)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn2)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn4)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn5)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn6)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn7)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn8)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn9)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn10)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn13)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign73010_e110176, assign73010_e110176_d_n0, assign73010_e110176_d_n2, assign73010_e110176_d_n4, assign73010_e110176_d_n5, assign73010_e110176_d_n6, assign73010_e110176_d_n7, assign73010_e110176_d_n8, assign73010_e110176_d_n9, assign73010_e110176_d_n10, assign73010_e110176_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73010_e110178;
        locals.var_dnm_dn0 = assign73010_e110178_d_n0;
        locals.var_dnm_dn2 = assign73010_e110178_d_n2;
        locals.var_dnm_dn4 = assign73010_e110178_d_n4;
        locals.var_dnm_dn5 = assign73010_e110178_d_n5;
        locals.var_dnm_dn6 = assign73010_e110178_d_n6;
        locals.var_dnm_dn7 = assign73010_e110178_d_n7;
        locals.var_dnm_dn8 = assign73010_e110178_d_n8;
        locals.var_dnm_dn9 = assign73010_e110178_d_n9;
        locals.var_dnm_dn10 = assign73010_e110178_d_n10;
        locals.var_dnm_dn13 = assign73010_e110178_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign73020_e110191, assign73020_e110191_d_n0, assign73020_e110191_d_n2, assign73020_e110191_d_n4, assign73020_e110191_d_n5, assign73020_e110191_d_n6, assign73020_e110191_d_n7, assign73020_e110191_d_n8, assign73020_e110191_d_n9, assign73020_e110191_d_n10, assign73020_e110191_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign73020_e110189: f64 = (1.0 / locals.var_dnm);
        (assign73020_e110189, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73020_e110191;
        locals.var_dnm_dn0 = assign73020_e110191_d_n0;
        locals.var_dnm_dn2 = assign73020_e110191_d_n2;
        locals.var_dnm_dn4 = assign73020_e110191_d_n4;
        locals.var_dnm_dn5 = assign73020_e110191_d_n5;
        locals.var_dnm_dn6 = assign73020_e110191_d_n6;
        locals.var_dnm_dn7 = assign73020_e110191_d_n7;
        locals.var_dnm_dn8 = assign73020_e110191_d_n8;
        locals.var_dnm_dn9 = assign73020_e110191_d_n9;
        locals.var_dnm_dn10 = assign73020_e110191_d_n10;
        locals.var_dnm_dn13 = assign73020_e110191_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign73030_e110208, assign73030_e110208_d_n0, assign73030_e110208_d_n2, assign73030_e110208_d_n4, assign73030_e110208_d_n5, assign73030_e110208_d_n6, assign73030_e110208_d_n7, assign73030_e110208_d_n8, assign73030_e110208_d_n9, assign73030_e110208_d_n10, assign73030_e110208_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign73030_e110203: f64 = (locals.var_ddriftldc * 0.1);
        let assign73030_e110204: f64 = (locals.var_tmf1 * assign73030_e110203);
        let assign73030_e110206: f64 = (assign73030_e110204 * locals.var_dnm);
        (assign73030_e110206, ((((locals.var_tmf1_dn0 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn13 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign73030_e110208;
        locals.var_tmf0_dn0 = assign73030_e110208_d_n0;
        locals.var_tmf0_dn2 = assign73030_e110208_d_n2;
        locals.var_tmf0_dn4 = assign73030_e110208_d_n4;
        locals.var_tmf0_dn5 = assign73030_e110208_d_n5;
        locals.var_tmf0_dn6 = assign73030_e110208_d_n6;
        locals.var_tmf0_dn7 = assign73030_e110208_d_n7;
        locals.var_tmf0_dn8 = assign73030_e110208_d_n8;
        locals.var_tmf0_dn9 = assign73030_e110208_d_n9;
        locals.var_tmf0_dn10 = assign73030_e110208_d_n10;
        locals.var_tmf0_dn13 = assign73030_e110208_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign73040_e110227, assign73040_e110227_d_n0, assign73040_e110227_d_n2, assign73040_e110227_d_n4, assign73040_e110227_d_n5, assign73040_e110227_d_n6, assign73040_e110227_d_n7, assign73040_e110227_d_n8, assign73040_e110227_d_n9, assign73040_e110227_d_n10, assign73040_e110227_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign73040_e110219: f64 = (locals.var_ddriftldc * 0.1);
        let assign73040_e110221: f64 = (assign73040_e110219 * locals.var_xmp);
        let assign73040_e110223: f64 = (assign73040_e110221 * locals.var_dnm);
        let assign73040_e110225: f64 = (assign73040_e110223 / locals.var_arg);
        (assign73040_e110225, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn0)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn2)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn4)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn5)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn6)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn7)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn8)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn9)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn10)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn13 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn13)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73040_e110227;
        locals.var_t0_dn0 = assign73040_e110227_d_n0;
        locals.var_t0_dn2 = assign73040_e110227_d_n2;
        locals.var_t0_dn4 = assign73040_e110227_d_n4;
        locals.var_t0_dn5 = assign73040_e110227_d_n5;
        locals.var_t0_dn6 = assign73040_e110227_d_n6;
        locals.var_t0_dn7 = assign73040_e110227_d_n7;
        locals.var_t0_dn8 = assign73040_e110227_d_n8;
        locals.var_t0_dn9 = assign73040_e110227_d_n9;
        locals.var_t0_dn10 = assign73040_e110227_d_n10;
        locals.var_t0_dn13 = assign73040_e110227_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73050_e110244, assign73050_e110244_d_n0, assign73050_e110244_d_n2, assign73050_e110244_d_n4, assign73050_e110244_d_n5, assign73050_e110244_d_n6, assign73050_e110244_d_n7, assign73050_e110244_d_n8, assign73050_e110244_d_n9, assign73050_e110244_d_n10, assign73050_e110244_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign73050_e110239: f64 = (locals.var_ddriftldc * 0.1);
        let assign73050_e110240: f64 = (locals.var_ddriftldc - assign73050_e110239);
        let assign73050_e110242: f64 = (assign73050_e110240 + locals.var_tmf0);
        (assign73050_e110242, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn13 - (locals.var_ddriftldc_dn13 * 0.1)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign73050_e110244;
        locals.var_t1_dn0 = assign73050_e110244_d_n0;
        locals.var_t1_dn2 = assign73050_e110244_d_n2;
        locals.var_t1_dn4 = assign73050_e110244_d_n4;
        locals.var_t1_dn5 = assign73050_e110244_d_n5;
        locals.var_t1_dn6 = assign73050_e110244_d_n6;
        locals.var_t1_dn7 = assign73050_e110244_d_n7;
        locals.var_t1_dn8 = assign73050_e110244_d_n8;
        locals.var_t1_dn9 = assign73050_e110244_d_n9;
        locals.var_t1_dn10 = assign73050_e110244_d_n10;
        locals.var_t1_dn13 = assign73050_e110244_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign73060_e110255, assign73060_e110255_d_n0, assign73060_e110255_d_n2, assign73060_e110255_d_n4, assign73060_e110255_d_n5, assign73060_e110255_d_n6, assign73060_e110255_d_n7, assign73060_e110255_d_n8, assign73060_e110255_d_n9, assign73060_e110255_d_n10, assign73060_e110255_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73060_e110255;
        locals.var_t0_dn0 = assign73060_e110255_d_n0;
        locals.var_t0_dn2 = assign73060_e110255_d_n2;
        locals.var_t0_dn4 = assign73060_e110255_d_n4;
        locals.var_t0_dn5 = assign73060_e110255_d_n5;
        locals.var_t0_dn6 = assign73060_e110255_d_n6;
        locals.var_t0_dn7 = assign73060_e110255_d_n7;
        locals.var_t0_dn8 = assign73060_e110255_d_n8;
        locals.var_t0_dn9 = assign73060_e110255_d_n9;
        locals.var_t0_dn10 = assign73060_e110255_d_n10;
        locals.var_t0_dn13 = assign73060_e110255_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73070_e110267, assign73070_e110267_d_n0, assign73070_e110267_d_n2, assign73070_e110267_d_n4, assign73070_e110267_d_n5, assign73070_e110267_d_n6, assign73070_e110267_d_n7, assign73070_e110267_d_n8, assign73070_e110267_d_n9, assign73070_e110267_d_n10, assign73070_e110267_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 == 0.0)) {
        (locals.var_wdld0, locals.var_wdld0_dn0, locals.var_wdld0_dn2, locals.var_wdld0_dn4, locals.var_wdld0_dn5, locals.var_wdld0_dn6, locals.var_wdld0_dn7, locals.var_wdld0_dn8, locals.var_wdld0_dn9, locals.var_wdld0_dn10, locals.var_wdld0_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign73070_e110267;
        locals.var_t1_dn0 = assign73070_e110267_d_n0;
        locals.var_t1_dn2 = assign73070_e110267_d_n2;
        locals.var_t1_dn4 = assign73070_e110267_d_n4;
        locals.var_t1_dn5 = assign73070_e110267_d_n5;
        locals.var_t1_dn6 = assign73070_e110267_d_n6;
        locals.var_t1_dn7 = assign73070_e110267_d_n7;
        locals.var_t1_dn8 = assign73070_e110267_d_n8;
        locals.var_t1_dn9 = assign73070_e110267_d_n9;
        locals.var_t1_dn10 = assign73070_e110267_d_n10;
        locals.var_t1_dn13 = assign73070_e110267_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign73080_e110279, assign73080_e110279_d_n0, assign73080_e110279_d_n2, assign73080_e110279_d_n4, assign73080_e110279_d_n5, assign73080_e110279_d_n6, assign73080_e110279_d_n7, assign73080_e110279_d_n8, assign73080_e110279_d_n9, assign73080_e110279_d_n10, assign73080_e110279_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73080_e110279;
        locals.var_t0_dn0 = assign73080_e110279_d_n0;
        locals.var_t0_dn2 = assign73080_e110279_d_n2;
        locals.var_t0_dn4 = assign73080_e110279_d_n4;
        locals.var_t0_dn5 = assign73080_e110279_d_n5;
        locals.var_t0_dn6 = assign73080_e110279_d_n6;
        locals.var_t0_dn7 = assign73080_e110279_d_n7;
        locals.var_t0_dn8 = assign73080_e110279_d_n8;
        locals.var_t0_dn9 = assign73080_e110279_d_n9;
        locals.var_t0_dn10 = assign73080_e110279_d_n10;
        locals.var_t0_dn13 = assign73080_e110279_d_n13;
        locals.var_t0_rv = 0.0;

        let assign73090_e110282: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1694 = assign73090_e110282;
        locals.var_guard1694_rv = 0.0;

        let (assign73100_e110295,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1694 != 0.0)) {
        let assign73100_e110293: f64 = (locals.var_flg_fd_mode + 2.0);
        (assign73100_e110293,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign73100_e110295;
        locals.var_flg_fd_mode_rv = 0.0;

        let (assign73110_e110310, assign73110_e110310_d_n0, assign73110_e110310_d_n2, assign73110_e110310_d_n4, assign73110_e110310_d_n5, assign73110_e110310_d_n6, assign73110_e110310_d_n7, assign73110_e110310_d_n8, assign73110_e110310_d_n9, assign73110_e110310_d_n10, assign73110_e110310_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 == 0.0)) {
        let (assign73110_e110308, assign73110_e110308_d_n0, assign73110_e110308_d_n2, assign73110_e110308_d_n4, assign73110_e110308_d_n5, assign73110_e110308_d_n6, assign73110_e110308_d_n7, assign73110_e110308_d_n8, assign73110_e110308_d_n9, assign73110_e110308_d_n10, assign73110_e110308_d_n13,) = {
            if (locals.var_wdld0 <= locals.var_ddriftldc) {
                (locals.var_wdld0, locals.var_wdld0_dn0, locals.var_wdld0_dn2, locals.var_wdld0_dn4, locals.var_wdld0_dn5, locals.var_wdld0_dn6, locals.var_wdld0_dn7, locals.var_wdld0_dn8, locals.var_wdld0_dn9, locals.var_wdld0_dn10, locals.var_wdld0_dn13,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
            }
        };
        (assign73110_e110308, assign73110_e110308_d_n0, assign73110_e110308_d_n2, assign73110_e110308_d_n4, assign73110_e110308_d_n5, assign73110_e110308_d_n6, assign73110_e110308_d_n7, assign73110_e110308_d_n8, assign73110_e110308_d_n9, assign73110_e110308_d_n10, assign73110_e110308_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign73110_e110310;
        locals.var_t1_dn0 = assign73110_e110310_d_n0;
        locals.var_t1_dn2 = assign73110_e110310_d_n2;
        locals.var_t1_dn4 = assign73110_e110310_d_n4;
        locals.var_t1_dn5 = assign73110_e110310_d_n5;
        locals.var_t1_dn6 = assign73110_e110310_d_n6;
        locals.var_t1_dn7 = assign73110_e110310_d_n7;
        locals.var_t1_dn8 = assign73110_e110310_d_n8;
        locals.var_t1_dn9 = assign73110_e110310_d_n9;
        locals.var_t1_dn10 = assign73110_e110310_d_n10;
        locals.var_t1_dn13 = assign73110_e110310_d_n13;
        locals.var_t1_rv = 0.0;

        let assign73120_e110313: f64 = if locals.var_wdld0 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard1695 = assign73120_e110313;
        locals.var_guard1695_rv = 0.0;

        let (assign73130_e110327,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 == 0.0)) && (locals.var_guard1695 != 0.0)) {
        let assign73130_e110325: f64 = (locals.var_flg_fd_mode + 2.0);
        (assign73130_e110325,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign73130_e110327;
        locals.var_flg_fd_mode_rv = 0.0;

        let assign73140_e110330: f64 = if locals.var_flg_fd_mode >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1696 = assign73140_e110330;
        locals.var_guard1696_rv = 0.0;

        let (assign73150_e110339, assign73150_e110339_d_n0, assign73150_e110339_d_n2, assign73150_e110339_d_n4, assign73150_e110339_d_n5, assign73150_e110339_d_n6, assign73150_e110339_d_n7, assign73150_e110339_d_n8, assign73150_e110339_d_n9, assign73150_e110339_d_n10, assign73150_e110339_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_bef1, locals.var_ps0ld_bef1_dn0, locals.var_ps0ld_bef1_dn2, locals.var_ps0ld_bef1_dn4, locals.var_ps0ld_bef1_dn5, locals.var_ps0ld_bef1_dn6, locals.var_ps0ld_bef1_dn7, locals.var_ps0ld_bef1_dn8, locals.var_ps0ld_bef1_dn9, locals.var_ps0ld_bef1_dn10, locals.var_ps0ld_bef1_dn13,)
    }
};
        locals.var_ps0ld_bef1 = assign73150_e110339;
        locals.var_ps0ld_bef1_dn0 = assign73150_e110339_d_n0;
        locals.var_ps0ld_bef1_dn2 = assign73150_e110339_d_n2;
        locals.var_ps0ld_bef1_dn4 = assign73150_e110339_d_n4;
        locals.var_ps0ld_bef1_dn5 = assign73150_e110339_d_n5;
        locals.var_ps0ld_bef1_dn6 = assign73150_e110339_d_n6;
        locals.var_ps0ld_bef1_dn7 = assign73150_e110339_d_n7;
        locals.var_ps0ld_bef1_dn8 = assign73150_e110339_d_n8;
        locals.var_ps0ld_bef1_dn9 = assign73150_e110339_d_n9;
        locals.var_ps0ld_bef1_dn10 = assign73150_e110339_d_n10;
        locals.var_ps0ld_bef1_dn13 = assign73150_e110339_d_n13;
        locals.var_ps0ld_bef1_rv = 0.0;

        let (assign73160_e110350, assign73160_e110350_d_n0, assign73160_e110350_d_n2, assign73160_e110350_d_n4, assign73160_e110350_d_n5, assign73160_e110350_d_n6, assign73160_e110350_d_n7, assign73160_e110350_d_n8, assign73160_e110350_d_n9, assign73160_e110350_d_n10, assign73160_e110350_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) {
        let assign73160_e110348: f64 = (locals.var_t1 * locals.var_q_nsubld);
        (assign73160_e110348, (locals.var_t1_dn0 * locals.var_q_nsubld), (locals.var_t1_dn2 * locals.var_q_nsubld), (locals.var_t1_dn4 * locals.var_q_nsubld), (locals.var_t1_dn5 * locals.var_q_nsubld), (locals.var_t1_dn6 * locals.var_q_nsubld), (locals.var_t1_dn7 * locals.var_q_nsubld), (locals.var_t1_dn8 * locals.var_q_nsubld), (locals.var_t1_dn9 * locals.var_q_nsubld), (locals.var_t1_dn10 * locals.var_q_nsubld), (locals.var_t1_dn13 * locals.var_q_nsubld),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign73160_e110350;
        locals.var_qbuld_dn0 = assign73160_e110350_d_n0;
        locals.var_qbuld_dn2 = assign73160_e110350_d_n2;
        locals.var_qbuld_dn4 = assign73160_e110350_d_n4;
        locals.var_qbuld_dn5 = assign73160_e110350_d_n5;
        locals.var_qbuld_dn6 = assign73160_e110350_d_n6;
        locals.var_qbuld_dn7 = assign73160_e110350_d_n7;
        locals.var_qbuld_dn8 = assign73160_e110350_d_n8;
        locals.var_qbuld_dn9 = assign73160_e110350_d_n9;
        locals.var_qbuld_dn10 = assign73160_e110350_d_n10;
        locals.var_qbuld_dn13 = assign73160_e110350_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign73170_e110363, assign73170_e110363_d_n0, assign73170_e110363_d_n2, assign73170_e110363_d_n4, assign73170_e110363_d_n5, assign73170_e110363_d_n6, assign73170_e110363_d_n7, assign73170_e110363_d_n8, assign73170_e110363_d_n9, assign73170_e110363_d_n10, assign73170_e110363_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) {
        let assign73170_e110360: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign73170_e110361: f64 = (locals.var_vgpld - assign73170_e110360);
        (assign73170_e110361, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (locals.var_vgpld_dn6 - (locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (-(locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn13 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign73170_e110363;
        locals.var_ps0ld_dn0 = assign73170_e110363_d_n0;
        locals.var_ps0ld_dn2 = assign73170_e110363_d_n2;
        locals.var_ps0ld_dn4 = assign73170_e110363_d_n4;
        locals.var_ps0ld_dn5 = assign73170_e110363_d_n5;
        locals.var_ps0ld_dn6 = assign73170_e110363_d_n6;
        locals.var_ps0ld_dn7 = assign73170_e110363_d_n7;
        locals.var_ps0ld_dn8 = assign73170_e110363_d_n8;
        locals.var_ps0ld_dn9 = assign73170_e110363_d_n9;
        locals.var_ps0ld_dn10 = assign73170_e110363_d_n10;
        locals.var_ps0ld_dn13 = assign73170_e110363_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign73180_e110366: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1697 = assign73180_e110366;
        locals.var_guard1697_rv = 0.0;

        let assign73190_e110370: f64 = (locals.var_ps0ld_bef1 - 0.1);
        let assign73190_e110375: f64 = if ((locals.var_ps0ld > assign73190_e110370) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1698 = assign73190_e110375;
        locals.var_guard1698_rv = 0.0;

        let (assign73200_e110392, assign73200_e110392_d_n0, assign73200_e110392_d_n2, assign73200_e110392_d_n4, assign73200_e110392_d_n5, assign73200_e110392_d_n6, assign73200_e110392_d_n7, assign73200_e110392_d_n8, assign73200_e110392_d_n9, assign73200_e110392_d_n10, assign73200_e110392_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73200_e110388: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1);
        let assign73200_e110390: f64 = (assign73200_e110388 + 0.1);
        (assign73200_e110390, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1_dn10), (locals.var_ps0ld_dn13 - locals.var_ps0ld_bef1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign73200_e110392;
        locals.var_tmf1_dn0 = assign73200_e110392_d_n0;
        locals.var_tmf1_dn2 = assign73200_e110392_d_n2;
        locals.var_tmf1_dn4 = assign73200_e110392_d_n4;
        locals.var_tmf1_dn5 = assign73200_e110392_d_n5;
        locals.var_tmf1_dn6 = assign73200_e110392_d_n6;
        locals.var_tmf1_dn7 = assign73200_e110392_d_n7;
        locals.var_tmf1_dn8 = assign73200_e110392_d_n8;
        locals.var_tmf1_dn9 = assign73200_e110392_d_n9;
        locals.var_tmf1_dn10 = assign73200_e110392_d_n10;
        locals.var_tmf1_dn13 = assign73200_e110392_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign73210_e110407, assign73210_e110407_d_n0, assign73210_e110407_d_n2, assign73210_e110407_d_n4, assign73210_e110407_d_n5, assign73210_e110407_d_n6, assign73210_e110407_d_n7, assign73210_e110407_d_n8, assign73210_e110407_d_n9, assign73210_e110407_d_n10, assign73210_e110407_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73210_e110405: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign73210_e110405, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign73210_e110407;
        locals.var_x2_dn0 = assign73210_e110407_d_n0;
        locals.var_x2_dn2 = assign73210_e110407_d_n2;
        locals.var_x2_dn4 = assign73210_e110407_d_n4;
        locals.var_x2_dn5 = assign73210_e110407_d_n5;
        locals.var_x2_dn6 = assign73210_e110407_d_n6;
        locals.var_x2_dn7 = assign73210_e110407_d_n7;
        locals.var_x2_dn8 = assign73210_e110407_d_n8;
        locals.var_x2_dn9 = assign73210_e110407_d_n9;
        locals.var_x2_dn10 = assign73210_e110407_d_n10;
        locals.var_x2_dn13 = assign73210_e110407_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign73220_e110422, assign73220_e110422_d_n0, assign73220_e110422_d_n2, assign73220_e110422_d_n4, assign73220_e110422_d_n5, assign73220_e110422_d_n6, assign73220_e110422_d_n7, assign73220_e110422_d_n8, assign73220_e110422_d_n9, assign73220_e110422_d_n10, assign73220_e110422_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73220_e110420: f64 = (0.1 * 0.1);
        (assign73220_e110420, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign73220_e110422;
        locals.var_xmax2_dn0 = assign73220_e110422_d_n0;
        locals.var_xmax2_dn2 = assign73220_e110422_d_n2;
        locals.var_xmax2_dn4 = assign73220_e110422_d_n4;
        locals.var_xmax2_dn5 = assign73220_e110422_d_n5;
        locals.var_xmax2_dn6 = assign73220_e110422_d_n6;
        locals.var_xmax2_dn7 = assign73220_e110422_d_n7;
        locals.var_xmax2_dn8 = assign73220_e110422_d_n8;
        locals.var_xmax2_dn9 = assign73220_e110422_d_n9;
        locals.var_xmax2_dn10 = assign73220_e110422_d_n10;
        locals.var_xmax2_dn13 = assign73220_e110422_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign73230_e110435, assign73230_e110435_d_n0, assign73230_e110435_d_n2, assign73230_e110435_d_n4, assign73230_e110435_d_n5, assign73230_e110435_d_n6, assign73230_e110435_d_n7, assign73230_e110435_d_n8, assign73230_e110435_d_n9, assign73230_e110435_d_n10, assign73230_e110435_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign73230_e110435;
        locals.var_xp_dn0 = assign73230_e110435_d_n0;
        locals.var_xp_dn2 = assign73230_e110435_d_n2;
        locals.var_xp_dn4 = assign73230_e110435_d_n4;
        locals.var_xp_dn5 = assign73230_e110435_d_n5;
        locals.var_xp_dn6 = assign73230_e110435_d_n6;
        locals.var_xp_dn7 = assign73230_e110435_d_n7;
        locals.var_xp_dn8 = assign73230_e110435_d_n8;
        locals.var_xp_dn9 = assign73230_e110435_d_n9;
        locals.var_xp_dn10 = assign73230_e110435_d_n10;
        locals.var_xp_dn13 = assign73230_e110435_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign73240_e110448, assign73240_e110448_d_n0, assign73240_e110448_d_n2, assign73240_e110448_d_n4, assign73240_e110448_d_n5, assign73240_e110448_d_n6, assign73240_e110448_d_n7, assign73240_e110448_d_n8, assign73240_e110448_d_n9, assign73240_e110448_d_n10, assign73240_e110448_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign73240_e110448;
        locals.var_xmp_dn0 = assign73240_e110448_d_n0;
        locals.var_xmp_dn2 = assign73240_e110448_d_n2;
        locals.var_xmp_dn4 = assign73240_e110448_d_n4;
        locals.var_xmp_dn5 = assign73240_e110448_d_n5;
        locals.var_xmp_dn6 = assign73240_e110448_d_n6;
        locals.var_xmp_dn7 = assign73240_e110448_d_n7;
        locals.var_xmp_dn8 = assign73240_e110448_d_n8;
        locals.var_xmp_dn9 = assign73240_e110448_d_n9;
        locals.var_xmp_dn10 = assign73240_e110448_d_n10;
        locals.var_xmp_dn13 = assign73240_e110448_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign73250_e110461,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign73250_e110461;
        locals.var_m0_rv = 0.0;

        let (assign73260_e110474,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73260_e110474;
        locals.var_mm_rv = 0.0;

        let (assign73270_e110487, assign73270_e110487_d_n0, assign73270_e110487_d_n2, assign73270_e110487_d_n4, assign73270_e110487_d_n5, assign73270_e110487_d_n6, assign73270_e110487_d_n7, assign73270_e110487_d_n8, assign73270_e110487_d_n9, assign73270_e110487_d_n10, assign73270_e110487_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign73270_e110487;
        locals.var_arg_dn0 = assign73270_e110487_d_n0;
        locals.var_arg_dn2 = assign73270_e110487_d_n2;
        locals.var_arg_dn4 = assign73270_e110487_d_n4;
        locals.var_arg_dn5 = assign73270_e110487_d_n5;
        locals.var_arg_dn6 = assign73270_e110487_d_n6;
        locals.var_arg_dn7 = assign73270_e110487_d_n7;
        locals.var_arg_dn8 = assign73270_e110487_d_n8;
        locals.var_arg_dn9 = assign73270_e110487_d_n9;
        locals.var_arg_dn10 = assign73270_e110487_d_n10;
        locals.var_arg_dn13 = assign73270_e110487_d_n13;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_264(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign73280_e110500, assign73280_e110500_d_n0, assign73280_e110500_d_n2, assign73280_e110500_d_n4, assign73280_e110500_d_n5, assign73280_e110500_d_n6, assign73280_e110500_d_n7, assign73280_e110500_d_n8, assign73280_e110500_d_n9, assign73280_e110500_d_n10, assign73280_e110500_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73280_e110500;
        locals.var_dnm_dn0 = assign73280_e110500_d_n0;
        locals.var_dnm_dn2 = assign73280_e110500_d_n2;
        locals.var_dnm_dn4 = assign73280_e110500_d_n4;
        locals.var_dnm_dn5 = assign73280_e110500_d_n5;
        locals.var_dnm_dn6 = assign73280_e110500_d_n6;
        locals.var_dnm_dn7 = assign73280_e110500_d_n7;
        locals.var_dnm_dn8 = assign73280_e110500_d_n8;
        locals.var_dnm_dn9 = assign73280_e110500_d_n9;
        locals.var_dnm_dn10 = assign73280_e110500_d_n10;
        locals.var_dnm_dn13 = assign73280_e110500_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign73290_e110515, assign73290_e110515_d_n0, assign73290_e110515_d_n2, assign73290_e110515_d_n4, assign73290_e110515_d_n5, assign73290_e110515_d_n6, assign73290_e110515_d_n7, assign73290_e110515_d_n8, assign73290_e110515_d_n9, assign73290_e110515_d_n10, assign73290_e110515_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73290_e110513: f64 = (locals.var_xp * locals.var_x2);
        (assign73290_e110513, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign73290_e110515;
        locals.var_xp_dn0 = assign73290_e110515_d_n0;
        locals.var_xp_dn2 = assign73290_e110515_d_n2;
        locals.var_xp_dn4 = assign73290_e110515_d_n4;
        locals.var_xp_dn5 = assign73290_e110515_d_n5;
        locals.var_xp_dn6 = assign73290_e110515_d_n6;
        locals.var_xp_dn7 = assign73290_e110515_d_n7;
        locals.var_xp_dn8 = assign73290_e110515_d_n8;
        locals.var_xp_dn9 = assign73290_e110515_d_n9;
        locals.var_xp_dn10 = assign73290_e110515_d_n10;
        locals.var_xp_dn13 = assign73290_e110515_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign73300_e110530, assign73300_e110530_d_n0, assign73300_e110530_d_n2, assign73300_e110530_d_n4, assign73300_e110530_d_n5, assign73300_e110530_d_n6, assign73300_e110530_d_n7, assign73300_e110530_d_n8, assign73300_e110530_d_n9, assign73300_e110530_d_n10, assign73300_e110530_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73300_e110528: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign73300_e110528, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign73300_e110530;
        locals.var_xmp_dn0 = assign73300_e110530_d_n0;
        locals.var_xmp_dn2 = assign73300_e110530_d_n2;
        locals.var_xmp_dn4 = assign73300_e110530_d_n4;
        locals.var_xmp_dn5 = assign73300_e110530_d_n5;
        locals.var_xmp_dn6 = assign73300_e110530_d_n6;
        locals.var_xmp_dn7 = assign73300_e110530_d_n7;
        locals.var_xmp_dn8 = assign73300_e110530_d_n8;
        locals.var_xmp_dn9 = assign73300_e110530_d_n9;
        locals.var_xmp_dn10 = assign73300_e110530_d_n10;
        locals.var_xmp_dn13 = assign73300_e110530_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign73310_e110545, assign73310_e110545_d_n0, assign73310_e110545_d_n2, assign73310_e110545_d_n4, assign73310_e110545_d_n5, assign73310_e110545_d_n6, assign73310_e110545_d_n7, assign73310_e110545_d_n8, assign73310_e110545_d_n9, assign73310_e110545_d_n10, assign73310_e110545_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73310_e110543: f64 = (locals.var_xp * locals.var_x2);
        (assign73310_e110543, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign73310_e110545;
        locals.var_xp_dn0 = assign73310_e110545_d_n0;
        locals.var_xp_dn2 = assign73310_e110545_d_n2;
        locals.var_xp_dn4 = assign73310_e110545_d_n4;
        locals.var_xp_dn5 = assign73310_e110545_d_n5;
        locals.var_xp_dn6 = assign73310_e110545_d_n6;
        locals.var_xp_dn7 = assign73310_e110545_d_n7;
        locals.var_xp_dn8 = assign73310_e110545_d_n8;
        locals.var_xp_dn9 = assign73310_e110545_d_n9;
        locals.var_xp_dn10 = assign73310_e110545_d_n10;
        locals.var_xp_dn13 = assign73310_e110545_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign73320_e110560, assign73320_e110560_d_n0, assign73320_e110560_d_n2, assign73320_e110560_d_n4, assign73320_e110560_d_n5, assign73320_e110560_d_n6, assign73320_e110560_d_n7, assign73320_e110560_d_n8, assign73320_e110560_d_n9, assign73320_e110560_d_n10, assign73320_e110560_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73320_e110558: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign73320_e110558, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign73320_e110560;
        locals.var_xmp_dn0 = assign73320_e110560_d_n0;
        locals.var_xmp_dn2 = assign73320_e110560_d_n2;
        locals.var_xmp_dn4 = assign73320_e110560_d_n4;
        locals.var_xmp_dn5 = assign73320_e110560_d_n5;
        locals.var_xmp_dn6 = assign73320_e110560_d_n6;
        locals.var_xmp_dn7 = assign73320_e110560_d_n7;
        locals.var_xmp_dn8 = assign73320_e110560_d_n8;
        locals.var_xmp_dn9 = assign73320_e110560_d_n9;
        locals.var_xmp_dn10 = assign73320_e110560_d_n10;
        locals.var_xmp_dn13 = assign73320_e110560_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign73330_e110575, assign73330_e110575_d_n0, assign73330_e110575_d_n2, assign73330_e110575_d_n4, assign73330_e110575_d_n5, assign73330_e110575_d_n6, assign73330_e110575_d_n7, assign73330_e110575_d_n8, assign73330_e110575_d_n9, assign73330_e110575_d_n10, assign73330_e110575_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73330_e110573: f64 = (locals.var_xp + locals.var_xmp);
        (assign73330_e110573, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign73330_e110575;
        locals.var_arg_dn0 = assign73330_e110575_d_n0;
        locals.var_arg_dn2 = assign73330_e110575_d_n2;
        locals.var_arg_dn4 = assign73330_e110575_d_n4;
        locals.var_arg_dn5 = assign73330_e110575_d_n5;
        locals.var_arg_dn6 = assign73330_e110575_d_n6;
        locals.var_arg_dn7 = assign73330_e110575_d_n7;
        locals.var_arg_dn8 = assign73330_e110575_d_n8;
        locals.var_arg_dn9 = assign73330_e110575_d_n9;
        locals.var_arg_dn10 = assign73330_e110575_d_n10;
        locals.var_arg_dn13 = assign73330_e110575_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign73340_e110588, assign73340_e110588_d_n0, assign73340_e110588_d_n2, assign73340_e110588_d_n4, assign73340_e110588_d_n5, assign73340_e110588_d_n6, assign73340_e110588_d_n7, assign73340_e110588_d_n8, assign73340_e110588_d_n9, assign73340_e110588_d_n10, assign73340_e110588_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73340_e110588;
        locals.var_dnm_dn0 = assign73340_e110588_d_n0;
        locals.var_dnm_dn2 = assign73340_e110588_d_n2;
        locals.var_dnm_dn4 = assign73340_e110588_d_n4;
        locals.var_dnm_dn5 = assign73340_e110588_d_n5;
        locals.var_dnm_dn6 = assign73340_e110588_d_n6;
        locals.var_dnm_dn7 = assign73340_e110588_d_n7;
        locals.var_dnm_dn8 = assign73340_e110588_d_n8;
        locals.var_dnm_dn9 = assign73340_e110588_d_n9;
        locals.var_dnm_dn10 = assign73340_e110588_d_n10;
        locals.var_dnm_dn13 = assign73340_e110588_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign73350_e110603: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1699 = assign73350_e110603;
        locals.var_guard1699_rv = 0.0;

        let assign73360_e110606: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1700 = assign73360_e110606;
        locals.var_guard1700_rv = 0.0;

        let (assign73370_e110623,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73370_e110623;
        locals.var_mm_rv = 0.0;

        let assign73380_e110626: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1701 = assign73380_e110626;
        locals.var_guard1701_rv = 0.0;

        let (assign73390_e110646,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 == 0.0)) && (locals.var_guard1701 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73390_e110646;
        locals.var_mm_rv = 0.0;

        let assign73400_e110649: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1702 = assign73400_e110649;
        locals.var_guard1702_rv = 0.0;

        let (assign73410_e110672,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 == 0.0)) && (locals.var_guard1701 == 0.0)) && (locals.var_guard1702 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73410_e110672;
        locals.var_mm_rv = 0.0;

        let assign73420_e110675: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1703 = assign73420_e110675;
        locals.var_guard1703_rv = 0.0;

        let (assign73430_e110701,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 == 0.0)) && (locals.var_guard1701 == 0.0)) && (locals.var_guard1702 == 0.0)) && (locals.var_guard1703 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73430_e110701;
        locals.var_mm_rv = 0.0;

        let (assign73440_e110716,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign73440_e110716;
        locals.var_m0_rv = 0.0;

        let mut assign73450_loop_guard: usize = 0;
        while {
            let assign73450_cond_e110732: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign73450_cond_e110732 != 0.0
        } {
            assign73450_loop_guard += 1;
            assert!(assign73450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign73450_body0_e110748, assign73450_body0_e110748_d_n0, assign73450_body0_e110748_d_n2, assign73450_body0_e110748_d_n4, assign73450_body0_e110748_d_n5, assign73450_body0_e110748_d_n6, assign73450_body0_e110748_d_n7, assign73450_body0_e110748_d_n8, assign73450_body0_e110748_d_n9, assign73450_body0_e110748_d_n10, assign73450_body0_e110748_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) {
        let assign73450_body0_e110746: f64 = (locals.var_dnm).sqrt();
        (assign73450_body0_e110746, (locals.var_dnm_dn0 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn2 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn4 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn5 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn6 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn7 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn8 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn9 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn10 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn13 / (2.0 * assign73450_body0_e110746)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign73450_body0_e110748;
            locals.var_dnm_dn0 = assign73450_body0_e110748_d_n0;
            locals.var_dnm_dn2 = assign73450_body0_e110748_d_n2;
            locals.var_dnm_dn4 = assign73450_body0_e110748_d_n4;
            locals.var_dnm_dn5 = assign73450_body0_e110748_d_n5;
            locals.var_dnm_dn6 = assign73450_body0_e110748_d_n6;
            locals.var_dnm_dn7 = assign73450_body0_e110748_d_n7;
            locals.var_dnm_dn8 = assign73450_body0_e110748_d_n8;
            locals.var_dnm_dn9 = assign73450_body0_e110748_d_n9;
            locals.var_dnm_dn10 = assign73450_body0_e110748_d_n10;
            locals.var_dnm_dn13 = assign73450_body0_e110748_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign73450_body1_e110765,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) {
        let assign73450_body1_e110763: f64 = (locals.var_m0 + 1.0);
        (assign73450_body1_e110763,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign73450_body1_e110765;
            locals.var_m0_rv = 0.0;
        }

        let (assign73460_e110792, assign73460_e110792_d_n0, assign73460_e110792_d_n2, assign73460_e110792_d_n4, assign73460_e110792_d_n5, assign73460_e110792_d_n6, assign73460_e110792_d_n7, assign73460_e110792_d_n8, assign73460_e110792_d_n9, assign73460_e110792_d_n10, assign73460_e110792_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 == 0.0)) {
        let (assign73460_e110790, assign73460_e110790_d_n0, assign73460_e110790_d_n2, assign73460_e110790_d_n4, assign73460_e110790_d_n5, assign73460_e110790_d_n6, assign73460_e110790_d_n7, assign73460_e110790_d_n8, assign73460_e110790_d_n9, assign73460_e110790_d_n10, assign73460_e110790_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign73460_e110787: f64 = (2.0 * 2.0);
                let assign73460_e110788: f64 = (1.0 / assign73460_e110787);
                let assign73460_e110789: f64 = (locals.var_dnm).powf(assign73460_e110788);
                (assign73460_e110789, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn0)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn2)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn4)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn5)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn6)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn7)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn8)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn9)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn10)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn13)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign73460_e110790, assign73460_e110790_d_n0, assign73460_e110790_d_n2, assign73460_e110790_d_n4, assign73460_e110790_d_n5, assign73460_e110790_d_n6, assign73460_e110790_d_n7, assign73460_e110790_d_n8, assign73460_e110790_d_n9, assign73460_e110790_d_n10, assign73460_e110790_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73460_e110792;
        locals.var_dnm_dn0 = assign73460_e110792_d_n0;
        locals.var_dnm_dn2 = assign73460_e110792_d_n2;
        locals.var_dnm_dn4 = assign73460_e110792_d_n4;
        locals.var_dnm_dn5 = assign73460_e110792_d_n5;
        locals.var_dnm_dn6 = assign73460_e110792_d_n6;
        locals.var_dnm_dn7 = assign73460_e110792_d_n7;
        locals.var_dnm_dn8 = assign73460_e110792_d_n8;
        locals.var_dnm_dn9 = assign73460_e110792_d_n9;
        locals.var_dnm_dn10 = assign73460_e110792_d_n10;
        locals.var_dnm_dn13 = assign73460_e110792_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign73470_e110807, assign73470_e110807_d_n0, assign73470_e110807_d_n2, assign73470_e110807_d_n4, assign73470_e110807_d_n5, assign73470_e110807_d_n6, assign73470_e110807_d_n7, assign73470_e110807_d_n8, assign73470_e110807_d_n9, assign73470_e110807_d_n10, assign73470_e110807_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73470_e110805: f64 = (1.0 / locals.var_dnm);
        (assign73470_e110805, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73470_e110807;
        locals.var_dnm_dn0 = assign73470_e110807_d_n0;
        locals.var_dnm_dn2 = assign73470_e110807_d_n2;
        locals.var_dnm_dn4 = assign73470_e110807_d_n4;
        locals.var_dnm_dn5 = assign73470_e110807_d_n5;
        locals.var_dnm_dn6 = assign73470_e110807_d_n6;
        locals.var_dnm_dn7 = assign73470_e110807_d_n7;
        locals.var_dnm_dn8 = assign73470_e110807_d_n8;
        locals.var_dnm_dn9 = assign73470_e110807_d_n9;
        locals.var_dnm_dn10 = assign73470_e110807_d_n10;
        locals.var_dnm_dn13 = assign73470_e110807_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign73480_e110824, assign73480_e110824_d_n0, assign73480_e110824_d_n2, assign73480_e110824_d_n4, assign73480_e110824_d_n5, assign73480_e110824_d_n6, assign73480_e110824_d_n7, assign73480_e110824_d_n8, assign73480_e110824_d_n9, assign73480_e110824_d_n10, assign73480_e110824_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73480_e110820: f64 = (locals.var_tmf1 * 0.1);
        let assign73480_e110822: f64 = (assign73480_e110820 * locals.var_dnm);
        (assign73480_e110822, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign73480_e110824;
        locals.var_tmf0_dn0 = assign73480_e110824_d_n0;
        locals.var_tmf0_dn2 = assign73480_e110824_d_n2;
        locals.var_tmf0_dn4 = assign73480_e110824_d_n4;
        locals.var_tmf0_dn5 = assign73480_e110824_d_n5;
        locals.var_tmf0_dn6 = assign73480_e110824_d_n6;
        locals.var_tmf0_dn7 = assign73480_e110824_d_n7;
        locals.var_tmf0_dn8 = assign73480_e110824_d_n8;
        locals.var_tmf0_dn9 = assign73480_e110824_d_n9;
        locals.var_tmf0_dn10 = assign73480_e110824_d_n10;
        locals.var_tmf0_dn13 = assign73480_e110824_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign73490_e110843, assign73490_e110843_d_n0, assign73490_e110843_d_n2, assign73490_e110843_d_n4, assign73490_e110843_d_n5, assign73490_e110843_d_n6, assign73490_e110843_d_n7, assign73490_e110843_d_n8, assign73490_e110843_d_n9, assign73490_e110843_d_n10, assign73490_e110843_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73490_e110837: f64 = (0.1 * locals.var_xmp);
        let assign73490_e110839: f64 = (assign73490_e110837 * locals.var_dnm);
        let assign73490_e110841: f64 = (assign73490_e110839 / locals.var_arg);
        (assign73490_e110841, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn0)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn2)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn4)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn5)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn6)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn7)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn8)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn9)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn10)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn13)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73490_e110843;
        locals.var_t0_dn0 = assign73490_e110843_d_n0;
        locals.var_t0_dn2 = assign73490_e110843_d_n2;
        locals.var_t0_dn4 = assign73490_e110843_d_n4;
        locals.var_t0_dn5 = assign73490_e110843_d_n5;
        locals.var_t0_dn6 = assign73490_e110843_d_n6;
        locals.var_t0_dn7 = assign73490_e110843_d_n7;
        locals.var_t0_dn8 = assign73490_e110843_d_n8;
        locals.var_t0_dn9 = assign73490_e110843_d_n9;
        locals.var_t0_dn10 = assign73490_e110843_d_n10;
        locals.var_t0_dn13 = assign73490_e110843_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73500_e110860, assign73500_e110860_d_n0, assign73500_e110860_d_n2, assign73500_e110860_d_n4, assign73500_e110860_d_n5, assign73500_e110860_d_n6, assign73500_e110860_d_n7, assign73500_e110860_d_n8, assign73500_e110860_d_n9, assign73500_e110860_d_n10, assign73500_e110860_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73500_e110856: f64 = (locals.var_ps0ld_bef1 - 0.1);
        let assign73500_e110858: f64 = (assign73500_e110856 + locals.var_tmf0);
        (assign73500_e110858, (locals.var_ps0ld_bef1_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign73500_e110860;
        locals.var_ps0ld_dn0 = assign73500_e110860_d_n0;
        locals.var_ps0ld_dn2 = assign73500_e110860_d_n2;
        locals.var_ps0ld_dn4 = assign73500_e110860_d_n4;
        locals.var_ps0ld_dn5 = assign73500_e110860_d_n5;
        locals.var_ps0ld_dn6 = assign73500_e110860_d_n6;
        locals.var_ps0ld_dn7 = assign73500_e110860_d_n7;
        locals.var_ps0ld_dn8 = assign73500_e110860_d_n8;
        locals.var_ps0ld_dn9 = assign73500_e110860_d_n9;
        locals.var_ps0ld_dn10 = assign73500_e110860_d_n10;
        locals.var_ps0ld_dn13 = assign73500_e110860_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign73510_e110873, assign73510_e110873_d_n0, assign73510_e110873_d_n2, assign73510_e110873_d_n4, assign73510_e110873_d_n5, assign73510_e110873_d_n6, assign73510_e110873_d_n7, assign73510_e110873_d_n8, assign73510_e110873_d_n9, assign73510_e110873_d_n10, assign73510_e110873_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73510_e110873;
        locals.var_t0_dn0 = assign73510_e110873_d_n0;
        locals.var_t0_dn2 = assign73510_e110873_d_n2;
        locals.var_t0_dn4 = assign73510_e110873_d_n4;
        locals.var_t0_dn5 = assign73510_e110873_d_n5;
        locals.var_t0_dn6 = assign73510_e110873_d_n6;
        locals.var_t0_dn7 = assign73510_e110873_d_n7;
        locals.var_t0_dn8 = assign73510_e110873_d_n8;
        locals.var_t0_dn9 = assign73510_e110873_d_n9;
        locals.var_t0_dn10 = assign73510_e110873_d_n10;
        locals.var_t0_dn13 = assign73510_e110873_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73520_e110887, assign73520_e110887_d_n0, assign73520_e110887_d_n2, assign73520_e110887_d_n4, assign73520_e110887_d_n5, assign73520_e110887_d_n6, assign73520_e110887_d_n7, assign73520_e110887_d_n8, assign73520_e110887_d_n9, assign73520_e110887_d_n10, assign73520_e110887_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign73520_e110887;
        locals.var_ps0ld_dn0 = assign73520_e110887_d_n0;
        locals.var_ps0ld_dn2 = assign73520_e110887_d_n2;
        locals.var_ps0ld_dn4 = assign73520_e110887_d_n4;
        locals.var_ps0ld_dn5 = assign73520_e110887_d_n5;
        locals.var_ps0ld_dn6 = assign73520_e110887_d_n6;
        locals.var_ps0ld_dn7 = assign73520_e110887_d_n7;
        locals.var_ps0ld_dn8 = assign73520_e110887_d_n8;
        locals.var_ps0ld_dn9 = assign73520_e110887_d_n9;
        locals.var_ps0ld_dn10 = assign73520_e110887_d_n10;
        locals.var_ps0ld_dn13 = assign73520_e110887_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign73530_e110901, assign73530_e110901_d_n0, assign73530_e110901_d_n2, assign73530_e110901_d_n4, assign73530_e110901_d_n5, assign73530_e110901_d_n6, assign73530_e110901_d_n7, assign73530_e110901_d_n8, assign73530_e110901_d_n9, assign73530_e110901_d_n10, assign73530_e110901_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73530_e110901;
        locals.var_t0_dn0 = assign73530_e110901_d_n0;
        locals.var_t0_dn2 = assign73530_e110901_d_n2;
        locals.var_t0_dn4 = assign73530_e110901_d_n4;
        locals.var_t0_dn5 = assign73530_e110901_d_n5;
        locals.var_t0_dn6 = assign73530_e110901_d_n6;
        locals.var_t0_dn7 = assign73530_e110901_d_n7;
        locals.var_t0_dn8 = assign73530_e110901_d_n8;
        locals.var_t0_dn9 = assign73530_e110901_d_n9;
        locals.var_t0_dn10 = assign73530_e110901_d_n10;
        locals.var_t0_dn13 = assign73530_e110901_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73540_e110918, assign73540_e110918_d_n0, assign73540_e110918_d_n2, assign73540_e110918_d_n4, assign73540_e110918_d_n5, assign73540_e110918_d_n6, assign73540_e110918_d_n7, assign73540_e110918_d_n8, assign73540_e110918_d_n9, assign73540_e110918_d_n10, assign73540_e110918_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 == 0.0)) {
        let (assign73540_e110916, assign73540_e110916_d_n0, assign73540_e110916_d_n2, assign73540_e110916_d_n4, assign73540_e110916_d_n5, assign73540_e110916_d_n6, assign73540_e110916_d_n7, assign73540_e110916_d_n8, assign73540_e110916_d_n9, assign73540_e110916_d_n10, assign73540_e110916_d_n13,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
            } else {
                (locals.var_ps0ld_bef1, locals.var_ps0ld_bef1_dn0, locals.var_ps0ld_bef1_dn2, locals.var_ps0ld_bef1_dn4, locals.var_ps0ld_bef1_dn5, locals.var_ps0ld_bef1_dn6, locals.var_ps0ld_bef1_dn7, locals.var_ps0ld_bef1_dn8, locals.var_ps0ld_bef1_dn9, locals.var_ps0ld_bef1_dn10, locals.var_ps0ld_bef1_dn13,)
            }
        };
        (assign73540_e110916, assign73540_e110916_d_n0, assign73540_e110916_d_n2, assign73540_e110916_d_n4, assign73540_e110916_d_n5, assign73540_e110916_d_n6, assign73540_e110916_d_n7, assign73540_e110916_d_n8, assign73540_e110916_d_n9, assign73540_e110916_d_n10, assign73540_e110916_d_n13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign73540_e110918;
        locals.var_ps0ld_dn0 = assign73540_e110918_d_n0;
        locals.var_ps0ld_dn2 = assign73540_e110918_d_n2;
        locals.var_ps0ld_dn4 = assign73540_e110918_d_n4;
        locals.var_ps0ld_dn5 = assign73540_e110918_d_n5;
        locals.var_ps0ld_dn6 = assign73540_e110918_d_n6;
        locals.var_ps0ld_dn7 = assign73540_e110918_d_n7;
        locals.var_ps0ld_dn8 = assign73540_e110918_d_n8;
        locals.var_ps0ld_dn9 = assign73540_e110918_d_n9;
        locals.var_ps0ld_dn10 = assign73540_e110918_d_n10;
        locals.var_ps0ld_dn13 = assign73540_e110918_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign73550_e110925, assign73550_e110925_d_n0, assign73550_e110925_d_n2, assign73550_e110925_d_n4, assign73550_e110925_d_n5, assign73550_e110925_d_n6, assign73550_e110925_d_n7, assign73550_e110925_d_n8, assign73550_e110925_d_n9, assign73550_e110925_d_n10, assign73550_e110925_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn13,)
    }
};
        locals.var_ps0ld_ini = assign73550_e110925;
        locals.var_ps0ld_ini_dn0 = assign73550_e110925_d_n0;
        locals.var_ps0ld_ini_dn2 = assign73550_e110925_d_n2;
        locals.var_ps0ld_ini_dn4 = assign73550_e110925_d_n4;
        locals.var_ps0ld_ini_dn5 = assign73550_e110925_d_n5;
        locals.var_ps0ld_ini_dn6 = assign73550_e110925_d_n6;
        locals.var_ps0ld_ini_dn7 = assign73550_e110925_d_n7;
        locals.var_ps0ld_ini_dn8 = assign73550_e110925_d_n8;
        locals.var_ps0ld_ini_dn9 = assign73550_e110925_d_n9;
        locals.var_ps0ld_ini_dn10 = assign73550_e110925_d_n10;
        locals.var_ps0ld_ini_dn13 = assign73550_e110925_d_n13;
        locals.var_ps0ld_ini_rv = 0.0;

        let assign73560_e110928: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1704 = assign73560_e110928;
        locals.var_guard1704_rv = 0.0;

        let (assign73570_e110937,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign73570_e110937;
        locals.var_flg_conv_rv = 0.0;

    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_297(
        locals: &mut StampLocals,
    ) {
        let (assign80980_e123556, assign80980_e123556_d_n0, assign80980_e123556_d_n2, assign80980_e123556_d_n4, assign80980_e123556_d_n5, assign80980_e123556_d_n6, assign80980_e123556_d_n7, assign80980_e123556_d_n8, assign80980_e123556_d_n9, assign80980_e123556_d_n10, assign80980_e123556_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign80980_e123554: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign80980_e123554, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign80980_e123556;
        locals.var_tmf4_dn0 = assign80980_e123556_d_n0;
        locals.var_tmf4_dn2 = assign80980_e123556_d_n2;
        locals.var_tmf4_dn4 = assign80980_e123556_d_n4;
        locals.var_tmf4_dn5 = assign80980_e123556_d_n5;
        locals.var_tmf4_dn6 = assign80980_e123556_d_n6;
        locals.var_tmf4_dn7 = assign80980_e123556_d_n7;
        locals.var_tmf4_dn8 = assign80980_e123556_d_n8;
        locals.var_tmf4_dn9 = assign80980_e123556_d_n9;
        locals.var_tmf4_dn10 = assign80980_e123556_d_n10;
        locals.var_tmf4_dn13 = assign80980_e123556_d_n13;
        locals.var_tmf4_rv = 0.0;

        let (assign80990_e123574, assign80990_e123574_d_n0, assign80990_e123574_d_n2, assign80990_e123574_d_n4, assign80990_e123574_d_n5, assign80990_e123574_d_n6, assign80990_e123574_d_n7, assign80990_e123574_d_n8, assign80990_e123574_d_n9, assign80990_e123574_d_n10, assign80990_e123574_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign80990_e123565: f64 = (1.0 + locals.var_tmf1);
        let assign80990_e123567: f64 = (assign80990_e123565 + locals.var_tmf2);
        let assign80990_e123569: f64 = (assign80990_e123567 + locals.var_tmf3);
        let assign80990_e123571: f64 = (assign80990_e123569 + locals.var_tmf4);
        let assign80990_e123572: f64 = (1.0 / assign80990_e123571);
        (assign80990_e123572, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) + locals.var_tmf3_dn13) + locals.var_tmf4_dn13) / (assign80990_e123571 * assign80990_e123571))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign80990_e123574;
        locals.var_tmf0_dn0 = assign80990_e123574_d_n0;
        locals.var_tmf0_dn2 = assign80990_e123574_d_n2;
        locals.var_tmf0_dn4 = assign80990_e123574_d_n4;
        locals.var_tmf0_dn5 = assign80990_e123574_d_n5;
        locals.var_tmf0_dn6 = assign80990_e123574_d_n6;
        locals.var_tmf0_dn7 = assign80990_e123574_d_n7;
        locals.var_tmf0_dn8 = assign80990_e123574_d_n8;
        locals.var_tmf0_dn9 = assign80990_e123574_d_n9;
        locals.var_tmf0_dn10 = assign80990_e123574_d_n10;
        locals.var_tmf0_dn13 = assign80990_e123574_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign81000_e123599, assign81000_e123599_d_n0, assign81000_e123599_d_n2, assign81000_e123599_d_n4, assign81000_e123599_d_n5, assign81000_e123599_d_n6, assign81000_e123599_d_n7, assign81000_e123599_d_n8, assign81000_e123599_d_n9, assign81000_e123599_d_n10, assign81000_e123599_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign81000_e123583: f64 = (2.0 * locals.var_tmf1);
        let assign81000_e123584: f64 = (1.0 + assign81000_e123583);
        let assign81000_e123587: f64 = (3.0 * locals.var_tmf2);
        let assign81000_e123588: f64 = (assign81000_e123584 + assign81000_e123587);
        let assign81000_e123591: f64 = (4.0 * locals.var_tmf3);
        let assign81000_e123592: f64 = (assign81000_e123588 + assign81000_e123591);
        let assign81000_e123593: f64 = (-assign81000_e123592);
        let assign81000_e123595: f64 = (assign81000_e123593 * locals.var_tmf0);
        let assign81000_e123597: f64 = (assign81000_e123595 * locals.var_tmf0);
        (assign81000_e123597, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn13) + (3.0 * locals.var_tmf2_dn13)) + (4.0 * locals.var_tmf3_dn13))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn13)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign81000_e123599;
        locals.var_t11_dn0 = assign81000_e123599_d_n0;
        locals.var_t11_dn2 = assign81000_e123599_d_n2;
        locals.var_t11_dn4 = assign81000_e123599_d_n4;
        locals.var_t11_dn5 = assign81000_e123599_d_n5;
        locals.var_t11_dn6 = assign81000_e123599_d_n6;
        locals.var_t11_dn7 = assign81000_e123599_d_n7;
        locals.var_t11_dn8 = assign81000_e123599_d_n8;
        locals.var_t11_dn9 = assign81000_e123599_d_n9;
        locals.var_t11_dn10 = assign81000_e123599_d_n10;
        locals.var_t11_dn13 = assign81000_e123599_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign81010_e123611, assign81010_e123611_d_n0, assign81010_e123611_d_n2, assign81010_e123611_d_n4, assign81010_e123611_d_n5, assign81010_e123611_d_n6, assign81010_e123611_d_n7, assign81010_e123611_d_n8, assign81010_e123611_d_n9, assign81010_e123611_d_n10, assign81010_e123611_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign81010_e123608: f64 = (1.0 - locals.var_tmf0);
        let assign81010_e123609: f64 = (locals.var_t2 * assign81010_e123608);
        (assign81010_e123609, ((locals.var_t2_dn0 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn13 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign81010_e123611;
        locals.var_ty_dn0 = assign81010_e123611_d_n0;
        locals.var_ty_dn2 = assign81010_e123611_d_n2;
        locals.var_ty_dn4 = assign81010_e123611_d_n4;
        locals.var_ty_dn5 = assign81010_e123611_d_n5;
        locals.var_ty_dn6 = assign81010_e123611_d_n6;
        locals.var_ty_dn7 = assign81010_e123611_d_n7;
        locals.var_ty_dn8 = assign81010_e123611_d_n8;
        locals.var_ty_dn9 = assign81010_e123611_d_n9;
        locals.var_ty_dn10 = assign81010_e123611_d_n10;
        locals.var_ty_dn13 = assign81010_e123611_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign81020_e123625, assign81020_e123625_d_n0, assign81020_e123625_d_n2, assign81020_e123625_d_n4, assign81020_e123625_d_n5, assign81020_e123625_d_n6, assign81020_e123625_d_n7, assign81020_e123625_d_n8, assign81020_e123625_d_n9, assign81020_e123625_d_n10, assign81020_e123625_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign81020_e123619: f64 = (1.0 - locals.var_tmf0);
        let assign81020_e123622: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign81020_e123623: f64 = (assign81020_e123619 + assign81020_e123622);
        (assign81020_e123623, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn13) + ((locals.var_tmf1_dn13 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81020_e123625;
        locals.var_t0_dn0 = assign81020_e123625_d_n0;
        locals.var_t0_dn2 = assign81020_e123625_d_n2;
        locals.var_t0_dn4 = assign81020_e123625_d_n4;
        locals.var_t0_dn5 = assign81020_e123625_d_n5;
        locals.var_t0_dn6 = assign81020_e123625_d_n6;
        locals.var_t0_dn7 = assign81020_e123625_d_n7;
        locals.var_t0_dn8 = assign81020_e123625_d_n8;
        locals.var_t0_dn9 = assign81020_e123625_d_n9;
        locals.var_t0_dn10 = assign81020_e123625_d_n10;
        locals.var_t0_dn13 = assign81020_e123625_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign81030_e123634, assign81030_e123634_d_n0, assign81030_e123634_d_n2, assign81030_e123634_d_n4, assign81030_e123634_d_n5, assign81030_e123634_d_n6, assign81030_e123634_d_n7, assign81030_e123634_d_n8, assign81030_e123634_d_n9, assign81030_e123634_d_n10, assign81030_e123634_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign81030_e123632: f64 = (-locals.var_t11);
        (assign81030_e123632, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn13),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign81030_e123634;
        locals.var_t11_dn0 = assign81030_e123634_d_n0;
        locals.var_t11_dn2 = assign81030_e123634_d_n2;
        locals.var_t11_dn4 = assign81030_e123634_d_n4;
        locals.var_t11_dn5 = assign81030_e123634_d_n5;
        locals.var_t11_dn6 = assign81030_e123634_d_n6;
        locals.var_t11_dn7 = assign81030_e123634_d_n7;
        locals.var_t11_dn8 = assign81030_e123634_d_n8;
        locals.var_t11_dn9 = assign81030_e123634_d_n9;
        locals.var_t11_dn10 = assign81030_e123634_d_n10;
        locals.var_t11_dn13 = assign81030_e123634_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign81040_e123644, assign81040_e123644_d_n0, assign81040_e123644_d_n2, assign81040_e123644_d_n4, assign81040_e123644_d_n5, assign81040_e123644_d_n6, assign81040_e123644_d_n7, assign81040_e123644_d_n8, assign81040_e123644_d_n9, assign81040_e123644_d_n10, assign81040_e123644_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign81040_e123642: f64 = (locals.var_vbs_bnd_over__blk1888 + locals.var_ty);
        (assign81040_e123642, (locals.var_vbs_bnd_over__blk1888_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk1888_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk1888_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk1888_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk1888_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk1888_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk1888_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk1888_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk1888_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk1888_dn13 + locals.var_ty_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign81040_e123644;
        locals.var_t10_dn0 = assign81040_e123644_d_n0;
        locals.var_t10_dn2 = assign81040_e123644_d_n2;
        locals.var_t10_dn4 = assign81040_e123644_d_n4;
        locals.var_t10_dn5 = assign81040_e123644_d_n5;
        locals.var_t10_dn6 = assign81040_e123644_d_n6;
        locals.var_t10_dn7 = assign81040_e123644_d_n7;
        locals.var_t10_dn8 = assign81040_e123644_d_n8;
        locals.var_t10_dn9 = assign81040_e123644_d_n9;
        locals.var_t10_dn10 = assign81040_e123644_d_n10;
        locals.var_t10_dn13 = assign81040_e123644_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign81050_e123653, assign81050_e123653_d_n0, assign81050_e123653_d_n2, assign81050_e123653_d_n4, assign81050_e123653_d_n5, assign81050_e123653_d_n6, assign81050_e123653_d_n7, assign81050_e123653_d_n8, assign81050_e123653_d_n9, assign81050_e123653_d_n10, assign81050_e123653_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign81050_e123653;
        locals.var_t10_dn0 = assign81050_e123653_d_n0;
        locals.var_t10_dn2 = assign81050_e123653_d_n2;
        locals.var_t10_dn4 = assign81050_e123653_d_n4;
        locals.var_t10_dn5 = assign81050_e123653_d_n5;
        locals.var_t10_dn6 = assign81050_e123653_d_n6;
        locals.var_t10_dn7 = assign81050_e123653_d_n7;
        locals.var_t10_dn8 = assign81050_e123653_d_n8;
        locals.var_t10_dn9 = assign81050_e123653_d_n9;
        locals.var_t10_dn10 = assign81050_e123653_d_n10;
        locals.var_t10_dn13 = assign81050_e123653_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign81060_e123660, assign81060_e123660_d_n0, assign81060_e123660_d_n2, assign81060_e123660_d_n4, assign81060_e123660_d_n5, assign81060_e123660_d_n6, assign81060_e123660_d_n7, assign81060_e123660_d_n8, assign81060_e123660_d_n9, assign81060_e123660_d_n10, assign81060_e123660_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) {
        let assign81060_e123658: f64 = (-locals.var_t10);
        (assign81060_e123658, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn13),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign81060_e123660;
        locals.var_vxbgmtcl_dn0 = assign81060_e123660_d_n0;
        locals.var_vxbgmtcl_dn2 = assign81060_e123660_d_n2;
        locals.var_vxbgmtcl_dn4 = assign81060_e123660_d_n4;
        locals.var_vxbgmtcl_dn5 = assign81060_e123660_d_n5;
        locals.var_vxbgmtcl_dn6 = assign81060_e123660_d_n6;
        locals.var_vxbgmtcl_dn7 = assign81060_e123660_d_n7;
        locals.var_vxbgmtcl_dn8 = assign81060_e123660_d_n8;
        locals.var_vxbgmtcl_dn9 = assign81060_e123660_d_n9;
        locals.var_vxbgmtcl_dn10 = assign81060_e123660_d_n10;
        locals.var_vxbgmtcl_dn13 = assign81060_e123660_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign81070_e123667, assign81070_e123667_d_n0, assign81070_e123667_d_n2, assign81070_e123667_d_n4, assign81070_e123667_d_n5, assign81070_e123667_d_n6, assign81070_e123667_d_n7, assign81070_e123667_d_n8, assign81070_e123667_d_n9, assign81070_e123667_d_n10, assign81070_e123667_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign81070_e123667;
        locals.var_vxbgmtcl_dn0 = assign81070_e123667_d_n0;
        locals.var_vxbgmtcl_dn2 = assign81070_e123667_d_n2;
        locals.var_vxbgmtcl_dn4 = assign81070_e123667_d_n4;
        locals.var_vxbgmtcl_dn5 = assign81070_e123667_d_n5;
        locals.var_vxbgmtcl_dn6 = assign81070_e123667_d_n6;
        locals.var_vxbgmtcl_dn7 = assign81070_e123667_d_n7;
        locals.var_vxbgmtcl_dn8 = assign81070_e123667_d_n8;
        locals.var_vxbgmtcl_dn9 = assign81070_e123667_d_n9;
        locals.var_vxbgmtcl_dn10 = assign81070_e123667_d_n10;
        locals.var_vxbgmtcl_dn13 = assign81070_e123667_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign81080_e123673, assign81080_e123673_d_n0, assign81080_e123673_d_n2, assign81080_e123673_d_n4, assign81080_e123673_d_n5, assign81080_e123673_d_n6, assign81080_e123673_d_n7, assign81080_e123673_d_n8, assign81080_e123673_d_n9, assign81080_e123673_d_n10, assign81080_e123673_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81080_e123671: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign81080_e123671, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn13 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn13,)
    }
};
        locals.var_fac1 = assign81080_e123673;
        locals.var_fac1_dn0 = assign81080_e123673_d_n0;
        locals.var_fac1_dn2 = assign81080_e123673_d_n2;
        locals.var_fac1_dn4 = assign81080_e123673_d_n4;
        locals.var_fac1_dn5 = assign81080_e123673_d_n5;
        locals.var_fac1_dn6 = assign81080_e123673_d_n6;
        locals.var_fac1_dn7 = assign81080_e123673_d_n7;
        locals.var_fac1_dn8 = assign81080_e123673_d_n8;
        locals.var_fac1_dn9 = assign81080_e123673_d_n9;
        locals.var_fac1_dn10 = assign81080_e123673_d_n10;
        locals.var_fac1_dn13 = assign81080_e123673_d_n13;
        locals.var_fac1_rv = 0.0;

        let (assign81090_e123679, assign81090_e123679_d_n0, assign81090_e123679_d_n2, assign81090_e123679_d_n4, assign81090_e123679_d_n5, assign81090_e123679_d_n6, assign81090_e123679_d_n7, assign81090_e123679_d_n8, assign81090_e123679_d_n9, assign81090_e123679_d_n10, assign81090_e123679_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81090_e123677: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign81090_e123677, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn13 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn13)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn13,)
    }
};
        locals.var_fac1p2 = assign81090_e123679;
        locals.var_fac1p2_dn0 = assign81090_e123679_d_n0;
        locals.var_fac1p2_dn2 = assign81090_e123679_d_n2;
        locals.var_fac1p2_dn4 = assign81090_e123679_d_n4;
        locals.var_fac1p2_dn5 = assign81090_e123679_d_n5;
        locals.var_fac1p2_dn6 = assign81090_e123679_d_n6;
        locals.var_fac1p2_dn7 = assign81090_e123679_d_n7;
        locals.var_fac1p2_dn8 = assign81090_e123679_d_n8;
        locals.var_fac1p2_dn9 = assign81090_e123679_d_n9;
        locals.var_fac1p2_dn10 = assign81090_e123679_d_n10;
        locals.var_fac1p2_dn13 = assign81090_e123679_d_n13;
        locals.var_fac1p2_rv = 0.0;

        let (assign81100_e123686, assign81100_e123686_d_n2, assign81100_e123686_d_n6, assign81100_e123686_d_n7, assign81100_e123686_d_n8,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81100_e123682: f64 = (-locals.var_vgbgmt);
        let assign81100_e123684: f64 = (assign81100_e123682 + locals.var_uc_vfbover);
        (assign81100_e123684, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn6), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn8,)
    }
};
        locals.var_vgpld = assign81100_e123686;
        locals.var_vgpld_dn2 = assign81100_e123686_d_n2;
        locals.var_vgpld_dn6 = assign81100_e123686_d_n6;
        locals.var_vgpld_dn7 = assign81100_e123686_d_n7;
        locals.var_vgpld_dn8 = assign81100_e123686_d_n8;
        locals.var_vgpld_rv = 0.0;

        let (assign81110_e123695, assign81110_e123695_d_n0, assign81110_e123695_d_n2, assign81110_e123695_d_n4, assign81110_e123695_d_n5, assign81110_e123695_d_n6, assign81110_e123695_d_n7, assign81110_e123695_d_n8, assign81110_e123695_d_n9, assign81110_e123695_d_n10, assign81110_e123695_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81110_e123689: f64 = (-locals.var_vxbgmtcl);
        let assign81110_e123692: f64 = (10.0 * 2.220446049250313e-16);
        let assign81110_e123693: f64 = (assign81110_e123689 + assign81110_e123692);
        (assign81110_e123693, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn13,)
    }
};
        locals.var_vgb_fb_ld = assign81110_e123695;
        locals.var_vgb_fb_ld_dn0 = assign81110_e123695_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign81110_e123695_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign81110_e123695_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign81110_e123695_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign81110_e123695_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign81110_e123695_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign81110_e123695_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign81110_e123695_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign81110_e123695_d_n10;
        locals.var_vgb_fb_ld_dn13 = assign81110_e123695_d_n13;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign81120_e123699, assign81120_e123699_d_n0, assign81120_e123699_d_n2, assign81120_e123699_d_n4, assign81120_e123699_d_n5, assign81120_e123699_d_n6, assign81120_e123699_d_n7, assign81120_e123699_d_n8, assign81120_e123699_d_n9, assign81120_e123699_d_n10, assign81120_e123699_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk1882, locals.var_q_dep_ld__blk1882_dn0, locals.var_q_dep_ld__blk1882_dn2, locals.var_q_dep_ld__blk1882_dn4, locals.var_q_dep_ld__blk1882_dn5, locals.var_q_dep_ld__blk1882_dn6, locals.var_q_dep_ld__blk1882_dn7, locals.var_q_dep_ld__blk1882_dn8, locals.var_q_dep_ld__blk1882_dn9, locals.var_q_dep_ld__blk1882_dn10, locals.var_q_dep_ld__blk1882_dn13,)
    }
};
        locals.var_q_dep_ld__blk1882 = assign81120_e123699;
        locals.var_q_dep_ld__blk1882_dn0 = assign81120_e123699_d_n0;
        locals.var_q_dep_ld__blk1882_dn2 = assign81120_e123699_d_n2;
        locals.var_q_dep_ld__blk1882_dn4 = assign81120_e123699_d_n4;
        locals.var_q_dep_ld__blk1882_dn5 = assign81120_e123699_d_n5;
        locals.var_q_dep_ld__blk1882_dn6 = assign81120_e123699_d_n6;
        locals.var_q_dep_ld__blk1882_dn7 = assign81120_e123699_d_n7;
        locals.var_q_dep_ld__blk1882_dn8 = assign81120_e123699_d_n8;
        locals.var_q_dep_ld__blk1882_dn9 = assign81120_e123699_d_n9;
        locals.var_q_dep_ld__blk1882_dn10 = assign81120_e123699_d_n10;
        locals.var_q_dep_ld__blk1882_dn13 = assign81120_e123699_d_n13;
        locals.var_q_dep_ld__blk1882_rv = 0.0;

        let (assign81130_e123705,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81130_e123703: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign81130_e123703,)
    } else {
        (locals.var_q_nsubld__blk1883,)
    }
};
        locals.var_q_nsubld__blk1883 = assign81130_e123705;
        locals.var_q_nsubld__blk1883_rv = 0.0;

        let (assign81140_e123711, assign81140_e123711_d_n0, assign81140_e123711_d_n2, assign81140_e123711_d_n4, assign81140_e123711_d_n5, assign81140_e123711_d_n6, assign81140_e123711_d_n7, assign81140_e123711_d_n8, assign81140_e123711_d_n9, assign81140_e123711_d_n10, assign81140_e123711_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81140_e123709: f64 = (locals.var_nin / locals.var_nover_func);
        (assign81140_e123709, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81140_e123711;
        locals.var_t0_dn0 = assign81140_e123711_d_n0;
        locals.var_t0_dn2 = assign81140_e123711_d_n2;
        locals.var_t0_dn4 = assign81140_e123711_d_n4;
        locals.var_t0_dn5 = assign81140_e123711_d_n5;
        locals.var_t0_dn6 = assign81140_e123711_d_n6;
        locals.var_t0_dn7 = assign81140_e123711_d_n7;
        locals.var_t0_dn8 = assign81140_e123711_d_n8;
        locals.var_t0_dn9 = assign81140_e123711_d_n9;
        locals.var_t0_dn10 = assign81140_e123711_d_n10;
        locals.var_t0_dn13 = assign81140_e123711_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign81150_e123717, assign81150_e123717_d_n0, assign81150_e123717_d_n2, assign81150_e123717_d_n4, assign81150_e123717_d_n5, assign81150_e123717_d_n6, assign81150_e123717_d_n7, assign81150_e123717_d_n8, assign81150_e123717_d_n9, assign81150_e123717_d_n10, assign81150_e123717_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81150_e123715: f64 = (locals.var_t0 * locals.var_t0);
        (assign81150_e123715, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign81150_e123717;
        locals.var_cnst1over_dn0 = assign81150_e123717_d_n0;
        locals.var_cnst1over_dn2 = assign81150_e123717_d_n2;
        locals.var_cnst1over_dn4 = assign81150_e123717_d_n4;
        locals.var_cnst1over_dn5 = assign81150_e123717_d_n5;
        locals.var_cnst1over_dn6 = assign81150_e123717_d_n6;
        locals.var_cnst1over_dn7 = assign81150_e123717_d_n7;
        locals.var_cnst1over_dn8 = assign81150_e123717_d_n8;
        locals.var_cnst1over_dn9 = assign81150_e123717_d_n9;
        locals.var_cnst1over_dn10 = assign81150_e123717_d_n10;
        locals.var_cnst1over_dn13 = assign81150_e123717_d_n13;
        locals.var_cnst1over_rv = 0.0;

        let assign81160_e123720: f64 = (-locals.var_vxbgmtcl);
        let assign81160_e123721: f64 = (locals.var_beta * assign81160_e123720);
        let assign81160_e123723: f64 = if assign81160_e123721 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1900 = assign81160_e123723;
        locals.var_guard1900_rv = 0.0;

        let (assign81170_e123738, assign81170_e123738_d_n0, assign81170_e123738_d_n2, assign81170_e123738_d_n4, assign81170_e123738_d_n5, assign81170_e123738_d_n6, assign81170_e123738_d_n7, assign81170_e123738_d_n8, assign81170_e123738_d_n9, assign81170_e123738_d_n10, assign81170_e123738_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) {
        let assign81170_e123731: f64 = (-locals.var_vxbgmtcl);
        let assign81170_e123732: f64 = (locals.var_beta * assign81170_e123731);
        let assign81170_e123733: f64 = (1.0 + assign81170_e123732);
        let assign81170_e123735: f64 = (assign81170_e123733 - 500.0);
        let assign81170_e123736: f64 = (1.403592217853e217 * assign81170_e123735);
        (assign81170_e123736, (1.403592217853e217 * ((locals.var_beta_dn0 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn13 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign81170_e123738;
        locals.var_exp_bvbs_dn0 = assign81170_e123738_d_n0;
        locals.var_exp_bvbs_dn2 = assign81170_e123738_d_n2;
        locals.var_exp_bvbs_dn4 = assign81170_e123738_d_n4;
        locals.var_exp_bvbs_dn5 = assign81170_e123738_d_n5;
        locals.var_exp_bvbs_dn6 = assign81170_e123738_d_n6;
        locals.var_exp_bvbs_dn7 = assign81170_e123738_d_n7;
        locals.var_exp_bvbs_dn8 = assign81170_e123738_d_n8;
        locals.var_exp_bvbs_dn9 = assign81170_e123738_d_n9;
        locals.var_exp_bvbs_dn10 = assign81170_e123738_d_n10;
        locals.var_exp_bvbs_dn13 = assign81170_e123738_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign81180_e123744, assign81180_e123744_d_n0, assign81180_e123744_d_n2, assign81180_e123744_d_n4, assign81180_e123744_d_n5, assign81180_e123744_d_n6, assign81180_e123744_d_n7, assign81180_e123744_d_n8, assign81180_e123744_d_n9, assign81180_e123744_d_n10, assign81180_e123744_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81180_e123744;
        locals.var_t0_dn0 = assign81180_e123744_d_n0;
        locals.var_t0_dn2 = assign81180_e123744_d_n2;
        locals.var_t0_dn4 = assign81180_e123744_d_n4;
        locals.var_t0_dn5 = assign81180_e123744_d_n5;
        locals.var_t0_dn6 = assign81180_e123744_d_n6;
        locals.var_t0_dn7 = assign81180_e123744_d_n7;
        locals.var_t0_dn8 = assign81180_e123744_d_n8;
        locals.var_t0_dn9 = assign81180_e123744_d_n9;
        locals.var_t0_dn10 = assign81180_e123744_d_n10;
        locals.var_t0_dn13 = assign81180_e123744_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign81190_e123754, assign81190_e123754_d_n0, assign81190_e123754_d_n2, assign81190_e123754_d_n4, assign81190_e123754_d_n5, assign81190_e123754_d_n6, assign81190_e123754_d_n7, assign81190_e123754_d_n8, assign81190_e123754_d_n9, assign81190_e123754_d_n10, assign81190_e123754_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        let assign81190_e123751: f64 = (-locals.var_vxbgmtcl);
        let assign81190_e123752: f64 = (locals.var_beta * assign81190_e123751);
        (assign81190_e123752, ((locals.var_beta_dn0 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign81190_e123754;
        locals.var_tmf1_dn0 = assign81190_e123754_d_n0;
        locals.var_tmf1_dn2 = assign81190_e123754_d_n2;
        locals.var_tmf1_dn4 = assign81190_e123754_d_n4;
        locals.var_tmf1_dn5 = assign81190_e123754_d_n5;
        locals.var_tmf1_dn6 = assign81190_e123754_d_n6;
        locals.var_tmf1_dn7 = assign81190_e123754_d_n7;
        locals.var_tmf1_dn8 = assign81190_e123754_d_n8;
        locals.var_tmf1_dn9 = assign81190_e123754_d_n9;
        locals.var_tmf1_dn10 = assign81190_e123754_d_n10;
        locals.var_tmf1_dn13 = assign81190_e123754_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign81200_e123761, assign81200_e123761_d_n0, assign81200_e123761_d_n2, assign81200_e123761_d_n4, assign81200_e123761_d_n5, assign81200_e123761_d_n6, assign81200_e123761_d_n7, assign81200_e123761_d_n8, assign81200_e123761_d_n9, assign81200_e123761_d_n10, assign81200_e123761_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign81200_e123761;
        locals.var_exp_bvbs_dn0 = assign81200_e123761_d_n0;
        locals.var_exp_bvbs_dn2 = assign81200_e123761_d_n2;
        locals.var_exp_bvbs_dn4 = assign81200_e123761_d_n4;
        locals.var_exp_bvbs_dn5 = assign81200_e123761_d_n5;
        locals.var_exp_bvbs_dn6 = assign81200_e123761_d_n6;
        locals.var_exp_bvbs_dn7 = assign81200_e123761_d_n7;
        locals.var_exp_bvbs_dn8 = assign81200_e123761_d_n8;
        locals.var_exp_bvbs_dn9 = assign81200_e123761_d_n9;
        locals.var_exp_bvbs_dn10 = assign81200_e123761_d_n10;
        locals.var_exp_bvbs_dn13 = assign81200_e123761_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_298(
        locals: &mut StampLocals,
    ) {
        let mut assign81210_loop_guard: usize = 0;
        while {
            let assign81210_cond_e123769: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign81210_cond_e123769 != 0.0
        } {
            assign81210_loop_guard += 1;
            assert!(assign81210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign81210_body0_e123778, assign81210_body0_e123778_d_n0, assign81210_body0_e123778_d_n2, assign81210_body0_e123778_d_n4, assign81210_body0_e123778_d_n5, assign81210_body0_e123778_d_n6, assign81210_body0_e123778_d_n7, assign81210_body0_e123778_d_n8, assign81210_body0_e123778_d_n9, assign81210_body0_e123778_d_n10, assign81210_body0_e123778_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        let assign81210_body0_e123776: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign81210_body0_e123776, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn13 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
            locals.var_exp_bvbs = assign81210_body0_e123778;
            locals.var_exp_bvbs_dn0 = assign81210_body0_e123778_d_n0;
            locals.var_exp_bvbs_dn2 = assign81210_body0_e123778_d_n2;
            locals.var_exp_bvbs_dn4 = assign81210_body0_e123778_d_n4;
            locals.var_exp_bvbs_dn5 = assign81210_body0_e123778_d_n5;
            locals.var_exp_bvbs_dn6 = assign81210_body0_e123778_d_n6;
            locals.var_exp_bvbs_dn7 = assign81210_body0_e123778_d_n7;
            locals.var_exp_bvbs_dn8 = assign81210_body0_e123778_d_n8;
            locals.var_exp_bvbs_dn9 = assign81210_body0_e123778_d_n9;
            locals.var_exp_bvbs_dn10 = assign81210_body0_e123778_d_n10;
            locals.var_exp_bvbs_dn13 = assign81210_body0_e123778_d_n13;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign81210_body1_e123787, assign81210_body1_e123787_d_n0, assign81210_body1_e123787_d_n2, assign81210_body1_e123787_d_n4, assign81210_body1_e123787_d_n5, assign81210_body1_e123787_d_n6, assign81210_body1_e123787_d_n7, assign81210_body1_e123787_d_n8, assign81210_body1_e123787_d_n9, assign81210_body1_e123787_d_n10, assign81210_body1_e123787_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        let assign81210_body1_e123785: f64 = (locals.var_tmf1 - 60.0);
        (assign81210_body1_e123785, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
            locals.var_tmf1 = assign81210_body1_e123787;
            locals.var_tmf1_dn0 = assign81210_body1_e123787_d_n0;
            locals.var_tmf1_dn2 = assign81210_body1_e123787_d_n2;
            locals.var_tmf1_dn4 = assign81210_body1_e123787_d_n4;
            locals.var_tmf1_dn5 = assign81210_body1_e123787_d_n5;
            locals.var_tmf1_dn6 = assign81210_body1_e123787_d_n6;
            locals.var_tmf1_dn7 = assign81210_body1_e123787_d_n7;
            locals.var_tmf1_dn8 = assign81210_body1_e123787_d_n8;
            locals.var_tmf1_dn9 = assign81210_body1_e123787_d_n9;
            locals.var_tmf1_dn10 = assign81210_body1_e123787_d_n10;
            locals.var_tmf1_dn13 = assign81210_body1_e123787_d_n13;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign81220_e123797, assign81220_e123797_d_n0, assign81220_e123797_d_n2, assign81220_e123797_d_n4, assign81220_e123797_d_n5, assign81220_e123797_d_n6, assign81220_e123797_d_n7, assign81220_e123797_d_n8, assign81220_e123797_d_n9, assign81220_e123797_d_n10, assign81220_e123797_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        let assign81220_e123794: f64 = (locals.var_tmf1).exp();
        let assign81220_e123795: f64 = (locals.var_exp_bvbs * assign81220_e123794);
        (assign81220_e123795, ((locals.var_exp_bvbs_dn0 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn13 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn13))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign81220_e123797;
        locals.var_exp_bvbs_dn0 = assign81220_e123797_d_n0;
        locals.var_exp_bvbs_dn2 = assign81220_e123797_d_n2;
        locals.var_exp_bvbs_dn4 = assign81220_e123797_d_n4;
        locals.var_exp_bvbs_dn5 = assign81220_e123797_d_n5;
        locals.var_exp_bvbs_dn6 = assign81220_e123797_d_n6;
        locals.var_exp_bvbs_dn7 = assign81220_e123797_d_n7;
        locals.var_exp_bvbs_dn8 = assign81220_e123797_d_n8;
        locals.var_exp_bvbs_dn9 = assign81220_e123797_d_n9;
        locals.var_exp_bvbs_dn10 = assign81220_e123797_d_n10;
        locals.var_exp_bvbs_dn13 = assign81220_e123797_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign81230_e123804, assign81230_e123804_d_n0, assign81230_e123804_d_n2, assign81230_e123804_d_n4, assign81230_e123804_d_n5, assign81230_e123804_d_n6, assign81230_e123804_d_n7, assign81230_e123804_d_n8, assign81230_e123804_d_n9, assign81230_e123804_d_n10, assign81230_e123804_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81230_e123804;
        locals.var_t0_dn0 = assign81230_e123804_d_n0;
        locals.var_t0_dn2 = assign81230_e123804_d_n2;
        locals.var_t0_dn4 = assign81230_e123804_d_n4;
        locals.var_t0_dn5 = assign81230_e123804_d_n5;
        locals.var_t0_dn6 = assign81230_e123804_d_n6;
        locals.var_t0_dn7 = assign81230_e123804_d_n7;
        locals.var_t0_dn8 = assign81230_e123804_d_n8;
        locals.var_t0_dn9 = assign81230_e123804_d_n9;
        locals.var_t0_dn10 = assign81230_e123804_d_n10;
        locals.var_t0_dn13 = assign81230_e123804_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign81240_e123817, assign81240_e123817_d_n0, assign81240_e123817_d_n2, assign81240_e123817_d_n4, assign81240_e123817_d_n5, assign81240_e123817_d_n6, assign81240_e123817_d_n7, assign81240_e123817_d_n8, assign81240_e123817_d_n9, assign81240_e123817_d_n10, assign81240_e123817_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81240_e123809: f64 = (-locals.var_vgpld);
        let assign81240_e123811: f64 = (assign81240_e123809 * 0.5);
        let assign81240_e123813: f64 = (assign81240_e123811 - 0.5);
        let assign81240_e123815: f64 = (assign81240_e123813 - 1.0);
        (assign81240_e123815, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, ((-locals.var_vgpld_dn6) * 0.5), ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign81240_e123817;
        locals.var_tmf1_dn0 = assign81240_e123817_d_n0;
        locals.var_tmf1_dn2 = assign81240_e123817_d_n2;
        locals.var_tmf1_dn4 = assign81240_e123817_d_n4;
        locals.var_tmf1_dn5 = assign81240_e123817_d_n5;
        locals.var_tmf1_dn6 = assign81240_e123817_d_n6;
        locals.var_tmf1_dn7 = assign81240_e123817_d_n7;
        locals.var_tmf1_dn8 = assign81240_e123817_d_n8;
        locals.var_tmf1_dn9 = assign81240_e123817_d_n9;
        locals.var_tmf1_dn10 = assign81240_e123817_d_n10;
        locals.var_tmf1_dn13 = assign81240_e123817_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign81250_e123827, assign81250_e123827_d_n0, assign81250_e123827_d_n2, assign81250_e123827_d_n4, assign81250_e123827_d_n5, assign81250_e123827_d_n6, assign81250_e123827_d_n7, assign81250_e123827_d_n8, assign81250_e123827_d_n9, assign81250_e123827_d_n10, assign81250_e123827_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81250_e123823: f64 = (4.0 * 0.5);
        let assign81250_e123825: f64 = assign81250_e123823;
        (assign81250_e123825, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign81250_e123827;
        locals.var_tmf2_dn0 = assign81250_e123827_d_n0;
        locals.var_tmf2_dn2 = assign81250_e123827_d_n2;
        locals.var_tmf2_dn4 = assign81250_e123827_d_n4;
        locals.var_tmf2_dn5 = assign81250_e123827_d_n5;
        locals.var_tmf2_dn6 = assign81250_e123827_d_n6;
        locals.var_tmf2_dn7 = assign81250_e123827_d_n7;
        locals.var_tmf2_dn8 = assign81250_e123827_d_n8;
        locals.var_tmf2_dn9 = assign81250_e123827_d_n9;
        locals.var_tmf2_dn10 = assign81250_e123827_d_n10;
        locals.var_tmf2_dn13 = assign81250_e123827_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign81260_e123839, assign81260_e123839_d_n0, assign81260_e123839_d_n2, assign81260_e123839_d_n4, assign81260_e123839_d_n5, assign81260_e123839_d_n6, assign81260_e123839_d_n7, assign81260_e123839_d_n8, assign81260_e123839_d_n9, assign81260_e123839_d_n10, assign81260_e123839_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign81260_e123837, assign81260_e123837_d_n0, assign81260_e123837_d_n2, assign81260_e123837_d_n4, assign81260_e123837_d_n5, assign81260_e123837_d_n6, assign81260_e123837_d_n7, assign81260_e123837_d_n8, assign81260_e123837_d_n9, assign81260_e123837_d_n10, assign81260_e123837_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign81260_e123836: f64 = (-locals.var_tmf2);
                (assign81260_e123836, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign81260_e123837, assign81260_e123837_d_n0, assign81260_e123837_d_n2, assign81260_e123837_d_n4, assign81260_e123837_d_n5, assign81260_e123837_d_n6, assign81260_e123837_d_n7, assign81260_e123837_d_n8, assign81260_e123837_d_n9, assign81260_e123837_d_n10, assign81260_e123837_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign81260_e123839;
        locals.var_tmf2_dn0 = assign81260_e123839_d_n0;
        locals.var_tmf2_dn2 = assign81260_e123839_d_n2;
        locals.var_tmf2_dn4 = assign81260_e123839_d_n4;
        locals.var_tmf2_dn5 = assign81260_e123839_d_n5;
        locals.var_tmf2_dn6 = assign81260_e123839_d_n6;
        locals.var_tmf2_dn7 = assign81260_e123839_d_n7;
        locals.var_tmf2_dn8 = assign81260_e123839_d_n8;
        locals.var_tmf2_dn9 = assign81260_e123839_d_n9;
        locals.var_tmf2_dn10 = assign81260_e123839_d_n10;
        locals.var_tmf2_dn13 = assign81260_e123839_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign81270_e123850, assign81270_e123850_d_n0, assign81270_e123850_d_n2, assign81270_e123850_d_n4, assign81270_e123850_d_n5, assign81270_e123850_d_n6, assign81270_e123850_d_n7, assign81270_e123850_d_n8, assign81270_e123850_d_n9, assign81270_e123850_d_n10, assign81270_e123850_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81270_e123845: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign81270_e123847: f64 = (assign81270_e123845 + locals.var_tmf2);
        let assign81270_e123848: f64 = (assign81270_e123847).sqrt();
        (assign81270_e123848, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign81270_e123848)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign81270_e123850;
        locals.var_tmf2_dn0 = assign81270_e123850_d_n0;
        locals.var_tmf2_dn2 = assign81270_e123850_d_n2;
        locals.var_tmf2_dn4 = assign81270_e123850_d_n4;
        locals.var_tmf2_dn5 = assign81270_e123850_d_n5;
        locals.var_tmf2_dn6 = assign81270_e123850_d_n6;
        locals.var_tmf2_dn7 = assign81270_e123850_d_n7;
        locals.var_tmf2_dn8 = assign81270_e123850_d_n8;
        locals.var_tmf2_dn9 = assign81270_e123850_d_n9;
        locals.var_tmf2_dn10 = assign81270_e123850_d_n10;
        locals.var_tmf2_dn13 = assign81270_e123850_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign81280_e123862, assign81280_e123862_d_n0, assign81280_e123862_d_n2, assign81280_e123862_d_n4, assign81280_e123862_d_n5, assign81280_e123862_d_n6, assign81280_e123862_d_n7, assign81280_e123862_d_n8, assign81280_e123862_d_n9, assign81280_e123862_d_n10, assign81280_e123862_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81280_e123858: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign81280_e123859: f64 = (1.0 + assign81280_e123858);
        let assign81280_e123860: f64 = (0.5 * assign81280_e123859);
        (assign81280_e123860, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81280_e123862;
        locals.var_t0_dn0 = assign81280_e123862_d_n0;
        locals.var_t0_dn2 = assign81280_e123862_d_n2;
        locals.var_t0_dn4 = assign81280_e123862_d_n4;
        locals.var_t0_dn5 = assign81280_e123862_d_n5;
        locals.var_t0_dn6 = assign81280_e123862_d_n6;
        locals.var_t0_dn7 = assign81280_e123862_d_n7;
        locals.var_t0_dn8 = assign81280_e123862_d_n8;
        locals.var_t0_dn9 = assign81280_e123862_d_n9;
        locals.var_t0_dn10 = assign81280_e123862_d_n10;
        locals.var_t0_dn13 = assign81280_e123862_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign81290_e123874, assign81290_e123874_d_n0, assign81290_e123874_d_n2, assign81290_e123874_d_n4, assign81290_e123874_d_n5, assign81290_e123874_d_n6, assign81290_e123874_d_n7, assign81290_e123874_d_n8, assign81290_e123874_d_n9, assign81290_e123874_d_n10, assign81290_e123874_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81290_e123870: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign81290_e123871: f64 = (0.5 * assign81290_e123870);
        let assign81290_e123872: f64 = (0.5 + assign81290_e123871);
        (assign81290_e123872, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign81290_e123874;
        locals.var_t1_dn0 = assign81290_e123874_d_n0;
        locals.var_t1_dn2 = assign81290_e123874_d_n2;
        locals.var_t1_dn4 = assign81290_e123874_d_n4;
        locals.var_t1_dn5 = assign81290_e123874_d_n5;
        locals.var_t1_dn6 = assign81290_e123874_d_n6;
        locals.var_t1_dn7 = assign81290_e123874_d_n7;
        locals.var_t1_dn8 = assign81290_e123874_d_n8;
        locals.var_t1_dn9 = assign81290_e123874_d_n9;
        locals.var_t1_dn10 = assign81290_e123874_d_n10;
        locals.var_t1_dn13 = assign81290_e123874_d_n13;
        locals.var_t1_rv = 0.0;

        let assign81300_e123877: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81300_e123880: f64 = (-locals.var_t1);
        let assign81300_e123885: f64 = if ((assign81300_e123877 > assign81300_e123880) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1901 = assign81300_e123885;
        locals.var_guard1901_rv = 0.0;

        let (assign81310_e123899, assign81310_e123899_d_n0, assign81310_e123899_d_n2, assign81310_e123899_d_n4, assign81310_e123899_d_n5, assign81310_e123899_d_n6, assign81310_e123899_d_n7, assign81310_e123899_d_n8, assign81310_e123899_d_n9, assign81310_e123899_d_n10, assign81310_e123899_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81310_e123893: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81310_e123895: f64 = assign81310_e123893;
        let assign81310_e123897: f64 = (assign81310_e123895 + locals.var_t1);
        (assign81310_e123897, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), ((locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6) + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), (locals.var_vxbgmtcl_dn9 + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn13 + locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign81310_e123899;
        locals.var_tmf1_dn0 = assign81310_e123899_d_n0;
        locals.var_tmf1_dn2 = assign81310_e123899_d_n2;
        locals.var_tmf1_dn4 = assign81310_e123899_d_n4;
        locals.var_tmf1_dn5 = assign81310_e123899_d_n5;
        locals.var_tmf1_dn6 = assign81310_e123899_d_n6;
        locals.var_tmf1_dn7 = assign81310_e123899_d_n7;
        locals.var_tmf1_dn8 = assign81310_e123899_d_n8;
        locals.var_tmf1_dn9 = assign81310_e123899_d_n9;
        locals.var_tmf1_dn10 = assign81310_e123899_d_n10;
        locals.var_tmf1_dn13 = assign81310_e123899_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign81320_e123909, assign81320_e123909_d_n0, assign81320_e123909_d_n2, assign81320_e123909_d_n4, assign81320_e123909_d_n5, assign81320_e123909_d_n6, assign81320_e123909_d_n7, assign81320_e123909_d_n8, assign81320_e123909_d_n9, assign81320_e123909_d_n10, assign81320_e123909_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81320_e123907: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign81320_e123907, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign81320_e123909;
        locals.var_x2_dn0 = assign81320_e123909_d_n0;
        locals.var_x2_dn2 = assign81320_e123909_d_n2;
        locals.var_x2_dn4 = assign81320_e123909_d_n4;
        locals.var_x2_dn5 = assign81320_e123909_d_n5;
        locals.var_x2_dn6 = assign81320_e123909_d_n6;
        locals.var_x2_dn7 = assign81320_e123909_d_n7;
        locals.var_x2_dn8 = assign81320_e123909_d_n8;
        locals.var_x2_dn9 = assign81320_e123909_d_n9;
        locals.var_x2_dn10 = assign81320_e123909_d_n10;
        locals.var_x2_dn13 = assign81320_e123909_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign81330_e123919, assign81330_e123919_d_n0, assign81330_e123919_d_n2, assign81330_e123919_d_n4, assign81330_e123919_d_n5, assign81330_e123919_d_n6, assign81330_e123919_d_n7, assign81330_e123919_d_n8, assign81330_e123919_d_n9, assign81330_e123919_d_n10, assign81330_e123919_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81330_e123917: f64 = (locals.var_t1 * locals.var_t1);
        (assign81330_e123917, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign81330_e123919;
        locals.var_xmax2_dn0 = assign81330_e123919_d_n0;
        locals.var_xmax2_dn2 = assign81330_e123919_d_n2;
        locals.var_xmax2_dn4 = assign81330_e123919_d_n4;
        locals.var_xmax2_dn5 = assign81330_e123919_d_n5;
        locals.var_xmax2_dn6 = assign81330_e123919_d_n6;
        locals.var_xmax2_dn7 = assign81330_e123919_d_n7;
        locals.var_xmax2_dn8 = assign81330_e123919_d_n8;
        locals.var_xmax2_dn9 = assign81330_e123919_d_n9;
        locals.var_xmax2_dn10 = assign81330_e123919_d_n10;
        locals.var_xmax2_dn13 = assign81330_e123919_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign81340_e123927, assign81340_e123927_d_n0, assign81340_e123927_d_n2, assign81340_e123927_d_n4, assign81340_e123927_d_n5, assign81340_e123927_d_n6, assign81340_e123927_d_n7, assign81340_e123927_d_n8, assign81340_e123927_d_n9, assign81340_e123927_d_n10, assign81340_e123927_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign81340_e123927;
        locals.var_xp_dn0 = assign81340_e123927_d_n0;
        locals.var_xp_dn2 = assign81340_e123927_d_n2;
        locals.var_xp_dn4 = assign81340_e123927_d_n4;
        locals.var_xp_dn5 = assign81340_e123927_d_n5;
        locals.var_xp_dn6 = assign81340_e123927_d_n6;
        locals.var_xp_dn7 = assign81340_e123927_d_n7;
        locals.var_xp_dn8 = assign81340_e123927_d_n8;
        locals.var_xp_dn9 = assign81340_e123927_d_n9;
        locals.var_xp_dn10 = assign81340_e123927_d_n10;
        locals.var_xp_dn13 = assign81340_e123927_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign81350_e123935, assign81350_e123935_d_n0, assign81350_e123935_d_n2, assign81350_e123935_d_n4, assign81350_e123935_d_n5, assign81350_e123935_d_n6, assign81350_e123935_d_n7, assign81350_e123935_d_n8, assign81350_e123935_d_n9, assign81350_e123935_d_n10, assign81350_e123935_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign81350_e123935;
        locals.var_xmp_dn0 = assign81350_e123935_d_n0;
        locals.var_xmp_dn2 = assign81350_e123935_d_n2;
        locals.var_xmp_dn4 = assign81350_e123935_d_n4;
        locals.var_xmp_dn5 = assign81350_e123935_d_n5;
        locals.var_xmp_dn6 = assign81350_e123935_d_n6;
        locals.var_xmp_dn7 = assign81350_e123935_d_n7;
        locals.var_xmp_dn8 = assign81350_e123935_d_n8;
        locals.var_xmp_dn9 = assign81350_e123935_d_n9;
        locals.var_xmp_dn10 = assign81350_e123935_d_n10;
        locals.var_xmp_dn13 = assign81350_e123935_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign81360_e123943,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign81360_e123943;
        locals.var_m0_rv = 0.0;

        let (assign81370_e123951,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81370_e123951;
        locals.var_mm_rv = 0.0;

        let (assign81380_e123959, assign81380_e123959_d_n0, assign81380_e123959_d_n2, assign81380_e123959_d_n4, assign81380_e123959_d_n5, assign81380_e123959_d_n6, assign81380_e123959_d_n7, assign81380_e123959_d_n8, assign81380_e123959_d_n9, assign81380_e123959_d_n10, assign81380_e123959_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign81380_e123959;
        locals.var_arg_dn0 = assign81380_e123959_d_n0;
        locals.var_arg_dn2 = assign81380_e123959_d_n2;
        locals.var_arg_dn4 = assign81380_e123959_d_n4;
        locals.var_arg_dn5 = assign81380_e123959_d_n5;
        locals.var_arg_dn6 = assign81380_e123959_d_n6;
        locals.var_arg_dn7 = assign81380_e123959_d_n7;
        locals.var_arg_dn8 = assign81380_e123959_d_n8;
        locals.var_arg_dn9 = assign81380_e123959_d_n9;
        locals.var_arg_dn10 = assign81380_e123959_d_n10;
        locals.var_arg_dn13 = assign81380_e123959_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign81390_e123967, assign81390_e123967_d_n0, assign81390_e123967_d_n2, assign81390_e123967_d_n4, assign81390_e123967_d_n5, assign81390_e123967_d_n6, assign81390_e123967_d_n7, assign81390_e123967_d_n8, assign81390_e123967_d_n9, assign81390_e123967_d_n10, assign81390_e123967_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign81390_e123967;
        locals.var_dnm_dn0 = assign81390_e123967_d_n0;
        locals.var_dnm_dn2 = assign81390_e123967_d_n2;
        locals.var_dnm_dn4 = assign81390_e123967_d_n4;
        locals.var_dnm_dn5 = assign81390_e123967_d_n5;
        locals.var_dnm_dn6 = assign81390_e123967_d_n6;
        locals.var_dnm_dn7 = assign81390_e123967_d_n7;
        locals.var_dnm_dn8 = assign81390_e123967_d_n8;
        locals.var_dnm_dn9 = assign81390_e123967_d_n9;
        locals.var_dnm_dn10 = assign81390_e123967_d_n10;
        locals.var_dnm_dn13 = assign81390_e123967_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign81400_e123977, assign81400_e123977_d_n0, assign81400_e123977_d_n2, assign81400_e123977_d_n4, assign81400_e123977_d_n5, assign81400_e123977_d_n6, assign81400_e123977_d_n7, assign81400_e123977_d_n8, assign81400_e123977_d_n9, assign81400_e123977_d_n10, assign81400_e123977_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81400_e123975: f64 = (locals.var_xp * locals.var_x2);
        (assign81400_e123975, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign81400_e123977;
        locals.var_xp_dn0 = assign81400_e123977_d_n0;
        locals.var_xp_dn2 = assign81400_e123977_d_n2;
        locals.var_xp_dn4 = assign81400_e123977_d_n4;
        locals.var_xp_dn5 = assign81400_e123977_d_n5;
        locals.var_xp_dn6 = assign81400_e123977_d_n6;
        locals.var_xp_dn7 = assign81400_e123977_d_n7;
        locals.var_xp_dn8 = assign81400_e123977_d_n8;
        locals.var_xp_dn9 = assign81400_e123977_d_n9;
        locals.var_xp_dn10 = assign81400_e123977_d_n10;
        locals.var_xp_dn13 = assign81400_e123977_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign81410_e123987, assign81410_e123987_d_n0, assign81410_e123987_d_n2, assign81410_e123987_d_n4, assign81410_e123987_d_n5, assign81410_e123987_d_n6, assign81410_e123987_d_n7, assign81410_e123987_d_n8, assign81410_e123987_d_n9, assign81410_e123987_d_n10, assign81410_e123987_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81410_e123985: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign81410_e123985, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign81410_e123987;
        locals.var_xmp_dn0 = assign81410_e123987_d_n0;
        locals.var_xmp_dn2 = assign81410_e123987_d_n2;
        locals.var_xmp_dn4 = assign81410_e123987_d_n4;
        locals.var_xmp_dn5 = assign81410_e123987_d_n5;
        locals.var_xmp_dn6 = assign81410_e123987_d_n6;
        locals.var_xmp_dn7 = assign81410_e123987_d_n7;
        locals.var_xmp_dn8 = assign81410_e123987_d_n8;
        locals.var_xmp_dn9 = assign81410_e123987_d_n9;
        locals.var_xmp_dn10 = assign81410_e123987_d_n10;
        locals.var_xmp_dn13 = assign81410_e123987_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign81420_e123997, assign81420_e123997_d_n0, assign81420_e123997_d_n2, assign81420_e123997_d_n4, assign81420_e123997_d_n5, assign81420_e123997_d_n6, assign81420_e123997_d_n7, assign81420_e123997_d_n8, assign81420_e123997_d_n9, assign81420_e123997_d_n10, assign81420_e123997_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81420_e123995: f64 = (locals.var_xp + locals.var_xmp);
        (assign81420_e123995, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign81420_e123997;
        locals.var_arg_dn0 = assign81420_e123997_d_n0;
        locals.var_arg_dn2 = assign81420_e123997_d_n2;
        locals.var_arg_dn4 = assign81420_e123997_d_n4;
        locals.var_arg_dn5 = assign81420_e123997_d_n5;
        locals.var_arg_dn6 = assign81420_e123997_d_n6;
        locals.var_arg_dn7 = assign81420_e123997_d_n7;
        locals.var_arg_dn8 = assign81420_e123997_d_n8;
        locals.var_arg_dn9 = assign81420_e123997_d_n9;
        locals.var_arg_dn10 = assign81420_e123997_d_n10;
        locals.var_arg_dn13 = assign81420_e123997_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign81430_e124005, assign81430_e124005_d_n0, assign81430_e124005_d_n2, assign81430_e124005_d_n4, assign81430_e124005_d_n5, assign81430_e124005_d_n6, assign81430_e124005_d_n7, assign81430_e124005_d_n8, assign81430_e124005_d_n9, assign81430_e124005_d_n10, assign81430_e124005_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign81430_e124005;
        locals.var_dnm_dn0 = assign81430_e124005_d_n0;
        locals.var_dnm_dn2 = assign81430_e124005_d_n2;
        locals.var_dnm_dn4 = assign81430_e124005_d_n4;
        locals.var_dnm_dn5 = assign81430_e124005_d_n5;
        locals.var_dnm_dn6 = assign81430_e124005_d_n6;
        locals.var_dnm_dn7 = assign81430_e124005_d_n7;
        locals.var_dnm_dn8 = assign81430_e124005_d_n8;
        locals.var_dnm_dn9 = assign81430_e124005_d_n9;
        locals.var_dnm_dn10 = assign81430_e124005_d_n10;
        locals.var_dnm_dn13 = assign81430_e124005_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign81440_e124020: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1902 = assign81440_e124020;
        locals.var_guard1902_rv = 0.0;

        let assign81450_e124023: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1903 = assign81450_e124023;
        locals.var_guard1903_rv = 0.0;

        let (assign81460_e124035,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) && (locals.var_guard1903 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81460_e124035;
        locals.var_mm_rv = 0.0;

        let assign81470_e124038: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1904 = assign81470_e124038;
        locals.var_guard1904_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_299(
        locals: &mut StampLocals,
    ) {
        let (assign81480_e124053,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) && (locals.var_guard1903 == 0.0)) && (locals.var_guard1904 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81480_e124053;
        locals.var_mm_rv = 0.0;

        let assign81490_e124056: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1905 = assign81490_e124056;
        locals.var_guard1905_rv = 0.0;

        let (assign81500_e124074,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) && (locals.var_guard1903 == 0.0)) && (locals.var_guard1904 == 0.0)) && (locals.var_guard1905 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81500_e124074;
        locals.var_mm_rv = 0.0;

        let assign81510_e124077: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1906 = assign81510_e124077;
        locals.var_guard1906_rv = 0.0;

        let (assign81520_e124098,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) && (locals.var_guard1903 == 0.0)) && (locals.var_guard1904 == 0.0)) && (locals.var_guard1905 == 0.0)) && (locals.var_guard1906 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81520_e124098;
        locals.var_mm_rv = 0.0;

        let (assign81530_e124108,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign81530_e124108;
        locals.var_m0_rv = 0.0;

        let mut assign81540_loop_guard: usize = 0;
        while {
            let assign81540_cond_e124119: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign81540_cond_e124119 != 0.0
        } {
            assign81540_loop_guard += 1;
            assert!(assign81540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign81540_body0_e124130, assign81540_body0_e124130_d_n0, assign81540_body0_e124130_d_n2, assign81540_body0_e124130_d_n4, assign81540_body0_e124130_d_n5, assign81540_body0_e124130_d_n6, assign81540_body0_e124130_d_n7, assign81540_body0_e124130_d_n8, assign81540_body0_e124130_d_n9, assign81540_body0_e124130_d_n10, assign81540_body0_e124130_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) {
        let assign81540_body0_e124128: f64 = (locals.var_dnm).sqrt();
        (assign81540_body0_e124128, (locals.var_dnm_dn0 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn2 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn4 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn5 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn6 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn7 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn8 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn9 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn10 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn13 / (2.0 * assign81540_body0_e124128)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign81540_body0_e124130;
            locals.var_dnm_dn0 = assign81540_body0_e124130_d_n0;
            locals.var_dnm_dn2 = assign81540_body0_e124130_d_n2;
            locals.var_dnm_dn4 = assign81540_body0_e124130_d_n4;
            locals.var_dnm_dn5 = assign81540_body0_e124130_d_n5;
            locals.var_dnm_dn6 = assign81540_body0_e124130_d_n6;
            locals.var_dnm_dn7 = assign81540_body0_e124130_d_n7;
            locals.var_dnm_dn8 = assign81540_body0_e124130_d_n8;
            locals.var_dnm_dn9 = assign81540_body0_e124130_d_n9;
            locals.var_dnm_dn10 = assign81540_body0_e124130_d_n10;
            locals.var_dnm_dn13 = assign81540_body0_e124130_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign81540_body1_e124142,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) {
        let assign81540_body1_e124140: f64 = (locals.var_m0 + 1.0);
        (assign81540_body1_e124140,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign81540_body1_e124142;
            locals.var_m0_rv = 0.0;
        }

        let (assign81550_e124164, assign81550_e124164_d_n0, assign81550_e124164_d_n2, assign81550_e124164_d_n4, assign81550_e124164_d_n5, assign81550_e124164_d_n6, assign81550_e124164_d_n7, assign81550_e124164_d_n8, assign81550_e124164_d_n9, assign81550_e124164_d_n10, assign81550_e124164_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 == 0.0)) {
        let (assign81550_e124162, assign81550_e124162_d_n0, assign81550_e124162_d_n2, assign81550_e124162_d_n4, assign81550_e124162_d_n5, assign81550_e124162_d_n6, assign81550_e124162_d_n7, assign81550_e124162_d_n8, assign81550_e124162_d_n9, assign81550_e124162_d_n10, assign81550_e124162_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign81550_e124159: f64 = 2.0;
                let assign81550_e124160: f64 = (1.0 / assign81550_e124159);
                let assign81550_e124161: f64 = (locals.var_dnm).powf(assign81550_e124160);
                (assign81550_e124161, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn0)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn2)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn4)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn5)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn6)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn7)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn8)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn9)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn10)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn13)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign81550_e124162, assign81550_e124162_d_n0, assign81550_e124162_d_n2, assign81550_e124162_d_n4, assign81550_e124162_d_n5, assign81550_e124162_d_n6, assign81550_e124162_d_n7, assign81550_e124162_d_n8, assign81550_e124162_d_n9, assign81550_e124162_d_n10, assign81550_e124162_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign81550_e124164;
        locals.var_dnm_dn0 = assign81550_e124164_d_n0;
        locals.var_dnm_dn2 = assign81550_e124164_d_n2;
        locals.var_dnm_dn4 = assign81550_e124164_d_n4;
        locals.var_dnm_dn5 = assign81550_e124164_d_n5;
        locals.var_dnm_dn6 = assign81550_e124164_d_n6;
        locals.var_dnm_dn7 = assign81550_e124164_d_n7;
        locals.var_dnm_dn8 = assign81550_e124164_d_n8;
        locals.var_dnm_dn9 = assign81550_e124164_d_n9;
        locals.var_dnm_dn10 = assign81550_e124164_d_n10;
        locals.var_dnm_dn13 = assign81550_e124164_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign81560_e124174, assign81560_e124174_d_n0, assign81560_e124174_d_n2, assign81560_e124174_d_n4, assign81560_e124174_d_n5, assign81560_e124174_d_n6, assign81560_e124174_d_n7, assign81560_e124174_d_n8, assign81560_e124174_d_n9, assign81560_e124174_d_n10, assign81560_e124174_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81560_e124172: f64 = (1.0 / locals.var_dnm);
        (assign81560_e124172, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign81560_e124174;
        locals.var_dnm_dn0 = assign81560_e124174_d_n0;
        locals.var_dnm_dn2 = assign81560_e124174_d_n2;
        locals.var_dnm_dn4 = assign81560_e124174_d_n4;
        locals.var_dnm_dn5 = assign81560_e124174_d_n5;
        locals.var_dnm_dn6 = assign81560_e124174_d_n6;
        locals.var_dnm_dn7 = assign81560_e124174_d_n7;
        locals.var_dnm_dn8 = assign81560_e124174_d_n8;
        locals.var_dnm_dn9 = assign81560_e124174_d_n9;
        locals.var_dnm_dn10 = assign81560_e124174_d_n10;
        locals.var_dnm_dn13 = assign81560_e124174_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign81570_e124186, assign81570_e124186_d_n0, assign81570_e124186_d_n2, assign81570_e124186_d_n4, assign81570_e124186_d_n5, assign81570_e124186_d_n6, assign81570_e124186_d_n7, assign81570_e124186_d_n8, assign81570_e124186_d_n9, assign81570_e124186_d_n10, assign81570_e124186_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81570_e124182: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign81570_e124184: f64 = (assign81570_e124182 * locals.var_dnm);
        (assign81570_e124184, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn13)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign81570_e124186;
        locals.var_tmf0_dn0 = assign81570_e124186_d_n0;
        locals.var_tmf0_dn2 = assign81570_e124186_d_n2;
        locals.var_tmf0_dn4 = assign81570_e124186_d_n4;
        locals.var_tmf0_dn5 = assign81570_e124186_d_n5;
        locals.var_tmf0_dn6 = assign81570_e124186_d_n6;
        locals.var_tmf0_dn7 = assign81570_e124186_d_n7;
        locals.var_tmf0_dn8 = assign81570_e124186_d_n8;
        locals.var_tmf0_dn9 = assign81570_e124186_d_n9;
        locals.var_tmf0_dn10 = assign81570_e124186_d_n10;
        locals.var_tmf0_dn13 = assign81570_e124186_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign81580_e124200, assign81580_e124200_d_n0, assign81580_e124200_d_n2, assign81580_e124200_d_n4, assign81580_e124200_d_n5, assign81580_e124200_d_n6, assign81580_e124200_d_n7, assign81580_e124200_d_n8, assign81580_e124200_d_n9, assign81580_e124200_d_n10, assign81580_e124200_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81580_e124194: f64 = (locals.var_t1 * locals.var_xmp);
        let assign81580_e124196: f64 = (assign81580_e124194 * locals.var_dnm);
        let assign81580_e124198: f64 = (assign81580_e124196 / locals.var_arg);
        (assign81580_e124198, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn0)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn2)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn4)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn5)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn6)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn7)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn8)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn9)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn10)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn13 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn13)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81580_e124200;
        locals.var_t0_dn0 = assign81580_e124200_d_n0;
        locals.var_t0_dn2 = assign81580_e124200_d_n2;
        locals.var_t0_dn4 = assign81580_e124200_d_n4;
        locals.var_t0_dn5 = assign81580_e124200_d_n5;
        locals.var_t0_dn6 = assign81580_e124200_d_n6;
        locals.var_t0_dn7 = assign81580_e124200_d_n7;
        locals.var_t0_dn8 = assign81580_e124200_d_n8;
        locals.var_t0_dn9 = assign81580_e124200_d_n9;
        locals.var_t0_dn10 = assign81580_e124200_d_n10;
        locals.var_t0_dn13 = assign81580_e124200_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign81590_e124212, assign81590_e124212_d_n0, assign81590_e124212_d_n2, assign81590_e124212_d_n4, assign81590_e124212_d_n5, assign81590_e124212_d_n6, assign81590_e124212_d_n7, assign81590_e124212_d_n8, assign81590_e124212_d_n9, assign81590_e124212_d_n10, assign81590_e124212_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81590_e124208: f64 = (-locals.var_t1);
        let assign81590_e124210: f64 = (assign81590_e124208 + locals.var_tmf0);
        (assign81590_e124210, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign81590_e124212;
        locals.var_t1_dn0 = assign81590_e124212_d_n0;
        locals.var_t1_dn2 = assign81590_e124212_d_n2;
        locals.var_t1_dn4 = assign81590_e124212_d_n4;
        locals.var_t1_dn5 = assign81590_e124212_d_n5;
        locals.var_t1_dn6 = assign81590_e124212_d_n6;
        locals.var_t1_dn7 = assign81590_e124212_d_n7;
        locals.var_t1_dn8 = assign81590_e124212_d_n8;
        locals.var_t1_dn9 = assign81590_e124212_d_n9;
        locals.var_t1_dn10 = assign81590_e124212_d_n10;
        locals.var_t1_dn13 = assign81590_e124212_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign81600_e124220, assign81600_e124220_d_n0, assign81600_e124220_d_n2, assign81600_e124220_d_n4, assign81600_e124220_d_n5, assign81600_e124220_d_n6, assign81600_e124220_d_n7, assign81600_e124220_d_n8, assign81600_e124220_d_n9, assign81600_e124220_d_n10, assign81600_e124220_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81600_e124220;
        locals.var_t0_dn0 = assign81600_e124220_d_n0;
        locals.var_t0_dn2 = assign81600_e124220_d_n2;
        locals.var_t0_dn4 = assign81600_e124220_d_n4;
        locals.var_t0_dn5 = assign81600_e124220_d_n5;
        locals.var_t0_dn6 = assign81600_e124220_d_n6;
        locals.var_t0_dn7 = assign81600_e124220_d_n7;
        locals.var_t0_dn8 = assign81600_e124220_d_n8;
        locals.var_t0_dn9 = assign81600_e124220_d_n9;
        locals.var_t0_dn10 = assign81600_e124220_d_n10;
        locals.var_t0_dn13 = assign81600_e124220_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign81610_e124231, assign81610_e124231_d_n0, assign81610_e124231_d_n2, assign81610_e124231_d_n4, assign81610_e124231_d_n5, assign81610_e124231_d_n6, assign81610_e124231_d_n7, assign81610_e124231_d_n8, assign81610_e124231_d_n9, assign81610_e124231_d_n10, assign81610_e124231_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 == 0.0)) {
        let assign81610_e124229: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign81610_e124229, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign81610_e124231;
        locals.var_t1_dn0 = assign81610_e124231_d_n0;
        locals.var_t1_dn2 = assign81610_e124231_d_n2;
        locals.var_t1_dn4 = assign81610_e124231_d_n4;
        locals.var_t1_dn5 = assign81610_e124231_d_n5;
        locals.var_t1_dn6 = assign81610_e124231_d_n6;
        locals.var_t1_dn7 = assign81610_e124231_d_n7;
        locals.var_t1_dn8 = assign81610_e124231_d_n8;
        locals.var_t1_dn9 = assign81610_e124231_d_n9;
        locals.var_t1_dn10 = assign81610_e124231_d_n10;
        locals.var_t1_dn13 = assign81610_e124231_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign81620_e124240, assign81620_e124240_d_n0, assign81620_e124240_d_n2, assign81620_e124240_d_n4, assign81620_e124240_d_n5, assign81620_e124240_d_n6, assign81620_e124240_d_n7, assign81620_e124240_d_n8, assign81620_e124240_d_n9, assign81620_e124240_d_n10, assign81620_e124240_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81620_e124240;
        locals.var_t0_dn0 = assign81620_e124240_d_n0;
        locals.var_t0_dn2 = assign81620_e124240_d_n2;
        locals.var_t0_dn4 = assign81620_e124240_d_n4;
        locals.var_t0_dn5 = assign81620_e124240_d_n5;
        locals.var_t0_dn6 = assign81620_e124240_d_n6;
        locals.var_t0_dn7 = assign81620_e124240_d_n7;
        locals.var_t0_dn8 = assign81620_e124240_d_n8;
        locals.var_t0_dn9 = assign81620_e124240_d_n9;
        locals.var_t0_dn10 = assign81620_e124240_d_n10;
        locals.var_t0_dn13 = assign81620_e124240_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign81630_e124248, assign81630_e124248_d_n0, assign81630_e124248_d_n2, assign81630_e124248_d_n4, assign81630_e124248_d_n5, assign81630_e124248_d_n6, assign81630_e124248_d_n7, assign81630_e124248_d_n8, assign81630_e124248_d_n9, assign81630_e124248_d_n10, assign81630_e124248_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81630_e124246: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign81630_e124246, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, (locals.var_t1_dn6 - locals.var_vgpld_dn6), (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign81630_e124248;
        locals.var_vxbgmtcl_dn0 = assign81630_e124248_d_n0;
        locals.var_vxbgmtcl_dn2 = assign81630_e124248_d_n2;
        locals.var_vxbgmtcl_dn4 = assign81630_e124248_d_n4;
        locals.var_vxbgmtcl_dn5 = assign81630_e124248_d_n5;
        locals.var_vxbgmtcl_dn6 = assign81630_e124248_d_n6;
        locals.var_vxbgmtcl_dn7 = assign81630_e124248_d_n7;
        locals.var_vxbgmtcl_dn8 = assign81630_e124248_d_n8;
        locals.var_vxbgmtcl_dn9 = assign81630_e124248_d_n9;
        locals.var_vxbgmtcl_dn10 = assign81630_e124248_d_n10;
        locals.var_vxbgmtcl_dn13 = assign81630_e124248_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign81640_e124259, assign81640_e124259_d_n0, assign81640_e124259_d_n2, assign81640_e124259_d_n4, assign81640_e124259_d_n5, assign81640_e124259_d_n6, assign81640_e124259_d_n7, assign81640_e124259_d_n8, assign81640_e124259_d_n9, assign81640_e124259_d_n10, assign81640_e124259_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81640_e124253: f64 = (-locals.var_vxbgmtcl);
        let assign81640_e124256: f64 = (10.0 * 2.220446049250313e-16);
        let assign81640_e124257: f64 = (assign81640_e124253 + assign81640_e124256);
        (assign81640_e124257, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn13,)
    }
};
        locals.var_vgb_fb_ld = assign81640_e124259;
        locals.var_vgb_fb_ld_dn0 = assign81640_e124259_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign81640_e124259_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign81640_e124259_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign81640_e124259_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign81640_e124259_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign81640_e124259_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign81640_e124259_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign81640_e124259_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign81640_e124259_d_n10;
        locals.var_vgb_fb_ld_dn13 = assign81640_e124259_d_n13;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign81650_e124262: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard1907 = assign81650_e124262;
        locals.var_guard1907_rv = 0.0;

        let (assign81670_e124283, assign81670_e124283_d_n0, assign81670_e124283_d_n2, assign81670_e124283_d_n4, assign81670_e124283_d_n5, assign81670_e124283_d_n6, assign81670_e124283_d_n7, assign81670_e124283_d_n8, assign81670_e124283_d_n9, assign81670_e124283_d_n10, assign81670_e124283_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81670_e124275: f64 = (2.0 * locals.var_beta_inv);
        let assign81670_e124277: f64 = (-locals.var_vgs_min);
        let assign81670_e124279: f64 = (assign81670_e124277 / locals.var_fac1);
        let assign81670_e124280: f64 = (assign81670_e124279).ln();
        let assign81670_e124281: f64 = (assign81670_e124275 * assign81670_e124280);
        (assign81670_e124281, (((2.0 * locals.var_beta_inv_dn0) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn2) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn4) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn5) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn6) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn7) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn8) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn9) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn10) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn13) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn13) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn13,)
    }
};
        locals.var_ps0_min = assign81670_e124283;
        locals.var_ps0_min_dn0 = assign81670_e124283_d_n0;
        locals.var_ps0_min_dn2 = assign81670_e124283_d_n2;
        locals.var_ps0_min_dn4 = assign81670_e124283_d_n4;
        locals.var_ps0_min_dn5 = assign81670_e124283_d_n5;
        locals.var_ps0_min_dn6 = assign81670_e124283_d_n6;
        locals.var_ps0_min_dn7 = assign81670_e124283_d_n7;
        locals.var_ps0_min_dn8 = assign81670_e124283_d_n8;
        locals.var_ps0_min_dn9 = assign81670_e124283_d_n9;
        locals.var_ps0_min_dn10 = assign81670_e124283_d_n10;
        locals.var_ps0_min_dn13 = assign81670_e124283_d_n13;
        locals.var_ps0_min_rv = 0.0;

        let (assign81680_e124293, assign81680_e124293_d_n0, assign81680_e124293_d_n2, assign81680_e124293_d_n4, assign81680_e124293_d_n5, assign81680_e124293_d_n6, assign81680_e124293_d_n7, assign81680_e124293_d_n8, assign81680_e124293_d_n9, assign81680_e124293_d_n10, assign81680_e124293_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81680_e124290: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81680_e124291: f64 = (locals.var_beta * assign81680_e124290);
        (assign81680_e124291, ((locals.var_beta_dn0 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign81680_e124290) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign81680_e124290) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign81680_e124290) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign81680_e124290) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn9)), ((locals.var_beta_dn10 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn13 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign81680_e124293;
        locals.var_tx_dn0 = assign81680_e124293_d_n0;
        locals.var_tx_dn2 = assign81680_e124293_d_n2;
        locals.var_tx_dn4 = assign81680_e124293_d_n4;
        locals.var_tx_dn5 = assign81680_e124293_d_n5;
        locals.var_tx_dn6 = assign81680_e124293_d_n6;
        locals.var_tx_dn7 = assign81680_e124293_d_n7;
        locals.var_tx_dn8 = assign81680_e124293_d_n8;
        locals.var_tx_dn9 = assign81680_e124293_d_n9;
        locals.var_tx_dn10 = assign81680_e124293_d_n10;
        locals.var_tx_dn13 = assign81680_e124293_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign81690_e124303, assign81690_e124303_d_n0, assign81690_e124303_d_n2, assign81690_e124303_d_n4, assign81690_e124303_d_n5, assign81690_e124303_d_n6, assign81690_e124303_d_n7, assign81690_e124303_d_n8, assign81690_e124303_d_n9, assign81690_e124303_d_n10, assign81690_e124303_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81690_e124300: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign81690_e124301: f64 = (1.0 / assign81690_e124300);
        (assign81690_e124301, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn13 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn13)) / (assign81690_e124300 * assign81690_e124300))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign81690_e124303;
        locals.var_t1_dn0 = assign81690_e124303_d_n0;
        locals.var_t1_dn2 = assign81690_e124303_d_n2;
        locals.var_t1_dn4 = assign81690_e124303_d_n4;
        locals.var_t1_dn5 = assign81690_e124303_d_n5;
        locals.var_t1_dn6 = assign81690_e124303_d_n6;
        locals.var_t1_dn7 = assign81690_e124303_d_n7;
        locals.var_t1_dn8 = assign81690_e124303_d_n8;
        locals.var_t1_dn9 = assign81690_e124303_d_n9;
        locals.var_t1_dn10 = assign81690_e124303_d_n10;
        locals.var_t1_dn13 = assign81690_e124303_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign81700_e124311, assign81700_e124311_d_n0, assign81700_e124311_d_n2, assign81700_e124311_d_n4, assign81700_e124311_d_n5, assign81700_e124311_d_n6, assign81700_e124311_d_n7, assign81700_e124311_d_n8, assign81700_e124311_d_n9, assign81700_e124311_d_n10, assign81700_e124311_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81700_e124309: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign81700_e124309, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn13 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign81700_e124311;
        locals.var_ty_dn0 = assign81700_e124311_d_n0;
        locals.var_ty_dn2 = assign81700_e124311_d_n2;
        locals.var_ty_dn4 = assign81700_e124311_d_n4;
        locals.var_ty_dn5 = assign81700_e124311_d_n5;
        locals.var_ty_dn6 = assign81700_e124311_d_n6;
        locals.var_ty_dn7 = assign81700_e124311_d_n7;
        locals.var_ty_dn8 = assign81700_e124311_d_n8;
        locals.var_ty_dn9 = assign81700_e124311_d_n9;
        locals.var_ty_dn10 = assign81700_e124311_d_n10;
        locals.var_ty_dn13 = assign81700_e124311_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign81710_e124323, assign81710_e124323_d_n0, assign81710_e124323_d_n2, assign81710_e124323_d_n4, assign81710_e124323_d_n5, assign81710_e124323_d_n6, assign81710_e124323_d_n7, assign81710_e124323_d_n8, assign81710_e124323_d_n9, assign81710_e124323_d_n10, assign81710_e124323_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81710_e124318: f64 = (3.0 * 1.414213562373095);
        let assign81710_e124320: f64 = (assign81710_e124318 * locals.var_ty);
        let assign81710_e124321: f64 = (2.0 + assign81710_e124320);
        (assign81710_e124321, (assign81710_e124318 * locals.var_ty_dn0), (assign81710_e124318 * locals.var_ty_dn2), (assign81710_e124318 * locals.var_ty_dn4), (assign81710_e124318 * locals.var_ty_dn5), (assign81710_e124318 * locals.var_ty_dn6), (assign81710_e124318 * locals.var_ty_dn7), (assign81710_e124318 * locals.var_ty_dn8), (assign81710_e124318 * locals.var_ty_dn9), (assign81710_e124318 * locals.var_ty_dn10), (assign81710_e124318 * locals.var_ty_dn13),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn13,)
    }
};
        locals.var_ac41 = assign81710_e124323;
        locals.var_ac41_dn0 = assign81710_e124323_d_n0;
        locals.var_ac41_dn2 = assign81710_e124323_d_n2;
        locals.var_ac41_dn4 = assign81710_e124323_d_n4;
        locals.var_ac41_dn5 = assign81710_e124323_d_n5;
        locals.var_ac41_dn6 = assign81710_e124323_d_n6;
        locals.var_ac41_dn7 = assign81710_e124323_d_n7;
        locals.var_ac41_dn8 = assign81710_e124323_d_n8;
        locals.var_ac41_dn9 = assign81710_e124323_d_n9;
        locals.var_ac41_dn10 = assign81710_e124323_d_n10;
        locals.var_ac41_dn13 = assign81710_e124323_d_n13;
        locals.var_ac41_rv = 0.0;

        let (assign81720_e124335, assign81720_e124335_d_n0, assign81720_e124335_d_n2, assign81720_e124335_d_n4, assign81720_e124335_d_n5, assign81720_e124335_d_n6, assign81720_e124335_d_n7, assign81720_e124335_d_n8, assign81720_e124335_d_n9, assign81720_e124335_d_n10, assign81720_e124335_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81720_e124329: f64 = (8.0 * locals.var_ac41);
        let assign81720_e124331: f64 = (assign81720_e124329 * locals.var_ac41);
        let assign81720_e124333: f64 = (assign81720_e124331 * locals.var_ac41);
        (assign81720_e124333, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn13) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn13)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn13)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn13,)
    }
};
        locals.var_ac4 = assign81720_e124335;
        locals.var_ac4_dn0 = assign81720_e124335_d_n0;
        locals.var_ac4_dn2 = assign81720_e124335_d_n2;
        locals.var_ac4_dn4 = assign81720_e124335_d_n4;
        locals.var_ac4_dn5 = assign81720_e124335_d_n5;
        locals.var_ac4_dn6 = assign81720_e124335_d_n6;
        locals.var_ac4_dn7 = assign81720_e124335_d_n7;
        locals.var_ac4_dn8 = assign81720_e124335_d_n8;
        locals.var_ac4_dn9 = assign81720_e124335_d_n9;
        locals.var_ac4_dn10 = assign81720_e124335_d_n10;
        locals.var_ac4_dn13 = assign81720_e124335_d_n13;
        locals.var_ac4_rv = 0.0;

        let (assign81730_e124351, assign81730_e124351_d_n0, assign81730_e124351_d_n2, assign81730_e124351_d_n4, assign81730_e124351_d_n5, assign81730_e124351_d_n6, assign81730_e124351_d_n7, assign81730_e124351_d_n8, assign81730_e124351_d_n9, assign81730_e124351_d_n10, assign81730_e124351_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81730_e124341: f64 = (7.0 * 1.414213562373095);
        let assign81730_e124344: f64 = (9.0 * locals.var_ty);
        let assign81730_e124347: f64 = (locals.var_tx - 2.0);
        let assign81730_e124348: f64 = (assign81730_e124344 * assign81730_e124347);
        let assign81730_e124349: f64 = (assign81730_e124341 - assign81730_e124348);
        (assign81730_e124349, (-(((9.0 * locals.var_ty_dn0) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn13) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn13))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn13,)
    }
};
        locals.var_ac31 = assign81730_e124351;
        locals.var_ac31_dn0 = assign81730_e124351_d_n0;
        locals.var_ac31_dn2 = assign81730_e124351_d_n2;
        locals.var_ac31_dn4 = assign81730_e124351_d_n4;
        locals.var_ac31_dn5 = assign81730_e124351_d_n5;
        locals.var_ac31_dn6 = assign81730_e124351_d_n6;
        locals.var_ac31_dn7 = assign81730_e124351_d_n7;
        locals.var_ac31_dn8 = assign81730_e124351_d_n8;
        locals.var_ac31_dn9 = assign81730_e124351_d_n9;
        locals.var_ac31_dn10 = assign81730_e124351_d_n10;
        locals.var_ac31_dn13 = assign81730_e124351_d_n13;
        locals.var_ac31_rv = 0.0;

        let (assign81740_e124359, assign81740_e124359_d_n0, assign81740_e124359_d_n2, assign81740_e124359_d_n4, assign81740_e124359_d_n5, assign81740_e124359_d_n6, assign81740_e124359_d_n7, assign81740_e124359_d_n8, assign81740_e124359_d_n9, assign81740_e124359_d_n10, assign81740_e124359_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81740_e124357: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign81740_e124357, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn13 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn13)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn13,)
    }
};
        locals.var_ac3 = assign81740_e124359;
        locals.var_ac3_dn0 = assign81740_e124359_d_n0;
        locals.var_ac3_dn2 = assign81740_e124359_d_n2;
        locals.var_ac3_dn4 = assign81740_e124359_d_n4;
        locals.var_ac3_dn5 = assign81740_e124359_d_n5;
        locals.var_ac3_dn6 = assign81740_e124359_d_n6;
        locals.var_ac3_dn7 = assign81740_e124359_d_n7;
        locals.var_ac3_dn8 = assign81740_e124359_d_n8;
        locals.var_ac3_dn9 = assign81740_e124359_d_n9;
        locals.var_ac3_dn10 = assign81740_e124359_d_n10;
        locals.var_ac3_dn13 = assign81740_e124359_d_n13;
        locals.var_ac3_rv = 0.0;

        let assign81750_e124363: f64 = (locals.var_ac3 * 1e-8);
        let assign81750_e124364: f64 = if locals.var_ac4 < assign81750_e124363 { 1.0 } else { 0.0 };
        locals.var_guard1908 = assign81750_e124364;
        locals.var_guard1908_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_300(
        locals: &mut StampLocals,
    ) {
        let (assign81770_e124385, assign81770_e124385_d_n0, assign81770_e124385_d_n2, assign81770_e124385_d_n4, assign81770_e124385_d_n5, assign81770_e124385_d_n6, assign81770_e124385_d_n7, assign81770_e124385_d_n8, assign81770_e124385_d_n9, assign81770_e124385_d_n10, assign81770_e124385_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) && (locals.var_guard1908 != 0.0)) {
        let assign81770_e124381: f64 = (0.5 * locals.var_ac4);
        let assign81770_e124383: f64 = (assign81770_e124381 / locals.var_ac31);
        (assign81770_e124383, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn13) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn13)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign81770_e124385;
        locals.var_ac1_dn0 = assign81770_e124385_d_n0;
        locals.var_ac1_dn2 = assign81770_e124385_d_n2;
        locals.var_ac1_dn4 = assign81770_e124385_d_n4;
        locals.var_ac1_dn5 = assign81770_e124385_d_n5;
        locals.var_ac1_dn6 = assign81770_e124385_d_n6;
        locals.var_ac1_dn7 = assign81770_e124385_d_n7;
        locals.var_ac1_dn8 = assign81770_e124385_d_n8;
        locals.var_ac1_dn9 = assign81770_e124385_d_n9;
        locals.var_ac1_dn10 = assign81770_e124385_d_n10;
        locals.var_ac1_dn13 = assign81770_e124385_d_n13;
        locals.var_ac1_rv = 0.0;

        let (assign81780_e124397, assign81780_e124397_d_n0, assign81780_e124397_d_n2, assign81780_e124397_d_n4, assign81780_e124397_d_n5, assign81780_e124397_d_n6, assign81780_e124397_d_n7, assign81780_e124397_d_n8, assign81780_e124397_d_n9, assign81780_e124397_d_n10, assign81780_e124397_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) && (locals.var_guard1908 == 0.0)) {
        let assign81780_e124394: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign81780_e124395: f64 = (assign81780_e124394).sqrt();
        (assign81780_e124395, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn13 + locals.var_ac3_dn13) / (2.0 * assign81780_e124395)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn13,)
    }
};
        locals.var_ac2 = assign81780_e124397;
        locals.var_ac2_dn0 = assign81780_e124397_d_n0;
        locals.var_ac2_dn2 = assign81780_e124397_d_n2;
        locals.var_ac2_dn4 = assign81780_e124397_d_n4;
        locals.var_ac2_dn5 = assign81780_e124397_d_n5;
        locals.var_ac2_dn6 = assign81780_e124397_d_n6;
        locals.var_ac2_dn7 = assign81780_e124397_d_n7;
        locals.var_ac2_dn8 = assign81780_e124397_d_n8;
        locals.var_ac2_dn9 = assign81780_e124397_d_n9;
        locals.var_ac2_dn10 = assign81780_e124397_d_n10;
        locals.var_ac2_dn13 = assign81780_e124397_d_n13;
        locals.var_ac2_rv = 0.0;

        let (assign81790_e124409, assign81790_e124409_d_n0, assign81790_e124409_d_n2, assign81790_e124409_d_n4, assign81790_e124409_d_n5, assign81790_e124409_d_n6, assign81790_e124409_d_n7, assign81790_e124409_d_n8, assign81790_e124409_d_n9, assign81790_e124409_d_n10, assign81790_e124409_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) && (locals.var_guard1908 == 0.0)) {
        let assign81790_e124405: f64 = (-locals.var_ac31);
        let assign81790_e124407: f64 = (assign81790_e124405 + locals.var_ac2);
        (assign81790_e124407, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn13) + locals.var_ac2_dn13),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign81790_e124409;
        locals.var_ac1_dn0 = assign81790_e124409_d_n0;
        locals.var_ac1_dn2 = assign81790_e124409_d_n2;
        locals.var_ac1_dn4 = assign81790_e124409_d_n4;
        locals.var_ac1_dn5 = assign81790_e124409_d_n5;
        locals.var_ac1_dn6 = assign81790_e124409_d_n6;
        locals.var_ac1_dn7 = assign81790_e124409_d_n7;
        locals.var_ac1_dn8 = assign81790_e124409_d_n8;
        locals.var_ac1_dn9 = assign81790_e124409_d_n9;
        locals.var_ac1_dn10 = assign81790_e124409_d_n10;
        locals.var_ac1_dn13 = assign81790_e124409_d_n13;
        locals.var_ac1_rv = 0.0;

        let (assign81800_e124417, assign81800_e124417_d_n0, assign81800_e124417_d_n2, assign81800_e124417_d_n4, assign81800_e124417_d_n5, assign81800_e124417_d_n6, assign81800_e124417_d_n7, assign81800_e124417_d_n8, assign81800_e124417_d_n9, assign81800_e124417_d_n10, assign81800_e124417_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81800_e124415: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign81800_e124415, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn13)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn13 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn13,)
    }
};
        locals.var_acd = assign81800_e124417;
        locals.var_acd_dn0 = assign81800_e124417_d_n0;
        locals.var_acd_dn2 = assign81800_e124417_d_n2;
        locals.var_acd_dn4 = assign81800_e124417_d_n4;
        locals.var_acd_dn5 = assign81800_e124417_d_n5;
        locals.var_acd_dn6 = assign81800_e124417_d_n6;
        locals.var_acd_dn7 = assign81800_e124417_d_n7;
        locals.var_acd_dn8 = assign81800_e124417_d_n8;
        locals.var_acd_dn9 = assign81800_e124417_d_n9;
        locals.var_acd_dn10 = assign81800_e124417_d_n10;
        locals.var_acd_dn13 = assign81800_e124417_d_n13;
        locals.var_acd_rv = 0.0;

        let (assign81810_e124440, assign81810_e124440_d_n0, assign81810_e124440_d_n2, assign81810_e124440_d_n4, assign81810_e124440_d_n5, assign81810_e124440_d_n6, assign81810_e124440_d_n7, assign81810_e124440_d_n8, assign81810_e124440_d_n9, assign81810_e124440_d_n10, assign81810_e124440_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81810_e124422: f64 = (-4.0);
        let assign81810_e124424: f64 = (assign81810_e124422 * 1.414213562373095);
        let assign81810_e124427: f64 = (12.0 * locals.var_ty);
        let assign81810_e124428: f64 = (assign81810_e124424 - assign81810_e124427);
        let assign81810_e124431: f64 = (2.0 * locals.var_acd);
        let assign81810_e124432: f64 = (assign81810_e124428 + assign81810_e124431);
        let assign81810_e124435: f64 = (1.414213562373095 * locals.var_acd);
        let assign81810_e124437: f64 = (assign81810_e124435 * locals.var_acd);
        let assign81810_e124438: f64 = (assign81810_e124432 + assign81810_e124437);
        (assign81810_e124438, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn13)) + (2.0 * locals.var_acd_dn13)) + (((1.414213562373095 * locals.var_acd_dn13) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn13))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn13,)
    }
};
        locals.var_acn = assign81810_e124440;
        locals.var_acn_dn0 = assign81810_e124440_d_n0;
        locals.var_acn_dn2 = assign81810_e124440_d_n2;
        locals.var_acn_dn4 = assign81810_e124440_d_n4;
        locals.var_acn_dn5 = assign81810_e124440_d_n5;
        locals.var_acn_dn6 = assign81810_e124440_d_n6;
        locals.var_acn_dn7 = assign81810_e124440_d_n7;
        locals.var_acn_dn8 = assign81810_e124440_d_n8;
        locals.var_acn_dn9 = assign81810_e124440_d_n9;
        locals.var_acn_dn10 = assign81810_e124440_d_n10;
        locals.var_acn_dn13 = assign81810_e124440_d_n13;
        locals.var_acn_rv = 0.0;

        let (assign81820_e124448, assign81820_e124448_d_n0, assign81820_e124448_d_n2, assign81820_e124448_d_n4, assign81820_e124448_d_n5, assign81820_e124448_d_n6, assign81820_e124448_d_n7, assign81820_e124448_d_n8, assign81820_e124448_d_n9, assign81820_e124448_d_n10, assign81820_e124448_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81820_e124446: f64 = (locals.var_acn / locals.var_acd);
        (assign81820_e124446, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn13 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn13)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign81820_e124448;
        locals.var_chi_dn0 = assign81820_e124448_d_n0;
        locals.var_chi_dn2 = assign81820_e124448_d_n2;
        locals.var_chi_dn4 = assign81820_e124448_d_n4;
        locals.var_chi_dn5 = assign81820_e124448_d_n5;
        locals.var_chi_dn6 = assign81820_e124448_d_n6;
        locals.var_chi_dn7 = assign81820_e124448_d_n7;
        locals.var_chi_dn8 = assign81820_e124448_d_n8;
        locals.var_chi_dn9 = assign81820_e124448_d_n9;
        locals.var_chi_dn10 = assign81820_e124448_d_n10;
        locals.var_chi_dn13 = assign81820_e124448_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign81830_e124456, assign81830_e124456_d_n0, assign81830_e124456_d_n2, assign81830_e124456_d_n4, assign81830_e124456_d_n5, assign81830_e124456_d_n6, assign81830_e124456_d_n7, assign81830_e124456_d_n8, assign81830_e124456_d_n9, assign81830_e124456_d_n10, assign81830_e124456_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81830_e124454: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign81830_e124454, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign81830_e124456;
        locals.var_t1_dn0 = assign81830_e124456_d_n0;
        locals.var_t1_dn2 = assign81830_e124456_d_n2;
        locals.var_t1_dn4 = assign81830_e124456_d_n4;
        locals.var_t1_dn5 = assign81830_e124456_d_n5;
        locals.var_t1_dn6 = assign81830_e124456_d_n6;
        locals.var_t1_dn7 = assign81830_e124456_d_n7;
        locals.var_t1_dn8 = assign81830_e124456_d_n8;
        locals.var_t1_dn9 = assign81830_e124456_d_n9;
        locals.var_t1_dn10 = assign81830_e124456_d_n10;
        locals.var_t1_dn13 = assign81830_e124456_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign81840_e124464, assign81840_e124464_d_n0, assign81840_e124464_d_n2, assign81840_e124464_d_n4, assign81840_e124464_d_n5, assign81840_e124464_d_n6, assign81840_e124464_d_n7, assign81840_e124464_d_n8, assign81840_e124464_d_n9, assign81840_e124464_d_n10, assign81840_e124464_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81840_e124462: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign81840_e124462, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn13 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn13)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign81840_e124464;
        locals.var_t2_dn0 = assign81840_e124464_d_n0;
        locals.var_t2_dn2 = assign81840_e124464_d_n2;
        locals.var_t2_dn4 = assign81840_e124464_d_n4;
        locals.var_t2_dn5 = assign81840_e124464_d_n5;
        locals.var_t2_dn6 = assign81840_e124464_d_n6;
        locals.var_t2_dn7 = assign81840_e124464_d_n7;
        locals.var_t2_dn8 = assign81840_e124464_d_n8;
        locals.var_t2_dn9 = assign81840_e124464_d_n9;
        locals.var_t2_dn10 = assign81840_e124464_d_n10;
        locals.var_t2_dn13 = assign81840_e124464_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign81850_e124475, assign81850_e124475_d_n0, assign81850_e124475_d_n2, assign81850_e124475_d_n4, assign81850_e124475_d_n5, assign81850_e124475_d_n6, assign81850_e124475_d_n7, assign81850_e124475_d_n8, assign81850_e124475_d_n9, assign81850_e124475_d_n10, assign81850_e124475_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81850_e124471: f64 = (locals.var_t2 * locals.var_t2);
        let assign81850_e124472: f64 = (1.0 + assign81850_e124471);
        let assign81850_e124473: f64 = (assign81850_e124472).sqrt();
        (assign81850_e124473, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign81850_e124473)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign81850_e124475;
        locals.var_t3_dn0 = assign81850_e124475_d_n0;
        locals.var_t3_dn2 = assign81850_e124475_d_n2;
        locals.var_t3_dn4 = assign81850_e124475_d_n4;
        locals.var_t3_dn5 = assign81850_e124475_d_n5;
        locals.var_t3_dn6 = assign81850_e124475_d_n6;
        locals.var_t3_dn7 = assign81850_e124475_d_n7;
        locals.var_t3_dn8 = assign81850_e124475_d_n8;
        locals.var_t3_dn9 = assign81850_e124475_d_n9;
        locals.var_t3_dn10 = assign81850_e124475_d_n10;
        locals.var_t3_dn13 = assign81850_e124475_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign81860_e124485, assign81860_e124485_d_n0, assign81860_e124485_d_n2, assign81860_e124485_d_n4, assign81860_e124485_d_n5, assign81860_e124485_d_n6, assign81860_e124485_d_n7, assign81860_e124485_d_n8, assign81860_e124485_d_n9, assign81860_e124485_d_n10, assign81860_e124485_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81860_e124481: f64 = (locals.var_t1 / locals.var_t3);
        let assign81860_e124483: f64 = (assign81860_e124481 - locals.var_vxbgmtcl);
        (assign81860_e124483, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn13 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign81860_e124485;
        locals.var_ps0ld_dn0 = assign81860_e124485_d_n0;
        locals.var_ps0ld_dn2 = assign81860_e124485_d_n2;
        locals.var_ps0ld_dn4 = assign81860_e124485_d_n4;
        locals.var_ps0ld_dn5 = assign81860_e124485_d_n5;
        locals.var_ps0ld_dn6 = assign81860_e124485_d_n6;
        locals.var_ps0ld_dn7 = assign81860_e124485_d_n7;
        locals.var_ps0ld_dn8 = assign81860_e124485_d_n8;
        locals.var_ps0ld_dn9 = assign81860_e124485_d_n9;
        locals.var_ps0ld_dn10 = assign81860_e124485_d_n10;
        locals.var_ps0ld_dn13 = assign81860_e124485_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign81870_e124493, assign81870_e124493_d_n0, assign81870_e124493_d_n2, assign81870_e124493_d_n4, assign81870_e124493_d_n5, assign81870_e124493_d_n6, assign81870_e124493_d_n7, assign81870_e124493_d_n8, assign81870_e124493_d_n9, assign81870_e124493_d_n10, assign81870_e124493_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81870_e124491: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign81870_e124491, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign81870_e124493;
        locals.var_t2_dn0 = assign81870_e124493_d_n0;
        locals.var_t2_dn2 = assign81870_e124493_d_n2;
        locals.var_t2_dn4 = assign81870_e124493_d_n4;
        locals.var_t2_dn5 = assign81870_e124493_d_n5;
        locals.var_t2_dn6 = assign81870_e124493_d_n6;
        locals.var_t2_dn7 = assign81870_e124493_d_n7;
        locals.var_t2_dn8 = assign81870_e124493_d_n8;
        locals.var_t2_dn9 = assign81870_e124493_d_n9;
        locals.var_t2_dn10 = assign81870_e124493_d_n10;
        locals.var_t2_dn13 = assign81870_e124493_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign81880_e124501, assign81880_e124501_d_n0, assign81880_e124501_d_n2, assign81880_e124501_d_n4, assign81880_e124501_d_n5, assign81880_e124501_d_n6, assign81880_e124501_d_n7, assign81880_e124501_d_n8, assign81880_e124501_d_n9, assign81880_e124501_d_n10, assign81880_e124501_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81880_e124499: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign81880_e124499, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign81880_e124501;
        locals.var_qsuld_dn0 = assign81880_e124501_d_n0;
        locals.var_qsuld_dn2 = assign81880_e124501_d_n2;
        locals.var_qsuld_dn4 = assign81880_e124501_d_n4;
        locals.var_qsuld_dn5 = assign81880_e124501_d_n5;
        locals.var_qsuld_dn6 = assign81880_e124501_d_n6;
        locals.var_qsuld_dn7 = assign81880_e124501_d_n7;
        locals.var_qsuld_dn8 = assign81880_e124501_d_n8;
        locals.var_qsuld_dn9 = assign81880_e124501_d_n9;
        locals.var_qsuld_dn10 = assign81880_e124501_d_n10;
        locals.var_qsuld_dn13 = assign81880_e124501_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign81890_e124507, assign81890_e124507_d_n0, assign81890_e124507_d_n2, assign81890_e124507_d_n4, assign81890_e124507_d_n5, assign81890_e124507_d_n6, assign81890_e124507_d_n7, assign81890_e124507_d_n8, assign81890_e124507_d_n9, assign81890_e124507_d_n10, assign81890_e124507_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign81890_e124507;
        locals.var_qbuld_dn0 = assign81890_e124507_d_n0;
        locals.var_qbuld_dn2 = assign81890_e124507_d_n2;
        locals.var_qbuld_dn4 = assign81890_e124507_d_n4;
        locals.var_qbuld_dn5 = assign81890_e124507_d_n5;
        locals.var_qbuld_dn6 = assign81890_e124507_d_n6;
        locals.var_qbuld_dn7 = assign81890_e124507_d_n7;
        locals.var_qbuld_dn8 = assign81890_e124507_d_n8;
        locals.var_qbuld_dn9 = assign81890_e124507_d_n9;
        locals.var_qbuld_dn10 = assign81890_e124507_d_n10;
        locals.var_qbuld_dn13 = assign81890_e124507_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign81900_e124513, assign81900_e124513_d_n0, assign81900_e124513_d_n2, assign81900_e124513_d_n4, assign81900_e124513_d_n5, assign81900_e124513_d_n6, assign81900_e124513_d_n7, assign81900_e124513_d_n8, assign81900_e124513_d_n9, assign81900_e124513_d_n10, assign81900_e124513_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini__blk1890, locals.var_ps0ld_ini__blk1890_dn0, locals.var_ps0ld_ini__blk1890_dn2, locals.var_ps0ld_ini__blk1890_dn4, locals.var_ps0ld_ini__blk1890_dn5, locals.var_ps0ld_ini__blk1890_dn6, locals.var_ps0ld_ini__blk1890_dn7, locals.var_ps0ld_ini__blk1890_dn8, locals.var_ps0ld_ini__blk1890_dn9, locals.var_ps0ld_ini__blk1890_dn10, locals.var_ps0ld_ini__blk1890_dn13,)
    }
};
        locals.var_ps0ld_ini__blk1890 = assign81900_e124513;
        locals.var_ps0ld_ini__blk1890_dn0 = assign81900_e124513_d_n0;
        locals.var_ps0ld_ini__blk1890_dn2 = assign81900_e124513_d_n2;
        locals.var_ps0ld_ini__blk1890_dn4 = assign81900_e124513_d_n4;
        locals.var_ps0ld_ini__blk1890_dn5 = assign81900_e124513_d_n5;
        locals.var_ps0ld_ini__blk1890_dn6 = assign81900_e124513_d_n6;
        locals.var_ps0ld_ini__blk1890_dn7 = assign81900_e124513_d_n7;
        locals.var_ps0ld_ini__blk1890_dn8 = assign81900_e124513_d_n8;
        locals.var_ps0ld_ini__blk1890_dn9 = assign81900_e124513_d_n9;
        locals.var_ps0ld_ini__blk1890_dn10 = assign81900_e124513_d_n10;
        locals.var_ps0ld_ini__blk1890_dn13 = assign81900_e124513_d_n13;
        locals.var_ps0ld_ini__blk1890_rv = 0.0;

        let assign81910_e124517: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81910_e124518: f64 = (locals.var_beta * assign81910_e124517);
        let assign81910_e124522: f64 = (10.0 * 2.220446049250313e-16);
        let assign81910_e124524: f64 = (assign81910_e124522 - 1.0);
        let assign81910_e124526: f64 = (assign81910_e124524 * locals.var_fac1p2);
        let assign81910_e124528: f64 = (assign81910_e124526 * locals.var_beta2);
        let assign81910_e124530: f64 = (assign81910_e124528 / 4.0);
        let assign81910_e124531: f64 = (1.0 + assign81910_e124530);
        let assign81910_e124532: f64 = if assign81910_e124518 < assign81910_e124531 { 1.0 } else { 0.0 };
        locals.var_guard1909 = assign81910_e124532;
        locals.var_guard1909_rv = 0.0;

        let (assign81920_e124547, assign81920_e124547_d_n0, assign81920_e124547_d_n2, assign81920_e124547_d_n4, assign81920_e124547_d_n5, assign81920_e124547_d_n6, assign81920_e124547_d_n7, assign81920_e124547_d_n8, assign81920_e124547_d_n9, assign81920_e124547_d_n10, assign81920_e124547_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1909 != 0.0)) {
        let assign81920_e124542: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign81920_e124544: f64 = (assign81920_e124542 / 2.0);
        let assign81920_e124545: f64 = (locals.var_vgpld + assign81920_e124544);
        (assign81920_e124545, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (locals.var_vgpld_dn6 + (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0)), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign81920_e124547;
        locals.var_ps0_inia_dn0 = assign81920_e124547_d_n0;
        locals.var_ps0_inia_dn2 = assign81920_e124547_d_n2;
        locals.var_ps0_inia_dn4 = assign81920_e124547_d_n4;
        locals.var_ps0_inia_dn5 = assign81920_e124547_d_n5;
        locals.var_ps0_inia_dn6 = assign81920_e124547_d_n6;
        locals.var_ps0_inia_dn7 = assign81920_e124547_d_n7;
        locals.var_ps0_inia_dn8 = assign81920_e124547_d_n8;
        locals.var_ps0_inia_dn9 = assign81920_e124547_d_n9;
        locals.var_ps0_inia_dn10 = assign81920_e124547_d_n10;
        locals.var_ps0_inia_dn13 = assign81920_e124547_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign81930_e124571, assign81930_e124571_d_n0, assign81930_e124571_d_n2, assign81930_e124571_d_n4, assign81930_e124571_d_n5, assign81930_e124571_d_n6, assign81930_e124571_d_n7, assign81930_e124571_d_n8, assign81930_e124571_d_n9, assign81930_e124571_d_n10, assign81930_e124571_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1909 == 0.0)) {
        let assign81930_e124560: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81930_e124561: f64 = (locals.var_beta * assign81930_e124560);
        let assign81930_e124563: f64 = (assign81930_e124561 - 1.0);
        let assign81930_e124564: f64 = (4.0 * assign81930_e124563);
        let assign81930_e124567: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign81930_e124568: f64 = (assign81930_e124564 / assign81930_e124567);
        let assign81930_e124569: f64 = (1.0 + assign81930_e124568);
        (assign81930_e124569, ((((4.0 * ((locals.var_beta_dn0 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn2 * assign81930_e124560) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn4 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn5 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn6 * assign81930_e124560) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn7 * assign81930_e124560) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn8 * assign81930_e124560) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn9 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn9))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn10 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn13 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn13))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign81930_e124567 * assign81930_e124567)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign81930_e124571;
        locals.var_tx_dn0 = assign81930_e124571_d_n0;
        locals.var_tx_dn2 = assign81930_e124571_d_n2;
        locals.var_tx_dn4 = assign81930_e124571_d_n4;
        locals.var_tx_dn5 = assign81930_e124571_d_n5;
        locals.var_tx_dn6 = assign81930_e124571_d_n6;
        locals.var_tx_dn7 = assign81930_e124571_d_n7;
        locals.var_tx_dn8 = assign81930_e124571_d_n8;
        locals.var_tx_dn9 = assign81930_e124571_d_n9;
        locals.var_tx_dn10 = assign81930_e124571_d_n10;
        locals.var_tx_dn13 = assign81930_e124571_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign81940_e124592, assign81940_e124592_d_n0, assign81940_e124592_d_n2, assign81940_e124592_d_n4, assign81940_e124592_d_n5, assign81940_e124592_d_n6, assign81940_e124592_d_n7, assign81940_e124592_d_n8, assign81940_e124592_d_n9, assign81940_e124592_d_n10, assign81940_e124592_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1909 == 0.0)) {
        let assign81940_e124582: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign81940_e124584: f64 = (assign81940_e124582 / 2.0);
        let assign81940_e124587: f64 = (locals.var_tx).sqrt();
        let assign81940_e124588: f64 = (1.0 - assign81940_e124587);
        let assign81940_e124589: f64 = (assign81940_e124584 * assign81940_e124588);
        let assign81940_e124590: f64 = (locals.var_vgpld + assign81940_e124589);
        (assign81940_e124590, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn0 / (2.0 * assign81940_e124587))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn2 / (2.0 * assign81940_e124587)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn4 / (2.0 * assign81940_e124587))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn5 / (2.0 * assign81940_e124587))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn6 / (2.0 * assign81940_e124587)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn7 / (2.0 * assign81940_e124587)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn8 / (2.0 * assign81940_e124587)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn9 / (2.0 * assign81940_e124587))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn10 / (2.0 * assign81940_e124587))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn13 / (2.0 * assign81940_e124587))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign81940_e124592;
        locals.var_ps0_inia_dn0 = assign81940_e124592_d_n0;
        locals.var_ps0_inia_dn2 = assign81940_e124592_d_n2;
        locals.var_ps0_inia_dn4 = assign81940_e124592_d_n4;
        locals.var_ps0_inia_dn5 = assign81940_e124592_d_n5;
        locals.var_ps0_inia_dn6 = assign81940_e124592_d_n6;
        locals.var_ps0_inia_dn7 = assign81940_e124592_d_n7;
        locals.var_ps0_inia_dn8 = assign81940_e124592_d_n8;
        locals.var_ps0_inia_dn9 = assign81940_e124592_d_n9;
        locals.var_ps0_inia_dn10 = assign81940_e124592_d_n10;
        locals.var_ps0_inia_dn13 = assign81940_e124592_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign81950_e124603, assign81950_e124603_d_n0, assign81950_e124603_d_n2, assign81950_e124603_d_n4, assign81950_e124603_d_n5, assign81950_e124603_d_n6, assign81950_e124603_d_n7, assign81950_e124603_d_n8, assign81950_e124603_d_n9, assign81950_e124603_d_n10, assign81950_e124603_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        let assign81950_e124600: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign81950_e124601: f64 = (locals.var_beta * assign81950_e124600);
        (assign81950_e124601, ((locals.var_beta_dn0 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign81950_e124603;
        locals.var_chi_dn0 = assign81950_e124603_d_n0;
        locals.var_chi_dn2 = assign81950_e124603_d_n2;
        locals.var_chi_dn4 = assign81950_e124603_d_n4;
        locals.var_chi_dn5 = assign81950_e124603_d_n5;
        locals.var_chi_dn6 = assign81950_e124603_d_n6;
        locals.var_chi_dn7 = assign81950_e124603_d_n7;
        locals.var_chi_dn8 = assign81950_e124603_d_n8;
        locals.var_chi_dn9 = assign81950_e124603_d_n9;
        locals.var_chi_dn10 = assign81950_e124603_d_n10;
        locals.var_chi_dn13 = assign81950_e124603_d_n13;
        locals.var_chi_rv = 0.0;

        let assign81960_e124606: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1910 = assign81960_e124606;
        locals.var_guard1910_rv = 0.0;

        let (assign81980_e124626, assign81980_e124626_d_n0, assign81980_e124626_d_n2, assign81980_e124626_d_n4, assign81980_e124626_d_n5, assign81980_e124626_d_n6, assign81980_e124626_d_n7, assign81980_e124626_d_n8, assign81980_e124626_d_n9, assign81980_e124626_d_n10, assign81980_e124626_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign81980_e124623: f64 = (-locals.var_chi);
        let assign81980_e124624: f64 = (assign81980_e124623).exp();
        (assign81980_e124624, (assign81980_e124624 * (-locals.var_chi_dn0)), (assign81980_e124624 * (-locals.var_chi_dn2)), (assign81980_e124624 * (-locals.var_chi_dn4)), (assign81980_e124624 * (-locals.var_chi_dn5)), (assign81980_e124624 * (-locals.var_chi_dn6)), (assign81980_e124624 * (-locals.var_chi_dn7)), (assign81980_e124624 * (-locals.var_chi_dn8)), (assign81980_e124624 * (-locals.var_chi_dn9)), (assign81980_e124624 * (-locals.var_chi_dn10)), (assign81980_e124624 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign81980_e124626;
        locals.var_ty_dn0 = assign81980_e124626_d_n0;
        locals.var_ty_dn2 = assign81980_e124626_d_n2;
        locals.var_ty_dn4 = assign81980_e124626_d_n4;
        locals.var_ty_dn5 = assign81980_e124626_d_n5;
        locals.var_ty_dn6 = assign81980_e124626_d_n6;
        locals.var_ty_dn7 = assign81980_e124626_d_n7;
        locals.var_ty_dn8 = assign81980_e124626_d_n8;
        locals.var_ty_dn9 = assign81980_e124626_d_n9;
        locals.var_ty_dn10 = assign81980_e124626_d_n10;
        locals.var_ty_dn13 = assign81980_e124626_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign81990_e124651, assign81990_e124651_d_n0, assign81990_e124651_d_n2, assign81990_e124651_d_n4, assign81990_e124651_d_n5, assign81990_e124651_d_n6, assign81990_e124651_d_n7, assign81990_e124651_d_n8, assign81990_e124651_d_n9, assign81990_e124651_d_n10, assign81990_e124651_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign81990_e124638: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81990_e124639: f64 = (locals.var_beta * assign81990_e124638);
        let assign81990_e124641: f64 = (assign81990_e124639 - 1.0);
        let assign81990_e124643: f64 = (assign81990_e124641 + locals.var_ty);
        let assign81990_e124644: f64 = (4.0 * assign81990_e124643);
        let assign81990_e124647: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign81990_e124648: f64 = (assign81990_e124644 / assign81990_e124647);
        let assign81990_e124649: f64 = (1.0 + assign81990_e124648);
        (assign81990_e124649, ((((4.0 * (((locals.var_beta_dn0 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn2 * assign81990_e124638) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn4 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn5 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn6 * assign81990_e124638) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))) + locals.var_ty_dn6)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn7 * assign81990_e124638) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn8 * assign81990_e124638) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn9 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn9)) + locals.var_ty_dn9)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn10 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn13 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn13)) + locals.var_ty_dn13)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign81990_e124647 * assign81990_e124647)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign81990_e124651;
        locals.var_tx_dn0 = assign81990_e124651_d_n0;
        locals.var_tx_dn2 = assign81990_e124651_d_n2;
        locals.var_tx_dn4 = assign81990_e124651_d_n4;
        locals.var_tx_dn5 = assign81990_e124651_d_n5;
        locals.var_tx_dn6 = assign81990_e124651_d_n6;
        locals.var_tx_dn7 = assign81990_e124651_d_n7;
        locals.var_tx_dn8 = assign81990_e124651_d_n8;
        locals.var_tx_dn9 = assign81990_e124651_d_n9;
        locals.var_tx_dn10 = assign81990_e124651_d_n10;
        locals.var_tx_dn13 = assign81990_e124651_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign82000_e124671, assign82000_e124671_d_n0, assign82000_e124671_d_n2, assign82000_e124671_d_n4, assign82000_e124671_d_n5, assign82000_e124671_d_n6, assign82000_e124671_d_n7, assign82000_e124671_d_n8, assign82000_e124671_d_n9, assign82000_e124671_d_n10, assign82000_e124671_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82000_e124661: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign82000_e124663: f64 = (assign82000_e124661 / 2.0);
        let assign82000_e124666: f64 = (locals.var_tx).sqrt();
        let assign82000_e124667: f64 = (1.0 - assign82000_e124666);
        let assign82000_e124668: f64 = (assign82000_e124663 * assign82000_e124667);
        let assign82000_e124669: f64 = (locals.var_vgpld + assign82000_e124668);
        (assign82000_e124669, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn0 / (2.0 * assign82000_e124666))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn2 / (2.0 * assign82000_e124666)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn4 / (2.0 * assign82000_e124666))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn5 / (2.0 * assign82000_e124666))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn6 / (2.0 * assign82000_e124666)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn7 / (2.0 * assign82000_e124666)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn8 / (2.0 * assign82000_e124666)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn9 / (2.0 * assign82000_e124666))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn10 / (2.0 * assign82000_e124666))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn13 / (2.0 * assign82000_e124666))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign82000_e124671;
        locals.var_ps0_inia_dn0 = assign82000_e124671_d_n0;
        locals.var_ps0_inia_dn2 = assign82000_e124671_d_n2;
        locals.var_ps0_inia_dn4 = assign82000_e124671_d_n4;
        locals.var_ps0_inia_dn5 = assign82000_e124671_d_n5;
        locals.var_ps0_inia_dn6 = assign82000_e124671_d_n6;
        locals.var_ps0_inia_dn7 = assign82000_e124671_d_n7;
        locals.var_ps0_inia_dn8 = assign82000_e124671_d_n8;
        locals.var_ps0_inia_dn9 = assign82000_e124671_d_n9;
        locals.var_ps0_inia_dn10 = assign82000_e124671_d_n10;
        locals.var_ps0_inia_dn13 = assign82000_e124671_d_n13;
        locals.var_ps0_inia_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_301(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82010_e124684, assign82010_e124684_d_n0, assign82010_e124684_d_n2, assign82010_e124684_d_n4, assign82010_e124684_d_n5, assign82010_e124684_d_n6, assign82010_e124684_d_n7, assign82010_e124684_d_n8, assign82010_e124684_d_n9, assign82010_e124684_d_n10, assign82010_e124684_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82010_e124681: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign82010_e124682: f64 = (locals.var_beta * assign82010_e124681);
        (assign82010_e124682, ((locals.var_beta_dn0 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82010_e124684;
        locals.var_chi_dn0 = assign82010_e124684_d_n0;
        locals.var_chi_dn2 = assign82010_e124684_d_n2;
        locals.var_chi_dn4 = assign82010_e124684_d_n4;
        locals.var_chi_dn5 = assign82010_e124684_d_n5;
        locals.var_chi_dn6 = assign82010_e124684_d_n6;
        locals.var_chi_dn7 = assign82010_e124684_d_n7;
        locals.var_chi_dn8 = assign82010_e124684_d_n8;
        locals.var_chi_dn9 = assign82010_e124684_d_n9;
        locals.var_chi_dn10 = assign82010_e124684_d_n10;
        locals.var_chi_dn13 = assign82010_e124684_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign82020_e124695, assign82020_e124695_d_n0, assign82020_e124695_d_n2, assign82020_e124695_d_n4, assign82020_e124695_d_n5, assign82020_e124695_d_n6, assign82020_e124695_d_n7, assign82020_e124695_d_n8, assign82020_e124695_d_n9, assign82020_e124695_d_n10, assign82020_e124695_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82020_e124692: f64 = (-locals.var_chi);
        let assign82020_e124693: f64 = (assign82020_e124692).exp();
        (assign82020_e124693, (assign82020_e124693 * (-locals.var_chi_dn0)), (assign82020_e124693 * (-locals.var_chi_dn2)), (assign82020_e124693 * (-locals.var_chi_dn4)), (assign82020_e124693 * (-locals.var_chi_dn5)), (assign82020_e124693 * (-locals.var_chi_dn6)), (assign82020_e124693 * (-locals.var_chi_dn7)), (assign82020_e124693 * (-locals.var_chi_dn8)), (assign82020_e124693 * (-locals.var_chi_dn9)), (assign82020_e124693 * (-locals.var_chi_dn10)), (assign82020_e124693 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign82020_e124695;
        locals.var_ty_dn0 = assign82020_e124695_d_n0;
        locals.var_ty_dn2 = assign82020_e124695_d_n2;
        locals.var_ty_dn4 = assign82020_e124695_d_n4;
        locals.var_ty_dn5 = assign82020_e124695_d_n5;
        locals.var_ty_dn6 = assign82020_e124695_d_n6;
        locals.var_ty_dn7 = assign82020_e124695_d_n7;
        locals.var_ty_dn8 = assign82020_e124695_d_n8;
        locals.var_ty_dn9 = assign82020_e124695_d_n9;
        locals.var_ty_dn10 = assign82020_e124695_d_n10;
        locals.var_ty_dn13 = assign82020_e124695_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign82030_e124720, assign82030_e124720_d_n0, assign82030_e124720_d_n2, assign82030_e124720_d_n4, assign82030_e124720_d_n5, assign82030_e124720_d_n6, assign82030_e124720_d_n7, assign82030_e124720_d_n8, assign82030_e124720_d_n9, assign82030_e124720_d_n10, assign82030_e124720_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82030_e124707: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign82030_e124708: f64 = (locals.var_beta * assign82030_e124707);
        let assign82030_e124710: f64 = (assign82030_e124708 - 1.0);
        let assign82030_e124712: f64 = (assign82030_e124710 + locals.var_ty);
        let assign82030_e124713: f64 = (4.0 * assign82030_e124712);
        let assign82030_e124716: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign82030_e124717: f64 = (assign82030_e124713 / assign82030_e124716);
        let assign82030_e124718: f64 = (1.0 + assign82030_e124717);
        (assign82030_e124718, ((((4.0 * (((locals.var_beta_dn0 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn2 * assign82030_e124707) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn4 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn5 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn6 * assign82030_e124707) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))) + locals.var_ty_dn6)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn7 * assign82030_e124707) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn8 * assign82030_e124707) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn9 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn9)) + locals.var_ty_dn9)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn10 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn13 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn13)) + locals.var_ty_dn13)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign82030_e124716 * assign82030_e124716)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign82030_e124720;
        locals.var_tx_dn0 = assign82030_e124720_d_n0;
        locals.var_tx_dn2 = assign82030_e124720_d_n2;
        locals.var_tx_dn4 = assign82030_e124720_d_n4;
        locals.var_tx_dn5 = assign82030_e124720_d_n5;
        locals.var_tx_dn6 = assign82030_e124720_d_n6;
        locals.var_tx_dn7 = assign82030_e124720_d_n7;
        locals.var_tx_dn8 = assign82030_e124720_d_n8;
        locals.var_tx_dn9 = assign82030_e124720_d_n9;
        locals.var_tx_dn10 = assign82030_e124720_d_n10;
        locals.var_tx_dn13 = assign82030_e124720_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign82040_e124740, assign82040_e124740_d_n0, assign82040_e124740_d_n2, assign82040_e124740_d_n4, assign82040_e124740_d_n5, assign82040_e124740_d_n6, assign82040_e124740_d_n7, assign82040_e124740_d_n8, assign82040_e124740_d_n9, assign82040_e124740_d_n10, assign82040_e124740_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82040_e124730: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign82040_e124732: f64 = (assign82040_e124730 / 2.0);
        let assign82040_e124735: f64 = (locals.var_tx).sqrt();
        let assign82040_e124736: f64 = (1.0 - assign82040_e124735);
        let assign82040_e124737: f64 = (assign82040_e124732 * assign82040_e124736);
        let assign82040_e124738: f64 = (locals.var_vgpld + assign82040_e124737);
        (assign82040_e124738, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn0 / (2.0 * assign82040_e124735))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn2 / (2.0 * assign82040_e124735)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn4 / (2.0 * assign82040_e124735))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn5 / (2.0 * assign82040_e124735))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn6 / (2.0 * assign82040_e124735)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn7 / (2.0 * assign82040_e124735)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn8 / (2.0 * assign82040_e124735)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn9 / (2.0 * assign82040_e124735))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn10 / (2.0 * assign82040_e124735))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn13 / (2.0 * assign82040_e124735))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign82040_e124740;
        locals.var_ps0_inia_dn0 = assign82040_e124740_d_n0;
        locals.var_ps0_inia_dn2 = assign82040_e124740_d_n2;
        locals.var_ps0_inia_dn4 = assign82040_e124740_d_n4;
        locals.var_ps0_inia_dn5 = assign82040_e124740_d_n5;
        locals.var_ps0_inia_dn6 = assign82040_e124740_d_n6;
        locals.var_ps0_inia_dn7 = assign82040_e124740_d_n7;
        locals.var_ps0_inia_dn8 = assign82040_e124740_d_n8;
        locals.var_ps0_inia_dn9 = assign82040_e124740_d_n9;
        locals.var_ps0_inia_dn10 = assign82040_e124740_d_n10;
        locals.var_ps0_inia_dn13 = assign82040_e124740_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign82050_e124753, assign82050_e124753_d_n0, assign82050_e124753_d_n2, assign82050_e124753_d_n4, assign82050_e124753_d_n5, assign82050_e124753_d_n6, assign82050_e124753_d_n7, assign82050_e124753_d_n8, assign82050_e124753_d_n9, assign82050_e124753_d_n10, assign82050_e124753_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82050_e124750: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign82050_e124751: f64 = (locals.var_beta * assign82050_e124750);
        (assign82050_e124751, ((locals.var_beta_dn0 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82050_e124753;
        locals.var_chi_dn0 = assign82050_e124753_d_n0;
        locals.var_chi_dn2 = assign82050_e124753_d_n2;
        locals.var_chi_dn4 = assign82050_e124753_d_n4;
        locals.var_chi_dn5 = assign82050_e124753_d_n5;
        locals.var_chi_dn6 = assign82050_e124753_d_n6;
        locals.var_chi_dn7 = assign82050_e124753_d_n7;
        locals.var_chi_dn8 = assign82050_e124753_d_n8;
        locals.var_chi_dn9 = assign82050_e124753_d_n9;
        locals.var_chi_dn10 = assign82050_e124753_d_n10;
        locals.var_chi_dn13 = assign82050_e124753_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign82070_e124795,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82070_e124774: f64 = (2.0_f64).sqrt();
        let assign82070_e124775: f64 = (9.0 * assign82070_e124774);
        let assign82070_e124776: f64 = (1.0 / assign82070_e124775);
        let assign82070_e124780: f64 = (-3.0);
        let assign82070_e124781: f64 = (assign82070_e124780).exp();
        let assign82070_e124782: f64 = (7.0 * assign82070_e124781);
        let assign82070_e124783: f64 = (5.0 + assign82070_e124782);
        let assign82070_e124787: f64 = (-3.0);
        let assign82070_e124788: f64 = (assign82070_e124787).exp();
        let assign82070_e124789: f64 = (2.0 + assign82070_e124788);
        let assign82070_e124790: f64 = (assign82070_e124789).sqrt();
        let assign82070_e124791: f64 = (54.0 * assign82070_e124790);
        let assign82070_e124792: f64 = (assign82070_e124783 / assign82070_e124791);
        let assign82070_e124793: f64 = (assign82070_e124776 - assign82070_e124792);
        (assign82070_e124793,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign82070_e124795;
        locals.var_ta_rv = 0.0;

        let (assign82080_e124823,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82080_e124805: f64 = (-3.0);
        let assign82080_e124806: f64 = (assign82080_e124805).exp();
        let assign82080_e124807: f64 = (1.0 + assign82080_e124806);
        let assign82080_e124811: f64 = (-3.0);
        let assign82080_e124812: f64 = (assign82080_e124811).exp();
        let assign82080_e124813: f64 = (2.0 + assign82080_e124812);
        let assign82080_e124814: f64 = (assign82080_e124813).sqrt();
        let assign82080_e124815: f64 = (2.0 * assign82080_e124814);
        let assign82080_e124816: f64 = (assign82080_e124807 / assign82080_e124815);
        let assign82080_e124818: f64 = (2.0_f64).sqrt();
        let assign82080_e124820: f64 = (assign82080_e124818 / 3.0);
        let assign82080_e124821: f64 = (assign82080_e124816 - assign82080_e124820);
        (assign82080_e124821,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign82080_e124823;
        locals.var_tb_rv = 0.0;

        let (assign82090_e124842, assign82090_e124842_d_n0, assign82090_e124842_d_n2, assign82090_e124842_d_n4, assign82090_e124842_d_n5, assign82090_e124842_d_n6, assign82090_e124842_d_n7, assign82090_e124842_d_n8, assign82090_e124842_d_n9, assign82090_e124842_d_n10, assign82090_e124842_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82090_e124833: f64 = (2.0_f64).sqrt();
        let assign82090_e124834: f64 = (1.0 / assign82090_e124833);
        let assign82090_e124838: f64 = (locals.var_beta * locals.var_fac1);
        let assign82090_e124839: f64 = (1.0 / assign82090_e124838);
        let assign82090_e124840: f64 = (assign82090_e124834 + assign82090_e124839);
        (assign82090_e124840, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn13 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn13)) / (assign82090_e124838 * assign82090_e124838))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn13,)
    }
};
        locals.var_tc = assign82090_e124842;
        locals.var_tc_dn0 = assign82090_e124842_d_n0;
        locals.var_tc_dn2 = assign82090_e124842_d_n2;
        locals.var_tc_dn4 = assign82090_e124842_d_n4;
        locals.var_tc_dn5 = assign82090_e124842_d_n5;
        locals.var_tc_dn6 = assign82090_e124842_d_n6;
        locals.var_tc_dn7 = assign82090_e124842_d_n7;
        locals.var_tc_dn8 = assign82090_e124842_d_n8;
        locals.var_tc_dn9 = assign82090_e124842_d_n9;
        locals.var_tc_dn10 = assign82090_e124842_d_n10;
        locals.var_tc_dn13 = assign82090_e124842_d_n13;
        locals.var_tc_rv = 0.0;

        let (assign82100_e124857, assign82100_e124857_d_n0, assign82100_e124857_d_n2, assign82100_e124857_d_n4, assign82100_e124857_d_n5, assign82100_e124857_d_n6, assign82100_e124857_d_n7, assign82100_e124857_d_n8, assign82100_e124857_d_n9, assign82100_e124857_d_n10, assign82100_e124857_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82100_e124852: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign82100_e124853: f64 = (-assign82100_e124852);
        let assign82100_e124855: f64 = (assign82100_e124853 / locals.var_fac1);
        (assign82100_e124855, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn9) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn13) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn13)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn13,)
    }
};
        locals.var_td = assign82100_e124857;
        locals.var_td_dn0 = assign82100_e124857_d_n0;
        locals.var_td_dn2 = assign82100_e124857_d_n2;
        locals.var_td_dn4 = assign82100_e124857_d_n4;
        locals.var_td_dn5 = assign82100_e124857_d_n5;
        locals.var_td_dn6 = assign82100_e124857_d_n6;
        locals.var_td_dn7 = assign82100_e124857_d_n7;
        locals.var_td_dn8 = assign82100_e124857_d_n8;
        locals.var_td_dn9 = assign82100_e124857_d_n9;
        locals.var_td_dn10 = assign82100_e124857_d_n10;
        locals.var_td_dn13 = assign82100_e124857_d_n13;
        locals.var_td_rv = 0.0;

        let (assign82110_e124895, assign82110_e124895_d_n0, assign82110_e124895_d_n2, assign82110_e124895_d_n4, assign82110_e124895_d_n5, assign82110_e124895_d_n6, assign82110_e124895_d_n7, assign82110_e124895_d_n8, assign82110_e124895_d_n9, assign82110_e124895_d_n10, assign82110_e124895_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82110_e124867: f64 = (locals.var_tb * locals.var_tb);
        let assign82110_e124869: f64 = (assign82110_e124867 * locals.var_tb);
        let assign82110_e124872: f64 = (27.0 * locals.var_ta);
        let assign82110_e124874: f64 = (assign82110_e124872 * locals.var_ta);
        let assign82110_e124876: f64 = (assign82110_e124874 * locals.var_ta);
        let assign82110_e124877: f64 = (assign82110_e124869 / assign82110_e124876);
        let assign82110_e124880: f64 = (locals.var_tb * locals.var_tc);
        let assign82110_e124883: f64 = (6.0 * locals.var_ta);
        let assign82110_e124885: f64 = (assign82110_e124883 * locals.var_ta);
        let assign82110_e124886: f64 = (assign82110_e124880 / assign82110_e124885);
        let assign82110_e124887: f64 = (assign82110_e124877 - assign82110_e124886);
        let assign82110_e124891: f64 = (2.0 * locals.var_ta);
        let assign82110_e124892: f64 = (locals.var_td / assign82110_e124891);
        let assign82110_e124893: f64 = (assign82110_e124887 + assign82110_e124892);
        (assign82110_e124893, ((-((locals.var_tb * locals.var_tc_dn0) / assign82110_e124885)) + (locals.var_td_dn0 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn2) / assign82110_e124885)) + (locals.var_td_dn2 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn4) / assign82110_e124885)) + (locals.var_td_dn4 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn5) / assign82110_e124885)) + (locals.var_td_dn5 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn6) / assign82110_e124885)) + (locals.var_td_dn6 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn7) / assign82110_e124885)) + (locals.var_td_dn7 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn8) / assign82110_e124885)) + (locals.var_td_dn8 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn9) / assign82110_e124885)) + (locals.var_td_dn9 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn10) / assign82110_e124885)) + (locals.var_td_dn10 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn13) / assign82110_e124885)) + (locals.var_td_dn13 / assign82110_e124891)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn13,)
    }
};
        locals.var_tq = assign82110_e124895;
        locals.var_tq_dn0 = assign82110_e124895_d_n0;
        locals.var_tq_dn2 = assign82110_e124895_d_n2;
        locals.var_tq_dn4 = assign82110_e124895_d_n4;
        locals.var_tq_dn5 = assign82110_e124895_d_n5;
        locals.var_tq_dn6 = assign82110_e124895_d_n6;
        locals.var_tq_dn7 = assign82110_e124895_d_n7;
        locals.var_tq_dn8 = assign82110_e124895_d_n8;
        locals.var_tq_dn9 = assign82110_e124895_d_n9;
        locals.var_tq_dn10 = assign82110_e124895_d_n10;
        locals.var_tq_dn13 = assign82110_e124895_d_n13;
        locals.var_tq_rv = 0.0;

        let (assign82120_e124919, assign82120_e124919_d_n0, assign82120_e124919_d_n2, assign82120_e124919_d_n4, assign82120_e124919_d_n5, assign82120_e124919_d_n6, assign82120_e124919_d_n7, assign82120_e124919_d_n8, assign82120_e124919_d_n9, assign82120_e124919_d_n10, assign82120_e124919_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82120_e124905: f64 = (3.0 * locals.var_ta);
        let assign82120_e124907: f64 = (assign82120_e124905 * locals.var_tc);
        let assign82120_e124910: f64 = (locals.var_tb * locals.var_tb);
        let assign82120_e124911: f64 = (assign82120_e124907 - assign82120_e124910);
        let assign82120_e124914: f64 = (9.0 * locals.var_ta);
        let assign82120_e124916: f64 = (assign82120_e124914 * locals.var_ta);
        let assign82120_e124917: f64 = (assign82120_e124911 / assign82120_e124916);
        (assign82120_e124917, ((assign82120_e124905 * locals.var_tc_dn0) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn2) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn4) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn5) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn6) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn7) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn8) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn9) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn10) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn13) / assign82120_e124916),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn13,)
    }
};
        locals.var_tp = assign82120_e124919;
        locals.var_tp_dn0 = assign82120_e124919_d_n0;
        locals.var_tp_dn2 = assign82120_e124919_d_n2;
        locals.var_tp_dn4 = assign82120_e124919_d_n4;
        locals.var_tp_dn5 = assign82120_e124919_d_n5;
        locals.var_tp_dn6 = assign82120_e124919_d_n6;
        locals.var_tp_dn7 = assign82120_e124919_d_n7;
        locals.var_tp_dn8 = assign82120_e124919_d_n8;
        locals.var_tp_dn9 = assign82120_e124919_d_n9;
        locals.var_tp_dn10 = assign82120_e124919_d_n10;
        locals.var_tp_dn13 = assign82120_e124919_d_n13;
        locals.var_tp_rv = 0.0;

        let (assign82130_e124938, assign82130_e124938_d_n0, assign82130_e124938_d_n2, assign82130_e124938_d_n4, assign82130_e124938_d_n5, assign82130_e124938_d_n6, assign82130_e124938_d_n7, assign82130_e124938_d_n8, assign82130_e124938_d_n9, assign82130_e124938_d_n10, assign82130_e124938_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82130_e124929: f64 = (locals.var_tq * locals.var_tq);
        let assign82130_e124932: f64 = (locals.var_tp * locals.var_tp);
        let assign82130_e124934: f64 = (assign82130_e124932 * locals.var_tp);
        let assign82130_e124935: f64 = (assign82130_e124929 + assign82130_e124934);
        let assign82130_e124936: f64 = (assign82130_e124935).sqrt();
        (assign82130_e124936, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn0))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn2))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn4))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn5))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn6))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn7))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn8))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn9))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn10))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn13 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn13)) + ((((locals.var_tp_dn13 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn13)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn13))) / (2.0 * assign82130_e124936)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign82130_e124938;
        locals.var_t5_dn0 = assign82130_e124938_d_n0;
        locals.var_t5_dn2 = assign82130_e124938_d_n2;
        locals.var_t5_dn4 = assign82130_e124938_d_n4;
        locals.var_t5_dn5 = assign82130_e124938_d_n5;
        locals.var_t5_dn6 = assign82130_e124938_d_n6;
        locals.var_t5_dn7 = assign82130_e124938_d_n7;
        locals.var_t5_dn8 = assign82130_e124938_d_n8;
        locals.var_t5_dn9 = assign82130_e124938_d_n9;
        locals.var_t5_dn10 = assign82130_e124938_d_n10;
        locals.var_t5_dn13 = assign82130_e124938_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign82140_e124953, assign82140_e124953_d_n0, assign82140_e124953_d_n2, assign82140_e124953_d_n4, assign82140_e124953_d_n5, assign82140_e124953_d_n6, assign82140_e124953_d_n7, assign82140_e124953_d_n8, assign82140_e124953_d_n9, assign82140_e124953_d_n10, assign82140_e124953_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82140_e124947: f64 = (-locals.var_tq);
        let assign82140_e124949: f64 = (assign82140_e124947 + locals.var_t5);
        let assign82140_e124951: f64 = (assign82140_e124949).powf(0.3333333333333333);
        (assign82140_e124951, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn13) + locals.var_t5_dn13))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn13) + locals.var_t5_dn13) / assign82140_e124949))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn13,)
    }
};
        locals.var_tu = assign82140_e124953;
        locals.var_tu_dn0 = assign82140_e124953_d_n0;
        locals.var_tu_dn2 = assign82140_e124953_d_n2;
        locals.var_tu_dn4 = assign82140_e124953_d_n4;
        locals.var_tu_dn5 = assign82140_e124953_d_n5;
        locals.var_tu_dn6 = assign82140_e124953_d_n6;
        locals.var_tu_dn7 = assign82140_e124953_d_n7;
        locals.var_tu_dn8 = assign82140_e124953_d_n8;
        locals.var_tu_dn9 = assign82140_e124953_d_n9;
        locals.var_tu_dn10 = assign82140_e124953_d_n10;
        locals.var_tu_dn13 = assign82140_e124953_d_n13;
        locals.var_tu_rv = 0.0;

        let (assign82150_e124968, assign82150_e124968_d_n0, assign82150_e124968_d_n2, assign82150_e124968_d_n4, assign82150_e124968_d_n5, assign82150_e124968_d_n6, assign82150_e124968_d_n7, assign82150_e124968_d_n8, assign82150_e124968_d_n9, assign82150_e124968_d_n10, assign82150_e124968_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82150_e124963: f64 = (locals.var_tq + locals.var_t5);
        let assign82150_e124965: f64 = (assign82150_e124963).powf(0.3333333333333333);
        let assign82150_e124966: f64 = (-assign82150_e124965);
        (assign82150_e124966, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn13 + locals.var_t5_dn13))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn13 + locals.var_t5_dn13) / assign82150_e124963))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn13,)
    }
};
        locals.var_tv = assign82150_e124968;
        locals.var_tv_dn0 = assign82150_e124968_d_n0;
        locals.var_tv_dn2 = assign82150_e124968_d_n2;
        locals.var_tv_dn4 = assign82150_e124968_d_n4;
        locals.var_tv_dn5 = assign82150_e124968_d_n5;
        locals.var_tv_dn6 = assign82150_e124968_d_n6;
        locals.var_tv_dn7 = assign82150_e124968_d_n7;
        locals.var_tv_dn8 = assign82150_e124968_d_n8;
        locals.var_tv_dn9 = assign82150_e124968_d_n9;
        locals.var_tv_dn10 = assign82150_e124968_d_n10;
        locals.var_tv_dn13 = assign82150_e124968_d_n13;
        locals.var_tv_rv = 0.0;

        let (assign82160_e124986, assign82160_e124986_d_n0, assign82160_e124986_d_n2, assign82160_e124986_d_n4, assign82160_e124986_d_n5, assign82160_e124986_d_n6, assign82160_e124986_d_n7, assign82160_e124986_d_n8, assign82160_e124986_d_n9, assign82160_e124986_d_n10, assign82160_e124986_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82160_e124978: f64 = (locals.var_tu + locals.var_tv);
        let assign82160_e124982: f64 = (3.0 * locals.var_ta);
        let assign82160_e124983: f64 = (locals.var_tb / assign82160_e124982);
        let assign82160_e124984: f64 = (assign82160_e124978 - assign82160_e124983);
        (assign82160_e124984, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn13 + locals.var_tv_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82160_e124986;
        locals.var_chi_dn0 = assign82160_e124986_d_n0;
        locals.var_chi_dn2 = assign82160_e124986_d_n2;
        locals.var_chi_dn4 = assign82160_e124986_d_n4;
        locals.var_chi_dn5 = assign82160_e124986_d_n5;
        locals.var_chi_dn6 = assign82160_e124986_d_n6;
        locals.var_chi_dn7 = assign82160_e124986_d_n7;
        locals.var_chi_dn8 = assign82160_e124986_d_n8;
        locals.var_chi_dn9 = assign82160_e124986_d_n9;
        locals.var_chi_dn10 = assign82160_e124986_d_n10;
        locals.var_chi_dn13 = assign82160_e124986_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign82170_e125000, assign82170_e125000_d_n0, assign82170_e125000_d_n2, assign82170_e125000_d_n4, assign82170_e125000_d_n5, assign82170_e125000_d_n6, assign82170_e125000_d_n7, assign82170_e125000_d_n8, assign82170_e125000_d_n9, assign82170_e125000_d_n10, assign82170_e125000_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82170_e124996: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign82170_e124998: f64 = (assign82170_e124996 - locals.var_vxbgmtcl);
        (assign82170_e124998, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign82170_e125000;
        locals.var_ps0_inia_dn0 = assign82170_e125000_d_n0;
        locals.var_ps0_inia_dn2 = assign82170_e125000_d_n2;
        locals.var_ps0_inia_dn4 = assign82170_e125000_d_n4;
        locals.var_ps0_inia_dn5 = assign82170_e125000_d_n5;
        locals.var_ps0_inia_dn6 = assign82170_e125000_d_n6;
        locals.var_ps0_inia_dn7 = assign82170_e125000_d_n7;
        locals.var_ps0_inia_dn8 = assign82170_e125000_d_n8;
        locals.var_ps0_inia_dn9 = assign82170_e125000_d_n9;
        locals.var_ps0_inia_dn10 = assign82170_e125000_d_n10;
        locals.var_ps0_inia_dn13 = assign82170_e125000_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let assign82180_e125003: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1911 = assign82180_e125003;
        locals.var_guard1911_rv = 0.0;

        let (assign82190_e125016, assign82190_e125016_d_n0, assign82190_e125016_d_n2, assign82190_e125016_d_n4, assign82190_e125016_d_n5, assign82190_e125016_d_n6, assign82190_e125016_d_n7, assign82190_e125016_d_n8, assign82190_e125016_d_n9, assign82190_e125016_d_n10, assign82190_e125016_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82190_e125012: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign82190_e125014: f64 = (assign82190_e125012 + 0.1);
        (assign82190_e125014, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn13,)
    }
};
        locals.var_vgpld_shift = assign82190_e125016;
        locals.var_vgpld_shift_dn0 = assign82190_e125016_d_n0;
        locals.var_vgpld_shift_dn2 = assign82190_e125016_d_n2;
        locals.var_vgpld_shift_dn4 = assign82190_e125016_d_n4;
        locals.var_vgpld_shift_dn5 = assign82190_e125016_d_n5;
        locals.var_vgpld_shift_dn6 = assign82190_e125016_d_n6;
        locals.var_vgpld_shift_dn7 = assign82190_e125016_d_n7;
        locals.var_vgpld_shift_dn8 = assign82190_e125016_d_n8;
        locals.var_vgpld_shift_dn9 = assign82190_e125016_d_n9;
        locals.var_vgpld_shift_dn10 = assign82190_e125016_d_n10;
        locals.var_vgpld_shift_dn13 = assign82190_e125016_d_n13;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign82200_e125027, assign82200_e125027_d_n0, assign82200_e125027_d_n2, assign82200_e125027_d_n4, assign82200_e125027_d_n5, assign82200_e125027_d_n6, assign82200_e125027_d_n7, assign82200_e125027_d_n8, assign82200_e125027_d_n9, assign82200_e125027_d_n10, assign82200_e125027_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82200_e125025: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign82200_e125025, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign82200_e125027;
        locals.var_cfs1_dn0 = assign82200_e125027_d_n0;
        locals.var_cfs1_dn2 = assign82200_e125027_d_n2;
        locals.var_cfs1_dn4 = assign82200_e125027_d_n4;
        locals.var_cfs1_dn5 = assign82200_e125027_d_n5;
        locals.var_cfs1_dn6 = assign82200_e125027_d_n6;
        locals.var_cfs1_dn7 = assign82200_e125027_d_n7;
        locals.var_cfs1_dn8 = assign82200_e125027_d_n8;
        locals.var_cfs1_dn9 = assign82200_e125027_d_n9;
        locals.var_cfs1_dn10 = assign82200_e125027_d_n10;
        locals.var_cfs1_dn13 = assign82200_e125027_d_n13;
        locals.var_cfs1_rv = 0.0;

        let (assign82210_e125038, assign82210_e125038_d_n0, assign82210_e125038_d_n2, assign82210_e125038_d_n4, assign82210_e125038_d_n5, assign82210_e125038_d_n6, assign82210_e125038_d_n7, assign82210_e125038_d_n8, assign82210_e125038_d_n9, assign82210_e125038_d_n10, assign82210_e125038_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82210_e125036: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign82210_e125036, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn13,)
    }
};
        locals.var_gammachi = assign82210_e125038;
        locals.var_gammachi_dn0 = assign82210_e125038_d_n0;
        locals.var_gammachi_dn2 = assign82210_e125038_d_n2;
        locals.var_gammachi_dn4 = assign82210_e125038_d_n4;
        locals.var_gammachi_dn5 = assign82210_e125038_d_n5;
        locals.var_gammachi_dn6 = assign82210_e125038_d_n6;
        locals.var_gammachi_dn7 = assign82210_e125038_d_n7;
        locals.var_gammachi_dn8 = assign82210_e125038_d_n8;
        locals.var_gammachi_dn9 = assign82210_e125038_d_n9;
        locals.var_gammachi_dn10 = assign82210_e125038_d_n10;
        locals.var_gammachi_dn13 = assign82210_e125038_d_n13;
        locals.var_gammachi_rv = 0.0;

        let (assign82220_e125049, assign82220_e125049_d_n0, assign82220_e125049_d_n2, assign82220_e125049_d_n4, assign82220_e125049_d_n5, assign82220_e125049_d_n6, assign82220_e125049_d_n7, assign82220_e125049_d_n8, assign82220_e125049_d_n9, assign82220_e125049_d_n10, assign82220_e125049_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82220_e125047: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign82220_e125047, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn13 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign82220_e125049;
        locals.var_t0_dn0 = assign82220_e125049_d_n0;
        locals.var_t0_dn2 = assign82220_e125049_d_n2;
        locals.var_t0_dn4 = assign82220_e125049_d_n4;
        locals.var_t0_dn5 = assign82220_e125049_d_n5;
        locals.var_t0_dn6 = assign82220_e125049_d_n6;
        locals.var_t0_dn7 = assign82220_e125049_d_n7;
        locals.var_t0_dn8 = assign82220_e125049_d_n8;
        locals.var_t0_dn9 = assign82220_e125049_d_n9;
        locals.var_t0_dn10 = assign82220_e125049_d_n10;
        locals.var_t0_dn13 = assign82220_e125049_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign82230_e125060, assign82230_e125060_d_n0, assign82230_e125060_d_n2, assign82230_e125060_d_n4, assign82230_e125060_d_n5, assign82230_e125060_d_n6, assign82230_e125060_d_n7, assign82230_e125060_d_n8, assign82230_e125060_d_n9, assign82230_e125060_d_n10, assign82230_e125060_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82230_e125058: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign82230_e125058, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn13 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn13)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign82230_e125060;
        locals.var_psi_dn0 = assign82230_e125060_d_n0;
        locals.var_psi_dn2 = assign82230_e125060_d_n2;
        locals.var_psi_dn4 = assign82230_e125060_d_n4;
        locals.var_psi_dn5 = assign82230_e125060_d_n5;
        locals.var_psi_dn6 = assign82230_e125060_d_n6;
        locals.var_psi_dn7 = assign82230_e125060_d_n7;
        locals.var_psi_dn8 = assign82230_e125060_d_n8;
        locals.var_psi_dn9 = assign82230_e125060_d_n9;
        locals.var_psi_dn10 = assign82230_e125060_d_n10;
        locals.var_psi_dn13 = assign82230_e125060_d_n13;
        locals.var_psi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_302(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82240_e125085, assign82240_e125085_d_n0, assign82240_e125085_d_n2, assign82240_e125085_d_n4, assign82240_e125085_d_n5, assign82240_e125085_d_n6, assign82240_e125085_d_n7, assign82240_e125085_d_n8, assign82240_e125085_d_n9, assign82240_e125085_d_n10, assign82240_e125085_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82240_e125069: f64 = (locals.var_gammachi * locals.var_t0);
        let assign82240_e125072: f64 = (locals.var_psi * locals.var_psi);
        let assign82240_e125073: f64 = (assign82240_e125069 + assign82240_e125072);
        let assign82240_e125074: f64 = (assign82240_e125073).ln();
        let assign82240_e125077: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign82240_e125078: f64 = (assign82240_e125077).ln();
        let assign82240_e125079: f64 = (assign82240_e125074 - assign82240_e125078);
        let assign82240_e125082: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign82240_e125083: f64 = (assign82240_e125079 + assign82240_e125082);
        (assign82240_e125083, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign82240_e125073) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign82240_e125077)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign82240_e125073) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign82240_e125077)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign82240_e125073) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign82240_e125077)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign82240_e125073) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign82240_e125077)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign82240_e125073) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign82240_e125077)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign82240_e125073) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign82240_e125077)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign82240_e125073) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign82240_e125077)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign82240_e125073) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign82240_e125077)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign82240_e125073) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign82240_e125077)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign82240_e125073) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign82240_e125077)) + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign82240_e125085;
        locals.var_chi_1_dn0 = assign82240_e125085_d_n0;
        locals.var_chi_1_dn2 = assign82240_e125085_d_n2;
        locals.var_chi_1_dn4 = assign82240_e125085_d_n4;
        locals.var_chi_1_dn5 = assign82240_e125085_d_n5;
        locals.var_chi_1_dn6 = assign82240_e125085_d_n6;
        locals.var_chi_1_dn7 = assign82240_e125085_d_n7;
        locals.var_chi_1_dn8 = assign82240_e125085_d_n8;
        locals.var_chi_1_dn9 = assign82240_e125085_d_n9;
        locals.var_chi_1_dn10 = assign82240_e125085_d_n10;
        locals.var_chi_1_dn13 = assign82240_e125085_d_n13;
        locals.var_chi_1_rv = 0.0;

        let assign82250_e125088: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1912 = assign82250_e125088;
        locals.var_guard1912_rv = 0.0;

        let (assign82260_e125103, assign82260_e125103_d_n0, assign82260_e125103_d_n2, assign82260_e125103_d_n4, assign82260_e125103_d_n5, assign82260_e125103_d_n6, assign82260_e125103_d_n7, assign82260_e125103_d_n8, assign82260_e125103_d_n9, assign82260_e125103_d_n10, assign82260_e125103_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82260_e125099: f64 = (locals.var_psi - locals.var_chi_1);
        let assign82260_e125101: f64 = (assign82260_e125099 - 1.0);
        (assign82260_e125101, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign82260_e125103;
        locals.var_tmf1_dn0 = assign82260_e125103_d_n0;
        locals.var_tmf1_dn2 = assign82260_e125103_d_n2;
        locals.var_tmf1_dn4 = assign82260_e125103_d_n4;
        locals.var_tmf1_dn5 = assign82260_e125103_d_n5;
        locals.var_tmf1_dn6 = assign82260_e125103_d_n6;
        locals.var_tmf1_dn7 = assign82260_e125103_d_n7;
        locals.var_tmf1_dn8 = assign82260_e125103_d_n8;
        locals.var_tmf1_dn9 = assign82260_e125103_d_n9;
        locals.var_tmf1_dn10 = assign82260_e125103_d_n10;
        locals.var_tmf1_dn13 = assign82260_e125103_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign82270_e125118, assign82270_e125118_d_n0, assign82270_e125118_d_n2, assign82270_e125118_d_n4, assign82270_e125118_d_n5, assign82270_e125118_d_n6, assign82270_e125118_d_n7, assign82270_e125118_d_n8, assign82270_e125118_d_n9, assign82270_e125118_d_n10, assign82270_e125118_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82270_e125114: f64 = (4.0 * locals.var_psi);
        let assign82270_e125116: f64 = assign82270_e125114;
        (assign82270_e125116, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn13),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign82270_e125118;
        locals.var_tmf2_dn0 = assign82270_e125118_d_n0;
        locals.var_tmf2_dn2 = assign82270_e125118_d_n2;
        locals.var_tmf2_dn4 = assign82270_e125118_d_n4;
        locals.var_tmf2_dn5 = assign82270_e125118_d_n5;
        locals.var_tmf2_dn6 = assign82270_e125118_d_n6;
        locals.var_tmf2_dn7 = assign82270_e125118_d_n7;
        locals.var_tmf2_dn8 = assign82270_e125118_d_n8;
        locals.var_tmf2_dn9 = assign82270_e125118_d_n9;
        locals.var_tmf2_dn10 = assign82270_e125118_d_n10;
        locals.var_tmf2_dn13 = assign82270_e125118_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign82280_e125135, assign82280_e125135_d_n0, assign82280_e125135_d_n2, assign82280_e125135_d_n4, assign82280_e125135_d_n5, assign82280_e125135_d_n6, assign82280_e125135_d_n7, assign82280_e125135_d_n8, assign82280_e125135_d_n9, assign82280_e125135_d_n10, assign82280_e125135_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let (assign82280_e125133, assign82280_e125133_d_n0, assign82280_e125133_d_n2, assign82280_e125133_d_n4, assign82280_e125133_d_n5, assign82280_e125133_d_n6, assign82280_e125133_d_n7, assign82280_e125133_d_n8, assign82280_e125133_d_n9, assign82280_e125133_d_n10, assign82280_e125133_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign82280_e125132: f64 = (-locals.var_tmf2);
                (assign82280_e125132, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign82280_e125133, assign82280_e125133_d_n0, assign82280_e125133_d_n2, assign82280_e125133_d_n4, assign82280_e125133_d_n5, assign82280_e125133_d_n6, assign82280_e125133_d_n7, assign82280_e125133_d_n8, assign82280_e125133_d_n9, assign82280_e125133_d_n10, assign82280_e125133_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign82280_e125135;
        locals.var_tmf2_dn0 = assign82280_e125135_d_n0;
        locals.var_tmf2_dn2 = assign82280_e125135_d_n2;
        locals.var_tmf2_dn4 = assign82280_e125135_d_n4;
        locals.var_tmf2_dn5 = assign82280_e125135_d_n5;
        locals.var_tmf2_dn6 = assign82280_e125135_d_n6;
        locals.var_tmf2_dn7 = assign82280_e125135_d_n7;
        locals.var_tmf2_dn8 = assign82280_e125135_d_n8;
        locals.var_tmf2_dn9 = assign82280_e125135_d_n9;
        locals.var_tmf2_dn10 = assign82280_e125135_d_n10;
        locals.var_tmf2_dn13 = assign82280_e125135_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign82290_e125151, assign82290_e125151_d_n0, assign82290_e125151_d_n2, assign82290_e125151_d_n4, assign82290_e125151_d_n5, assign82290_e125151_d_n6, assign82290_e125151_d_n7, assign82290_e125151_d_n8, assign82290_e125151_d_n9, assign82290_e125151_d_n10, assign82290_e125151_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82290_e125146: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign82290_e125148: f64 = (assign82290_e125146 + locals.var_tmf2);
        let assign82290_e125149: f64 = (assign82290_e125148).sqrt();
        (assign82290_e125149, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign82290_e125149)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign82290_e125151;
        locals.var_tmf2_dn0 = assign82290_e125151_d_n0;
        locals.var_tmf2_dn2 = assign82290_e125151_d_n2;
        locals.var_tmf2_dn4 = assign82290_e125151_d_n4;
        locals.var_tmf2_dn5 = assign82290_e125151_d_n5;
        locals.var_tmf2_dn6 = assign82290_e125151_d_n6;
        locals.var_tmf2_dn7 = assign82290_e125151_d_n7;
        locals.var_tmf2_dn8 = assign82290_e125151_d_n8;
        locals.var_tmf2_dn9 = assign82290_e125151_d_n9;
        locals.var_tmf2_dn10 = assign82290_e125151_d_n10;
        locals.var_tmf2_dn13 = assign82290_e125151_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign82300_e125168, assign82300_e125168_d_n0, assign82300_e125168_d_n2, assign82300_e125168_d_n4, assign82300_e125168_d_n5, assign82300_e125168_d_n6, assign82300_e125168_d_n7, assign82300_e125168_d_n8, assign82300_e125168_d_n9, assign82300_e125168_d_n10, assign82300_e125168_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82300_e125164: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign82300_e125165: f64 = (1.0 + assign82300_e125164);
        let assign82300_e125166: f64 = (0.5 * assign82300_e125165);
        (assign82300_e125166, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82300_e125168;
        locals.var_t1_dn0 = assign82300_e125168_d_n0;
        locals.var_t1_dn2 = assign82300_e125168_d_n2;
        locals.var_t1_dn4 = assign82300_e125168_d_n4;
        locals.var_t1_dn5 = assign82300_e125168_d_n5;
        locals.var_t1_dn6 = assign82300_e125168_d_n6;
        locals.var_t1_dn7 = assign82300_e125168_d_n7;
        locals.var_t1_dn8 = assign82300_e125168_d_n8;
        locals.var_t1_dn9 = assign82300_e125168_d_n9;
        locals.var_t1_dn10 = assign82300_e125168_d_n10;
        locals.var_t1_dn13 = assign82300_e125168_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign82310_e125185, assign82310_e125185_d_n0, assign82310_e125185_d_n2, assign82310_e125185_d_n4, assign82310_e125185_d_n5, assign82310_e125185_d_n6, assign82310_e125185_d_n7, assign82310_e125185_d_n8, assign82310_e125185_d_n9, assign82310_e125185_d_n10, assign82310_e125185_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82310_e125181: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign82310_e125182: f64 = (0.5 * assign82310_e125181);
        let assign82310_e125183: f64 = (locals.var_psi - assign82310_e125182);
        (assign82310_e125183, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign82310_e125185;
        locals.var_chi_1_dn0 = assign82310_e125185_d_n0;
        locals.var_chi_1_dn2 = assign82310_e125185_d_n2;
        locals.var_chi_1_dn4 = assign82310_e125185_d_n4;
        locals.var_chi_1_dn5 = assign82310_e125185_d_n5;
        locals.var_chi_1_dn6 = assign82310_e125185_d_n6;
        locals.var_chi_1_dn7 = assign82310_e125185_d_n7;
        locals.var_chi_1_dn8 = assign82310_e125185_d_n8;
        locals.var_chi_1_dn9 = assign82310_e125185_d_n9;
        locals.var_chi_1_dn10 = assign82310_e125185_d_n10;
        locals.var_chi_1_dn13 = assign82310_e125185_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign82320_e125202, assign82320_e125202_d_n0, assign82320_e125202_d_n2, assign82320_e125202_d_n4, assign82320_e125202_d_n5, assign82320_e125202_d_n6, assign82320_e125202_d_n7, assign82320_e125202_d_n8, assign82320_e125202_d_n9, assign82320_e125202_d_n10, assign82320_e125202_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 == 0.0)) {
        let (assign82320_e125200, assign82320_e125200_d_n0, assign82320_e125200_d_n2, assign82320_e125200_d_n4, assign82320_e125200_d_n5, assign82320_e125200_d_n6, assign82320_e125200_d_n7, assign82320_e125200_d_n8, assign82320_e125200_d_n9, assign82320_e125200_d_n10, assign82320_e125200_d_n13,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
            }
        };
        (assign82320_e125200, assign82320_e125200_d_n0, assign82320_e125200_d_n2, assign82320_e125200_d_n4, assign82320_e125200_d_n5, assign82320_e125200_d_n6, assign82320_e125200_d_n7, assign82320_e125200_d_n8, assign82320_e125200_d_n9, assign82320_e125200_d_n10, assign82320_e125200_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign82320_e125202;
        locals.var_chi_1_dn0 = assign82320_e125202_d_n0;
        locals.var_chi_1_dn2 = assign82320_e125202_d_n2;
        locals.var_chi_1_dn4 = assign82320_e125202_d_n4;
        locals.var_chi_1_dn5 = assign82320_e125202_d_n5;
        locals.var_chi_1_dn6 = assign82320_e125202_d_n6;
        locals.var_chi_1_dn7 = assign82320_e125202_d_n7;
        locals.var_chi_1_dn8 = assign82320_e125202_d_n8;
        locals.var_chi_1_dn9 = assign82320_e125202_d_n9;
        locals.var_chi_1_dn10 = assign82320_e125202_d_n10;
        locals.var_chi_1_dn13 = assign82320_e125202_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign82330_e125216, assign82330_e125216_d_n0, assign82330_e125216_d_n2, assign82330_e125216_d_n4, assign82330_e125216_d_n5, assign82330_e125216_d_n6, assign82330_e125216_d_n7, assign82330_e125216_d_n8, assign82330_e125216_d_n9, assign82330_e125216_d_n10, assign82330_e125216_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let (assign82330_e125214, assign82330_e125214_d_n0, assign82330_e125214_d_n2, assign82330_e125214_d_n4, assign82330_e125214_d_n5, assign82330_e125214_d_n6, assign82330_e125214_d_n7, assign82330_e125214_d_n8, assign82330_e125214_d_n9, assign82330_e125214_d_n10, assign82330_e125214_d_n13,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign82330_e125214, assign82330_e125214_d_n0, assign82330_e125214_d_n2, assign82330_e125214_d_n4, assign82330_e125214_d_n5, assign82330_e125214_d_n6, assign82330_e125214_d_n7, assign82330_e125214_d_n8, assign82330_e125214_d_n9, assign82330_e125214_d_n10, assign82330_e125214_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign82330_e125216;
        locals.var_chi_1_dn0 = assign82330_e125216_d_n0;
        locals.var_chi_1_dn2 = assign82330_e125216_d_n2;
        locals.var_chi_1_dn4 = assign82330_e125216_d_n4;
        locals.var_chi_1_dn5 = assign82330_e125216_d_n5;
        locals.var_chi_1_dn6 = assign82330_e125216_d_n6;
        locals.var_chi_1_dn7 = assign82330_e125216_d_n7;
        locals.var_chi_1_dn8 = assign82330_e125216_d_n8;
        locals.var_chi_1_dn9 = assign82330_e125216_d_n9;
        locals.var_chi_1_dn10 = assign82330_e125216_d_n10;
        locals.var_chi_1_dn13 = assign82330_e125216_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign82340_e125227, assign82340_e125227_d_n0, assign82340_e125227_d_n2, assign82340_e125227_d_n4, assign82340_e125227_d_n5, assign82340_e125227_d_n6, assign82340_e125227_d_n7, assign82340_e125227_d_n8, assign82340_e125227_d_n9, assign82340_e125227_d_n10, assign82340_e125227_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82340_e125225: f64 = (locals.var_psi - locals.var_chi_1);
        (assign82340_e125225, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign82340_e125227;
        locals.var_psi_dn0 = assign82340_e125227_d_n0;
        locals.var_psi_dn2 = assign82340_e125227_d_n2;
        locals.var_psi_dn4 = assign82340_e125227_d_n4;
        locals.var_psi_dn5 = assign82340_e125227_d_n5;
        locals.var_psi_dn6 = assign82340_e125227_d_n6;
        locals.var_psi_dn7 = assign82340_e125227_d_n7;
        locals.var_psi_dn8 = assign82340_e125227_d_n8;
        locals.var_psi_dn9 = assign82340_e125227_d_n9;
        locals.var_psi_dn10 = assign82340_e125227_d_n10;
        locals.var_psi_dn13 = assign82340_e125227_d_n13;
        locals.var_psi_rv = 0.0;

        let (assign82350_e125240, assign82350_e125240_d_n0, assign82350_e125240_d_n2, assign82350_e125240_d_n4, assign82350_e125240_d_n5, assign82350_e125240_d_n6, assign82350_e125240_d_n7, assign82350_e125240_d_n8, assign82350_e125240_d_n9, assign82350_e125240_d_n10, assign82350_e125240_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82350_e125237: f64 = (locals.var_beta * 0.1);
        let assign82350_e125238: f64 = (locals.var_psi + assign82350_e125237);
        (assign82350_e125238, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn13 + (locals.var_beta_dn13 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign82350_e125240;
        locals.var_psi_dn0 = assign82350_e125240_d_n0;
        locals.var_psi_dn2 = assign82350_e125240_d_n2;
        locals.var_psi_dn4 = assign82350_e125240_d_n4;
        locals.var_psi_dn5 = assign82350_e125240_d_n5;
        locals.var_psi_dn6 = assign82350_e125240_d_n6;
        locals.var_psi_dn7 = assign82350_e125240_d_n7;
        locals.var_psi_dn8 = assign82350_e125240_d_n8;
        locals.var_psi_dn9 = assign82350_e125240_d_n9;
        locals.var_psi_dn10 = assign82350_e125240_d_n10;
        locals.var_psi_dn13 = assign82350_e125240_d_n13;
        locals.var_psi_rv = 0.0;

        let (assign82360_e125261, assign82360_e125261_d_n0, assign82360_e125261_d_n2, assign82360_e125261_d_n4, assign82360_e125261_d_n5, assign82360_e125261_d_n6, assign82360_e125261_d_n7, assign82360_e125261_d_n8, assign82360_e125261_d_n9, assign82360_e125261_d_n10, assign82360_e125261_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82360_e125249: f64 = (locals.var_gammachi * locals.var_t0);
        let assign82360_e125252: f64 = (locals.var_psi * locals.var_psi);
        let assign82360_e125253: f64 = (assign82360_e125249 + assign82360_e125252);
        let assign82360_e125254: f64 = (assign82360_e125253).ln();
        let assign82360_e125257: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign82360_e125258: f64 = (assign82360_e125257).ln();
        let assign82360_e125259: f64 = (assign82360_e125254 - assign82360_e125258);
        (assign82360_e125259, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign82360_e125253) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign82360_e125257)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign82360_e125253) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign82360_e125257)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign82360_e125253) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign82360_e125257)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign82360_e125253) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign82360_e125257)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign82360_e125253) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign82360_e125257)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign82360_e125253) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign82360_e125257)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign82360_e125253) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign82360_e125257)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign82360_e125253) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign82360_e125257)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign82360_e125253) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign82360_e125257)), (((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign82360_e125253) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign82360_e125257)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82360_e125261;
        locals.var_t1_dn0 = assign82360_e125261_d_n0;
        locals.var_t1_dn2 = assign82360_e125261_d_n2;
        locals.var_t1_dn4 = assign82360_e125261_d_n4;
        locals.var_t1_dn5 = assign82360_e125261_d_n5;
        locals.var_t1_dn6 = assign82360_e125261_d_n6;
        locals.var_t1_dn7 = assign82360_e125261_d_n7;
        locals.var_t1_dn8 = assign82360_e125261_d_n8;
        locals.var_t1_dn9 = assign82360_e125261_d_n9;
        locals.var_t1_dn10 = assign82360_e125261_d_n10;
        locals.var_t1_dn13 = assign82360_e125261_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign82370_e125274, assign82370_e125274_d_n0, assign82370_e125274_d_n2, assign82370_e125274_d_n4, assign82370_e125274_d_n5, assign82370_e125274_d_n6, assign82370_e125274_d_n7, assign82370_e125274_d_n8, assign82370_e125274_d_n9, assign82370_e125274_d_n10, assign82370_e125274_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82370_e125271: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign82370_e125272: f64 = (locals.var_t1 + assign82370_e125271);
        (assign82370_e125272, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn13 + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign82370_e125274;
        locals.var_chi_b_dn0 = assign82370_e125274_d_n0;
        locals.var_chi_b_dn2 = assign82370_e125274_d_n2;
        locals.var_chi_b_dn4 = assign82370_e125274_d_n4;
        locals.var_chi_b_dn5 = assign82370_e125274_d_n5;
        locals.var_chi_b_dn6 = assign82370_e125274_d_n6;
        locals.var_chi_b_dn7 = assign82370_e125274_d_n7;
        locals.var_chi_b_dn8 = assign82370_e125274_d_n8;
        locals.var_chi_b_dn9 = assign82370_e125274_d_n9;
        locals.var_chi_b_dn10 = assign82370_e125274_d_n10;
        locals.var_chi_b_dn13 = assign82370_e125274_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign82380_e125288, assign82380_e125288_d_n0, assign82380_e125288_d_n2, assign82380_e125288_d_n4, assign82380_e125288_d_n5, assign82380_e125288_d_n6, assign82380_e125288_d_n7, assign82380_e125288_d_n8, assign82380_e125288_d_n9, assign82380_e125288_d_n10, assign82380_e125288_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let (assign82380_e125286, assign82380_e125286_d_n0, assign82380_e125286_d_n2, assign82380_e125286_d_n4, assign82380_e125286_d_n5, assign82380_e125286_d_n6, assign82380_e125286_d_n7, assign82380_e125286_d_n8, assign82380_e125286_d_n9, assign82380_e125286_d_n10, assign82380_e125286_d_n13,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign82380_e125286, assign82380_e125286_d_n0, assign82380_e125286_d_n2, assign82380_e125286_d_n4, assign82380_e125286_d_n5, assign82380_e125286_d_n6, assign82380_e125286_d_n7, assign82380_e125286_d_n8, assign82380_e125286_d_n9, assign82380_e125286_d_n10, assign82380_e125286_d_n13,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign82380_e125288;
        locals.var_chi_b_dn0 = assign82380_e125288_d_n0;
        locals.var_chi_b_dn2 = assign82380_e125288_d_n2;
        locals.var_chi_b_dn4 = assign82380_e125288_d_n4;
        locals.var_chi_b_dn5 = assign82380_e125288_d_n5;
        locals.var_chi_b_dn6 = assign82380_e125288_d_n6;
        locals.var_chi_b_dn7 = assign82380_e125288_d_n7;
        locals.var_chi_b_dn8 = assign82380_e125288_d_n8;
        locals.var_chi_b_dn9 = assign82380_e125288_d_n9;
        locals.var_chi_b_dn10 = assign82380_e125288_d_n10;
        locals.var_chi_b_dn13 = assign82380_e125288_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign82390_e125297, assign82390_e125297_d_n0, assign82390_e125297_d_n2, assign82390_e125297_d_n4, assign82390_e125297_d_n5, assign82390_e125297_d_n6, assign82390_e125297_d_n7, assign82390_e125297_d_n8, assign82390_e125297_d_n9, assign82390_e125297_d_n10, assign82390_e125297_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign82390_e125297;
        locals.var_chi_a_dn0 = assign82390_e125297_d_n0;
        locals.var_chi_a_dn2 = assign82390_e125297_d_n2;
        locals.var_chi_a_dn4 = assign82390_e125297_d_n4;
        locals.var_chi_a_dn5 = assign82390_e125297_d_n5;
        locals.var_chi_a_dn6 = assign82390_e125297_d_n6;
        locals.var_chi_a_dn7 = assign82390_e125297_d_n7;
        locals.var_chi_a_dn8 = assign82390_e125297_d_n8;
        locals.var_chi_a_dn9 = assign82390_e125297_d_n9;
        locals.var_chi_a_dn10 = assign82390_e125297_d_n10;
        locals.var_chi_a_dn13 = assign82390_e125297_d_n13;
        locals.var_chi_a_rv = 0.0;

        let assign82400_e125300: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1913 = assign82400_e125300;
        locals.var_guard1913_rv = 0.0;

        let assign82410_e125305: f64 = (0.2 * locals.var_chi_b);
        let assign82410_e125306: f64 = (locals.var_chi_b - assign82410_e125305);
        let assign82410_e125310: f64 = (0.2 * locals.var_chi_b);
        let assign82410_e125313: f64 = if ((locals.var_chi_a > assign82410_e125306) && (assign82410_e125310 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1914 = assign82410_e125313;
        locals.var_guard1914_rv = 0.0;

        let (assign82420_e125332, assign82420_e125332_d_n0, assign82420_e125332_d_n2, assign82420_e125332_d_n4, assign82420_e125332_d_n5, assign82420_e125332_d_n6, assign82420_e125332_d_n7, assign82420_e125332_d_n8, assign82420_e125332_d_n9, assign82420_e125332_d_n10, assign82420_e125332_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82420_e125326: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign82420_e125329: f64 = (0.2 * locals.var_chi_b);
        let assign82420_e125330: f64 = (assign82420_e125326 + assign82420_e125329);
        (assign82420_e125330, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn13 - locals.var_chi_b_dn13) + (0.2 * locals.var_chi_b_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign82420_e125332;
        locals.var_tmf1_dn0 = assign82420_e125332_d_n0;
        locals.var_tmf1_dn2 = assign82420_e125332_d_n2;
        locals.var_tmf1_dn4 = assign82420_e125332_d_n4;
        locals.var_tmf1_dn5 = assign82420_e125332_d_n5;
        locals.var_tmf1_dn6 = assign82420_e125332_d_n6;
        locals.var_tmf1_dn7 = assign82420_e125332_d_n7;
        locals.var_tmf1_dn8 = assign82420_e125332_d_n8;
        locals.var_tmf1_dn9 = assign82420_e125332_d_n9;
        locals.var_tmf1_dn10 = assign82420_e125332_d_n10;
        locals.var_tmf1_dn13 = assign82420_e125332_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign82430_e125347, assign82430_e125347_d_n0, assign82430_e125347_d_n2, assign82430_e125347_d_n4, assign82430_e125347_d_n5, assign82430_e125347_d_n6, assign82430_e125347_d_n7, assign82430_e125347_d_n8, assign82430_e125347_d_n9, assign82430_e125347_d_n10, assign82430_e125347_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82430_e125345: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign82430_e125345, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign82430_e125347;
        locals.var_x2_dn0 = assign82430_e125347_d_n0;
        locals.var_x2_dn2 = assign82430_e125347_d_n2;
        locals.var_x2_dn4 = assign82430_e125347_d_n4;
        locals.var_x2_dn5 = assign82430_e125347_d_n5;
        locals.var_x2_dn6 = assign82430_e125347_d_n6;
        locals.var_x2_dn7 = assign82430_e125347_d_n7;
        locals.var_x2_dn8 = assign82430_e125347_d_n8;
        locals.var_x2_dn9 = assign82430_e125347_d_n9;
        locals.var_x2_dn10 = assign82430_e125347_d_n10;
        locals.var_x2_dn13 = assign82430_e125347_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign82440_e125366, assign82440_e125366_d_n0, assign82440_e125366_d_n2, assign82440_e125366_d_n4, assign82440_e125366_d_n5, assign82440_e125366_d_n6, assign82440_e125366_d_n7, assign82440_e125366_d_n8, assign82440_e125366_d_n9, assign82440_e125366_d_n10, assign82440_e125366_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82440_e125360: f64 = (0.2 * locals.var_chi_b);
        let assign82440_e125363: f64 = (0.2 * locals.var_chi_b);
        let assign82440_e125364: f64 = (assign82440_e125360 * assign82440_e125363);
        (assign82440_e125364, (((0.2 * locals.var_chi_b_dn0) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn13) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn13))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign82440_e125366;
        locals.var_xmax2_dn0 = assign82440_e125366_d_n0;
        locals.var_xmax2_dn2 = assign82440_e125366_d_n2;
        locals.var_xmax2_dn4 = assign82440_e125366_d_n4;
        locals.var_xmax2_dn5 = assign82440_e125366_d_n5;
        locals.var_xmax2_dn6 = assign82440_e125366_d_n6;
        locals.var_xmax2_dn7 = assign82440_e125366_d_n7;
        locals.var_xmax2_dn8 = assign82440_e125366_d_n8;
        locals.var_xmax2_dn9 = assign82440_e125366_d_n9;
        locals.var_xmax2_dn10 = assign82440_e125366_d_n10;
        locals.var_xmax2_dn13 = assign82440_e125366_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign82450_e125379, assign82450_e125379_d_n0, assign82450_e125379_d_n2, assign82450_e125379_d_n4, assign82450_e125379_d_n5, assign82450_e125379_d_n6, assign82450_e125379_d_n7, assign82450_e125379_d_n8, assign82450_e125379_d_n9, assign82450_e125379_d_n10, assign82450_e125379_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign82450_e125379;
        locals.var_xp_dn0 = assign82450_e125379_d_n0;
        locals.var_xp_dn2 = assign82450_e125379_d_n2;
        locals.var_xp_dn4 = assign82450_e125379_d_n4;
        locals.var_xp_dn5 = assign82450_e125379_d_n5;
        locals.var_xp_dn6 = assign82450_e125379_d_n6;
        locals.var_xp_dn7 = assign82450_e125379_d_n7;
        locals.var_xp_dn8 = assign82450_e125379_d_n8;
        locals.var_xp_dn9 = assign82450_e125379_d_n9;
        locals.var_xp_dn10 = assign82450_e125379_d_n10;
        locals.var_xp_dn13 = assign82450_e125379_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign82460_e125392, assign82460_e125392_d_n0, assign82460_e125392_d_n2, assign82460_e125392_d_n4, assign82460_e125392_d_n5, assign82460_e125392_d_n6, assign82460_e125392_d_n7, assign82460_e125392_d_n8, assign82460_e125392_d_n9, assign82460_e125392_d_n10, assign82460_e125392_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign82460_e125392;
        locals.var_xmp_dn0 = assign82460_e125392_d_n0;
        locals.var_xmp_dn2 = assign82460_e125392_d_n2;
        locals.var_xmp_dn4 = assign82460_e125392_d_n4;
        locals.var_xmp_dn5 = assign82460_e125392_d_n5;
        locals.var_xmp_dn6 = assign82460_e125392_d_n6;
        locals.var_xmp_dn7 = assign82460_e125392_d_n7;
        locals.var_xmp_dn8 = assign82460_e125392_d_n8;
        locals.var_xmp_dn9 = assign82460_e125392_d_n9;
        locals.var_xmp_dn10 = assign82460_e125392_d_n10;
        locals.var_xmp_dn13 = assign82460_e125392_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign82470_e125405,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign82470_e125405;
        locals.var_m0_rv = 0.0;

        let (assign82480_e125418,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82480_e125418;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_303(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82490_e125431, assign82490_e125431_d_n0, assign82490_e125431_d_n2, assign82490_e125431_d_n4, assign82490_e125431_d_n5, assign82490_e125431_d_n6, assign82490_e125431_d_n7, assign82490_e125431_d_n8, assign82490_e125431_d_n9, assign82490_e125431_d_n10, assign82490_e125431_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign82490_e125431;
        locals.var_arg_dn0 = assign82490_e125431_d_n0;
        locals.var_arg_dn2 = assign82490_e125431_d_n2;
        locals.var_arg_dn4 = assign82490_e125431_d_n4;
        locals.var_arg_dn5 = assign82490_e125431_d_n5;
        locals.var_arg_dn6 = assign82490_e125431_d_n6;
        locals.var_arg_dn7 = assign82490_e125431_d_n7;
        locals.var_arg_dn8 = assign82490_e125431_d_n8;
        locals.var_arg_dn9 = assign82490_e125431_d_n9;
        locals.var_arg_dn10 = assign82490_e125431_d_n10;
        locals.var_arg_dn13 = assign82490_e125431_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign82500_e125444, assign82500_e125444_d_n0, assign82500_e125444_d_n2, assign82500_e125444_d_n4, assign82500_e125444_d_n5, assign82500_e125444_d_n6, assign82500_e125444_d_n7, assign82500_e125444_d_n8, assign82500_e125444_d_n9, assign82500_e125444_d_n10, assign82500_e125444_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign82500_e125444;
        locals.var_dnm_dn0 = assign82500_e125444_d_n0;
        locals.var_dnm_dn2 = assign82500_e125444_d_n2;
        locals.var_dnm_dn4 = assign82500_e125444_d_n4;
        locals.var_dnm_dn5 = assign82500_e125444_d_n5;
        locals.var_dnm_dn6 = assign82500_e125444_d_n6;
        locals.var_dnm_dn7 = assign82500_e125444_d_n7;
        locals.var_dnm_dn8 = assign82500_e125444_d_n8;
        locals.var_dnm_dn9 = assign82500_e125444_d_n9;
        locals.var_dnm_dn10 = assign82500_e125444_d_n10;
        locals.var_dnm_dn13 = assign82500_e125444_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign82510_e125459, assign82510_e125459_d_n0, assign82510_e125459_d_n2, assign82510_e125459_d_n4, assign82510_e125459_d_n5, assign82510_e125459_d_n6, assign82510_e125459_d_n7, assign82510_e125459_d_n8, assign82510_e125459_d_n9, assign82510_e125459_d_n10, assign82510_e125459_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82510_e125457: f64 = (locals.var_xp * locals.var_x2);
        (assign82510_e125457, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign82510_e125459;
        locals.var_xp_dn0 = assign82510_e125459_d_n0;
        locals.var_xp_dn2 = assign82510_e125459_d_n2;
        locals.var_xp_dn4 = assign82510_e125459_d_n4;
        locals.var_xp_dn5 = assign82510_e125459_d_n5;
        locals.var_xp_dn6 = assign82510_e125459_d_n6;
        locals.var_xp_dn7 = assign82510_e125459_d_n7;
        locals.var_xp_dn8 = assign82510_e125459_d_n8;
        locals.var_xp_dn9 = assign82510_e125459_d_n9;
        locals.var_xp_dn10 = assign82510_e125459_d_n10;
        locals.var_xp_dn13 = assign82510_e125459_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign82520_e125474, assign82520_e125474_d_n0, assign82520_e125474_d_n2, assign82520_e125474_d_n4, assign82520_e125474_d_n5, assign82520_e125474_d_n6, assign82520_e125474_d_n7, assign82520_e125474_d_n8, assign82520_e125474_d_n9, assign82520_e125474_d_n10, assign82520_e125474_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82520_e125472: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign82520_e125472, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign82520_e125474;
        locals.var_xmp_dn0 = assign82520_e125474_d_n0;
        locals.var_xmp_dn2 = assign82520_e125474_d_n2;
        locals.var_xmp_dn4 = assign82520_e125474_d_n4;
        locals.var_xmp_dn5 = assign82520_e125474_d_n5;
        locals.var_xmp_dn6 = assign82520_e125474_d_n6;
        locals.var_xmp_dn7 = assign82520_e125474_d_n7;
        locals.var_xmp_dn8 = assign82520_e125474_d_n8;
        locals.var_xmp_dn9 = assign82520_e125474_d_n9;
        locals.var_xmp_dn10 = assign82520_e125474_d_n10;
        locals.var_xmp_dn13 = assign82520_e125474_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign82530_e125489, assign82530_e125489_d_n0, assign82530_e125489_d_n2, assign82530_e125489_d_n4, assign82530_e125489_d_n5, assign82530_e125489_d_n6, assign82530_e125489_d_n7, assign82530_e125489_d_n8, assign82530_e125489_d_n9, assign82530_e125489_d_n10, assign82530_e125489_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82530_e125487: f64 = (locals.var_xp * locals.var_x2);
        (assign82530_e125487, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign82530_e125489;
        locals.var_xp_dn0 = assign82530_e125489_d_n0;
        locals.var_xp_dn2 = assign82530_e125489_d_n2;
        locals.var_xp_dn4 = assign82530_e125489_d_n4;
        locals.var_xp_dn5 = assign82530_e125489_d_n5;
        locals.var_xp_dn6 = assign82530_e125489_d_n6;
        locals.var_xp_dn7 = assign82530_e125489_d_n7;
        locals.var_xp_dn8 = assign82530_e125489_d_n8;
        locals.var_xp_dn9 = assign82530_e125489_d_n9;
        locals.var_xp_dn10 = assign82530_e125489_d_n10;
        locals.var_xp_dn13 = assign82530_e125489_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign82540_e125504, assign82540_e125504_d_n0, assign82540_e125504_d_n2, assign82540_e125504_d_n4, assign82540_e125504_d_n5, assign82540_e125504_d_n6, assign82540_e125504_d_n7, assign82540_e125504_d_n8, assign82540_e125504_d_n9, assign82540_e125504_d_n10, assign82540_e125504_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82540_e125502: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign82540_e125502, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign82540_e125504;
        locals.var_xmp_dn0 = assign82540_e125504_d_n0;
        locals.var_xmp_dn2 = assign82540_e125504_d_n2;
        locals.var_xmp_dn4 = assign82540_e125504_d_n4;
        locals.var_xmp_dn5 = assign82540_e125504_d_n5;
        locals.var_xmp_dn6 = assign82540_e125504_d_n6;
        locals.var_xmp_dn7 = assign82540_e125504_d_n7;
        locals.var_xmp_dn8 = assign82540_e125504_d_n8;
        locals.var_xmp_dn9 = assign82540_e125504_d_n9;
        locals.var_xmp_dn10 = assign82540_e125504_d_n10;
        locals.var_xmp_dn13 = assign82540_e125504_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign82550_e125519, assign82550_e125519_d_n0, assign82550_e125519_d_n2, assign82550_e125519_d_n4, assign82550_e125519_d_n5, assign82550_e125519_d_n6, assign82550_e125519_d_n7, assign82550_e125519_d_n8, assign82550_e125519_d_n9, assign82550_e125519_d_n10, assign82550_e125519_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82550_e125517: f64 = (locals.var_xp + locals.var_xmp);
        (assign82550_e125517, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign82550_e125519;
        locals.var_arg_dn0 = assign82550_e125519_d_n0;
        locals.var_arg_dn2 = assign82550_e125519_d_n2;
        locals.var_arg_dn4 = assign82550_e125519_d_n4;
        locals.var_arg_dn5 = assign82550_e125519_d_n5;
        locals.var_arg_dn6 = assign82550_e125519_d_n6;
        locals.var_arg_dn7 = assign82550_e125519_d_n7;
        locals.var_arg_dn8 = assign82550_e125519_d_n8;
        locals.var_arg_dn9 = assign82550_e125519_d_n9;
        locals.var_arg_dn10 = assign82550_e125519_d_n10;
        locals.var_arg_dn13 = assign82550_e125519_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign82560_e125532, assign82560_e125532_d_n0, assign82560_e125532_d_n2, assign82560_e125532_d_n4, assign82560_e125532_d_n5, assign82560_e125532_d_n6, assign82560_e125532_d_n7, assign82560_e125532_d_n8, assign82560_e125532_d_n9, assign82560_e125532_d_n10, assign82560_e125532_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign82560_e125532;
        locals.var_dnm_dn0 = assign82560_e125532_d_n0;
        locals.var_dnm_dn2 = assign82560_e125532_d_n2;
        locals.var_dnm_dn4 = assign82560_e125532_d_n4;
        locals.var_dnm_dn5 = assign82560_e125532_d_n5;
        locals.var_dnm_dn6 = assign82560_e125532_d_n6;
        locals.var_dnm_dn7 = assign82560_e125532_d_n7;
        locals.var_dnm_dn8 = assign82560_e125532_d_n8;
        locals.var_dnm_dn9 = assign82560_e125532_d_n9;
        locals.var_dnm_dn10 = assign82560_e125532_d_n10;
        locals.var_dnm_dn13 = assign82560_e125532_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign82570_e125547: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1915 = assign82570_e125547;
        locals.var_guard1915_rv = 0.0;

        let assign82580_e125550: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1916 = assign82580_e125550;
        locals.var_guard1916_rv = 0.0;

        let (assign82590_e125567,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82590_e125567;
        locals.var_mm_rv = 0.0;

        let assign82600_e125570: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1917 = assign82600_e125570;
        locals.var_guard1917_rv = 0.0;

        let (assign82610_e125590,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 == 0.0)) && (locals.var_guard1917 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82610_e125590;
        locals.var_mm_rv = 0.0;

        let assign82620_e125593: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1918 = assign82620_e125593;
        locals.var_guard1918_rv = 0.0;

        let (assign82630_e125616,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 == 0.0)) && (locals.var_guard1917 == 0.0)) && (locals.var_guard1918 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82630_e125616;
        locals.var_mm_rv = 0.0;

        let assign82640_e125619: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1919 = assign82640_e125619;
        locals.var_guard1919_rv = 0.0;

        let (assign82650_e125645,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 == 0.0)) && (locals.var_guard1917 == 0.0)) && (locals.var_guard1918 == 0.0)) && (locals.var_guard1919 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82650_e125645;
        locals.var_mm_rv = 0.0;

        let (assign82660_e125660,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign82660_e125660;
        locals.var_m0_rv = 0.0;

        let mut assign82670_loop_guard: usize = 0;
        while {
            let assign82670_cond_e125676: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign82670_cond_e125676 != 0.0
        } {
            assign82670_loop_guard += 1;
            assert!(assign82670_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign82670_body0_e125692, assign82670_body0_e125692_d_n0, assign82670_body0_e125692_d_n2, assign82670_body0_e125692_d_n4, assign82670_body0_e125692_d_n5, assign82670_body0_e125692_d_n6, assign82670_body0_e125692_d_n7, assign82670_body0_e125692_d_n8, assign82670_body0_e125692_d_n9, assign82670_body0_e125692_d_n10, assign82670_body0_e125692_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) {
        let assign82670_body0_e125690: f64 = (locals.var_dnm).sqrt();
        (assign82670_body0_e125690, (locals.var_dnm_dn0 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn2 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn4 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn5 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn6 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn7 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn8 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn9 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn10 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn13 / (2.0 * assign82670_body0_e125690)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign82670_body0_e125692;
            locals.var_dnm_dn0 = assign82670_body0_e125692_d_n0;
            locals.var_dnm_dn2 = assign82670_body0_e125692_d_n2;
            locals.var_dnm_dn4 = assign82670_body0_e125692_d_n4;
            locals.var_dnm_dn5 = assign82670_body0_e125692_d_n5;
            locals.var_dnm_dn6 = assign82670_body0_e125692_d_n6;
            locals.var_dnm_dn7 = assign82670_body0_e125692_d_n7;
            locals.var_dnm_dn8 = assign82670_body0_e125692_d_n8;
            locals.var_dnm_dn9 = assign82670_body0_e125692_d_n9;
            locals.var_dnm_dn10 = assign82670_body0_e125692_d_n10;
            locals.var_dnm_dn13 = assign82670_body0_e125692_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign82670_body1_e125709,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) {
        let assign82670_body1_e125707: f64 = (locals.var_m0 + 1.0);
        (assign82670_body1_e125707,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign82670_body1_e125709;
            locals.var_m0_rv = 0.0;
        }

        let (assign82680_e125736, assign82680_e125736_d_n0, assign82680_e125736_d_n2, assign82680_e125736_d_n4, assign82680_e125736_d_n5, assign82680_e125736_d_n6, assign82680_e125736_d_n7, assign82680_e125736_d_n8, assign82680_e125736_d_n9, assign82680_e125736_d_n10, assign82680_e125736_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 == 0.0)) {
        let (assign82680_e125734, assign82680_e125734_d_n0, assign82680_e125734_d_n2, assign82680_e125734_d_n4, assign82680_e125734_d_n5, assign82680_e125734_d_n6, assign82680_e125734_d_n7, assign82680_e125734_d_n8, assign82680_e125734_d_n9, assign82680_e125734_d_n10, assign82680_e125734_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign82680_e125731: f64 = (2.0 * 2.0);
                let assign82680_e125732: f64 = (1.0 / assign82680_e125731);
                let assign82680_e125733: f64 = (locals.var_dnm).powf(assign82680_e125732);
                (assign82680_e125733, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn0)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn2)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn4)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn5)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn6)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn7)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn8)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn9)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn10)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn13)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign82680_e125734, assign82680_e125734_d_n0, assign82680_e125734_d_n2, assign82680_e125734_d_n4, assign82680_e125734_d_n5, assign82680_e125734_d_n6, assign82680_e125734_d_n7, assign82680_e125734_d_n8, assign82680_e125734_d_n9, assign82680_e125734_d_n10, assign82680_e125734_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign82680_e125736;
        locals.var_dnm_dn0 = assign82680_e125736_d_n0;
        locals.var_dnm_dn2 = assign82680_e125736_d_n2;
        locals.var_dnm_dn4 = assign82680_e125736_d_n4;
        locals.var_dnm_dn5 = assign82680_e125736_d_n5;
        locals.var_dnm_dn6 = assign82680_e125736_d_n6;
        locals.var_dnm_dn7 = assign82680_e125736_d_n7;
        locals.var_dnm_dn8 = assign82680_e125736_d_n8;
        locals.var_dnm_dn9 = assign82680_e125736_d_n9;
        locals.var_dnm_dn10 = assign82680_e125736_d_n10;
        locals.var_dnm_dn13 = assign82680_e125736_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign82690_e125751, assign82690_e125751_d_n0, assign82690_e125751_d_n2, assign82690_e125751_d_n4, assign82690_e125751_d_n5, assign82690_e125751_d_n6, assign82690_e125751_d_n7, assign82690_e125751_d_n8, assign82690_e125751_d_n9, assign82690_e125751_d_n10, assign82690_e125751_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82690_e125749: f64 = (1.0 / locals.var_dnm);
        (assign82690_e125749, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign82690_e125751;
        locals.var_dnm_dn0 = assign82690_e125751_d_n0;
        locals.var_dnm_dn2 = assign82690_e125751_d_n2;
        locals.var_dnm_dn4 = assign82690_e125751_d_n4;
        locals.var_dnm_dn5 = assign82690_e125751_d_n5;
        locals.var_dnm_dn6 = assign82690_e125751_d_n6;
        locals.var_dnm_dn7 = assign82690_e125751_d_n7;
        locals.var_dnm_dn8 = assign82690_e125751_d_n8;
        locals.var_dnm_dn9 = assign82690_e125751_d_n9;
        locals.var_dnm_dn10 = assign82690_e125751_d_n10;
        locals.var_dnm_dn13 = assign82690_e125751_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign82700_e125770, assign82700_e125770_d_n0, assign82700_e125770_d_n2, assign82700_e125770_d_n4, assign82700_e125770_d_n5, assign82700_e125770_d_n6, assign82700_e125770_d_n7, assign82700_e125770_d_n8, assign82700_e125770_d_n9, assign82700_e125770_d_n10, assign82700_e125770_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82700_e125765: f64 = (0.2 * locals.var_chi_b);
        let assign82700_e125766: f64 = (locals.var_tmf1 * assign82700_e125765);
        let assign82700_e125768: f64 = (assign82700_e125766 * locals.var_dnm);
        (assign82700_e125768, ((((locals.var_tmf1_dn0 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn13))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign82700_e125770;
        locals.var_tmf0_dn0 = assign82700_e125770_d_n0;
        locals.var_tmf0_dn2 = assign82700_e125770_d_n2;
        locals.var_tmf0_dn4 = assign82700_e125770_d_n4;
        locals.var_tmf0_dn5 = assign82700_e125770_d_n5;
        locals.var_tmf0_dn6 = assign82700_e125770_d_n6;
        locals.var_tmf0_dn7 = assign82700_e125770_d_n7;
        locals.var_tmf0_dn8 = assign82700_e125770_d_n8;
        locals.var_tmf0_dn9 = assign82700_e125770_d_n9;
        locals.var_tmf0_dn10 = assign82700_e125770_d_n10;
        locals.var_tmf0_dn13 = assign82700_e125770_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign82710_e125791, assign82710_e125791_d_n0, assign82710_e125791_d_n2, assign82710_e125791_d_n4, assign82710_e125791_d_n5, assign82710_e125791_d_n6, assign82710_e125791_d_n7, assign82710_e125791_d_n8, assign82710_e125791_d_n9, assign82710_e125791_d_n10, assign82710_e125791_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82710_e125783: f64 = (0.2 * locals.var_chi_b);
        let assign82710_e125785: f64 = (assign82710_e125783 * locals.var_xmp);
        let assign82710_e125787: f64 = (assign82710_e125785 * locals.var_dnm);
        let assign82710_e125789: f64 = (assign82710_e125787 / locals.var_arg);
        (assign82710_e125789, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn0)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn2)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn4)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn5)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn6)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn7)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn8)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn9)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn10)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn13) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn13)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82710_e125791;
        locals.var_t1_dn0 = assign82710_e125791_d_n0;
        locals.var_t1_dn2 = assign82710_e125791_d_n2;
        locals.var_t1_dn4 = assign82710_e125791_d_n4;
        locals.var_t1_dn5 = assign82710_e125791_d_n5;
        locals.var_t1_dn6 = assign82710_e125791_d_n6;
        locals.var_t1_dn7 = assign82710_e125791_d_n7;
        locals.var_t1_dn8 = assign82710_e125791_d_n8;
        locals.var_t1_dn9 = assign82710_e125791_d_n9;
        locals.var_t1_dn10 = assign82710_e125791_d_n10;
        locals.var_t1_dn13 = assign82710_e125791_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign82720_e125810, assign82720_e125810_d_n0, assign82720_e125810_d_n2, assign82720_e125810_d_n4, assign82720_e125810_d_n5, assign82720_e125810_d_n6, assign82720_e125810_d_n7, assign82720_e125810_d_n8, assign82720_e125810_d_n9, assign82720_e125810_d_n10, assign82720_e125810_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82720_e125805: f64 = (0.2 * locals.var_chi_b);
        let assign82720_e125806: f64 = (locals.var_chi_b - assign82720_e125805);
        let assign82720_e125808: f64 = (assign82720_e125806 + locals.var_tmf0);
        (assign82720_e125808, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn13 - (0.2 * locals.var_chi_b_dn13)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82720_e125810;
        locals.var_chi_dn0 = assign82720_e125810_d_n0;
        locals.var_chi_dn2 = assign82720_e125810_d_n2;
        locals.var_chi_dn4 = assign82720_e125810_d_n4;
        locals.var_chi_dn5 = assign82720_e125810_d_n5;
        locals.var_chi_dn6 = assign82720_e125810_d_n6;
        locals.var_chi_dn7 = assign82720_e125810_d_n7;
        locals.var_chi_dn8 = assign82720_e125810_d_n8;
        locals.var_chi_dn9 = assign82720_e125810_d_n9;
        locals.var_chi_dn10 = assign82720_e125810_d_n10;
        locals.var_chi_dn13 = assign82720_e125810_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign82730_e125823, assign82730_e125823_d_n0, assign82730_e125823_d_n2, assign82730_e125823_d_n4, assign82730_e125823_d_n5, assign82730_e125823_d_n6, assign82730_e125823_d_n7, assign82730_e125823_d_n8, assign82730_e125823_d_n9, assign82730_e125823_d_n10, assign82730_e125823_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82730_e125823;
        locals.var_t1_dn0 = assign82730_e125823_d_n0;
        locals.var_t1_dn2 = assign82730_e125823_d_n2;
        locals.var_t1_dn4 = assign82730_e125823_d_n4;
        locals.var_t1_dn5 = assign82730_e125823_d_n5;
        locals.var_t1_dn6 = assign82730_e125823_d_n6;
        locals.var_t1_dn7 = assign82730_e125823_d_n7;
        locals.var_t1_dn8 = assign82730_e125823_d_n8;
        locals.var_t1_dn9 = assign82730_e125823_d_n9;
        locals.var_t1_dn10 = assign82730_e125823_d_n10;
        locals.var_t1_dn13 = assign82730_e125823_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign82740_e125837, assign82740_e125837_d_n0, assign82740_e125837_d_n2, assign82740_e125837_d_n4, assign82740_e125837_d_n5, assign82740_e125837_d_n6, assign82740_e125837_d_n7, assign82740_e125837_d_n8, assign82740_e125837_d_n9, assign82740_e125837_d_n10, assign82740_e125837_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82740_e125837;
        locals.var_chi_dn0 = assign82740_e125837_d_n0;
        locals.var_chi_dn2 = assign82740_e125837_d_n2;
        locals.var_chi_dn4 = assign82740_e125837_d_n4;
        locals.var_chi_dn5 = assign82740_e125837_d_n5;
        locals.var_chi_dn6 = assign82740_e125837_d_n6;
        locals.var_chi_dn7 = assign82740_e125837_d_n7;
        locals.var_chi_dn8 = assign82740_e125837_d_n8;
        locals.var_chi_dn9 = assign82740_e125837_d_n9;
        locals.var_chi_dn10 = assign82740_e125837_d_n10;
        locals.var_chi_dn13 = assign82740_e125837_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign82750_e125851, assign82750_e125851_d_n0, assign82750_e125851_d_n2, assign82750_e125851_d_n4, assign82750_e125851_d_n5, assign82750_e125851_d_n6, assign82750_e125851_d_n7, assign82750_e125851_d_n8, assign82750_e125851_d_n9, assign82750_e125851_d_n10, assign82750_e125851_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82750_e125851;
        locals.var_t1_dn0 = assign82750_e125851_d_n0;
        locals.var_t1_dn2 = assign82750_e125851_d_n2;
        locals.var_t1_dn4 = assign82750_e125851_d_n4;
        locals.var_t1_dn5 = assign82750_e125851_d_n5;
        locals.var_t1_dn6 = assign82750_e125851_d_n6;
        locals.var_t1_dn7 = assign82750_e125851_d_n7;
        locals.var_t1_dn8 = assign82750_e125851_d_n8;
        locals.var_t1_dn9 = assign82750_e125851_d_n9;
        locals.var_t1_dn10 = assign82750_e125851_d_n10;
        locals.var_t1_dn13 = assign82750_e125851_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign82760_e125868, assign82760_e125868_d_n0, assign82760_e125868_d_n2, assign82760_e125868_d_n4, assign82760_e125868_d_n5, assign82760_e125868_d_n6, assign82760_e125868_d_n7, assign82760_e125868_d_n8, assign82760_e125868_d_n9, assign82760_e125868_d_n10, assign82760_e125868_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 == 0.0)) {
        let (assign82760_e125866, assign82760_e125866_d_n0, assign82760_e125866_d_n2, assign82760_e125866_d_n4, assign82760_e125866_d_n5, assign82760_e125866_d_n6, assign82760_e125866_d_n7, assign82760_e125866_d_n8, assign82760_e125866_d_n9, assign82760_e125866_d_n10, assign82760_e125866_d_n13,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            }
        };
        (assign82760_e125866, assign82760_e125866_d_n0, assign82760_e125866_d_n2, assign82760_e125866_d_n4, assign82760_e125866_d_n5, assign82760_e125866_d_n6, assign82760_e125866_d_n7, assign82760_e125866_d_n8, assign82760_e125866_d_n9, assign82760_e125866_d_n10, assign82760_e125866_d_n13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82760_e125868;
        locals.var_chi_dn0 = assign82760_e125868_d_n0;
        locals.var_chi_dn2 = assign82760_e125868_d_n2;
        locals.var_chi_dn4 = assign82760_e125868_d_n4;
        locals.var_chi_dn5 = assign82760_e125868_d_n5;
        locals.var_chi_dn6 = assign82760_e125868_d_n6;
        locals.var_chi_dn7 = assign82760_e125868_d_n7;
        locals.var_chi_dn8 = assign82760_e125868_d_n8;
        locals.var_chi_dn9 = assign82760_e125868_d_n9;
        locals.var_chi_dn10 = assign82760_e125868_d_n10;
        locals.var_chi_dn13 = assign82760_e125868_d_n13;
        locals.var_chi_rv = 0.0;

        let assign82770_e125871: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1920 = assign82770_e125871;
        locals.var_guard1920_rv = 0.0;

        let (assign82780_e125884, assign82780_e125884_d_n0, assign82780_e125884_d_n2, assign82780_e125884_d_n4, assign82780_e125884_d_n5, assign82780_e125884_d_n6, assign82780_e125884_d_n7, assign82780_e125884_d_n8, assign82780_e125884_d_n9, assign82780_e125884_d_n10, assign82780_e125884_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82780_e125880: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign82780_e125882: f64 = (assign82780_e125880 - locals.var_vxbgmtcl);
        (assign82780_e125882, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign82780_e125884;
        locals.var_ps0ld_dn0 = assign82780_e125884_d_n0;
        locals.var_ps0ld_dn2 = assign82780_e125884_d_n2;
        locals.var_ps0ld_dn4 = assign82780_e125884_d_n4;
        locals.var_ps0ld_dn5 = assign82780_e125884_d_n5;
        locals.var_ps0ld_dn6 = assign82780_e125884_d_n6;
        locals.var_ps0ld_dn7 = assign82780_e125884_d_n7;
        locals.var_ps0ld_dn8 = assign82780_e125884_d_n8;
        locals.var_ps0ld_dn9 = assign82780_e125884_d_n9;
        locals.var_ps0ld_dn10 = assign82780_e125884_d_n10;
        locals.var_ps0ld_dn13 = assign82780_e125884_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign82790_e125887: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1921 = assign82790_e125887;
        locals.var_guard1921_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_304(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82800_e125900, assign82800_e125900_d_n0, assign82800_e125900_d_n2, assign82800_e125900_d_n4, assign82800_e125900_d_n5, assign82800_e125900_d_n6, assign82800_e125900_d_n7, assign82800_e125900_d_n8, assign82800_e125900_d_n9, assign82800_e125900_d_n10, assign82800_e125900_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 != 0.0)) {
        let assign82800_e125898: f64 = (p.p334 - locals.var_wdep_func);
        (assign82800_e125898, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82800_e125900;
        locals.var_t2_dn0 = assign82800_e125900_d_n0;
        locals.var_t2_dn2 = assign82800_e125900_d_n2;
        locals.var_t2_dn4 = assign82800_e125900_d_n4;
        locals.var_t2_dn5 = assign82800_e125900_d_n5;
        locals.var_t2_dn6 = assign82800_e125900_d_n6;
        locals.var_t2_dn7 = assign82800_e125900_d_n7;
        locals.var_t2_dn8 = assign82800_e125900_d_n8;
        locals.var_t2_dn9 = assign82800_e125900_d_n9;
        locals.var_t2_dn10 = assign82800_e125900_d_n10;
        locals.var_t2_dn13 = assign82800_e125900_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign82810_e125925, assign82810_e125925_d_n0, assign82810_e125925_d_n2, assign82810_e125925_d_n4, assign82810_e125925_d_n5, assign82810_e125925_d_n6, assign82810_e125925_d_n7, assign82810_e125925_d_n8, assign82810_e125925_d_n9, assign82810_e125925_d_n10, assign82810_e125925_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) {
        let assign82810_e125912: f64 = (locals.var_vdsi + p.p137);
        let assign82810_e125915: f64 = (locals.var_vdsi + p.p137);
        let assign82810_e125916: f64 = (assign82810_e125912 * assign82810_e125915);
        let assign82810_e125919: f64 = (4.0 * 0.1);
        let assign82810_e125921: f64 = (assign82810_e125919 * 0.1);
        let assign82810_e125922: f64 = (assign82810_e125916 + assign82810_e125921);
        let assign82810_e125923: f64 = (assign82810_e125922).sqrt();
        (assign82810_e125923, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign82810_e125915) + (assign82810_e125912 * locals.var_vdsi_dn5)) / (2.0 * assign82810_e125923)), 0.0, (((locals.var_vdsi_dn7 * assign82810_e125915) + (assign82810_e125912 * locals.var_vdsi_dn7)) / (2.0 * assign82810_e125923)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign82810_e125925;
        locals.var_tmf2_dn0 = assign82810_e125925_d_n0;
        locals.var_tmf2_dn2 = assign82810_e125925_d_n2;
        locals.var_tmf2_dn4 = assign82810_e125925_d_n4;
        locals.var_tmf2_dn5 = assign82810_e125925_d_n5;
        locals.var_tmf2_dn6 = assign82810_e125925_d_n6;
        locals.var_tmf2_dn7 = assign82810_e125925_d_n7;
        locals.var_tmf2_dn8 = assign82810_e125925_d_n8;
        locals.var_tmf2_dn9 = assign82810_e125925_d_n9;
        locals.var_tmf2_dn10 = assign82810_e125925_d_n10;
        locals.var_tmf2_dn13 = assign82810_e125925_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign82820_e125945, assign82820_e125945_d_n0, assign82820_e125945_d_n2, assign82820_e125945_d_n4, assign82820_e125945_d_n5, assign82820_e125945_d_n6, assign82820_e125945_d_n7, assign82820_e125945_d_n8, assign82820_e125945_d_n9, assign82820_e125945_d_n10, assign82820_e125945_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) {
        let assign82820_e125939: f64 = (locals.var_vdsi + p.p137);
        let assign82820_e125941: f64 = (assign82820_e125939 / locals.var_tmf2);
        let assign82820_e125942: f64 = (1.0 + assign82820_e125941);
        let assign82820_e125943: f64 = (0.5 * assign82820_e125942);
        (assign82820_e125943, (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign82820_e125939 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign82820_e125939 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign82820_e125945;
        locals.var_t9_dn0 = assign82820_e125945_d_n0;
        locals.var_t9_dn2 = assign82820_e125945_d_n2;
        locals.var_t9_dn4 = assign82820_e125945_d_n4;
        locals.var_t9_dn5 = assign82820_e125945_d_n5;
        locals.var_t9_dn6 = assign82820_e125945_d_n6;
        locals.var_t9_dn7 = assign82820_e125945_d_n7;
        locals.var_t9_dn8 = assign82820_e125945_d_n8;
        locals.var_t9_dn9 = assign82820_e125945_d_n9;
        locals.var_t9_dn10 = assign82820_e125945_d_n10;
        locals.var_t9_dn13 = assign82820_e125945_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign82830_e125963, assign82830_e125963_d_n0, assign82830_e125963_d_n2, assign82830_e125963_d_n4, assign82830_e125963_d_n5, assign82830_e125963_d_n6, assign82830_e125963_d_n7, assign82830_e125963_d_n8, assign82830_e125963_d_n9, assign82830_e125963_d_n10, assign82830_e125963_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) {
        let assign82830_e125958: f64 = (locals.var_vdsi + p.p137);
        let assign82830_e125960: f64 = (assign82830_e125958 + locals.var_tmf2);
        let assign82830_e125961: f64 = (0.5 * assign82830_e125960);
        (assign82830_e125961, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82830_e125963;
        locals.var_t2_dn0 = assign82830_e125963_d_n0;
        locals.var_t2_dn2 = assign82830_e125963_d_n2;
        locals.var_t2_dn4 = assign82830_e125963_d_n4;
        locals.var_t2_dn5 = assign82830_e125963_d_n5;
        locals.var_t2_dn6 = assign82830_e125963_d_n6;
        locals.var_t2_dn7 = assign82830_e125963_d_n7;
        locals.var_t2_dn8 = assign82830_e125963_d_n8;
        locals.var_t2_dn9 = assign82830_e125963_d_n9;
        locals.var_t2_dn10 = assign82830_e125963_d_n10;
        locals.var_t2_dn13 = assign82830_e125963_d_n13;
        locals.var_t2_rv = 0.0;

        let assign82840_e125966: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1922 = assign82840_e125966;
        locals.var_guard1922_rv = 0.0;

        let (assign82850_e125980, assign82850_e125980_d_n0, assign82850_e125980_d_n2, assign82850_e125980_d_n4, assign82850_e125980_d_n5, assign82850_e125980_d_n6, assign82850_e125980_d_n7, assign82850_e125980_d_n8, assign82850_e125980_d_n9, assign82850_e125980_d_n10, assign82850_e125980_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82850_e125980;
        locals.var_t2_dn0 = assign82850_e125980_d_n0;
        locals.var_t2_dn2 = assign82850_e125980_d_n2;
        locals.var_t2_dn4 = assign82850_e125980_d_n4;
        locals.var_t2_dn5 = assign82850_e125980_d_n5;
        locals.var_t2_dn6 = assign82850_e125980_d_n6;
        locals.var_t2_dn7 = assign82850_e125980_d_n7;
        locals.var_t2_dn8 = assign82850_e125980_d_n8;
        locals.var_t2_dn9 = assign82850_e125980_d_n9;
        locals.var_t2_dn10 = assign82850_e125980_d_n10;
        locals.var_t2_dn13 = assign82850_e125980_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign82860_e125994, assign82860_e125994_d_n0, assign82860_e125994_d_n2, assign82860_e125994_d_n4, assign82860_e125994_d_n5, assign82860_e125994_d_n6, assign82860_e125994_d_n7, assign82860_e125994_d_n8, assign82860_e125994_d_n9, assign82860_e125994_d_n10, assign82860_e125994_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign82860_e125994;
        locals.var_t9_dn0 = assign82860_e125994_d_n0;
        locals.var_t9_dn2 = assign82860_e125994_d_n2;
        locals.var_t9_dn4 = assign82860_e125994_d_n4;
        locals.var_t9_dn5 = assign82860_e125994_d_n5;
        locals.var_t9_dn6 = assign82860_e125994_d_n6;
        locals.var_t9_dn7 = assign82860_e125994_d_n7;
        locals.var_t9_dn8 = assign82860_e125994_d_n8;
        locals.var_t9_dn9 = assign82860_e125994_d_n9;
        locals.var_t9_dn10 = assign82860_e125994_d_n10;
        locals.var_t9_dn13 = assign82860_e125994_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign82870_e126011, assign82870_e126011_d_n0, assign82870_e126011_d_n2, assign82870_e126011_d_n4, assign82870_e126011_d_n5, assign82870_e126011_d_n6, assign82870_e126011_d_n7, assign82870_e126011_d_n8, assign82870_e126011_d_n9, assign82870_e126011_d_n10, assign82870_e126011_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) {
        let assign82870_e126006: f64 = (locals.var_kjunc * locals.var_t2);
        let assign82870_e126007: f64 = (assign82870_e126006).sqrt();
        let assign82870_e126009: f64 = (assign82870_e126007 * p.p432);
        (assign82870_e126009, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign82870_e126007)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign82870_e126011;
        locals.var_wjunc0_dn0 = assign82870_e126011_d_n0;
        locals.var_wjunc0_dn2 = assign82870_e126011_d_n2;
        locals.var_wjunc0_dn4 = assign82870_e126011_d_n4;
        locals.var_wjunc0_dn5 = assign82870_e126011_d_n5;
        locals.var_wjunc0_dn6 = assign82870_e126011_d_n6;
        locals.var_wjunc0_dn7 = assign82870_e126011_d_n7;
        locals.var_wjunc0_dn8 = assign82870_e126011_d_n8;
        locals.var_wjunc0_dn9 = assign82870_e126011_d_n9;
        locals.var_wjunc0_dn10 = assign82870_e126011_d_n10;
        locals.var_wjunc0_dn13 = assign82870_e126011_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign82880_e126025, assign82880_e126025_d_n0, assign82880_e126025_d_n2, assign82880_e126025_d_n4, assign82880_e126025_d_n5, assign82880_e126025_d_n6, assign82880_e126025_d_n7, assign82880_e126025_d_n8, assign82880_e126025_d_n9, assign82880_e126025_d_n10, assign82880_e126025_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) {
        let assign82880_e126023: f64 = (p.p334 - locals.var_wjunc0);
        (assign82880_e126023, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82880_e126025;
        locals.var_t2_dn0 = assign82880_e126025_d_n0;
        locals.var_t2_dn2 = assign82880_e126025_d_n2;
        locals.var_t2_dn4 = assign82880_e126025_d_n4;
        locals.var_t2_dn5 = assign82880_e126025_d_n5;
        locals.var_t2_dn6 = assign82880_e126025_d_n6;
        locals.var_t2_dn7 = assign82880_e126025_d_n7;
        locals.var_t2_dn8 = assign82880_e126025_d_n8;
        locals.var_t2_dn9 = assign82880_e126025_d_n9;
        locals.var_t2_dn10 = assign82880_e126025_d_n10;
        locals.var_t2_dn13 = assign82880_e126025_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign82890_e126047, assign82890_e126047_d_n0, assign82890_e126047_d_n2, assign82890_e126047_d_n4, assign82890_e126047_d_n5, assign82890_e126047_d_n6, assign82890_e126047_d_n7, assign82890_e126047_d_n8, assign82890_e126047_d_n9, assign82890_e126047_d_n10, assign82890_e126047_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82890_e126034: f64 = (locals.var_t2 * locals.var_t2);
        let assign82890_e126038: f64 = (p.p334 * 0.01);
        let assign82890_e126039: f64 = (4.0 * assign82890_e126038);
        let assign82890_e126042: f64 = (p.p334 * 0.01);
        let assign82890_e126043: f64 = (assign82890_e126039 * assign82890_e126042);
        let assign82890_e126044: f64 = (assign82890_e126034 + assign82890_e126043);
        let assign82890_e126045: f64 = (assign82890_e126044).sqrt();
        (assign82890_e126045, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign82890_e126045)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign82890_e126047;
        locals.var_tmf2_dn0 = assign82890_e126047_d_n0;
        locals.var_tmf2_dn2 = assign82890_e126047_d_n2;
        locals.var_tmf2_dn4 = assign82890_e126047_d_n4;
        locals.var_tmf2_dn5 = assign82890_e126047_d_n5;
        locals.var_tmf2_dn6 = assign82890_e126047_d_n6;
        locals.var_tmf2_dn7 = assign82890_e126047_d_n7;
        locals.var_tmf2_dn8 = assign82890_e126047_d_n8;
        locals.var_tmf2_dn9 = assign82890_e126047_d_n9;
        locals.var_tmf2_dn10 = assign82890_e126047_d_n10;
        locals.var_tmf2_dn13 = assign82890_e126047_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign82900_e126062, assign82900_e126062_d_n0, assign82900_e126062_d_n2, assign82900_e126062_d_n4, assign82900_e126062_d_n5, assign82900_e126062_d_n6, assign82900_e126062_d_n7, assign82900_e126062_d_n8, assign82900_e126062_d_n9, assign82900_e126062_d_n10, assign82900_e126062_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82900_e126058: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign82900_e126059: f64 = (1.0 + assign82900_e126058);
        let assign82900_e126060: f64 = (0.5 * assign82900_e126059);
        (assign82900_e126060, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign82900_e126062;
        locals.var_t9_dn0 = assign82900_e126062_d_n0;
        locals.var_t9_dn2 = assign82900_e126062_d_n2;
        locals.var_t9_dn4 = assign82900_e126062_d_n4;
        locals.var_t9_dn5 = assign82900_e126062_d_n5;
        locals.var_t9_dn6 = assign82900_e126062_d_n6;
        locals.var_t9_dn7 = assign82900_e126062_d_n7;
        locals.var_t9_dn8 = assign82900_e126062_d_n8;
        locals.var_t9_dn9 = assign82900_e126062_d_n9;
        locals.var_t9_dn10 = assign82900_e126062_d_n10;
        locals.var_t9_dn13 = assign82900_e126062_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign82910_e126075, assign82910_e126075_d_n0, assign82910_e126075_d_n2, assign82910_e126075_d_n4, assign82910_e126075_d_n5, assign82910_e126075_d_n6, assign82910_e126075_d_n7, assign82910_e126075_d_n8, assign82910_e126075_d_n9, assign82910_e126075_d_n10, assign82910_e126075_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82910_e126072: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign82910_e126073: f64 = (0.5 * assign82910_e126072);
        (assign82910_e126073, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82910_e126075;
        locals.var_t2_dn0 = assign82910_e126075_d_n0;
        locals.var_t2_dn2 = assign82910_e126075_d_n2;
        locals.var_t2_dn4 = assign82910_e126075_d_n4;
        locals.var_t2_dn5 = assign82910_e126075_d_n5;
        locals.var_t2_dn6 = assign82910_e126075_d_n6;
        locals.var_t2_dn7 = assign82910_e126075_d_n7;
        locals.var_t2_dn8 = assign82910_e126075_d_n8;
        locals.var_t2_dn9 = assign82910_e126075_d_n9;
        locals.var_t2_dn10 = assign82910_e126075_d_n10;
        locals.var_t2_dn13 = assign82910_e126075_d_n13;
        locals.var_t2_rv = 0.0;

        let assign82920_e126078: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1923 = assign82920_e126078;
        locals.var_guard1923_rv = 0.0;

        let (assign82930_e126089, assign82930_e126089_d_n0, assign82930_e126089_d_n2, assign82930_e126089_d_n4, assign82930_e126089_d_n5, assign82930_e126089_d_n6, assign82930_e126089_d_n7, assign82930_e126089_d_n8, assign82930_e126089_d_n9, assign82930_e126089_d_n10, assign82930_e126089_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1923 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82930_e126089;
        locals.var_t2_dn0 = assign82930_e126089_d_n0;
        locals.var_t2_dn2 = assign82930_e126089_d_n2;
        locals.var_t2_dn4 = assign82930_e126089_d_n4;
        locals.var_t2_dn5 = assign82930_e126089_d_n5;
        locals.var_t2_dn6 = assign82930_e126089_d_n6;
        locals.var_t2_dn7 = assign82930_e126089_d_n7;
        locals.var_t2_dn8 = assign82930_e126089_d_n8;
        locals.var_t2_dn9 = assign82930_e126089_d_n9;
        locals.var_t2_dn10 = assign82930_e126089_d_n10;
        locals.var_t2_dn13 = assign82930_e126089_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign82940_e126100, assign82940_e126100_d_n0, assign82940_e126100_d_n2, assign82940_e126100_d_n4, assign82940_e126100_d_n5, assign82940_e126100_d_n6, assign82940_e126100_d_n7, assign82940_e126100_d_n8, assign82940_e126100_d_n9, assign82940_e126100_d_n10, assign82940_e126100_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1923 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign82940_e126100;
        locals.var_t9_dn0 = assign82940_e126100_d_n0;
        locals.var_t9_dn2 = assign82940_e126100_d_n2;
        locals.var_t9_dn4 = assign82940_e126100_d_n4;
        locals.var_t9_dn5 = assign82940_e126100_d_n5;
        locals.var_t9_dn6 = assign82940_e126100_d_n6;
        locals.var_t9_dn7 = assign82940_e126100_d_n7;
        locals.var_t9_dn8 = assign82940_e126100_d_n8;
        locals.var_t9_dn9 = assign82940_e126100_d_n9;
        locals.var_t9_dn10 = assign82940_e126100_d_n10;
        locals.var_t9_dn13 = assign82940_e126100_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign82950_e126109, assign82950_e126109_d_n0, assign82950_e126109_d_n2, assign82950_e126109_d_n4, assign82950_e126109_d_n5, assign82950_e126109_d_n6, assign82950_e126109_d_n7, assign82950_e126109_d_n8, assign82950_e126109_d_n9, assign82950_e126109_d_n10, assign82950_e126109_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign82950_e126109;
        locals.var_ddriftldc_dn0 = assign82950_e126109_d_n0;
        locals.var_ddriftldc_dn2 = assign82950_e126109_d_n2;
        locals.var_ddriftldc_dn4 = assign82950_e126109_d_n4;
        locals.var_ddriftldc_dn5 = assign82950_e126109_d_n5;
        locals.var_ddriftldc_dn6 = assign82950_e126109_d_n6;
        locals.var_ddriftldc_dn7 = assign82950_e126109_d_n7;
        locals.var_ddriftldc_dn8 = assign82950_e126109_d_n8;
        locals.var_ddriftldc_dn9 = assign82950_e126109_d_n9;
        locals.var_ddriftldc_dn10 = assign82950_e126109_d_n10;
        locals.var_ddriftldc_dn13 = assign82950_e126109_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign82960_e126126, assign82960_e126126_d_n0, assign82960_e126126_d_n2, assign82960_e126126_d_n4, assign82960_e126126_d_n5, assign82960_e126126_d_n6, assign82960_e126126_d_n7, assign82960_e126126_d_n8, assign82960_e126126_d_n9, assign82960_e126126_d_n10, assign82960_e126126_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82960_e126118: f64 = (locals.var_q_nsubld__blk1883 * locals.var_ddriftldc);
        let assign82960_e126120: f64 = (assign82960_e126118 * locals.var_ddriftldc);
        let assign82960_e126122: f64 = (assign82960_e126120 / 2.0);
        let assign82960_e126124: f64 = (assign82960_e126122 / 1.034943e-10);
        (assign82960_e126124, (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign82960_e126126;
        locals.var_dphi_sb_dn0 = assign82960_e126126_d_n0;
        locals.var_dphi_sb_dn2 = assign82960_e126126_d_n2;
        locals.var_dphi_sb_dn4 = assign82960_e126126_d_n4;
        locals.var_dphi_sb_dn5 = assign82960_e126126_d_n5;
        locals.var_dphi_sb_dn6 = assign82960_e126126_d_n6;
        locals.var_dphi_sb_dn7 = assign82960_e126126_d_n7;
        locals.var_dphi_sb_dn8 = assign82960_e126126_d_n8;
        locals.var_dphi_sb_dn9 = assign82960_e126126_d_n9;
        locals.var_dphi_sb_dn10 = assign82960_e126126_d_n10;
        locals.var_dphi_sb_dn13 = assign82960_e126126_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign82970_e126140, assign82970_e126140_d_n0, assign82970_e126140_d_n2, assign82970_e126140_d_n4, assign82970_e126140_d_n5, assign82970_e126140_d_n6, assign82970_e126140_d_n7, assign82970_e126140_d_n8, assign82970_e126140_d_n9, assign82970_e126140_d_n10, assign82970_e126140_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82970_e126135: f64 = (2.0 * locals.var_beta);
        let assign82970_e126137: f64 = (assign82970_e126135 * locals.var_dphi_sb);
        let assign82970_e126138: f64 = (assign82970_e126137).sqrt();
        (assign82970_e126138, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn0)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn2)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn4)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn5)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn6)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn7)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn8)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn9)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn10)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn13)) / (2.0 * assign82970_e126138)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign82970_e126140;
        locals.var_t0_dn0 = assign82970_e126140_d_n0;
        locals.var_t0_dn2 = assign82970_e126140_d_n2;
        locals.var_t0_dn4 = assign82970_e126140_d_n4;
        locals.var_t0_dn5 = assign82970_e126140_d_n5;
        locals.var_t0_dn6 = assign82970_e126140_d_n6;
        locals.var_t0_dn7 = assign82970_e126140_d_n7;
        locals.var_t0_dn8 = assign82970_e126140_d_n8;
        locals.var_t0_dn9 = assign82970_e126140_d_n9;
        locals.var_t0_dn10 = assign82970_e126140_d_n10;
        locals.var_t0_dn13 = assign82970_e126140_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign82980_e126156, assign82980_e126156_d_n0, assign82980_e126156_d_n2, assign82980_e126156_d_n4, assign82980_e126156_d_n5, assign82980_e126156_d_n6, assign82980_e126156_d_n7, assign82980_e126156_d_n8, assign82980_e126156_d_n9, assign82980_e126156_d_n10, assign82980_e126156_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82980_e126148: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign82980_e126150: f64 = (-locals.var_t0);
        let assign82980_e126151: f64 = { let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign82980_e126152: f64 = (assign82980_e126148 + assign82980_e126151);
        let assign82980_e126154: f64 = (assign82980_e126152 / 2.0);
        (assign82980_e126154, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82980_e126156;
        locals.var_t1_dn0 = assign82980_e126156_d_n0;
        locals.var_t1_dn2 = assign82980_e126156_d_n2;
        locals.var_t1_dn4 = assign82980_e126156_d_n4;
        locals.var_t1_dn5 = assign82980_e126156_d_n5;
        locals.var_t1_dn6 = assign82980_e126156_d_n6;
        locals.var_t1_dn7 = assign82980_e126156_d_n7;
        locals.var_t1_dn8 = assign82980_e126156_d_n8;
        locals.var_t1_dn9 = assign82980_e126156_d_n9;
        locals.var_t1_dn10 = assign82980_e126156_d_n10;
        locals.var_t1_dn13 = assign82980_e126156_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign82990_e126168, assign82990_e126168_d_n0, assign82990_e126168_d_n2, assign82990_e126168_d_n4, assign82990_e126168_d_n5, assign82990_e126168_d_n6, assign82990_e126168_d_n7, assign82990_e126168_d_n8, assign82990_e126168_d_n9, assign82990_e126168_d_n10, assign82990_e126168_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82990_e126164: f64 = (locals.var_t1).ln();
        let assign82990_e126166: f64 = (assign82990_e126164 / locals.var_dphi_sb);
        (assign82990_e126166, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign82990_e126168;
        locals.var_c_sb_dn0 = assign82990_e126168_d_n0;
        locals.var_c_sb_dn2 = assign82990_e126168_d_n2;
        locals.var_c_sb_dn4 = assign82990_e126168_d_n4;
        locals.var_c_sb_dn5 = assign82990_e126168_d_n5;
        locals.var_c_sb_dn6 = assign82990_e126168_d_n6;
        locals.var_c_sb_dn7 = assign82990_e126168_d_n7;
        locals.var_c_sb_dn8 = assign82990_e126168_d_n8;
        locals.var_c_sb_dn9 = assign82990_e126168_d_n9;
        locals.var_c_sb_dn10 = assign82990_e126168_d_n10;
        locals.var_c_sb_dn13 = assign82990_e126168_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign83000_e126179, assign83000_e126179_d_n0, assign83000_e126179_d_n2, assign83000_e126179_d_n4, assign83000_e126179_d_n5, assign83000_e126179_d_n6, assign83000_e126179_d_n7, assign83000_e126179_d_n8, assign83000_e126179_d_n9, assign83000_e126179_d_n10, assign83000_e126179_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign83000_e126177: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign83000_e126177, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
        locals.var_ps0ld_vxb = assign83000_e126179;
        locals.var_ps0ld_vxb_dn0 = assign83000_e126179_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign83000_e126179_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign83000_e126179_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign83000_e126179_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign83000_e126179_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign83000_e126179_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign83000_e126179_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign83000_e126179_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign83000_e126179_d_n10;
        locals.var_ps0ld_vxb_dn13 = assign83000_e126179_d_n13;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign83010_e126192, assign83010_e126192_d_n0, assign83010_e126192_d_n2, assign83010_e126192_d_n4, assign83010_e126192_d_n5, assign83010_e126192_d_n6, assign83010_e126192_d_n7, assign83010_e126192_d_n8, assign83010_e126192_d_n9, assign83010_e126192_d_n10, assign83010_e126192_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign83010_e126189: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign83010_e126190: f64 = (locals.var_c_sb * assign83010_e126189);
        (assign83010_e126190, ((locals.var_c_sb_dn0 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign83010_e126192;
        locals.var_ty_dn0 = assign83010_e126192_d_n0;
        locals.var_ty_dn2 = assign83010_e126192_d_n2;
        locals.var_ty_dn4 = assign83010_e126192_d_n4;
        locals.var_ty_dn5 = assign83010_e126192_d_n5;
        locals.var_ty_dn6 = assign83010_e126192_d_n6;
        locals.var_ty_dn7 = assign83010_e126192_d_n7;
        locals.var_ty_dn8 = assign83010_e126192_d_n8;
        locals.var_ty_dn9 = assign83010_e126192_d_n9;
        locals.var_ty_dn10 = assign83010_e126192_d_n10;
        locals.var_ty_dn13 = assign83010_e126192_d_n13;
        locals.var_ty_rv = 0.0;

        let assign83020_e126195: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard1924 = assign83020_e126195;
        locals.var_guard1924_rv = 0.0;

        let (assign83030_e126207, assign83030_e126207_d_n0, assign83030_e126207_d_n2, assign83030_e126207_d_n4, assign83030_e126207_d_n5, assign83030_e126207_d_n6, assign83030_e126207_d_n7, assign83030_e126207_d_n8, assign83030_e126207_d_n9, assign83030_e126207_d_n10, assign83030_e126207_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1924 != 0.0)) {
        let assign83030_e126205: f64 = (locals.var_ty).exp();
        (assign83030_e126205, (assign83030_e126205 * locals.var_ty_dn0), (assign83030_e126205 * locals.var_ty_dn2), (assign83030_e126205 * locals.var_ty_dn4), (assign83030_e126205 * locals.var_ty_dn5), (assign83030_e126205 * locals.var_ty_dn6), (assign83030_e126205 * locals.var_ty_dn7), (assign83030_e126205 * locals.var_ty_dn8), (assign83030_e126205 * locals.var_ty_dn9), (assign83030_e126205 * locals.var_ty_dn10), (assign83030_e126205 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign83030_e126207;
        locals.var_t1_dn0 = assign83030_e126207_d_n0;
        locals.var_t1_dn2 = assign83030_e126207_d_n2;
        locals.var_t1_dn4 = assign83030_e126207_d_n4;
        locals.var_t1_dn5 = assign83030_e126207_d_n5;
        locals.var_t1_dn6 = assign83030_e126207_d_n6;
        locals.var_t1_dn7 = assign83030_e126207_d_n7;
        locals.var_t1_dn8 = assign83030_e126207_d_n8;
        locals.var_t1_dn9 = assign83030_e126207_d_n9;
        locals.var_t1_dn10 = assign83030_e126207_d_n10;
        locals.var_t1_dn13 = assign83030_e126207_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign83040_e126222, assign83040_e126222_d_n0, assign83040_e126222_d_n2, assign83040_e126222_d_n4, assign83040_e126222_d_n5, assign83040_e126222_d_n6, assign83040_e126222_d_n7, assign83040_e126222_d_n8, assign83040_e126222_d_n9, assign83040_e126222_d_n10, assign83040_e126222_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1924 != 0.0)) {
        let assign83040_e126217: f64 = (-locals.var_c_sb);
        let assign83040_e126219: f64 = (assign83040_e126217 * locals.var_dphi_sb);
        let assign83040_e126220: f64 = (assign83040_e126219).exp();
        (assign83040_e126220, (assign83040_e126220 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn0))), (assign83040_e126220 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn2))), (assign83040_e126220 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn4))), (assign83040_e126220 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn5))), (assign83040_e126220 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn6))), (assign83040_e126220 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn7))), (assign83040_e126220 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn8))), (assign83040_e126220 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn9))), (assign83040_e126220 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn10))), (assign83040_e126220 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83040_e126222;
        locals.var_t0_dn0 = assign83040_e126222_d_n0;
        locals.var_t0_dn2 = assign83040_e126222_d_n2;
        locals.var_t0_dn4 = assign83040_e126222_d_n4;
        locals.var_t0_dn5 = assign83040_e126222_d_n5;
        locals.var_t0_dn6 = assign83040_e126222_d_n6;
        locals.var_t0_dn7 = assign83040_e126222_d_n7;
        locals.var_t0_dn8 = assign83040_e126222_d_n8;
        locals.var_t0_dn9 = assign83040_e126222_d_n9;
        locals.var_t0_dn10 = assign83040_e126222_d_n10;
        locals.var_t0_dn13 = assign83040_e126222_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_305(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign83050_e126235, assign83050_e126235_d_n0, assign83050_e126235_d_n2, assign83050_e126235_d_n4, assign83050_e126235_d_n5, assign83050_e126235_d_n6, assign83050_e126235_d_n7, assign83050_e126235_d_n8, assign83050_e126235_d_n9, assign83050_e126235_d_n10, assign83050_e126235_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1924 != 0.0)) {
        let assign83050_e126233: f64 = (locals.var_t1 - locals.var_t0);
        (assign83050_e126233, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign83050_e126235;
        locals.var_t2_dn0 = assign83050_e126235_d_n0;
        locals.var_t2_dn2 = assign83050_e126235_d_n2;
        locals.var_t2_dn4 = assign83050_e126235_d_n4;
        locals.var_t2_dn5 = assign83050_e126235_d_n5;
        locals.var_t2_dn6 = assign83050_e126235_d_n6;
        locals.var_t2_dn7 = assign83050_e126235_d_n7;
        locals.var_t2_dn8 = assign83050_e126235_d_n8;
        locals.var_t2_dn9 = assign83050_e126235_d_n9;
        locals.var_t2_dn10 = assign83050_e126235_d_n10;
        locals.var_t2_dn13 = assign83050_e126235_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign83060_e126251, assign83060_e126251_d_n0, assign83060_e126251_d_n2, assign83060_e126251_d_n4, assign83060_e126251_d_n5, assign83060_e126251_d_n6, assign83060_e126251_d_n7, assign83060_e126251_d_n8, assign83060_e126251_d_n9, assign83060_e126251_d_n10, assign83060_e126251_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1924 != 0.0)) {
        let assign83060_e126246: f64 = (1.0 + locals.var_t2);
        let assign83060_e126247: f64 = (assign83060_e126246).ln();
        let assign83060_e126249: f64 = (assign83060_e126247 / locals.var_c_sb);
        (assign83060_e126249, ((((locals.var_t2_dn0 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign83060_e126251;
        locals.var_phi_b_dn0 = assign83060_e126251_d_n0;
        locals.var_phi_b_dn2 = assign83060_e126251_d_n2;
        locals.var_phi_b_dn4 = assign83060_e126251_d_n4;
        locals.var_phi_b_dn5 = assign83060_e126251_d_n5;
        locals.var_phi_b_dn6 = assign83060_e126251_d_n6;
        locals.var_phi_b_dn7 = assign83060_e126251_d_n7;
        locals.var_phi_b_dn8 = assign83060_e126251_d_n8;
        locals.var_phi_b_dn9 = assign83060_e126251_d_n9;
        locals.var_phi_b_dn10 = assign83060_e126251_d_n10;
        locals.var_phi_b_dn13 = assign83060_e126251_d_n13;
        locals.var_phi_b_rv = 0.0;

        let (assign83070_e126265, assign83070_e126265_d_n0, assign83070_e126265_d_n2, assign83070_e126265_d_n4, assign83070_e126265_d_n5, assign83070_e126265_d_n6, assign83070_e126265_d_n7, assign83070_e126265_d_n8, assign83070_e126265_d_n9, assign83070_e126265_d_n10, assign83070_e126265_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1924 == 0.0)) {
        let assign83070_e126263: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign83070_e126263, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign83070_e126265;
        locals.var_phi_b_dn0 = assign83070_e126265_d_n0;
        locals.var_phi_b_dn2 = assign83070_e126265_d_n2;
        locals.var_phi_b_dn4 = assign83070_e126265_d_n4;
        locals.var_phi_b_dn5 = assign83070_e126265_d_n5;
        locals.var_phi_b_dn6 = assign83070_e126265_d_n6;
        locals.var_phi_b_dn7 = assign83070_e126265_d_n7;
        locals.var_phi_b_dn8 = assign83070_e126265_d_n8;
        locals.var_phi_b_dn9 = assign83070_e126265_d_n9;
        locals.var_phi_b_dn10 = assign83070_e126265_d_n10;
        locals.var_phi_b_dn13 = assign83070_e126265_d_n13;
        locals.var_phi_b_rv = 0.0;

        let (assign83080_e126276, assign83080_e126276_d_n0, assign83080_e126276_d_n2, assign83080_e126276_d_n4, assign83080_e126276_d_n5, assign83080_e126276_d_n6, assign83080_e126276_d_n7, assign83080_e126276_d_n8, assign83080_e126276_d_n9, assign83080_e126276_d_n10, assign83080_e126276_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign83080_e126274: f64 = (locals.var_beta * locals.var_phi_b);
        (assign83080_e126274, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
        locals.var_chib = assign83080_e126276;
        locals.var_chib_dn0 = assign83080_e126276_d_n0;
        locals.var_chib_dn2 = assign83080_e126276_d_n2;
        locals.var_chib_dn4 = assign83080_e126276_d_n4;
        locals.var_chib_dn5 = assign83080_e126276_d_n5;
        locals.var_chib_dn6 = assign83080_e126276_d_n6;
        locals.var_chib_dn7 = assign83080_e126276_d_n7;
        locals.var_chib_dn8 = assign83080_e126276_d_n8;
        locals.var_chib_dn9 = assign83080_e126276_d_n9;
        locals.var_chib_dn10 = assign83080_e126276_d_n10;
        locals.var_chib_dn13 = assign83080_e126276_d_n13;
        locals.var_chib_rv = 0.0;

        let assign83090_e126280: f64 = (locals.var_chi / 100.0);
        let assign83090_e126285: f64 = if ((locals.var_chib > assign83090_e126280) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1925 = assign83090_e126285;
        locals.var_guard1925_rv = 0.0;

        let (assign83100_e126298,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1925 != 0.0)) {
        let assign83100_e126296: f64 = (locals.var_flg_fd_mode__blk1889 + 1.0);
        (assign83100_e126296,)
    } else {
        (locals.var_flg_fd_mode__blk1889,)
    }
};
        locals.var_flg_fd_mode__blk1889 = assign83100_e126298;
        locals.var_flg_fd_mode__blk1889_rv = 0.0;

        let (assign83110_e126309, assign83110_e126309_d_n0, assign83110_e126309_d_n2, assign83110_e126309_d_n4, assign83110_e126309_d_n5, assign83110_e126309_d_n6, assign83110_e126309_d_n7, assign83110_e126309_d_n8, assign83110_e126309_d_n9, assign83110_e126309_d_n10, assign83110_e126309_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1925 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign83110_e126309;
        locals.var_chi_dn0 = assign83110_e126309_d_n0;
        locals.var_chi_dn2 = assign83110_e126309_d_n2;
        locals.var_chi_dn4 = assign83110_e126309_d_n4;
        locals.var_chi_dn5 = assign83110_e126309_d_n5;
        locals.var_chi_dn6 = assign83110_e126309_d_n6;
        locals.var_chi_dn7 = assign83110_e126309_d_n7;
        locals.var_chi_dn8 = assign83110_e126309_d_n8;
        locals.var_chi_dn9 = assign83110_e126309_d_n9;
        locals.var_chi_dn10 = assign83110_e126309_d_n10;
        locals.var_chi_dn13 = assign83110_e126309_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign83120_e126320, assign83120_e126320_d_n0, assign83120_e126320_d_n2, assign83120_e126320_d_n4, assign83120_e126320_d_n5, assign83120_e126320_d_n6, assign83120_e126320_d_n7, assign83120_e126320_d_n8, assign83120_e126320_d_n9, assign83120_e126320_d_n10, assign83120_e126320_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        let assign83120_e126316: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign83120_e126318: f64 = (assign83120_e126316 - locals.var_vxbgmtcl);
        (assign83120_e126318, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign83120_e126320;
        locals.var_ps0ld_dn0 = assign83120_e126320_d_n0;
        locals.var_ps0ld_dn2 = assign83120_e126320_d_n2;
        locals.var_ps0ld_dn4 = assign83120_e126320_d_n4;
        locals.var_ps0ld_dn5 = assign83120_e126320_d_n5;
        locals.var_ps0ld_dn6 = assign83120_e126320_d_n6;
        locals.var_ps0ld_dn7 = assign83120_e126320_d_n7;
        locals.var_ps0ld_dn8 = assign83120_e126320_d_n8;
        locals.var_ps0ld_dn9 = assign83120_e126320_d_n9;
        locals.var_ps0ld_dn10 = assign83120_e126320_d_n10;
        locals.var_ps0ld_dn13 = assign83120_e126320_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign83130_e126322: f64 = (locals.var_chi).abs();
        let assign83130_e126324: f64 = if assign83130_e126322 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1926 = assign83130_e126324;
        locals.var_guard1926_rv = 0.0;

        let (assign83140_e126339, assign83140_e126339_d_n0, assign83140_e126339_d_n2, assign83140_e126339_d_n4, assign83140_e126339_d_n5, assign83140_e126339_d_n6, assign83140_e126339_d_n7, assign83140_e126339_d_n8, assign83140_e126339_d_n9, assign83140_e126339_d_n10, assign83140_e126339_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1926 != 0.0)) {
        let assign83140_e126333: f64 = (locals.var_chi - 1.0);
        let assign83140_e126335: f64 = (-locals.var_chi);
        let assign83140_e126336: f64 = (assign83140_e126335).exp();
        let assign83140_e126337: f64 = (assign83140_e126333 + assign83140_e126336);
        (assign83140_e126337, (locals.var_chi_dn0 + (assign83140_e126336 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign83140_e126336 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign83140_e126336 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign83140_e126336 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign83140_e126336 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign83140_e126336 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign83140_e126336 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign83140_e126336 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign83140_e126336 * (-locals.var_chi_dn10))), (locals.var_chi_dn13 + (assign83140_e126336 * (-locals.var_chi_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign83140_e126339;
        locals.var_t1_dn0 = assign83140_e126339_d_n0;
        locals.var_t1_dn2 = assign83140_e126339_d_n2;
        locals.var_t1_dn4 = assign83140_e126339_d_n4;
        locals.var_t1_dn5 = assign83140_e126339_d_n5;
        locals.var_t1_dn6 = assign83140_e126339_d_n6;
        locals.var_t1_dn7 = assign83140_e126339_d_n7;
        locals.var_t1_dn8 = assign83140_e126339_d_n8;
        locals.var_t1_dn9 = assign83140_e126339_d_n9;
        locals.var_t1_dn10 = assign83140_e126339_d_n10;
        locals.var_t1_dn13 = assign83140_e126339_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign83150_e126349, assign83150_e126349_d_n0, assign83150_e126349_d_n2, assign83150_e126349_d_n4, assign83150_e126349_d_n5, assign83150_e126349_d_n6, assign83150_e126349_d_n7, assign83150_e126349_d_n8, assign83150_e126349_d_n9, assign83150_e126349_d_n10, assign83150_e126349_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1926 != 0.0)) {
        let assign83150_e126347: f64 = (locals.var_t1).sqrt();
        (assign83150_e126347, (locals.var_t1_dn0 / (2.0 * assign83150_e126347)), (locals.var_t1_dn2 / (2.0 * assign83150_e126347)), (locals.var_t1_dn4 / (2.0 * assign83150_e126347)), (locals.var_t1_dn5 / (2.0 * assign83150_e126347)), (locals.var_t1_dn6 / (2.0 * assign83150_e126347)), (locals.var_t1_dn7 / (2.0 * assign83150_e126347)), (locals.var_t1_dn8 / (2.0 * assign83150_e126347)), (locals.var_t1_dn9 / (2.0 * assign83150_e126347)), (locals.var_t1_dn10 / (2.0 * assign83150_e126347)), (locals.var_t1_dn13 / (2.0 * assign83150_e126347)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign83150_e126349;
        locals.var_t2_dn0 = assign83150_e126349_d_n0;
        locals.var_t2_dn2 = assign83150_e126349_d_n2;
        locals.var_t2_dn4 = assign83150_e126349_d_n4;
        locals.var_t2_dn5 = assign83150_e126349_d_n5;
        locals.var_t2_dn6 = assign83150_e126349_d_n6;
        locals.var_t2_dn7 = assign83150_e126349_d_n7;
        locals.var_t2_dn8 = assign83150_e126349_d_n8;
        locals.var_t2_dn9 = assign83150_e126349_d_n9;
        locals.var_t2_dn10 = assign83150_e126349_d_n10;
        locals.var_t2_dn13 = assign83150_e126349_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign83170_e126380, assign83170_e126380_d_n0, assign83170_e126380_d_n2, assign83170_e126380_d_n4, assign83170_e126380_d_n5, assign83170_e126380_d_n6, assign83170_e126380_d_n7, assign83170_e126380_d_n8, assign83170_e126380_d_n9, assign83170_e126380_d_n10, assign83170_e126380_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1926 == 0.0)) {
        let assign83170_e126371: f64 = (0.7071067811865475 * locals.var_chi);
        let assign83170_e126375: f64 = (locals.var_chi * 0.3333333333333333);
        let assign83170_e126376: f64 = (1.0 - assign83170_e126375);
        let assign83170_e126377: f64 = (assign83170_e126376).sqrt();
        let assign83170_e126378: f64 = (assign83170_e126371 * assign83170_e126377);
        (assign83170_e126378, (((0.7071067811865475 * locals.var_chi_dn0) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn13) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn13 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign83170_e126380;
        locals.var_t2_dn0 = assign83170_e126380_d_n0;
        locals.var_t2_dn2 = assign83170_e126380_d_n2;
        locals.var_t2_dn4 = assign83170_e126380_d_n4;
        locals.var_t2_dn5 = assign83170_e126380_d_n5;
        locals.var_t2_dn6 = assign83170_e126380_d_n6;
        locals.var_t2_dn7 = assign83170_e126380_d_n7;
        locals.var_t2_dn8 = assign83170_e126380_d_n8;
        locals.var_t2_dn9 = assign83170_e126380_d_n9;
        locals.var_t2_dn10 = assign83170_e126380_d_n10;
        locals.var_t2_dn13 = assign83170_e126380_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign83180_e126389, assign83180_e126389_d_n0, assign83180_e126389_d_n2, assign83180_e126389_d_n4, assign83180_e126389_d_n5, assign83180_e126389_d_n6, assign83180_e126389_d_n7, assign83180_e126389_d_n8, assign83180_e126389_d_n9, assign83180_e126389_d_n10, assign83180_e126389_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        let assign83180_e126387: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign83180_e126387, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign83180_e126389;
        locals.var_qbuld_dn0 = assign83180_e126389_d_n0;
        locals.var_qbuld_dn2 = assign83180_e126389_d_n2;
        locals.var_qbuld_dn4 = assign83180_e126389_d_n4;
        locals.var_qbuld_dn5 = assign83180_e126389_d_n5;
        locals.var_qbuld_dn6 = assign83180_e126389_d_n6;
        locals.var_qbuld_dn7 = assign83180_e126389_d_n7;
        locals.var_qbuld_dn8 = assign83180_e126389_d_n8;
        locals.var_qbuld_dn9 = assign83180_e126389_d_n9;
        locals.var_qbuld_dn10 = assign83180_e126389_d_n10;
        locals.var_qbuld_dn13 = assign83180_e126389_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign83190_e126400, assign83190_e126400_d_n0, assign83190_e126400_d_n2, assign83190_e126400_d_n4, assign83190_e126400_d_n5, assign83190_e126400_d_n6, assign83190_e126400_d_n7, assign83190_e126400_d_n8, assign83190_e126400_d_n9, assign83190_e126400_d_n10, assign83190_e126400_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        let assign83190_e126397: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign83190_e126398: f64 = (locals.var_cox0_func * assign83190_e126397);
        (assign83190_e126398, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (-locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn13)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign83190_e126400;
        locals.var_qsuld_dn0 = assign83190_e126400_d_n0;
        locals.var_qsuld_dn2 = assign83190_e126400_d_n2;
        locals.var_qsuld_dn4 = assign83190_e126400_d_n4;
        locals.var_qsuld_dn5 = assign83190_e126400_d_n5;
        locals.var_qsuld_dn6 = assign83190_e126400_d_n6;
        locals.var_qsuld_dn7 = assign83190_e126400_d_n7;
        locals.var_qsuld_dn8 = assign83190_e126400_d_n8;
        locals.var_qsuld_dn9 = assign83190_e126400_d_n9;
        locals.var_qsuld_dn10 = assign83190_e126400_d_n10;
        locals.var_qsuld_dn13 = assign83190_e126400_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign83200_e126409, assign83200_e126409_d_n0, assign83200_e126409_d_n2, assign83200_e126409_d_n4, assign83200_e126409_d_n5, assign83200_e126409_d_n6, assign83200_e126409_d_n7, assign83200_e126409_d_n8, assign83200_e126409_d_n9, assign83200_e126409_d_n10, assign83200_e126409_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        let assign83200_e126407: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk1883);
        (assign83200_e126407, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn13 / locals.var_q_nsubld__blk1883),)
    } else {
        (locals.var_wdld0__blk1927, locals.var_wdld0__blk1927_dn0, locals.var_wdld0__blk1927_dn2, locals.var_wdld0__blk1927_dn4, locals.var_wdld0__blk1927_dn5, locals.var_wdld0__blk1927_dn6, locals.var_wdld0__blk1927_dn7, locals.var_wdld0__blk1927_dn8, locals.var_wdld0__blk1927_dn9, locals.var_wdld0__blk1927_dn10, locals.var_wdld0__blk1927_dn13,)
    }
};
        locals.var_wdld0__blk1927 = assign83200_e126409;
        locals.var_wdld0__blk1927_dn0 = assign83200_e126409_d_n0;
        locals.var_wdld0__blk1927_dn2 = assign83200_e126409_d_n2;
        locals.var_wdld0__blk1927_dn4 = assign83200_e126409_d_n4;
        locals.var_wdld0__blk1927_dn5 = assign83200_e126409_d_n5;
        locals.var_wdld0__blk1927_dn6 = assign83200_e126409_d_n6;
        locals.var_wdld0__blk1927_dn7 = assign83200_e126409_d_n7;
        locals.var_wdld0__blk1927_dn8 = assign83200_e126409_d_n8;
        locals.var_wdld0__blk1927_dn9 = assign83200_e126409_d_n9;
        locals.var_wdld0__blk1927_dn10 = assign83200_e126409_d_n10;
        locals.var_wdld0__blk1927_dn13 = assign83200_e126409_d_n13;
        locals.var_wdld0__blk1927_rv = 0.0;

        let assign83210_e126412: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1929 = assign83210_e126412;
        locals.var_guard1929_rv = 0.0;

        let assign83220_e126417: f64 = (locals.var_ddriftldc * 0.1);
        let assign83220_e126418: f64 = (locals.var_ddriftldc - assign83220_e126417);
        let assign83220_e126422: f64 = (locals.var_ddriftldc * 0.1);
        let assign83220_e126425: f64 = if ((locals.var_wdld0__blk1927 > assign83220_e126418) && (assign83220_e126422 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1930 = assign83220_e126425;
        locals.var_guard1930_rv = 0.0;

        let (assign83230_e126442, assign83230_e126442_d_n0, assign83230_e126442_d_n2, assign83230_e126442_d_n4, assign83230_e126442_d_n5, assign83230_e126442_d_n6, assign83230_e126442_d_n7, assign83230_e126442_d_n8, assign83230_e126442_d_n9, assign83230_e126442_d_n10, assign83230_e126442_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83230_e126436: f64 = (locals.var_wdld0__blk1927 - locals.var_ddriftldc);
        let assign83230_e126439: f64 = (locals.var_ddriftldc * 0.1);
        let assign83230_e126440: f64 = (assign83230_e126436 + assign83230_e126439);
        (assign83230_e126440, ((locals.var_wdld0__blk1927_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk1927_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk1927_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk1927_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk1927_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk1927_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk1927_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk1927_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk1927_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk1927_dn13 - locals.var_ddriftldc_dn13) + (locals.var_ddriftldc_dn13 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign83230_e126442;
        locals.var_tmf1_dn0 = assign83230_e126442_d_n0;
        locals.var_tmf1_dn2 = assign83230_e126442_d_n2;
        locals.var_tmf1_dn4 = assign83230_e126442_d_n4;
        locals.var_tmf1_dn5 = assign83230_e126442_d_n5;
        locals.var_tmf1_dn6 = assign83230_e126442_d_n6;
        locals.var_tmf1_dn7 = assign83230_e126442_d_n7;
        locals.var_tmf1_dn8 = assign83230_e126442_d_n8;
        locals.var_tmf1_dn9 = assign83230_e126442_d_n9;
        locals.var_tmf1_dn10 = assign83230_e126442_d_n10;
        locals.var_tmf1_dn13 = assign83230_e126442_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign83240_e126455, assign83240_e126455_d_n0, assign83240_e126455_d_n2, assign83240_e126455_d_n4, assign83240_e126455_d_n5, assign83240_e126455_d_n6, assign83240_e126455_d_n7, assign83240_e126455_d_n8, assign83240_e126455_d_n9, assign83240_e126455_d_n10, assign83240_e126455_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83240_e126453: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign83240_e126453, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign83240_e126455;
        locals.var_x2_dn0 = assign83240_e126455_d_n0;
        locals.var_x2_dn2 = assign83240_e126455_d_n2;
        locals.var_x2_dn4 = assign83240_e126455_d_n4;
        locals.var_x2_dn5 = assign83240_e126455_d_n5;
        locals.var_x2_dn6 = assign83240_e126455_d_n6;
        locals.var_x2_dn7 = assign83240_e126455_d_n7;
        locals.var_x2_dn8 = assign83240_e126455_d_n8;
        locals.var_x2_dn9 = assign83240_e126455_d_n9;
        locals.var_x2_dn10 = assign83240_e126455_d_n10;
        locals.var_x2_dn13 = assign83240_e126455_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign83250_e126472, assign83250_e126472_d_n0, assign83250_e126472_d_n2, assign83250_e126472_d_n4, assign83250_e126472_d_n5, assign83250_e126472_d_n6, assign83250_e126472_d_n7, assign83250_e126472_d_n8, assign83250_e126472_d_n9, assign83250_e126472_d_n10, assign83250_e126472_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83250_e126466: f64 = (locals.var_ddriftldc * 0.1);
        let assign83250_e126469: f64 = (locals.var_ddriftldc * 0.1);
        let assign83250_e126470: f64 = (assign83250_e126466 * assign83250_e126469);
        (assign83250_e126470, (((locals.var_ddriftldc_dn0 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn13 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn13 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign83250_e126472;
        locals.var_xmax2_dn0 = assign83250_e126472_d_n0;
        locals.var_xmax2_dn2 = assign83250_e126472_d_n2;
        locals.var_xmax2_dn4 = assign83250_e126472_d_n4;
        locals.var_xmax2_dn5 = assign83250_e126472_d_n5;
        locals.var_xmax2_dn6 = assign83250_e126472_d_n6;
        locals.var_xmax2_dn7 = assign83250_e126472_d_n7;
        locals.var_xmax2_dn8 = assign83250_e126472_d_n8;
        locals.var_xmax2_dn9 = assign83250_e126472_d_n9;
        locals.var_xmax2_dn10 = assign83250_e126472_d_n10;
        locals.var_xmax2_dn13 = assign83250_e126472_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign83260_e126483, assign83260_e126483_d_n0, assign83260_e126483_d_n2, assign83260_e126483_d_n4, assign83260_e126483_d_n5, assign83260_e126483_d_n6, assign83260_e126483_d_n7, assign83260_e126483_d_n8, assign83260_e126483_d_n9, assign83260_e126483_d_n10, assign83260_e126483_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83260_e126483;
        locals.var_xp_dn0 = assign83260_e126483_d_n0;
        locals.var_xp_dn2 = assign83260_e126483_d_n2;
        locals.var_xp_dn4 = assign83260_e126483_d_n4;
        locals.var_xp_dn5 = assign83260_e126483_d_n5;
        locals.var_xp_dn6 = assign83260_e126483_d_n6;
        locals.var_xp_dn7 = assign83260_e126483_d_n7;
        locals.var_xp_dn8 = assign83260_e126483_d_n8;
        locals.var_xp_dn9 = assign83260_e126483_d_n9;
        locals.var_xp_dn10 = assign83260_e126483_d_n10;
        locals.var_xp_dn13 = assign83260_e126483_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign83270_e126494, assign83270_e126494_d_n0, assign83270_e126494_d_n2, assign83270_e126494_d_n4, assign83270_e126494_d_n5, assign83270_e126494_d_n6, assign83270_e126494_d_n7, assign83270_e126494_d_n8, assign83270_e126494_d_n9, assign83270_e126494_d_n10, assign83270_e126494_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83270_e126494;
        locals.var_xmp_dn0 = assign83270_e126494_d_n0;
        locals.var_xmp_dn2 = assign83270_e126494_d_n2;
        locals.var_xmp_dn4 = assign83270_e126494_d_n4;
        locals.var_xmp_dn5 = assign83270_e126494_d_n5;
        locals.var_xmp_dn6 = assign83270_e126494_d_n6;
        locals.var_xmp_dn7 = assign83270_e126494_d_n7;
        locals.var_xmp_dn8 = assign83270_e126494_d_n8;
        locals.var_xmp_dn9 = assign83270_e126494_d_n9;
        locals.var_xmp_dn10 = assign83270_e126494_d_n10;
        locals.var_xmp_dn13 = assign83270_e126494_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign83280_e126505,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83280_e126505;
        locals.var_m0_rv = 0.0;

        let (assign83290_e126516,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83290_e126516;
        locals.var_mm_rv = 0.0;

        let (assign83300_e126527, assign83300_e126527_d_n0, assign83300_e126527_d_n2, assign83300_e126527_d_n4, assign83300_e126527_d_n5, assign83300_e126527_d_n6, assign83300_e126527_d_n7, assign83300_e126527_d_n8, assign83300_e126527_d_n9, assign83300_e126527_d_n10, assign83300_e126527_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign83300_e126527;
        locals.var_arg_dn0 = assign83300_e126527_d_n0;
        locals.var_arg_dn2 = assign83300_e126527_d_n2;
        locals.var_arg_dn4 = assign83300_e126527_d_n4;
        locals.var_arg_dn5 = assign83300_e126527_d_n5;
        locals.var_arg_dn6 = assign83300_e126527_d_n6;
        locals.var_arg_dn7 = assign83300_e126527_d_n7;
        locals.var_arg_dn8 = assign83300_e126527_d_n8;
        locals.var_arg_dn9 = assign83300_e126527_d_n9;
        locals.var_arg_dn10 = assign83300_e126527_d_n10;
        locals.var_arg_dn13 = assign83300_e126527_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign83310_e126538, assign83310_e126538_d_n0, assign83310_e126538_d_n2, assign83310_e126538_d_n4, assign83310_e126538_d_n5, assign83310_e126538_d_n6, assign83310_e126538_d_n7, assign83310_e126538_d_n8, assign83310_e126538_d_n9, assign83310_e126538_d_n10, assign83310_e126538_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83310_e126538;
        locals.var_dnm_dn0 = assign83310_e126538_d_n0;
        locals.var_dnm_dn2 = assign83310_e126538_d_n2;
        locals.var_dnm_dn4 = assign83310_e126538_d_n4;
        locals.var_dnm_dn5 = assign83310_e126538_d_n5;
        locals.var_dnm_dn6 = assign83310_e126538_d_n6;
        locals.var_dnm_dn7 = assign83310_e126538_d_n7;
        locals.var_dnm_dn8 = assign83310_e126538_d_n8;
        locals.var_dnm_dn9 = assign83310_e126538_d_n9;
        locals.var_dnm_dn10 = assign83310_e126538_d_n10;
        locals.var_dnm_dn13 = assign83310_e126538_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign83320_e126551, assign83320_e126551_d_n0, assign83320_e126551_d_n2, assign83320_e126551_d_n4, assign83320_e126551_d_n5, assign83320_e126551_d_n6, assign83320_e126551_d_n7, assign83320_e126551_d_n8, assign83320_e126551_d_n9, assign83320_e126551_d_n10, assign83320_e126551_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83320_e126549: f64 = (locals.var_xp * locals.var_x2);
        (assign83320_e126549, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83320_e126551;
        locals.var_xp_dn0 = assign83320_e126551_d_n0;
        locals.var_xp_dn2 = assign83320_e126551_d_n2;
        locals.var_xp_dn4 = assign83320_e126551_d_n4;
        locals.var_xp_dn5 = assign83320_e126551_d_n5;
        locals.var_xp_dn6 = assign83320_e126551_d_n6;
        locals.var_xp_dn7 = assign83320_e126551_d_n7;
        locals.var_xp_dn8 = assign83320_e126551_d_n8;
        locals.var_xp_dn9 = assign83320_e126551_d_n9;
        locals.var_xp_dn10 = assign83320_e126551_d_n10;
        locals.var_xp_dn13 = assign83320_e126551_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign83330_e126564, assign83330_e126564_d_n0, assign83330_e126564_d_n2, assign83330_e126564_d_n4, assign83330_e126564_d_n5, assign83330_e126564_d_n6, assign83330_e126564_d_n7, assign83330_e126564_d_n8, assign83330_e126564_d_n9, assign83330_e126564_d_n10, assign83330_e126564_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83330_e126562: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83330_e126562, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83330_e126564;
        locals.var_xmp_dn0 = assign83330_e126564_d_n0;
        locals.var_xmp_dn2 = assign83330_e126564_d_n2;
        locals.var_xmp_dn4 = assign83330_e126564_d_n4;
        locals.var_xmp_dn5 = assign83330_e126564_d_n5;
        locals.var_xmp_dn6 = assign83330_e126564_d_n6;
        locals.var_xmp_dn7 = assign83330_e126564_d_n7;
        locals.var_xmp_dn8 = assign83330_e126564_d_n8;
        locals.var_xmp_dn9 = assign83330_e126564_d_n9;
        locals.var_xmp_dn10 = assign83330_e126564_d_n10;
        locals.var_xmp_dn13 = assign83330_e126564_d_n13;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_306(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign83340_e126577, assign83340_e126577_d_n0, assign83340_e126577_d_n2, assign83340_e126577_d_n4, assign83340_e126577_d_n5, assign83340_e126577_d_n6, assign83340_e126577_d_n7, assign83340_e126577_d_n8, assign83340_e126577_d_n9, assign83340_e126577_d_n10, assign83340_e126577_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83340_e126575: f64 = (locals.var_xp * locals.var_x2);
        (assign83340_e126575, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83340_e126577;
        locals.var_xp_dn0 = assign83340_e126577_d_n0;
        locals.var_xp_dn2 = assign83340_e126577_d_n2;
        locals.var_xp_dn4 = assign83340_e126577_d_n4;
        locals.var_xp_dn5 = assign83340_e126577_d_n5;
        locals.var_xp_dn6 = assign83340_e126577_d_n6;
        locals.var_xp_dn7 = assign83340_e126577_d_n7;
        locals.var_xp_dn8 = assign83340_e126577_d_n8;
        locals.var_xp_dn9 = assign83340_e126577_d_n9;
        locals.var_xp_dn10 = assign83340_e126577_d_n10;
        locals.var_xp_dn13 = assign83340_e126577_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign83350_e126590, assign83350_e126590_d_n0, assign83350_e126590_d_n2, assign83350_e126590_d_n4, assign83350_e126590_d_n5, assign83350_e126590_d_n6, assign83350_e126590_d_n7, assign83350_e126590_d_n8, assign83350_e126590_d_n9, assign83350_e126590_d_n10, assign83350_e126590_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83350_e126588: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83350_e126588, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83350_e126590;
        locals.var_xmp_dn0 = assign83350_e126590_d_n0;
        locals.var_xmp_dn2 = assign83350_e126590_d_n2;
        locals.var_xmp_dn4 = assign83350_e126590_d_n4;
        locals.var_xmp_dn5 = assign83350_e126590_d_n5;
        locals.var_xmp_dn6 = assign83350_e126590_d_n6;
        locals.var_xmp_dn7 = assign83350_e126590_d_n7;
        locals.var_xmp_dn8 = assign83350_e126590_d_n8;
        locals.var_xmp_dn9 = assign83350_e126590_d_n9;
        locals.var_xmp_dn10 = assign83350_e126590_d_n10;
        locals.var_xmp_dn13 = assign83350_e126590_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign83360_e126603, assign83360_e126603_d_n0, assign83360_e126603_d_n2, assign83360_e126603_d_n4, assign83360_e126603_d_n5, assign83360_e126603_d_n6, assign83360_e126603_d_n7, assign83360_e126603_d_n8, assign83360_e126603_d_n9, assign83360_e126603_d_n10, assign83360_e126603_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83360_e126601: f64 = (locals.var_xp + locals.var_xmp);
        (assign83360_e126601, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign83360_e126603;
        locals.var_arg_dn0 = assign83360_e126603_d_n0;
        locals.var_arg_dn2 = assign83360_e126603_d_n2;
        locals.var_arg_dn4 = assign83360_e126603_d_n4;
        locals.var_arg_dn5 = assign83360_e126603_d_n5;
        locals.var_arg_dn6 = assign83360_e126603_d_n6;
        locals.var_arg_dn7 = assign83360_e126603_d_n7;
        locals.var_arg_dn8 = assign83360_e126603_d_n8;
        locals.var_arg_dn9 = assign83360_e126603_d_n9;
        locals.var_arg_dn10 = assign83360_e126603_d_n10;
        locals.var_arg_dn13 = assign83360_e126603_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign83370_e126614, assign83370_e126614_d_n0, assign83370_e126614_d_n2, assign83370_e126614_d_n4, assign83370_e126614_d_n5, assign83370_e126614_d_n6, assign83370_e126614_d_n7, assign83370_e126614_d_n8, assign83370_e126614_d_n9, assign83370_e126614_d_n10, assign83370_e126614_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83370_e126614;
        locals.var_dnm_dn0 = assign83370_e126614_d_n0;
        locals.var_dnm_dn2 = assign83370_e126614_d_n2;
        locals.var_dnm_dn4 = assign83370_e126614_d_n4;
        locals.var_dnm_dn5 = assign83370_e126614_d_n5;
        locals.var_dnm_dn6 = assign83370_e126614_d_n6;
        locals.var_dnm_dn7 = assign83370_e126614_d_n7;
        locals.var_dnm_dn8 = assign83370_e126614_d_n8;
        locals.var_dnm_dn9 = assign83370_e126614_d_n9;
        locals.var_dnm_dn10 = assign83370_e126614_d_n10;
        locals.var_dnm_dn13 = assign83370_e126614_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign83380_e126629: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1931 = assign83380_e126629;
        locals.var_guard1931_rv = 0.0;

        let assign83390_e126632: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1932 = assign83390_e126632;
        locals.var_guard1932_rv = 0.0;

        let (assign83400_e126647,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83400_e126647;
        locals.var_mm_rv = 0.0;

        let assign83410_e126650: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1933 = assign83410_e126650;
        locals.var_guard1933_rv = 0.0;

        let (assign83420_e126668,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 == 0.0)) && (locals.var_guard1933 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83420_e126668;
        locals.var_mm_rv = 0.0;

        let assign83430_e126671: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1934 = assign83430_e126671;
        locals.var_guard1934_rv = 0.0;

        let (assign83440_e126692,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 == 0.0)) && (locals.var_guard1933 == 0.0)) && (locals.var_guard1934 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83440_e126692;
        locals.var_mm_rv = 0.0;

        let assign83450_e126695: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1935 = assign83450_e126695;
        locals.var_guard1935_rv = 0.0;

        let (assign83460_e126719,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 == 0.0)) && (locals.var_guard1933 == 0.0)) && (locals.var_guard1934 == 0.0)) && (locals.var_guard1935 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83460_e126719;
        locals.var_mm_rv = 0.0;

        let (assign83470_e126732,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83470_e126732;
        locals.var_m0_rv = 0.0;

        let mut assign83480_loop_guard: usize = 0;
        while {
            let assign83480_cond_e126746: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign83480_cond_e126746 != 0.0
        } {
            assign83480_loop_guard += 1;
            assert!(assign83480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign83480_body0_e126760, assign83480_body0_e126760_d_n0, assign83480_body0_e126760_d_n2, assign83480_body0_e126760_d_n4, assign83480_body0_e126760_d_n5, assign83480_body0_e126760_d_n6, assign83480_body0_e126760_d_n7, assign83480_body0_e126760_d_n8, assign83480_body0_e126760_d_n9, assign83480_body0_e126760_d_n10, assign83480_body0_e126760_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) {
        let assign83480_body0_e126758: f64 = (locals.var_dnm).sqrt();
        (assign83480_body0_e126758, (locals.var_dnm_dn0 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn2 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn4 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn5 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn6 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn7 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn8 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn9 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn10 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn13 / (2.0 * assign83480_body0_e126758)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign83480_body0_e126760;
            locals.var_dnm_dn0 = assign83480_body0_e126760_d_n0;
            locals.var_dnm_dn2 = assign83480_body0_e126760_d_n2;
            locals.var_dnm_dn4 = assign83480_body0_e126760_d_n4;
            locals.var_dnm_dn5 = assign83480_body0_e126760_d_n5;
            locals.var_dnm_dn6 = assign83480_body0_e126760_d_n6;
            locals.var_dnm_dn7 = assign83480_body0_e126760_d_n7;
            locals.var_dnm_dn8 = assign83480_body0_e126760_d_n8;
            locals.var_dnm_dn9 = assign83480_body0_e126760_d_n9;
            locals.var_dnm_dn10 = assign83480_body0_e126760_d_n10;
            locals.var_dnm_dn13 = assign83480_body0_e126760_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign83480_body1_e126775,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) {
        let assign83480_body1_e126773: f64 = (locals.var_m0 + 1.0);
        (assign83480_body1_e126773,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign83480_body1_e126775;
            locals.var_m0_rv = 0.0;
        }

        let (assign83490_e126800, assign83490_e126800_d_n0, assign83490_e126800_d_n2, assign83490_e126800_d_n4, assign83490_e126800_d_n5, assign83490_e126800_d_n6, assign83490_e126800_d_n7, assign83490_e126800_d_n8, assign83490_e126800_d_n9, assign83490_e126800_d_n10, assign83490_e126800_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 == 0.0)) {
        let (assign83490_e126798, assign83490_e126798_d_n0, assign83490_e126798_d_n2, assign83490_e126798_d_n4, assign83490_e126798_d_n5, assign83490_e126798_d_n6, assign83490_e126798_d_n7, assign83490_e126798_d_n8, assign83490_e126798_d_n9, assign83490_e126798_d_n10, assign83490_e126798_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign83490_e126795: f64 = (2.0 * 2.0);
                let assign83490_e126796: f64 = (1.0 / assign83490_e126795);
                let assign83490_e126797: f64 = (locals.var_dnm).powf(assign83490_e126796);
                (assign83490_e126797, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn0)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn2)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn4)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn5)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn6)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn7)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn8)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn9)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn10)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn13)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign83490_e126798, assign83490_e126798_d_n0, assign83490_e126798_d_n2, assign83490_e126798_d_n4, assign83490_e126798_d_n5, assign83490_e126798_d_n6, assign83490_e126798_d_n7, assign83490_e126798_d_n8, assign83490_e126798_d_n9, assign83490_e126798_d_n10, assign83490_e126798_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83490_e126800;
        locals.var_dnm_dn0 = assign83490_e126800_d_n0;
        locals.var_dnm_dn2 = assign83490_e126800_d_n2;
        locals.var_dnm_dn4 = assign83490_e126800_d_n4;
        locals.var_dnm_dn5 = assign83490_e126800_d_n5;
        locals.var_dnm_dn6 = assign83490_e126800_d_n6;
        locals.var_dnm_dn7 = assign83490_e126800_d_n7;
        locals.var_dnm_dn8 = assign83490_e126800_d_n8;
        locals.var_dnm_dn9 = assign83490_e126800_d_n9;
        locals.var_dnm_dn10 = assign83490_e126800_d_n10;
        locals.var_dnm_dn13 = assign83490_e126800_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign83500_e126813, assign83500_e126813_d_n0, assign83500_e126813_d_n2, assign83500_e126813_d_n4, assign83500_e126813_d_n5, assign83500_e126813_d_n6, assign83500_e126813_d_n7, assign83500_e126813_d_n8, assign83500_e126813_d_n9, assign83500_e126813_d_n10, assign83500_e126813_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83500_e126811: f64 = (1.0 / locals.var_dnm);
        (assign83500_e126811, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83500_e126813;
        locals.var_dnm_dn0 = assign83500_e126813_d_n0;
        locals.var_dnm_dn2 = assign83500_e126813_d_n2;
        locals.var_dnm_dn4 = assign83500_e126813_d_n4;
        locals.var_dnm_dn5 = assign83500_e126813_d_n5;
        locals.var_dnm_dn6 = assign83500_e126813_d_n6;
        locals.var_dnm_dn7 = assign83500_e126813_d_n7;
        locals.var_dnm_dn8 = assign83500_e126813_d_n8;
        locals.var_dnm_dn9 = assign83500_e126813_d_n9;
        locals.var_dnm_dn10 = assign83500_e126813_d_n10;
        locals.var_dnm_dn13 = assign83500_e126813_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign83510_e126830, assign83510_e126830_d_n0, assign83510_e126830_d_n2, assign83510_e126830_d_n4, assign83510_e126830_d_n5, assign83510_e126830_d_n6, assign83510_e126830_d_n7, assign83510_e126830_d_n8, assign83510_e126830_d_n9, assign83510_e126830_d_n10, assign83510_e126830_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83510_e126825: f64 = (locals.var_ddriftldc * 0.1);
        let assign83510_e126826: f64 = (locals.var_tmf1 * assign83510_e126825);
        let assign83510_e126828: f64 = (assign83510_e126826 * locals.var_dnm);
        (assign83510_e126828, ((((locals.var_tmf1_dn0 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn13 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign83510_e126830;
        locals.var_tmf0_dn0 = assign83510_e126830_d_n0;
        locals.var_tmf0_dn2 = assign83510_e126830_d_n2;
        locals.var_tmf0_dn4 = assign83510_e126830_d_n4;
        locals.var_tmf0_dn5 = assign83510_e126830_d_n5;
        locals.var_tmf0_dn6 = assign83510_e126830_d_n6;
        locals.var_tmf0_dn7 = assign83510_e126830_d_n7;
        locals.var_tmf0_dn8 = assign83510_e126830_d_n8;
        locals.var_tmf0_dn9 = assign83510_e126830_d_n9;
        locals.var_tmf0_dn10 = assign83510_e126830_d_n10;
        locals.var_tmf0_dn13 = assign83510_e126830_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign83520_e126849, assign83520_e126849_d_n0, assign83520_e126849_d_n2, assign83520_e126849_d_n4, assign83520_e126849_d_n5, assign83520_e126849_d_n6, assign83520_e126849_d_n7, assign83520_e126849_d_n8, assign83520_e126849_d_n9, assign83520_e126849_d_n10, assign83520_e126849_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83520_e126841: f64 = (locals.var_ddriftldc * 0.1);
        let assign83520_e126843: f64 = (assign83520_e126841 * locals.var_xmp);
        let assign83520_e126845: f64 = (assign83520_e126843 * locals.var_dnm);
        let assign83520_e126847: f64 = (assign83520_e126845 / locals.var_arg);
        (assign83520_e126847, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn0)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn2)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn4)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn5)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn6)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn7)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn8)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn9)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn10)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn13 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn13)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83520_e126849;
        locals.var_t0_dn0 = assign83520_e126849_d_n0;
        locals.var_t0_dn2 = assign83520_e126849_d_n2;
        locals.var_t0_dn4 = assign83520_e126849_d_n4;
        locals.var_t0_dn5 = assign83520_e126849_d_n5;
        locals.var_t0_dn6 = assign83520_e126849_d_n6;
        locals.var_t0_dn7 = assign83520_e126849_d_n7;
        locals.var_t0_dn8 = assign83520_e126849_d_n8;
        locals.var_t0_dn9 = assign83520_e126849_d_n9;
        locals.var_t0_dn10 = assign83520_e126849_d_n10;
        locals.var_t0_dn13 = assign83520_e126849_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign83530_e126866, assign83530_e126866_d_n0, assign83530_e126866_d_n2, assign83530_e126866_d_n4, assign83530_e126866_d_n5, assign83530_e126866_d_n6, assign83530_e126866_d_n7, assign83530_e126866_d_n8, assign83530_e126866_d_n9, assign83530_e126866_d_n10, assign83530_e126866_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83530_e126861: f64 = (locals.var_ddriftldc * 0.1);
        let assign83530_e126862: f64 = (locals.var_ddriftldc - assign83530_e126861);
        let assign83530_e126864: f64 = (assign83530_e126862 + locals.var_tmf0);
        (assign83530_e126864, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn13 - (locals.var_ddriftldc_dn13 * 0.1)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign83530_e126866;
        locals.var_t1_dn0 = assign83530_e126866_d_n0;
        locals.var_t1_dn2 = assign83530_e126866_d_n2;
        locals.var_t1_dn4 = assign83530_e126866_d_n4;
        locals.var_t1_dn5 = assign83530_e126866_d_n5;
        locals.var_t1_dn6 = assign83530_e126866_d_n6;
        locals.var_t1_dn7 = assign83530_e126866_d_n7;
        locals.var_t1_dn8 = assign83530_e126866_d_n8;
        locals.var_t1_dn9 = assign83530_e126866_d_n9;
        locals.var_t1_dn10 = assign83530_e126866_d_n10;
        locals.var_t1_dn13 = assign83530_e126866_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign83540_e126877, assign83540_e126877_d_n0, assign83540_e126877_d_n2, assign83540_e126877_d_n4, assign83540_e126877_d_n5, assign83540_e126877_d_n6, assign83540_e126877_d_n7, assign83540_e126877_d_n8, assign83540_e126877_d_n9, assign83540_e126877_d_n10, assign83540_e126877_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83540_e126877;
        locals.var_t0_dn0 = assign83540_e126877_d_n0;
        locals.var_t0_dn2 = assign83540_e126877_d_n2;
        locals.var_t0_dn4 = assign83540_e126877_d_n4;
        locals.var_t0_dn5 = assign83540_e126877_d_n5;
        locals.var_t0_dn6 = assign83540_e126877_d_n6;
        locals.var_t0_dn7 = assign83540_e126877_d_n7;
        locals.var_t0_dn8 = assign83540_e126877_d_n8;
        locals.var_t0_dn9 = assign83540_e126877_d_n9;
        locals.var_t0_dn10 = assign83540_e126877_d_n10;
        locals.var_t0_dn13 = assign83540_e126877_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign83550_e126889, assign83550_e126889_d_n0, assign83550_e126889_d_n2, assign83550_e126889_d_n4, assign83550_e126889_d_n5, assign83550_e126889_d_n6, assign83550_e126889_d_n7, assign83550_e126889_d_n8, assign83550_e126889_d_n9, assign83550_e126889_d_n10, assign83550_e126889_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 == 0.0)) {
        (locals.var_wdld0__blk1927, locals.var_wdld0__blk1927_dn0, locals.var_wdld0__blk1927_dn2, locals.var_wdld0__blk1927_dn4, locals.var_wdld0__blk1927_dn5, locals.var_wdld0__blk1927_dn6, locals.var_wdld0__blk1927_dn7, locals.var_wdld0__blk1927_dn8, locals.var_wdld0__blk1927_dn9, locals.var_wdld0__blk1927_dn10, locals.var_wdld0__blk1927_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign83550_e126889;
        locals.var_t1_dn0 = assign83550_e126889_d_n0;
        locals.var_t1_dn2 = assign83550_e126889_d_n2;
        locals.var_t1_dn4 = assign83550_e126889_d_n4;
        locals.var_t1_dn5 = assign83550_e126889_d_n5;
        locals.var_t1_dn6 = assign83550_e126889_d_n6;
        locals.var_t1_dn7 = assign83550_e126889_d_n7;
        locals.var_t1_dn8 = assign83550_e126889_d_n8;
        locals.var_t1_dn9 = assign83550_e126889_d_n9;
        locals.var_t1_dn10 = assign83550_e126889_d_n10;
        locals.var_t1_dn13 = assign83550_e126889_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign83560_e126901, assign83560_e126901_d_n0, assign83560_e126901_d_n2, assign83560_e126901_d_n4, assign83560_e126901_d_n5, assign83560_e126901_d_n6, assign83560_e126901_d_n7, assign83560_e126901_d_n8, assign83560_e126901_d_n9, assign83560_e126901_d_n10, assign83560_e126901_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83560_e126901;
        locals.var_t0_dn0 = assign83560_e126901_d_n0;
        locals.var_t0_dn2 = assign83560_e126901_d_n2;
        locals.var_t0_dn4 = assign83560_e126901_d_n4;
        locals.var_t0_dn5 = assign83560_e126901_d_n5;
        locals.var_t0_dn6 = assign83560_e126901_d_n6;
        locals.var_t0_dn7 = assign83560_e126901_d_n7;
        locals.var_t0_dn8 = assign83560_e126901_d_n8;
        locals.var_t0_dn9 = assign83560_e126901_d_n9;
        locals.var_t0_dn10 = assign83560_e126901_d_n10;
        locals.var_t0_dn13 = assign83560_e126901_d_n13;
        locals.var_t0_rv = 0.0;

        let assign83570_e126904: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1936 = assign83570_e126904;
        locals.var_guard1936_rv = 0.0;

        let (assign83580_e126917,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1936 != 0.0)) {
        let assign83580_e126915: f64 = (locals.var_flg_fd_mode__blk1889 + 2.0);
        (assign83580_e126915,)
    } else {
        (locals.var_flg_fd_mode__blk1889,)
    }
};
        locals.var_flg_fd_mode__blk1889 = assign83580_e126917;
        locals.var_flg_fd_mode__blk1889_rv = 0.0;

        let (assign83590_e126932, assign83590_e126932_d_n0, assign83590_e126932_d_n2, assign83590_e126932_d_n4, assign83590_e126932_d_n5, assign83590_e126932_d_n6, assign83590_e126932_d_n7, assign83590_e126932_d_n8, assign83590_e126932_d_n9, assign83590_e126932_d_n10, assign83590_e126932_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 == 0.0)) {
        let (assign83590_e126930, assign83590_e126930_d_n0, assign83590_e126930_d_n2, assign83590_e126930_d_n4, assign83590_e126930_d_n5, assign83590_e126930_d_n6, assign83590_e126930_d_n7, assign83590_e126930_d_n8, assign83590_e126930_d_n9, assign83590_e126930_d_n10, assign83590_e126930_d_n13,) = {
            if (locals.var_wdld0__blk1927 <= locals.var_ddriftldc) {
                (locals.var_wdld0__blk1927, locals.var_wdld0__blk1927_dn0, locals.var_wdld0__blk1927_dn2, locals.var_wdld0__blk1927_dn4, locals.var_wdld0__blk1927_dn5, locals.var_wdld0__blk1927_dn6, locals.var_wdld0__blk1927_dn7, locals.var_wdld0__blk1927_dn8, locals.var_wdld0__blk1927_dn9, locals.var_wdld0__blk1927_dn10, locals.var_wdld0__blk1927_dn13,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
            }
        };
        (assign83590_e126930, assign83590_e126930_d_n0, assign83590_e126930_d_n2, assign83590_e126930_d_n4, assign83590_e126930_d_n5, assign83590_e126930_d_n6, assign83590_e126930_d_n7, assign83590_e126930_d_n8, assign83590_e126930_d_n9, assign83590_e126930_d_n10, assign83590_e126930_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign83590_e126932;
        locals.var_t1_dn0 = assign83590_e126932_d_n0;
        locals.var_t1_dn2 = assign83590_e126932_d_n2;
        locals.var_t1_dn4 = assign83590_e126932_d_n4;
        locals.var_t1_dn5 = assign83590_e126932_d_n5;
        locals.var_t1_dn6 = assign83590_e126932_d_n6;
        locals.var_t1_dn7 = assign83590_e126932_d_n7;
        locals.var_t1_dn8 = assign83590_e126932_d_n8;
        locals.var_t1_dn9 = assign83590_e126932_d_n9;
        locals.var_t1_dn10 = assign83590_e126932_d_n10;
        locals.var_t1_dn13 = assign83590_e126932_d_n13;
        locals.var_t1_rv = 0.0;

        let assign83600_e126935: f64 = if locals.var_wdld0__blk1927 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard1937 = assign83600_e126935;
        locals.var_guard1937_rv = 0.0;

        let (assign83610_e126949,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 == 0.0)) && (locals.var_guard1937 != 0.0)) {
        let assign83610_e126947: f64 = (locals.var_flg_fd_mode__blk1889 + 2.0);
        (assign83610_e126947,)
    } else {
        (locals.var_flg_fd_mode__blk1889,)
    }
};
        locals.var_flg_fd_mode__blk1889 = assign83610_e126949;
        locals.var_flg_fd_mode__blk1889_rv = 0.0;

        let assign83620_e126952: f64 = if locals.var_flg_fd_mode__blk1889 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1938 = assign83620_e126952;
        locals.var_guard1938_rv = 0.0;

        let (assign83630_e126961, assign83630_e126961_d_n0, assign83630_e126961_d_n2, assign83630_e126961_d_n4, assign83630_e126961_d_n5, assign83630_e126961_d_n6, assign83630_e126961_d_n7, assign83630_e126961_d_n8, assign83630_e126961_d_n9, assign83630_e126961_d_n10, assign83630_e126961_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_bef1__blk1928, locals.var_ps0ld_bef1__blk1928_dn0, locals.var_ps0ld_bef1__blk1928_dn2, locals.var_ps0ld_bef1__blk1928_dn4, locals.var_ps0ld_bef1__blk1928_dn5, locals.var_ps0ld_bef1__blk1928_dn6, locals.var_ps0ld_bef1__blk1928_dn7, locals.var_ps0ld_bef1__blk1928_dn8, locals.var_ps0ld_bef1__blk1928_dn9, locals.var_ps0ld_bef1__blk1928_dn10, locals.var_ps0ld_bef1__blk1928_dn13,)
    }
};
        locals.var_ps0ld_bef1__blk1928 = assign83630_e126961;
        locals.var_ps0ld_bef1__blk1928_dn0 = assign83630_e126961_d_n0;
        locals.var_ps0ld_bef1__blk1928_dn2 = assign83630_e126961_d_n2;
        locals.var_ps0ld_bef1__blk1928_dn4 = assign83630_e126961_d_n4;
        locals.var_ps0ld_bef1__blk1928_dn5 = assign83630_e126961_d_n5;
        locals.var_ps0ld_bef1__blk1928_dn6 = assign83630_e126961_d_n6;
        locals.var_ps0ld_bef1__blk1928_dn7 = assign83630_e126961_d_n7;
        locals.var_ps0ld_bef1__blk1928_dn8 = assign83630_e126961_d_n8;
        locals.var_ps0ld_bef1__blk1928_dn9 = assign83630_e126961_d_n9;
        locals.var_ps0ld_bef1__blk1928_dn10 = assign83630_e126961_d_n10;
        locals.var_ps0ld_bef1__blk1928_dn13 = assign83630_e126961_d_n13;
        locals.var_ps0ld_bef1__blk1928_rv = 0.0;

        let (assign83640_e126972, assign83640_e126972_d_n0, assign83640_e126972_d_n2, assign83640_e126972_d_n4, assign83640_e126972_d_n5, assign83640_e126972_d_n6, assign83640_e126972_d_n7, assign83640_e126972_d_n8, assign83640_e126972_d_n9, assign83640_e126972_d_n10, assign83640_e126972_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) {
        let assign83640_e126970: f64 = (locals.var_t1 * locals.var_q_nsubld__blk1883);
        (assign83640_e126970, (locals.var_t1_dn0 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn2 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn4 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn5 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn6 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn7 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn8 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn9 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn10 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn13 * locals.var_q_nsubld__blk1883),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign83640_e126972;
        locals.var_qbuld_dn0 = assign83640_e126972_d_n0;
        locals.var_qbuld_dn2 = assign83640_e126972_d_n2;
        locals.var_qbuld_dn4 = assign83640_e126972_d_n4;
        locals.var_qbuld_dn5 = assign83640_e126972_d_n5;
        locals.var_qbuld_dn6 = assign83640_e126972_d_n6;
        locals.var_qbuld_dn7 = assign83640_e126972_d_n7;
        locals.var_qbuld_dn8 = assign83640_e126972_d_n8;
        locals.var_qbuld_dn9 = assign83640_e126972_d_n9;
        locals.var_qbuld_dn10 = assign83640_e126972_d_n10;
        locals.var_qbuld_dn13 = assign83640_e126972_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign83650_e126985, assign83650_e126985_d_n0, assign83650_e126985_d_n2, assign83650_e126985_d_n4, assign83650_e126985_d_n5, assign83650_e126985_d_n6, assign83650_e126985_d_n7, assign83650_e126985_d_n8, assign83650_e126985_d_n9, assign83650_e126985_d_n10, assign83650_e126985_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) {
        let assign83650_e126982: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign83650_e126983: f64 = (locals.var_vgpld - assign83650_e126982);
        (assign83650_e126983, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (locals.var_vgpld_dn6 - (locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (-(locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn13 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign83650_e126985;
        locals.var_ps0ld_dn0 = assign83650_e126985_d_n0;
        locals.var_ps0ld_dn2 = assign83650_e126985_d_n2;
        locals.var_ps0ld_dn4 = assign83650_e126985_d_n4;
        locals.var_ps0ld_dn5 = assign83650_e126985_d_n5;
        locals.var_ps0ld_dn6 = assign83650_e126985_d_n6;
        locals.var_ps0ld_dn7 = assign83650_e126985_d_n7;
        locals.var_ps0ld_dn8 = assign83650_e126985_d_n8;
        locals.var_ps0ld_dn9 = assign83650_e126985_d_n9;
        locals.var_ps0ld_dn10 = assign83650_e126985_d_n10;
        locals.var_ps0ld_dn13 = assign83650_e126985_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign83660_e126988: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1939 = assign83660_e126988;
        locals.var_guard1939_rv = 0.0;

        let assign83670_e126992: f64 = (locals.var_ps0ld_bef1__blk1928 - 0.1);
        let assign83670_e126997: f64 = if ((locals.var_ps0ld > assign83670_e126992) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1940 = assign83670_e126997;
        locals.var_guard1940_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_307(
        locals: &mut StampLocals,
    ) {
        let (assign83680_e127014, assign83680_e127014_d_n0, assign83680_e127014_d_n2, assign83680_e127014_d_n4, assign83680_e127014_d_n5, assign83680_e127014_d_n6, assign83680_e127014_d_n7, assign83680_e127014_d_n8, assign83680_e127014_d_n9, assign83680_e127014_d_n10, assign83680_e127014_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83680_e127010: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1__blk1928);
        let assign83680_e127012: f64 = (assign83680_e127010 + 0.1);
        (assign83680_e127012, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1__blk1928_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1__blk1928_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1__blk1928_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1__blk1928_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1__blk1928_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1__blk1928_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1__blk1928_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1__blk1928_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1__blk1928_dn10), (locals.var_ps0ld_dn13 - locals.var_ps0ld_bef1__blk1928_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign83680_e127014;
        locals.var_tmf1_dn0 = assign83680_e127014_d_n0;
        locals.var_tmf1_dn2 = assign83680_e127014_d_n2;
        locals.var_tmf1_dn4 = assign83680_e127014_d_n4;
        locals.var_tmf1_dn5 = assign83680_e127014_d_n5;
        locals.var_tmf1_dn6 = assign83680_e127014_d_n6;
        locals.var_tmf1_dn7 = assign83680_e127014_d_n7;
        locals.var_tmf1_dn8 = assign83680_e127014_d_n8;
        locals.var_tmf1_dn9 = assign83680_e127014_d_n9;
        locals.var_tmf1_dn10 = assign83680_e127014_d_n10;
        locals.var_tmf1_dn13 = assign83680_e127014_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign83690_e127029, assign83690_e127029_d_n0, assign83690_e127029_d_n2, assign83690_e127029_d_n4, assign83690_e127029_d_n5, assign83690_e127029_d_n6, assign83690_e127029_d_n7, assign83690_e127029_d_n8, assign83690_e127029_d_n9, assign83690_e127029_d_n10, assign83690_e127029_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83690_e127027: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign83690_e127027, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign83690_e127029;
        locals.var_x2_dn0 = assign83690_e127029_d_n0;
        locals.var_x2_dn2 = assign83690_e127029_d_n2;
        locals.var_x2_dn4 = assign83690_e127029_d_n4;
        locals.var_x2_dn5 = assign83690_e127029_d_n5;
        locals.var_x2_dn6 = assign83690_e127029_d_n6;
        locals.var_x2_dn7 = assign83690_e127029_d_n7;
        locals.var_x2_dn8 = assign83690_e127029_d_n8;
        locals.var_x2_dn9 = assign83690_e127029_d_n9;
        locals.var_x2_dn10 = assign83690_e127029_d_n10;
        locals.var_x2_dn13 = assign83690_e127029_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign83700_e127044, assign83700_e127044_d_n0, assign83700_e127044_d_n2, assign83700_e127044_d_n4, assign83700_e127044_d_n5, assign83700_e127044_d_n6, assign83700_e127044_d_n7, assign83700_e127044_d_n8, assign83700_e127044_d_n9, assign83700_e127044_d_n10, assign83700_e127044_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83700_e127042: f64 = (0.1 * 0.1);
        (assign83700_e127042, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign83700_e127044;
        locals.var_xmax2_dn0 = assign83700_e127044_d_n0;
        locals.var_xmax2_dn2 = assign83700_e127044_d_n2;
        locals.var_xmax2_dn4 = assign83700_e127044_d_n4;
        locals.var_xmax2_dn5 = assign83700_e127044_d_n5;
        locals.var_xmax2_dn6 = assign83700_e127044_d_n6;
        locals.var_xmax2_dn7 = assign83700_e127044_d_n7;
        locals.var_xmax2_dn8 = assign83700_e127044_d_n8;
        locals.var_xmax2_dn9 = assign83700_e127044_d_n9;
        locals.var_xmax2_dn10 = assign83700_e127044_d_n10;
        locals.var_xmax2_dn13 = assign83700_e127044_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign83710_e127057, assign83710_e127057_d_n0, assign83710_e127057_d_n2, assign83710_e127057_d_n4, assign83710_e127057_d_n5, assign83710_e127057_d_n6, assign83710_e127057_d_n7, assign83710_e127057_d_n8, assign83710_e127057_d_n9, assign83710_e127057_d_n10, assign83710_e127057_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83710_e127057;
        locals.var_xp_dn0 = assign83710_e127057_d_n0;
        locals.var_xp_dn2 = assign83710_e127057_d_n2;
        locals.var_xp_dn4 = assign83710_e127057_d_n4;
        locals.var_xp_dn5 = assign83710_e127057_d_n5;
        locals.var_xp_dn6 = assign83710_e127057_d_n6;
        locals.var_xp_dn7 = assign83710_e127057_d_n7;
        locals.var_xp_dn8 = assign83710_e127057_d_n8;
        locals.var_xp_dn9 = assign83710_e127057_d_n9;
        locals.var_xp_dn10 = assign83710_e127057_d_n10;
        locals.var_xp_dn13 = assign83710_e127057_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign83720_e127070, assign83720_e127070_d_n0, assign83720_e127070_d_n2, assign83720_e127070_d_n4, assign83720_e127070_d_n5, assign83720_e127070_d_n6, assign83720_e127070_d_n7, assign83720_e127070_d_n8, assign83720_e127070_d_n9, assign83720_e127070_d_n10, assign83720_e127070_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83720_e127070;
        locals.var_xmp_dn0 = assign83720_e127070_d_n0;
        locals.var_xmp_dn2 = assign83720_e127070_d_n2;
        locals.var_xmp_dn4 = assign83720_e127070_d_n4;
        locals.var_xmp_dn5 = assign83720_e127070_d_n5;
        locals.var_xmp_dn6 = assign83720_e127070_d_n6;
        locals.var_xmp_dn7 = assign83720_e127070_d_n7;
        locals.var_xmp_dn8 = assign83720_e127070_d_n8;
        locals.var_xmp_dn9 = assign83720_e127070_d_n9;
        locals.var_xmp_dn10 = assign83720_e127070_d_n10;
        locals.var_xmp_dn13 = assign83720_e127070_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign83730_e127083,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83730_e127083;
        locals.var_m0_rv = 0.0;

        let (assign83740_e127096,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83740_e127096;
        locals.var_mm_rv = 0.0;

        let (assign83750_e127109, assign83750_e127109_d_n0, assign83750_e127109_d_n2, assign83750_e127109_d_n4, assign83750_e127109_d_n5, assign83750_e127109_d_n6, assign83750_e127109_d_n7, assign83750_e127109_d_n8, assign83750_e127109_d_n9, assign83750_e127109_d_n10, assign83750_e127109_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign83750_e127109;
        locals.var_arg_dn0 = assign83750_e127109_d_n0;
        locals.var_arg_dn2 = assign83750_e127109_d_n2;
        locals.var_arg_dn4 = assign83750_e127109_d_n4;
        locals.var_arg_dn5 = assign83750_e127109_d_n5;
        locals.var_arg_dn6 = assign83750_e127109_d_n6;
        locals.var_arg_dn7 = assign83750_e127109_d_n7;
        locals.var_arg_dn8 = assign83750_e127109_d_n8;
        locals.var_arg_dn9 = assign83750_e127109_d_n9;
        locals.var_arg_dn10 = assign83750_e127109_d_n10;
        locals.var_arg_dn13 = assign83750_e127109_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign83760_e127122, assign83760_e127122_d_n0, assign83760_e127122_d_n2, assign83760_e127122_d_n4, assign83760_e127122_d_n5, assign83760_e127122_d_n6, assign83760_e127122_d_n7, assign83760_e127122_d_n8, assign83760_e127122_d_n9, assign83760_e127122_d_n10, assign83760_e127122_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83760_e127122;
        locals.var_dnm_dn0 = assign83760_e127122_d_n0;
        locals.var_dnm_dn2 = assign83760_e127122_d_n2;
        locals.var_dnm_dn4 = assign83760_e127122_d_n4;
        locals.var_dnm_dn5 = assign83760_e127122_d_n5;
        locals.var_dnm_dn6 = assign83760_e127122_d_n6;
        locals.var_dnm_dn7 = assign83760_e127122_d_n7;
        locals.var_dnm_dn8 = assign83760_e127122_d_n8;
        locals.var_dnm_dn9 = assign83760_e127122_d_n9;
        locals.var_dnm_dn10 = assign83760_e127122_d_n10;
        locals.var_dnm_dn13 = assign83760_e127122_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign83770_e127137, assign83770_e127137_d_n0, assign83770_e127137_d_n2, assign83770_e127137_d_n4, assign83770_e127137_d_n5, assign83770_e127137_d_n6, assign83770_e127137_d_n7, assign83770_e127137_d_n8, assign83770_e127137_d_n9, assign83770_e127137_d_n10, assign83770_e127137_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83770_e127135: f64 = (locals.var_xp * locals.var_x2);
        (assign83770_e127135, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83770_e127137;
        locals.var_xp_dn0 = assign83770_e127137_d_n0;
        locals.var_xp_dn2 = assign83770_e127137_d_n2;
        locals.var_xp_dn4 = assign83770_e127137_d_n4;
        locals.var_xp_dn5 = assign83770_e127137_d_n5;
        locals.var_xp_dn6 = assign83770_e127137_d_n6;
        locals.var_xp_dn7 = assign83770_e127137_d_n7;
        locals.var_xp_dn8 = assign83770_e127137_d_n8;
        locals.var_xp_dn9 = assign83770_e127137_d_n9;
        locals.var_xp_dn10 = assign83770_e127137_d_n10;
        locals.var_xp_dn13 = assign83770_e127137_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign83780_e127152, assign83780_e127152_d_n0, assign83780_e127152_d_n2, assign83780_e127152_d_n4, assign83780_e127152_d_n5, assign83780_e127152_d_n6, assign83780_e127152_d_n7, assign83780_e127152_d_n8, assign83780_e127152_d_n9, assign83780_e127152_d_n10, assign83780_e127152_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83780_e127150: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83780_e127150, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83780_e127152;
        locals.var_xmp_dn0 = assign83780_e127152_d_n0;
        locals.var_xmp_dn2 = assign83780_e127152_d_n2;
        locals.var_xmp_dn4 = assign83780_e127152_d_n4;
        locals.var_xmp_dn5 = assign83780_e127152_d_n5;
        locals.var_xmp_dn6 = assign83780_e127152_d_n6;
        locals.var_xmp_dn7 = assign83780_e127152_d_n7;
        locals.var_xmp_dn8 = assign83780_e127152_d_n8;
        locals.var_xmp_dn9 = assign83780_e127152_d_n9;
        locals.var_xmp_dn10 = assign83780_e127152_d_n10;
        locals.var_xmp_dn13 = assign83780_e127152_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign83790_e127167, assign83790_e127167_d_n0, assign83790_e127167_d_n2, assign83790_e127167_d_n4, assign83790_e127167_d_n5, assign83790_e127167_d_n6, assign83790_e127167_d_n7, assign83790_e127167_d_n8, assign83790_e127167_d_n9, assign83790_e127167_d_n10, assign83790_e127167_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83790_e127165: f64 = (locals.var_xp * locals.var_x2);
        (assign83790_e127165, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83790_e127167;
        locals.var_xp_dn0 = assign83790_e127167_d_n0;
        locals.var_xp_dn2 = assign83790_e127167_d_n2;
        locals.var_xp_dn4 = assign83790_e127167_d_n4;
        locals.var_xp_dn5 = assign83790_e127167_d_n5;
        locals.var_xp_dn6 = assign83790_e127167_d_n6;
        locals.var_xp_dn7 = assign83790_e127167_d_n7;
        locals.var_xp_dn8 = assign83790_e127167_d_n8;
        locals.var_xp_dn9 = assign83790_e127167_d_n9;
        locals.var_xp_dn10 = assign83790_e127167_d_n10;
        locals.var_xp_dn13 = assign83790_e127167_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign83800_e127182, assign83800_e127182_d_n0, assign83800_e127182_d_n2, assign83800_e127182_d_n4, assign83800_e127182_d_n5, assign83800_e127182_d_n6, assign83800_e127182_d_n7, assign83800_e127182_d_n8, assign83800_e127182_d_n9, assign83800_e127182_d_n10, assign83800_e127182_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83800_e127180: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83800_e127180, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83800_e127182;
        locals.var_xmp_dn0 = assign83800_e127182_d_n0;
        locals.var_xmp_dn2 = assign83800_e127182_d_n2;
        locals.var_xmp_dn4 = assign83800_e127182_d_n4;
        locals.var_xmp_dn5 = assign83800_e127182_d_n5;
        locals.var_xmp_dn6 = assign83800_e127182_d_n6;
        locals.var_xmp_dn7 = assign83800_e127182_d_n7;
        locals.var_xmp_dn8 = assign83800_e127182_d_n8;
        locals.var_xmp_dn9 = assign83800_e127182_d_n9;
        locals.var_xmp_dn10 = assign83800_e127182_d_n10;
        locals.var_xmp_dn13 = assign83800_e127182_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign83810_e127197, assign83810_e127197_d_n0, assign83810_e127197_d_n2, assign83810_e127197_d_n4, assign83810_e127197_d_n5, assign83810_e127197_d_n6, assign83810_e127197_d_n7, assign83810_e127197_d_n8, assign83810_e127197_d_n9, assign83810_e127197_d_n10, assign83810_e127197_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83810_e127195: f64 = (locals.var_xp + locals.var_xmp);
        (assign83810_e127195, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign83810_e127197;
        locals.var_arg_dn0 = assign83810_e127197_d_n0;
        locals.var_arg_dn2 = assign83810_e127197_d_n2;
        locals.var_arg_dn4 = assign83810_e127197_d_n4;
        locals.var_arg_dn5 = assign83810_e127197_d_n5;
        locals.var_arg_dn6 = assign83810_e127197_d_n6;
        locals.var_arg_dn7 = assign83810_e127197_d_n7;
        locals.var_arg_dn8 = assign83810_e127197_d_n8;
        locals.var_arg_dn9 = assign83810_e127197_d_n9;
        locals.var_arg_dn10 = assign83810_e127197_d_n10;
        locals.var_arg_dn13 = assign83810_e127197_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign83820_e127210, assign83820_e127210_d_n0, assign83820_e127210_d_n2, assign83820_e127210_d_n4, assign83820_e127210_d_n5, assign83820_e127210_d_n6, assign83820_e127210_d_n7, assign83820_e127210_d_n8, assign83820_e127210_d_n9, assign83820_e127210_d_n10, assign83820_e127210_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83820_e127210;
        locals.var_dnm_dn0 = assign83820_e127210_d_n0;
        locals.var_dnm_dn2 = assign83820_e127210_d_n2;
        locals.var_dnm_dn4 = assign83820_e127210_d_n4;
        locals.var_dnm_dn5 = assign83820_e127210_d_n5;
        locals.var_dnm_dn6 = assign83820_e127210_d_n6;
        locals.var_dnm_dn7 = assign83820_e127210_d_n7;
        locals.var_dnm_dn8 = assign83820_e127210_d_n8;
        locals.var_dnm_dn9 = assign83820_e127210_d_n9;
        locals.var_dnm_dn10 = assign83820_e127210_d_n10;
        locals.var_dnm_dn13 = assign83820_e127210_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign83830_e127225: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1941 = assign83830_e127225;
        locals.var_guard1941_rv = 0.0;

        let assign83840_e127228: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1942 = assign83840_e127228;
        locals.var_guard1942_rv = 0.0;

        let (assign83850_e127245,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83850_e127245;
        locals.var_mm_rv = 0.0;

        let assign83860_e127248: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1943 = assign83860_e127248;
        locals.var_guard1943_rv = 0.0;

        let (assign83870_e127268,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 == 0.0)) && (locals.var_guard1943 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83870_e127268;
        locals.var_mm_rv = 0.0;

        let assign83880_e127271: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1944 = assign83880_e127271;
        locals.var_guard1944_rv = 0.0;

        let (assign83890_e127294,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 == 0.0)) && (locals.var_guard1943 == 0.0)) && (locals.var_guard1944 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83890_e127294;
        locals.var_mm_rv = 0.0;

        let assign83900_e127297: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1945 = assign83900_e127297;
        locals.var_guard1945_rv = 0.0;

        let (assign83910_e127323,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 == 0.0)) && (locals.var_guard1943 == 0.0)) && (locals.var_guard1944 == 0.0)) && (locals.var_guard1945 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83910_e127323;
        locals.var_mm_rv = 0.0;

        let (assign83920_e127338,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83920_e127338;
        locals.var_m0_rv = 0.0;

        let mut assign83930_loop_guard: usize = 0;
        while {
            let assign83930_cond_e127354: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign83930_cond_e127354 != 0.0
        } {
            assign83930_loop_guard += 1;
            assert!(assign83930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign83930_body0_e127370, assign83930_body0_e127370_d_n0, assign83930_body0_e127370_d_n2, assign83930_body0_e127370_d_n4, assign83930_body0_e127370_d_n5, assign83930_body0_e127370_d_n6, assign83930_body0_e127370_d_n7, assign83930_body0_e127370_d_n8, assign83930_body0_e127370_d_n9, assign83930_body0_e127370_d_n10, assign83930_body0_e127370_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) {
        let assign83930_body0_e127368: f64 = (locals.var_dnm).sqrt();
        (assign83930_body0_e127368, (locals.var_dnm_dn0 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn2 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn4 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn5 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn6 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn7 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn8 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn9 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn10 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn13 / (2.0 * assign83930_body0_e127368)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign83930_body0_e127370;
            locals.var_dnm_dn0 = assign83930_body0_e127370_d_n0;
            locals.var_dnm_dn2 = assign83930_body0_e127370_d_n2;
            locals.var_dnm_dn4 = assign83930_body0_e127370_d_n4;
            locals.var_dnm_dn5 = assign83930_body0_e127370_d_n5;
            locals.var_dnm_dn6 = assign83930_body0_e127370_d_n6;
            locals.var_dnm_dn7 = assign83930_body0_e127370_d_n7;
            locals.var_dnm_dn8 = assign83930_body0_e127370_d_n8;
            locals.var_dnm_dn9 = assign83930_body0_e127370_d_n9;
            locals.var_dnm_dn10 = assign83930_body0_e127370_d_n10;
            locals.var_dnm_dn13 = assign83930_body0_e127370_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign83930_body1_e127387,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) {
        let assign83930_body1_e127385: f64 = (locals.var_m0 + 1.0);
        (assign83930_body1_e127385,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign83930_body1_e127387;
            locals.var_m0_rv = 0.0;
        }

        let (assign83940_e127414, assign83940_e127414_d_n0, assign83940_e127414_d_n2, assign83940_e127414_d_n4, assign83940_e127414_d_n5, assign83940_e127414_d_n6, assign83940_e127414_d_n7, assign83940_e127414_d_n8, assign83940_e127414_d_n9, assign83940_e127414_d_n10, assign83940_e127414_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 == 0.0)) {
        let (assign83940_e127412, assign83940_e127412_d_n0, assign83940_e127412_d_n2, assign83940_e127412_d_n4, assign83940_e127412_d_n5, assign83940_e127412_d_n6, assign83940_e127412_d_n7, assign83940_e127412_d_n8, assign83940_e127412_d_n9, assign83940_e127412_d_n10, assign83940_e127412_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign83940_e127409: f64 = (2.0 * 2.0);
                let assign83940_e127410: f64 = (1.0 / assign83940_e127409);
                let assign83940_e127411: f64 = (locals.var_dnm).powf(assign83940_e127410);
                (assign83940_e127411, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn0)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn2)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn4)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn5)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn6)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn7)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn8)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn9)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn10)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn13)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign83940_e127412, assign83940_e127412_d_n0, assign83940_e127412_d_n2, assign83940_e127412_d_n4, assign83940_e127412_d_n5, assign83940_e127412_d_n6, assign83940_e127412_d_n7, assign83940_e127412_d_n8, assign83940_e127412_d_n9, assign83940_e127412_d_n10, assign83940_e127412_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83940_e127414;
        locals.var_dnm_dn0 = assign83940_e127414_d_n0;
        locals.var_dnm_dn2 = assign83940_e127414_d_n2;
        locals.var_dnm_dn4 = assign83940_e127414_d_n4;
        locals.var_dnm_dn5 = assign83940_e127414_d_n5;
        locals.var_dnm_dn6 = assign83940_e127414_d_n6;
        locals.var_dnm_dn7 = assign83940_e127414_d_n7;
        locals.var_dnm_dn8 = assign83940_e127414_d_n8;
        locals.var_dnm_dn9 = assign83940_e127414_d_n9;
        locals.var_dnm_dn10 = assign83940_e127414_d_n10;
        locals.var_dnm_dn13 = assign83940_e127414_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign83950_e127429, assign83950_e127429_d_n0, assign83950_e127429_d_n2, assign83950_e127429_d_n4, assign83950_e127429_d_n5, assign83950_e127429_d_n6, assign83950_e127429_d_n7, assign83950_e127429_d_n8, assign83950_e127429_d_n9, assign83950_e127429_d_n10, assign83950_e127429_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83950_e127427: f64 = (1.0 / locals.var_dnm);
        (assign83950_e127427, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83950_e127429;
        locals.var_dnm_dn0 = assign83950_e127429_d_n0;
        locals.var_dnm_dn2 = assign83950_e127429_d_n2;
        locals.var_dnm_dn4 = assign83950_e127429_d_n4;
        locals.var_dnm_dn5 = assign83950_e127429_d_n5;
        locals.var_dnm_dn6 = assign83950_e127429_d_n6;
        locals.var_dnm_dn7 = assign83950_e127429_d_n7;
        locals.var_dnm_dn8 = assign83950_e127429_d_n8;
        locals.var_dnm_dn9 = assign83950_e127429_d_n9;
        locals.var_dnm_dn10 = assign83950_e127429_d_n10;
        locals.var_dnm_dn13 = assign83950_e127429_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign83960_e127446, assign83960_e127446_d_n0, assign83960_e127446_d_n2, assign83960_e127446_d_n4, assign83960_e127446_d_n5, assign83960_e127446_d_n6, assign83960_e127446_d_n7, assign83960_e127446_d_n8, assign83960_e127446_d_n9, assign83960_e127446_d_n10, assign83960_e127446_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83960_e127442: f64 = (locals.var_tmf1 * 0.1);
        let assign83960_e127444: f64 = (assign83960_e127442 * locals.var_dnm);
        (assign83960_e127444, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign83960_e127446;
        locals.var_tmf0_dn0 = assign83960_e127446_d_n0;
        locals.var_tmf0_dn2 = assign83960_e127446_d_n2;
        locals.var_tmf0_dn4 = assign83960_e127446_d_n4;
        locals.var_tmf0_dn5 = assign83960_e127446_d_n5;
        locals.var_tmf0_dn6 = assign83960_e127446_d_n6;
        locals.var_tmf0_dn7 = assign83960_e127446_d_n7;
        locals.var_tmf0_dn8 = assign83960_e127446_d_n8;
        locals.var_tmf0_dn9 = assign83960_e127446_d_n9;
        locals.var_tmf0_dn10 = assign83960_e127446_d_n10;
        locals.var_tmf0_dn13 = assign83960_e127446_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign83970_e127465, assign83970_e127465_d_n0, assign83970_e127465_d_n2, assign83970_e127465_d_n4, assign83970_e127465_d_n5, assign83970_e127465_d_n6, assign83970_e127465_d_n7, assign83970_e127465_d_n8, assign83970_e127465_d_n9, assign83970_e127465_d_n10, assign83970_e127465_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83970_e127459: f64 = (0.1 * locals.var_xmp);
        let assign83970_e127461: f64 = (assign83970_e127459 * locals.var_dnm);
        let assign83970_e127463: f64 = (assign83970_e127461 / locals.var_arg);
        (assign83970_e127463, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn0)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn2)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn4)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn5)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn6)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn7)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn8)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn9)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn10)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn13)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83970_e127465;
        locals.var_t0_dn0 = assign83970_e127465_d_n0;
        locals.var_t0_dn2 = assign83970_e127465_d_n2;
        locals.var_t0_dn4 = assign83970_e127465_d_n4;
        locals.var_t0_dn5 = assign83970_e127465_d_n5;
        locals.var_t0_dn6 = assign83970_e127465_d_n6;
        locals.var_t0_dn7 = assign83970_e127465_d_n7;
        locals.var_t0_dn8 = assign83970_e127465_d_n8;
        locals.var_t0_dn9 = assign83970_e127465_d_n9;
        locals.var_t0_dn10 = assign83970_e127465_d_n10;
        locals.var_t0_dn13 = assign83970_e127465_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_308(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign83980_e127482, assign83980_e127482_d_n0, assign83980_e127482_d_n2, assign83980_e127482_d_n4, assign83980_e127482_d_n5, assign83980_e127482_d_n6, assign83980_e127482_d_n7, assign83980_e127482_d_n8, assign83980_e127482_d_n9, assign83980_e127482_d_n10, assign83980_e127482_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83980_e127478: f64 = (locals.var_ps0ld_bef1__blk1928 - 0.1);
        let assign83980_e127480: f64 = (assign83980_e127478 + locals.var_tmf0);
        (assign83980_e127480, (locals.var_ps0ld_bef1__blk1928_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1__blk1928_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1__blk1928_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1__blk1928_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1__blk1928_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1__blk1928_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1__blk1928_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1__blk1928_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1__blk1928_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1__blk1928_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign83980_e127482;
        locals.var_ps0ld_dn0 = assign83980_e127482_d_n0;
        locals.var_ps0ld_dn2 = assign83980_e127482_d_n2;
        locals.var_ps0ld_dn4 = assign83980_e127482_d_n4;
        locals.var_ps0ld_dn5 = assign83980_e127482_d_n5;
        locals.var_ps0ld_dn6 = assign83980_e127482_d_n6;
        locals.var_ps0ld_dn7 = assign83980_e127482_d_n7;
        locals.var_ps0ld_dn8 = assign83980_e127482_d_n8;
        locals.var_ps0ld_dn9 = assign83980_e127482_d_n9;
        locals.var_ps0ld_dn10 = assign83980_e127482_d_n10;
        locals.var_ps0ld_dn13 = assign83980_e127482_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign83990_e127495, assign83990_e127495_d_n0, assign83990_e127495_d_n2, assign83990_e127495_d_n4, assign83990_e127495_d_n5, assign83990_e127495_d_n6, assign83990_e127495_d_n7, assign83990_e127495_d_n8, assign83990_e127495_d_n9, assign83990_e127495_d_n10, assign83990_e127495_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83990_e127495;
        locals.var_t0_dn0 = assign83990_e127495_d_n0;
        locals.var_t0_dn2 = assign83990_e127495_d_n2;
        locals.var_t0_dn4 = assign83990_e127495_d_n4;
        locals.var_t0_dn5 = assign83990_e127495_d_n5;
        locals.var_t0_dn6 = assign83990_e127495_d_n6;
        locals.var_t0_dn7 = assign83990_e127495_d_n7;
        locals.var_t0_dn8 = assign83990_e127495_d_n8;
        locals.var_t0_dn9 = assign83990_e127495_d_n9;
        locals.var_t0_dn10 = assign83990_e127495_d_n10;
        locals.var_t0_dn13 = assign83990_e127495_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign84000_e127509, assign84000_e127509_d_n0, assign84000_e127509_d_n2, assign84000_e127509_d_n4, assign84000_e127509_d_n5, assign84000_e127509_d_n6, assign84000_e127509_d_n7, assign84000_e127509_d_n8, assign84000_e127509_d_n9, assign84000_e127509_d_n10, assign84000_e127509_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign84000_e127509;
        locals.var_ps0ld_dn0 = assign84000_e127509_d_n0;
        locals.var_ps0ld_dn2 = assign84000_e127509_d_n2;
        locals.var_ps0ld_dn4 = assign84000_e127509_d_n4;
        locals.var_ps0ld_dn5 = assign84000_e127509_d_n5;
        locals.var_ps0ld_dn6 = assign84000_e127509_d_n6;
        locals.var_ps0ld_dn7 = assign84000_e127509_d_n7;
        locals.var_ps0ld_dn8 = assign84000_e127509_d_n8;
        locals.var_ps0ld_dn9 = assign84000_e127509_d_n9;
        locals.var_ps0ld_dn10 = assign84000_e127509_d_n10;
        locals.var_ps0ld_dn13 = assign84000_e127509_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign84010_e127523, assign84010_e127523_d_n0, assign84010_e127523_d_n2, assign84010_e127523_d_n4, assign84010_e127523_d_n5, assign84010_e127523_d_n6, assign84010_e127523_d_n7, assign84010_e127523_d_n8, assign84010_e127523_d_n9, assign84010_e127523_d_n10, assign84010_e127523_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84010_e127523;
        locals.var_t0_dn0 = assign84010_e127523_d_n0;
        locals.var_t0_dn2 = assign84010_e127523_d_n2;
        locals.var_t0_dn4 = assign84010_e127523_d_n4;
        locals.var_t0_dn5 = assign84010_e127523_d_n5;
        locals.var_t0_dn6 = assign84010_e127523_d_n6;
        locals.var_t0_dn7 = assign84010_e127523_d_n7;
        locals.var_t0_dn8 = assign84010_e127523_d_n8;
        locals.var_t0_dn9 = assign84010_e127523_d_n9;
        locals.var_t0_dn10 = assign84010_e127523_d_n10;
        locals.var_t0_dn13 = assign84010_e127523_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign84020_e127540, assign84020_e127540_d_n0, assign84020_e127540_d_n2, assign84020_e127540_d_n4, assign84020_e127540_d_n5, assign84020_e127540_d_n6, assign84020_e127540_d_n7, assign84020_e127540_d_n8, assign84020_e127540_d_n9, assign84020_e127540_d_n10, assign84020_e127540_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 == 0.0)) {
        let (assign84020_e127538, assign84020_e127538_d_n0, assign84020_e127538_d_n2, assign84020_e127538_d_n4, assign84020_e127538_d_n5, assign84020_e127538_d_n6, assign84020_e127538_d_n7, assign84020_e127538_d_n8, assign84020_e127538_d_n9, assign84020_e127538_d_n10, assign84020_e127538_d_n13,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1__blk1928) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
            } else {
                (locals.var_ps0ld_bef1__blk1928, locals.var_ps0ld_bef1__blk1928_dn0, locals.var_ps0ld_bef1__blk1928_dn2, locals.var_ps0ld_bef1__blk1928_dn4, locals.var_ps0ld_bef1__blk1928_dn5, locals.var_ps0ld_bef1__blk1928_dn6, locals.var_ps0ld_bef1__blk1928_dn7, locals.var_ps0ld_bef1__blk1928_dn8, locals.var_ps0ld_bef1__blk1928_dn9, locals.var_ps0ld_bef1__blk1928_dn10, locals.var_ps0ld_bef1__blk1928_dn13,)
            }
        };
        (assign84020_e127538, assign84020_e127538_d_n0, assign84020_e127538_d_n2, assign84020_e127538_d_n4, assign84020_e127538_d_n5, assign84020_e127538_d_n6, assign84020_e127538_d_n7, assign84020_e127538_d_n8, assign84020_e127538_d_n9, assign84020_e127538_d_n10, assign84020_e127538_d_n13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign84020_e127540;
        locals.var_ps0ld_dn0 = assign84020_e127540_d_n0;
        locals.var_ps0ld_dn2 = assign84020_e127540_d_n2;
        locals.var_ps0ld_dn4 = assign84020_e127540_d_n4;
        locals.var_ps0ld_dn5 = assign84020_e127540_d_n5;
        locals.var_ps0ld_dn6 = assign84020_e127540_d_n6;
        locals.var_ps0ld_dn7 = assign84020_e127540_d_n7;
        locals.var_ps0ld_dn8 = assign84020_e127540_d_n8;
        locals.var_ps0ld_dn9 = assign84020_e127540_d_n9;
        locals.var_ps0ld_dn10 = assign84020_e127540_d_n10;
        locals.var_ps0ld_dn13 = assign84020_e127540_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign84030_e127547, assign84030_e127547_d_n0, assign84030_e127547_d_n2, assign84030_e127547_d_n4, assign84030_e127547_d_n5, assign84030_e127547_d_n6, assign84030_e127547_d_n7, assign84030_e127547_d_n8, assign84030_e127547_d_n9, assign84030_e127547_d_n10, assign84030_e127547_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini__blk1890, locals.var_ps0ld_ini__blk1890_dn0, locals.var_ps0ld_ini__blk1890_dn2, locals.var_ps0ld_ini__blk1890_dn4, locals.var_ps0ld_ini__blk1890_dn5, locals.var_ps0ld_ini__blk1890_dn6, locals.var_ps0ld_ini__blk1890_dn7, locals.var_ps0ld_ini__blk1890_dn8, locals.var_ps0ld_ini__blk1890_dn9, locals.var_ps0ld_ini__blk1890_dn10, locals.var_ps0ld_ini__blk1890_dn13,)
    }
};
        locals.var_ps0ld_ini__blk1890 = assign84030_e127547;
        locals.var_ps0ld_ini__blk1890_dn0 = assign84030_e127547_d_n0;
        locals.var_ps0ld_ini__blk1890_dn2 = assign84030_e127547_d_n2;
        locals.var_ps0ld_ini__blk1890_dn4 = assign84030_e127547_d_n4;
        locals.var_ps0ld_ini__blk1890_dn5 = assign84030_e127547_d_n5;
        locals.var_ps0ld_ini__blk1890_dn6 = assign84030_e127547_d_n6;
        locals.var_ps0ld_ini__blk1890_dn7 = assign84030_e127547_d_n7;
        locals.var_ps0ld_ini__blk1890_dn8 = assign84030_e127547_d_n8;
        locals.var_ps0ld_ini__blk1890_dn9 = assign84030_e127547_d_n9;
        locals.var_ps0ld_ini__blk1890_dn10 = assign84030_e127547_d_n10;
        locals.var_ps0ld_ini__blk1890_dn13 = assign84030_e127547_d_n13;
        locals.var_ps0ld_ini__blk1890_rv = 0.0;

        let assign84040_e127550: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1946 = assign84040_e127550;
        locals.var_guard1946_rv = 0.0;

        let (assign84050_e127559,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign84050_e127559;
        locals.var_flg_conv_rv = 0.0;

        let (assign84060_e127575, assign84060_e127575_d_n0, assign84060_e127575_d_n2, assign84060_e127575_d_n4, assign84060_e127575_d_n5, assign84060_e127575_d_n6, assign84060_e127575_d_n7, assign84060_e127575_d_n8, assign84060_e127575_d_n9, assign84060_e127575_d_n10, assign84060_e127575_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84060_e127569: f64 = (1.034943e-10 / locals.var_q_nsubld__blk1883);
        let assign84060_e127571: f64 = (assign84060_e127569 * locals.var_beta_inv);
        let assign84060_e127572: f64 = (2.0 * assign84060_e127571);
        let assign84060_e127573: f64 = (assign84060_e127572).sqrt();
        (assign84060_e127573, ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn0)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn2)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn4)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn5)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn6)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn7)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn8)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn9)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn10)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn13)) / (2.0 * assign84060_e127573)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign84060_e127575;
        locals.var_c_w_ld_dn0 = assign84060_e127575_d_n0;
        locals.var_c_w_ld_dn2 = assign84060_e127575_d_n2;
        locals.var_c_w_ld_dn4 = assign84060_e127575_d_n4;
        locals.var_c_w_ld_dn5 = assign84060_e127575_d_n5;
        locals.var_c_w_ld_dn6 = assign84060_e127575_d_n6;
        locals.var_c_w_ld_dn7 = assign84060_e127575_d_n7;
        locals.var_c_w_ld_dn8 = assign84060_e127575_d_n8;
        locals.var_c_w_ld_dn9 = assign84060_e127575_d_n9;
        locals.var_c_w_ld_dn10 = assign84060_e127575_d_n10;
        locals.var_c_w_ld_dn13 = assign84060_e127575_d_n13;
        locals.var_c_w_ld_rv = 0.0;

        let assign84070_e127578: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1947 = assign84070_e127578;
        locals.var_guard1947_rv = 0.0;

        let (assign84080_e127591, assign84080_e127591_d_n0, assign84080_e127591_d_n2, assign84080_e127591_d_n4, assign84080_e127591_d_n5, assign84080_e127591_d_n6, assign84080_e127591_d_n7, assign84080_e127591_d_n8, assign84080_e127591_d_n9, assign84080_e127591_d_n10, assign84080_e127591_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 != 0.0)) {
        let assign84080_e127589: f64 = (p.p334 - locals.var_wdep_func);
        (assign84080_e127589, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84080_e127591;
        locals.var_t2_dn0 = assign84080_e127591_d_n0;
        locals.var_t2_dn2 = assign84080_e127591_d_n2;
        locals.var_t2_dn4 = assign84080_e127591_d_n4;
        locals.var_t2_dn5 = assign84080_e127591_d_n5;
        locals.var_t2_dn6 = assign84080_e127591_d_n6;
        locals.var_t2_dn7 = assign84080_e127591_d_n7;
        locals.var_t2_dn8 = assign84080_e127591_d_n8;
        locals.var_t2_dn9 = assign84080_e127591_d_n9;
        locals.var_t2_dn10 = assign84080_e127591_d_n10;
        locals.var_t2_dn13 = assign84080_e127591_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84090_e127616, assign84090_e127616_d_n0, assign84090_e127616_d_n2, assign84090_e127616_d_n4, assign84090_e127616_d_n5, assign84090_e127616_d_n6, assign84090_e127616_d_n7, assign84090_e127616_d_n8, assign84090_e127616_d_n9, assign84090_e127616_d_n10, assign84090_e127616_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) {
        let assign84090_e127603: f64 = (locals.var_vdsi + p.p137);
        let assign84090_e127606: f64 = (locals.var_vdsi + p.p137);
        let assign84090_e127607: f64 = (assign84090_e127603 * assign84090_e127606);
        let assign84090_e127610: f64 = (4.0 * 0.1);
        let assign84090_e127612: f64 = (assign84090_e127610 * 0.1);
        let assign84090_e127613: f64 = (assign84090_e127607 + assign84090_e127612);
        let assign84090_e127614: f64 = (assign84090_e127613).sqrt();
        (assign84090_e127614, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign84090_e127606) + (assign84090_e127603 * locals.var_vdsi_dn5)) / (2.0 * assign84090_e127614)), 0.0, (((locals.var_vdsi_dn7 * assign84090_e127606) + (assign84090_e127603 * locals.var_vdsi_dn7)) / (2.0 * assign84090_e127614)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84090_e127616;
        locals.var_tmf2_dn0 = assign84090_e127616_d_n0;
        locals.var_tmf2_dn2 = assign84090_e127616_d_n2;
        locals.var_tmf2_dn4 = assign84090_e127616_d_n4;
        locals.var_tmf2_dn5 = assign84090_e127616_d_n5;
        locals.var_tmf2_dn6 = assign84090_e127616_d_n6;
        locals.var_tmf2_dn7 = assign84090_e127616_d_n7;
        locals.var_tmf2_dn8 = assign84090_e127616_d_n8;
        locals.var_tmf2_dn9 = assign84090_e127616_d_n9;
        locals.var_tmf2_dn10 = assign84090_e127616_d_n10;
        locals.var_tmf2_dn13 = assign84090_e127616_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign84100_e127636, assign84100_e127636_d_n0, assign84100_e127636_d_n2, assign84100_e127636_d_n4, assign84100_e127636_d_n5, assign84100_e127636_d_n6, assign84100_e127636_d_n7, assign84100_e127636_d_n8, assign84100_e127636_d_n9, assign84100_e127636_d_n10, assign84100_e127636_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) {
        let assign84100_e127630: f64 = (locals.var_vdsi + p.p137);
        let assign84100_e127632: f64 = (assign84100_e127630 / locals.var_tmf2);
        let assign84100_e127633: f64 = (1.0 + assign84100_e127632);
        let assign84100_e127634: f64 = (0.5 * assign84100_e127633);
        (assign84100_e127634, (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign84100_e127630 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign84100_e127630 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84100_e127636;
        locals.var_t9_dn0 = assign84100_e127636_d_n0;
        locals.var_t9_dn2 = assign84100_e127636_d_n2;
        locals.var_t9_dn4 = assign84100_e127636_d_n4;
        locals.var_t9_dn5 = assign84100_e127636_d_n5;
        locals.var_t9_dn6 = assign84100_e127636_d_n6;
        locals.var_t9_dn7 = assign84100_e127636_d_n7;
        locals.var_t9_dn8 = assign84100_e127636_d_n8;
        locals.var_t9_dn9 = assign84100_e127636_d_n9;
        locals.var_t9_dn10 = assign84100_e127636_d_n10;
        locals.var_t9_dn13 = assign84100_e127636_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84110_e127654, assign84110_e127654_d_n0, assign84110_e127654_d_n2, assign84110_e127654_d_n4, assign84110_e127654_d_n5, assign84110_e127654_d_n6, assign84110_e127654_d_n7, assign84110_e127654_d_n8, assign84110_e127654_d_n9, assign84110_e127654_d_n10, assign84110_e127654_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) {
        let assign84110_e127649: f64 = (locals.var_vdsi + p.p137);
        let assign84110_e127651: f64 = (assign84110_e127649 + locals.var_tmf2);
        let assign84110_e127652: f64 = (0.5 * assign84110_e127651);
        (assign84110_e127652, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84110_e127654;
        locals.var_t2_dn0 = assign84110_e127654_d_n0;
        locals.var_t2_dn2 = assign84110_e127654_d_n2;
        locals.var_t2_dn4 = assign84110_e127654_d_n4;
        locals.var_t2_dn5 = assign84110_e127654_d_n5;
        locals.var_t2_dn6 = assign84110_e127654_d_n6;
        locals.var_t2_dn7 = assign84110_e127654_d_n7;
        locals.var_t2_dn8 = assign84110_e127654_d_n8;
        locals.var_t2_dn9 = assign84110_e127654_d_n9;
        locals.var_t2_dn10 = assign84110_e127654_d_n10;
        locals.var_t2_dn13 = assign84110_e127654_d_n13;
        locals.var_t2_rv = 0.0;

        let assign84120_e127657: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1948 = assign84120_e127657;
        locals.var_guard1948_rv = 0.0;

        let (assign84130_e127671, assign84130_e127671_d_n0, assign84130_e127671_d_n2, assign84130_e127671_d_n4, assign84130_e127671_d_n5, assign84130_e127671_d_n6, assign84130_e127671_d_n7, assign84130_e127671_d_n8, assign84130_e127671_d_n9, assign84130_e127671_d_n10, assign84130_e127671_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) && (locals.var_guard1948 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84130_e127671;
        locals.var_t2_dn0 = assign84130_e127671_d_n0;
        locals.var_t2_dn2 = assign84130_e127671_d_n2;
        locals.var_t2_dn4 = assign84130_e127671_d_n4;
        locals.var_t2_dn5 = assign84130_e127671_d_n5;
        locals.var_t2_dn6 = assign84130_e127671_d_n6;
        locals.var_t2_dn7 = assign84130_e127671_d_n7;
        locals.var_t2_dn8 = assign84130_e127671_d_n8;
        locals.var_t2_dn9 = assign84130_e127671_d_n9;
        locals.var_t2_dn10 = assign84130_e127671_d_n10;
        locals.var_t2_dn13 = assign84130_e127671_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84140_e127685, assign84140_e127685_d_n0, assign84140_e127685_d_n2, assign84140_e127685_d_n4, assign84140_e127685_d_n5, assign84140_e127685_d_n6, assign84140_e127685_d_n7, assign84140_e127685_d_n8, assign84140_e127685_d_n9, assign84140_e127685_d_n10, assign84140_e127685_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) && (locals.var_guard1948 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84140_e127685;
        locals.var_t9_dn0 = assign84140_e127685_d_n0;
        locals.var_t9_dn2 = assign84140_e127685_d_n2;
        locals.var_t9_dn4 = assign84140_e127685_d_n4;
        locals.var_t9_dn5 = assign84140_e127685_d_n5;
        locals.var_t9_dn6 = assign84140_e127685_d_n6;
        locals.var_t9_dn7 = assign84140_e127685_d_n7;
        locals.var_t9_dn8 = assign84140_e127685_d_n8;
        locals.var_t9_dn9 = assign84140_e127685_d_n9;
        locals.var_t9_dn10 = assign84140_e127685_d_n10;
        locals.var_t9_dn13 = assign84140_e127685_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84150_e127702, assign84150_e127702_d_n0, assign84150_e127702_d_n2, assign84150_e127702_d_n4, assign84150_e127702_d_n5, assign84150_e127702_d_n6, assign84150_e127702_d_n7, assign84150_e127702_d_n8, assign84150_e127702_d_n9, assign84150_e127702_d_n10, assign84150_e127702_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) {
        let assign84150_e127697: f64 = (locals.var_kjunc * locals.var_t2);
        let assign84150_e127698: f64 = (assign84150_e127697).sqrt();
        let assign84150_e127700: f64 = (assign84150_e127698 * p.p432);
        (assign84150_e127700, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign84150_e127698)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign84150_e127702;
        locals.var_wjunc0_dn0 = assign84150_e127702_d_n0;
        locals.var_wjunc0_dn2 = assign84150_e127702_d_n2;
        locals.var_wjunc0_dn4 = assign84150_e127702_d_n4;
        locals.var_wjunc0_dn5 = assign84150_e127702_d_n5;
        locals.var_wjunc0_dn6 = assign84150_e127702_d_n6;
        locals.var_wjunc0_dn7 = assign84150_e127702_d_n7;
        locals.var_wjunc0_dn8 = assign84150_e127702_d_n8;
        locals.var_wjunc0_dn9 = assign84150_e127702_d_n9;
        locals.var_wjunc0_dn10 = assign84150_e127702_d_n10;
        locals.var_wjunc0_dn13 = assign84150_e127702_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign84160_e127716, assign84160_e127716_d_n0, assign84160_e127716_d_n2, assign84160_e127716_d_n4, assign84160_e127716_d_n5, assign84160_e127716_d_n6, assign84160_e127716_d_n7, assign84160_e127716_d_n8, assign84160_e127716_d_n9, assign84160_e127716_d_n10, assign84160_e127716_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) {
        let assign84160_e127714: f64 = (p.p334 - locals.var_wjunc0);
        (assign84160_e127714, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84160_e127716;
        locals.var_t2_dn0 = assign84160_e127716_d_n0;
        locals.var_t2_dn2 = assign84160_e127716_d_n2;
        locals.var_t2_dn4 = assign84160_e127716_d_n4;
        locals.var_t2_dn5 = assign84160_e127716_d_n5;
        locals.var_t2_dn6 = assign84160_e127716_d_n6;
        locals.var_t2_dn7 = assign84160_e127716_d_n7;
        locals.var_t2_dn8 = assign84160_e127716_d_n8;
        locals.var_t2_dn9 = assign84160_e127716_d_n9;
        locals.var_t2_dn10 = assign84160_e127716_d_n10;
        locals.var_t2_dn13 = assign84160_e127716_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84170_e127738, assign84170_e127738_d_n0, assign84170_e127738_d_n2, assign84170_e127738_d_n4, assign84170_e127738_d_n5, assign84170_e127738_d_n6, assign84170_e127738_d_n7, assign84170_e127738_d_n8, assign84170_e127738_d_n9, assign84170_e127738_d_n10, assign84170_e127738_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84170_e127725: f64 = (locals.var_t2 * locals.var_t2);
        let assign84170_e127729: f64 = (p.p334 * 0.01);
        let assign84170_e127730: f64 = (4.0 * assign84170_e127729);
        let assign84170_e127733: f64 = (p.p334 * 0.01);
        let assign84170_e127734: f64 = (assign84170_e127730 * assign84170_e127733);
        let assign84170_e127735: f64 = (assign84170_e127725 + assign84170_e127734);
        let assign84170_e127736: f64 = (assign84170_e127735).sqrt();
        (assign84170_e127736, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign84170_e127736)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84170_e127738;
        locals.var_tmf2_dn0 = assign84170_e127738_d_n0;
        locals.var_tmf2_dn2 = assign84170_e127738_d_n2;
        locals.var_tmf2_dn4 = assign84170_e127738_d_n4;
        locals.var_tmf2_dn5 = assign84170_e127738_d_n5;
        locals.var_tmf2_dn6 = assign84170_e127738_d_n6;
        locals.var_tmf2_dn7 = assign84170_e127738_d_n7;
        locals.var_tmf2_dn8 = assign84170_e127738_d_n8;
        locals.var_tmf2_dn9 = assign84170_e127738_d_n9;
        locals.var_tmf2_dn10 = assign84170_e127738_d_n10;
        locals.var_tmf2_dn13 = assign84170_e127738_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign84180_e127753, assign84180_e127753_d_n0, assign84180_e127753_d_n2, assign84180_e127753_d_n4, assign84180_e127753_d_n5, assign84180_e127753_d_n6, assign84180_e127753_d_n7, assign84180_e127753_d_n8, assign84180_e127753_d_n9, assign84180_e127753_d_n10, assign84180_e127753_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84180_e127749: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign84180_e127750: f64 = (1.0 + assign84180_e127749);
        let assign84180_e127751: f64 = (0.5 * assign84180_e127750);
        (assign84180_e127751, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84180_e127753;
        locals.var_t9_dn0 = assign84180_e127753_d_n0;
        locals.var_t9_dn2 = assign84180_e127753_d_n2;
        locals.var_t9_dn4 = assign84180_e127753_d_n4;
        locals.var_t9_dn5 = assign84180_e127753_d_n5;
        locals.var_t9_dn6 = assign84180_e127753_d_n6;
        locals.var_t9_dn7 = assign84180_e127753_d_n7;
        locals.var_t9_dn8 = assign84180_e127753_d_n8;
        locals.var_t9_dn9 = assign84180_e127753_d_n9;
        locals.var_t9_dn10 = assign84180_e127753_d_n10;
        locals.var_t9_dn13 = assign84180_e127753_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84190_e127766, assign84190_e127766_d_n0, assign84190_e127766_d_n2, assign84190_e127766_d_n4, assign84190_e127766_d_n5, assign84190_e127766_d_n6, assign84190_e127766_d_n7, assign84190_e127766_d_n8, assign84190_e127766_d_n9, assign84190_e127766_d_n10, assign84190_e127766_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84190_e127763: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign84190_e127764: f64 = (0.5 * assign84190_e127763);
        (assign84190_e127764, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84190_e127766;
        locals.var_t2_dn0 = assign84190_e127766_d_n0;
        locals.var_t2_dn2 = assign84190_e127766_d_n2;
        locals.var_t2_dn4 = assign84190_e127766_d_n4;
        locals.var_t2_dn5 = assign84190_e127766_d_n5;
        locals.var_t2_dn6 = assign84190_e127766_d_n6;
        locals.var_t2_dn7 = assign84190_e127766_d_n7;
        locals.var_t2_dn8 = assign84190_e127766_d_n8;
        locals.var_t2_dn9 = assign84190_e127766_d_n9;
        locals.var_t2_dn10 = assign84190_e127766_d_n10;
        locals.var_t2_dn13 = assign84190_e127766_d_n13;
        locals.var_t2_rv = 0.0;

        let assign84200_e127769: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1949 = assign84200_e127769;
        locals.var_guard1949_rv = 0.0;

        let (assign84210_e127780, assign84210_e127780_d_n0, assign84210_e127780_d_n2, assign84210_e127780_d_n4, assign84210_e127780_d_n5, assign84210_e127780_d_n6, assign84210_e127780_d_n7, assign84210_e127780_d_n8, assign84210_e127780_d_n9, assign84210_e127780_d_n10, assign84210_e127780_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1949 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84210_e127780;
        locals.var_t2_dn0 = assign84210_e127780_d_n0;
        locals.var_t2_dn2 = assign84210_e127780_d_n2;
        locals.var_t2_dn4 = assign84210_e127780_d_n4;
        locals.var_t2_dn5 = assign84210_e127780_d_n5;
        locals.var_t2_dn6 = assign84210_e127780_d_n6;
        locals.var_t2_dn7 = assign84210_e127780_d_n7;
        locals.var_t2_dn8 = assign84210_e127780_d_n8;
        locals.var_t2_dn9 = assign84210_e127780_d_n9;
        locals.var_t2_dn10 = assign84210_e127780_d_n10;
        locals.var_t2_dn13 = assign84210_e127780_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84220_e127791, assign84220_e127791_d_n0, assign84220_e127791_d_n2, assign84220_e127791_d_n4, assign84220_e127791_d_n5, assign84220_e127791_d_n6, assign84220_e127791_d_n7, assign84220_e127791_d_n8, assign84220_e127791_d_n9, assign84220_e127791_d_n10, assign84220_e127791_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1949 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84220_e127791;
        locals.var_t9_dn0 = assign84220_e127791_d_n0;
        locals.var_t9_dn2 = assign84220_e127791_d_n2;
        locals.var_t9_dn4 = assign84220_e127791_d_n4;
        locals.var_t9_dn5 = assign84220_e127791_d_n5;
        locals.var_t9_dn6 = assign84220_e127791_d_n6;
        locals.var_t9_dn7 = assign84220_e127791_d_n7;
        locals.var_t9_dn8 = assign84220_e127791_d_n8;
        locals.var_t9_dn9 = assign84220_e127791_d_n9;
        locals.var_t9_dn10 = assign84220_e127791_d_n10;
        locals.var_t9_dn13 = assign84220_e127791_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84230_e127800, assign84230_e127800_d_n0, assign84230_e127800_d_n2, assign84230_e127800_d_n4, assign84230_e127800_d_n5, assign84230_e127800_d_n6, assign84230_e127800_d_n7, assign84230_e127800_d_n8, assign84230_e127800_d_n9, assign84230_e127800_d_n10, assign84230_e127800_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign84230_e127800;
        locals.var_ddriftldc_dn0 = assign84230_e127800_d_n0;
        locals.var_ddriftldc_dn2 = assign84230_e127800_d_n2;
        locals.var_ddriftldc_dn4 = assign84230_e127800_d_n4;
        locals.var_ddriftldc_dn5 = assign84230_e127800_d_n5;
        locals.var_ddriftldc_dn6 = assign84230_e127800_d_n6;
        locals.var_ddriftldc_dn7 = assign84230_e127800_d_n7;
        locals.var_ddriftldc_dn8 = assign84230_e127800_d_n8;
        locals.var_ddriftldc_dn9 = assign84230_e127800_d_n9;
        locals.var_ddriftldc_dn10 = assign84230_e127800_d_n10;
        locals.var_ddriftldc_dn13 = assign84230_e127800_d_n13;
        locals.var_ddriftldc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_309(
        locals: &mut StampLocals,
    ) {
        let (assign84240_e127817, assign84240_e127817_d_n0, assign84240_e127817_d_n2, assign84240_e127817_d_n4, assign84240_e127817_d_n5, assign84240_e127817_d_n6, assign84240_e127817_d_n7, assign84240_e127817_d_n8, assign84240_e127817_d_n9, assign84240_e127817_d_n10, assign84240_e127817_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84240_e127809: f64 = (locals.var_q_nsubld__blk1883 * locals.var_ddriftldc);
        let assign84240_e127811: f64 = (assign84240_e127809 * locals.var_ddriftldc);
        let assign84240_e127813: f64 = (assign84240_e127811 / 2.0);
        let assign84240_e127815: f64 = (assign84240_e127813 / 1.034943e-10);
        (assign84240_e127815, (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign84240_e127817;
        locals.var_dphi_sb_dn0 = assign84240_e127817_d_n0;
        locals.var_dphi_sb_dn2 = assign84240_e127817_d_n2;
        locals.var_dphi_sb_dn4 = assign84240_e127817_d_n4;
        locals.var_dphi_sb_dn5 = assign84240_e127817_d_n5;
        locals.var_dphi_sb_dn6 = assign84240_e127817_d_n6;
        locals.var_dphi_sb_dn7 = assign84240_e127817_d_n7;
        locals.var_dphi_sb_dn8 = assign84240_e127817_d_n8;
        locals.var_dphi_sb_dn9 = assign84240_e127817_d_n9;
        locals.var_dphi_sb_dn10 = assign84240_e127817_d_n10;
        locals.var_dphi_sb_dn13 = assign84240_e127817_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign84250_e127831, assign84250_e127831_d_n0, assign84250_e127831_d_n2, assign84250_e127831_d_n4, assign84250_e127831_d_n5, assign84250_e127831_d_n6, assign84250_e127831_d_n7, assign84250_e127831_d_n8, assign84250_e127831_d_n9, assign84250_e127831_d_n10, assign84250_e127831_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84250_e127826: f64 = (2.0 * locals.var_beta);
        let assign84250_e127828: f64 = (assign84250_e127826 * locals.var_dphi_sb);
        let assign84250_e127829: f64 = (assign84250_e127828).sqrt();
        (assign84250_e127829, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn0)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn2)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn4)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn5)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn6)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn7)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn8)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn9)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn10)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn13)) / (2.0 * assign84250_e127829)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84250_e127831;
        locals.var_t0_dn0 = assign84250_e127831_d_n0;
        locals.var_t0_dn2 = assign84250_e127831_d_n2;
        locals.var_t0_dn4 = assign84250_e127831_d_n4;
        locals.var_t0_dn5 = assign84250_e127831_d_n5;
        locals.var_t0_dn6 = assign84250_e127831_d_n6;
        locals.var_t0_dn7 = assign84250_e127831_d_n7;
        locals.var_t0_dn8 = assign84250_e127831_d_n8;
        locals.var_t0_dn9 = assign84250_e127831_d_n9;
        locals.var_t0_dn10 = assign84250_e127831_d_n10;
        locals.var_t0_dn13 = assign84250_e127831_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign84260_e127847, assign84260_e127847_d_n0, assign84260_e127847_d_n2, assign84260_e127847_d_n4, assign84260_e127847_d_n5, assign84260_e127847_d_n6, assign84260_e127847_d_n7, assign84260_e127847_d_n8, assign84260_e127847_d_n9, assign84260_e127847_d_n10, assign84260_e127847_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84260_e127839: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign84260_e127841: f64 = (-locals.var_t0);
        let assign84260_e127842: f64 = { let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign84260_e127843: f64 = (assign84260_e127839 + assign84260_e127842);
        let assign84260_e127845: f64 = (assign84260_e127843 / 2.0);
        (assign84260_e127845, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign84260_e127847;
        locals.var_t1_dn0 = assign84260_e127847_d_n0;
        locals.var_t1_dn2 = assign84260_e127847_d_n2;
        locals.var_t1_dn4 = assign84260_e127847_d_n4;
        locals.var_t1_dn5 = assign84260_e127847_d_n5;
        locals.var_t1_dn6 = assign84260_e127847_d_n6;
        locals.var_t1_dn7 = assign84260_e127847_d_n7;
        locals.var_t1_dn8 = assign84260_e127847_d_n8;
        locals.var_t1_dn9 = assign84260_e127847_d_n9;
        locals.var_t1_dn10 = assign84260_e127847_d_n10;
        locals.var_t1_dn13 = assign84260_e127847_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign84270_e127859, assign84270_e127859_d_n0, assign84270_e127859_d_n2, assign84270_e127859_d_n4, assign84270_e127859_d_n5, assign84270_e127859_d_n6, assign84270_e127859_d_n7, assign84270_e127859_d_n8, assign84270_e127859_d_n9, assign84270_e127859_d_n10, assign84270_e127859_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84270_e127855: f64 = (locals.var_t1).ln();
        let assign84270_e127857: f64 = (assign84270_e127855 / locals.var_dphi_sb);
        (assign84270_e127857, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign84270_e127859;
        locals.var_c_sb_dn0 = assign84270_e127859_d_n0;
        locals.var_c_sb_dn2 = assign84270_e127859_d_n2;
        locals.var_c_sb_dn4 = assign84270_e127859_d_n4;
        locals.var_c_sb_dn5 = assign84270_e127859_d_n5;
        locals.var_c_sb_dn6 = assign84270_e127859_d_n6;
        locals.var_c_sb_dn7 = assign84270_e127859_d_n7;
        locals.var_c_sb_dn8 = assign84270_e127859_d_n8;
        locals.var_c_sb_dn9 = assign84270_e127859_d_n9;
        locals.var_c_sb_dn10 = assign84270_e127859_d_n10;
        locals.var_c_sb_dn13 = assign84270_e127859_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign84280_e127868,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign84280_e127868;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_310(
        locals: &mut StampLocals,
    ) {
        let mut assign84290_loop_guard: usize = 0;
        while {
            let assign84290_cond_e127878: f64 = (locals.var_lp_s0_max + 1.0);
            let assign84290_cond_e127880: f64 = if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_lp_s0 <= assign84290_cond_e127878)) { 1.0 } else { 0.0 };
            assign84290_cond_e127880 != 0.0
        } {
            assign84290_loop_guard += 1;
            assert!(assign84290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign84290_body3_e127916, assign84290_body3_e127916_d_n0, assign84290_body3_e127916_d_n2, assign84290_body3_e127916_d_n4, assign84290_body3_e127916_d_n5, assign84290_body3_e127916_d_n6, assign84290_body3_e127916_d_n7, assign84290_body3_e127916_d_n8, assign84290_body3_e127916_d_n9, assign84290_body3_e127916_d_n10, assign84290_body3_e127916_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body3_e127914: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign84290_body3_e127914, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign84290_body3_e127916;
            locals.var_ps0ld_vxb_dn0 = assign84290_body3_e127916_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign84290_body3_e127916_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign84290_body3_e127916_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign84290_body3_e127916_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign84290_body3_e127916_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign84290_body3_e127916_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign84290_body3_e127916_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign84290_body3_e127916_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign84290_body3_e127916_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign84290_body3_e127916_d_n13;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign84290_body4_e127927, assign84290_body4_e127927_d_n0, assign84290_body4_e127927_d_n2, assign84290_body4_e127927_d_n4, assign84290_body4_e127927_d_n5, assign84290_body4_e127927_d_n6, assign84290_body4_e127927_d_n7, assign84290_body4_e127927_d_n8, assign84290_body4_e127927_d_n9, assign84290_body4_e127927_d_n10, assign84290_body4_e127927_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body4_e127925: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign84290_body4_e127925, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign84290_body4_e127927;
            locals.var_chi_dn0 = assign84290_body4_e127927_d_n0;
            locals.var_chi_dn2 = assign84290_body4_e127927_d_n2;
            locals.var_chi_dn4 = assign84290_body4_e127927_d_n4;
            locals.var_chi_dn5 = assign84290_body4_e127927_d_n5;
            locals.var_chi_dn6 = assign84290_body4_e127927_d_n6;
            locals.var_chi_dn7 = assign84290_body4_e127927_d_n7;
            locals.var_chi_dn8 = assign84290_body4_e127927_d_n8;
            locals.var_chi_dn9 = assign84290_body4_e127927_d_n9;
            locals.var_chi_dn10 = assign84290_body4_e127927_d_n10;
            locals.var_chi_dn13 = assign84290_body4_e127927_d_n13;
            locals.var_chi_rv = 0.0;
            let (assign84290_body5_e127940, assign84290_body5_e127940_d_n0, assign84290_body5_e127940_d_n2, assign84290_body5_e127940_d_n4, assign84290_body5_e127940_d_n5, assign84290_body5_e127940_d_n6, assign84290_body5_e127940_d_n7, assign84290_body5_e127940_d_n8, assign84290_body5_e127940_d_n9, assign84290_body5_e127940_d_n10, assign84290_body5_e127940_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body5_e127937: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign84290_body5_e127938: f64 = (locals.var_c_sb * assign84290_body5_e127937);
        (assign84290_body5_e127938, ((locals.var_c_sb_dn0 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign84290_body5_e127940;
            locals.var_ty_dn0 = assign84290_body5_e127940_d_n0;
            locals.var_ty_dn2 = assign84290_body5_e127940_d_n2;
            locals.var_ty_dn4 = assign84290_body5_e127940_d_n4;
            locals.var_ty_dn5 = assign84290_body5_e127940_d_n5;
            locals.var_ty_dn6 = assign84290_body5_e127940_d_n6;
            locals.var_ty_dn7 = assign84290_body5_e127940_d_n7;
            locals.var_ty_dn8 = assign84290_body5_e127940_d_n8;
            locals.var_ty_dn9 = assign84290_body5_e127940_d_n9;
            locals.var_ty_dn10 = assign84290_body5_e127940_d_n10;
            locals.var_ty_dn13 = assign84290_body5_e127940_d_n13;
            locals.var_ty_rv = 0.0;
            let assign84290_body6_e127943: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1951 = assign84290_body6_e127943;
            locals.var_guard1951_rv = 0.0;
            let (assign84290_body7_e127955, assign84290_body7_e127955_d_n0, assign84290_body7_e127955_d_n2, assign84290_body7_e127955_d_n4, assign84290_body7_e127955_d_n5, assign84290_body7_e127955_d_n6, assign84290_body7_e127955_d_n7, assign84290_body7_e127955_d_n8, assign84290_body7_e127955_d_n9, assign84290_body7_e127955_d_n10, assign84290_body7_e127955_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 != 0.0)) {
        let assign84290_body7_e127953: f64 = (locals.var_ty).exp();
        (assign84290_body7_e127953, (assign84290_body7_e127953 * locals.var_ty_dn0), (assign84290_body7_e127953 * locals.var_ty_dn2), (assign84290_body7_e127953 * locals.var_ty_dn4), (assign84290_body7_e127953 * locals.var_ty_dn5), (assign84290_body7_e127953 * locals.var_ty_dn6), (assign84290_body7_e127953 * locals.var_ty_dn7), (assign84290_body7_e127953 * locals.var_ty_dn8), (assign84290_body7_e127953 * locals.var_ty_dn9), (assign84290_body7_e127953 * locals.var_ty_dn10), (assign84290_body7_e127953 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84290_body7_e127955;
            locals.var_t1_dn0 = assign84290_body7_e127955_d_n0;
            locals.var_t1_dn2 = assign84290_body7_e127955_d_n2;
            locals.var_t1_dn4 = assign84290_body7_e127955_d_n4;
            locals.var_t1_dn5 = assign84290_body7_e127955_d_n5;
            locals.var_t1_dn6 = assign84290_body7_e127955_d_n6;
            locals.var_t1_dn7 = assign84290_body7_e127955_d_n7;
            locals.var_t1_dn8 = assign84290_body7_e127955_d_n8;
            locals.var_t1_dn9 = assign84290_body7_e127955_d_n9;
            locals.var_t1_dn10 = assign84290_body7_e127955_d_n10;
            locals.var_t1_dn13 = assign84290_body7_e127955_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign84290_body8_e127970, assign84290_body8_e127970_d_n0, assign84290_body8_e127970_d_n2, assign84290_body8_e127970_d_n4, assign84290_body8_e127970_d_n5, assign84290_body8_e127970_d_n6, assign84290_body8_e127970_d_n7, assign84290_body8_e127970_d_n8, assign84290_body8_e127970_d_n9, assign84290_body8_e127970_d_n10, assign84290_body8_e127970_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 != 0.0)) {
        let assign84290_body8_e127965: f64 = (-locals.var_c_sb);
        let assign84290_body8_e127967: f64 = (assign84290_body8_e127965 * locals.var_dphi_sb);
        let assign84290_body8_e127968: f64 = (assign84290_body8_e127967).exp();
        (assign84290_body8_e127968, (assign84290_body8_e127968 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn0))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn2))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn4))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn5))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn6))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn7))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn8))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn9))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn10))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84290_body8_e127970;
            locals.var_t0_dn0 = assign84290_body8_e127970_d_n0;
            locals.var_t0_dn2 = assign84290_body8_e127970_d_n2;
            locals.var_t0_dn4 = assign84290_body8_e127970_d_n4;
            locals.var_t0_dn5 = assign84290_body8_e127970_d_n5;
            locals.var_t0_dn6 = assign84290_body8_e127970_d_n6;
            locals.var_t0_dn7 = assign84290_body8_e127970_d_n7;
            locals.var_t0_dn8 = assign84290_body8_e127970_d_n8;
            locals.var_t0_dn9 = assign84290_body8_e127970_d_n9;
            locals.var_t0_dn10 = assign84290_body8_e127970_d_n10;
            locals.var_t0_dn13 = assign84290_body8_e127970_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign84290_body9_e127983, assign84290_body9_e127983_d_n0, assign84290_body9_e127983_d_n2, assign84290_body9_e127983_d_n4, assign84290_body9_e127983_d_n5, assign84290_body9_e127983_d_n6, assign84290_body9_e127983_d_n7, assign84290_body9_e127983_d_n8, assign84290_body9_e127983_d_n9, assign84290_body9_e127983_d_n10, assign84290_body9_e127983_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 != 0.0)) {
        let assign84290_body9_e127981: f64 = (locals.var_t1 - locals.var_t0);
        (assign84290_body9_e127981, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign84290_body9_e127983;
            locals.var_t2_dn0 = assign84290_body9_e127983_d_n0;
            locals.var_t2_dn2 = assign84290_body9_e127983_d_n2;
            locals.var_t2_dn4 = assign84290_body9_e127983_d_n4;
            locals.var_t2_dn5 = assign84290_body9_e127983_d_n5;
            locals.var_t2_dn6 = assign84290_body9_e127983_d_n6;
            locals.var_t2_dn7 = assign84290_body9_e127983_d_n7;
            locals.var_t2_dn8 = assign84290_body9_e127983_d_n8;
            locals.var_t2_dn9 = assign84290_body9_e127983_d_n9;
            locals.var_t2_dn10 = assign84290_body9_e127983_d_n10;
            locals.var_t2_dn13 = assign84290_body9_e127983_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign84290_body10_e127999, assign84290_body10_e127999_d_n0, assign84290_body10_e127999_d_n2, assign84290_body10_e127999_d_n4, assign84290_body10_e127999_d_n5, assign84290_body10_e127999_d_n6, assign84290_body10_e127999_d_n7, assign84290_body10_e127999_d_n8, assign84290_body10_e127999_d_n9, assign84290_body10_e127999_d_n10, assign84290_body10_e127999_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 != 0.0)) {
        let assign84290_body10_e127994: f64 = (1.0 + locals.var_t2);
        let assign84290_body10_e127995: f64 = (assign84290_body10_e127994).ln();
        let assign84290_body10_e127997: f64 = (assign84290_body10_e127995 / locals.var_c_sb);
        (assign84290_body10_e127997, ((((locals.var_t2_dn0 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign84290_body10_e127999;
            locals.var_phi_b_dn0 = assign84290_body10_e127999_d_n0;
            locals.var_phi_b_dn2 = assign84290_body10_e127999_d_n2;
            locals.var_phi_b_dn4 = assign84290_body10_e127999_d_n4;
            locals.var_phi_b_dn5 = assign84290_body10_e127999_d_n5;
            locals.var_phi_b_dn6 = assign84290_body10_e127999_d_n6;
            locals.var_phi_b_dn7 = assign84290_body10_e127999_d_n7;
            locals.var_phi_b_dn8 = assign84290_body10_e127999_d_n8;
            locals.var_phi_b_dn9 = assign84290_body10_e127999_d_n9;
            locals.var_phi_b_dn10 = assign84290_body10_e127999_d_n10;
            locals.var_phi_b_dn13 = assign84290_body10_e127999_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign84290_body11_e128014, assign84290_body11_e128014_d_n0, assign84290_body11_e128014_d_n2, assign84290_body11_e128014_d_n4, assign84290_body11_e128014_d_n5, assign84290_body11_e128014_d_n6, assign84290_body11_e128014_d_n7, assign84290_body11_e128014_d_n8, assign84290_body11_e128014_d_n9, assign84290_body11_e128014_d_n10, assign84290_body11_e128014_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 != 0.0)) {
        let assign84290_body11_e128011: f64 = (1.0 + locals.var_t2);
        let assign84290_body11_e128012: f64 = (locals.var_t1 / assign84290_body11_e128011);
        (assign84290_body11_e128012, (((locals.var_t1_dn0 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn0)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn2 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn2)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn4 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn4)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn5 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn5)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn6 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn6)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn7 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn7)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn8 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn8)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn9 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn9)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn10 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn10)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn13 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn13)) / (assign84290_body11_e128011 * assign84290_body11_e128011)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign84290_body11_e128014;
            locals.var_phi_b_dpss_dn0 = assign84290_body11_e128014_d_n0;
            locals.var_phi_b_dpss_dn2 = assign84290_body11_e128014_d_n2;
            locals.var_phi_b_dpss_dn4 = assign84290_body11_e128014_d_n4;
            locals.var_phi_b_dpss_dn5 = assign84290_body11_e128014_d_n5;
            locals.var_phi_b_dpss_dn6 = assign84290_body11_e128014_d_n6;
            locals.var_phi_b_dpss_dn7 = assign84290_body11_e128014_d_n7;
            locals.var_phi_b_dpss_dn8 = assign84290_body11_e128014_d_n8;
            locals.var_phi_b_dpss_dn9 = assign84290_body11_e128014_d_n9;
            locals.var_phi_b_dpss_dn10 = assign84290_body11_e128014_d_n10;
            locals.var_phi_b_dpss_dn13 = assign84290_body11_e128014_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign84290_body13_e128042, assign84290_body13_e128042_d_n0, assign84290_body13_e128042_d_n2, assign84290_body13_e128042_d_n4, assign84290_body13_e128042_d_n5, assign84290_body13_e128042_d_n6, assign84290_body13_e128042_d_n7, assign84290_body13_e128042_d_n8, assign84290_body13_e128042_d_n9, assign84290_body13_e128042_d_n10, assign84290_body13_e128042_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 == 0.0)) {
        let assign84290_body13_e128040: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign84290_body13_e128040, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign84290_body13_e128042;
            locals.var_phi_b_dn0 = assign84290_body13_e128042_d_n0;
            locals.var_phi_b_dn2 = assign84290_body13_e128042_d_n2;
            locals.var_phi_b_dn4 = assign84290_body13_e128042_d_n4;
            locals.var_phi_b_dn5 = assign84290_body13_e128042_d_n5;
            locals.var_phi_b_dn6 = assign84290_body13_e128042_d_n6;
            locals.var_phi_b_dn7 = assign84290_body13_e128042_d_n7;
            locals.var_phi_b_dn8 = assign84290_body13_e128042_d_n8;
            locals.var_phi_b_dn9 = assign84290_body13_e128042_d_n9;
            locals.var_phi_b_dn10 = assign84290_body13_e128042_d_n10;
            locals.var_phi_b_dn13 = assign84290_body13_e128042_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign84290_body14_e128054, assign84290_body14_e128054_d_n0, assign84290_body14_e128054_d_n2, assign84290_body14_e128054_d_n4, assign84290_body14_e128054_d_n5, assign84290_body14_e128054_d_n6, assign84290_body14_e128054_d_n7, assign84290_body14_e128054_d_n8, assign84290_body14_e128054_d_n9, assign84290_body14_e128054_d_n10, assign84290_body14_e128054_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign84290_body14_e128054;
            locals.var_phi_b_dpss_dn0 = assign84290_body14_e128054_d_n0;
            locals.var_phi_b_dpss_dn2 = assign84290_body14_e128054_d_n2;
            locals.var_phi_b_dpss_dn4 = assign84290_body14_e128054_d_n4;
            locals.var_phi_b_dpss_dn5 = assign84290_body14_e128054_d_n5;
            locals.var_phi_b_dpss_dn6 = assign84290_body14_e128054_d_n6;
            locals.var_phi_b_dpss_dn7 = assign84290_body14_e128054_d_n7;
            locals.var_phi_b_dpss_dn8 = assign84290_body14_e128054_d_n8;
            locals.var_phi_b_dpss_dn9 = assign84290_body14_e128054_d_n9;
            locals.var_phi_b_dpss_dn10 = assign84290_body14_e128054_d_n10;
            locals.var_phi_b_dpss_dn13 = assign84290_body14_e128054_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign84290_body15_e128065, assign84290_body15_e128065_d_n0, assign84290_body15_e128065_d_n2, assign84290_body15_e128065_d_n4, assign84290_body15_e128065_d_n5, assign84290_body15_e128065_d_n6, assign84290_body15_e128065_d_n7, assign84290_body15_e128065_d_n8, assign84290_body15_e128065_d_n9, assign84290_body15_e128065_d_n10, assign84290_body15_e128065_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body15_e128063: f64 = (locals.var_beta * locals.var_phi_b);
        (assign84290_body15_e128063, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign84290_body15_e128065;
            locals.var_chib_dn0 = assign84290_body15_e128065_d_n0;
            locals.var_chib_dn2 = assign84290_body15_e128065_d_n2;
            locals.var_chib_dn4 = assign84290_body15_e128065_d_n4;
            locals.var_chib_dn5 = assign84290_body15_e128065_d_n5;
            locals.var_chib_dn6 = assign84290_body15_e128065_d_n6;
            locals.var_chib_dn7 = assign84290_body15_e128065_d_n7;
            locals.var_chib_dn8 = assign84290_body15_e128065_d_n8;
            locals.var_chib_dn9 = assign84290_body15_e128065_d_n9;
            locals.var_chib_dn10 = assign84290_body15_e128065_d_n10;
            locals.var_chib_dn13 = assign84290_body15_e128065_d_n13;
            locals.var_chib_rv = 0.0;
            let assign84290_body16_e128068: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1952 = assign84290_body16_e128068;
            locals.var_guard1952_rv = 0.0;
            let (assign84290_body18_e128093, assign84290_body18_e128093_d_n0, assign84290_body18_e128093_d_n2, assign84290_body18_e128093_d_n4, assign84290_body18_e128093_d_n5, assign84290_body18_e128093_d_n6, assign84290_body18_e128093_d_n7, assign84290_body18_e128093_d_n8, assign84290_body18_e128093_d_n9, assign84290_body18_e128093_d_n10, assign84290_body18_e128093_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 != 0.0)) {
        let assign84290_body18_e128091: f64 = (-0.7071067811865475);
        (assign84290_body18_e128091, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84290_body18_e128093;
            locals.var_t0_dn0 = assign84290_body18_e128093_d_n0;
            locals.var_t0_dn2 = assign84290_body18_e128093_d_n2;
            locals.var_t0_dn4 = assign84290_body18_e128093_d_n4;
            locals.var_t0_dn5 = assign84290_body18_e128093_d_n5;
            locals.var_t0_dn6 = assign84290_body18_e128093_d_n6;
            locals.var_t0_dn7 = assign84290_body18_e128093_d_n7;
            locals.var_t0_dn8 = assign84290_body18_e128093_d_n8;
            locals.var_t0_dn9 = assign84290_body18_e128093_d_n9;
            locals.var_t0_dn10 = assign84290_body18_e128093_d_n10;
            locals.var_t0_dn13 = assign84290_body18_e128093_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign84290_body19_e128106, assign84290_body19_e128106_d_n0, assign84290_body19_e128106_d_n2, assign84290_body19_e128106_d_n4, assign84290_body19_e128106_d_n5, assign84290_body19_e128106_d_n6, assign84290_body19_e128106_d_n7, assign84290_body19_e128106_d_n8, assign84290_body19_e128106_d_n9, assign84290_body19_e128106_d_n10, assign84290_body19_e128106_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 != 0.0)) {
        let assign84290_body19_e128104: f64 = (locals.var_chi * locals.var_t0);
        (assign84290_body19_e128104, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn13 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign84290_body19_e128106;
            locals.var_fb_dn0 = assign84290_body19_e128106_d_n0;
            locals.var_fb_dn2 = assign84290_body19_e128106_d_n2;
            locals.var_fb_dn4 = assign84290_body19_e128106_d_n4;
            locals.var_fb_dn5 = assign84290_body19_e128106_d_n5;
            locals.var_fb_dn6 = assign84290_body19_e128106_d_n6;
            locals.var_fb_dn7 = assign84290_body19_e128106_d_n7;
            locals.var_fb_dn8 = assign84290_body19_e128106_d_n8;
            locals.var_fb_dn9 = assign84290_body19_e128106_d_n9;
            locals.var_fb_dn10 = assign84290_body19_e128106_d_n10;
            locals.var_fb_dn13 = assign84290_body19_e128106_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign84290_body20_e128119, assign84290_body20_e128119_d_n0, assign84290_body20_e128119_d_n2, assign84290_body20_e128119_d_n4, assign84290_body20_e128119_d_n5, assign84290_body20_e128119_d_n6, assign84290_body20_e128119_d_n7, assign84290_body20_e128119_d_n8, assign84290_body20_e128119_d_n9, assign84290_body20_e128119_d_n10, assign84290_body20_e128119_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 != 0.0)) {
        let assign84290_body20_e128117: f64 = (locals.var_beta * locals.var_t0);
        (assign84290_body20_e128117, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn13 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign84290_body20_e128119;
            locals.var_fb_dpss_dn0 = assign84290_body20_e128119_d_n0;
            locals.var_fb_dpss_dn2 = assign84290_body20_e128119_d_n2;
            locals.var_fb_dpss_dn4 = assign84290_body20_e128119_d_n4;
            locals.var_fb_dpss_dn5 = assign84290_body20_e128119_d_n5;
            locals.var_fb_dpss_dn6 = assign84290_body20_e128119_d_n6;
            locals.var_fb_dpss_dn7 = assign84290_body20_e128119_d_n7;
            locals.var_fb_dpss_dn8 = assign84290_body20_e128119_d_n8;
            locals.var_fb_dpss_dn9 = assign84290_body20_e128119_d_n9;
            locals.var_fb_dpss_dn10 = assign84290_body20_e128119_d_n10;
            locals.var_fb_dpss_dn13 = assign84290_body20_e128119_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let assign84290_body21_e128122: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1953 = assign84290_body21_e128122;
            locals.var_guard1953_rv = 0.0;
            let (assign84290_body23_e128174, assign84290_body23_e128174_d_n0, assign84290_body23_e128174_d_n2, assign84290_body23_e128174_d_n4, assign84290_body23_e128174_d_n5, assign84290_body23_e128174_d_n6, assign84290_body23_e128174_d_n7, assign84290_body23_e128174_d_n8, assign84290_body23_e128174_d_n9, assign84290_body23_e128174_d_n10, assign84290_body23_e128174_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) {
        let assign84290_body23_e128152: f64 = (locals.var_chi * locals.var_chi);
        let assign84290_body23_e128154: f64 = (assign84290_body23_e128152 / 2.0);
        let assign84290_body23_e128158: f64 = (locals.var_chi / 3.0);
        let assign84290_body23_e128162: f64 = (locals.var_chi / 4.0);
        let assign84290_body23_e128166: f64 = (locals.var_chi / 5.0);
        let assign84290_body23_e128167: f64 = (1.0 - assign84290_body23_e128166);
        let assign84290_body23_e128168: f64 = (assign84290_body23_e128162 * assign84290_body23_e128167);
        let assign84290_body23_e128169: f64 = (1.0 - assign84290_body23_e128168);
        let assign84290_body23_e128170: f64 = (assign84290_body23_e128158 * assign84290_body23_e128169);
        let assign84290_body23_e128171: f64 = (1.0 - assign84290_body23_e128170);
        let assign84290_body23_e128172: f64 = (assign84290_body23_e128154 * assign84290_body23_e128171);
        (assign84290_body23_e128172, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn0 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn0 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn2 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn2 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn4 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn4 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn5 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn5 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn6 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn6 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn7 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn7 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn8 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn8 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn9 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn9 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn10 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn10 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn13 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn13 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84290_body23_e128174;
            locals.var_t0_dn0 = assign84290_body23_e128174_d_n0;
            locals.var_t0_dn2 = assign84290_body23_e128174_d_n2;
            locals.var_t0_dn4 = assign84290_body23_e128174_d_n4;
            locals.var_t0_dn5 = assign84290_body23_e128174_d_n5;
            locals.var_t0_dn6 = assign84290_body23_e128174_d_n6;
            locals.var_t0_dn7 = assign84290_body23_e128174_d_n7;
            locals.var_t0_dn8 = assign84290_body23_e128174_d_n8;
            locals.var_t0_dn9 = assign84290_body23_e128174_d_n9;
            locals.var_t0_dn10 = assign84290_body23_e128174_d_n10;
            locals.var_t0_dn13 = assign84290_body23_e128174_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign84290_body24_e128206, assign84290_body24_e128206_d_n0, assign84290_body24_e128206_d_n2, assign84290_body24_e128206_d_n4, assign84290_body24_e128206_d_n5, assign84290_body24_e128206_d_n6, assign84290_body24_e128206_d_n7, assign84290_body24_e128206_d_n8, assign84290_body24_e128206_d_n9, assign84290_body24_e128206_d_n10, assign84290_body24_e128206_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) {
        let assign84290_body24_e128190: f64 = (locals.var_chi / 2.0);
        let assign84290_body24_e128194: f64 = (locals.var_chi / 3.0);
        let assign84290_body24_e128198: f64 = (locals.var_chi / 4.0);
        let assign84290_body24_e128199: f64 = (1.0 - assign84290_body24_e128198);
        let assign84290_body24_e128200: f64 = (assign84290_body24_e128194 * assign84290_body24_e128199);
        let assign84290_body24_e128201: f64 = (1.0 - assign84290_body24_e128200);
        let assign84290_body24_e128202: f64 = (assign84290_body24_e128190 * assign84290_body24_e128201);
        let assign84290_body24_e128203: f64 = (1.0 - assign84290_body24_e128202);
        let assign84290_body24_e128204: f64 = (locals.var_chi * assign84290_body24_e128203);
        (assign84290_body24_e128204, ((locals.var_chi_dn0 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn0 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn2 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn4 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn5 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn6 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn7 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn8 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn9 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn10 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn13 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84290_body24_e128206;
            locals.var_t1_dn0 = assign84290_body24_e128206_d_n0;
            locals.var_t1_dn2 = assign84290_body24_e128206_d_n2;
            locals.var_t1_dn4 = assign84290_body24_e128206_d_n4;
            locals.var_t1_dn5 = assign84290_body24_e128206_d_n5;
            locals.var_t1_dn6 = assign84290_body24_e128206_d_n6;
            locals.var_t1_dn7 = assign84290_body24_e128206_d_n7;
            locals.var_t1_dn8 = assign84290_body24_e128206_d_n8;
            locals.var_t1_dn9 = assign84290_body24_e128206_d_n9;
            locals.var_t1_dn10 = assign84290_body24_e128206_d_n10;
            locals.var_t1_dn13 = assign84290_body24_e128206_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign84290_body25_e128242, assign84290_body25_e128242_d_n0, assign84290_body25_e128242_d_n2, assign84290_body25_e128242_d_n4, assign84290_body25_e128242_d_n5, assign84290_body25_e128242_d_n6, assign84290_body25_e128242_d_n7, assign84290_body25_e128242_d_n8, assign84290_body25_e128242_d_n9, assign84290_body25_e128242_d_n10, assign84290_body25_e128242_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) {
        let assign84290_body25_e128220: f64 = (locals.var_chib * locals.var_chib);
        let assign84290_body25_e128222: f64 = (assign84290_body25_e128220 / 2.0);
        let assign84290_body25_e128226: f64 = (locals.var_chib / 3.0);
        let assign84290_body25_e128230: f64 = (locals.var_chib / 4.0);
        let assign84290_body25_e128234: f64 = (locals.var_chib / 5.0);
        let assign84290_body25_e128235: f64 = (1.0 - assign84290_body25_e128234);
        let assign84290_body25_e128236: f64 = (assign84290_body25_e128230 * assign84290_body25_e128235);
        let assign84290_body25_e128237: f64 = (1.0 - assign84290_body25_e128236);
        let assign84290_body25_e128238: f64 = (assign84290_body25_e128226 * assign84290_body25_e128237);
        let assign84290_body25_e128239: f64 = (1.0 - assign84290_body25_e128238);
        let assign84290_body25_e128240: f64 = (assign84290_body25_e128222 * assign84290_body25_e128239);
        (assign84290_body25_e128240, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn0 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn0 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn2 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn2 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn4 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn4 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn5 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn5 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn6 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn6 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn7 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn7 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn8 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn8 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn9 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn9 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn10 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn10 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn13 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn13 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign84290_body25_e128242;
            locals.var_t2_dn0 = assign84290_body25_e128242_d_n0;
            locals.var_t2_dn2 = assign84290_body25_e128242_d_n2;
            locals.var_t2_dn4 = assign84290_body25_e128242_d_n4;
            locals.var_t2_dn5 = assign84290_body25_e128242_d_n5;
            locals.var_t2_dn6 = assign84290_body25_e128242_d_n6;
            locals.var_t2_dn7 = assign84290_body25_e128242_d_n7;
            locals.var_t2_dn8 = assign84290_body25_e128242_d_n8;
            locals.var_t2_dn9 = assign84290_body25_e128242_d_n9;
            locals.var_t2_dn10 = assign84290_body25_e128242_d_n10;
            locals.var_t2_dn13 = assign84290_body25_e128242_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign84290_body26_e128274, assign84290_body26_e128274_d_n0, assign84290_body26_e128274_d_n2, assign84290_body26_e128274_d_n4, assign84290_body26_e128274_d_n5, assign84290_body26_e128274_d_n6, assign84290_body26_e128274_d_n7, assign84290_body26_e128274_d_n8, assign84290_body26_e128274_d_n9, assign84290_body26_e128274_d_n10, assign84290_body26_e128274_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) {
        let assign84290_body26_e128258: f64 = (locals.var_chib / 2.0);
        let assign84290_body26_e128262: f64 = (locals.var_chib / 3.0);
        let assign84290_body26_e128266: f64 = (locals.var_chib / 4.0);
        let assign84290_body26_e128267: f64 = (1.0 - assign84290_body26_e128266);
        let assign84290_body26_e128268: f64 = (assign84290_body26_e128262 * assign84290_body26_e128267);
        let assign84290_body26_e128269: f64 = (1.0 - assign84290_body26_e128268);
        let assign84290_body26_e128270: f64 = (assign84290_body26_e128258 * assign84290_body26_e128269);
        let assign84290_body26_e128271: f64 = (1.0 - assign84290_body26_e128270);
        let assign84290_body26_e128272: f64 = (locals.var_chib * assign84290_body26_e128271);
        (assign84290_body26_e128272, ((locals.var_chib_dn0 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn0 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn2 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn4 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn5 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn6 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn7 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn8 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn9 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn10 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn13 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign84290_body26_e128274;
            locals.var_t3_dn0 = assign84290_body26_e128274_d_n0;
            locals.var_t3_dn2 = assign84290_body26_e128274_d_n2;
            locals.var_t3_dn4 = assign84290_body26_e128274_d_n4;
            locals.var_t3_dn5 = assign84290_body26_e128274_d_n5;
            locals.var_t3_dn6 = assign84290_body26_e128274_d_n6;
            locals.var_t3_dn7 = assign84290_body26_e128274_d_n7;
            locals.var_t3_dn8 = assign84290_body26_e128274_d_n8;
            locals.var_t3_dn9 = assign84290_body26_e128274_d_n9;
            locals.var_t3_dn10 = assign84290_body26_e128274_d_n10;
            locals.var_t3_dn13 = assign84290_body26_e128274_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign84290_body27_e128290, assign84290_body27_e128290_d_n0, assign84290_body27_e128290_d_n2, assign84290_body27_e128290_d_n4, assign84290_body27_e128290_d_n5, assign84290_body27_e128290_d_n6, assign84290_body27_e128290_d_n7, assign84290_body27_e128290_d_n8, assign84290_body27_e128290_d_n9, assign84290_body27_e128290_d_n10, assign84290_body27_e128290_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) {
        let assign84290_body27_e128288: f64 = (locals.var_t0 - locals.var_t2);
        (assign84290_body27_e128288, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign84290_body27_e128290;
            locals.var_t4_dn0 = assign84290_body27_e128290_d_n0;
            locals.var_t4_dn2 = assign84290_body27_e128290_d_n2;
            locals.var_t4_dn4 = assign84290_body27_e128290_d_n4;
            locals.var_t4_dn5 = assign84290_body27_e128290_d_n5;
            locals.var_t4_dn6 = assign84290_body27_e128290_d_n6;
            locals.var_t4_dn7 = assign84290_body27_e128290_d_n7;
            locals.var_t4_dn8 = assign84290_body27_e128290_d_n8;
            locals.var_t4_dn9 = assign84290_body27_e128290_d_n9;
            locals.var_t4_dn10 = assign84290_body27_e128290_d_n10;
            locals.var_t4_dn13 = assign84290_body27_e128290_d_n13;
            locals.var_t4_rv = 0.0;
            let assign84290_body28_e128293: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1954 = assign84290_body28_e128293;
            locals.var_guard1954_rv = 0.0;
            let (assign84290_body29_e128310, assign84290_body29_e128310_d_n0, assign84290_body29_e128310_d_n2, assign84290_body29_e128310_d_n4, assign84290_body29_e128310_d_n5, assign84290_body29_e128310_d_n6, assign84290_body29_e128310_d_n7, assign84290_body29_e128310_d_n8, assign84290_body29_e128310_d_n9, assign84290_body29_e128310_d_n10, assign84290_body29_e128310_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) && (locals.var_guard1954 != 0.0)) {
        let assign84290_body29_e128308: f64 = (locals.var_t4).sqrt();
        (assign84290_body29_e128308, (locals.var_t4_dn0 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn2 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn4 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn5 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn6 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn7 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn8 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn9 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn10 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn13 / (2.0 * assign84290_body29_e128308)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign84290_body29_e128310;
            locals.var_fb_dn0 = assign84290_body29_e128310_d_n0;
            locals.var_fb_dn2 = assign84290_body29_e128310_d_n2;
            locals.var_fb_dn4 = assign84290_body29_e128310_d_n4;
            locals.var_fb_dn5 = assign84290_body29_e128310_d_n5;
            locals.var_fb_dn6 = assign84290_body29_e128310_d_n6;
            locals.var_fb_dn7 = assign84290_body29_e128310_d_n7;
            locals.var_fb_dn8 = assign84290_body29_e128310_d_n8;
            locals.var_fb_dn9 = assign84290_body29_e128310_d_n9;
            locals.var_fb_dn10 = assign84290_body29_e128310_d_n10;
            locals.var_fb_dn13 = assign84290_body29_e128310_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign84290_body30_e128336, assign84290_body30_e128336_d_n0, assign84290_body30_e128336_d_n2, assign84290_body30_e128336_d_n4, assign84290_body30_e128336_d_n5, assign84290_body30_e128336_d_n6, assign84290_body30_e128336_d_n7, assign84290_body30_e128336_d_n8, assign84290_body30_e128336_d_n9, assign84290_body30_e128336_d_n10, assign84290_body30_e128336_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) && (locals.var_guard1954 != 0.0)) {
        let assign84290_body30_e128326: f64 = (locals.var_beta * 0.5);
        let assign84290_body30_e128330: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign84290_body30_e128331: f64 = (locals.var_t1 - assign84290_body30_e128330);
        let assign84290_body30_e128332: f64 = (assign84290_body30_e128326 * assign84290_body30_e128331);
        let assign84290_body30_e128334: f64 = (assign84290_body30_e128332 / locals.var_fb);
        (assign84290_body30_e128334, ((((((locals.var_beta_dn0 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign84290_body30_e128336;
            locals.var_fb_dpss_dn0 = assign84290_body30_e128336_d_n0;
            locals.var_fb_dpss_dn2 = assign84290_body30_e128336_d_n2;
            locals.var_fb_dpss_dn4 = assign84290_body30_e128336_d_n4;
            locals.var_fb_dpss_dn5 = assign84290_body30_e128336_d_n5;
            locals.var_fb_dpss_dn6 = assign84290_body30_e128336_d_n6;
            locals.var_fb_dpss_dn7 = assign84290_body30_e128336_d_n7;
            locals.var_fb_dpss_dn8 = assign84290_body30_e128336_d_n8;
            locals.var_fb_dpss_dn9 = assign84290_body30_e128336_d_n9;
            locals.var_fb_dpss_dn10 = assign84290_body30_e128336_d_n10;
            locals.var_fb_dpss_dn13 = assign84290_body30_e128336_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign84290_body32_e128372, assign84290_body32_e128372_d_n0, assign84290_body32_e128372_d_n2, assign84290_body32_e128372_d_n4, assign84290_body32_e128372_d_n5, assign84290_body32_e128372_d_n6, assign84290_body32_e128372_d_n7, assign84290_body32_e128372_d_n8, assign84290_body32_e128372_d_n9, assign84290_body32_e128372_d_n10, assign84290_body32_e128372_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) && (locals.var_guard1954 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign84290_body32_e128372;
            locals.var_fb_dn0 = assign84290_body32_e128372_d_n0;
            locals.var_fb_dn2 = assign84290_body32_e128372_d_n2;
            locals.var_fb_dn4 = assign84290_body32_e128372_d_n4;
            locals.var_fb_dn5 = assign84290_body32_e128372_d_n5;
            locals.var_fb_dn6 = assign84290_body32_e128372_d_n6;
            locals.var_fb_dn7 = assign84290_body32_e128372_d_n7;
            locals.var_fb_dn8 = assign84290_body32_e128372_d_n8;
            locals.var_fb_dn9 = assign84290_body32_e128372_d_n9;
            locals.var_fb_dn10 = assign84290_body32_e128372_d_n10;
            locals.var_fb_dn13 = assign84290_body32_e128372_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign84290_body33_e128389, assign84290_body33_e128389_d_n0, assign84290_body33_e128389_d_n2, assign84290_body33_e128389_d_n4, assign84290_body33_e128389_d_n5, assign84290_body33_e128389_d_n6, assign84290_body33_e128389_d_n7, assign84290_body33_e128389_d_n8, assign84290_body33_e128389_d_n9, assign84290_body33_e128389_d_n10, assign84290_body33_e128389_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) && (locals.var_guard1954 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign84290_body33_e128389;
            locals.var_fb_dpss_dn0 = assign84290_body33_e128389_d_n0;
            locals.var_fb_dpss_dn2 = assign84290_body33_e128389_d_n2;
            locals.var_fb_dpss_dn4 = assign84290_body33_e128389_d_n4;
            locals.var_fb_dpss_dn5 = assign84290_body33_e128389_d_n5;
            locals.var_fb_dpss_dn6 = assign84290_body33_e128389_d_n6;
            locals.var_fb_dpss_dn7 = assign84290_body33_e128389_d_n7;
            locals.var_fb_dpss_dn8 = assign84290_body33_e128389_d_n8;
            locals.var_fb_dpss_dn9 = assign84290_body33_e128389_d_n9;
            locals.var_fb_dpss_dn10 = assign84290_body33_e128389_d_n10;
            locals.var_fb_dpss_dn13 = assign84290_body33_e128389_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign84290_body34_e128406, assign84290_body34_e128406_d_n0, assign84290_body34_e128406_d_n2, assign84290_body34_e128406_d_n4, assign84290_body34_e128406_d_n5, assign84290_body34_e128406_d_n6, assign84290_body34_e128406_d_n7, assign84290_body34_e128406_d_n8, assign84290_body34_e128406_d_n9, assign84290_body34_e128406_d_n10, assign84290_body34_e128406_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) {
        let assign84290_body34_e128403: f64 = (-locals.var_chi);
        let assign84290_body34_e128404: f64 = (assign84290_body34_e128403).exp();
        (assign84290_body34_e128404, (assign84290_body34_e128404 * (-locals.var_chi_dn0)), (assign84290_body34_e128404 * (-locals.var_chi_dn2)), (assign84290_body34_e128404 * (-locals.var_chi_dn4)), (assign84290_body34_e128404 * (-locals.var_chi_dn5)), (assign84290_body34_e128404 * (-locals.var_chi_dn6)), (assign84290_body34_e128404 * (-locals.var_chi_dn7)), (assign84290_body34_e128404 * (-locals.var_chi_dn8)), (assign84290_body34_e128404 * (-locals.var_chi_dn9)), (assign84290_body34_e128404 * (-locals.var_chi_dn10)), (assign84290_body34_e128404 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84290_body34_e128406;
            locals.var_t0_dn0 = assign84290_body34_e128406_d_n0;
            locals.var_t0_dn2 = assign84290_body34_e128406_d_n2;
            locals.var_t0_dn4 = assign84290_body34_e128406_d_n4;
            locals.var_t0_dn5 = assign84290_body34_e128406_d_n5;
            locals.var_t0_dn6 = assign84290_body34_e128406_d_n6;
            locals.var_t0_dn7 = assign84290_body34_e128406_d_n7;
            locals.var_t0_dn8 = assign84290_body34_e128406_d_n8;
            locals.var_t0_dn9 = assign84290_body34_e128406_d_n9;
            locals.var_t0_dn10 = assign84290_body34_e128406_d_n10;
            locals.var_t0_dn13 = assign84290_body34_e128406_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign84290_body35_e128423, assign84290_body35_e128423_d_n0, assign84290_body35_e128423_d_n2, assign84290_body35_e128423_d_n4, assign84290_body35_e128423_d_n5, assign84290_body35_e128423_d_n6, assign84290_body35_e128423_d_n7, assign84290_body35_e128423_d_n8, assign84290_body35_e128423_d_n9, assign84290_body35_e128423_d_n10, assign84290_body35_e128423_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) {
        let assign84290_body35_e128420: f64 = (-locals.var_chib);
        let assign84290_body35_e128421: f64 = (assign84290_body35_e128420).exp();
        (assign84290_body35_e128421, (assign84290_body35_e128421 * (-locals.var_chib_dn0)), (assign84290_body35_e128421 * (-locals.var_chib_dn2)), (assign84290_body35_e128421 * (-locals.var_chib_dn4)), (assign84290_body35_e128421 * (-locals.var_chib_dn5)), (assign84290_body35_e128421 * (-locals.var_chib_dn6)), (assign84290_body35_e128421 * (-locals.var_chib_dn7)), (assign84290_body35_e128421 * (-locals.var_chib_dn8)), (assign84290_body35_e128421 * (-locals.var_chib_dn9)), (assign84290_body35_e128421 * (-locals.var_chib_dn10)), (assign84290_body35_e128421 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84290_body35_e128423;
            locals.var_t1_dn0 = assign84290_body35_e128423_d_n0;
            locals.var_t1_dn2 = assign84290_body35_e128423_d_n2;
            locals.var_t1_dn4 = assign84290_body35_e128423_d_n4;
            locals.var_t1_dn5 = assign84290_body35_e128423_d_n5;
            locals.var_t1_dn6 = assign84290_body35_e128423_d_n6;
            locals.var_t1_dn7 = assign84290_body35_e128423_d_n7;
            locals.var_t1_dn8 = assign84290_body35_e128423_d_n8;
            locals.var_t1_dn9 = assign84290_body35_e128423_d_n9;
            locals.var_t1_dn10 = assign84290_body35_e128423_d_n10;
            locals.var_t1_dn13 = assign84290_body35_e128423_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign84290_body36_e128444, assign84290_body36_e128444_d_n0, assign84290_body36_e128444_d_n2, assign84290_body36_e128444_d_n4, assign84290_body36_e128444_d_n5, assign84290_body36_e128444_d_n6, assign84290_body36_e128444_d_n7, assign84290_body36_e128444_d_n8, assign84290_body36_e128444_d_n9, assign84290_body36_e128444_d_n10, assign84290_body36_e128444_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) {
        let assign84290_body36_e128438: f64 = (locals.var_chi - locals.var_chib);
        let assign84290_body36_e128441: f64 = (locals.var_t0 - locals.var_t1);
        let assign84290_body36_e128442: f64 = (assign84290_body36_e128438 + assign84290_body36_e128441);
        (assign84290_body36_e128442, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign84290_body36_e128444;
            locals.var_t4_dn0 = assign84290_body36_e128444_d_n0;
            locals.var_t4_dn2 = assign84290_body36_e128444_d_n2;
            locals.var_t4_dn4 = assign84290_body36_e128444_d_n4;
            locals.var_t4_dn5 = assign84290_body36_e128444_d_n5;
            locals.var_t4_dn6 = assign84290_body36_e128444_d_n6;
            locals.var_t4_dn7 = assign84290_body36_e128444_d_n7;
            locals.var_t4_dn8 = assign84290_body36_e128444_d_n8;
            locals.var_t4_dn9 = assign84290_body36_e128444_d_n9;
            locals.var_t4_dn10 = assign84290_body36_e128444_d_n10;
            locals.var_t4_dn13 = assign84290_body36_e128444_d_n13;
            locals.var_t4_rv = 0.0;
            let assign84290_body37_e128447: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1955 = assign84290_body37_e128447;
            locals.var_guard1955_rv = 0.0;
            let (assign84290_body38_e128465, assign84290_body38_e128465_d_n0, assign84290_body38_e128465_d_n2, assign84290_body38_e128465_d_n4, assign84290_body38_e128465_d_n5, assign84290_body38_e128465_d_n6, assign84290_body38_e128465_d_n7, assign84290_body38_e128465_d_n8, assign84290_body38_e128465_d_n9, assign84290_body38_e128465_d_n10, assign84290_body38_e128465_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) && (locals.var_guard1955 != 0.0)) {
        let assign84290_body38_e128463: f64 = (locals.var_t4).sqrt();
        (assign84290_body38_e128463, (locals.var_t4_dn0 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn2 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn4 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn5 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn6 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn7 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn8 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn9 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn10 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn13 / (2.0 * assign84290_body38_e128463)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign84290_body38_e128465;
            locals.var_fb_dn0 = assign84290_body38_e128465_d_n0;
            locals.var_fb_dn2 = assign84290_body38_e128465_d_n2;
            locals.var_fb_dn4 = assign84290_body38_e128465_d_n4;
            locals.var_fb_dn5 = assign84290_body38_e128465_d_n5;
            locals.var_fb_dn6 = assign84290_body38_e128465_d_n6;
            locals.var_fb_dn7 = assign84290_body38_e128465_d_n7;
            locals.var_fb_dn8 = assign84290_body38_e128465_d_n8;
            locals.var_fb_dn9 = assign84290_body38_e128465_d_n9;
            locals.var_fb_dn10 = assign84290_body38_e128465_d_n10;
            locals.var_fb_dn13 = assign84290_body38_e128465_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign84290_body39_e128496, assign84290_body39_e128496_d_n0, assign84290_body39_e128496_d_n2, assign84290_body39_e128496_d_n4, assign84290_body39_e128496_d_n5, assign84290_body39_e128496_d_n6, assign84290_body39_e128496_d_n7, assign84290_body39_e128496_d_n8, assign84290_body39_e128496_d_n9, assign84290_body39_e128496_d_n10, assign84290_body39_e128496_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) && (locals.var_guard1955 != 0.0)) {
        let assign84290_body39_e128482: f64 = (locals.var_beta * 0.5);
        let assign84290_body39_e128485: f64 = (1.0 - locals.var_t0);
        let assign84290_body39_e128489: f64 = (1.0 - locals.var_t1);
        let assign84290_body39_e128490: f64 = (locals.var_phi_b_dpss * assign84290_body39_e128489);
        let assign84290_body39_e128491: f64 = (assign84290_body39_e128485 - assign84290_body39_e128490);
        let assign84290_body39_e128492: f64 = (assign84290_body39_e128482 * assign84290_body39_e128491);
        let assign84290_body39_e128494: f64 = (assign84290_body39_e128492 / locals.var_fb);
        (assign84290_body39_e128494, ((((((locals.var_beta_dn0 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign84290_body39_e128496;
            locals.var_fb_dpss_dn0 = assign84290_body39_e128496_d_n0;
            locals.var_fb_dpss_dn2 = assign84290_body39_e128496_d_n2;
            locals.var_fb_dpss_dn4 = assign84290_body39_e128496_d_n4;
            locals.var_fb_dpss_dn5 = assign84290_body39_e128496_d_n5;
            locals.var_fb_dpss_dn6 = assign84290_body39_e128496_d_n6;
            locals.var_fb_dpss_dn7 = assign84290_body39_e128496_d_n7;
            locals.var_fb_dpss_dn8 = assign84290_body39_e128496_d_n8;
            locals.var_fb_dpss_dn9 = assign84290_body39_e128496_d_n9;
            locals.var_fb_dpss_dn10 = assign84290_body39_e128496_d_n10;
            locals.var_fb_dpss_dn13 = assign84290_body39_e128496_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign84290_body41_e128534, assign84290_body41_e128534_d_n0, assign84290_body41_e128534_d_n2, assign84290_body41_e128534_d_n4, assign84290_body41_e128534_d_n5, assign84290_body41_e128534_d_n6, assign84290_body41_e128534_d_n7, assign84290_body41_e128534_d_n8, assign84290_body41_e128534_d_n9, assign84290_body41_e128534_d_n10, assign84290_body41_e128534_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) && (locals.var_guard1955 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign84290_body41_e128534;
            locals.var_fb_dn0 = assign84290_body41_e128534_d_n0;
            locals.var_fb_dn2 = assign84290_body41_e128534_d_n2;
            locals.var_fb_dn4 = assign84290_body41_e128534_d_n4;
            locals.var_fb_dn5 = assign84290_body41_e128534_d_n5;
            locals.var_fb_dn6 = assign84290_body41_e128534_d_n6;
            locals.var_fb_dn7 = assign84290_body41_e128534_d_n7;
            locals.var_fb_dn8 = assign84290_body41_e128534_d_n8;
            locals.var_fb_dn9 = assign84290_body41_e128534_d_n9;
            locals.var_fb_dn10 = assign84290_body41_e128534_d_n10;
            locals.var_fb_dn13 = assign84290_body41_e128534_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign84290_body42_e128552, assign84290_body42_e128552_d_n0, assign84290_body42_e128552_d_n2, assign84290_body42_e128552_d_n4, assign84290_body42_e128552_d_n5, assign84290_body42_e128552_d_n6, assign84290_body42_e128552_d_n7, assign84290_body42_e128552_d_n8, assign84290_body42_e128552_d_n9, assign84290_body42_e128552_d_n10, assign84290_body42_e128552_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) && (locals.var_guard1955 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign84290_body42_e128552;
            locals.var_fb_dpss_dn0 = assign84290_body42_e128552_d_n0;
            locals.var_fb_dpss_dn2 = assign84290_body42_e128552_d_n2;
            locals.var_fb_dpss_dn4 = assign84290_body42_e128552_d_n4;
            locals.var_fb_dpss_dn5 = assign84290_body42_e128552_d_n5;
            locals.var_fb_dpss_dn6 = assign84290_body42_e128552_d_n6;
            locals.var_fb_dpss_dn7 = assign84290_body42_e128552_d_n7;
            locals.var_fb_dpss_dn8 = assign84290_body42_e128552_d_n8;
            locals.var_fb_dpss_dn9 = assign84290_body42_e128552_d_n9;
            locals.var_fb_dpss_dn10 = assign84290_body42_e128552_d_n10;
            locals.var_fb_dpss_dn13 = assign84290_body42_e128552_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let assign84290_body43_e128555: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1956 = assign84290_body43_e128555;
            locals.var_guard1956_rv = 0.0;
            let (assign84290_body45_e128579, assign84290_body45_e128579_d_n0, assign84290_body45_e128579_d_n2, assign84290_body45_e128579_d_n4, assign84290_body45_e128579_d_n5, assign84290_body45_e128579_d_n6, assign84290_body45_e128579_d_n7, assign84290_body45_e128579_d_n8, assign84290_body45_e128579_d_n9, assign84290_body45_e128579_d_n10, assign84290_body45_e128579_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84290_body45_e128579;
            locals.var_fs01_dn0 = assign84290_body45_e128579_d_n0;
            locals.var_fs01_dn2 = assign84290_body45_e128579_d_n2;
            locals.var_fs01_dn4 = assign84290_body45_e128579_d_n4;
            locals.var_fs01_dn5 = assign84290_body45_e128579_d_n5;
            locals.var_fs01_dn6 = assign84290_body45_e128579_d_n6;
            locals.var_fs01_dn7 = assign84290_body45_e128579_d_n7;
            locals.var_fs01_dn8 = assign84290_body45_e128579_d_n8;
            locals.var_fs01_dn9 = assign84290_body45_e128579_d_n9;
            locals.var_fs01_dn10 = assign84290_body45_e128579_d_n10;
            locals.var_fs01_dn13 = assign84290_body45_e128579_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign84290_body46_e128590, assign84290_body46_e128590_d_n0, assign84290_body46_e128590_d_n2, assign84290_body46_e128590_d_n4, assign84290_body46_e128590_d_n5, assign84290_body46_e128590_d_n6, assign84290_body46_e128590_d_n7, assign84290_body46_e128590_d_n8, assign84290_body46_e128590_d_n9, assign84290_body46_e128590_d_n10, assign84290_body46_e128590_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84290_body46_e128590;
            locals.var_fs01_dps0_dn0 = assign84290_body46_e128590_d_n0;
            locals.var_fs01_dps0_dn2 = assign84290_body46_e128590_d_n2;
            locals.var_fs01_dps0_dn4 = assign84290_body46_e128590_d_n4;
            locals.var_fs01_dps0_dn5 = assign84290_body46_e128590_d_n5;
            locals.var_fs01_dps0_dn6 = assign84290_body46_e128590_d_n6;
            locals.var_fs01_dps0_dn7 = assign84290_body46_e128590_d_n7;
            locals.var_fs01_dps0_dn8 = assign84290_body46_e128590_d_n8;
            locals.var_fs01_dps0_dn9 = assign84290_body46_e128590_d_n9;
            locals.var_fs01_dps0_dn10 = assign84290_body46_e128590_d_n10;
            locals.var_fs01_dps0_dn13 = assign84290_body46_e128590_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign84290_body47_e128602, assign84290_body47_e128602_d_n0, assign84290_body47_e128602_d_n2, assign84290_body47_e128602_d_n4, assign84290_body47_e128602_d_n5, assign84290_body47_e128602_d_n6, assign84290_body47_e128602_d_n7, assign84290_body47_e128602_d_n8, assign84290_body47_e128602_d_n9, assign84290_body47_e128602_d_n10, assign84290_body47_e128602_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 != 0.0)) {
        let assign84290_body47_e128600: f64 = (-locals.var_fb);
        (assign84290_body47_e128600, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84290_body47_e128602;
            locals.var_fs02_dn0 = assign84290_body47_e128602_d_n0;
            locals.var_fs02_dn2 = assign84290_body47_e128602_d_n2;
            locals.var_fs02_dn4 = assign84290_body47_e128602_d_n4;
            locals.var_fs02_dn5 = assign84290_body47_e128602_d_n5;
            locals.var_fs02_dn6 = assign84290_body47_e128602_d_n6;
            locals.var_fs02_dn7 = assign84290_body47_e128602_d_n7;
            locals.var_fs02_dn8 = assign84290_body47_e128602_d_n8;
            locals.var_fs02_dn9 = assign84290_body47_e128602_d_n9;
            locals.var_fs02_dn10 = assign84290_body47_e128602_d_n10;
            locals.var_fs02_dn13 = assign84290_body47_e128602_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign84290_body48_e128614, assign84290_body48_e128614_d_n0, assign84290_body48_e128614_d_n2, assign84290_body48_e128614_d_n4, assign84290_body48_e128614_d_n5, assign84290_body48_e128614_d_n6, assign84290_body48_e128614_d_n7, assign84290_body48_e128614_d_n8, assign84290_body48_e128614_d_n9, assign84290_body48_e128614_d_n10, assign84290_body48_e128614_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 != 0.0)) {
        let assign84290_body48_e128612: f64 = (-locals.var_fb_dpss);
        (assign84290_body48_e128612, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84290_body48_e128614;
            locals.var_fs02_dps0_dn0 = assign84290_body48_e128614_d_n0;
            locals.var_fs02_dps0_dn2 = assign84290_body48_e128614_d_n2;
            locals.var_fs02_dps0_dn4 = assign84290_body48_e128614_d_n4;
            locals.var_fs02_dps0_dn5 = assign84290_body48_e128614_d_n5;
            locals.var_fs02_dps0_dn6 = assign84290_body48_e128614_d_n6;
            locals.var_fs02_dps0_dn7 = assign84290_body48_e128614_d_n7;
            locals.var_fs02_dps0_dn8 = assign84290_body48_e128614_d_n8;
            locals.var_fs02_dps0_dn9 = assign84290_body48_e128614_d_n9;
            locals.var_fs02_dps0_dn10 = assign84290_body48_e128614_d_n10;
            locals.var_fs02_dps0_dn13 = assign84290_body48_e128614_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign84290_body49_e128617: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1957 = assign84290_body49_e128617;
            locals.var_guard1957_rv = 0.0;
            let assign84290_body50_e128620: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1958 = assign84290_body50_e128620;
            locals.var_guard1958_rv = 0.0;
            let (assign84290_body51_e128658, assign84290_body51_e128658_d_n0, assign84290_body51_e128658_d_n2, assign84290_body51_e128658_d_n4, assign84290_body51_e128658_d_n5, assign84290_body51_e128658_d_n6, assign84290_body51_e128658_d_n7, assign84290_body51_e128658_d_n8, assign84290_body51_e128658_d_n9, assign84290_body51_e128658_d_n10, assign84290_body51_e128658_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 != 0.0)) {
        let assign84290_body51_e128636: f64 = (locals.var_chi * locals.var_chi);
        let assign84290_body51_e128638: f64 = (assign84290_body51_e128636 / 2.0);
        let assign84290_body51_e128642: f64 = (locals.var_chi / 3.0);
        let assign84290_body51_e128646: f64 = (locals.var_chi / 4.0);
        let assign84290_body51_e128650: f64 = (locals.var_chi / 5.0);
        let assign84290_body51_e128651: f64 = (1.0 + assign84290_body51_e128650);
        let assign84290_body51_e128652: f64 = (assign84290_body51_e128646 * assign84290_body51_e128651);
        let assign84290_body51_e128653: f64 = (1.0 + assign84290_body51_e128652);
        let assign84290_body51_e128654: f64 = (assign84290_body51_e128642 * assign84290_body51_e128653);
        let assign84290_body51_e128655: f64 = (1.0 + assign84290_body51_e128654);
        let assign84290_body51_e128656: f64 = (assign84290_body51_e128638 * assign84290_body51_e128655);
        (assign84290_body51_e128656, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn0 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn0 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn2 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn2 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn4 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn4 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn5 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn5 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn6 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn6 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn7 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn7 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn8 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn8 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn9 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn9 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn10 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn10 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn13 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn13 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84290_body51_e128658;
            locals.var_t0_dn0 = assign84290_body51_e128658_d_n0;
            locals.var_t0_dn2 = assign84290_body51_e128658_d_n2;
            locals.var_t0_dn4 = assign84290_body51_e128658_d_n4;
            locals.var_t0_dn5 = assign84290_body51_e128658_d_n5;
            locals.var_t0_dn6 = assign84290_body51_e128658_d_n6;
            locals.var_t0_dn7 = assign84290_body51_e128658_d_n7;
            locals.var_t0_dn8 = assign84290_body51_e128658_d_n8;
            locals.var_t0_dn9 = assign84290_body51_e128658_d_n9;
            locals.var_t0_dn10 = assign84290_body51_e128658_d_n10;
            locals.var_t0_dn13 = assign84290_body51_e128658_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign84290_body52_e128692, assign84290_body52_e128692_d_n0, assign84290_body52_e128692_d_n2, assign84290_body52_e128692_d_n4, assign84290_body52_e128692_d_n5, assign84290_body52_e128692_d_n6, assign84290_body52_e128692_d_n7, assign84290_body52_e128692_d_n8, assign84290_body52_e128692_d_n9, assign84290_body52_e128692_d_n10, assign84290_body52_e128692_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 != 0.0)) {
        let assign84290_body52_e128676: f64 = (locals.var_chi / 2.0);
        let assign84290_body52_e128680: f64 = (locals.var_chi / 3.0);
        let assign84290_body52_e128684: f64 = (locals.var_chi / 4.0);
        let assign84290_body52_e128685: f64 = (1.0 + assign84290_body52_e128684);
        let assign84290_body52_e128686: f64 = (assign84290_body52_e128680 * assign84290_body52_e128685);
        let assign84290_body52_e128687: f64 = (1.0 + assign84290_body52_e128686);
        let assign84290_body52_e128688: f64 = (assign84290_body52_e128676 * assign84290_body52_e128687);
        let assign84290_body52_e128689: f64 = (1.0 + assign84290_body52_e128688);
        let assign84290_body52_e128690: f64 = (locals.var_chi * assign84290_body52_e128689);
        (assign84290_body52_e128690, ((locals.var_chi_dn0 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn0 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn2 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn4 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn5 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn6 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn7 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn8 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn9 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn10 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn13 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84290_body52_e128692;
            locals.var_t1_dn0 = assign84290_body52_e128692_d_n0;
            locals.var_t1_dn2 = assign84290_body52_e128692_d_n2;
            locals.var_t1_dn4 = assign84290_body52_e128692_d_n4;
            locals.var_t1_dn5 = assign84290_body52_e128692_d_n5;
            locals.var_t1_dn6 = assign84290_body52_e128692_d_n6;
            locals.var_t1_dn7 = assign84290_body52_e128692_d_n7;
            locals.var_t1_dn8 = assign84290_body52_e128692_d_n8;
            locals.var_t1_dn9 = assign84290_body52_e128692_d_n9;
            locals.var_t1_dn10 = assign84290_body52_e128692_d_n10;
            locals.var_t1_dn13 = assign84290_body52_e128692_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign84290_body53_e128710, assign84290_body53_e128710_d_n0, assign84290_body53_e128710_d_n2, assign84290_body53_e128710_d_n4, assign84290_body53_e128710_d_n5, assign84290_body53_e128710_d_n6, assign84290_body53_e128710_d_n7, assign84290_body53_e128710_d_n8, assign84290_body53_e128710_d_n9, assign84290_body53_e128710_d_n10, assign84290_body53_e128710_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 != 0.0)) {
        let assign84290_body53_e128708: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign84290_body53_e128708, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84290_body53_e128710;
            locals.var_fs01_dn0 = assign84290_body53_e128710_d_n0;
            locals.var_fs01_dn2 = assign84290_body53_e128710_d_n2;
            locals.var_fs01_dn4 = assign84290_body53_e128710_d_n4;
            locals.var_fs01_dn5 = assign84290_body53_e128710_d_n5;
            locals.var_fs01_dn6 = assign84290_body53_e128710_d_n6;
            locals.var_fs01_dn7 = assign84290_body53_e128710_d_n7;
            locals.var_fs01_dn8 = assign84290_body53_e128710_d_n8;
            locals.var_fs01_dn9 = assign84290_body53_e128710_d_n9;
            locals.var_fs01_dn10 = assign84290_body53_e128710_d_n10;
            locals.var_fs01_dn13 = assign84290_body53_e128710_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign84290_body54_e128730, assign84290_body54_e128730_d_n0, assign84290_body54_e128730_d_n2, assign84290_body54_e128730_d_n4, assign84290_body54_e128730_d_n5, assign84290_body54_e128730_d_n6, assign84290_body54_e128730_d_n7, assign84290_body54_e128730_d_n8, assign84290_body54_e128730_d_n9, assign84290_body54_e128730_d_n10, assign84290_body54_e128730_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 != 0.0)) {
        let assign84290_body54_e128726: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign84290_body54_e128728: f64 = (assign84290_body54_e128726 * locals.var_beta);
        (assign84290_body54_e128728, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84290_body54_e128730;
            locals.var_fs01_dps0_dn0 = assign84290_body54_e128730_d_n0;
            locals.var_fs01_dps0_dn2 = assign84290_body54_e128730_d_n2;
            locals.var_fs01_dps0_dn4 = assign84290_body54_e128730_d_n4;
            locals.var_fs01_dps0_dn5 = assign84290_body54_e128730_d_n5;
            locals.var_fs01_dps0_dn6 = assign84290_body54_e128730_d_n6;
            locals.var_fs01_dps0_dn7 = assign84290_body54_e128730_d_n7;
            locals.var_fs01_dps0_dn8 = assign84290_body54_e128730_d_n8;
            locals.var_fs01_dps0_dn9 = assign84290_body54_e128730_d_n9;
            locals.var_fs01_dps0_dn10 = assign84290_body54_e128730_d_n10;
            locals.var_fs01_dps0_dn13 = assign84290_body54_e128730_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign84290_body55_e128748, assign84290_body55_e128748_d_n0, assign84290_body55_e128748_d_n2, assign84290_body55_e128748_d_n4, assign84290_body55_e128748_d_n5, assign84290_body55_e128748_d_n6, assign84290_body55_e128748_d_n7, assign84290_body55_e128748_d_n8, assign84290_body55_e128748_d_n9, assign84290_body55_e128748_d_n10, assign84290_body55_e128748_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 == 0.0)) {
        let assign84290_body55_e128746: f64 = (locals.var_chi).exp();
        (assign84290_body55_e128746, (assign84290_body55_e128746 * locals.var_chi_dn0), (assign84290_body55_e128746 * locals.var_chi_dn2), (assign84290_body55_e128746 * locals.var_chi_dn4), (assign84290_body55_e128746 * locals.var_chi_dn5), (assign84290_body55_e128746 * locals.var_chi_dn6), (assign84290_body55_e128746 * locals.var_chi_dn7), (assign84290_body55_e128746 * locals.var_chi_dn8), (assign84290_body55_e128746 * locals.var_chi_dn9), (assign84290_body55_e128746 * locals.var_chi_dn10), (assign84290_body55_e128746 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign84290_body55_e128748;
            locals.var_exp_chi_dn0 = assign84290_body55_e128748_d_n0;
            locals.var_exp_chi_dn2 = assign84290_body55_e128748_d_n2;
            locals.var_exp_chi_dn4 = assign84290_body55_e128748_d_n4;
            locals.var_exp_chi_dn5 = assign84290_body55_e128748_d_n5;
            locals.var_exp_chi_dn6 = assign84290_body55_e128748_d_n6;
            locals.var_exp_chi_dn7 = assign84290_body55_e128748_d_n7;
            locals.var_exp_chi_dn8 = assign84290_body55_e128748_d_n8;
            locals.var_exp_chi_dn9 = assign84290_body55_e128748_d_n9;
            locals.var_exp_chi_dn10 = assign84290_body55_e128748_d_n10;
            locals.var_exp_chi_dn13 = assign84290_body55_e128748_d_n13;
            locals.var_exp_chi_rv = 0.0;
            let (assign84290_body56_e128767, assign84290_body56_e128767_d_n0, assign84290_body56_e128767_d_n2, assign84290_body56_e128767_d_n4, assign84290_body56_e128767_d_n5, assign84290_body56_e128767_d_n6, assign84290_body56_e128767_d_n7, assign84290_body56_e128767_d_n8, assign84290_body56_e128767_d_n9, assign84290_body56_e128767_d_n10, assign84290_body56_e128767_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 == 0.0)) {
        let assign84290_body56_e128765: f64 = (locals.var_exp_chi - 1.0);
        (assign84290_body56_e128765, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84290_body56_e128767;
            locals.var_t1_dn0 = assign84290_body56_e128767_d_n0;
            locals.var_t1_dn2 = assign84290_body56_e128767_d_n2;
            locals.var_t1_dn4 = assign84290_body56_e128767_d_n4;
            locals.var_t1_dn5 = assign84290_body56_e128767_d_n5;
            locals.var_t1_dn6 = assign84290_body56_e128767_d_n6;
            locals.var_t1_dn7 = assign84290_body56_e128767_d_n7;
            locals.var_t1_dn8 = assign84290_body56_e128767_d_n8;
            locals.var_t1_dn9 = assign84290_body56_e128767_d_n9;
            locals.var_t1_dn10 = assign84290_body56_e128767_d_n10;
            locals.var_t1_dn13 = assign84290_body56_e128767_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign84290_body57_e128788, assign84290_body57_e128788_d_n0, assign84290_body57_e128788_d_n2, assign84290_body57_e128788_d_n4, assign84290_body57_e128788_d_n5, assign84290_body57_e128788_d_n6, assign84290_body57_e128788_d_n7, assign84290_body57_e128788_d_n8, assign84290_body57_e128788_d_n9, assign84290_body57_e128788_d_n10, assign84290_body57_e128788_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 == 0.0)) {
        let assign84290_body57_e128785: f64 = (locals.var_t1 - locals.var_chi);
        let assign84290_body57_e128786: f64 = (locals.var_cfs1 * assign84290_body57_e128785);
        (assign84290_body57_e128786, ((locals.var_cfs1_dn0 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84290_body57_e128788;
            locals.var_fs01_dn0 = assign84290_body57_e128788_d_n0;
            locals.var_fs01_dn2 = assign84290_body57_e128788_d_n2;
            locals.var_fs01_dn4 = assign84290_body57_e128788_d_n4;
            locals.var_fs01_dn5 = assign84290_body57_e128788_d_n5;
            locals.var_fs01_dn6 = assign84290_body57_e128788_d_n6;
            locals.var_fs01_dn7 = assign84290_body57_e128788_d_n7;
            locals.var_fs01_dn8 = assign84290_body57_e128788_d_n8;
            locals.var_fs01_dn9 = assign84290_body57_e128788_d_n9;
            locals.var_fs01_dn10 = assign84290_body57_e128788_d_n10;
            locals.var_fs01_dn13 = assign84290_body57_e128788_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign84290_body58_e128809, assign84290_body58_e128809_d_n0, assign84290_body58_e128809_d_n2, assign84290_body58_e128809_d_n4, assign84290_body58_e128809_d_n5, assign84290_body58_e128809_d_n6, assign84290_body58_e128809_d_n7, assign84290_body58_e128809_d_n8, assign84290_body58_e128809_d_n9, assign84290_body58_e128809_d_n10, assign84290_body58_e128809_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 == 0.0)) {
        let assign84290_body58_e128805: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign84290_body58_e128807: f64 = (assign84290_body58_e128805 * locals.var_t1);
        (assign84290_body58_e128807, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84290_body58_e128809;
            locals.var_fs01_dps0_dn0 = assign84290_body58_e128809_d_n0;
            locals.var_fs01_dps0_dn2 = assign84290_body58_e128809_d_n2;
            locals.var_fs01_dps0_dn4 = assign84290_body58_e128809_d_n4;
            locals.var_fs01_dps0_dn5 = assign84290_body58_e128809_d_n5;
            locals.var_fs01_dps0_dn6 = assign84290_body58_e128809_d_n6;
            locals.var_fs01_dps0_dn7 = assign84290_body58_e128809_d_n7;
            locals.var_fs01_dps0_dn8 = assign84290_body58_e128809_d_n8;
            locals.var_fs01_dps0_dn9 = assign84290_body58_e128809_d_n9;
            locals.var_fs01_dps0_dn10 = assign84290_body58_e128809_d_n10;
            locals.var_fs01_dps0_dn13 = assign84290_body58_e128809_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign84290_body60_e128844, assign84290_body60_e128844_d_n0, assign84290_body60_e128844_d_n2, assign84290_body60_e128844_d_n4, assign84290_body60_e128844_d_n5, assign84290_body60_e128844_d_n6, assign84290_body60_e128844_d_n7, assign84290_body60_e128844_d_n8, assign84290_body60_e128844_d_n9, assign84290_body60_e128844_d_n10, assign84290_body60_e128844_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 == 0.0)) {
        let assign84290_body60_e128841: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign84290_body60_e128842: f64 = (assign84290_body60_e128841).exp();
        (assign84290_body60_e128842, (assign84290_body60_e128842 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign84290_body60_e128842 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign84290_body60_e128842 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign84290_body60_e128842 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign84290_body60_e128842 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign84290_body60_e128842 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign84290_body60_e128842 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign84290_body60_e128842 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign84290_body60_e128842 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign84290_body60_e128842 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign84290_body60_e128844;
            locals.var_exp_bps0_dn0 = assign84290_body60_e128844_d_n0;
            locals.var_exp_bps0_dn2 = assign84290_body60_e128844_d_n2;
            locals.var_exp_bps0_dn4 = assign84290_body60_e128844_d_n4;
            locals.var_exp_bps0_dn5 = assign84290_body60_e128844_d_n5;
            locals.var_exp_bps0_dn6 = assign84290_body60_e128844_d_n6;
            locals.var_exp_bps0_dn7 = assign84290_body60_e128844_d_n7;
            locals.var_exp_bps0_dn8 = assign84290_body60_e128844_d_n8;
            locals.var_exp_bps0_dn9 = assign84290_body60_e128844_d_n9;
            locals.var_exp_bps0_dn10 = assign84290_body60_e128844_d_n10;
            locals.var_exp_bps0_dn13 = assign84290_body60_e128844_d_n13;
            locals.var_exp_bps0_rv = 0.0;
            let (assign84290_body61_e128867, assign84290_body61_e128867_d_n0, assign84290_body61_e128867_d_n2, assign84290_body61_e128867_d_n4, assign84290_body61_e128867_d_n5, assign84290_body61_e128867_d_n6, assign84290_body61_e128867_d_n7, assign84290_body61_e128867_d_n8, assign84290_body61_e128867_d_n9, assign84290_body61_e128867_d_n10, assign84290_body61_e128867_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 == 0.0)) {
        let assign84290_body61_e128862: f64 = (locals.var_chi + 1.0);
        let assign84290_body61_e128863: f64 = (locals.var_exp_bvbs * assign84290_body61_e128862);
        let assign84290_body61_e128864: f64 = (locals.var_exp_bps0 - assign84290_body61_e128863);
        let assign84290_body61_e128865: f64 = (locals.var_cnst1over * assign84290_body61_e128864);
        (assign84290_body61_e128865, ((locals.var_cnst1over_dn0 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84290_body61_e128867;
            locals.var_fs01_dn0 = assign84290_body61_e128867_d_n0;
            locals.var_fs01_dn2 = assign84290_body61_e128867_d_n2;
            locals.var_fs01_dn4 = assign84290_body61_e128867_d_n4;
            locals.var_fs01_dn5 = assign84290_body61_e128867_d_n5;
            locals.var_fs01_dn6 = assign84290_body61_e128867_d_n6;
            locals.var_fs01_dn7 = assign84290_body61_e128867_d_n7;
            locals.var_fs01_dn8 = assign84290_body61_e128867_d_n8;
            locals.var_fs01_dn9 = assign84290_body61_e128867_d_n9;
            locals.var_fs01_dn10 = assign84290_body61_e128867_d_n10;
            locals.var_fs01_dn13 = assign84290_body61_e128867_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign84290_body62_e128888, assign84290_body62_e128888_d_n0, assign84290_body62_e128888_d_n2, assign84290_body62_e128888_d_n4, assign84290_body62_e128888_d_n5, assign84290_body62_e128888_d_n6, assign84290_body62_e128888_d_n7, assign84290_body62_e128888_d_n8, assign84290_body62_e128888_d_n9, assign84290_body62_e128888_d_n10, assign84290_body62_e128888_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 == 0.0)) {
        let assign84290_body62_e128882: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign84290_body62_e128885: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign84290_body62_e128886: f64 = (assign84290_body62_e128882 * assign84290_body62_e128885);
        (assign84290_body62_e128886, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84290_body62_e128888;
            locals.var_fs01_dps0_dn0 = assign84290_body62_e128888_d_n0;
            locals.var_fs01_dps0_dn2 = assign84290_body62_e128888_d_n2;
            locals.var_fs01_dps0_dn4 = assign84290_body62_e128888_d_n4;
            locals.var_fs01_dps0_dn5 = assign84290_body62_e128888_d_n5;
            locals.var_fs01_dps0_dn6 = assign84290_body62_e128888_d_n6;
            locals.var_fs01_dps0_dn7 = assign84290_body62_e128888_d_n7;
            locals.var_fs01_dps0_dn8 = assign84290_body62_e128888_d_n8;
            locals.var_fs01_dps0_dn9 = assign84290_body62_e128888_d_n9;
            locals.var_fs01_dps0_dn10 = assign84290_body62_e128888_d_n10;
            locals.var_fs01_dps0_dn13 = assign84290_body62_e128888_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign84290_body63_e128891: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1959 = assign84290_body63_e128891;
            locals.var_guard1959_rv = 0.0;
            let (assign84290_body64_e128910, assign84290_body64_e128910_d_n0, assign84290_body64_e128910_d_n2, assign84290_body64_e128910_d_n4, assign84290_body64_e128910_d_n5, assign84290_body64_e128910_d_n6, assign84290_body64_e128910_d_n7, assign84290_body64_e128910_d_n8, assign84290_body64_e128910_d_n9, assign84290_body64_e128910_d_n10, assign84290_body64_e128910_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1959 != 0.0)) {
        let assign84290_body64_e128905: f64 = (locals.var_fb * locals.var_fb);
        let assign84290_body64_e128907: f64 = (assign84290_body64_e128905 + locals.var_fs01);
        let assign84290_body64_e128908: f64 = (assign84290_body64_e128907).sqrt();
        (assign84290_body64_e128908, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fs01_dn13) / (2.0 * assign84290_body64_e128908)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84290_body64_e128910;
            locals.var_fs02_dn0 = assign84290_body64_e128910_d_n0;
            locals.var_fs02_dn2 = assign84290_body64_e128910_d_n2;
            locals.var_fs02_dn4 = assign84290_body64_e128910_d_n4;
            locals.var_fs02_dn5 = assign84290_body64_e128910_d_n5;
            locals.var_fs02_dn6 = assign84290_body64_e128910_d_n6;
            locals.var_fs02_dn7 = assign84290_body64_e128910_d_n7;
            locals.var_fs02_dn8 = assign84290_body64_e128910_d_n8;
            locals.var_fs02_dn9 = assign84290_body64_e128910_d_n9;
            locals.var_fs02_dn10 = assign84290_body64_e128910_d_n10;
            locals.var_fs02_dn13 = assign84290_body64_e128910_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign84290_body65_e128934, assign84290_body65_e128934_d_n0, assign84290_body65_e128934_d_n2, assign84290_body65_e128934_d_n4, assign84290_body65_e128934_d_n5, assign84290_body65_e128934_d_n6, assign84290_body65_e128934_d_n7, assign84290_body65_e128934_d_n8, assign84290_body65_e128934_d_n9, assign84290_body65_e128934_d_n10, assign84290_body65_e128934_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1959 != 0.0)) {
        let assign84290_body65_e128925: f64 = (2.0 * locals.var_fb_dpss);
        let assign84290_body65_e128927: f64 = (assign84290_body65_e128925 * locals.var_fb);
        let assign84290_body65_e128929: f64 = (assign84290_body65_e128927 + locals.var_fs01_dps0);
        let assign84290_body65_e128930: f64 = (0.5 * assign84290_body65_e128929);
        let assign84290_body65_e128932: f64 = (assign84290_body65_e128930 / locals.var_fs02);
        (assign84290_body65_e128932, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn13) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn13)) + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84290_body65_e128934;
            locals.var_fs02_dps0_dn0 = assign84290_body65_e128934_d_n0;
            locals.var_fs02_dps0_dn2 = assign84290_body65_e128934_d_n2;
            locals.var_fs02_dps0_dn4 = assign84290_body65_e128934_d_n4;
            locals.var_fs02_dps0_dn5 = assign84290_body65_e128934_d_n5;
            locals.var_fs02_dps0_dn6 = assign84290_body65_e128934_d_n6;
            locals.var_fs02_dps0_dn7 = assign84290_body65_e128934_d_n7;
            locals.var_fs02_dps0_dn8 = assign84290_body65_e128934_d_n8;
            locals.var_fs02_dps0_dn9 = assign84290_body65_e128934_d_n9;
            locals.var_fs02_dps0_dn10 = assign84290_body65_e128934_d_n10;
            locals.var_fs02_dps0_dn13 = assign84290_body65_e128934_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign84290_body67_e128966, assign84290_body67_e128966_d_n0, assign84290_body67_e128966_d_n2, assign84290_body67_e128966_d_n4, assign84290_body67_e128966_d_n5, assign84290_body67_e128966_d_n6, assign84290_body67_e128966_d_n7, assign84290_body67_e128966_d_n8, assign84290_body67_e128966_d_n9, assign84290_body67_e128966_d_n10, assign84290_body67_e128966_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1959 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84290_body67_e128966;
            locals.var_fs02_dn0 = assign84290_body67_e128966_d_n0;
            locals.var_fs02_dn2 = assign84290_body67_e128966_d_n2;
            locals.var_fs02_dn4 = assign84290_body67_e128966_d_n4;
            locals.var_fs02_dn5 = assign84290_body67_e128966_d_n5;
            locals.var_fs02_dn6 = assign84290_body67_e128966_d_n6;
            locals.var_fs02_dn7 = assign84290_body67_e128966_d_n7;
            locals.var_fs02_dn8 = assign84290_body67_e128966_d_n8;
            locals.var_fs02_dn9 = assign84290_body67_e128966_d_n9;
            locals.var_fs02_dn10 = assign84290_body67_e128966_d_n10;
            locals.var_fs02_dn13 = assign84290_body67_e128966_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign84290_body68_e128981, assign84290_body68_e128981_d_n0, assign84290_body68_e128981_d_n2, assign84290_body68_e128981_d_n4, assign84290_body68_e128981_d_n5, assign84290_body68_e128981_d_n6, assign84290_body68_e128981_d_n7, assign84290_body68_e128981_d_n8, assign84290_body68_e128981_d_n9, assign84290_body68_e128981_d_n10, assign84290_body68_e128981_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1959 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84290_body68_e128981;
            locals.var_fs02_dps0_dn0 = assign84290_body68_e128981_d_n0;
            locals.var_fs02_dps0_dn2 = assign84290_body68_e128981_d_n2;
            locals.var_fs02_dps0_dn4 = assign84290_body68_e128981_d_n4;
            locals.var_fs02_dps0_dn5 = assign84290_body68_e128981_d_n5;
            locals.var_fs02_dps0_dn6 = assign84290_body68_e128981_d_n6;
            locals.var_fs02_dps0_dn7 = assign84290_body68_e128981_d_n7;
            locals.var_fs02_dps0_dn8 = assign84290_body68_e128981_d_n8;
            locals.var_fs02_dps0_dn9 = assign84290_body68_e128981_d_n9;
            locals.var_fs02_dps0_dn10 = assign84290_body68_e128981_d_n10;
            locals.var_fs02_dps0_dn13 = assign84290_body68_e128981_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign84290_body69_e128997, assign84290_body69_e128997_d_n0, assign84290_body69_e128997_d_n2, assign84290_body69_e128997_d_n4, assign84290_body69_e128997_d_n5, assign84290_body69_e128997_d_n6, assign84290_body69_e128997_d_n7, assign84290_body69_e128997_d_n8, assign84290_body69_e128997_d_n9, assign84290_body69_e128997_d_n10, assign84290_body69_e128997_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body69_e128989: f64 = (-locals.var_vgpld);
        let assign84290_body69_e128991: f64 = (assign84290_body69_e128989 + locals.var_ps0ld);
        let assign84290_body69_e128994: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign84290_body69_e128995: f64 = (assign84290_body69_e128991 + assign84290_body69_e128994);
        (assign84290_body69_e128995, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign84290_body69_e128997;
            locals.var_fs0_dn0 = assign84290_body69_e128997_d_n0;
            locals.var_fs0_dn2 = assign84290_body69_e128997_d_n2;
            locals.var_fs0_dn4 = assign84290_body69_e128997_d_n4;
            locals.var_fs0_dn5 = assign84290_body69_e128997_d_n5;
            locals.var_fs0_dn6 = assign84290_body69_e128997_d_n6;
            locals.var_fs0_dn7 = assign84290_body69_e128997_d_n7;
            locals.var_fs0_dn8 = assign84290_body69_e128997_d_n8;
            locals.var_fs0_dn9 = assign84290_body69_e128997_d_n9;
            locals.var_fs0_dn10 = assign84290_body69_e128997_d_n10;
            locals.var_fs0_dn13 = assign84290_body69_e128997_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign84290_body70_e129010, assign84290_body70_e129010_d_n0, assign84290_body70_e129010_d_n2, assign84290_body70_e129010_d_n4, assign84290_body70_e129010_d_n5, assign84290_body70_e129010_d_n6, assign84290_body70_e129010_d_n7, assign84290_body70_e129010_d_n8, assign84290_body70_e129010_d_n9, assign84290_body70_e129010_d_n10, assign84290_body70_e129010_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body70_e129007: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign84290_body70_e129008: f64 = (1.0 + assign84290_body70_e129007);
        (assign84290_body70_e129008, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign84290_body70_e129010;
            locals.var_fs0_dps0_dn0 = assign84290_body70_e129010_d_n0;
            locals.var_fs0_dps0_dn2 = assign84290_body70_e129010_d_n2;
            locals.var_fs0_dps0_dn4 = assign84290_body70_e129010_d_n4;
            locals.var_fs0_dps0_dn5 = assign84290_body70_e129010_d_n5;
            locals.var_fs0_dps0_dn6 = assign84290_body70_e129010_d_n6;
            locals.var_fs0_dps0_dn7 = assign84290_body70_e129010_d_n7;
            locals.var_fs0_dps0_dn8 = assign84290_body70_e129010_d_n8;
            locals.var_fs0_dps0_dn9 = assign84290_body70_e129010_d_n9;
            locals.var_fs0_dps0_dn10 = assign84290_body70_e129010_d_n10;
            locals.var_fs0_dps0_dn13 = assign84290_body70_e129010_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign84290_body71_e129013: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1960 = assign84290_body71_e129013;
            locals.var_guard1960_rv = 0.0;
            let (assign84290_body72_e129026,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 != 0.0)) {
        let assign84290_body72_e129024: f64 = (locals.var_lp_s0_max + 1.0);
        (assign84290_body72_e129024,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign84290_body72_e129026;
            locals.var_lp_s0_rv = 0.0;
            let (assign84290_body73_e129041, assign84290_body73_e129041_d_n0, assign84290_body73_e129041_d_n2, assign84290_body73_e129041_d_n4, assign84290_body73_e129041_d_n5, assign84290_body73_e129041_d_n6, assign84290_body73_e129041_d_n7, assign84290_body73_e129041_d_n8, assign84290_body73_e129041_d_n9, assign84290_body73_e129041_d_n10, assign84290_body73_e129041_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 == 0.0)) {
        let assign84290_body73_e129037: f64 = (-locals.var_fs0);
        let assign84290_body73_e129039: f64 = (assign84290_body73_e129037 / locals.var_fs0_dps0);
        (assign84290_body73_e129039, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign84290_body73_e129041;
            locals.var_dps0_dn0 = assign84290_body73_e129041_d_n0;
            locals.var_dps0_dn2 = assign84290_body73_e129041_d_n2;
            locals.var_dps0_dn4 = assign84290_body73_e129041_d_n4;
            locals.var_dps0_dn5 = assign84290_body73_e129041_d_n5;
            locals.var_dps0_dn6 = assign84290_body73_e129041_d_n6;
            locals.var_dps0_dn7 = assign84290_body73_e129041_d_n7;
            locals.var_dps0_dn8 = assign84290_body73_e129041_d_n8;
            locals.var_dps0_dn9 = assign84290_body73_e129041_d_n9;
            locals.var_dps0_dn10 = assign84290_body73_e129041_d_n10;
            locals.var_dps0_dn13 = assign84290_body73_e129041_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign84290_body74_e129066, assign84290_body74_e129066_d_n0, assign84290_body74_e129066_d_n2, assign84290_body74_e129066_d_n4, assign84290_body74_e129066_d_n5, assign84290_body74_e129066_d_n6, assign84290_body74_e129066_d_n7, assign84290_body74_e129066_d_n8, assign84290_body74_e129066_d_n9, assign84290_body74_e129066_d_n10, assign84290_body74_e129066_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 == 0.0)) {
        let assign84290_body74_e129053: f64 = (0.5 * 0.1);
        let assign84290_body74_e129057: f64 = (locals.var_ps0ld).abs();
        let (assign84290_body74_e129062, assign84290_body74_e129062_d_n0, assign84290_body74_e129062_d_n2, assign84290_body74_e129062_d_n4, assign84290_body74_e129062_d_n5, assign84290_body74_e129062_d_n6, assign84290_body74_e129062_d_n7, assign84290_body74_e129062_d_n8, assign84290_body74_e129062_d_n9, assign84290_body74_e129062_d_n10, assign84290_body74_e129062_d_n13,) = {
            if (1.0 >= assign84290_body74_e129057) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign84290_body74_e129061: f64 = (locals.var_ps0ld).abs();
                (assign84290_body74_e129061, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign84290_body74_e129063: f64 = (1.0 + assign84290_body74_e129062);
        let assign84290_body74_e129064: f64 = (assign84290_body74_e129053 * assign84290_body74_e129063);
        (assign84290_body74_e129064, (assign84290_body74_e129053 * assign84290_body74_e129062_d_n0), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n2), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n4), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n5), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n6), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n7), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n8), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n9), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n10), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign84290_body74_e129066;
            locals.var_dplim_dn0 = assign84290_body74_e129066_d_n0;
            locals.var_dplim_dn2 = assign84290_body74_e129066_d_n2;
            locals.var_dplim_dn4 = assign84290_body74_e129066_d_n4;
            locals.var_dplim_dn5 = assign84290_body74_e129066_d_n5;
            locals.var_dplim_dn6 = assign84290_body74_e129066_d_n6;
            locals.var_dplim_dn7 = assign84290_body74_e129066_d_n7;
            locals.var_dplim_dn8 = assign84290_body74_e129066_d_n8;
            locals.var_dplim_dn9 = assign84290_body74_e129066_d_n9;
            locals.var_dplim_dn10 = assign84290_body74_e129066_d_n10;
            locals.var_dplim_dn13 = assign84290_body74_e129066_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign84290_body75_e129068: f64 = (locals.var_dps0).abs();
            let assign84290_body75_e129070: f64 = if assign84290_body75_e129068 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1961 = assign84290_body75_e129070;
            locals.var_guard1961_rv = 0.0;
            let (assign84290_body76_e129092, assign84290_body76_e129092_d_n0, assign84290_body76_e129092_d_n2, assign84290_body76_e129092_d_n4, assign84290_body76_e129092_d_n5, assign84290_body76_e129092_d_n6, assign84290_body76_e129092_d_n7, assign84290_body76_e129092_d_n8, assign84290_body76_e129092_d_n9, assign84290_body76_e129092_d_n10, assign84290_body76_e129092_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 == 0.0)) && (locals.var_guard1961 != 0.0)) {
        let (assign84290_body76_e129089,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign84290_body76_e129088: f64 = (-1.0);
                (assign84290_body76_e129088,)
            }
        };
        let assign84290_body76_e129090: f64 = (locals.var_dplim * assign84290_body76_e129089);
        (assign84290_body76_e129090, (locals.var_dplim_dn0 * assign84290_body76_e129089), (locals.var_dplim_dn2 * assign84290_body76_e129089), (locals.var_dplim_dn4 * assign84290_body76_e129089), (locals.var_dplim_dn5 * assign84290_body76_e129089), (locals.var_dplim_dn6 * assign84290_body76_e129089), (locals.var_dplim_dn7 * assign84290_body76_e129089), (locals.var_dplim_dn8 * assign84290_body76_e129089), (locals.var_dplim_dn9 * assign84290_body76_e129089), (locals.var_dplim_dn10 * assign84290_body76_e129089), (locals.var_dplim_dn13 * assign84290_body76_e129089),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign84290_body76_e129092;
            locals.var_dps0_dn0 = assign84290_body76_e129092_d_n0;
            locals.var_dps0_dn2 = assign84290_body76_e129092_d_n2;
            locals.var_dps0_dn4 = assign84290_body76_e129092_d_n4;
            locals.var_dps0_dn5 = assign84290_body76_e129092_d_n5;
            locals.var_dps0_dn6 = assign84290_body76_e129092_d_n6;
            locals.var_dps0_dn7 = assign84290_body76_e129092_d_n7;
            locals.var_dps0_dn8 = assign84290_body76_e129092_d_n8;
            locals.var_dps0_dn9 = assign84290_body76_e129092_d_n9;
            locals.var_dps0_dn10 = assign84290_body76_e129092_d_n10;
            locals.var_dps0_dn13 = assign84290_body76_e129092_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign84290_body77_e129106, assign84290_body77_e129106_d_n0, assign84290_body77_e129106_d_n2, assign84290_body77_e129106_d_n4, assign84290_body77_e129106_d_n5, assign84290_body77_e129106_d_n6, assign84290_body77_e129106_d_n7, assign84290_body77_e129106_d_n8, assign84290_body77_e129106_d_n9, assign84290_body77_e129106_d_n10, assign84290_body77_e129106_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 == 0.0)) {
        let assign84290_body77_e129104: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign84290_body77_e129104, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign84290_body77_e129106;
            locals.var_ps0ld_dn0 = assign84290_body77_e129106_d_n0;
            locals.var_ps0ld_dn2 = assign84290_body77_e129106_d_n2;
            locals.var_ps0ld_dn4 = assign84290_body77_e129106_d_n4;
            locals.var_ps0ld_dn5 = assign84290_body77_e129106_d_n5;
            locals.var_ps0ld_dn6 = assign84290_body77_e129106_d_n6;
            locals.var_ps0ld_dn7 = assign84290_body77_e129106_d_n7;
            locals.var_ps0ld_dn8 = assign84290_body77_e129106_d_n8;
            locals.var_ps0ld_dn9 = assign84290_body77_e129106_d_n9;
            locals.var_ps0ld_dn10 = assign84290_body77_e129106_d_n10;
            locals.var_ps0ld_dn13 = assign84290_body77_e129106_d_n13;
            locals.var_ps0ld_rv = 0.0;
            let assign84290_body78_e129108: f64 = (locals.var_dps0).abs();
            let assign84290_body78_e129112: f64 = (locals.var_fs0).abs();
            let assign84290_body78_e129115: f64 = if ((assign84290_body78_e129108 <= 1e-12) && (assign84290_body78_e129112 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1962 = assign84290_body78_e129115;
            locals.var_guard1962_rv = 0.0;
            let (assign84290_body79_e129129,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 == 0.0)) && (locals.var_guard1962 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign84290_body79_e129129;
            locals.var_flg_conv_rv = 0.0;
            let (assign84290_body80_e129140,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body80_e129138: f64 = (locals.var_lp_s0 + 1.0);
        (assign84290_body80_e129138,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign84290_body80_e129140;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_311(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign84310_e129154, assign84310_e129154_d_n0, assign84310_e129154_d_n2, assign84310_e129154_d_n4, assign84310_e129154_d_n5, assign84310_e129154_d_n6, assign84310_e129154_d_n7, assign84310_e129154_d_n8, assign84310_e129154_d_n9, assign84310_e129154_d_n10, assign84310_e129154_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84310_e129152: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign84310_e129152, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk1881, locals.var_wdld__blk1881_dn0, locals.var_wdld__blk1881_dn2, locals.var_wdld__blk1881_dn4, locals.var_wdld__blk1881_dn5, locals.var_wdld__blk1881_dn6, locals.var_wdld__blk1881_dn7, locals.var_wdld__blk1881_dn8, locals.var_wdld__blk1881_dn9, locals.var_wdld__blk1881_dn10, locals.var_wdld__blk1881_dn13,)
    }
};
        locals.var_wdld__blk1881 = assign84310_e129154;
        locals.var_wdld__blk1881_dn0 = assign84310_e129154_d_n0;
        locals.var_wdld__blk1881_dn2 = assign84310_e129154_d_n2;
        locals.var_wdld__blk1881_dn4 = assign84310_e129154_d_n4;
        locals.var_wdld__blk1881_dn5 = assign84310_e129154_d_n5;
        locals.var_wdld__blk1881_dn6 = assign84310_e129154_d_n6;
        locals.var_wdld__blk1881_dn7 = assign84310_e129154_d_n7;
        locals.var_wdld__blk1881_dn8 = assign84310_e129154_d_n8;
        locals.var_wdld__blk1881_dn9 = assign84310_e129154_d_n9;
        locals.var_wdld__blk1881_dn10 = assign84310_e129154_d_n10;
        locals.var_wdld__blk1881_dn13 = assign84310_e129154_d_n13;
        locals.var_wdld__blk1881_rv = 0.0;

        let (assign84320_e129165, assign84320_e129165_d_n0, assign84320_e129165_d_n2, assign84320_e129165_d_n4, assign84320_e129165_d_n5, assign84320_e129165_d_n6, assign84320_e129165_d_n7, assign84320_e129165_d_n8, assign84320_e129165_d_n9, assign84320_e129165_d_n10, assign84320_e129165_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84320_e129163: f64 = (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881);
        (assign84320_e129163, (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn0), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn2), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn4), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn5), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn6), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn7), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn8), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn9), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn10), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn13),)
    } else {
        (locals.var_q_dep_ld__blk1882, locals.var_q_dep_ld__blk1882_dn0, locals.var_q_dep_ld__blk1882_dn2, locals.var_q_dep_ld__blk1882_dn4, locals.var_q_dep_ld__blk1882_dn5, locals.var_q_dep_ld__blk1882_dn6, locals.var_q_dep_ld__blk1882_dn7, locals.var_q_dep_ld__blk1882_dn8, locals.var_q_dep_ld__blk1882_dn9, locals.var_q_dep_ld__blk1882_dn10, locals.var_q_dep_ld__blk1882_dn13,)
    }
};
        locals.var_q_dep_ld__blk1882 = assign84320_e129165;
        locals.var_q_dep_ld__blk1882_dn0 = assign84320_e129165_d_n0;
        locals.var_q_dep_ld__blk1882_dn2 = assign84320_e129165_d_n2;
        locals.var_q_dep_ld__blk1882_dn4 = assign84320_e129165_d_n4;
        locals.var_q_dep_ld__blk1882_dn5 = assign84320_e129165_d_n5;
        locals.var_q_dep_ld__blk1882_dn6 = assign84320_e129165_d_n6;
        locals.var_q_dep_ld__blk1882_dn7 = assign84320_e129165_d_n7;
        locals.var_q_dep_ld__blk1882_dn8 = assign84320_e129165_d_n8;
        locals.var_q_dep_ld__blk1882_dn9 = assign84320_e129165_d_n9;
        locals.var_q_dep_ld__blk1882_dn10 = assign84320_e129165_d_n10;
        locals.var_q_dep_ld__blk1882_dn13 = assign84320_e129165_d_n13;
        locals.var_q_dep_ld__blk1882_rv = 0.0;

        let (assign84330_e129180, assign84330_e129180_d_n0, assign84330_e129180_d_n2, assign84330_e129180_d_n4, assign84330_e129180_d_n5, assign84330_e129180_d_n6, assign84330_e129180_d_n7, assign84330_e129180_d_n8, assign84330_e129180_d_n9, assign84330_e129180_d_n10, assign84330_e129180_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84330_e129174: f64 = (locals.var_q_dep_ld__blk1882 / locals.var_cnst0over_func);
        let assign84330_e129177: f64 = (10.0 * 2.220446049250313e-16);
        let assign84330_e129178: f64 = (assign84330_e129174 + assign84330_e129177);
        (assign84330_e129178, (((locals.var_q_dep_ld__blk1882_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign84330_e129180;
        locals.var_xi0p12_dn0 = assign84330_e129180_d_n0;
        locals.var_xi0p12_dn2 = assign84330_e129180_d_n2;
        locals.var_xi0p12_dn4 = assign84330_e129180_d_n4;
        locals.var_xi0p12_dn5 = assign84330_e129180_d_n5;
        locals.var_xi0p12_dn6 = assign84330_e129180_d_n6;
        locals.var_xi0p12_dn7 = assign84330_e129180_d_n7;
        locals.var_xi0p12_dn8 = assign84330_e129180_d_n8;
        locals.var_xi0p12_dn9 = assign84330_e129180_d_n9;
        locals.var_xi0p12_dn10 = assign84330_e129180_d_n10;
        locals.var_xi0p12_dn13 = assign84330_e129180_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign84340_e129191, assign84340_e129191_d_n0, assign84340_e129191_d_n2, assign84340_e129191_d_n4, assign84340_e129191_d_n5, assign84340_e129191_d_n6, assign84340_e129191_d_n7, assign84340_e129191_d_n8, assign84340_e129191_d_n9, assign84340_e129191_d_n10, assign84340_e129191_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84340_e129189: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign84340_e129189, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign84340_e129191;
        locals.var_qbuld_dn0 = assign84340_e129191_d_n0;
        locals.var_qbuld_dn2 = assign84340_e129191_d_n2;
        locals.var_qbuld_dn4 = assign84340_e129191_d_n4;
        locals.var_qbuld_dn5 = assign84340_e129191_d_n5;
        locals.var_qbuld_dn6 = assign84340_e129191_d_n6;
        locals.var_qbuld_dn7 = assign84340_e129191_d_n7;
        locals.var_qbuld_dn8 = assign84340_e129191_d_n8;
        locals.var_qbuld_dn9 = assign84340_e129191_d_n9;
        locals.var_qbuld_dn10 = assign84340_e129191_d_n10;
        locals.var_qbuld_dn13 = assign84340_e129191_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign84350_e129204, assign84350_e129204_d_n0, assign84350_e129204_d_n2, assign84350_e129204_d_n4, assign84350_e129204_d_n5, assign84350_e129204_d_n6, assign84350_e129204_d_n7, assign84350_e129204_d_n8, assign84350_e129204_d_n9, assign84350_e129204_d_n10, assign84350_e129204_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84350_e129201: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign84350_e129202: f64 = (1.0 / assign84350_e129201);
        (assign84350_e129202, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign84350_e129201 * assign84350_e129201))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign84350_e129204;
        locals.var_t1_dn0 = assign84350_e129204_d_n0;
        locals.var_t1_dn2 = assign84350_e129204_d_n2;
        locals.var_t1_dn4 = assign84350_e129204_d_n4;
        locals.var_t1_dn5 = assign84350_e129204_d_n5;
        locals.var_t1_dn6 = assign84350_e129204_d_n6;
        locals.var_t1_dn7 = assign84350_e129204_d_n7;
        locals.var_t1_dn8 = assign84350_e129204_d_n8;
        locals.var_t1_dn9 = assign84350_e129204_d_n9;
        locals.var_t1_dn10 = assign84350_e129204_d_n10;
        locals.var_t1_dn13 = assign84350_e129204_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign84360_e129217, assign84360_e129217_d_n0, assign84360_e129217_d_n2, assign84360_e129217_d_n4, assign84360_e129217_d_n5, assign84360_e129217_d_n6, assign84360_e129217_d_n7, assign84360_e129217_d_n8, assign84360_e129217_d_n9, assign84360_e129217_d_n10, assign84360_e129217_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84360_e129213: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign84360_e129215: f64 = (assign84360_e129213 * locals.var_t1);
        (assign84360_e129215, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign84360_e129217;
        locals.var_qiuld_dn0 = assign84360_e129217_d_n0;
        locals.var_qiuld_dn2 = assign84360_e129217_d_n2;
        locals.var_qiuld_dn4 = assign84360_e129217_d_n4;
        locals.var_qiuld_dn5 = assign84360_e129217_d_n5;
        locals.var_qiuld_dn6 = assign84360_e129217_d_n6;
        locals.var_qiuld_dn7 = assign84360_e129217_d_n7;
        locals.var_qiuld_dn8 = assign84360_e129217_d_n8;
        locals.var_qiuld_dn9 = assign84360_e129217_d_n9;
        locals.var_qiuld_dn10 = assign84360_e129217_d_n10;
        locals.var_qiuld_dn13 = assign84360_e129217_d_n13;
        locals.var_qiuld_rv = 0.0;

        let (assign84370_e129228, assign84370_e129228_d_n0, assign84370_e129228_d_n2, assign84370_e129228_d_n4, assign84370_e129228_d_n5, assign84370_e129228_d_n6, assign84370_e129228_d_n7, assign84370_e129228_d_n8, assign84370_e129228_d_n9, assign84370_e129228_d_n10, assign84370_e129228_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84370_e129226: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign84370_e129226, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign84370_e129228;
        locals.var_qsuld_dn0 = assign84370_e129228_d_n0;
        locals.var_qsuld_dn2 = assign84370_e129228_d_n2;
        locals.var_qsuld_dn4 = assign84370_e129228_d_n4;
        locals.var_qsuld_dn5 = assign84370_e129228_d_n5;
        locals.var_qsuld_dn6 = assign84370_e129228_d_n6;
        locals.var_qsuld_dn7 = assign84370_e129228_d_n7;
        locals.var_qsuld_dn8 = assign84370_e129228_d_n8;
        locals.var_qsuld_dn9 = assign84370_e129228_d_n9;
        locals.var_qsuld_dn10 = assign84370_e129228_d_n10;
        locals.var_qsuld_dn13 = assign84370_e129228_d_n13;
        locals.var_qsuld_rv = 0.0;

        let assign84380_e129231: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1964 = assign84380_e129231;
        locals.var_guard1964_rv = 0.0;

        let (assign84390_e129241, assign84390_e129241_d_n0, assign84390_e129241_d_n2, assign84390_e129241_d_n4, assign84390_e129241_d_n5, assign84390_e129241_d_n6, assign84390_e129241_d_n7, assign84390_e129241_d_n8, assign84390_e129241_d_n9, assign84390_e129241_d_n10, assign84390_e129241_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84390_e129237: f64 = (-locals.var_vxbgmtcl);
        let assign84390_e129238: f64 = (locals.var_beta * assign84390_e129237);
        let assign84390_e129239: f64 = (assign84390_e129238).exp();
        (assign84390_e129239, (assign84390_e129239 * ((locals.var_beta_dn0 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign84390_e129239 * ((locals.var_beta_dn2 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign84390_e129239 * ((locals.var_beta_dn4 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign84390_e129239 * ((locals.var_beta_dn5 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign84390_e129239 * ((locals.var_beta_dn6 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign84390_e129239 * ((locals.var_beta_dn7 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign84390_e129239 * ((locals.var_beta_dn8 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign84390_e129239 * ((locals.var_beta_dn9 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign84390_e129239 * ((locals.var_beta_dn10 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign84390_e129239 * ((locals.var_beta_dn13 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign84390_e129241;
        locals.var_exp_bvbs_dn0 = assign84390_e129241_d_n0;
        locals.var_exp_bvbs_dn2 = assign84390_e129241_d_n2;
        locals.var_exp_bvbs_dn4 = assign84390_e129241_d_n4;
        locals.var_exp_bvbs_dn5 = assign84390_e129241_d_n5;
        locals.var_exp_bvbs_dn6 = assign84390_e129241_d_n6;
        locals.var_exp_bvbs_dn7 = assign84390_e129241_d_n7;
        locals.var_exp_bvbs_dn8 = assign84390_e129241_d_n8;
        locals.var_exp_bvbs_dn9 = assign84390_e129241_d_n9;
        locals.var_exp_bvbs_dn10 = assign84390_e129241_d_n10;
        locals.var_exp_bvbs_dn13 = assign84390_e129241_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign84400_e129249, assign84400_e129249_d_n0, assign84400_e129249_d_n2, assign84400_e129249_d_n4, assign84400_e129249_d_n5, assign84400_e129249_d_n6, assign84400_e129249_d_n7, assign84400_e129249_d_n8, assign84400_e129249_d_n9, assign84400_e129249_d_n10, assign84400_e129249_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84400_e129247: f64 = (locals.var_nin / locals.var_nover_func);
        (assign84400_e129247, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84400_e129249;
        locals.var_t0_dn0 = assign84400_e129249_d_n0;
        locals.var_t0_dn2 = assign84400_e129249_d_n2;
        locals.var_t0_dn4 = assign84400_e129249_d_n4;
        locals.var_t0_dn5 = assign84400_e129249_d_n5;
        locals.var_t0_dn6 = assign84400_e129249_d_n6;
        locals.var_t0_dn7 = assign84400_e129249_d_n7;
        locals.var_t0_dn8 = assign84400_e129249_d_n8;
        locals.var_t0_dn9 = assign84400_e129249_d_n9;
        locals.var_t0_dn10 = assign84400_e129249_d_n10;
        locals.var_t0_dn13 = assign84400_e129249_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign84410_e129257, assign84410_e129257_d_n0, assign84410_e129257_d_n2, assign84410_e129257_d_n4, assign84410_e129257_d_n5, assign84410_e129257_d_n6, assign84410_e129257_d_n7, assign84410_e129257_d_n8, assign84410_e129257_d_n9, assign84410_e129257_d_n10, assign84410_e129257_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84410_e129255: f64 = (locals.var_t0 * locals.var_t0);
        (assign84410_e129255, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign84410_e129257;
        locals.var_cnst1over_dn0 = assign84410_e129257_d_n0;
        locals.var_cnst1over_dn2 = assign84410_e129257_d_n2;
        locals.var_cnst1over_dn4 = assign84410_e129257_d_n4;
        locals.var_cnst1over_dn5 = assign84410_e129257_d_n5;
        locals.var_cnst1over_dn6 = assign84410_e129257_d_n6;
        locals.var_cnst1over_dn7 = assign84410_e129257_d_n7;
        locals.var_cnst1over_dn8 = assign84410_e129257_d_n8;
        locals.var_cnst1over_dn9 = assign84410_e129257_d_n9;
        locals.var_cnst1over_dn10 = assign84410_e129257_d_n10;
        locals.var_cnst1over_dn13 = assign84410_e129257_d_n13;
        locals.var_cnst1over_rv = 0.0;

        let (assign84420_e129265, assign84420_e129265_d_n0, assign84420_e129265_d_n2, assign84420_e129265_d_n4, assign84420_e129265_d_n5, assign84420_e129265_d_n6, assign84420_e129265_d_n7, assign84420_e129265_d_n8, assign84420_e129265_d_n9, assign84420_e129265_d_n10, assign84420_e129265_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84420_e129263: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign84420_e129263, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign84420_e129265;
        locals.var_cfs1_dn0 = assign84420_e129265_d_n0;
        locals.var_cfs1_dn2 = assign84420_e129265_d_n2;
        locals.var_cfs1_dn4 = assign84420_e129265_d_n4;
        locals.var_cfs1_dn5 = assign84420_e129265_d_n5;
        locals.var_cfs1_dn6 = assign84420_e129265_d_n6;
        locals.var_cfs1_dn7 = assign84420_e129265_d_n7;
        locals.var_cfs1_dn8 = assign84420_e129265_d_n8;
        locals.var_cfs1_dn9 = assign84420_e129265_d_n9;
        locals.var_cfs1_dn10 = assign84420_e129265_d_n10;
        locals.var_cfs1_dn13 = assign84420_e129265_d_n13;
        locals.var_cfs1_rv = 0.0;

        let (assign84430_e129271, assign84430_e129271_d_n0, assign84430_e129271_d_n2, assign84430_e129271_d_n4, assign84430_e129271_d_n5, assign84430_e129271_d_n6, assign84430_e129271_d_n7, assign84430_e129271_d_n8, assign84430_e129271_d_n9, assign84430_e129271_d_n10, assign84430_e129271_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (locals.var_ps0ld_ini__blk1890, locals.var_ps0ld_ini__blk1890_dn0, locals.var_ps0ld_ini__blk1890_dn2, locals.var_ps0ld_ini__blk1890_dn4, locals.var_ps0ld_ini__blk1890_dn5, locals.var_ps0ld_ini__blk1890_dn6, locals.var_ps0ld_ini__blk1890_dn7, locals.var_ps0ld_ini__blk1890_dn8, locals.var_ps0ld_ini__blk1890_dn9, locals.var_ps0ld_ini__blk1890_dn10, locals.var_ps0ld_ini__blk1890_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign84430_e129271;
        locals.var_ps0ld_dn0 = assign84430_e129271_d_n0;
        locals.var_ps0ld_dn2 = assign84430_e129271_d_n2;
        locals.var_ps0ld_dn4 = assign84430_e129271_d_n4;
        locals.var_ps0ld_dn5 = assign84430_e129271_d_n5;
        locals.var_ps0ld_dn6 = assign84430_e129271_d_n6;
        locals.var_ps0ld_dn7 = assign84430_e129271_d_n7;
        locals.var_ps0ld_dn8 = assign84430_e129271_d_n8;
        locals.var_ps0ld_dn9 = assign84430_e129271_d_n9;
        locals.var_ps0ld_dn10 = assign84430_e129271_d_n10;
        locals.var_ps0ld_dn13 = assign84430_e129271_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign84440_e129277,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign84440_e129277;
        locals.var_flg_conv_rv = 0.0;

        let (assign84450_e129290, assign84450_e129290_d_n0, assign84450_e129290_d_n2, assign84450_e129290_d_n4, assign84450_e129290_d_n5, assign84450_e129290_d_n6, assign84450_e129290_d_n7, assign84450_e129290_d_n8, assign84450_e129290_d_n9, assign84450_e129290_d_n10, assign84450_e129290_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84450_e129284: f64 = (1.034943e-10 / locals.var_q_nsubld__blk1883);
        let assign84450_e129286: f64 = (assign84450_e129284 * locals.var_beta_inv);
        let assign84450_e129287: f64 = (2.0 * assign84450_e129286);
        let assign84450_e129288: f64 = (assign84450_e129287).sqrt();
        (assign84450_e129288, ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn0)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn2)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn4)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn5)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn6)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn7)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn8)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn9)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn10)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn13)) / (2.0 * assign84450_e129288)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign84450_e129290;
        locals.var_c_w_ld_dn0 = assign84450_e129290_d_n0;
        locals.var_c_w_ld_dn2 = assign84450_e129290_d_n2;
        locals.var_c_w_ld_dn4 = assign84450_e129290_d_n4;
        locals.var_c_w_ld_dn5 = assign84450_e129290_d_n5;
        locals.var_c_w_ld_dn6 = assign84450_e129290_d_n6;
        locals.var_c_w_ld_dn7 = assign84450_e129290_d_n7;
        locals.var_c_w_ld_dn8 = assign84450_e129290_d_n8;
        locals.var_c_w_ld_dn9 = assign84450_e129290_d_n9;
        locals.var_c_w_ld_dn10 = assign84450_e129290_d_n10;
        locals.var_c_w_ld_dn13 = assign84450_e129290_d_n13;
        locals.var_c_w_ld_rv = 0.0;

        let assign84460_e129293: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1965 = assign84460_e129293;
        locals.var_guard1965_rv = 0.0;

        let (assign84470_e129303, assign84470_e129303_d_n0, assign84470_e129303_d_n2, assign84470_e129303_d_n4, assign84470_e129303_d_n5, assign84470_e129303_d_n6, assign84470_e129303_d_n7, assign84470_e129303_d_n8, assign84470_e129303_d_n9, assign84470_e129303_d_n10, assign84470_e129303_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 != 0.0)) {
        let assign84470_e129301: f64 = (p.p334 - locals.var_wdep_func);
        (assign84470_e129301, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84470_e129303;
        locals.var_t2_dn0 = assign84470_e129303_d_n0;
        locals.var_t2_dn2 = assign84470_e129303_d_n2;
        locals.var_t2_dn4 = assign84470_e129303_d_n4;
        locals.var_t2_dn5 = assign84470_e129303_d_n5;
        locals.var_t2_dn6 = assign84470_e129303_d_n6;
        locals.var_t2_dn7 = assign84470_e129303_d_n7;
        locals.var_t2_dn8 = assign84470_e129303_d_n8;
        locals.var_t2_dn9 = assign84470_e129303_d_n9;
        locals.var_t2_dn10 = assign84470_e129303_d_n10;
        locals.var_t2_dn13 = assign84470_e129303_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84480_e129325, assign84480_e129325_d_n0, assign84480_e129325_d_n2, assign84480_e129325_d_n4, assign84480_e129325_d_n5, assign84480_e129325_d_n6, assign84480_e129325_d_n7, assign84480_e129325_d_n8, assign84480_e129325_d_n9, assign84480_e129325_d_n10, assign84480_e129325_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84480_e129312: f64 = (locals.var_vdsi + p.p137);
        let assign84480_e129315: f64 = (locals.var_vdsi + p.p137);
        let assign84480_e129316: f64 = (assign84480_e129312 * assign84480_e129315);
        let assign84480_e129319: f64 = (4.0 * 0.1);
        let assign84480_e129321: f64 = (assign84480_e129319 * 0.1);
        let assign84480_e129322: f64 = (assign84480_e129316 + assign84480_e129321);
        let assign84480_e129323: f64 = (assign84480_e129322).sqrt();
        (assign84480_e129323, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign84480_e129315) + (assign84480_e129312 * locals.var_vdsi_dn5)) / (2.0 * assign84480_e129323)), 0.0, (((locals.var_vdsi_dn7 * assign84480_e129315) + (assign84480_e129312 * locals.var_vdsi_dn7)) / (2.0 * assign84480_e129323)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84480_e129325;
        locals.var_tmf2_dn0 = assign84480_e129325_d_n0;
        locals.var_tmf2_dn2 = assign84480_e129325_d_n2;
        locals.var_tmf2_dn4 = assign84480_e129325_d_n4;
        locals.var_tmf2_dn5 = assign84480_e129325_d_n5;
        locals.var_tmf2_dn6 = assign84480_e129325_d_n6;
        locals.var_tmf2_dn7 = assign84480_e129325_d_n7;
        locals.var_tmf2_dn8 = assign84480_e129325_d_n8;
        locals.var_tmf2_dn9 = assign84480_e129325_d_n9;
        locals.var_tmf2_dn10 = assign84480_e129325_d_n10;
        locals.var_tmf2_dn13 = assign84480_e129325_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign84490_e129342, assign84490_e129342_d_n0, assign84490_e129342_d_n2, assign84490_e129342_d_n4, assign84490_e129342_d_n5, assign84490_e129342_d_n6, assign84490_e129342_d_n7, assign84490_e129342_d_n8, assign84490_e129342_d_n9, assign84490_e129342_d_n10, assign84490_e129342_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84490_e129336: f64 = (locals.var_vdsi + p.p137);
        let assign84490_e129338: f64 = (assign84490_e129336 / locals.var_tmf2);
        let assign84490_e129339: f64 = (1.0 + assign84490_e129338);
        let assign84490_e129340: f64 = (0.5 * assign84490_e129339);
        (assign84490_e129340, (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign84490_e129336 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign84490_e129336 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84490_e129342;
        locals.var_t9_dn0 = assign84490_e129342_d_n0;
        locals.var_t9_dn2 = assign84490_e129342_d_n2;
        locals.var_t9_dn4 = assign84490_e129342_d_n4;
        locals.var_t9_dn5 = assign84490_e129342_d_n5;
        locals.var_t9_dn6 = assign84490_e129342_d_n6;
        locals.var_t9_dn7 = assign84490_e129342_d_n7;
        locals.var_t9_dn8 = assign84490_e129342_d_n8;
        locals.var_t9_dn9 = assign84490_e129342_d_n9;
        locals.var_t9_dn10 = assign84490_e129342_d_n10;
        locals.var_t9_dn13 = assign84490_e129342_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84500_e129357, assign84500_e129357_d_n0, assign84500_e129357_d_n2, assign84500_e129357_d_n4, assign84500_e129357_d_n5, assign84500_e129357_d_n6, assign84500_e129357_d_n7, assign84500_e129357_d_n8, assign84500_e129357_d_n9, assign84500_e129357_d_n10, assign84500_e129357_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84500_e129352: f64 = (locals.var_vdsi + p.p137);
        let assign84500_e129354: f64 = (assign84500_e129352 + locals.var_tmf2);
        let assign84500_e129355: f64 = (0.5 * assign84500_e129354);
        (assign84500_e129355, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84500_e129357;
        locals.var_t2_dn0 = assign84500_e129357_d_n0;
        locals.var_t2_dn2 = assign84500_e129357_d_n2;
        locals.var_t2_dn4 = assign84500_e129357_d_n4;
        locals.var_t2_dn5 = assign84500_e129357_d_n5;
        locals.var_t2_dn6 = assign84500_e129357_d_n6;
        locals.var_t2_dn7 = assign84500_e129357_d_n7;
        locals.var_t2_dn8 = assign84500_e129357_d_n8;
        locals.var_t2_dn9 = assign84500_e129357_d_n9;
        locals.var_t2_dn10 = assign84500_e129357_d_n10;
        locals.var_t2_dn13 = assign84500_e129357_d_n13;
        locals.var_t2_rv = 0.0;

        let assign84510_e129360: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1966 = assign84510_e129360;
        locals.var_guard1966_rv = 0.0;

        let (assign84520_e129371, assign84520_e129371_d_n0, assign84520_e129371_d_n2, assign84520_e129371_d_n4, assign84520_e129371_d_n5, assign84520_e129371_d_n6, assign84520_e129371_d_n7, assign84520_e129371_d_n8, assign84520_e129371_d_n9, assign84520_e129371_d_n10, assign84520_e129371_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) && (locals.var_guard1966 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84520_e129371;
        locals.var_t2_dn0 = assign84520_e129371_d_n0;
        locals.var_t2_dn2 = assign84520_e129371_d_n2;
        locals.var_t2_dn4 = assign84520_e129371_d_n4;
        locals.var_t2_dn5 = assign84520_e129371_d_n5;
        locals.var_t2_dn6 = assign84520_e129371_d_n6;
        locals.var_t2_dn7 = assign84520_e129371_d_n7;
        locals.var_t2_dn8 = assign84520_e129371_d_n8;
        locals.var_t2_dn9 = assign84520_e129371_d_n9;
        locals.var_t2_dn10 = assign84520_e129371_d_n10;
        locals.var_t2_dn13 = assign84520_e129371_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84530_e129382, assign84530_e129382_d_n0, assign84530_e129382_d_n2, assign84530_e129382_d_n4, assign84530_e129382_d_n5, assign84530_e129382_d_n6, assign84530_e129382_d_n7, assign84530_e129382_d_n8, assign84530_e129382_d_n9, assign84530_e129382_d_n10, assign84530_e129382_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) && (locals.var_guard1966 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84530_e129382;
        locals.var_t9_dn0 = assign84530_e129382_d_n0;
        locals.var_t9_dn2 = assign84530_e129382_d_n2;
        locals.var_t9_dn4 = assign84530_e129382_d_n4;
        locals.var_t9_dn5 = assign84530_e129382_d_n5;
        locals.var_t9_dn6 = assign84530_e129382_d_n6;
        locals.var_t9_dn7 = assign84530_e129382_d_n7;
        locals.var_t9_dn8 = assign84530_e129382_d_n8;
        locals.var_t9_dn9 = assign84530_e129382_d_n9;
        locals.var_t9_dn10 = assign84530_e129382_d_n10;
        locals.var_t9_dn13 = assign84530_e129382_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84540_e129396, assign84540_e129396_d_n0, assign84540_e129396_d_n2, assign84540_e129396_d_n4, assign84540_e129396_d_n5, assign84540_e129396_d_n6, assign84540_e129396_d_n7, assign84540_e129396_d_n8, assign84540_e129396_d_n9, assign84540_e129396_d_n10, assign84540_e129396_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84540_e129391: f64 = (locals.var_kjunc * locals.var_t2);
        let assign84540_e129392: f64 = (assign84540_e129391).sqrt();
        let assign84540_e129394: f64 = (assign84540_e129392 * p.p432);
        (assign84540_e129394, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign84540_e129392)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign84540_e129396;
        locals.var_wjunc0_dn0 = assign84540_e129396_d_n0;
        locals.var_wjunc0_dn2 = assign84540_e129396_d_n2;
        locals.var_wjunc0_dn4 = assign84540_e129396_d_n4;
        locals.var_wjunc0_dn5 = assign84540_e129396_d_n5;
        locals.var_wjunc0_dn6 = assign84540_e129396_d_n6;
        locals.var_wjunc0_dn7 = assign84540_e129396_d_n7;
        locals.var_wjunc0_dn8 = assign84540_e129396_d_n8;
        locals.var_wjunc0_dn9 = assign84540_e129396_d_n9;
        locals.var_wjunc0_dn10 = assign84540_e129396_d_n10;
        locals.var_wjunc0_dn13 = assign84540_e129396_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign84550_e129407, assign84550_e129407_d_n0, assign84550_e129407_d_n2, assign84550_e129407_d_n4, assign84550_e129407_d_n5, assign84550_e129407_d_n6, assign84550_e129407_d_n7, assign84550_e129407_d_n8, assign84550_e129407_d_n9, assign84550_e129407_d_n10, assign84550_e129407_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84550_e129405: f64 = (p.p334 - locals.var_wjunc0);
        (assign84550_e129405, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84550_e129407;
        locals.var_t2_dn0 = assign84550_e129407_d_n0;
        locals.var_t2_dn2 = assign84550_e129407_d_n2;
        locals.var_t2_dn4 = assign84550_e129407_d_n4;
        locals.var_t2_dn5 = assign84550_e129407_d_n5;
        locals.var_t2_dn6 = assign84550_e129407_d_n6;
        locals.var_t2_dn7 = assign84550_e129407_d_n7;
        locals.var_t2_dn8 = assign84550_e129407_d_n8;
        locals.var_t2_dn9 = assign84550_e129407_d_n9;
        locals.var_t2_dn10 = assign84550_e129407_d_n10;
        locals.var_t2_dn13 = assign84550_e129407_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84560_e129426, assign84560_e129426_d_n0, assign84560_e129426_d_n2, assign84560_e129426_d_n4, assign84560_e129426_d_n5, assign84560_e129426_d_n6, assign84560_e129426_d_n7, assign84560_e129426_d_n8, assign84560_e129426_d_n9, assign84560_e129426_d_n10, assign84560_e129426_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84560_e129413: f64 = (locals.var_t2 * locals.var_t2);
        let assign84560_e129417: f64 = (p.p334 * 0.01);
        let assign84560_e129418: f64 = (4.0 * assign84560_e129417);
        let assign84560_e129421: f64 = (p.p334 * 0.01);
        let assign84560_e129422: f64 = (assign84560_e129418 * assign84560_e129421);
        let assign84560_e129423: f64 = (assign84560_e129413 + assign84560_e129422);
        let assign84560_e129424: f64 = (assign84560_e129423).sqrt();
        (assign84560_e129424, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign84560_e129424)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84560_e129426;
        locals.var_tmf2_dn0 = assign84560_e129426_d_n0;
        locals.var_tmf2_dn2 = assign84560_e129426_d_n2;
        locals.var_tmf2_dn4 = assign84560_e129426_d_n4;
        locals.var_tmf2_dn5 = assign84560_e129426_d_n5;
        locals.var_tmf2_dn6 = assign84560_e129426_d_n6;
        locals.var_tmf2_dn7 = assign84560_e129426_d_n7;
        locals.var_tmf2_dn8 = assign84560_e129426_d_n8;
        locals.var_tmf2_dn9 = assign84560_e129426_d_n9;
        locals.var_tmf2_dn10 = assign84560_e129426_d_n10;
        locals.var_tmf2_dn13 = assign84560_e129426_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_312(
        locals: &mut StampLocals,
    ) {
        let (assign84570_e129438, assign84570_e129438_d_n0, assign84570_e129438_d_n2, assign84570_e129438_d_n4, assign84570_e129438_d_n5, assign84570_e129438_d_n6, assign84570_e129438_d_n7, assign84570_e129438_d_n8, assign84570_e129438_d_n9, assign84570_e129438_d_n10, assign84570_e129438_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84570_e129434: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign84570_e129435: f64 = (1.0 + assign84570_e129434);
        let assign84570_e129436: f64 = (0.5 * assign84570_e129435);
        (assign84570_e129436, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84570_e129438;
        locals.var_t9_dn0 = assign84570_e129438_d_n0;
        locals.var_t9_dn2 = assign84570_e129438_d_n2;
        locals.var_t9_dn4 = assign84570_e129438_d_n4;
        locals.var_t9_dn5 = assign84570_e129438_d_n5;
        locals.var_t9_dn6 = assign84570_e129438_d_n6;
        locals.var_t9_dn7 = assign84570_e129438_d_n7;
        locals.var_t9_dn8 = assign84570_e129438_d_n8;
        locals.var_t9_dn9 = assign84570_e129438_d_n9;
        locals.var_t9_dn10 = assign84570_e129438_d_n10;
        locals.var_t9_dn13 = assign84570_e129438_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84580_e129448, assign84580_e129448_d_n0, assign84580_e129448_d_n2, assign84580_e129448_d_n4, assign84580_e129448_d_n5, assign84580_e129448_d_n6, assign84580_e129448_d_n7, assign84580_e129448_d_n8, assign84580_e129448_d_n9, assign84580_e129448_d_n10, assign84580_e129448_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84580_e129445: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign84580_e129446: f64 = (0.5 * assign84580_e129445);
        (assign84580_e129446, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84580_e129448;
        locals.var_t2_dn0 = assign84580_e129448_d_n0;
        locals.var_t2_dn2 = assign84580_e129448_d_n2;
        locals.var_t2_dn4 = assign84580_e129448_d_n4;
        locals.var_t2_dn5 = assign84580_e129448_d_n5;
        locals.var_t2_dn6 = assign84580_e129448_d_n6;
        locals.var_t2_dn7 = assign84580_e129448_d_n7;
        locals.var_t2_dn8 = assign84580_e129448_d_n8;
        locals.var_t2_dn9 = assign84580_e129448_d_n9;
        locals.var_t2_dn10 = assign84580_e129448_d_n10;
        locals.var_t2_dn13 = assign84580_e129448_d_n13;
        locals.var_t2_rv = 0.0;

        let assign84590_e129451: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1967 = assign84590_e129451;
        locals.var_guard1967_rv = 0.0;

        let (assign84600_e129459, assign84600_e129459_d_n0, assign84600_e129459_d_n2, assign84600_e129459_d_n4, assign84600_e129459_d_n5, assign84600_e129459_d_n6, assign84600_e129459_d_n7, assign84600_e129459_d_n8, assign84600_e129459_d_n9, assign84600_e129459_d_n10, assign84600_e129459_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1967 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84600_e129459;
        locals.var_t2_dn0 = assign84600_e129459_d_n0;
        locals.var_t2_dn2 = assign84600_e129459_d_n2;
        locals.var_t2_dn4 = assign84600_e129459_d_n4;
        locals.var_t2_dn5 = assign84600_e129459_d_n5;
        locals.var_t2_dn6 = assign84600_e129459_d_n6;
        locals.var_t2_dn7 = assign84600_e129459_d_n7;
        locals.var_t2_dn8 = assign84600_e129459_d_n8;
        locals.var_t2_dn9 = assign84600_e129459_d_n9;
        locals.var_t2_dn10 = assign84600_e129459_d_n10;
        locals.var_t2_dn13 = assign84600_e129459_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84610_e129467, assign84610_e129467_d_n0, assign84610_e129467_d_n2, assign84610_e129467_d_n4, assign84610_e129467_d_n5, assign84610_e129467_d_n6, assign84610_e129467_d_n7, assign84610_e129467_d_n8, assign84610_e129467_d_n9, assign84610_e129467_d_n10, assign84610_e129467_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1967 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84610_e129467;
        locals.var_t9_dn0 = assign84610_e129467_d_n0;
        locals.var_t9_dn2 = assign84610_e129467_d_n2;
        locals.var_t9_dn4 = assign84610_e129467_d_n4;
        locals.var_t9_dn5 = assign84610_e129467_d_n5;
        locals.var_t9_dn6 = assign84610_e129467_d_n6;
        locals.var_t9_dn7 = assign84610_e129467_d_n7;
        locals.var_t9_dn8 = assign84610_e129467_d_n8;
        locals.var_t9_dn9 = assign84610_e129467_d_n9;
        locals.var_t9_dn10 = assign84610_e129467_d_n10;
        locals.var_t9_dn13 = assign84610_e129467_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84620_e129473, assign84620_e129473_d_n0, assign84620_e129473_d_n2, assign84620_e129473_d_n4, assign84620_e129473_d_n5, assign84620_e129473_d_n6, assign84620_e129473_d_n7, assign84620_e129473_d_n8, assign84620_e129473_d_n9, assign84620_e129473_d_n10, assign84620_e129473_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign84620_e129473;
        locals.var_ddriftldc_dn0 = assign84620_e129473_d_n0;
        locals.var_ddriftldc_dn2 = assign84620_e129473_d_n2;
        locals.var_ddriftldc_dn4 = assign84620_e129473_d_n4;
        locals.var_ddriftldc_dn5 = assign84620_e129473_d_n5;
        locals.var_ddriftldc_dn6 = assign84620_e129473_d_n6;
        locals.var_ddriftldc_dn7 = assign84620_e129473_d_n7;
        locals.var_ddriftldc_dn8 = assign84620_e129473_d_n8;
        locals.var_ddriftldc_dn9 = assign84620_e129473_d_n9;
        locals.var_ddriftldc_dn10 = assign84620_e129473_d_n10;
        locals.var_ddriftldc_dn13 = assign84620_e129473_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign84630_e129487, assign84630_e129487_d_n0, assign84630_e129487_d_n2, assign84630_e129487_d_n4, assign84630_e129487_d_n5, assign84630_e129487_d_n6, assign84630_e129487_d_n7, assign84630_e129487_d_n8, assign84630_e129487_d_n9, assign84630_e129487_d_n10, assign84630_e129487_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84630_e129479: f64 = (locals.var_q_nsubld__blk1883 * locals.var_ddriftldc);
        let assign84630_e129481: f64 = (assign84630_e129479 * locals.var_ddriftldc);
        let assign84630_e129483: f64 = (assign84630_e129481 / 2.0);
        let assign84630_e129485: f64 = (assign84630_e129483 / 1.034943e-10);
        (assign84630_e129485, (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign84630_e129487;
        locals.var_dphi_sb_dn0 = assign84630_e129487_d_n0;
        locals.var_dphi_sb_dn2 = assign84630_e129487_d_n2;
        locals.var_dphi_sb_dn4 = assign84630_e129487_d_n4;
        locals.var_dphi_sb_dn5 = assign84630_e129487_d_n5;
        locals.var_dphi_sb_dn6 = assign84630_e129487_d_n6;
        locals.var_dphi_sb_dn7 = assign84630_e129487_d_n7;
        locals.var_dphi_sb_dn8 = assign84630_e129487_d_n8;
        locals.var_dphi_sb_dn9 = assign84630_e129487_d_n9;
        locals.var_dphi_sb_dn10 = assign84630_e129487_d_n10;
        locals.var_dphi_sb_dn13 = assign84630_e129487_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign84640_e129498, assign84640_e129498_d_n0, assign84640_e129498_d_n2, assign84640_e129498_d_n4, assign84640_e129498_d_n5, assign84640_e129498_d_n6, assign84640_e129498_d_n7, assign84640_e129498_d_n8, assign84640_e129498_d_n9, assign84640_e129498_d_n10, assign84640_e129498_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84640_e129493: f64 = (2.0 * locals.var_beta);
        let assign84640_e129495: f64 = (assign84640_e129493 * locals.var_dphi_sb);
        let assign84640_e129496: f64 = (assign84640_e129495).sqrt();
        (assign84640_e129496, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn0)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn2)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn4)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn5)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn6)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn7)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn8)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn9)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn10)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn13)) / (2.0 * assign84640_e129496)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84640_e129498;
        locals.var_t0_dn0 = assign84640_e129498_d_n0;
        locals.var_t0_dn2 = assign84640_e129498_d_n2;
        locals.var_t0_dn4 = assign84640_e129498_d_n4;
        locals.var_t0_dn5 = assign84640_e129498_d_n5;
        locals.var_t0_dn6 = assign84640_e129498_d_n6;
        locals.var_t0_dn7 = assign84640_e129498_d_n7;
        locals.var_t0_dn8 = assign84640_e129498_d_n8;
        locals.var_t0_dn9 = assign84640_e129498_d_n9;
        locals.var_t0_dn10 = assign84640_e129498_d_n10;
        locals.var_t0_dn13 = assign84640_e129498_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign84650_e129511, assign84650_e129511_d_n0, assign84650_e129511_d_n2, assign84650_e129511_d_n4, assign84650_e129511_d_n5, assign84650_e129511_d_n6, assign84650_e129511_d_n7, assign84650_e129511_d_n8, assign84650_e129511_d_n9, assign84650_e129511_d_n10, assign84650_e129511_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84650_e129503: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign84650_e129505: f64 = (-locals.var_t0);
        let assign84650_e129506: f64 = { let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign84650_e129507: f64 = (assign84650_e129503 + assign84650_e129506);
        let assign84650_e129509: f64 = (assign84650_e129507 / 2.0);
        (assign84650_e129509, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign84650_e129511;
        locals.var_t1_dn0 = assign84650_e129511_d_n0;
        locals.var_t1_dn2 = assign84650_e129511_d_n2;
        locals.var_t1_dn4 = assign84650_e129511_d_n4;
        locals.var_t1_dn5 = assign84650_e129511_d_n5;
        locals.var_t1_dn6 = assign84650_e129511_d_n6;
        locals.var_t1_dn7 = assign84650_e129511_d_n7;
        locals.var_t1_dn8 = assign84650_e129511_d_n8;
        locals.var_t1_dn9 = assign84650_e129511_d_n9;
        locals.var_t1_dn10 = assign84650_e129511_d_n10;
        locals.var_t1_dn13 = assign84650_e129511_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign84660_e129520, assign84660_e129520_d_n0, assign84660_e129520_d_n2, assign84660_e129520_d_n4, assign84660_e129520_d_n5, assign84660_e129520_d_n6, assign84660_e129520_d_n7, assign84660_e129520_d_n8, assign84660_e129520_d_n9, assign84660_e129520_d_n10, assign84660_e129520_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84660_e129516: f64 = (locals.var_t1).ln();
        let assign84660_e129518: f64 = (assign84660_e129516 / locals.var_dphi_sb);
        (assign84660_e129518, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign84660_e129520;
        locals.var_c_sb_dn0 = assign84660_e129520_d_n0;
        locals.var_c_sb_dn2 = assign84660_e129520_d_n2;
        locals.var_c_sb_dn4 = assign84660_e129520_d_n4;
        locals.var_c_sb_dn5 = assign84660_e129520_d_n5;
        locals.var_c_sb_dn6 = assign84660_e129520_d_n6;
        locals.var_c_sb_dn7 = assign84660_e129520_d_n7;
        locals.var_c_sb_dn8 = assign84660_e129520_d_n8;
        locals.var_c_sb_dn9 = assign84660_e129520_d_n9;
        locals.var_c_sb_dn10 = assign84660_e129520_d_n10;
        locals.var_c_sb_dn13 = assign84660_e129520_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign84670_e129526,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign84670_e129526;
        locals.var_lp_s0_rv = 0.0;

    }
}

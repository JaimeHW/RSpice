#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13750_e8087, assign13750_e8087_d_n0, assign13750_e8087_d_n2, assign13750_e8087_d_n4, assign13750_e8087_d_n5, assign13750_e8087_d_n6, assign13750_e8087_d_n7, assign13750_e8087_d_n8, assign13750_e8087_d_n9, assign13750_e8087_d_n10, assign13750_e8087_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard304 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign13750_e8087;
        locals.var_ninvdehres_dn0 = assign13750_e8087_d_n0;
        locals.var_ninvdehres_dn2 = assign13750_e8087_d_n2;
        locals.var_ninvdehres_dn4 = assign13750_e8087_d_n4;
        locals.var_ninvdehres_dn5 = assign13750_e8087_d_n5;
        locals.var_ninvdehres_dn6 = assign13750_e8087_d_n6;
        locals.var_ninvdehres_dn7 = assign13750_e8087_d_n7;
        locals.var_ninvdehres_dn8 = assign13750_e8087_d_n8;
        locals.var_ninvdehres_dn9 = assign13750_e8087_d_n9;
        locals.var_ninvdehres_dn10 = assign13750_e8087_d_n10;
        locals.var_ninvdehres_dn13 = assign13750_e8087_d_n13;
        locals.var_ninvdehres_rv = 0.0;

        let (assign13760_e8103, assign13760_e8103_d_n0, assign13760_e8103_d_n2, assign13760_e8103_d_n4, assign13760_e8103_d_n5, assign13760_e8103_d_n6, assign13760_e8103_d_n7, assign13760_e8103_d_n8, assign13760_e8103_d_n9, assign13760_e8103_d_n10, assign13760_e8103_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (p.p53 != 0.0)) {
        let assign13760_e8094: f64 = (p.p328 * locals.var_tdiff0);
        let assign13760_e8095: f64 = (locals.var_uc_rth0 + assign13760_e8094);
        let assign13760_e8098: f64 = (p.p329 * locals.var_tdiff0_2);
        let assign13760_e8099: f64 = (assign13760_e8095 + assign13760_e8098);
        let assign13760_e8101: f64 = (assign13760_e8099 * locals.var_rthtemp0);
        (assign13760_e8101, (((p.p328 * locals.var_tdiff0_dn0) + (p.p329 * locals.var_tdiff0_2_dn0)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn2) + (p.p329 * locals.var_tdiff0_2_dn2)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn4) + (p.p329 * locals.var_tdiff0_2_dn4)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn5) + (p.p329 * locals.var_tdiff0_2_dn5)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn6) + (p.p329 * locals.var_tdiff0_2_dn6)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn7) + (p.p329 * locals.var_tdiff0_2_dn7)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn8) + (p.p329 * locals.var_tdiff0_2_dn8)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn9) + (p.p329 * locals.var_tdiff0_2_dn9)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn10) + (p.p329 * locals.var_tdiff0_2_dn10)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn13) + (p.p329 * locals.var_tdiff0_2_dn13)) * locals.var_rthtemp0),)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn13,)
    }
};
        locals.var_rth = assign13760_e8103;
        locals.var_rth_dn0 = assign13760_e8103_d_n0;
        locals.var_rth_dn2 = assign13760_e8103_d_n2;
        locals.var_rth_dn4 = assign13760_e8103_d_n4;
        locals.var_rth_dn5 = assign13760_e8103_d_n5;
        locals.var_rth_dn6 = assign13760_e8103_d_n6;
        locals.var_rth_dn7 = assign13760_e8103_d_n7;
        locals.var_rth_dn8 = assign13760_e8103_d_n8;
        locals.var_rth_dn9 = assign13760_e8103_d_n9;
        locals.var_rth_dn10 = assign13760_e8103_d_n10;
        locals.var_rth_dn13 = assign13760_e8103_d_n13;
        locals.var_rth_rv = 0.0;

        let assign13780_e8111: f64 = if locals.var_rth < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard306 = assign13780_e8111;
        locals.var_guard306_rv = 0.0;

        let (assign13790_e8119, assign13790_e8119_d_n0, assign13790_e8119_d_n2, assign13790_e8119_d_n4, assign13790_e8119_d_n5, assign13790_e8119_d_n6, assign13790_e8119_d_n7, assign13790_e8119_d_n8, assign13790_e8119_d_n9, assign13790_e8119_d_n10, assign13790_e8119_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (p.p53 != 0.0)) && (locals.var_guard306 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn13,)
    }
};
        locals.var_rth = assign13790_e8119;
        locals.var_rth_dn0 = assign13790_e8119_d_n0;
        locals.var_rth_dn2 = assign13790_e8119_d_n2;
        locals.var_rth_dn4 = assign13790_e8119_d_n4;
        locals.var_rth_dn5 = assign13790_e8119_d_n5;
        locals.var_rth_dn6 = assign13790_e8119_d_n6;
        locals.var_rth_dn7 = assign13790_e8119_d_n7;
        locals.var_rth_dn8 = assign13790_e8119_d_n8;
        locals.var_rth_dn9 = assign13790_e8119_d_n9;
        locals.var_rth_dn10 = assign13790_e8119_d_n10;
        locals.var_rth_dn13 = assign13790_e8119_d_n13;
        locals.var_rth_rv = 0.0;

        let (assign13800_e8131, assign13800_e8131_d_n0, assign13800_e8131_d_n2, assign13800_e8131_d_n4, assign13800_e8131_d_n5, assign13800_e8131_d_n6, assign13800_e8131_d_n7, assign13800_e8131_d_n8, assign13800_e8131_d_n9, assign13800_e8131_d_n10, assign13800_e8131_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13800_e8124: f64 = (p.p330 * locals.var_tdiff0);
        let assign13800_e8125: f64 = (locals.var_uc_powrat + assign13800_e8124);
        let assign13800_e8128: f64 = (p.p331 * locals.var_tdiff0_2);
        let assign13800_e8129: f64 = (assign13800_e8125 + assign13800_e8128);
        (assign13800_e8129, ((p.p330 * locals.var_tdiff0_dn0) + (p.p331 * locals.var_tdiff0_2_dn0)), ((p.p330 * locals.var_tdiff0_dn2) + (p.p331 * locals.var_tdiff0_2_dn2)), ((p.p330 * locals.var_tdiff0_dn4) + (p.p331 * locals.var_tdiff0_2_dn4)), ((p.p330 * locals.var_tdiff0_dn5) + (p.p331 * locals.var_tdiff0_2_dn5)), ((p.p330 * locals.var_tdiff0_dn6) + (p.p331 * locals.var_tdiff0_2_dn6)), ((p.p330 * locals.var_tdiff0_dn7) + (p.p331 * locals.var_tdiff0_2_dn7)), ((p.p330 * locals.var_tdiff0_dn8) + (p.p331 * locals.var_tdiff0_2_dn8)), ((p.p330 * locals.var_tdiff0_dn9) + (p.p331 * locals.var_tdiff0_2_dn9)), ((p.p330 * locals.var_tdiff0_dn10) + (p.p331 * locals.var_tdiff0_2_dn10)), ((p.p330 * locals.var_tdiff0_dn13) + (p.p331 * locals.var_tdiff0_2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign13800_e8131;
        locals.var_t2_dn0 = assign13800_e8131_d_n0;
        locals.var_t2_dn2 = assign13800_e8131_d_n2;
        locals.var_t2_dn4 = assign13800_e8131_d_n4;
        locals.var_t2_dn5 = assign13800_e8131_d_n5;
        locals.var_t2_dn6 = assign13800_e8131_d_n6;
        locals.var_t2_dn7 = assign13800_e8131_d_n7;
        locals.var_t2_dn8 = assign13800_e8131_d_n8;
        locals.var_t2_dn9 = assign13800_e8131_d_n9;
        locals.var_t2_dn10 = assign13800_e8131_d_n10;
        locals.var_t2_dn13 = assign13800_e8131_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign13810_e8139, assign13810_e8139_d_n0, assign13810_e8139_d_n2, assign13810_e8139_d_n4, assign13810_e8139_d_n5, assign13810_e8139_d_n6, assign13810_e8139_d_n7, assign13810_e8139_d_n8, assign13810_e8139_d_n9, assign13810_e8139_d_n10, assign13810_e8139_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13810_e8135: f64 = locals.var_t2;
        let assign13810_e8137: f64 = (assign13810_e8135 - 0.05);
        (assign13810_e8137, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign13810_e8139;
        locals.var_tmf1_dn0 = assign13810_e8139_d_n0;
        locals.var_tmf1_dn2 = assign13810_e8139_d_n2;
        locals.var_tmf1_dn4 = assign13810_e8139_d_n4;
        locals.var_tmf1_dn5 = assign13810_e8139_d_n5;
        locals.var_tmf1_dn6 = assign13810_e8139_d_n6;
        locals.var_tmf1_dn7 = assign13810_e8139_d_n7;
        locals.var_tmf1_dn8 = assign13810_e8139_d_n8;
        locals.var_tmf1_dn9 = assign13810_e8139_d_n9;
        locals.var_tmf1_dn10 = assign13810_e8139_d_n10;
        locals.var_tmf1_dn13 = assign13810_e8139_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign13820_e8147, assign13820_e8147_d_n0, assign13820_e8147_d_n2, assign13820_e8147_d_n4, assign13820_e8147_d_n5, assign13820_e8147_d_n6, assign13820_e8147_d_n7, assign13820_e8147_d_n8, assign13820_e8147_d_n9, assign13820_e8147_d_n10, assign13820_e8147_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13820_e8147;
        locals.var_tmf2_dn0 = assign13820_e8147_d_n0;
        locals.var_tmf2_dn2 = assign13820_e8147_d_n2;
        locals.var_tmf2_dn4 = assign13820_e8147_d_n4;
        locals.var_tmf2_dn5 = assign13820_e8147_d_n5;
        locals.var_tmf2_dn6 = assign13820_e8147_d_n6;
        locals.var_tmf2_dn7 = assign13820_e8147_d_n7;
        locals.var_tmf2_dn8 = assign13820_e8147_d_n8;
        locals.var_tmf2_dn9 = assign13820_e8147_d_n9;
        locals.var_tmf2_dn10 = assign13820_e8147_d_n10;
        locals.var_tmf2_dn13 = assign13820_e8147_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign13830_e8157, assign13830_e8157_d_n0, assign13830_e8157_d_n2, assign13830_e8157_d_n4, assign13830_e8157_d_n5, assign13830_e8157_d_n6, assign13830_e8157_d_n7, assign13830_e8157_d_n8, assign13830_e8157_d_n9, assign13830_e8157_d_n10, assign13830_e8157_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let (assign13830_e8155, assign13830_e8155_d_n0, assign13830_e8155_d_n2, assign13830_e8155_d_n4, assign13830_e8155_d_n5, assign13830_e8155_d_n6, assign13830_e8155_d_n7, assign13830_e8155_d_n8, assign13830_e8155_d_n9, assign13830_e8155_d_n10, assign13830_e8155_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign13830_e8154: f64 = (-locals.var_tmf2);
                (assign13830_e8154, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign13830_e8155, assign13830_e8155_d_n0, assign13830_e8155_d_n2, assign13830_e8155_d_n4, assign13830_e8155_d_n5, assign13830_e8155_d_n6, assign13830_e8155_d_n7, assign13830_e8155_d_n8, assign13830_e8155_d_n9, assign13830_e8155_d_n10, assign13830_e8155_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13830_e8157;
        locals.var_tmf2_dn0 = assign13830_e8157_d_n0;
        locals.var_tmf2_dn2 = assign13830_e8157_d_n2;
        locals.var_tmf2_dn4 = assign13830_e8157_d_n4;
        locals.var_tmf2_dn5 = assign13830_e8157_d_n5;
        locals.var_tmf2_dn6 = assign13830_e8157_d_n6;
        locals.var_tmf2_dn7 = assign13830_e8157_d_n7;
        locals.var_tmf2_dn8 = assign13830_e8157_d_n8;
        locals.var_tmf2_dn9 = assign13830_e8157_d_n9;
        locals.var_tmf2_dn10 = assign13830_e8157_d_n10;
        locals.var_tmf2_dn13 = assign13830_e8157_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign13840_e8166, assign13840_e8166_d_n0, assign13840_e8166_d_n2, assign13840_e8166_d_n4, assign13840_e8166_d_n5, assign13840_e8166_d_n6, assign13840_e8166_d_n7, assign13840_e8166_d_n8, assign13840_e8166_d_n9, assign13840_e8166_d_n10, assign13840_e8166_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13840_e8161: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13840_e8163: f64 = (assign13840_e8161 + locals.var_tmf2);
        let assign13840_e8164: f64 = (assign13840_e8163).sqrt();
        (assign13840_e8164, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign13840_e8164)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13840_e8166;
        locals.var_tmf2_dn0 = assign13840_e8166_d_n0;
        locals.var_tmf2_dn2 = assign13840_e8166_d_n2;
        locals.var_tmf2_dn4 = assign13840_e8166_d_n4;
        locals.var_tmf2_dn5 = assign13840_e8166_d_n5;
        locals.var_tmf2_dn6 = assign13840_e8166_d_n6;
        locals.var_tmf2_dn7 = assign13840_e8166_d_n7;
        locals.var_tmf2_dn8 = assign13840_e8166_d_n8;
        locals.var_tmf2_dn9 = assign13840_e8166_d_n9;
        locals.var_tmf2_dn10 = assign13840_e8166_d_n10;
        locals.var_tmf2_dn13 = assign13840_e8166_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign13850_e8176, assign13850_e8176_d_n0, assign13850_e8176_d_n2, assign13850_e8176_d_n4, assign13850_e8176_d_n5, assign13850_e8176_d_n6, assign13850_e8176_d_n7, assign13850_e8176_d_n8, assign13850_e8176_d_n9, assign13850_e8176_d_n10, assign13850_e8176_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13850_e8172: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign13850_e8173: f64 = (1.0 + assign13850_e8172);
        let assign13850_e8174: f64 = (0.5 * assign13850_e8173);
        (assign13850_e8174, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign13850_e8176;
        locals.var_t0_dn0 = assign13850_e8176_d_n0;
        locals.var_t0_dn2 = assign13850_e8176_d_n2;
        locals.var_t0_dn4 = assign13850_e8176_d_n4;
        locals.var_t0_dn5 = assign13850_e8176_d_n5;
        locals.var_t0_dn6 = assign13850_e8176_d_n6;
        locals.var_t0_dn7 = assign13850_e8176_d_n7;
        locals.var_t0_dn8 = assign13850_e8176_d_n8;
        locals.var_t0_dn9 = assign13850_e8176_d_n9;
        locals.var_t0_dn10 = assign13850_e8176_d_n10;
        locals.var_t0_dn13 = assign13850_e8176_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign13860_e8186, assign13860_e8186_d_n0, assign13860_e8186_d_n2, assign13860_e8186_d_n4, assign13860_e8186_d_n5, assign13860_e8186_d_n6, assign13860_e8186_d_n7, assign13860_e8186_d_n8, assign13860_e8186_d_n9, assign13860_e8186_d_n10, assign13860_e8186_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13860_e8182: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13860_e8183: f64 = (0.5 * assign13860_e8182);
        let assign13860_e8184: f64 = assign13860_e8183;
        (assign13860_e8184, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign13860_e8186;
        locals.var_t2_dn0 = assign13860_e8186_d_n0;
        locals.var_t2_dn2 = assign13860_e8186_d_n2;
        locals.var_t2_dn4 = assign13860_e8186_d_n4;
        locals.var_t2_dn5 = assign13860_e8186_d_n5;
        locals.var_t2_dn6 = assign13860_e8186_d_n6;
        locals.var_t2_dn7 = assign13860_e8186_d_n7;
        locals.var_t2_dn8 = assign13860_e8186_d_n8;
        locals.var_t2_dn9 = assign13860_e8186_d_n9;
        locals.var_t2_dn10 = assign13860_e8186_d_n10;
        locals.var_t2_dn13 = assign13860_e8186_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign13870_e8194, assign13870_e8194_d_n0, assign13870_e8194_d_n2, assign13870_e8194_d_n4, assign13870_e8194_d_n5, assign13870_e8194_d_n6, assign13870_e8194_d_n7, assign13870_e8194_d_n8, assign13870_e8194_d_n9, assign13870_e8194_d_n10, assign13870_e8194_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13870_e8190: f64 = (1.0 - locals.var_t2);
        let assign13870_e8192: f64 = (assign13870_e8190 - 0.05);
        (assign13870_e8192, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign13870_e8194;
        locals.var_tmf1_dn0 = assign13870_e8194_d_n0;
        locals.var_tmf1_dn2 = assign13870_e8194_d_n2;
        locals.var_tmf1_dn4 = assign13870_e8194_d_n4;
        locals.var_tmf1_dn5 = assign13870_e8194_d_n5;
        locals.var_tmf1_dn6 = assign13870_e8194_d_n6;
        locals.var_tmf1_dn7 = assign13870_e8194_d_n7;
        locals.var_tmf1_dn8 = assign13870_e8194_d_n8;
        locals.var_tmf1_dn9 = assign13870_e8194_d_n9;
        locals.var_tmf1_dn10 = assign13870_e8194_d_n10;
        locals.var_tmf1_dn13 = assign13870_e8194_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign13880_e8202, assign13880_e8202_d_n0, assign13880_e8202_d_n2, assign13880_e8202_d_n4, assign13880_e8202_d_n5, assign13880_e8202_d_n6, assign13880_e8202_d_n7, assign13880_e8202_d_n8, assign13880_e8202_d_n9, assign13880_e8202_d_n10, assign13880_e8202_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13880_e8198: f64 = 4.0;
        let assign13880_e8200: f64 = (assign13880_e8198 * 0.05);
        (assign13880_e8200, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13880_e8202;
        locals.var_tmf2_dn0 = assign13880_e8202_d_n0;
        locals.var_tmf2_dn2 = assign13880_e8202_d_n2;
        locals.var_tmf2_dn4 = assign13880_e8202_d_n4;
        locals.var_tmf2_dn5 = assign13880_e8202_d_n5;
        locals.var_tmf2_dn6 = assign13880_e8202_d_n6;
        locals.var_tmf2_dn7 = assign13880_e8202_d_n7;
        locals.var_tmf2_dn8 = assign13880_e8202_d_n8;
        locals.var_tmf2_dn9 = assign13880_e8202_d_n9;
        locals.var_tmf2_dn10 = assign13880_e8202_d_n10;
        locals.var_tmf2_dn13 = assign13880_e8202_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign13890_e8212, assign13890_e8212_d_n0, assign13890_e8212_d_n2, assign13890_e8212_d_n4, assign13890_e8212_d_n5, assign13890_e8212_d_n6, assign13890_e8212_d_n7, assign13890_e8212_d_n8, assign13890_e8212_d_n9, assign13890_e8212_d_n10, assign13890_e8212_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let (assign13890_e8210, assign13890_e8210_d_n0, assign13890_e8210_d_n2, assign13890_e8210_d_n4, assign13890_e8210_d_n5, assign13890_e8210_d_n6, assign13890_e8210_d_n7, assign13890_e8210_d_n8, assign13890_e8210_d_n9, assign13890_e8210_d_n10, assign13890_e8210_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign13890_e8209: f64 = (-locals.var_tmf2);
                (assign13890_e8209, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign13890_e8210, assign13890_e8210_d_n0, assign13890_e8210_d_n2, assign13890_e8210_d_n4, assign13890_e8210_d_n5, assign13890_e8210_d_n6, assign13890_e8210_d_n7, assign13890_e8210_d_n8, assign13890_e8210_d_n9, assign13890_e8210_d_n10, assign13890_e8210_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13890_e8212;
        locals.var_tmf2_dn0 = assign13890_e8212_d_n0;
        locals.var_tmf2_dn2 = assign13890_e8212_d_n2;
        locals.var_tmf2_dn4 = assign13890_e8212_d_n4;
        locals.var_tmf2_dn5 = assign13890_e8212_d_n5;
        locals.var_tmf2_dn6 = assign13890_e8212_d_n6;
        locals.var_tmf2_dn7 = assign13890_e8212_d_n7;
        locals.var_tmf2_dn8 = assign13890_e8212_d_n8;
        locals.var_tmf2_dn9 = assign13890_e8212_d_n9;
        locals.var_tmf2_dn10 = assign13890_e8212_d_n10;
        locals.var_tmf2_dn13 = assign13890_e8212_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign13900_e8221, assign13900_e8221_d_n0, assign13900_e8221_d_n2, assign13900_e8221_d_n4, assign13900_e8221_d_n5, assign13900_e8221_d_n6, assign13900_e8221_d_n7, assign13900_e8221_d_n8, assign13900_e8221_d_n9, assign13900_e8221_d_n10, assign13900_e8221_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13900_e8216: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13900_e8218: f64 = (assign13900_e8216 + locals.var_tmf2);
        let assign13900_e8219: f64 = (assign13900_e8218).sqrt();
        (assign13900_e8219, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign13900_e8219)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13900_e8221;
        locals.var_tmf2_dn0 = assign13900_e8221_d_n0;
        locals.var_tmf2_dn2 = assign13900_e8221_d_n2;
        locals.var_tmf2_dn4 = assign13900_e8221_d_n4;
        locals.var_tmf2_dn5 = assign13900_e8221_d_n5;
        locals.var_tmf2_dn6 = assign13900_e8221_d_n6;
        locals.var_tmf2_dn7 = assign13900_e8221_d_n7;
        locals.var_tmf2_dn8 = assign13900_e8221_d_n8;
        locals.var_tmf2_dn9 = assign13900_e8221_d_n9;
        locals.var_tmf2_dn10 = assign13900_e8221_d_n10;
        locals.var_tmf2_dn13 = assign13900_e8221_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign13910_e8231, assign13910_e8231_d_n0, assign13910_e8231_d_n2, assign13910_e8231_d_n4, assign13910_e8231_d_n5, assign13910_e8231_d_n6, assign13910_e8231_d_n7, assign13910_e8231_d_n8, assign13910_e8231_d_n9, assign13910_e8231_d_n10, assign13910_e8231_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13910_e8227: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign13910_e8228: f64 = (1.0 + assign13910_e8227);
        let assign13910_e8229: f64 = (0.5 * assign13910_e8228);
        (assign13910_e8229, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign13910_e8231;
        locals.var_t0_dn0 = assign13910_e8231_d_n0;
        locals.var_t0_dn2 = assign13910_e8231_d_n2;
        locals.var_t0_dn4 = assign13910_e8231_d_n4;
        locals.var_t0_dn5 = assign13910_e8231_d_n5;
        locals.var_t0_dn6 = assign13910_e8231_d_n6;
        locals.var_t0_dn7 = assign13910_e8231_d_n7;
        locals.var_t0_dn8 = assign13910_e8231_d_n8;
        locals.var_t0_dn9 = assign13910_e8231_d_n9;
        locals.var_t0_dn10 = assign13910_e8231_d_n10;
        locals.var_t0_dn13 = assign13910_e8231_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign13920_e8241, assign13920_e8241_d_n0, assign13920_e8241_d_n2, assign13920_e8241_d_n4, assign13920_e8241_d_n5, assign13920_e8241_d_n6, assign13920_e8241_d_n7, assign13920_e8241_d_n8, assign13920_e8241_d_n9, assign13920_e8241_d_n10, assign13920_e8241_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13920_e8237: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13920_e8238: f64 = (0.5 * assign13920_e8237);
        let assign13920_e8239: f64 = (1.0 - assign13920_e8238);
        (assign13920_e8239, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_powratio, locals.var_powratio_dn0, locals.var_powratio_dn2, locals.var_powratio_dn4, locals.var_powratio_dn5, locals.var_powratio_dn6, locals.var_powratio_dn7, locals.var_powratio_dn8, locals.var_powratio_dn9, locals.var_powratio_dn10, locals.var_powratio_dn13,)
    }
};
        locals.var_powratio = assign13920_e8241;
        locals.var_powratio_dn0 = assign13920_e8241_d_n0;
        locals.var_powratio_dn2 = assign13920_e8241_d_n2;
        locals.var_powratio_dn4 = assign13920_e8241_d_n4;
        locals.var_powratio_dn5 = assign13920_e8241_d_n5;
        locals.var_powratio_dn6 = assign13920_e8241_d_n6;
        locals.var_powratio_dn7 = assign13920_e8241_d_n7;
        locals.var_powratio_dn8 = assign13920_e8241_d_n8;
        locals.var_powratio_dn9 = assign13920_e8241_d_n9;
        locals.var_powratio_dn10 = assign13920_e8241_d_n10;
        locals.var_powratio_dn13 = assign13920_e8241_d_n13;
        locals.var_powratio_rv = 0.0;

        let (assign13930_e8252, assign13930_e8252_d_n0, assign13930_e8252_d_n2, assign13930_e8252_d_n4, assign13930_e8252_d_n5, assign13930_e8252_d_n6, assign13930_e8252_d_n7, assign13930_e8252_d_n8, assign13930_e8252_d_n9, assign13930_e8252_d_n10, assign13930_e8252_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13930_e8245: f64 = (2.0 * locals.var_beta_inv);
        let assign13930_e8248: f64 = (locals.var_nsub / locals.var_nin);
        let assign13930_e8249: f64 = (assign13930_e8248).ln();
        let assign13930_e8250: f64 = (assign13930_e8245 * assign13930_e8249);
        (assign13930_e8250, (((2.0 * locals.var_beta_inv_dn0) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn2) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn4) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn4 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn5) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn5 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn6) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn7) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn8) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn8 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn9) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn9 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn10) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn13) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn13 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn4, locals.var_pb2_dn5, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn8, locals.var_pb2_dn9, locals.var_pb2_dn10, locals.var_pb2_dn13,)
    }
};
        locals.var_pb2 = assign13930_e8252;
        locals.var_pb2_dn0 = assign13930_e8252_d_n0;
        locals.var_pb2_dn2 = assign13930_e8252_d_n2;
        locals.var_pb2_dn4 = assign13930_e8252_d_n4;
        locals.var_pb2_dn5 = assign13930_e8252_d_n5;
        locals.var_pb2_dn6 = assign13930_e8252_d_n6;
        locals.var_pb2_dn7 = assign13930_e8252_d_n7;
        locals.var_pb2_dn8 = assign13930_e8252_d_n8;
        locals.var_pb2_dn9 = assign13930_e8252_d_n9;
        locals.var_pb2_dn10 = assign13930_e8252_d_n10;
        locals.var_pb2_dn13 = assign13930_e8252_d_n13;
        locals.var_pb2_rv = 0.0;

        let (assign13940_e8260, assign13940_e8260_d_n0, assign13940_e8260_d_n2, assign13940_e8260_d_n4, assign13940_e8260_d_n5, assign13940_e8260_d_n6, assign13940_e8260_d_n7, assign13940_e8260_d_n8, assign13940_e8260_d_n9, assign13940_e8260_d_n10, assign13940_e8260_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13940_e8256: f64 = (2.0 * 1.034943e-10);
        let assign13940_e8258: f64 = (assign13940_e8256 / 1.6021918e-19);
        (assign13940_e8258, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13940_e8260;
        locals.var_t1_dn0 = assign13940_e8260_d_n0;
        locals.var_t1_dn2 = assign13940_e8260_d_n2;
        locals.var_t1_dn4 = assign13940_e8260_d_n4;
        locals.var_t1_dn5 = assign13940_e8260_d_n5;
        locals.var_t1_dn6 = assign13940_e8260_d_n6;
        locals.var_t1_dn7 = assign13940_e8260_d_n7;
        locals.var_t1_dn8 = assign13940_e8260_d_n8;
        locals.var_t1_dn9 = assign13940_e8260_d_n9;
        locals.var_t1_dn10 = assign13940_e8260_d_n10;
        locals.var_t1_dn13 = assign13940_e8260_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign13950_e8267, assign13950_e8267_d_n0, assign13950_e8267_d_n2, assign13950_e8267_d_n4, assign13950_e8267_d_n5, assign13950_e8267_d_n6, assign13950_e8267_d_n7, assign13950_e8267_d_n8, assign13950_e8267_d_n9, assign13950_e8267_d_n10, assign13950_e8267_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13950_e8264: f64 = (locals.var_t1 / locals.var_nsub);
        let assign13950_e8265: f64 = (assign13950_e8264).sqrt();
        (assign13950_e8265, ((((locals.var_t1_dn0 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn2 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn4 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn5 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn6 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn7 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn8 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn9 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn10 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn13 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn13)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)),)
    } else {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn13,)
    }
};
        locals.var_wdpl = assign13950_e8267;
        locals.var_wdpl_dn0 = assign13950_e8267_d_n0;
        locals.var_wdpl_dn2 = assign13950_e8267_d_n2;
        locals.var_wdpl_dn4 = assign13950_e8267_d_n4;
        locals.var_wdpl_dn5 = assign13950_e8267_d_n5;
        locals.var_wdpl_dn6 = assign13950_e8267_d_n6;
        locals.var_wdpl_dn7 = assign13950_e8267_d_n7;
        locals.var_wdpl_dn8 = assign13950_e8267_d_n8;
        locals.var_wdpl_dn9 = assign13950_e8267_d_n9;
        locals.var_wdpl_dn10 = assign13950_e8267_d_n10;
        locals.var_wdpl_dn13 = assign13950_e8267_d_n13;
        locals.var_wdpl_rv = 0.0;

        let (assign13960_e8274, assign13960_e8274_d_n0, assign13960_e8274_d_n2, assign13960_e8274_d_n4, assign13960_e8274_d_n5, assign13960_e8274_d_n6, assign13960_e8274_d_n7, assign13960_e8274_d_n8, assign13960_e8274_d_n9, assign13960_e8274_d_n10, assign13960_e8274_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13960_e8271: f64 = (locals.var_t1 / locals.var_ef_nsubp);
        let assign13960_e8272: f64 = (assign13960_e8271).sqrt();
        (assign13960_e8272, ((((locals.var_t1_dn0 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn0)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn2 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn2)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn4 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn4)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn5 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn5)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn6 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn6)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn7 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn7)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn8 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn8)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn9 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn9)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn10 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn10)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn13 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn13)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)),)
    } else {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn13,)
    }
};
        locals.var_wdplp = assign13960_e8274;
        locals.var_wdplp_dn0 = assign13960_e8274_d_n0;
        locals.var_wdplp_dn2 = assign13960_e8274_d_n2;
        locals.var_wdplp_dn4 = assign13960_e8274_d_n4;
        locals.var_wdplp_dn5 = assign13960_e8274_d_n5;
        locals.var_wdplp_dn6 = assign13960_e8274_d_n6;
        locals.var_wdplp_dn7 = assign13960_e8274_d_n7;
        locals.var_wdplp_dn8 = assign13960_e8274_d_n8;
        locals.var_wdplp_dn9 = assign13960_e8274_d_n9;
        locals.var_wdplp_dn10 = assign13960_e8274_d_n10;
        locals.var_wdplp_dn13 = assign13960_e8274_d_n13;
        locals.var_wdplp_rv = 0.0;

        let assign13970_e8277: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard307 = assign13970_e8277;
        locals.var_guard307_rv = 0.0;

        let (assign13980_e8292, assign13980_e8292_d_n0, assign13980_e8292_d_n2, assign13980_e8292_d_n4, assign13980_e8292_d_n5, assign13980_e8292_d_n6, assign13980_e8292_d_n7, assign13980_e8292_d_n8, assign13980_e8292_d_n9, assign13980_e8292_d_n10, assign13980_e8292_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard307 != 0.0)) {
        let assign13980_e8283: f64 = (2.0 * 1.034943e-10);
        let assign13980_e8285: f64 = (assign13980_e8283 * 1.6021918e-19);
        let assign13980_e8287: f64 = (assign13980_e8285 * locals.var_nsub);
        let assign13980_e8289: f64 = (assign13980_e8287 * locals.var_beta_inv);
        let assign13980_e8290: f64 = (assign13980_e8289).sqrt();
        (assign13980_e8290, ((((assign13980_e8285 * locals.var_nsub_dn0) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn0)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn2) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn2)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn4) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn4)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn5) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn5)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn6) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn6)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn7) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn7)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn8) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn8)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn9) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn9)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn10) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn10)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn13) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn13)) / (2.0 * assign13980_e8290)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign13980_e8292;
        locals.var_cnst0_dn0 = assign13980_e8292_d_n0;
        locals.var_cnst0_dn2 = assign13980_e8292_d_n2;
        locals.var_cnst0_dn4 = assign13980_e8292_d_n4;
        locals.var_cnst0_dn5 = assign13980_e8292_d_n5;
        locals.var_cnst0_dn6 = assign13980_e8292_d_n6;
        locals.var_cnst0_dn7 = assign13980_e8292_d_n7;
        locals.var_cnst0_dn8 = assign13980_e8292_d_n8;
        locals.var_cnst0_dn9 = assign13980_e8292_d_n9;
        locals.var_cnst0_dn10 = assign13980_e8292_d_n10;
        locals.var_cnst0_dn13 = assign13980_e8292_d_n13;
        locals.var_cnst0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13990_e8300, assign13990_e8300_d_n0, assign13990_e8300_d_n2, assign13990_e8300_d_n4, assign13990_e8300_d_n5, assign13990_e8300_d_n6, assign13990_e8300_d_n7, assign13990_e8300_d_n8, assign13990_e8300_d_n9, assign13990_e8300_d_n10, assign13990_e8300_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard307 != 0.0)) {
        let assign13990_e8298: f64 = (locals.var_nin / locals.var_nsub);
        (assign13990_e8298, (((locals.var_nin_dn0 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn2 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn4 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn5 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn6 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn7 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn8 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn9 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn10 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn13 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn13)) / (locals.var_nsub * locals.var_nsub)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13990_e8300;
        locals.var_t1_dn0 = assign13990_e8300_d_n0;
        locals.var_t1_dn2 = assign13990_e8300_d_n2;
        locals.var_t1_dn4 = assign13990_e8300_d_n4;
        locals.var_t1_dn5 = assign13990_e8300_d_n5;
        locals.var_t1_dn6 = assign13990_e8300_d_n6;
        locals.var_t1_dn7 = assign13990_e8300_d_n7;
        locals.var_t1_dn8 = assign13990_e8300_d_n8;
        locals.var_t1_dn9 = assign13990_e8300_d_n9;
        locals.var_t1_dn10 = assign13990_e8300_d_n10;
        locals.var_t1_dn13 = assign13990_e8300_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign14000_e8308, assign14000_e8308_d_n0, assign14000_e8308_d_n2, assign14000_e8308_d_n4, assign14000_e8308_d_n5, assign14000_e8308_d_n6, assign14000_e8308_d_n7, assign14000_e8308_d_n8, assign14000_e8308_d_n9, assign14000_e8308_d_n10, assign14000_e8308_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard307 != 0.0)) {
        let assign14000_e8306: f64 = (locals.var_t1 * locals.var_t1);
        (assign14000_e8306, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign14000_e8308;
        locals.var_cnst1_dn0 = assign14000_e8308_d_n0;
        locals.var_cnst1_dn2 = assign14000_e8308_d_n2;
        locals.var_cnst1_dn4 = assign14000_e8308_d_n4;
        locals.var_cnst1_dn5 = assign14000_e8308_d_n5;
        locals.var_cnst1_dn6 = assign14000_e8308_d_n6;
        locals.var_cnst1_dn7 = assign14000_e8308_d_n7;
        locals.var_cnst1_dn8 = assign14000_e8308_d_n8;
        locals.var_cnst1_dn9 = assign14000_e8308_d_n9;
        locals.var_cnst1_dn10 = assign14000_e8308_d_n10;
        locals.var_cnst1_dn13 = assign14000_e8308_d_n13;
        locals.var_cnst1_rv = 0.0;

        let assign14010_e8311: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign14010_e8311;
        locals.var_guard308_rv = 0.0;

        let assign14020_e8314: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard309 = assign14020_e8314;
        locals.var_guard309_rv = 0.0;

        let (assign14030_e8327, assign14030_e8327_d_n0, assign14030_e8327_d_n2, assign14030_e8327_d_n4, assign14030_e8327_d_n5, assign14030_e8327_d_n6, assign14030_e8327_d_n7, assign14030_e8327_d_n8, assign14030_e8327_d_n9, assign14030_e8327_d_n10, assign14030_e8327_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard308 != 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign14030_e8323: f64 = (locals.var_uc_nover / locals.var_nsub);
        let assign14030_e8324: f64 = (assign14030_e8323).sqrt();
        let assign14030_e8325: f64 = (locals.var_cnst0 * assign14030_e8324);
        (assign14030_e8325, ((locals.var_cnst0_dn0 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn2 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn4 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn5 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn6 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn7 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn8 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn9 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn10 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn13 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn13) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    }
};
        locals.var_cnst0over = assign14030_e8327;
        locals.var_cnst0over_dn0 = assign14030_e8327_d_n0;
        locals.var_cnst0over_dn2 = assign14030_e8327_d_n2;
        locals.var_cnst0over_dn4 = assign14030_e8327_d_n4;
        locals.var_cnst0over_dn5 = assign14030_e8327_d_n5;
        locals.var_cnst0over_dn6 = assign14030_e8327_d_n6;
        locals.var_cnst0over_dn7 = assign14030_e8327_d_n7;
        locals.var_cnst0over_dn8 = assign14030_e8327_d_n8;
        locals.var_cnst0over_dn9 = assign14030_e8327_d_n9;
        locals.var_cnst0over_dn10 = assign14030_e8327_d_n10;
        locals.var_cnst0over_dn13 = assign14030_e8327_d_n13;
        locals.var_cnst0over_rv = 0.0;

        let assign14040_e8330: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard310 = assign14040_e8330;
        locals.var_guard310_rv = 0.0;

        let (assign14050_e8343, assign14050_e8343_d_n0, assign14050_e8343_d_n2, assign14050_e8343_d_n4, assign14050_e8343_d_n5, assign14050_e8343_d_n6, assign14050_e8343_d_n7, assign14050_e8343_d_n8, assign14050_e8343_d_n9, assign14050_e8343_d_n10, assign14050_e8343_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard308 != 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign14050_e8339: f64 = (locals.var_uc_novers / locals.var_nsub);
        let assign14050_e8340: f64 = (assign14050_e8339).sqrt();
        let assign14050_e8341: f64 = (locals.var_cnst0 * assign14050_e8340);
        (assign14050_e8341, ((locals.var_cnst0_dn0 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn2 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn4 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn5 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn6 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn7 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn8 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn9 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn10 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn13 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn13) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    }
};
        locals.var_cnst0overs = assign14050_e8343;
        locals.var_cnst0overs_dn0 = assign14050_e8343_d_n0;
        locals.var_cnst0overs_dn2 = assign14050_e8343_d_n2;
        locals.var_cnst0overs_dn4 = assign14050_e8343_d_n4;
        locals.var_cnst0overs_dn5 = assign14050_e8343_d_n5;
        locals.var_cnst0overs_dn6 = assign14050_e8343_d_n6;
        locals.var_cnst0overs_dn7 = assign14050_e8343_d_n7;
        locals.var_cnst0overs_dn8 = assign14050_e8343_d_n8;
        locals.var_cnst0overs_dn9 = assign14050_e8343_d_n9;
        locals.var_cnst0overs_dn10 = assign14050_e8343_d_n10;
        locals.var_cnst0overs_dn13 = assign14050_e8343_d_n13;
        locals.var_cnst0overs_rv = 0.0;

        let assign14060_e8346: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign14060_e8346;
        locals.var_guard311_rv = 0.0;

        let (assign14070_e8360, assign14070_e8360_d_n0, assign14070_e8360_d_n2, assign14070_e8360_d_n4, assign14070_e8360_d_n5, assign14070_e8360_d_n6, assign14070_e8360_d_n7, assign14070_e8360_d_n8, assign14070_e8360_d_n9, assign14070_e8360_d_n10, assign14070_e8360_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard308 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign14070_e8356: f64 = (locals.var_uc_nover / locals.var_uc_ndepm);
        let assign14070_e8357: f64 = (assign14070_e8356).sqrt();
        let assign14070_e8358: f64 = (locals.var_cnst0 * assign14070_e8357);
        (assign14070_e8358, ((locals.var_cnst0_dn0 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn2 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn4 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn5 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn6 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn7 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn8 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn9 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn10 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn13 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn13) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    }
};
        locals.var_cnst0over = assign14070_e8360;
        locals.var_cnst0over_dn0 = assign14070_e8360_d_n0;
        locals.var_cnst0over_dn2 = assign14070_e8360_d_n2;
        locals.var_cnst0over_dn4 = assign14070_e8360_d_n4;
        locals.var_cnst0over_dn5 = assign14070_e8360_d_n5;
        locals.var_cnst0over_dn6 = assign14070_e8360_d_n6;
        locals.var_cnst0over_dn7 = assign14070_e8360_d_n7;
        locals.var_cnst0over_dn8 = assign14070_e8360_d_n8;
        locals.var_cnst0over_dn9 = assign14070_e8360_d_n9;
        locals.var_cnst0over_dn10 = assign14070_e8360_d_n10;
        locals.var_cnst0over_dn13 = assign14070_e8360_d_n13;
        locals.var_cnst0over_rv = 0.0;

        let assign14080_e8363: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign14080_e8363;
        locals.var_guard312_rv = 0.0;

        let (assign14090_e8377, assign14090_e8377_d_n0, assign14090_e8377_d_n2, assign14090_e8377_d_n4, assign14090_e8377_d_n5, assign14090_e8377_d_n6, assign14090_e8377_d_n7, assign14090_e8377_d_n8, assign14090_e8377_d_n9, assign14090_e8377_d_n10, assign14090_e8377_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard308 == 0.0)) && (locals.var_guard312 != 0.0)) {
        let assign14090_e8373: f64 = (locals.var_uc_novers / locals.var_uc_ndepm);
        let assign14090_e8374: f64 = (assign14090_e8373).sqrt();
        let assign14090_e8375: f64 = (locals.var_cnst0 * assign14090_e8374);
        (assign14090_e8375, ((locals.var_cnst0_dn0 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn2 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn4 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn5 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn6 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn7 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn8 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn9 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn10 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn13 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn13) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    }
};
        locals.var_cnst0overs = assign14090_e8377;
        locals.var_cnst0overs_dn0 = assign14090_e8377_d_n0;
        locals.var_cnst0overs_dn2 = assign14090_e8377_d_n2;
        locals.var_cnst0overs_dn4 = assign14090_e8377_d_n4;
        locals.var_cnst0overs_dn5 = assign14090_e8377_d_n5;
        locals.var_cnst0overs_dn6 = assign14090_e8377_d_n6;
        locals.var_cnst0overs_dn7 = assign14090_e8377_d_n7;
        locals.var_cnst0overs_dn8 = assign14090_e8377_d_n8;
        locals.var_cnst0overs_dn9 = assign14090_e8377_d_n9;
        locals.var_cnst0overs_dn10 = assign14090_e8377_d_n10;
        locals.var_cnst0overs_dn13 = assign14090_e8377_d_n13;
        locals.var_cnst0overs_rv = 0.0;

        let assign14100_e8380: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard313 = assign14100_e8380;
        locals.var_guard313_rv = 0.0;

        let assign14110_e8383: f64 = if locals.var_uc_rd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign14110_e8383;
        locals.var_guard314_rv = 0.0;

        let (assign14120_e8407, assign14120_e8407_d_n0, assign14120_e8407_d_n2, assign14120_e8407_d_n4, assign14120_e8407_d_n5, assign14120_e8407_d_n6, assign14120_e8407_d_n7, assign14120_e8407_d_n8, assign14120_e8407_d_n9, assign14120_e8407_d_n10, assign14120_e8407_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign14120_e8392: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign14120_e8394: f64 = (assign14120_e8392 * 1000000.0);
        let assign14120_e8396: f64 = (assign14120_e8394 + locals.var_uc_rdict1);
        let assign14120_e8397: f64 = (locals.var_rdtemp0 * assign14120_e8396);
        let assign14120_e8400: f64 = (p.p68 * p.p100);
        let assign14120_e8402: f64 = (assign14120_e8400 * 1000000.0);
        let assign14120_e8404: f64 = (assign14120_e8402 + p.p101);
        let assign14120_e8405: f64 = (assign14120_e8397 * assign14120_e8404);
        (assign14120_e8405, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign14120_e8407;
        locals.var_t2_dn0 = assign14120_e8407_d_n0;
        locals.var_t2_dn2 = assign14120_e8407_d_n2;
        locals.var_t2_dn4 = assign14120_e8407_d_n4;
        locals.var_t2_dn5 = assign14120_e8407_d_n5;
        locals.var_t2_dn6 = assign14120_e8407_d_n6;
        locals.var_t2_dn7 = assign14120_e8407_d_n7;
        locals.var_t2_dn8 = assign14120_e8407_d_n8;
        locals.var_t2_dn9 = assign14120_e8407_d_n9;
        locals.var_t2_dn10 = assign14120_e8407_d_n10;
        locals.var_t2_dn13 = assign14120_e8407_d_n13;
        locals.var_t2_rv = 0.0;

        let assign14130_e8410: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign14130_e8410;
        locals.var_guard315_rv = 0.0;

        let (assign14140_e8430, assign14140_e8430_d_n0, assign14140_e8430_d_n2, assign14140_e8430_d_n4, assign14140_e8430_d_n5, assign14140_e8430_d_n6, assign14140_e8430_d_n7, assign14140_e8430_d_n8, assign14140_e8430_d_n9, assign14140_e8430_d_n10, assign14140_e8430_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14140_e8421: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign14140_e8422: f64 = (locals.var_uc_rd + assign14140_e8421);
        let assign14140_e8425: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign14140_e8426: f64 = (assign14140_e8422 + assign14140_e8425);
        let assign14140_e8428: f64 = (assign14140_e8426 * locals.var_t2);
        (assign14140_e8428, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign14140_e8430;
        locals.var_rde_dn0 = assign14140_e8430_d_n0;
        locals.var_rde_dn2 = assign14140_e8430_d_n2;
        locals.var_rde_dn4 = assign14140_e8430_d_n4;
        locals.var_rde_dn5 = assign14140_e8430_d_n5;
        locals.var_rde_dn6 = assign14140_e8430_d_n6;
        locals.var_rde_dn7 = assign14140_e8430_d_n7;
        locals.var_rde_dn8 = assign14140_e8430_d_n8;
        locals.var_rde_dn9 = assign14140_e8430_d_n9;
        locals.var_rde_dn10 = assign14140_e8430_d_n10;
        locals.var_rde_dn13 = assign14140_e8430_d_n13;
        locals.var_rde_rv = 0.0;

        let (assign14150_e8448, assign14150_e8448_d_n0, assign14150_e8448_d_n2, assign14150_e8448_d_n4, assign14150_e8448_d_n5, assign14150_e8448_d_n6, assign14150_e8448_d_n7, assign14150_e8448_d_n8, assign14150_e8448_d_n9, assign14150_e8448_d_n10, assign14150_e8448_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14150_e8441: f64 = (0.005 * locals.var_uc_rd);
        let assign14150_e8442: f64 = (locals.var_rde - assign14150_e8441);
        let assign14150_e8445: f64 = (0.01 * locals.var_uc_rd);
        let assign14150_e8446: f64 = (assign14150_e8442 - assign14150_e8445);
        (assign14150_e8446, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14150_e8448;
        locals.var_tmf1_dn0 = assign14150_e8448_d_n0;
        locals.var_tmf1_dn2 = assign14150_e8448_d_n2;
        locals.var_tmf1_dn4 = assign14150_e8448_d_n4;
        locals.var_tmf1_dn5 = assign14150_e8448_d_n5;
        locals.var_tmf1_dn6 = assign14150_e8448_d_n6;
        locals.var_tmf1_dn7 = assign14150_e8448_d_n7;
        locals.var_tmf1_dn8 = assign14150_e8448_d_n8;
        locals.var_tmf1_dn9 = assign14150_e8448_d_n9;
        locals.var_tmf1_dn10 = assign14150_e8448_d_n10;
        locals.var_tmf1_dn13 = assign14150_e8448_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign14160_e8466, assign14160_e8466_d_n0, assign14160_e8466_d_n2, assign14160_e8466_d_n4, assign14160_e8466_d_n5, assign14160_e8466_d_n6, assign14160_e8466_d_n7, assign14160_e8466_d_n8, assign14160_e8466_d_n9, assign14160_e8466_d_n10, assign14160_e8466_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14160_e8459: f64 = (0.005 * locals.var_uc_rd);
        let assign14160_e8460: f64 = (4.0 * assign14160_e8459);
        let assign14160_e8463: f64 = (0.01 * locals.var_uc_rd);
        let assign14160_e8464: f64 = (assign14160_e8460 * assign14160_e8463);
        (assign14160_e8464, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14160_e8466;
        locals.var_tmf2_dn0 = assign14160_e8466_d_n0;
        locals.var_tmf2_dn2 = assign14160_e8466_d_n2;
        locals.var_tmf2_dn4 = assign14160_e8466_d_n4;
        locals.var_tmf2_dn5 = assign14160_e8466_d_n5;
        locals.var_tmf2_dn6 = assign14160_e8466_d_n6;
        locals.var_tmf2_dn7 = assign14160_e8466_d_n7;
        locals.var_tmf2_dn8 = assign14160_e8466_d_n8;
        locals.var_tmf2_dn9 = assign14160_e8466_d_n9;
        locals.var_tmf2_dn10 = assign14160_e8466_d_n10;
        locals.var_tmf2_dn13 = assign14160_e8466_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14170_e8482, assign14170_e8482_d_n0, assign14170_e8482_d_n2, assign14170_e8482_d_n4, assign14170_e8482_d_n5, assign14170_e8482_d_n6, assign14170_e8482_d_n7, assign14170_e8482_d_n8, assign14170_e8482_d_n9, assign14170_e8482_d_n10, assign14170_e8482_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let (assign14170_e8480, assign14170_e8480_d_n0, assign14170_e8480_d_n2, assign14170_e8480_d_n4, assign14170_e8480_d_n5, assign14170_e8480_d_n6, assign14170_e8480_d_n7, assign14170_e8480_d_n8, assign14170_e8480_d_n9, assign14170_e8480_d_n10, assign14170_e8480_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14170_e8479: f64 = (-locals.var_tmf2);
                (assign14170_e8479, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14170_e8480, assign14170_e8480_d_n0, assign14170_e8480_d_n2, assign14170_e8480_d_n4, assign14170_e8480_d_n5, assign14170_e8480_d_n6, assign14170_e8480_d_n7, assign14170_e8480_d_n8, assign14170_e8480_d_n9, assign14170_e8480_d_n10, assign14170_e8480_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14170_e8482;
        locals.var_tmf2_dn0 = assign14170_e8482_d_n0;
        locals.var_tmf2_dn2 = assign14170_e8482_d_n2;
        locals.var_tmf2_dn4 = assign14170_e8482_d_n4;
        locals.var_tmf2_dn5 = assign14170_e8482_d_n5;
        locals.var_tmf2_dn6 = assign14170_e8482_d_n6;
        locals.var_tmf2_dn7 = assign14170_e8482_d_n7;
        locals.var_tmf2_dn8 = assign14170_e8482_d_n8;
        locals.var_tmf2_dn9 = assign14170_e8482_d_n9;
        locals.var_tmf2_dn10 = assign14170_e8482_d_n10;
        locals.var_tmf2_dn13 = assign14170_e8482_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14180_e8497, assign14180_e8497_d_n0, assign14180_e8497_d_n2, assign14180_e8497_d_n4, assign14180_e8497_d_n5, assign14180_e8497_d_n6, assign14180_e8497_d_n7, assign14180_e8497_d_n8, assign14180_e8497_d_n9, assign14180_e8497_d_n10, assign14180_e8497_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14180_e8492: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14180_e8494: f64 = (assign14180_e8492 + locals.var_tmf2);
        let assign14180_e8495: f64 = (assign14180_e8494).sqrt();
        (assign14180_e8495, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14180_e8495)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14180_e8497;
        locals.var_tmf2_dn0 = assign14180_e8497_d_n0;
        locals.var_tmf2_dn2 = assign14180_e8497_d_n2;
        locals.var_tmf2_dn4 = assign14180_e8497_d_n4;
        locals.var_tmf2_dn5 = assign14180_e8497_d_n5;
        locals.var_tmf2_dn6 = assign14180_e8497_d_n6;
        locals.var_tmf2_dn7 = assign14180_e8497_d_n7;
        locals.var_tmf2_dn8 = assign14180_e8497_d_n8;
        locals.var_tmf2_dn9 = assign14180_e8497_d_n9;
        locals.var_tmf2_dn10 = assign14180_e8497_d_n10;
        locals.var_tmf2_dn13 = assign14180_e8497_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14190_e8513, assign14190_e8513_d_n0, assign14190_e8513_d_n2, assign14190_e8513_d_n4, assign14190_e8513_d_n5, assign14190_e8513_d_n6, assign14190_e8513_d_n7, assign14190_e8513_d_n8, assign14190_e8513_d_n9, assign14190_e8513_d_n10, assign14190_e8513_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14190_e8509: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14190_e8510: f64 = (1.0 + assign14190_e8509);
        let assign14190_e8511: f64 = (0.5 * assign14190_e8510);
        (assign14190_e8511, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14190_e8513;
        locals.var_t0_dn0 = assign14190_e8513_d_n0;
        locals.var_t0_dn2 = assign14190_e8513_d_n2;
        locals.var_t0_dn4 = assign14190_e8513_d_n4;
        locals.var_t0_dn5 = assign14190_e8513_d_n5;
        locals.var_t0_dn6 = assign14190_e8513_d_n6;
        locals.var_t0_dn7 = assign14190_e8513_d_n7;
        locals.var_t0_dn8 = assign14190_e8513_d_n8;
        locals.var_t0_dn9 = assign14190_e8513_d_n9;
        locals.var_t0_dn10 = assign14190_e8513_d_n10;
        locals.var_t0_dn13 = assign14190_e8513_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign14200_e8531, assign14200_e8531_d_n0, assign14200_e8531_d_n2, assign14200_e8531_d_n4, assign14200_e8531_d_n5, assign14200_e8531_d_n6, assign14200_e8531_d_n7, assign14200_e8531_d_n8, assign14200_e8531_d_n9, assign14200_e8531_d_n10, assign14200_e8531_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14200_e8523: f64 = (0.005 * locals.var_uc_rd);
        let assign14200_e8527: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14200_e8528: f64 = (0.5 * assign14200_e8527);
        let assign14200_e8529: f64 = (assign14200_e8523 + assign14200_e8528);
        (assign14200_e8529, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign14200_e8531;
        locals.var_rde_dn0 = assign14200_e8531_d_n0;
        locals.var_rde_dn2 = assign14200_e8531_d_n2;
        locals.var_rde_dn4 = assign14200_e8531_d_n4;
        locals.var_rde_dn5 = assign14200_e8531_d_n5;
        locals.var_rde_dn6 = assign14200_e8531_d_n6;
        locals.var_rde_dn7 = assign14200_e8531_d_n7;
        locals.var_rde_dn8 = assign14200_e8531_d_n8;
        locals.var_rde_dn9 = assign14200_e8531_d_n9;
        locals.var_rde_dn10 = assign14200_e8531_d_n10;
        locals.var_rde_dn13 = assign14200_e8531_d_n13;
        locals.var_rde_rv = 0.0;

        let (assign14210_e8552, assign14210_e8552_d_n0, assign14210_e8552_d_n2, assign14210_e8552_d_n4, assign14210_e8552_d_n5, assign14210_e8552_d_n6, assign14210_e8552_d_n7, assign14210_e8552_d_n8, assign14210_e8552_d_n9, assign14210_e8552_d_n10, assign14210_e8552_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14210_e8543: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign14210_e8544: f64 = (locals.var_uc_rd + assign14210_e8543);
        let assign14210_e8547: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign14210_e8548: f64 = (assign14210_e8544 + assign14210_e8547);
        let assign14210_e8550: f64 = (assign14210_e8548 * locals.var_t2);
        (assign14210_e8550, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign14210_e8552;
        locals.var_rde_dn0 = assign14210_e8552_d_n0;
        locals.var_rde_dn2 = assign14210_e8552_d_n2;
        locals.var_rde_dn4 = assign14210_e8552_d_n4;
        locals.var_rde_dn5 = assign14210_e8552_d_n5;
        locals.var_rde_dn6 = assign14210_e8552_d_n6;
        locals.var_rde_dn7 = assign14210_e8552_d_n7;
        locals.var_rde_dn8 = assign14210_e8552_d_n8;
        locals.var_rde_dn9 = assign14210_e8552_d_n9;
        locals.var_rde_dn10 = assign14210_e8552_d_n10;
        locals.var_rde_dn13 = assign14210_e8552_d_n13;
        locals.var_rde_rv = 0.0;

        let (assign14220_e8571, assign14220_e8571_d_n0, assign14220_e8571_d_n2, assign14220_e8571_d_n4, assign14220_e8571_d_n5, assign14220_e8571_d_n6, assign14220_e8571_d_n7, assign14220_e8571_d_n8, assign14220_e8571_d_n9, assign14220_e8571_d_n10, assign14220_e8571_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14220_e8564: f64 = (0.005 * locals.var_uc_rd);
        let assign14220_e8565: f64 = (locals.var_rde - assign14220_e8564);
        let assign14220_e8568: f64 = (0.01 * locals.var_uc_rd);
        let assign14220_e8569: f64 = (assign14220_e8565 - assign14220_e8568);
        (assign14220_e8569, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14220_e8571;
        locals.var_tmf1_dn0 = assign14220_e8571_d_n0;
        locals.var_tmf1_dn2 = assign14220_e8571_d_n2;
        locals.var_tmf1_dn4 = assign14220_e8571_d_n4;
        locals.var_tmf1_dn5 = assign14220_e8571_d_n5;
        locals.var_tmf1_dn6 = assign14220_e8571_d_n6;
        locals.var_tmf1_dn7 = assign14220_e8571_d_n7;
        locals.var_tmf1_dn8 = assign14220_e8571_d_n8;
        locals.var_tmf1_dn9 = assign14220_e8571_d_n9;
        locals.var_tmf1_dn10 = assign14220_e8571_d_n10;
        locals.var_tmf1_dn13 = assign14220_e8571_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign14230_e8590, assign14230_e8590_d_n0, assign14230_e8590_d_n2, assign14230_e8590_d_n4, assign14230_e8590_d_n5, assign14230_e8590_d_n6, assign14230_e8590_d_n7, assign14230_e8590_d_n8, assign14230_e8590_d_n9, assign14230_e8590_d_n10, assign14230_e8590_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14230_e8583: f64 = (0.005 * locals.var_uc_rd);
        let assign14230_e8584: f64 = (4.0 * assign14230_e8583);
        let assign14230_e8587: f64 = (0.01 * locals.var_uc_rd);
        let assign14230_e8588: f64 = (assign14230_e8584 * assign14230_e8587);
        (assign14230_e8588, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14230_e8590;
        locals.var_tmf2_dn0 = assign14230_e8590_d_n0;
        locals.var_tmf2_dn2 = assign14230_e8590_d_n2;
        locals.var_tmf2_dn4 = assign14230_e8590_d_n4;
        locals.var_tmf2_dn5 = assign14230_e8590_d_n5;
        locals.var_tmf2_dn6 = assign14230_e8590_d_n6;
        locals.var_tmf2_dn7 = assign14230_e8590_d_n7;
        locals.var_tmf2_dn8 = assign14230_e8590_d_n8;
        locals.var_tmf2_dn9 = assign14230_e8590_d_n9;
        locals.var_tmf2_dn10 = assign14230_e8590_d_n10;
        locals.var_tmf2_dn13 = assign14230_e8590_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14240_e8607, assign14240_e8607_d_n0, assign14240_e8607_d_n2, assign14240_e8607_d_n4, assign14240_e8607_d_n5, assign14240_e8607_d_n6, assign14240_e8607_d_n7, assign14240_e8607_d_n8, assign14240_e8607_d_n9, assign14240_e8607_d_n10, assign14240_e8607_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let (assign14240_e8605, assign14240_e8605_d_n0, assign14240_e8605_d_n2, assign14240_e8605_d_n4, assign14240_e8605_d_n5, assign14240_e8605_d_n6, assign14240_e8605_d_n7, assign14240_e8605_d_n8, assign14240_e8605_d_n9, assign14240_e8605_d_n10, assign14240_e8605_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14240_e8604: f64 = (-locals.var_tmf2);
                (assign14240_e8604, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14240_e8605, assign14240_e8605_d_n0, assign14240_e8605_d_n2, assign14240_e8605_d_n4, assign14240_e8605_d_n5, assign14240_e8605_d_n6, assign14240_e8605_d_n7, assign14240_e8605_d_n8, assign14240_e8605_d_n9, assign14240_e8605_d_n10, assign14240_e8605_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14240_e8607;
        locals.var_tmf2_dn0 = assign14240_e8607_d_n0;
        locals.var_tmf2_dn2 = assign14240_e8607_d_n2;
        locals.var_tmf2_dn4 = assign14240_e8607_d_n4;
        locals.var_tmf2_dn5 = assign14240_e8607_d_n5;
        locals.var_tmf2_dn6 = assign14240_e8607_d_n6;
        locals.var_tmf2_dn7 = assign14240_e8607_d_n7;
        locals.var_tmf2_dn8 = assign14240_e8607_d_n8;
        locals.var_tmf2_dn9 = assign14240_e8607_d_n9;
        locals.var_tmf2_dn10 = assign14240_e8607_d_n10;
        locals.var_tmf2_dn13 = assign14240_e8607_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14250_e8623, assign14250_e8623_d_n0, assign14250_e8623_d_n2, assign14250_e8623_d_n4, assign14250_e8623_d_n5, assign14250_e8623_d_n6, assign14250_e8623_d_n7, assign14250_e8623_d_n8, assign14250_e8623_d_n9, assign14250_e8623_d_n10, assign14250_e8623_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14250_e8618: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14250_e8620: f64 = (assign14250_e8618 + locals.var_tmf2);
        let assign14250_e8621: f64 = (assign14250_e8620).sqrt();
        (assign14250_e8621, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14250_e8621)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14250_e8623;
        locals.var_tmf2_dn0 = assign14250_e8623_d_n0;
        locals.var_tmf2_dn2 = assign14250_e8623_d_n2;
        locals.var_tmf2_dn4 = assign14250_e8623_d_n4;
        locals.var_tmf2_dn5 = assign14250_e8623_d_n5;
        locals.var_tmf2_dn6 = assign14250_e8623_d_n6;
        locals.var_tmf2_dn7 = assign14250_e8623_d_n7;
        locals.var_tmf2_dn8 = assign14250_e8623_d_n8;
        locals.var_tmf2_dn9 = assign14250_e8623_d_n9;
        locals.var_tmf2_dn10 = assign14250_e8623_d_n10;
        locals.var_tmf2_dn13 = assign14250_e8623_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14260_e8640, assign14260_e8640_d_n0, assign14260_e8640_d_n2, assign14260_e8640_d_n4, assign14260_e8640_d_n5, assign14260_e8640_d_n6, assign14260_e8640_d_n7, assign14260_e8640_d_n8, assign14260_e8640_d_n9, assign14260_e8640_d_n10, assign14260_e8640_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14260_e8636: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14260_e8637: f64 = (1.0 + assign14260_e8636);
        let assign14260_e8638: f64 = (0.5 * assign14260_e8637);
        (assign14260_e8638, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14260_e8640;
        locals.var_t0_dn0 = assign14260_e8640_d_n0;
        locals.var_t0_dn2 = assign14260_e8640_d_n2;
        locals.var_t0_dn4 = assign14260_e8640_d_n4;
        locals.var_t0_dn5 = assign14260_e8640_d_n5;
        locals.var_t0_dn6 = assign14260_e8640_d_n6;
        locals.var_t0_dn7 = assign14260_e8640_d_n7;
        locals.var_t0_dn8 = assign14260_e8640_d_n8;
        locals.var_t0_dn9 = assign14260_e8640_d_n9;
        locals.var_t0_dn10 = assign14260_e8640_d_n10;
        locals.var_t0_dn13 = assign14260_e8640_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14270_e8659, assign14270_e8659_d_n0, assign14270_e8659_d_n2, assign14270_e8659_d_n4, assign14270_e8659_d_n5, assign14270_e8659_d_n6, assign14270_e8659_d_n7, assign14270_e8659_d_n8, assign14270_e8659_d_n9, assign14270_e8659_d_n10, assign14270_e8659_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14270_e8651: f64 = (0.005 * locals.var_uc_rd);
        let assign14270_e8655: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14270_e8656: f64 = (0.5 * assign14270_e8655);
        let assign14270_e8657: f64 = (assign14270_e8651 + assign14270_e8656);
        (assign14270_e8657, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign14270_e8659;
        locals.var_rde_dn0 = assign14270_e8659_d_n0;
        locals.var_rde_dn2 = assign14270_e8659_d_n2;
        locals.var_rde_dn4 = assign14270_e8659_d_n4;
        locals.var_rde_dn5 = assign14270_e8659_d_n5;
        locals.var_rde_dn6 = assign14270_e8659_d_n6;
        locals.var_rde_dn7 = assign14270_e8659_d_n7;
        locals.var_rde_dn8 = assign14270_e8659_d_n8;
        locals.var_rde_dn9 = assign14270_e8659_d_n9;
        locals.var_rde_dn10 = assign14270_e8659_d_n10;
        locals.var_rde_dn13 = assign14270_e8659_d_n13;
        locals.var_rde_rv = 0.0;

        let (assign14280_e8668, assign14280_e8668_d_n0, assign14280_e8668_d_n2, assign14280_e8668_d_n4, assign14280_e8668_d_n5, assign14280_e8668_d_n6, assign14280_e8668_d_n7, assign14280_e8668_d_n8, assign14280_e8668_d_n9, assign14280_e8668_d_n10, assign14280_e8668_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign14280_e8668;
        locals.var_rde_dn0 = assign14280_e8668_d_n0;
        locals.var_rde_dn2 = assign14280_e8668_d_n2;
        locals.var_rde_dn4 = assign14280_e8668_d_n4;
        locals.var_rde_dn5 = assign14280_e8668_d_n5;
        locals.var_rde_dn6 = assign14280_e8668_d_n6;
        locals.var_rde_dn7 = assign14280_e8668_d_n7;
        locals.var_rde_dn8 = assign14280_e8668_d_n8;
        locals.var_rde_dn9 = assign14280_e8668_d_n9;
        locals.var_rde_dn10 = assign14280_e8668_d_n10;
        locals.var_rde_dn13 = assign14280_e8668_d_n13;
        locals.var_rde_rv = 0.0;

        let assign14290_e8671: f64 = if locals.var_uc_rs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard316 = assign14290_e8671;
        locals.var_guard316_rv = 0.0;

        let (assign14300_e8695, assign14300_e8695_d_n0, assign14300_e8695_d_n2, assign14300_e8695_d_n4, assign14300_e8695_d_n5, assign14300_e8695_d_n6, assign14300_e8695_d_n7, assign14300_e8695_d_n8, assign14300_e8695_d_n9, assign14300_e8695_d_n10, assign14300_e8695_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) {
        let assign14300_e8680: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign14300_e8682: f64 = (assign14300_e8680 * 1000000.0);
        let assign14300_e8684: f64 = (assign14300_e8682 + locals.var_uc_rdict1);
        let assign14300_e8685: f64 = (locals.var_rdtemp0 * assign14300_e8684);
        let assign14300_e8688: f64 = (p.p70 * p.p100);
        let assign14300_e8690: f64 = (assign14300_e8688 * 1000000.0);
        let assign14300_e8692: f64 = (assign14300_e8690 + p.p101);
        let assign14300_e8693: f64 = (assign14300_e8685 * assign14300_e8692);
        (assign14300_e8693, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign14300_e8695;
        locals.var_t2_dn0 = assign14300_e8695_d_n0;
        locals.var_t2_dn2 = assign14300_e8695_d_n2;
        locals.var_t2_dn4 = assign14300_e8695_d_n4;
        locals.var_t2_dn5 = assign14300_e8695_d_n5;
        locals.var_t2_dn6 = assign14300_e8695_d_n6;
        locals.var_t2_dn7 = assign14300_e8695_d_n7;
        locals.var_t2_dn8 = assign14300_e8695_d_n8;
        locals.var_t2_dn9 = assign14300_e8695_d_n9;
        locals.var_t2_dn10 = assign14300_e8695_d_n10;
        locals.var_t2_dn13 = assign14300_e8695_d_n13;
        locals.var_t2_rv = 0.0;

        let assign14310_e8698: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard317 = assign14310_e8698;
        locals.var_guard317_rv = 0.0;

        let (assign14320_e8718, assign14320_e8718_d_n0, assign14320_e8718_d_n2, assign14320_e8718_d_n4, assign14320_e8718_d_n5, assign14320_e8718_d_n6, assign14320_e8718_d_n7, assign14320_e8718_d_n8, assign14320_e8718_d_n9, assign14320_e8718_d_n10, assign14320_e8718_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14320_e8709: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign14320_e8710: f64 = (locals.var_uc_rs + assign14320_e8709);
        let assign14320_e8713: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign14320_e8714: f64 = (assign14320_e8710 + assign14320_e8713);
        let assign14320_e8716: f64 = (assign14320_e8714 * locals.var_t2);
        (assign14320_e8716, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign14320_e8718;
        locals.var_rse_dn0 = assign14320_e8718_d_n0;
        locals.var_rse_dn2 = assign14320_e8718_d_n2;
        locals.var_rse_dn4 = assign14320_e8718_d_n4;
        locals.var_rse_dn5 = assign14320_e8718_d_n5;
        locals.var_rse_dn6 = assign14320_e8718_d_n6;
        locals.var_rse_dn7 = assign14320_e8718_d_n7;
        locals.var_rse_dn8 = assign14320_e8718_d_n8;
        locals.var_rse_dn9 = assign14320_e8718_d_n9;
        locals.var_rse_dn10 = assign14320_e8718_d_n10;
        locals.var_rse_dn13 = assign14320_e8718_d_n13;
        locals.var_rse_rv = 0.0;

        let (assign14330_e8736, assign14330_e8736_d_n0, assign14330_e8736_d_n2, assign14330_e8736_d_n4, assign14330_e8736_d_n5, assign14330_e8736_d_n6, assign14330_e8736_d_n7, assign14330_e8736_d_n8, assign14330_e8736_d_n9, assign14330_e8736_d_n10, assign14330_e8736_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14330_e8729: f64 = (0.005 * locals.var_uc_rs);
        let assign14330_e8730: f64 = (locals.var_rse - assign14330_e8729);
        let assign14330_e8733: f64 = (0.01 * locals.var_uc_rs);
        let assign14330_e8734: f64 = (assign14330_e8730 - assign14330_e8733);
        (assign14330_e8734, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14330_e8736;
        locals.var_tmf1_dn0 = assign14330_e8736_d_n0;
        locals.var_tmf1_dn2 = assign14330_e8736_d_n2;
        locals.var_tmf1_dn4 = assign14330_e8736_d_n4;
        locals.var_tmf1_dn5 = assign14330_e8736_d_n5;
        locals.var_tmf1_dn6 = assign14330_e8736_d_n6;
        locals.var_tmf1_dn7 = assign14330_e8736_d_n7;
        locals.var_tmf1_dn8 = assign14330_e8736_d_n8;
        locals.var_tmf1_dn9 = assign14330_e8736_d_n9;
        locals.var_tmf1_dn10 = assign14330_e8736_d_n10;
        locals.var_tmf1_dn13 = assign14330_e8736_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign14340_e8754, assign14340_e8754_d_n0, assign14340_e8754_d_n2, assign14340_e8754_d_n4, assign14340_e8754_d_n5, assign14340_e8754_d_n6, assign14340_e8754_d_n7, assign14340_e8754_d_n8, assign14340_e8754_d_n9, assign14340_e8754_d_n10, assign14340_e8754_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14340_e8747: f64 = (0.005 * locals.var_uc_rs);
        let assign14340_e8748: f64 = (4.0 * assign14340_e8747);
        let assign14340_e8751: f64 = (0.01 * locals.var_uc_rs);
        let assign14340_e8752: f64 = (assign14340_e8748 * assign14340_e8751);
        (assign14340_e8752, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14340_e8754;
        locals.var_tmf2_dn0 = assign14340_e8754_d_n0;
        locals.var_tmf2_dn2 = assign14340_e8754_d_n2;
        locals.var_tmf2_dn4 = assign14340_e8754_d_n4;
        locals.var_tmf2_dn5 = assign14340_e8754_d_n5;
        locals.var_tmf2_dn6 = assign14340_e8754_d_n6;
        locals.var_tmf2_dn7 = assign14340_e8754_d_n7;
        locals.var_tmf2_dn8 = assign14340_e8754_d_n8;
        locals.var_tmf2_dn9 = assign14340_e8754_d_n9;
        locals.var_tmf2_dn10 = assign14340_e8754_d_n10;
        locals.var_tmf2_dn13 = assign14340_e8754_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14350_e8770, assign14350_e8770_d_n0, assign14350_e8770_d_n2, assign14350_e8770_d_n4, assign14350_e8770_d_n5, assign14350_e8770_d_n6, assign14350_e8770_d_n7, assign14350_e8770_d_n8, assign14350_e8770_d_n9, assign14350_e8770_d_n10, assign14350_e8770_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let (assign14350_e8768, assign14350_e8768_d_n0, assign14350_e8768_d_n2, assign14350_e8768_d_n4, assign14350_e8768_d_n5, assign14350_e8768_d_n6, assign14350_e8768_d_n7, assign14350_e8768_d_n8, assign14350_e8768_d_n9, assign14350_e8768_d_n10, assign14350_e8768_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14350_e8767: f64 = (-locals.var_tmf2);
                (assign14350_e8767, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14350_e8768, assign14350_e8768_d_n0, assign14350_e8768_d_n2, assign14350_e8768_d_n4, assign14350_e8768_d_n5, assign14350_e8768_d_n6, assign14350_e8768_d_n7, assign14350_e8768_d_n8, assign14350_e8768_d_n9, assign14350_e8768_d_n10, assign14350_e8768_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14350_e8770;
        locals.var_tmf2_dn0 = assign14350_e8770_d_n0;
        locals.var_tmf2_dn2 = assign14350_e8770_d_n2;
        locals.var_tmf2_dn4 = assign14350_e8770_d_n4;
        locals.var_tmf2_dn5 = assign14350_e8770_d_n5;
        locals.var_tmf2_dn6 = assign14350_e8770_d_n6;
        locals.var_tmf2_dn7 = assign14350_e8770_d_n7;
        locals.var_tmf2_dn8 = assign14350_e8770_d_n8;
        locals.var_tmf2_dn9 = assign14350_e8770_d_n9;
        locals.var_tmf2_dn10 = assign14350_e8770_d_n10;
        locals.var_tmf2_dn13 = assign14350_e8770_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14360_e8785, assign14360_e8785_d_n0, assign14360_e8785_d_n2, assign14360_e8785_d_n4, assign14360_e8785_d_n5, assign14360_e8785_d_n6, assign14360_e8785_d_n7, assign14360_e8785_d_n8, assign14360_e8785_d_n9, assign14360_e8785_d_n10, assign14360_e8785_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14360_e8780: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14360_e8782: f64 = (assign14360_e8780 + locals.var_tmf2);
        let assign14360_e8783: f64 = (assign14360_e8782).sqrt();
        (assign14360_e8783, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14360_e8783)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14360_e8785;
        locals.var_tmf2_dn0 = assign14360_e8785_d_n0;
        locals.var_tmf2_dn2 = assign14360_e8785_d_n2;
        locals.var_tmf2_dn4 = assign14360_e8785_d_n4;
        locals.var_tmf2_dn5 = assign14360_e8785_d_n5;
        locals.var_tmf2_dn6 = assign14360_e8785_d_n6;
        locals.var_tmf2_dn7 = assign14360_e8785_d_n7;
        locals.var_tmf2_dn8 = assign14360_e8785_d_n8;
        locals.var_tmf2_dn9 = assign14360_e8785_d_n9;
        locals.var_tmf2_dn10 = assign14360_e8785_d_n10;
        locals.var_tmf2_dn13 = assign14360_e8785_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14370_e8801, assign14370_e8801_d_n0, assign14370_e8801_d_n2, assign14370_e8801_d_n4, assign14370_e8801_d_n5, assign14370_e8801_d_n6, assign14370_e8801_d_n7, assign14370_e8801_d_n8, assign14370_e8801_d_n9, assign14370_e8801_d_n10, assign14370_e8801_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14370_e8797: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14370_e8798: f64 = (1.0 + assign14370_e8797);
        let assign14370_e8799: f64 = (0.5 * assign14370_e8798);
        (assign14370_e8799, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14370_e8801;
        locals.var_t0_dn0 = assign14370_e8801_d_n0;
        locals.var_t0_dn2 = assign14370_e8801_d_n2;
        locals.var_t0_dn4 = assign14370_e8801_d_n4;
        locals.var_t0_dn5 = assign14370_e8801_d_n5;
        locals.var_t0_dn6 = assign14370_e8801_d_n6;
        locals.var_t0_dn7 = assign14370_e8801_d_n7;
        locals.var_t0_dn8 = assign14370_e8801_d_n8;
        locals.var_t0_dn9 = assign14370_e8801_d_n9;
        locals.var_t0_dn10 = assign14370_e8801_d_n10;
        locals.var_t0_dn13 = assign14370_e8801_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign14380_e8819, assign14380_e8819_d_n0, assign14380_e8819_d_n2, assign14380_e8819_d_n4, assign14380_e8819_d_n5, assign14380_e8819_d_n6, assign14380_e8819_d_n7, assign14380_e8819_d_n8, assign14380_e8819_d_n9, assign14380_e8819_d_n10, assign14380_e8819_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14380_e8811: f64 = (0.005 * locals.var_uc_rs);
        let assign14380_e8815: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14380_e8816: f64 = (0.5 * assign14380_e8815);
        let assign14380_e8817: f64 = (assign14380_e8811 + assign14380_e8816);
        (assign14380_e8817, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign14380_e8819;
        locals.var_rse_dn0 = assign14380_e8819_d_n0;
        locals.var_rse_dn2 = assign14380_e8819_d_n2;
        locals.var_rse_dn4 = assign14380_e8819_d_n4;
        locals.var_rse_dn5 = assign14380_e8819_d_n5;
        locals.var_rse_dn6 = assign14380_e8819_d_n6;
        locals.var_rse_dn7 = assign14380_e8819_d_n7;
        locals.var_rse_dn8 = assign14380_e8819_d_n8;
        locals.var_rse_dn9 = assign14380_e8819_d_n9;
        locals.var_rse_dn10 = assign14380_e8819_d_n10;
        locals.var_rse_dn13 = assign14380_e8819_d_n13;
        locals.var_rse_rv = 0.0;

        let (assign14390_e8840, assign14390_e8840_d_n0, assign14390_e8840_d_n2, assign14390_e8840_d_n4, assign14390_e8840_d_n5, assign14390_e8840_d_n6, assign14390_e8840_d_n7, assign14390_e8840_d_n8, assign14390_e8840_d_n9, assign14390_e8840_d_n10, assign14390_e8840_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14390_e8831: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign14390_e8832: f64 = (locals.var_uc_rs + assign14390_e8831);
        let assign14390_e8835: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign14390_e8836: f64 = (assign14390_e8832 + assign14390_e8835);
        let assign14390_e8838: f64 = (assign14390_e8836 * locals.var_t2);
        (assign14390_e8838, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign14390_e8840;
        locals.var_rse_dn0 = assign14390_e8840_d_n0;
        locals.var_rse_dn2 = assign14390_e8840_d_n2;
        locals.var_rse_dn4 = assign14390_e8840_d_n4;
        locals.var_rse_dn5 = assign14390_e8840_d_n5;
        locals.var_rse_dn6 = assign14390_e8840_d_n6;
        locals.var_rse_dn7 = assign14390_e8840_d_n7;
        locals.var_rse_dn8 = assign14390_e8840_d_n8;
        locals.var_rse_dn9 = assign14390_e8840_d_n9;
        locals.var_rse_dn10 = assign14390_e8840_d_n10;
        locals.var_rse_dn13 = assign14390_e8840_d_n13;
        locals.var_rse_rv = 0.0;

        let (assign14400_e8859, assign14400_e8859_d_n0, assign14400_e8859_d_n2, assign14400_e8859_d_n4, assign14400_e8859_d_n5, assign14400_e8859_d_n6, assign14400_e8859_d_n7, assign14400_e8859_d_n8, assign14400_e8859_d_n9, assign14400_e8859_d_n10, assign14400_e8859_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14400_e8852: f64 = (0.005 * locals.var_uc_rs);
        let assign14400_e8853: f64 = (locals.var_rse - assign14400_e8852);
        let assign14400_e8856: f64 = (0.01 * locals.var_uc_rs);
        let assign14400_e8857: f64 = (assign14400_e8853 - assign14400_e8856);
        (assign14400_e8857, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14400_e8859;
        locals.var_tmf1_dn0 = assign14400_e8859_d_n0;
        locals.var_tmf1_dn2 = assign14400_e8859_d_n2;
        locals.var_tmf1_dn4 = assign14400_e8859_d_n4;
        locals.var_tmf1_dn5 = assign14400_e8859_d_n5;
        locals.var_tmf1_dn6 = assign14400_e8859_d_n6;
        locals.var_tmf1_dn7 = assign14400_e8859_d_n7;
        locals.var_tmf1_dn8 = assign14400_e8859_d_n8;
        locals.var_tmf1_dn9 = assign14400_e8859_d_n9;
        locals.var_tmf1_dn10 = assign14400_e8859_d_n10;
        locals.var_tmf1_dn13 = assign14400_e8859_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign14410_e8878, assign14410_e8878_d_n0, assign14410_e8878_d_n2, assign14410_e8878_d_n4, assign14410_e8878_d_n5, assign14410_e8878_d_n6, assign14410_e8878_d_n7, assign14410_e8878_d_n8, assign14410_e8878_d_n9, assign14410_e8878_d_n10, assign14410_e8878_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14410_e8871: f64 = (0.005 * locals.var_uc_rs);
        let assign14410_e8872: f64 = (4.0 * assign14410_e8871);
        let assign14410_e8875: f64 = (0.01 * locals.var_uc_rs);
        let assign14410_e8876: f64 = (assign14410_e8872 * assign14410_e8875);
        (assign14410_e8876, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14410_e8878;
        locals.var_tmf2_dn0 = assign14410_e8878_d_n0;
        locals.var_tmf2_dn2 = assign14410_e8878_d_n2;
        locals.var_tmf2_dn4 = assign14410_e8878_d_n4;
        locals.var_tmf2_dn5 = assign14410_e8878_d_n5;
        locals.var_tmf2_dn6 = assign14410_e8878_d_n6;
        locals.var_tmf2_dn7 = assign14410_e8878_d_n7;
        locals.var_tmf2_dn8 = assign14410_e8878_d_n8;
        locals.var_tmf2_dn9 = assign14410_e8878_d_n9;
        locals.var_tmf2_dn10 = assign14410_e8878_d_n10;
        locals.var_tmf2_dn13 = assign14410_e8878_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14420_e8895, assign14420_e8895_d_n0, assign14420_e8895_d_n2, assign14420_e8895_d_n4, assign14420_e8895_d_n5, assign14420_e8895_d_n6, assign14420_e8895_d_n7, assign14420_e8895_d_n8, assign14420_e8895_d_n9, assign14420_e8895_d_n10, assign14420_e8895_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let (assign14420_e8893, assign14420_e8893_d_n0, assign14420_e8893_d_n2, assign14420_e8893_d_n4, assign14420_e8893_d_n5, assign14420_e8893_d_n6, assign14420_e8893_d_n7, assign14420_e8893_d_n8, assign14420_e8893_d_n9, assign14420_e8893_d_n10, assign14420_e8893_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14420_e8892: f64 = (-locals.var_tmf2);
                (assign14420_e8892, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14420_e8893, assign14420_e8893_d_n0, assign14420_e8893_d_n2, assign14420_e8893_d_n4, assign14420_e8893_d_n5, assign14420_e8893_d_n6, assign14420_e8893_d_n7, assign14420_e8893_d_n8, assign14420_e8893_d_n9, assign14420_e8893_d_n10, assign14420_e8893_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14420_e8895;
        locals.var_tmf2_dn0 = assign14420_e8895_d_n0;
        locals.var_tmf2_dn2 = assign14420_e8895_d_n2;
        locals.var_tmf2_dn4 = assign14420_e8895_d_n4;
        locals.var_tmf2_dn5 = assign14420_e8895_d_n5;
        locals.var_tmf2_dn6 = assign14420_e8895_d_n6;
        locals.var_tmf2_dn7 = assign14420_e8895_d_n7;
        locals.var_tmf2_dn8 = assign14420_e8895_d_n8;
        locals.var_tmf2_dn9 = assign14420_e8895_d_n9;
        locals.var_tmf2_dn10 = assign14420_e8895_d_n10;
        locals.var_tmf2_dn13 = assign14420_e8895_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14430_e8911, assign14430_e8911_d_n0, assign14430_e8911_d_n2, assign14430_e8911_d_n4, assign14430_e8911_d_n5, assign14430_e8911_d_n6, assign14430_e8911_d_n7, assign14430_e8911_d_n8, assign14430_e8911_d_n9, assign14430_e8911_d_n10, assign14430_e8911_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14430_e8906: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14430_e8908: f64 = (assign14430_e8906 + locals.var_tmf2);
        let assign14430_e8909: f64 = (assign14430_e8908).sqrt();
        (assign14430_e8909, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14430_e8909)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14430_e8911;
        locals.var_tmf2_dn0 = assign14430_e8911_d_n0;
        locals.var_tmf2_dn2 = assign14430_e8911_d_n2;
        locals.var_tmf2_dn4 = assign14430_e8911_d_n4;
        locals.var_tmf2_dn5 = assign14430_e8911_d_n5;
        locals.var_tmf2_dn6 = assign14430_e8911_d_n6;
        locals.var_tmf2_dn7 = assign14430_e8911_d_n7;
        locals.var_tmf2_dn8 = assign14430_e8911_d_n8;
        locals.var_tmf2_dn9 = assign14430_e8911_d_n9;
        locals.var_tmf2_dn10 = assign14430_e8911_d_n10;
        locals.var_tmf2_dn13 = assign14430_e8911_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14440_e8928, assign14440_e8928_d_n0, assign14440_e8928_d_n2, assign14440_e8928_d_n4, assign14440_e8928_d_n5, assign14440_e8928_d_n6, assign14440_e8928_d_n7, assign14440_e8928_d_n8, assign14440_e8928_d_n9, assign14440_e8928_d_n10, assign14440_e8928_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14440_e8924: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14440_e8925: f64 = (1.0 + assign14440_e8924);
        let assign14440_e8926: f64 = (0.5 * assign14440_e8925);
        (assign14440_e8926, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14440_e8928;
        locals.var_t0_dn0 = assign14440_e8928_d_n0;
        locals.var_t0_dn2 = assign14440_e8928_d_n2;
        locals.var_t0_dn4 = assign14440_e8928_d_n4;
        locals.var_t0_dn5 = assign14440_e8928_d_n5;
        locals.var_t0_dn6 = assign14440_e8928_d_n6;
        locals.var_t0_dn7 = assign14440_e8928_d_n7;
        locals.var_t0_dn8 = assign14440_e8928_d_n8;
        locals.var_t0_dn9 = assign14440_e8928_d_n9;
        locals.var_t0_dn10 = assign14440_e8928_d_n10;
        locals.var_t0_dn13 = assign14440_e8928_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign14450_e8947, assign14450_e8947_d_n0, assign14450_e8947_d_n2, assign14450_e8947_d_n4, assign14450_e8947_d_n5, assign14450_e8947_d_n6, assign14450_e8947_d_n7, assign14450_e8947_d_n8, assign14450_e8947_d_n9, assign14450_e8947_d_n10, assign14450_e8947_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14450_e8939: f64 = (0.005 * locals.var_uc_rs);
        let assign14450_e8943: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14450_e8944: f64 = (0.5 * assign14450_e8943);
        let assign14450_e8945: f64 = (assign14450_e8939 + assign14450_e8944);
        (assign14450_e8945, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign14450_e8947;
        locals.var_rse_dn0 = assign14450_e8947_d_n0;
        locals.var_rse_dn2 = assign14450_e8947_d_n2;
        locals.var_rse_dn4 = assign14450_e8947_d_n4;
        locals.var_rse_dn5 = assign14450_e8947_d_n5;
        locals.var_rse_dn6 = assign14450_e8947_d_n6;
        locals.var_rse_dn7 = assign14450_e8947_d_n7;
        locals.var_rse_dn8 = assign14450_e8947_d_n8;
        locals.var_rse_dn9 = assign14450_e8947_d_n9;
        locals.var_rse_dn10 = assign14450_e8947_d_n10;
        locals.var_rse_dn13 = assign14450_e8947_d_n13;
        locals.var_rse_rv = 0.0;

        let (assign14460_e8956, assign14460_e8956_d_n0, assign14460_e8956_d_n2, assign14460_e8956_d_n4, assign14460_e8956_d_n5, assign14460_e8956_d_n6, assign14460_e8956_d_n7, assign14460_e8956_d_n8, assign14460_e8956_d_n9, assign14460_e8956_d_n10, assign14460_e8956_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign14460_e8956;
        locals.var_rse_dn0 = assign14460_e8956_d_n0;
        locals.var_rse_dn2 = assign14460_e8956_d_n2;
        locals.var_rse_dn4 = assign14460_e8956_d_n4;
        locals.var_rse_dn5 = assign14460_e8956_d_n5;
        locals.var_rse_dn6 = assign14460_e8956_d_n6;
        locals.var_rse_dn7 = assign14460_e8956_d_n7;
        locals.var_rse_dn8 = assign14460_e8956_d_n8;
        locals.var_rse_dn9 = assign14460_e8956_d_n9;
        locals.var_rse_dn10 = assign14460_e8956_d_n10;
        locals.var_rse_dn13 = assign14460_e8956_d_n13;
        locals.var_rse_rv = 0.0;

        let assign14470_e8959: f64 = if locals.var_uc_rdvd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard318 = assign14470_e8959;
        locals.var_guard318_rv = 0.0;

        let (assign14480_e8983, assign14480_e8983_d_n0, assign14480_e8983_d_n2, assign14480_e8983_d_n4, assign14480_e8983_d_n5, assign14480_e8983_d_n6, assign14480_e8983_d_n7, assign14480_e8983_d_n8, assign14480_e8983_d_n9, assign14480_e8983_d_n10, assign14480_e8983_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14480_e8968: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign14480_e8970: f64 = (assign14480_e8968 * 1000000.0);
        let assign14480_e8972: f64 = (assign14480_e8970 + locals.var_uc_rdict1);
        let assign14480_e8973: f64 = (locals.var_rdvdtemp0 * assign14480_e8972);
        let assign14480_e8976: f64 = (p.p68 * p.p100);
        let assign14480_e8978: f64 = (assign14480_e8976 * 1000000.0);
        let assign14480_e8980: f64 = (assign14480_e8978 + p.p101);
        let assign14480_e8981: f64 = (assign14480_e8973 * assign14480_e8980);
        (assign14480_e8981, ((locals.var_rdvdtemp0_dn0 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn2 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn4 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn5 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn6 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn7 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn8 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn9 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn10 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn13 * assign14480_e8972) * assign14480_e8980),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign14480_e8983;
        locals.var_t4_dn0 = assign14480_e8983_d_n0;
        locals.var_t4_dn2 = assign14480_e8983_d_n2;
        locals.var_t4_dn4 = assign14480_e8983_d_n4;
        locals.var_t4_dn5 = assign14480_e8983_d_n5;
        locals.var_t4_dn6 = assign14480_e8983_d_n6;
        locals.var_t4_dn7 = assign14480_e8983_d_n7;
        locals.var_t4_dn8 = assign14480_e8983_d_n8;
        locals.var_t4_dn9 = assign14480_e8983_d_n9;
        locals.var_t4_dn10 = assign14480_e8983_d_n10;
        locals.var_t4_dn13 = assign14480_e8983_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign14490_e8997, assign14490_e8997_d_n0, assign14490_e8997_d_n2, assign14490_e8997_d_n4, assign14490_e8997_d_n5, assign14490_e8997_d_n6, assign14490_e8997_d_n7, assign14490_e8997_d_n8, assign14490_e8997_d_n9, assign14490_e8997_d_n10, assign14490_e8997_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14490_e8991: f64 = (1.0 - locals.var_uc_rdov13);
        let assign14490_e8993: f64 = (assign14490_e8991 * p.p63);
        let assign14490_e8995: f64 = (assign14490_e8993 * 1000000.0);
        (assign14490_e8995, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign14490_e8997;
        locals.var_t1_dn0 = assign14490_e8997_d_n0;
        locals.var_t1_dn2 = assign14490_e8997_d_n2;
        locals.var_t1_dn4 = assign14490_e8997_d_n4;
        locals.var_t1_dn5 = assign14490_e8997_d_n5;
        locals.var_t1_dn6 = assign14490_e8997_d_n6;
        locals.var_t1_dn7 = assign14490_e8997_d_n7;
        locals.var_t1_dn8 = assign14490_e8997_d_n8;
        locals.var_t1_dn9 = assign14490_e8997_d_n9;
        locals.var_t1_dn10 = assign14490_e8997_d_n10;
        locals.var_t1_dn13 = assign14490_e8997_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14500_e9018, assign14500_e9018_d_n0, assign14500_e9018_d_n2, assign14500_e9018_d_n4, assign14500_e9018_d_n5, assign14500_e9018_d_n6, assign14500_e9018_d_n7, assign14500_e9018_d_n8, assign14500_e9018_d_n9, assign14500_e9018_d_n10, assign14500_e9018_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14500_e9005: f64 = (p.p99 * p.p99);
        let assign14500_e9009: f64 = (0.0001 * 0.01);
        let assign14500_e9010: f64 = (4.0 * assign14500_e9009);
        let assign14500_e9013: f64 = (0.0001 * 0.01);
        let assign14500_e9014: f64 = (assign14500_e9010 * assign14500_e9013);
        let assign14500_e9015: f64 = (assign14500_e9005 + assign14500_e9014);
        let assign14500_e9016: f64 = (assign14500_e9015).sqrt();
        (assign14500_e9016, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14500_e9018;
        locals.var_tmf2_dn0 = assign14500_e9018_d_n0;
        locals.var_tmf2_dn2 = assign14500_e9018_d_n2;
        locals.var_tmf2_dn4 = assign14500_e9018_d_n4;
        locals.var_tmf2_dn5 = assign14500_e9018_d_n5;
        locals.var_tmf2_dn6 = assign14500_e9018_d_n6;
        locals.var_tmf2_dn7 = assign14500_e9018_d_n7;
        locals.var_tmf2_dn8 = assign14500_e9018_d_n8;
        locals.var_tmf2_dn9 = assign14500_e9018_d_n9;
        locals.var_tmf2_dn10 = assign14500_e9018_d_n10;
        locals.var_tmf2_dn13 = assign14500_e9018_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14510_e9032, assign14510_e9032_d_n0, assign14510_e9032_d_n2, assign14510_e9032_d_n4, assign14510_e9032_d_n5, assign14510_e9032_d_n6, assign14510_e9032_d_n7, assign14510_e9032_d_n8, assign14510_e9032_d_n9, assign14510_e9032_d_n10, assign14510_e9032_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14510_e9028: f64 = (p.p99 / locals.var_tmf2);
        let assign14510_e9029: f64 = (1.0 + assign14510_e9028);
        let assign14510_e9030: f64 = (0.5 * assign14510_e9029);
        (assign14510_e9030, (0.5 * (-((p.p99 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14510_e9032;
        locals.var_t0_dn0 = assign14510_e9032_d_n0;
        locals.var_t0_dn2 = assign14510_e9032_d_n2;
        locals.var_t0_dn4 = assign14510_e9032_d_n4;
        locals.var_t0_dn5 = assign14510_e9032_d_n5;
        locals.var_t0_dn6 = assign14510_e9032_d_n6;
        locals.var_t0_dn7 = assign14510_e9032_d_n7;
        locals.var_t0_dn8 = assign14510_e9032_d_n8;
        locals.var_t0_dn9 = assign14510_e9032_d_n9;
        locals.var_t0_dn10 = assign14510_e9032_d_n10;
        locals.var_t0_dn13 = assign14510_e9032_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign14520_e9044, assign14520_e9044_d_n0, assign14520_e9044_d_n2, assign14520_e9044_d_n4, assign14520_e9044_d_n5, assign14520_e9044_d_n6, assign14520_e9044_d_n7, assign14520_e9044_d_n8, assign14520_e9044_d_n9, assign14520_e9044_d_n10, assign14520_e9044_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14520_e9041: f64 = (p.p99 + locals.var_tmf2);
        let assign14520_e9042: f64 = (0.5 * assign14520_e9041);
        (assign14520_e9042, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign14520_e9044;
        locals.var_t2_dn0 = assign14520_e9044_d_n0;
        locals.var_t2_dn2 = assign14520_e9044_d_n2;
        locals.var_t2_dn4 = assign14520_e9044_d_n4;
        locals.var_t2_dn5 = assign14520_e9044_d_n5;
        locals.var_t2_dn6 = assign14520_e9044_d_n6;
        locals.var_t2_dn7 = assign14520_e9044_d_n7;
        locals.var_t2_dn8 = assign14520_e9044_d_n8;
        locals.var_t2_dn9 = assign14520_e9044_d_n9;
        locals.var_t2_dn10 = assign14520_e9044_d_n10;
        locals.var_t2_dn13 = assign14520_e9044_d_n13;
        locals.var_t2_rv = 0.0;

        let assign14530_e9047: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign14530_e9047;
        locals.var_guard319_rv = 0.0;

        let (assign14540_e9057, assign14540_e9057_d_n0, assign14540_e9057_d_n2, assign14540_e9057_d_n4, assign14540_e9057_d_n5, assign14540_e9057_d_n6, assign14540_e9057_d_n7, assign14540_e9057_d_n8, assign14540_e9057_d_n9, assign14540_e9057_d_n10, assign14540_e9057_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign14540_e9057;
        locals.var_t2_dn0 = assign14540_e9057_d_n0;
        locals.var_t2_dn2 = assign14540_e9057_d_n2;
        locals.var_t2_dn4 = assign14540_e9057_d_n4;
        locals.var_t2_dn5 = assign14540_e9057_d_n5;
        locals.var_t2_dn6 = assign14540_e9057_d_n6;
        locals.var_t2_dn7 = assign14540_e9057_d_n7;
        locals.var_t2_dn8 = assign14540_e9057_d_n8;
        locals.var_t2_dn9 = assign14540_e9057_d_n9;
        locals.var_t2_dn10 = assign14540_e9057_d_n10;
        locals.var_t2_dn13 = assign14540_e9057_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign14550_e9067, assign14550_e9067_d_n0, assign14550_e9067_d_n2, assign14550_e9067_d_n4, assign14550_e9067_d_n5, assign14550_e9067_d_n6, assign14550_e9067_d_n7, assign14550_e9067_d_n8, assign14550_e9067_d_n9, assign14550_e9067_d_n10, assign14550_e9067_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14550_e9067;
        locals.var_t0_dn0 = assign14550_e9067_d_n0;
        locals.var_t0_dn2 = assign14550_e9067_d_n2;
        locals.var_t0_dn4 = assign14550_e9067_d_n4;
        locals.var_t0_dn5 = assign14550_e9067_d_n5;
        locals.var_t0_dn6 = assign14550_e9067_d_n6;
        locals.var_t0_dn7 = assign14550_e9067_d_n7;
        locals.var_t0_dn8 = assign14550_e9067_d_n8;
        locals.var_t0_dn9 = assign14550_e9067_d_n9;
        locals.var_t0_dn10 = assign14550_e9067_d_n10;
        locals.var_t0_dn13 = assign14550_e9067_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign14560_e9078, assign14560_e9078_d_n0, assign14560_e9078_d_n2, assign14560_e9078_d_n4, assign14560_e9078_d_n5, assign14560_e9078_d_n6, assign14560_e9078_d_n7, assign14560_e9078_d_n8, assign14560_e9078_d_n9, assign14560_e9078_d_n10, assign14560_e9078_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14560_e9074: f64 = (-p.p98);
        let assign14560_e9076: f64 = (assign14560_e9074 / locals.var_t2);
        (assign14560_e9076, (-((assign14560_e9074 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign14560_e9078;
        locals.var_t8_dn0 = assign14560_e9078_d_n0;
        locals.var_t8_dn2 = assign14560_e9078_d_n2;
        locals.var_t8_dn4 = assign14560_e9078_d_n4;
        locals.var_t8_dn5 = assign14560_e9078_d_n5;
        locals.var_t8_dn6 = assign14560_e9078_d_n6;
        locals.var_t8_dn7 = assign14560_e9078_d_n7;
        locals.var_t8_dn8 = assign14560_e9078_d_n8;
        locals.var_t8_dn9 = assign14560_e9078_d_n9;
        locals.var_t8_dn10 = assign14560_e9078_d_n10;
        locals.var_t8_dn13 = assign14560_e9078_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign14570_e9094, assign14570_e9094_d_n0, assign14570_e9094_d_n2, assign14570_e9094_d_n4, assign14570_e9094_d_n5, assign14570_e9094_d_n6, assign14570_e9094_d_n7, assign14570_e9094_d_n8, assign14570_e9094_d_n9, assign14570_e9094_d_n10, assign14570_e9094_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14570_e9086: f64 = (locals.var_t8 * p.p63);
        let assign14570_e9088: f64 = (assign14570_e9086 * 1000000.0);
        let assign14570_e9090: f64 = (assign14570_e9088 + 1.0);
        let assign14570_e9092: f64 = (assign14570_e9090 + p.p98);
        (assign14570_e9092, ((locals.var_t8_dn0 * p.p63) * 1000000.0), ((locals.var_t8_dn2 * p.p63) * 1000000.0), ((locals.var_t8_dn4 * p.p63) * 1000000.0), ((locals.var_t8_dn5 * p.p63) * 1000000.0), ((locals.var_t8_dn6 * p.p63) * 1000000.0), ((locals.var_t8_dn7 * p.p63) * 1000000.0), ((locals.var_t8_dn8 * p.p63) * 1000000.0), ((locals.var_t8_dn9 * p.p63) * 1000000.0), ((locals.var_t8_dn10 * p.p63) * 1000000.0), ((locals.var_t8_dn13 * p.p63) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign14570_e9094;
        locals.var_t3_dn0 = assign14570_e9094_d_n0;
        locals.var_t3_dn2 = assign14570_e9094_d_n2;
        locals.var_t3_dn4 = assign14570_e9094_d_n4;
        locals.var_t3_dn5 = assign14570_e9094_d_n5;
        locals.var_t3_dn6 = assign14570_e9094_d_n6;
        locals.var_t3_dn7 = assign14570_e9094_d_n7;
        locals.var_t3_dn8 = assign14570_e9094_d_n8;
        locals.var_t3_dn9 = assign14570_e9094_d_n9;
        locals.var_t3_dn10 = assign14570_e9094_d_n10;
        locals.var_t3_dn13 = assign14570_e9094_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign14580_e9108, assign14580_e9108_d_n0, assign14580_e9108_d_n2, assign14580_e9108_d_n4, assign14580_e9108_d_n5, assign14580_e9108_d_n6, assign14580_e9108_d_n7, assign14580_e9108_d_n8, assign14580_e9108_d_n9, assign14580_e9108_d_n10, assign14580_e9108_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14580_e9102: f64 = (locals.var_t3 * locals.var_t4);
        let assign14580_e9104: f64 = (assign14580_e9102 - locals.var_t4);
        let assign14580_e9106: f64 = (assign14580_e9104 - 0.01);
        (assign14580_e9106, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn13 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn13)) - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14580_e9108;
        locals.var_tmf1_dn0 = assign14580_e9108_d_n0;
        locals.var_tmf1_dn2 = assign14580_e9108_d_n2;
        locals.var_tmf1_dn4 = assign14580_e9108_d_n4;
        locals.var_tmf1_dn5 = assign14580_e9108_d_n5;
        locals.var_tmf1_dn6 = assign14580_e9108_d_n6;
        locals.var_tmf1_dn7 = assign14580_e9108_d_n7;
        locals.var_tmf1_dn8 = assign14580_e9108_d_n8;
        locals.var_tmf1_dn9 = assign14580_e9108_d_n9;
        locals.var_tmf1_dn10 = assign14580_e9108_d_n10;
        locals.var_tmf1_dn13 = assign14580_e9108_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign14590_e9120, assign14590_e9120_d_n0, assign14590_e9120_d_n2, assign14590_e9120_d_n4, assign14590_e9120_d_n5, assign14590_e9120_d_n6, assign14590_e9120_d_n7, assign14590_e9120_d_n8, assign14590_e9120_d_n9, assign14590_e9120_d_n10, assign14590_e9120_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14590_e9116: f64 = (4.0 * locals.var_t4);
        let assign14590_e9118: f64 = (assign14590_e9116 * 0.01);
        (assign14590_e9118, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn13) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14590_e9120;
        locals.var_tmf2_dn0 = assign14590_e9120_d_n0;
        locals.var_tmf2_dn2 = assign14590_e9120_d_n2;
        locals.var_tmf2_dn4 = assign14590_e9120_d_n4;
        locals.var_tmf2_dn5 = assign14590_e9120_d_n5;
        locals.var_tmf2_dn6 = assign14590_e9120_d_n6;
        locals.var_tmf2_dn7 = assign14590_e9120_d_n7;
        locals.var_tmf2_dn8 = assign14590_e9120_d_n8;
        locals.var_tmf2_dn9 = assign14590_e9120_d_n9;
        locals.var_tmf2_dn10 = assign14590_e9120_d_n10;
        locals.var_tmf2_dn13 = assign14590_e9120_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14600_e9134, assign14600_e9134_d_n0, assign14600_e9134_d_n2, assign14600_e9134_d_n4, assign14600_e9134_d_n5, assign14600_e9134_d_n6, assign14600_e9134_d_n7, assign14600_e9134_d_n8, assign14600_e9134_d_n9, assign14600_e9134_d_n10, assign14600_e9134_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign14600_e9132, assign14600_e9132_d_n0, assign14600_e9132_d_n2, assign14600_e9132_d_n4, assign14600_e9132_d_n5, assign14600_e9132_d_n6, assign14600_e9132_d_n7, assign14600_e9132_d_n8, assign14600_e9132_d_n9, assign14600_e9132_d_n10, assign14600_e9132_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14600_e9131: f64 = (-locals.var_tmf2);
                (assign14600_e9131, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14600_e9132, assign14600_e9132_d_n0, assign14600_e9132_d_n2, assign14600_e9132_d_n4, assign14600_e9132_d_n5, assign14600_e9132_d_n6, assign14600_e9132_d_n7, assign14600_e9132_d_n8, assign14600_e9132_d_n9, assign14600_e9132_d_n10, assign14600_e9132_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14600_e9134;
        locals.var_tmf2_dn0 = assign14600_e9134_d_n0;
        locals.var_tmf2_dn2 = assign14600_e9134_d_n2;
        locals.var_tmf2_dn4 = assign14600_e9134_d_n4;
        locals.var_tmf2_dn5 = assign14600_e9134_d_n5;
        locals.var_tmf2_dn6 = assign14600_e9134_d_n6;
        locals.var_tmf2_dn7 = assign14600_e9134_d_n7;
        locals.var_tmf2_dn8 = assign14600_e9134_d_n8;
        locals.var_tmf2_dn9 = assign14600_e9134_d_n9;
        locals.var_tmf2_dn10 = assign14600_e9134_d_n10;
        locals.var_tmf2_dn13 = assign14600_e9134_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14610_e9147, assign14610_e9147_d_n0, assign14610_e9147_d_n2, assign14610_e9147_d_n4, assign14610_e9147_d_n5, assign14610_e9147_d_n6, assign14610_e9147_d_n7, assign14610_e9147_d_n8, assign14610_e9147_d_n9, assign14610_e9147_d_n10, assign14610_e9147_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14610_e9142: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14610_e9144: f64 = (assign14610_e9142 + locals.var_tmf2);
        let assign14610_e9145: f64 = (assign14610_e9144).sqrt();
        (assign14610_e9145, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14610_e9145)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14610_e9147;
        locals.var_tmf2_dn0 = assign14610_e9147_d_n0;
        locals.var_tmf2_dn2 = assign14610_e9147_d_n2;
        locals.var_tmf2_dn4 = assign14610_e9147_d_n4;
        locals.var_tmf2_dn5 = assign14610_e9147_d_n5;
        locals.var_tmf2_dn6 = assign14610_e9147_d_n6;
        locals.var_tmf2_dn7 = assign14610_e9147_d_n7;
        locals.var_tmf2_dn8 = assign14610_e9147_d_n8;
        locals.var_tmf2_dn9 = assign14610_e9147_d_n9;
        locals.var_tmf2_dn10 = assign14610_e9147_d_n10;
        locals.var_tmf2_dn13 = assign14610_e9147_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14620_e9161, assign14620_e9161_d_n0, assign14620_e9161_d_n2, assign14620_e9161_d_n4, assign14620_e9161_d_n5, assign14620_e9161_d_n6, assign14620_e9161_d_n7, assign14620_e9161_d_n8, assign14620_e9161_d_n9, assign14620_e9161_d_n10, assign14620_e9161_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14620_e9157: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14620_e9158: f64 = (1.0 + assign14620_e9157);
        let assign14620_e9159: f64 = (0.5 * assign14620_e9158);
        (assign14620_e9159, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign14620_e9161;
        locals.var_t6_dn0 = assign14620_e9161_d_n0;
        locals.var_t6_dn2 = assign14620_e9161_d_n2;
        locals.var_t6_dn4 = assign14620_e9161_d_n4;
        locals.var_t6_dn5 = assign14620_e9161_d_n5;
        locals.var_t6_dn6 = assign14620_e9161_d_n6;
        locals.var_t6_dn7 = assign14620_e9161_d_n7;
        locals.var_t6_dn8 = assign14620_e9161_d_n8;
        locals.var_t6_dn9 = assign14620_e9161_d_n9;
        locals.var_t6_dn10 = assign14620_e9161_d_n10;
        locals.var_t6_dn13 = assign14620_e9161_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign14630_e9175, assign14630_e9175_d_n0, assign14630_e9175_d_n2, assign14630_e9175_d_n4, assign14630_e9175_d_n5, assign14630_e9175_d_n6, assign14630_e9175_d_n7, assign14630_e9175_d_n8, assign14630_e9175_d_n9, assign14630_e9175_d_n10, assign14630_e9175_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14630_e9171: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14630_e9172: f64 = (0.5 * assign14630_e9171);
        let assign14630_e9173: f64 = (locals.var_t4 + assign14630_e9172);
        (assign14630_e9173, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign14630_e9175;
        locals.var_t5_dn0 = assign14630_e9175_d_n0;
        locals.var_t5_dn2 = assign14630_e9175_d_n2;
        locals.var_t5_dn4 = assign14630_e9175_d_n4;
        locals.var_t5_dn5 = assign14630_e9175_d_n5;
        locals.var_t5_dn6 = assign14630_e9175_d_n6;
        locals.var_t5_dn7 = assign14630_e9175_d_n7;
        locals.var_t5_dn8 = assign14630_e9175_d_n8;
        locals.var_t5_dn9 = assign14630_e9175_d_n9;
        locals.var_t5_dn10 = assign14630_e9175_d_n10;
        locals.var_t5_dn13 = assign14630_e9175_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign14640_e9191, assign14640_e9191_d_n0, assign14640_e9191_d_n2, assign14640_e9191_d_n4, assign14640_e9191_d_n5, assign14640_e9191_d_n6, assign14640_e9191_d_n7, assign14640_e9191_d_n8, assign14640_e9191_d_n9, assign14640_e9191_d_n10, assign14640_e9191_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14640_e9184: f64 = (p.p98 + 1.0);
        let assign14640_e9185: f64 = (locals.var_t4 * assign14640_e9184);
        let assign14640_e9187: f64 = (assign14640_e9185 - locals.var_t5);
        let assign14640_e9189: f64 = (assign14640_e9187 - 5e-5);
        (assign14640_e9189, ((locals.var_t4_dn0 * assign14640_e9184) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign14640_e9184) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign14640_e9184) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign14640_e9184) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign14640_e9184) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign14640_e9184) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign14640_e9184) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign14640_e9184) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign14640_e9184) - locals.var_t5_dn10), ((locals.var_t4_dn13 * assign14640_e9184) - locals.var_t5_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14640_e9191;
        locals.var_tmf1_dn0 = assign14640_e9191_d_n0;
        locals.var_tmf1_dn2 = assign14640_e9191_d_n2;
        locals.var_tmf1_dn4 = assign14640_e9191_d_n4;
        locals.var_tmf1_dn5 = assign14640_e9191_d_n5;
        locals.var_tmf1_dn6 = assign14640_e9191_d_n6;
        locals.var_tmf1_dn7 = assign14640_e9191_d_n7;
        locals.var_tmf1_dn8 = assign14640_e9191_d_n8;
        locals.var_tmf1_dn9 = assign14640_e9191_d_n9;
        locals.var_tmf1_dn10 = assign14640_e9191_d_n10;
        locals.var_tmf1_dn13 = assign14640_e9191_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign14650_e9207, assign14650_e9207_d_n0, assign14650_e9207_d_n2, assign14650_e9207_d_n4, assign14650_e9207_d_n5, assign14650_e9207_d_n6, assign14650_e9207_d_n7, assign14650_e9207_d_n8, assign14650_e9207_d_n9, assign14650_e9207_d_n10, assign14650_e9207_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14650_e9201: f64 = (p.p98 + 1.0);
        let assign14650_e9202: f64 = (locals.var_t4 * assign14650_e9201);
        let assign14650_e9203: f64 = (4.0 * assign14650_e9202);
        let assign14650_e9205: f64 = (assign14650_e9203 * 5e-5);
        (assign14650_e9205, ((4.0 * (locals.var_t4_dn0 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn13 * assign14650_e9201)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14650_e9207;
        locals.var_tmf2_dn0 = assign14650_e9207_d_n0;
        locals.var_tmf2_dn2 = assign14650_e9207_d_n2;
        locals.var_tmf2_dn4 = assign14650_e9207_d_n4;
        locals.var_tmf2_dn5 = assign14650_e9207_d_n5;
        locals.var_tmf2_dn6 = assign14650_e9207_d_n6;
        locals.var_tmf2_dn7 = assign14650_e9207_d_n7;
        locals.var_tmf2_dn8 = assign14650_e9207_d_n8;
        locals.var_tmf2_dn9 = assign14650_e9207_d_n9;
        locals.var_tmf2_dn10 = assign14650_e9207_d_n10;
        locals.var_tmf2_dn13 = assign14650_e9207_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14660_e9221, assign14660_e9221_d_n0, assign14660_e9221_d_n2, assign14660_e9221_d_n4, assign14660_e9221_d_n5, assign14660_e9221_d_n6, assign14660_e9221_d_n7, assign14660_e9221_d_n8, assign14660_e9221_d_n9, assign14660_e9221_d_n10, assign14660_e9221_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign14660_e9219, assign14660_e9219_d_n0, assign14660_e9219_d_n2, assign14660_e9219_d_n4, assign14660_e9219_d_n5, assign14660_e9219_d_n6, assign14660_e9219_d_n7, assign14660_e9219_d_n8, assign14660_e9219_d_n9, assign14660_e9219_d_n10, assign14660_e9219_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14660_e9218: f64 = (-locals.var_tmf2);
                (assign14660_e9218, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14660_e9219, assign14660_e9219_d_n0, assign14660_e9219_d_n2, assign14660_e9219_d_n4, assign14660_e9219_d_n5, assign14660_e9219_d_n6, assign14660_e9219_d_n7, assign14660_e9219_d_n8, assign14660_e9219_d_n9, assign14660_e9219_d_n10, assign14660_e9219_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14660_e9221;
        locals.var_tmf2_dn0 = assign14660_e9221_d_n0;
        locals.var_tmf2_dn2 = assign14660_e9221_d_n2;
        locals.var_tmf2_dn4 = assign14660_e9221_d_n4;
        locals.var_tmf2_dn5 = assign14660_e9221_d_n5;
        locals.var_tmf2_dn6 = assign14660_e9221_d_n6;
        locals.var_tmf2_dn7 = assign14660_e9221_d_n7;
        locals.var_tmf2_dn8 = assign14660_e9221_d_n8;
        locals.var_tmf2_dn9 = assign14660_e9221_d_n9;
        locals.var_tmf2_dn10 = assign14660_e9221_d_n10;
        locals.var_tmf2_dn13 = assign14660_e9221_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14670_e9234, assign14670_e9234_d_n0, assign14670_e9234_d_n2, assign14670_e9234_d_n4, assign14670_e9234_d_n5, assign14670_e9234_d_n6, assign14670_e9234_d_n7, assign14670_e9234_d_n8, assign14670_e9234_d_n9, assign14670_e9234_d_n10, assign14670_e9234_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14670_e9229: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14670_e9231: f64 = (assign14670_e9229 + locals.var_tmf2);
        let assign14670_e9232: f64 = (assign14670_e9231).sqrt();
        (assign14670_e9232, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14670_e9232)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14670_e9234;
        locals.var_tmf2_dn0 = assign14670_e9234_d_n0;
        locals.var_tmf2_dn2 = assign14670_e9234_d_n2;
        locals.var_tmf2_dn4 = assign14670_e9234_d_n4;
        locals.var_tmf2_dn5 = assign14670_e9234_d_n5;
        locals.var_tmf2_dn6 = assign14670_e9234_d_n6;
        locals.var_tmf2_dn7 = assign14670_e9234_d_n7;
        locals.var_tmf2_dn8 = assign14670_e9234_d_n8;
        locals.var_tmf2_dn9 = assign14670_e9234_d_n9;
        locals.var_tmf2_dn10 = assign14670_e9234_d_n10;
        locals.var_tmf2_dn13 = assign14670_e9234_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14680_e9248, assign14680_e9248_d_n0, assign14680_e9248_d_n2, assign14680_e9248_d_n4, assign14680_e9248_d_n5, assign14680_e9248_d_n6, assign14680_e9248_d_n7, assign14680_e9248_d_n8, assign14680_e9248_d_n9, assign14680_e9248_d_n10, assign14680_e9248_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14680_e9244: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14680_e9245: f64 = (1.0 + assign14680_e9244);
        let assign14680_e9246: f64 = (0.5 * assign14680_e9245);
        (assign14680_e9246, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign14680_e9248;
        locals.var_t6_dn0 = assign14680_e9248_d_n0;
        locals.var_t6_dn2 = assign14680_e9248_d_n2;
        locals.var_t6_dn4 = assign14680_e9248_d_n4;
        locals.var_t6_dn5 = assign14680_e9248_d_n5;
        locals.var_t6_dn6 = assign14680_e9248_d_n6;
        locals.var_t6_dn7 = assign14680_e9248_d_n7;
        locals.var_t6_dn8 = assign14680_e9248_d_n8;
        locals.var_t6_dn9 = assign14680_e9248_d_n9;
        locals.var_t6_dn10 = assign14680_e9248_d_n10;
        locals.var_t6_dn13 = assign14680_e9248_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign14690_e9266, assign14690_e9266_d_n0, assign14690_e9266_d_n2, assign14690_e9266_d_n4, assign14690_e9266_d_n5, assign14690_e9266_d_n6, assign14690_e9266_d_n7, assign14690_e9266_d_n8, assign14690_e9266_d_n9, assign14690_e9266_d_n10, assign14690_e9266_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14690_e9257: f64 = (p.p98 + 1.0);
        let assign14690_e9258: f64 = (locals.var_t4 * assign14690_e9257);
        let assign14690_e9262: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14690_e9263: f64 = (0.5 * assign14690_e9262);
        let assign14690_e9264: f64 = (assign14690_e9258 - assign14690_e9263);
        (assign14690_e9264, ((locals.var_t4_dn0 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn13 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign14690_e9266;
        locals.var_t7_dn0 = assign14690_e9266_d_n0;
        locals.var_t7_dn2 = assign14690_e9266_d_n2;
        locals.var_t7_dn4 = assign14690_e9266_d_n4;
        locals.var_t7_dn5 = assign14690_e9266_d_n5;
        locals.var_t7_dn6 = assign14690_e9266_d_n6;
        locals.var_t7_dn7 = assign14690_e9266_d_n7;
        locals.var_t7_dn8 = assign14690_e9266_d_n8;
        locals.var_t7_dn9 = assign14690_e9266_d_n9;
        locals.var_t7_dn10 = assign14690_e9266_d_n10;
        locals.var_t7_dn13 = assign14690_e9266_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign14700_e9282, assign14700_e9282_d_n0, assign14700_e9282_d_n2, assign14700_e9282_d_n4, assign14700_e9282_d_n5, assign14700_e9282_d_n6, assign14700_e9282_d_n7, assign14700_e9282_d_n8, assign14700_e9282_d_n9, assign14700_e9282_d_n10, assign14700_e9282_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14700_e9275: f64 = (locals.var_t1 * locals.var_t4);
        let assign14700_e9276: f64 = (locals.var_t7 + assign14700_e9275);
        let assign14700_e9278: f64 = assign14700_e9276;
        let assign14700_e9280: f64 = (assign14700_e9278 - 5e-5);
        (assign14700_e9280, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn13 + ((locals.var_t1_dn13 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14700_e9282;
        locals.var_tmf1_dn0 = assign14700_e9282_d_n0;
        locals.var_tmf1_dn2 = assign14700_e9282_d_n2;
        locals.var_tmf1_dn4 = assign14700_e9282_d_n4;
        locals.var_tmf1_dn5 = assign14700_e9282_d_n5;
        locals.var_tmf1_dn6 = assign14700_e9282_d_n6;
        locals.var_tmf1_dn7 = assign14700_e9282_d_n7;
        locals.var_tmf1_dn8 = assign14700_e9282_d_n8;
        locals.var_tmf1_dn9 = assign14700_e9282_d_n9;
        locals.var_tmf1_dn10 = assign14700_e9282_d_n10;
        locals.var_tmf1_dn13 = assign14700_e9282_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign14710_e9294, assign14710_e9294_d_n0, assign14710_e9294_d_n2, assign14710_e9294_d_n4, assign14710_e9294_d_n5, assign14710_e9294_d_n6, assign14710_e9294_d_n7, assign14710_e9294_d_n8, assign14710_e9294_d_n9, assign14710_e9294_d_n10, assign14710_e9294_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14710_e9294;
        locals.var_tmf2_dn0 = assign14710_e9294_d_n0;
        locals.var_tmf2_dn2 = assign14710_e9294_d_n2;
        locals.var_tmf2_dn4 = assign14710_e9294_d_n4;
        locals.var_tmf2_dn5 = assign14710_e9294_d_n5;
        locals.var_tmf2_dn6 = assign14710_e9294_d_n6;
        locals.var_tmf2_dn7 = assign14710_e9294_d_n7;
        locals.var_tmf2_dn8 = assign14710_e9294_d_n8;
        locals.var_tmf2_dn9 = assign14710_e9294_d_n9;
        locals.var_tmf2_dn10 = assign14710_e9294_d_n10;
        locals.var_tmf2_dn13 = assign14710_e9294_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14720_e9308, assign14720_e9308_d_n0, assign14720_e9308_d_n2, assign14720_e9308_d_n4, assign14720_e9308_d_n5, assign14720_e9308_d_n6, assign14720_e9308_d_n7, assign14720_e9308_d_n8, assign14720_e9308_d_n9, assign14720_e9308_d_n10, assign14720_e9308_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign14720_e9306, assign14720_e9306_d_n0, assign14720_e9306_d_n2, assign14720_e9306_d_n4, assign14720_e9306_d_n5, assign14720_e9306_d_n6, assign14720_e9306_d_n7, assign14720_e9306_d_n8, assign14720_e9306_d_n9, assign14720_e9306_d_n10, assign14720_e9306_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14720_e9305: f64 = (-locals.var_tmf2);
                (assign14720_e9305, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14720_e9306, assign14720_e9306_d_n0, assign14720_e9306_d_n2, assign14720_e9306_d_n4, assign14720_e9306_d_n5, assign14720_e9306_d_n6, assign14720_e9306_d_n7, assign14720_e9306_d_n8, assign14720_e9306_d_n9, assign14720_e9306_d_n10, assign14720_e9306_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14720_e9308;
        locals.var_tmf2_dn0 = assign14720_e9308_d_n0;
        locals.var_tmf2_dn2 = assign14720_e9308_d_n2;
        locals.var_tmf2_dn4 = assign14720_e9308_d_n4;
        locals.var_tmf2_dn5 = assign14720_e9308_d_n5;
        locals.var_tmf2_dn6 = assign14720_e9308_d_n6;
        locals.var_tmf2_dn7 = assign14720_e9308_d_n7;
        locals.var_tmf2_dn8 = assign14720_e9308_d_n8;
        locals.var_tmf2_dn9 = assign14720_e9308_d_n9;
        locals.var_tmf2_dn10 = assign14720_e9308_d_n10;
        locals.var_tmf2_dn13 = assign14720_e9308_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14730_e9321, assign14730_e9321_d_n0, assign14730_e9321_d_n2, assign14730_e9321_d_n4, assign14730_e9321_d_n5, assign14730_e9321_d_n6, assign14730_e9321_d_n7, assign14730_e9321_d_n8, assign14730_e9321_d_n9, assign14730_e9321_d_n10, assign14730_e9321_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14730_e9316: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14730_e9318: f64 = (assign14730_e9316 + locals.var_tmf2);
        let assign14730_e9319: f64 = (assign14730_e9318).sqrt();
        (assign14730_e9319, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14730_e9319)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14730_e9321;
        locals.var_tmf2_dn0 = assign14730_e9321_d_n0;
        locals.var_tmf2_dn2 = assign14730_e9321_d_n2;
        locals.var_tmf2_dn4 = assign14730_e9321_d_n4;
        locals.var_tmf2_dn5 = assign14730_e9321_d_n5;
        locals.var_tmf2_dn6 = assign14730_e9321_d_n6;
        locals.var_tmf2_dn7 = assign14730_e9321_d_n7;
        locals.var_tmf2_dn8 = assign14730_e9321_d_n8;
        locals.var_tmf2_dn9 = assign14730_e9321_d_n9;
        locals.var_tmf2_dn10 = assign14730_e9321_d_n10;
        locals.var_tmf2_dn13 = assign14730_e9321_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14740_e9335, assign14740_e9335_d_n0, assign14740_e9335_d_n2, assign14740_e9335_d_n4, assign14740_e9335_d_n5, assign14740_e9335_d_n6, assign14740_e9335_d_n7, assign14740_e9335_d_n8, assign14740_e9335_d_n9, assign14740_e9335_d_n10, assign14740_e9335_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14740_e9331: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14740_e9332: f64 = (1.0 + assign14740_e9331);
        let assign14740_e9333: f64 = (0.5 * assign14740_e9332);
        (assign14740_e9333, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign14740_e9335;
        locals.var_t6_dn0 = assign14740_e9335_d_n0;
        locals.var_t6_dn2 = assign14740_e9335_d_n2;
        locals.var_t6_dn4 = assign14740_e9335_d_n4;
        locals.var_t6_dn5 = assign14740_e9335_d_n5;
        locals.var_t6_dn6 = assign14740_e9335_d_n6;
        locals.var_t6_dn7 = assign14740_e9335_d_n7;
        locals.var_t6_dn8 = assign14740_e9335_d_n8;
        locals.var_t6_dn9 = assign14740_e9335_d_n9;
        locals.var_t6_dn10 = assign14740_e9335_d_n10;
        locals.var_t6_dn13 = assign14740_e9335_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign14750_e9349, assign14750_e9349_d_n0, assign14750_e9349_d_n2, assign14750_e9349_d_n4, assign14750_e9349_d_n5, assign14750_e9349_d_n6, assign14750_e9349_d_n7, assign14750_e9349_d_n8, assign14750_e9349_d_n9, assign14750_e9349_d_n10, assign14750_e9349_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14750_e9345: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14750_e9346: f64 = (0.5 * assign14750_e9345);
        let assign14750_e9347: f64 = assign14750_e9346;
        (assign14750_e9347, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign14750_e9349;
        locals.var_t2_dn0 = assign14750_e9349_d_n0;
        locals.var_t2_dn2 = assign14750_e9349_d_n2;
        locals.var_t2_dn4 = assign14750_e9349_d_n4;
        locals.var_t2_dn5 = assign14750_e9349_d_n5;
        locals.var_t2_dn6 = assign14750_e9349_d_n6;
        locals.var_t2_dn7 = assign14750_e9349_d_n7;
        locals.var_t2_dn8 = assign14750_e9349_d_n8;
        locals.var_t2_dn9 = assign14750_e9349_d_n9;
        locals.var_t2_dn10 = assign14750_e9349_d_n10;
        locals.var_t2_dn13 = assign14750_e9349_d_n13;
        locals.var_t2_rv = 0.0;

        let assign14760_e9356: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard320 = assign14760_e9356;
        locals.var_guard320_rv = 0.0;

        let (assign14770_e9376, assign14770_e9376_d_n0, assign14770_e9376_d_n2, assign14770_e9376_d_n4, assign14770_e9376_d_n5, assign14770_e9376_d_n6, assign14770_e9376_d_n7, assign14770_e9376_d_n8, assign14770_e9376_d_n9, assign14770_e9376_d_n10, assign14770_e9376_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14770_e9367: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign14770_e9368: f64 = (locals.var_uc_rdvd + assign14770_e9367);
        let assign14770_e9371: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign14770_e9372: f64 = (assign14770_e9368 + assign14770_e9371);
        let assign14770_e9374: f64 = (assign14770_e9372 * locals.var_t2);
        (assign14770_e9374, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign14770_e9376;
        locals.var_rdvde_dn0 = assign14770_e9376_d_n0;
        locals.var_rdvde_dn2 = assign14770_e9376_d_n2;
        locals.var_rdvde_dn4 = assign14770_e9376_d_n4;
        locals.var_rdvde_dn5 = assign14770_e9376_d_n5;
        locals.var_rdvde_dn6 = assign14770_e9376_d_n6;
        locals.var_rdvde_dn7 = assign14770_e9376_d_n7;
        locals.var_rdvde_dn8 = assign14770_e9376_d_n8;
        locals.var_rdvde_dn9 = assign14770_e9376_d_n9;
        locals.var_rdvde_dn10 = assign14770_e9376_d_n10;
        locals.var_rdvde_dn13 = assign14770_e9376_d_n13;
        locals.var_rdvde_rv = 0.0;

        let (assign14780_e9394, assign14780_e9394_d_n0, assign14780_e9394_d_n2, assign14780_e9394_d_n4, assign14780_e9394_d_n5, assign14780_e9394_d_n6, assign14780_e9394_d_n7, assign14780_e9394_d_n8, assign14780_e9394_d_n9, assign14780_e9394_d_n10, assign14780_e9394_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14780_e9387: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14780_e9388: f64 = (locals.var_rdvde - assign14780_e9387);
        let assign14780_e9391: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14780_e9392: f64 = (assign14780_e9388 - assign14780_e9391);
        (assign14780_e9392, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14780_e9394;
        locals.var_tmf1_dn0 = assign14780_e9394_d_n0;
        locals.var_tmf1_dn2 = assign14780_e9394_d_n2;
        locals.var_tmf1_dn4 = assign14780_e9394_d_n4;
        locals.var_tmf1_dn5 = assign14780_e9394_d_n5;
        locals.var_tmf1_dn6 = assign14780_e9394_d_n6;
        locals.var_tmf1_dn7 = assign14780_e9394_d_n7;
        locals.var_tmf1_dn8 = assign14780_e9394_d_n8;
        locals.var_tmf1_dn9 = assign14780_e9394_d_n9;
        locals.var_tmf1_dn10 = assign14780_e9394_d_n10;
        locals.var_tmf1_dn13 = assign14780_e9394_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign14790_e9412, assign14790_e9412_d_n0, assign14790_e9412_d_n2, assign14790_e9412_d_n4, assign14790_e9412_d_n5, assign14790_e9412_d_n6, assign14790_e9412_d_n7, assign14790_e9412_d_n8, assign14790_e9412_d_n9, assign14790_e9412_d_n10, assign14790_e9412_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14790_e9405: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14790_e9406: f64 = (4.0 * assign14790_e9405);
        let assign14790_e9409: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14790_e9410: f64 = (assign14790_e9406 * assign14790_e9409);
        (assign14790_e9410, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14790_e9412;
        locals.var_tmf2_dn0 = assign14790_e9412_d_n0;
        locals.var_tmf2_dn2 = assign14790_e9412_d_n2;
        locals.var_tmf2_dn4 = assign14790_e9412_d_n4;
        locals.var_tmf2_dn5 = assign14790_e9412_d_n5;
        locals.var_tmf2_dn6 = assign14790_e9412_d_n6;
        locals.var_tmf2_dn7 = assign14790_e9412_d_n7;
        locals.var_tmf2_dn8 = assign14790_e9412_d_n8;
        locals.var_tmf2_dn9 = assign14790_e9412_d_n9;
        locals.var_tmf2_dn10 = assign14790_e9412_d_n10;
        locals.var_tmf2_dn13 = assign14790_e9412_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14800_e9428, assign14800_e9428_d_n0, assign14800_e9428_d_n2, assign14800_e9428_d_n4, assign14800_e9428_d_n5, assign14800_e9428_d_n6, assign14800_e9428_d_n7, assign14800_e9428_d_n8, assign14800_e9428_d_n9, assign14800_e9428_d_n10, assign14800_e9428_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign14800_e9426, assign14800_e9426_d_n0, assign14800_e9426_d_n2, assign14800_e9426_d_n4, assign14800_e9426_d_n5, assign14800_e9426_d_n6, assign14800_e9426_d_n7, assign14800_e9426_d_n8, assign14800_e9426_d_n9, assign14800_e9426_d_n10, assign14800_e9426_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14800_e9425: f64 = (-locals.var_tmf2);
                (assign14800_e9425, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14800_e9426, assign14800_e9426_d_n0, assign14800_e9426_d_n2, assign14800_e9426_d_n4, assign14800_e9426_d_n5, assign14800_e9426_d_n6, assign14800_e9426_d_n7, assign14800_e9426_d_n8, assign14800_e9426_d_n9, assign14800_e9426_d_n10, assign14800_e9426_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14800_e9428;
        locals.var_tmf2_dn0 = assign14800_e9428_d_n0;
        locals.var_tmf2_dn2 = assign14800_e9428_d_n2;
        locals.var_tmf2_dn4 = assign14800_e9428_d_n4;
        locals.var_tmf2_dn5 = assign14800_e9428_d_n5;
        locals.var_tmf2_dn6 = assign14800_e9428_d_n6;
        locals.var_tmf2_dn7 = assign14800_e9428_d_n7;
        locals.var_tmf2_dn8 = assign14800_e9428_d_n8;
        locals.var_tmf2_dn9 = assign14800_e9428_d_n9;
        locals.var_tmf2_dn10 = assign14800_e9428_d_n10;
        locals.var_tmf2_dn13 = assign14800_e9428_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14810_e9443, assign14810_e9443_d_n0, assign14810_e9443_d_n2, assign14810_e9443_d_n4, assign14810_e9443_d_n5, assign14810_e9443_d_n6, assign14810_e9443_d_n7, assign14810_e9443_d_n8, assign14810_e9443_d_n9, assign14810_e9443_d_n10, assign14810_e9443_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14810_e9438: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14810_e9440: f64 = (assign14810_e9438 + locals.var_tmf2);
        let assign14810_e9441: f64 = (assign14810_e9440).sqrt();
        (assign14810_e9441, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14810_e9441)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14810_e9443;
        locals.var_tmf2_dn0 = assign14810_e9443_d_n0;
        locals.var_tmf2_dn2 = assign14810_e9443_d_n2;
        locals.var_tmf2_dn4 = assign14810_e9443_d_n4;
        locals.var_tmf2_dn5 = assign14810_e9443_d_n5;
        locals.var_tmf2_dn6 = assign14810_e9443_d_n6;
        locals.var_tmf2_dn7 = assign14810_e9443_d_n7;
        locals.var_tmf2_dn8 = assign14810_e9443_d_n8;
        locals.var_tmf2_dn9 = assign14810_e9443_d_n9;
        locals.var_tmf2_dn10 = assign14810_e9443_d_n10;
        locals.var_tmf2_dn13 = assign14810_e9443_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14820_e9459, assign14820_e9459_d_n0, assign14820_e9459_d_n2, assign14820_e9459_d_n4, assign14820_e9459_d_n5, assign14820_e9459_d_n6, assign14820_e9459_d_n7, assign14820_e9459_d_n8, assign14820_e9459_d_n9, assign14820_e9459_d_n10, assign14820_e9459_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14820_e9455: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14820_e9456: f64 = (1.0 + assign14820_e9455);
        let assign14820_e9457: f64 = (0.5 * assign14820_e9456);
        (assign14820_e9457, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14820_e9459;
        locals.var_t0_dn0 = assign14820_e9459_d_n0;
        locals.var_t0_dn2 = assign14820_e9459_d_n2;
        locals.var_t0_dn4 = assign14820_e9459_d_n4;
        locals.var_t0_dn5 = assign14820_e9459_d_n5;
        locals.var_t0_dn6 = assign14820_e9459_d_n6;
        locals.var_t0_dn7 = assign14820_e9459_d_n7;
        locals.var_t0_dn8 = assign14820_e9459_d_n8;
        locals.var_t0_dn9 = assign14820_e9459_d_n9;
        locals.var_t0_dn10 = assign14820_e9459_d_n10;
        locals.var_t0_dn13 = assign14820_e9459_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign14830_e9477, assign14830_e9477_d_n0, assign14830_e9477_d_n2, assign14830_e9477_d_n4, assign14830_e9477_d_n5, assign14830_e9477_d_n6, assign14830_e9477_d_n7, assign14830_e9477_d_n8, assign14830_e9477_d_n9, assign14830_e9477_d_n10, assign14830_e9477_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14830_e9469: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14830_e9473: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14830_e9474: f64 = (0.5 * assign14830_e9473);
        let assign14830_e9475: f64 = (assign14830_e9469 + assign14830_e9474);
        (assign14830_e9475, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign14830_e9477;
        locals.var_rdvde_dn0 = assign14830_e9477_d_n0;
        locals.var_rdvde_dn2 = assign14830_e9477_d_n2;
        locals.var_rdvde_dn4 = assign14830_e9477_d_n4;
        locals.var_rdvde_dn5 = assign14830_e9477_d_n5;
        locals.var_rdvde_dn6 = assign14830_e9477_d_n6;
        locals.var_rdvde_dn7 = assign14830_e9477_d_n7;
        locals.var_rdvde_dn8 = assign14830_e9477_d_n8;
        locals.var_rdvde_dn9 = assign14830_e9477_d_n9;
        locals.var_rdvde_dn10 = assign14830_e9477_d_n10;
        locals.var_rdvde_dn13 = assign14830_e9477_d_n13;
        locals.var_rdvde_rv = 0.0;

        let (assign14840_e9498, assign14840_e9498_d_n0, assign14840_e9498_d_n2, assign14840_e9498_d_n4, assign14840_e9498_d_n5, assign14840_e9498_d_n6, assign14840_e9498_d_n7, assign14840_e9498_d_n8, assign14840_e9498_d_n9, assign14840_e9498_d_n10, assign14840_e9498_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14840_e9489: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign14840_e9490: f64 = (locals.var_uc_rdvd + assign14840_e9489);
        let assign14840_e9493: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign14840_e9494: f64 = (assign14840_e9490 + assign14840_e9493);
        let assign14840_e9496: f64 = (assign14840_e9494 * locals.var_t2);
        (assign14840_e9496, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign14840_e9498;
        locals.var_rdvde_dn0 = assign14840_e9498_d_n0;
        locals.var_rdvde_dn2 = assign14840_e9498_d_n2;
        locals.var_rdvde_dn4 = assign14840_e9498_d_n4;
        locals.var_rdvde_dn5 = assign14840_e9498_d_n5;
        locals.var_rdvde_dn6 = assign14840_e9498_d_n6;
        locals.var_rdvde_dn7 = assign14840_e9498_d_n7;
        locals.var_rdvde_dn8 = assign14840_e9498_d_n8;
        locals.var_rdvde_dn9 = assign14840_e9498_d_n9;
        locals.var_rdvde_dn10 = assign14840_e9498_d_n10;
        locals.var_rdvde_dn13 = assign14840_e9498_d_n13;
        locals.var_rdvde_rv = 0.0;

        let (assign14850_e9517, assign14850_e9517_d_n0, assign14850_e9517_d_n2, assign14850_e9517_d_n4, assign14850_e9517_d_n5, assign14850_e9517_d_n6, assign14850_e9517_d_n7, assign14850_e9517_d_n8, assign14850_e9517_d_n9, assign14850_e9517_d_n10, assign14850_e9517_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14850_e9510: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14850_e9511: f64 = (locals.var_rdvde - assign14850_e9510);
        let assign14850_e9514: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14850_e9515: f64 = (assign14850_e9511 - assign14850_e9514);
        (assign14850_e9515, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14850_e9517;
        locals.var_tmf1_dn0 = assign14850_e9517_d_n0;
        locals.var_tmf1_dn2 = assign14850_e9517_d_n2;
        locals.var_tmf1_dn4 = assign14850_e9517_d_n4;
        locals.var_tmf1_dn5 = assign14850_e9517_d_n5;
        locals.var_tmf1_dn6 = assign14850_e9517_d_n6;
        locals.var_tmf1_dn7 = assign14850_e9517_d_n7;
        locals.var_tmf1_dn8 = assign14850_e9517_d_n8;
        locals.var_tmf1_dn9 = assign14850_e9517_d_n9;
        locals.var_tmf1_dn10 = assign14850_e9517_d_n10;
        locals.var_tmf1_dn13 = assign14850_e9517_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign14860_e9536, assign14860_e9536_d_n0, assign14860_e9536_d_n2, assign14860_e9536_d_n4, assign14860_e9536_d_n5, assign14860_e9536_d_n6, assign14860_e9536_d_n7, assign14860_e9536_d_n8, assign14860_e9536_d_n9, assign14860_e9536_d_n10, assign14860_e9536_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14860_e9529: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14860_e9530: f64 = (4.0 * assign14860_e9529);
        let assign14860_e9533: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14860_e9534: f64 = (assign14860_e9530 * assign14860_e9533);
        (assign14860_e9534, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14860_e9536;
        locals.var_tmf2_dn0 = assign14860_e9536_d_n0;
        locals.var_tmf2_dn2 = assign14860_e9536_d_n2;
        locals.var_tmf2_dn4 = assign14860_e9536_d_n4;
        locals.var_tmf2_dn5 = assign14860_e9536_d_n5;
        locals.var_tmf2_dn6 = assign14860_e9536_d_n6;
        locals.var_tmf2_dn7 = assign14860_e9536_d_n7;
        locals.var_tmf2_dn8 = assign14860_e9536_d_n8;
        locals.var_tmf2_dn9 = assign14860_e9536_d_n9;
        locals.var_tmf2_dn10 = assign14860_e9536_d_n10;
        locals.var_tmf2_dn13 = assign14860_e9536_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14870_e9553, assign14870_e9553_d_n0, assign14870_e9553_d_n2, assign14870_e9553_d_n4, assign14870_e9553_d_n5, assign14870_e9553_d_n6, assign14870_e9553_d_n7, assign14870_e9553_d_n8, assign14870_e9553_d_n9, assign14870_e9553_d_n10, assign14870_e9553_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let (assign14870_e9551, assign14870_e9551_d_n0, assign14870_e9551_d_n2, assign14870_e9551_d_n4, assign14870_e9551_d_n5, assign14870_e9551_d_n6, assign14870_e9551_d_n7, assign14870_e9551_d_n8, assign14870_e9551_d_n9, assign14870_e9551_d_n10, assign14870_e9551_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14870_e9550: f64 = (-locals.var_tmf2);
                (assign14870_e9550, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14870_e9551, assign14870_e9551_d_n0, assign14870_e9551_d_n2, assign14870_e9551_d_n4, assign14870_e9551_d_n5, assign14870_e9551_d_n6, assign14870_e9551_d_n7, assign14870_e9551_d_n8, assign14870_e9551_d_n9, assign14870_e9551_d_n10, assign14870_e9551_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14870_e9553;
        locals.var_tmf2_dn0 = assign14870_e9553_d_n0;
        locals.var_tmf2_dn2 = assign14870_e9553_d_n2;
        locals.var_tmf2_dn4 = assign14870_e9553_d_n4;
        locals.var_tmf2_dn5 = assign14870_e9553_d_n5;
        locals.var_tmf2_dn6 = assign14870_e9553_d_n6;
        locals.var_tmf2_dn7 = assign14870_e9553_d_n7;
        locals.var_tmf2_dn8 = assign14870_e9553_d_n8;
        locals.var_tmf2_dn9 = assign14870_e9553_d_n9;
        locals.var_tmf2_dn10 = assign14870_e9553_d_n10;
        locals.var_tmf2_dn13 = assign14870_e9553_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14880_e9569, assign14880_e9569_d_n0, assign14880_e9569_d_n2, assign14880_e9569_d_n4, assign14880_e9569_d_n5, assign14880_e9569_d_n6, assign14880_e9569_d_n7, assign14880_e9569_d_n8, assign14880_e9569_d_n9, assign14880_e9569_d_n10, assign14880_e9569_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14880_e9564: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14880_e9566: f64 = (assign14880_e9564 + locals.var_tmf2);
        let assign14880_e9567: f64 = (assign14880_e9566).sqrt();
        (assign14880_e9567, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14880_e9567)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14880_e9569;
        locals.var_tmf2_dn0 = assign14880_e9569_d_n0;
        locals.var_tmf2_dn2 = assign14880_e9569_d_n2;
        locals.var_tmf2_dn4 = assign14880_e9569_d_n4;
        locals.var_tmf2_dn5 = assign14880_e9569_d_n5;
        locals.var_tmf2_dn6 = assign14880_e9569_d_n6;
        locals.var_tmf2_dn7 = assign14880_e9569_d_n7;
        locals.var_tmf2_dn8 = assign14880_e9569_d_n8;
        locals.var_tmf2_dn9 = assign14880_e9569_d_n9;
        locals.var_tmf2_dn10 = assign14880_e9569_d_n10;
        locals.var_tmf2_dn13 = assign14880_e9569_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14890_e9586, assign14890_e9586_d_n0, assign14890_e9586_d_n2, assign14890_e9586_d_n4, assign14890_e9586_d_n5, assign14890_e9586_d_n6, assign14890_e9586_d_n7, assign14890_e9586_d_n8, assign14890_e9586_d_n9, assign14890_e9586_d_n10, assign14890_e9586_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14890_e9582: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14890_e9583: f64 = (1.0 + assign14890_e9582);
        let assign14890_e9584: f64 = (0.5 * assign14890_e9583);
        (assign14890_e9584, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14890_e9586;
        locals.var_t0_dn0 = assign14890_e9586_d_n0;
        locals.var_t0_dn2 = assign14890_e9586_d_n2;
        locals.var_t0_dn4 = assign14890_e9586_d_n4;
        locals.var_t0_dn5 = assign14890_e9586_d_n5;
        locals.var_t0_dn6 = assign14890_e9586_d_n6;
        locals.var_t0_dn7 = assign14890_e9586_d_n7;
        locals.var_t0_dn8 = assign14890_e9586_d_n8;
        locals.var_t0_dn9 = assign14890_e9586_d_n9;
        locals.var_t0_dn10 = assign14890_e9586_d_n10;
        locals.var_t0_dn13 = assign14890_e9586_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign14900_e9605, assign14900_e9605_d_n0, assign14900_e9605_d_n2, assign14900_e9605_d_n4, assign14900_e9605_d_n5, assign14900_e9605_d_n6, assign14900_e9605_d_n7, assign14900_e9605_d_n8, assign14900_e9605_d_n9, assign14900_e9605_d_n10, assign14900_e9605_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14900_e9597: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14900_e9601: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14900_e9602: f64 = (0.5 * assign14900_e9601);
        let assign14900_e9603: f64 = (assign14900_e9597 + assign14900_e9602);
        (assign14900_e9603, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign14900_e9605;
        locals.var_rdvde_dn0 = assign14900_e9605_d_n0;
        locals.var_rdvde_dn2 = assign14900_e9605_d_n2;
        locals.var_rdvde_dn4 = assign14900_e9605_d_n4;
        locals.var_rdvde_dn5 = assign14900_e9605_d_n5;
        locals.var_rdvde_dn6 = assign14900_e9605_d_n6;
        locals.var_rdvde_dn7 = assign14900_e9605_d_n7;
        locals.var_rdvde_dn8 = assign14900_e9605_d_n8;
        locals.var_rdvde_dn9 = assign14900_e9605_d_n9;
        locals.var_rdvde_dn10 = assign14900_e9605_d_n10;
        locals.var_rdvde_dn13 = assign14900_e9605_d_n13;
        locals.var_rdvde_rv = 0.0;

        let (assign14910_e9629, assign14910_e9629_d_n0, assign14910_e9629_d_n2, assign14910_e9629_d_n4, assign14910_e9629_d_n5, assign14910_e9629_d_n6, assign14910_e9629_d_n7, assign14910_e9629_d_n8, assign14910_e9629_d_n9, assign14910_e9629_d_n10, assign14910_e9629_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14910_e9614: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign14910_e9616: f64 = (assign14910_e9614 * 1000000.0);
        let assign14910_e9618: f64 = (assign14910_e9616 + locals.var_uc_rdict1);
        let assign14910_e9619: f64 = (locals.var_rdvdtemp0 * assign14910_e9618);
        let assign14910_e9622: f64 = (p.p70 * p.p100);
        let assign14910_e9624: f64 = (assign14910_e9622 * 1000000.0);
        let assign14910_e9626: f64 = (assign14910_e9624 + p.p101);
        let assign14910_e9627: f64 = (assign14910_e9619 * assign14910_e9626);
        (assign14910_e9627, ((locals.var_rdvdtemp0_dn0 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn2 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn4 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn5 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn6 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn7 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn8 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn9 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn10 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn13 * assign14910_e9618) * assign14910_e9626),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign14910_e9629;
        locals.var_t4_dn0 = assign14910_e9629_d_n0;
        locals.var_t4_dn2 = assign14910_e9629_d_n2;
        locals.var_t4_dn4 = assign14910_e9629_d_n4;
        locals.var_t4_dn5 = assign14910_e9629_d_n5;
        locals.var_t4_dn6 = assign14910_e9629_d_n6;
        locals.var_t4_dn7 = assign14910_e9629_d_n7;
        locals.var_t4_dn8 = assign14910_e9629_d_n8;
        locals.var_t4_dn9 = assign14910_e9629_d_n9;
        locals.var_t4_dn10 = assign14910_e9629_d_n10;
        locals.var_t4_dn13 = assign14910_e9629_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign14920_e9643, assign14920_e9643_d_n0, assign14920_e9643_d_n2, assign14920_e9643_d_n4, assign14920_e9643_d_n5, assign14920_e9643_d_n6, assign14920_e9643_d_n7, assign14920_e9643_d_n8, assign14920_e9643_d_n9, assign14920_e9643_d_n10, assign14920_e9643_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14920_e9637: f64 = (1.0 - locals.var_uc_rdov13);
        let assign14920_e9639: f64 = (assign14920_e9637 * p.p66);
        let assign14920_e9641: f64 = (assign14920_e9639 * 1000000.0);
        (assign14920_e9641, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign14920_e9643;
        locals.var_t1_dn0 = assign14920_e9643_d_n0;
        locals.var_t1_dn2 = assign14920_e9643_d_n2;
        locals.var_t1_dn4 = assign14920_e9643_d_n4;
        locals.var_t1_dn5 = assign14920_e9643_d_n5;
        locals.var_t1_dn6 = assign14920_e9643_d_n6;
        locals.var_t1_dn7 = assign14920_e9643_d_n7;
        locals.var_t1_dn8 = assign14920_e9643_d_n8;
        locals.var_t1_dn9 = assign14920_e9643_d_n9;
        locals.var_t1_dn10 = assign14920_e9643_d_n10;
        locals.var_t1_dn13 = assign14920_e9643_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14930_e9659, assign14930_e9659_d_n0, assign14930_e9659_d_n2, assign14930_e9659_d_n4, assign14930_e9659_d_n5, assign14930_e9659_d_n6, assign14930_e9659_d_n7, assign14930_e9659_d_n8, assign14930_e9659_d_n9, assign14930_e9659_d_n10, assign14930_e9659_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14930_e9651: f64 = (locals.var_t8 * p.p66);
        let assign14930_e9653: f64 = (assign14930_e9651 * 1000000.0);
        let assign14930_e9655: f64 = (assign14930_e9653 + 1.0);
        let assign14930_e9657: f64 = (assign14930_e9655 + p.p98);
        (assign14930_e9657, ((locals.var_t8_dn0 * p.p66) * 1000000.0), ((locals.var_t8_dn2 * p.p66) * 1000000.0), ((locals.var_t8_dn4 * p.p66) * 1000000.0), ((locals.var_t8_dn5 * p.p66) * 1000000.0), ((locals.var_t8_dn6 * p.p66) * 1000000.0), ((locals.var_t8_dn7 * p.p66) * 1000000.0), ((locals.var_t8_dn8 * p.p66) * 1000000.0), ((locals.var_t8_dn9 * p.p66) * 1000000.0), ((locals.var_t8_dn10 * p.p66) * 1000000.0), ((locals.var_t8_dn13 * p.p66) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign14930_e9659;
        locals.var_t3_dn0 = assign14930_e9659_d_n0;
        locals.var_t3_dn2 = assign14930_e9659_d_n2;
        locals.var_t3_dn4 = assign14930_e9659_d_n4;
        locals.var_t3_dn5 = assign14930_e9659_d_n5;
        locals.var_t3_dn6 = assign14930_e9659_d_n6;
        locals.var_t3_dn7 = assign14930_e9659_d_n7;
        locals.var_t3_dn8 = assign14930_e9659_d_n8;
        locals.var_t3_dn9 = assign14930_e9659_d_n9;
        locals.var_t3_dn10 = assign14930_e9659_d_n10;
        locals.var_t3_dn13 = assign14930_e9659_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign14940_e9673, assign14940_e9673_d_n0, assign14940_e9673_d_n2, assign14940_e9673_d_n4, assign14940_e9673_d_n5, assign14940_e9673_d_n6, assign14940_e9673_d_n7, assign14940_e9673_d_n8, assign14940_e9673_d_n9, assign14940_e9673_d_n10, assign14940_e9673_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14940_e9667: f64 = (locals.var_t3 * locals.var_t4);
        let assign14940_e9669: f64 = (assign14940_e9667 - locals.var_t4);
        let assign14940_e9671: f64 = (assign14940_e9669 - 0.01);
        (assign14940_e9671, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn13 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn13)) - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14940_e9673;
        locals.var_tmf1_dn0 = assign14940_e9673_d_n0;
        locals.var_tmf1_dn2 = assign14940_e9673_d_n2;
        locals.var_tmf1_dn4 = assign14940_e9673_d_n4;
        locals.var_tmf1_dn5 = assign14940_e9673_d_n5;
        locals.var_tmf1_dn6 = assign14940_e9673_d_n6;
        locals.var_tmf1_dn7 = assign14940_e9673_d_n7;
        locals.var_tmf1_dn8 = assign14940_e9673_d_n8;
        locals.var_tmf1_dn9 = assign14940_e9673_d_n9;
        locals.var_tmf1_dn10 = assign14940_e9673_d_n10;
        locals.var_tmf1_dn13 = assign14940_e9673_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign14950_e9685, assign14950_e9685_d_n0, assign14950_e9685_d_n2, assign14950_e9685_d_n4, assign14950_e9685_d_n5, assign14950_e9685_d_n6, assign14950_e9685_d_n7, assign14950_e9685_d_n8, assign14950_e9685_d_n9, assign14950_e9685_d_n10, assign14950_e9685_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14950_e9681: f64 = (4.0 * locals.var_t4);
        let assign14950_e9683: f64 = (assign14950_e9681 * 0.01);
        (assign14950_e9683, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn13) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14950_e9685;
        locals.var_tmf2_dn0 = assign14950_e9685_d_n0;
        locals.var_tmf2_dn2 = assign14950_e9685_d_n2;
        locals.var_tmf2_dn4 = assign14950_e9685_d_n4;
        locals.var_tmf2_dn5 = assign14950_e9685_d_n5;
        locals.var_tmf2_dn6 = assign14950_e9685_d_n6;
        locals.var_tmf2_dn7 = assign14950_e9685_d_n7;
        locals.var_tmf2_dn8 = assign14950_e9685_d_n8;
        locals.var_tmf2_dn9 = assign14950_e9685_d_n9;
        locals.var_tmf2_dn10 = assign14950_e9685_d_n10;
        locals.var_tmf2_dn13 = assign14950_e9685_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14960_e9699, assign14960_e9699_d_n0, assign14960_e9699_d_n2, assign14960_e9699_d_n4, assign14960_e9699_d_n5, assign14960_e9699_d_n6, assign14960_e9699_d_n7, assign14960_e9699_d_n8, assign14960_e9699_d_n9, assign14960_e9699_d_n10, assign14960_e9699_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign14960_e9697, assign14960_e9697_d_n0, assign14960_e9697_d_n2, assign14960_e9697_d_n4, assign14960_e9697_d_n5, assign14960_e9697_d_n6, assign14960_e9697_d_n7, assign14960_e9697_d_n8, assign14960_e9697_d_n9, assign14960_e9697_d_n10, assign14960_e9697_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14960_e9696: f64 = (-locals.var_tmf2);
                (assign14960_e9696, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14960_e9697, assign14960_e9697_d_n0, assign14960_e9697_d_n2, assign14960_e9697_d_n4, assign14960_e9697_d_n5, assign14960_e9697_d_n6, assign14960_e9697_d_n7, assign14960_e9697_d_n8, assign14960_e9697_d_n9, assign14960_e9697_d_n10, assign14960_e9697_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14960_e9699;
        locals.var_tmf2_dn0 = assign14960_e9699_d_n0;
        locals.var_tmf2_dn2 = assign14960_e9699_d_n2;
        locals.var_tmf2_dn4 = assign14960_e9699_d_n4;
        locals.var_tmf2_dn5 = assign14960_e9699_d_n5;
        locals.var_tmf2_dn6 = assign14960_e9699_d_n6;
        locals.var_tmf2_dn7 = assign14960_e9699_d_n7;
        locals.var_tmf2_dn8 = assign14960_e9699_d_n8;
        locals.var_tmf2_dn9 = assign14960_e9699_d_n9;
        locals.var_tmf2_dn10 = assign14960_e9699_d_n10;
        locals.var_tmf2_dn13 = assign14960_e9699_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14970_e9712, assign14970_e9712_d_n0, assign14970_e9712_d_n2, assign14970_e9712_d_n4, assign14970_e9712_d_n5, assign14970_e9712_d_n6, assign14970_e9712_d_n7, assign14970_e9712_d_n8, assign14970_e9712_d_n9, assign14970_e9712_d_n10, assign14970_e9712_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14970_e9707: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14970_e9709: f64 = (assign14970_e9707 + locals.var_tmf2);
        let assign14970_e9710: f64 = (assign14970_e9709).sqrt();
        (assign14970_e9710, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14970_e9710)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14970_e9712;
        locals.var_tmf2_dn0 = assign14970_e9712_d_n0;
        locals.var_tmf2_dn2 = assign14970_e9712_d_n2;
        locals.var_tmf2_dn4 = assign14970_e9712_d_n4;
        locals.var_tmf2_dn5 = assign14970_e9712_d_n5;
        locals.var_tmf2_dn6 = assign14970_e9712_d_n6;
        locals.var_tmf2_dn7 = assign14970_e9712_d_n7;
        locals.var_tmf2_dn8 = assign14970_e9712_d_n8;
        locals.var_tmf2_dn9 = assign14970_e9712_d_n9;
        locals.var_tmf2_dn10 = assign14970_e9712_d_n10;
        locals.var_tmf2_dn13 = assign14970_e9712_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign14980_e9726, assign14980_e9726_d_n0, assign14980_e9726_d_n2, assign14980_e9726_d_n4, assign14980_e9726_d_n5, assign14980_e9726_d_n6, assign14980_e9726_d_n7, assign14980_e9726_d_n8, assign14980_e9726_d_n9, assign14980_e9726_d_n10, assign14980_e9726_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14980_e9722: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14980_e9723: f64 = (1.0 + assign14980_e9722);
        let assign14980_e9724: f64 = (0.5 * assign14980_e9723);
        (assign14980_e9724, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign14980_e9726;
        locals.var_t6_dn0 = assign14980_e9726_d_n0;
        locals.var_t6_dn2 = assign14980_e9726_d_n2;
        locals.var_t6_dn4 = assign14980_e9726_d_n4;
        locals.var_t6_dn5 = assign14980_e9726_d_n5;
        locals.var_t6_dn6 = assign14980_e9726_d_n6;
        locals.var_t6_dn7 = assign14980_e9726_d_n7;
        locals.var_t6_dn8 = assign14980_e9726_d_n8;
        locals.var_t6_dn9 = assign14980_e9726_d_n9;
        locals.var_t6_dn10 = assign14980_e9726_d_n10;
        locals.var_t6_dn13 = assign14980_e9726_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign14990_e9740, assign14990_e9740_d_n0, assign14990_e9740_d_n2, assign14990_e9740_d_n4, assign14990_e9740_d_n5, assign14990_e9740_d_n6, assign14990_e9740_d_n7, assign14990_e9740_d_n8, assign14990_e9740_d_n9, assign14990_e9740_d_n10, assign14990_e9740_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14990_e9736: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14990_e9737: f64 = (0.5 * assign14990_e9736);
        let assign14990_e9738: f64 = (locals.var_t4 + assign14990_e9737);
        (assign14990_e9738, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign14990_e9740;
        locals.var_t5_dn0 = assign14990_e9740_d_n0;
        locals.var_t5_dn2 = assign14990_e9740_d_n2;
        locals.var_t5_dn4 = assign14990_e9740_d_n4;
        locals.var_t5_dn5 = assign14990_e9740_d_n5;
        locals.var_t5_dn6 = assign14990_e9740_d_n6;
        locals.var_t5_dn7 = assign14990_e9740_d_n7;
        locals.var_t5_dn8 = assign14990_e9740_d_n8;
        locals.var_t5_dn9 = assign14990_e9740_d_n9;
        locals.var_t5_dn10 = assign14990_e9740_d_n10;
        locals.var_t5_dn13 = assign14990_e9740_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign15000_e9756, assign15000_e9756_d_n0, assign15000_e9756_d_n2, assign15000_e9756_d_n4, assign15000_e9756_d_n5, assign15000_e9756_d_n6, assign15000_e9756_d_n7, assign15000_e9756_d_n8, assign15000_e9756_d_n9, assign15000_e9756_d_n10, assign15000_e9756_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15000_e9749: f64 = (p.p98 + 1.0);
        let assign15000_e9750: f64 = (locals.var_t4 * assign15000_e9749);
        let assign15000_e9752: f64 = (assign15000_e9750 - locals.var_t5);
        let assign15000_e9754: f64 = (assign15000_e9752 - 5e-5);
        (assign15000_e9754, ((locals.var_t4_dn0 * assign15000_e9749) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign15000_e9749) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign15000_e9749) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign15000_e9749) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign15000_e9749) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign15000_e9749) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign15000_e9749) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign15000_e9749) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign15000_e9749) - locals.var_t5_dn10), ((locals.var_t4_dn13 * assign15000_e9749) - locals.var_t5_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign15000_e9756;
        locals.var_tmf1_dn0 = assign15000_e9756_d_n0;
        locals.var_tmf1_dn2 = assign15000_e9756_d_n2;
        locals.var_tmf1_dn4 = assign15000_e9756_d_n4;
        locals.var_tmf1_dn5 = assign15000_e9756_d_n5;
        locals.var_tmf1_dn6 = assign15000_e9756_d_n6;
        locals.var_tmf1_dn7 = assign15000_e9756_d_n7;
        locals.var_tmf1_dn8 = assign15000_e9756_d_n8;
        locals.var_tmf1_dn9 = assign15000_e9756_d_n9;
        locals.var_tmf1_dn10 = assign15000_e9756_d_n10;
        locals.var_tmf1_dn13 = assign15000_e9756_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign15010_e9772, assign15010_e9772_d_n0, assign15010_e9772_d_n2, assign15010_e9772_d_n4, assign15010_e9772_d_n5, assign15010_e9772_d_n6, assign15010_e9772_d_n7, assign15010_e9772_d_n8, assign15010_e9772_d_n9, assign15010_e9772_d_n10, assign15010_e9772_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15010_e9766: f64 = (p.p98 + 1.0);
        let assign15010_e9767: f64 = (locals.var_t4 * assign15010_e9766);
        let assign15010_e9768: f64 = (4.0 * assign15010_e9767);
        let assign15010_e9770: f64 = (assign15010_e9768 * 5e-5);
        (assign15010_e9770, ((4.0 * (locals.var_t4_dn0 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn13 * assign15010_e9766)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15010_e9772;
        locals.var_tmf2_dn0 = assign15010_e9772_d_n0;
        locals.var_tmf2_dn2 = assign15010_e9772_d_n2;
        locals.var_tmf2_dn4 = assign15010_e9772_d_n4;
        locals.var_tmf2_dn5 = assign15010_e9772_d_n5;
        locals.var_tmf2_dn6 = assign15010_e9772_d_n6;
        locals.var_tmf2_dn7 = assign15010_e9772_d_n7;
        locals.var_tmf2_dn8 = assign15010_e9772_d_n8;
        locals.var_tmf2_dn9 = assign15010_e9772_d_n9;
        locals.var_tmf2_dn10 = assign15010_e9772_d_n10;
        locals.var_tmf2_dn13 = assign15010_e9772_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15020_e9786, assign15020_e9786_d_n0, assign15020_e9786_d_n2, assign15020_e9786_d_n4, assign15020_e9786_d_n5, assign15020_e9786_d_n6, assign15020_e9786_d_n7, assign15020_e9786_d_n8, assign15020_e9786_d_n9, assign15020_e9786_d_n10, assign15020_e9786_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign15020_e9784, assign15020_e9784_d_n0, assign15020_e9784_d_n2, assign15020_e9784_d_n4, assign15020_e9784_d_n5, assign15020_e9784_d_n6, assign15020_e9784_d_n7, assign15020_e9784_d_n8, assign15020_e9784_d_n9, assign15020_e9784_d_n10, assign15020_e9784_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign15020_e9783: f64 = (-locals.var_tmf2);
                (assign15020_e9783, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign15020_e9784, assign15020_e9784_d_n0, assign15020_e9784_d_n2, assign15020_e9784_d_n4, assign15020_e9784_d_n5, assign15020_e9784_d_n6, assign15020_e9784_d_n7, assign15020_e9784_d_n8, assign15020_e9784_d_n9, assign15020_e9784_d_n10, assign15020_e9784_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15020_e9786;
        locals.var_tmf2_dn0 = assign15020_e9786_d_n0;
        locals.var_tmf2_dn2 = assign15020_e9786_d_n2;
        locals.var_tmf2_dn4 = assign15020_e9786_d_n4;
        locals.var_tmf2_dn5 = assign15020_e9786_d_n5;
        locals.var_tmf2_dn6 = assign15020_e9786_d_n6;
        locals.var_tmf2_dn7 = assign15020_e9786_d_n7;
        locals.var_tmf2_dn8 = assign15020_e9786_d_n8;
        locals.var_tmf2_dn9 = assign15020_e9786_d_n9;
        locals.var_tmf2_dn10 = assign15020_e9786_d_n10;
        locals.var_tmf2_dn13 = assign15020_e9786_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15030_e9799, assign15030_e9799_d_n0, assign15030_e9799_d_n2, assign15030_e9799_d_n4, assign15030_e9799_d_n5, assign15030_e9799_d_n6, assign15030_e9799_d_n7, assign15030_e9799_d_n8, assign15030_e9799_d_n9, assign15030_e9799_d_n10, assign15030_e9799_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15030_e9794: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15030_e9796: f64 = (assign15030_e9794 + locals.var_tmf2);
        let assign15030_e9797: f64 = (assign15030_e9796).sqrt();
        (assign15030_e9797, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign15030_e9797)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15030_e9799;
        locals.var_tmf2_dn0 = assign15030_e9799_d_n0;
        locals.var_tmf2_dn2 = assign15030_e9799_d_n2;
        locals.var_tmf2_dn4 = assign15030_e9799_d_n4;
        locals.var_tmf2_dn5 = assign15030_e9799_d_n5;
        locals.var_tmf2_dn6 = assign15030_e9799_d_n6;
        locals.var_tmf2_dn7 = assign15030_e9799_d_n7;
        locals.var_tmf2_dn8 = assign15030_e9799_d_n8;
        locals.var_tmf2_dn9 = assign15030_e9799_d_n9;
        locals.var_tmf2_dn10 = assign15030_e9799_d_n10;
        locals.var_tmf2_dn13 = assign15030_e9799_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15040_e9813, assign15040_e9813_d_n0, assign15040_e9813_d_n2, assign15040_e9813_d_n4, assign15040_e9813_d_n5, assign15040_e9813_d_n6, assign15040_e9813_d_n7, assign15040_e9813_d_n8, assign15040_e9813_d_n9, assign15040_e9813_d_n10, assign15040_e9813_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15040_e9809: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15040_e9810: f64 = (1.0 + assign15040_e9809);
        let assign15040_e9811: f64 = (0.5 * assign15040_e9810);
        (assign15040_e9811, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign15040_e9813;
        locals.var_t6_dn0 = assign15040_e9813_d_n0;
        locals.var_t6_dn2 = assign15040_e9813_d_n2;
        locals.var_t6_dn4 = assign15040_e9813_d_n4;
        locals.var_t6_dn5 = assign15040_e9813_d_n5;
        locals.var_t6_dn6 = assign15040_e9813_d_n6;
        locals.var_t6_dn7 = assign15040_e9813_d_n7;
        locals.var_t6_dn8 = assign15040_e9813_d_n8;
        locals.var_t6_dn9 = assign15040_e9813_d_n9;
        locals.var_t6_dn10 = assign15040_e9813_d_n10;
        locals.var_t6_dn13 = assign15040_e9813_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign15050_e9831, assign15050_e9831_d_n0, assign15050_e9831_d_n2, assign15050_e9831_d_n4, assign15050_e9831_d_n5, assign15050_e9831_d_n6, assign15050_e9831_d_n7, assign15050_e9831_d_n8, assign15050_e9831_d_n9, assign15050_e9831_d_n10, assign15050_e9831_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15050_e9822: f64 = (p.p98 + 1.0);
        let assign15050_e9823: f64 = (locals.var_t4 * assign15050_e9822);
        let assign15050_e9827: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15050_e9828: f64 = (0.5 * assign15050_e9827);
        let assign15050_e9829: f64 = (assign15050_e9823 - assign15050_e9828);
        (assign15050_e9829, ((locals.var_t4_dn0 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn13 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign15050_e9831;
        locals.var_t7_dn0 = assign15050_e9831_d_n0;
        locals.var_t7_dn2 = assign15050_e9831_d_n2;
        locals.var_t7_dn4 = assign15050_e9831_d_n4;
        locals.var_t7_dn5 = assign15050_e9831_d_n5;
        locals.var_t7_dn6 = assign15050_e9831_d_n6;
        locals.var_t7_dn7 = assign15050_e9831_d_n7;
        locals.var_t7_dn8 = assign15050_e9831_d_n8;
        locals.var_t7_dn9 = assign15050_e9831_d_n9;
        locals.var_t7_dn10 = assign15050_e9831_d_n10;
        locals.var_t7_dn13 = assign15050_e9831_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign15060_e9847, assign15060_e9847_d_n0, assign15060_e9847_d_n2, assign15060_e9847_d_n4, assign15060_e9847_d_n5, assign15060_e9847_d_n6, assign15060_e9847_d_n7, assign15060_e9847_d_n8, assign15060_e9847_d_n9, assign15060_e9847_d_n10, assign15060_e9847_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15060_e9840: f64 = (locals.var_t1 * locals.var_t4);
        let assign15060_e9841: f64 = (locals.var_t7 + assign15060_e9840);
        let assign15060_e9843: f64 = assign15060_e9841;
        let assign15060_e9845: f64 = (assign15060_e9843 - 5e-5);
        (assign15060_e9845, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn13 + ((locals.var_t1_dn13 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign15060_e9847;
        locals.var_tmf1_dn0 = assign15060_e9847_d_n0;
        locals.var_tmf1_dn2 = assign15060_e9847_d_n2;
        locals.var_tmf1_dn4 = assign15060_e9847_d_n4;
        locals.var_tmf1_dn5 = assign15060_e9847_d_n5;
        locals.var_tmf1_dn6 = assign15060_e9847_d_n6;
        locals.var_tmf1_dn7 = assign15060_e9847_d_n7;
        locals.var_tmf1_dn8 = assign15060_e9847_d_n8;
        locals.var_tmf1_dn9 = assign15060_e9847_d_n9;
        locals.var_tmf1_dn10 = assign15060_e9847_d_n10;
        locals.var_tmf1_dn13 = assign15060_e9847_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign15070_e9859, assign15070_e9859_d_n0, assign15070_e9859_d_n2, assign15070_e9859_d_n4, assign15070_e9859_d_n5, assign15070_e9859_d_n6, assign15070_e9859_d_n7, assign15070_e9859_d_n8, assign15070_e9859_d_n9, assign15070_e9859_d_n10, assign15070_e9859_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15070_e9859;
        locals.var_tmf2_dn0 = assign15070_e9859_d_n0;
        locals.var_tmf2_dn2 = assign15070_e9859_d_n2;
        locals.var_tmf2_dn4 = assign15070_e9859_d_n4;
        locals.var_tmf2_dn5 = assign15070_e9859_d_n5;
        locals.var_tmf2_dn6 = assign15070_e9859_d_n6;
        locals.var_tmf2_dn7 = assign15070_e9859_d_n7;
        locals.var_tmf2_dn8 = assign15070_e9859_d_n8;
        locals.var_tmf2_dn9 = assign15070_e9859_d_n9;
        locals.var_tmf2_dn10 = assign15070_e9859_d_n10;
        locals.var_tmf2_dn13 = assign15070_e9859_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15080_e9873, assign15080_e9873_d_n0, assign15080_e9873_d_n2, assign15080_e9873_d_n4, assign15080_e9873_d_n5, assign15080_e9873_d_n6, assign15080_e9873_d_n7, assign15080_e9873_d_n8, assign15080_e9873_d_n9, assign15080_e9873_d_n10, assign15080_e9873_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign15080_e9871, assign15080_e9871_d_n0, assign15080_e9871_d_n2, assign15080_e9871_d_n4, assign15080_e9871_d_n5, assign15080_e9871_d_n6, assign15080_e9871_d_n7, assign15080_e9871_d_n8, assign15080_e9871_d_n9, assign15080_e9871_d_n10, assign15080_e9871_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign15080_e9870: f64 = (-locals.var_tmf2);
                (assign15080_e9870, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign15080_e9871, assign15080_e9871_d_n0, assign15080_e9871_d_n2, assign15080_e9871_d_n4, assign15080_e9871_d_n5, assign15080_e9871_d_n6, assign15080_e9871_d_n7, assign15080_e9871_d_n8, assign15080_e9871_d_n9, assign15080_e9871_d_n10, assign15080_e9871_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15080_e9873;
        locals.var_tmf2_dn0 = assign15080_e9873_d_n0;
        locals.var_tmf2_dn2 = assign15080_e9873_d_n2;
        locals.var_tmf2_dn4 = assign15080_e9873_d_n4;
        locals.var_tmf2_dn5 = assign15080_e9873_d_n5;
        locals.var_tmf2_dn6 = assign15080_e9873_d_n6;
        locals.var_tmf2_dn7 = assign15080_e9873_d_n7;
        locals.var_tmf2_dn8 = assign15080_e9873_d_n8;
        locals.var_tmf2_dn9 = assign15080_e9873_d_n9;
        locals.var_tmf2_dn10 = assign15080_e9873_d_n10;
        locals.var_tmf2_dn13 = assign15080_e9873_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15090_e9886, assign15090_e9886_d_n0, assign15090_e9886_d_n2, assign15090_e9886_d_n4, assign15090_e9886_d_n5, assign15090_e9886_d_n6, assign15090_e9886_d_n7, assign15090_e9886_d_n8, assign15090_e9886_d_n9, assign15090_e9886_d_n10, assign15090_e9886_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15090_e9881: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15090_e9883: f64 = (assign15090_e9881 + locals.var_tmf2);
        let assign15090_e9884: f64 = (assign15090_e9883).sqrt();
        (assign15090_e9884, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign15090_e9884)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15090_e9886;
        locals.var_tmf2_dn0 = assign15090_e9886_d_n0;
        locals.var_tmf2_dn2 = assign15090_e9886_d_n2;
        locals.var_tmf2_dn4 = assign15090_e9886_d_n4;
        locals.var_tmf2_dn5 = assign15090_e9886_d_n5;
        locals.var_tmf2_dn6 = assign15090_e9886_d_n6;
        locals.var_tmf2_dn7 = assign15090_e9886_d_n7;
        locals.var_tmf2_dn8 = assign15090_e9886_d_n8;
        locals.var_tmf2_dn9 = assign15090_e9886_d_n9;
        locals.var_tmf2_dn10 = assign15090_e9886_d_n10;
        locals.var_tmf2_dn13 = assign15090_e9886_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15100_e9900, assign15100_e9900_d_n0, assign15100_e9900_d_n2, assign15100_e9900_d_n4, assign15100_e9900_d_n5, assign15100_e9900_d_n6, assign15100_e9900_d_n7, assign15100_e9900_d_n8, assign15100_e9900_d_n9, assign15100_e9900_d_n10, assign15100_e9900_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15100_e9896: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15100_e9897: f64 = (1.0 + assign15100_e9896);
        let assign15100_e9898: f64 = (0.5 * assign15100_e9897);
        (assign15100_e9898, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign15100_e9900;
        locals.var_t6_dn0 = assign15100_e9900_d_n0;
        locals.var_t6_dn2 = assign15100_e9900_d_n2;
        locals.var_t6_dn4 = assign15100_e9900_d_n4;
        locals.var_t6_dn5 = assign15100_e9900_d_n5;
        locals.var_t6_dn6 = assign15100_e9900_d_n6;
        locals.var_t6_dn7 = assign15100_e9900_d_n7;
        locals.var_t6_dn8 = assign15100_e9900_d_n8;
        locals.var_t6_dn9 = assign15100_e9900_d_n9;
        locals.var_t6_dn10 = assign15100_e9900_d_n10;
        locals.var_t6_dn13 = assign15100_e9900_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign15110_e9914, assign15110_e9914_d_n0, assign15110_e9914_d_n2, assign15110_e9914_d_n4, assign15110_e9914_d_n5, assign15110_e9914_d_n6, assign15110_e9914_d_n7, assign15110_e9914_d_n8, assign15110_e9914_d_n9, assign15110_e9914_d_n10, assign15110_e9914_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15110_e9910: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15110_e9911: f64 = (0.5 * assign15110_e9910);
        let assign15110_e9912: f64 = assign15110_e9911;
        (assign15110_e9912, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign15110_e9914;
        locals.var_t2_dn0 = assign15110_e9914_d_n0;
        locals.var_t2_dn2 = assign15110_e9914_d_n2;
        locals.var_t2_dn4 = assign15110_e9914_d_n4;
        locals.var_t2_dn5 = assign15110_e9914_d_n5;
        locals.var_t2_dn6 = assign15110_e9914_d_n6;
        locals.var_t2_dn7 = assign15110_e9914_d_n7;
        locals.var_t2_dn8 = assign15110_e9914_d_n8;
        locals.var_t2_dn9 = assign15110_e9914_d_n9;
        locals.var_t2_dn10 = assign15110_e9914_d_n10;
        locals.var_t2_dn13 = assign15110_e9914_d_n13;
        locals.var_t2_rv = 0.0;

        let assign15120_e9921: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard321 = assign15120_e9921;
        locals.var_guard321_rv = 0.0;

        let (assign15130_e9941, assign15130_e9941_d_n0, assign15130_e9941_d_n2, assign15130_e9941_d_n4, assign15130_e9941_d_n5, assign15130_e9941_d_n6, assign15130_e9941_d_n7, assign15130_e9941_d_n8, assign15130_e9941_d_n9, assign15130_e9941_d_n10, assign15130_e9941_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15130_e9932: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign15130_e9933: f64 = (locals.var_uc_rdvd + assign15130_e9932);
        let assign15130_e9936: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign15130_e9937: f64 = (assign15130_e9933 + assign15130_e9936);
        let assign15130_e9939: f64 = (assign15130_e9937 * locals.var_t2);
        (assign15130_e9939, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign15130_e9941;
        locals.var_rsvde_dn0 = assign15130_e9941_d_n0;
        locals.var_rsvde_dn2 = assign15130_e9941_d_n2;
        locals.var_rsvde_dn4 = assign15130_e9941_d_n4;
        locals.var_rsvde_dn5 = assign15130_e9941_d_n5;
        locals.var_rsvde_dn6 = assign15130_e9941_d_n6;
        locals.var_rsvde_dn7 = assign15130_e9941_d_n7;
        locals.var_rsvde_dn8 = assign15130_e9941_d_n8;
        locals.var_rsvde_dn9 = assign15130_e9941_d_n9;
        locals.var_rsvde_dn10 = assign15130_e9941_d_n10;
        locals.var_rsvde_dn13 = assign15130_e9941_d_n13;
        locals.var_rsvde_rv = 0.0;

        let (assign15140_e9959, assign15140_e9959_d_n0, assign15140_e9959_d_n2, assign15140_e9959_d_n4, assign15140_e9959_d_n5, assign15140_e9959_d_n6, assign15140_e9959_d_n7, assign15140_e9959_d_n8, assign15140_e9959_d_n9, assign15140_e9959_d_n10, assign15140_e9959_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15140_e9952: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15140_e9953: f64 = (locals.var_rsvde - assign15140_e9952);
        let assign15140_e9956: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15140_e9957: f64 = (assign15140_e9953 - assign15140_e9956);
        (assign15140_e9957, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign15140_e9959;
        locals.var_tmf1_dn0 = assign15140_e9959_d_n0;
        locals.var_tmf1_dn2 = assign15140_e9959_d_n2;
        locals.var_tmf1_dn4 = assign15140_e9959_d_n4;
        locals.var_tmf1_dn5 = assign15140_e9959_d_n5;
        locals.var_tmf1_dn6 = assign15140_e9959_d_n6;
        locals.var_tmf1_dn7 = assign15140_e9959_d_n7;
        locals.var_tmf1_dn8 = assign15140_e9959_d_n8;
        locals.var_tmf1_dn9 = assign15140_e9959_d_n9;
        locals.var_tmf1_dn10 = assign15140_e9959_d_n10;
        locals.var_tmf1_dn13 = assign15140_e9959_d_n13;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15150_e9977, assign15150_e9977_d_n0, assign15150_e9977_d_n2, assign15150_e9977_d_n4, assign15150_e9977_d_n5, assign15150_e9977_d_n6, assign15150_e9977_d_n7, assign15150_e9977_d_n8, assign15150_e9977_d_n9, assign15150_e9977_d_n10, assign15150_e9977_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15150_e9970: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15150_e9971: f64 = (4.0 * assign15150_e9970);
        let assign15150_e9974: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15150_e9975: f64 = (assign15150_e9971 * assign15150_e9974);
        (assign15150_e9975, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15150_e9977;
        locals.var_tmf2_dn0 = assign15150_e9977_d_n0;
        locals.var_tmf2_dn2 = assign15150_e9977_d_n2;
        locals.var_tmf2_dn4 = assign15150_e9977_d_n4;
        locals.var_tmf2_dn5 = assign15150_e9977_d_n5;
        locals.var_tmf2_dn6 = assign15150_e9977_d_n6;
        locals.var_tmf2_dn7 = assign15150_e9977_d_n7;
        locals.var_tmf2_dn8 = assign15150_e9977_d_n8;
        locals.var_tmf2_dn9 = assign15150_e9977_d_n9;
        locals.var_tmf2_dn10 = assign15150_e9977_d_n10;
        locals.var_tmf2_dn13 = assign15150_e9977_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15160_e9993, assign15160_e9993_d_n0, assign15160_e9993_d_n2, assign15160_e9993_d_n4, assign15160_e9993_d_n5, assign15160_e9993_d_n6, assign15160_e9993_d_n7, assign15160_e9993_d_n8, assign15160_e9993_d_n9, assign15160_e9993_d_n10, assign15160_e9993_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let (assign15160_e9991, assign15160_e9991_d_n0, assign15160_e9991_d_n2, assign15160_e9991_d_n4, assign15160_e9991_d_n5, assign15160_e9991_d_n6, assign15160_e9991_d_n7, assign15160_e9991_d_n8, assign15160_e9991_d_n9, assign15160_e9991_d_n10, assign15160_e9991_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign15160_e9990: f64 = (-locals.var_tmf2);
                (assign15160_e9990, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign15160_e9991, assign15160_e9991_d_n0, assign15160_e9991_d_n2, assign15160_e9991_d_n4, assign15160_e9991_d_n5, assign15160_e9991_d_n6, assign15160_e9991_d_n7, assign15160_e9991_d_n8, assign15160_e9991_d_n9, assign15160_e9991_d_n10, assign15160_e9991_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15160_e9993;
        locals.var_tmf2_dn0 = assign15160_e9993_d_n0;
        locals.var_tmf2_dn2 = assign15160_e9993_d_n2;
        locals.var_tmf2_dn4 = assign15160_e9993_d_n4;
        locals.var_tmf2_dn5 = assign15160_e9993_d_n5;
        locals.var_tmf2_dn6 = assign15160_e9993_d_n6;
        locals.var_tmf2_dn7 = assign15160_e9993_d_n7;
        locals.var_tmf2_dn8 = assign15160_e9993_d_n8;
        locals.var_tmf2_dn9 = assign15160_e9993_d_n9;
        locals.var_tmf2_dn10 = assign15160_e9993_d_n10;
        locals.var_tmf2_dn13 = assign15160_e9993_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15170_e10008, assign15170_e10008_d_n0, assign15170_e10008_d_n2, assign15170_e10008_d_n4, assign15170_e10008_d_n5, assign15170_e10008_d_n6, assign15170_e10008_d_n7, assign15170_e10008_d_n8, assign15170_e10008_d_n9, assign15170_e10008_d_n10, assign15170_e10008_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15170_e10003: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15170_e10005: f64 = (assign15170_e10003 + locals.var_tmf2);
        let assign15170_e10006: f64 = (assign15170_e10005).sqrt();
        (assign15170_e10006, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign15170_e10006)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15170_e10008;
        locals.var_tmf2_dn0 = assign15170_e10008_d_n0;
        locals.var_tmf2_dn2 = assign15170_e10008_d_n2;
        locals.var_tmf2_dn4 = assign15170_e10008_d_n4;
        locals.var_tmf2_dn5 = assign15170_e10008_d_n5;
        locals.var_tmf2_dn6 = assign15170_e10008_d_n6;
        locals.var_tmf2_dn7 = assign15170_e10008_d_n7;
        locals.var_tmf2_dn8 = assign15170_e10008_d_n8;
        locals.var_tmf2_dn9 = assign15170_e10008_d_n9;
        locals.var_tmf2_dn10 = assign15170_e10008_d_n10;
        locals.var_tmf2_dn13 = assign15170_e10008_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15180_e10024, assign15180_e10024_d_n0, assign15180_e10024_d_n2, assign15180_e10024_d_n4, assign15180_e10024_d_n5, assign15180_e10024_d_n6, assign15180_e10024_d_n7, assign15180_e10024_d_n8, assign15180_e10024_d_n9, assign15180_e10024_d_n10, assign15180_e10024_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15180_e10020: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15180_e10021: f64 = (1.0 + assign15180_e10020);
        let assign15180_e10022: f64 = (0.5 * assign15180_e10021);
        (assign15180_e10022, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign15180_e10024;
        locals.var_t0_dn0 = assign15180_e10024_d_n0;
        locals.var_t0_dn2 = assign15180_e10024_d_n2;
        locals.var_t0_dn4 = assign15180_e10024_d_n4;
        locals.var_t0_dn5 = assign15180_e10024_d_n5;
        locals.var_t0_dn6 = assign15180_e10024_d_n6;
        locals.var_t0_dn7 = assign15180_e10024_d_n7;
        locals.var_t0_dn8 = assign15180_e10024_d_n8;
        locals.var_t0_dn9 = assign15180_e10024_d_n9;
        locals.var_t0_dn10 = assign15180_e10024_d_n10;
        locals.var_t0_dn13 = assign15180_e10024_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign15190_e10042, assign15190_e10042_d_n0, assign15190_e10042_d_n2, assign15190_e10042_d_n4, assign15190_e10042_d_n5, assign15190_e10042_d_n6, assign15190_e10042_d_n7, assign15190_e10042_d_n8, assign15190_e10042_d_n9, assign15190_e10042_d_n10, assign15190_e10042_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15190_e10034: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15190_e10038: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15190_e10039: f64 = (0.5 * assign15190_e10038);
        let assign15190_e10040: f64 = (assign15190_e10034 + assign15190_e10039);
        (assign15190_e10040, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign15190_e10042;
        locals.var_rsvde_dn0 = assign15190_e10042_d_n0;
        locals.var_rsvde_dn2 = assign15190_e10042_d_n2;
        locals.var_rsvde_dn4 = assign15190_e10042_d_n4;
        locals.var_rsvde_dn5 = assign15190_e10042_d_n5;
        locals.var_rsvde_dn6 = assign15190_e10042_d_n6;
        locals.var_rsvde_dn7 = assign15190_e10042_d_n7;
        locals.var_rsvde_dn8 = assign15190_e10042_d_n8;
        locals.var_rsvde_dn9 = assign15190_e10042_d_n9;
        locals.var_rsvde_dn10 = assign15190_e10042_d_n10;
        locals.var_rsvde_dn13 = assign15190_e10042_d_n13;
        locals.var_rsvde_rv = 0.0;

        let (assign15200_e10063, assign15200_e10063_d_n0, assign15200_e10063_d_n2, assign15200_e10063_d_n4, assign15200_e10063_d_n5, assign15200_e10063_d_n6, assign15200_e10063_d_n7, assign15200_e10063_d_n8, assign15200_e10063_d_n9, assign15200_e10063_d_n10, assign15200_e10063_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15200_e10054: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign15200_e10055: f64 = (locals.var_uc_rdvd + assign15200_e10054);
        let assign15200_e10058: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign15200_e10059: f64 = (assign15200_e10055 + assign15200_e10058);
        let assign15200_e10061: f64 = (assign15200_e10059 * locals.var_t2);
        (assign15200_e10061, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign15200_e10063;
        locals.var_rsvde_dn0 = assign15200_e10063_d_n0;
        locals.var_rsvde_dn2 = assign15200_e10063_d_n2;
        locals.var_rsvde_dn4 = assign15200_e10063_d_n4;
        locals.var_rsvde_dn5 = assign15200_e10063_d_n5;
        locals.var_rsvde_dn6 = assign15200_e10063_d_n6;
        locals.var_rsvde_dn7 = assign15200_e10063_d_n7;
        locals.var_rsvde_dn8 = assign15200_e10063_d_n8;
        locals.var_rsvde_dn9 = assign15200_e10063_d_n9;
        locals.var_rsvde_dn10 = assign15200_e10063_d_n10;
        locals.var_rsvde_dn13 = assign15200_e10063_d_n13;
        locals.var_rsvde_rv = 0.0;

        let (assign15210_e10082, assign15210_e10082_d_n0, assign15210_e10082_d_n2, assign15210_e10082_d_n4, assign15210_e10082_d_n5, assign15210_e10082_d_n6, assign15210_e10082_d_n7, assign15210_e10082_d_n8, assign15210_e10082_d_n9, assign15210_e10082_d_n10, assign15210_e10082_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15210_e10075: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15210_e10076: f64 = (locals.var_rsvde - assign15210_e10075);
        let assign15210_e10079: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15210_e10080: f64 = (assign15210_e10076 - assign15210_e10079);
        (assign15210_e10080, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign15210_e10082;
        locals.var_tmf1_dn0 = assign15210_e10082_d_n0;
        locals.var_tmf1_dn2 = assign15210_e10082_d_n2;
        locals.var_tmf1_dn4 = assign15210_e10082_d_n4;
        locals.var_tmf1_dn5 = assign15210_e10082_d_n5;
        locals.var_tmf1_dn6 = assign15210_e10082_d_n6;
        locals.var_tmf1_dn7 = assign15210_e10082_d_n7;
        locals.var_tmf1_dn8 = assign15210_e10082_d_n8;
        locals.var_tmf1_dn9 = assign15210_e10082_d_n9;
        locals.var_tmf1_dn10 = assign15210_e10082_d_n10;
        locals.var_tmf1_dn13 = assign15210_e10082_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign15220_e10101, assign15220_e10101_d_n0, assign15220_e10101_d_n2, assign15220_e10101_d_n4, assign15220_e10101_d_n5, assign15220_e10101_d_n6, assign15220_e10101_d_n7, assign15220_e10101_d_n8, assign15220_e10101_d_n9, assign15220_e10101_d_n10, assign15220_e10101_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15220_e10094: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15220_e10095: f64 = (4.0 * assign15220_e10094);
        let assign15220_e10098: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15220_e10099: f64 = (assign15220_e10095 * assign15220_e10098);
        (assign15220_e10099, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15220_e10101;
        locals.var_tmf2_dn0 = assign15220_e10101_d_n0;
        locals.var_tmf2_dn2 = assign15220_e10101_d_n2;
        locals.var_tmf2_dn4 = assign15220_e10101_d_n4;
        locals.var_tmf2_dn5 = assign15220_e10101_d_n5;
        locals.var_tmf2_dn6 = assign15220_e10101_d_n6;
        locals.var_tmf2_dn7 = assign15220_e10101_d_n7;
        locals.var_tmf2_dn8 = assign15220_e10101_d_n8;
        locals.var_tmf2_dn9 = assign15220_e10101_d_n9;
        locals.var_tmf2_dn10 = assign15220_e10101_d_n10;
        locals.var_tmf2_dn13 = assign15220_e10101_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15230_e10118, assign15230_e10118_d_n0, assign15230_e10118_d_n2, assign15230_e10118_d_n4, assign15230_e10118_d_n5, assign15230_e10118_d_n6, assign15230_e10118_d_n7, assign15230_e10118_d_n8, assign15230_e10118_d_n9, assign15230_e10118_d_n10, assign15230_e10118_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let (assign15230_e10116, assign15230_e10116_d_n0, assign15230_e10116_d_n2, assign15230_e10116_d_n4, assign15230_e10116_d_n5, assign15230_e10116_d_n6, assign15230_e10116_d_n7, assign15230_e10116_d_n8, assign15230_e10116_d_n9, assign15230_e10116_d_n10, assign15230_e10116_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign15230_e10115: f64 = (-locals.var_tmf2);
                (assign15230_e10115, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign15230_e10116, assign15230_e10116_d_n0, assign15230_e10116_d_n2, assign15230_e10116_d_n4, assign15230_e10116_d_n5, assign15230_e10116_d_n6, assign15230_e10116_d_n7, assign15230_e10116_d_n8, assign15230_e10116_d_n9, assign15230_e10116_d_n10, assign15230_e10116_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15230_e10118;
        locals.var_tmf2_dn0 = assign15230_e10118_d_n0;
        locals.var_tmf2_dn2 = assign15230_e10118_d_n2;
        locals.var_tmf2_dn4 = assign15230_e10118_d_n4;
        locals.var_tmf2_dn5 = assign15230_e10118_d_n5;
        locals.var_tmf2_dn6 = assign15230_e10118_d_n6;
        locals.var_tmf2_dn7 = assign15230_e10118_d_n7;
        locals.var_tmf2_dn8 = assign15230_e10118_d_n8;
        locals.var_tmf2_dn9 = assign15230_e10118_d_n9;
        locals.var_tmf2_dn10 = assign15230_e10118_d_n10;
        locals.var_tmf2_dn13 = assign15230_e10118_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15240_e10134, assign15240_e10134_d_n0, assign15240_e10134_d_n2, assign15240_e10134_d_n4, assign15240_e10134_d_n5, assign15240_e10134_d_n6, assign15240_e10134_d_n7, assign15240_e10134_d_n8, assign15240_e10134_d_n9, assign15240_e10134_d_n10, assign15240_e10134_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15240_e10129: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15240_e10131: f64 = (assign15240_e10129 + locals.var_tmf2);
        let assign15240_e10132: f64 = (assign15240_e10131).sqrt();
        (assign15240_e10132, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign15240_e10132)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15240_e10134;
        locals.var_tmf2_dn0 = assign15240_e10134_d_n0;
        locals.var_tmf2_dn2 = assign15240_e10134_d_n2;
        locals.var_tmf2_dn4 = assign15240_e10134_d_n4;
        locals.var_tmf2_dn5 = assign15240_e10134_d_n5;
        locals.var_tmf2_dn6 = assign15240_e10134_d_n6;
        locals.var_tmf2_dn7 = assign15240_e10134_d_n7;
        locals.var_tmf2_dn8 = assign15240_e10134_d_n8;
        locals.var_tmf2_dn9 = assign15240_e10134_d_n9;
        locals.var_tmf2_dn10 = assign15240_e10134_d_n10;
        locals.var_tmf2_dn13 = assign15240_e10134_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign15250_e10151, assign15250_e10151_d_n0, assign15250_e10151_d_n2, assign15250_e10151_d_n4, assign15250_e10151_d_n5, assign15250_e10151_d_n6, assign15250_e10151_d_n7, assign15250_e10151_d_n8, assign15250_e10151_d_n9, assign15250_e10151_d_n10, assign15250_e10151_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15250_e10147: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15250_e10148: f64 = (1.0 + assign15250_e10147);
        let assign15250_e10149: f64 = (0.5 * assign15250_e10148);
        (assign15250_e10149, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign15250_e10151;
        locals.var_t0_dn0 = assign15250_e10151_d_n0;
        locals.var_t0_dn2 = assign15250_e10151_d_n2;
        locals.var_t0_dn4 = assign15250_e10151_d_n4;
        locals.var_t0_dn5 = assign15250_e10151_d_n5;
        locals.var_t0_dn6 = assign15250_e10151_d_n6;
        locals.var_t0_dn7 = assign15250_e10151_d_n7;
        locals.var_t0_dn8 = assign15250_e10151_d_n8;
        locals.var_t0_dn9 = assign15250_e10151_d_n9;
        locals.var_t0_dn10 = assign15250_e10151_d_n10;
        locals.var_t0_dn13 = assign15250_e10151_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign15260_e10170, assign15260_e10170_d_n0, assign15260_e10170_d_n2, assign15260_e10170_d_n4, assign15260_e10170_d_n5, assign15260_e10170_d_n6, assign15260_e10170_d_n7, assign15260_e10170_d_n8, assign15260_e10170_d_n9, assign15260_e10170_d_n10, assign15260_e10170_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15260_e10162: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15260_e10166: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15260_e10167: f64 = (0.5 * assign15260_e10166);
        let assign15260_e10168: f64 = (assign15260_e10162 + assign15260_e10167);
        (assign15260_e10168, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign15260_e10170;
        locals.var_rsvde_dn0 = assign15260_e10170_d_n0;
        locals.var_rsvde_dn2 = assign15260_e10170_d_n2;
        locals.var_rsvde_dn4 = assign15260_e10170_d_n4;
        locals.var_rsvde_dn5 = assign15260_e10170_d_n5;
        locals.var_rsvde_dn6 = assign15260_e10170_d_n6;
        locals.var_rsvde_dn7 = assign15260_e10170_d_n7;
        locals.var_rsvde_dn8 = assign15260_e10170_d_n8;
        locals.var_rsvde_dn9 = assign15260_e10170_d_n9;
        locals.var_rsvde_dn10 = assign15260_e10170_d_n10;
        locals.var_rsvde_dn13 = assign15260_e10170_d_n13;
        locals.var_rsvde_rv = 0.0;

        let (assign15270_e10179, assign15270_e10179_d_n0, assign15270_e10179_d_n2, assign15270_e10179_d_n4, assign15270_e10179_d_n5, assign15270_e10179_d_n6, assign15270_e10179_d_n7, assign15270_e10179_d_n8, assign15270_e10179_d_n9, assign15270_e10179_d_n10, assign15270_e10179_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign15270_e10179;
        locals.var_rdvde_dn0 = assign15270_e10179_d_n0;
        locals.var_rdvde_dn2 = assign15270_e10179_d_n2;
        locals.var_rdvde_dn4 = assign15270_e10179_d_n4;
        locals.var_rdvde_dn5 = assign15270_e10179_d_n5;
        locals.var_rdvde_dn6 = assign15270_e10179_d_n6;
        locals.var_rdvde_dn7 = assign15270_e10179_d_n7;
        locals.var_rdvde_dn8 = assign15270_e10179_d_n8;
        locals.var_rdvde_dn9 = assign15270_e10179_d_n9;
        locals.var_rdvde_dn10 = assign15270_e10179_d_n10;
        locals.var_rdvde_dn13 = assign15270_e10179_d_n13;
        locals.var_rdvde_rv = 0.0;

        let (assign15280_e10188, assign15280_e10188_d_n0, assign15280_e10188_d_n2, assign15280_e10188_d_n4, assign15280_e10188_d_n5, assign15280_e10188_d_n6, assign15280_e10188_d_n7, assign15280_e10188_d_n8, assign15280_e10188_d_n9, assign15280_e10188_d_n10, assign15280_e10188_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign15280_e10188;
        locals.var_rsvde_dn0 = assign15280_e10188_d_n0;
        locals.var_rsvde_dn2 = assign15280_e10188_d_n2;
        locals.var_rsvde_dn4 = assign15280_e10188_d_n4;
        locals.var_rsvde_dn5 = assign15280_e10188_d_n5;
        locals.var_rsvde_dn6 = assign15280_e10188_d_n6;
        locals.var_rsvde_dn7 = assign15280_e10188_d_n7;
        locals.var_rsvde_dn8 = assign15280_e10188_d_n8;
        locals.var_rsvde_dn9 = assign15280_e10188_d_n9;
        locals.var_rsvde_dn10 = assign15280_e10188_d_n10;
        locals.var_rsvde_dn13 = assign15280_e10188_d_n13;
        locals.var_rsvde_rv = 0.0;

        let (assign15290_e10195, assign15290_e10195_d_n0, assign15290_e10195_d_n2, assign15290_e10195_d_n4, assign15290_e10195_d_n5, assign15290_e10195_d_n6, assign15290_e10195_d_n7, assign15290_e10195_d_n8, assign15290_e10195_d_n9, assign15290_e10195_d_n10, assign15290_e10195_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15290_e10192: f64 = (locals.var_beta_inv).sqrt();
        let assign15290_e10193: f64 = (locals.var_costi00 * assign15290_e10192);
        (assign15290_e10193, (locals.var_costi00 * (locals.var_beta_inv_dn0 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn2 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn4 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn5 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn6 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn7 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn8 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn9 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn10 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn13 / (2.0 * assign15290_e10192))),)
    } else {
        (locals.var_costi0, locals.var_costi0_dn0, locals.var_costi0_dn2, locals.var_costi0_dn4, locals.var_costi0_dn5, locals.var_costi0_dn6, locals.var_costi0_dn7, locals.var_costi0_dn8, locals.var_costi0_dn9, locals.var_costi0_dn10, locals.var_costi0_dn13,)
    }
};
        locals.var_costi0 = assign15290_e10195;
        locals.var_costi0_dn0 = assign15290_e10195_d_n0;
        locals.var_costi0_dn2 = assign15290_e10195_d_n2;
        locals.var_costi0_dn4 = assign15290_e10195_d_n4;
        locals.var_costi0_dn5 = assign15290_e10195_d_n5;
        locals.var_costi0_dn6 = assign15290_e10195_d_n6;
        locals.var_costi0_dn7 = assign15290_e10195_d_n7;
        locals.var_costi0_dn8 = assign15290_e10195_d_n8;
        locals.var_costi0_dn9 = assign15290_e10195_d_n9;
        locals.var_costi0_dn10 = assign15290_e10195_d_n10;
        locals.var_costi0_dn13 = assign15290_e10195_d_n13;
        locals.var_costi0_rv = 0.0;

        let (assign15300_e10201, assign15300_e10201_d_n0, assign15300_e10201_d_n2, assign15300_e10201_d_n4, assign15300_e10201_d_n5, assign15300_e10201_d_n6, assign15300_e10201_d_n7, assign15300_e10201_d_n8, assign15300_e10201_d_n9, assign15300_e10201_d_n10, assign15300_e10201_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15300_e10199: f64 = (locals.var_costi0 * locals.var_costi0);
        (assign15300_e10199, ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0)), ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2)), ((locals.var_costi0_dn4 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn4)), ((locals.var_costi0_dn5 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn5)), ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6)), ((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7)), ((locals.var_costi0_dn8 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn8)), ((locals.var_costi0_dn9 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn9)), ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10)), ((locals.var_costi0_dn13 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn13)),)
    } else {
        (locals.var_costi0_p2, locals.var_costi0_p2_dn0, locals.var_costi0_p2_dn2, locals.var_costi0_p2_dn4, locals.var_costi0_p2_dn5, locals.var_costi0_p2_dn6, locals.var_costi0_p2_dn7, locals.var_costi0_p2_dn8, locals.var_costi0_p2_dn9, locals.var_costi0_p2_dn10, locals.var_costi0_p2_dn13,)
    }
};
        locals.var_costi0_p2 = assign15300_e10201;
        locals.var_costi0_p2_dn0 = assign15300_e10201_d_n0;
        locals.var_costi0_p2_dn2 = assign15300_e10201_d_n2;
        locals.var_costi0_p2_dn4 = assign15300_e10201_d_n4;
        locals.var_costi0_p2_dn5 = assign15300_e10201_d_n5;
        locals.var_costi0_p2_dn6 = assign15300_e10201_d_n6;
        locals.var_costi0_p2_dn7 = assign15300_e10201_d_n7;
        locals.var_costi0_p2_dn8 = assign15300_e10201_d_n8;
        locals.var_costi0_p2_dn9 = assign15300_e10201_d_n9;
        locals.var_costi0_p2_dn10 = assign15300_e10201_d_n10;
        locals.var_costi0_p2_dn13 = assign15300_e10201_d_n13;
        locals.var_costi0_p2_rv = 0.0;

        let (assign15310_e10209, assign15310_e10209_d_n0, assign15310_e10209_d_n2, assign15310_e10209_d_n4, assign15310_e10209_d_n5, assign15310_e10209_d_n6, assign15310_e10209_d_n7, assign15310_e10209_d_n8, assign15310_e10209_d_n9, assign15310_e10209_d_n10, assign15310_e10209_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15310_e10205: f64 = (locals.var_nin * locals.var_nin);
        let assign15310_e10207: f64 = (assign15310_e10205 * locals.var_nsti_p2);
        (assign15310_e10207, (((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2), (((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2), (((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_nsti_p2), (((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_nsti_p2), (((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2), (((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_nsti_p2), (((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_nsti_p2), (((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_nsti_p2), (((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2), (((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_nsti_p2),)
    } else {
        (locals.var_costi1, locals.var_costi1_dn0, locals.var_costi1_dn2, locals.var_costi1_dn4, locals.var_costi1_dn5, locals.var_costi1_dn6, locals.var_costi1_dn7, locals.var_costi1_dn8, locals.var_costi1_dn9, locals.var_costi1_dn10, locals.var_costi1_dn13,)
    }
};
        locals.var_costi1 = assign15310_e10209;
        locals.var_costi1_dn0 = assign15310_e10209_d_n0;
        locals.var_costi1_dn2 = assign15310_e10209_d_n2;
        locals.var_costi1_dn4 = assign15310_e10209_d_n4;
        locals.var_costi1_dn5 = assign15310_e10209_d_n5;
        locals.var_costi1_dn6 = assign15310_e10209_d_n6;
        locals.var_costi1_dn7 = assign15310_e10209_d_n7;
        locals.var_costi1_dn8 = assign15310_e10209_d_n8;
        locals.var_costi1_dn9 = assign15310_e10209_d_n9;
        locals.var_costi1_dn10 = assign15310_e10209_d_n10;
        locals.var_costi1_dn13 = assign15310_e10209_d_n13;
        locals.var_costi1_rv = 0.0;

        let (assign15320_e10217, assign15320_e10217_d_n0, assign15320_e10217_d_n2, assign15320_e10217_d_n4, assign15320_e10217_d_n5, assign15320_e10217_d_n6, assign15320_e10217_d_n7, assign15320_e10217_d_n8, assign15320_e10217_d_n9, assign15320_e10217_d_n10, assign15320_e10217_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15320_e10214: f64 = (p.p448 * locals.var_tdiff);
        let assign15320_e10215: f64 = (p.p447 + assign15320_e10214);
        (assign15320_e10215, (p.p448 * locals.var_tdiff_dn0), (p.p448 * locals.var_tdiff_dn2), (p.p448 * locals.var_tdiff_dn4), (p.p448 * locals.var_tdiff_dn5), (p.p448 * locals.var_tdiff_dn6), (p.p448 * locals.var_tdiff_dn7), (p.p448 * locals.var_tdiff_dn8), (p.p448 * locals.var_tdiff_dn9), (p.p448 * locals.var_tdiff_dn10), (p.p448 * locals.var_tdiff_dn13),)
    } else {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn13,)
    }
};
        locals.var_hbdceff = assign15320_e10217;
        locals.var_hbdceff_dn0 = assign15320_e10217_d_n0;
        locals.var_hbdceff_dn2 = assign15320_e10217_d_n2;
        locals.var_hbdceff_dn4 = assign15320_e10217_d_n4;
        locals.var_hbdceff_dn5 = assign15320_e10217_d_n5;
        locals.var_hbdceff_dn6 = assign15320_e10217_d_n6;
        locals.var_hbdceff_dn7 = assign15320_e10217_d_n7;
        locals.var_hbdceff_dn8 = assign15320_e10217_d_n8;
        locals.var_hbdceff_dn9 = assign15320_e10217_d_n9;
        locals.var_hbdceff_dn10 = assign15320_e10217_d_n10;
        locals.var_hbdceff_dn13 = assign15320_e10217_d_n13;
        locals.var_hbdceff_rv = 0.0;

        let (assign15330_e10221,) = {
    if (locals.var_guard289 != 0.0) {
        (p.p193,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15330_e10221;
        locals.var_uc_subtmp_rv = 0.0;

        let assign15360_e10234: f64 = if locals.var_uc_subtmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard324 = assign15360_e10234;
        locals.var_guard324_rv = 0.0;

        let (assign15370_e10240,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard324 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15370_e10240;
        locals.var_uc_subtmp_rv = 0.0;

        let assign15380_e10243: f64 = if locals.var_uc_subtmp > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard325 = assign15380_e10243;
        locals.var_guard325_rv = 0.0;

        let (assign15390_e10249,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard325 != 0.0)) {
        (0.005,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15390_e10249;
        locals.var_uc_subtmp_rv = 0.0;

        let assign15400_e10252: f64 = if locals.var_uc_cordrift > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard326 = assign15400_e10252;
        locals.var_guard326_rv = 0.0;

        let (assign15410_e10265, assign15410_e10265_d_n0, assign15410_e10265_d_n2, assign15410_e10265_d_n4, assign15410_e10265_d_n5, assign15410_e10265_d_n6, assign15410_e10265_d_n7, assign15410_e10265_d_n8, assign15410_e10265_d_n9, assign15410_e10265_d_n10, assign15410_e10265_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let (assign15410_e10263, assign15410_e10263_d_n0, assign15410_e10263_d_n2, assign15410_e10263_d_n4, assign15410_e10263_d_n5, assign15410_e10263_d_n6, assign15410_e10263_d_n7, assign15410_e10263_d_n8, assign15410_e10263_d_n9, assign15410_e10263_d_n10, assign15410_e10263_d_n13,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15410_e10262: f64 = (locals.var_tratio).powf(p.p416);
                (assign15410_e10262, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn0)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn2)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn4)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn5)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn6)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn7)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn8)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn9)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn10)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn13)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn13 / locals.var_tratio))) },)
            }
        };
        (assign15410_e10263, assign15410_e10263_d_n0, assign15410_e10263_d_n2, assign15410_e10263_d_n4, assign15410_e10263_d_n5, assign15410_e10263_d_n6, assign15410_e10263_d_n7, assign15410_e10263_d_n8, assign15410_e10263_d_n9, assign15410_e10263_d_n10, assign15410_e10263_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign15410_e10265;
        locals.var_t1_dn0 = assign15410_e10265_d_n0;
        locals.var_t1_dn2 = assign15410_e10265_d_n2;
        locals.var_t1_dn4 = assign15410_e10265_d_n4;
        locals.var_t1_dn5 = assign15410_e10265_d_n5;
        locals.var_t1_dn6 = assign15410_e10265_d_n6;
        locals.var_t1_dn7 = assign15410_e10265_d_n7;
        locals.var_t1_dn8 = assign15410_e10265_d_n8;
        locals.var_t1_dn9 = assign15410_e10265_d_n9;
        locals.var_t1_dn10 = assign15410_e10265_d_n10;
        locals.var_t1_dn13 = assign15410_e10265_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign15420_e10273, assign15420_e10273_d_n0, assign15420_e10273_d_n2, assign15420_e10273_d_n4, assign15420_e10273_d_n5, assign15420_e10273_d_n6, assign15420_e10273_d_n7, assign15420_e10273_d_n8, assign15420_e10273_d_n9, assign15420_e10273_d_n10, assign15420_e10273_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15420_e10271: f64 = (locals.var_mks_rdrmues / locals.var_t1);
        (assign15420_e10271, (-((locals.var_mks_rdrmues * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmues, locals.var_rrdrmues_dn0, locals.var_rrdrmues_dn2, locals.var_rrdrmues_dn4, locals.var_rrdrmues_dn5, locals.var_rrdrmues_dn6, locals.var_rrdrmues_dn7, locals.var_rrdrmues_dn8, locals.var_rrdrmues_dn9, locals.var_rrdrmues_dn10, locals.var_rrdrmues_dn13,)
    }
};
        locals.var_rrdrmues = assign15420_e10273;
        locals.var_rrdrmues_dn0 = assign15420_e10273_d_n0;
        locals.var_rrdrmues_dn2 = assign15420_e10273_d_n2;
        locals.var_rrdrmues_dn4 = assign15420_e10273_d_n4;
        locals.var_rrdrmues_dn5 = assign15420_e10273_d_n5;
        locals.var_rrdrmues_dn6 = assign15420_e10273_d_n6;
        locals.var_rrdrmues_dn7 = assign15420_e10273_d_n7;
        locals.var_rrdrmues_dn8 = assign15420_e10273_d_n8;
        locals.var_rrdrmues_dn9 = assign15420_e10273_d_n9;
        locals.var_rrdrmues_dn10 = assign15420_e10273_d_n10;
        locals.var_rrdrmues_dn13 = assign15420_e10273_d_n13;
        locals.var_rrdrmues_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15430_e10295, assign15430_e10295_d_n0, assign15430_e10295_d_n2, assign15430_e10295_d_n4, assign15430_e10295_d_n5, assign15430_e10295_d_n6, assign15430_e10295_d_n7, assign15430_e10295_d_n8, assign15430_e10295_d_n9, assign15430_e10295_d_n10, assign15430_e10295_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15430_e10280: f64 = (0.4 * locals.var_tratio);
        let assign15430_e10281: f64 = (1.8 + assign15430_e10280);
        let assign15430_e10284: f64 = (0.1 * locals.var_tratio);
        let assign15430_e10286: f64 = (assign15430_e10284 * locals.var_tratio);
        let assign15430_e10287: f64 = (assign15430_e10281 + assign15430_e10286);
        let assign15430_e10291: f64 = (1.0 - locals.var_tratio);
        let assign15430_e10292: f64 = (p.p418 * assign15430_e10291);
        let assign15430_e10293: f64 = (assign15430_e10287 - assign15430_e10292);
        (assign15430_e10293, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn0))) - (p.p418 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn2))) - (p.p418 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn4))) - (p.p418 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn5))) - (p.p418 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn6))) - (p.p418 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn7))) - (p.p418 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn8))) - (p.p418 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn9))) - (p.p418 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn10))) - (p.p418 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn13))) - (p.p418 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign15430_e10295;
        locals.var_t0_dn0 = assign15430_e10295_d_n0;
        locals.var_t0_dn2 = assign15430_e10295_d_n2;
        locals.var_t0_dn4 = assign15430_e10295_d_n4;
        locals.var_t0_dn5 = assign15430_e10295_d_n5;
        locals.var_t0_dn6 = assign15430_e10295_d_n6;
        locals.var_t0_dn7 = assign15430_e10295_d_n7;
        locals.var_t0_dn8 = assign15430_e10295_d_n8;
        locals.var_t0_dn9 = assign15430_e10295_d_n9;
        locals.var_t0_dn10 = assign15430_e10295_d_n10;
        locals.var_t0_dn13 = assign15430_e10295_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign15440_e10303, assign15440_e10303_d_n0, assign15440_e10303_d_n2, assign15440_e10303_d_n4, assign15440_e10303_d_n5, assign15440_e10303_d_n6, assign15440_e10303_d_n7, assign15440_e10303_d_n8, assign15440_e10303_d_n9, assign15440_e10303_d_n10, assign15440_e10303_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15440_e10301: f64 = (locals.var_mks_rdrvmaxs / locals.var_t0);
        (assign15440_e10301, (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmaxs, locals.var_rrdrvmaxs_dn0, locals.var_rrdrvmaxs_dn2, locals.var_rrdrvmaxs_dn4, locals.var_rrdrvmaxs_dn5, locals.var_rrdrvmaxs_dn6, locals.var_rrdrvmaxs_dn7, locals.var_rrdrvmaxs_dn8, locals.var_rrdrvmaxs_dn9, locals.var_rrdrvmaxs_dn10, locals.var_rrdrvmaxs_dn13,)
    }
};
        locals.var_rrdrvmaxs = assign15440_e10303;
        locals.var_rrdrvmaxs_dn0 = assign15440_e10303_d_n0;
        locals.var_rrdrvmaxs_dn2 = assign15440_e10303_d_n2;
        locals.var_rrdrvmaxs_dn4 = assign15440_e10303_d_n4;
        locals.var_rrdrvmaxs_dn5 = assign15440_e10303_d_n5;
        locals.var_rrdrvmaxs_dn6 = assign15440_e10303_d_n6;
        locals.var_rrdrvmaxs_dn7 = assign15440_e10303_d_n7;
        locals.var_rrdrvmaxs_dn8 = assign15440_e10303_d_n8;
        locals.var_rrdrvmaxs_dn9 = assign15440_e10303_d_n9;
        locals.var_rrdrvmaxs_dn10 = assign15440_e10303_d_n10;
        locals.var_rrdrvmaxs_dn13 = assign15440_e10303_d_n13;
        locals.var_rrdrvmaxs_rv = 0.0;

        let (assign15450_e10315, assign15450_e10315_d_n0, assign15450_e10315_d_n2, assign15450_e10315_d_n4, assign15450_e10315_d_n5, assign15450_e10315_d_n6, assign15450_e10315_d_n7, assign15450_e10315_d_n8, assign15450_e10315_d_n9, assign15450_e10315_d_n10, assign15450_e10315_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15450_e10311: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign15450_e10312: f64 = (p.p439 * assign15450_e10311);
        let assign15450_e10313: f64 = (locals.var_uc_rdrbb_s + assign15450_e10312);
        (assign15450_e10313, (locals.var_uc_rdrbb_s_dn0 + (p.p439 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_s_dn2 + (p.p439 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_s_dn4 + (p.p439 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_s_dn5 + (p.p439 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_s_dn6 + (p.p439 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_s_dn7 + (p.p439 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_s_dn8 + (p.p439 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_s_dn9 + (p.p439 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_s_dn10 + (p.p439 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_s_dn13 + (p.p439 * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_uc_rdrbb_s, locals.var_uc_rdrbb_s_dn0, locals.var_uc_rdrbb_s_dn2, locals.var_uc_rdrbb_s_dn4, locals.var_uc_rdrbb_s_dn5, locals.var_uc_rdrbb_s_dn6, locals.var_uc_rdrbb_s_dn7, locals.var_uc_rdrbb_s_dn8, locals.var_uc_rdrbb_s_dn9, locals.var_uc_rdrbb_s_dn10, locals.var_uc_rdrbb_s_dn13,)
    }
};
        locals.var_uc_rdrbb_s = assign15450_e10315;
        locals.var_uc_rdrbb_s_dn0 = assign15450_e10315_d_n0;
        locals.var_uc_rdrbb_s_dn2 = assign15450_e10315_d_n2;
        locals.var_uc_rdrbb_s_dn4 = assign15450_e10315_d_n4;
        locals.var_uc_rdrbb_s_dn5 = assign15450_e10315_d_n5;
        locals.var_uc_rdrbb_s_dn6 = assign15450_e10315_d_n6;
        locals.var_uc_rdrbb_s_dn7 = assign15450_e10315_d_n7;
        locals.var_uc_rdrbb_s_dn8 = assign15450_e10315_d_n8;
        locals.var_uc_rdrbb_s_dn9 = assign15450_e10315_d_n9;
        locals.var_uc_rdrbb_s_dn10 = assign15450_e10315_d_n10;
        locals.var_uc_rdrbb_s_dn13 = assign15450_e10315_d_n13;
        locals.var_uc_rdrbb_s_rv = 0.0;

        let (assign15460_e10328, assign15460_e10328_d_n0, assign15460_e10328_d_n2, assign15460_e10328_d_n4, assign15460_e10328_d_n5, assign15460_e10328_d_n6, assign15460_e10328_d_n7, assign15460_e10328_d_n8, assign15460_e10328_d_n9, assign15460_e10328_d_n10, assign15460_e10328_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let (assign15460_e10326, assign15460_e10326_d_n0, assign15460_e10326_d_n2, assign15460_e10326_d_n4, assign15460_e10326_d_n5, assign15460_e10326_d_n6, assign15460_e10326_d_n7, assign15460_e10326_d_n8, assign15460_e10326_d_n9, assign15460_e10326_d_n10, assign15460_e10326_d_n13,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15460_e10325: f64 = (locals.var_tratio).powf(p.p415);
                (assign15460_e10325, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn0)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn2)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn4)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn5)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn6)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn7)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn8)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn9)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn10)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn13)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn13 / locals.var_tratio))) },)
            }
        };
        (assign15460_e10326, assign15460_e10326_d_n0, assign15460_e10326_d_n2, assign15460_e10326_d_n4, assign15460_e10326_d_n5, assign15460_e10326_d_n6, assign15460_e10326_d_n7, assign15460_e10326_d_n8, assign15460_e10326_d_n9, assign15460_e10326_d_n10, assign15460_e10326_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign15460_e10328;
        locals.var_t1_dn0 = assign15460_e10328_d_n0;
        locals.var_t1_dn2 = assign15460_e10328_d_n2;
        locals.var_t1_dn4 = assign15460_e10328_d_n4;
        locals.var_t1_dn5 = assign15460_e10328_d_n5;
        locals.var_t1_dn6 = assign15460_e10328_d_n6;
        locals.var_t1_dn7 = assign15460_e10328_d_n7;
        locals.var_t1_dn8 = assign15460_e10328_d_n8;
        locals.var_t1_dn9 = assign15460_e10328_d_n9;
        locals.var_t1_dn10 = assign15460_e10328_d_n10;
        locals.var_t1_dn13 = assign15460_e10328_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign15470_e10336, assign15470_e10336_d_n0, assign15470_e10336_d_n2, assign15470_e10336_d_n4, assign15470_e10336_d_n5, assign15470_e10336_d_n6, assign15470_e10336_d_n7, assign15470_e10336_d_n8, assign15470_e10336_d_n9, assign15470_e10336_d_n10, assign15470_e10336_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15470_e10334: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign15470_e10334, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmue, locals.var_rrdrmue_dn0, locals.var_rrdrmue_dn2, locals.var_rrdrmue_dn4, locals.var_rrdrmue_dn5, locals.var_rrdrmue_dn6, locals.var_rrdrmue_dn7, locals.var_rrdrmue_dn8, locals.var_rrdrmue_dn9, locals.var_rrdrmue_dn10, locals.var_rrdrmue_dn13,)
    }
};
        locals.var_rrdrmue = assign15470_e10336;
        locals.var_rrdrmue_dn0 = assign15470_e10336_d_n0;
        locals.var_rrdrmue_dn2 = assign15470_e10336_d_n2;
        locals.var_rrdrmue_dn4 = assign15470_e10336_d_n4;
        locals.var_rrdrmue_dn5 = assign15470_e10336_d_n5;
        locals.var_rrdrmue_dn6 = assign15470_e10336_d_n6;
        locals.var_rrdrmue_dn7 = assign15470_e10336_d_n7;
        locals.var_rrdrmue_dn8 = assign15470_e10336_d_n8;
        locals.var_rrdrmue_dn9 = assign15470_e10336_d_n9;
        locals.var_rrdrmue_dn10 = assign15470_e10336_d_n10;
        locals.var_rrdrmue_dn13 = assign15470_e10336_d_n13;
        locals.var_rrdrmue_rv = 0.0;

        let (assign15480_e10358, assign15480_e10358_d_n0, assign15480_e10358_d_n2, assign15480_e10358_d_n4, assign15480_e10358_d_n5, assign15480_e10358_d_n6, assign15480_e10358_d_n7, assign15480_e10358_d_n8, assign15480_e10358_d_n9, assign15480_e10358_d_n10, assign15480_e10358_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15480_e10343: f64 = (0.4 * locals.var_tratio);
        let assign15480_e10344: f64 = (1.8 + assign15480_e10343);
        let assign15480_e10347: f64 = (0.1 * locals.var_tratio);
        let assign15480_e10349: f64 = (assign15480_e10347 * locals.var_tratio);
        let assign15480_e10350: f64 = (assign15480_e10344 + assign15480_e10349);
        let assign15480_e10354: f64 = (1.0 - locals.var_tratio);
        let assign15480_e10355: f64 = (p.p417 * assign15480_e10354);
        let assign15480_e10356: f64 = (assign15480_e10350 - assign15480_e10355);
        (assign15480_e10356, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn0))) - (p.p417 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn2))) - (p.p417 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn4))) - (p.p417 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn5))) - (p.p417 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn6))) - (p.p417 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn7))) - (p.p417 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn8))) - (p.p417 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn9))) - (p.p417 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn10))) - (p.p417 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn13))) - (p.p417 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign15480_e10358;
        locals.var_t0_dn0 = assign15480_e10358_d_n0;
        locals.var_t0_dn2 = assign15480_e10358_d_n2;
        locals.var_t0_dn4 = assign15480_e10358_d_n4;
        locals.var_t0_dn5 = assign15480_e10358_d_n5;
        locals.var_t0_dn6 = assign15480_e10358_d_n6;
        locals.var_t0_dn7 = assign15480_e10358_d_n7;
        locals.var_t0_dn8 = assign15480_e10358_d_n8;
        locals.var_t0_dn9 = assign15480_e10358_d_n9;
        locals.var_t0_dn10 = assign15480_e10358_d_n10;
        locals.var_t0_dn13 = assign15480_e10358_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign15490_e10366, assign15490_e10366_d_n0, assign15490_e10366_d_n2, assign15490_e10366_d_n4, assign15490_e10366_d_n5, assign15490_e10366_d_n6, assign15490_e10366_d_n7, assign15490_e10366_d_n8, assign15490_e10366_d_n9, assign15490_e10366_d_n10, assign15490_e10366_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15490_e10364: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign15490_e10364, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmax, locals.var_rrdrvmax_dn0, locals.var_rrdrvmax_dn2, locals.var_rrdrvmax_dn4, locals.var_rrdrvmax_dn5, locals.var_rrdrvmax_dn6, locals.var_rrdrvmax_dn7, locals.var_rrdrvmax_dn8, locals.var_rrdrvmax_dn9, locals.var_rrdrvmax_dn10, locals.var_rrdrvmax_dn13,)
    }
};
        locals.var_rrdrvmax = assign15490_e10366;
        locals.var_rrdrvmax_dn0 = assign15490_e10366_d_n0;
        locals.var_rrdrvmax_dn2 = assign15490_e10366_d_n2;
        locals.var_rrdrvmax_dn4 = assign15490_e10366_d_n4;
        locals.var_rrdrvmax_dn5 = assign15490_e10366_d_n5;
        locals.var_rrdrvmax_dn6 = assign15490_e10366_d_n6;
        locals.var_rrdrvmax_dn7 = assign15490_e10366_d_n7;
        locals.var_rrdrvmax_dn8 = assign15490_e10366_d_n8;
        locals.var_rrdrvmax_dn9 = assign15490_e10366_d_n9;
        locals.var_rrdrvmax_dn10 = assign15490_e10366_d_n10;
        locals.var_rrdrvmax_dn13 = assign15490_e10366_d_n13;
        locals.var_rrdrvmax_rv = 0.0;

        let (assign15500_e10378, assign15500_e10378_d_n0, assign15500_e10378_d_n2, assign15500_e10378_d_n4, assign15500_e10378_d_n5, assign15500_e10378_d_n6, assign15500_e10378_d_n7, assign15500_e10378_d_n8, assign15500_e10378_d_n9, assign15500_e10378_d_n10, assign15500_e10378_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15500_e10374: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign15500_e10375: f64 = (p.p438 * assign15500_e10374);
        let assign15500_e10376: f64 = (locals.var_uc_rdrbb + assign15500_e10375);
        (assign15500_e10376, (locals.var_uc_rdrbb_dn0 + (p.p438 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_dn2 + (p.p438 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_dn4 + (p.p438 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_dn5 + (p.p438 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_dn6 + (p.p438 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_dn7 + (p.p438 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_dn8 + (p.p438 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_dn9 + (p.p438 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_dn10 + (p.p438 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_dn13 + (p.p438 * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn13,)
    }
};
        locals.var_uc_rdrbb = assign15500_e10378;
        locals.var_uc_rdrbb_dn0 = assign15500_e10378_d_n0;
        locals.var_uc_rdrbb_dn2 = assign15500_e10378_d_n2;
        locals.var_uc_rdrbb_dn4 = assign15500_e10378_d_n4;
        locals.var_uc_rdrbb_dn5 = assign15500_e10378_d_n5;
        locals.var_uc_rdrbb_dn6 = assign15500_e10378_d_n6;
        locals.var_uc_rdrbb_dn7 = assign15500_e10378_d_n7;
        locals.var_uc_rdrbb_dn8 = assign15500_e10378_d_n8;
        locals.var_uc_rdrbb_dn9 = assign15500_e10378_d_n9;
        locals.var_uc_rdrbb_dn10 = assign15500_e10378_d_n10;
        locals.var_uc_rdrbb_dn13 = assign15500_e10378_d_n13;
        locals.var_uc_rdrbb_rv = 0.0;

        let assign15520_e10386: f64 = if locals.var_uc_rdrbb < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard328 = assign15520_e10386;
        locals.var_guard328_rv = 0.0;

        let (assign15530_e10394, assign15530_e10394_d_n0, assign15530_e10394_d_n2, assign15530_e10394_d_n4, assign15530_e10394_d_n5, assign15530_e10394_d_n6, assign15530_e10394_d_n7, assign15530_e10394_d_n8, assign15530_e10394_d_n9, assign15530_e10394_d_n10, assign15530_e10394_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) && (locals.var_guard328 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn13,)
    }
};
        locals.var_uc_rdrbb = assign15530_e10394;
        locals.var_uc_rdrbb_dn0 = assign15530_e10394_d_n0;
        locals.var_uc_rdrbb_dn2 = assign15530_e10394_d_n2;
        locals.var_uc_rdrbb_dn4 = assign15530_e10394_d_n4;
        locals.var_uc_rdrbb_dn5 = assign15530_e10394_d_n5;
        locals.var_uc_rdrbb_dn6 = assign15530_e10394_d_n6;
        locals.var_uc_rdrbb_dn7 = assign15530_e10394_d_n7;
        locals.var_uc_rdrbb_dn8 = assign15530_e10394_d_n8;
        locals.var_uc_rdrbb_dn9 = assign15530_e10394_d_n9;
        locals.var_uc_rdrbb_dn10 = assign15530_e10394_d_n10;
        locals.var_uc_rdrbb_dn13 = assign15530_e10394_d_n13;
        locals.var_uc_rdrbb_rv = 0.0;

        let (assign15540_e10400, assign15540_e10400_d_n0, assign15540_e10400_d_n2, assign15540_e10400_d_n4, assign15540_e10400_d_n5, assign15540_e10400_d_n6, assign15540_e10400_d_n7, assign15540_e10400_d_n8, assign15540_e10400_d_n9, assign15540_e10400_d_n10, assign15540_e10400_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15540_e10398: f64 = (locals.var_tratio * locals.var_tratio);
        (assign15540_e10398, ((locals.var_tratio_dn0 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn0)), ((locals.var_tratio_dn2 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn2)), ((locals.var_tratio_dn4 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn4)), ((locals.var_tratio_dn5 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn5)), ((locals.var_tratio_dn6 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn6)), ((locals.var_tratio_dn7 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn7)), ((locals.var_tratio_dn8 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn8)), ((locals.var_tratio_dn9 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn9)), ((locals.var_tratio_dn10 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn10)), ((locals.var_tratio_dn13 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign15540_e10400;
        locals.var_t0_dn0 = assign15540_e10400_d_n0;
        locals.var_t0_dn2 = assign15540_e10400_d_n2;
        locals.var_t0_dn4 = assign15540_e10400_d_n4;
        locals.var_t0_dn5 = assign15540_e10400_d_n5;
        locals.var_t0_dn6 = assign15540_e10400_d_n6;
        locals.var_t0_dn7 = assign15540_e10400_d_n7;
        locals.var_t0_dn8 = assign15540_e10400_d_n8;
        locals.var_t0_dn9 = assign15540_e10400_d_n9;
        locals.var_t0_dn10 = assign15540_e10400_d_n10;
        locals.var_t0_dn13 = assign15540_e10400_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign15550_e10419, assign15550_e10419_d_n0, assign15550_e10419_d_n2, assign15550_e10419_d_n4, assign15550_e10419_d_n5, assign15550_e10419_d_n6, assign15550_e10419_d_n7, assign15550_e10419_d_n8, assign15550_e10419_d_n9, assign15550_e10419_d_n10, assign15550_e10419_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15550_e10405: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15550_e10408: f64 = (locals.var_eg * locals.var_beta);
        let assign15550_e10409: f64 = (assign15550_e10405 - assign15550_e10408);
        let assign15550_e10412: f64 = (p.p499 * locals.var_log_tratio);
        let assign15550_e10413: f64 = (assign15550_e10409 + assign15550_e10412);
        let assign15550_e10415: f64 = (assign15550_e10413 / locals.var_uc_njd);
        let assign15550_e10416: f64 = (assign15550_e10415).exp();
        let assign15550_e10417: f64 = (locals.var_uc_js0d * assign15550_e10416);
        (assign15550_e10417, (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn13,)
    }
};
        locals.var_js = assign15550_e10419;
        locals.var_js_dn0 = assign15550_e10419_d_n0;
        locals.var_js_dn2 = assign15550_e10419_d_n2;
        locals.var_js_dn4 = assign15550_e10419_d_n4;
        locals.var_js_dn5 = assign15550_e10419_d_n5;
        locals.var_js_dn6 = assign15550_e10419_d_n6;
        locals.var_js_dn7 = assign15550_e10419_d_n7;
        locals.var_js_dn8 = assign15550_e10419_d_n8;
        locals.var_js_dn9 = assign15550_e10419_d_n9;
        locals.var_js_dn10 = assign15550_e10419_d_n10;
        locals.var_js_dn13 = assign15550_e10419_d_n13;
        locals.var_js_rv = 0.0;

        let (assign15560_e10438, assign15560_e10438_d_n0, assign15560_e10438_d_n2, assign15560_e10438_d_n4, assign15560_e10438_d_n5, assign15560_e10438_d_n6, assign15560_e10438_d_n7, assign15560_e10438_d_n8, assign15560_e10438_d_n9, assign15560_e10438_d_n10, assign15560_e10438_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15560_e10424: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15560_e10427: f64 = (locals.var_eg * locals.var_beta);
        let assign15560_e10428: f64 = (assign15560_e10424 - assign15560_e10427);
        let assign15560_e10431: f64 = (p.p499 * locals.var_log_tratio);
        let assign15560_e10432: f64 = (assign15560_e10428 + assign15560_e10431);
        let assign15560_e10434: f64 = (assign15560_e10432 / p.p497);
        let assign15560_e10435: f64 = (assign15560_e10434).exp();
        let assign15560_e10436: f64 = (locals.var_uc_js0swd * assign15560_e10435);
        (assign15560_e10436, (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / p.p497))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn13,)
    }
};
        locals.var_jssw = assign15560_e10438;
        locals.var_jssw_dn0 = assign15560_e10438_d_n0;
        locals.var_jssw_dn2 = assign15560_e10438_d_n2;
        locals.var_jssw_dn4 = assign15560_e10438_d_n4;
        locals.var_jssw_dn5 = assign15560_e10438_d_n5;
        locals.var_jssw_dn6 = assign15560_e10438_d_n6;
        locals.var_jssw_dn7 = assign15560_e10438_d_n7;
        locals.var_jssw_dn8 = assign15560_e10438_d_n8;
        locals.var_jssw_dn9 = assign15560_e10438_d_n9;
        locals.var_jssw_dn10 = assign15560_e10438_d_n10;
        locals.var_jssw_dn13 = assign15560_e10438_d_n13;
        locals.var_jssw_rv = 0.0;

        let (assign15570_e10457, assign15570_e10457_d_n0, assign15570_e10457_d_n2, assign15570_e10457_d_n4, assign15570_e10457_d_n5, assign15570_e10457_d_n6, assign15570_e10457_d_n7, assign15570_e10457_d_n8, assign15570_e10457_d_n9, assign15570_e10457_d_n10, assign15570_e10457_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15570_e10443: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15570_e10446: f64 = (locals.var_eg * locals.var_beta);
        let assign15570_e10447: f64 = (assign15570_e10443 - assign15570_e10446);
        let assign15570_e10450: f64 = (p.p499 * locals.var_log_tratio);
        let assign15570_e10451: f64 = (assign15570_e10447 + assign15570_e10450);
        let assign15570_e10453: f64 = (assign15570_e10451 / p.p498);
        let assign15570_e10454: f64 = (assign15570_e10453).exp();
        let assign15570_e10455: f64 = (p.p495 * assign15570_e10454);
        (assign15570_e10455, (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / p.p498))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn13,)
    }
};
        locals.var_jsswg = assign15570_e10457;
        locals.var_jsswg_dn0 = assign15570_e10457_d_n0;
        locals.var_jsswg_dn2 = assign15570_e10457_d_n2;
        locals.var_jsswg_dn4 = assign15570_e10457_d_n4;
        locals.var_jsswg_dn5 = assign15570_e10457_d_n5;
        locals.var_jsswg_dn6 = assign15570_e10457_d_n6;
        locals.var_jsswg_dn7 = assign15570_e10457_d_n7;
        locals.var_jsswg_dn8 = assign15570_e10457_d_n8;
        locals.var_jsswg_dn9 = assign15570_e10457_d_n9;
        locals.var_jsswg_dn10 = assign15570_e10457_d_n10;
        locals.var_jsswg_dn13 = assign15570_e10457_d_n13;
        locals.var_jsswg_rv = 0.0;

        let (assign15580_e10476, assign15580_e10476_d_n0, assign15580_e10476_d_n2, assign15580_e10476_d_n4, assign15580_e10476_d_n5, assign15580_e10476_d_n6, assign15580_e10476_d_n7, assign15580_e10476_d_n8, assign15580_e10476_d_n9, assign15580_e10476_d_n10, assign15580_e10476_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15580_e10462: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15580_e10465: f64 = (locals.var_eg * locals.var_beta);
        let assign15580_e10466: f64 = (assign15580_e10462 - assign15580_e10465);
        let assign15580_e10469: f64 = (p.p509 * locals.var_log_tratio);
        let assign15580_e10470: f64 = (assign15580_e10466 + assign15580_e10469);
        let assign15580_e10472: f64 = (assign15580_e10470 / locals.var_uc_njd);
        let assign15580_e10473: f64 = (assign15580_e10472).exp();
        let assign15580_e10474: f64 = (locals.var_uc_js0d * assign15580_e10473);
        (assign15580_e10474, (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn13,)
    }
};
        locals.var_js2 = assign15580_e10476;
        locals.var_js2_dn0 = assign15580_e10476_d_n0;
        locals.var_js2_dn2 = assign15580_e10476_d_n2;
        locals.var_js2_dn4 = assign15580_e10476_d_n4;
        locals.var_js2_dn5 = assign15580_e10476_d_n5;
        locals.var_js2_dn6 = assign15580_e10476_d_n6;
        locals.var_js2_dn7 = assign15580_e10476_d_n7;
        locals.var_js2_dn8 = assign15580_e10476_d_n8;
        locals.var_js2_dn9 = assign15580_e10476_d_n9;
        locals.var_js2_dn10 = assign15580_e10476_d_n10;
        locals.var_js2_dn13 = assign15580_e10476_d_n13;
        locals.var_js2_rv = 0.0;

        let (assign15590_e10495, assign15590_e10495_d_n0, assign15590_e10495_d_n2, assign15590_e10495_d_n4, assign15590_e10495_d_n5, assign15590_e10495_d_n6, assign15590_e10495_d_n7, assign15590_e10495_d_n8, assign15590_e10495_d_n9, assign15590_e10495_d_n10, assign15590_e10495_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15590_e10481: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15590_e10484: f64 = (locals.var_eg * locals.var_beta);
        let assign15590_e10485: f64 = (assign15590_e10481 - assign15590_e10484);
        let assign15590_e10488: f64 = (p.p509 * locals.var_log_tratio);
        let assign15590_e10489: f64 = (assign15590_e10485 + assign15590_e10488);
        let assign15590_e10491: f64 = (assign15590_e10489 / p.p497);
        let assign15590_e10492: f64 = (assign15590_e10491).exp();
        let assign15590_e10493: f64 = (locals.var_uc_js0swd * assign15590_e10492);
        (assign15590_e10493, (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / p.p497))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn13,)
    }
};
        locals.var_jssw2 = assign15590_e10495;
        locals.var_jssw2_dn0 = assign15590_e10495_d_n0;
        locals.var_jssw2_dn2 = assign15590_e10495_d_n2;
        locals.var_jssw2_dn4 = assign15590_e10495_d_n4;
        locals.var_jssw2_dn5 = assign15590_e10495_d_n5;
        locals.var_jssw2_dn6 = assign15590_e10495_d_n6;
        locals.var_jssw2_dn7 = assign15590_e10495_d_n7;
        locals.var_jssw2_dn8 = assign15590_e10495_d_n8;
        locals.var_jssw2_dn9 = assign15590_e10495_d_n9;
        locals.var_jssw2_dn10 = assign15590_e10495_d_n10;
        locals.var_jssw2_dn13 = assign15590_e10495_d_n13;
        locals.var_jssw2_rv = 0.0;

        let (assign15600_e10514, assign15600_e10514_d_n0, assign15600_e10514_d_n2, assign15600_e10514_d_n4, assign15600_e10514_d_n5, assign15600_e10514_d_n6, assign15600_e10514_d_n7, assign15600_e10514_d_n8, assign15600_e10514_d_n9, assign15600_e10514_d_n10, assign15600_e10514_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15600_e10500: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15600_e10503: f64 = (locals.var_eg * locals.var_beta);
        let assign15600_e10504: f64 = (assign15600_e10500 - assign15600_e10503);
        let assign15600_e10507: f64 = (p.p509 * locals.var_log_tratio);
        let assign15600_e10508: f64 = (assign15600_e10504 + assign15600_e10507);
        let assign15600_e10510: f64 = (assign15600_e10508 / p.p498);
        let assign15600_e10511: f64 = (assign15600_e10510).exp();
        let assign15600_e10512: f64 = (p.p495 * assign15600_e10511);
        (assign15600_e10512, (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / p.p498))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn13,)
    }
};
        locals.var_jsswg2 = assign15600_e10514;
        locals.var_jsswg2_dn0 = assign15600_e10514_d_n0;
        locals.var_jsswg2_dn2 = assign15600_e10514_d_n2;
        locals.var_jsswg2_dn4 = assign15600_e10514_d_n4;
        locals.var_jsswg2_dn5 = assign15600_e10514_d_n5;
        locals.var_jsswg2_dn6 = assign15600_e10514_d_n6;
        locals.var_jsswg2_dn7 = assign15600_e10514_d_n7;
        locals.var_jsswg2_dn8 = assign15600_e10514_d_n8;
        locals.var_jsswg2_dn9 = assign15600_e10514_d_n9;
        locals.var_jsswg2_dn10 = assign15600_e10514_d_n10;
        locals.var_jsswg2_dn13 = assign15600_e10514_d_n13;
        locals.var_jsswg2_rv = 0.0;

        let assign15610_e10517: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard329 = assign15610_e10517;
        locals.var_guard329_rv = 0.0;

        let assign15620_e10520: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard330 = assign15620_e10520;
        locals.var_guard330_rv = 0.0;

        let (assign15630_e10530, assign15630_e10530_d_n0, assign15630_e10530_d_n2, assign15630_e10530_d_n4, assign15630_e10530_d_n5, assign15630_e10530_d_n6, assign15630_e10530_d_n7, assign15630_e10530_d_n8, assign15630_e10530_d_n9, assign15630_e10530_d_n10, assign15630_e10530_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15630_e10528: f64 = (p.p13 * locals.var_js);
        (assign15630_e10528, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign15630_e10530;
        locals.var_isbd_btm_dn0 = assign15630_e10530_d_n0;
        locals.var_isbd_btm_dn2 = assign15630_e10530_d_n2;
        locals.var_isbd_btm_dn4 = assign15630_e10530_d_n4;
        locals.var_isbd_btm_dn5 = assign15630_e10530_d_n5;
        locals.var_isbd_btm_dn6 = assign15630_e10530_d_n6;
        locals.var_isbd_btm_dn7 = assign15630_e10530_d_n7;
        locals.var_isbd_btm_dn8 = assign15630_e10530_d_n8;
        locals.var_isbd_btm_dn9 = assign15630_e10530_d_n9;
        locals.var_isbd_btm_dn10 = assign15630_e10530_d_n10;
        locals.var_isbd_btm_dn13 = assign15630_e10530_d_n13;
        locals.var_isbd_btm_rv = 0.0;

        let (assign15640_e10540, assign15640_e10540_d_n0, assign15640_e10540_d_n2, assign15640_e10540_d_n4, assign15640_e10540_d_n5, assign15640_e10540_d_n6, assign15640_e10540_d_n7, assign15640_e10540_d_n8, assign15640_e10540_d_n9, assign15640_e10540_d_n10, assign15640_e10540_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15640_e10538: f64 = (p.p13 * locals.var_js2);
        (assign15640_e10538, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign15640_e10540;
        locals.var_isbd2_btm_dn0 = assign15640_e10540_d_n0;
        locals.var_isbd2_btm_dn2 = assign15640_e10540_d_n2;
        locals.var_isbd2_btm_dn4 = assign15640_e10540_d_n4;
        locals.var_isbd2_btm_dn5 = assign15640_e10540_d_n5;
        locals.var_isbd2_btm_dn6 = assign15640_e10540_d_n6;
        locals.var_isbd2_btm_dn7 = assign15640_e10540_d_n7;
        locals.var_isbd2_btm_dn8 = assign15640_e10540_d_n8;
        locals.var_isbd2_btm_dn9 = assign15640_e10540_d_n9;
        locals.var_isbd2_btm_dn10 = assign15640_e10540_d_n10;
        locals.var_isbd2_btm_dn13 = assign15640_e10540_d_n13;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign15650_e10552, assign15650_e10552_d_n0, assign15650_e10552_d_n2, assign15650_e10552_d_n4, assign15650_e10552_d_n5, assign15650_e10552_d_n6, assign15650_e10552_d_n7, assign15650_e10552_d_n8, assign15650_e10552_d_n9, assign15650_e10552_d_n10, assign15650_e10552_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15650_e10548: f64 = (p.p15 - locals.var_weff_nf);
        let assign15650_e10550: f64 = (assign15650_e10548 * locals.var_jssw);
        (assign15650_e10550, (assign15650_e10548 * locals.var_jssw_dn0), (assign15650_e10548 * locals.var_jssw_dn2), (assign15650_e10548 * locals.var_jssw_dn4), (assign15650_e10548 * locals.var_jssw_dn5), (assign15650_e10548 * locals.var_jssw_dn6), (assign15650_e10548 * locals.var_jssw_dn7), (assign15650_e10548 * locals.var_jssw_dn8), (assign15650_e10548 * locals.var_jssw_dn9), (assign15650_e10548 * locals.var_jssw_dn10), (assign15650_e10548 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign15650_e10552;
        locals.var_isbd_sws_dn0 = assign15650_e10552_d_n0;
        locals.var_isbd_sws_dn2 = assign15650_e10552_d_n2;
        locals.var_isbd_sws_dn4 = assign15650_e10552_d_n4;
        locals.var_isbd_sws_dn5 = assign15650_e10552_d_n5;
        locals.var_isbd_sws_dn6 = assign15650_e10552_d_n6;
        locals.var_isbd_sws_dn7 = assign15650_e10552_d_n7;
        locals.var_isbd_sws_dn8 = assign15650_e10552_d_n8;
        locals.var_isbd_sws_dn9 = assign15650_e10552_d_n9;
        locals.var_isbd_sws_dn10 = assign15650_e10552_d_n10;
        locals.var_isbd_sws_dn13 = assign15650_e10552_d_n13;
        locals.var_isbd_sws_rv = 0.0;

        let (assign15660_e10564, assign15660_e10564_d_n0, assign15660_e10564_d_n2, assign15660_e10564_d_n4, assign15660_e10564_d_n5, assign15660_e10564_d_n6, assign15660_e10564_d_n7, assign15660_e10564_d_n8, assign15660_e10564_d_n9, assign15660_e10564_d_n10, assign15660_e10564_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15660_e10560: f64 = (p.p15 - locals.var_weff_nf);
        let assign15660_e10562: f64 = (assign15660_e10560 * locals.var_jssw2);
        (assign15660_e10562, (assign15660_e10560 * locals.var_jssw2_dn0), (assign15660_e10560 * locals.var_jssw2_dn2), (assign15660_e10560 * locals.var_jssw2_dn4), (assign15660_e10560 * locals.var_jssw2_dn5), (assign15660_e10560 * locals.var_jssw2_dn6), (assign15660_e10560 * locals.var_jssw2_dn7), (assign15660_e10560 * locals.var_jssw2_dn8), (assign15660_e10560 * locals.var_jssw2_dn9), (assign15660_e10560 * locals.var_jssw2_dn10), (assign15660_e10560 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign15660_e10564;
        locals.var_isbd2_sws_dn0 = assign15660_e10564_d_n0;
        locals.var_isbd2_sws_dn2 = assign15660_e10564_d_n2;
        locals.var_isbd2_sws_dn4 = assign15660_e10564_d_n4;
        locals.var_isbd2_sws_dn5 = assign15660_e10564_d_n5;
        locals.var_isbd2_sws_dn6 = assign15660_e10564_d_n6;
        locals.var_isbd2_sws_dn7 = assign15660_e10564_d_n7;
        locals.var_isbd2_sws_dn8 = assign15660_e10564_d_n8;
        locals.var_isbd2_sws_dn9 = assign15660_e10564_d_n9;
        locals.var_isbd2_sws_dn10 = assign15660_e10564_d_n10;
        locals.var_isbd2_sws_dn13 = assign15660_e10564_d_n13;
        locals.var_isbd2_sws_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15670_e10574, assign15670_e10574_d_n0, assign15670_e10574_d_n2, assign15670_e10574_d_n4, assign15670_e10574_d_n5, assign15670_e10574_d_n6, assign15670_e10574_d_n7, assign15670_e10574_d_n8, assign15670_e10574_d_n9, assign15670_e10574_d_n10, assign15670_e10574_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15670_e10572: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign15670_e10572, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign15670_e10574;
        locals.var_isbd_swg_dn0 = assign15670_e10574_d_n0;
        locals.var_isbd_swg_dn2 = assign15670_e10574_d_n2;
        locals.var_isbd_swg_dn4 = assign15670_e10574_d_n4;
        locals.var_isbd_swg_dn5 = assign15670_e10574_d_n5;
        locals.var_isbd_swg_dn6 = assign15670_e10574_d_n6;
        locals.var_isbd_swg_dn7 = assign15670_e10574_d_n7;
        locals.var_isbd_swg_dn8 = assign15670_e10574_d_n8;
        locals.var_isbd_swg_dn9 = assign15670_e10574_d_n9;
        locals.var_isbd_swg_dn10 = assign15670_e10574_d_n10;
        locals.var_isbd_swg_dn13 = assign15670_e10574_d_n13;
        locals.var_isbd_swg_rv = 0.0;

        let (assign15680_e10584, assign15680_e10584_d_n0, assign15680_e10584_d_n2, assign15680_e10584_d_n4, assign15680_e10584_d_n5, assign15680_e10584_d_n6, assign15680_e10584_d_n7, assign15680_e10584_d_n8, assign15680_e10584_d_n9, assign15680_e10584_d_n10, assign15680_e10584_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15680_e10582: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign15680_e10582, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign15680_e10584;
        locals.var_isbd2_swg_dn0 = assign15680_e10584_d_n0;
        locals.var_isbd2_swg_dn2 = assign15680_e10584_d_n2;
        locals.var_isbd2_swg_dn4 = assign15680_e10584_d_n4;
        locals.var_isbd2_swg_dn5 = assign15680_e10584_d_n5;
        locals.var_isbd2_swg_dn6 = assign15680_e10584_d_n6;
        locals.var_isbd2_swg_dn7 = assign15680_e10584_d_n7;
        locals.var_isbd2_swg_dn8 = assign15680_e10584_d_n8;
        locals.var_isbd2_swg_dn9 = assign15680_e10584_d_n9;
        locals.var_isbd2_swg_dn10 = assign15680_e10584_d_n10;
        locals.var_isbd2_swg_dn13 = assign15680_e10584_d_n13;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign15690_e10595, assign15690_e10595_d_n0, assign15690_e10595_d_n2, assign15690_e10595_d_n4, assign15690_e10595_d_n5, assign15690_e10595_d_n6, assign15690_e10595_d_n7, assign15690_e10595_d_n8, assign15690_e10595_d_n9, assign15690_e10595_d_n10, assign15690_e10595_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign15690_e10593: f64 = (p.p13 * locals.var_js);
        (assign15690_e10593, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign15690_e10595;
        locals.var_isbd_btm_dn0 = assign15690_e10595_d_n0;
        locals.var_isbd_btm_dn2 = assign15690_e10595_d_n2;
        locals.var_isbd_btm_dn4 = assign15690_e10595_d_n4;
        locals.var_isbd_btm_dn5 = assign15690_e10595_d_n5;
        locals.var_isbd_btm_dn6 = assign15690_e10595_d_n6;
        locals.var_isbd_btm_dn7 = assign15690_e10595_d_n7;
        locals.var_isbd_btm_dn8 = assign15690_e10595_d_n8;
        locals.var_isbd_btm_dn9 = assign15690_e10595_d_n9;
        locals.var_isbd_btm_dn10 = assign15690_e10595_d_n10;
        locals.var_isbd_btm_dn13 = assign15690_e10595_d_n13;
        locals.var_isbd_btm_rv = 0.0;

        let (assign15700_e10606, assign15700_e10606_d_n0, assign15700_e10606_d_n2, assign15700_e10606_d_n4, assign15700_e10606_d_n5, assign15700_e10606_d_n6, assign15700_e10606_d_n7, assign15700_e10606_d_n8, assign15700_e10606_d_n9, assign15700_e10606_d_n10, assign15700_e10606_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign15700_e10604: f64 = (p.p13 * locals.var_js2);
        (assign15700_e10604, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign15700_e10606;
        locals.var_isbd2_btm_dn0 = assign15700_e10606_d_n0;
        locals.var_isbd2_btm_dn2 = assign15700_e10606_d_n2;
        locals.var_isbd2_btm_dn4 = assign15700_e10606_d_n4;
        locals.var_isbd2_btm_dn5 = assign15700_e10606_d_n5;
        locals.var_isbd2_btm_dn6 = assign15700_e10606_d_n6;
        locals.var_isbd2_btm_dn7 = assign15700_e10606_d_n7;
        locals.var_isbd2_btm_dn8 = assign15700_e10606_d_n8;
        locals.var_isbd2_btm_dn9 = assign15700_e10606_d_n9;
        locals.var_isbd2_btm_dn10 = assign15700_e10606_d_n10;
        locals.var_isbd2_btm_dn13 = assign15700_e10606_d_n13;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign15710_e10615, assign15710_e10615_d_n0, assign15710_e10615_d_n2, assign15710_e10615_d_n4, assign15710_e10615_d_n5, assign15710_e10615_d_n6, assign15710_e10615_d_n7, assign15710_e10615_d_n8, assign15710_e10615_d_n9, assign15710_e10615_d_n10, assign15710_e10615_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign15710_e10615;
        locals.var_isbd_sws_dn0 = assign15710_e10615_d_n0;
        locals.var_isbd_sws_dn2 = assign15710_e10615_d_n2;
        locals.var_isbd_sws_dn4 = assign15710_e10615_d_n4;
        locals.var_isbd_sws_dn5 = assign15710_e10615_d_n5;
        locals.var_isbd_sws_dn6 = assign15710_e10615_d_n6;
        locals.var_isbd_sws_dn7 = assign15710_e10615_d_n7;
        locals.var_isbd_sws_dn8 = assign15710_e10615_d_n8;
        locals.var_isbd_sws_dn9 = assign15710_e10615_d_n9;
        locals.var_isbd_sws_dn10 = assign15710_e10615_d_n10;
        locals.var_isbd_sws_dn13 = assign15710_e10615_d_n13;
        locals.var_isbd_sws_rv = 0.0;

        let (assign15720_e10624, assign15720_e10624_d_n0, assign15720_e10624_d_n2, assign15720_e10624_d_n4, assign15720_e10624_d_n5, assign15720_e10624_d_n6, assign15720_e10624_d_n7, assign15720_e10624_d_n8, assign15720_e10624_d_n9, assign15720_e10624_d_n10, assign15720_e10624_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign15720_e10624;
        locals.var_isbd2_sws_dn0 = assign15720_e10624_d_n0;
        locals.var_isbd2_sws_dn2 = assign15720_e10624_d_n2;
        locals.var_isbd2_sws_dn4 = assign15720_e10624_d_n4;
        locals.var_isbd2_sws_dn5 = assign15720_e10624_d_n5;
        locals.var_isbd2_sws_dn6 = assign15720_e10624_d_n6;
        locals.var_isbd2_sws_dn7 = assign15720_e10624_d_n7;
        locals.var_isbd2_sws_dn8 = assign15720_e10624_d_n8;
        locals.var_isbd2_sws_dn9 = assign15720_e10624_d_n9;
        locals.var_isbd2_sws_dn10 = assign15720_e10624_d_n10;
        locals.var_isbd2_sws_dn13 = assign15720_e10624_d_n13;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign15730_e10635, assign15730_e10635_d_n0, assign15730_e10635_d_n2, assign15730_e10635_d_n4, assign15730_e10635_d_n5, assign15730_e10635_d_n6, assign15730_e10635_d_n7, assign15730_e10635_d_n8, assign15730_e10635_d_n9, assign15730_e10635_d_n10, assign15730_e10635_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign15730_e10633: f64 = (p.p15 * locals.var_jsswg);
        (assign15730_e10633, (p.p15 * locals.var_jsswg_dn0), (p.p15 * locals.var_jsswg_dn2), (p.p15 * locals.var_jsswg_dn4), (p.p15 * locals.var_jsswg_dn5), (p.p15 * locals.var_jsswg_dn6), (p.p15 * locals.var_jsswg_dn7), (p.p15 * locals.var_jsswg_dn8), (p.p15 * locals.var_jsswg_dn9), (p.p15 * locals.var_jsswg_dn10), (p.p15 * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign15730_e10635;
        locals.var_isbd_swg_dn0 = assign15730_e10635_d_n0;
        locals.var_isbd_swg_dn2 = assign15730_e10635_d_n2;
        locals.var_isbd_swg_dn4 = assign15730_e10635_d_n4;
        locals.var_isbd_swg_dn5 = assign15730_e10635_d_n5;
        locals.var_isbd_swg_dn6 = assign15730_e10635_d_n6;
        locals.var_isbd_swg_dn7 = assign15730_e10635_d_n7;
        locals.var_isbd_swg_dn8 = assign15730_e10635_d_n8;
        locals.var_isbd_swg_dn9 = assign15730_e10635_d_n9;
        locals.var_isbd_swg_dn10 = assign15730_e10635_d_n10;
        locals.var_isbd_swg_dn13 = assign15730_e10635_d_n13;
        locals.var_isbd_swg_rv = 0.0;

        let (assign15740_e10646, assign15740_e10646_d_n0, assign15740_e10646_d_n2, assign15740_e10646_d_n4, assign15740_e10646_d_n5, assign15740_e10646_d_n6, assign15740_e10646_d_n7, assign15740_e10646_d_n8, assign15740_e10646_d_n9, assign15740_e10646_d_n10, assign15740_e10646_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign15740_e10644: f64 = (p.p15 * locals.var_jsswg2);
        (assign15740_e10644, (p.p15 * locals.var_jsswg2_dn0), (p.p15 * locals.var_jsswg2_dn2), (p.p15 * locals.var_jsswg2_dn4), (p.p15 * locals.var_jsswg2_dn5), (p.p15 * locals.var_jsswg2_dn6), (p.p15 * locals.var_jsswg2_dn7), (p.p15 * locals.var_jsswg2_dn8), (p.p15 * locals.var_jsswg2_dn9), (p.p15 * locals.var_jsswg2_dn10), (p.p15 * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign15740_e10646;
        locals.var_isbd2_swg_dn0 = assign15740_e10646_d_n0;
        locals.var_isbd2_swg_dn2 = assign15740_e10646_d_n2;
        locals.var_isbd2_swg_dn4 = assign15740_e10646_d_n4;
        locals.var_isbd2_swg_dn5 = assign15740_e10646_d_n5;
        locals.var_isbd2_swg_dn6 = assign15740_e10646_d_n6;
        locals.var_isbd2_swg_dn7 = assign15740_e10646_d_n7;
        locals.var_isbd2_swg_dn8 = assign15740_e10646_d_n8;
        locals.var_isbd2_swg_dn9 = assign15740_e10646_d_n9;
        locals.var_isbd2_swg_dn10 = assign15740_e10646_d_n10;
        locals.var_isbd2_swg_dn13 = assign15740_e10646_d_n13;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign15750_e10655, assign15750_e10655_d_n0, assign15750_e10655_d_n2, assign15750_e10655_d_n4, assign15750_e10655_d_n5, assign15750_e10655_d_n6, assign15750_e10655_d_n7, assign15750_e10655_d_n8, assign15750_e10655_d_n9, assign15750_e10655_d_n10, assign15750_e10655_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign15750_e10653: f64 = (p.p13 * locals.var_js);
        (assign15750_e10653, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign15750_e10655;
        locals.var_isbd_btm_dn0 = assign15750_e10655_d_n0;
        locals.var_isbd_btm_dn2 = assign15750_e10655_d_n2;
        locals.var_isbd_btm_dn4 = assign15750_e10655_d_n4;
        locals.var_isbd_btm_dn5 = assign15750_e10655_d_n5;
        locals.var_isbd_btm_dn6 = assign15750_e10655_d_n6;
        locals.var_isbd_btm_dn7 = assign15750_e10655_d_n7;
        locals.var_isbd_btm_dn8 = assign15750_e10655_d_n8;
        locals.var_isbd_btm_dn9 = assign15750_e10655_d_n9;
        locals.var_isbd_btm_dn10 = assign15750_e10655_d_n10;
        locals.var_isbd_btm_dn13 = assign15750_e10655_d_n13;
        locals.var_isbd_btm_rv = 0.0;

        let (assign15760_e10664, assign15760_e10664_d_n0, assign15760_e10664_d_n2, assign15760_e10664_d_n4, assign15760_e10664_d_n5, assign15760_e10664_d_n6, assign15760_e10664_d_n7, assign15760_e10664_d_n8, assign15760_e10664_d_n9, assign15760_e10664_d_n10, assign15760_e10664_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign15760_e10662: f64 = (p.p13 * locals.var_js2);
        (assign15760_e10662, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign15760_e10664;
        locals.var_isbd2_btm_dn0 = assign15760_e10664_d_n0;
        locals.var_isbd2_btm_dn2 = assign15760_e10664_d_n2;
        locals.var_isbd2_btm_dn4 = assign15760_e10664_d_n4;
        locals.var_isbd2_btm_dn5 = assign15760_e10664_d_n5;
        locals.var_isbd2_btm_dn6 = assign15760_e10664_d_n6;
        locals.var_isbd2_btm_dn7 = assign15760_e10664_d_n7;
        locals.var_isbd2_btm_dn8 = assign15760_e10664_d_n8;
        locals.var_isbd2_btm_dn9 = assign15760_e10664_d_n9;
        locals.var_isbd2_btm_dn10 = assign15760_e10664_d_n10;
        locals.var_isbd2_btm_dn13 = assign15760_e10664_d_n13;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign15770_e10673, assign15770_e10673_d_n0, assign15770_e10673_d_n2, assign15770_e10673_d_n4, assign15770_e10673_d_n5, assign15770_e10673_d_n6, assign15770_e10673_d_n7, assign15770_e10673_d_n8, assign15770_e10673_d_n9, assign15770_e10673_d_n10, assign15770_e10673_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign15770_e10671: f64 = (p.p15 * locals.var_jssw);
        (assign15770_e10671, (p.p15 * locals.var_jssw_dn0), (p.p15 * locals.var_jssw_dn2), (p.p15 * locals.var_jssw_dn4), (p.p15 * locals.var_jssw_dn5), (p.p15 * locals.var_jssw_dn6), (p.p15 * locals.var_jssw_dn7), (p.p15 * locals.var_jssw_dn8), (p.p15 * locals.var_jssw_dn9), (p.p15 * locals.var_jssw_dn10), (p.p15 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign15770_e10673;
        locals.var_isbd_sws_dn0 = assign15770_e10673_d_n0;
        locals.var_isbd_sws_dn2 = assign15770_e10673_d_n2;
        locals.var_isbd_sws_dn4 = assign15770_e10673_d_n4;
        locals.var_isbd_sws_dn5 = assign15770_e10673_d_n5;
        locals.var_isbd_sws_dn6 = assign15770_e10673_d_n6;
        locals.var_isbd_sws_dn7 = assign15770_e10673_d_n7;
        locals.var_isbd_sws_dn8 = assign15770_e10673_d_n8;
        locals.var_isbd_sws_dn9 = assign15770_e10673_d_n9;
        locals.var_isbd_sws_dn10 = assign15770_e10673_d_n10;
        locals.var_isbd_sws_dn13 = assign15770_e10673_d_n13;
        locals.var_isbd_sws_rv = 0.0;

        let (assign15780_e10682, assign15780_e10682_d_n0, assign15780_e10682_d_n2, assign15780_e10682_d_n4, assign15780_e10682_d_n5, assign15780_e10682_d_n6, assign15780_e10682_d_n7, assign15780_e10682_d_n8, assign15780_e10682_d_n9, assign15780_e10682_d_n10, assign15780_e10682_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign15780_e10680: f64 = (p.p15 * locals.var_jssw2);
        (assign15780_e10680, (p.p15 * locals.var_jssw2_dn0), (p.p15 * locals.var_jssw2_dn2), (p.p15 * locals.var_jssw2_dn4), (p.p15 * locals.var_jssw2_dn5), (p.p15 * locals.var_jssw2_dn6), (p.p15 * locals.var_jssw2_dn7), (p.p15 * locals.var_jssw2_dn8), (p.p15 * locals.var_jssw2_dn9), (p.p15 * locals.var_jssw2_dn10), (p.p15 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign15780_e10682;
        locals.var_isbd2_sws_dn0 = assign15780_e10682_d_n0;
        locals.var_isbd2_sws_dn2 = assign15780_e10682_d_n2;
        locals.var_isbd2_sws_dn4 = assign15780_e10682_d_n4;
        locals.var_isbd2_sws_dn5 = assign15780_e10682_d_n5;
        locals.var_isbd2_sws_dn6 = assign15780_e10682_d_n6;
        locals.var_isbd2_sws_dn7 = assign15780_e10682_d_n7;
        locals.var_isbd2_sws_dn8 = assign15780_e10682_d_n8;
        locals.var_isbd2_sws_dn9 = assign15780_e10682_d_n9;
        locals.var_isbd2_sws_dn10 = assign15780_e10682_d_n10;
        locals.var_isbd2_sws_dn13 = assign15780_e10682_d_n13;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign15790_e10689, assign15790_e10689_d_n0, assign15790_e10689_d_n2, assign15790_e10689_d_n4, assign15790_e10689_d_n5, assign15790_e10689_d_n6, assign15790_e10689_d_n7, assign15790_e10689_d_n8, assign15790_e10689_d_n9, assign15790_e10689_d_n10, assign15790_e10689_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign15790_e10689;
        locals.var_isbd_swg_dn0 = assign15790_e10689_d_n0;
        locals.var_isbd_swg_dn2 = assign15790_e10689_d_n2;
        locals.var_isbd_swg_dn4 = assign15790_e10689_d_n4;
        locals.var_isbd_swg_dn5 = assign15790_e10689_d_n5;
        locals.var_isbd_swg_dn6 = assign15790_e10689_d_n6;
        locals.var_isbd_swg_dn7 = assign15790_e10689_d_n7;
        locals.var_isbd_swg_dn8 = assign15790_e10689_d_n8;
        locals.var_isbd_swg_dn9 = assign15790_e10689_d_n9;
        locals.var_isbd_swg_dn10 = assign15790_e10689_d_n10;
        locals.var_isbd_swg_dn13 = assign15790_e10689_d_n13;
        locals.var_isbd_swg_rv = 0.0;

        let (assign15800_e10696, assign15800_e10696_d_n0, assign15800_e10696_d_n2, assign15800_e10696_d_n4, assign15800_e10696_d_n5, assign15800_e10696_d_n6, assign15800_e10696_d_n7, assign15800_e10696_d_n8, assign15800_e10696_d_n9, assign15800_e10696_d_n10, assign15800_e10696_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign15800_e10696;
        locals.var_isbd2_swg_dn0 = assign15800_e10696_d_n0;
        locals.var_isbd2_swg_dn2 = assign15800_e10696_d_n2;
        locals.var_isbd2_swg_dn4 = assign15800_e10696_d_n4;
        locals.var_isbd2_swg_dn5 = assign15800_e10696_d_n5;
        locals.var_isbd2_swg_dn6 = assign15800_e10696_d_n6;
        locals.var_isbd2_swg_dn7 = assign15800_e10696_d_n7;
        locals.var_isbd2_swg_dn8 = assign15800_e10696_d_n8;
        locals.var_isbd2_swg_dn9 = assign15800_e10696_d_n9;
        locals.var_isbd2_swg_dn10 = assign15800_e10696_d_n10;
        locals.var_isbd2_swg_dn13 = assign15800_e10696_d_n13;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign15810_e10704, assign15810_e10704_d_n0, assign15810_e10704_d_n2, assign15810_e10704_d_n4, assign15810_e10704_d_n5, assign15810_e10704_d_n6, assign15810_e10704_d_n7, assign15810_e10704_d_n8, assign15810_e10704_d_n9, assign15810_e10704_d_n10, assign15810_e10704_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15810_e10700: f64 = (locals.var_isbd_btm + locals.var_isbd_sws);
        let assign15810_e10702: f64 = (assign15810_e10700 + locals.var_isbd_swg);
        (assign15810_e10702, ((locals.var_isbd_btm_dn0 + locals.var_isbd_sws_dn0) + locals.var_isbd_swg_dn0), ((locals.var_isbd_btm_dn2 + locals.var_isbd_sws_dn2) + locals.var_isbd_swg_dn2), ((locals.var_isbd_btm_dn4 + locals.var_isbd_sws_dn4) + locals.var_isbd_swg_dn4), ((locals.var_isbd_btm_dn5 + locals.var_isbd_sws_dn5) + locals.var_isbd_swg_dn5), ((locals.var_isbd_btm_dn6 + locals.var_isbd_sws_dn6) + locals.var_isbd_swg_dn6), ((locals.var_isbd_btm_dn7 + locals.var_isbd_sws_dn7) + locals.var_isbd_swg_dn7), ((locals.var_isbd_btm_dn8 + locals.var_isbd_sws_dn8) + locals.var_isbd_swg_dn8), ((locals.var_isbd_btm_dn9 + locals.var_isbd_sws_dn9) + locals.var_isbd_swg_dn9), ((locals.var_isbd_btm_dn10 + locals.var_isbd_sws_dn10) + locals.var_isbd_swg_dn10), ((locals.var_isbd_btm_dn13 + locals.var_isbd_sws_dn13) + locals.var_isbd_swg_dn13),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn13,)
    }
};
        locals.var_isbd = assign15810_e10704;
        locals.var_isbd_dn0 = assign15810_e10704_d_n0;
        locals.var_isbd_dn2 = assign15810_e10704_d_n2;
        locals.var_isbd_dn4 = assign15810_e10704_d_n4;
        locals.var_isbd_dn5 = assign15810_e10704_d_n5;
        locals.var_isbd_dn6 = assign15810_e10704_d_n6;
        locals.var_isbd_dn7 = assign15810_e10704_d_n7;
        locals.var_isbd_dn8 = assign15810_e10704_d_n8;
        locals.var_isbd_dn9 = assign15810_e10704_d_n9;
        locals.var_isbd_dn10 = assign15810_e10704_d_n10;
        locals.var_isbd_dn13 = assign15810_e10704_d_n13;
        locals.var_isbd_rv = 0.0;

        let assign15820_e10707: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard331 = assign15820_e10707;
        locals.var_guard331_rv = 0.0;

        let (assign15830_e10715, assign15830_e10715_d_n0, assign15830_e10715_d_n2, assign15830_e10715_d_n4, assign15830_e10715_d_n5, assign15830_e10715_d_n6, assign15830_e10715_d_n7, assign15830_e10715_d_n8, assign15830_e10715_d_n9, assign15830_e10715_d_n10, assign15830_e10715_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard331 != 0.0)) {
        let assign15830_e10713: f64 = (locals.var_isbd + 1e-25);
        (assign15830_e10713, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign15830_e10715;
        locals.var_t2_dn0 = assign15830_e10715_d_n0;
        locals.var_t2_dn2 = assign15830_e10715_d_n2;
        locals.var_t2_dn4 = assign15830_e10715_d_n4;
        locals.var_t2_dn5 = assign15830_e10715_d_n5;
        locals.var_t2_dn6 = assign15830_e10715_d_n6;
        locals.var_t2_dn7 = assign15830_e10715_d_n7;
        locals.var_t2_dn8 = assign15830_e10715_d_n8;
        locals.var_t2_dn9 = assign15830_e10715_d_n9;
        locals.var_t2_dn10 = assign15830_e10715_d_n10;
        locals.var_t2_dn13 = assign15830_e10715_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign15840_e10732, assign15840_e10732_d_n0, assign15840_e10732_d_n2, assign15840_e10732_d_n4, assign15840_e10732_d_n5, assign15840_e10732_d_n6, assign15840_e10732_d_n7, assign15840_e10732_d_n8, assign15840_e10732_d_n9, assign15840_e10732_d_n10, assign15840_e10732_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard331 != 0.0)) {
        let assign15840_e10721: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign15840_e10724: f64 = (locals.var_uc_vdiffjd * locals.var_t0);
        let assign15840_e10726: f64 = (assign15840_e10724 / locals.var_t2);
        let assign15840_e10728: f64 = (assign15840_e10726 + 1.0);
        let assign15840_e10729: f64 = (assign15840_e10728).ln();
        let assign15840_e10730: f64 = (assign15840_e10721 * assign15840_e10729);
        (assign15840_e10730, (((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn0) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn2) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn4) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn5) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn6) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn7) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn8) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn9) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn10) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn13) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))),)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn0, locals.var_vbdt_dn2, locals.var_vbdt_dn4, locals.var_vbdt_dn5, locals.var_vbdt_dn6, locals.var_vbdt_dn7, locals.var_vbdt_dn8, locals.var_vbdt_dn9, locals.var_vbdt_dn10, locals.var_vbdt_dn13,)
    }
};
        locals.var_vbdt = assign15840_e10732;
        locals.var_vbdt_dn0 = assign15840_e10732_d_n0;
        locals.var_vbdt_dn2 = assign15840_e10732_d_n2;
        locals.var_vbdt_dn4 = assign15840_e10732_d_n4;
        locals.var_vbdt_dn5 = assign15840_e10732_d_n5;
        locals.var_vbdt_dn6 = assign15840_e10732_d_n6;
        locals.var_vbdt_dn7 = assign15840_e10732_d_n7;
        locals.var_vbdt_dn8 = assign15840_e10732_d_n8;
        locals.var_vbdt_dn9 = assign15840_e10732_d_n9;
        locals.var_vbdt_dn10 = assign15840_e10732_d_n10;
        locals.var_vbdt_dn13 = assign15840_e10732_d_n13;
        locals.var_vbdt_rv = 0.0;

        let (assign15850_e10743, assign15850_e10743_d_n0, assign15850_e10743_d_n2, assign15850_e10743_d_n4, assign15850_e10743_d_n5, assign15850_e10743_d_n6, assign15850_e10743_d_n7, assign15850_e10743_d_n8, assign15850_e10743_d_n9, assign15850_e10743_d_n10, assign15850_e10743_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard331 != 0.0)) {
        let assign15850_e10738: f64 = (locals.var_tratio - 1.0);
        let assign15850_e10740: f64 = (assign15850_e10738 * p.p512);
        let assign15850_e10741: f64 = (assign15850_e10740).exp();
        (assign15850_e10741, (assign15850_e10741 * (locals.var_tratio_dn0 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn2 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn4 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn5 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn6 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn7 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn8 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn9 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn10 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn13 * p.p512)),)
    } else {
        (locals.var_exptempd, locals.var_exptempd_dn0, locals.var_exptempd_dn2, locals.var_exptempd_dn4, locals.var_exptempd_dn5, locals.var_exptempd_dn6, locals.var_exptempd_dn7, locals.var_exptempd_dn8, locals.var_exptempd_dn9, locals.var_exptempd_dn10, locals.var_exptempd_dn13,)
    }
};
        locals.var_exptempd = assign15850_e10743;
        locals.var_exptempd_dn0 = assign15850_e10743_d_n0;
        locals.var_exptempd_dn2 = assign15850_e10743_d_n2;
        locals.var_exptempd_dn4 = assign15850_e10743_d_n4;
        locals.var_exptempd_dn5 = assign15850_e10743_d_n5;
        locals.var_exptempd_dn6 = assign15850_e10743_d_n6;
        locals.var_exptempd_dn7 = assign15850_e10743_d_n7;
        locals.var_exptempd_dn8 = assign15850_e10743_d_n8;
        locals.var_exptempd_dn9 = assign15850_e10743_d_n9;
        locals.var_exptempd_dn10 = assign15850_e10743_d_n10;
        locals.var_exptempd_dn13 = assign15850_e10743_d_n13;
        locals.var_exptempd_rv = 0.0;

        let (assign15860_e10753, assign15860_e10753_d_n0, assign15860_e10753_d_n2, assign15860_e10753_d_n4, assign15860_e10753_d_n5, assign15860_e10753_d_n6, assign15860_e10753_d_n7, assign15860_e10753_d_n8, assign15860_e10753_d_n9, assign15860_e10753_d_n10, assign15860_e10753_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard331 != 0.0)) {
        let assign15860_e10750: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign15860_e10751: f64 = (1.0 / assign15860_e10750);
        (assign15860_e10751, (-((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))),)
    } else {
        (locals.var_jd_nvtm_invd, locals.var_jd_nvtm_invd_dn0, locals.var_jd_nvtm_invd_dn2, locals.var_jd_nvtm_invd_dn4, locals.var_jd_nvtm_invd_dn5, locals.var_jd_nvtm_invd_dn6, locals.var_jd_nvtm_invd_dn7, locals.var_jd_nvtm_invd_dn8, locals.var_jd_nvtm_invd_dn9, locals.var_jd_nvtm_invd_dn10, locals.var_jd_nvtm_invd_dn13,)
    }
};
        locals.var_jd_nvtm_invd = assign15860_e10753;
        locals.var_jd_nvtm_invd_dn0 = assign15860_e10753_d_n0;
        locals.var_jd_nvtm_invd_dn2 = assign15860_e10753_d_n2;
        locals.var_jd_nvtm_invd_dn4 = assign15860_e10753_d_n4;
        locals.var_jd_nvtm_invd_dn5 = assign15860_e10753_d_n5;
        locals.var_jd_nvtm_invd_dn6 = assign15860_e10753_d_n6;
        locals.var_jd_nvtm_invd_dn7 = assign15860_e10753_d_n7;
        locals.var_jd_nvtm_invd_dn8 = assign15860_e10753_d_n8;
        locals.var_jd_nvtm_invd_dn9 = assign15860_e10753_d_n9;
        locals.var_jd_nvtm_invd_dn10 = assign15860_e10753_d_n10;
        locals.var_jd_nvtm_invd_dn13 = assign15860_e10753_d_n13;
        locals.var_jd_nvtm_invd_rv = 0.0;

        let (assign15870_e10762, assign15870_e10762_d_n0, assign15870_e10762_d_n2, assign15870_e10762_d_n4, assign15870_e10762_d_n5, assign15870_e10762_d_n6, assign15870_e10762_d_n7, assign15870_e10762_d_n8, assign15870_e10762_d_n9, assign15870_e10762_d_n10, assign15870_e10762_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard331 != 0.0)) {
        let assign15870_e10759: f64 = (locals.var_vbdt * locals.var_jd_nvtm_invd);
        let assign15870_e10760: f64 = (assign15870_e10759).exp();
        (assign15870_e10760, (assign15870_e10760 * ((locals.var_vbdt_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn0))), (assign15870_e10760 * ((locals.var_vbdt_dn2 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn2))), (assign15870_e10760 * ((locals.var_vbdt_dn4 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn4))), (assign15870_e10760 * ((locals.var_vbdt_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn5))), (assign15870_e10760 * ((locals.var_vbdt_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn6))), (assign15870_e10760 * ((locals.var_vbdt_dn7 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn7))), (assign15870_e10760 * ((locals.var_vbdt_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn8))), (assign15870_e10760 * ((locals.var_vbdt_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn9))), (assign15870_e10760 * ((locals.var_vbdt_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn10))), (assign15870_e10760 * ((locals.var_vbdt_dn13 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn13))),)
    } else {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn13,)
    }
};
        locals.var_jd_expcd = assign15870_e10762;
        locals.var_jd_expcd_dn0 = assign15870_e10762_d_n0;
        locals.var_jd_expcd_dn2 = assign15870_e10762_d_n2;
        locals.var_jd_expcd_dn4 = assign15870_e10762_d_n4;
        locals.var_jd_expcd_dn5 = assign15870_e10762_d_n5;
        locals.var_jd_expcd_dn6 = assign15870_e10762_d_n6;
        locals.var_jd_expcd_dn7 = assign15870_e10762_d_n7;
        locals.var_jd_expcd_dn8 = assign15870_e10762_d_n8;
        locals.var_jd_expcd_dn9 = assign15870_e10762_d_n9;
        locals.var_jd_expcd_dn10 = assign15870_e10762_d_n10;
        locals.var_jd_expcd_dn13 = assign15870_e10762_d_n13;
        locals.var_jd_expcd_rv = 0.0;

        let (assign15880_e10781, assign15880_e10781_d_n0, assign15880_e10781_d_n2, assign15880_e10781_d_n4, assign15880_e10781_d_n5, assign15880_e10781_d_n6, assign15880_e10781_d_n7, assign15880_e10781_d_n8, assign15880_e10781_d_n9, assign15880_e10781_d_n10, assign15880_e10781_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15880_e10767: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15880_e10770: f64 = (locals.var_eg * locals.var_beta);
        let assign15880_e10771: f64 = (assign15880_e10767 - assign15880_e10770);
        let assign15880_e10774: f64 = (p.p522 * locals.var_log_tratio);
        let assign15880_e10775: f64 = (assign15880_e10771 + assign15880_e10774);
        let assign15880_e10777: f64 = (assign15880_e10775 / locals.var_uc_njs);
        let assign15880_e10778: f64 = (assign15880_e10777).exp();
        let assign15880_e10779: f64 = (locals.var_uc_js0s * assign15880_e10778);
        (assign15880_e10779, (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn13,)
    }
};
        locals.var_js = assign15880_e10781;
        locals.var_js_dn0 = assign15880_e10781_d_n0;
        locals.var_js_dn2 = assign15880_e10781_d_n2;
        locals.var_js_dn4 = assign15880_e10781_d_n4;
        locals.var_js_dn5 = assign15880_e10781_d_n5;
        locals.var_js_dn6 = assign15880_e10781_d_n6;
        locals.var_js_dn7 = assign15880_e10781_d_n7;
        locals.var_js_dn8 = assign15880_e10781_d_n8;
        locals.var_js_dn9 = assign15880_e10781_d_n9;
        locals.var_js_dn10 = assign15880_e10781_d_n10;
        locals.var_js_dn13 = assign15880_e10781_d_n13;
        locals.var_js_rv = 0.0;

        let (assign15890_e10800, assign15890_e10800_d_n0, assign15890_e10800_d_n2, assign15890_e10800_d_n4, assign15890_e10800_d_n5, assign15890_e10800_d_n6, assign15890_e10800_d_n7, assign15890_e10800_d_n8, assign15890_e10800_d_n9, assign15890_e10800_d_n10, assign15890_e10800_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15890_e10786: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15890_e10789: f64 = (locals.var_eg * locals.var_beta);
        let assign15890_e10790: f64 = (assign15890_e10786 - assign15890_e10789);
        let assign15890_e10793: f64 = (p.p522 * locals.var_log_tratio);
        let assign15890_e10794: f64 = (assign15890_e10790 + assign15890_e10793);
        let assign15890_e10796: f64 = (assign15890_e10794 / p.p520);
        let assign15890_e10797: f64 = (assign15890_e10796).exp();
        let assign15890_e10798: f64 = (locals.var_uc_js0sws * assign15890_e10797);
        (assign15890_e10798, (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / p.p520))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn13,)
    }
};
        locals.var_jssw = assign15890_e10800;
        locals.var_jssw_dn0 = assign15890_e10800_d_n0;
        locals.var_jssw_dn2 = assign15890_e10800_d_n2;
        locals.var_jssw_dn4 = assign15890_e10800_d_n4;
        locals.var_jssw_dn5 = assign15890_e10800_d_n5;
        locals.var_jssw_dn6 = assign15890_e10800_d_n6;
        locals.var_jssw_dn7 = assign15890_e10800_d_n7;
        locals.var_jssw_dn8 = assign15890_e10800_d_n8;
        locals.var_jssw_dn9 = assign15890_e10800_d_n9;
        locals.var_jssw_dn10 = assign15890_e10800_d_n10;
        locals.var_jssw_dn13 = assign15890_e10800_d_n13;
        locals.var_jssw_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15900_e10819, assign15900_e10819_d_n0, assign15900_e10819_d_n2, assign15900_e10819_d_n4, assign15900_e10819_d_n5, assign15900_e10819_d_n6, assign15900_e10819_d_n7, assign15900_e10819_d_n8, assign15900_e10819_d_n9, assign15900_e10819_d_n10, assign15900_e10819_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15900_e10805: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15900_e10808: f64 = (locals.var_eg * locals.var_beta);
        let assign15900_e10809: f64 = (assign15900_e10805 - assign15900_e10808);
        let assign15900_e10812: f64 = (p.p522 * locals.var_log_tratio);
        let assign15900_e10813: f64 = (assign15900_e10809 + assign15900_e10812);
        let assign15900_e10815: f64 = (assign15900_e10813 / p.p521);
        let assign15900_e10816: f64 = (assign15900_e10815).exp();
        let assign15900_e10817: f64 = (p.p518 * assign15900_e10816);
        (assign15900_e10817, (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / p.p521))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn13,)
    }
};
        locals.var_jsswg = assign15900_e10819;
        locals.var_jsswg_dn0 = assign15900_e10819_d_n0;
        locals.var_jsswg_dn2 = assign15900_e10819_d_n2;
        locals.var_jsswg_dn4 = assign15900_e10819_d_n4;
        locals.var_jsswg_dn5 = assign15900_e10819_d_n5;
        locals.var_jsswg_dn6 = assign15900_e10819_d_n6;
        locals.var_jsswg_dn7 = assign15900_e10819_d_n7;
        locals.var_jsswg_dn8 = assign15900_e10819_d_n8;
        locals.var_jsswg_dn9 = assign15900_e10819_d_n9;
        locals.var_jsswg_dn10 = assign15900_e10819_d_n10;
        locals.var_jsswg_dn13 = assign15900_e10819_d_n13;
        locals.var_jsswg_rv = 0.0;

        let (assign15910_e10838, assign15910_e10838_d_n0, assign15910_e10838_d_n2, assign15910_e10838_d_n4, assign15910_e10838_d_n5, assign15910_e10838_d_n6, assign15910_e10838_d_n7, assign15910_e10838_d_n8, assign15910_e10838_d_n9, assign15910_e10838_d_n10, assign15910_e10838_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15910_e10824: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15910_e10827: f64 = (locals.var_eg * locals.var_beta);
        let assign15910_e10828: f64 = (assign15910_e10824 - assign15910_e10827);
        let assign15910_e10831: f64 = (p.p532 * locals.var_log_tratio);
        let assign15910_e10832: f64 = (assign15910_e10828 + assign15910_e10831);
        let assign15910_e10834: f64 = (assign15910_e10832 / locals.var_uc_njs);
        let assign15910_e10835: f64 = (assign15910_e10834).exp();
        let assign15910_e10836: f64 = (locals.var_uc_js0s * assign15910_e10835);
        (assign15910_e10836, (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn13,)
    }
};
        locals.var_js2 = assign15910_e10838;
        locals.var_js2_dn0 = assign15910_e10838_d_n0;
        locals.var_js2_dn2 = assign15910_e10838_d_n2;
        locals.var_js2_dn4 = assign15910_e10838_d_n4;
        locals.var_js2_dn5 = assign15910_e10838_d_n5;
        locals.var_js2_dn6 = assign15910_e10838_d_n6;
        locals.var_js2_dn7 = assign15910_e10838_d_n7;
        locals.var_js2_dn8 = assign15910_e10838_d_n8;
        locals.var_js2_dn9 = assign15910_e10838_d_n9;
        locals.var_js2_dn10 = assign15910_e10838_d_n10;
        locals.var_js2_dn13 = assign15910_e10838_d_n13;
        locals.var_js2_rv = 0.0;

        let (assign15920_e10857, assign15920_e10857_d_n0, assign15920_e10857_d_n2, assign15920_e10857_d_n4, assign15920_e10857_d_n5, assign15920_e10857_d_n6, assign15920_e10857_d_n7, assign15920_e10857_d_n8, assign15920_e10857_d_n9, assign15920_e10857_d_n10, assign15920_e10857_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15920_e10843: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15920_e10846: f64 = (locals.var_eg * locals.var_beta);
        let assign15920_e10847: f64 = (assign15920_e10843 - assign15920_e10846);
        let assign15920_e10850: f64 = (p.p532 * locals.var_log_tratio);
        let assign15920_e10851: f64 = (assign15920_e10847 + assign15920_e10850);
        let assign15920_e10853: f64 = (assign15920_e10851 / p.p520);
        let assign15920_e10854: f64 = (assign15920_e10853).exp();
        let assign15920_e10855: f64 = (locals.var_uc_js0sws * assign15920_e10854);
        (assign15920_e10855, (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / p.p520))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn13,)
    }
};
        locals.var_jssw2 = assign15920_e10857;
        locals.var_jssw2_dn0 = assign15920_e10857_d_n0;
        locals.var_jssw2_dn2 = assign15920_e10857_d_n2;
        locals.var_jssw2_dn4 = assign15920_e10857_d_n4;
        locals.var_jssw2_dn5 = assign15920_e10857_d_n5;
        locals.var_jssw2_dn6 = assign15920_e10857_d_n6;
        locals.var_jssw2_dn7 = assign15920_e10857_d_n7;
        locals.var_jssw2_dn8 = assign15920_e10857_d_n8;
        locals.var_jssw2_dn9 = assign15920_e10857_d_n9;
        locals.var_jssw2_dn10 = assign15920_e10857_d_n10;
        locals.var_jssw2_dn13 = assign15920_e10857_d_n13;
        locals.var_jssw2_rv = 0.0;

        let (assign15930_e10876, assign15930_e10876_d_n0, assign15930_e10876_d_n2, assign15930_e10876_d_n4, assign15930_e10876_d_n5, assign15930_e10876_d_n6, assign15930_e10876_d_n7, assign15930_e10876_d_n8, assign15930_e10876_d_n9, assign15930_e10876_d_n10, assign15930_e10876_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15930_e10862: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15930_e10865: f64 = (locals.var_eg * locals.var_beta);
        let assign15930_e10866: f64 = (assign15930_e10862 - assign15930_e10865);
        let assign15930_e10869: f64 = (p.p532 * locals.var_log_tratio);
        let assign15930_e10870: f64 = (assign15930_e10866 + assign15930_e10869);
        let assign15930_e10872: f64 = (assign15930_e10870 / p.p521);
        let assign15930_e10873: f64 = (assign15930_e10872).exp();
        let assign15930_e10874: f64 = (p.p518 * assign15930_e10873);
        (assign15930_e10874, (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / p.p521))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn13,)
    }
};
        locals.var_jsswg2 = assign15930_e10876;
        locals.var_jsswg2_dn0 = assign15930_e10876_d_n0;
        locals.var_jsswg2_dn2 = assign15930_e10876_d_n2;
        locals.var_jsswg2_dn4 = assign15930_e10876_d_n4;
        locals.var_jsswg2_dn5 = assign15930_e10876_d_n5;
        locals.var_jsswg2_dn6 = assign15930_e10876_d_n6;
        locals.var_jsswg2_dn7 = assign15930_e10876_d_n7;
        locals.var_jsswg2_dn8 = assign15930_e10876_d_n8;
        locals.var_jsswg2_dn9 = assign15930_e10876_d_n9;
        locals.var_jsswg2_dn10 = assign15930_e10876_d_n10;
        locals.var_jsswg2_dn13 = assign15930_e10876_d_n13;
        locals.var_jsswg2_rv = 0.0;

        let assign15940_e10879: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard332 = assign15940_e10879;
        locals.var_guard332_rv = 0.0;

        let assign15950_e10882: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard333 = assign15950_e10882;
        locals.var_guard333_rv = 0.0;

        let (assign15960_e10892, assign15960_e10892_d_n0, assign15960_e10892_d_n2, assign15960_e10892_d_n4, assign15960_e10892_d_n5, assign15960_e10892_d_n6, assign15960_e10892_d_n7, assign15960_e10892_d_n8, assign15960_e10892_d_n9, assign15960_e10892_d_n10, assign15960_e10892_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign15960_e10890: f64 = (p.p14 * locals.var_js);
        (assign15960_e10890, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign15960_e10892;
        locals.var_isbs_btm_dn0 = assign15960_e10892_d_n0;
        locals.var_isbs_btm_dn2 = assign15960_e10892_d_n2;
        locals.var_isbs_btm_dn4 = assign15960_e10892_d_n4;
        locals.var_isbs_btm_dn5 = assign15960_e10892_d_n5;
        locals.var_isbs_btm_dn6 = assign15960_e10892_d_n6;
        locals.var_isbs_btm_dn7 = assign15960_e10892_d_n7;
        locals.var_isbs_btm_dn8 = assign15960_e10892_d_n8;
        locals.var_isbs_btm_dn9 = assign15960_e10892_d_n9;
        locals.var_isbs_btm_dn10 = assign15960_e10892_d_n10;
        locals.var_isbs_btm_dn13 = assign15960_e10892_d_n13;
        locals.var_isbs_btm_rv = 0.0;

        let (assign15970_e10902, assign15970_e10902_d_n0, assign15970_e10902_d_n2, assign15970_e10902_d_n4, assign15970_e10902_d_n5, assign15970_e10902_d_n6, assign15970_e10902_d_n7, assign15970_e10902_d_n8, assign15970_e10902_d_n9, assign15970_e10902_d_n10, assign15970_e10902_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign15970_e10900: f64 = (p.p14 * locals.var_js2);
        (assign15970_e10900, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign15970_e10902;
        locals.var_isbs2_btm_dn0 = assign15970_e10902_d_n0;
        locals.var_isbs2_btm_dn2 = assign15970_e10902_d_n2;
        locals.var_isbs2_btm_dn4 = assign15970_e10902_d_n4;
        locals.var_isbs2_btm_dn5 = assign15970_e10902_d_n5;
        locals.var_isbs2_btm_dn6 = assign15970_e10902_d_n6;
        locals.var_isbs2_btm_dn7 = assign15970_e10902_d_n7;
        locals.var_isbs2_btm_dn8 = assign15970_e10902_d_n8;
        locals.var_isbs2_btm_dn9 = assign15970_e10902_d_n9;
        locals.var_isbs2_btm_dn10 = assign15970_e10902_d_n10;
        locals.var_isbs2_btm_dn13 = assign15970_e10902_d_n13;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign15980_e10914, assign15980_e10914_d_n0, assign15980_e10914_d_n2, assign15980_e10914_d_n4, assign15980_e10914_d_n5, assign15980_e10914_d_n6, assign15980_e10914_d_n7, assign15980_e10914_d_n8, assign15980_e10914_d_n9, assign15980_e10914_d_n10, assign15980_e10914_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign15980_e10910: f64 = (p.p16 - locals.var_weff_nf);
        let assign15980_e10912: f64 = (assign15980_e10910 * locals.var_jssw);
        (assign15980_e10912, (assign15980_e10910 * locals.var_jssw_dn0), (assign15980_e10910 * locals.var_jssw_dn2), (assign15980_e10910 * locals.var_jssw_dn4), (assign15980_e10910 * locals.var_jssw_dn5), (assign15980_e10910 * locals.var_jssw_dn6), (assign15980_e10910 * locals.var_jssw_dn7), (assign15980_e10910 * locals.var_jssw_dn8), (assign15980_e10910 * locals.var_jssw_dn9), (assign15980_e10910 * locals.var_jssw_dn10), (assign15980_e10910 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign15980_e10914;
        locals.var_isbs_sws_dn0 = assign15980_e10914_d_n0;
        locals.var_isbs_sws_dn2 = assign15980_e10914_d_n2;
        locals.var_isbs_sws_dn4 = assign15980_e10914_d_n4;
        locals.var_isbs_sws_dn5 = assign15980_e10914_d_n5;
        locals.var_isbs_sws_dn6 = assign15980_e10914_d_n6;
        locals.var_isbs_sws_dn7 = assign15980_e10914_d_n7;
        locals.var_isbs_sws_dn8 = assign15980_e10914_d_n8;
        locals.var_isbs_sws_dn9 = assign15980_e10914_d_n9;
        locals.var_isbs_sws_dn10 = assign15980_e10914_d_n10;
        locals.var_isbs_sws_dn13 = assign15980_e10914_d_n13;
        locals.var_isbs_sws_rv = 0.0;

        let (assign15990_e10926, assign15990_e10926_d_n0, assign15990_e10926_d_n2, assign15990_e10926_d_n4, assign15990_e10926_d_n5, assign15990_e10926_d_n6, assign15990_e10926_d_n7, assign15990_e10926_d_n8, assign15990_e10926_d_n9, assign15990_e10926_d_n10, assign15990_e10926_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign15990_e10922: f64 = (p.p16 - locals.var_weff_nf);
        let assign15990_e10924: f64 = (assign15990_e10922 * locals.var_jssw2);
        (assign15990_e10924, (assign15990_e10922 * locals.var_jssw2_dn0), (assign15990_e10922 * locals.var_jssw2_dn2), (assign15990_e10922 * locals.var_jssw2_dn4), (assign15990_e10922 * locals.var_jssw2_dn5), (assign15990_e10922 * locals.var_jssw2_dn6), (assign15990_e10922 * locals.var_jssw2_dn7), (assign15990_e10922 * locals.var_jssw2_dn8), (assign15990_e10922 * locals.var_jssw2_dn9), (assign15990_e10922 * locals.var_jssw2_dn10), (assign15990_e10922 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign15990_e10926;
        locals.var_isbs2_sws_dn0 = assign15990_e10926_d_n0;
        locals.var_isbs2_sws_dn2 = assign15990_e10926_d_n2;
        locals.var_isbs2_sws_dn4 = assign15990_e10926_d_n4;
        locals.var_isbs2_sws_dn5 = assign15990_e10926_d_n5;
        locals.var_isbs2_sws_dn6 = assign15990_e10926_d_n6;
        locals.var_isbs2_sws_dn7 = assign15990_e10926_d_n7;
        locals.var_isbs2_sws_dn8 = assign15990_e10926_d_n8;
        locals.var_isbs2_sws_dn9 = assign15990_e10926_d_n9;
        locals.var_isbs2_sws_dn10 = assign15990_e10926_d_n10;
        locals.var_isbs2_sws_dn13 = assign15990_e10926_d_n13;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign16000_e10936, assign16000_e10936_d_n0, assign16000_e10936_d_n2, assign16000_e10936_d_n4, assign16000_e10936_d_n5, assign16000_e10936_d_n6, assign16000_e10936_d_n7, assign16000_e10936_d_n8, assign16000_e10936_d_n9, assign16000_e10936_d_n10, assign16000_e10936_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign16000_e10934: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign16000_e10934, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign16000_e10936;
        locals.var_isbs_swg_dn0 = assign16000_e10936_d_n0;
        locals.var_isbs_swg_dn2 = assign16000_e10936_d_n2;
        locals.var_isbs_swg_dn4 = assign16000_e10936_d_n4;
        locals.var_isbs_swg_dn5 = assign16000_e10936_d_n5;
        locals.var_isbs_swg_dn6 = assign16000_e10936_d_n6;
        locals.var_isbs_swg_dn7 = assign16000_e10936_d_n7;
        locals.var_isbs_swg_dn8 = assign16000_e10936_d_n8;
        locals.var_isbs_swg_dn9 = assign16000_e10936_d_n9;
        locals.var_isbs_swg_dn10 = assign16000_e10936_d_n10;
        locals.var_isbs_swg_dn13 = assign16000_e10936_d_n13;
        locals.var_isbs_swg_rv = 0.0;

        let (assign16010_e10946, assign16010_e10946_d_n0, assign16010_e10946_d_n2, assign16010_e10946_d_n4, assign16010_e10946_d_n5, assign16010_e10946_d_n6, assign16010_e10946_d_n7, assign16010_e10946_d_n8, assign16010_e10946_d_n9, assign16010_e10946_d_n10, assign16010_e10946_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign16010_e10944: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign16010_e10944, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign16010_e10946;
        locals.var_isbs2_swg_dn0 = assign16010_e10946_d_n0;
        locals.var_isbs2_swg_dn2 = assign16010_e10946_d_n2;
        locals.var_isbs2_swg_dn4 = assign16010_e10946_d_n4;
        locals.var_isbs2_swg_dn5 = assign16010_e10946_d_n5;
        locals.var_isbs2_swg_dn6 = assign16010_e10946_d_n6;
        locals.var_isbs2_swg_dn7 = assign16010_e10946_d_n7;
        locals.var_isbs2_swg_dn8 = assign16010_e10946_d_n8;
        locals.var_isbs2_swg_dn9 = assign16010_e10946_d_n9;
        locals.var_isbs2_swg_dn10 = assign16010_e10946_d_n10;
        locals.var_isbs2_swg_dn13 = assign16010_e10946_d_n13;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign16020_e10957, assign16020_e10957_d_n0, assign16020_e10957_d_n2, assign16020_e10957_d_n4, assign16020_e10957_d_n5, assign16020_e10957_d_n6, assign16020_e10957_d_n7, assign16020_e10957_d_n8, assign16020_e10957_d_n9, assign16020_e10957_d_n10, assign16020_e10957_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        let assign16020_e10955: f64 = (p.p14 * locals.var_js);
        (assign16020_e10955, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign16020_e10957;
        locals.var_isbs_btm_dn0 = assign16020_e10957_d_n0;
        locals.var_isbs_btm_dn2 = assign16020_e10957_d_n2;
        locals.var_isbs_btm_dn4 = assign16020_e10957_d_n4;
        locals.var_isbs_btm_dn5 = assign16020_e10957_d_n5;
        locals.var_isbs_btm_dn6 = assign16020_e10957_d_n6;
        locals.var_isbs_btm_dn7 = assign16020_e10957_d_n7;
        locals.var_isbs_btm_dn8 = assign16020_e10957_d_n8;
        locals.var_isbs_btm_dn9 = assign16020_e10957_d_n9;
        locals.var_isbs_btm_dn10 = assign16020_e10957_d_n10;
        locals.var_isbs_btm_dn13 = assign16020_e10957_d_n13;
        locals.var_isbs_btm_rv = 0.0;

        let (assign16030_e10968, assign16030_e10968_d_n0, assign16030_e10968_d_n2, assign16030_e10968_d_n4, assign16030_e10968_d_n5, assign16030_e10968_d_n6, assign16030_e10968_d_n7, assign16030_e10968_d_n8, assign16030_e10968_d_n9, assign16030_e10968_d_n10, assign16030_e10968_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        let assign16030_e10966: f64 = (p.p14 * locals.var_js2);
        (assign16030_e10966, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign16030_e10968;
        locals.var_isbs2_btm_dn0 = assign16030_e10968_d_n0;
        locals.var_isbs2_btm_dn2 = assign16030_e10968_d_n2;
        locals.var_isbs2_btm_dn4 = assign16030_e10968_d_n4;
        locals.var_isbs2_btm_dn5 = assign16030_e10968_d_n5;
        locals.var_isbs2_btm_dn6 = assign16030_e10968_d_n6;
        locals.var_isbs2_btm_dn7 = assign16030_e10968_d_n7;
        locals.var_isbs2_btm_dn8 = assign16030_e10968_d_n8;
        locals.var_isbs2_btm_dn9 = assign16030_e10968_d_n9;
        locals.var_isbs2_btm_dn10 = assign16030_e10968_d_n10;
        locals.var_isbs2_btm_dn13 = assign16030_e10968_d_n13;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign16040_e10977, assign16040_e10977_d_n0, assign16040_e10977_d_n2, assign16040_e10977_d_n4, assign16040_e10977_d_n5, assign16040_e10977_d_n6, assign16040_e10977_d_n7, assign16040_e10977_d_n8, assign16040_e10977_d_n9, assign16040_e10977_d_n10, assign16040_e10977_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign16040_e10977;
        locals.var_isbs_sws_dn0 = assign16040_e10977_d_n0;
        locals.var_isbs_sws_dn2 = assign16040_e10977_d_n2;
        locals.var_isbs_sws_dn4 = assign16040_e10977_d_n4;
        locals.var_isbs_sws_dn5 = assign16040_e10977_d_n5;
        locals.var_isbs_sws_dn6 = assign16040_e10977_d_n6;
        locals.var_isbs_sws_dn7 = assign16040_e10977_d_n7;
        locals.var_isbs_sws_dn8 = assign16040_e10977_d_n8;
        locals.var_isbs_sws_dn9 = assign16040_e10977_d_n9;
        locals.var_isbs_sws_dn10 = assign16040_e10977_d_n10;
        locals.var_isbs_sws_dn13 = assign16040_e10977_d_n13;
        locals.var_isbs_sws_rv = 0.0;

        let (assign16050_e10986, assign16050_e10986_d_n0, assign16050_e10986_d_n2, assign16050_e10986_d_n4, assign16050_e10986_d_n5, assign16050_e10986_d_n6, assign16050_e10986_d_n7, assign16050_e10986_d_n8, assign16050_e10986_d_n9, assign16050_e10986_d_n10, assign16050_e10986_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign16050_e10986;
        locals.var_isbs2_sws_dn0 = assign16050_e10986_d_n0;
        locals.var_isbs2_sws_dn2 = assign16050_e10986_d_n2;
        locals.var_isbs2_sws_dn4 = assign16050_e10986_d_n4;
        locals.var_isbs2_sws_dn5 = assign16050_e10986_d_n5;
        locals.var_isbs2_sws_dn6 = assign16050_e10986_d_n6;
        locals.var_isbs2_sws_dn7 = assign16050_e10986_d_n7;
        locals.var_isbs2_sws_dn8 = assign16050_e10986_d_n8;
        locals.var_isbs2_sws_dn9 = assign16050_e10986_d_n9;
        locals.var_isbs2_sws_dn10 = assign16050_e10986_d_n10;
        locals.var_isbs2_sws_dn13 = assign16050_e10986_d_n13;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign16060_e10997, assign16060_e10997_d_n0, assign16060_e10997_d_n2, assign16060_e10997_d_n4, assign16060_e10997_d_n5, assign16060_e10997_d_n6, assign16060_e10997_d_n7, assign16060_e10997_d_n8, assign16060_e10997_d_n9, assign16060_e10997_d_n10, assign16060_e10997_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        let assign16060_e10995: f64 = (p.p16 * locals.var_jsswg);
        (assign16060_e10995, (p.p16 * locals.var_jsswg_dn0), (p.p16 * locals.var_jsswg_dn2), (p.p16 * locals.var_jsswg_dn4), (p.p16 * locals.var_jsswg_dn5), (p.p16 * locals.var_jsswg_dn6), (p.p16 * locals.var_jsswg_dn7), (p.p16 * locals.var_jsswg_dn8), (p.p16 * locals.var_jsswg_dn9), (p.p16 * locals.var_jsswg_dn10), (p.p16 * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign16060_e10997;
        locals.var_isbs_swg_dn0 = assign16060_e10997_d_n0;
        locals.var_isbs_swg_dn2 = assign16060_e10997_d_n2;
        locals.var_isbs_swg_dn4 = assign16060_e10997_d_n4;
        locals.var_isbs_swg_dn5 = assign16060_e10997_d_n5;
        locals.var_isbs_swg_dn6 = assign16060_e10997_d_n6;
        locals.var_isbs_swg_dn7 = assign16060_e10997_d_n7;
        locals.var_isbs_swg_dn8 = assign16060_e10997_d_n8;
        locals.var_isbs_swg_dn9 = assign16060_e10997_d_n9;
        locals.var_isbs_swg_dn10 = assign16060_e10997_d_n10;
        locals.var_isbs_swg_dn13 = assign16060_e10997_d_n13;
        locals.var_isbs_swg_rv = 0.0;

        let (assign16070_e11008, assign16070_e11008_d_n0, assign16070_e11008_d_n2, assign16070_e11008_d_n4, assign16070_e11008_d_n5, assign16070_e11008_d_n6, assign16070_e11008_d_n7, assign16070_e11008_d_n8, assign16070_e11008_d_n9, assign16070_e11008_d_n10, assign16070_e11008_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        let assign16070_e11006: f64 = (p.p16 * locals.var_jsswg2);
        (assign16070_e11006, (p.p16 * locals.var_jsswg2_dn0), (p.p16 * locals.var_jsswg2_dn2), (p.p16 * locals.var_jsswg2_dn4), (p.p16 * locals.var_jsswg2_dn5), (p.p16 * locals.var_jsswg2_dn6), (p.p16 * locals.var_jsswg2_dn7), (p.p16 * locals.var_jsswg2_dn8), (p.p16 * locals.var_jsswg2_dn9), (p.p16 * locals.var_jsswg2_dn10), (p.p16 * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign16070_e11008;
        locals.var_isbs2_swg_dn0 = assign16070_e11008_d_n0;
        locals.var_isbs2_swg_dn2 = assign16070_e11008_d_n2;
        locals.var_isbs2_swg_dn4 = assign16070_e11008_d_n4;
        locals.var_isbs2_swg_dn5 = assign16070_e11008_d_n5;
        locals.var_isbs2_swg_dn6 = assign16070_e11008_d_n6;
        locals.var_isbs2_swg_dn7 = assign16070_e11008_d_n7;
        locals.var_isbs2_swg_dn8 = assign16070_e11008_d_n8;
        locals.var_isbs2_swg_dn9 = assign16070_e11008_d_n9;
        locals.var_isbs2_swg_dn10 = assign16070_e11008_d_n10;
        locals.var_isbs2_swg_dn13 = assign16070_e11008_d_n13;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign16080_e11017, assign16080_e11017_d_n0, assign16080_e11017_d_n2, assign16080_e11017_d_n4, assign16080_e11017_d_n5, assign16080_e11017_d_n6, assign16080_e11017_d_n7, assign16080_e11017_d_n8, assign16080_e11017_d_n9, assign16080_e11017_d_n10, assign16080_e11017_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        let assign16080_e11015: f64 = (p.p14 * locals.var_js);
        (assign16080_e11015, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign16080_e11017;
        locals.var_isbs_btm_dn0 = assign16080_e11017_d_n0;
        locals.var_isbs_btm_dn2 = assign16080_e11017_d_n2;
        locals.var_isbs_btm_dn4 = assign16080_e11017_d_n4;
        locals.var_isbs_btm_dn5 = assign16080_e11017_d_n5;
        locals.var_isbs_btm_dn6 = assign16080_e11017_d_n6;
        locals.var_isbs_btm_dn7 = assign16080_e11017_d_n7;
        locals.var_isbs_btm_dn8 = assign16080_e11017_d_n8;
        locals.var_isbs_btm_dn9 = assign16080_e11017_d_n9;
        locals.var_isbs_btm_dn10 = assign16080_e11017_d_n10;
        locals.var_isbs_btm_dn13 = assign16080_e11017_d_n13;
        locals.var_isbs_btm_rv = 0.0;

        let (assign16090_e11026, assign16090_e11026_d_n0, assign16090_e11026_d_n2, assign16090_e11026_d_n4, assign16090_e11026_d_n5, assign16090_e11026_d_n6, assign16090_e11026_d_n7, assign16090_e11026_d_n8, assign16090_e11026_d_n9, assign16090_e11026_d_n10, assign16090_e11026_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        let assign16090_e11024: f64 = (p.p14 * locals.var_js2);
        (assign16090_e11024, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign16090_e11026;
        locals.var_isbs2_btm_dn0 = assign16090_e11026_d_n0;
        locals.var_isbs2_btm_dn2 = assign16090_e11026_d_n2;
        locals.var_isbs2_btm_dn4 = assign16090_e11026_d_n4;
        locals.var_isbs2_btm_dn5 = assign16090_e11026_d_n5;
        locals.var_isbs2_btm_dn6 = assign16090_e11026_d_n6;
        locals.var_isbs2_btm_dn7 = assign16090_e11026_d_n7;
        locals.var_isbs2_btm_dn8 = assign16090_e11026_d_n8;
        locals.var_isbs2_btm_dn9 = assign16090_e11026_d_n9;
        locals.var_isbs2_btm_dn10 = assign16090_e11026_d_n10;
        locals.var_isbs2_btm_dn13 = assign16090_e11026_d_n13;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign16100_e11035, assign16100_e11035_d_n0, assign16100_e11035_d_n2, assign16100_e11035_d_n4, assign16100_e11035_d_n5, assign16100_e11035_d_n6, assign16100_e11035_d_n7, assign16100_e11035_d_n8, assign16100_e11035_d_n9, assign16100_e11035_d_n10, assign16100_e11035_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        let assign16100_e11033: f64 = (p.p16 * locals.var_jssw);
        (assign16100_e11033, (p.p16 * locals.var_jssw_dn0), (p.p16 * locals.var_jssw_dn2), (p.p16 * locals.var_jssw_dn4), (p.p16 * locals.var_jssw_dn5), (p.p16 * locals.var_jssw_dn6), (p.p16 * locals.var_jssw_dn7), (p.p16 * locals.var_jssw_dn8), (p.p16 * locals.var_jssw_dn9), (p.p16 * locals.var_jssw_dn10), (p.p16 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign16100_e11035;
        locals.var_isbs_sws_dn0 = assign16100_e11035_d_n0;
        locals.var_isbs_sws_dn2 = assign16100_e11035_d_n2;
        locals.var_isbs_sws_dn4 = assign16100_e11035_d_n4;
        locals.var_isbs_sws_dn5 = assign16100_e11035_d_n5;
        locals.var_isbs_sws_dn6 = assign16100_e11035_d_n6;
        locals.var_isbs_sws_dn7 = assign16100_e11035_d_n7;
        locals.var_isbs_sws_dn8 = assign16100_e11035_d_n8;
        locals.var_isbs_sws_dn9 = assign16100_e11035_d_n9;
        locals.var_isbs_sws_dn10 = assign16100_e11035_d_n10;
        locals.var_isbs_sws_dn13 = assign16100_e11035_d_n13;
        locals.var_isbs_sws_rv = 0.0;

        let (assign16110_e11044, assign16110_e11044_d_n0, assign16110_e11044_d_n2, assign16110_e11044_d_n4, assign16110_e11044_d_n5, assign16110_e11044_d_n6, assign16110_e11044_d_n7, assign16110_e11044_d_n8, assign16110_e11044_d_n9, assign16110_e11044_d_n10, assign16110_e11044_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        let assign16110_e11042: f64 = (p.p16 * locals.var_jssw2);
        (assign16110_e11042, (p.p16 * locals.var_jssw2_dn0), (p.p16 * locals.var_jssw2_dn2), (p.p16 * locals.var_jssw2_dn4), (p.p16 * locals.var_jssw2_dn5), (p.p16 * locals.var_jssw2_dn6), (p.p16 * locals.var_jssw2_dn7), (p.p16 * locals.var_jssw2_dn8), (p.p16 * locals.var_jssw2_dn9), (p.p16 * locals.var_jssw2_dn10), (p.p16 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign16110_e11044;
        locals.var_isbs2_sws_dn0 = assign16110_e11044_d_n0;
        locals.var_isbs2_sws_dn2 = assign16110_e11044_d_n2;
        locals.var_isbs2_sws_dn4 = assign16110_e11044_d_n4;
        locals.var_isbs2_sws_dn5 = assign16110_e11044_d_n5;
        locals.var_isbs2_sws_dn6 = assign16110_e11044_d_n6;
        locals.var_isbs2_sws_dn7 = assign16110_e11044_d_n7;
        locals.var_isbs2_sws_dn8 = assign16110_e11044_d_n8;
        locals.var_isbs2_sws_dn9 = assign16110_e11044_d_n9;
        locals.var_isbs2_sws_dn10 = assign16110_e11044_d_n10;
        locals.var_isbs2_sws_dn13 = assign16110_e11044_d_n13;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign16120_e11051, assign16120_e11051_d_n0, assign16120_e11051_d_n2, assign16120_e11051_d_n4, assign16120_e11051_d_n5, assign16120_e11051_d_n6, assign16120_e11051_d_n7, assign16120_e11051_d_n8, assign16120_e11051_d_n9, assign16120_e11051_d_n10, assign16120_e11051_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign16120_e11051;
        locals.var_isbs_swg_dn0 = assign16120_e11051_d_n0;
        locals.var_isbs_swg_dn2 = assign16120_e11051_d_n2;
        locals.var_isbs_swg_dn4 = assign16120_e11051_d_n4;
        locals.var_isbs_swg_dn5 = assign16120_e11051_d_n5;
        locals.var_isbs_swg_dn6 = assign16120_e11051_d_n6;
        locals.var_isbs_swg_dn7 = assign16120_e11051_d_n7;
        locals.var_isbs_swg_dn8 = assign16120_e11051_d_n8;
        locals.var_isbs_swg_dn9 = assign16120_e11051_d_n9;
        locals.var_isbs_swg_dn10 = assign16120_e11051_d_n10;
        locals.var_isbs_swg_dn13 = assign16120_e11051_d_n13;
        locals.var_isbs_swg_rv = 0.0;

        let (assign16130_e11058, assign16130_e11058_d_n0, assign16130_e11058_d_n2, assign16130_e11058_d_n4, assign16130_e11058_d_n5, assign16130_e11058_d_n6, assign16130_e11058_d_n7, assign16130_e11058_d_n8, assign16130_e11058_d_n9, assign16130_e11058_d_n10, assign16130_e11058_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign16130_e11058;
        locals.var_isbs2_swg_dn0 = assign16130_e11058_d_n0;
        locals.var_isbs2_swg_dn2 = assign16130_e11058_d_n2;
        locals.var_isbs2_swg_dn4 = assign16130_e11058_d_n4;
        locals.var_isbs2_swg_dn5 = assign16130_e11058_d_n5;
        locals.var_isbs2_swg_dn6 = assign16130_e11058_d_n6;
        locals.var_isbs2_swg_dn7 = assign16130_e11058_d_n7;
        locals.var_isbs2_swg_dn8 = assign16130_e11058_d_n8;
        locals.var_isbs2_swg_dn9 = assign16130_e11058_d_n9;
        locals.var_isbs2_swg_dn10 = assign16130_e11058_d_n10;
        locals.var_isbs2_swg_dn13 = assign16130_e11058_d_n13;
        locals.var_isbs2_swg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_35(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16140_e11066, assign16140_e11066_d_n0, assign16140_e11066_d_n2, assign16140_e11066_d_n4, assign16140_e11066_d_n5, assign16140_e11066_d_n6, assign16140_e11066_d_n7, assign16140_e11066_d_n8, assign16140_e11066_d_n9, assign16140_e11066_d_n10, assign16140_e11066_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16140_e11062: f64 = (locals.var_isbs_btm + locals.var_isbs_sws);
        let assign16140_e11064: f64 = (assign16140_e11062 + locals.var_isbs_swg);
        (assign16140_e11064, ((locals.var_isbs_btm_dn0 + locals.var_isbs_sws_dn0) + locals.var_isbs_swg_dn0), ((locals.var_isbs_btm_dn2 + locals.var_isbs_sws_dn2) + locals.var_isbs_swg_dn2), ((locals.var_isbs_btm_dn4 + locals.var_isbs_sws_dn4) + locals.var_isbs_swg_dn4), ((locals.var_isbs_btm_dn5 + locals.var_isbs_sws_dn5) + locals.var_isbs_swg_dn5), ((locals.var_isbs_btm_dn6 + locals.var_isbs_sws_dn6) + locals.var_isbs_swg_dn6), ((locals.var_isbs_btm_dn7 + locals.var_isbs_sws_dn7) + locals.var_isbs_swg_dn7), ((locals.var_isbs_btm_dn8 + locals.var_isbs_sws_dn8) + locals.var_isbs_swg_dn8), ((locals.var_isbs_btm_dn9 + locals.var_isbs_sws_dn9) + locals.var_isbs_swg_dn9), ((locals.var_isbs_btm_dn10 + locals.var_isbs_sws_dn10) + locals.var_isbs_swg_dn10), ((locals.var_isbs_btm_dn13 + locals.var_isbs_sws_dn13) + locals.var_isbs_swg_dn13),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn13,)
    }
};
        locals.var_isbs = assign16140_e11066;
        locals.var_isbs_dn0 = assign16140_e11066_d_n0;
        locals.var_isbs_dn2 = assign16140_e11066_d_n2;
        locals.var_isbs_dn4 = assign16140_e11066_d_n4;
        locals.var_isbs_dn5 = assign16140_e11066_d_n5;
        locals.var_isbs_dn6 = assign16140_e11066_d_n6;
        locals.var_isbs_dn7 = assign16140_e11066_d_n7;
        locals.var_isbs_dn8 = assign16140_e11066_d_n8;
        locals.var_isbs_dn9 = assign16140_e11066_d_n9;
        locals.var_isbs_dn10 = assign16140_e11066_d_n10;
        locals.var_isbs_dn13 = assign16140_e11066_d_n13;
        locals.var_isbs_rv = 0.0;

        let assign16150_e11069: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard334 = assign16150_e11069;
        locals.var_guard334_rv = 0.0;

        let (assign16160_e11077, assign16160_e11077_d_n0, assign16160_e11077_d_n2, assign16160_e11077_d_n4, assign16160_e11077_d_n5, assign16160_e11077_d_n6, assign16160_e11077_d_n7, assign16160_e11077_d_n8, assign16160_e11077_d_n9, assign16160_e11077_d_n10, assign16160_e11077_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard334 != 0.0)) {
        let assign16160_e11075: f64 = (locals.var_isbs + 1e-25);
        (assign16160_e11075, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign16160_e11077;
        locals.var_t3_dn0 = assign16160_e11077_d_n0;
        locals.var_t3_dn2 = assign16160_e11077_d_n2;
        locals.var_t3_dn4 = assign16160_e11077_d_n4;
        locals.var_t3_dn5 = assign16160_e11077_d_n5;
        locals.var_t3_dn6 = assign16160_e11077_d_n6;
        locals.var_t3_dn7 = assign16160_e11077_d_n7;
        locals.var_t3_dn8 = assign16160_e11077_d_n8;
        locals.var_t3_dn9 = assign16160_e11077_d_n9;
        locals.var_t3_dn10 = assign16160_e11077_d_n10;
        locals.var_t3_dn13 = assign16160_e11077_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign16170_e11094, assign16170_e11094_d_n0, assign16170_e11094_d_n2, assign16170_e11094_d_n4, assign16170_e11094_d_n5, assign16170_e11094_d_n6, assign16170_e11094_d_n7, assign16170_e11094_d_n8, assign16170_e11094_d_n9, assign16170_e11094_d_n10, assign16170_e11094_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard334 != 0.0)) {
        let assign16170_e11083: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign16170_e11086: f64 = (locals.var_uc_vdiffjs * locals.var_t0);
        let assign16170_e11088: f64 = (assign16170_e11086 / locals.var_t3);
        let assign16170_e11090: f64 = (assign16170_e11088 + 1.0);
        let assign16170_e11091: f64 = (assign16170_e11090).ln();
        let assign16170_e11092: f64 = (assign16170_e11083 * assign16170_e11091);
        (assign16170_e11092, (((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn0) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn2) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn4) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn5) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn6) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn7) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn8) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn9) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn10) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn13) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))),)
    } else {
        (locals.var_vbst, locals.var_vbst_dn0, locals.var_vbst_dn2, locals.var_vbst_dn4, locals.var_vbst_dn5, locals.var_vbst_dn6, locals.var_vbst_dn7, locals.var_vbst_dn8, locals.var_vbst_dn9, locals.var_vbst_dn10, locals.var_vbst_dn13,)
    }
};
        locals.var_vbst = assign16170_e11094;
        locals.var_vbst_dn0 = assign16170_e11094_d_n0;
        locals.var_vbst_dn2 = assign16170_e11094_d_n2;
        locals.var_vbst_dn4 = assign16170_e11094_d_n4;
        locals.var_vbst_dn5 = assign16170_e11094_d_n5;
        locals.var_vbst_dn6 = assign16170_e11094_d_n6;
        locals.var_vbst_dn7 = assign16170_e11094_d_n7;
        locals.var_vbst_dn8 = assign16170_e11094_d_n8;
        locals.var_vbst_dn9 = assign16170_e11094_d_n9;
        locals.var_vbst_dn10 = assign16170_e11094_d_n10;
        locals.var_vbst_dn13 = assign16170_e11094_d_n13;
        locals.var_vbst_rv = 0.0;

        let (assign16180_e11105, assign16180_e11105_d_n0, assign16180_e11105_d_n2, assign16180_e11105_d_n4, assign16180_e11105_d_n5, assign16180_e11105_d_n6, assign16180_e11105_d_n7, assign16180_e11105_d_n8, assign16180_e11105_d_n9, assign16180_e11105_d_n10, assign16180_e11105_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard334 != 0.0)) {
        let assign16180_e11100: f64 = (locals.var_tratio - 1.0);
        let assign16180_e11102: f64 = (assign16180_e11100 * p.p535);
        let assign16180_e11103: f64 = (assign16180_e11102).exp();
        (assign16180_e11103, (assign16180_e11103 * (locals.var_tratio_dn0 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn2 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn4 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn5 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn6 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn7 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn8 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn9 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn10 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn13 * p.p535)),)
    } else {
        (locals.var_exptemps, locals.var_exptemps_dn0, locals.var_exptemps_dn2, locals.var_exptemps_dn4, locals.var_exptemps_dn5, locals.var_exptemps_dn6, locals.var_exptemps_dn7, locals.var_exptemps_dn8, locals.var_exptemps_dn9, locals.var_exptemps_dn10, locals.var_exptemps_dn13,)
    }
};
        locals.var_exptemps = assign16180_e11105;
        locals.var_exptemps_dn0 = assign16180_e11105_d_n0;
        locals.var_exptemps_dn2 = assign16180_e11105_d_n2;
        locals.var_exptemps_dn4 = assign16180_e11105_d_n4;
        locals.var_exptemps_dn5 = assign16180_e11105_d_n5;
        locals.var_exptemps_dn6 = assign16180_e11105_d_n6;
        locals.var_exptemps_dn7 = assign16180_e11105_d_n7;
        locals.var_exptemps_dn8 = assign16180_e11105_d_n8;
        locals.var_exptemps_dn9 = assign16180_e11105_d_n9;
        locals.var_exptemps_dn10 = assign16180_e11105_d_n10;
        locals.var_exptemps_dn13 = assign16180_e11105_d_n13;
        locals.var_exptemps_rv = 0.0;

        let (assign16190_e11115, assign16190_e11115_d_n0, assign16190_e11115_d_n2, assign16190_e11115_d_n4, assign16190_e11115_d_n5, assign16190_e11115_d_n6, assign16190_e11115_d_n7, assign16190_e11115_d_n8, assign16190_e11115_d_n9, assign16190_e11115_d_n10, assign16190_e11115_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard334 != 0.0)) {
        let assign16190_e11112: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign16190_e11113: f64 = (1.0 / assign16190_e11112);
        (assign16190_e11113, (-((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))),)
    } else {
        (locals.var_jd_nvtm_invs, locals.var_jd_nvtm_invs_dn0, locals.var_jd_nvtm_invs_dn2, locals.var_jd_nvtm_invs_dn4, locals.var_jd_nvtm_invs_dn5, locals.var_jd_nvtm_invs_dn6, locals.var_jd_nvtm_invs_dn7, locals.var_jd_nvtm_invs_dn8, locals.var_jd_nvtm_invs_dn9, locals.var_jd_nvtm_invs_dn10, locals.var_jd_nvtm_invs_dn13,)
    }
};
        locals.var_jd_nvtm_invs = assign16190_e11115;
        locals.var_jd_nvtm_invs_dn0 = assign16190_e11115_d_n0;
        locals.var_jd_nvtm_invs_dn2 = assign16190_e11115_d_n2;
        locals.var_jd_nvtm_invs_dn4 = assign16190_e11115_d_n4;
        locals.var_jd_nvtm_invs_dn5 = assign16190_e11115_d_n5;
        locals.var_jd_nvtm_invs_dn6 = assign16190_e11115_d_n6;
        locals.var_jd_nvtm_invs_dn7 = assign16190_e11115_d_n7;
        locals.var_jd_nvtm_invs_dn8 = assign16190_e11115_d_n8;
        locals.var_jd_nvtm_invs_dn9 = assign16190_e11115_d_n9;
        locals.var_jd_nvtm_invs_dn10 = assign16190_e11115_d_n10;
        locals.var_jd_nvtm_invs_dn13 = assign16190_e11115_d_n13;
        locals.var_jd_nvtm_invs_rv = 0.0;

        let (assign16200_e11124, assign16200_e11124_d_n0, assign16200_e11124_d_n2, assign16200_e11124_d_n4, assign16200_e11124_d_n5, assign16200_e11124_d_n6, assign16200_e11124_d_n7, assign16200_e11124_d_n8, assign16200_e11124_d_n9, assign16200_e11124_d_n10, assign16200_e11124_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard334 != 0.0)) {
        let assign16200_e11121: f64 = (locals.var_vbst * locals.var_jd_nvtm_invs);
        let assign16200_e11122: f64 = (assign16200_e11121).exp();
        (assign16200_e11122, (assign16200_e11122 * ((locals.var_vbst_dn0 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn0))), (assign16200_e11122 * ((locals.var_vbst_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn2))), (assign16200_e11122 * ((locals.var_vbst_dn4 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn4))), (assign16200_e11122 * ((locals.var_vbst_dn5 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn5))), (assign16200_e11122 * ((locals.var_vbst_dn6 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn6))), (assign16200_e11122 * ((locals.var_vbst_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn7))), (assign16200_e11122 * ((locals.var_vbst_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn8))), (assign16200_e11122 * ((locals.var_vbst_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn9))), (assign16200_e11122 * ((locals.var_vbst_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn10))), (assign16200_e11122 * ((locals.var_vbst_dn13 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn13))),)
    } else {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn13,)
    }
};
        locals.var_jd_expcs = assign16200_e11124;
        locals.var_jd_expcs_dn0 = assign16200_e11124_d_n0;
        locals.var_jd_expcs_dn2 = assign16200_e11124_d_n2;
        locals.var_jd_expcs_dn4 = assign16200_e11124_d_n4;
        locals.var_jd_expcs_dn5 = assign16200_e11124_d_n5;
        locals.var_jd_expcs_dn6 = assign16200_e11124_d_n6;
        locals.var_jd_expcs_dn7 = assign16200_e11124_d_n7;
        locals.var_jd_expcs_dn8 = assign16200_e11124_d_n8;
        locals.var_jd_expcs_dn9 = assign16200_e11124_d_n9;
        locals.var_jd_expcs_dn10 = assign16200_e11124_d_n10;
        locals.var_jd_expcs_dn13 = assign16200_e11124_d_n13;
        locals.var_jd_expcs_rv = 0.0;

        let (assign16210_e11136, assign16210_e11136_d_n0, assign16210_e11136_d_n2, assign16210_e11136_d_n4, assign16210_e11136_d_n5, assign16210_e11136_d_n6, assign16210_e11136_d_n7, assign16210_e11136_d_n8, assign16210_e11136_d_n9, assign16210_e11136_d_n10, assign16210_e11136_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16210_e11128: f64 = (p.p500 * p.p13);
        let assign16210_e11132: f64 = (p.p481 * locals.var_tdiff);
        let assign16210_e11133: f64 = (1.0 + assign16210_e11132);
        let assign16210_e11134: f64 = (assign16210_e11128 * assign16210_e11133);
        (assign16210_e11134, (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn0)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn2)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn4)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn5)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn6)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn7)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn8)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn9)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn10)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn13,)
    }
};
        locals.var_czbd = assign16210_e11136;
        locals.var_czbd_dn0 = assign16210_e11136_d_n0;
        locals.var_czbd_dn2 = assign16210_e11136_d_n2;
        locals.var_czbd_dn4 = assign16210_e11136_d_n4;
        locals.var_czbd_dn5 = assign16210_e11136_d_n5;
        locals.var_czbd_dn6 = assign16210_e11136_d_n6;
        locals.var_czbd_dn7 = assign16210_e11136_d_n7;
        locals.var_czbd_dn8 = assign16210_e11136_d_n8;
        locals.var_czbd_dn9 = assign16210_e11136_d_n9;
        locals.var_czbd_dn10 = assign16210_e11136_d_n10;
        locals.var_czbd_dn13 = assign16210_e11136_d_n13;
        locals.var_czbd_rv = 0.0;

        let assign16220_e11139: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard335 = assign16220_e11139;
        locals.var_guard335_rv = 0.0;

        let (assign16230_e11155, assign16230_e11155_d_n0, assign16230_e11155_d_n2, assign16230_e11155_d_n4, assign16230_e11155_d_n5, assign16230_e11155_d_n6, assign16230_e11155_d_n7, assign16230_e11155_d_n8, assign16230_e11155_d_n9, assign16230_e11155_d_n10, assign16230_e11155_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard335 != 0.0)) {
        let assign16230_e11146: f64 = (p.p15 - locals.var_weff_nf);
        let assign16230_e11147: f64 = (p.p501 * assign16230_e11146);
        let assign16230_e11151: f64 = (p.p483 * locals.var_tdiff);
        let assign16230_e11152: f64 = (1.0 + assign16230_e11151);
        let assign16230_e11153: f64 = (assign16230_e11147 * assign16230_e11152);
        (assign16230_e11153, (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn0)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn2)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn4)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn5)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn6)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn7)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn8)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn9)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn10)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign16230_e11155;
        locals.var_czbdsw_dn0 = assign16230_e11155_d_n0;
        locals.var_czbdsw_dn2 = assign16230_e11155_d_n2;
        locals.var_czbdsw_dn4 = assign16230_e11155_d_n4;
        locals.var_czbdsw_dn5 = assign16230_e11155_d_n5;
        locals.var_czbdsw_dn6 = assign16230_e11155_d_n6;
        locals.var_czbdsw_dn7 = assign16230_e11155_d_n7;
        locals.var_czbdsw_dn8 = assign16230_e11155_d_n8;
        locals.var_czbdsw_dn9 = assign16230_e11155_d_n9;
        locals.var_czbdsw_dn10 = assign16230_e11155_d_n10;
        locals.var_czbdsw_dn13 = assign16230_e11155_d_n13;
        locals.var_czbdsw_rv = 0.0;

        let (assign16240_e11169, assign16240_e11169_d_n0, assign16240_e11169_d_n2, assign16240_e11169_d_n4, assign16240_e11169_d_n5, assign16240_e11169_d_n6, assign16240_e11169_d_n7, assign16240_e11169_d_n8, assign16240_e11169_d_n9, assign16240_e11169_d_n10, assign16240_e11169_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard335 != 0.0)) {
        let assign16240_e11161: f64 = (p.p502 * locals.var_weff_nf);
        let assign16240_e11165: f64 = (p.p485 * locals.var_tdiff);
        let assign16240_e11166: f64 = (1.0 + assign16240_e11165);
        let assign16240_e11167: f64 = (assign16240_e11161 * assign16240_e11166);
        (assign16240_e11167, (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn0)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn2)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn4)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn5)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn6)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn7)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn8)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn9)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn10)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign16240_e11169;
        locals.var_czbdswg_dn0 = assign16240_e11169_d_n0;
        locals.var_czbdswg_dn2 = assign16240_e11169_d_n2;
        locals.var_czbdswg_dn4 = assign16240_e11169_d_n4;
        locals.var_czbdswg_dn5 = assign16240_e11169_d_n5;
        locals.var_czbdswg_dn6 = assign16240_e11169_d_n6;
        locals.var_czbdswg_dn7 = assign16240_e11169_d_n7;
        locals.var_czbdswg_dn8 = assign16240_e11169_d_n8;
        locals.var_czbdswg_dn9 = assign16240_e11169_d_n9;
        locals.var_czbdswg_dn10 = assign16240_e11169_d_n10;
        locals.var_czbdswg_dn13 = assign16240_e11169_d_n13;
        locals.var_czbdswg_rv = 0.0;

        let (assign16250_e11176, assign16250_e11176_d_n0, assign16250_e11176_d_n2, assign16250_e11176_d_n4, assign16250_e11176_d_n5, assign16250_e11176_d_n6, assign16250_e11176_d_n7, assign16250_e11176_d_n8, assign16250_e11176_d_n9, assign16250_e11176_d_n10, assign16250_e11176_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard335 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign16250_e11176;
        locals.var_czbdsw_dn0 = assign16250_e11176_d_n0;
        locals.var_czbdsw_dn2 = assign16250_e11176_d_n2;
        locals.var_czbdsw_dn4 = assign16250_e11176_d_n4;
        locals.var_czbdsw_dn5 = assign16250_e11176_d_n5;
        locals.var_czbdsw_dn6 = assign16250_e11176_d_n6;
        locals.var_czbdsw_dn7 = assign16250_e11176_d_n7;
        locals.var_czbdsw_dn8 = assign16250_e11176_d_n8;
        locals.var_czbdsw_dn9 = assign16250_e11176_d_n9;
        locals.var_czbdsw_dn10 = assign16250_e11176_d_n10;
        locals.var_czbdsw_dn13 = assign16250_e11176_d_n13;
        locals.var_czbdsw_rv = 0.0;

        let (assign16260_e11191, assign16260_e11191_d_n0, assign16260_e11191_d_n2, assign16260_e11191_d_n4, assign16260_e11191_d_n5, assign16260_e11191_d_n6, assign16260_e11191_d_n7, assign16260_e11191_d_n8, assign16260_e11191_d_n9, assign16260_e11191_d_n10, assign16260_e11191_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard335 == 0.0)) {
        let assign16260_e11183: f64 = (p.p502 * p.p15);
        let assign16260_e11187: f64 = (p.p485 * locals.var_tdiff);
        let assign16260_e11188: f64 = (1.0 + assign16260_e11187);
        let assign16260_e11189: f64 = (assign16260_e11183 * assign16260_e11188);
        (assign16260_e11189, (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn0)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn2)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn4)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn5)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn6)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn7)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn8)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn9)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn10)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign16260_e11191;
        locals.var_czbdswg_dn0 = assign16260_e11191_d_n0;
        locals.var_czbdswg_dn2 = assign16260_e11191_d_n2;
        locals.var_czbdswg_dn4 = assign16260_e11191_d_n4;
        locals.var_czbdswg_dn5 = assign16260_e11191_d_n5;
        locals.var_czbdswg_dn6 = assign16260_e11191_d_n6;
        locals.var_czbdswg_dn7 = assign16260_e11191_d_n7;
        locals.var_czbdswg_dn8 = assign16260_e11191_d_n8;
        locals.var_czbdswg_dn9 = assign16260_e11191_d_n9;
        locals.var_czbdswg_dn10 = assign16260_e11191_d_n10;
        locals.var_czbdswg_dn13 = assign16260_e11191_d_n13;
        locals.var_czbdswg_rv = 0.0;

        let assign16270_e11194: f64 = if locals.var_czbd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard336 = assign16270_e11194;
        locals.var_guard336_rv = 0.0;

        let (assign16280_e11200, assign16280_e11200_d_n0, assign16280_e11200_d_n2, assign16280_e11200_d_n4, assign16280_e11200_d_n5, assign16280_e11200_d_n6, assign16280_e11200_d_n7, assign16280_e11200_d_n8, assign16280_e11200_d_n9, assign16280_e11200_d_n10, assign16280_e11200_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard336 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn13,)
    }
};
        locals.var_czbd = assign16280_e11200;
        locals.var_czbd_dn0 = assign16280_e11200_d_n0;
        locals.var_czbd_dn2 = assign16280_e11200_d_n2;
        locals.var_czbd_dn4 = assign16280_e11200_d_n4;
        locals.var_czbd_dn5 = assign16280_e11200_d_n5;
        locals.var_czbd_dn6 = assign16280_e11200_d_n6;
        locals.var_czbd_dn7 = assign16280_e11200_d_n7;
        locals.var_czbd_dn8 = assign16280_e11200_d_n8;
        locals.var_czbd_dn9 = assign16280_e11200_d_n9;
        locals.var_czbd_dn10 = assign16280_e11200_d_n10;
        locals.var_czbd_dn13 = assign16280_e11200_d_n13;
        locals.var_czbd_rv = 0.0;

        let assign16290_e11203: f64 = if locals.var_czbdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard337 = assign16290_e11203;
        locals.var_guard337_rv = 0.0;

        let (assign16300_e11209, assign16300_e11209_d_n0, assign16300_e11209_d_n2, assign16300_e11209_d_n4, assign16300_e11209_d_n5, assign16300_e11209_d_n6, assign16300_e11209_d_n7, assign16300_e11209_d_n8, assign16300_e11209_d_n9, assign16300_e11209_d_n10, assign16300_e11209_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard337 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign16300_e11209;
        locals.var_czbdsw_dn0 = assign16300_e11209_d_n0;
        locals.var_czbdsw_dn2 = assign16300_e11209_d_n2;
        locals.var_czbdsw_dn4 = assign16300_e11209_d_n4;
        locals.var_czbdsw_dn5 = assign16300_e11209_d_n5;
        locals.var_czbdsw_dn6 = assign16300_e11209_d_n6;
        locals.var_czbdsw_dn7 = assign16300_e11209_d_n7;
        locals.var_czbdsw_dn8 = assign16300_e11209_d_n8;
        locals.var_czbdsw_dn9 = assign16300_e11209_d_n9;
        locals.var_czbdsw_dn10 = assign16300_e11209_d_n10;
        locals.var_czbdsw_dn13 = assign16300_e11209_d_n13;
        locals.var_czbdsw_rv = 0.0;

        let assign16310_e11212: f64 = if locals.var_czbdswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard338 = assign16310_e11212;
        locals.var_guard338_rv = 0.0;

        let (assign16320_e11218, assign16320_e11218_d_n0, assign16320_e11218_d_n2, assign16320_e11218_d_n4, assign16320_e11218_d_n5, assign16320_e11218_d_n6, assign16320_e11218_d_n7, assign16320_e11218_d_n8, assign16320_e11218_d_n9, assign16320_e11218_d_n10, assign16320_e11218_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard338 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign16320_e11218;
        locals.var_czbdswg_dn0 = assign16320_e11218_d_n0;
        locals.var_czbdswg_dn2 = assign16320_e11218_d_n2;
        locals.var_czbdswg_dn4 = assign16320_e11218_d_n4;
        locals.var_czbdswg_dn5 = assign16320_e11218_d_n5;
        locals.var_czbdswg_dn6 = assign16320_e11218_d_n6;
        locals.var_czbdswg_dn7 = assign16320_e11218_d_n7;
        locals.var_czbdswg_dn8 = assign16320_e11218_d_n8;
        locals.var_czbdswg_dn9 = assign16320_e11218_d_n9;
        locals.var_czbdswg_dn10 = assign16320_e11218_d_n10;
        locals.var_czbdswg_dn13 = assign16320_e11218_d_n13;
        locals.var_czbdswg_rv = 0.0;

        let (assign16330_e11226, assign16330_e11226_d_n0, assign16330_e11226_d_n2, assign16330_e11226_d_n4, assign16330_e11226_d_n5, assign16330_e11226_d_n6, assign16330_e11226_d_n7, assign16330_e11226_d_n8, assign16330_e11226_d_n9, assign16330_e11226_d_n10, assign16330_e11226_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16330_e11223: f64 = (p.p487 * locals.var_tdiff);
        let assign16330_e11224: f64 = (p.p506 - assign16330_e11223);
        (assign16330_e11224, (-(p.p487 * locals.var_tdiff_dn0)), (-(p.p487 * locals.var_tdiff_dn2)), (-(p.p487 * locals.var_tdiff_dn4)), (-(p.p487 * locals.var_tdiff_dn5)), (-(p.p487 * locals.var_tdiff_dn6)), (-(p.p487 * locals.var_tdiff_dn7)), (-(p.p487 * locals.var_tdiff_dn8)), (-(p.p487 * locals.var_tdiff_dn9)), (-(p.p487 * locals.var_tdiff_dn10)), (-(p.p487 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn13,)
    }
};
        locals.var_pzbd = assign16330_e11226;
        locals.var_pzbd_dn0 = assign16330_e11226_d_n0;
        locals.var_pzbd_dn2 = assign16330_e11226_d_n2;
        locals.var_pzbd_dn4 = assign16330_e11226_d_n4;
        locals.var_pzbd_dn5 = assign16330_e11226_d_n5;
        locals.var_pzbd_dn6 = assign16330_e11226_d_n6;
        locals.var_pzbd_dn7 = assign16330_e11226_d_n7;
        locals.var_pzbd_dn8 = assign16330_e11226_d_n8;
        locals.var_pzbd_dn9 = assign16330_e11226_d_n9;
        locals.var_pzbd_dn10 = assign16330_e11226_d_n10;
        locals.var_pzbd_dn13 = assign16330_e11226_d_n13;
        locals.var_pzbd_rv = 0.0;

        let (assign16340_e11234, assign16340_e11234_d_n0, assign16340_e11234_d_n2, assign16340_e11234_d_n4, assign16340_e11234_d_n5, assign16340_e11234_d_n6, assign16340_e11234_d_n7, assign16340_e11234_d_n8, assign16340_e11234_d_n9, assign16340_e11234_d_n10, assign16340_e11234_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16340_e11231: f64 = (p.p489 * locals.var_tdiff);
        let assign16340_e11232: f64 = (p.p507 - assign16340_e11231);
        (assign16340_e11232, (-(p.p489 * locals.var_tdiff_dn0)), (-(p.p489 * locals.var_tdiff_dn2)), (-(p.p489 * locals.var_tdiff_dn4)), (-(p.p489 * locals.var_tdiff_dn5)), (-(p.p489 * locals.var_tdiff_dn6)), (-(p.p489 * locals.var_tdiff_dn7)), (-(p.p489 * locals.var_tdiff_dn8)), (-(p.p489 * locals.var_tdiff_dn9)), (-(p.p489 * locals.var_tdiff_dn10)), (-(p.p489 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn13,)
    }
};
        locals.var_pzbdsw = assign16340_e11234;
        locals.var_pzbdsw_dn0 = assign16340_e11234_d_n0;
        locals.var_pzbdsw_dn2 = assign16340_e11234_d_n2;
        locals.var_pzbdsw_dn4 = assign16340_e11234_d_n4;
        locals.var_pzbdsw_dn5 = assign16340_e11234_d_n5;
        locals.var_pzbdsw_dn6 = assign16340_e11234_d_n6;
        locals.var_pzbdsw_dn7 = assign16340_e11234_d_n7;
        locals.var_pzbdsw_dn8 = assign16340_e11234_d_n8;
        locals.var_pzbdsw_dn9 = assign16340_e11234_d_n9;
        locals.var_pzbdsw_dn10 = assign16340_e11234_d_n10;
        locals.var_pzbdsw_dn13 = assign16340_e11234_d_n13;
        locals.var_pzbdsw_rv = 0.0;

        let (assign16350_e11242, assign16350_e11242_d_n0, assign16350_e11242_d_n2, assign16350_e11242_d_n4, assign16350_e11242_d_n5, assign16350_e11242_d_n6, assign16350_e11242_d_n7, assign16350_e11242_d_n8, assign16350_e11242_d_n9, assign16350_e11242_d_n10, assign16350_e11242_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16350_e11239: f64 = (p.p491 * locals.var_tdiff);
        let assign16350_e11240: f64 = (p.p508 - assign16350_e11239);
        (assign16350_e11240, (-(p.p491 * locals.var_tdiff_dn0)), (-(p.p491 * locals.var_tdiff_dn2)), (-(p.p491 * locals.var_tdiff_dn4)), (-(p.p491 * locals.var_tdiff_dn5)), (-(p.p491 * locals.var_tdiff_dn6)), (-(p.p491 * locals.var_tdiff_dn7)), (-(p.p491 * locals.var_tdiff_dn8)), (-(p.p491 * locals.var_tdiff_dn9)), (-(p.p491 * locals.var_tdiff_dn10)), (-(p.p491 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn13,)
    }
};
        locals.var_pzbdswg = assign16350_e11242;
        locals.var_pzbdswg_dn0 = assign16350_e11242_d_n0;
        locals.var_pzbdswg_dn2 = assign16350_e11242_d_n2;
        locals.var_pzbdswg_dn4 = assign16350_e11242_d_n4;
        locals.var_pzbdswg_dn5 = assign16350_e11242_d_n5;
        locals.var_pzbdswg_dn6 = assign16350_e11242_d_n6;
        locals.var_pzbdswg_dn7 = assign16350_e11242_d_n7;
        locals.var_pzbdswg_dn8 = assign16350_e11242_d_n8;
        locals.var_pzbdswg_dn9 = assign16350_e11242_d_n9;
        locals.var_pzbdswg_dn10 = assign16350_e11242_d_n10;
        locals.var_pzbdswg_dn13 = assign16350_e11242_d_n13;
        locals.var_pzbdswg_rv = 0.0;

        let assign16360_e11249: f64 = if ((locals.var_pzbd < 0.01) && (p.p13 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard339 = assign16360_e11249;
        locals.var_guard339_rv = 0.0;

        let (assign16370_e11255, assign16370_e11255_d_n0, assign16370_e11255_d_n2, assign16370_e11255_d_n4, assign16370_e11255_d_n5, assign16370_e11255_d_n6, assign16370_e11255_d_n7, assign16370_e11255_d_n8, assign16370_e11255_d_n9, assign16370_e11255_d_n10, assign16370_e11255_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard339 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn13,)
    }
};
        locals.var_pzbd = assign16370_e11255;
        locals.var_pzbd_dn0 = assign16370_e11255_d_n0;
        locals.var_pzbd_dn2 = assign16370_e11255_d_n2;
        locals.var_pzbd_dn4 = assign16370_e11255_d_n4;
        locals.var_pzbd_dn5 = assign16370_e11255_d_n5;
        locals.var_pzbd_dn6 = assign16370_e11255_d_n6;
        locals.var_pzbd_dn7 = assign16370_e11255_d_n7;
        locals.var_pzbd_dn8 = assign16370_e11255_d_n8;
        locals.var_pzbd_dn9 = assign16370_e11255_d_n9;
        locals.var_pzbd_dn10 = assign16370_e11255_d_n10;
        locals.var_pzbd_dn13 = assign16370_e11255_d_n13;
        locals.var_pzbd_rv = 0.0;

        let assign16380_e11262: f64 = if ((locals.var_pzbdsw < 0.01) && (p.p15 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard340 = assign16380_e11262;
        locals.var_guard340_rv = 0.0;

        let (assign16390_e11268, assign16390_e11268_d_n0, assign16390_e11268_d_n2, assign16390_e11268_d_n4, assign16390_e11268_d_n5, assign16390_e11268_d_n6, assign16390_e11268_d_n7, assign16390_e11268_d_n8, assign16390_e11268_d_n9, assign16390_e11268_d_n10, assign16390_e11268_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard340 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn13,)
    }
};
        locals.var_pzbdsw = assign16390_e11268;
        locals.var_pzbdsw_dn0 = assign16390_e11268_d_n0;
        locals.var_pzbdsw_dn2 = assign16390_e11268_d_n2;
        locals.var_pzbdsw_dn4 = assign16390_e11268_d_n4;
        locals.var_pzbdsw_dn5 = assign16390_e11268_d_n5;
        locals.var_pzbdsw_dn6 = assign16390_e11268_d_n6;
        locals.var_pzbdsw_dn7 = assign16390_e11268_d_n7;
        locals.var_pzbdsw_dn8 = assign16390_e11268_d_n8;
        locals.var_pzbdsw_dn9 = assign16390_e11268_d_n9;
        locals.var_pzbdsw_dn10 = assign16390_e11268_d_n10;
        locals.var_pzbdsw_dn13 = assign16390_e11268_d_n13;
        locals.var_pzbdsw_rv = 0.0;

        let assign16400_e11275: f64 = if ((locals.var_pzbdswg < 0.01) && (p.p15 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard341 = assign16400_e11275;
        locals.var_guard341_rv = 0.0;

        let (assign16410_e11281, assign16410_e11281_d_n0, assign16410_e11281_d_n2, assign16410_e11281_d_n4, assign16410_e11281_d_n5, assign16410_e11281_d_n6, assign16410_e11281_d_n7, assign16410_e11281_d_n8, assign16410_e11281_d_n9, assign16410_e11281_d_n10, assign16410_e11281_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard341 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn13,)
    }
};
        locals.var_pzbdswg = assign16410_e11281;
        locals.var_pzbdswg_dn0 = assign16410_e11281_d_n0;
        locals.var_pzbdswg_dn2 = assign16410_e11281_d_n2;
        locals.var_pzbdswg_dn4 = assign16410_e11281_d_n4;
        locals.var_pzbdswg_dn5 = assign16410_e11281_d_n5;
        locals.var_pzbdswg_dn6 = assign16410_e11281_d_n6;
        locals.var_pzbdswg_dn7 = assign16410_e11281_d_n7;
        locals.var_pzbdswg_dn8 = assign16410_e11281_d_n8;
        locals.var_pzbdswg_dn9 = assign16410_e11281_d_n9;
        locals.var_pzbdswg_dn10 = assign16410_e11281_d_n10;
        locals.var_pzbdswg_dn13 = assign16410_e11281_d_n13;
        locals.var_pzbdswg_rv = 0.0;

        let (assign16420_e11293, assign16420_e11293_d_n0, assign16420_e11293_d_n2, assign16420_e11293_d_n4, assign16420_e11293_d_n5, assign16420_e11293_d_n6, assign16420_e11293_d_n7, assign16420_e11293_d_n8, assign16420_e11293_d_n9, assign16420_e11293_d_n10, assign16420_e11293_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16420_e11285: f64 = (p.p523 * p.p14);
        let assign16420_e11289: f64 = (p.p482 * locals.var_tdiff);
        let assign16420_e11290: f64 = (1.0 + assign16420_e11289);
        let assign16420_e11291: f64 = (assign16420_e11285 * assign16420_e11290);
        (assign16420_e11291, (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn0)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn2)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn4)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn5)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn6)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn7)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn8)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn9)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn10)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn13,)
    }
};
        locals.var_czbs = assign16420_e11293;
        locals.var_czbs_dn0 = assign16420_e11293_d_n0;
        locals.var_czbs_dn2 = assign16420_e11293_d_n2;
        locals.var_czbs_dn4 = assign16420_e11293_d_n4;
        locals.var_czbs_dn5 = assign16420_e11293_d_n5;
        locals.var_czbs_dn6 = assign16420_e11293_d_n6;
        locals.var_czbs_dn7 = assign16420_e11293_d_n7;
        locals.var_czbs_dn8 = assign16420_e11293_d_n8;
        locals.var_czbs_dn9 = assign16420_e11293_d_n9;
        locals.var_czbs_dn10 = assign16420_e11293_d_n10;
        locals.var_czbs_dn13 = assign16420_e11293_d_n13;
        locals.var_czbs_rv = 0.0;

        let assign16430_e11296: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard342 = assign16430_e11296;
        locals.var_guard342_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_36(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (assign16440_e11312, assign16440_e11312_d_n0, assign16440_e11312_d_n2, assign16440_e11312_d_n4, assign16440_e11312_d_n5, assign16440_e11312_d_n6, assign16440_e11312_d_n7, assign16440_e11312_d_n8, assign16440_e11312_d_n9, assign16440_e11312_d_n10, assign16440_e11312_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard342 != 0.0)) {
        let assign16440_e11303: f64 = (p.p16 - locals.var_weff_nf);
        let assign16440_e11304: f64 = (p.p524 * assign16440_e11303);
        let assign16440_e11308: f64 = (p.p484 * locals.var_tdiff);
        let assign16440_e11309: f64 = (1.0 + assign16440_e11308);
        let assign16440_e11310: f64 = (assign16440_e11304 * assign16440_e11309);
        (assign16440_e11310, (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn0)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn2)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn4)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn5)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn6)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn7)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn8)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn9)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn10)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign16440_e11312;
        locals.var_czbssw_dn0 = assign16440_e11312_d_n0;
        locals.var_czbssw_dn2 = assign16440_e11312_d_n2;
        locals.var_czbssw_dn4 = assign16440_e11312_d_n4;
        locals.var_czbssw_dn5 = assign16440_e11312_d_n5;
        locals.var_czbssw_dn6 = assign16440_e11312_d_n6;
        locals.var_czbssw_dn7 = assign16440_e11312_d_n7;
        locals.var_czbssw_dn8 = assign16440_e11312_d_n8;
        locals.var_czbssw_dn9 = assign16440_e11312_d_n9;
        locals.var_czbssw_dn10 = assign16440_e11312_d_n10;
        locals.var_czbssw_dn13 = assign16440_e11312_d_n13;
        locals.var_czbssw_rv = 0.0;

        let (assign16450_e11326, assign16450_e11326_d_n0, assign16450_e11326_d_n2, assign16450_e11326_d_n4, assign16450_e11326_d_n5, assign16450_e11326_d_n6, assign16450_e11326_d_n7, assign16450_e11326_d_n8, assign16450_e11326_d_n9, assign16450_e11326_d_n10, assign16450_e11326_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard342 != 0.0)) {
        let assign16450_e11318: f64 = (p.p525 * locals.var_weff_nf);
        let assign16450_e11322: f64 = (p.p486 * locals.var_tdiff);
        let assign16450_e11323: f64 = (1.0 + assign16450_e11322);
        let assign16450_e11324: f64 = (assign16450_e11318 * assign16450_e11323);
        (assign16450_e11324, (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn0)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn2)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn4)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn5)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn6)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn7)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn8)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn9)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn10)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign16450_e11326;
        locals.var_czbsswg_dn0 = assign16450_e11326_d_n0;
        locals.var_czbsswg_dn2 = assign16450_e11326_d_n2;
        locals.var_czbsswg_dn4 = assign16450_e11326_d_n4;
        locals.var_czbsswg_dn5 = assign16450_e11326_d_n5;
        locals.var_czbsswg_dn6 = assign16450_e11326_d_n6;
        locals.var_czbsswg_dn7 = assign16450_e11326_d_n7;
        locals.var_czbsswg_dn8 = assign16450_e11326_d_n8;
        locals.var_czbsswg_dn9 = assign16450_e11326_d_n9;
        locals.var_czbsswg_dn10 = assign16450_e11326_d_n10;
        locals.var_czbsswg_dn13 = assign16450_e11326_d_n13;
        locals.var_czbsswg_rv = 0.0;

        let (assign16460_e11333, assign16460_e11333_d_n0, assign16460_e11333_d_n2, assign16460_e11333_d_n4, assign16460_e11333_d_n5, assign16460_e11333_d_n6, assign16460_e11333_d_n7, assign16460_e11333_d_n8, assign16460_e11333_d_n9, assign16460_e11333_d_n10, assign16460_e11333_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard342 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign16460_e11333;
        locals.var_czbssw_dn0 = assign16460_e11333_d_n0;
        locals.var_czbssw_dn2 = assign16460_e11333_d_n2;
        locals.var_czbssw_dn4 = assign16460_e11333_d_n4;
        locals.var_czbssw_dn5 = assign16460_e11333_d_n5;
        locals.var_czbssw_dn6 = assign16460_e11333_d_n6;
        locals.var_czbssw_dn7 = assign16460_e11333_d_n7;
        locals.var_czbssw_dn8 = assign16460_e11333_d_n8;
        locals.var_czbssw_dn9 = assign16460_e11333_d_n9;
        locals.var_czbssw_dn10 = assign16460_e11333_d_n10;
        locals.var_czbssw_dn13 = assign16460_e11333_d_n13;
        locals.var_czbssw_rv = 0.0;

        let (assign16470_e11348, assign16470_e11348_d_n0, assign16470_e11348_d_n2, assign16470_e11348_d_n4, assign16470_e11348_d_n5, assign16470_e11348_d_n6, assign16470_e11348_d_n7, assign16470_e11348_d_n8, assign16470_e11348_d_n9, assign16470_e11348_d_n10, assign16470_e11348_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard342 == 0.0)) {
        let assign16470_e11340: f64 = (p.p525 * p.p16);
        let assign16470_e11344: f64 = (p.p486 * locals.var_tdiff);
        let assign16470_e11345: f64 = (1.0 + assign16470_e11344);
        let assign16470_e11346: f64 = (assign16470_e11340 * assign16470_e11345);
        (assign16470_e11346, (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn0)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn2)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn4)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn5)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn6)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn7)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn8)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn9)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn10)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign16470_e11348;
        locals.var_czbsswg_dn0 = assign16470_e11348_d_n0;
        locals.var_czbsswg_dn2 = assign16470_e11348_d_n2;
        locals.var_czbsswg_dn4 = assign16470_e11348_d_n4;
        locals.var_czbsswg_dn5 = assign16470_e11348_d_n5;
        locals.var_czbsswg_dn6 = assign16470_e11348_d_n6;
        locals.var_czbsswg_dn7 = assign16470_e11348_d_n7;
        locals.var_czbsswg_dn8 = assign16470_e11348_d_n8;
        locals.var_czbsswg_dn9 = assign16470_e11348_d_n9;
        locals.var_czbsswg_dn10 = assign16470_e11348_d_n10;
        locals.var_czbsswg_dn13 = assign16470_e11348_d_n13;
        locals.var_czbsswg_rv = 0.0;

        let assign16480_e11351: f64 = if locals.var_czbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard343 = assign16480_e11351;
        locals.var_guard343_rv = 0.0;

        let (assign16490_e11357, assign16490_e11357_d_n0, assign16490_e11357_d_n2, assign16490_e11357_d_n4, assign16490_e11357_d_n5, assign16490_e11357_d_n6, assign16490_e11357_d_n7, assign16490_e11357_d_n8, assign16490_e11357_d_n9, assign16490_e11357_d_n10, assign16490_e11357_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard343 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn13,)
    }
};
        locals.var_czbs = assign16490_e11357;
        locals.var_czbs_dn0 = assign16490_e11357_d_n0;
        locals.var_czbs_dn2 = assign16490_e11357_d_n2;
        locals.var_czbs_dn4 = assign16490_e11357_d_n4;
        locals.var_czbs_dn5 = assign16490_e11357_d_n5;
        locals.var_czbs_dn6 = assign16490_e11357_d_n6;
        locals.var_czbs_dn7 = assign16490_e11357_d_n7;
        locals.var_czbs_dn8 = assign16490_e11357_d_n8;
        locals.var_czbs_dn9 = assign16490_e11357_d_n9;
        locals.var_czbs_dn10 = assign16490_e11357_d_n10;
        locals.var_czbs_dn13 = assign16490_e11357_d_n13;
        locals.var_czbs_rv = 0.0;

        let assign16500_e11360: f64 = if locals.var_czbssw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard344 = assign16500_e11360;
        locals.var_guard344_rv = 0.0;

        let (assign16510_e11366, assign16510_e11366_d_n0, assign16510_e11366_d_n2, assign16510_e11366_d_n4, assign16510_e11366_d_n5, assign16510_e11366_d_n6, assign16510_e11366_d_n7, assign16510_e11366_d_n8, assign16510_e11366_d_n9, assign16510_e11366_d_n10, assign16510_e11366_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard344 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign16510_e11366;
        locals.var_czbssw_dn0 = assign16510_e11366_d_n0;
        locals.var_czbssw_dn2 = assign16510_e11366_d_n2;
        locals.var_czbssw_dn4 = assign16510_e11366_d_n4;
        locals.var_czbssw_dn5 = assign16510_e11366_d_n5;
        locals.var_czbssw_dn6 = assign16510_e11366_d_n6;
        locals.var_czbssw_dn7 = assign16510_e11366_d_n7;
        locals.var_czbssw_dn8 = assign16510_e11366_d_n8;
        locals.var_czbssw_dn9 = assign16510_e11366_d_n9;
        locals.var_czbssw_dn10 = assign16510_e11366_d_n10;
        locals.var_czbssw_dn13 = assign16510_e11366_d_n13;
        locals.var_czbssw_rv = 0.0;

        let assign16520_e11369: f64 = if locals.var_czbsswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard345 = assign16520_e11369;
        locals.var_guard345_rv = 0.0;

        let (assign16530_e11375, assign16530_e11375_d_n0, assign16530_e11375_d_n2, assign16530_e11375_d_n4, assign16530_e11375_d_n5, assign16530_e11375_d_n6, assign16530_e11375_d_n7, assign16530_e11375_d_n8, assign16530_e11375_d_n9, assign16530_e11375_d_n10, assign16530_e11375_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard345 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign16530_e11375;
        locals.var_czbsswg_dn0 = assign16530_e11375_d_n0;
        locals.var_czbsswg_dn2 = assign16530_e11375_d_n2;
        locals.var_czbsswg_dn4 = assign16530_e11375_d_n4;
        locals.var_czbsswg_dn5 = assign16530_e11375_d_n5;
        locals.var_czbsswg_dn6 = assign16530_e11375_d_n6;
        locals.var_czbsswg_dn7 = assign16530_e11375_d_n7;
        locals.var_czbsswg_dn8 = assign16530_e11375_d_n8;
        locals.var_czbsswg_dn9 = assign16530_e11375_d_n9;
        locals.var_czbsswg_dn10 = assign16530_e11375_d_n10;
        locals.var_czbsswg_dn13 = assign16530_e11375_d_n13;
        locals.var_czbsswg_rv = 0.0;

        let (assign16540_e11383, assign16540_e11383_d_n0, assign16540_e11383_d_n2, assign16540_e11383_d_n4, assign16540_e11383_d_n5, assign16540_e11383_d_n6, assign16540_e11383_d_n7, assign16540_e11383_d_n8, assign16540_e11383_d_n9, assign16540_e11383_d_n10, assign16540_e11383_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16540_e11380: f64 = (p.p488 * locals.var_tdiff);
        let assign16540_e11381: f64 = (p.p529 - assign16540_e11380);
        (assign16540_e11381, (-(p.p488 * locals.var_tdiff_dn0)), (-(p.p488 * locals.var_tdiff_dn2)), (-(p.p488 * locals.var_tdiff_dn4)), (-(p.p488 * locals.var_tdiff_dn5)), (-(p.p488 * locals.var_tdiff_dn6)), (-(p.p488 * locals.var_tdiff_dn7)), (-(p.p488 * locals.var_tdiff_dn8)), (-(p.p488 * locals.var_tdiff_dn9)), (-(p.p488 * locals.var_tdiff_dn10)), (-(p.p488 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn13,)
    }
};
        locals.var_pzbs = assign16540_e11383;
        locals.var_pzbs_dn0 = assign16540_e11383_d_n0;
        locals.var_pzbs_dn2 = assign16540_e11383_d_n2;
        locals.var_pzbs_dn4 = assign16540_e11383_d_n4;
        locals.var_pzbs_dn5 = assign16540_e11383_d_n5;
        locals.var_pzbs_dn6 = assign16540_e11383_d_n6;
        locals.var_pzbs_dn7 = assign16540_e11383_d_n7;
        locals.var_pzbs_dn8 = assign16540_e11383_d_n8;
        locals.var_pzbs_dn9 = assign16540_e11383_d_n9;
        locals.var_pzbs_dn10 = assign16540_e11383_d_n10;
        locals.var_pzbs_dn13 = assign16540_e11383_d_n13;
        locals.var_pzbs_rv = 0.0;

        let (assign16550_e11391, assign16550_e11391_d_n0, assign16550_e11391_d_n2, assign16550_e11391_d_n4, assign16550_e11391_d_n5, assign16550_e11391_d_n6, assign16550_e11391_d_n7, assign16550_e11391_d_n8, assign16550_e11391_d_n9, assign16550_e11391_d_n10, assign16550_e11391_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16550_e11388: f64 = (p.p490 * locals.var_tdiff);
        let assign16550_e11389: f64 = (p.p530 - assign16550_e11388);
        (assign16550_e11389, (-(p.p490 * locals.var_tdiff_dn0)), (-(p.p490 * locals.var_tdiff_dn2)), (-(p.p490 * locals.var_tdiff_dn4)), (-(p.p490 * locals.var_tdiff_dn5)), (-(p.p490 * locals.var_tdiff_dn6)), (-(p.p490 * locals.var_tdiff_dn7)), (-(p.p490 * locals.var_tdiff_dn8)), (-(p.p490 * locals.var_tdiff_dn9)), (-(p.p490 * locals.var_tdiff_dn10)), (-(p.p490 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn13,)
    }
};
        locals.var_pzbssw = assign16550_e11391;
        locals.var_pzbssw_dn0 = assign16550_e11391_d_n0;
        locals.var_pzbssw_dn2 = assign16550_e11391_d_n2;
        locals.var_pzbssw_dn4 = assign16550_e11391_d_n4;
        locals.var_pzbssw_dn5 = assign16550_e11391_d_n5;
        locals.var_pzbssw_dn6 = assign16550_e11391_d_n6;
        locals.var_pzbssw_dn7 = assign16550_e11391_d_n7;
        locals.var_pzbssw_dn8 = assign16550_e11391_d_n8;
        locals.var_pzbssw_dn9 = assign16550_e11391_d_n9;
        locals.var_pzbssw_dn10 = assign16550_e11391_d_n10;
        locals.var_pzbssw_dn13 = assign16550_e11391_d_n13;
        locals.var_pzbssw_rv = 0.0;

        let (assign16560_e11399, assign16560_e11399_d_n0, assign16560_e11399_d_n2, assign16560_e11399_d_n4, assign16560_e11399_d_n5, assign16560_e11399_d_n6, assign16560_e11399_d_n7, assign16560_e11399_d_n8, assign16560_e11399_d_n9, assign16560_e11399_d_n10, assign16560_e11399_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16560_e11396: f64 = (p.p492 * locals.var_tdiff);
        let assign16560_e11397: f64 = (p.p531 - assign16560_e11396);
        (assign16560_e11397, (-(p.p492 * locals.var_tdiff_dn0)), (-(p.p492 * locals.var_tdiff_dn2)), (-(p.p492 * locals.var_tdiff_dn4)), (-(p.p492 * locals.var_tdiff_dn5)), (-(p.p492 * locals.var_tdiff_dn6)), (-(p.p492 * locals.var_tdiff_dn7)), (-(p.p492 * locals.var_tdiff_dn8)), (-(p.p492 * locals.var_tdiff_dn9)), (-(p.p492 * locals.var_tdiff_dn10)), (-(p.p492 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn13,)
    }
};
        locals.var_pzbsswg = assign16560_e11399;
        locals.var_pzbsswg_dn0 = assign16560_e11399_d_n0;
        locals.var_pzbsswg_dn2 = assign16560_e11399_d_n2;
        locals.var_pzbsswg_dn4 = assign16560_e11399_d_n4;
        locals.var_pzbsswg_dn5 = assign16560_e11399_d_n5;
        locals.var_pzbsswg_dn6 = assign16560_e11399_d_n6;
        locals.var_pzbsswg_dn7 = assign16560_e11399_d_n7;
        locals.var_pzbsswg_dn8 = assign16560_e11399_d_n8;
        locals.var_pzbsswg_dn9 = assign16560_e11399_d_n9;
        locals.var_pzbsswg_dn10 = assign16560_e11399_d_n10;
        locals.var_pzbsswg_dn13 = assign16560_e11399_d_n13;
        locals.var_pzbsswg_rv = 0.0;

        let assign16570_e11406: f64 = if ((locals.var_pzbs < 0.01) && (p.p14 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard346 = assign16570_e11406;
        locals.var_guard346_rv = 0.0;

        let (assign16580_e11412, assign16580_e11412_d_n0, assign16580_e11412_d_n2, assign16580_e11412_d_n4, assign16580_e11412_d_n5, assign16580_e11412_d_n6, assign16580_e11412_d_n7, assign16580_e11412_d_n8, assign16580_e11412_d_n9, assign16580_e11412_d_n10, assign16580_e11412_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard346 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn13,)
    }
};
        locals.var_pzbs = assign16580_e11412;
        locals.var_pzbs_dn0 = assign16580_e11412_d_n0;
        locals.var_pzbs_dn2 = assign16580_e11412_d_n2;
        locals.var_pzbs_dn4 = assign16580_e11412_d_n4;
        locals.var_pzbs_dn5 = assign16580_e11412_d_n5;
        locals.var_pzbs_dn6 = assign16580_e11412_d_n6;
        locals.var_pzbs_dn7 = assign16580_e11412_d_n7;
        locals.var_pzbs_dn8 = assign16580_e11412_d_n8;
        locals.var_pzbs_dn9 = assign16580_e11412_d_n9;
        locals.var_pzbs_dn10 = assign16580_e11412_d_n10;
        locals.var_pzbs_dn13 = assign16580_e11412_d_n13;
        locals.var_pzbs_rv = 0.0;

        let assign16590_e11419: f64 = if ((locals.var_pzbssw < 0.01) && (p.p16 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard347 = assign16590_e11419;
        locals.var_guard347_rv = 0.0;

        let (assign16600_e11425, assign16600_e11425_d_n0, assign16600_e11425_d_n2, assign16600_e11425_d_n4, assign16600_e11425_d_n5, assign16600_e11425_d_n6, assign16600_e11425_d_n7, assign16600_e11425_d_n8, assign16600_e11425_d_n9, assign16600_e11425_d_n10, assign16600_e11425_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard347 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn13,)
    }
};
        locals.var_pzbssw = assign16600_e11425;
        locals.var_pzbssw_dn0 = assign16600_e11425_d_n0;
        locals.var_pzbssw_dn2 = assign16600_e11425_d_n2;
        locals.var_pzbssw_dn4 = assign16600_e11425_d_n4;
        locals.var_pzbssw_dn5 = assign16600_e11425_d_n5;
        locals.var_pzbssw_dn6 = assign16600_e11425_d_n6;
        locals.var_pzbssw_dn7 = assign16600_e11425_d_n7;
        locals.var_pzbssw_dn8 = assign16600_e11425_d_n8;
        locals.var_pzbssw_dn9 = assign16600_e11425_d_n9;
        locals.var_pzbssw_dn10 = assign16600_e11425_d_n10;
        locals.var_pzbssw_dn13 = assign16600_e11425_d_n13;
        locals.var_pzbssw_rv = 0.0;

        let assign16610_e11432: f64 = if ((locals.var_pzbsswg < 0.01) && (p.p16 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard348 = assign16610_e11432;
        locals.var_guard348_rv = 0.0;

        let (assign16620_e11438, assign16620_e11438_d_n0, assign16620_e11438_d_n2, assign16620_e11438_d_n4, assign16620_e11438_d_n5, assign16620_e11438_d_n6, assign16620_e11438_d_n7, assign16620_e11438_d_n8, assign16620_e11438_d_n9, assign16620_e11438_d_n10, assign16620_e11438_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard348 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn13,)
    }
};
        locals.var_pzbsswg = assign16620_e11438;
        locals.var_pzbsswg_dn0 = assign16620_e11438_d_n0;
        locals.var_pzbsswg_dn2 = assign16620_e11438_d_n2;
        locals.var_pzbsswg_dn4 = assign16620_e11438_d_n4;
        locals.var_pzbsswg_dn5 = assign16620_e11438_d_n5;
        locals.var_pzbsswg_dn6 = assign16620_e11438_d_n6;
        locals.var_pzbsswg_dn7 = assign16620_e11438_d_n7;
        locals.var_pzbsswg_dn8 = assign16620_e11438_d_n8;
        locals.var_pzbsswg_dn9 = assign16620_e11438_d_n9;
        locals.var_pzbsswg_dn10 = assign16620_e11438_d_n10;
        locals.var_pzbsswg_dn13 = assign16620_e11438_d_n13;
        locals.var_pzbsswg_rv = 0.0;

        let assign16630_e11441: f64 = (p.p87 * (nv5 - nv7));
        locals.var_vdsi = assign16630_e11441;
        locals.var_vdsi_dn5 = p.p87;
        locals.var_vdsi_dn7 = (-p.p87);
        locals.var_vdsi_rv = 0.0;

        let assign16640_e11444: f64 = (p.p87 * (nv6 - nv7));
        locals.var_vgsi = assign16640_e11444;
        locals.var_vgsi_dn6 = p.p87;
        locals.var_vgsi_dn7 = (-p.p87);
        locals.var_vgsi_rv = 0.0;

        let assign16650_e11447: f64 = (p.p87 * (nv8 - nv7));
        locals.var_vbsi = assign16650_e11447;
        locals.var_vbsi_dn7 = (-p.p87);
        locals.var_vbsi_dn8 = p.p87;
        locals.var_vbsi_rv = 0.0;

        let assign16660_e11450: f64 = (p.p87 * (nv0 - nv2));
        locals.var_vdsei = assign16660_e11450;
        locals.var_vdsei_dn0 = p.p87;
        locals.var_vdsei_dn2 = (-p.p87);
        locals.var_vdsei_rv = 0.0;

        let assign16670_e11453: f64 = (p.p87 * (nv6 - nv2));
        locals.var_vgsei = assign16670_e11453;
        locals.var_vgsei_dn2 = (-p.p87);
        locals.var_vgsei_dn6 = p.p87;
        locals.var_vgsei_rv = 0.0;

        let assign16680_e11456: f64 = (p.p87 * (nv8 - nv2));
        locals.var_vbsei = assign16680_e11456;
        locals.var_vbsei_dn2 = (-p.p87);
        locals.var_vbsei_dn8 = p.p87;
        locals.var_vbsei_rv = 0.0;

        let assign16690_e11459: f64 = (p.p87 * (nv0 - nv5));
        locals.var_vddp = assign16690_e11459;
        locals.var_vddp_dn0 = p.p87;
        locals.var_vddp_dn5 = (-p.p87);
        locals.var_vddp_rv = 0.0;

        let assign16700_e11462: f64 = (p.p87 * (nv7 - nv2));
        locals.var_vsps = assign16700_e11462;
        locals.var_vsps_dn2 = (-p.p87);
        locals.var_vsps_dn7 = p.p87;
        locals.var_vsps_rv = 0.0;

        let assign16710_e11465: f64 = (p.p87 * (nv10 - nv2));
        locals.var_vsbs = assign16710_e11465;
        locals.var_vsbs_dn2 = (-p.p87);
        locals.var_vsbs_dn10 = p.p87;
        locals.var_vsbs_rv = 0.0;

        let assign16720_e11468: f64 = (p.p87 * (nv9 - nv0));
        locals.var_vdbd = assign16720_e11468;
        locals.var_vdbd_dn0 = (-p.p87);
        locals.var_vdbd_dn9 = p.p87;
        locals.var_vdbd_rv = 0.0;

        let assign16730_e11471: f64 = (p.p87 * (nv8 - nv7));
        locals.var_vbpsp = assign16730_e11471;
        locals.var_vbpsp_dn7 = (-p.p87);
        locals.var_vbpsp_dn8 = p.p87;
        locals.var_vbpsp_rv = 0.0;

        let assign16740_e11474: f64 = (p.p87 * (nv8 - nv5));
        locals.var_vbpdp = assign16740_e11474;
        locals.var_vbpdp_dn5 = (-p.p87);
        locals.var_vbpdp_dn8 = p.p87;
        locals.var_vbpdp_rv = 0.0;

        locals.var_vbs_jct = locals.var_vsbs;
        locals.var_vbs_jct_dn2 = locals.var_vsbs_dn2;
        locals.var_vbs_jct_dn10 = locals.var_vsbs_dn10;
        locals.var_vbs_jct_rv = 0.0;

        locals.var_vbd_jct = locals.var_vdbd;
        locals.var_vbd_jct_dn0 = locals.var_vdbd_dn0;
        locals.var_vbd_jct_dn9 = locals.var_vdbd_dn9;
        locals.var_vbd_jct_rv = 0.0;

        locals.var_vbsi_jct = locals.var_vbpsp;
        locals.var_vbsi_jct_dn7 = locals.var_vbpsp_dn7;
        locals.var_vbsi_jct_dn8 = locals.var_vbpsp_dn8;
        locals.var_vbsi_jct_rv = 0.0;

        locals.var_vbdi_jct = locals.var_vbpdp;
        locals.var_vbdi_jct_dn5 = locals.var_vbpdp_dn5;
        locals.var_vbdi_jct_dn8 = locals.var_vbpdp_dn8;
        locals.var_vbdi_jct_rv = 0.0;

        locals.var_vsubs = 0.0;
        locals.var_vsubs_rv = 0.0;

        let (assign16800_e11483, assign16800_e11483_d_n11,) = {
    if (locals.var_flg_nqs != 0.0) {
        ((nv11 - 0.0), 1.0,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn11,)
    }
};
        locals.var_qi_nqs = assign16800_e11483;
        locals.var_qi_nqs_dn11 = assign16800_e11483_d_n11;
        locals.var_qi_nqs_rv = 0.0;

        let (assign16810_e11487, assign16810_e11487_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        ((nv12 - 0.0), 1.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn12,)
    }
};
        locals.var_qb_nqs = assign16810_e11487;
        locals.var_qb_nqs_dn12 = assign16810_e11487_d_n12;
        locals.var_qb_nqs_rv = 0.0;

        let (assign16820_e11492, assign16820_e11492_d_n11,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn11,)
    }
};
        locals.var_qi_nqs = assign16820_e11492;
        locals.var_qi_nqs_dn11 = assign16820_e11492_d_n11;
        locals.var_qi_nqs_rv = 0.0;

        let (assign16830_e11497, assign16830_e11497_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn12,)
    }
};
        locals.var_qb_nqs = assign16830_e11497;
        locals.var_qb_nqs_dn12 = assign16830_e11497_d_n12;
        locals.var_qb_nqs_rv = 0.0;

        let assign16840_e11500: f64 = (locals.var_vgsi - locals.var_vdsi);
        locals.var_vgd = assign16840_e11500;
        locals.var_vgd_dn5 = (-locals.var_vdsi_dn5);
        locals.var_vgd_dn6 = locals.var_vgsi_dn6;
        locals.var_vgd_dn7 = (locals.var_vgsi_dn7 - locals.var_vdsi_dn7);
        locals.var_vgd_rv = 0.0;

        let assign16850_e11503: f64 = (locals.var_vbsi - locals.var_vdsi);
        locals.var_vbd = assign16850_e11503;
        locals.var_vbd_dn5 = (-locals.var_vdsi_dn5);
        locals.var_vbd_dn7 = (locals.var_vbsi_dn7 - locals.var_vdsi_dn7);
        locals.var_vbd_dn8 = locals.var_vbsi_dn8;
        locals.var_vbd_rv = 0.0;

        let assign16860_e11506: f64 = if locals.var_vdsi >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard349 = assign16860_e11506;
        locals.var_guard349_rv = 0.0;

        let (assign16870_e11510,) = {
    if (locals.var_guard349 != 0.0) {
        (1.0,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign16870_e11510;
        locals.var_mode_rv = 0.0;

        let (assign16880_e11514, assign16880_e11514_d_n0, assign16880_e11514_d_n2, assign16880_e11514_d_n4, assign16880_e11514_d_n5, assign16880_e11514_d_n6, assign16880_e11514_d_n7, assign16880_e11514_d_n8, assign16880_e11514_d_n9, assign16880_e11514_d_n10, assign16880_e11514_d_n13,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vdsi, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, locals.var_vdsi_dn7, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    }
};
        locals.var_vds = assign16880_e11514;
        locals.var_vds_dn0 = assign16880_e11514_d_n0;
        locals.var_vds_dn2 = assign16880_e11514_d_n2;
        locals.var_vds_dn4 = assign16880_e11514_d_n4;
        locals.var_vds_dn5 = assign16880_e11514_d_n5;
        locals.var_vds_dn6 = assign16880_e11514_d_n6;
        locals.var_vds_dn7 = assign16880_e11514_d_n7;
        locals.var_vds_dn8 = assign16880_e11514_d_n8;
        locals.var_vds_dn9 = assign16880_e11514_d_n9;
        locals.var_vds_dn10 = assign16880_e11514_d_n10;
        locals.var_vds_dn13 = assign16880_e11514_d_n13;
        locals.var_vds_rv = 0.0;

        let (assign16890_e11518, assign16890_e11518_d_n5, assign16890_e11518_d_n6, assign16890_e11518_d_n7,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vgsi, 0.0, locals.var_vgsi_dn6, locals.var_vgsi_dn7,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn5, locals.var_vgs_dn6, locals.var_vgs_dn7,)
    }
};
        locals.var_vgs = assign16890_e11518;
        locals.var_vgs_dn5 = assign16890_e11518_d_n5;
        locals.var_vgs_dn6 = assign16890_e11518_d_n6;
        locals.var_vgs_dn7 = assign16890_e11518_d_n7;
        locals.var_vgs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_37(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let (assign16900_e11522, assign16900_e11522_d_n5, assign16900_e11522_d_n7, assign16900_e11522_d_n8,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vbsi, 0.0, locals.var_vbsi_dn7, locals.var_vbsi_dn8,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn5, locals.var_vbs_dn7, locals.var_vbs_dn8,)
    }
};
        locals.var_vbs = assign16900_e11522;
        locals.var_vbs_dn5 = assign16900_e11522_d_n5;
        locals.var_vbs_dn7 = assign16900_e11522_d_n7;
        locals.var_vbs_dn8 = assign16900_e11522_d_n8;
        locals.var_vbs_rv = 0.0;

        let (assign16910_e11526, assign16910_e11526_d_n0, assign16910_e11526_d_n2,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    }
};
        locals.var_vdse = assign16910_e11526;
        locals.var_vdse_dn0 = assign16910_e11526_d_n0;
        locals.var_vdse_dn2 = assign16910_e11526_d_n2;
        locals.var_vdse_rv = 0.0;

        let (assign16920_e11530, assign16920_e11530_d_n0, assign16920_e11530_d_n2, assign16920_e11530_d_n6,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vgsei, 0.0, locals.var_vgsei_dn2, locals.var_vgsei_dn6,)
    } else {
        (locals.var_vgse, locals.var_vgse_dn0, locals.var_vgse_dn2, locals.var_vgse_dn6,)
    }
};
        locals.var_vgse = assign16920_e11530;
        locals.var_vgse_dn0 = assign16920_e11530_d_n0;
        locals.var_vgse_dn2 = assign16920_e11530_d_n2;
        locals.var_vgse_dn6 = assign16920_e11530_d_n6;
        locals.var_vgse_rv = 0.0;

        let (assign16930_e11534, assign16930_e11534_d_n0, assign16930_e11534_d_n2, assign16930_e11534_d_n8,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vbsei, 0.0, locals.var_vbsei_dn2, locals.var_vbsei_dn8,)
    } else {
        (locals.var_vbse, locals.var_vbse_dn0, locals.var_vbse_dn2, locals.var_vbse_dn8,)
    }
};
        locals.var_vbse = assign16930_e11534;
        locals.var_vbse_dn0 = assign16930_e11534_d_n0;
        locals.var_vbse_dn2 = assign16930_e11534_d_n2;
        locals.var_vbse_dn8 = assign16930_e11534_d_n8;
        locals.var_vbse_rv = 0.0;

        let (assign16940_e11540,) = {
    if (locals.var_guard349 == 0.0) {
        let assign16940_e11538: f64 = (-1.0);
        (assign16940_e11538,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign16940_e11540;
        locals.var_mode_rv = 0.0;

        let (assign16950_e11546, assign16950_e11546_d_n0, assign16950_e11546_d_n2, assign16950_e11546_d_n4, assign16950_e11546_d_n5, assign16950_e11546_d_n6, assign16950_e11546_d_n7, assign16950_e11546_d_n8, assign16950_e11546_d_n9, assign16950_e11546_d_n10, assign16950_e11546_d_n13,) = {
    if (locals.var_guard349 == 0.0) {
        let assign16950_e11544: f64 = (-locals.var_vdsi);
        (assign16950_e11544, 0.0, 0.0, 0.0, (-locals.var_vdsi_dn5), 0.0, (-locals.var_vdsi_dn7), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    }
};
        locals.var_vds = assign16950_e11546;
        locals.var_vds_dn0 = assign16950_e11546_d_n0;
        locals.var_vds_dn2 = assign16950_e11546_d_n2;
        locals.var_vds_dn4 = assign16950_e11546_d_n4;
        locals.var_vds_dn5 = assign16950_e11546_d_n5;
        locals.var_vds_dn6 = assign16950_e11546_d_n6;
        locals.var_vds_dn7 = assign16950_e11546_d_n7;
        locals.var_vds_dn8 = assign16950_e11546_d_n8;
        locals.var_vds_dn9 = assign16950_e11546_d_n9;
        locals.var_vds_dn10 = assign16950_e11546_d_n10;
        locals.var_vds_dn13 = assign16950_e11546_d_n13;
        locals.var_vds_rv = 0.0;

        let (assign16960_e11551, assign16960_e11551_d_n5, assign16960_e11551_d_n6, assign16960_e11551_d_n7,) = {
    if (locals.var_guard349 == 0.0) {
        (locals.var_vgd, locals.var_vgd_dn5, locals.var_vgd_dn6, locals.var_vgd_dn7,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn5, locals.var_vgs_dn6, locals.var_vgs_dn7,)
    }
};
        locals.var_vgs = assign16960_e11551;
        locals.var_vgs_dn5 = assign16960_e11551_d_n5;
        locals.var_vgs_dn6 = assign16960_e11551_d_n6;
        locals.var_vgs_dn7 = assign16960_e11551_d_n7;
        locals.var_vgs_rv = 0.0;

        let (assign16970_e11556, assign16970_e11556_d_n5, assign16970_e11556_d_n7, assign16970_e11556_d_n8,) = {
    if (locals.var_guard349 == 0.0) {
        (locals.var_vbd, locals.var_vbd_dn5, locals.var_vbd_dn7, locals.var_vbd_dn8,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn5, locals.var_vbs_dn7, locals.var_vbs_dn8,)
    }
};
        locals.var_vbs = assign16970_e11556;
        locals.var_vbs_dn5 = assign16970_e11556_d_n5;
        locals.var_vbs_dn7 = assign16970_e11556_d_n7;
        locals.var_vbs_dn8 = assign16970_e11556_d_n8;
        locals.var_vbs_rv = 0.0;

        let (assign16980_e11562, assign16980_e11562_d_n0, assign16980_e11562_d_n2,) = {
    if (locals.var_guard349 == 0.0) {
        let assign16980_e11560: f64 = (-locals.var_vdsei);
        (assign16980_e11560, (-locals.var_vdsei_dn0), (-locals.var_vdsei_dn2),)
    } else {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    }
};
        locals.var_vdse = assign16980_e11562;
        locals.var_vdse_dn0 = assign16980_e11562_d_n0;
        locals.var_vdse_dn2 = assign16980_e11562_d_n2;
        locals.var_vdse_rv = 0.0;

        let (assign16990_e11569, assign16990_e11569_d_n0, assign16990_e11569_d_n2, assign16990_e11569_d_n6,) = {
    if (locals.var_guard349 == 0.0) {
        let assign16990_e11567: f64 = (locals.var_vgsei - locals.var_vdsei);
        (assign16990_e11567, (-locals.var_vdsei_dn0), (locals.var_vgsei_dn2 - locals.var_vdsei_dn2), locals.var_vgsei_dn6,)
    } else {
        (locals.var_vgse, locals.var_vgse_dn0, locals.var_vgse_dn2, locals.var_vgse_dn6,)
    }
};
        locals.var_vgse = assign16990_e11569;
        locals.var_vgse_dn0 = assign16990_e11569_d_n0;
        locals.var_vgse_dn2 = assign16990_e11569_d_n2;
        locals.var_vgse_dn6 = assign16990_e11569_d_n6;
        locals.var_vgse_rv = 0.0;

        let (assign17000_e11576, assign17000_e11576_d_n0, assign17000_e11576_d_n2, assign17000_e11576_d_n8,) = {
    if (locals.var_guard349 == 0.0) {
        let assign17000_e11574: f64 = (locals.var_vbsei - locals.var_vdsei);
        (assign17000_e11574, (-locals.var_vdsei_dn0), (locals.var_vbsei_dn2 - locals.var_vdsei_dn2), locals.var_vbsei_dn8,)
    } else {
        (locals.var_vbse, locals.var_vbse_dn0, locals.var_vbse_dn2, locals.var_vbse_dn8,)
    }
};
        locals.var_vbse = assign17000_e11576;
        locals.var_vbse_dn0 = assign17000_e11576_d_n0;
        locals.var_vbse_dn2 = assign17000_e11576_d_n2;
        locals.var_vbse_dn8 = assign17000_e11576_d_n8;
        locals.var_vbse_rv = 0.0;

        let assign17030_e11589: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard352 = assign17030_e11589;
        locals.var_guard352_rv = 0.0;

        let (assign17040_e11593, assign17040_e11593_d_n0, assign17040_e11593_d_n2, assign17040_e11593_d_n4, assign17040_e11593_d_n5, assign17040_e11593_d_n6, assign17040_e11593_d_n7, assign17040_e11593_d_n8, assign17040_e11593_d_n9, assign17040_e11593_d_n10, assign17040_e11593_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        ((nv4 - 0.0), 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn0, locals.var_deltemp_dn2, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6, locals.var_deltemp_dn7, locals.var_deltemp_dn8, locals.var_deltemp_dn9, locals.var_deltemp_dn10, locals.var_deltemp_dn13,)
    }
};
        locals.var_deltemp = assign17040_e11593;
        locals.var_deltemp_dn0 = assign17040_e11593_d_n0;
        locals.var_deltemp_dn2 = assign17040_e11593_d_n2;
        locals.var_deltemp_dn4 = assign17040_e11593_d_n4;
        locals.var_deltemp_dn5 = assign17040_e11593_d_n5;
        locals.var_deltemp_dn6 = assign17040_e11593_d_n6;
        locals.var_deltemp_dn7 = assign17040_e11593_d_n7;
        locals.var_deltemp_dn8 = assign17040_e11593_d_n8;
        locals.var_deltemp_dn9 = assign17040_e11593_d_n9;
        locals.var_deltemp_dn10 = assign17040_e11593_d_n10;
        locals.var_deltemp_dn13 = assign17040_e11593_d_n13;
        locals.var_deltemp_rv = 0.0;

        let assign17050_e11596: f64 = if p.p53 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard353 = assign17050_e11596;
        locals.var_guard353_rv = 0.0;

        let (assign17060_e11608, assign17060_e11608_d_n0, assign17060_e11608_d_n2, assign17060_e11608_d_n4, assign17060_e11608_d_n5, assign17060_e11608_d_n6, assign17060_e11608_d_n7, assign17060_e11608_d_n8, assign17060_e11608_d_n9, assign17060_e11608_d_n10, assign17060_e11608_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign17060_e11602: f64 = (p.p433 - locals.var_deltemp);
        let assign17060_e11605: f64 = (p.p337 * 10.0);
        let assign17060_e11606: f64 = (assign17060_e11602 - assign17060_e11605);
        (assign17060_e11606, (-locals.var_deltemp_dn0), (-locals.var_deltemp_dn2), (-locals.var_deltemp_dn4), (-locals.var_deltemp_dn5), (-locals.var_deltemp_dn6), (-locals.var_deltemp_dn7), (-locals.var_deltemp_dn8), (-locals.var_deltemp_dn9), (-locals.var_deltemp_dn10), (-locals.var_deltemp_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign17060_e11608;
        locals.var_tmf1_dn0 = assign17060_e11608_d_n0;
        locals.var_tmf1_dn2 = assign17060_e11608_d_n2;
        locals.var_tmf1_dn4 = assign17060_e11608_d_n4;
        locals.var_tmf1_dn5 = assign17060_e11608_d_n5;
        locals.var_tmf1_dn6 = assign17060_e11608_d_n6;
        locals.var_tmf1_dn7 = assign17060_e11608_d_n7;
        locals.var_tmf1_dn8 = assign17060_e11608_d_n8;
        locals.var_tmf1_dn9 = assign17060_e11608_d_n9;
        locals.var_tmf1_dn10 = assign17060_e11608_d_n10;
        locals.var_tmf1_dn13 = assign17060_e11608_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign17070_e11620, assign17070_e11620_d_n0, assign17070_e11620_d_n2, assign17070_e11620_d_n4, assign17070_e11620_d_n5, assign17070_e11620_d_n6, assign17070_e11620_d_n7, assign17070_e11620_d_n8, assign17070_e11620_d_n9, assign17070_e11620_d_n10, assign17070_e11620_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign17070_e11614: f64 = (4.0 * p.p433);
        let assign17070_e11617: f64 = (p.p337 * 10.0);
        let assign17070_e11618: f64 = (assign17070_e11614 * assign17070_e11617);
        (assign17070_e11618, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17070_e11620;
        locals.var_tmf2_dn0 = assign17070_e11620_d_n0;
        locals.var_tmf2_dn2 = assign17070_e11620_d_n2;
        locals.var_tmf2_dn4 = assign17070_e11620_d_n4;
        locals.var_tmf2_dn5 = assign17070_e11620_d_n5;
        locals.var_tmf2_dn6 = assign17070_e11620_d_n6;
        locals.var_tmf2_dn7 = assign17070_e11620_d_n7;
        locals.var_tmf2_dn8 = assign17070_e11620_d_n8;
        locals.var_tmf2_dn9 = assign17070_e11620_d_n9;
        locals.var_tmf2_dn10 = assign17070_e11620_d_n10;
        locals.var_tmf2_dn13 = assign17070_e11620_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign17080_e11632, assign17080_e11632_d_n0, assign17080_e11632_d_n2, assign17080_e11632_d_n4, assign17080_e11632_d_n5, assign17080_e11632_d_n6, assign17080_e11632_d_n7, assign17080_e11632_d_n8, assign17080_e11632_d_n9, assign17080_e11632_d_n10, assign17080_e11632_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let (assign17080_e11630, assign17080_e11630_d_n0, assign17080_e11630_d_n2, assign17080_e11630_d_n4, assign17080_e11630_d_n5, assign17080_e11630_d_n6, assign17080_e11630_d_n7, assign17080_e11630_d_n8, assign17080_e11630_d_n9, assign17080_e11630_d_n10, assign17080_e11630_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign17080_e11629: f64 = (-locals.var_tmf2);
                (assign17080_e11629, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign17080_e11630, assign17080_e11630_d_n0, assign17080_e11630_d_n2, assign17080_e11630_d_n4, assign17080_e11630_d_n5, assign17080_e11630_d_n6, assign17080_e11630_d_n7, assign17080_e11630_d_n8, assign17080_e11630_d_n9, assign17080_e11630_d_n10, assign17080_e11630_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17080_e11632;
        locals.var_tmf2_dn0 = assign17080_e11632_d_n0;
        locals.var_tmf2_dn2 = assign17080_e11632_d_n2;
        locals.var_tmf2_dn4 = assign17080_e11632_d_n4;
        locals.var_tmf2_dn5 = assign17080_e11632_d_n5;
        locals.var_tmf2_dn6 = assign17080_e11632_d_n6;
        locals.var_tmf2_dn7 = assign17080_e11632_d_n7;
        locals.var_tmf2_dn8 = assign17080_e11632_d_n8;
        locals.var_tmf2_dn9 = assign17080_e11632_d_n9;
        locals.var_tmf2_dn10 = assign17080_e11632_d_n10;
        locals.var_tmf2_dn13 = assign17080_e11632_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign17090_e11643, assign17090_e11643_d_n0, assign17090_e11643_d_n2, assign17090_e11643_d_n4, assign17090_e11643_d_n5, assign17090_e11643_d_n6, assign17090_e11643_d_n7, assign17090_e11643_d_n8, assign17090_e11643_d_n9, assign17090_e11643_d_n10, assign17090_e11643_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign17090_e11638: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign17090_e11640: f64 = (assign17090_e11638 + locals.var_tmf2);
        let assign17090_e11641: f64 = (assign17090_e11640).sqrt();
        (assign17090_e11641, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign17090_e11641)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17090_e11643;
        locals.var_tmf2_dn0 = assign17090_e11643_d_n0;
        locals.var_tmf2_dn2 = assign17090_e11643_d_n2;
        locals.var_tmf2_dn4 = assign17090_e11643_d_n4;
        locals.var_tmf2_dn5 = assign17090_e11643_d_n5;
        locals.var_tmf2_dn6 = assign17090_e11643_d_n6;
        locals.var_tmf2_dn7 = assign17090_e11643_d_n7;
        locals.var_tmf2_dn8 = assign17090_e11643_d_n8;
        locals.var_tmf2_dn9 = assign17090_e11643_d_n9;
        locals.var_tmf2_dn10 = assign17090_e11643_d_n10;
        locals.var_tmf2_dn13 = assign17090_e11643_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign17100_e11655, assign17100_e11655_d_n0, assign17100_e11655_d_n2, assign17100_e11655_d_n4, assign17100_e11655_d_n5, assign17100_e11655_d_n6, assign17100_e11655_d_n7, assign17100_e11655_d_n8, assign17100_e11655_d_n9, assign17100_e11655_d_n10, assign17100_e11655_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign17100_e11651: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign17100_e11652: f64 = (1.0 + assign17100_e11651);
        let assign17100_e11653: f64 = (0.5 * assign17100_e11652);
        (assign17100_e11653, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign17100_e11655;
        locals.var_t0_dn0 = assign17100_e11655_d_n0;
        locals.var_t0_dn2 = assign17100_e11655_d_n2;
        locals.var_t0_dn4 = assign17100_e11655_d_n4;
        locals.var_t0_dn5 = assign17100_e11655_d_n5;
        locals.var_t0_dn6 = assign17100_e11655_d_n6;
        locals.var_t0_dn7 = assign17100_e11655_d_n7;
        locals.var_t0_dn8 = assign17100_e11655_d_n8;
        locals.var_t0_dn9 = assign17100_e11655_d_n9;
        locals.var_t0_dn10 = assign17100_e11655_d_n10;
        locals.var_t0_dn13 = assign17100_e11655_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign17110_e11667, assign17110_e11667_d_n0, assign17110_e11667_d_n2, assign17110_e11667_d_n4, assign17110_e11667_d_n5, assign17110_e11667_d_n6, assign17110_e11667_d_n7, assign17110_e11667_d_n8, assign17110_e11667_d_n9, assign17110_e11667_d_n10, assign17110_e11667_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign17110_e11663: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign17110_e11664: f64 = (0.5 * assign17110_e11663);
        let assign17110_e11665: f64 = (p.p433 - assign17110_e11664);
        (assign17110_e11665, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn0, locals.var_deltemp_dn2, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6, locals.var_deltemp_dn7, locals.var_deltemp_dn8, locals.var_deltemp_dn9, locals.var_deltemp_dn10, locals.var_deltemp_dn13,)
    }
};
        locals.var_deltemp = assign17110_e11667;
        locals.var_deltemp_dn0 = assign17110_e11667_d_n0;
        locals.var_deltemp_dn2 = assign17110_e11667_d_n2;
        locals.var_deltemp_dn4 = assign17110_e11667_d_n4;
        locals.var_deltemp_dn5 = assign17110_e11667_d_n5;
        locals.var_deltemp_dn6 = assign17110_e11667_d_n6;
        locals.var_deltemp_dn7 = assign17110_e11667_d_n7;
        locals.var_deltemp_dn8 = assign17110_e11667_d_n8;
        locals.var_deltemp_dn9 = assign17110_e11667_d_n9;
        locals.var_deltemp_dn10 = assign17110_e11667_d_n10;
        locals.var_deltemp_dn13 = assign17110_e11667_d_n13;
        locals.var_deltemp_rv = 0.0;

        let (assign17130_e11676, assign17130_e11676_d_n0, assign17130_e11676_d_n2, assign17130_e11676_d_n4, assign17130_e11676_d_n5, assign17130_e11676_d_n6, assign17130_e11676_d_n7, assign17130_e11676_d_n8, assign17130_e11676_d_n9, assign17130_e11676_d_n10, assign17130_e11676_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17130_e11672: f64 = ctx_temp;
        let assign17130_e11674: f64 = (assign17130_e11672 + p.p11);
        (assign17130_e11674, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign17130_e11676;
        locals.var_ttemp_dn0 = assign17130_e11676_d_n0;
        locals.var_ttemp_dn2 = assign17130_e11676_d_n2;
        locals.var_ttemp_dn4 = assign17130_e11676_d_n4;
        locals.var_ttemp_dn5 = assign17130_e11676_d_n5;
        locals.var_ttemp_dn6 = assign17130_e11676_d_n6;
        locals.var_ttemp_dn7 = assign17130_e11676_d_n7;
        locals.var_ttemp_dn8 = assign17130_e11676_d_n8;
        locals.var_ttemp_dn9 = assign17130_e11676_d_n9;
        locals.var_ttemp_dn10 = assign17130_e11676_d_n10;
        locals.var_ttemp_dn13 = assign17130_e11676_d_n13;
        locals.var_ttemp_rv = 0.0;

        let (assign17140_e11680, assign17140_e11680_d_n0, assign17140_e11680_d_n2, assign17140_e11680_d_n4, assign17140_e11680_d_n5, assign17140_e11680_d_n6, assign17140_e11680_d_n7, assign17140_e11680_d_n8, assign17140_e11680_d_n9, assign17140_e11680_d_n10, assign17140_e11680_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    } else {
        (locals.var_ttemp0, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn13,)
    }
};
        locals.var_ttemp0 = assign17140_e11680;
        locals.var_ttemp0_dn0 = assign17140_e11680_d_n0;
        locals.var_ttemp0_dn2 = assign17140_e11680_d_n2;
        locals.var_ttemp0_dn4 = assign17140_e11680_d_n4;
        locals.var_ttemp0_dn5 = assign17140_e11680_d_n5;
        locals.var_ttemp0_dn6 = assign17140_e11680_d_n6;
        locals.var_ttemp0_dn7 = assign17140_e11680_d_n7;
        locals.var_ttemp0_dn8 = assign17140_e11680_d_n8;
        locals.var_ttemp0_dn9 = assign17140_e11680_d_n9;
        locals.var_ttemp0_dn10 = assign17140_e11680_d_n10;
        locals.var_ttemp0_dn13 = assign17140_e11680_d_n13;
        locals.var_ttemp0_rv = 0.0;

        let (assign17150_e11686, assign17150_e11686_d_n0, assign17150_e11686_d_n2, assign17150_e11686_d_n4, assign17150_e11686_d_n5, assign17150_e11686_d_n6, assign17150_e11686_d_n7, assign17150_e11686_d_n8, assign17150_e11686_d_n9, assign17150_e11686_d_n10, assign17150_e11686_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17150_e11684: f64 = (locals.var_ttemp + locals.var_deltemp);
        (assign17150_e11684, (locals.var_ttemp_dn0 + locals.var_deltemp_dn0), (locals.var_ttemp_dn2 + locals.var_deltemp_dn2), (locals.var_ttemp_dn4 + locals.var_deltemp_dn4), (locals.var_ttemp_dn5 + locals.var_deltemp_dn5), (locals.var_ttemp_dn6 + locals.var_deltemp_dn6), (locals.var_ttemp_dn7 + locals.var_deltemp_dn7), (locals.var_ttemp_dn8 + locals.var_deltemp_dn8), (locals.var_ttemp_dn9 + locals.var_deltemp_dn9), (locals.var_ttemp_dn10 + locals.var_deltemp_dn10), (locals.var_ttemp_dn13 + locals.var_deltemp_dn13),)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign17150_e11686;
        locals.var_ttemp_dn0 = assign17150_e11686_d_n0;
        locals.var_ttemp_dn2 = assign17150_e11686_d_n2;
        locals.var_ttemp_dn4 = assign17150_e11686_d_n4;
        locals.var_ttemp_dn5 = assign17150_e11686_d_n5;
        locals.var_ttemp_dn6 = assign17150_e11686_d_n6;
        locals.var_ttemp_dn7 = assign17150_e11686_d_n7;
        locals.var_ttemp_dn8 = assign17150_e11686_d_n8;
        locals.var_ttemp_dn9 = assign17150_e11686_d_n9;
        locals.var_ttemp_dn10 = assign17150_e11686_d_n10;
        locals.var_ttemp_dn13 = assign17150_e11686_d_n13;
        locals.var_ttemp_rv = 0.0;

        let (assign17160_e11692, assign17160_e11692_d_n0, assign17160_e11692_d_n2, assign17160_e11692_d_n4, assign17160_e11692_d_n5, assign17160_e11692_d_n6, assign17160_e11692_d_n7, assign17160_e11692_d_n8, assign17160_e11692_d_n9, assign17160_e11692_d_n10, assign17160_e11692_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17160_e11690: f64 = (locals.var_ttemp0 - locals.var_ktnom);
        (assign17160_e11690, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn13,)
    } else {
        (locals.var_tdiff0, locals.var_tdiff0_dn0, locals.var_tdiff0_dn2, locals.var_tdiff0_dn4, locals.var_tdiff0_dn5, locals.var_tdiff0_dn6, locals.var_tdiff0_dn7, locals.var_tdiff0_dn8, locals.var_tdiff0_dn9, locals.var_tdiff0_dn10, locals.var_tdiff0_dn13,)
    }
};
        locals.var_tdiff0 = assign17160_e11692;
        locals.var_tdiff0_dn0 = assign17160_e11692_d_n0;
        locals.var_tdiff0_dn2 = assign17160_e11692_d_n2;
        locals.var_tdiff0_dn4 = assign17160_e11692_d_n4;
        locals.var_tdiff0_dn5 = assign17160_e11692_d_n5;
        locals.var_tdiff0_dn6 = assign17160_e11692_d_n6;
        locals.var_tdiff0_dn7 = assign17160_e11692_d_n7;
        locals.var_tdiff0_dn8 = assign17160_e11692_d_n8;
        locals.var_tdiff0_dn9 = assign17160_e11692_d_n9;
        locals.var_tdiff0_dn10 = assign17160_e11692_d_n10;
        locals.var_tdiff0_dn13 = assign17160_e11692_d_n13;
        locals.var_tdiff0_rv = 0.0;

        let (assign17170_e11702, assign17170_e11702_d_n0, assign17170_e11702_d_n2, assign17170_e11702_d_n4, assign17170_e11702_d_n5, assign17170_e11702_d_n6, assign17170_e11702_d_n7, assign17170_e11702_d_n8, assign17170_e11702_d_n9, assign17170_e11702_d_n10, assign17170_e11702_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17170_e11696: f64 = (locals.var_ttemp0 * locals.var_ttemp0);
        let assign17170_e11699: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign17170_e11700: f64 = (assign17170_e11696 - assign17170_e11699);
        (assign17170_e11700, ((locals.var_ttemp0_dn0 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn0)), ((locals.var_ttemp0_dn2 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn2)), ((locals.var_ttemp0_dn4 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn4)), ((locals.var_ttemp0_dn5 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn5)), ((locals.var_ttemp0_dn6 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn6)), ((locals.var_ttemp0_dn7 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn7)), ((locals.var_ttemp0_dn8 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn8)), ((locals.var_ttemp0_dn9 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn9)), ((locals.var_ttemp0_dn10 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn10)), ((locals.var_ttemp0_dn13 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn13)),)
    } else {
        (locals.var_tdiff0_2, locals.var_tdiff0_2_dn0, locals.var_tdiff0_2_dn2, locals.var_tdiff0_2_dn4, locals.var_tdiff0_2_dn5, locals.var_tdiff0_2_dn6, locals.var_tdiff0_2_dn7, locals.var_tdiff0_2_dn8, locals.var_tdiff0_2_dn9, locals.var_tdiff0_2_dn10, locals.var_tdiff0_2_dn13,)
    }
};
        locals.var_tdiff0_2 = assign17170_e11702;
        locals.var_tdiff0_2_dn0 = assign17170_e11702_d_n0;
        locals.var_tdiff0_2_dn2 = assign17170_e11702_d_n2;
        locals.var_tdiff0_2_dn4 = assign17170_e11702_d_n4;
        locals.var_tdiff0_2_dn5 = assign17170_e11702_d_n5;
        locals.var_tdiff0_2_dn6 = assign17170_e11702_d_n6;
        locals.var_tdiff0_2_dn7 = assign17170_e11702_d_n7;
        locals.var_tdiff0_2_dn8 = assign17170_e11702_d_n8;
        locals.var_tdiff0_2_dn9 = assign17170_e11702_d_n9;
        locals.var_tdiff0_2_dn10 = assign17170_e11702_d_n10;
        locals.var_tdiff0_2_dn13 = assign17170_e11702_d_n13;
        locals.var_tdiff0_2_rv = 0.0;

        let (assign17180_e11708, assign17180_e11708_d_n0, assign17180_e11708_d_n2, assign17180_e11708_d_n4, assign17180_e11708_d_n5, assign17180_e11708_d_n6, assign17180_e11708_d_n7, assign17180_e11708_d_n8, assign17180_e11708_d_n9, assign17180_e11708_d_n10, assign17180_e11708_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17180_e11706: f64 = (locals.var_ttemp - locals.var_ktnom);
        (assign17180_e11706, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    } else {
        (locals.var_tdiff, locals.var_tdiff_dn0, locals.var_tdiff_dn2, locals.var_tdiff_dn4, locals.var_tdiff_dn5, locals.var_tdiff_dn6, locals.var_tdiff_dn7, locals.var_tdiff_dn8, locals.var_tdiff_dn9, locals.var_tdiff_dn10, locals.var_tdiff_dn13,)
    }
};
        locals.var_tdiff = assign17180_e11708;
        locals.var_tdiff_dn0 = assign17180_e11708_d_n0;
        locals.var_tdiff_dn2 = assign17180_e11708_d_n2;
        locals.var_tdiff_dn4 = assign17180_e11708_d_n4;
        locals.var_tdiff_dn5 = assign17180_e11708_d_n5;
        locals.var_tdiff_dn6 = assign17180_e11708_d_n6;
        locals.var_tdiff_dn7 = assign17180_e11708_d_n7;
        locals.var_tdiff_dn8 = assign17180_e11708_d_n8;
        locals.var_tdiff_dn9 = assign17180_e11708_d_n9;
        locals.var_tdiff_dn10 = assign17180_e11708_d_n10;
        locals.var_tdiff_dn13 = assign17180_e11708_d_n13;
        locals.var_tdiff_rv = 0.0;

        let (assign17190_e11718, assign17190_e11718_d_n0, assign17190_e11718_d_n2, assign17190_e11718_d_n4, assign17190_e11718_d_n5, assign17190_e11718_d_n6, assign17190_e11718_d_n7, assign17190_e11718_d_n8, assign17190_e11718_d_n9, assign17190_e11718_d_n10, assign17190_e11718_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17190_e11712: f64 = (locals.var_ttemp * locals.var_ttemp);
        let assign17190_e11715: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign17190_e11716: f64 = (assign17190_e11712 - assign17190_e11715);
        (assign17190_e11716, ((locals.var_ttemp_dn0 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn0)), ((locals.var_ttemp_dn2 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn2)), ((locals.var_ttemp_dn4 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn4)), ((locals.var_ttemp_dn5 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn5)), ((locals.var_ttemp_dn6 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn6)), ((locals.var_ttemp_dn7 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn7)), ((locals.var_ttemp_dn8 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn8)), ((locals.var_ttemp_dn9 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn9)), ((locals.var_ttemp_dn10 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn10)), ((locals.var_ttemp_dn13 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_tdiff_2, locals.var_tdiff_2_dn0, locals.var_tdiff_2_dn2, locals.var_tdiff_2_dn4, locals.var_tdiff_2_dn5, locals.var_tdiff_2_dn6, locals.var_tdiff_2_dn7, locals.var_tdiff_2_dn8, locals.var_tdiff_2_dn9, locals.var_tdiff_2_dn10, locals.var_tdiff_2_dn13,)
    }
};
        locals.var_tdiff_2 = assign17190_e11718;
        locals.var_tdiff_2_dn0 = assign17190_e11718_d_n0;
        locals.var_tdiff_2_dn2 = assign17190_e11718_d_n2;
        locals.var_tdiff_2_dn4 = assign17190_e11718_d_n4;
        locals.var_tdiff_2_dn5 = assign17190_e11718_d_n5;
        locals.var_tdiff_2_dn6 = assign17190_e11718_d_n6;
        locals.var_tdiff_2_dn7 = assign17190_e11718_d_n7;
        locals.var_tdiff_2_dn8 = assign17190_e11718_d_n8;
        locals.var_tdiff_2_dn9 = assign17190_e11718_d_n9;
        locals.var_tdiff_2_dn10 = assign17190_e11718_d_n10;
        locals.var_tdiff_2_dn13 = assign17190_e11718_d_n13;
        locals.var_tdiff_2_rv = 0.0;

        let (assign17200_e11724, assign17200_e11724_d_n0, assign17200_e11724_d_n2, assign17200_e11724_d_n4, assign17200_e11724_d_n5, assign17200_e11724_d_n6, assign17200_e11724_d_n7, assign17200_e11724_d_n8, assign17200_e11724_d_n9, assign17200_e11724_d_n10, assign17200_e11724_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17200_e11722: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign17200_e11722, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn13 / locals.var_ktnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn0, locals.var_tratio_dn2, locals.var_tratio_dn4, locals.var_tratio_dn5, locals.var_tratio_dn6, locals.var_tratio_dn7, locals.var_tratio_dn8, locals.var_tratio_dn9, locals.var_tratio_dn10, locals.var_tratio_dn13,)
    }
};
        locals.var_tratio = assign17200_e11724;
        locals.var_tratio_dn0 = assign17200_e11724_d_n0;
        locals.var_tratio_dn2 = assign17200_e11724_d_n2;
        locals.var_tratio_dn4 = assign17200_e11724_d_n4;
        locals.var_tratio_dn5 = assign17200_e11724_d_n5;
        locals.var_tratio_dn6 = assign17200_e11724_d_n6;
        locals.var_tratio_dn7 = assign17200_e11724_d_n7;
        locals.var_tratio_dn8 = assign17200_e11724_d_n8;
        locals.var_tratio_dn9 = assign17200_e11724_d_n9;
        locals.var_tratio_dn10 = assign17200_e11724_d_n10;
        locals.var_tratio_dn13 = assign17200_e11724_d_n13;
        locals.var_tratio_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_38(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17210_e11729, assign17210_e11729_d_n0, assign17210_e11729_d_n2, assign17210_e11729_d_n4, assign17210_e11729_d_n5, assign17210_e11729_d_n6, assign17210_e11729_d_n7, assign17210_e11729_d_n8, assign17210_e11729_d_n9, assign17210_e11729_d_n10, assign17210_e11729_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17210_e11727: f64 = (locals.var_tratio).ln();
        (assign17210_e11727, (locals.var_tratio_dn0 / locals.var_tratio), (locals.var_tratio_dn2 / locals.var_tratio), (locals.var_tratio_dn4 / locals.var_tratio), (locals.var_tratio_dn5 / locals.var_tratio), (locals.var_tratio_dn6 / locals.var_tratio), (locals.var_tratio_dn7 / locals.var_tratio), (locals.var_tratio_dn8 / locals.var_tratio), (locals.var_tratio_dn9 / locals.var_tratio), (locals.var_tratio_dn10 / locals.var_tratio), (locals.var_tratio_dn13 / locals.var_tratio),)
    } else {
        (locals.var_log_tratio, locals.var_log_tratio_dn0, locals.var_log_tratio_dn2, locals.var_log_tratio_dn4, locals.var_log_tratio_dn5, locals.var_log_tratio_dn6, locals.var_log_tratio_dn7, locals.var_log_tratio_dn8, locals.var_log_tratio_dn9, locals.var_log_tratio_dn10, locals.var_log_tratio_dn13,)
    }
};
        locals.var_log_tratio = assign17210_e11729;
        locals.var_log_tratio_dn0 = assign17210_e11729_d_n0;
        locals.var_log_tratio_dn2 = assign17210_e11729_d_n2;
        locals.var_log_tratio_dn4 = assign17210_e11729_d_n4;
        locals.var_log_tratio_dn5 = assign17210_e11729_d_n5;
        locals.var_log_tratio_dn6 = assign17210_e11729_d_n6;
        locals.var_log_tratio_dn7 = assign17210_e11729_d_n7;
        locals.var_log_tratio_dn8 = assign17210_e11729_d_n8;
        locals.var_log_tratio_dn9 = assign17210_e11729_d_n9;
        locals.var_log_tratio_dn10 = assign17210_e11729_d_n10;
        locals.var_log_tratio_dn13 = assign17210_e11729_d_n13;
        locals.var_log_tratio_rv = 0.0;

        let (assign17220_e11741, assign17220_e11741_d_n0, assign17220_e11741_d_n2, assign17220_e11741_d_n4, assign17220_e11741_d_n5, assign17220_e11741_d_n6, assign17220_e11741_d_n7, assign17220_e11741_d_n8, assign17220_e11741_d_n9, assign17220_e11741_d_n10, assign17220_e11741_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17220_e11734: f64 = (locals.var_uc_bgtmp1 * locals.var_tdiff);
        let assign17220_e11735: f64 = (locals.var_egtnom - assign17220_e11734);
        let assign17220_e11738: f64 = (locals.var_uc_bgtmp2 * locals.var_tdiff_2);
        let assign17220_e11739: f64 = (assign17220_e11735 - assign17220_e11738);
        (assign17220_e11739, ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn0)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn0)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn2)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn2)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn4)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn4)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn5)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn5)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn6)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn6)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn7)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn7)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn8)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn8)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn9)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn9)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn10)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn10)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn13)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn13)),)
    } else {
        (locals.var_eg, locals.var_eg_dn0, locals.var_eg_dn2, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, locals.var_eg_dn10, locals.var_eg_dn13,)
    }
};
        locals.var_eg = assign17220_e11741;
        locals.var_eg_dn0 = assign17220_e11741_d_n0;
        locals.var_eg_dn2 = assign17220_e11741_d_n2;
        locals.var_eg_dn4 = assign17220_e11741_d_n4;
        locals.var_eg_dn5 = assign17220_e11741_d_n5;
        locals.var_eg_dn6 = assign17220_e11741_d_n6;
        locals.var_eg_dn7 = assign17220_e11741_d_n7;
        locals.var_eg_dn8 = assign17220_e11741_d_n8;
        locals.var_eg_dn9 = assign17220_e11741_d_n9;
        locals.var_eg_dn10 = assign17220_e11741_d_n10;
        locals.var_eg_dn13 = assign17220_e11741_d_n13;
        locals.var_eg_rv = 0.0;

        let (assign17230_e11746, assign17230_e11746_d_n0, assign17230_e11746_d_n2, assign17230_e11746_d_n4, assign17230_e11746_d_n5, assign17230_e11746_d_n6, assign17230_e11746_d_n7, assign17230_e11746_d_n8, assign17230_e11746_d_n9, assign17230_e11746_d_n10, assign17230_e11746_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17230_e11744: f64 = (locals.var_eg).sqrt();
        (assign17230_e11744, (locals.var_eg_dn0 / (2.0 * assign17230_e11744)), (locals.var_eg_dn2 / (2.0 * assign17230_e11744)), (locals.var_eg_dn4 / (2.0 * assign17230_e11744)), (locals.var_eg_dn5 / (2.0 * assign17230_e11744)), (locals.var_eg_dn6 / (2.0 * assign17230_e11744)), (locals.var_eg_dn7 / (2.0 * assign17230_e11744)), (locals.var_eg_dn8 / (2.0 * assign17230_e11744)), (locals.var_eg_dn9 / (2.0 * assign17230_e11744)), (locals.var_eg_dn10 / (2.0 * assign17230_e11744)), (locals.var_eg_dn13 / (2.0 * assign17230_e11744)),)
    } else {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn13,)
    }
};
        locals.var_sqrt_eg = assign17230_e11746;
        locals.var_sqrt_eg_dn0 = assign17230_e11746_d_n0;
        locals.var_sqrt_eg_dn2 = assign17230_e11746_d_n2;
        locals.var_sqrt_eg_dn4 = assign17230_e11746_d_n4;
        locals.var_sqrt_eg_dn5 = assign17230_e11746_d_n5;
        locals.var_sqrt_eg_dn6 = assign17230_e11746_d_n6;
        locals.var_sqrt_eg_dn7 = assign17230_e11746_d_n7;
        locals.var_sqrt_eg_dn8 = assign17230_e11746_d_n8;
        locals.var_sqrt_eg_dn9 = assign17230_e11746_d_n9;
        locals.var_sqrt_eg_dn10 = assign17230_e11746_d_n10;
        locals.var_sqrt_eg_dn13 = assign17230_e11746_d_n13;
        locals.var_sqrt_eg_rv = 0.0;

        let (assign17240_e11752, assign17240_e11752_d_n0, assign17240_e11752_d_n2, assign17240_e11752_d_n4, assign17240_e11752_d_n5, assign17240_e11752_d_n6, assign17240_e11752_d_n7, assign17240_e11752_d_n8, assign17240_e11752_d_n9, assign17240_e11752_d_n10, assign17240_e11752_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17240_e11750: f64 = (1.0 / locals.var_ttemp);
        (assign17240_e11750, (-(locals.var_ttemp_dn0 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn2 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn4 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn5 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn6 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn7 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn8 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn9 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn10 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn13 / (locals.var_ttemp * locals.var_ttemp))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17240_e11752;
        locals.var_t1_dn0 = assign17240_e11752_d_n0;
        locals.var_t1_dn2 = assign17240_e11752_d_n2;
        locals.var_t1_dn4 = assign17240_e11752_d_n4;
        locals.var_t1_dn5 = assign17240_e11752_d_n5;
        locals.var_t1_dn6 = assign17240_e11752_d_n6;
        locals.var_t1_dn7 = assign17240_e11752_d_n7;
        locals.var_t1_dn8 = assign17240_e11752_d_n8;
        locals.var_t1_dn9 = assign17240_e11752_d_n9;
        locals.var_t1_dn10 = assign17240_e11752_d_n10;
        locals.var_t1_dn13 = assign17240_e11752_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17250_e11758, assign17250_e11758_d_n0, assign17250_e11758_d_n2, assign17250_e11758_d_n4, assign17250_e11758_d_n5, assign17250_e11758_d_n6, assign17250_e11758_d_n7, assign17250_e11758_d_n8, assign17250_e11758_d_n9, assign17250_e11758_d_n10, assign17250_e11758_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17250_e11756: f64 = (1.0 / locals.var_ktnom);
        (assign17250_e11756, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign17250_e11758;
        locals.var_t2_dn0 = assign17250_e11758_d_n0;
        locals.var_t2_dn2 = assign17250_e11758_d_n2;
        locals.var_t2_dn4 = assign17250_e11758_d_n4;
        locals.var_t2_dn5 = assign17250_e11758_d_n5;
        locals.var_t2_dn6 = assign17250_e11758_d_n6;
        locals.var_t2_dn7 = assign17250_e11758_d_n7;
        locals.var_t2_dn8 = assign17250_e11758_d_n8;
        locals.var_t2_dn9 = assign17250_e11758_d_n9;
        locals.var_t2_dn10 = assign17250_e11758_d_n10;
        locals.var_t2_dn13 = assign17250_e11758_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign17260_e11780, assign17260_e11780_d_n0, assign17260_e11780_d_n2, assign17260_e11780_d_n4, assign17260_e11780_d_n5, assign17260_e11780_d_n6, assign17260_e11780_d_n7, assign17260_e11780_d_n8, assign17260_e11780_d_n9, assign17260_e11780_d_n10, assign17260_e11780_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17260_e11762: f64 = (locals.var_egtnom + p.p259);
        let assign17260_e11766: f64 = (locals.var_t1 - locals.var_t2);
        let assign17260_e11767: f64 = (p.p260 * assign17260_e11766);
        let assign17260_e11768: f64 = (assign17260_e11762 + assign17260_e11767);
        let assign17260_e11772: f64 = (locals.var_t1 * locals.var_t1);
        let assign17260_e11775: f64 = (locals.var_t2 * locals.var_t2);
        let assign17260_e11776: f64 = (assign17260_e11772 - assign17260_e11775);
        let assign17260_e11777: f64 = (p.p261 * assign17260_e11776);
        let assign17260_e11778: f64 = (assign17260_e11768 + assign17260_e11777);
        (assign17260_e11778, ((p.p260 * (locals.var_t1_dn0 - locals.var_t2_dn0)) + (p.p261 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) - ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))))), ((p.p260 * (locals.var_t1_dn2 - locals.var_t2_dn2)) + (p.p261 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) - ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))))), ((p.p260 * (locals.var_t1_dn4 - locals.var_t2_dn4)) + (p.p261 * (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) - ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))))), ((p.p260 * (locals.var_t1_dn5 - locals.var_t2_dn5)) + (p.p261 * (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) - ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))))), ((p.p260 * (locals.var_t1_dn6 - locals.var_t2_dn6)) + (p.p261 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) - ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))))), ((p.p260 * (locals.var_t1_dn7 - locals.var_t2_dn7)) + (p.p261 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) - ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))))), ((p.p260 * (locals.var_t1_dn8 - locals.var_t2_dn8)) + (p.p261 * (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) - ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))))), ((p.p260 * (locals.var_t1_dn9 - locals.var_t2_dn9)) + (p.p261 * (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) - ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))))), ((p.p260 * (locals.var_t1_dn10 - locals.var_t2_dn10)) + (p.p261 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) - ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))))), ((p.p260 * (locals.var_t1_dn13 - locals.var_t2_dn13)) + (p.p261 * (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) - ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign17260_e11780;
        locals.var_t3_dn0 = assign17260_e11780_d_n0;
        locals.var_t3_dn2 = assign17260_e11780_d_n2;
        locals.var_t3_dn4 = assign17260_e11780_d_n4;
        locals.var_t3_dn5 = assign17260_e11780_d_n5;
        locals.var_t3_dn6 = assign17260_e11780_d_n6;
        locals.var_t3_dn7 = assign17260_e11780_d_n7;
        locals.var_t3_dn8 = assign17260_e11780_d_n8;
        locals.var_t3_dn9 = assign17260_e11780_d_n9;
        locals.var_t3_dn10 = assign17260_e11780_d_n10;
        locals.var_t3_dn13 = assign17260_e11780_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign17270_e11785, assign17270_e11785_d_n0, assign17270_e11785_d_n2, assign17270_e11785_d_n4, assign17270_e11785_d_n5, assign17270_e11785_d_n6, assign17270_e11785_d_n7, assign17270_e11785_d_n8, assign17270_e11785_d_n9, assign17270_e11785_d_n10, assign17270_e11785_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17270_e11783: f64 = (locals.var_t3).sqrt();
        (assign17270_e11783, (locals.var_t3_dn0 / (2.0 * assign17270_e11783)), (locals.var_t3_dn2 / (2.0 * assign17270_e11783)), (locals.var_t3_dn4 / (2.0 * assign17270_e11783)), (locals.var_t3_dn5 / (2.0 * assign17270_e11783)), (locals.var_t3_dn6 / (2.0 * assign17270_e11783)), (locals.var_t3_dn7 / (2.0 * assign17270_e11783)), (locals.var_t3_dn8 / (2.0 * assign17270_e11783)), (locals.var_t3_dn9 / (2.0 * assign17270_e11783)), (locals.var_t3_dn10 / (2.0 * assign17270_e11783)), (locals.var_t3_dn13 / (2.0 * assign17270_e11783)),)
    } else {
        (locals.var_egp12, locals.var_egp12_dn0, locals.var_egp12_dn2, locals.var_egp12_dn4, locals.var_egp12_dn5, locals.var_egp12_dn6, locals.var_egp12_dn7, locals.var_egp12_dn8, locals.var_egp12_dn9, locals.var_egp12_dn10, locals.var_egp12_dn13,)
    }
};
        locals.var_egp12 = assign17270_e11785;
        locals.var_egp12_dn0 = assign17270_e11785_d_n0;
        locals.var_egp12_dn2 = assign17270_e11785_d_n2;
        locals.var_egp12_dn4 = assign17270_e11785_d_n4;
        locals.var_egp12_dn5 = assign17270_e11785_d_n5;
        locals.var_egp12_dn6 = assign17270_e11785_d_n6;
        locals.var_egp12_dn7 = assign17270_e11785_d_n7;
        locals.var_egp12_dn8 = assign17270_e11785_d_n8;
        locals.var_egp12_dn9 = assign17270_e11785_d_n9;
        locals.var_egp12_dn10 = assign17270_e11785_d_n10;
        locals.var_egp12_dn13 = assign17270_e11785_d_n13;
        locals.var_egp12_rv = 0.0;

        let (assign17280_e11791, assign17280_e11791_d_n0, assign17280_e11791_d_n2, assign17280_e11791_d_n4, assign17280_e11791_d_n5, assign17280_e11791_d_n6, assign17280_e11791_d_n7, assign17280_e11791_d_n8, assign17280_e11791_d_n9, assign17280_e11791_d_n10, assign17280_e11791_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17280_e11789: f64 = (locals.var_t3 * locals.var_egp12);
        (assign17280_e11789, ((locals.var_t3_dn0 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn0)), ((locals.var_t3_dn2 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn2)), ((locals.var_t3_dn4 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn4)), ((locals.var_t3_dn5 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn5)), ((locals.var_t3_dn6 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn6)), ((locals.var_t3_dn7 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn7)), ((locals.var_t3_dn8 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn8)), ((locals.var_t3_dn9 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn9)), ((locals.var_t3_dn10 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn10)), ((locals.var_t3_dn13 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn13)),)
    } else {
        (locals.var_egp32, locals.var_egp32_dn0, locals.var_egp32_dn2, locals.var_egp32_dn4, locals.var_egp32_dn5, locals.var_egp32_dn6, locals.var_egp32_dn7, locals.var_egp32_dn8, locals.var_egp32_dn9, locals.var_egp32_dn10, locals.var_egp32_dn13,)
    }
};
        locals.var_egp32 = assign17280_e11791;
        locals.var_egp32_dn0 = assign17280_e11791_d_n0;
        locals.var_egp32_dn2 = assign17280_e11791_d_n2;
        locals.var_egp32_dn4 = assign17280_e11791_d_n4;
        locals.var_egp32_dn5 = assign17280_e11791_d_n5;
        locals.var_egp32_dn6 = assign17280_e11791_d_n6;
        locals.var_egp32_dn7 = assign17280_e11791_d_n7;
        locals.var_egp32_dn8 = assign17280_e11791_d_n8;
        locals.var_egp32_dn9 = assign17280_e11791_d_n9;
        locals.var_egp32_dn10 = assign17280_e11791_d_n10;
        locals.var_egp32_dn13 = assign17280_e11791_d_n13;
        locals.var_egp32_rv = 0.0;

        let (assign17290_e11799, assign17290_e11799_d_n0, assign17290_e11799_d_n2, assign17290_e11799_d_n4, assign17290_e11799_d_n5, assign17290_e11799_d_n6, assign17290_e11799_d_n7, assign17290_e11799_d_n8, assign17290_e11799_d_n9, assign17290_e11799_d_n10, assign17290_e11799_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17290_e11796: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign17290_e11797: f64 = (1.6021918e-19 / assign17290_e11796);
        (assign17290_e11797, (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn0)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn2)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn4)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn5)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn6)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn7)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn8)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn9)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn13)) / (assign17290_e11796 * assign17290_e11796))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn13,)
    }
};
        locals.var_beta = assign17290_e11799;
        locals.var_beta_dn0 = assign17290_e11799_d_n0;
        locals.var_beta_dn2 = assign17290_e11799_d_n2;
        locals.var_beta_dn4 = assign17290_e11799_d_n4;
        locals.var_beta_dn5 = assign17290_e11799_d_n5;
        locals.var_beta_dn6 = assign17290_e11799_d_n6;
        locals.var_beta_dn7 = assign17290_e11799_d_n7;
        locals.var_beta_dn8 = assign17290_e11799_d_n8;
        locals.var_beta_dn9 = assign17290_e11799_d_n9;
        locals.var_beta_dn10 = assign17290_e11799_d_n10;
        locals.var_beta_dn13 = assign17290_e11799_d_n13;
        locals.var_beta_rv = 0.0;

        let (assign17300_e11805, assign17300_e11805_d_n0, assign17300_e11805_d_n2, assign17300_e11805_d_n4, assign17300_e11805_d_n5, assign17300_e11805_d_n6, assign17300_e11805_d_n7, assign17300_e11805_d_n8, assign17300_e11805_d_n9, assign17300_e11805_d_n10, assign17300_e11805_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17300_e11803: f64 = (1.0 / locals.var_beta);
        (assign17300_e11803, (-(locals.var_beta_dn0 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn2 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn4 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn5 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn6 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn7 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn8 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn9 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn13 / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_beta_inv, locals.var_beta_inv_dn0, locals.var_beta_inv_dn2, locals.var_beta_inv_dn4, locals.var_beta_inv_dn5, locals.var_beta_inv_dn6, locals.var_beta_inv_dn7, locals.var_beta_inv_dn8, locals.var_beta_inv_dn9, locals.var_beta_inv_dn10, locals.var_beta_inv_dn13,)
    }
};
        locals.var_beta_inv = assign17300_e11805;
        locals.var_beta_inv_dn0 = assign17300_e11805_d_n0;
        locals.var_beta_inv_dn2 = assign17300_e11805_d_n2;
        locals.var_beta_inv_dn4 = assign17300_e11805_d_n4;
        locals.var_beta_inv_dn5 = assign17300_e11805_d_n5;
        locals.var_beta_inv_dn6 = assign17300_e11805_d_n6;
        locals.var_beta_inv_dn7 = assign17300_e11805_d_n7;
        locals.var_beta_inv_dn8 = assign17300_e11805_d_n8;
        locals.var_beta_inv_dn9 = assign17300_e11805_d_n9;
        locals.var_beta_inv_dn10 = assign17300_e11805_d_n10;
        locals.var_beta_inv_dn13 = assign17300_e11805_d_n13;
        locals.var_beta_inv_rv = 0.0;

        let (assign17310_e11811, assign17310_e11811_d_n0, assign17310_e11811_d_n2, assign17310_e11811_d_n4, assign17310_e11811_d_n5, assign17310_e11811_d_n6, assign17310_e11811_d_n7, assign17310_e11811_d_n8, assign17310_e11811_d_n9, assign17310_e11811_d_n10, assign17310_e11811_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17310_e11809: f64 = (locals.var_beta * locals.var_beta);
        (assign17310_e11809, ((locals.var_beta_dn0 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn0)), ((locals.var_beta_dn2 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn2)), ((locals.var_beta_dn4 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn4)), ((locals.var_beta_dn5 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn5)), ((locals.var_beta_dn6 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn6)), ((locals.var_beta_dn7 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn7)), ((locals.var_beta_dn8 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn8)), ((locals.var_beta_dn9 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn9)), ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10)), ((locals.var_beta_dn13 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn13)),)
    } else {
        (locals.var_beta2, locals.var_beta2_dn0, locals.var_beta2_dn2, locals.var_beta2_dn4, locals.var_beta2_dn5, locals.var_beta2_dn6, locals.var_beta2_dn7, locals.var_beta2_dn8, locals.var_beta2_dn9, locals.var_beta2_dn10, locals.var_beta2_dn13,)
    }
};
        locals.var_beta2 = assign17310_e11811;
        locals.var_beta2_dn0 = assign17310_e11811_d_n0;
        locals.var_beta2_dn2 = assign17310_e11811_d_n2;
        locals.var_beta2_dn4 = assign17310_e11811_d_n4;
        locals.var_beta2_dn5 = assign17310_e11811_d_n5;
        locals.var_beta2_dn6 = assign17310_e11811_d_n6;
        locals.var_beta2_dn7 = assign17310_e11811_d_n7;
        locals.var_beta2_dn8 = assign17310_e11811_d_n8;
        locals.var_beta2_dn9 = assign17310_e11811_d_n9;
        locals.var_beta2_dn10 = assign17310_e11811_d_n10;
        locals.var_beta2_dn13 = assign17310_e11811_d_n13;
        locals.var_beta2_rv = 0.0;

        let (assign17320_e11819,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17320_e11816: f64 = (1.3806226e-23 * locals.var_ktnom);
        let assign17320_e11817: f64 = (1.6021918e-19 / assign17320_e11816);
        (assign17320_e11817,)
    } else {
        (locals.var_betatnom,)
    }
};
        locals.var_betatnom = assign17320_e11819;
        locals.var_betatnom_rv = 0.0;

        let (assign17330_e11842, assign17330_e11842_d_n0, assign17330_e11842_d_n2, assign17330_e11842_d_n4, assign17330_e11842_d_n5, assign17330_e11842_d_n6, assign17330_e11842_d_n7, assign17330_e11842_d_n8, assign17330_e11842_d_n9, assign17330_e11842_d_n10, assign17330_e11842_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17330_e11824: f64 = (locals.var_log_tratio * 1.5);
        let assign17330_e11825: f64 = (assign17330_e11824).exp();
        let assign17330_e11826: f64 = (1.04e16 * assign17330_e11825);
        let assign17330_e11828: f64 = (-locals.var_eg);
        let assign17330_e11830: f64 = (assign17330_e11828 / 2.0);
        let assign17330_e11832: f64 = (assign17330_e11830 * locals.var_beta);
        let assign17330_e11835: f64 = (locals.var_egtnom / 2.0);
        let assign17330_e11837: f64 = (assign17330_e11835 * locals.var_betatnom);
        let assign17330_e11838: f64 = (assign17330_e11832 + assign17330_e11837);
        let assign17330_e11839: f64 = (assign17330_e11838).exp();
        let assign17330_e11840: f64 = (assign17330_e11826 * assign17330_e11839);
        (assign17330_e11840, (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn0 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn0) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn0))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn2 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn2) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn2))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn4 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn4) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn4))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn5 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn5) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn5))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn6 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn6) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn6))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn7 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn7) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn7))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn8 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn8) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn8))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn9 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn9) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn9))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn10 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn10) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn10))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn13 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn13) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn13))))),)
    } else {
        (locals.var_nin, locals.var_nin_dn0, locals.var_nin_dn2, locals.var_nin_dn4, locals.var_nin_dn5, locals.var_nin_dn6, locals.var_nin_dn7, locals.var_nin_dn8, locals.var_nin_dn9, locals.var_nin_dn10, locals.var_nin_dn13,)
    }
};
        locals.var_nin = assign17330_e11842;
        locals.var_nin_dn0 = assign17330_e11842_d_n0;
        locals.var_nin_dn2 = assign17330_e11842_d_n2;
        locals.var_nin_dn4 = assign17330_e11842_d_n4;
        locals.var_nin_dn5 = assign17330_e11842_d_n5;
        locals.var_nin_dn6 = assign17330_e11842_d_n6;
        locals.var_nin_dn7 = assign17330_e11842_d_n7;
        locals.var_nin_dn8 = assign17330_e11842_d_n8;
        locals.var_nin_dn9 = assign17330_e11842_d_n9;
        locals.var_nin_dn10 = assign17330_e11842_d_n10;
        locals.var_nin_dn13 = assign17330_e11842_d_n13;
        locals.var_nin_rv = 0.0;

        let (assign17340_e11849, assign17340_e11849_d_n0, assign17340_e11849_d_n2, assign17340_e11849_d_n4, assign17340_e11849_d_n5, assign17340_e11849_d_n6, assign17340_e11849_d_n7, assign17340_e11849_d_n8, assign17340_e11849_d_n9, assign17340_e11849_d_n10, assign17340_e11849_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17340_e11846: f64 = (locals.var_log_tratio * locals.var_uc_muetmp);
        let assign17340_e11847: f64 = (assign17340_e11846).exp();
        (assign17340_e11847, (assign17340_e11847 * (locals.var_log_tratio_dn0 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn2 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn4 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn5 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn6 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn7 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn8 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn9 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn10 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn13 * locals.var_uc_muetmp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17340_e11849;
        locals.var_t1_dn0 = assign17340_e11849_d_n0;
        locals.var_t1_dn2 = assign17340_e11849_d_n2;
        locals.var_t1_dn4 = assign17340_e11849_d_n4;
        locals.var_t1_dn5 = assign17340_e11849_d_n5;
        locals.var_t1_dn6 = assign17340_e11849_d_n6;
        locals.var_t1_dn7 = assign17340_e11849_d_n7;
        locals.var_t1_dn8 = assign17340_e11849_d_n8;
        locals.var_t1_dn9 = assign17340_e11849_d_n9;
        locals.var_t1_dn10 = assign17340_e11849_d_n10;
        locals.var_t1_dn13 = assign17340_e11849_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17350_e11855, assign17350_e11855_d_n0, assign17350_e11855_d_n2, assign17350_e11855_d_n4, assign17350_e11855_d_n5, assign17350_e11855_d_n6, assign17350_e11855_d_n7, assign17350_e11855_d_n8, assign17350_e11855_d_n9, assign17350_e11855_d_n10, assign17350_e11855_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17350_e11853: f64 = (locals.var_t1 / locals.var_mueph);
        (assign17350_e11853, (((locals.var_t1_dn0 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn0)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn2 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn2)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn4 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn4)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn5 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn5)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn6 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn6)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn7 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn7)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn8 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn8)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn9 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn9)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn10 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn10)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn13 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn13)) / (locals.var_mueph * locals.var_mueph)),)
    } else {
        (locals.var_mphn0, locals.var_mphn0_dn0, locals.var_mphn0_dn2, locals.var_mphn0_dn4, locals.var_mphn0_dn5, locals.var_mphn0_dn6, locals.var_mphn0_dn7, locals.var_mphn0_dn8, locals.var_mphn0_dn9, locals.var_mphn0_dn10, locals.var_mphn0_dn13,)
    }
};
        locals.var_mphn0 = assign17350_e11855;
        locals.var_mphn0_dn0 = assign17350_e11855_d_n0;
        locals.var_mphn0_dn2 = assign17350_e11855_d_n2;
        locals.var_mphn0_dn4 = assign17350_e11855_d_n4;
        locals.var_mphn0_dn5 = assign17350_e11855_d_n5;
        locals.var_mphn0_dn6 = assign17350_e11855_d_n6;
        locals.var_mphn0_dn7 = assign17350_e11855_d_n7;
        locals.var_mphn0_dn8 = assign17350_e11855_d_n8;
        locals.var_mphn0_dn9 = assign17350_e11855_d_n9;
        locals.var_mphn0_dn10 = assign17350_e11855_d_n10;
        locals.var_mphn0_dn13 = assign17350_e11855_d_n13;
        locals.var_mphn0_rv = 0.0;

        let assign17360_e11862: f64 = if ((locals.var_uc_codep != 0.0) && (locals.var_uc_codep < 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard355 = assign17360_e11862;
        locals.var_guard355_rv = 0.0;

        let (assign17370_e11877, assign17370_e11877_d_n0, assign17370_e11877_d_n2, assign17370_e11877_d_n4, assign17370_e11877_d_n5, assign17370_e11877_d_n6, assign17370_e11877_d_n7, assign17370_e11877_d_n8, assign17370_e11877_d_n9, assign17370_e11877_d_n10, assign17370_e11877_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17370_e11868: f64 = (2.0 * 1.034943e-10);
        let assign17370_e11870: f64 = (assign17370_e11868 * 1.6021918e-19);
        let assign17370_e11872: f64 = (assign17370_e11870 * locals.var_uc_ndepm);
        let assign17370_e11874: f64 = (assign17370_e11872 * locals.var_beta_inv);
        let assign17370_e11875: f64 = (assign17370_e11874).sqrt();
        (assign17370_e11875, ((((assign17370_e11870 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn0)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn2)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn4)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn5)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn6)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn7)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn8)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn9)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn10)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn13) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn13)) / (2.0 * assign17370_e11875)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign17370_e11877;
        locals.var_cnst0_dn0 = assign17370_e11877_d_n0;
        locals.var_cnst0_dn2 = assign17370_e11877_d_n2;
        locals.var_cnst0_dn4 = assign17370_e11877_d_n4;
        locals.var_cnst0_dn5 = assign17370_e11877_d_n5;
        locals.var_cnst0_dn6 = assign17370_e11877_d_n6;
        locals.var_cnst0_dn7 = assign17370_e11877_d_n7;
        locals.var_cnst0_dn8 = assign17370_e11877_d_n8;
        locals.var_cnst0_dn9 = assign17370_e11877_d_n9;
        locals.var_cnst0_dn10 = assign17370_e11877_d_n10;
        locals.var_cnst0_dn13 = assign17370_e11877_d_n13;
        locals.var_cnst0_rv = 0.0;

        let (assign17380_e11889, assign17380_e11889_d_n0, assign17380_e11889_d_n2, assign17380_e11889_d_n4, assign17380_e11889_d_n5, assign17380_e11889_d_n6, assign17380_e11889_d_n7, assign17380_e11889_d_n8, assign17380_e11889_d_n9, assign17380_e11889_d_n10, assign17380_e11889_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17380_e11883: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign17380_e11885: f64 = (assign17380_e11883 * __rspice_inv_cse_0);
        let assign17380_e11887: f64 = (assign17380_e11885 * __rspice_inv_cse_0);
        (assign17380_e11887, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign17380_e11889;
        locals.var_cnst1_dn0 = assign17380_e11889_d_n0;
        locals.var_cnst1_dn2 = assign17380_e11889_d_n2;
        locals.var_cnst1_dn4 = assign17380_e11889_d_n4;
        locals.var_cnst1_dn5 = assign17380_e11889_d_n5;
        locals.var_cnst1_dn6 = assign17380_e11889_d_n6;
        locals.var_cnst1_dn7 = assign17380_e11889_d_n7;
        locals.var_cnst1_dn8 = assign17380_e11889_d_n8;
        locals.var_cnst1_dn9 = assign17380_e11889_d_n9;
        locals.var_cnst1_dn10 = assign17380_e11889_d_n10;
        locals.var_cnst1_dn13 = assign17380_e11889_d_n13;
        locals.var_cnst1_rv = 0.0;

        let (assign17390_e11902, assign17390_e11902_d_n0, assign17390_e11902_d_n2, assign17390_e11902_d_n4, assign17390_e11902_d_n5, assign17390_e11902_d_n6, assign17390_e11902_d_n7, assign17390_e11902_d_n8, assign17390_e11902_d_n9, assign17390_e11902_d_n10, assign17390_e11902_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17390_e11895: f64 = (2.0 * locals.var_beta_inv);
        let assign17390_e11898: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign17390_e11899: f64 = (assign17390_e11898).ln();
        let assign17390_e11900: f64 = (assign17390_e11895 * assign17390_e11899);
        (assign17390_e11900, (((2.0 * locals.var_beta_inv_dn0) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn2) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn4) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn5) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn6) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn7) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn8) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn9) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn10) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn13) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn13 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign17390_e11902;
        locals.var_pb2n_dn0 = assign17390_e11902_d_n0;
        locals.var_pb2n_dn2 = assign17390_e11902_d_n2;
        locals.var_pb2n_dn4 = assign17390_e11902_d_n4;
        locals.var_pb2n_dn5 = assign17390_e11902_d_n5;
        locals.var_pb2n_dn6 = assign17390_e11902_d_n6;
        locals.var_pb2n_dn7 = assign17390_e11902_d_n7;
        locals.var_pb2n_dn8 = assign17390_e11902_d_n8;
        locals.var_pb2n_dn9 = assign17390_e11902_d_n9;
        locals.var_pb2n_dn10 = assign17390_e11902_d_n10;
        locals.var_pb2n_dn13 = assign17390_e11902_d_n13;
        locals.var_pb2n_rv = 0.0;

        let (assign17400_e11917, assign17400_e11917_d_n0, assign17400_e11917_d_n2, assign17400_e11917_d_n4, assign17400_e11917_d_n5, assign17400_e11917_d_n6, assign17400_e11917_d_n7, assign17400_e11917_d_n8, assign17400_e11917_d_n9, assign17400_e11917_d_n10, assign17400_e11917_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17400_e11909: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign17400_e11911: f64 = (assign17400_e11909 * __rspice_inv_cse_1);
        let assign17400_e11913: f64 = (assign17400_e11911 * __rspice_inv_cse_1);
        let assign17400_e11914: f64 = (assign17400_e11913).ln();
        let assign17400_e11915: f64 = (locals.var_beta_inv * assign17400_e11914);
        (assign17400_e11915, ((locals.var_beta_inv_dn0 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn2 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn4 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn5 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn6 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn7 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn8 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn9 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn10 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn13 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign17400_e11917;
        locals.var_vbipn_dn0 = assign17400_e11917_d_n0;
        locals.var_vbipn_dn2 = assign17400_e11917_d_n2;
        locals.var_vbipn_dn4 = assign17400_e11917_d_n4;
        locals.var_vbipn_dn5 = assign17400_e11917_d_n5;
        locals.var_vbipn_dn6 = assign17400_e11917_d_n6;
        locals.var_vbipn_dn7 = assign17400_e11917_d_n7;
        locals.var_vbipn_dn8 = assign17400_e11917_d_n8;
        locals.var_vbipn_dn9 = assign17400_e11917_d_n9;
        locals.var_vbipn_dn10 = assign17400_e11917_d_n10;
        locals.var_vbipn_dn13 = assign17400_e11917_d_n13;
        locals.var_vbipn_rv = 0.0;

        let (assign17410_e11926, assign17410_e11926_d_n0, assign17410_e11926_d_n2, assign17410_e11926_d_n4, assign17410_e11926_d_n5, assign17410_e11926_d_n6, assign17410_e11926_d_n7, assign17410_e11926_d_n8, assign17410_e11926_d_n9, assign17410_e11926_d_n10, assign17410_e11926_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17410_e11923: f64 = (locals.var_log_tratio * p.p380);
        let assign17410_e11924: f64 = (assign17410_e11923).exp();
        (assign17410_e11924, (assign17410_e11924 * (locals.var_log_tratio_dn0 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn2 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn4 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn5 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn6 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn7 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn8 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn9 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn10 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn13 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17410_e11926;
        locals.var_t1_dn0 = assign17410_e11926_d_n0;
        locals.var_t1_dn2 = assign17410_e11926_d_n2;
        locals.var_t1_dn4 = assign17410_e11926_d_n4;
        locals.var_t1_dn5 = assign17410_e11926_d_n5;
        locals.var_t1_dn6 = assign17410_e11926_d_n6;
        locals.var_t1_dn7 = assign17410_e11926_d_n7;
        locals.var_t1_dn8 = assign17410_e11926_d_n8;
        locals.var_t1_dn9 = assign17410_e11926_d_n9;
        locals.var_t1_dn10 = assign17410_e11926_d_n10;
        locals.var_t1_dn13 = assign17410_e11926_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17420_e11934, assign17420_e11934_d_n0, assign17420_e11934_d_n2, assign17420_e11934_d_n4, assign17420_e11934_d_n5, assign17420_e11934_d_n6, assign17420_e11934_d_n7, assign17420_e11934_d_n8, assign17420_e11934_d_n9, assign17420_e11934_d_n10, assign17420_e11934_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17420_e11932: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign17420_e11932, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn13 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign17420_e11934;
        locals.var_depmphn0_dn0 = assign17420_e11934_d_n0;
        locals.var_depmphn0_dn2 = assign17420_e11934_d_n2;
        locals.var_depmphn0_dn4 = assign17420_e11934_d_n4;
        locals.var_depmphn0_dn5 = assign17420_e11934_d_n5;
        locals.var_depmphn0_dn6 = assign17420_e11934_d_n6;
        locals.var_depmphn0_dn7 = assign17420_e11934_d_n7;
        locals.var_depmphn0_dn8 = assign17420_e11934_d_n8;
        locals.var_depmphn0_dn9 = assign17420_e11934_d_n9;
        locals.var_depmphn0_dn10 = assign17420_e11934_d_n10;
        locals.var_depmphn0_dn13 = assign17420_e11934_d_n13;
        locals.var_depmphn0_rv = 0.0;

        let (assign17430_e11956, assign17430_e11956_d_n0, assign17430_e11956_d_n2, assign17430_e11956_d_n4, assign17430_e11956_d_n5, assign17430_e11956_d_n6, assign17430_e11956_d_n7, assign17430_e11956_d_n8, assign17430_e11956_d_n9, assign17430_e11956_d_n10, assign17430_e11956_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17430_e11941: f64 = (0.4 * locals.var_tratio);
        let assign17430_e11942: f64 = (1.8 + assign17430_e11941);
        let assign17430_e11945: f64 = (0.1 * locals.var_tratio);
        let assign17430_e11947: f64 = (assign17430_e11945 * locals.var_tratio);
        let assign17430_e11948: f64 = (assign17430_e11942 + assign17430_e11947);
        let assign17430_e11952: f64 = (1.0 - locals.var_tratio);
        let assign17430_e11953: f64 = (p.p379 * assign17430_e11952);
        let assign17430_e11954: f64 = (assign17430_e11948 - assign17430_e11953);
        (assign17430_e11954, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn13))) - (p.p379 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign17430_e11956;
        locals.var_t0_dn0 = assign17430_e11956_d_n0;
        locals.var_t0_dn2 = assign17430_e11956_d_n2;
        locals.var_t0_dn4 = assign17430_e11956_d_n4;
        locals.var_t0_dn5 = assign17430_e11956_d_n5;
        locals.var_t0_dn6 = assign17430_e11956_d_n6;
        locals.var_t0_dn7 = assign17430_e11956_d_n7;
        locals.var_t0_dn8 = assign17430_e11956_d_n8;
        locals.var_t0_dn9 = assign17430_e11956_d_n9;
        locals.var_t0_dn10 = assign17430_e11956_d_n10;
        locals.var_t0_dn13 = assign17430_e11956_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_39(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17440_e11964, assign17440_e11964_d_n0, assign17440_e11964_d_n2, assign17440_e11964_d_n4, assign17440_e11964_d_n5, assign17440_e11964_d_n6, assign17440_e11964_d_n7, assign17440_e11964_d_n8, assign17440_e11964_d_n9, assign17440_e11964_d_n10, assign17440_e11964_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17440_e11962: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign17440_e11962, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn13 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17440_e11964;
        locals.var_uc_depvmax_dn0 = assign17440_e11964_d_n0;
        locals.var_uc_depvmax_dn2 = assign17440_e11964_d_n2;
        locals.var_uc_depvmax_dn4 = assign17440_e11964_d_n4;
        locals.var_uc_depvmax_dn5 = assign17440_e11964_d_n5;
        locals.var_uc_depvmax_dn6 = assign17440_e11964_d_n6;
        locals.var_uc_depvmax_dn7 = assign17440_e11964_d_n7;
        locals.var_uc_depvmax_dn8 = assign17440_e11964_d_n8;
        locals.var_uc_depvmax_dn9 = assign17440_e11964_d_n9;
        locals.var_uc_depvmax_dn10 = assign17440_e11964_d_n10;
        locals.var_uc_depvmax_dn13 = assign17440_e11964_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let assign17460_e11972: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign17460_e11972;
        locals.var_guard357_rv = 0.0;

        let (assign17470_e11980, assign17470_e11980_d_n0, assign17470_e11980_d_n2, assign17470_e11980_d_n4, assign17470_e11980_d_n5, assign17470_e11980_d_n6, assign17470_e11980_d_n7, assign17470_e11980_d_n8, assign17470_e11980_d_n9, assign17470_e11980_d_n10, assign17470_e11980_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) && (locals.var_guard357 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17470_e11980;
        locals.var_uc_depvmax_dn0 = assign17470_e11980_d_n0;
        locals.var_uc_depvmax_dn2 = assign17470_e11980_d_n2;
        locals.var_uc_depvmax_dn4 = assign17470_e11980_d_n4;
        locals.var_uc_depvmax_dn5 = assign17470_e11980_d_n5;
        locals.var_uc_depvmax_dn6 = assign17470_e11980_d_n6;
        locals.var_uc_depvmax_dn7 = assign17470_e11980_d_n7;
        locals.var_uc_depvmax_dn8 = assign17470_e11980_d_n8;
        locals.var_uc_depvmax_dn9 = assign17470_e11980_d_n9;
        locals.var_uc_depvmax_dn10 = assign17470_e11980_d_n10;
        locals.var_uc_depvmax_dn13 = assign17470_e11980_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign17480_e11990, assign17480_e11990_d_n0, assign17480_e11990_d_n2, assign17480_e11990_d_n4, assign17480_e11990_d_n5, assign17480_e11990_d_n6, assign17480_e11990_d_n7, assign17480_e11990_d_n8, assign17480_e11990_d_n9, assign17480_e11990_d_n10, assign17480_e11990_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17480_e11987: f64 = (locals.var_tratio).powf(p.p381);
        let assign17480_e11988: f64 = (locals.var_uc_depmue0 / assign17480_e11987);
        (assign17480_e11988, (((locals.var_uc_depmue0_dn0 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn2 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn4 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn5 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn6 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn7 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn8 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn9 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn10 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn13 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn13)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign17480_e11990;
        locals.var_uc_depmue0_dn0 = assign17480_e11990_d_n0;
        locals.var_uc_depmue0_dn2 = assign17480_e11990_d_n2;
        locals.var_uc_depmue0_dn4 = assign17480_e11990_d_n4;
        locals.var_uc_depmue0_dn5 = assign17480_e11990_d_n5;
        locals.var_uc_depmue0_dn6 = assign17480_e11990_d_n6;
        locals.var_uc_depmue0_dn7 = assign17480_e11990_d_n7;
        locals.var_uc_depmue0_dn8 = assign17480_e11990_d_n8;
        locals.var_uc_depmue0_dn9 = assign17480_e11990_d_n9;
        locals.var_uc_depmue0_dn10 = assign17480_e11990_d_n10;
        locals.var_uc_depmue0_dn13 = assign17480_e11990_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign17490_e12000, assign17490_e12000_d_n0, assign17490_e12000_d_n2, assign17490_e12000_d_n4, assign17490_e12000_d_n5, assign17490_e12000_d_n6, assign17490_e12000_d_n7, assign17490_e12000_d_n8, assign17490_e12000_d_n9, assign17490_e12000_d_n10, assign17490_e12000_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17490_e11997: f64 = (locals.var_tratio).powf(p.p382);
        let assign17490_e11998: f64 = (locals.var_uc_depmue2 / assign17490_e11997);
        (assign17490_e11998, (((locals.var_uc_depmue2_dn0 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn2 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn4 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn5 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn6 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn7 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn8 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn9 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn10 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn13 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn13)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)),)
    } else {
        (locals.var_uc_depmue2, locals.var_uc_depmue2_dn0, locals.var_uc_depmue2_dn2, locals.var_uc_depmue2_dn4, locals.var_uc_depmue2_dn5, locals.var_uc_depmue2_dn6, locals.var_uc_depmue2_dn7, locals.var_uc_depmue2_dn8, locals.var_uc_depmue2_dn9, locals.var_uc_depmue2_dn10, locals.var_uc_depmue2_dn13,)
    }
};
        locals.var_uc_depmue2 = assign17490_e12000;
        locals.var_uc_depmue2_dn0 = assign17490_e12000_d_n0;
        locals.var_uc_depmue2_dn2 = assign17490_e12000_d_n2;
        locals.var_uc_depmue2_dn4 = assign17490_e12000_d_n4;
        locals.var_uc_depmue2_dn5 = assign17490_e12000_d_n5;
        locals.var_uc_depmue2_dn6 = assign17490_e12000_d_n6;
        locals.var_uc_depmue2_dn7 = assign17490_e12000_d_n7;
        locals.var_uc_depmue2_dn8 = assign17490_e12000_d_n8;
        locals.var_uc_depmue2_dn9 = assign17490_e12000_d_n9;
        locals.var_uc_depmue2_dn10 = assign17490_e12000_d_n10;
        locals.var_uc_depmue2_dn13 = assign17490_e12000_d_n13;
        locals.var_uc_depmue2_rv = 0.0;

        let assign17500_e12003: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard358 = assign17500_e12003;
        locals.var_guard358_rv = 0.0;

        let (assign17510_e12021, assign17510_e12021_d_n0, assign17510_e12021_d_n2, assign17510_e12021_d_n4, assign17510_e12021_d_n5, assign17510_e12021_d_n6, assign17510_e12021_d_n7, assign17510_e12021_d_n8, assign17510_e12021_d_n9, assign17510_e12021_d_n10, assign17510_e12021_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17510_e12012: f64 = (2.0 * 1.034943e-10);
        let assign17510_e12014: f64 = (assign17510_e12012 * 1.6021918e-19);
        let assign17510_e12016: f64 = (assign17510_e12014 * locals.var_uc_ndepm);
        let assign17510_e12018: f64 = (assign17510_e12016 * locals.var_beta_inv);
        let assign17510_e12019: f64 = (assign17510_e12018).sqrt();
        (assign17510_e12019, ((((assign17510_e12014 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn0)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn2)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn4)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn5)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn6)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn7)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn8)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn9)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn10)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn13) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn13)) / (2.0 * assign17510_e12019)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign17510_e12021;
        locals.var_cnst0_dn0 = assign17510_e12021_d_n0;
        locals.var_cnst0_dn2 = assign17510_e12021_d_n2;
        locals.var_cnst0_dn4 = assign17510_e12021_d_n4;
        locals.var_cnst0_dn5 = assign17510_e12021_d_n5;
        locals.var_cnst0_dn6 = assign17510_e12021_d_n6;
        locals.var_cnst0_dn7 = assign17510_e12021_d_n7;
        locals.var_cnst0_dn8 = assign17510_e12021_d_n8;
        locals.var_cnst0_dn9 = assign17510_e12021_d_n9;
        locals.var_cnst0_dn10 = assign17510_e12021_d_n10;
        locals.var_cnst0_dn13 = assign17510_e12021_d_n13;
        locals.var_cnst0_rv = 0.0;

        let (assign17520_e12036, assign17520_e12036_d_n0, assign17520_e12036_d_n2, assign17520_e12036_d_n4, assign17520_e12036_d_n5, assign17520_e12036_d_n6, assign17520_e12036_d_n7, assign17520_e12036_d_n8, assign17520_e12036_d_n9, assign17520_e12036_d_n10, assign17520_e12036_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17520_e12030: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign17520_e12032: f64 = (assign17520_e12030 * __rspice_inv_cse_0);
        let assign17520_e12034: f64 = (assign17520_e12032 * __rspice_inv_cse_0);
        (assign17520_e12034, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign17520_e12036;
        locals.var_cnst1_dn0 = assign17520_e12036_d_n0;
        locals.var_cnst1_dn2 = assign17520_e12036_d_n2;
        locals.var_cnst1_dn4 = assign17520_e12036_d_n4;
        locals.var_cnst1_dn5 = assign17520_e12036_d_n5;
        locals.var_cnst1_dn6 = assign17520_e12036_d_n6;
        locals.var_cnst1_dn7 = assign17520_e12036_d_n7;
        locals.var_cnst1_dn8 = assign17520_e12036_d_n8;
        locals.var_cnst1_dn9 = assign17520_e12036_d_n9;
        locals.var_cnst1_dn10 = assign17520_e12036_d_n10;
        locals.var_cnst1_dn13 = assign17520_e12036_d_n13;
        locals.var_cnst1_rv = 0.0;

        let (assign17530_e12052, assign17530_e12052_d_n0, assign17530_e12052_d_n2, assign17530_e12052_d_n4, assign17530_e12052_d_n5, assign17530_e12052_d_n6, assign17530_e12052_d_n7, assign17530_e12052_d_n8, assign17530_e12052_d_n9, assign17530_e12052_d_n10, assign17530_e12052_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17530_e12045: f64 = (2.0 * locals.var_beta_inv);
        let assign17530_e12048: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign17530_e12049: f64 = (assign17530_e12048).ln();
        let assign17530_e12050: f64 = (assign17530_e12045 * assign17530_e12049);
        (assign17530_e12050, (((2.0 * locals.var_beta_inv_dn0) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn2) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn4) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn5) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn6) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn7) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn8) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn9) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn10) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn13) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn13 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign17530_e12052;
        locals.var_pb2n_dn0 = assign17530_e12052_d_n0;
        locals.var_pb2n_dn2 = assign17530_e12052_d_n2;
        locals.var_pb2n_dn4 = assign17530_e12052_d_n4;
        locals.var_pb2n_dn5 = assign17530_e12052_d_n5;
        locals.var_pb2n_dn6 = assign17530_e12052_d_n6;
        locals.var_pb2n_dn7 = assign17530_e12052_d_n7;
        locals.var_pb2n_dn8 = assign17530_e12052_d_n8;
        locals.var_pb2n_dn9 = assign17530_e12052_d_n9;
        locals.var_pb2n_dn10 = assign17530_e12052_d_n10;
        locals.var_pb2n_dn13 = assign17530_e12052_d_n13;
        locals.var_pb2n_rv = 0.0;

        let (assign17540_e12070, assign17540_e12070_d_n0, assign17540_e12070_d_n2, assign17540_e12070_d_n4, assign17540_e12070_d_n5, assign17540_e12070_d_n6, assign17540_e12070_d_n7, assign17540_e12070_d_n8, assign17540_e12070_d_n9, assign17540_e12070_d_n10, assign17540_e12070_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17540_e12062: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign17540_e12064: f64 = (assign17540_e12062 * __rspice_inv_cse_1);
        let assign17540_e12066: f64 = (assign17540_e12064 * __rspice_inv_cse_1);
        let assign17540_e12067: f64 = (assign17540_e12066).ln();
        let assign17540_e12068: f64 = (locals.var_beta_inv * assign17540_e12067);
        (assign17540_e12068, ((locals.var_beta_inv_dn0 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn2 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn4 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn5 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn6 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn7 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn8 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn9 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn10 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn13 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign17540_e12070;
        locals.var_vbipn_dn0 = assign17540_e12070_d_n0;
        locals.var_vbipn_dn2 = assign17540_e12070_d_n2;
        locals.var_vbipn_dn4 = assign17540_e12070_d_n4;
        locals.var_vbipn_dn5 = assign17540_e12070_d_n5;
        locals.var_vbipn_dn6 = assign17540_e12070_d_n6;
        locals.var_vbipn_dn7 = assign17540_e12070_d_n7;
        locals.var_vbipn_dn8 = assign17540_e12070_d_n8;
        locals.var_vbipn_dn9 = assign17540_e12070_d_n9;
        locals.var_vbipn_dn10 = assign17540_e12070_d_n10;
        locals.var_vbipn_dn13 = assign17540_e12070_d_n13;
        locals.var_vbipn_rv = 0.0;

        let (assign17550_e12082, assign17550_e12082_d_n0, assign17550_e12082_d_n2, assign17550_e12082_d_n4, assign17550_e12082_d_n5, assign17550_e12082_d_n6, assign17550_e12082_d_n7, assign17550_e12082_d_n8, assign17550_e12082_d_n9, assign17550_e12082_d_n10, assign17550_e12082_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17550_e12079: f64 = (locals.var_log_tratio * p.p380);
        let assign17550_e12080: f64 = (assign17550_e12079).exp();
        (assign17550_e12080, (assign17550_e12080 * (locals.var_log_tratio_dn0 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn2 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn4 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn5 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn6 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn7 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn8 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn9 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn10 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn13 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17550_e12082;
        locals.var_t1_dn0 = assign17550_e12082_d_n0;
        locals.var_t1_dn2 = assign17550_e12082_d_n2;
        locals.var_t1_dn4 = assign17550_e12082_d_n4;
        locals.var_t1_dn5 = assign17550_e12082_d_n5;
        locals.var_t1_dn6 = assign17550_e12082_d_n6;
        locals.var_t1_dn7 = assign17550_e12082_d_n7;
        locals.var_t1_dn8 = assign17550_e12082_d_n8;
        locals.var_t1_dn9 = assign17550_e12082_d_n9;
        locals.var_t1_dn10 = assign17550_e12082_d_n10;
        locals.var_t1_dn13 = assign17550_e12082_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17560_e12093, assign17560_e12093_d_n0, assign17560_e12093_d_n2, assign17560_e12093_d_n4, assign17560_e12093_d_n5, assign17560_e12093_d_n6, assign17560_e12093_d_n7, assign17560_e12093_d_n8, assign17560_e12093_d_n9, assign17560_e12093_d_n10, assign17560_e12093_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17560_e12091: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign17560_e12091, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn13 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign17560_e12093;
        locals.var_depmphn0_dn0 = assign17560_e12093_d_n0;
        locals.var_depmphn0_dn2 = assign17560_e12093_d_n2;
        locals.var_depmphn0_dn4 = assign17560_e12093_d_n4;
        locals.var_depmphn0_dn5 = assign17560_e12093_d_n5;
        locals.var_depmphn0_dn6 = assign17560_e12093_d_n6;
        locals.var_depmphn0_dn7 = assign17560_e12093_d_n7;
        locals.var_depmphn0_dn8 = assign17560_e12093_d_n8;
        locals.var_depmphn0_dn9 = assign17560_e12093_d_n9;
        locals.var_depmphn0_dn10 = assign17560_e12093_d_n10;
        locals.var_depmphn0_dn13 = assign17560_e12093_d_n13;
        locals.var_depmphn0_rv = 0.0;

        let (assign17570_e12118, assign17570_e12118_d_n0, assign17570_e12118_d_n2, assign17570_e12118_d_n4, assign17570_e12118_d_n5, assign17570_e12118_d_n6, assign17570_e12118_d_n7, assign17570_e12118_d_n8, assign17570_e12118_d_n9, assign17570_e12118_d_n10, assign17570_e12118_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17570_e12103: f64 = (0.4 * locals.var_tratio);
        let assign17570_e12104: f64 = (1.8 + assign17570_e12103);
        let assign17570_e12107: f64 = (0.1 * locals.var_tratio);
        let assign17570_e12109: f64 = (assign17570_e12107 * locals.var_tratio);
        let assign17570_e12110: f64 = (assign17570_e12104 + assign17570_e12109);
        let assign17570_e12114: f64 = (1.0 - locals.var_tratio);
        let assign17570_e12115: f64 = (p.p379 * assign17570_e12114);
        let assign17570_e12116: f64 = (assign17570_e12110 - assign17570_e12115);
        (assign17570_e12116, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn13))) - (p.p379 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign17570_e12118;
        locals.var_t0_dn0 = assign17570_e12118_d_n0;
        locals.var_t0_dn2 = assign17570_e12118_d_n2;
        locals.var_t0_dn4 = assign17570_e12118_d_n4;
        locals.var_t0_dn5 = assign17570_e12118_d_n5;
        locals.var_t0_dn6 = assign17570_e12118_d_n6;
        locals.var_t0_dn7 = assign17570_e12118_d_n7;
        locals.var_t0_dn8 = assign17570_e12118_d_n8;
        locals.var_t0_dn9 = assign17570_e12118_d_n9;
        locals.var_t0_dn10 = assign17570_e12118_d_n10;
        locals.var_t0_dn13 = assign17570_e12118_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign17580_e12129, assign17580_e12129_d_n0, assign17580_e12129_d_n2, assign17580_e12129_d_n4, assign17580_e12129_d_n5, assign17580_e12129_d_n6, assign17580_e12129_d_n7, assign17580_e12129_d_n8, assign17580_e12129_d_n9, assign17580_e12129_d_n10, assign17580_e12129_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17580_e12127: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign17580_e12127, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn13 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17580_e12129;
        locals.var_uc_depvmax_dn0 = assign17580_e12129_d_n0;
        locals.var_uc_depvmax_dn2 = assign17580_e12129_d_n2;
        locals.var_uc_depvmax_dn4 = assign17580_e12129_d_n4;
        locals.var_uc_depvmax_dn5 = assign17580_e12129_d_n5;
        locals.var_uc_depvmax_dn6 = assign17580_e12129_d_n6;
        locals.var_uc_depvmax_dn7 = assign17580_e12129_d_n7;
        locals.var_uc_depvmax_dn8 = assign17580_e12129_d_n8;
        locals.var_uc_depvmax_dn9 = assign17580_e12129_d_n9;
        locals.var_uc_depvmax_dn10 = assign17580_e12129_d_n10;
        locals.var_uc_depvmax_dn13 = assign17580_e12129_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let assign17600_e12137: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard360 = assign17600_e12137;
        locals.var_guard360_rv = 0.0;

        let (assign17610_e12148, assign17610_e12148_d_n0, assign17610_e12148_d_n2, assign17610_e12148_d_n4, assign17610_e12148_d_n5, assign17610_e12148_d_n6, assign17610_e12148_d_n7, assign17610_e12148_d_n8, assign17610_e12148_d_n9, assign17610_e12148_d_n10, assign17610_e12148_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard360 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17610_e12148;
        locals.var_uc_depvmax_dn0 = assign17610_e12148_d_n0;
        locals.var_uc_depvmax_dn2 = assign17610_e12148_d_n2;
        locals.var_uc_depvmax_dn4 = assign17610_e12148_d_n4;
        locals.var_uc_depvmax_dn5 = assign17610_e12148_d_n5;
        locals.var_uc_depvmax_dn6 = assign17610_e12148_d_n6;
        locals.var_uc_depvmax_dn7 = assign17610_e12148_d_n7;
        locals.var_uc_depvmax_dn8 = assign17610_e12148_d_n8;
        locals.var_uc_depvmax_dn9 = assign17610_e12148_d_n9;
        locals.var_uc_depvmax_dn10 = assign17610_e12148_d_n10;
        locals.var_uc_depvmax_dn13 = assign17610_e12148_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign17620_e12161, assign17620_e12161_d_n0, assign17620_e12161_d_n2, assign17620_e12161_d_n4, assign17620_e12161_d_n5, assign17620_e12161_d_n6, assign17620_e12161_d_n7, assign17620_e12161_d_n8, assign17620_e12161_d_n9, assign17620_e12161_d_n10, assign17620_e12161_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17620_e12158: f64 = (locals.var_tratio).powf(p.p381);
        let assign17620_e12159: f64 = (locals.var_uc_depmue0 / assign17620_e12158);
        (assign17620_e12159, (((locals.var_uc_depmue0_dn0 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn2 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn4 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn5 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn6 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn7 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn8 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn9 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn10 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn13 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn13)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign17620_e12161;
        locals.var_uc_depmue0_dn0 = assign17620_e12161_d_n0;
        locals.var_uc_depmue0_dn2 = assign17620_e12161_d_n2;
        locals.var_uc_depmue0_dn4 = assign17620_e12161_d_n4;
        locals.var_uc_depmue0_dn5 = assign17620_e12161_d_n5;
        locals.var_uc_depmue0_dn6 = assign17620_e12161_d_n6;
        locals.var_uc_depmue0_dn7 = assign17620_e12161_d_n7;
        locals.var_uc_depmue0_dn8 = assign17620_e12161_d_n8;
        locals.var_uc_depmue0_dn9 = assign17620_e12161_d_n9;
        locals.var_uc_depmue0_dn10 = assign17620_e12161_d_n10;
        locals.var_uc_depmue0_dn13 = assign17620_e12161_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign17630_e12176, assign17630_e12176_d_n0, assign17630_e12176_d_n2, assign17630_e12176_d_n4, assign17630_e12176_d_n5, assign17630_e12176_d_n6, assign17630_e12176_d_n7, assign17630_e12176_d_n8, assign17630_e12176_d_n9, assign17630_e12176_d_n10, assign17630_e12176_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17630_e12172: f64 = (locals.var_tratio - 1.0);
        let assign17630_e12173: f64 = (p.p365 * assign17630_e12172);
        let assign17630_e12174: f64 = (p.p364 + assign17630_e12173);
        (assign17630_e12174, (p.p365 * locals.var_tratio_dn0), (p.p365 * locals.var_tratio_dn2), (p.p365 * locals.var_tratio_dn4), (p.p365 * locals.var_tratio_dn5), (p.p365 * locals.var_tratio_dn6), (p.p365 * locals.var_tratio_dn7), (p.p365 * locals.var_tratio_dn8), (p.p365 * locals.var_tratio_dn9), (p.p365 * locals.var_tratio_dn10), (p.p365 * locals.var_tratio_dn13),)
    } else {
        (locals.var_uc_depwlp, locals.var_uc_depwlp_dn0, locals.var_uc_depwlp_dn2, locals.var_uc_depwlp_dn4, locals.var_uc_depwlp_dn5, locals.var_uc_depwlp_dn6, locals.var_uc_depwlp_dn7, locals.var_uc_depwlp_dn8, locals.var_uc_depwlp_dn9, locals.var_uc_depwlp_dn10, locals.var_uc_depwlp_dn13,)
    }
};
        locals.var_uc_depwlp = assign17630_e12176;
        locals.var_uc_depwlp_dn0 = assign17630_e12176_d_n0;
        locals.var_uc_depwlp_dn2 = assign17630_e12176_d_n2;
        locals.var_uc_depwlp_dn4 = assign17630_e12176_d_n4;
        locals.var_uc_depwlp_dn5 = assign17630_e12176_d_n5;
        locals.var_uc_depwlp_dn6 = assign17630_e12176_d_n6;
        locals.var_uc_depwlp_dn7 = assign17630_e12176_d_n7;
        locals.var_uc_depwlp_dn8 = assign17630_e12176_d_n8;
        locals.var_uc_depwlp_dn9 = assign17630_e12176_d_n9;
        locals.var_uc_depwlp_dn10 = assign17630_e12176_d_n10;
        locals.var_uc_depwlp_dn13 = assign17630_e12176_d_n13;
        locals.var_uc_depwlp_rv = 0.0;

        let (assign17640_e12186, assign17640_e12186_d_n0, assign17640_e12186_d_n2, assign17640_e12186_d_n4, assign17640_e12186_d_n5, assign17640_e12186_d_n6, assign17640_e12186_d_n7, assign17640_e12186_d_n8, assign17640_e12186_d_n9, assign17640_e12186_d_n10, assign17640_e12186_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign17640_e12186;
        locals.var_pb2n_dn0 = assign17640_e12186_d_n0;
        locals.var_pb2n_dn2 = assign17640_e12186_d_n2;
        locals.var_pb2n_dn4 = assign17640_e12186_d_n4;
        locals.var_pb2n_dn5 = assign17640_e12186_d_n5;
        locals.var_pb2n_dn6 = assign17640_e12186_d_n6;
        locals.var_pb2n_dn7 = assign17640_e12186_d_n7;
        locals.var_pb2n_dn8 = assign17640_e12186_d_n8;
        locals.var_pb2n_dn9 = assign17640_e12186_d_n9;
        locals.var_pb2n_dn10 = assign17640_e12186_d_n10;
        locals.var_pb2n_dn13 = assign17640_e12186_d_n13;
        locals.var_pb2n_rv = 0.0;

        let (assign17650_e12205, assign17650_e12205_d_n0, assign17650_e12205_d_n2, assign17650_e12205_d_n4, assign17650_e12205_d_n5, assign17650_e12205_d_n6, assign17650_e12205_d_n7, assign17650_e12205_d_n8, assign17650_e12205_d_n9, assign17650_e12205_d_n10, assign17650_e12205_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 == 0.0)) {
        let assign17650_e12197: f64 = (locals.var_uc_njunc / locals.var_nin);
        let assign17650_e12199: f64 = (assign17650_e12197 * locals.var_nsub);
        let assign17650_e12201: f64 = (assign17650_e12199 / locals.var_nin);
        let assign17650_e12202: f64 = (assign17650_e12201).ln();
        let assign17650_e12203: f64 = (locals.var_beta_inv * assign17650_e12202);
        (assign17650_e12203, ((locals.var_beta_inv_dn0 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn0)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn2 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn2)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn4 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn4)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn5 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn5)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn6 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn6)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn7 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn7)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn8 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn8)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn9 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn9)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn10 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn10)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn13 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn13)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign17650_e12205;
        locals.var_vbipn_dn0 = assign17650_e12205_d_n0;
        locals.var_vbipn_dn2 = assign17650_e12205_d_n2;
        locals.var_vbipn_dn4 = assign17650_e12205_d_n4;
        locals.var_vbipn_dn5 = assign17650_e12205_d_n5;
        locals.var_vbipn_dn6 = assign17650_e12205_d_n6;
        locals.var_vbipn_dn7 = assign17650_e12205_d_n7;
        locals.var_vbipn_dn8 = assign17650_e12205_d_n8;
        locals.var_vbipn_dn9 = assign17650_e12205_d_n9;
        locals.var_vbipn_dn10 = assign17650_e12205_d_n10;
        locals.var_vbipn_dn13 = assign17650_e12205_d_n13;
        locals.var_vbipn_rv = 0.0;

        let (assign17660_e12215, assign17660_e12215_d_n0, assign17660_e12215_d_n2, assign17660_e12215_d_n4, assign17660_e12215_d_n5, assign17660_e12215_d_n6, assign17660_e12215_d_n7, assign17660_e12215_d_n8, assign17660_e12215_d_n9, assign17660_e12215_d_n10, assign17660_e12215_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign17660_e12215;
        locals.var_depmphn0_dn0 = assign17660_e12215_d_n0;
        locals.var_depmphn0_dn2 = assign17660_e12215_d_n2;
        locals.var_depmphn0_dn4 = assign17660_e12215_d_n4;
        locals.var_depmphn0_dn5 = assign17660_e12215_d_n5;
        locals.var_depmphn0_dn6 = assign17660_e12215_d_n6;
        locals.var_depmphn0_dn7 = assign17660_e12215_d_n7;
        locals.var_depmphn0_dn8 = assign17660_e12215_d_n8;
        locals.var_depmphn0_dn9 = assign17660_e12215_d_n9;
        locals.var_depmphn0_dn10 = assign17660_e12215_d_n10;
        locals.var_depmphn0_dn13 = assign17660_e12215_d_n13;
        locals.var_depmphn0_rv = 0.0;

        let (assign17670_e12221, assign17670_e12221_d_n0, assign17670_e12221_d_n2, assign17670_e12221_d_n4, assign17670_e12221_d_n5, assign17670_e12221_d_n6, assign17670_e12221_d_n7, assign17670_e12221_d_n8, assign17670_e12221_d_n9, assign17670_e12221_d_n10, assign17670_e12221_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17670_e12219: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        (assign17670_e12219, ((locals.var_ptovr0_dn0 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn0)), ((locals.var_ptovr0_dn2 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn2)), ((locals.var_ptovr0_dn4 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn4)), ((locals.var_ptovr0_dn5 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn5)), ((locals.var_ptovr0_dn6 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn6)), ((locals.var_ptovr0_dn7 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn7)), ((locals.var_ptovr0_dn8 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn8)), ((locals.var_ptovr0_dn9 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn9)), ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10)), ((locals.var_ptovr0_dn13 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_ptovr, locals.var_ptovr_dn0, locals.var_ptovr_dn2, locals.var_ptovr_dn4, locals.var_ptovr_dn5, locals.var_ptovr_dn6, locals.var_ptovr_dn7, locals.var_ptovr_dn8, locals.var_ptovr_dn9, locals.var_ptovr_dn10, locals.var_ptovr_dn13,)
    }
};
        locals.var_ptovr = assign17670_e12221;
        locals.var_ptovr_dn0 = assign17670_e12221_d_n0;
        locals.var_ptovr_dn2 = assign17670_e12221_d_n2;
        locals.var_ptovr_dn4 = assign17670_e12221_d_n4;
        locals.var_ptovr_dn5 = assign17670_e12221_d_n5;
        locals.var_ptovr_dn6 = assign17670_e12221_d_n6;
        locals.var_ptovr_dn7 = assign17670_e12221_d_n7;
        locals.var_ptovr_dn8 = assign17670_e12221_d_n8;
        locals.var_ptovr_dn9 = assign17670_e12221_d_n9;
        locals.var_ptovr_dn10 = assign17670_e12221_d_n10;
        locals.var_ptovr_dn13 = assign17670_e12221_d_n13;
        locals.var_ptovr_rv = 0.0;

        let (assign17680_e12227, assign17680_e12227_d_n0, assign17680_e12227_d_n2, assign17680_e12227_d_n4, assign17680_e12227_d_n5, assign17680_e12227_d_n6, assign17680_e12227_d_n7, assign17680_e12227_d_n8, assign17680_e12227_d_n9, assign17680_e12227_d_n10, assign17680_e12227_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17680_e12225: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign17680_e12225, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn13 / locals.var_ktnom),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17680_e12227;
        locals.var_t1_dn0 = assign17680_e12227_d_n0;
        locals.var_t1_dn2 = assign17680_e12227_d_n2;
        locals.var_t1_dn4 = assign17680_e12227_d_n4;
        locals.var_t1_dn5 = assign17680_e12227_d_n5;
        locals.var_t1_dn6 = assign17680_e12227_d_n6;
        locals.var_t1_dn7 = assign17680_e12227_d_n7;
        locals.var_t1_dn8 = assign17680_e12227_d_n8;
        locals.var_t1_dn9 = assign17680_e12227_d_n9;
        locals.var_t1_dn10 = assign17680_e12227_d_n10;
        locals.var_t1_dn13 = assign17680_e12227_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17690_e12247, assign17690_e12247_d_n0, assign17690_e12247_d_n2, assign17690_e12247_d_n4, assign17690_e12247_d_n5, assign17690_e12247_d_n6, assign17690_e12247_d_n7, assign17690_e12247_d_n8, assign17690_e12247_d_n9, assign17690_e12247_d_n10, assign17690_e12247_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17690_e12232: f64 = (0.4 * locals.var_t1);
        let assign17690_e12233: f64 = (1.8 + assign17690_e12232);
        let assign17690_e12236: f64 = (0.1 * locals.var_t1);
        let assign17690_e12238: f64 = (assign17690_e12236 * locals.var_t1);
        let assign17690_e12239: f64 = (assign17690_e12233 + assign17690_e12238);
        let assign17690_e12243: f64 = (1.0 - locals.var_t1);
        let assign17690_e12244: f64 = (locals.var_uc_vtmp * assign17690_e12243);
        let assign17690_e12245: f64 = (assign17690_e12239 - assign17690_e12244);
        (assign17690_e12245, (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn0))) - (locals.var_uc_vtmp * (-locals.var_t1_dn0))), (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn2))) - (locals.var_uc_vtmp * (-locals.var_t1_dn2))), (((0.4 * locals.var_t1_dn4) + (((0.1 * locals.var_t1_dn4) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn4))) - (locals.var_uc_vtmp * (-locals.var_t1_dn4))), (((0.4 * locals.var_t1_dn5) + (((0.1 * locals.var_t1_dn5) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn5))) - (locals.var_uc_vtmp * (-locals.var_t1_dn5))), (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn6))) - (locals.var_uc_vtmp * (-locals.var_t1_dn6))), (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn7))) - (locals.var_uc_vtmp * (-locals.var_t1_dn7))), (((0.4 * locals.var_t1_dn8) + (((0.1 * locals.var_t1_dn8) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn8))) - (locals.var_uc_vtmp * (-locals.var_t1_dn8))), (((0.4 * locals.var_t1_dn9) + (((0.1 * locals.var_t1_dn9) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn9))) - (locals.var_uc_vtmp * (-locals.var_t1_dn9))), (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn10))) - (locals.var_uc_vtmp * (-locals.var_t1_dn10))), (((0.4 * locals.var_t1_dn13) + (((0.1 * locals.var_t1_dn13) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn13))) - (locals.var_uc_vtmp * (-locals.var_t1_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign17690_e12247;
        locals.var_t0_dn0 = assign17690_e12247_d_n0;
        locals.var_t0_dn2 = assign17690_e12247_d_n2;
        locals.var_t0_dn4 = assign17690_e12247_d_n4;
        locals.var_t0_dn5 = assign17690_e12247_d_n5;
        locals.var_t0_dn6 = assign17690_e12247_d_n6;
        locals.var_t0_dn7 = assign17690_e12247_d_n7;
        locals.var_t0_dn8 = assign17690_e12247_d_n8;
        locals.var_t0_dn9 = assign17690_e12247_d_n9;
        locals.var_t0_dn10 = assign17690_e12247_d_n10;
        locals.var_t0_dn13 = assign17690_e12247_d_n13;
        locals.var_t0_rv = 0.0;

        let assign17700_e12250: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard361 = assign17700_e12250;
        locals.var_guard361_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_40(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17710_e12270, assign17710_e12270_d_n0, assign17710_e12270_d_n2, assign17710_e12270_d_n4, assign17710_e12270_d_n5, assign17710_e12270_d_n6, assign17710_e12270_d_n7, assign17710_e12270_d_n8, assign17710_e12270_d_n9, assign17710_e12270_d_n10, assign17710_e12270_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard361 != 0.0)) {
        let assign17710_e12256: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign17710_e12258: f64 = (assign17710_e12256 / locals.var_t0);
        let assign17710_e12262: f64 = (p.p90 * locals.var_tdiff0);
        let assign17710_e12263: f64 = (1.0 + assign17710_e12262);
        let assign17710_e12266: f64 = (p.p91 * locals.var_tdiff0_2);
        let assign17710_e12267: f64 = (assign17710_e12263 + assign17710_e12266);
        let assign17710_e12268: f64 = (assign17710_e12258 * assign17710_e12267);
        (assign17710_e12268, (((-((assign17710_e12256 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn0) + (p.p91 * locals.var_tdiff0_2_dn0)))), (((-((assign17710_e12256 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn2) + (p.p91 * locals.var_tdiff0_2_dn2)))), (((-((assign17710_e12256 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn4) + (p.p91 * locals.var_tdiff0_2_dn4)))), (((-((assign17710_e12256 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn5) + (p.p91 * locals.var_tdiff0_2_dn5)))), (((-((assign17710_e12256 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn6) + (p.p91 * locals.var_tdiff0_2_dn6)))), (((-((assign17710_e12256 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn7) + (p.p91 * locals.var_tdiff0_2_dn7)))), (((-((assign17710_e12256 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn8) + (p.p91 * locals.var_tdiff0_2_dn8)))), (((-((assign17710_e12256 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn9) + (p.p91 * locals.var_tdiff0_2_dn9)))), (((-((assign17710_e12256 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn10) + (p.p91 * locals.var_tdiff0_2_dn10)))), (((-((assign17710_e12256 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn13) + (p.p91 * locals.var_tdiff0_2_dn13)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn13,)
    }
};
        locals.var_vmaxeff = assign17710_e12270;
        locals.var_vmaxeff_dn0 = assign17710_e12270_d_n0;
        locals.var_vmaxeff_dn2 = assign17710_e12270_d_n2;
        locals.var_vmaxeff_dn4 = assign17710_e12270_d_n4;
        locals.var_vmaxeff_dn5 = assign17710_e12270_d_n5;
        locals.var_vmaxeff_dn6 = assign17710_e12270_d_n6;
        locals.var_vmaxeff_dn7 = assign17710_e12270_d_n7;
        locals.var_vmaxeff_dn8 = assign17710_e12270_d_n8;
        locals.var_vmaxeff_dn9 = assign17710_e12270_d_n9;
        locals.var_vmaxeff_dn10 = assign17710_e12270_d_n10;
        locals.var_vmaxeff_dn13 = assign17710_e12270_d_n13;
        locals.var_vmaxeff_rv = 0.0;

        let (assign17720_e12291, assign17720_e12291_d_n0, assign17720_e12291_d_n2, assign17720_e12291_d_n4, assign17720_e12291_d_n5, assign17720_e12291_d_n6, assign17720_e12291_d_n7, assign17720_e12291_d_n8, assign17720_e12291_d_n9, assign17720_e12291_d_n10, assign17720_e12291_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard361 == 0.0)) {
        let assign17720_e12277: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign17720_e12279: f64 = (assign17720_e12277 / locals.var_t0);
        let assign17720_e12283: f64 = (p.p90 * locals.var_tdiff);
        let assign17720_e12284: f64 = (1.0 + assign17720_e12283);
        let assign17720_e12287: f64 = (p.p91 * locals.var_tdiff_2);
        let assign17720_e12288: f64 = (assign17720_e12284 + assign17720_e12287);
        let assign17720_e12289: f64 = (assign17720_e12279 * assign17720_e12288);
        (assign17720_e12289, (((-((assign17720_e12277 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn0) + (p.p91 * locals.var_tdiff_2_dn0)))), (((-((assign17720_e12277 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn2) + (p.p91 * locals.var_tdiff_2_dn2)))), (((-((assign17720_e12277 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn4) + (p.p91 * locals.var_tdiff_2_dn4)))), (((-((assign17720_e12277 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn5) + (p.p91 * locals.var_tdiff_2_dn5)))), (((-((assign17720_e12277 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn6) + (p.p91 * locals.var_tdiff_2_dn6)))), (((-((assign17720_e12277 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn7) + (p.p91 * locals.var_tdiff_2_dn7)))), (((-((assign17720_e12277 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn8) + (p.p91 * locals.var_tdiff_2_dn8)))), (((-((assign17720_e12277 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn9) + (p.p91 * locals.var_tdiff_2_dn9)))), (((-((assign17720_e12277 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn10) + (p.p91 * locals.var_tdiff_2_dn10)))), (((-((assign17720_e12277 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn13) + (p.p91 * locals.var_tdiff_2_dn13)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn13,)
    }
};
        locals.var_vmaxeff = assign17720_e12291;
        locals.var_vmaxeff_dn0 = assign17720_e12291_d_n0;
        locals.var_vmaxeff_dn2 = assign17720_e12291_d_n2;
        locals.var_vmaxeff_dn4 = assign17720_e12291_d_n4;
        locals.var_vmaxeff_dn5 = assign17720_e12291_d_n5;
        locals.var_vmaxeff_dn6 = assign17720_e12291_d_n6;
        locals.var_vmaxeff_dn7 = assign17720_e12291_d_n7;
        locals.var_vmaxeff_dn8 = assign17720_e12291_d_n8;
        locals.var_vmaxeff_dn9 = assign17720_e12291_d_n9;
        locals.var_vmaxeff_dn10 = assign17720_e12291_d_n10;
        locals.var_vmaxeff_dn13 = assign17720_e12291_d_n13;
        locals.var_vmaxeff_rv = 0.0;

        let assign17740_e12299: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard363 = assign17740_e12299;
        locals.var_guard363_rv = 0.0;

        let (assign17750_e12315, assign17750_e12315_d_n0, assign17750_e12315_d_n2, assign17750_e12315_d_n4, assign17750_e12315_d_n5, assign17750_e12315_d_n6, assign17750_e12315_d_n7, assign17750_e12315_d_n8, assign17750_e12315_d_n9, assign17750_e12315_d_n10, assign17750_e12315_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17750_e12307: f64 = (p.p324 * locals.var_tdiff0);
        let assign17750_e12308: f64 = (1.0 + assign17750_e12307);
        let assign17750_e12311: f64 = (p.p325 * locals.var_tdiff0_2);
        let assign17750_e12312: f64 = (assign17750_e12308 + assign17750_e12311);
        let assign17750_e12313: f64 = (locals.var_ninvd0 * assign17750_e12312);
        (assign17750_e12313, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn0) + (p.p325 * locals.var_tdiff0_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn2) + (p.p325 * locals.var_tdiff0_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn4) + (p.p325 * locals.var_tdiff0_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn5) + (p.p325 * locals.var_tdiff0_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn6) + (p.p325 * locals.var_tdiff0_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn7) + (p.p325 * locals.var_tdiff0_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn8) + (p.p325 * locals.var_tdiff0_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn9) + (p.p325 * locals.var_tdiff0_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn10) + (p.p325 * locals.var_tdiff0_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn13) + (p.p325 * locals.var_tdiff0_2_dn13))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign17750_e12315;
        locals.var_ninvde_dn0 = assign17750_e12315_d_n0;
        locals.var_ninvde_dn2 = assign17750_e12315_d_n2;
        locals.var_ninvde_dn4 = assign17750_e12315_d_n4;
        locals.var_ninvde_dn5 = assign17750_e12315_d_n5;
        locals.var_ninvde_dn6 = assign17750_e12315_d_n6;
        locals.var_ninvde_dn7 = assign17750_e12315_d_n7;
        locals.var_ninvde_dn8 = assign17750_e12315_d_n8;
        locals.var_ninvde_dn9 = assign17750_e12315_d_n9;
        locals.var_ninvde_dn10 = assign17750_e12315_d_n10;
        locals.var_ninvde_dn13 = assign17750_e12315_d_n13;
        locals.var_ninvde_rv = 0.0;

        let (assign17760_e12329, assign17760_e12329_d_n0, assign17760_e12329_d_n2, assign17760_e12329_d_n4, assign17760_e12329_d_n5, assign17760_e12329_d_n6, assign17760_e12329_d_n7, assign17760_e12329_d_n8, assign17760_e12329_d_n9, assign17760_e12329_d_n10, assign17760_e12329_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17760_e12322: f64 = (p.p390 * locals.var_tdiff0);
        let assign17760_e12323: f64 = (1.0 + assign17760_e12322);
        let assign17760_e12326: f64 = (p.p391 * locals.var_tdiff0_2);
        let assign17760_e12327: f64 = (assign17760_e12323 + assign17760_e12326);
        (assign17760_e12327, ((p.p390 * locals.var_tdiff0_dn0) + (p.p391 * locals.var_tdiff0_2_dn0)), ((p.p390 * locals.var_tdiff0_dn2) + (p.p391 * locals.var_tdiff0_2_dn2)), ((p.p390 * locals.var_tdiff0_dn4) + (p.p391 * locals.var_tdiff0_2_dn4)), ((p.p390 * locals.var_tdiff0_dn5) + (p.p391 * locals.var_tdiff0_2_dn5)), ((p.p390 * locals.var_tdiff0_dn6) + (p.p391 * locals.var_tdiff0_2_dn6)), ((p.p390 * locals.var_tdiff0_dn7) + (p.p391 * locals.var_tdiff0_2_dn7)), ((p.p390 * locals.var_tdiff0_dn8) + (p.p391 * locals.var_tdiff0_2_dn8)), ((p.p390 * locals.var_tdiff0_dn9) + (p.p391 * locals.var_tdiff0_2_dn9)), ((p.p390 * locals.var_tdiff0_dn10) + (p.p391 * locals.var_tdiff0_2_dn10)), ((p.p390 * locals.var_tdiff0_dn13) + (p.p391 * locals.var_tdiff0_2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17760_e12329;
        locals.var_t1_dn0 = assign17760_e12329_d_n0;
        locals.var_t1_dn2 = assign17760_e12329_d_n2;
        locals.var_t1_dn4 = assign17760_e12329_d_n4;
        locals.var_t1_dn5 = assign17760_e12329_d_n5;
        locals.var_t1_dn6 = assign17760_e12329_d_n6;
        locals.var_t1_dn7 = assign17760_e12329_d_n7;
        locals.var_t1_dn8 = assign17760_e12329_d_n8;
        locals.var_t1_dn9 = assign17760_e12329_d_n9;
        locals.var_t1_dn10 = assign17760_e12329_d_n10;
        locals.var_t1_dn13 = assign17760_e12329_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17770_e12337, assign17770_e12337_d_n0, assign17770_e12337_d_n2, assign17770_e12337_d_n4, assign17770_e12337_d_n5, assign17770_e12337_d_n6, assign17770_e12337_d_n7, assign17770_e12337_d_n8, assign17770_e12337_d_n9, assign17770_e12337_d_n10, assign17770_e12337_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17770_e12335: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign17770_e12335, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn13 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign17770_e12337;
        locals.var_ninvdecres_dn0 = assign17770_e12337_d_n0;
        locals.var_ninvdecres_dn2 = assign17770_e12337_d_n2;
        locals.var_ninvdecres_dn4 = assign17770_e12337_d_n4;
        locals.var_ninvdecres_dn5 = assign17770_e12337_d_n5;
        locals.var_ninvdecres_dn6 = assign17770_e12337_d_n6;
        locals.var_ninvdecres_dn7 = assign17770_e12337_d_n7;
        locals.var_ninvdecres_dn8 = assign17770_e12337_d_n8;
        locals.var_ninvdecres_dn9 = assign17770_e12337_d_n9;
        locals.var_ninvdecres_dn10 = assign17770_e12337_d_n10;
        locals.var_ninvdecres_dn13 = assign17770_e12337_d_n13;
        locals.var_ninvdecres_rv = 0.0;

        let (assign17780_e12345, assign17780_e12345_d_n0, assign17780_e12345_d_n2, assign17780_e12345_d_n4, assign17780_e12345_d_n5, assign17780_e12345_d_n6, assign17780_e12345_d_n7, assign17780_e12345_d_n8, assign17780_e12345_d_n9, assign17780_e12345_d_n10, assign17780_e12345_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17780_e12343: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign17780_e12343, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn13 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign17780_e12345;
        locals.var_ninvdehres_dn0 = assign17780_e12345_d_n0;
        locals.var_ninvdehres_dn2 = assign17780_e12345_d_n2;
        locals.var_ninvdehres_dn4 = assign17780_e12345_d_n4;
        locals.var_ninvdehres_dn5 = assign17780_e12345_d_n5;
        locals.var_ninvdehres_dn6 = assign17780_e12345_d_n6;
        locals.var_ninvdehres_dn7 = assign17780_e12345_d_n7;
        locals.var_ninvdehres_dn8 = assign17780_e12345_d_n8;
        locals.var_ninvdehres_dn9 = assign17780_e12345_d_n9;
        locals.var_ninvdehres_dn10 = assign17780_e12345_d_n10;
        locals.var_ninvdehres_dn13 = assign17780_e12345_d_n13;
        locals.var_ninvdehres_rv = 0.0;

        let (assign17790_e12362, assign17790_e12362_d_n0, assign17790_e12362_d_n2, assign17790_e12362_d_n4, assign17790_e12362_d_n5, assign17790_e12362_d_n6, assign17790_e12362_d_n7, assign17790_e12362_d_n8, assign17790_e12362_d_n9, assign17790_e12362_d_n10, assign17790_e12362_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17790_e12354: f64 = (p.p324 * locals.var_tdiff);
        let assign17790_e12355: f64 = (1.0 + assign17790_e12354);
        let assign17790_e12358: f64 = (p.p325 * locals.var_tdiff_2);
        let assign17790_e12359: f64 = (assign17790_e12355 + assign17790_e12358);
        let assign17790_e12360: f64 = (locals.var_ninvd0 * assign17790_e12359);
        (assign17790_e12360, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn0) + (p.p325 * locals.var_tdiff_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn2) + (p.p325 * locals.var_tdiff_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn4) + (p.p325 * locals.var_tdiff_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn5) + (p.p325 * locals.var_tdiff_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn6) + (p.p325 * locals.var_tdiff_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn7) + (p.p325 * locals.var_tdiff_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn8) + (p.p325 * locals.var_tdiff_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn9) + (p.p325 * locals.var_tdiff_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn10) + (p.p325 * locals.var_tdiff_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn13) + (p.p325 * locals.var_tdiff_2_dn13))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign17790_e12362;
        locals.var_ninvde_dn0 = assign17790_e12362_d_n0;
        locals.var_ninvde_dn2 = assign17790_e12362_d_n2;
        locals.var_ninvde_dn4 = assign17790_e12362_d_n4;
        locals.var_ninvde_dn5 = assign17790_e12362_d_n5;
        locals.var_ninvde_dn6 = assign17790_e12362_d_n6;
        locals.var_ninvde_dn7 = assign17790_e12362_d_n7;
        locals.var_ninvde_dn8 = assign17790_e12362_d_n8;
        locals.var_ninvde_dn9 = assign17790_e12362_d_n9;
        locals.var_ninvde_dn10 = assign17790_e12362_d_n10;
        locals.var_ninvde_dn13 = assign17790_e12362_d_n13;
        locals.var_ninvde_rv = 0.0;

        let (assign17800_e12377, assign17800_e12377_d_n0, assign17800_e12377_d_n2, assign17800_e12377_d_n4, assign17800_e12377_d_n5, assign17800_e12377_d_n6, assign17800_e12377_d_n7, assign17800_e12377_d_n8, assign17800_e12377_d_n9, assign17800_e12377_d_n10, assign17800_e12377_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17800_e12370: f64 = (p.p390 * locals.var_tdiff);
        let assign17800_e12371: f64 = (1.0 + assign17800_e12370);
        let assign17800_e12374: f64 = (p.p391 * locals.var_tdiff_2);
        let assign17800_e12375: f64 = (assign17800_e12371 + assign17800_e12374);
        (assign17800_e12375, ((p.p390 * locals.var_tdiff_dn0) + (p.p391 * locals.var_tdiff_2_dn0)), ((p.p390 * locals.var_tdiff_dn2) + (p.p391 * locals.var_tdiff_2_dn2)), ((p.p390 * locals.var_tdiff_dn4) + (p.p391 * locals.var_tdiff_2_dn4)), ((p.p390 * locals.var_tdiff_dn5) + (p.p391 * locals.var_tdiff_2_dn5)), ((p.p390 * locals.var_tdiff_dn6) + (p.p391 * locals.var_tdiff_2_dn6)), ((p.p390 * locals.var_tdiff_dn7) + (p.p391 * locals.var_tdiff_2_dn7)), ((p.p390 * locals.var_tdiff_dn8) + (p.p391 * locals.var_tdiff_2_dn8)), ((p.p390 * locals.var_tdiff_dn9) + (p.p391 * locals.var_tdiff_2_dn9)), ((p.p390 * locals.var_tdiff_dn10) + (p.p391 * locals.var_tdiff_2_dn10)), ((p.p390 * locals.var_tdiff_dn13) + (p.p391 * locals.var_tdiff_2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17800_e12377;
        locals.var_t1_dn0 = assign17800_e12377_d_n0;
        locals.var_t1_dn2 = assign17800_e12377_d_n2;
        locals.var_t1_dn4 = assign17800_e12377_d_n4;
        locals.var_t1_dn5 = assign17800_e12377_d_n5;
        locals.var_t1_dn6 = assign17800_e12377_d_n6;
        locals.var_t1_dn7 = assign17800_e12377_d_n7;
        locals.var_t1_dn8 = assign17800_e12377_d_n8;
        locals.var_t1_dn9 = assign17800_e12377_d_n9;
        locals.var_t1_dn10 = assign17800_e12377_d_n10;
        locals.var_t1_dn13 = assign17800_e12377_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17810_e12386, assign17810_e12386_d_n0, assign17810_e12386_d_n2, assign17810_e12386_d_n4, assign17810_e12386_d_n5, assign17810_e12386_d_n6, assign17810_e12386_d_n7, assign17810_e12386_d_n8, assign17810_e12386_d_n9, assign17810_e12386_d_n10, assign17810_e12386_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17810_e12384: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign17810_e12384, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn13 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign17810_e12386;
        locals.var_ninvdecres_dn0 = assign17810_e12386_d_n0;
        locals.var_ninvdecres_dn2 = assign17810_e12386_d_n2;
        locals.var_ninvdecres_dn4 = assign17810_e12386_d_n4;
        locals.var_ninvdecres_dn5 = assign17810_e12386_d_n5;
        locals.var_ninvdecres_dn6 = assign17810_e12386_d_n6;
        locals.var_ninvdecres_dn7 = assign17810_e12386_d_n7;
        locals.var_ninvdecres_dn8 = assign17810_e12386_d_n8;
        locals.var_ninvdecres_dn9 = assign17810_e12386_d_n9;
        locals.var_ninvdecres_dn10 = assign17810_e12386_d_n10;
        locals.var_ninvdecres_dn13 = assign17810_e12386_d_n13;
        locals.var_ninvdecres_rv = 0.0;

        let (assign17820_e12395, assign17820_e12395_d_n0, assign17820_e12395_d_n2, assign17820_e12395_d_n4, assign17820_e12395_d_n5, assign17820_e12395_d_n6, assign17820_e12395_d_n7, assign17820_e12395_d_n8, assign17820_e12395_d_n9, assign17820_e12395_d_n10, assign17820_e12395_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17820_e12393: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign17820_e12393, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn13 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign17820_e12395;
        locals.var_ninvdehres_dn0 = assign17820_e12395_d_n0;
        locals.var_ninvdehres_dn2 = assign17820_e12395_d_n2;
        locals.var_ninvdehres_dn4 = assign17820_e12395_d_n4;
        locals.var_ninvdehres_dn5 = assign17820_e12395_d_n5;
        locals.var_ninvdehres_dn6 = assign17820_e12395_d_n6;
        locals.var_ninvdehres_dn7 = assign17820_e12395_d_n7;
        locals.var_ninvdehres_dn8 = assign17820_e12395_d_n8;
        locals.var_ninvdehres_dn9 = assign17820_e12395_d_n9;
        locals.var_ninvdehres_dn10 = assign17820_e12395_d_n10;
        locals.var_ninvdehres_dn13 = assign17820_e12395_d_n13;
        locals.var_ninvdehres_rv = 0.0;

        let assign17840_e12403: f64 = if locals.var_ninvde < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign17840_e12403;
        locals.var_guard365_rv = 0.0;

        let (assign17850_e12409, assign17850_e12409_d_n0, assign17850_e12409_d_n2, assign17850_e12409_d_n4, assign17850_e12409_d_n5, assign17850_e12409_d_n6, assign17850_e12409_d_n7, assign17850_e12409_d_n8, assign17850_e12409_d_n9, assign17850_e12409_d_n10, assign17850_e12409_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard365 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign17850_e12409;
        locals.var_ninvde_dn0 = assign17850_e12409_d_n0;
        locals.var_ninvde_dn2 = assign17850_e12409_d_n2;
        locals.var_ninvde_dn4 = assign17850_e12409_d_n4;
        locals.var_ninvde_dn5 = assign17850_e12409_d_n5;
        locals.var_ninvde_dn6 = assign17850_e12409_d_n6;
        locals.var_ninvde_dn7 = assign17850_e12409_d_n7;
        locals.var_ninvde_dn8 = assign17850_e12409_d_n8;
        locals.var_ninvde_dn9 = assign17850_e12409_d_n9;
        locals.var_ninvde_dn10 = assign17850_e12409_d_n10;
        locals.var_ninvde_dn13 = assign17850_e12409_d_n13;
        locals.var_ninvde_rv = 0.0;

        let assign17870_e12417: f64 = if locals.var_ninvdecres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard367 = assign17870_e12417;
        locals.var_guard367_rv = 0.0;

        let (assign17880_e12423, assign17880_e12423_d_n0, assign17880_e12423_d_n2, assign17880_e12423_d_n4, assign17880_e12423_d_n5, assign17880_e12423_d_n6, assign17880_e12423_d_n7, assign17880_e12423_d_n8, assign17880_e12423_d_n9, assign17880_e12423_d_n10, assign17880_e12423_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard367 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign17880_e12423;
        locals.var_ninvdecres_dn0 = assign17880_e12423_d_n0;
        locals.var_ninvdecres_dn2 = assign17880_e12423_d_n2;
        locals.var_ninvdecres_dn4 = assign17880_e12423_d_n4;
        locals.var_ninvdecres_dn5 = assign17880_e12423_d_n5;
        locals.var_ninvdecres_dn6 = assign17880_e12423_d_n6;
        locals.var_ninvdecres_dn7 = assign17880_e12423_d_n7;
        locals.var_ninvdecres_dn8 = assign17880_e12423_d_n8;
        locals.var_ninvdecres_dn9 = assign17880_e12423_d_n9;
        locals.var_ninvdecres_dn10 = assign17880_e12423_d_n10;
        locals.var_ninvdecres_dn13 = assign17880_e12423_d_n13;
        locals.var_ninvdecres_rv = 0.0;

        let assign17900_e12431: f64 = if locals.var_ninvdehres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign17900_e12431;
        locals.var_guard369_rv = 0.0;

        let (assign17910_e12437, assign17910_e12437_d_n0, assign17910_e12437_d_n2, assign17910_e12437_d_n4, assign17910_e12437_d_n5, assign17910_e12437_d_n6, assign17910_e12437_d_n7, assign17910_e12437_d_n8, assign17910_e12437_d_n9, assign17910_e12437_d_n10, assign17910_e12437_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard369 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign17910_e12437;
        locals.var_ninvdehres_dn0 = assign17910_e12437_d_n0;
        locals.var_ninvdehres_dn2 = assign17910_e12437_d_n2;
        locals.var_ninvdehres_dn4 = assign17910_e12437_d_n4;
        locals.var_ninvdehres_dn5 = assign17910_e12437_d_n5;
        locals.var_ninvdehres_dn6 = assign17910_e12437_d_n6;
        locals.var_ninvdehres_dn7 = assign17910_e12437_d_n7;
        locals.var_ninvdehres_dn8 = assign17910_e12437_d_n8;
        locals.var_ninvdehres_dn9 = assign17910_e12437_d_n9;
        locals.var_ninvdehres_dn10 = assign17910_e12437_d_n10;
        locals.var_ninvdehres_dn13 = assign17910_e12437_d_n13;
        locals.var_ninvdehres_rv = 0.0;

        let (assign17920_e12453, assign17920_e12453_d_n0, assign17920_e12453_d_n2, assign17920_e12453_d_n4, assign17920_e12453_d_n5, assign17920_e12453_d_n6, assign17920_e12453_d_n7, assign17920_e12453_d_n8, assign17920_e12453_d_n9, assign17920_e12453_d_n10, assign17920_e12453_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (p.p53 != 0.0)) {
        let assign17920_e12444: f64 = (p.p328 * locals.var_tdiff0);
        let assign17920_e12445: f64 = (locals.var_uc_rth0 + assign17920_e12444);
        let assign17920_e12448: f64 = (p.p329 * locals.var_tdiff0_2);
        let assign17920_e12449: f64 = (assign17920_e12445 + assign17920_e12448);
        let assign17920_e12451: f64 = (assign17920_e12449 * locals.var_rthtemp0);
        (assign17920_e12451, (((p.p328 * locals.var_tdiff0_dn0) + (p.p329 * locals.var_tdiff0_2_dn0)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn2) + (p.p329 * locals.var_tdiff0_2_dn2)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn4) + (p.p329 * locals.var_tdiff0_2_dn4)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn5) + (p.p329 * locals.var_tdiff0_2_dn5)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn6) + (p.p329 * locals.var_tdiff0_2_dn6)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn7) + (p.p329 * locals.var_tdiff0_2_dn7)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn8) + (p.p329 * locals.var_tdiff0_2_dn8)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn9) + (p.p329 * locals.var_tdiff0_2_dn9)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn10) + (p.p329 * locals.var_tdiff0_2_dn10)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn13) + (p.p329 * locals.var_tdiff0_2_dn13)) * locals.var_rthtemp0),)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn13,)
    }
};
        locals.var_rth = assign17920_e12453;
        locals.var_rth_dn0 = assign17920_e12453_d_n0;
        locals.var_rth_dn2 = assign17920_e12453_d_n2;
        locals.var_rth_dn4 = assign17920_e12453_d_n4;
        locals.var_rth_dn5 = assign17920_e12453_d_n5;
        locals.var_rth_dn6 = assign17920_e12453_d_n6;
        locals.var_rth_dn7 = assign17920_e12453_d_n7;
        locals.var_rth_dn8 = assign17920_e12453_d_n8;
        locals.var_rth_dn9 = assign17920_e12453_d_n9;
        locals.var_rth_dn10 = assign17920_e12453_d_n10;
        locals.var_rth_dn13 = assign17920_e12453_d_n13;
        locals.var_rth_rv = 0.0;

        let assign17940_e12461: f64 = if locals.var_rth < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard371 = assign17940_e12461;
        locals.var_guard371_rv = 0.0;

        let (assign17950_e12469, assign17950_e12469_d_n0, assign17950_e12469_d_n2, assign17950_e12469_d_n4, assign17950_e12469_d_n5, assign17950_e12469_d_n6, assign17950_e12469_d_n7, assign17950_e12469_d_n8, assign17950_e12469_d_n9, assign17950_e12469_d_n10, assign17950_e12469_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (p.p53 != 0.0)) && (locals.var_guard371 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn13,)
    }
};
        locals.var_rth = assign17950_e12469;
        locals.var_rth_dn0 = assign17950_e12469_d_n0;
        locals.var_rth_dn2 = assign17950_e12469_d_n2;
        locals.var_rth_dn4 = assign17950_e12469_d_n4;
        locals.var_rth_dn5 = assign17950_e12469_d_n5;
        locals.var_rth_dn6 = assign17950_e12469_d_n6;
        locals.var_rth_dn7 = assign17950_e12469_d_n7;
        locals.var_rth_dn8 = assign17950_e12469_d_n8;
        locals.var_rth_dn9 = assign17950_e12469_d_n9;
        locals.var_rth_dn10 = assign17950_e12469_d_n10;
        locals.var_rth_dn13 = assign17950_e12469_d_n13;
        locals.var_rth_rv = 0.0;

        let (assign17960_e12481, assign17960_e12481_d_n0, assign17960_e12481_d_n2, assign17960_e12481_d_n4, assign17960_e12481_d_n5, assign17960_e12481_d_n6, assign17960_e12481_d_n7, assign17960_e12481_d_n8, assign17960_e12481_d_n9, assign17960_e12481_d_n10, assign17960_e12481_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17960_e12474: f64 = (p.p330 * locals.var_tdiff0);
        let assign17960_e12475: f64 = (locals.var_uc_powrat + assign17960_e12474);
        let assign17960_e12478: f64 = (p.p331 * locals.var_tdiff0_2);
        let assign17960_e12479: f64 = (assign17960_e12475 + assign17960_e12478);
        (assign17960_e12479, ((p.p330 * locals.var_tdiff0_dn0) + (p.p331 * locals.var_tdiff0_2_dn0)), ((p.p330 * locals.var_tdiff0_dn2) + (p.p331 * locals.var_tdiff0_2_dn2)), ((p.p330 * locals.var_tdiff0_dn4) + (p.p331 * locals.var_tdiff0_2_dn4)), ((p.p330 * locals.var_tdiff0_dn5) + (p.p331 * locals.var_tdiff0_2_dn5)), ((p.p330 * locals.var_tdiff0_dn6) + (p.p331 * locals.var_tdiff0_2_dn6)), ((p.p330 * locals.var_tdiff0_dn7) + (p.p331 * locals.var_tdiff0_2_dn7)), ((p.p330 * locals.var_tdiff0_dn8) + (p.p331 * locals.var_tdiff0_2_dn8)), ((p.p330 * locals.var_tdiff0_dn9) + (p.p331 * locals.var_tdiff0_2_dn9)), ((p.p330 * locals.var_tdiff0_dn10) + (p.p331 * locals.var_tdiff0_2_dn10)), ((p.p330 * locals.var_tdiff0_dn13) + (p.p331 * locals.var_tdiff0_2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign17960_e12481;
        locals.var_t2_dn0 = assign17960_e12481_d_n0;
        locals.var_t2_dn2 = assign17960_e12481_d_n2;
        locals.var_t2_dn4 = assign17960_e12481_d_n4;
        locals.var_t2_dn5 = assign17960_e12481_d_n5;
        locals.var_t2_dn6 = assign17960_e12481_d_n6;
        locals.var_t2_dn7 = assign17960_e12481_d_n7;
        locals.var_t2_dn8 = assign17960_e12481_d_n8;
        locals.var_t2_dn9 = assign17960_e12481_d_n9;
        locals.var_t2_dn10 = assign17960_e12481_d_n10;
        locals.var_t2_dn13 = assign17960_e12481_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign17970_e12489, assign17970_e12489_d_n0, assign17970_e12489_d_n2, assign17970_e12489_d_n4, assign17970_e12489_d_n5, assign17970_e12489_d_n6, assign17970_e12489_d_n7, assign17970_e12489_d_n8, assign17970_e12489_d_n9, assign17970_e12489_d_n10, assign17970_e12489_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17970_e12485: f64 = locals.var_t2;
        let assign17970_e12487: f64 = (assign17970_e12485 - 0.05);
        (assign17970_e12487, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign17970_e12489;
        locals.var_tmf1_dn0 = assign17970_e12489_d_n0;
        locals.var_tmf1_dn2 = assign17970_e12489_d_n2;
        locals.var_tmf1_dn4 = assign17970_e12489_d_n4;
        locals.var_tmf1_dn5 = assign17970_e12489_d_n5;
        locals.var_tmf1_dn6 = assign17970_e12489_d_n6;
        locals.var_tmf1_dn7 = assign17970_e12489_d_n7;
        locals.var_tmf1_dn8 = assign17970_e12489_d_n8;
        locals.var_tmf1_dn9 = assign17970_e12489_d_n9;
        locals.var_tmf1_dn10 = assign17970_e12489_d_n10;
        locals.var_tmf1_dn13 = assign17970_e12489_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign17980_e12497, assign17980_e12497_d_n0, assign17980_e12497_d_n2, assign17980_e12497_d_n4, assign17980_e12497_d_n5, assign17980_e12497_d_n6, assign17980_e12497_d_n7, assign17980_e12497_d_n8, assign17980_e12497_d_n9, assign17980_e12497_d_n10, assign17980_e12497_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17980_e12497;
        locals.var_tmf2_dn0 = assign17980_e12497_d_n0;
        locals.var_tmf2_dn2 = assign17980_e12497_d_n2;
        locals.var_tmf2_dn4 = assign17980_e12497_d_n4;
        locals.var_tmf2_dn5 = assign17980_e12497_d_n5;
        locals.var_tmf2_dn6 = assign17980_e12497_d_n6;
        locals.var_tmf2_dn7 = assign17980_e12497_d_n7;
        locals.var_tmf2_dn8 = assign17980_e12497_d_n8;
        locals.var_tmf2_dn9 = assign17980_e12497_d_n9;
        locals.var_tmf2_dn10 = assign17980_e12497_d_n10;
        locals.var_tmf2_dn13 = assign17980_e12497_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign17990_e12507, assign17990_e12507_d_n0, assign17990_e12507_d_n2, assign17990_e12507_d_n4, assign17990_e12507_d_n5, assign17990_e12507_d_n6, assign17990_e12507_d_n7, assign17990_e12507_d_n8, assign17990_e12507_d_n9, assign17990_e12507_d_n10, assign17990_e12507_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let (assign17990_e12505, assign17990_e12505_d_n0, assign17990_e12505_d_n2, assign17990_e12505_d_n4, assign17990_e12505_d_n5, assign17990_e12505_d_n6, assign17990_e12505_d_n7, assign17990_e12505_d_n8, assign17990_e12505_d_n9, assign17990_e12505_d_n10, assign17990_e12505_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign17990_e12504: f64 = (-locals.var_tmf2);
                (assign17990_e12504, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign17990_e12505, assign17990_e12505_d_n0, assign17990_e12505_d_n2, assign17990_e12505_d_n4, assign17990_e12505_d_n5, assign17990_e12505_d_n6, assign17990_e12505_d_n7, assign17990_e12505_d_n8, assign17990_e12505_d_n9, assign17990_e12505_d_n10, assign17990_e12505_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17990_e12507;
        locals.var_tmf2_dn0 = assign17990_e12507_d_n0;
        locals.var_tmf2_dn2 = assign17990_e12507_d_n2;
        locals.var_tmf2_dn4 = assign17990_e12507_d_n4;
        locals.var_tmf2_dn5 = assign17990_e12507_d_n5;
        locals.var_tmf2_dn6 = assign17990_e12507_d_n6;
        locals.var_tmf2_dn7 = assign17990_e12507_d_n7;
        locals.var_tmf2_dn8 = assign17990_e12507_d_n8;
        locals.var_tmf2_dn9 = assign17990_e12507_d_n9;
        locals.var_tmf2_dn10 = assign17990_e12507_d_n10;
        locals.var_tmf2_dn13 = assign17990_e12507_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18000_e12516, assign18000_e12516_d_n0, assign18000_e12516_d_n2, assign18000_e12516_d_n4, assign18000_e12516_d_n5, assign18000_e12516_d_n6, assign18000_e12516_d_n7, assign18000_e12516_d_n8, assign18000_e12516_d_n9, assign18000_e12516_d_n10, assign18000_e12516_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18000_e12511: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18000_e12513: f64 = (assign18000_e12511 + locals.var_tmf2);
        let assign18000_e12514: f64 = (assign18000_e12513).sqrt();
        (assign18000_e12514, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18000_e12514)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18000_e12516;
        locals.var_tmf2_dn0 = assign18000_e12516_d_n0;
        locals.var_tmf2_dn2 = assign18000_e12516_d_n2;
        locals.var_tmf2_dn4 = assign18000_e12516_d_n4;
        locals.var_tmf2_dn5 = assign18000_e12516_d_n5;
        locals.var_tmf2_dn6 = assign18000_e12516_d_n6;
        locals.var_tmf2_dn7 = assign18000_e12516_d_n7;
        locals.var_tmf2_dn8 = assign18000_e12516_d_n8;
        locals.var_tmf2_dn9 = assign18000_e12516_d_n9;
        locals.var_tmf2_dn10 = assign18000_e12516_d_n10;
        locals.var_tmf2_dn13 = assign18000_e12516_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18010_e12526, assign18010_e12526_d_n0, assign18010_e12526_d_n2, assign18010_e12526_d_n4, assign18010_e12526_d_n5, assign18010_e12526_d_n6, assign18010_e12526_d_n7, assign18010_e12526_d_n8, assign18010_e12526_d_n9, assign18010_e12526_d_n10, assign18010_e12526_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18010_e12522: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18010_e12523: f64 = (1.0 + assign18010_e12522);
        let assign18010_e12524: f64 = (0.5 * assign18010_e12523);
        (assign18010_e12524, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18010_e12526;
        locals.var_t0_dn0 = assign18010_e12526_d_n0;
        locals.var_t0_dn2 = assign18010_e12526_d_n2;
        locals.var_t0_dn4 = assign18010_e12526_d_n4;
        locals.var_t0_dn5 = assign18010_e12526_d_n5;
        locals.var_t0_dn6 = assign18010_e12526_d_n6;
        locals.var_t0_dn7 = assign18010_e12526_d_n7;
        locals.var_t0_dn8 = assign18010_e12526_d_n8;
        locals.var_t0_dn9 = assign18010_e12526_d_n9;
        locals.var_t0_dn10 = assign18010_e12526_d_n10;
        locals.var_t0_dn13 = assign18010_e12526_d_n13;
        locals.var_t0_rv = 0.0;

    }
}

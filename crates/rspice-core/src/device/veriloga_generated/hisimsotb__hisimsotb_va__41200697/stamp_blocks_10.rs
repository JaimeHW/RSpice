#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17160_e21075, assign17160_e21075_d_n0, assign17160_e21075_d_n2, assign17160_e21075_d_n4, assign17160_e21075_d_n5, assign17160_e21075_d_n6, assign17160_e21075_d_n8, assign17160_e21075_d_n10, assign17160_e21075_d_n11, assign17160_e21075_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17160_e21073: f64 = (locals.var_lg + p.p264);
        (assign17160_e21073, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk289, locals.var_t3__blk289_dn0, locals.var_t3__blk289_dn2, locals.var_t3__blk289_dn4, locals.var_t3__blk289_dn5, locals.var_t3__blk289_dn6, locals.var_t3__blk289_dn8, locals.var_t3__blk289_dn10, locals.var_t3__blk289_dn11, locals.var_t3__blk289_dn12,)
    }
};
        locals.var_t3__blk289 = assign17160_e21075;
        locals.var_t3__blk289_dn0 = assign17160_e21075_d_n0;
        locals.var_t3__blk289_dn2 = assign17160_e21075_d_n2;
        locals.var_t3__blk289_dn4 = assign17160_e21075_d_n4;
        locals.var_t3__blk289_dn5 = assign17160_e21075_d_n5;
        locals.var_t3__blk289_dn6 = assign17160_e21075_d_n6;
        locals.var_t3__blk289_dn8 = assign17160_e21075_d_n8;
        locals.var_t3__blk289_dn10 = assign17160_e21075_d_n10;
        locals.var_t3__blk289_dn11 = assign17160_e21075_d_n11;
        locals.var_t3__blk289_dn12 = assign17160_e21075_d_n12;
        locals.var_t3__blk289_rv = 0.0;

        let (assign17170_e21089, assign17170_e21089_d_n0, assign17170_e21089_d_n2, assign17170_e21089_d_n4, assign17170_e21089_d_n5, assign17170_e21089_d_n6, assign17170_e21089_d_n8, assign17170_e21089_d_n10, assign17170_e21089_d_n11, assign17170_e21089_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17170_e21083: f64 = (locals.var_t3__blk289 - p.p265);
        let assign17170_e21086: f64 = (locals.var_t3__blk289 * 0.001);
        let assign17170_e21087: f64 = (assign17170_e21083 - assign17170_e21086);
        (assign17170_e21087, (locals.var_t3__blk289_dn0 - (locals.var_t3__blk289_dn0 * 0.001)), (locals.var_t3__blk289_dn2 - (locals.var_t3__blk289_dn2 * 0.001)), (locals.var_t3__blk289_dn4 - (locals.var_t3__blk289_dn4 * 0.001)), (locals.var_t3__blk289_dn5 - (locals.var_t3__blk289_dn5 * 0.001)), (locals.var_t3__blk289_dn6 - (locals.var_t3__blk289_dn6 * 0.001)), (locals.var_t3__blk289_dn8 - (locals.var_t3__blk289_dn8 * 0.001)), (locals.var_t3__blk289_dn10 - (locals.var_t3__blk289_dn10 * 0.001)), (locals.var_t3__blk289_dn11 - (locals.var_t3__blk289_dn11 * 0.001)), (locals.var_t3__blk289_dn12 - (locals.var_t3__blk289_dn12 * 0.001)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign17170_e21089;
        locals.var_tmf1_dn0 = assign17170_e21089_d_n0;
        locals.var_tmf1_dn2 = assign17170_e21089_d_n2;
        locals.var_tmf1_dn4 = assign17170_e21089_d_n4;
        locals.var_tmf1_dn5 = assign17170_e21089_d_n5;
        locals.var_tmf1_dn6 = assign17170_e21089_d_n6;
        locals.var_tmf1_dn8 = assign17170_e21089_d_n8;
        locals.var_tmf1_dn10 = assign17170_e21089_d_n10;
        locals.var_tmf1_dn11 = assign17170_e21089_d_n11;
        locals.var_tmf1_dn12 = assign17170_e21089_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign17180_e21103, assign17180_e21103_d_n0, assign17180_e21103_d_n2, assign17180_e21103_d_n4, assign17180_e21103_d_n5, assign17180_e21103_d_n6, assign17180_e21103_d_n8, assign17180_e21103_d_n10, assign17180_e21103_d_n11, assign17180_e21103_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17180_e21097: f64 = (4.0 * p.p265);
        let assign17180_e21100: f64 = (locals.var_t3__blk289 * 0.001);
        let assign17180_e21101: f64 = (assign17180_e21097 * assign17180_e21100);
        (assign17180_e21101, (assign17180_e21097 * (locals.var_t3__blk289_dn0 * 0.001)), (assign17180_e21097 * (locals.var_t3__blk289_dn2 * 0.001)), (assign17180_e21097 * (locals.var_t3__blk289_dn4 * 0.001)), (assign17180_e21097 * (locals.var_t3__blk289_dn5 * 0.001)), (assign17180_e21097 * (locals.var_t3__blk289_dn6 * 0.001)), (assign17180_e21097 * (locals.var_t3__blk289_dn8 * 0.001)), (assign17180_e21097 * (locals.var_t3__blk289_dn10 * 0.001)), (assign17180_e21097 * (locals.var_t3__blk289_dn11 * 0.001)), (assign17180_e21097 * (locals.var_t3__blk289_dn12 * 0.001)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17180_e21103;
        locals.var_tmf2_dn0 = assign17180_e21103_d_n0;
        locals.var_tmf2_dn2 = assign17180_e21103_d_n2;
        locals.var_tmf2_dn4 = assign17180_e21103_d_n4;
        locals.var_tmf2_dn5 = assign17180_e21103_d_n5;
        locals.var_tmf2_dn6 = assign17180_e21103_d_n6;
        locals.var_tmf2_dn8 = assign17180_e21103_d_n8;
        locals.var_tmf2_dn10 = assign17180_e21103_d_n10;
        locals.var_tmf2_dn11 = assign17180_e21103_d_n11;
        locals.var_tmf2_dn12 = assign17180_e21103_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17190_e21117, assign17190_e21117_d_n0, assign17190_e21117_d_n2, assign17190_e21117_d_n4, assign17190_e21117_d_n5, assign17190_e21117_d_n6, assign17190_e21117_d_n8, assign17190_e21117_d_n10, assign17190_e21117_d_n11, assign17190_e21117_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let (assign17190_e21115, assign17190_e21115_d_n0, assign17190_e21115_d_n2, assign17190_e21115_d_n4, assign17190_e21115_d_n5, assign17190_e21115_d_n6, assign17190_e21115_d_n8, assign17190_e21115_d_n10, assign17190_e21115_d_n11, assign17190_e21115_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign17190_e21114: f64 = (-locals.var_tmf2);
                (assign17190_e21114, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign17190_e21115, assign17190_e21115_d_n0, assign17190_e21115_d_n2, assign17190_e21115_d_n4, assign17190_e21115_d_n5, assign17190_e21115_d_n6, assign17190_e21115_d_n8, assign17190_e21115_d_n10, assign17190_e21115_d_n11, assign17190_e21115_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17190_e21117;
        locals.var_tmf2_dn0 = assign17190_e21117_d_n0;
        locals.var_tmf2_dn2 = assign17190_e21117_d_n2;
        locals.var_tmf2_dn4 = assign17190_e21117_d_n4;
        locals.var_tmf2_dn5 = assign17190_e21117_d_n5;
        locals.var_tmf2_dn6 = assign17190_e21117_d_n6;
        locals.var_tmf2_dn8 = assign17190_e21117_d_n8;
        locals.var_tmf2_dn10 = assign17190_e21117_d_n10;
        locals.var_tmf2_dn11 = assign17190_e21117_d_n11;
        locals.var_tmf2_dn12 = assign17190_e21117_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17200_e21130, assign17200_e21130_d_n0, assign17200_e21130_d_n2, assign17200_e21130_d_n4, assign17200_e21130_d_n5, assign17200_e21130_d_n6, assign17200_e21130_d_n8, assign17200_e21130_d_n10, assign17200_e21130_d_n11, assign17200_e21130_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17200_e21125: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign17200_e21127: f64 = (assign17200_e21125 + locals.var_tmf2);
        let assign17200_e21128: f64 = (assign17200_e21127).sqrt();
        (assign17200_e21128, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign17200_e21128)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign17200_e21128)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign17200_e21128)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign17200_e21128)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign17200_e21128)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign17200_e21128)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign17200_e21128)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign17200_e21128)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign17200_e21128)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17200_e21130;
        locals.var_tmf2_dn0 = assign17200_e21130_d_n0;
        locals.var_tmf2_dn2 = assign17200_e21130_d_n2;
        locals.var_tmf2_dn4 = assign17200_e21130_d_n4;
        locals.var_tmf2_dn5 = assign17200_e21130_d_n5;
        locals.var_tmf2_dn6 = assign17200_e21130_d_n6;
        locals.var_tmf2_dn8 = assign17200_e21130_d_n8;
        locals.var_tmf2_dn10 = assign17200_e21130_d_n10;
        locals.var_tmf2_dn11 = assign17200_e21130_d_n11;
        locals.var_tmf2_dn12 = assign17200_e21130_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17210_e21144, assign17210_e21144_d_n0, assign17210_e21144_d_n2, assign17210_e21144_d_n4, assign17210_e21144_d_n5, assign17210_e21144_d_n6, assign17210_e21144_d_n8, assign17210_e21144_d_n10, assign17210_e21144_d_n11, assign17210_e21144_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17210_e21140: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign17210_e21141: f64 = (1.0 + assign17210_e21140);
        let assign17210_e21142: f64 = (0.5 * assign17210_e21141);
        (assign17210_e21142, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0__blk286, locals.var_t0__blk286_dn0, locals.var_t0__blk286_dn2, locals.var_t0__blk286_dn4, locals.var_t0__blk286_dn5, locals.var_t0__blk286_dn6, locals.var_t0__blk286_dn8, locals.var_t0__blk286_dn10, locals.var_t0__blk286_dn11, locals.var_t0__blk286_dn12,)
    }
};
        locals.var_t0__blk286 = assign17210_e21144;
        locals.var_t0__blk286_dn0 = assign17210_e21144_d_n0;
        locals.var_t0__blk286_dn2 = assign17210_e21144_d_n2;
        locals.var_t0__blk286_dn4 = assign17210_e21144_d_n4;
        locals.var_t0__blk286_dn5 = assign17210_e21144_d_n5;
        locals.var_t0__blk286_dn6 = assign17210_e21144_d_n6;
        locals.var_t0__blk286_dn8 = assign17210_e21144_d_n8;
        locals.var_t0__blk286_dn10 = assign17210_e21144_d_n10;
        locals.var_t0__blk286_dn11 = assign17210_e21144_d_n11;
        locals.var_t0__blk286_dn12 = assign17210_e21144_d_n12;
        locals.var_t0__blk286_rv = 0.0;

        let (assign17220_e21158, assign17220_e21158_d_n0, assign17220_e21158_d_n2, assign17220_e21158_d_n4, assign17220_e21158_d_n5, assign17220_e21158_d_n6, assign17220_e21158_d_n8, assign17220_e21158_d_n10, assign17220_e21158_d_n11, assign17220_e21158_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17220_e21154: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign17220_e21155: f64 = (0.5 * assign17220_e21154);
        let assign17220_e21156: f64 = (p.p265 + assign17220_e21155);
        (assign17220_e21156, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t3__blk289, locals.var_t3__blk289_dn0, locals.var_t3__blk289_dn2, locals.var_t3__blk289_dn4, locals.var_t3__blk289_dn5, locals.var_t3__blk289_dn6, locals.var_t3__blk289_dn8, locals.var_t3__blk289_dn10, locals.var_t3__blk289_dn11, locals.var_t3__blk289_dn12,)
    }
};
        locals.var_t3__blk289 = assign17220_e21158;
        locals.var_t3__blk289_dn0 = assign17220_e21158_d_n0;
        locals.var_t3__blk289_dn2 = assign17220_e21158_d_n2;
        locals.var_t3__blk289_dn4 = assign17220_e21158_d_n4;
        locals.var_t3__blk289_dn5 = assign17220_e21158_d_n5;
        locals.var_t3__blk289_dn6 = assign17220_e21158_d_n6;
        locals.var_t3__blk289_dn8 = assign17220_e21158_d_n8;
        locals.var_t3__blk289_dn10 = assign17220_e21158_d_n10;
        locals.var_t3__blk289_dn11 = assign17220_e21158_d_n11;
        locals.var_t3__blk289_dn12 = assign17220_e21158_d_n12;
        locals.var_t3__blk289_rv = 0.0;

        let (assign17230_e21172, assign17230_e21172_d_n0, assign17230_e21172_d_n2, assign17230_e21172_d_n4, assign17230_e21172_d_n5, assign17230_e21172_d_n6, assign17230_e21172_d_n8, assign17230_e21172_d_n10, assign17230_e21172_d_n11, assign17230_e21172_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17230_e21166: f64 = (locals.var_t3__blk289 * p.p213);
        let assign17230_e21168: f64 = (assign17230_e21166 / 1000000.0);
        let assign17230_e21170: f64 = (assign17230_e21168 * locals.var_cgs_weff_nf__blk301);
        (assign17230_e21170, ((((locals.var_t3__blk289_dn0 * p.p213) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17230_e21168 * locals.var_cgs_weff_nf__blk301_dn0)), ((((locals.var_t3__blk289_dn2 * p.p213) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17230_e21168 * locals.var_cgs_weff_nf__blk301_dn2)), ((((locals.var_t3__blk289_dn4 * p.p213) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17230_e21168 * locals.var_cgs_weff_nf__blk301_dn4)), ((((locals.var_t3__blk289_dn5 * p.p213) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17230_e21168 * locals.var_cgs_weff_nf__blk301_dn5)), ((((locals.var_t3__blk289_dn6 * p.p213) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17230_e21168 * locals.var_cgs_weff_nf__blk301_dn6)), ((((locals.var_t3__blk289_dn8 * p.p213) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17230_e21168 * locals.var_cgs_weff_nf__blk301_dn8)), ((((locals.var_t3__blk289_dn10 * p.p213) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17230_e21168 * locals.var_cgs_weff_nf__blk301_dn10)), ((((locals.var_t3__blk289_dn11 * p.p213) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17230_e21168 * locals.var_cgs_weff_nf__blk301_dn11)), ((((locals.var_t3__blk289_dn12 * p.p213) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17230_e21168 * locals.var_cgs_weff_nf__blk301_dn12)),)
    } else {
        (locals.var_t3__blk289, locals.var_t3__blk289_dn0, locals.var_t3__blk289_dn2, locals.var_t3__blk289_dn4, locals.var_t3__blk289_dn5, locals.var_t3__blk289_dn6, locals.var_t3__blk289_dn8, locals.var_t3__blk289_dn10, locals.var_t3__blk289_dn11, locals.var_t3__blk289_dn12,)
    }
};
        locals.var_t3__blk289 = assign17230_e21172;
        locals.var_t3__blk289_dn0 = assign17230_e21172_d_n0;
        locals.var_t3__blk289_dn2 = assign17230_e21172_d_n2;
        locals.var_t3__blk289_dn4 = assign17230_e21172_d_n4;
        locals.var_t3__blk289_dn5 = assign17230_e21172_d_n5;
        locals.var_t3__blk289_dn6 = assign17230_e21172_d_n6;
        locals.var_t3__blk289_dn8 = assign17230_e21172_d_n8;
        locals.var_t3__blk289_dn10 = assign17230_e21172_d_n10;
        locals.var_t3__blk289_dn11 = assign17230_e21172_d_n11;
        locals.var_t3__blk289_dn12 = assign17230_e21172_d_n12;
        locals.var_t3__blk289_rv = 0.0;

        let (assign17240_e21186, assign17240_e21186_d_n0, assign17240_e21186_d_n2, assign17240_e21186_d_n4, assign17240_e21186_d_n5, assign17240_e21186_d_n6, assign17240_e21186_d_n8, assign17240_e21186_d_n10, assign17240_e21186_d_n11, assign17240_e21186_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17240_e21181: f64 = (locals.var_etun).powf(p.p262);
        let assign17240_e21182: f64 = (locals.var_t3__blk289 * assign17240_e21181);
        let assign17240_e21184: f64 = (assign17240_e21182 * locals.var_t2__blk288);
        (assign17240_e21184, ((((locals.var_t3__blk289_dn0 * assign17240_e21181) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p262) as f64).is_finite() && ((p.p262) as f64).fract() == 0.0 { if p.p262 == 0.0 { 0.0 } else { (p.p262 * ((locals.var_etun).powf(p.p262 - 1.0) * locals.var_etun_dn0)) } } else { (assign17240_e21181 * (p.p262 * (locals.var_etun_dn0 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17240_e21182 * locals.var_t2__blk288_dn0)), ((((locals.var_t3__blk289_dn2 * assign17240_e21181) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p262) as f64).is_finite() && ((p.p262) as f64).fract() == 0.0 { if p.p262 == 0.0 { 0.0 } else { (p.p262 * ((locals.var_etun).powf(p.p262 - 1.0) * locals.var_etun_dn2)) } } else { (assign17240_e21181 * (p.p262 * (locals.var_etun_dn2 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17240_e21182 * locals.var_t2__blk288_dn2)), ((((locals.var_t3__blk289_dn4 * assign17240_e21181) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p262) as f64).is_finite() && ((p.p262) as f64).fract() == 0.0 { if p.p262 == 0.0 { 0.0 } else { (p.p262 * ((locals.var_etun).powf(p.p262 - 1.0) * locals.var_etun_dn4)) } } else { (assign17240_e21181 * (p.p262 * (locals.var_etun_dn4 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17240_e21182 * locals.var_t2__blk288_dn4)), ((((locals.var_t3__blk289_dn5 * assign17240_e21181) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p262) as f64).is_finite() && ((p.p262) as f64).fract() == 0.0 { if p.p262 == 0.0 { 0.0 } else { (p.p262 * ((locals.var_etun).powf(p.p262 - 1.0) * locals.var_etun_dn5)) } } else { (assign17240_e21181 * (p.p262 * (locals.var_etun_dn5 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17240_e21182 * locals.var_t2__blk288_dn5)), ((((locals.var_t3__blk289_dn6 * assign17240_e21181) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p262) as f64).is_finite() && ((p.p262) as f64).fract() == 0.0 { if p.p262 == 0.0 { 0.0 } else { (p.p262 * ((locals.var_etun).powf(p.p262 - 1.0) * locals.var_etun_dn6)) } } else { (assign17240_e21181 * (p.p262 * (locals.var_etun_dn6 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17240_e21182 * locals.var_t2__blk288_dn6)), ((((locals.var_t3__blk289_dn8 * assign17240_e21181) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p262) as f64).is_finite() && ((p.p262) as f64).fract() == 0.0 { if p.p262 == 0.0 { 0.0 } else { (p.p262 * ((locals.var_etun).powf(p.p262 - 1.0) * locals.var_etun_dn8)) } } else { (assign17240_e21181 * (p.p262 * (locals.var_etun_dn8 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17240_e21182 * locals.var_t2__blk288_dn8)), ((((locals.var_t3__blk289_dn10 * assign17240_e21181) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p262) as f64).is_finite() && ((p.p262) as f64).fract() == 0.0 { if p.p262 == 0.0 { 0.0 } else { (p.p262 * ((locals.var_etun).powf(p.p262 - 1.0) * locals.var_etun_dn10)) } } else { (assign17240_e21181 * (p.p262 * (locals.var_etun_dn10 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17240_e21182 * locals.var_t2__blk288_dn10)), ((((locals.var_t3__blk289_dn11 * assign17240_e21181) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p262) as f64).is_finite() && ((p.p262) as f64).fract() == 0.0 { if p.p262 == 0.0 { 0.0 } else { (p.p262 * ((locals.var_etun).powf(p.p262 - 1.0) * locals.var_etun_dn11)) } } else { (assign17240_e21181 * (p.p262 * (locals.var_etun_dn11 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17240_e21182 * locals.var_t2__blk288_dn11)), ((((locals.var_t3__blk289_dn12 * assign17240_e21181) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p262) as f64).is_finite() && ((p.p262) as f64).fract() == 0.0 { if p.p262 == 0.0 { 0.0 } else { (p.p262 * ((locals.var_etun).powf(p.p262 - 1.0) * locals.var_etun_dn12)) } } else { (assign17240_e21181 * (p.p262 * (locals.var_etun_dn12 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17240_e21182 * locals.var_t2__blk288_dn12)),)
    } else {
        (locals.var_igb1, locals.var_igb1_dn0, locals.var_igb1_dn2, locals.var_igb1_dn4, locals.var_igb1_dn5, locals.var_igb1_dn6, locals.var_igb1_dn8, locals.var_igb1_dn10, locals.var_igb1_dn11, locals.var_igb1_dn12,)
    }
};
        locals.var_igb1 = assign17240_e21186;
        locals.var_igb1_dn0 = assign17240_e21186_d_n0;
        locals.var_igb1_dn2 = assign17240_e21186_d_n2;
        locals.var_igb1_dn4 = assign17240_e21186_d_n4;
        locals.var_igb1_dn5 = assign17240_e21186_d_n5;
        locals.var_igb1_dn6 = assign17240_e21186_d_n6;
        locals.var_igb1_dn8 = assign17240_e21186_d_n8;
        locals.var_igb1_dn10 = assign17240_e21186_d_n10;
        locals.var_igb1_dn11 = assign17240_e21186_d_n11;
        locals.var_igb1_dn12 = assign17240_e21186_d_n12;
        locals.var_igb1_rv = 0.0;

        let (assign17250_e21205, assign17250_e21205_d_n0, assign17250_e21205_d_n2, assign17250_e21205_d_n4, assign17250_e21205_d_n5, assign17250_e21205_d_n6, assign17250_e21205_d_n8, assign17250_e21205_d_n10, assign17250_e21205_d_n11, assign17250_e21205_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17250_e21193: f64 = (-locals.var_vgs);
        let assign17250_e21196: f64 = (p.p269 * locals.var_vbs);
        let assign17250_e21197: f64 = (assign17250_e21193 + assign17250_e21196);
        let assign17250_e21199: f64 = (assign17250_e21197 + locals.var_vfb);
        let assign17250_e21201: f64 = (assign17250_e21199 + p.p268);
        let assign17250_e21203: f64 = (assign17250_e21201 / locals.var_cgs_tfox0__blk298);
        (assign17250_e21203, (((p.p269 * locals.var_vbs_dn0) + locals.var_vfb_dn0) / locals.var_cgs_tfox0__blk298), (((p.p269 * locals.var_vbs_dn2) + locals.var_vfb_dn2) / locals.var_cgs_tfox0__blk298), (((p.p269 * locals.var_vbs_dn4) + locals.var_vfb_dn4) / locals.var_cgs_tfox0__blk298), ((((-locals.var_vgs_dn5) + (p.p269 * locals.var_vbs_dn5)) + locals.var_vfb_dn5) / locals.var_cgs_tfox0__blk298), (((p.p269 * locals.var_vbs_dn6) + locals.var_vfb_dn6) / locals.var_cgs_tfox0__blk298), (((p.p269 * locals.var_vbs_dn8) + locals.var_vfb_dn8) / locals.var_cgs_tfox0__blk298), (((p.p269 * locals.var_vbs_dn10) + locals.var_vfb_dn10) / locals.var_cgs_tfox0__blk298), ((((-locals.var_vgs_dn11) + (p.p269 * locals.var_vbs_dn11)) + locals.var_vfb_dn11) / locals.var_cgs_tfox0__blk298), ((((-locals.var_vgs_dn12) + (p.p269 * locals.var_vbs_dn12)) + locals.var_vfb_dn12) / locals.var_cgs_tfox0__blk298),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign17250_e21205;
        locals.var_etun_dn0 = assign17250_e21205_d_n0;
        locals.var_etun_dn2 = assign17250_e21205_d_n2;
        locals.var_etun_dn4 = assign17250_e21205_d_n4;
        locals.var_etun_dn5 = assign17250_e21205_d_n5;
        locals.var_etun_dn6 = assign17250_e21205_d_n6;
        locals.var_etun_dn8 = assign17250_e21205_d_n8;
        locals.var_etun_dn10 = assign17250_e21205_d_n10;
        locals.var_etun_dn11 = assign17250_e21205_d_n11;
        locals.var_etun_dn12 = assign17250_e21205_d_n12;
        locals.var_etun_rv = 0.0;

        let (assign17260_e21222, assign17260_e21222_d_n0, assign17260_e21222_d_n2, assign17260_e21222_d_n4, assign17260_e21222_d_n5, assign17260_e21222_d_n6, assign17260_e21222_d_n8, assign17260_e21222_d_n10, assign17260_e21222_d_n11, assign17260_e21222_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17260_e21213: f64 = (locals.var_etun * locals.var_etun);
        let assign17260_e21216: f64 = (4.0 * 0.01);
        let assign17260_e21218: f64 = (assign17260_e21216 * 0.01);
        let assign17260_e21219: f64 = (assign17260_e21213 + assign17260_e21218);
        let assign17260_e21220: f64 = (assign17260_e21219).sqrt();
        (assign17260_e21220, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign17260_e21220)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign17260_e21220)), (((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4)) / (2.0 * assign17260_e21220)), (((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5)) / (2.0 * assign17260_e21220)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign17260_e21220)), (((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8)) / (2.0 * assign17260_e21220)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign17260_e21220)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign17260_e21220)), (((locals.var_etun_dn12 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn12)) / (2.0 * assign17260_e21220)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17260_e21222;
        locals.var_tmf2_dn0 = assign17260_e21222_d_n0;
        locals.var_tmf2_dn2 = assign17260_e21222_d_n2;
        locals.var_tmf2_dn4 = assign17260_e21222_d_n4;
        locals.var_tmf2_dn5 = assign17260_e21222_d_n5;
        locals.var_tmf2_dn6 = assign17260_e21222_d_n6;
        locals.var_tmf2_dn8 = assign17260_e21222_d_n8;
        locals.var_tmf2_dn10 = assign17260_e21222_d_n10;
        locals.var_tmf2_dn11 = assign17260_e21222_d_n11;
        locals.var_tmf2_dn12 = assign17260_e21222_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17270_e21236, assign17270_e21236_d_n0, assign17270_e21236_d_n2, assign17270_e21236_d_n4, assign17270_e21236_d_n5, assign17270_e21236_d_n6, assign17270_e21236_d_n8, assign17270_e21236_d_n10, assign17270_e21236_d_n11, assign17270_e21236_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17270_e21232: f64 = (locals.var_etun / locals.var_tmf2);
        let assign17270_e21233: f64 = (1.0 + assign17270_e21232);
        let assign17270_e21234: f64 = (0.5 * assign17270_e21233);
        (assign17270_e21234, (0.5 * (((locals.var_etun_dn0 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn2 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn4 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn5 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn6 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn8 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn10 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn11 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn12 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5__blk291, locals.var_t5__blk291_dn0, locals.var_t5__blk291_dn2, locals.var_t5__blk291_dn4, locals.var_t5__blk291_dn5, locals.var_t5__blk291_dn6, locals.var_t5__blk291_dn8, locals.var_t5__blk291_dn10, locals.var_t5__blk291_dn11, locals.var_t5__blk291_dn12,)
    }
};
        locals.var_t5__blk291 = assign17270_e21236;
        locals.var_t5__blk291_dn0 = assign17270_e21236_d_n0;
        locals.var_t5__blk291_dn2 = assign17270_e21236_d_n2;
        locals.var_t5__blk291_dn4 = assign17270_e21236_d_n4;
        locals.var_t5__blk291_dn5 = assign17270_e21236_d_n5;
        locals.var_t5__blk291_dn6 = assign17270_e21236_d_n6;
        locals.var_t5__blk291_dn8 = assign17270_e21236_d_n8;
        locals.var_t5__blk291_dn10 = assign17270_e21236_d_n10;
        locals.var_t5__blk291_dn11 = assign17270_e21236_d_n11;
        locals.var_t5__blk291_dn12 = assign17270_e21236_d_n12;
        locals.var_t5__blk291_rv = 0.0;

        let (assign17280_e21252, assign17280_e21252_d_n0, assign17280_e21252_d_n2, assign17280_e21252_d_n4, assign17280_e21252_d_n5, assign17280_e21252_d_n6, assign17280_e21252_d_n8, assign17280_e21252_d_n10, assign17280_e21252_d_n11, assign17280_e21252_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17280_e21245: f64 = (locals.var_etun + locals.var_tmf2);
        let assign17280_e21246: f64 = (0.5 * assign17280_e21245);
        let assign17280_e21249: f64 = (1e-10 * 0.01);
        let assign17280_e21250: f64 = (assign17280_e21246 + assign17280_e21249);
        (assign17280_e21250, (0.5 * (locals.var_etun_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_etun_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_etun_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_etun_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_etun_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign17280_e21252;
        locals.var_etun_dn0 = assign17280_e21252_d_n0;
        locals.var_etun_dn2 = assign17280_e21252_d_n2;
        locals.var_etun_dn4 = assign17280_e21252_d_n4;
        locals.var_etun_dn5 = assign17280_e21252_d_n5;
        locals.var_etun_dn6 = assign17280_e21252_d_n6;
        locals.var_etun_dn8 = assign17280_e21252_d_n8;
        locals.var_etun_dn10 = assign17280_e21252_d_n10;
        locals.var_etun_dn11 = assign17280_e21252_d_n11;
        locals.var_etun_dn12 = assign17280_e21252_d_n12;
        locals.var_etun_rv = 0.0;

        let assign17290_e21255: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign17290_e21255;
        locals.var_guard314_rv = 0.0;

        let (assign17300_e21265, assign17300_e21265_d_n0, assign17300_e21265_d_n2, assign17300_e21265_d_n4, assign17300_e21265_d_n5, assign17300_e21265_d_n6, assign17300_e21265_d_n8, assign17300_e21265_d_n10, assign17300_e21265_d_n11, assign17300_e21265_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard314 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign17300_e21265;
        locals.var_etun_dn0 = assign17300_e21265_d_n0;
        locals.var_etun_dn2 = assign17300_e21265_d_n2;
        locals.var_etun_dn4 = assign17300_e21265_d_n4;
        locals.var_etun_dn5 = assign17300_e21265_d_n5;
        locals.var_etun_dn6 = assign17300_e21265_d_n6;
        locals.var_etun_dn8 = assign17300_e21265_d_n8;
        locals.var_etun_dn10 = assign17300_e21265_d_n10;
        locals.var_etun_dn11 = assign17300_e21265_d_n11;
        locals.var_etun_dn12 = assign17300_e21265_d_n12;
        locals.var_etun_rv = 0.0;

        let (assign17310_e21275, assign17310_e21275_d_n0, assign17310_e21275_d_n2, assign17310_e21275_d_n4, assign17310_e21275_d_n5, assign17310_e21275_d_n6, assign17310_e21275_d_n8, assign17310_e21275_d_n10, assign17310_e21275_d_n11, assign17310_e21275_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard314 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk291, locals.var_t5__blk291_dn0, locals.var_t5__blk291_dn2, locals.var_t5__blk291_dn4, locals.var_t5__blk291_dn5, locals.var_t5__blk291_dn6, locals.var_t5__blk291_dn8, locals.var_t5__blk291_dn10, locals.var_t5__blk291_dn11, locals.var_t5__blk291_dn12,)
    }
};
        locals.var_t5__blk291 = assign17310_e21275;
        locals.var_t5__blk291_dn0 = assign17310_e21275_d_n0;
        locals.var_t5__blk291_dn2 = assign17310_e21275_d_n2;
        locals.var_t5__blk291_dn4 = assign17310_e21275_d_n4;
        locals.var_t5__blk291_dn5 = assign17310_e21275_d_n5;
        locals.var_t5__blk291_dn6 = assign17310_e21275_d_n6;
        locals.var_t5__blk291_dn8 = assign17310_e21275_d_n8;
        locals.var_t5__blk291_dn10 = assign17310_e21275_d_n10;
        locals.var_t5__blk291_dn11 = assign17310_e21275_d_n11;
        locals.var_t5__blk291_dn12 = assign17310_e21275_d_n12;
        locals.var_t5__blk291_rv = 0.0;

        let (assign17320_e21285, assign17320_e21285_d_n0, assign17320_e21285_d_n2, assign17320_e21285_d_n4, assign17320_e21285_d_n5, assign17320_e21285_d_n6, assign17320_e21285_d_n8, assign17320_e21285_d_n10, assign17320_e21285_d_n11, assign17320_e21285_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17320_e21283: f64 = (locals.var_etun + 1e-50);
        (assign17320_e21283, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign17320_e21285;
        locals.var_etun_dn0 = assign17320_e21285_d_n0;
        locals.var_etun_dn2 = assign17320_e21285_d_n2;
        locals.var_etun_dn4 = assign17320_e21285_d_n4;
        locals.var_etun_dn5 = assign17320_e21285_d_n5;
        locals.var_etun_dn6 = assign17320_e21285_d_n6;
        locals.var_etun_dn8 = assign17320_e21285_d_n8;
        locals.var_etun_dn10 = assign17320_e21285_d_n10;
        locals.var_etun_dn11 = assign17320_e21285_d_n11;
        locals.var_etun_dn12 = assign17320_e21285_d_n12;
        locals.var_etun_rv = 0.0;

        let (assign17330_e21298, assign17330_e21298_d_n0, assign17330_e21298_d_n2, assign17330_e21298_d_n4, assign17330_e21298_d_n5, assign17330_e21298_d_n6, assign17330_e21298_d_n8, assign17330_e21298_d_n10, assign17330_e21298_d_n11, assign17330_e21298_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17330_e21292: f64 = (-p.p267);
        let assign17330_e21295: f64 = (locals.var_etun).powf(p.p271);
        let assign17330_e21296: f64 = (assign17330_e21292 / assign17330_e21295);
        (assign17330_e21296, (-((assign17330_e21292 * if 0.0 == 0.0 && ((p.p271) as f64).is_finite() && ((p.p271) as f64).fract() == 0.0 { if p.p271 == 0.0 { 0.0 } else { (p.p271 * ((locals.var_etun).powf(p.p271 - 1.0) * locals.var_etun_dn0)) } } else { (assign17330_e21295 * (p.p271 * (locals.var_etun_dn0 / locals.var_etun))) }) / (assign17330_e21295 * assign17330_e21295))), (-((assign17330_e21292 * if 0.0 == 0.0 && ((p.p271) as f64).is_finite() && ((p.p271) as f64).fract() == 0.0 { if p.p271 == 0.0 { 0.0 } else { (p.p271 * ((locals.var_etun).powf(p.p271 - 1.0) * locals.var_etun_dn2)) } } else { (assign17330_e21295 * (p.p271 * (locals.var_etun_dn2 / locals.var_etun))) }) / (assign17330_e21295 * assign17330_e21295))), (-((assign17330_e21292 * if 0.0 == 0.0 && ((p.p271) as f64).is_finite() && ((p.p271) as f64).fract() == 0.0 { if p.p271 == 0.0 { 0.0 } else { (p.p271 * ((locals.var_etun).powf(p.p271 - 1.0) * locals.var_etun_dn4)) } } else { (assign17330_e21295 * (p.p271 * (locals.var_etun_dn4 / locals.var_etun))) }) / (assign17330_e21295 * assign17330_e21295))), (-((assign17330_e21292 * if 0.0 == 0.0 && ((p.p271) as f64).is_finite() && ((p.p271) as f64).fract() == 0.0 { if p.p271 == 0.0 { 0.0 } else { (p.p271 * ((locals.var_etun).powf(p.p271 - 1.0) * locals.var_etun_dn5)) } } else { (assign17330_e21295 * (p.p271 * (locals.var_etun_dn5 / locals.var_etun))) }) / (assign17330_e21295 * assign17330_e21295))), (-((assign17330_e21292 * if 0.0 == 0.0 && ((p.p271) as f64).is_finite() && ((p.p271) as f64).fract() == 0.0 { if p.p271 == 0.0 { 0.0 } else { (p.p271 * ((locals.var_etun).powf(p.p271 - 1.0) * locals.var_etun_dn6)) } } else { (assign17330_e21295 * (p.p271 * (locals.var_etun_dn6 / locals.var_etun))) }) / (assign17330_e21295 * assign17330_e21295))), (-((assign17330_e21292 * if 0.0 == 0.0 && ((p.p271) as f64).is_finite() && ((p.p271) as f64).fract() == 0.0 { if p.p271 == 0.0 { 0.0 } else { (p.p271 * ((locals.var_etun).powf(p.p271 - 1.0) * locals.var_etun_dn8)) } } else { (assign17330_e21295 * (p.p271 * (locals.var_etun_dn8 / locals.var_etun))) }) / (assign17330_e21295 * assign17330_e21295))), (-((assign17330_e21292 * if 0.0 == 0.0 && ((p.p271) as f64).is_finite() && ((p.p271) as f64).fract() == 0.0 { if p.p271 == 0.0 { 0.0 } else { (p.p271 * ((locals.var_etun).powf(p.p271 - 1.0) * locals.var_etun_dn10)) } } else { (assign17330_e21295 * (p.p271 * (locals.var_etun_dn10 / locals.var_etun))) }) / (assign17330_e21295 * assign17330_e21295))), (-((assign17330_e21292 * if 0.0 == 0.0 && ((p.p271) as f64).is_finite() && ((p.p271) as f64).fract() == 0.0 { if p.p271 == 0.0 { 0.0 } else { (p.p271 * ((locals.var_etun).powf(p.p271 - 1.0) * locals.var_etun_dn11)) } } else { (assign17330_e21295 * (p.p271 * (locals.var_etun_dn11 / locals.var_etun))) }) / (assign17330_e21295 * assign17330_e21295))), (-((assign17330_e21292 * if 0.0 == 0.0 && ((p.p271) as f64).is_finite() && ((p.p271) as f64).fract() == 0.0 { if p.p271 == 0.0 { 0.0 } else { (p.p271 * ((locals.var_etun).powf(p.p271 - 1.0) * locals.var_etun_dn12)) } } else { (assign17330_e21295 * (p.p271 * (locals.var_etun_dn12 / locals.var_etun))) }) / (assign17330_e21295 * assign17330_e21295))),)
    } else {
        (locals.var_t1__blk287, locals.var_t1__blk287_dn0, locals.var_t1__blk287_dn2, locals.var_t1__blk287_dn4, locals.var_t1__blk287_dn5, locals.var_t1__blk287_dn6, locals.var_t1__blk287_dn8, locals.var_t1__blk287_dn10, locals.var_t1__blk287_dn11, locals.var_t1__blk287_dn12,)
    }
};
        locals.var_t1__blk287 = assign17330_e21298;
        locals.var_t1__blk287_dn0 = assign17330_e21298_d_n0;
        locals.var_t1__blk287_dn2 = assign17330_e21298_d_n2;
        locals.var_t1__blk287_dn4 = assign17330_e21298_d_n4;
        locals.var_t1__blk287_dn5 = assign17330_e21298_d_n5;
        locals.var_t1__blk287_dn6 = assign17330_e21298_d_n6;
        locals.var_t1__blk287_dn8 = assign17330_e21298_d_n8;
        locals.var_t1__blk287_dn10 = assign17330_e21298_d_n10;
        locals.var_t1__blk287_dn11 = assign17330_e21298_d_n11;
        locals.var_t1__blk287_dn12 = assign17330_e21298_d_n12;
        locals.var_t1__blk287_rv = 0.0;

        let assign17340_e21301: f64 = (-34.0);
        let assign17340_e21302: f64 = if locals.var_t1__blk287 < assign17340_e21301 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign17340_e21302;
        locals.var_guard315_rv = 0.0;

        let (assign17350_e21312, assign17350_e21312_d_n0, assign17350_e21312_d_n2, assign17350_e21312_d_n4, assign17350_e21312_d_n5, assign17350_e21312_d_n6, assign17350_e21312_d_n8, assign17350_e21312_d_n10, assign17350_e21312_d_n11, assign17350_e21312_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igb2, locals.var_igb2_dn0, locals.var_igb2_dn2, locals.var_igb2_dn4, locals.var_igb2_dn5, locals.var_igb2_dn6, locals.var_igb2_dn8, locals.var_igb2_dn10, locals.var_igb2_dn11, locals.var_igb2_dn12,)
    }
};
        locals.var_igb2 = assign17350_e21312;
        locals.var_igb2_dn0 = assign17350_e21312_d_n0;
        locals.var_igb2_dn2 = assign17350_e21312_d_n2;
        locals.var_igb2_dn4 = assign17350_e21312_d_n4;
        locals.var_igb2_dn5 = assign17350_e21312_d_n5;
        locals.var_igb2_dn6 = assign17350_e21312_d_n6;
        locals.var_igb2_dn8 = assign17350_e21312_d_n8;
        locals.var_igb2_dn10 = assign17350_e21312_d_n10;
        locals.var_igb2_dn11 = assign17350_e21312_d_n11;
        locals.var_igb2_dn12 = assign17350_e21312_d_n12;
        locals.var_igb2_rv = 0.0;

        let (assign17360_e21324, assign17360_e21324_d_n0, assign17360_e21324_d_n2, assign17360_e21324_d_n4, assign17360_e21324_d_n5, assign17360_e21324_d_n6, assign17360_e21324_d_n8, assign17360_e21324_d_n10, assign17360_e21324_d_n11, assign17360_e21324_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign17360_e21322: f64 = (locals.var_t1__blk287).exp();
        (assign17360_e21322, (assign17360_e21322 * locals.var_t1__blk287_dn0), (assign17360_e21322 * locals.var_t1__blk287_dn2), (assign17360_e21322 * locals.var_t1__blk287_dn4), (assign17360_e21322 * locals.var_t1__blk287_dn5), (assign17360_e21322 * locals.var_t1__blk287_dn6), (assign17360_e21322 * locals.var_t1__blk287_dn8), (assign17360_e21322 * locals.var_t1__blk287_dn10), (assign17360_e21322 * locals.var_t1__blk287_dn11), (assign17360_e21322 * locals.var_t1__blk287_dn12),)
    } else {
        (locals.var_t2__blk288, locals.var_t2__blk288_dn0, locals.var_t2__blk288_dn2, locals.var_t2__blk288_dn4, locals.var_t2__blk288_dn5, locals.var_t2__blk288_dn6, locals.var_t2__blk288_dn8, locals.var_t2__blk288_dn10, locals.var_t2__blk288_dn11, locals.var_t2__blk288_dn12,)
    }
};
        locals.var_t2__blk288 = assign17360_e21324;
        locals.var_t2__blk288_dn0 = assign17360_e21324_d_n0;
        locals.var_t2__blk288_dn2 = assign17360_e21324_d_n2;
        locals.var_t2__blk288_dn4 = assign17360_e21324_d_n4;
        locals.var_t2__blk288_dn5 = assign17360_e21324_d_n5;
        locals.var_t2__blk288_dn6 = assign17360_e21324_d_n6;
        locals.var_t2__blk288_dn8 = assign17360_e21324_d_n8;
        locals.var_t2__blk288_dn10 = assign17360_e21324_d_n10;
        locals.var_t2__blk288_dn11 = assign17360_e21324_d_n11;
        locals.var_t2__blk288_dn12 = assign17360_e21324_d_n12;
        locals.var_t2__blk288_rv = 0.0;

        let (assign17370_e21337, assign17370_e21337_d_n0, assign17370_e21337_d_n2, assign17370_e21337_d_n4, assign17370_e21337_d_n5, assign17370_e21337_d_n6, assign17370_e21337_d_n8, assign17370_e21337_d_n10, assign17370_e21337_d_n11, assign17370_e21337_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign17370_e21335: f64 = (locals.var_lg + p.p272);
        (assign17370_e21335, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk289, locals.var_t3__blk289_dn0, locals.var_t3__blk289_dn2, locals.var_t3__blk289_dn4, locals.var_t3__blk289_dn5, locals.var_t3__blk289_dn6, locals.var_t3__blk289_dn8, locals.var_t3__blk289_dn10, locals.var_t3__blk289_dn11, locals.var_t3__blk289_dn12,)
    }
};
        locals.var_t3__blk289 = assign17370_e21337;
        locals.var_t3__blk289_dn0 = assign17370_e21337_d_n0;
        locals.var_t3__blk289_dn2 = assign17370_e21337_d_n2;
        locals.var_t3__blk289_dn4 = assign17370_e21337_d_n4;
        locals.var_t3__blk289_dn5 = assign17370_e21337_d_n5;
        locals.var_t3__blk289_dn6 = assign17370_e21337_d_n6;
        locals.var_t3__blk289_dn8 = assign17370_e21337_d_n8;
        locals.var_t3__blk289_dn10 = assign17370_e21337_d_n10;
        locals.var_t3__blk289_dn11 = assign17370_e21337_d_n11;
        locals.var_t3__blk289_dn12 = assign17370_e21337_d_n12;
        locals.var_t3__blk289_rv = 0.0;

        let (assign17380_e21354, assign17380_e21354_d_n0, assign17380_e21354_d_n2, assign17380_e21354_d_n4, assign17380_e21354_d_n5, assign17380_e21354_d_n6, assign17380_e21354_d_n8, assign17380_e21354_d_n10, assign17380_e21354_d_n11, assign17380_e21354_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign17380_e21348: f64 = (locals.var_t3__blk289 - p.p273);
        let assign17380_e21351: f64 = (locals.var_t3__blk289 * 0.001);
        let assign17380_e21352: f64 = (assign17380_e21348 - assign17380_e21351);
        (assign17380_e21352, (locals.var_t3__blk289_dn0 - (locals.var_t3__blk289_dn0 * 0.001)), (locals.var_t3__blk289_dn2 - (locals.var_t3__blk289_dn2 * 0.001)), (locals.var_t3__blk289_dn4 - (locals.var_t3__blk289_dn4 * 0.001)), (locals.var_t3__blk289_dn5 - (locals.var_t3__blk289_dn5 * 0.001)), (locals.var_t3__blk289_dn6 - (locals.var_t3__blk289_dn6 * 0.001)), (locals.var_t3__blk289_dn8 - (locals.var_t3__blk289_dn8 * 0.001)), (locals.var_t3__blk289_dn10 - (locals.var_t3__blk289_dn10 * 0.001)), (locals.var_t3__blk289_dn11 - (locals.var_t3__blk289_dn11 * 0.001)), (locals.var_t3__blk289_dn12 - (locals.var_t3__blk289_dn12 * 0.001)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign17380_e21354;
        locals.var_tmf1_dn0 = assign17380_e21354_d_n0;
        locals.var_tmf1_dn2 = assign17380_e21354_d_n2;
        locals.var_tmf1_dn4 = assign17380_e21354_d_n4;
        locals.var_tmf1_dn5 = assign17380_e21354_d_n5;
        locals.var_tmf1_dn6 = assign17380_e21354_d_n6;
        locals.var_tmf1_dn8 = assign17380_e21354_d_n8;
        locals.var_tmf1_dn10 = assign17380_e21354_d_n10;
        locals.var_tmf1_dn11 = assign17380_e21354_d_n11;
        locals.var_tmf1_dn12 = assign17380_e21354_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign17390_e21371, assign17390_e21371_d_n0, assign17390_e21371_d_n2, assign17390_e21371_d_n4, assign17390_e21371_d_n5, assign17390_e21371_d_n6, assign17390_e21371_d_n8, assign17390_e21371_d_n10, assign17390_e21371_d_n11, assign17390_e21371_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign17390_e21365: f64 = (4.0 * p.p273);
        let assign17390_e21368: f64 = (locals.var_t3__blk289 * 0.001);
        let assign17390_e21369: f64 = (assign17390_e21365 * assign17390_e21368);
        (assign17390_e21369, (assign17390_e21365 * (locals.var_t3__blk289_dn0 * 0.001)), (assign17390_e21365 * (locals.var_t3__blk289_dn2 * 0.001)), (assign17390_e21365 * (locals.var_t3__blk289_dn4 * 0.001)), (assign17390_e21365 * (locals.var_t3__blk289_dn5 * 0.001)), (assign17390_e21365 * (locals.var_t3__blk289_dn6 * 0.001)), (assign17390_e21365 * (locals.var_t3__blk289_dn8 * 0.001)), (assign17390_e21365 * (locals.var_t3__blk289_dn10 * 0.001)), (assign17390_e21365 * (locals.var_t3__blk289_dn11 * 0.001)), (assign17390_e21365 * (locals.var_t3__blk289_dn12 * 0.001)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17390_e21371;
        locals.var_tmf2_dn0 = assign17390_e21371_d_n0;
        locals.var_tmf2_dn2 = assign17390_e21371_d_n2;
        locals.var_tmf2_dn4 = assign17390_e21371_d_n4;
        locals.var_tmf2_dn5 = assign17390_e21371_d_n5;
        locals.var_tmf2_dn6 = assign17390_e21371_d_n6;
        locals.var_tmf2_dn8 = assign17390_e21371_d_n8;
        locals.var_tmf2_dn10 = assign17390_e21371_d_n10;
        locals.var_tmf2_dn11 = assign17390_e21371_d_n11;
        locals.var_tmf2_dn12 = assign17390_e21371_d_n12;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_70(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17400_e21388, assign17400_e21388_d_n0, assign17400_e21388_d_n2, assign17400_e21388_d_n4, assign17400_e21388_d_n5, assign17400_e21388_d_n6, assign17400_e21388_d_n8, assign17400_e21388_d_n10, assign17400_e21388_d_n11, assign17400_e21388_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 == 0.0)) {
        let (assign17400_e21386, assign17400_e21386_d_n0, assign17400_e21386_d_n2, assign17400_e21386_d_n4, assign17400_e21386_d_n5, assign17400_e21386_d_n6, assign17400_e21386_d_n8, assign17400_e21386_d_n10, assign17400_e21386_d_n11, assign17400_e21386_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign17400_e21385: f64 = (-locals.var_tmf2);
                (assign17400_e21385, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign17400_e21386, assign17400_e21386_d_n0, assign17400_e21386_d_n2, assign17400_e21386_d_n4, assign17400_e21386_d_n5, assign17400_e21386_d_n6, assign17400_e21386_d_n8, assign17400_e21386_d_n10, assign17400_e21386_d_n11, assign17400_e21386_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17400_e21388;
        locals.var_tmf2_dn0 = assign17400_e21388_d_n0;
        locals.var_tmf2_dn2 = assign17400_e21388_d_n2;
        locals.var_tmf2_dn4 = assign17400_e21388_d_n4;
        locals.var_tmf2_dn5 = assign17400_e21388_d_n5;
        locals.var_tmf2_dn6 = assign17400_e21388_d_n6;
        locals.var_tmf2_dn8 = assign17400_e21388_d_n8;
        locals.var_tmf2_dn10 = assign17400_e21388_d_n10;
        locals.var_tmf2_dn11 = assign17400_e21388_d_n11;
        locals.var_tmf2_dn12 = assign17400_e21388_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17410_e21404, assign17410_e21404_d_n0, assign17410_e21404_d_n2, assign17410_e21404_d_n4, assign17410_e21404_d_n5, assign17410_e21404_d_n6, assign17410_e21404_d_n8, assign17410_e21404_d_n10, assign17410_e21404_d_n11, assign17410_e21404_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign17410_e21399: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign17410_e21401: f64 = (assign17410_e21399 + locals.var_tmf2);
        let assign17410_e21402: f64 = (assign17410_e21401).sqrt();
        (assign17410_e21402, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign17410_e21402)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign17410_e21402)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign17410_e21402)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign17410_e21402)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign17410_e21402)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign17410_e21402)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign17410_e21402)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign17410_e21402)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign17410_e21402)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17410_e21404;
        locals.var_tmf2_dn0 = assign17410_e21404_d_n0;
        locals.var_tmf2_dn2 = assign17410_e21404_d_n2;
        locals.var_tmf2_dn4 = assign17410_e21404_d_n4;
        locals.var_tmf2_dn5 = assign17410_e21404_d_n5;
        locals.var_tmf2_dn6 = assign17410_e21404_d_n6;
        locals.var_tmf2_dn8 = assign17410_e21404_d_n8;
        locals.var_tmf2_dn10 = assign17410_e21404_d_n10;
        locals.var_tmf2_dn11 = assign17410_e21404_d_n11;
        locals.var_tmf2_dn12 = assign17410_e21404_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17420_e21421, assign17420_e21421_d_n0, assign17420_e21421_d_n2, assign17420_e21421_d_n4, assign17420_e21421_d_n5, assign17420_e21421_d_n6, assign17420_e21421_d_n8, assign17420_e21421_d_n10, assign17420_e21421_d_n11, assign17420_e21421_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign17420_e21417: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign17420_e21418: f64 = (1.0 + assign17420_e21417);
        let assign17420_e21419: f64 = (0.5 * assign17420_e21418);
        (assign17420_e21419, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0__blk286, locals.var_t0__blk286_dn0, locals.var_t0__blk286_dn2, locals.var_t0__blk286_dn4, locals.var_t0__blk286_dn5, locals.var_t0__blk286_dn6, locals.var_t0__blk286_dn8, locals.var_t0__blk286_dn10, locals.var_t0__blk286_dn11, locals.var_t0__blk286_dn12,)
    }
};
        locals.var_t0__blk286 = assign17420_e21421;
        locals.var_t0__blk286_dn0 = assign17420_e21421_d_n0;
        locals.var_t0__blk286_dn2 = assign17420_e21421_d_n2;
        locals.var_t0__blk286_dn4 = assign17420_e21421_d_n4;
        locals.var_t0__blk286_dn5 = assign17420_e21421_d_n5;
        locals.var_t0__blk286_dn6 = assign17420_e21421_d_n6;
        locals.var_t0__blk286_dn8 = assign17420_e21421_d_n8;
        locals.var_t0__blk286_dn10 = assign17420_e21421_d_n10;
        locals.var_t0__blk286_dn11 = assign17420_e21421_d_n11;
        locals.var_t0__blk286_dn12 = assign17420_e21421_d_n12;
        locals.var_t0__blk286_rv = 0.0;

        let (assign17430_e21438, assign17430_e21438_d_n0, assign17430_e21438_d_n2, assign17430_e21438_d_n4, assign17430_e21438_d_n5, assign17430_e21438_d_n6, assign17430_e21438_d_n8, assign17430_e21438_d_n10, assign17430_e21438_d_n11, assign17430_e21438_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign17430_e21434: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign17430_e21435: f64 = (0.5 * assign17430_e21434);
        let assign17430_e21436: f64 = (p.p273 + assign17430_e21435);
        (assign17430_e21436, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t3__blk289, locals.var_t3__blk289_dn0, locals.var_t3__blk289_dn2, locals.var_t3__blk289_dn4, locals.var_t3__blk289_dn5, locals.var_t3__blk289_dn6, locals.var_t3__blk289_dn8, locals.var_t3__blk289_dn10, locals.var_t3__blk289_dn11, locals.var_t3__blk289_dn12,)
    }
};
        locals.var_t3__blk289 = assign17430_e21438;
        locals.var_t3__blk289_dn0 = assign17430_e21438_d_n0;
        locals.var_t3__blk289_dn2 = assign17430_e21438_d_n2;
        locals.var_t3__blk289_dn4 = assign17430_e21438_d_n4;
        locals.var_t3__blk289_dn5 = assign17430_e21438_d_n5;
        locals.var_t3__blk289_dn6 = assign17430_e21438_d_n6;
        locals.var_t3__blk289_dn8 = assign17430_e21438_d_n8;
        locals.var_t3__blk289_dn10 = assign17430_e21438_d_n10;
        locals.var_t3__blk289_dn11 = assign17430_e21438_d_n11;
        locals.var_t3__blk289_dn12 = assign17430_e21438_d_n12;
        locals.var_t3__blk289_rv = 0.0;

        let (assign17440_e21455, assign17440_e21455_d_n0, assign17440_e21455_d_n2, assign17440_e21455_d_n4, assign17440_e21455_d_n5, assign17440_e21455_d_n6, assign17440_e21455_d_n8, assign17440_e21455_d_n10, assign17440_e21455_d_n11, assign17440_e21455_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign17440_e21449: f64 = (locals.var_t3__blk289 * p.p266);
        let assign17440_e21451: f64 = (assign17440_e21449 / 1000000.0);
        let assign17440_e21453: f64 = (assign17440_e21451 * locals.var_cgs_weff_nf__blk301);
        (assign17440_e21453, ((((locals.var_t3__blk289_dn0 * p.p266) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17440_e21451 * locals.var_cgs_weff_nf__blk301_dn0)), ((((locals.var_t3__blk289_dn2 * p.p266) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17440_e21451 * locals.var_cgs_weff_nf__blk301_dn2)), ((((locals.var_t3__blk289_dn4 * p.p266) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17440_e21451 * locals.var_cgs_weff_nf__blk301_dn4)), ((((locals.var_t3__blk289_dn5 * p.p266) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17440_e21451 * locals.var_cgs_weff_nf__blk301_dn5)), ((((locals.var_t3__blk289_dn6 * p.p266) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17440_e21451 * locals.var_cgs_weff_nf__blk301_dn6)), ((((locals.var_t3__blk289_dn8 * p.p266) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17440_e21451 * locals.var_cgs_weff_nf__blk301_dn8)), ((((locals.var_t3__blk289_dn10 * p.p266) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17440_e21451 * locals.var_cgs_weff_nf__blk301_dn10)), ((((locals.var_t3__blk289_dn11 * p.p266) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17440_e21451 * locals.var_cgs_weff_nf__blk301_dn11)), ((((locals.var_t3__blk289_dn12 * p.p266) / 1000000.0) * locals.var_cgs_weff_nf__blk301) + (assign17440_e21451 * locals.var_cgs_weff_nf__blk301_dn12)),)
    } else {
        (locals.var_t3__blk289, locals.var_t3__blk289_dn0, locals.var_t3__blk289_dn2, locals.var_t3__blk289_dn4, locals.var_t3__blk289_dn5, locals.var_t3__blk289_dn6, locals.var_t3__blk289_dn8, locals.var_t3__blk289_dn10, locals.var_t3__blk289_dn11, locals.var_t3__blk289_dn12,)
    }
};
        locals.var_t3__blk289 = assign17440_e21455;
        locals.var_t3__blk289_dn0 = assign17440_e21455_d_n0;
        locals.var_t3__blk289_dn2 = assign17440_e21455_d_n2;
        locals.var_t3__blk289_dn4 = assign17440_e21455_d_n4;
        locals.var_t3__blk289_dn5 = assign17440_e21455_d_n5;
        locals.var_t3__blk289_dn6 = assign17440_e21455_d_n6;
        locals.var_t3__blk289_dn8 = assign17440_e21455_d_n8;
        locals.var_t3__blk289_dn10 = assign17440_e21455_d_n10;
        locals.var_t3__blk289_dn11 = assign17440_e21455_d_n11;
        locals.var_t3__blk289_dn12 = assign17440_e21455_d_n12;
        locals.var_t3__blk289_rv = 0.0;

        let (assign17450_e21472, assign17450_e21472_d_n0, assign17450_e21472_d_n2, assign17450_e21472_d_n4, assign17450_e21472_d_n5, assign17450_e21472_d_n6, assign17450_e21472_d_n8, assign17450_e21472_d_n10, assign17450_e21472_d_n11, assign17450_e21472_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign17450_e21467: f64 = (locals.var_etun).powf(p.p270);
        let assign17450_e21468: f64 = (locals.var_t3__blk289 * assign17450_e21467);
        let assign17450_e21470: f64 = (assign17450_e21468 * locals.var_t2__blk288);
        (assign17450_e21470, ((((locals.var_t3__blk289_dn0 * assign17450_e21467) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p270) as f64).is_finite() && ((p.p270) as f64).fract() == 0.0 { if p.p270 == 0.0 { 0.0 } else { (p.p270 * ((locals.var_etun).powf(p.p270 - 1.0) * locals.var_etun_dn0)) } } else { (assign17450_e21467 * (p.p270 * (locals.var_etun_dn0 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17450_e21468 * locals.var_t2__blk288_dn0)), ((((locals.var_t3__blk289_dn2 * assign17450_e21467) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p270) as f64).is_finite() && ((p.p270) as f64).fract() == 0.0 { if p.p270 == 0.0 { 0.0 } else { (p.p270 * ((locals.var_etun).powf(p.p270 - 1.0) * locals.var_etun_dn2)) } } else { (assign17450_e21467 * (p.p270 * (locals.var_etun_dn2 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17450_e21468 * locals.var_t2__blk288_dn2)), ((((locals.var_t3__blk289_dn4 * assign17450_e21467) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p270) as f64).is_finite() && ((p.p270) as f64).fract() == 0.0 { if p.p270 == 0.0 { 0.0 } else { (p.p270 * ((locals.var_etun).powf(p.p270 - 1.0) * locals.var_etun_dn4)) } } else { (assign17450_e21467 * (p.p270 * (locals.var_etun_dn4 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17450_e21468 * locals.var_t2__blk288_dn4)), ((((locals.var_t3__blk289_dn5 * assign17450_e21467) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p270) as f64).is_finite() && ((p.p270) as f64).fract() == 0.0 { if p.p270 == 0.0 { 0.0 } else { (p.p270 * ((locals.var_etun).powf(p.p270 - 1.0) * locals.var_etun_dn5)) } } else { (assign17450_e21467 * (p.p270 * (locals.var_etun_dn5 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17450_e21468 * locals.var_t2__blk288_dn5)), ((((locals.var_t3__blk289_dn6 * assign17450_e21467) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p270) as f64).is_finite() && ((p.p270) as f64).fract() == 0.0 { if p.p270 == 0.0 { 0.0 } else { (p.p270 * ((locals.var_etun).powf(p.p270 - 1.0) * locals.var_etun_dn6)) } } else { (assign17450_e21467 * (p.p270 * (locals.var_etun_dn6 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17450_e21468 * locals.var_t2__blk288_dn6)), ((((locals.var_t3__blk289_dn8 * assign17450_e21467) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p270) as f64).is_finite() && ((p.p270) as f64).fract() == 0.0 { if p.p270 == 0.0 { 0.0 } else { (p.p270 * ((locals.var_etun).powf(p.p270 - 1.0) * locals.var_etun_dn8)) } } else { (assign17450_e21467 * (p.p270 * (locals.var_etun_dn8 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17450_e21468 * locals.var_t2__blk288_dn8)), ((((locals.var_t3__blk289_dn10 * assign17450_e21467) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p270) as f64).is_finite() && ((p.p270) as f64).fract() == 0.0 { if p.p270 == 0.0 { 0.0 } else { (p.p270 * ((locals.var_etun).powf(p.p270 - 1.0) * locals.var_etun_dn10)) } } else { (assign17450_e21467 * (p.p270 * (locals.var_etun_dn10 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17450_e21468 * locals.var_t2__blk288_dn10)), ((((locals.var_t3__blk289_dn11 * assign17450_e21467) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p270) as f64).is_finite() && ((p.p270) as f64).fract() == 0.0 { if p.p270 == 0.0 { 0.0 } else { (p.p270 * ((locals.var_etun).powf(p.p270 - 1.0) * locals.var_etun_dn11)) } } else { (assign17450_e21467 * (p.p270 * (locals.var_etun_dn11 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17450_e21468 * locals.var_t2__blk288_dn11)), ((((locals.var_t3__blk289_dn12 * assign17450_e21467) + (locals.var_t3__blk289 * if 0.0 == 0.0 && ((p.p270) as f64).is_finite() && ((p.p270) as f64).fract() == 0.0 { if p.p270 == 0.0 { 0.0 } else { (p.p270 * ((locals.var_etun).powf(p.p270 - 1.0) * locals.var_etun_dn12)) } } else { (assign17450_e21467 * (p.p270 * (locals.var_etun_dn12 / locals.var_etun))) })) * locals.var_t2__blk288) + (assign17450_e21468 * locals.var_t2__blk288_dn12)),)
    } else {
        (locals.var_igb2, locals.var_igb2_dn0, locals.var_igb2_dn2, locals.var_igb2_dn4, locals.var_igb2_dn5, locals.var_igb2_dn6, locals.var_igb2_dn8, locals.var_igb2_dn10, locals.var_igb2_dn11, locals.var_igb2_dn12,)
    }
};
        locals.var_igb2 = assign17450_e21472;
        locals.var_igb2_dn0 = assign17450_e21472_d_n0;
        locals.var_igb2_dn2 = assign17450_e21472_d_n2;
        locals.var_igb2_dn4 = assign17450_e21472_d_n4;
        locals.var_igb2_dn5 = assign17450_e21472_d_n5;
        locals.var_igb2_dn6 = assign17450_e21472_d_n6;
        locals.var_igb2_dn8 = assign17450_e21472_d_n8;
        locals.var_igb2_dn10 = assign17450_e21472_d_n10;
        locals.var_igb2_dn11 = assign17450_e21472_d_n11;
        locals.var_igb2_dn12 = assign17450_e21472_d_n12;
        locals.var_igb2_rv = 0.0;

        let (assign17460_e21483, assign17460_e21483_d_n0, assign17460_e21483_d_n2, assign17460_e21483_d_n4, assign17460_e21483_d_n5, assign17460_e21483_d_n6, assign17460_e21483_d_n8, assign17460_e21483_d_n10, assign17460_e21483_d_n11, assign17460_e21483_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17460_e21479: f64 = (-locals.var_igb1);
        let assign17460_e21481: f64 = (assign17460_e21479 * 0.001);
        (assign17460_e21481, ((-locals.var_igb1_dn0) * 0.001), ((-locals.var_igb1_dn2) * 0.001), ((-locals.var_igb1_dn4) * 0.001), ((-locals.var_igb1_dn5) * 0.001), ((-locals.var_igb1_dn6) * 0.001), ((-locals.var_igb1_dn8) * 0.001), ((-locals.var_igb1_dn10) * 0.001), ((-locals.var_igb1_dn11) * 0.001), ((-locals.var_igb1_dn12) * 0.001),)
    } else {
        (locals.var_t1__blk287, locals.var_t1__blk287_dn0, locals.var_t1__blk287_dn2, locals.var_t1__blk287_dn4, locals.var_t1__blk287_dn5, locals.var_t1__blk287_dn6, locals.var_t1__blk287_dn8, locals.var_t1__blk287_dn10, locals.var_t1__blk287_dn11, locals.var_t1__blk287_dn12,)
    }
};
        locals.var_t1__blk287 = assign17460_e21483;
        locals.var_t1__blk287_dn0 = assign17460_e21483_d_n0;
        locals.var_t1__blk287_dn2 = assign17460_e21483_d_n2;
        locals.var_t1__blk287_dn4 = assign17460_e21483_d_n4;
        locals.var_t1__blk287_dn5 = assign17460_e21483_d_n5;
        locals.var_t1__blk287_dn6 = assign17460_e21483_d_n6;
        locals.var_t1__blk287_dn8 = assign17460_e21483_d_n8;
        locals.var_t1__blk287_dn10 = assign17460_e21483_d_n10;
        locals.var_t1__blk287_dn11 = assign17460_e21483_d_n11;
        locals.var_t1__blk287_dn12 = assign17460_e21483_d_n12;
        locals.var_t1__blk287_rv = 0.0;

        let assign17470_e21486: f64 = if locals.var_t1__blk287 < 1e-50 { 1.0 } else { 0.0 };
        locals.var_guard316 = assign17470_e21486;
        locals.var_guard316_rv = 0.0;

        let (assign17480_e21496, assign17480_e21496_d_n0, assign17480_e21496_d_n2, assign17480_e21496_d_n4, assign17480_e21496_d_n5, assign17480_e21496_d_n6, assign17480_e21496_d_n8, assign17480_e21496_d_n10, assign17480_e21496_d_n11, assign17480_e21496_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) && (locals.var_guard316 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk287, locals.var_t1__blk287_dn0, locals.var_t1__blk287_dn2, locals.var_t1__blk287_dn4, locals.var_t1__blk287_dn5, locals.var_t1__blk287_dn6, locals.var_t1__blk287_dn8, locals.var_t1__blk287_dn10, locals.var_t1__blk287_dn11, locals.var_t1__blk287_dn12,)
    }
};
        locals.var_t1__blk287 = assign17480_e21496;
        locals.var_t1__blk287_dn0 = assign17480_e21496_d_n0;
        locals.var_t1__blk287_dn2 = assign17480_e21496_d_n2;
        locals.var_t1__blk287_dn4 = assign17480_e21496_d_n4;
        locals.var_t1__blk287_dn5 = assign17480_e21496_d_n5;
        locals.var_t1__blk287_dn6 = assign17480_e21496_d_n6;
        locals.var_t1__blk287_dn8 = assign17480_e21496_d_n8;
        locals.var_t1__blk287_dn10 = assign17480_e21496_d_n10;
        locals.var_t1__blk287_dn11 = assign17480_e21496_d_n11;
        locals.var_t1__blk287_dn12 = assign17480_e21496_d_n12;
        locals.var_t1__blk287_rv = 0.0;

        let (assign17490_e21510, assign17490_e21510_d_n0, assign17490_e21510_d_n2, assign17490_e21510_d_n4, assign17490_e21510_d_n5, assign17490_e21510_d_n6, assign17490_e21510_d_n8, assign17490_e21510_d_n10, assign17490_e21510_d_n11, assign17490_e21510_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17490_e21503: f64 = (-locals.var_igb1);
        let assign17490_e21505: f64 = (-locals.var_igb2);
        let assign17490_e21506: f64 = (assign17490_e21503 - assign17490_e21505);
        let assign17490_e21508: f64 = (assign17490_e21506 - locals.var_t1__blk287);
        (assign17490_e21508, (((-locals.var_igb1_dn0) - (-locals.var_igb2_dn0)) - locals.var_t1__blk287_dn0), (((-locals.var_igb1_dn2) - (-locals.var_igb2_dn2)) - locals.var_t1__blk287_dn2), (((-locals.var_igb1_dn4) - (-locals.var_igb2_dn4)) - locals.var_t1__blk287_dn4), (((-locals.var_igb1_dn5) - (-locals.var_igb2_dn5)) - locals.var_t1__blk287_dn5), (((-locals.var_igb1_dn6) - (-locals.var_igb2_dn6)) - locals.var_t1__blk287_dn6), (((-locals.var_igb1_dn8) - (-locals.var_igb2_dn8)) - locals.var_t1__blk287_dn8), (((-locals.var_igb1_dn10) - (-locals.var_igb2_dn10)) - locals.var_t1__blk287_dn10), (((-locals.var_igb1_dn11) - (-locals.var_igb2_dn11)) - locals.var_t1__blk287_dn11), (((-locals.var_igb1_dn12) - (-locals.var_igb2_dn12)) - locals.var_t1__blk287_dn12),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign17490_e21510;
        locals.var_tmf1_dn0 = assign17490_e21510_d_n0;
        locals.var_tmf1_dn2 = assign17490_e21510_d_n2;
        locals.var_tmf1_dn4 = assign17490_e21510_d_n4;
        locals.var_tmf1_dn5 = assign17490_e21510_d_n5;
        locals.var_tmf1_dn6 = assign17490_e21510_d_n6;
        locals.var_tmf1_dn8 = assign17490_e21510_d_n8;
        locals.var_tmf1_dn10 = assign17490_e21510_d_n10;
        locals.var_tmf1_dn11 = assign17490_e21510_d_n11;
        locals.var_tmf1_dn12 = assign17490_e21510_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign17500_e21523, assign17500_e21523_d_n0, assign17500_e21523_d_n2, assign17500_e21523_d_n4, assign17500_e21523_d_n5, assign17500_e21523_d_n6, assign17500_e21523_d_n8, assign17500_e21523_d_n10, assign17500_e21523_d_n11, assign17500_e21523_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17500_e21518: f64 = (-locals.var_igb2);
        let assign17500_e21519: f64 = (4.0 * assign17500_e21518);
        let assign17500_e21521: f64 = (assign17500_e21519 * locals.var_t1__blk287);
        (assign17500_e21521, (((4.0 * (-locals.var_igb2_dn0)) * locals.var_t1__blk287) + (assign17500_e21519 * locals.var_t1__blk287_dn0)), (((4.0 * (-locals.var_igb2_dn2)) * locals.var_t1__blk287) + (assign17500_e21519 * locals.var_t1__blk287_dn2)), (((4.0 * (-locals.var_igb2_dn4)) * locals.var_t1__blk287) + (assign17500_e21519 * locals.var_t1__blk287_dn4)), (((4.0 * (-locals.var_igb2_dn5)) * locals.var_t1__blk287) + (assign17500_e21519 * locals.var_t1__blk287_dn5)), (((4.0 * (-locals.var_igb2_dn6)) * locals.var_t1__blk287) + (assign17500_e21519 * locals.var_t1__blk287_dn6)), (((4.0 * (-locals.var_igb2_dn8)) * locals.var_t1__blk287) + (assign17500_e21519 * locals.var_t1__blk287_dn8)), (((4.0 * (-locals.var_igb2_dn10)) * locals.var_t1__blk287) + (assign17500_e21519 * locals.var_t1__blk287_dn10)), (((4.0 * (-locals.var_igb2_dn11)) * locals.var_t1__blk287) + (assign17500_e21519 * locals.var_t1__blk287_dn11)), (((4.0 * (-locals.var_igb2_dn12)) * locals.var_t1__blk287) + (assign17500_e21519 * locals.var_t1__blk287_dn12)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17500_e21523;
        locals.var_tmf2_dn0 = assign17500_e21523_d_n0;
        locals.var_tmf2_dn2 = assign17500_e21523_d_n2;
        locals.var_tmf2_dn4 = assign17500_e21523_d_n4;
        locals.var_tmf2_dn5 = assign17500_e21523_d_n5;
        locals.var_tmf2_dn6 = assign17500_e21523_d_n6;
        locals.var_tmf2_dn8 = assign17500_e21523_d_n8;
        locals.var_tmf2_dn10 = assign17500_e21523_d_n10;
        locals.var_tmf2_dn11 = assign17500_e21523_d_n11;
        locals.var_tmf2_dn12 = assign17500_e21523_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17510_e21537, assign17510_e21537_d_n0, assign17510_e21537_d_n2, assign17510_e21537_d_n4, assign17510_e21537_d_n5, assign17510_e21537_d_n6, assign17510_e21537_d_n8, assign17510_e21537_d_n10, assign17510_e21537_d_n11, assign17510_e21537_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let (assign17510_e21535, assign17510_e21535_d_n0, assign17510_e21535_d_n2, assign17510_e21535_d_n4, assign17510_e21535_d_n5, assign17510_e21535_d_n6, assign17510_e21535_d_n8, assign17510_e21535_d_n10, assign17510_e21535_d_n11, assign17510_e21535_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign17510_e21534: f64 = (-locals.var_tmf2);
                (assign17510_e21534, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign17510_e21535, assign17510_e21535_d_n0, assign17510_e21535_d_n2, assign17510_e21535_d_n4, assign17510_e21535_d_n5, assign17510_e21535_d_n6, assign17510_e21535_d_n8, assign17510_e21535_d_n10, assign17510_e21535_d_n11, assign17510_e21535_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17510_e21537;
        locals.var_tmf2_dn0 = assign17510_e21537_d_n0;
        locals.var_tmf2_dn2 = assign17510_e21537_d_n2;
        locals.var_tmf2_dn4 = assign17510_e21537_d_n4;
        locals.var_tmf2_dn5 = assign17510_e21537_d_n5;
        locals.var_tmf2_dn6 = assign17510_e21537_d_n6;
        locals.var_tmf2_dn8 = assign17510_e21537_d_n8;
        locals.var_tmf2_dn10 = assign17510_e21537_d_n10;
        locals.var_tmf2_dn11 = assign17510_e21537_d_n11;
        locals.var_tmf2_dn12 = assign17510_e21537_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17520_e21550, assign17520_e21550_d_n0, assign17520_e21550_d_n2, assign17520_e21550_d_n4, assign17520_e21550_d_n5, assign17520_e21550_d_n6, assign17520_e21550_d_n8, assign17520_e21550_d_n10, assign17520_e21550_d_n11, assign17520_e21550_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17520_e21545: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign17520_e21547: f64 = (assign17520_e21545 + locals.var_tmf2);
        let assign17520_e21548: f64 = (assign17520_e21547).sqrt();
        (assign17520_e21548, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign17520_e21548)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign17520_e21548)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign17520_e21548)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign17520_e21548)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign17520_e21548)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign17520_e21548)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign17520_e21548)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign17520_e21548)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign17520_e21548)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17520_e21550;
        locals.var_tmf2_dn0 = assign17520_e21550_d_n0;
        locals.var_tmf2_dn2 = assign17520_e21550_d_n2;
        locals.var_tmf2_dn4 = assign17520_e21550_d_n4;
        locals.var_tmf2_dn5 = assign17520_e21550_d_n5;
        locals.var_tmf2_dn6 = assign17520_e21550_d_n6;
        locals.var_tmf2_dn8 = assign17520_e21550_d_n8;
        locals.var_tmf2_dn10 = assign17520_e21550_d_n10;
        locals.var_tmf2_dn11 = assign17520_e21550_d_n11;
        locals.var_tmf2_dn12 = assign17520_e21550_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17550_e21579,) = {
    if (locals.var_guard305 == 0.0) {
        (0.5,)
    } else {
        (locals.var_glpart1,)
    }
};
        locals.var_glpart1 = assign17550_e21579;
        locals.var_glpart1_rv = 0.0;

        let assign17560_e21582: f64 = if p.p18 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard317 = assign17560_e21582;
        locals.var_guard317_rv = 0.0;

        let (assign17580_e21603, assign17580_e21603_d_n0, assign17580_e21603_d_n2, assign17580_e21603_d_n4, assign17580_e21603_d_n5, assign17580_e21603_d_n6, assign17580_e21603_d_n8, assign17580_e21603_d_n10, assign17580_e21603_d_n11, assign17580_e21603_d_n12,) = {
    if (locals.var_guard317 == 0.0) {
        let assign17580_e21592: f64 = (locals.var_vds + p.p199);
        let assign17580_e21593: f64 = (p.p198 * assign17580_e21592);
        let assign17580_e21595: f64 = (assign17580_e21593 - locals.var_vgs);
        let assign17580_e21598: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign17580_e21600: f64 = (assign17580_e21598 * p.p200);
        let assign17580_e21601: f64 = (assign17580_e21595 - assign17580_e21600);
        (assign17580_e21601, ((p.p198 * locals.var_vds_dn0) - ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p200)), ((p.p198 * locals.var_vds_dn2) - ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p200)), ((p.p198 * locals.var_vds_dn4) - ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) * p.p200)), (((p.p198 * locals.var_vds_dn5) - locals.var_vgs_dn5) - ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) * p.p200)), ((p.p198 * locals.var_vds_dn6) - ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p200)), ((p.p198 * locals.var_vds_dn8) - ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) * p.p200)), ((p.p198 * locals.var_vds_dn10) - ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p200)), (((p.p198 * locals.var_vds_dn11) - locals.var_vgs_dn11) - ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p200)), (((p.p198 * locals.var_vds_dn12) - locals.var_vgs_dn12) - ((locals.var_dvthsc_dn12 + locals.var_dvthlp_dn12) * p.p200)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign17580_e21603;
        locals.var_t1_dn0 = assign17580_e21603_d_n0;
        locals.var_t1_dn2 = assign17580_e21603_d_n2;
        locals.var_t1_dn4 = assign17580_e21603_d_n4;
        locals.var_t1_dn5 = assign17580_e21603_d_n5;
        locals.var_t1_dn6 = assign17580_e21603_d_n6;
        locals.var_t1_dn8 = assign17580_e21603_d_n8;
        locals.var_t1_dn10 = assign17580_e21603_d_n10;
        locals.var_t1_dn11 = assign17580_e21603_d_n11;
        locals.var_t1_dn12 = assign17580_e21603_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign17590_e21610, assign17590_e21610_d_n0, assign17590_e21610_d_n2, assign17590_e21610_d_n4, assign17590_e21610_d_n5, assign17590_e21610_d_n6, assign17590_e21610_d_n8, assign17590_e21610_d_n10, assign17590_e21610_d_n11, assign17590_e21610_d_n12,) = {
    if (locals.var_guard317 == 0.0) {
        let assign17590_e21608: f64 = (locals.var_t1 / p.p228);
        (assign17590_e21608, (locals.var_t1_dn0 / p.p228), (locals.var_t1_dn2 / p.p228), (locals.var_t1_dn4 / p.p228), (locals.var_t1_dn5 / p.p228), (locals.var_t1_dn6 / p.p228), (locals.var_t1_dn8 / p.p228), (locals.var_t1_dn10 / p.p228), (locals.var_t1_dn11 / p.p228), (locals.var_t1_dn12 / p.p228),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn8, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn12,)
    }
};
        locals.var_e1 = assign17590_e21610;
        locals.var_e1_dn0 = assign17590_e21610_d_n0;
        locals.var_e1_dn2 = assign17590_e21610_d_n2;
        locals.var_e1_dn4 = assign17590_e21610_d_n4;
        locals.var_e1_dn5 = assign17590_e21610_d_n5;
        locals.var_e1_dn6 = assign17590_e21610_d_n6;
        locals.var_e1_dn8 = assign17590_e21610_d_n8;
        locals.var_e1_dn10 = assign17590_e21610_d_n10;
        locals.var_e1_dn11 = assign17590_e21610_d_n11;
        locals.var_e1_dn12 = assign17590_e21610_d_n12;
        locals.var_e1_rv = 0.0;

        let (assign17600_e21624, assign17600_e21624_d_n0, assign17600_e21624_d_n2, assign17600_e21624_d_n4, assign17600_e21624_d_n5, assign17600_e21624_d_n6, assign17600_e21624_d_n8, assign17600_e21624_d_n10, assign17600_e21624_d_n11, assign17600_e21624_d_n12,) = {
    if (locals.var_guard317 == 0.0) {
        let assign17600_e21615: f64 = (locals.var_e1 * locals.var_e1);
        let assign17600_e21618: f64 = (4.0 * 0.01);
        let assign17600_e21620: f64 = (assign17600_e21618 * 0.01);
        let assign17600_e21621: f64 = (assign17600_e21615 + assign17600_e21620);
        let assign17600_e21622: f64 = (assign17600_e21621).sqrt();
        (assign17600_e21622, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign17600_e21622)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign17600_e21622)), (((locals.var_e1_dn4 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn4)) / (2.0 * assign17600_e21622)), (((locals.var_e1_dn5 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn5)) / (2.0 * assign17600_e21622)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign17600_e21622)), (((locals.var_e1_dn8 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn8)) / (2.0 * assign17600_e21622)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign17600_e21622)), (((locals.var_e1_dn11 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn11)) / (2.0 * assign17600_e21622)), (((locals.var_e1_dn12 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn12)) / (2.0 * assign17600_e21622)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17600_e21624;
        locals.var_tmf2_dn0 = assign17600_e21624_d_n0;
        locals.var_tmf2_dn2 = assign17600_e21624_d_n2;
        locals.var_tmf2_dn4 = assign17600_e21624_d_n4;
        locals.var_tmf2_dn5 = assign17600_e21624_d_n5;
        locals.var_tmf2_dn6 = assign17600_e21624_d_n6;
        locals.var_tmf2_dn8 = assign17600_e21624_d_n8;
        locals.var_tmf2_dn10 = assign17600_e21624_d_n10;
        locals.var_tmf2_dn11 = assign17600_e21624_d_n11;
        locals.var_tmf2_dn12 = assign17600_e21624_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17610_e21635, assign17610_e21635_d_n0, assign17610_e21635_d_n2, assign17610_e21635_d_n4, assign17610_e21635_d_n5, assign17610_e21635_d_n6, assign17610_e21635_d_n8, assign17610_e21635_d_n10, assign17610_e21635_d_n11, assign17610_e21635_d_n12,) = {
    if (locals.var_guard317 == 0.0) {
        let assign17610_e21631: f64 = (locals.var_e1 / locals.var_tmf2);
        let assign17610_e21632: f64 = (1.0 + assign17610_e21631);
        let assign17610_e21633: f64 = (0.5 * assign17610_e21632);
        (assign17610_e21633, (0.5 * (((locals.var_e1_dn0 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn2 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn4 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn5 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn6 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn8 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn10 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn11 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn12 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign17610_e21635;
        locals.var_t5_dn0 = assign17610_e21635_d_n0;
        locals.var_t5_dn2 = assign17610_e21635_d_n2;
        locals.var_t5_dn4 = assign17610_e21635_d_n4;
        locals.var_t5_dn5 = assign17610_e21635_d_n5;
        locals.var_t5_dn6 = assign17610_e21635_d_n6;
        locals.var_t5_dn8 = assign17610_e21635_d_n8;
        locals.var_t5_dn10 = assign17610_e21635_d_n10;
        locals.var_t5_dn11 = assign17610_e21635_d_n11;
        locals.var_t5_dn12 = assign17610_e21635_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign17620_e21648, assign17620_e21648_d_n0, assign17620_e21648_d_n2, assign17620_e21648_d_n4, assign17620_e21648_d_n5, assign17620_e21648_d_n6, assign17620_e21648_d_n8, assign17620_e21648_d_n10, assign17620_e21648_d_n11, assign17620_e21648_d_n12,) = {
    if (locals.var_guard317 == 0.0) {
        let assign17620_e21641: f64 = (locals.var_e1 + locals.var_tmf2);
        let assign17620_e21642: f64 = (0.5 * assign17620_e21641);
        let assign17620_e21645: f64 = (1e-10 * 0.01);
        let assign17620_e21646: f64 = (assign17620_e21642 + assign17620_e21645);
        (assign17620_e21646, (0.5 * (locals.var_e1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_e1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_e1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_e1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_e1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_e1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn4, locals.var_egidl_dn5, locals.var_egidl_dn6, locals.var_egidl_dn8, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn12,)
    }
};
        locals.var_egidl = assign17620_e21648;
        locals.var_egidl_dn0 = assign17620_e21648_d_n0;
        locals.var_egidl_dn2 = assign17620_e21648_d_n2;
        locals.var_egidl_dn4 = assign17620_e21648_d_n4;
        locals.var_egidl_dn5 = assign17620_e21648_d_n5;
        locals.var_egidl_dn6 = assign17620_e21648_d_n6;
        locals.var_egidl_dn8 = assign17620_e21648_d_n8;
        locals.var_egidl_dn10 = assign17620_e21648_d_n10;
        locals.var_egidl_dn11 = assign17620_e21648_d_n11;
        locals.var_egidl_dn12 = assign17620_e21648_d_n12;
        locals.var_egidl_rv = 0.0;

        let assign17630_e21651: f64 = if locals.var_egidl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard318 = assign17630_e21651;
        locals.var_guard318_rv = 0.0;

        let (assign17640_e21658, assign17640_e21658_d_n0, assign17640_e21658_d_n2, assign17640_e21658_d_n4, assign17640_e21658_d_n5, assign17640_e21658_d_n6, assign17640_e21658_d_n8, assign17640_e21658_d_n10, assign17640_e21658_d_n11, assign17640_e21658_d_n12,) = {
    if ((locals.var_guard317 == 0.0) && (locals.var_guard318 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn4, locals.var_egidl_dn5, locals.var_egidl_dn6, locals.var_egidl_dn8, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn12,)
    }
};
        locals.var_egidl = assign17640_e21658;
        locals.var_egidl_dn0 = assign17640_e21658_d_n0;
        locals.var_egidl_dn2 = assign17640_e21658_d_n2;
        locals.var_egidl_dn4 = assign17640_e21658_d_n4;
        locals.var_egidl_dn5 = assign17640_e21658_d_n5;
        locals.var_egidl_dn6 = assign17640_e21658_d_n6;
        locals.var_egidl_dn8 = assign17640_e21658_d_n8;
        locals.var_egidl_dn10 = assign17640_e21658_d_n10;
        locals.var_egidl_dn11 = assign17640_e21658_d_n11;
        locals.var_egidl_dn12 = assign17640_e21658_d_n12;
        locals.var_egidl_rv = 0.0;

        let (assign17650_e21665, assign17650_e21665_d_n0, assign17650_e21665_d_n2, assign17650_e21665_d_n4, assign17650_e21665_d_n5, assign17650_e21665_d_n6, assign17650_e21665_d_n8, assign17650_e21665_d_n10, assign17650_e21665_d_n11, assign17650_e21665_d_n12,) = {
    if ((locals.var_guard317 == 0.0) && (locals.var_guard318 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign17650_e21665;
        locals.var_t5_dn0 = assign17650_e21665_d_n0;
        locals.var_t5_dn2 = assign17650_e21665_d_n2;
        locals.var_t5_dn4 = assign17650_e21665_d_n4;
        locals.var_t5_dn5 = assign17650_e21665_d_n5;
        locals.var_t5_dn6 = assign17650_e21665_d_n6;
        locals.var_t5_dn8 = assign17650_e21665_d_n8;
        locals.var_t5_dn10 = assign17650_e21665_d_n10;
        locals.var_t5_dn11 = assign17650_e21665_d_n11;
        locals.var_t5_dn12 = assign17650_e21665_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign17660_e21677, assign17660_e21677_d_n0, assign17660_e21677_d_n2, assign17660_e21677_d_n4, assign17660_e21677_d_n5, assign17660_e21677_d_n6, assign17660_e21677_d_n8, assign17660_e21677_d_n10, assign17660_e21677_d_n11, assign17660_e21677_d_n12,) = {
    if (locals.var_guard317 == 0.0) {
        let assign17660_e21669: f64 = (-locals.var_mks_gidl2);
        let assign17660_e21671: f64 = (assign17660_e21669 * locals.var_egp32);
        let assign17660_e21674: f64 = (locals.var_egidl + 1e-50);
        let assign17660_e21675: f64 = (assign17660_e21671 / assign17660_e21674);
        (assign17660_e21675, ((((assign17660_e21669 * locals.var_egp32_dn0) * assign17660_e21674) - (assign17660_e21671 * locals.var_egidl_dn0)) / (assign17660_e21674 * assign17660_e21674)), ((((assign17660_e21669 * locals.var_egp32_dn2) * assign17660_e21674) - (assign17660_e21671 * locals.var_egidl_dn2)) / (assign17660_e21674 * assign17660_e21674)), ((((assign17660_e21669 * locals.var_egp32_dn4) * assign17660_e21674) - (assign17660_e21671 * locals.var_egidl_dn4)) / (assign17660_e21674 * assign17660_e21674)), ((((assign17660_e21669 * locals.var_egp32_dn5) * assign17660_e21674) - (assign17660_e21671 * locals.var_egidl_dn5)) / (assign17660_e21674 * assign17660_e21674)), ((((assign17660_e21669 * locals.var_egp32_dn6) * assign17660_e21674) - (assign17660_e21671 * locals.var_egidl_dn6)) / (assign17660_e21674 * assign17660_e21674)), ((((assign17660_e21669 * locals.var_egp32_dn8) * assign17660_e21674) - (assign17660_e21671 * locals.var_egidl_dn8)) / (assign17660_e21674 * assign17660_e21674)), ((((assign17660_e21669 * locals.var_egp32_dn10) * assign17660_e21674) - (assign17660_e21671 * locals.var_egidl_dn10)) / (assign17660_e21674 * assign17660_e21674)), ((((assign17660_e21669 * locals.var_egp32_dn11) * assign17660_e21674) - (assign17660_e21671 * locals.var_egidl_dn11)) / (assign17660_e21674 * assign17660_e21674)), ((((assign17660_e21669 * locals.var_egp32_dn12) * assign17660_e21674) - (assign17660_e21671 * locals.var_egidl_dn12)) / (assign17660_e21674 * assign17660_e21674)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign17660_e21677;
        locals.var_t0_dn0 = assign17660_e21677_d_n0;
        locals.var_t0_dn2 = assign17660_e21677_d_n2;
        locals.var_t0_dn4 = assign17660_e21677_d_n4;
        locals.var_t0_dn5 = assign17660_e21677_d_n5;
        locals.var_t0_dn6 = assign17660_e21677_d_n6;
        locals.var_t0_dn8 = assign17660_e21677_d_n8;
        locals.var_t0_dn10 = assign17660_e21677_d_n10;
        locals.var_t0_dn11 = assign17660_e21677_d_n11;
        locals.var_t0_dn12 = assign17660_e21677_d_n12;
        locals.var_t0_rv = 0.0;

        let assign17670_e21680: f64 = (-34.0);
        let assign17670_e21681: f64 = if locals.var_t0 < assign17670_e21680 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign17670_e21681;
        locals.var_guard319_rv = 0.0;

        let (assign17690_e21702, assign17690_e21702_d_n0, assign17690_e21702_d_n2, assign17690_e21702_d_n4, assign17690_e21702_d_n5, assign17690_e21702_d_n6, assign17690_e21702_d_n8, assign17690_e21702_d_n10, assign17690_e21702_d_n11, assign17690_e21702_d_n12,) = {
    if ((locals.var_guard317 == 0.0) && (locals.var_guard319 == 0.0)) {
        let assign17690_e21696: f64 = (locals.var_uc_gidl1 / locals.var_egp12);
        let assign17690_e21698: f64 = (assign17690_e21696 * 1.6021918e-19);
        let assign17690_e21700: f64 = (assign17690_e21698 * locals.var_weff_nf);
        (assign17690_e21700, ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17690_e21698 * locals.var_weff_nf_dn0)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17690_e21698 * locals.var_weff_nf_dn2)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn4) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17690_e21698 * locals.var_weff_nf_dn4)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn5) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17690_e21698 * locals.var_weff_nf_dn5)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17690_e21698 * locals.var_weff_nf_dn6)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn8) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17690_e21698 * locals.var_weff_nf_dn8)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17690_e21698 * locals.var_weff_nf_dn10)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17690_e21698 * locals.var_weff_nf_dn11)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn12) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17690_e21698 * locals.var_weff_nf_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign17690_e21702;
        locals.var_t2_dn0 = assign17690_e21702_d_n0;
        locals.var_t2_dn2 = assign17690_e21702_d_n2;
        locals.var_t2_dn4 = assign17690_e21702_d_n4;
        locals.var_t2_dn5 = assign17690_e21702_d_n5;
        locals.var_t2_dn6 = assign17690_e21702_d_n6;
        locals.var_t2_dn8 = assign17690_e21702_d_n8;
        locals.var_t2_dn10 = assign17690_e21702_d_n10;
        locals.var_t2_dn11 = assign17690_e21702_d_n11;
        locals.var_t2_dn12 = assign17690_e21702_d_n12;
        locals.var_t2_rv = 0.0;

        let assign17730_e21752: f64 = if p.p18 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard320 = assign17730_e21752;
        locals.var_guard320_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17750_e21776, assign17750_e21776_d_n0, assign17750_e21776_d_n2, assign17750_e21776_d_n4, assign17750_e21776_d_n5, assign17750_e21776_d_n6, assign17750_e21776_d_n8, assign17750_e21776_d_n10, assign17750_e21776_d_n11, assign17750_e21776_d_n12,) = {
    if (locals.var_guard320 == 0.0) {
        let assign17750_e21761: f64 = (-locals.var_vds);
        let assign17750_e21763: f64 = (assign17750_e21761 + p.p199);
        let assign17750_e21764: f64 = (p.p198 * assign17750_e21763);
        let assign17750_e21767: f64 = (locals.var_vgs - locals.var_vds);
        let assign17750_e21768: f64 = (assign17750_e21764 - assign17750_e21767);
        let assign17750_e21771: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign17750_e21773: f64 = (assign17750_e21771 * p.p200);
        let assign17750_e21774: f64 = (assign17750_e21768 - assign17750_e21773);
        (assign17750_e21774, (((p.p198 * (-locals.var_vds_dn0)) - (-locals.var_vds_dn0)) - ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p200)), (((p.p198 * (-locals.var_vds_dn2)) - (-locals.var_vds_dn2)) - ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p200)), (((p.p198 * (-locals.var_vds_dn4)) - (-locals.var_vds_dn4)) - ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) * p.p200)), (((p.p198 * (-locals.var_vds_dn5)) - (locals.var_vgs_dn5 - locals.var_vds_dn5)) - ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) * p.p200)), (((p.p198 * (-locals.var_vds_dn6)) - (-locals.var_vds_dn6)) - ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p200)), (((p.p198 * (-locals.var_vds_dn8)) - (-locals.var_vds_dn8)) - ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) * p.p200)), (((p.p198 * (-locals.var_vds_dn10)) - (-locals.var_vds_dn10)) - ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p200)), (((p.p198 * (-locals.var_vds_dn11)) - (locals.var_vgs_dn11 - locals.var_vds_dn11)) - ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p200)), (((p.p198 * (-locals.var_vds_dn12)) - (locals.var_vgs_dn12 - locals.var_vds_dn12)) - ((locals.var_dvthsc_dn12 + locals.var_dvthlp_dn12) * p.p200)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign17750_e21776;
        locals.var_t1_dn0 = assign17750_e21776_d_n0;
        locals.var_t1_dn2 = assign17750_e21776_d_n2;
        locals.var_t1_dn4 = assign17750_e21776_d_n4;
        locals.var_t1_dn5 = assign17750_e21776_d_n5;
        locals.var_t1_dn6 = assign17750_e21776_d_n6;
        locals.var_t1_dn8 = assign17750_e21776_d_n8;
        locals.var_t1_dn10 = assign17750_e21776_d_n10;
        locals.var_t1_dn11 = assign17750_e21776_d_n11;
        locals.var_t1_dn12 = assign17750_e21776_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign17760_e21783, assign17760_e21783_d_n0, assign17760_e21783_d_n2, assign17760_e21783_d_n4, assign17760_e21783_d_n5, assign17760_e21783_d_n6, assign17760_e21783_d_n8, assign17760_e21783_d_n10, assign17760_e21783_d_n11, assign17760_e21783_d_n12,) = {
    if (locals.var_guard320 == 0.0) {
        let assign17760_e21781: f64 = (locals.var_t1 / p.p228);
        (assign17760_e21781, (locals.var_t1_dn0 / p.p228), (locals.var_t1_dn2 / p.p228), (locals.var_t1_dn4 / p.p228), (locals.var_t1_dn5 / p.p228), (locals.var_t1_dn6 / p.p228), (locals.var_t1_dn8 / p.p228), (locals.var_t1_dn10 / p.p228), (locals.var_t1_dn11 / p.p228), (locals.var_t1_dn12 / p.p228),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn8, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn12,)
    }
};
        locals.var_e1 = assign17760_e21783;
        locals.var_e1_dn0 = assign17760_e21783_d_n0;
        locals.var_e1_dn2 = assign17760_e21783_d_n2;
        locals.var_e1_dn4 = assign17760_e21783_d_n4;
        locals.var_e1_dn5 = assign17760_e21783_d_n5;
        locals.var_e1_dn6 = assign17760_e21783_d_n6;
        locals.var_e1_dn8 = assign17760_e21783_d_n8;
        locals.var_e1_dn10 = assign17760_e21783_d_n10;
        locals.var_e1_dn11 = assign17760_e21783_d_n11;
        locals.var_e1_dn12 = assign17760_e21783_d_n12;
        locals.var_e1_rv = 0.0;

        let (assign17770_e21797, assign17770_e21797_d_n0, assign17770_e21797_d_n2, assign17770_e21797_d_n4, assign17770_e21797_d_n5, assign17770_e21797_d_n6, assign17770_e21797_d_n8, assign17770_e21797_d_n10, assign17770_e21797_d_n11, assign17770_e21797_d_n12,) = {
    if (locals.var_guard320 == 0.0) {
        let assign17770_e21788: f64 = (locals.var_e1 * locals.var_e1);
        let assign17770_e21791: f64 = (4.0 * 0.01);
        let assign17770_e21793: f64 = (assign17770_e21791 * 0.01);
        let assign17770_e21794: f64 = (assign17770_e21788 + assign17770_e21793);
        let assign17770_e21795: f64 = (assign17770_e21794).sqrt();
        (assign17770_e21795, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign17770_e21795)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign17770_e21795)), (((locals.var_e1_dn4 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn4)) / (2.0 * assign17770_e21795)), (((locals.var_e1_dn5 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn5)) / (2.0 * assign17770_e21795)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign17770_e21795)), (((locals.var_e1_dn8 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn8)) / (2.0 * assign17770_e21795)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign17770_e21795)), (((locals.var_e1_dn11 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn11)) / (2.0 * assign17770_e21795)), (((locals.var_e1_dn12 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn12)) / (2.0 * assign17770_e21795)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17770_e21797;
        locals.var_tmf2_dn0 = assign17770_e21797_d_n0;
        locals.var_tmf2_dn2 = assign17770_e21797_d_n2;
        locals.var_tmf2_dn4 = assign17770_e21797_d_n4;
        locals.var_tmf2_dn5 = assign17770_e21797_d_n5;
        locals.var_tmf2_dn6 = assign17770_e21797_d_n6;
        locals.var_tmf2_dn8 = assign17770_e21797_d_n8;
        locals.var_tmf2_dn10 = assign17770_e21797_d_n10;
        locals.var_tmf2_dn11 = assign17770_e21797_d_n11;
        locals.var_tmf2_dn12 = assign17770_e21797_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17780_e21808, assign17780_e21808_d_n0, assign17780_e21808_d_n2, assign17780_e21808_d_n4, assign17780_e21808_d_n5, assign17780_e21808_d_n6, assign17780_e21808_d_n8, assign17780_e21808_d_n10, assign17780_e21808_d_n11, assign17780_e21808_d_n12,) = {
    if (locals.var_guard320 == 0.0) {
        let assign17780_e21804: f64 = (locals.var_e1 / locals.var_tmf2);
        let assign17780_e21805: f64 = (1.0 + assign17780_e21804);
        let assign17780_e21806: f64 = (0.5 * assign17780_e21805);
        (assign17780_e21806, (0.5 * (((locals.var_e1_dn0 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn2 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn4 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn5 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn6 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn8 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn10 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn11 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn12 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign17780_e21808;
        locals.var_t5_dn0 = assign17780_e21808_d_n0;
        locals.var_t5_dn2 = assign17780_e21808_d_n2;
        locals.var_t5_dn4 = assign17780_e21808_d_n4;
        locals.var_t5_dn5 = assign17780_e21808_d_n5;
        locals.var_t5_dn6 = assign17780_e21808_d_n6;
        locals.var_t5_dn8 = assign17780_e21808_d_n8;
        locals.var_t5_dn10 = assign17780_e21808_d_n10;
        locals.var_t5_dn11 = assign17780_e21808_d_n11;
        locals.var_t5_dn12 = assign17780_e21808_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign17790_e21821, assign17790_e21821_d_n0, assign17790_e21821_d_n2, assign17790_e21821_d_n4, assign17790_e21821_d_n5, assign17790_e21821_d_n6, assign17790_e21821_d_n8, assign17790_e21821_d_n10, assign17790_e21821_d_n11, assign17790_e21821_d_n12,) = {
    if (locals.var_guard320 == 0.0) {
        let assign17790_e21814: f64 = (locals.var_e1 + locals.var_tmf2);
        let assign17790_e21815: f64 = (0.5 * assign17790_e21814);
        let assign17790_e21818: f64 = (1e-10 * 0.01);
        let assign17790_e21819: f64 = (assign17790_e21815 + assign17790_e21818);
        (assign17790_e21819, (0.5 * (locals.var_e1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_e1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_e1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_e1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_e1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_e1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn4, locals.var_egisl_dn5, locals.var_egisl_dn6, locals.var_egisl_dn8, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn12,)
    }
};
        locals.var_egisl = assign17790_e21821;
        locals.var_egisl_dn0 = assign17790_e21821_d_n0;
        locals.var_egisl_dn2 = assign17790_e21821_d_n2;
        locals.var_egisl_dn4 = assign17790_e21821_d_n4;
        locals.var_egisl_dn5 = assign17790_e21821_d_n5;
        locals.var_egisl_dn6 = assign17790_e21821_d_n6;
        locals.var_egisl_dn8 = assign17790_e21821_d_n8;
        locals.var_egisl_dn10 = assign17790_e21821_d_n10;
        locals.var_egisl_dn11 = assign17790_e21821_d_n11;
        locals.var_egisl_dn12 = assign17790_e21821_d_n12;
        locals.var_egisl_rv = 0.0;

        let assign17800_e21824: f64 = if locals.var_egisl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard321 = assign17800_e21824;
        locals.var_guard321_rv = 0.0;

        let (assign17810_e21831, assign17810_e21831_d_n0, assign17810_e21831_d_n2, assign17810_e21831_d_n4, assign17810_e21831_d_n5, assign17810_e21831_d_n6, assign17810_e21831_d_n8, assign17810_e21831_d_n10, assign17810_e21831_d_n11, assign17810_e21831_d_n12,) = {
    if ((locals.var_guard320 == 0.0) && (locals.var_guard321 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn4, locals.var_egisl_dn5, locals.var_egisl_dn6, locals.var_egisl_dn8, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn12,)
    }
};
        locals.var_egisl = assign17810_e21831;
        locals.var_egisl_dn0 = assign17810_e21831_d_n0;
        locals.var_egisl_dn2 = assign17810_e21831_d_n2;
        locals.var_egisl_dn4 = assign17810_e21831_d_n4;
        locals.var_egisl_dn5 = assign17810_e21831_d_n5;
        locals.var_egisl_dn6 = assign17810_e21831_d_n6;
        locals.var_egisl_dn8 = assign17810_e21831_d_n8;
        locals.var_egisl_dn10 = assign17810_e21831_d_n10;
        locals.var_egisl_dn11 = assign17810_e21831_d_n11;
        locals.var_egisl_dn12 = assign17810_e21831_d_n12;
        locals.var_egisl_rv = 0.0;

        let (assign17820_e21838, assign17820_e21838_d_n0, assign17820_e21838_d_n2, assign17820_e21838_d_n4, assign17820_e21838_d_n5, assign17820_e21838_d_n6, assign17820_e21838_d_n8, assign17820_e21838_d_n10, assign17820_e21838_d_n11, assign17820_e21838_d_n12,) = {
    if ((locals.var_guard320 == 0.0) && (locals.var_guard321 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign17820_e21838;
        locals.var_t5_dn0 = assign17820_e21838_d_n0;
        locals.var_t5_dn2 = assign17820_e21838_d_n2;
        locals.var_t5_dn4 = assign17820_e21838_d_n4;
        locals.var_t5_dn5 = assign17820_e21838_d_n5;
        locals.var_t5_dn6 = assign17820_e21838_d_n6;
        locals.var_t5_dn8 = assign17820_e21838_d_n8;
        locals.var_t5_dn10 = assign17820_e21838_d_n10;
        locals.var_t5_dn11 = assign17820_e21838_d_n11;
        locals.var_t5_dn12 = assign17820_e21838_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign17830_e21850, assign17830_e21850_d_n0, assign17830_e21850_d_n2, assign17830_e21850_d_n4, assign17830_e21850_d_n5, assign17830_e21850_d_n6, assign17830_e21850_d_n8, assign17830_e21850_d_n10, assign17830_e21850_d_n11, assign17830_e21850_d_n12,) = {
    if (locals.var_guard320 == 0.0) {
        let assign17830_e21842: f64 = (-locals.var_mks_gidl2);
        let assign17830_e21844: f64 = (assign17830_e21842 * locals.var_egp32);
        let assign17830_e21847: f64 = (locals.var_egisl + 1e-50);
        let assign17830_e21848: f64 = (assign17830_e21844 / assign17830_e21847);
        (assign17830_e21848, ((((assign17830_e21842 * locals.var_egp32_dn0) * assign17830_e21847) - (assign17830_e21844 * locals.var_egisl_dn0)) / (assign17830_e21847 * assign17830_e21847)), ((((assign17830_e21842 * locals.var_egp32_dn2) * assign17830_e21847) - (assign17830_e21844 * locals.var_egisl_dn2)) / (assign17830_e21847 * assign17830_e21847)), ((((assign17830_e21842 * locals.var_egp32_dn4) * assign17830_e21847) - (assign17830_e21844 * locals.var_egisl_dn4)) / (assign17830_e21847 * assign17830_e21847)), ((((assign17830_e21842 * locals.var_egp32_dn5) * assign17830_e21847) - (assign17830_e21844 * locals.var_egisl_dn5)) / (assign17830_e21847 * assign17830_e21847)), ((((assign17830_e21842 * locals.var_egp32_dn6) * assign17830_e21847) - (assign17830_e21844 * locals.var_egisl_dn6)) / (assign17830_e21847 * assign17830_e21847)), ((((assign17830_e21842 * locals.var_egp32_dn8) * assign17830_e21847) - (assign17830_e21844 * locals.var_egisl_dn8)) / (assign17830_e21847 * assign17830_e21847)), ((((assign17830_e21842 * locals.var_egp32_dn10) * assign17830_e21847) - (assign17830_e21844 * locals.var_egisl_dn10)) / (assign17830_e21847 * assign17830_e21847)), ((((assign17830_e21842 * locals.var_egp32_dn11) * assign17830_e21847) - (assign17830_e21844 * locals.var_egisl_dn11)) / (assign17830_e21847 * assign17830_e21847)), ((((assign17830_e21842 * locals.var_egp32_dn12) * assign17830_e21847) - (assign17830_e21844 * locals.var_egisl_dn12)) / (assign17830_e21847 * assign17830_e21847)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign17830_e21850;
        locals.var_t0_dn0 = assign17830_e21850_d_n0;
        locals.var_t0_dn2 = assign17830_e21850_d_n2;
        locals.var_t0_dn4 = assign17830_e21850_d_n4;
        locals.var_t0_dn5 = assign17830_e21850_d_n5;
        locals.var_t0_dn6 = assign17830_e21850_d_n6;
        locals.var_t0_dn8 = assign17830_e21850_d_n8;
        locals.var_t0_dn10 = assign17830_e21850_d_n10;
        locals.var_t0_dn11 = assign17830_e21850_d_n11;
        locals.var_t0_dn12 = assign17830_e21850_d_n12;
        locals.var_t0_rv = 0.0;

        let assign17840_e21853: f64 = (-34.0);
        let assign17840_e21854: f64 = if locals.var_t0 < assign17840_e21853 { 1.0 } else { 0.0 };
        locals.var_guard322 = assign17840_e21854;
        locals.var_guard322_rv = 0.0;

        let (assign17860_e21875, assign17860_e21875_d_n0, assign17860_e21875_d_n2, assign17860_e21875_d_n4, assign17860_e21875_d_n5, assign17860_e21875_d_n6, assign17860_e21875_d_n8, assign17860_e21875_d_n10, assign17860_e21875_d_n11, assign17860_e21875_d_n12,) = {
    if ((locals.var_guard320 == 0.0) && (locals.var_guard322 == 0.0)) {
        let assign17860_e21869: f64 = (locals.var_uc_gidl1 / locals.var_egp12);
        let assign17860_e21871: f64 = (assign17860_e21869 * 1.6021918e-19);
        let assign17860_e21873: f64 = (assign17860_e21871 * locals.var_weff_nf);
        (assign17860_e21873, ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17860_e21871 * locals.var_weff_nf_dn0)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17860_e21871 * locals.var_weff_nf_dn2)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn4) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17860_e21871 * locals.var_weff_nf_dn4)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn5) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17860_e21871 * locals.var_weff_nf_dn5)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17860_e21871 * locals.var_weff_nf_dn6)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn8) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17860_e21871 * locals.var_weff_nf_dn8)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17860_e21871 * locals.var_weff_nf_dn10)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17860_e21871 * locals.var_weff_nf_dn11)), ((((-((locals.var_uc_gidl1 * locals.var_egp12_dn12) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf) + (assign17860_e21871 * locals.var_weff_nf_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign17860_e21875;
        locals.var_t2_dn0 = assign17860_e21875_d_n0;
        locals.var_t2_dn2 = assign17860_e21875_d_n2;
        locals.var_t2_dn4 = assign17860_e21875_d_n4;
        locals.var_t2_dn5 = assign17860_e21875_d_n5;
        locals.var_t2_dn6 = assign17860_e21875_d_n6;
        locals.var_t2_dn8 = assign17860_e21875_d_n8;
        locals.var_t2_dn10 = assign17860_e21875_d_n10;
        locals.var_t2_dn11 = assign17860_e21875_d_n11;
        locals.var_t2_dn12 = assign17860_e21875_d_n12;
        locals.var_t2_rv = 0.0;

        locals.var_aclm = p.p176;
        locals.var_aclm_rv = 0.0;

        locals.var_ec = 0.0;
        locals.var_ec_dn0 = 0.0;
        locals.var_ec_dn2 = 0.0;
        locals.var_ec_dn4 = 0.0;
        locals.var_ec_dn5 = 0.0;
        locals.var_ec_dn6 = 0.0;
        locals.var_ec_dn8 = 0.0;
        locals.var_ec_dn10 = 0.0;
        locals.var_ec_dn11 = 0.0;
        locals.var_ec_dn12 = 0.0;
        locals.var_ec_rv = 0.0;

        let assign17920_e21926: f64 = if locals.var_flg_noqi != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard323 = assign17920_e21926;
        locals.var_guard323_rv = 0.0;

        let (assign17930_e21932, assign17930_e21932_d_n0, assign17930_e21932_d_n2, assign17930_e21932_d_n4, assign17930_e21932_d_n5, assign17930_e21932_d_n6, assign17930_e21932_d_n8, assign17930_e21932_d_n10, assign17930_e21932_d_n11, assign17930_e21932_d_n12,) = {
    if (locals.var_guard323 != 0.0) {
        let assign17930_e21930: f64 = (locals.var_vds + locals.var_ps0);
        (assign17930_e21930, (locals.var_vds_dn0 + locals.var_ps0_dn0), (locals.var_vds_dn2 + locals.var_ps0_dn2), (locals.var_vds_dn4 + locals.var_ps0_dn4), (locals.var_vds_dn5 + locals.var_ps0_dn5), (locals.var_vds_dn6 + locals.var_ps0_dn6), (locals.var_vds_dn8 + locals.var_ps0_dn8), (locals.var_vds_dn10 + locals.var_ps0_dn10), (locals.var_vds_dn11 + locals.var_ps0_dn11), (locals.var_vds_dn12 + locals.var_ps0_dn12),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign17930_e21932;
        locals.var_t2_dn0 = assign17930_e21932_d_n0;
        locals.var_t2_dn2 = assign17930_e21932_d_n2;
        locals.var_t2_dn4 = assign17930_e21932_d_n4;
        locals.var_t2_dn5 = assign17930_e21932_d_n5;
        locals.var_t2_dn6 = assign17930_e21932_d_n6;
        locals.var_t2_dn8 = assign17930_e21932_d_n8;
        locals.var_t2_dn10 = assign17930_e21932_d_n10;
        locals.var_t2_dn11 = assign17930_e21932_d_n11;
        locals.var_t2_dn12 = assign17930_e21932_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign17940_e21944, assign17940_e21944_d_n0, assign17940_e21944_d_n2, assign17940_e21944_d_n4, assign17940_e21944_d_n5, assign17940_e21944_d_n6, assign17940_e21944_d_n8, assign17940_e21944_d_n10, assign17940_e21944_d_n11, assign17940_e21944_d_n12,) = {
    if (locals.var_guard323 != 0.0) {
        let assign17940_e21936: f64 = (locals.var_aclm * locals.var_t2);
        let assign17940_e21939: f64 = (1.0 - locals.var_aclm);
        let assign17940_e21941: f64 = (assign17940_e21939 * locals.var_psl);
        let assign17940_e21942: f64 = (assign17940_e21936 + assign17940_e21941);
        (assign17940_e21942, ((locals.var_aclm * locals.var_t2_dn0) + (assign17940_e21939 * locals.var_psl_dn0)), ((locals.var_aclm * locals.var_t2_dn2) + (assign17940_e21939 * locals.var_psl_dn2)), ((locals.var_aclm * locals.var_t2_dn4) + (assign17940_e21939 * locals.var_psl_dn4)), ((locals.var_aclm * locals.var_t2_dn5) + (assign17940_e21939 * locals.var_psl_dn5)), ((locals.var_aclm * locals.var_t2_dn6) + (assign17940_e21939 * locals.var_psl_dn6)), ((locals.var_aclm * locals.var_t2_dn8) + (assign17940_e21939 * locals.var_psl_dn8)), ((locals.var_aclm * locals.var_t2_dn10) + (assign17940_e21939 * locals.var_psl_dn10)), ((locals.var_aclm * locals.var_t2_dn11) + (assign17940_e21939 * locals.var_psl_dn11)), ((locals.var_aclm * locals.var_t2_dn12) + (assign17940_e21939 * locals.var_psl_dn12)),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn8, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12,)
    }
};
        locals.var_psdl = assign17940_e21944;
        locals.var_psdl_dn0 = assign17940_e21944_d_n0;
        locals.var_psdl_dn2 = assign17940_e21944_d_n2;
        locals.var_psdl_dn4 = assign17940_e21944_d_n4;
        locals.var_psdl_dn5 = assign17940_e21944_d_n5;
        locals.var_psdl_dn6 = assign17940_e21944_d_n6;
        locals.var_psdl_dn8 = assign17940_e21944_d_n8;
        locals.var_psdl_dn10 = assign17940_e21944_d_n10;
        locals.var_psdl_dn11 = assign17940_e21944_d_n11;
        locals.var_psdl_dn12 = assign17940_e21944_d_n12;
        locals.var_psdl_rv = 0.0;

        let assign17950_e21948: f64 = (locals.var_ps0 + locals.var_vds);
        let assign17950_e21951: f64 = (10.0 * 2.220446049250313e-16);
        let assign17950_e21952: f64 = (assign17950_e21948 - assign17950_e21951);
        let assign17950_e21953: f64 = if locals.var_psdl > assign17950_e21952 { 1.0 } else { 0.0 };
        locals.var_guard324 = assign17950_e21953;
        locals.var_guard324_rv = 0.0;

        let (assign17960_e21965, assign17960_e21965_d_n0, assign17960_e21965_d_n2, assign17960_e21965_d_n4, assign17960_e21965_d_n5, assign17960_e21965_d_n6, assign17960_e21965_d_n8, assign17960_e21965_d_n10, assign17960_e21965_d_n11, assign17960_e21965_d_n12,) = {
    if ((locals.var_guard323 != 0.0) && (locals.var_guard324 != 0.0)) {
        let assign17960_e21959: f64 = (locals.var_ps0 + locals.var_vds);
        let assign17960_e21962: f64 = (10.0 * 2.220446049250313e-16);
        let assign17960_e21963: f64 = (assign17960_e21959 - assign17960_e21962);
        (assign17960_e21963, (locals.var_ps0_dn0 + locals.var_vds_dn0), (locals.var_ps0_dn2 + locals.var_vds_dn2), (locals.var_ps0_dn4 + locals.var_vds_dn4), (locals.var_ps0_dn5 + locals.var_vds_dn5), (locals.var_ps0_dn6 + locals.var_vds_dn6), (locals.var_ps0_dn8 + locals.var_vds_dn8), (locals.var_ps0_dn10 + locals.var_vds_dn10), (locals.var_ps0_dn11 + locals.var_vds_dn11), (locals.var_ps0_dn12 + locals.var_vds_dn12),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn8, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12,)
    }
};
        locals.var_psdl = assign17960_e21965;
        locals.var_psdl_dn0 = assign17960_e21965_d_n0;
        locals.var_psdl_dn2 = assign17960_e21965_d_n2;
        locals.var_psdl_dn4 = assign17960_e21965_d_n4;
        locals.var_psdl_dn5 = assign17960_e21965_d_n5;
        locals.var_psdl_dn6 = assign17960_e21965_d_n6;
        locals.var_psdl_dn8 = assign17960_e21965_d_n8;
        locals.var_psdl_dn10 = assign17960_e21965_d_n10;
        locals.var_psdl_dn11 = assign17960_e21965_d_n11;
        locals.var_psdl_dn12 = assign17960_e21965_d_n12;
        locals.var_psdl_rv = 0.0;

        let assign17970_e21968: f64 = if p.p45 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard325 = assign17970_e21968;
        locals.var_guard325_rv = 0.0;

        let assign17980_e21971: f64 = if locals.var_idd > 1e-15 { 1.0 } else { 0.0 };
        locals.var_guard326 = assign17980_e21971;
        locals.var_guard326_rv = 0.0;

        let (assign17990_e21986, assign17990_e21986_d_n0, assign17990_e21986_d_n2, assign17990_e21986_d_n4, assign17990_e21986_d_n5, assign17990_e21986_d_n6, assign17990_e21986_d_n8, assign17990_e21986_d_n10, assign17990_e21986_d_n11, assign17990_e21986_d_n12,) = {
    if (((locals.var_guard323 == 0.0) && (locals.var_guard325 != 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign17990_e21980: f64 = (locals.var_idd * locals.var_beta_inv);
        let assign17990_e21982: f64 = (assign17990_e21980 / locals.var_leff);
        let assign17990_e21984: f64 = (assign17990_e21982 / locals.var_qn0);
        (assign17990_e21984, (((((((locals.var_idd_dn0 * locals.var_beta_inv) * locals.var_leff) - (assign17990_e21980 * locals.var_leff_dn0)) / (locals.var_leff * locals.var_leff)) * locals.var_qn0) - (assign17990_e21982 * locals.var_qn0_dn0)) / (locals.var_qn0 * locals.var_qn0)), (((((((locals.var_idd_dn2 * locals.var_beta_inv) * locals.var_leff) - (assign17990_e21980 * locals.var_leff_dn2)) / (locals.var_leff * locals.var_leff)) * locals.var_qn0) - (assign17990_e21982 * locals.var_qn0_dn2)) / (locals.var_qn0 * locals.var_qn0)), ((((((((locals.var_idd_dn4 * locals.var_beta_inv) + (locals.var_idd * locals.var_beta_inv_dn4)) * locals.var_leff) - (assign17990_e21980 * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff)) * locals.var_qn0) - (assign17990_e21982 * locals.var_qn0_dn4)) / (locals.var_qn0 * locals.var_qn0)), (((((((locals.var_idd_dn5 * locals.var_beta_inv) * locals.var_leff) - (assign17990_e21980 * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff)) * locals.var_qn0) - (assign17990_e21982 * locals.var_qn0_dn5)) / (locals.var_qn0 * locals.var_qn0)), (((((((locals.var_idd_dn6 * locals.var_beta_inv) * locals.var_leff) - (assign17990_e21980 * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff)) * locals.var_qn0) - (assign17990_e21982 * locals.var_qn0_dn6)) / (locals.var_qn0 * locals.var_qn0)), (((((((locals.var_idd_dn8 * locals.var_beta_inv) * locals.var_leff) - (assign17990_e21980 * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff)) * locals.var_qn0) - (assign17990_e21982 * locals.var_qn0_dn8)) / (locals.var_qn0 * locals.var_qn0)), (((((((locals.var_idd_dn10 * locals.var_beta_inv) * locals.var_leff) - (assign17990_e21980 * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff)) * locals.var_qn0) - (assign17990_e21982 * locals.var_qn0_dn10)) / (locals.var_qn0 * locals.var_qn0)), (((((((locals.var_idd_dn11 * locals.var_beta_inv) * locals.var_leff) - (assign17990_e21980 * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff)) * locals.var_qn0) - (assign17990_e21982 * locals.var_qn0_dn11)) / (locals.var_qn0 * locals.var_qn0)), (((((((locals.var_idd_dn12 * locals.var_beta_inv) * locals.var_leff) - (assign17990_e21980 * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff)) * locals.var_qn0) - (assign17990_e21982 * locals.var_qn0_dn12)) / (locals.var_qn0 * locals.var_qn0)),)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn8, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn12,)
    }
};
        locals.var_ec = assign17990_e21986;
        locals.var_ec_dn0 = assign17990_e21986_d_n0;
        locals.var_ec_dn2 = assign17990_e21986_d_n2;
        locals.var_ec_dn4 = assign17990_e21986_d_n4;
        locals.var_ec_dn5 = assign17990_e21986_d_n5;
        locals.var_ec_dn6 = assign17990_e21986_d_n6;
        locals.var_ec_dn8 = assign17990_e21986_d_n8;
        locals.var_ec_dn10 = assign17990_e21986_d_n10;
        locals.var_ec_dn11 = assign17990_e21986_d_n11;
        locals.var_ec_dn12 = assign17990_e21986_d_n12;
        locals.var_ec_rv = 0.0;

        locals.var_cox0 = locals.var_c_fox0;
        locals.var_cox0_rv = 0.0;

        let assign18010_e21990: f64 = (1.0 / locals.var_cox0);
        locals.var_cox0_inv = assign18010_e21990;
        locals.var_cox0_inv_rv = 0.0;

        let assign18020_e22001: f64 = if (((p.p19 >= 1.0) && (p.p175 > 0.0)) && (locals.var_mks_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard327 = assign18020_e22001;
        locals.var_guard327_rv = 0.0;

        let (assign18030_e22005,) = {
    if (locals.var_guard327 != 0.0) {
        (p.p175,)
    } else {
        (locals.var_lov,)
    }
};
        locals.var_lov = assign18030_e22005;
        locals.var_lov_rv = 0.0;

        let (assign18040_e22014, assign18040_e22014_d_n0, assign18040_e22014_d_n2, assign18040_e22014_d_n4, assign18040_e22014_d_n5, assign18040_e22014_d_n6, assign18040_e22014_d_n8, assign18040_e22014_d_n10, assign18040_e22014_d_n11, assign18040_e22014_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18040_e22010: f64 = (locals.var_mks_nover / locals.var_nsub);
        let assign18040_e22011: f64 = (assign18040_e22010).sqrt();
        let assign18040_e22012: f64 = (locals.var_cnst0soi * assign18040_e22011);
        (assign18040_e22012, ((locals.var_cnst0soi_dn0 * assign18040_e22011) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18040_e22011)))), ((locals.var_cnst0soi_dn2 * assign18040_e22011) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18040_e22011)))), ((locals.var_cnst0soi_dn4 * assign18040_e22011) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18040_e22011)))), ((locals.var_cnst0soi_dn5 * assign18040_e22011) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18040_e22011)))), ((locals.var_cnst0soi_dn6 * assign18040_e22011) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18040_e22011)))), ((locals.var_cnst0soi_dn8 * assign18040_e22011) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18040_e22011)))), ((locals.var_cnst0soi_dn10 * assign18040_e22011) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18040_e22011)))), ((locals.var_cnst0soi_dn11 * assign18040_e22011) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18040_e22011)))), ((locals.var_cnst0soi_dn12 * assign18040_e22011) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn12) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18040_e22011)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn8, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn12,)
    }
};
        locals.var_cnst0over = assign18040_e22014;
        locals.var_cnst0over_dn0 = assign18040_e22014_d_n0;
        locals.var_cnst0over_dn2 = assign18040_e22014_d_n2;
        locals.var_cnst0over_dn4 = assign18040_e22014_d_n4;
        locals.var_cnst0over_dn5 = assign18040_e22014_d_n5;
        locals.var_cnst0over_dn6 = assign18040_e22014_d_n6;
        locals.var_cnst0over_dn8 = assign18040_e22014_d_n8;
        locals.var_cnst0over_dn10 = assign18040_e22014_d_n10;
        locals.var_cnst0over_dn11 = assign18040_e22014_d_n11;
        locals.var_cnst0over_dn12 = assign18040_e22014_d_n12;
        locals.var_cnst0over_rv = 0.0;

        let (assign18050_e22022,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18050_e22018: f64 = (1.0 - -1.0);
        let assign18050_e22020: f64 = (assign18050_e22018 / 2.0);
        (assign18050_e22020,)
    } else {
        (locals.var_flg_ovloops,)
    }
};
        locals.var_flg_ovloops = assign18050_e22022;
        locals.var_flg_ovloops_rv = 0.0;

        let (assign18060_e22030,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18060_e22026: f64 = (1.0 + -1.0);
        let assign18060_e22028: f64 = (assign18060_e22026 / 2.0);
        (assign18060_e22028,)
    } else {
        (locals.var_flg_ovloopd,)
    }
};
        locals.var_flg_ovloopd = assign18060_e22030;
        locals.var_flg_ovloopd_rv = 0.0;

        let (assign18070_e22040,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18070_e22034: f64 = (locals.var_flg_ovloops * locals.var_modenml);
        let assign18070_e22037: f64 = (locals.var_flg_ovloopd * locals.var_modervs);
        let assign18070_e22038: f64 = (assign18070_e22034 + assign18070_e22037);
        (assign18070_e22038,)
    } else {
        (locals.var_flg_overs,)
    }
};
        locals.var_flg_overs = assign18070_e22040;
        locals.var_flg_overs_rv = 0.0;

        let (assign18080_e22050,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18080_e22044: f64 = (locals.var_flg_ovloops * locals.var_modervs);
        let assign18080_e22047: f64 = (locals.var_flg_ovloopd * locals.var_modenml);
        let assign18080_e22048: f64 = (assign18080_e22044 + assign18080_e22047);
        (assign18080_e22048,)
    } else {
        (locals.var_flg_overd,)
    }
};
        locals.var_flg_overd = assign18080_e22050;
        locals.var_flg_overd_rv = 0.0;

        let (assign18090_e22064, assign18090_e22064_d_n0, assign18090_e22064_d_n2, assign18090_e22064_d_n5,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_flg_ovloops != 0.0)) {
        let assign18090_e22056: f64 = (locals.var_modenml * locals.var_vgse);
        let assign18090_e22060: f64 = (locals.var_vgse - locals.var_vdse);
        let assign18090_e22061: f64 = (locals.var_modervs * assign18090_e22060);
        let assign18090_e22062: f64 = (assign18090_e22056 + assign18090_e22061);
        (assign18090_e22062, ((locals.var_modenml * locals.var_vgse_dn0) + (locals.var_modervs * (locals.var_vgse_dn0 - locals.var_vdse_dn0))), ((locals.var_modenml * locals.var_vgse_dn2) + (locals.var_modervs * (locals.var_vgse_dn2 - locals.var_vdse_dn2))), ((locals.var_modenml * locals.var_vgse_dn5) + (locals.var_modervs * locals.var_vgse_dn5)),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn5,)
    }
};
        locals.var_vgbgmt = assign18090_e22064;
        locals.var_vgbgmt_dn0 = assign18090_e22064_d_n0;
        locals.var_vgbgmt_dn2 = assign18090_e22064_d_n2;
        locals.var_vgbgmt_dn5 = assign18090_e22064_d_n5;
        locals.var_vgbgmt_rv = 0.0;

        let (assign18100_e22078, assign18100_e22078_d_n0, assign18100_e22078_d_n2, assign18100_e22078_d_n5,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_flg_ovloopd != 0.0)) {
        let assign18100_e22070: f64 = (locals.var_modervs * locals.var_vgse);
        let assign18100_e22074: f64 = (locals.var_vgse - locals.var_vdse);
        let assign18100_e22075: f64 = (locals.var_modenml * assign18100_e22074);
        let assign18100_e22076: f64 = (assign18100_e22070 + assign18100_e22075);
        (assign18100_e22076, ((locals.var_modervs * locals.var_vgse_dn0) + (locals.var_modenml * (locals.var_vgse_dn0 - locals.var_vdse_dn0))), ((locals.var_modervs * locals.var_vgse_dn2) + (locals.var_modenml * (locals.var_vgse_dn2 - locals.var_vdse_dn2))), ((locals.var_modervs * locals.var_vgse_dn5) + (locals.var_modenml * locals.var_vgse_dn5)),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn5,)
    }
};
        locals.var_vgbgmt = assign18100_e22078;
        locals.var_vgbgmt_dn0 = assign18100_e22078_d_n0;
        locals.var_vgbgmt_dn2 = assign18100_e22078_d_n2;
        locals.var_vgbgmt_dn5 = assign18100_e22078_d_n5;
        locals.var_vgbgmt_rv = 0.0;

        let (assign18110_e22082,) = {
    if (locals.var_guard327 != 0.0) {
        (0.0,)
    } else {
        (locals.var_vxbgmt,)
    }
};
        locals.var_vxbgmt = assign18110_e22082;
        locals.var_vxbgmt_rv = 0.0;

        let (assign18120_e22087, assign18120_e22087_d_n0, assign18120_e22087_d_n2, assign18120_e22087_d_n4, assign18120_e22087_d_n5, assign18120_e22087_d_n6, assign18120_e22087_d_n8, assign18120_e22087_d_n10, assign18120_e22087_d_n11, assign18120_e22087_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18120_e22085: f64 = (-locals.var_vxbgmt);
        (assign18120_e22085, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign18120_e22087;
        locals.var_t0_dn0 = assign18120_e22087_d_n0;
        locals.var_t0_dn2 = assign18120_e22087_d_n2;
        locals.var_t0_dn4 = assign18120_e22087_d_n4;
        locals.var_t0_dn5 = assign18120_e22087_d_n5;
        locals.var_t0_dn6 = assign18120_e22087_d_n6;
        locals.var_t0_dn8 = assign18120_e22087_d_n8;
        locals.var_t0_dn10 = assign18120_e22087_d_n10;
        locals.var_t0_dn11 = assign18120_e22087_d_n11;
        locals.var_t0_dn12 = assign18120_e22087_d_n12;
        locals.var_t0_rv = 0.0;

        let assign18130_e22090: f64 = if locals.var_t0 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard328 = assign18130_e22090;
        locals.var_guard328_rv = 0.0;

        let (assign18140_e22098, assign18140_e22098_d_n0, assign18140_e22098_d_n2, assign18140_e22098_d_n4, assign18140_e22098_d_n5, assign18140_e22098_d_n6, assign18140_e22098_d_n8, assign18140_e22098_d_n10, assign18140_e22098_d_n11, assign18140_e22098_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign18140_e22096: f64 = (locals.var_t0 - locals.var_vbs_bnd);
        (assign18140_e22096, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign18140_e22098;
        locals.var_t1_dn0 = assign18140_e22098_d_n0;
        locals.var_t1_dn2 = assign18140_e22098_d_n2;
        locals.var_t1_dn4 = assign18140_e22098_d_n4;
        locals.var_t1_dn5 = assign18140_e22098_d_n5;
        locals.var_t1_dn6 = assign18140_e22098_d_n6;
        locals.var_t1_dn8 = assign18140_e22098_d_n8;
        locals.var_t1_dn10 = assign18140_e22098_d_n10;
        locals.var_t1_dn11 = assign18140_e22098_d_n11;
        locals.var_t1_dn12 = assign18140_e22098_d_n12;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_72(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18150_e22106, assign18150_e22106_d_n0, assign18150_e22106_d_n2, assign18150_e22106_d_n4, assign18150_e22106_d_n5, assign18150_e22106_d_n6, assign18150_e22106_d_n8, assign18150_e22106_d_n10, assign18150_e22106_d_n11, assign18150_e22106_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign18150_e22104: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign18150_e22104, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign18150_e22106;
        locals.var_t2_dn0 = assign18150_e22106_d_n0;
        locals.var_t2_dn2 = assign18150_e22106_d_n2;
        locals.var_t2_dn4 = assign18150_e22106_d_n4;
        locals.var_t2_dn5 = assign18150_e22106_d_n5;
        locals.var_t2_dn6 = assign18150_e22106_d_n6;
        locals.var_t2_dn8 = assign18150_e22106_d_n8;
        locals.var_t2_dn10 = assign18150_e22106_d_n10;
        locals.var_t2_dn11 = assign18150_e22106_d_n11;
        locals.var_t2_dn12 = assign18150_e22106_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign18160_e22114, assign18160_e22114_d_n0, assign18160_e22114_d_n2, assign18160_e22114_d_n4, assign18160_e22114_d_n5, assign18160_e22114_d_n6, assign18160_e22114_d_n8, assign18160_e22114_d_n10, assign18160_e22114_d_n11, assign18160_e22114_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign18160_e22112: f64 = (locals.var_t1 / locals.var_t2);
        (assign18160_e22112, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn12 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn12)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign18160_e22114;
        locals.var_tmf1_dn0 = assign18160_e22114_d_n0;
        locals.var_tmf1_dn2 = assign18160_e22114_d_n2;
        locals.var_tmf1_dn4 = assign18160_e22114_d_n4;
        locals.var_tmf1_dn5 = assign18160_e22114_d_n5;
        locals.var_tmf1_dn6 = assign18160_e22114_d_n6;
        locals.var_tmf1_dn8 = assign18160_e22114_d_n8;
        locals.var_tmf1_dn10 = assign18160_e22114_d_n10;
        locals.var_tmf1_dn11 = assign18160_e22114_d_n11;
        locals.var_tmf1_dn12 = assign18160_e22114_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign18170_e22122, assign18170_e22122_d_n0, assign18170_e22122_d_n2, assign18170_e22122_d_n4, assign18170_e22122_d_n5, assign18170_e22122_d_n6, assign18170_e22122_d_n8, assign18170_e22122_d_n10, assign18170_e22122_d_n11, assign18170_e22122_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign18170_e22120: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign18170_e22120, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign18170_e22122;
        locals.var_tmf2_dn0 = assign18170_e22122_d_n0;
        locals.var_tmf2_dn2 = assign18170_e22122_d_n2;
        locals.var_tmf2_dn4 = assign18170_e22122_d_n4;
        locals.var_tmf2_dn5 = assign18170_e22122_d_n5;
        locals.var_tmf2_dn6 = assign18170_e22122_d_n6;
        locals.var_tmf2_dn8 = assign18170_e22122_d_n8;
        locals.var_tmf2_dn10 = assign18170_e22122_d_n10;
        locals.var_tmf2_dn11 = assign18170_e22122_d_n11;
        locals.var_tmf2_dn12 = assign18170_e22122_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign18180_e22130, assign18180_e22130_d_n0, assign18180_e22130_d_n2, assign18180_e22130_d_n4, assign18180_e22130_d_n5, assign18180_e22130_d_n6, assign18180_e22130_d_n8, assign18180_e22130_d_n10, assign18180_e22130_d_n11, assign18180_e22130_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign18180_e22128: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign18180_e22128, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn8, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12,)
    }
};
        locals.var_tmf3 = assign18180_e22130;
        locals.var_tmf3_dn0 = assign18180_e22130_d_n0;
        locals.var_tmf3_dn2 = assign18180_e22130_d_n2;
        locals.var_tmf3_dn4 = assign18180_e22130_d_n4;
        locals.var_tmf3_dn5 = assign18180_e22130_d_n5;
        locals.var_tmf3_dn6 = assign18180_e22130_d_n6;
        locals.var_tmf3_dn8 = assign18180_e22130_d_n8;
        locals.var_tmf3_dn10 = assign18180_e22130_d_n10;
        locals.var_tmf3_dn11 = assign18180_e22130_d_n11;
        locals.var_tmf3_dn12 = assign18180_e22130_d_n12;
        locals.var_tmf3_rv = 0.0;

        let (assign18190_e22138, assign18190_e22138_d_n0, assign18190_e22138_d_n2, assign18190_e22138_d_n4, assign18190_e22138_d_n5, assign18190_e22138_d_n6, assign18190_e22138_d_n8, assign18190_e22138_d_n10, assign18190_e22138_d_n11, assign18190_e22138_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign18190_e22136: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign18190_e22136, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn8, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12,)
    }
};
        locals.var_tmf4 = assign18190_e22138;
        locals.var_tmf4_dn0 = assign18190_e22138_d_n0;
        locals.var_tmf4_dn2 = assign18190_e22138_d_n2;
        locals.var_tmf4_dn4 = assign18190_e22138_d_n4;
        locals.var_tmf4_dn5 = assign18190_e22138_d_n5;
        locals.var_tmf4_dn6 = assign18190_e22138_d_n6;
        locals.var_tmf4_dn8 = assign18190_e22138_d_n8;
        locals.var_tmf4_dn10 = assign18190_e22138_d_n10;
        locals.var_tmf4_dn11 = assign18190_e22138_d_n11;
        locals.var_tmf4_dn12 = assign18190_e22138_d_n12;
        locals.var_tmf4_rv = 0.0;

        let (assign18200_e22154, assign18200_e22154_d_n0, assign18200_e22154_d_n2, assign18200_e22154_d_n4, assign18200_e22154_d_n5, assign18200_e22154_d_n6, assign18200_e22154_d_n8, assign18200_e22154_d_n10, assign18200_e22154_d_n11, assign18200_e22154_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign18200_e22145: f64 = (1.0 + locals.var_tmf1);
        let assign18200_e22147: f64 = (assign18200_e22145 + locals.var_tmf2);
        let assign18200_e22149: f64 = (assign18200_e22147 + locals.var_tmf3);
        let assign18200_e22151: f64 = (assign18200_e22149 + locals.var_tmf4);
        let assign18200_e22152: f64 = (1.0 / assign18200_e22151);
        (assign18200_e22152, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign18200_e22151 * assign18200_e22151))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign18200_e22151 * assign18200_e22151))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign18200_e22151 * assign18200_e22151))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign18200_e22151 * assign18200_e22151))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign18200_e22151 * assign18200_e22151))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign18200_e22151 * assign18200_e22151))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign18200_e22151 * assign18200_e22151))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign18200_e22151 * assign18200_e22151))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign18200_e22151 * assign18200_e22151))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12,)
    }
};
        locals.var_ty = assign18200_e22154;
        locals.var_ty_dn0 = assign18200_e22154_d_n0;
        locals.var_ty_dn2 = assign18200_e22154_d_n2;
        locals.var_ty_dn4 = assign18200_e22154_d_n4;
        locals.var_ty_dn5 = assign18200_e22154_d_n5;
        locals.var_ty_dn6 = assign18200_e22154_d_n6;
        locals.var_ty_dn8 = assign18200_e22154_d_n8;
        locals.var_ty_dn10 = assign18200_e22154_d_n10;
        locals.var_ty_dn11 = assign18200_e22154_d_n11;
        locals.var_ty_dn12 = assign18200_e22154_d_n12;
        locals.var_ty_rv = 0.0;

        let (assign18210_e22177, assign18210_e22177_d_n0, assign18210_e22177_d_n2, assign18210_e22177_d_n4, assign18210_e22177_d_n5, assign18210_e22177_d_n6, assign18210_e22177_d_n8, assign18210_e22177_d_n10, assign18210_e22177_d_n11, assign18210_e22177_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign18210_e22161: f64 = (2.0 * locals.var_tmf1);
        let assign18210_e22162: f64 = (1.0 + assign18210_e22161);
        let assign18210_e22165: f64 = (3.0 * locals.var_tmf2);
        let assign18210_e22166: f64 = (assign18210_e22162 + assign18210_e22165);
        let assign18210_e22169: f64 = (4.0 * locals.var_tmf3);
        let assign18210_e22170: f64 = (assign18210_e22166 + assign18210_e22169);
        let assign18210_e22171: f64 = (-assign18210_e22170);
        let assign18210_e22173: f64 = (assign18210_e22171 * locals.var_ty);
        let assign18210_e22175: f64 = (assign18210_e22173 * locals.var_ty);
        (assign18210_e22175, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_ty) + (assign18210_e22171 * locals.var_ty_dn0)) * locals.var_ty) + (assign18210_e22173 * locals.var_ty_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_ty) + (assign18210_e22171 * locals.var_ty_dn2)) * locals.var_ty) + (assign18210_e22173 * locals.var_ty_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_ty) + (assign18210_e22171 * locals.var_ty_dn4)) * locals.var_ty) + (assign18210_e22173 * locals.var_ty_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_ty) + (assign18210_e22171 * locals.var_ty_dn5)) * locals.var_ty) + (assign18210_e22173 * locals.var_ty_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_ty) + (assign18210_e22171 * locals.var_ty_dn6)) * locals.var_ty) + (assign18210_e22173 * locals.var_ty_dn6)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_ty) + (assign18210_e22171 * locals.var_ty_dn8)) * locals.var_ty) + (assign18210_e22173 * locals.var_ty_dn8)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_ty) + (assign18210_e22171 * locals.var_ty_dn10)) * locals.var_ty) + (assign18210_e22173 * locals.var_ty_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_ty) + (assign18210_e22171 * locals.var_ty_dn11)) * locals.var_ty) + (assign18210_e22173 * locals.var_ty_dn11)), (((((-(((2.0 * locals.var_tmf1_dn12) + (3.0 * locals.var_tmf2_dn12)) + (4.0 * locals.var_tmf3_dn12))) * locals.var_ty) + (assign18210_e22171 * locals.var_ty_dn12)) * locals.var_ty) + (assign18210_e22173 * locals.var_ty_dn12)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn8, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign18210_e22177;
        locals.var_t11_dn0 = assign18210_e22177_d_n0;
        locals.var_t11_dn2 = assign18210_e22177_d_n2;
        locals.var_t11_dn4 = assign18210_e22177_d_n4;
        locals.var_t11_dn5 = assign18210_e22177_d_n5;
        locals.var_t11_dn6 = assign18210_e22177_d_n6;
        locals.var_t11_dn8 = assign18210_e22177_d_n8;
        locals.var_t11_dn10 = assign18210_e22177_d_n10;
        locals.var_t11_dn11 = assign18210_e22177_d_n11;
        locals.var_t11_dn12 = assign18210_e22177_d_n12;
        locals.var_t11_rv = 0.0;

        let (assign18220_e22187, assign18220_e22187_d_n0, assign18220_e22187_d_n2, assign18220_e22187_d_n4, assign18220_e22187_d_n5, assign18220_e22187_d_n6, assign18220_e22187_d_n8, assign18220_e22187_d_n10, assign18220_e22187_d_n11, assign18220_e22187_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign18220_e22184: f64 = (1.0 - locals.var_ty);
        let assign18220_e22185: f64 = (locals.var_t2 * assign18220_e22184);
        (assign18220_e22185, ((locals.var_t2_dn0 * assign18220_e22184) + (locals.var_t2 * (-locals.var_ty_dn0))), ((locals.var_t2_dn2 * assign18220_e22184) + (locals.var_t2 * (-locals.var_ty_dn2))), ((locals.var_t2_dn4 * assign18220_e22184) + (locals.var_t2 * (-locals.var_ty_dn4))), ((locals.var_t2_dn5 * assign18220_e22184) + (locals.var_t2 * (-locals.var_ty_dn5))), ((locals.var_t2_dn6 * assign18220_e22184) + (locals.var_t2 * (-locals.var_ty_dn6))), ((locals.var_t2_dn8 * assign18220_e22184) + (locals.var_t2 * (-locals.var_ty_dn8))), ((locals.var_t2_dn10 * assign18220_e22184) + (locals.var_t2 * (-locals.var_ty_dn10))), ((locals.var_t2_dn11 * assign18220_e22184) + (locals.var_t2 * (-locals.var_ty_dn11))), ((locals.var_t2_dn12 * assign18220_e22184) + (locals.var_t2 * (-locals.var_ty_dn12))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12,)
    }
};
        locals.var_ty = assign18220_e22187;
        locals.var_ty_dn0 = assign18220_e22187_d_n0;
        locals.var_ty_dn2 = assign18220_e22187_d_n2;
        locals.var_ty_dn4 = assign18220_e22187_d_n4;
        locals.var_ty_dn5 = assign18220_e22187_d_n5;
        locals.var_ty_dn6 = assign18220_e22187_d_n6;
        locals.var_ty_dn8 = assign18220_e22187_d_n8;
        locals.var_ty_dn10 = assign18220_e22187_d_n10;
        locals.var_ty_dn11 = assign18220_e22187_d_n11;
        locals.var_ty_dn12 = assign18220_e22187_d_n12;
        locals.var_ty_rv = 0.0;

        let (assign18230_e22194, assign18230_e22194_d_n0, assign18230_e22194_d_n2, assign18230_e22194_d_n4, assign18230_e22194_d_n5, assign18230_e22194_d_n6, assign18230_e22194_d_n8, assign18230_e22194_d_n10, assign18230_e22194_d_n11, assign18230_e22194_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign18230_e22192: f64 = (-locals.var_t11);
        (assign18230_e22192, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn8), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn12),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn8, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign18230_e22194;
        locals.var_t11_dn0 = assign18230_e22194_d_n0;
        locals.var_t11_dn2 = assign18230_e22194_d_n2;
        locals.var_t11_dn4 = assign18230_e22194_d_n4;
        locals.var_t11_dn5 = assign18230_e22194_d_n5;
        locals.var_t11_dn6 = assign18230_e22194_d_n6;
        locals.var_t11_dn8 = assign18230_e22194_d_n8;
        locals.var_t11_dn10 = assign18230_e22194_d_n10;
        locals.var_t11_dn11 = assign18230_e22194_d_n11;
        locals.var_t11_dn12 = assign18230_e22194_d_n12;
        locals.var_t11_rv = 0.0;

        let (assign18240_e22202, assign18240_e22202_d_n0, assign18240_e22202_d_n2, assign18240_e22202_d_n4, assign18240_e22202_d_n5, assign18240_e22202_d_n6, assign18240_e22202_d_n8, assign18240_e22202_d_n10, assign18240_e22202_d_n11, assign18240_e22202_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign18240_e22200: f64 = (locals.var_vbs_bnd + locals.var_ty);
        (assign18240_e22200, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign18240_e22202;
        locals.var_t10_dn0 = assign18240_e22202_d_n0;
        locals.var_t10_dn2 = assign18240_e22202_d_n2;
        locals.var_t10_dn4 = assign18240_e22202_d_n4;
        locals.var_t10_dn5 = assign18240_e22202_d_n5;
        locals.var_t10_dn6 = assign18240_e22202_d_n6;
        locals.var_t10_dn8 = assign18240_e22202_d_n8;
        locals.var_t10_dn10 = assign18240_e22202_d_n10;
        locals.var_t10_dn11 = assign18240_e22202_d_n11;
        locals.var_t10_dn12 = assign18240_e22202_d_n12;
        locals.var_t10_rv = 0.0;

        let (assign18250_e22209, assign18250_e22209_d_n0, assign18250_e22209_d_n2, assign18250_e22209_d_n4, assign18250_e22209_d_n5, assign18250_e22209_d_n6, assign18250_e22209_d_n8, assign18250_e22209_d_n10, assign18250_e22209_d_n11, assign18250_e22209_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard328 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign18250_e22209;
        locals.var_t10_dn0 = assign18250_e22209_d_n0;
        locals.var_t10_dn2 = assign18250_e22209_d_n2;
        locals.var_t10_dn4 = assign18250_e22209_d_n4;
        locals.var_t10_dn5 = assign18250_e22209_d_n5;
        locals.var_t10_dn6 = assign18250_e22209_d_n6;
        locals.var_t10_dn8 = assign18250_e22209_d_n8;
        locals.var_t10_dn10 = assign18250_e22209_d_n10;
        locals.var_t10_dn11 = assign18250_e22209_d_n11;
        locals.var_t10_dn12 = assign18250_e22209_d_n12;
        locals.var_t10_rv = 0.0;

        let (assign18260_e22216, assign18260_e22216_d_n0, assign18260_e22216_d_n2, assign18260_e22216_d_n4, assign18260_e22216_d_n5, assign18260_e22216_d_n6, assign18260_e22216_d_n8, assign18260_e22216_d_n10, assign18260_e22216_d_n11, assign18260_e22216_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18260_e22212: f64 = (-locals.var_t10);
        let assign18260_e22214: f64 = (assign18260_e22212 - 1e-12);
        (assign18260_e22214, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn8), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn12),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12,)
    }
};
        locals.var_vxbgmtcl = assign18260_e22216;
        locals.var_vxbgmtcl_dn0 = assign18260_e22216_d_n0;
        locals.var_vxbgmtcl_dn2 = assign18260_e22216_d_n2;
        locals.var_vxbgmtcl_dn4 = assign18260_e22216_d_n4;
        locals.var_vxbgmtcl_dn5 = assign18260_e22216_d_n5;
        locals.var_vxbgmtcl_dn6 = assign18260_e22216_d_n6;
        locals.var_vxbgmtcl_dn8 = assign18260_e22216_d_n8;
        locals.var_vxbgmtcl_dn10 = assign18260_e22216_d_n10;
        locals.var_vxbgmtcl_dn11 = assign18260_e22216_d_n11;
        locals.var_vxbgmtcl_dn12 = assign18260_e22216_d_n12;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign18270_e22222, assign18270_e22222_d_n0, assign18270_e22222_d_n2, assign18270_e22222_d_n4, assign18270_e22222_d_n5, assign18270_e22222_d_n6, assign18270_e22222_d_n8, assign18270_e22222_d_n10, assign18270_e22222_d_n11, assign18270_e22222_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18270_e22220: f64 = (locals.var_cnst0over * locals.var_cox0_inv);
        (assign18270_e22220, (locals.var_cnst0over_dn0 * locals.var_cox0_inv), (locals.var_cnst0over_dn2 * locals.var_cox0_inv), (locals.var_cnst0over_dn4 * locals.var_cox0_inv), (locals.var_cnst0over_dn5 * locals.var_cox0_inv), (locals.var_cnst0over_dn6 * locals.var_cox0_inv), (locals.var_cnst0over_dn8 * locals.var_cox0_inv), (locals.var_cnst0over_dn10 * locals.var_cox0_inv), (locals.var_cnst0over_dn11 * locals.var_cox0_inv), (locals.var_cnst0over_dn12 * locals.var_cox0_inv),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn8, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn12,)
    }
};
        locals.var_fac1 = assign18270_e22222;
        locals.var_fac1_dn0 = assign18270_e22222_d_n0;
        locals.var_fac1_dn2 = assign18270_e22222_d_n2;
        locals.var_fac1_dn4 = assign18270_e22222_d_n4;
        locals.var_fac1_dn5 = assign18270_e22222_d_n5;
        locals.var_fac1_dn6 = assign18270_e22222_d_n6;
        locals.var_fac1_dn8 = assign18270_e22222_d_n8;
        locals.var_fac1_dn10 = assign18270_e22222_d_n10;
        locals.var_fac1_dn11 = assign18270_e22222_d_n11;
        locals.var_fac1_dn12 = assign18270_e22222_d_n12;
        locals.var_fac1_rv = 0.0;

        let (assign18280_e22228, assign18280_e22228_d_n0, assign18280_e22228_d_n2, assign18280_e22228_d_n4, assign18280_e22228_d_n5, assign18280_e22228_d_n6, assign18280_e22228_d_n8, assign18280_e22228_d_n10, assign18280_e22228_d_n11, assign18280_e22228_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18280_e22226: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign18280_e22226, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn12 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn12)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn8, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn12,)
    }
};
        locals.var_fac1p2 = assign18280_e22228;
        locals.var_fac1p2_dn0 = assign18280_e22228_d_n0;
        locals.var_fac1p2_dn2 = assign18280_e22228_d_n2;
        locals.var_fac1p2_dn4 = assign18280_e22228_d_n4;
        locals.var_fac1p2_dn5 = assign18280_e22228_d_n5;
        locals.var_fac1p2_dn6 = assign18280_e22228_d_n6;
        locals.var_fac1p2_dn8 = assign18280_e22228_d_n8;
        locals.var_fac1p2_dn10 = assign18280_e22228_d_n10;
        locals.var_fac1p2_dn11 = assign18280_e22228_d_n11;
        locals.var_fac1p2_dn12 = assign18280_e22228_d_n12;
        locals.var_fac1p2_rv = 0.0;

        let (assign18290_e22235, assign18290_e22235_d_n0, assign18290_e22235_d_n2, assign18290_e22235_d_n5,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18290_e22231: f64 = (-locals.var_vgbgmt);
        let assign18290_e22233: f64 = (assign18290_e22231 + p.p39);
        (assign18290_e22233, (-locals.var_vgbgmt_dn0), (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn5),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn0, locals.var_vgpld_dn2, locals.var_vgpld_dn5,)
    }
};
        locals.var_vgpld = assign18290_e22235;
        locals.var_vgpld_dn0 = assign18290_e22235_d_n0;
        locals.var_vgpld_dn2 = assign18290_e22235_d_n2;
        locals.var_vgpld_dn5 = assign18290_e22235_d_n5;
        locals.var_vgpld_rv = 0.0;

        let (assign18300_e22246, assign18300_e22246_d_n0, assign18300_e22246_d_n2, assign18300_e22246_d_n4, assign18300_e22246_d_n5, assign18300_e22246_d_n6, assign18300_e22246_d_n8, assign18300_e22246_d_n10, assign18300_e22246_d_n11, assign18300_e22246_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18300_e22239: f64 = (2.0 / locals.var_beta);
        let assign18300_e22242: f64 = (locals.var_mks_nover / locals.var_nin);
        let assign18300_e22243: f64 = (assign18300_e22242).ln();
        let assign18300_e22244: f64 = (assign18300_e22239 * assign18300_e22243);
        (assign18300_e22244, (assign18300_e22239 * ((-((locals.var_mks_nover * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign18300_e22242)), (assign18300_e22239 * ((-((locals.var_mks_nover * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign18300_e22242)), (((-((2.0 * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign18300_e22243) + (assign18300_e22239 * ((-((locals.var_mks_nover * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign18300_e22242))), (assign18300_e22239 * ((-((locals.var_mks_nover * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign18300_e22242)), (assign18300_e22239 * ((-((locals.var_mks_nover * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign18300_e22242)), (assign18300_e22239 * ((-((locals.var_mks_nover * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign18300_e22242)), (assign18300_e22239 * ((-((locals.var_mks_nover * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign18300_e22242)), (assign18300_e22239 * ((-((locals.var_mks_nover * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign18300_e22242)), (assign18300_e22239 * ((-((locals.var_mks_nover * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))) / assign18300_e22242)),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn4, locals.var_pb2over_dn5, locals.var_pb2over_dn6, locals.var_pb2over_dn8, locals.var_pb2over_dn10, locals.var_pb2over_dn11, locals.var_pb2over_dn12,)
    }
};
        locals.var_pb2over = assign18300_e22246;
        locals.var_pb2over_dn0 = assign18300_e22246_d_n0;
        locals.var_pb2over_dn2 = assign18300_e22246_d_n2;
        locals.var_pb2over_dn4 = assign18300_e22246_d_n4;
        locals.var_pb2over_dn5 = assign18300_e22246_d_n5;
        locals.var_pb2over_dn6 = assign18300_e22246_d_n6;
        locals.var_pb2over_dn8 = assign18300_e22246_d_n8;
        locals.var_pb2over_dn10 = assign18300_e22246_d_n10;
        locals.var_pb2over_dn11 = assign18300_e22246_d_n11;
        locals.var_pb2over_dn12 = assign18300_e22246_d_n12;
        locals.var_pb2over_rv = 0.0;

        let (assign18310_e22251, assign18310_e22251_d_n0, assign18310_e22251_d_n2, assign18310_e22251_d_n4, assign18310_e22251_d_n5, assign18310_e22251_d_n6, assign18310_e22251_d_n8, assign18310_e22251_d_n10, assign18310_e22251_d_n11, assign18310_e22251_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign18310_e22249: f64 = (-locals.var_vxbgmtcl);
        (assign18310_e22249, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn12,)
    }
};
        locals.var_vgb_fb_ld = assign18310_e22251;
        locals.var_vgb_fb_ld_dn0 = assign18310_e22251_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign18310_e22251_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign18310_e22251_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign18310_e22251_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign18310_e22251_d_n6;
        locals.var_vgb_fb_ld_dn8 = assign18310_e22251_d_n8;
        locals.var_vgb_fb_ld_dn10 = assign18310_e22251_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign18310_e22251_d_n11;
        locals.var_vgb_fb_ld_dn12 = assign18310_e22251_d_n12;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign18320_e22254: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard329 = assign18320_e22254;
        locals.var_guard329_rv = 0.0;

        let (assign18340_e22271, assign18340_e22271_d_n0, assign18340_e22271_d_n2, assign18340_e22271_d_n4, assign18340_e22271_d_n5, assign18340_e22271_d_n6, assign18340_e22271_d_n8, assign18340_e22271_d_n10, assign18340_e22271_d_n11, assign18340_e22271_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18340_e22268: f64 = (locals.var_beta * locals.var_cnst0over);
        let assign18340_e22269: f64 = (locals.var_cox0 / assign18340_e22268);
        (assign18340_e22269, (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn0)) / (assign18340_e22268 * assign18340_e22268))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn2)) / (assign18340_e22268 * assign18340_e22268))), (-((locals.var_cox0 * ((locals.var_beta_dn4 * locals.var_cnst0over) + (locals.var_beta * locals.var_cnst0over_dn4))) / (assign18340_e22268 * assign18340_e22268))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn5)) / (assign18340_e22268 * assign18340_e22268))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn6)) / (assign18340_e22268 * assign18340_e22268))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn8)) / (assign18340_e22268 * assign18340_e22268))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn10)) / (assign18340_e22268 * assign18340_e22268))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn11)) / (assign18340_e22268 * assign18340_e22268))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn12)) / (assign18340_e22268 * assign18340_e22268))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12,)
    }
};
        locals.var_ty = assign18340_e22271;
        locals.var_ty_dn0 = assign18340_e22271_d_n0;
        locals.var_ty_dn2 = assign18340_e22271_d_n2;
        locals.var_ty_dn4 = assign18340_e22271_d_n4;
        locals.var_ty_dn5 = assign18340_e22271_d_n5;
        locals.var_ty_dn6 = assign18340_e22271_d_n6;
        locals.var_ty_dn8 = assign18340_e22271_d_n8;
        locals.var_ty_dn10 = assign18340_e22271_d_n10;
        locals.var_ty_dn11 = assign18340_e22271_d_n11;
        locals.var_ty_dn12 = assign18340_e22271_d_n12;
        locals.var_ty_rv = 0.0;

        let (assign18350_e22283, assign18350_e22283_d_n0, assign18350_e22283_d_n2, assign18350_e22283_d_n4, assign18350_e22283_d_n5, assign18350_e22283_d_n6, assign18350_e22283_d_n8, assign18350_e22283_d_n10, assign18350_e22283_d_n11, assign18350_e22283_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18350_e22278: f64 = (3.0 * 1.414213562373095);
        let assign18350_e22280: f64 = (assign18350_e22278 * locals.var_ty);
        let assign18350_e22281: f64 = (2.0 + assign18350_e22280);
        (assign18350_e22281, (assign18350_e22278 * locals.var_ty_dn0), (assign18350_e22278 * locals.var_ty_dn2), (assign18350_e22278 * locals.var_ty_dn4), (assign18350_e22278 * locals.var_ty_dn5), (assign18350_e22278 * locals.var_ty_dn6), (assign18350_e22278 * locals.var_ty_dn8), (assign18350_e22278 * locals.var_ty_dn10), (assign18350_e22278 * locals.var_ty_dn11), (assign18350_e22278 * locals.var_ty_dn12),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn8, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12,)
    }
};
        locals.var_ac41 = assign18350_e22283;
        locals.var_ac41_dn0 = assign18350_e22283_d_n0;
        locals.var_ac41_dn2 = assign18350_e22283_d_n2;
        locals.var_ac41_dn4 = assign18350_e22283_d_n4;
        locals.var_ac41_dn5 = assign18350_e22283_d_n5;
        locals.var_ac41_dn6 = assign18350_e22283_d_n6;
        locals.var_ac41_dn8 = assign18350_e22283_d_n8;
        locals.var_ac41_dn10 = assign18350_e22283_d_n10;
        locals.var_ac41_dn11 = assign18350_e22283_d_n11;
        locals.var_ac41_dn12 = assign18350_e22283_d_n12;
        locals.var_ac41_rv = 0.0;

        let (assign18360_e22295, assign18360_e22295_d_n0, assign18360_e22295_d_n2, assign18360_e22295_d_n4, assign18360_e22295_d_n5, assign18360_e22295_d_n6, assign18360_e22295_d_n8, assign18360_e22295_d_n10, assign18360_e22295_d_n11, assign18360_e22295_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18360_e22289: f64 = (8.0 * locals.var_ac41);
        let assign18360_e22291: f64 = (assign18360_e22289 * locals.var_ac41);
        let assign18360_e22293: f64 = (assign18360_e22291 * locals.var_ac41);
        (assign18360_e22293, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign18360_e22289 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign18360_e22291 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign18360_e22289 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign18360_e22291 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign18360_e22289 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign18360_e22291 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign18360_e22289 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign18360_e22291 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign18360_e22289 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign18360_e22291 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign18360_e22289 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign18360_e22291 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign18360_e22289 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign18360_e22291 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign18360_e22289 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign18360_e22291 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign18360_e22289 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign18360_e22291 * locals.var_ac41_dn12)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn8, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12,)
    }
};
        locals.var_ac4 = assign18360_e22295;
        locals.var_ac4_dn0 = assign18360_e22295_d_n0;
        locals.var_ac4_dn2 = assign18360_e22295_d_n2;
        locals.var_ac4_dn4 = assign18360_e22295_d_n4;
        locals.var_ac4_dn5 = assign18360_e22295_d_n5;
        locals.var_ac4_dn6 = assign18360_e22295_d_n6;
        locals.var_ac4_dn8 = assign18360_e22295_d_n8;
        locals.var_ac4_dn10 = assign18360_e22295_d_n10;
        locals.var_ac4_dn11 = assign18360_e22295_d_n11;
        locals.var_ac4_dn12 = assign18360_e22295_d_n12;
        locals.var_ac4_rv = 0.0;

        let (assign18370_e22303, assign18370_e22303_d_n0, assign18370_e22303_d_n2, assign18370_e22303_d_n4, assign18370_e22303_d_n5, assign18370_e22303_d_n6, assign18370_e22303_d_n8, assign18370_e22303_d_n10, assign18370_e22303_d_n11, assign18370_e22303_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18370_e22301: f64 = (locals.var_eg - locals.var_pb2over);
        (assign18370_e22301, (locals.var_eg_dn0 - locals.var_pb2over_dn0), (locals.var_eg_dn2 - locals.var_pb2over_dn2), (locals.var_eg_dn4 - locals.var_pb2over_dn4), (locals.var_eg_dn5 - locals.var_pb2over_dn5), (locals.var_eg_dn6 - locals.var_pb2over_dn6), (locals.var_eg_dn8 - locals.var_pb2over_dn8), (locals.var_eg_dn10 - locals.var_pb2over_dn10), (locals.var_eg_dn11 - locals.var_pb2over_dn11), (locals.var_eg_dn12 - locals.var_pb2over_dn12),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn8, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12,)
    }
};
        locals.var_ps0_min = assign18370_e22303;
        locals.var_ps0_min_dn0 = assign18370_e22303_d_n0;
        locals.var_ps0_min_dn2 = assign18370_e22303_d_n2;
        locals.var_ps0_min_dn4 = assign18370_e22303_d_n4;
        locals.var_ps0_min_dn5 = assign18370_e22303_d_n5;
        locals.var_ps0_min_dn6 = assign18370_e22303_d_n6;
        locals.var_ps0_min_dn8 = assign18370_e22303_d_n8;
        locals.var_ps0_min_dn10 = assign18370_e22303_d_n10;
        locals.var_ps0_min_dn11 = assign18370_e22303_d_n11;
        locals.var_ps0_min_dn12 = assign18370_e22303_d_n12;
        locals.var_ps0_min_rv = 0.0;

        let (assign18380_e22313, assign18380_e22313_d_n0, assign18380_e22313_d_n2, assign18380_e22313_d_n4, assign18380_e22313_d_n5, assign18380_e22313_d_n6, assign18380_e22313_d_n8, assign18380_e22313_d_n10, assign18380_e22313_d_n11, assign18380_e22313_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18380_e22310: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign18380_e22311: f64 = (locals.var_beta * assign18380_e22310);
        (assign18380_e22311, (locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign18380_e22310) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), (locals.var_beta * (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * locals.var_vxbgmtcl_dn6), (locals.var_beta * locals.var_vxbgmtcl_dn8), (locals.var_beta * locals.var_vxbgmtcl_dn10), (locals.var_beta * locals.var_vxbgmtcl_dn11), (locals.var_beta * locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign18380_e22313;
        locals.var_tx_dn0 = assign18380_e22313_d_n0;
        locals.var_tx_dn2 = assign18380_e22313_d_n2;
        locals.var_tx_dn4 = assign18380_e22313_d_n4;
        locals.var_tx_dn5 = assign18380_e22313_d_n5;
        locals.var_tx_dn6 = assign18380_e22313_d_n6;
        locals.var_tx_dn8 = assign18380_e22313_d_n8;
        locals.var_tx_dn10 = assign18380_e22313_d_n10;
        locals.var_tx_dn11 = assign18380_e22313_d_n11;
        locals.var_tx_dn12 = assign18380_e22313_d_n12;
        locals.var_tx_rv = 0.0;

        let (assign18390_e22329, assign18390_e22329_d_n0, assign18390_e22329_d_n2, assign18390_e22329_d_n4, assign18390_e22329_d_n5, assign18390_e22329_d_n6, assign18390_e22329_d_n8, assign18390_e22329_d_n10, assign18390_e22329_d_n11, assign18390_e22329_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18390_e22319: f64 = (7.0 * 1.414213562373095);
        let assign18390_e22322: f64 = (9.0 * locals.var_ty);
        let assign18390_e22325: f64 = (locals.var_tx - 2.0);
        let assign18390_e22326: f64 = (assign18390_e22322 * assign18390_e22325);
        let assign18390_e22327: f64 = (assign18390_e22319 - assign18390_e22326);
        (assign18390_e22327, (-(((9.0 * locals.var_ty_dn0) * assign18390_e22325) + (assign18390_e22322 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign18390_e22325) + (assign18390_e22322 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign18390_e22325) + (assign18390_e22322 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign18390_e22325) + (assign18390_e22322 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign18390_e22325) + (assign18390_e22322 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn8) * assign18390_e22325) + (assign18390_e22322 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn10) * assign18390_e22325) + (assign18390_e22322 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign18390_e22325) + (assign18390_e22322 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn12) * assign18390_e22325) + (assign18390_e22322 * locals.var_tx_dn12))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn8, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12,)
    }
};
        locals.var_ac31 = assign18390_e22329;
        locals.var_ac31_dn0 = assign18390_e22329_d_n0;
        locals.var_ac31_dn2 = assign18390_e22329_d_n2;
        locals.var_ac31_dn4 = assign18390_e22329_d_n4;
        locals.var_ac31_dn5 = assign18390_e22329_d_n5;
        locals.var_ac31_dn6 = assign18390_e22329_d_n6;
        locals.var_ac31_dn8 = assign18390_e22329_d_n8;
        locals.var_ac31_dn10 = assign18390_e22329_d_n10;
        locals.var_ac31_dn11 = assign18390_e22329_d_n11;
        locals.var_ac31_dn12 = assign18390_e22329_d_n12;
        locals.var_ac31_rv = 0.0;

        let (assign18400_e22337, assign18400_e22337_d_n0, assign18400_e22337_d_n2, assign18400_e22337_d_n4, assign18400_e22337_d_n5, assign18400_e22337_d_n6, assign18400_e22337_d_n8, assign18400_e22337_d_n10, assign18400_e22337_d_n11, assign18400_e22337_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18400_e22335: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign18400_e22335, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn8, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12,)
    }
};
        locals.var_ac3 = assign18400_e22337;
        locals.var_ac3_dn0 = assign18400_e22337_d_n0;
        locals.var_ac3_dn2 = assign18400_e22337_d_n2;
        locals.var_ac3_dn4 = assign18400_e22337_d_n4;
        locals.var_ac3_dn5 = assign18400_e22337_d_n5;
        locals.var_ac3_dn6 = assign18400_e22337_d_n6;
        locals.var_ac3_dn8 = assign18400_e22337_d_n8;
        locals.var_ac3_dn10 = assign18400_e22337_d_n10;
        locals.var_ac3_dn11 = assign18400_e22337_d_n11;
        locals.var_ac3_dn12 = assign18400_e22337_d_n12;
        locals.var_ac3_rv = 0.0;

        let assign18410_e22341: f64 = (locals.var_ac3 * 1e-8);
        let assign18410_e22342: f64 = if locals.var_ac4 < assign18410_e22341 { 1.0 } else { 0.0 };
        locals.var_guard330 = assign18410_e22342;
        locals.var_guard330_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_73(
        locals: &mut StampLocals,
    ) {
        let (assign18420_e22369, assign18420_e22369_d_n0, assign18420_e22369_d_n2, assign18420_e22369_d_n4, assign18420_e22369_d_n5, assign18420_e22369_d_n6, assign18420_e22369_d_n8, assign18420_e22369_d_n10, assign18420_e22369_d_n11, assign18420_e22369_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign18420_e22349: f64 = (-7.0);
        let assign18420_e22351: f64 = (assign18420_e22349 * 1.414213562373095);
        let assign18420_e22353: f64 = (assign18420_e22351 + locals.var_ac31);
        let assign18420_e22356: f64 = (0.5 * locals.var_ac4);
        let assign18420_e22358: f64 = (assign18420_e22356 / locals.var_ac31);
        let assign18420_e22359: f64 = (assign18420_e22353 + assign18420_e22358);
        let assign18420_e22362: f64 = (9.0 * locals.var_ty);
        let assign18420_e22365: f64 = (locals.var_tx - 2.0);
        let assign18420_e22366: f64 = (assign18420_e22362 * assign18420_e22365);
        let assign18420_e22367: f64 = (assign18420_e22359 + assign18420_e22366);
        (assign18420_e22367, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign18420_e22356 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn0) * assign18420_e22365) + (assign18420_e22362 * locals.var_tx_dn0))), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign18420_e22356 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn2) * assign18420_e22365) + (assign18420_e22362 * locals.var_tx_dn2))), ((locals.var_ac31_dn4 + ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign18420_e22356 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn4) * assign18420_e22365) + (assign18420_e22362 * locals.var_tx_dn4))), ((locals.var_ac31_dn5 + ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign18420_e22356 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn5) * assign18420_e22365) + (assign18420_e22362 * locals.var_tx_dn5))), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign18420_e22356 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn6) * assign18420_e22365) + (assign18420_e22362 * locals.var_tx_dn6))), ((locals.var_ac31_dn8 + ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign18420_e22356 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn8) * assign18420_e22365) + (assign18420_e22362 * locals.var_tx_dn8))), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign18420_e22356 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn10) * assign18420_e22365) + (assign18420_e22362 * locals.var_tx_dn10))), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign18420_e22356 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn11) * assign18420_e22365) + (assign18420_e22362 * locals.var_tx_dn11))), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign18420_e22356 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn12) * assign18420_e22365) + (assign18420_e22362 * locals.var_tx_dn12))),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn8, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12,)
    }
};
        locals.var_ac1 = assign18420_e22369;
        locals.var_ac1_dn0 = assign18420_e22369_d_n0;
        locals.var_ac1_dn2 = assign18420_e22369_d_n2;
        locals.var_ac1_dn4 = assign18420_e22369_d_n4;
        locals.var_ac1_dn5 = assign18420_e22369_d_n5;
        locals.var_ac1_dn6 = assign18420_e22369_d_n6;
        locals.var_ac1_dn8 = assign18420_e22369_d_n8;
        locals.var_ac1_dn10 = assign18420_e22369_d_n10;
        locals.var_ac1_dn11 = assign18420_e22369_d_n11;
        locals.var_ac1_dn12 = assign18420_e22369_d_n12;
        locals.var_ac1_rv = 0.0;

        let (assign18430_e22381, assign18430_e22381_d_n0, assign18430_e22381_d_n2, assign18430_e22381_d_n4, assign18430_e22381_d_n5, assign18430_e22381_d_n6, assign18430_e22381_d_n8, assign18430_e22381_d_n10, assign18430_e22381_d_n11, assign18430_e22381_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign18430_e22378: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign18430_e22379: f64 = (assign18430_e22378).sqrt();
        (assign18430_e22379, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign18430_e22379)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign18430_e22379)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign18430_e22379)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign18430_e22379)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign18430_e22379)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign18430_e22379)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign18430_e22379)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign18430_e22379)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign18430_e22379)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn8, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12,)
    }
};
        locals.var_ac2 = assign18430_e22381;
        locals.var_ac2_dn0 = assign18430_e22381_d_n0;
        locals.var_ac2_dn2 = assign18430_e22381_d_n2;
        locals.var_ac2_dn4 = assign18430_e22381_d_n4;
        locals.var_ac2_dn5 = assign18430_e22381_d_n5;
        locals.var_ac2_dn6 = assign18430_e22381_d_n6;
        locals.var_ac2_dn8 = assign18430_e22381_d_n8;
        locals.var_ac2_dn10 = assign18430_e22381_d_n10;
        locals.var_ac2_dn11 = assign18430_e22381_d_n11;
        locals.var_ac2_dn12 = assign18430_e22381_d_n12;
        locals.var_ac2_rv = 0.0;

        let (assign18440_e22403, assign18440_e22403_d_n0, assign18440_e22403_d_n2, assign18440_e22403_d_n4, assign18440_e22403_d_n5, assign18440_e22403_d_n6, assign18440_e22403_d_n8, assign18440_e22403_d_n10, assign18440_e22403_d_n11, assign18440_e22403_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign18440_e22389: f64 = (-7.0);
        let assign18440_e22391: f64 = (assign18440_e22389 * 1.414213562373095);
        let assign18440_e22393: f64 = (assign18440_e22391 + locals.var_ac2);
        let assign18440_e22396: f64 = (9.0 * locals.var_ty);
        let assign18440_e22399: f64 = (locals.var_tx - 2.0);
        let assign18440_e22400: f64 = (assign18440_e22396 * assign18440_e22399);
        let assign18440_e22401: f64 = (assign18440_e22393 + assign18440_e22400);
        (assign18440_e22401, (locals.var_ac2_dn0 + (((9.0 * locals.var_ty_dn0) * assign18440_e22399) + (assign18440_e22396 * locals.var_tx_dn0))), (locals.var_ac2_dn2 + (((9.0 * locals.var_ty_dn2) * assign18440_e22399) + (assign18440_e22396 * locals.var_tx_dn2))), (locals.var_ac2_dn4 + (((9.0 * locals.var_ty_dn4) * assign18440_e22399) + (assign18440_e22396 * locals.var_tx_dn4))), (locals.var_ac2_dn5 + (((9.0 * locals.var_ty_dn5) * assign18440_e22399) + (assign18440_e22396 * locals.var_tx_dn5))), (locals.var_ac2_dn6 + (((9.0 * locals.var_ty_dn6) * assign18440_e22399) + (assign18440_e22396 * locals.var_tx_dn6))), (locals.var_ac2_dn8 + (((9.0 * locals.var_ty_dn8) * assign18440_e22399) + (assign18440_e22396 * locals.var_tx_dn8))), (locals.var_ac2_dn10 + (((9.0 * locals.var_ty_dn10) * assign18440_e22399) + (assign18440_e22396 * locals.var_tx_dn10))), (locals.var_ac2_dn11 + (((9.0 * locals.var_ty_dn11) * assign18440_e22399) + (assign18440_e22396 * locals.var_tx_dn11))), (locals.var_ac2_dn12 + (((9.0 * locals.var_ty_dn12) * assign18440_e22399) + (assign18440_e22396 * locals.var_tx_dn12))),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn8, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12,)
    }
};
        locals.var_ac1 = assign18440_e22403;
        locals.var_ac1_dn0 = assign18440_e22403_d_n0;
        locals.var_ac1_dn2 = assign18440_e22403_d_n2;
        locals.var_ac1_dn4 = assign18440_e22403_d_n4;
        locals.var_ac1_dn5 = assign18440_e22403_d_n5;
        locals.var_ac1_dn6 = assign18440_e22403_d_n6;
        locals.var_ac1_dn8 = assign18440_e22403_d_n8;
        locals.var_ac1_dn10 = assign18440_e22403_d_n10;
        locals.var_ac1_dn11 = assign18440_e22403_d_n11;
        locals.var_ac1_dn12 = assign18440_e22403_d_n12;
        locals.var_ac1_rv = 0.0;

        let (assign18450_e22411, assign18450_e22411_d_n0, assign18450_e22411_d_n2, assign18450_e22411_d_n4, assign18450_e22411_d_n5, assign18450_e22411_d_n6, assign18450_e22411_d_n8, assign18450_e22411_d_n10, assign18450_e22411_d_n11, assign18450_e22411_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18450_e22409: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign18450_e22409, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign18450_e22409 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign18450_e22409 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign18450_e22409 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign18450_e22409 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign18450_e22409 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign18450_e22409 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign18450_e22409 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign18450_e22409 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign18450_e22409 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn8, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12,)
    }
};
        locals.var_acd = assign18450_e22411;
        locals.var_acd_dn0 = assign18450_e22411_d_n0;
        locals.var_acd_dn2 = assign18450_e22411_d_n2;
        locals.var_acd_dn4 = assign18450_e22411_d_n4;
        locals.var_acd_dn5 = assign18450_e22411_d_n5;
        locals.var_acd_dn6 = assign18450_e22411_d_n6;
        locals.var_acd_dn8 = assign18450_e22411_d_n8;
        locals.var_acd_dn10 = assign18450_e22411_d_n10;
        locals.var_acd_dn11 = assign18450_e22411_d_n11;
        locals.var_acd_dn12 = assign18450_e22411_d_n12;
        locals.var_acd_rv = 0.0;

        let (assign18460_e22434, assign18460_e22434_d_n0, assign18460_e22434_d_n2, assign18460_e22434_d_n4, assign18460_e22434_d_n5, assign18460_e22434_d_n6, assign18460_e22434_d_n8, assign18460_e22434_d_n10, assign18460_e22434_d_n11, assign18460_e22434_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18460_e22416: f64 = (-4.0);
        let assign18460_e22418: f64 = (assign18460_e22416 * 1.414213562373095);
        let assign18460_e22421: f64 = (12.0 * locals.var_ty);
        let assign18460_e22422: f64 = (assign18460_e22418 - assign18460_e22421);
        let assign18460_e22425: f64 = (2.0 * locals.var_acd);
        let assign18460_e22426: f64 = (assign18460_e22422 + assign18460_e22425);
        let assign18460_e22429: f64 = (1.414213562373095 * locals.var_acd);
        let assign18460_e22431: f64 = (assign18460_e22429 * locals.var_acd);
        let assign18460_e22432: f64 = (assign18460_e22426 + assign18460_e22431);
        (assign18460_e22432, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign18460_e22429 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign18460_e22429 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign18460_e22429 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign18460_e22429 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign18460_e22429 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign18460_e22429 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign18460_e22429 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign18460_e22429 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign18460_e22429 * locals.var_acd_dn12))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn8, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12,)
    }
};
        locals.var_acn = assign18460_e22434;
        locals.var_acn_dn0 = assign18460_e22434_d_n0;
        locals.var_acn_dn2 = assign18460_e22434_d_n2;
        locals.var_acn_dn4 = assign18460_e22434_d_n4;
        locals.var_acn_dn5 = assign18460_e22434_d_n5;
        locals.var_acn_dn6 = assign18460_e22434_d_n6;
        locals.var_acn_dn8 = assign18460_e22434_d_n8;
        locals.var_acn_dn10 = assign18460_e22434_d_n10;
        locals.var_acn_dn11 = assign18460_e22434_d_n11;
        locals.var_acn_dn12 = assign18460_e22434_d_n12;
        locals.var_acn_rv = 0.0;

        let (assign18470_e22442, assign18470_e22442_d_n0, assign18470_e22442_d_n2, assign18470_e22442_d_n4, assign18470_e22442_d_n5, assign18470_e22442_d_n6, assign18470_e22442_d_n8, assign18470_e22442_d_n10, assign18470_e22442_d_n11, assign18470_e22442_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18470_e22440: f64 = (locals.var_acn / locals.var_acd);
        (assign18470_e22440, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn12 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn12)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign18470_e22442;
        locals.var_chi_dn0 = assign18470_e22442_d_n0;
        locals.var_chi_dn2 = assign18470_e22442_d_n2;
        locals.var_chi_dn4 = assign18470_e22442_d_n4;
        locals.var_chi_dn5 = assign18470_e22442_d_n5;
        locals.var_chi_dn6 = assign18470_e22442_d_n6;
        locals.var_chi_dn8 = assign18470_e22442_d_n8;
        locals.var_chi_dn10 = assign18470_e22442_d_n10;
        locals.var_chi_dn11 = assign18470_e22442_d_n11;
        locals.var_chi_dn12 = assign18470_e22442_d_n12;
        locals.var_chi_rv = 0.0;

        let (assign18480_e22452, assign18480_e22452_d_n0, assign18480_e22452_d_n2, assign18480_e22452_d_n4, assign18480_e22452_d_n5, assign18480_e22452_d_n6, assign18480_e22452_d_n8, assign18480_e22452_d_n10, assign18480_e22452_d_n11, assign18480_e22452_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18480_e22448: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign18480_e22450: f64 = (assign18480_e22448 - locals.var_vxbgmtcl);
        (assign18480_e22450, ((locals.var_chi_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_dn5 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_dn8 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_dn10 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn4, locals.var_psa_dn5, locals.var_psa_dn6, locals.var_psa_dn8, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12,)
    }
};
        locals.var_psa = assign18480_e22452;
        locals.var_psa_dn0 = assign18480_e22452_d_n0;
        locals.var_psa_dn2 = assign18480_e22452_d_n2;
        locals.var_psa_dn4 = assign18480_e22452_d_n4;
        locals.var_psa_dn5 = assign18480_e22452_d_n5;
        locals.var_psa_dn6 = assign18480_e22452_d_n6;
        locals.var_psa_dn8 = assign18480_e22452_d_n8;
        locals.var_psa_dn10 = assign18480_e22452_d_n10;
        locals.var_psa_dn11 = assign18480_e22452_d_n11;
        locals.var_psa_dn12 = assign18480_e22452_d_n12;
        locals.var_psa_rv = 0.0;

        let (assign18490_e22460, assign18490_e22460_d_n0, assign18490_e22460_d_n2, assign18490_e22460_d_n4, assign18490_e22460_d_n5, assign18490_e22460_d_n6, assign18490_e22460_d_n8, assign18490_e22460_d_n10, assign18490_e22460_d_n11, assign18490_e22460_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18490_e22458: f64 = (locals.var_psa + locals.var_vxbgmtcl);
        (assign18490_e22458, (locals.var_psa_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_psa_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_psa_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_psa_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_psa_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_psa_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_psa_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_psa_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_psa_dn12 + locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign18490_e22460;
        locals.var_t1_dn0 = assign18490_e22460_d_n0;
        locals.var_t1_dn2 = assign18490_e22460_d_n2;
        locals.var_t1_dn4 = assign18490_e22460_d_n4;
        locals.var_t1_dn5 = assign18490_e22460_d_n5;
        locals.var_t1_dn6 = assign18490_e22460_d_n6;
        locals.var_t1_dn8 = assign18490_e22460_d_n8;
        locals.var_t1_dn10 = assign18490_e22460_d_n10;
        locals.var_t1_dn11 = assign18490_e22460_d_n11;
        locals.var_t1_dn12 = assign18490_e22460_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign18500_e22468, assign18500_e22468_d_n0, assign18500_e22468_d_n2, assign18500_e22468_d_n4, assign18500_e22468_d_n5, assign18500_e22468_d_n6, assign18500_e22468_d_n8, assign18500_e22468_d_n10, assign18500_e22468_d_n11, assign18500_e22468_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18500_e22466: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign18500_e22466, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign18500_e22468;
        locals.var_t2_dn0 = assign18500_e22468_d_n0;
        locals.var_t2_dn2 = assign18500_e22468_d_n2;
        locals.var_t2_dn4 = assign18500_e22468_d_n4;
        locals.var_t2_dn5 = assign18500_e22468_d_n5;
        locals.var_t2_dn6 = assign18500_e22468_d_n6;
        locals.var_t2_dn8 = assign18500_e22468_d_n8;
        locals.var_t2_dn10 = assign18500_e22468_d_n10;
        locals.var_t2_dn11 = assign18500_e22468_d_n11;
        locals.var_t2_dn12 = assign18500_e22468_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign18510_e22483, assign18510_e22483_d_n0, assign18510_e22483_d_n2, assign18510_e22483_d_n4, assign18510_e22483_d_n5, assign18510_e22483_d_n6, assign18510_e22483_d_n8, assign18510_e22483_d_n10, assign18510_e22483_d_n11, assign18510_e22483_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18510_e22476: f64 = (locals.var_t2 * locals.var_t2);
        let assign18510_e22477: f64 = (1.0 + assign18510_e22476);
        let assign18510_e22478: f64 = (assign18510_e22477).sqrt();
        let assign18510_e22479: f64 = (locals.var_t1 / assign18510_e22478);
        let assign18510_e22481: f64 = (assign18510_e22479 - locals.var_vxbgmtcl);
        (assign18510_e22481, ((((locals.var_t1_dn0 * assign18510_e22478) - (locals.var_t1 * (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign18510_e22478)))) / (assign18510_e22478 * assign18510_e22478)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * assign18510_e22478) - (locals.var_t1 * (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign18510_e22478)))) / (assign18510_e22478 * assign18510_e22478)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * assign18510_e22478) - (locals.var_t1 * (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign18510_e22478)))) / (assign18510_e22478 * assign18510_e22478)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * assign18510_e22478) - (locals.var_t1 * (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign18510_e22478)))) / (assign18510_e22478 * assign18510_e22478)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * assign18510_e22478) - (locals.var_t1 * (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign18510_e22478)))) / (assign18510_e22478 * assign18510_e22478)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn8 * assign18510_e22478) - (locals.var_t1 * (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign18510_e22478)))) / (assign18510_e22478 * assign18510_e22478)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn10 * assign18510_e22478) - (locals.var_t1 * (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign18510_e22478)))) / (assign18510_e22478 * assign18510_e22478)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * assign18510_e22478) - (locals.var_t1 * (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign18510_e22478)))) / (assign18510_e22478 * assign18510_e22478)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn12 * assign18510_e22478) - (locals.var_t1 * (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign18510_e22478)))) / (assign18510_e22478 * assign18510_e22478)) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn8, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12,)
    }
};
        locals.var_ps0ld = assign18510_e22483;
        locals.var_ps0ld_dn0 = assign18510_e22483_d_n0;
        locals.var_ps0ld_dn2 = assign18510_e22483_d_n2;
        locals.var_ps0ld_dn4 = assign18510_e22483_d_n4;
        locals.var_ps0ld_dn5 = assign18510_e22483_d_n5;
        locals.var_ps0ld_dn6 = assign18510_e22483_d_n6;
        locals.var_ps0ld_dn8 = assign18510_e22483_d_n8;
        locals.var_ps0ld_dn10 = assign18510_e22483_d_n10;
        locals.var_ps0ld_dn11 = assign18510_e22483_d_n11;
        locals.var_ps0ld_dn12 = assign18510_e22483_d_n12;
        locals.var_ps0ld_rv = 0.0;

        let (assign18520_e22493, assign18520_e22493_d_n0, assign18520_e22493_d_n2, assign18520_e22493_d_n4, assign18520_e22493_d_n5, assign18520_e22493_d_n6, assign18520_e22493_d_n8, assign18520_e22493_d_n10, assign18520_e22493_d_n11, assign18520_e22493_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign18520_e22490: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign18520_e22491: f64 = (locals.var_cox0 * assign18520_e22490);
        (assign18520_e22491, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (-locals.var_ps0ld_dn4)), (locals.var_cox0 * (locals.var_vgpld_dn5 - locals.var_ps0ld_dn5)), (locals.var_cox0 * (-locals.var_ps0ld_dn6)), (locals.var_cox0 * (-locals.var_ps0ld_dn8)), (locals.var_cox0 * (-locals.var_ps0ld_dn10)), (locals.var_cox0 * (-locals.var_ps0ld_dn11)), (locals.var_cox0 * (-locals.var_ps0ld_dn12)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12,)
    }
};
        locals.var_qsuld = assign18520_e22493;
        locals.var_qsuld_dn0 = assign18520_e22493_d_n0;
        locals.var_qsuld_dn2 = assign18520_e22493_d_n2;
        locals.var_qsuld_dn4 = assign18520_e22493_d_n4;
        locals.var_qsuld_dn5 = assign18520_e22493_d_n5;
        locals.var_qsuld_dn6 = assign18520_e22493_d_n6;
        locals.var_qsuld_dn8 = assign18520_e22493_d_n8;
        locals.var_qsuld_dn10 = assign18520_e22493_d_n10;
        locals.var_qsuld_dn11 = assign18520_e22493_d_n11;
        locals.var_qsuld_dn12 = assign18520_e22493_d_n12;
        locals.var_qsuld_rv = 0.0;

        let (assign18530_e22499, assign18530_e22499_d_n0, assign18530_e22499_d_n2, assign18530_e22499_d_n4, assign18530_e22499_d_n5, assign18530_e22499_d_n6, assign18530_e22499_d_n8, assign18530_e22499_d_n10, assign18530_e22499_d_n11, assign18530_e22499_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn8, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12,)
    }
};
        locals.var_qbuld = assign18530_e22499;
        locals.var_qbuld_dn0 = assign18530_e22499_d_n0;
        locals.var_qbuld_dn2 = assign18530_e22499_d_n2;
        locals.var_qbuld_dn4 = assign18530_e22499_d_n4;
        locals.var_qbuld_dn5 = assign18530_e22499_d_n5;
        locals.var_qbuld_dn6 = assign18530_e22499_d_n6;
        locals.var_qbuld_dn8 = assign18530_e22499_d_n8;
        locals.var_qbuld_dn10 = assign18530_e22499_d_n10;
        locals.var_qbuld_dn11 = assign18530_e22499_d_n11;
        locals.var_qbuld_dn12 = assign18530_e22499_d_n12;
        locals.var_qbuld_rv = 0.0;

        let (assign18550_e22513, assign18550_e22513_d_n0, assign18550_e22513_d_n2, assign18550_e22513_d_n4, assign18550_e22513_d_n5, assign18550_e22513_d_n6, assign18550_e22513_d_n8, assign18550_e22513_d_n10, assign18550_e22513_d_n11, assign18550_e22513_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign18550_e22513;
        locals.var_chi_dn0 = assign18550_e22513_d_n0;
        locals.var_chi_dn2 = assign18550_e22513_d_n2;
        locals.var_chi_dn4 = assign18550_e22513_d_n4;
        locals.var_chi_dn5 = assign18550_e22513_d_n5;
        locals.var_chi_dn6 = assign18550_e22513_d_n6;
        locals.var_chi_dn8 = assign18550_e22513_d_n8;
        locals.var_chi_dn10 = assign18550_e22513_d_n10;
        locals.var_chi_dn11 = assign18550_e22513_d_n11;
        locals.var_chi_dn12 = assign18550_e22513_d_n12;
        locals.var_chi_rv = 0.0;

        let (assign18560_e22524, assign18560_e22524_d_n0, assign18560_e22524_d_n2, assign18560_e22524_d_n4, assign18560_e22524_d_n5, assign18560_e22524_d_n6, assign18560_e22524_d_n8, assign18560_e22524_d_n10, assign18560_e22524_d_n11, assign18560_e22524_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign18560_e22520: f64 = (locals.var_chi / locals.var_beta);
        let assign18560_e22522: f64 = (assign18560_e22520 - locals.var_vxbgmtcl);
        (assign18560_e22522, ((locals.var_chi_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((((locals.var_chi_dn4 * locals.var_beta) - (locals.var_chi * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_dn5 / locals.var_beta) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_dn8 / locals.var_beta) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_dn10 / locals.var_beta) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12,)
    }
};
        locals.var_ps0_inia = assign18560_e22524;
        locals.var_ps0_inia_dn0 = assign18560_e22524_d_n0;
        locals.var_ps0_inia_dn2 = assign18560_e22524_d_n2;
        locals.var_ps0_inia_dn4 = assign18560_e22524_d_n4;
        locals.var_ps0_inia_dn5 = assign18560_e22524_d_n5;
        locals.var_ps0_inia_dn6 = assign18560_e22524_d_n6;
        locals.var_ps0_inia_dn8 = assign18560_e22524_d_n8;
        locals.var_ps0_inia_dn10 = assign18560_e22524_d_n10;
        locals.var_ps0_inia_dn11 = assign18560_e22524_d_n11;
        locals.var_ps0_inia_dn12 = assign18560_e22524_d_n12;
        locals.var_ps0_inia_rv = 0.0;

        let (assign18570_e22549, assign18570_e22549_d_n0, assign18570_e22549_d_n2, assign18570_e22549_d_n4, assign18570_e22549_d_n5, assign18570_e22549_d_n6, assign18570_e22549_d_n8, assign18570_e22549_d_n10, assign18570_e22549_d_n11, assign18570_e22549_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign18570_e22534: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign18570_e22535: f64 = (locals.var_beta * assign18570_e22534);
        let assign18570_e22537: f64 = (assign18570_e22535 - 1.0);
        let assign18570_e22539: f64 = (-locals.var_chi);
        let assign18570_e22540: f64 = (assign18570_e22539).exp();
        let assign18570_e22541: f64 = (assign18570_e22537 + assign18570_e22540);
        let assign18570_e22542: f64 = (4.0 * assign18570_e22541);
        let assign18570_e22545: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign18570_e22546: f64 = (assign18570_e22542 / assign18570_e22545);
        let assign18570_e22547: f64 = (1.0 + assign18570_e22546);
        (assign18570_e22547, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + (assign18570_e22540 * (-locals.var_chi_dn0)))) * assign18570_e22545) - (assign18570_e22542 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign18570_e22545 * assign18570_e22545)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + (assign18570_e22540 * (-locals.var_chi_dn2)))) * assign18570_e22545) - (assign18570_e22542 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign18570_e22545 * assign18570_e22545)), ((((4.0 * (((locals.var_beta_dn4 * assign18570_e22534) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + (assign18570_e22540 * (-locals.var_chi_dn4)))) * assign18570_e22545) - (assign18570_e22542 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign18570_e22545 * assign18570_e22545)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)) + (assign18570_e22540 * (-locals.var_chi_dn5)))) * assign18570_e22545) - (assign18570_e22542 * (locals.var_fac1p2_dn5 * locals.var_beta2))) / (assign18570_e22545 * assign18570_e22545)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn6) + (assign18570_e22540 * (-locals.var_chi_dn6)))) * assign18570_e22545) - (assign18570_e22542 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign18570_e22545 * assign18570_e22545)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn8) + (assign18570_e22540 * (-locals.var_chi_dn8)))) * assign18570_e22545) - (assign18570_e22542 * (locals.var_fac1p2_dn8 * locals.var_beta2))) / (assign18570_e22545 * assign18570_e22545)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn10) + (assign18570_e22540 * (-locals.var_chi_dn10)))) * assign18570_e22545) - (assign18570_e22542 * (locals.var_fac1p2_dn10 * locals.var_beta2))) / (assign18570_e22545 * assign18570_e22545)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn11) + (assign18570_e22540 * (-locals.var_chi_dn11)))) * assign18570_e22545) - (assign18570_e22542 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign18570_e22545 * assign18570_e22545)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn12) + (assign18570_e22540 * (-locals.var_chi_dn12)))) * assign18570_e22545) - (assign18570_e22542 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign18570_e22545 * assign18570_e22545)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign18570_e22549;
        locals.var_tx_dn0 = assign18570_e22549_d_n0;
        locals.var_tx_dn2 = assign18570_e22549_d_n2;
        locals.var_tx_dn4 = assign18570_e22549_d_n4;
        locals.var_tx_dn5 = assign18570_e22549_d_n5;
        locals.var_tx_dn6 = assign18570_e22549_d_n6;
        locals.var_tx_dn8 = assign18570_e22549_d_n8;
        locals.var_tx_dn10 = assign18570_e22549_d_n10;
        locals.var_tx_dn11 = assign18570_e22549_d_n11;
        locals.var_tx_dn12 = assign18570_e22549_d_n12;
        locals.var_tx_rv = 0.0;

        let assign18580_e22553: f64 = (10.0 * 2.220446049250313e-16);
        let assign18580_e22554: f64 = if locals.var_tx < assign18580_e22553 { 1.0 } else { 0.0 };
        locals.var_guard331 = assign18580_e22554;
        locals.var_guard331_rv = 0.0;

        let (assign18590_e22565, assign18590_e22565_d_n0, assign18590_e22565_d_n2, assign18590_e22565_d_n4, assign18590_e22565_d_n5, assign18590_e22565_d_n6, assign18590_e22565_d_n8, assign18590_e22565_d_n10, assign18590_e22565_d_n11, assign18590_e22565_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard331 != 0.0)) {
        let assign18590_e22563: f64 = (10.0 * 2.220446049250313e-16);
        (assign18590_e22563, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign18590_e22565;
        locals.var_tx_dn0 = assign18590_e22565_d_n0;
        locals.var_tx_dn2 = assign18590_e22565_d_n2;
        locals.var_tx_dn4 = assign18590_e22565_d_n4;
        locals.var_tx_dn5 = assign18590_e22565_d_n5;
        locals.var_tx_dn6 = assign18590_e22565_d_n6;
        locals.var_tx_dn8 = assign18590_e22565_d_n8;
        locals.var_tx_dn10 = assign18590_e22565_d_n10;
        locals.var_tx_dn11 = assign18590_e22565_d_n11;
        locals.var_tx_dn12 = assign18590_e22565_d_n12;
        locals.var_tx_rv = 0.0;

        let (assign18600_e22583, assign18600_e22583_d_n0, assign18600_e22583_d_n2, assign18600_e22583_d_n4, assign18600_e22583_d_n5, assign18600_e22583_d_n6, assign18600_e22583_d_n8, assign18600_e22583_d_n10, assign18600_e22583_d_n11, assign18600_e22583_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign18600_e22573: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign18600_e22575: f64 = (assign18600_e22573 / 2.0);
        let assign18600_e22578: f64 = (locals.var_tx).sqrt();
        let assign18600_e22579: f64 = (1.0 - assign18600_e22578);
        let assign18600_e22580: f64 = (assign18600_e22575 * assign18600_e22579);
        let assign18600_e22581: f64 = (locals.var_vgpld + assign18600_e22580);
        (assign18600_e22581, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) / 2.0) * assign18600_e22579) + (assign18600_e22575 * (-(locals.var_tx_dn0 / (2.0 * assign18600_e22578)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) / 2.0) * assign18600_e22579) + (assign18600_e22575 * (-(locals.var_tx_dn2 / (2.0 * assign18600_e22578)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign18600_e22579) + (assign18600_e22575 * (-(locals.var_tx_dn4 / (2.0 * assign18600_e22578))))), (locals.var_vgpld_dn5 + ((((locals.var_fac1p2_dn5 * locals.var_beta) / 2.0) * assign18600_e22579) + (assign18600_e22575 * (-(locals.var_tx_dn5 / (2.0 * assign18600_e22578)))))), ((((locals.var_fac1p2_dn6 * locals.var_beta) / 2.0) * assign18600_e22579) + (assign18600_e22575 * (-(locals.var_tx_dn6 / (2.0 * assign18600_e22578))))), ((((locals.var_fac1p2_dn8 * locals.var_beta) / 2.0) * assign18600_e22579) + (assign18600_e22575 * (-(locals.var_tx_dn8 / (2.0 * assign18600_e22578))))), ((((locals.var_fac1p2_dn10 * locals.var_beta) / 2.0) * assign18600_e22579) + (assign18600_e22575 * (-(locals.var_tx_dn10 / (2.0 * assign18600_e22578))))), ((((locals.var_fac1p2_dn11 * locals.var_beta) / 2.0) * assign18600_e22579) + (assign18600_e22575 * (-(locals.var_tx_dn11 / (2.0 * assign18600_e22578))))), ((((locals.var_fac1p2_dn12 * locals.var_beta) / 2.0) * assign18600_e22579) + (assign18600_e22575 * (-(locals.var_tx_dn12 / (2.0 * assign18600_e22578))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12,)
    }
};
        locals.var_ps0_inia = assign18600_e22583;
        locals.var_ps0_inia_dn0 = assign18600_e22583_d_n0;
        locals.var_ps0_inia_dn2 = assign18600_e22583_d_n2;
        locals.var_ps0_inia_dn4 = assign18600_e22583_d_n4;
        locals.var_ps0_inia_dn5 = assign18600_e22583_d_n5;
        locals.var_ps0_inia_dn6 = assign18600_e22583_d_n6;
        locals.var_ps0_inia_dn8 = assign18600_e22583_d_n8;
        locals.var_ps0_inia_dn10 = assign18600_e22583_d_n10;
        locals.var_ps0_inia_dn11 = assign18600_e22583_d_n11;
        locals.var_ps0_inia_dn12 = assign18600_e22583_d_n12;
        locals.var_ps0_inia_rv = 0.0;

        let (assign18610_e22594, assign18610_e22594_d_n0, assign18610_e22594_d_n2, assign18610_e22594_d_n4, assign18610_e22594_d_n5, assign18610_e22594_d_n6, assign18610_e22594_d_n8, assign18610_e22594_d_n10, assign18610_e22594_d_n11, assign18610_e22594_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign18610_e22591: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign18610_e22592: f64 = (locals.var_beta * assign18610_e22591);
        (assign18610_e22592, (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign18610_e22591) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 + locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign18610_e22594;
        locals.var_chi_dn0 = assign18610_e22594_d_n0;
        locals.var_chi_dn2 = assign18610_e22594_d_n2;
        locals.var_chi_dn4 = assign18610_e22594_d_n4;
        locals.var_chi_dn5 = assign18610_e22594_d_n5;
        locals.var_chi_dn6 = assign18610_e22594_d_n6;
        locals.var_chi_dn8 = assign18610_e22594_d_n8;
        locals.var_chi_dn10 = assign18610_e22594_d_n10;
        locals.var_chi_dn11 = assign18610_e22594_d_n11;
        locals.var_chi_dn12 = assign18610_e22594_d_n12;
        locals.var_chi_rv = 0.0;

        let (assign18620_e22619, assign18620_e22619_d_n0, assign18620_e22619_d_n2, assign18620_e22619_d_n4, assign18620_e22619_d_n5, assign18620_e22619_d_n6, assign18620_e22619_d_n8, assign18620_e22619_d_n10, assign18620_e22619_d_n11, assign18620_e22619_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign18620_e22604: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign18620_e22605: f64 = (locals.var_beta * assign18620_e22604);
        let assign18620_e22607: f64 = (assign18620_e22605 - 1.0);
        let assign18620_e22609: f64 = (-locals.var_chi);
        let assign18620_e22610: f64 = (assign18620_e22609).exp();
        let assign18620_e22611: f64 = (assign18620_e22607 + assign18620_e22610);
        let assign18620_e22612: f64 = (4.0 * assign18620_e22611);
        let assign18620_e22615: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign18620_e22616: f64 = (assign18620_e22612 / assign18620_e22615);
        let assign18620_e22617: f64 = (1.0 + assign18620_e22616);
        (assign18620_e22617, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + (assign18620_e22610 * (-locals.var_chi_dn0)))) * assign18620_e22615) - (assign18620_e22612 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign18620_e22615 * assign18620_e22615)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + (assign18620_e22610 * (-locals.var_chi_dn2)))) * assign18620_e22615) - (assign18620_e22612 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign18620_e22615 * assign18620_e22615)), ((((4.0 * (((locals.var_beta_dn4 * assign18620_e22604) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + (assign18620_e22610 * (-locals.var_chi_dn4)))) * assign18620_e22615) - (assign18620_e22612 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign18620_e22615 * assign18620_e22615)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)) + (assign18620_e22610 * (-locals.var_chi_dn5)))) * assign18620_e22615) - (assign18620_e22612 * (locals.var_fac1p2_dn5 * locals.var_beta2))) / (assign18620_e22615 * assign18620_e22615)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn6) + (assign18620_e22610 * (-locals.var_chi_dn6)))) * assign18620_e22615) - (assign18620_e22612 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign18620_e22615 * assign18620_e22615)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn8) + (assign18620_e22610 * (-locals.var_chi_dn8)))) * assign18620_e22615) - (assign18620_e22612 * (locals.var_fac1p2_dn8 * locals.var_beta2))) / (assign18620_e22615 * assign18620_e22615)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn10) + (assign18620_e22610 * (-locals.var_chi_dn10)))) * assign18620_e22615) - (assign18620_e22612 * (locals.var_fac1p2_dn10 * locals.var_beta2))) / (assign18620_e22615 * assign18620_e22615)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn11) + (assign18620_e22610 * (-locals.var_chi_dn11)))) * assign18620_e22615) - (assign18620_e22612 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign18620_e22615 * assign18620_e22615)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn12) + (assign18620_e22610 * (-locals.var_chi_dn12)))) * assign18620_e22615) - (assign18620_e22612 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign18620_e22615 * assign18620_e22615)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign18620_e22619;
        locals.var_tx_dn0 = assign18620_e22619_d_n0;
        locals.var_tx_dn2 = assign18620_e22619_d_n2;
        locals.var_tx_dn4 = assign18620_e22619_d_n4;
        locals.var_tx_dn5 = assign18620_e22619_d_n5;
        locals.var_tx_dn6 = assign18620_e22619_d_n6;
        locals.var_tx_dn8 = assign18620_e22619_d_n8;
        locals.var_tx_dn10 = assign18620_e22619_d_n10;
        locals.var_tx_dn11 = assign18620_e22619_d_n11;
        locals.var_tx_dn12 = assign18620_e22619_d_n12;
        locals.var_tx_rv = 0.0;

        let assign18630_e22623: f64 = (10.0 * 2.220446049250313e-16);
        let assign18630_e22624: f64 = if locals.var_tx < assign18630_e22623 { 1.0 } else { 0.0 };
        locals.var_guard332 = assign18630_e22624;
        locals.var_guard332_rv = 0.0;

        let (assign18640_e22635, assign18640_e22635_d_n0, assign18640_e22635_d_n2, assign18640_e22635_d_n4, assign18640_e22635_d_n5, assign18640_e22635_d_n6, assign18640_e22635_d_n8, assign18640_e22635_d_n10, assign18640_e22635_d_n11, assign18640_e22635_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard332 != 0.0)) {
        let assign18640_e22633: f64 = (10.0 * 2.220446049250313e-16);
        (assign18640_e22633, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign18640_e22635;
        locals.var_tx_dn0 = assign18640_e22635_d_n0;
        locals.var_tx_dn2 = assign18640_e22635_d_n2;
        locals.var_tx_dn4 = assign18640_e22635_d_n4;
        locals.var_tx_dn5 = assign18640_e22635_d_n5;
        locals.var_tx_dn6 = assign18640_e22635_d_n6;
        locals.var_tx_dn8 = assign18640_e22635_d_n8;
        locals.var_tx_dn10 = assign18640_e22635_d_n10;
        locals.var_tx_dn11 = assign18640_e22635_d_n11;
        locals.var_tx_dn12 = assign18640_e22635_d_n12;
        locals.var_tx_rv = 0.0;

        let (assign18650_e22653, assign18650_e22653_d_n0, assign18650_e22653_d_n2, assign18650_e22653_d_n4, assign18650_e22653_d_n5, assign18650_e22653_d_n6, assign18650_e22653_d_n8, assign18650_e22653_d_n10, assign18650_e22653_d_n11, assign18650_e22653_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign18650_e22643: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign18650_e22645: f64 = (assign18650_e22643 / 2.0);
        let assign18650_e22648: f64 = (locals.var_tx).sqrt();
        let assign18650_e22649: f64 = (1.0 - assign18650_e22648);
        let assign18650_e22650: f64 = (assign18650_e22645 * assign18650_e22649);
        let assign18650_e22651: f64 = (locals.var_vgpld + assign18650_e22650);
        (assign18650_e22651, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) / 2.0) * assign18650_e22649) + (assign18650_e22645 * (-(locals.var_tx_dn0 / (2.0 * assign18650_e22648)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) / 2.0) * assign18650_e22649) + (assign18650_e22645 * (-(locals.var_tx_dn2 / (2.0 * assign18650_e22648)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign18650_e22649) + (assign18650_e22645 * (-(locals.var_tx_dn4 / (2.0 * assign18650_e22648))))), (locals.var_vgpld_dn5 + ((((locals.var_fac1p2_dn5 * locals.var_beta) / 2.0) * assign18650_e22649) + (assign18650_e22645 * (-(locals.var_tx_dn5 / (2.0 * assign18650_e22648)))))), ((((locals.var_fac1p2_dn6 * locals.var_beta) / 2.0) * assign18650_e22649) + (assign18650_e22645 * (-(locals.var_tx_dn6 / (2.0 * assign18650_e22648))))), ((((locals.var_fac1p2_dn8 * locals.var_beta) / 2.0) * assign18650_e22649) + (assign18650_e22645 * (-(locals.var_tx_dn8 / (2.0 * assign18650_e22648))))), ((((locals.var_fac1p2_dn10 * locals.var_beta) / 2.0) * assign18650_e22649) + (assign18650_e22645 * (-(locals.var_tx_dn10 / (2.0 * assign18650_e22648))))), ((((locals.var_fac1p2_dn11 * locals.var_beta) / 2.0) * assign18650_e22649) + (assign18650_e22645 * (-(locals.var_tx_dn11 / (2.0 * assign18650_e22648))))), ((((locals.var_fac1p2_dn12 * locals.var_beta) / 2.0) * assign18650_e22649) + (assign18650_e22645 * (-(locals.var_tx_dn12 / (2.0 * assign18650_e22648))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12,)
    }
};
        locals.var_ps0_inia = assign18650_e22653;
        locals.var_ps0_inia_dn0 = assign18650_e22653_d_n0;
        locals.var_ps0_inia_dn2 = assign18650_e22653_d_n2;
        locals.var_ps0_inia_dn4 = assign18650_e22653_d_n4;
        locals.var_ps0_inia_dn5 = assign18650_e22653_d_n5;
        locals.var_ps0_inia_dn6 = assign18650_e22653_d_n6;
        locals.var_ps0_inia_dn8 = assign18650_e22653_d_n8;
        locals.var_ps0_inia_dn10 = assign18650_e22653_d_n10;
        locals.var_ps0_inia_dn11 = assign18650_e22653_d_n11;
        locals.var_ps0_inia_dn12 = assign18650_e22653_d_n12;
        locals.var_ps0_inia_rv = 0.0;

        let (assign18660_e22664, assign18660_e22664_d_n0, assign18660_e22664_d_n2, assign18660_e22664_d_n4, assign18660_e22664_d_n5, assign18660_e22664_d_n6, assign18660_e22664_d_n8, assign18660_e22664_d_n10, assign18660_e22664_d_n11, assign18660_e22664_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign18660_e22661: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign18660_e22662: f64 = (locals.var_beta * assign18660_e22661);
        (assign18660_e22662, (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign18660_e22661) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 + locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign18660_e22664;
        locals.var_chi_dn0 = assign18660_e22664_d_n0;
        locals.var_chi_dn2 = assign18660_e22664_d_n2;
        locals.var_chi_dn4 = assign18660_e22664_d_n4;
        locals.var_chi_dn5 = assign18660_e22664_d_n5;
        locals.var_chi_dn6 = assign18660_e22664_d_n6;
        locals.var_chi_dn8 = assign18660_e22664_d_n8;
        locals.var_chi_dn10 = assign18660_e22664_d_n10;
        locals.var_chi_dn11 = assign18660_e22664_d_n11;
        locals.var_chi_dn12 = assign18660_e22664_d_n12;
        locals.var_chi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_74(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign18670_e22667: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard333 = assign18670_e22667;
        locals.var_guard333_rv = 0.0;

        let (assign18690_e22702,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18690_e22686: f64 = (9.0 * 1.414213562373095);
        let assign18690_e22687: f64 = (1.0 / assign18690_e22686);
        let assign18690_e22691: f64 = (7.0 * 0.049787068367863944);
        let assign18690_e22692: f64 = (5.0 + assign18690_e22691);
        let assign18690_e22696: f64 = (2.0 + 0.049787068367863944);
        let assign18690_e22697: f64 = (assign18690_e22696).sqrt();
        let assign18690_e22698: f64 = (54.0 * assign18690_e22697);
        let assign18690_e22699: f64 = (assign18690_e22692 / assign18690_e22698);
        let assign18690_e22700: f64 = (assign18690_e22687 - assign18690_e22699);
        (assign18690_e22700,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign18690_e22702;
        locals.var_ta_rv = 0.0;

        let (assign18700_e22724,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18700_e22711: f64 = (1.0 + 0.049787068367863944);
        let assign18700_e22715: f64 = (2.0 + 0.049787068367863944);
        let assign18700_e22716: f64 = (assign18700_e22715).sqrt();
        let assign18700_e22717: f64 = (2.0 * assign18700_e22716);
        let assign18700_e22718: f64 = (assign18700_e22711 / assign18700_e22717);
        let assign18700_e22721: f64 = (1.414213562373095 / 3.0);
        let assign18700_e22722: f64 = (assign18700_e22718 - assign18700_e22721);
        (assign18700_e22722,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign18700_e22724;
        locals.var_tb_rv = 0.0;

        let (assign18710_e22741, assign18710_e22741_d_n0, assign18710_e22741_d_n2, assign18710_e22741_d_n4, assign18710_e22741_d_n5, assign18710_e22741_d_n6, assign18710_e22741_d_n8, assign18710_e22741_d_n10, assign18710_e22741_d_n11, assign18710_e22741_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18710_e22733: f64 = (1.0 / 1.414213562373095);
        let assign18710_e22737: f64 = (locals.var_beta * locals.var_fac1);
        let assign18710_e22738: f64 = (1.0 / assign18710_e22737);
        let assign18710_e22739: f64 = (assign18710_e22733 + assign18710_e22738);
        (assign18710_e22739, (-((locals.var_beta * locals.var_fac1_dn0) / (assign18710_e22737 * assign18710_e22737))), (-((locals.var_beta * locals.var_fac1_dn2) / (assign18710_e22737 * assign18710_e22737))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign18710_e22737 * assign18710_e22737))), (-((locals.var_beta * locals.var_fac1_dn5) / (assign18710_e22737 * assign18710_e22737))), (-((locals.var_beta * locals.var_fac1_dn6) / (assign18710_e22737 * assign18710_e22737))), (-((locals.var_beta * locals.var_fac1_dn8) / (assign18710_e22737 * assign18710_e22737))), (-((locals.var_beta * locals.var_fac1_dn10) / (assign18710_e22737 * assign18710_e22737))), (-((locals.var_beta * locals.var_fac1_dn11) / (assign18710_e22737 * assign18710_e22737))), (-((locals.var_beta * locals.var_fac1_dn12) / (assign18710_e22737 * assign18710_e22737))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn8, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn12,)
    }
};
        locals.var_tc = assign18710_e22741;
        locals.var_tc_dn0 = assign18710_e22741_d_n0;
        locals.var_tc_dn2 = assign18710_e22741_d_n2;
        locals.var_tc_dn4 = assign18710_e22741_d_n4;
        locals.var_tc_dn5 = assign18710_e22741_d_n5;
        locals.var_tc_dn6 = assign18710_e22741_d_n6;
        locals.var_tc_dn8 = assign18710_e22741_d_n8;
        locals.var_tc_dn10 = assign18710_e22741_d_n10;
        locals.var_tc_dn11 = assign18710_e22741_d_n11;
        locals.var_tc_dn12 = assign18710_e22741_d_n12;
        locals.var_tc_rv = 0.0;

        let (assign18720_e22755, assign18720_e22755_d_n0, assign18720_e22755_d_n2, assign18720_e22755_d_n4, assign18720_e22755_d_n5, assign18720_e22755_d_n6, assign18720_e22755_d_n8, assign18720_e22755_d_n10, assign18720_e22755_d_n11, assign18720_e22755_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18720_e22750: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign18720_e22751: f64 = (-assign18720_e22750);
        let assign18720_e22753: f64 = (assign18720_e22751 / locals.var_fac1);
        (assign18720_e22753, ((((-(locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) * locals.var_fac1) - (assign18720_e22751 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign18720_e22751 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign18720_e22751 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)) * locals.var_fac1) - (assign18720_e22751 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign18720_e22751 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn8) * locals.var_fac1) - (assign18720_e22751 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign18720_e22751 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign18720_e22751 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn12) * locals.var_fac1) - (assign18720_e22751 * locals.var_fac1_dn12)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn8, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn12,)
    }
};
        locals.var_td = assign18720_e22755;
        locals.var_td_dn0 = assign18720_e22755_d_n0;
        locals.var_td_dn2 = assign18720_e22755_d_n2;
        locals.var_td_dn4 = assign18720_e22755_d_n4;
        locals.var_td_dn5 = assign18720_e22755_d_n5;
        locals.var_td_dn6 = assign18720_e22755_d_n6;
        locals.var_td_dn8 = assign18720_e22755_d_n8;
        locals.var_td_dn10 = assign18720_e22755_d_n10;
        locals.var_td_dn11 = assign18720_e22755_d_n11;
        locals.var_td_dn12 = assign18720_e22755_d_n12;
        locals.var_td_rv = 0.0;

        let (assign18730_e22792, assign18730_e22792_d_n0, assign18730_e22792_d_n2, assign18730_e22792_d_n4, assign18730_e22792_d_n5, assign18730_e22792_d_n6, assign18730_e22792_d_n8, assign18730_e22792_d_n10, assign18730_e22792_d_n11, assign18730_e22792_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18730_e22764: f64 = (locals.var_tb * locals.var_tb);
        let assign18730_e22766: f64 = (assign18730_e22764 * locals.var_tb);
        let assign18730_e22769: f64 = (27.0 * locals.var_ta);
        let assign18730_e22771: f64 = (assign18730_e22769 * locals.var_ta);
        let assign18730_e22773: f64 = (assign18730_e22771 * locals.var_ta);
        let assign18730_e22774: f64 = (assign18730_e22766 / assign18730_e22773);
        let assign18730_e22777: f64 = (locals.var_tb * locals.var_tc);
        let assign18730_e22780: f64 = (6.0 * locals.var_ta);
        let assign18730_e22782: f64 = (assign18730_e22780 * locals.var_ta);
        let assign18730_e22783: f64 = (assign18730_e22777 / assign18730_e22782);
        let assign18730_e22784: f64 = (assign18730_e22774 - assign18730_e22783);
        let assign18730_e22788: f64 = (2.0 * locals.var_ta);
        let assign18730_e22789: f64 = (locals.var_td / assign18730_e22788);
        let assign18730_e22790: f64 = (assign18730_e22784 + assign18730_e22789);
        (assign18730_e22790, ((-((locals.var_tb * locals.var_tc_dn0) / assign18730_e22782)) + (locals.var_td_dn0 / assign18730_e22788)), ((-((locals.var_tb * locals.var_tc_dn2) / assign18730_e22782)) + (locals.var_td_dn2 / assign18730_e22788)), ((-((locals.var_tb * locals.var_tc_dn4) / assign18730_e22782)) + (locals.var_td_dn4 / assign18730_e22788)), ((-((locals.var_tb * locals.var_tc_dn5) / assign18730_e22782)) + (locals.var_td_dn5 / assign18730_e22788)), ((-((locals.var_tb * locals.var_tc_dn6) / assign18730_e22782)) + (locals.var_td_dn6 / assign18730_e22788)), ((-((locals.var_tb * locals.var_tc_dn8) / assign18730_e22782)) + (locals.var_td_dn8 / assign18730_e22788)), ((-((locals.var_tb * locals.var_tc_dn10) / assign18730_e22782)) + (locals.var_td_dn10 / assign18730_e22788)), ((-((locals.var_tb * locals.var_tc_dn11) / assign18730_e22782)) + (locals.var_td_dn11 / assign18730_e22788)), ((-((locals.var_tb * locals.var_tc_dn12) / assign18730_e22782)) + (locals.var_td_dn12 / assign18730_e22788)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn8, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn12,)
    }
};
        locals.var_tq = assign18730_e22792;
        locals.var_tq_dn0 = assign18730_e22792_d_n0;
        locals.var_tq_dn2 = assign18730_e22792_d_n2;
        locals.var_tq_dn4 = assign18730_e22792_d_n4;
        locals.var_tq_dn5 = assign18730_e22792_d_n5;
        locals.var_tq_dn6 = assign18730_e22792_d_n6;
        locals.var_tq_dn8 = assign18730_e22792_d_n8;
        locals.var_tq_dn10 = assign18730_e22792_d_n10;
        locals.var_tq_dn11 = assign18730_e22792_d_n11;
        locals.var_tq_dn12 = assign18730_e22792_d_n12;
        locals.var_tq_rv = 0.0;

        let (assign18740_e22815, assign18740_e22815_d_n0, assign18740_e22815_d_n2, assign18740_e22815_d_n4, assign18740_e22815_d_n5, assign18740_e22815_d_n6, assign18740_e22815_d_n8, assign18740_e22815_d_n10, assign18740_e22815_d_n11, assign18740_e22815_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18740_e22801: f64 = (3.0 * locals.var_ta);
        let assign18740_e22803: f64 = (assign18740_e22801 * locals.var_tc);
        let assign18740_e22806: f64 = (locals.var_tb * locals.var_tb);
        let assign18740_e22807: f64 = (assign18740_e22803 - assign18740_e22806);
        let assign18740_e22810: f64 = (9.0 * locals.var_ta);
        let assign18740_e22812: f64 = (assign18740_e22810 * locals.var_ta);
        let assign18740_e22813: f64 = (assign18740_e22807 / assign18740_e22812);
        (assign18740_e22813, ((assign18740_e22801 * locals.var_tc_dn0) / assign18740_e22812), ((assign18740_e22801 * locals.var_tc_dn2) / assign18740_e22812), ((assign18740_e22801 * locals.var_tc_dn4) / assign18740_e22812), ((assign18740_e22801 * locals.var_tc_dn5) / assign18740_e22812), ((assign18740_e22801 * locals.var_tc_dn6) / assign18740_e22812), ((assign18740_e22801 * locals.var_tc_dn8) / assign18740_e22812), ((assign18740_e22801 * locals.var_tc_dn10) / assign18740_e22812), ((assign18740_e22801 * locals.var_tc_dn11) / assign18740_e22812), ((assign18740_e22801 * locals.var_tc_dn12) / assign18740_e22812),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn8, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn12,)
    }
};
        locals.var_tp = assign18740_e22815;
        locals.var_tp_dn0 = assign18740_e22815_d_n0;
        locals.var_tp_dn2 = assign18740_e22815_d_n2;
        locals.var_tp_dn4 = assign18740_e22815_d_n4;
        locals.var_tp_dn5 = assign18740_e22815_d_n5;
        locals.var_tp_dn6 = assign18740_e22815_d_n6;
        locals.var_tp_dn8 = assign18740_e22815_d_n8;
        locals.var_tp_dn10 = assign18740_e22815_d_n10;
        locals.var_tp_dn11 = assign18740_e22815_d_n11;
        locals.var_tp_dn12 = assign18740_e22815_d_n12;
        locals.var_tp_rv = 0.0;

        let (assign18750_e22833, assign18750_e22833_d_n0, assign18750_e22833_d_n2, assign18750_e22833_d_n4, assign18750_e22833_d_n5, assign18750_e22833_d_n6, assign18750_e22833_d_n8, assign18750_e22833_d_n10, assign18750_e22833_d_n11, assign18750_e22833_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18750_e22824: f64 = (locals.var_tq * locals.var_tq);
        let assign18750_e22827: f64 = (locals.var_tp * locals.var_tp);
        let assign18750_e22829: f64 = (assign18750_e22827 * locals.var_tp);
        let assign18750_e22830: f64 = (assign18750_e22824 + assign18750_e22829);
        let assign18750_e22831: f64 = (assign18750_e22830).sqrt();
        (assign18750_e22831, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign18750_e22827 * locals.var_tp_dn0))) / (2.0 * assign18750_e22831)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign18750_e22827 * locals.var_tp_dn2))) / (2.0 * assign18750_e22831)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign18750_e22827 * locals.var_tp_dn4))) / (2.0 * assign18750_e22831)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign18750_e22827 * locals.var_tp_dn5))) / (2.0 * assign18750_e22831)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign18750_e22827 * locals.var_tp_dn6))) / (2.0 * assign18750_e22831)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign18750_e22827 * locals.var_tp_dn8))) / (2.0 * assign18750_e22831)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign18750_e22827 * locals.var_tp_dn10))) / (2.0 * assign18750_e22831)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign18750_e22827 * locals.var_tp_dn11))) / (2.0 * assign18750_e22831)), ((((locals.var_tq_dn12 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn12)) + ((((locals.var_tp_dn12 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn12)) * locals.var_tp) + (assign18750_e22827 * locals.var_tp_dn12))) / (2.0 * assign18750_e22831)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign18750_e22833;
        locals.var_t5_dn0 = assign18750_e22833_d_n0;
        locals.var_t5_dn2 = assign18750_e22833_d_n2;
        locals.var_t5_dn4 = assign18750_e22833_d_n4;
        locals.var_t5_dn5 = assign18750_e22833_d_n5;
        locals.var_t5_dn6 = assign18750_e22833_d_n6;
        locals.var_t5_dn8 = assign18750_e22833_d_n8;
        locals.var_t5_dn10 = assign18750_e22833_d_n10;
        locals.var_t5_dn11 = assign18750_e22833_d_n11;
        locals.var_t5_dn12 = assign18750_e22833_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign18760_e22847, assign18760_e22847_d_n0, assign18760_e22847_d_n2, assign18760_e22847_d_n4, assign18760_e22847_d_n5, assign18760_e22847_d_n6, assign18760_e22847_d_n8, assign18760_e22847_d_n10, assign18760_e22847_d_n11, assign18760_e22847_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18760_e22841: f64 = (-locals.var_tq);
        let assign18760_e22843: f64 = (assign18760_e22841 + locals.var_t5);
        let assign18760_e22845: f64 = (assign18760_e22843).powf(0.3333333333333333);
        (assign18760_e22845, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18760_e22843).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign18760_e22845 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign18760_e22843))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18760_e22843).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign18760_e22845 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign18760_e22843))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18760_e22843).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign18760_e22845 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign18760_e22843))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18760_e22843).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign18760_e22845 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign18760_e22843))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18760_e22843).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign18760_e22845 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign18760_e22843))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18760_e22843).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign18760_e22845 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign18760_e22843))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18760_e22843).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign18760_e22845 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign18760_e22843))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18760_e22843).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign18760_e22845 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign18760_e22843))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18760_e22843).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn12) + locals.var_t5_dn12))) } } else { (assign18760_e22845 * (0.3333333333333333 * (((-locals.var_tq_dn12) + locals.var_t5_dn12) / assign18760_e22843))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn8, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn12,)
    }
};
        locals.var_tu = assign18760_e22847;
        locals.var_tu_dn0 = assign18760_e22847_d_n0;
        locals.var_tu_dn2 = assign18760_e22847_d_n2;
        locals.var_tu_dn4 = assign18760_e22847_d_n4;
        locals.var_tu_dn5 = assign18760_e22847_d_n5;
        locals.var_tu_dn6 = assign18760_e22847_d_n6;
        locals.var_tu_dn8 = assign18760_e22847_d_n8;
        locals.var_tu_dn10 = assign18760_e22847_d_n10;
        locals.var_tu_dn11 = assign18760_e22847_d_n11;
        locals.var_tu_dn12 = assign18760_e22847_d_n12;
        locals.var_tu_rv = 0.0;

        let (assign18770_e22861, assign18770_e22861_d_n0, assign18770_e22861_d_n2, assign18770_e22861_d_n4, assign18770_e22861_d_n5, assign18770_e22861_d_n6, assign18770_e22861_d_n8, assign18770_e22861_d_n10, assign18770_e22861_d_n11, assign18770_e22861_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18770_e22856: f64 = (locals.var_tq + locals.var_t5);
        let assign18770_e22858: f64 = (assign18770_e22856).powf(0.3333333333333333);
        let assign18770_e22859: f64 = (-assign18770_e22858);
        (assign18770_e22859, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18770_e22856).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign18770_e22858 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign18770_e22856))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18770_e22856).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign18770_e22858 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign18770_e22856))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18770_e22856).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign18770_e22858 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign18770_e22856))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18770_e22856).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign18770_e22858 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign18770_e22856))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18770_e22856).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign18770_e22858 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign18770_e22856))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18770_e22856).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign18770_e22858 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign18770_e22856))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18770_e22856).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign18770_e22858 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign18770_e22856))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18770_e22856).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign18770_e22858 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign18770_e22856))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign18770_e22856).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn12 + locals.var_t5_dn12))) } } else { (assign18770_e22858 * (0.3333333333333333 * ((locals.var_tq_dn12 + locals.var_t5_dn12) / assign18770_e22856))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn8, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn12,)
    }
};
        locals.var_tv = assign18770_e22861;
        locals.var_tv_dn0 = assign18770_e22861_d_n0;
        locals.var_tv_dn2 = assign18770_e22861_d_n2;
        locals.var_tv_dn4 = assign18770_e22861_d_n4;
        locals.var_tv_dn5 = assign18770_e22861_d_n5;
        locals.var_tv_dn6 = assign18770_e22861_d_n6;
        locals.var_tv_dn8 = assign18770_e22861_d_n8;
        locals.var_tv_dn10 = assign18770_e22861_d_n10;
        locals.var_tv_dn11 = assign18770_e22861_d_n11;
        locals.var_tv_dn12 = assign18770_e22861_d_n12;
        locals.var_tv_rv = 0.0;

        let (assign18780_e22878, assign18780_e22878_d_n0, assign18780_e22878_d_n2, assign18780_e22878_d_n4, assign18780_e22878_d_n5, assign18780_e22878_d_n6, assign18780_e22878_d_n8, assign18780_e22878_d_n10, assign18780_e22878_d_n11, assign18780_e22878_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18780_e22870: f64 = (locals.var_tu + locals.var_tv);
        let assign18780_e22874: f64 = (3.0 * locals.var_ta);
        let assign18780_e22875: f64 = (locals.var_tb / assign18780_e22874);
        let assign18780_e22876: f64 = (assign18780_e22870 - assign18780_e22875);
        (assign18780_e22876, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn12 + locals.var_tv_dn12),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign18780_e22878;
        locals.var_tx_dn0 = assign18780_e22878_d_n0;
        locals.var_tx_dn2 = assign18780_e22878_d_n2;
        locals.var_tx_dn4 = assign18780_e22878_d_n4;
        locals.var_tx_dn5 = assign18780_e22878_d_n5;
        locals.var_tx_dn6 = assign18780_e22878_d_n6;
        locals.var_tx_dn8 = assign18780_e22878_d_n8;
        locals.var_tx_dn10 = assign18780_e22878_d_n10;
        locals.var_tx_dn11 = assign18780_e22878_d_n11;
        locals.var_tx_dn12 = assign18780_e22878_d_n12;
        locals.var_tx_rv = 0.0;

        let (assign18790_e22891, assign18790_e22891_d_n0, assign18790_e22891_d_n2, assign18790_e22891_d_n4, assign18790_e22891_d_n5, assign18790_e22891_d_n6, assign18790_e22891_d_n8, assign18790_e22891_d_n10, assign18790_e22891_d_n11, assign18790_e22891_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18790_e22887: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign18790_e22889: f64 = (assign18790_e22887 - locals.var_vxbgmtcl);
        (assign18790_e22889, ((locals.var_tx_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), (((locals.var_tx_dn4 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), ((locals.var_tx_dn5 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn5), ((locals.var_tx_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_tx_dn8 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn8), ((locals.var_tx_dn10 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12,)
    }
};
        locals.var_ps0_inia = assign18790_e22891;
        locals.var_ps0_inia_dn0 = assign18790_e22891_d_n0;
        locals.var_ps0_inia_dn2 = assign18790_e22891_d_n2;
        locals.var_ps0_inia_dn4 = assign18790_e22891_d_n4;
        locals.var_ps0_inia_dn5 = assign18790_e22891_d_n5;
        locals.var_ps0_inia_dn6 = assign18790_e22891_d_n6;
        locals.var_ps0_inia_dn8 = assign18790_e22891_d_n8;
        locals.var_ps0_inia_dn10 = assign18790_e22891_d_n10;
        locals.var_ps0_inia_dn11 = assign18790_e22891_d_n11;
        locals.var_ps0_inia_dn12 = assign18790_e22891_d_n12;
        locals.var_ps0_inia_rv = 0.0;

        let (assign18800_e22904, assign18800_e22904_d_n0, assign18800_e22904_d_n2, assign18800_e22904_d_n4, assign18800_e22904_d_n5, assign18800_e22904_d_n6, assign18800_e22904_d_n8, assign18800_e22904_d_n10, assign18800_e22904_d_n11, assign18800_e22904_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign18800_e22901: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign18800_e22902: f64 = (locals.var_beta * assign18800_e22901);
        (assign18800_e22902, (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign18800_e22901) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 + locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign18800_e22904;
        locals.var_chi_dn0 = assign18800_e22904_d_n0;
        locals.var_chi_dn2 = assign18800_e22904_d_n2;
        locals.var_chi_dn4 = assign18800_e22904_d_n4;
        locals.var_chi_dn5 = assign18800_e22904_d_n5;
        locals.var_chi_dn6 = assign18800_e22904_d_n6;
        locals.var_chi_dn8 = assign18800_e22904_d_n8;
        locals.var_chi_dn10 = assign18800_e22904_d_n10;
        locals.var_chi_dn11 = assign18800_e22904_d_n11;
        locals.var_chi_dn12 = assign18800_e22904_d_n12;
        locals.var_chi_rv = 0.0;

        let assign18810_e22907: f64 = if p.p30 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard334 = assign18810_e22907;
        locals.var_guard334_rv = 0.0;

        let (assign18830_e22931, assign18830_e22931_d_n0, assign18830_e22931_d_n2, assign18830_e22931_d_n4, assign18830_e22931_d_n5, assign18830_e22931_d_n6, assign18830_e22931_d_n8, assign18830_e22931_d_n10, assign18830_e22931_d_n11, assign18830_e22931_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18830_e22927: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign18830_e22929: f64 = (assign18830_e22927 + 0.1);
        (assign18830_e22929, (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5), locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn12,)
    }
};
        locals.var_vgpld_shift = assign18830_e22931;
        locals.var_vgpld_shift_dn0 = assign18830_e22931_d_n0;
        locals.var_vgpld_shift_dn2 = assign18830_e22931_d_n2;
        locals.var_vgpld_shift_dn4 = assign18830_e22931_d_n4;
        locals.var_vgpld_shift_dn5 = assign18830_e22931_d_n5;
        locals.var_vgpld_shift_dn6 = assign18830_e22931_d_n6;
        locals.var_vgpld_shift_dn8 = assign18830_e22931_d_n8;
        locals.var_vgpld_shift_dn10 = assign18830_e22931_d_n10;
        locals.var_vgpld_shift_dn11 = assign18830_e22931_d_n11;
        locals.var_vgpld_shift_dn12 = assign18830_e22931_d_n12;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign18840_e22946, assign18840_e22946_d_n0, assign18840_e22946_d_n2, assign18840_e22946_d_n4, assign18840_e22946_d_n5, assign18840_e22946_d_n6, assign18840_e22946_d_n8, assign18840_e22946_d_n10, assign18840_e22946_d_n11, assign18840_e22946_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18840_e22940: f64 = (-locals.var_vxbgmtcl);
        let assign18840_e22941: f64 = (locals.var_beta * assign18840_e22940);
        let assign18840_e22942: f64 = (assign18840_e22941).exp();
        let assign18840_e22944: f64 = (assign18840_e22942 + 1e-50);
        (assign18840_e22944, (assign18840_e22942 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign18840_e22942 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign18840_e22942 * ((locals.var_beta_dn4 * assign18840_e22940) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign18840_e22942 * (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), (assign18840_e22942 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign18840_e22942 * (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), (assign18840_e22942 * (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), (assign18840_e22942 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign18840_e22942 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn12,)
    }
};
        locals.var_exp_bvbs = assign18840_e22946;
        locals.var_exp_bvbs_dn0 = assign18840_e22946_d_n0;
        locals.var_exp_bvbs_dn2 = assign18840_e22946_d_n2;
        locals.var_exp_bvbs_dn4 = assign18840_e22946_d_n4;
        locals.var_exp_bvbs_dn5 = assign18840_e22946_d_n5;
        locals.var_exp_bvbs_dn6 = assign18840_e22946_d_n6;
        locals.var_exp_bvbs_dn8 = assign18840_e22946_d_n8;
        locals.var_exp_bvbs_dn10 = assign18840_e22946_d_n10;
        locals.var_exp_bvbs_dn11 = assign18840_e22946_d_n11;
        locals.var_exp_bvbs_dn12 = assign18840_e22946_d_n12;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign18850_e22957, assign18850_e22957_d_n0, assign18850_e22957_d_n2, assign18850_e22957_d_n4, assign18850_e22957_d_n5, assign18850_e22957_d_n6, assign18850_e22957_d_n8, assign18850_e22957_d_n10, assign18850_e22957_d_n11, assign18850_e22957_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18850_e22955: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign18850_e22955, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn4 / locals.var_mks_nover), (locals.var_nin_dn5 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn8 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign18850_e22957;
        locals.var_t0_dn0 = assign18850_e22957_d_n0;
        locals.var_t0_dn2 = assign18850_e22957_d_n2;
        locals.var_t0_dn4 = assign18850_e22957_d_n4;
        locals.var_t0_dn5 = assign18850_e22957_d_n5;
        locals.var_t0_dn6 = assign18850_e22957_d_n6;
        locals.var_t0_dn8 = assign18850_e22957_d_n8;
        locals.var_t0_dn10 = assign18850_e22957_d_n10;
        locals.var_t0_dn11 = assign18850_e22957_d_n11;
        locals.var_t0_dn12 = assign18850_e22957_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign18860_e22968, assign18860_e22968_d_n0, assign18860_e22968_d_n2, assign18860_e22968_d_n4, assign18860_e22968_d_n5, assign18860_e22968_d_n6, assign18860_e22968_d_n8, assign18860_e22968_d_n10, assign18860_e22968_d_n11, assign18860_e22968_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18860_e22966: f64 = (locals.var_t0 * locals.var_t0);
        (assign18860_e22966, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn8, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12,)
    }
};
        locals.var_cnst1over = assign18860_e22968;
        locals.var_cnst1over_dn0 = assign18860_e22968_d_n0;
        locals.var_cnst1over_dn2 = assign18860_e22968_d_n2;
        locals.var_cnst1over_dn4 = assign18860_e22968_d_n4;
        locals.var_cnst1over_dn5 = assign18860_e22968_d_n5;
        locals.var_cnst1over_dn6 = assign18860_e22968_d_n6;
        locals.var_cnst1over_dn8 = assign18860_e22968_d_n8;
        locals.var_cnst1over_dn10 = assign18860_e22968_d_n10;
        locals.var_cnst1over_dn11 = assign18860_e22968_d_n11;
        locals.var_cnst1over_dn12 = assign18860_e22968_d_n12;
        locals.var_cnst1over_rv = 0.0;

        let (assign18870_e22979, assign18870_e22979_d_n0, assign18870_e22979_d_n2, assign18870_e22979_d_n4, assign18870_e22979_d_n5, assign18870_e22979_d_n6, assign18870_e22979_d_n8, assign18870_e22979_d_n10, assign18870_e22979_d_n11, assign18870_e22979_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18870_e22977: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign18870_e22977, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn12)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn8, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn12,)
    }
};
        locals.var_gammachi = assign18870_e22979;
        locals.var_gammachi_dn0 = assign18870_e22979_d_n0;
        locals.var_gammachi_dn2 = assign18870_e22979_d_n2;
        locals.var_gammachi_dn4 = assign18870_e22979_d_n4;
        locals.var_gammachi_dn5 = assign18870_e22979_d_n5;
        locals.var_gammachi_dn6 = assign18870_e22979_d_n6;
        locals.var_gammachi_dn8 = assign18870_e22979_d_n8;
        locals.var_gammachi_dn10 = assign18870_e22979_d_n10;
        locals.var_gammachi_dn11 = assign18870_e22979_d_n11;
        locals.var_gammachi_dn12 = assign18870_e22979_d_n12;
        locals.var_gammachi_rv = 0.0;

        let (assign18880_e22990, assign18880_e22990_d_n0, assign18880_e22990_d_n2, assign18880_e22990_d_n4, assign18880_e22990_d_n5, assign18880_e22990_d_n6, assign18880_e22990_d_n8, assign18880_e22990_d_n10, assign18880_e22990_d_n11, assign18880_e22990_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18880_e22988: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign18880_e22988, (locals.var_beta2 * locals.var_fac1p2_dn0), (locals.var_beta2 * locals.var_fac1p2_dn2), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), (locals.var_beta2 * locals.var_fac1p2_dn5), (locals.var_beta2 * locals.var_fac1p2_dn6), (locals.var_beta2 * locals.var_fac1p2_dn8), (locals.var_beta2 * locals.var_fac1p2_dn10), (locals.var_beta2 * locals.var_fac1p2_dn11), (locals.var_beta2 * locals.var_fac1p2_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign18880_e22990;
        locals.var_t0_dn0 = assign18880_e22990_d_n0;
        locals.var_t0_dn2 = assign18880_e22990_d_n2;
        locals.var_t0_dn4 = assign18880_e22990_d_n4;
        locals.var_t0_dn5 = assign18880_e22990_d_n5;
        locals.var_t0_dn6 = assign18880_e22990_d_n6;
        locals.var_t0_dn8 = assign18880_e22990_d_n8;
        locals.var_t0_dn10 = assign18880_e22990_d_n10;
        locals.var_t0_dn11 = assign18880_e22990_d_n11;
        locals.var_t0_dn12 = assign18880_e22990_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign18890_e23001, assign18890_e23001_d_n0, assign18890_e23001_d_n2, assign18890_e23001_d_n4, assign18890_e23001_d_n5, assign18890_e23001_d_n6, assign18890_e23001_d_n8, assign18890_e23001_d_n10, assign18890_e23001_d_n11, assign18890_e23001_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18890_e22999: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign18890_e22999, (locals.var_beta * locals.var_vgpld_shift_dn0), (locals.var_beta * locals.var_vgpld_shift_dn2), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), (locals.var_beta * locals.var_vgpld_shift_dn5), (locals.var_beta * locals.var_vgpld_shift_dn6), (locals.var_beta * locals.var_vgpld_shift_dn8), (locals.var_beta * locals.var_vgpld_shift_dn10), (locals.var_beta * locals.var_vgpld_shift_dn11), (locals.var_beta * locals.var_vgpld_shift_dn12),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12,)
    }
};
        locals.var_psi = assign18890_e23001;
        locals.var_psi_dn0 = assign18890_e23001_d_n0;
        locals.var_psi_dn2 = assign18890_e23001_d_n2;
        locals.var_psi_dn4 = assign18890_e23001_d_n4;
        locals.var_psi_dn5 = assign18890_e23001_d_n5;
        locals.var_psi_dn6 = assign18890_e23001_d_n6;
        locals.var_psi_dn8 = assign18890_e23001_d_n8;
        locals.var_psi_dn10 = assign18890_e23001_d_n10;
        locals.var_psi_dn11 = assign18890_e23001_d_n11;
        locals.var_psi_dn12 = assign18890_e23001_d_n12;
        locals.var_psi_rv = 0.0;

        let (assign18900_e23026, assign18900_e23026_d_n0, assign18900_e23026_d_n2, assign18900_e23026_d_n4, assign18900_e23026_d_n5, assign18900_e23026_d_n6, assign18900_e23026_d_n8, assign18900_e23026_d_n10, assign18900_e23026_d_n11, assign18900_e23026_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18900_e23010: f64 = (locals.var_gammachi * locals.var_t0);
        let assign18900_e23013: f64 = (locals.var_psi * locals.var_psi);
        let assign18900_e23014: f64 = (assign18900_e23010 + assign18900_e23013);
        let assign18900_e23015: f64 = (assign18900_e23014).ln();
        let assign18900_e23018: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign18900_e23019: f64 = (assign18900_e23018).ln();
        let assign18900_e23020: f64 = (assign18900_e23015 - assign18900_e23019);
        let assign18900_e23023: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign18900_e23024: f64 = (assign18900_e23020 + assign18900_e23023);
        (assign18900_e23024, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign18900_e23014) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign18900_e23018)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign18900_e23014) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign18900_e23018)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign18900_e23014) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign18900_e23018)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign18900_e23014) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign18900_e23018)) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign18900_e23014) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign18900_e23018)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign18900_e23014) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign18900_e23018)) + (locals.var_beta * locals.var_vxbgmtcl_dn8)), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign18900_e23014) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign18900_e23018)) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign18900_e23014) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign18900_e23018)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign18900_e23014) - (((locals.var_cnst1over_dn12 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn12)) / assign18900_e23018)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn8, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12,)
    }
};
        locals.var_chi_1 = assign18900_e23026;
        locals.var_chi_1_dn0 = assign18900_e23026_d_n0;
        locals.var_chi_1_dn2 = assign18900_e23026_d_n2;
        locals.var_chi_1_dn4 = assign18900_e23026_d_n4;
        locals.var_chi_1_dn5 = assign18900_e23026_d_n5;
        locals.var_chi_1_dn6 = assign18900_e23026_d_n6;
        locals.var_chi_1_dn8 = assign18900_e23026_d_n8;
        locals.var_chi_1_dn10 = assign18900_e23026_d_n10;
        locals.var_chi_1_dn11 = assign18900_e23026_d_n11;
        locals.var_chi_1_dn12 = assign18900_e23026_d_n12;
        locals.var_chi_1_rv = 0.0;

        let (assign18910_e23039, assign18910_e23039_d_n0, assign18910_e23039_d_n2, assign18910_e23039_d_n4, assign18910_e23039_d_n5, assign18910_e23039_d_n6, assign18910_e23039_d_n8, assign18910_e23039_d_n10, assign18910_e23039_d_n11, assign18910_e23039_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18910_e23035: f64 = (locals.var_psi - locals.var_chi_1);
        let assign18910_e23037: f64 = (assign18910_e23035 - 1.0);
        (assign18910_e23037, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign18910_e23039;
        locals.var_tmf1_dn0 = assign18910_e23039_d_n0;
        locals.var_tmf1_dn2 = assign18910_e23039_d_n2;
        locals.var_tmf1_dn4 = assign18910_e23039_d_n4;
        locals.var_tmf1_dn5 = assign18910_e23039_d_n5;
        locals.var_tmf1_dn6 = assign18910_e23039_d_n6;
        locals.var_tmf1_dn8 = assign18910_e23039_d_n8;
        locals.var_tmf1_dn10 = assign18910_e23039_d_n10;
        locals.var_tmf1_dn11 = assign18910_e23039_d_n11;
        locals.var_tmf1_dn12 = assign18910_e23039_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign18920_e23052, assign18920_e23052_d_n0, assign18920_e23052_d_n2, assign18920_e23052_d_n4, assign18920_e23052_d_n5, assign18920_e23052_d_n6, assign18920_e23052_d_n8, assign18920_e23052_d_n10, assign18920_e23052_d_n11, assign18920_e23052_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18920_e23048: f64 = (4.0 * locals.var_psi);
        let assign18920_e23050: f64 = assign18920_e23048;
        (assign18920_e23050, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn12),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign18920_e23052;
        locals.var_tmf2_dn0 = assign18920_e23052_d_n0;
        locals.var_tmf2_dn2 = assign18920_e23052_d_n2;
        locals.var_tmf2_dn4 = assign18920_e23052_d_n4;
        locals.var_tmf2_dn5 = assign18920_e23052_d_n5;
        locals.var_tmf2_dn6 = assign18920_e23052_d_n6;
        locals.var_tmf2_dn8 = assign18920_e23052_d_n8;
        locals.var_tmf2_dn10 = assign18920_e23052_d_n10;
        locals.var_tmf2_dn11 = assign18920_e23052_d_n11;
        locals.var_tmf2_dn12 = assign18920_e23052_d_n12;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_75(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18930_e23067, assign18930_e23067_d_n0, assign18930_e23067_d_n2, assign18930_e23067_d_n4, assign18930_e23067_d_n5, assign18930_e23067_d_n6, assign18930_e23067_d_n8, assign18930_e23067_d_n10, assign18930_e23067_d_n11, assign18930_e23067_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let (assign18930_e23065, assign18930_e23065_d_n0, assign18930_e23065_d_n2, assign18930_e23065_d_n4, assign18930_e23065_d_n5, assign18930_e23065_d_n6, assign18930_e23065_d_n8, assign18930_e23065_d_n10, assign18930_e23065_d_n11, assign18930_e23065_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign18930_e23064: f64 = (-locals.var_tmf2);
                (assign18930_e23064, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign18930_e23065, assign18930_e23065_d_n0, assign18930_e23065_d_n2, assign18930_e23065_d_n4, assign18930_e23065_d_n5, assign18930_e23065_d_n6, assign18930_e23065_d_n8, assign18930_e23065_d_n10, assign18930_e23065_d_n11, assign18930_e23065_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign18930_e23067;
        locals.var_tmf2_dn0 = assign18930_e23067_d_n0;
        locals.var_tmf2_dn2 = assign18930_e23067_d_n2;
        locals.var_tmf2_dn4 = assign18930_e23067_d_n4;
        locals.var_tmf2_dn5 = assign18930_e23067_d_n5;
        locals.var_tmf2_dn6 = assign18930_e23067_d_n6;
        locals.var_tmf2_dn8 = assign18930_e23067_d_n8;
        locals.var_tmf2_dn10 = assign18930_e23067_d_n10;
        locals.var_tmf2_dn11 = assign18930_e23067_d_n11;
        locals.var_tmf2_dn12 = assign18930_e23067_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign18940_e23081, assign18940_e23081_d_n0, assign18940_e23081_d_n2, assign18940_e23081_d_n4, assign18940_e23081_d_n5, assign18940_e23081_d_n6, assign18940_e23081_d_n8, assign18940_e23081_d_n10, assign18940_e23081_d_n11, assign18940_e23081_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18940_e23076: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18940_e23078: f64 = (assign18940_e23076 + locals.var_tmf2);
        let assign18940_e23079: f64 = (assign18940_e23078).sqrt();
        (assign18940_e23079, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18940_e23079)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18940_e23079)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18940_e23079)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18940_e23079)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18940_e23079)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18940_e23079)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18940_e23079)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18940_e23079)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign18940_e23079)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign18940_e23081;
        locals.var_tmf2_dn0 = assign18940_e23081_d_n0;
        locals.var_tmf2_dn2 = assign18940_e23081_d_n2;
        locals.var_tmf2_dn4 = assign18940_e23081_d_n4;
        locals.var_tmf2_dn5 = assign18940_e23081_d_n5;
        locals.var_tmf2_dn6 = assign18940_e23081_d_n6;
        locals.var_tmf2_dn8 = assign18940_e23081_d_n8;
        locals.var_tmf2_dn10 = assign18940_e23081_d_n10;
        locals.var_tmf2_dn11 = assign18940_e23081_d_n11;
        locals.var_tmf2_dn12 = assign18940_e23081_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign18950_e23096, assign18950_e23096_d_n0, assign18950_e23096_d_n2, assign18950_e23096_d_n4, assign18950_e23096_d_n5, assign18950_e23096_d_n6, assign18950_e23096_d_n8, assign18950_e23096_d_n10, assign18950_e23096_d_n11, assign18950_e23096_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18950_e23092: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18950_e23093: f64 = (1.0 + assign18950_e23092);
        let assign18950_e23094: f64 = (0.5 * assign18950_e23093);
        (assign18950_e23094, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign18950_e23096;
        locals.var_t1_dn0 = assign18950_e23096_d_n0;
        locals.var_t1_dn2 = assign18950_e23096_d_n2;
        locals.var_t1_dn4 = assign18950_e23096_d_n4;
        locals.var_t1_dn5 = assign18950_e23096_d_n5;
        locals.var_t1_dn6 = assign18950_e23096_d_n6;
        locals.var_t1_dn8 = assign18950_e23096_d_n8;
        locals.var_t1_dn10 = assign18950_e23096_d_n10;
        locals.var_t1_dn11 = assign18950_e23096_d_n11;
        locals.var_t1_dn12 = assign18950_e23096_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign18960_e23115, assign18960_e23115_d_n0, assign18960_e23115_d_n2, assign18960_e23115_d_n4, assign18960_e23115_d_n5, assign18960_e23115_d_n6, assign18960_e23115_d_n8, assign18960_e23115_d_n10, assign18960_e23115_d_n11, assign18960_e23115_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18960_e23108: f64 = 2.0;
        let assign18960_e23109: f64 = (locals.var_tmf1 + assign18960_e23108);
        let assign18960_e23111: f64 = (assign18960_e23109 / locals.var_tmf2);
        let assign18960_e23112: f64 = (1.0 - assign18960_e23111);
        let assign18960_e23113: f64 = (0.5 * assign18960_e23112);
        (assign18960_e23113, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign18960_e23109 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign18960_e23109 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign18960_e23109 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign18960_e23109 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign18960_e23109 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign18960_e23109 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign18960_e23109 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign18960_e23109 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign18960_e23109 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign18960_e23115;
        locals.var_t2_dn0 = assign18960_e23115_d_n0;
        locals.var_t2_dn2 = assign18960_e23115_d_n2;
        locals.var_t2_dn4 = assign18960_e23115_d_n4;
        locals.var_t2_dn5 = assign18960_e23115_d_n5;
        locals.var_t2_dn6 = assign18960_e23115_d_n6;
        locals.var_t2_dn8 = assign18960_e23115_d_n8;
        locals.var_t2_dn10 = assign18960_e23115_d_n10;
        locals.var_t2_dn11 = assign18960_e23115_d_n11;
        locals.var_t2_dn12 = assign18960_e23115_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign18970_e23130, assign18970_e23130_d_n0, assign18970_e23130_d_n2, assign18970_e23130_d_n4, assign18970_e23130_d_n5, assign18970_e23130_d_n6, assign18970_e23130_d_n8, assign18970_e23130_d_n10, assign18970_e23130_d_n11, assign18970_e23130_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18970_e23126: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18970_e23127: f64 = (0.5 * assign18970_e23126);
        let assign18970_e23128: f64 = (locals.var_psi - assign18970_e23127);
        (assign18970_e23128, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn8, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12,)
    }
};
        locals.var_chi_1 = assign18970_e23130;
        locals.var_chi_1_dn0 = assign18970_e23130_d_n0;
        locals.var_chi_1_dn2 = assign18970_e23130_d_n2;
        locals.var_chi_1_dn4 = assign18970_e23130_d_n4;
        locals.var_chi_1_dn5 = assign18970_e23130_d_n5;
        locals.var_chi_1_dn6 = assign18970_e23130_d_n6;
        locals.var_chi_1_dn8 = assign18970_e23130_d_n8;
        locals.var_chi_1_dn10 = assign18970_e23130_d_n10;
        locals.var_chi_1_dn11 = assign18970_e23130_d_n11;
        locals.var_chi_1_dn12 = assign18970_e23130_d_n12;
        locals.var_chi_1_rv = 0.0;

        let (assign18980_e23141, assign18980_e23141_d_n0, assign18980_e23141_d_n2, assign18980_e23141_d_n4, assign18980_e23141_d_n5, assign18980_e23141_d_n6, assign18980_e23141_d_n8, assign18980_e23141_d_n10, assign18980_e23141_d_n11, assign18980_e23141_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18980_e23139: f64 = (locals.var_psi - locals.var_chi_1);
        (assign18980_e23139, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12,)
    }
};
        locals.var_psi = assign18980_e23141;
        locals.var_psi_dn0 = assign18980_e23141_d_n0;
        locals.var_psi_dn2 = assign18980_e23141_d_n2;
        locals.var_psi_dn4 = assign18980_e23141_d_n4;
        locals.var_psi_dn5 = assign18980_e23141_d_n5;
        locals.var_psi_dn6 = assign18980_e23141_d_n6;
        locals.var_psi_dn8 = assign18980_e23141_d_n8;
        locals.var_psi_dn10 = assign18980_e23141_d_n10;
        locals.var_psi_dn11 = assign18980_e23141_d_n11;
        locals.var_psi_dn12 = assign18980_e23141_d_n12;
        locals.var_psi_rv = 0.0;

        let (assign18990_e23154, assign18990_e23154_d_n0, assign18990_e23154_d_n2, assign18990_e23154_d_n4, assign18990_e23154_d_n5, assign18990_e23154_d_n6, assign18990_e23154_d_n8, assign18990_e23154_d_n10, assign18990_e23154_d_n11, assign18990_e23154_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign18990_e23151: f64 = (locals.var_beta * 0.1);
        let assign18990_e23152: f64 = (locals.var_psi + assign18990_e23151);
        (assign18990_e23152, locals.var_psi_dn0, locals.var_psi_dn2, (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12,)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12,)
    }
};
        locals.var_psi = assign18990_e23154;
        locals.var_psi_dn0 = assign18990_e23154_d_n0;
        locals.var_psi_dn2 = assign18990_e23154_d_n2;
        locals.var_psi_dn4 = assign18990_e23154_d_n4;
        locals.var_psi_dn5 = assign18990_e23154_d_n5;
        locals.var_psi_dn6 = assign18990_e23154_d_n6;
        locals.var_psi_dn8 = assign18990_e23154_d_n8;
        locals.var_psi_dn10 = assign18990_e23154_d_n10;
        locals.var_psi_dn11 = assign18990_e23154_d_n11;
        locals.var_psi_dn12 = assign18990_e23154_d_n12;
        locals.var_psi_rv = 0.0;

        let (assign19000_e23179, assign19000_e23179_d_n0, assign19000_e23179_d_n2, assign19000_e23179_d_n4, assign19000_e23179_d_n5, assign19000_e23179_d_n6, assign19000_e23179_d_n8, assign19000_e23179_d_n10, assign19000_e23179_d_n11, assign19000_e23179_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign19000_e23163: f64 = (locals.var_gammachi * locals.var_t0);
        let assign19000_e23166: f64 = (locals.var_psi * locals.var_psi);
        let assign19000_e23167: f64 = (assign19000_e23163 + assign19000_e23166);
        let assign19000_e23168: f64 = (assign19000_e23167).ln();
        let assign19000_e23171: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign19000_e23172: f64 = (assign19000_e23171).ln();
        let assign19000_e23173: f64 = (assign19000_e23168 - assign19000_e23172);
        let assign19000_e23176: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign19000_e23177: f64 = (assign19000_e23173 + assign19000_e23176);
        (assign19000_e23177, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign19000_e23167) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign19000_e23171)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign19000_e23167) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign19000_e23171)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign19000_e23167) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign19000_e23171)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign19000_e23167) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign19000_e23171)) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign19000_e23167) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign19000_e23171)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign19000_e23167) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign19000_e23171)) + (locals.var_beta * locals.var_vxbgmtcl_dn8)), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign19000_e23167) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign19000_e23171)) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign19000_e23167) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign19000_e23171)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign19000_e23167) - (((locals.var_cnst1over_dn12 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn12)) / assign19000_e23171)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn8, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn12,)
    }
};
        locals.var_chi_b = assign19000_e23179;
        locals.var_chi_b_dn0 = assign19000_e23179_d_n0;
        locals.var_chi_b_dn2 = assign19000_e23179_d_n2;
        locals.var_chi_b_dn4 = assign19000_e23179_d_n4;
        locals.var_chi_b_dn5 = assign19000_e23179_d_n5;
        locals.var_chi_b_dn6 = assign19000_e23179_d_n6;
        locals.var_chi_b_dn8 = assign19000_e23179_d_n8;
        locals.var_chi_b_dn10 = assign19000_e23179_d_n10;
        locals.var_chi_b_dn11 = assign19000_e23179_d_n11;
        locals.var_chi_b_dn12 = assign19000_e23179_d_n12;
        locals.var_chi_b_rv = 0.0;

        let (assign19010_e23192, assign19010_e23192_d_n0, assign19010_e23192_d_n2, assign19010_e23192_d_n4, assign19010_e23192_d_n5, assign19010_e23192_d_n6, assign19010_e23192_d_n8, assign19010_e23192_d_n10, assign19010_e23192_d_n11, assign19010_e23192_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign19010_e23188: f64 = (locals.var_chi_b / locals.var_beta);
        let assign19010_e23190: f64 = (assign19010_e23188 - locals.var_vxbgmtcl);
        (assign19010_e23190, ((locals.var_chi_b_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_b_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((((locals.var_chi_b_dn4 * locals.var_beta) - (locals.var_chi_b * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_b_dn5 / locals.var_beta) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_b_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_b_dn8 / locals.var_beta) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_b_dn10 / locals.var_beta) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_b_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_b_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn4, locals.var_ps0_inib_dn5, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn8, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12,)
    }
};
        locals.var_ps0_inib = assign19010_e23192;
        locals.var_ps0_inib_dn0 = assign19010_e23192_d_n0;
        locals.var_ps0_inib_dn2 = assign19010_e23192_d_n2;
        locals.var_ps0_inib_dn4 = assign19010_e23192_d_n4;
        locals.var_ps0_inib_dn5 = assign19010_e23192_d_n5;
        locals.var_ps0_inib_dn6 = assign19010_e23192_d_n6;
        locals.var_ps0_inib_dn8 = assign19010_e23192_d_n8;
        locals.var_ps0_inib_dn10 = assign19010_e23192_d_n10;
        locals.var_ps0_inib_dn11 = assign19010_e23192_d_n11;
        locals.var_ps0_inib_dn12 = assign19010_e23192_d_n12;
        locals.var_ps0_inib_rv = 0.0;

        let (assign19020_e23201, assign19020_e23201_d_n0, assign19020_e23201_d_n2, assign19020_e23201_d_n4, assign19020_e23201_d_n5, assign19020_e23201_d_n6, assign19020_e23201_d_n8, assign19020_e23201_d_n10, assign19020_e23201_d_n11, assign19020_e23201_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn8, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn12,)
    }
};
        locals.var_chi_a = assign19020_e23201;
        locals.var_chi_a_dn0 = assign19020_e23201_d_n0;
        locals.var_chi_a_dn2 = assign19020_e23201_d_n2;
        locals.var_chi_a_dn4 = assign19020_e23201_d_n4;
        locals.var_chi_a_dn5 = assign19020_e23201_d_n5;
        locals.var_chi_a_dn6 = assign19020_e23201_d_n6;
        locals.var_chi_a_dn8 = assign19020_e23201_d_n8;
        locals.var_chi_a_dn10 = assign19020_e23201_d_n10;
        locals.var_chi_a_dn11 = assign19020_e23201_d_n11;
        locals.var_chi_a_dn12 = assign19020_e23201_d_n12;
        locals.var_chi_a_rv = 0.0;

        let (assign19030_e23216, assign19030_e23216_d_n0, assign19030_e23216_d_n2, assign19030_e23216_d_n4, assign19030_e23216_d_n5, assign19030_e23216_d_n6, assign19030_e23216_d_n8, assign19030_e23216_d_n10, assign19030_e23216_d_n11, assign19030_e23216_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign19030_e23210: f64 = (locals.var_chi_b - locals.var_chi_a);
        let assign19030_e23213: f64 = (0.0008 * 75.0);
        let assign19030_e23214: f64 = (assign19030_e23210 - assign19030_e23213);
        (assign19030_e23214, (locals.var_chi_b_dn0 - locals.var_chi_a_dn0), (locals.var_chi_b_dn2 - locals.var_chi_a_dn2), (locals.var_chi_b_dn4 - locals.var_chi_a_dn4), (locals.var_chi_b_dn5 - locals.var_chi_a_dn5), (locals.var_chi_b_dn6 - locals.var_chi_a_dn6), (locals.var_chi_b_dn8 - locals.var_chi_a_dn8), (locals.var_chi_b_dn10 - locals.var_chi_a_dn10), (locals.var_chi_b_dn11 - locals.var_chi_a_dn11), (locals.var_chi_b_dn12 - locals.var_chi_a_dn12),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign19030_e23216;
        locals.var_tmf1_dn0 = assign19030_e23216_d_n0;
        locals.var_tmf1_dn2 = assign19030_e23216_d_n2;
        locals.var_tmf1_dn4 = assign19030_e23216_d_n4;
        locals.var_tmf1_dn5 = assign19030_e23216_d_n5;
        locals.var_tmf1_dn6 = assign19030_e23216_d_n6;
        locals.var_tmf1_dn8 = assign19030_e23216_d_n8;
        locals.var_tmf1_dn10 = assign19030_e23216_d_n10;
        locals.var_tmf1_dn11 = assign19030_e23216_d_n11;
        locals.var_tmf1_dn12 = assign19030_e23216_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign19040_e23231, assign19040_e23231_d_n0, assign19040_e23231_d_n2, assign19040_e23231_d_n4, assign19040_e23231_d_n5, assign19040_e23231_d_n6, assign19040_e23231_d_n8, assign19040_e23231_d_n10, assign19040_e23231_d_n11, assign19040_e23231_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign19040_e23225: f64 = (4.0 * locals.var_chi_b);
        let assign19040_e23228: f64 = (0.0008 * 75.0);
        let assign19040_e23229: f64 = (assign19040_e23225 * assign19040_e23228);
        (assign19040_e23229, ((4.0 * locals.var_chi_b_dn0) * assign19040_e23228), ((4.0 * locals.var_chi_b_dn2) * assign19040_e23228), ((4.0 * locals.var_chi_b_dn4) * assign19040_e23228), ((4.0 * locals.var_chi_b_dn5) * assign19040_e23228), ((4.0 * locals.var_chi_b_dn6) * assign19040_e23228), ((4.0 * locals.var_chi_b_dn8) * assign19040_e23228), ((4.0 * locals.var_chi_b_dn10) * assign19040_e23228), ((4.0 * locals.var_chi_b_dn11) * assign19040_e23228), ((4.0 * locals.var_chi_b_dn12) * assign19040_e23228),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign19040_e23231;
        locals.var_tmf2_dn0 = assign19040_e23231_d_n0;
        locals.var_tmf2_dn2 = assign19040_e23231_d_n2;
        locals.var_tmf2_dn4 = assign19040_e23231_d_n4;
        locals.var_tmf2_dn5 = assign19040_e23231_d_n5;
        locals.var_tmf2_dn6 = assign19040_e23231_d_n6;
        locals.var_tmf2_dn8 = assign19040_e23231_d_n8;
        locals.var_tmf2_dn10 = assign19040_e23231_d_n10;
        locals.var_tmf2_dn11 = assign19040_e23231_d_n11;
        locals.var_tmf2_dn12 = assign19040_e23231_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign19050_e23246, assign19050_e23246_d_n0, assign19050_e23246_d_n2, assign19050_e23246_d_n4, assign19050_e23246_d_n5, assign19050_e23246_d_n6, assign19050_e23246_d_n8, assign19050_e23246_d_n10, assign19050_e23246_d_n11, assign19050_e23246_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let (assign19050_e23244, assign19050_e23244_d_n0, assign19050_e23244_d_n2, assign19050_e23244_d_n4, assign19050_e23244_d_n5, assign19050_e23244_d_n6, assign19050_e23244_d_n8, assign19050_e23244_d_n10, assign19050_e23244_d_n11, assign19050_e23244_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign19050_e23243: f64 = (-locals.var_tmf2);
                (assign19050_e23243, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign19050_e23244, assign19050_e23244_d_n0, assign19050_e23244_d_n2, assign19050_e23244_d_n4, assign19050_e23244_d_n5, assign19050_e23244_d_n6, assign19050_e23244_d_n8, assign19050_e23244_d_n10, assign19050_e23244_d_n11, assign19050_e23244_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign19050_e23246;
        locals.var_tmf2_dn0 = assign19050_e23246_d_n0;
        locals.var_tmf2_dn2 = assign19050_e23246_d_n2;
        locals.var_tmf2_dn4 = assign19050_e23246_d_n4;
        locals.var_tmf2_dn5 = assign19050_e23246_d_n5;
        locals.var_tmf2_dn6 = assign19050_e23246_d_n6;
        locals.var_tmf2_dn8 = assign19050_e23246_d_n8;
        locals.var_tmf2_dn10 = assign19050_e23246_d_n10;
        locals.var_tmf2_dn11 = assign19050_e23246_d_n11;
        locals.var_tmf2_dn12 = assign19050_e23246_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign19060_e23260, assign19060_e23260_d_n0, assign19060_e23260_d_n2, assign19060_e23260_d_n4, assign19060_e23260_d_n5, assign19060_e23260_d_n6, assign19060_e23260_d_n8, assign19060_e23260_d_n10, assign19060_e23260_d_n11, assign19060_e23260_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign19060_e23255: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19060_e23257: f64 = (assign19060_e23255 + locals.var_tmf2);
        let assign19060_e23258: f64 = (assign19060_e23257).sqrt();
        (assign19060_e23258, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19060_e23258)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19060_e23258)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19060_e23258)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19060_e23258)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19060_e23258)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19060_e23258)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19060_e23258)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19060_e23258)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign19060_e23258)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign19060_e23260;
        locals.var_tmf2_dn0 = assign19060_e23260_d_n0;
        locals.var_tmf2_dn2 = assign19060_e23260_d_n2;
        locals.var_tmf2_dn4 = assign19060_e23260_d_n4;
        locals.var_tmf2_dn5 = assign19060_e23260_d_n5;
        locals.var_tmf2_dn6 = assign19060_e23260_d_n6;
        locals.var_tmf2_dn8 = assign19060_e23260_d_n8;
        locals.var_tmf2_dn10 = assign19060_e23260_d_n10;
        locals.var_tmf2_dn11 = assign19060_e23260_d_n11;
        locals.var_tmf2_dn12 = assign19060_e23260_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign19070_e23275, assign19070_e23275_d_n0, assign19070_e23275_d_n2, assign19070_e23275_d_n4, assign19070_e23275_d_n5, assign19070_e23275_d_n6, assign19070_e23275_d_n8, assign19070_e23275_d_n10, assign19070_e23275_d_n11, assign19070_e23275_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign19070_e23271: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19070_e23272: f64 = (1.0 + assign19070_e23271);
        let assign19070_e23273: f64 = (0.5 * assign19070_e23272);
        (assign19070_e23273, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign19070_e23275;
        locals.var_t1_dn0 = assign19070_e23275_d_n0;
        locals.var_t1_dn2 = assign19070_e23275_d_n2;
        locals.var_t1_dn4 = assign19070_e23275_d_n4;
        locals.var_t1_dn5 = assign19070_e23275_d_n5;
        locals.var_t1_dn6 = assign19070_e23275_d_n6;
        locals.var_t1_dn8 = assign19070_e23275_d_n8;
        locals.var_t1_dn10 = assign19070_e23275_d_n10;
        locals.var_t1_dn11 = assign19070_e23275_d_n11;
        locals.var_t1_dn12 = assign19070_e23275_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign19080_e23296, assign19080_e23296_d_n0, assign19080_e23296_d_n2, assign19080_e23296_d_n4, assign19080_e23296_d_n5, assign19080_e23296_d_n6, assign19080_e23296_d_n8, assign19080_e23296_d_n10, assign19080_e23296_d_n11, assign19080_e23296_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign19080_e23287: f64 = (2.0 * 0.0008);
        let assign19080_e23289: f64 = (assign19080_e23287 * 75.0);
        let assign19080_e23290: f64 = (locals.var_tmf1 + assign19080_e23289);
        let assign19080_e23292: f64 = (assign19080_e23290 / locals.var_tmf2);
        let assign19080_e23293: f64 = (1.0 - assign19080_e23292);
        let assign19080_e23294: f64 = (0.5 * assign19080_e23293);
        (assign19080_e23294, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign19080_e23290 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign19080_e23290 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign19080_e23290 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign19080_e23290 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign19080_e23290 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign19080_e23290 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign19080_e23290 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign19080_e23290 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign19080_e23290 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign19080_e23296;
        locals.var_t2_dn0 = assign19080_e23296_d_n0;
        locals.var_t2_dn2 = assign19080_e23296_d_n2;
        locals.var_t2_dn4 = assign19080_e23296_d_n4;
        locals.var_t2_dn5 = assign19080_e23296_d_n5;
        locals.var_t2_dn6 = assign19080_e23296_d_n6;
        locals.var_t2_dn8 = assign19080_e23296_d_n8;
        locals.var_t2_dn10 = assign19080_e23296_d_n10;
        locals.var_t2_dn11 = assign19080_e23296_d_n11;
        locals.var_t2_dn12 = assign19080_e23296_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign19090_e23311, assign19090_e23311_d_n0, assign19090_e23311_d_n2, assign19090_e23311_d_n4, assign19090_e23311_d_n5, assign19090_e23311_d_n6, assign19090_e23311_d_n8, assign19090_e23311_d_n10, assign19090_e23311_d_n11, assign19090_e23311_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign19090_e23307: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19090_e23308: f64 = (0.5 * assign19090_e23307);
        let assign19090_e23309: f64 = (locals.var_chi_b - assign19090_e23308);
        (assign19090_e23309, (locals.var_chi_b_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_chi_b_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_chi_b_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_chi_b_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign19090_e23311;
        locals.var_chi_dn0 = assign19090_e23311_d_n0;
        locals.var_chi_dn2 = assign19090_e23311_d_n2;
        locals.var_chi_dn4 = assign19090_e23311_d_n4;
        locals.var_chi_dn5 = assign19090_e23311_d_n5;
        locals.var_chi_dn6 = assign19090_e23311_d_n6;
        locals.var_chi_dn8 = assign19090_e23311_d_n8;
        locals.var_chi_dn10 = assign19090_e23311_d_n10;
        locals.var_chi_dn11 = assign19090_e23311_d_n11;
        locals.var_chi_dn12 = assign19090_e23311_d_n12;
        locals.var_chi_rv = 0.0;

        let (assign19100_e23322, assign19100_e23322_d_n0, assign19100_e23322_d_n2, assign19100_e23322_d_n4, assign19100_e23322_d_n5, assign19100_e23322_d_n6, assign19100_e23322_d_n8, assign19100_e23322_d_n10, assign19100_e23322_d_n11, assign19100_e23322_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign19100_e23318: f64 = (locals.var_chi / locals.var_beta);
        let assign19100_e23320: f64 = (assign19100_e23318 - locals.var_vxbgmtcl);
        (assign19100_e23320, ((locals.var_chi_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((((locals.var_chi_dn4 * locals.var_beta) - (locals.var_chi * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_dn5 / locals.var_beta) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_dn8 / locals.var_beta) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_dn10 / locals.var_beta) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn8, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12,)
    }
};
        locals.var_ps0ld = assign19100_e23322;
        locals.var_ps0ld_dn0 = assign19100_e23322_d_n0;
        locals.var_ps0ld_dn2 = assign19100_e23322_d_n2;
        locals.var_ps0ld_dn4 = assign19100_e23322_d_n4;
        locals.var_ps0ld_dn5 = assign19100_e23322_d_n5;
        locals.var_ps0ld_dn6 = assign19100_e23322_d_n6;
        locals.var_ps0ld_dn8 = assign19100_e23322_d_n8;
        locals.var_ps0ld_dn10 = assign19100_e23322_d_n10;
        locals.var_ps0ld_dn11 = assign19100_e23322_d_n11;
        locals.var_ps0ld_dn12 = assign19100_e23322_d_n12;
        locals.var_ps0ld_rv = 0.0;

        let (assign19110_e23335, assign19110_e23335_d_n0, assign19110_e23335_d_n2, assign19110_e23335_d_n4, assign19110_e23335_d_n5, assign19110_e23335_d_n6, assign19110_e23335_d_n8, assign19110_e23335_d_n10, assign19110_e23335_d_n11, assign19110_e23335_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign19110_e23329: f64 = (locals.var_chi - 1.0);
        let assign19110_e23331: f64 = (-locals.var_chi);
        let assign19110_e23332: f64 = (assign19110_e23331).exp();
        let assign19110_e23333: f64 = (assign19110_e23329 + assign19110_e23332);
        (assign19110_e23333, (locals.var_chi_dn0 + (assign19110_e23332 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign19110_e23332 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign19110_e23332 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign19110_e23332 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign19110_e23332 * (-locals.var_chi_dn6))), (locals.var_chi_dn8 + (assign19110_e23332 * (-locals.var_chi_dn8))), (locals.var_chi_dn10 + (assign19110_e23332 * (-locals.var_chi_dn10))), (locals.var_chi_dn11 + (assign19110_e23332 * (-locals.var_chi_dn11))), (locals.var_chi_dn12 + (assign19110_e23332 * (-locals.var_chi_dn12))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign19110_e23335;
        locals.var_t1_dn0 = assign19110_e23335_d_n0;
        locals.var_t1_dn2 = assign19110_e23335_d_n2;
        locals.var_t1_dn4 = assign19110_e23335_d_n4;
        locals.var_t1_dn5 = assign19110_e23335_d_n5;
        locals.var_t1_dn6 = assign19110_e23335_d_n6;
        locals.var_t1_dn8 = assign19110_e23335_d_n8;
        locals.var_t1_dn10 = assign19110_e23335_d_n10;
        locals.var_t1_dn11 = assign19110_e23335_d_n11;
        locals.var_t1_dn12 = assign19110_e23335_d_n12;
        locals.var_t1_rv = 0.0;

        let assign19120_e23339: f64 = (10.0 * 2.220446049250313e-16);
        let assign19120_e23340: f64 = if locals.var_t1 < assign19120_e23339 { 1.0 } else { 0.0 };
        locals.var_guard335 = assign19120_e23340;
        locals.var_guard335_rv = 0.0;

        let (assign19130_e23351, assign19130_e23351_d_n0, assign19130_e23351_d_n2, assign19130_e23351_d_n4, assign19130_e23351_d_n5, assign19130_e23351_d_n6, assign19130_e23351_d_n8, assign19130_e23351_d_n10, assign19130_e23351_d_n11, assign19130_e23351_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard335 != 0.0)) {
        let assign19130_e23349: f64 = (10.0 * 2.220446049250313e-16);
        (assign19130_e23349, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign19130_e23351;
        locals.var_t1_dn0 = assign19130_e23351_d_n0;
        locals.var_t1_dn2 = assign19130_e23351_d_n2;
        locals.var_t1_dn4 = assign19130_e23351_d_n4;
        locals.var_t1_dn5 = assign19130_e23351_d_n5;
        locals.var_t1_dn6 = assign19130_e23351_d_n6;
        locals.var_t1_dn8 = assign19130_e23351_d_n8;
        locals.var_t1_dn10 = assign19130_e23351_d_n10;
        locals.var_t1_dn11 = assign19130_e23351_d_n11;
        locals.var_t1_dn12 = assign19130_e23351_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign19140_e23361, assign19140_e23361_d_n0, assign19140_e23361_d_n2, assign19140_e23361_d_n4, assign19140_e23361_d_n5, assign19140_e23361_d_n6, assign19140_e23361_d_n8, assign19140_e23361_d_n10, assign19140_e23361_d_n11, assign19140_e23361_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign19140_e23358: f64 = (locals.var_t1).sqrt();
        let assign19140_e23359: f64 = (locals.var_cnst0over * assign19140_e23358);
        (assign19140_e23359, ((locals.var_cnst0over_dn0 * assign19140_e23358) + (locals.var_cnst0over * (locals.var_t1_dn0 / (2.0 * assign19140_e23358)))), ((locals.var_cnst0over_dn2 * assign19140_e23358) + (locals.var_cnst0over * (locals.var_t1_dn2 / (2.0 * assign19140_e23358)))), ((locals.var_cnst0over_dn4 * assign19140_e23358) + (locals.var_cnst0over * (locals.var_t1_dn4 / (2.0 * assign19140_e23358)))), ((locals.var_cnst0over_dn5 * assign19140_e23358) + (locals.var_cnst0over * (locals.var_t1_dn5 / (2.0 * assign19140_e23358)))), ((locals.var_cnst0over_dn6 * assign19140_e23358) + (locals.var_cnst0over * (locals.var_t1_dn6 / (2.0 * assign19140_e23358)))), ((locals.var_cnst0over_dn8 * assign19140_e23358) + (locals.var_cnst0over * (locals.var_t1_dn8 / (2.0 * assign19140_e23358)))), ((locals.var_cnst0over_dn10 * assign19140_e23358) + (locals.var_cnst0over * (locals.var_t1_dn10 / (2.0 * assign19140_e23358)))), ((locals.var_cnst0over_dn11 * assign19140_e23358) + (locals.var_cnst0over * (locals.var_t1_dn11 / (2.0 * assign19140_e23358)))), ((locals.var_cnst0over_dn12 * assign19140_e23358) + (locals.var_cnst0over * (locals.var_t1_dn12 / (2.0 * assign19140_e23358)))),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn8, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12,)
    }
};
        locals.var_qbuld = assign19140_e23361;
        locals.var_qbuld_dn0 = assign19140_e23361_d_n0;
        locals.var_qbuld_dn2 = assign19140_e23361_d_n2;
        locals.var_qbuld_dn4 = assign19140_e23361_d_n4;
        locals.var_qbuld_dn5 = assign19140_e23361_d_n5;
        locals.var_qbuld_dn6 = assign19140_e23361_d_n6;
        locals.var_qbuld_dn8 = assign19140_e23361_d_n8;
        locals.var_qbuld_dn10 = assign19140_e23361_d_n10;
        locals.var_qbuld_dn11 = assign19140_e23361_d_n11;
        locals.var_qbuld_dn12 = assign19140_e23361_d_n12;
        locals.var_qbuld_rv = 0.0;

        let (assign19150_e23372, assign19150_e23372_d_n0, assign19150_e23372_d_n2, assign19150_e23372_d_n4, assign19150_e23372_d_n5, assign19150_e23372_d_n6, assign19150_e23372_d_n8, assign19150_e23372_d_n10, assign19150_e23372_d_n11, assign19150_e23372_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign19150_e23369: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign19150_e23370: f64 = (locals.var_cox0 * assign19150_e23369);
        (assign19150_e23370, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (-locals.var_ps0ld_dn4)), (locals.var_cox0 * (locals.var_vgpld_dn5 - locals.var_ps0ld_dn5)), (locals.var_cox0 * (-locals.var_ps0ld_dn6)), (locals.var_cox0 * (-locals.var_ps0ld_dn8)), (locals.var_cox0 * (-locals.var_ps0ld_dn10)), (locals.var_cox0 * (-locals.var_ps0ld_dn11)), (locals.var_cox0 * (-locals.var_ps0ld_dn12)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12,)
    }
};
        locals.var_qsuld = assign19150_e23372;
        locals.var_qsuld_dn0 = assign19150_e23372_d_n0;
        locals.var_qsuld_dn2 = assign19150_e23372_d_n2;
        locals.var_qsuld_dn4 = assign19150_e23372_d_n4;
        locals.var_qsuld_dn5 = assign19150_e23372_d_n5;
        locals.var_qsuld_dn6 = assign19150_e23372_d_n6;
        locals.var_qsuld_dn8 = assign19150_e23372_d_n8;
        locals.var_qsuld_dn10 = assign19150_e23372_d_n10;
        locals.var_qsuld_dn11 = assign19150_e23372_d_n11;
        locals.var_qsuld_dn12 = assign19150_e23372_d_n12;
        locals.var_qsuld_rv = 0.0;

        let assign19160_e23375: f64 = if p.p30 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard336 = assign19160_e23375;
        locals.var_guard336_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_76(
        locals: &mut StampLocals,
    ) {
        let (assign19170_e23388, assign19170_e23388_d_n0, assign19170_e23388_d_n2, assign19170_e23388_d_n4, assign19170_e23388_d_n5, assign19170_e23388_d_n6, assign19170_e23388_d_n8, assign19170_e23388_d_n10, assign19170_e23388_d_n11, assign19170_e23388_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19170_e23384: f64 = (-locals.var_vxbgmtcl);
        let assign19170_e23385: f64 = (locals.var_beta * assign19170_e23384);
        let assign19170_e23386: f64 = (assign19170_e23385).exp();
        (assign19170_e23386, (assign19170_e23386 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign19170_e23386 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign19170_e23386 * ((locals.var_beta_dn4 * assign19170_e23384) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign19170_e23386 * (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), (assign19170_e23386 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign19170_e23386 * (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), (assign19170_e23386 * (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), (assign19170_e23386 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign19170_e23386 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn12,)
    }
};
        locals.var_exp_bvbs = assign19170_e23388;
        locals.var_exp_bvbs_dn0 = assign19170_e23388_d_n0;
        locals.var_exp_bvbs_dn2 = assign19170_e23388_d_n2;
        locals.var_exp_bvbs_dn4 = assign19170_e23388_d_n4;
        locals.var_exp_bvbs_dn5 = assign19170_e23388_d_n5;
        locals.var_exp_bvbs_dn6 = assign19170_e23388_d_n6;
        locals.var_exp_bvbs_dn8 = assign19170_e23388_d_n8;
        locals.var_exp_bvbs_dn10 = assign19170_e23388_d_n10;
        locals.var_exp_bvbs_dn11 = assign19170_e23388_d_n11;
        locals.var_exp_bvbs_dn12 = assign19170_e23388_d_n12;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign19180_e23399, assign19180_e23399_d_n0, assign19180_e23399_d_n2, assign19180_e23399_d_n4, assign19180_e23399_d_n5, assign19180_e23399_d_n6, assign19180_e23399_d_n8, assign19180_e23399_d_n10, assign19180_e23399_d_n11, assign19180_e23399_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19180_e23397: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign19180_e23397, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn4 / locals.var_mks_nover), (locals.var_nin_dn5 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn8 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign19180_e23399;
        locals.var_t0_dn0 = assign19180_e23399_d_n0;
        locals.var_t0_dn2 = assign19180_e23399_d_n2;
        locals.var_t0_dn4 = assign19180_e23399_d_n4;
        locals.var_t0_dn5 = assign19180_e23399_d_n5;
        locals.var_t0_dn6 = assign19180_e23399_d_n6;
        locals.var_t0_dn8 = assign19180_e23399_d_n8;
        locals.var_t0_dn10 = assign19180_e23399_d_n10;
        locals.var_t0_dn11 = assign19180_e23399_d_n11;
        locals.var_t0_dn12 = assign19180_e23399_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign19190_e23410, assign19190_e23410_d_n0, assign19190_e23410_d_n2, assign19190_e23410_d_n4, assign19190_e23410_d_n5, assign19190_e23410_d_n6, assign19190_e23410_d_n8, assign19190_e23410_d_n10, assign19190_e23410_d_n11, assign19190_e23410_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19190_e23408: f64 = (locals.var_t0 * locals.var_t0);
        (assign19190_e23408, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn8, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12,)
    }
};
        locals.var_cnst1over = assign19190_e23410;
        locals.var_cnst1over_dn0 = assign19190_e23410_d_n0;
        locals.var_cnst1over_dn2 = assign19190_e23410_d_n2;
        locals.var_cnst1over_dn4 = assign19190_e23410_d_n4;
        locals.var_cnst1over_dn5 = assign19190_e23410_d_n5;
        locals.var_cnst1over_dn6 = assign19190_e23410_d_n6;
        locals.var_cnst1over_dn8 = assign19190_e23410_d_n8;
        locals.var_cnst1over_dn10 = assign19190_e23410_d_n10;
        locals.var_cnst1over_dn11 = assign19190_e23410_d_n11;
        locals.var_cnst1over_dn12 = assign19190_e23410_d_n12;
        locals.var_cnst1over_rv = 0.0;

        let (assign19200_e23421, assign19200_e23421_d_n0, assign19200_e23421_d_n2, assign19200_e23421_d_n4, assign19200_e23421_d_n5, assign19200_e23421_d_n6, assign19200_e23421_d_n8, assign19200_e23421_d_n10, assign19200_e23421_d_n11, assign19200_e23421_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19200_e23419: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign19200_e23419, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn12)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn8, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn12,)
    }
};
        locals.var_cfs1 = assign19200_e23421;
        locals.var_cfs1_dn0 = assign19200_e23421_d_n0;
        locals.var_cfs1_dn2 = assign19200_e23421_d_n2;
        locals.var_cfs1_dn4 = assign19200_e23421_d_n4;
        locals.var_cfs1_dn5 = assign19200_e23421_d_n5;
        locals.var_cfs1_dn6 = assign19200_e23421_d_n6;
        locals.var_cfs1_dn8 = assign19200_e23421_d_n8;
        locals.var_cfs1_dn10 = assign19200_e23421_d_n10;
        locals.var_cfs1_dn11 = assign19200_e23421_d_n11;
        locals.var_cfs1_dn12 = assign19200_e23421_d_n12;
        locals.var_cfs1_rv = 0.0;

        let (assign19210_e23430,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign19210_e23430;
        locals.var_flg_conv_rv = 0.0;

        let (assign19220_e23439,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign19220_e23439;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_77(
        locals: &mut StampLocals,
    ) {
        let mut assign19230_loop_guard: usize = 0;
        while {
            let assign19230_cond_e23449: f64 = (40.0 + 1.0);
            let assign19230_cond_e23451: f64 = if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_lp_s0 <= assign19230_cond_e23449)) { 1.0 } else { 0.0 };
            assign19230_cond_e23451 != 0.0
        } {
            assign19230_loop_guard += 1;
            assert!(assign19230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign19230_body0_e23464, assign19230_body0_e23464_d_n0, assign19230_body0_e23464_d_n2, assign19230_body0_e23464_d_n4, assign19230_body0_e23464_d_n5, assign19230_body0_e23464_d_n6, assign19230_body0_e23464_d_n8, assign19230_body0_e23464_d_n10, assign19230_body0_e23464_d_n11, assign19230_body0_e23464_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19230_body0_e23461: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        let assign19230_body0_e23462: f64 = (locals.var_beta * assign19230_body0_e23461);
        (assign19230_body0_e23462, (locals.var_beta * (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign19230_body0_e23461) + (locals.var_beta * (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0ld_dn12 + locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
            locals.var_chi = assign19230_body0_e23464;
            locals.var_chi_dn0 = assign19230_body0_e23464_d_n0;
            locals.var_chi_dn2 = assign19230_body0_e23464_d_n2;
            locals.var_chi_dn4 = assign19230_body0_e23464_d_n4;
            locals.var_chi_dn5 = assign19230_body0_e23464_d_n5;
            locals.var_chi_dn6 = assign19230_body0_e23464_d_n6;
            locals.var_chi_dn8 = assign19230_body0_e23464_d_n8;
            locals.var_chi_dn10 = assign19230_body0_e23464_d_n10;
            locals.var_chi_dn11 = assign19230_body0_e23464_d_n11;
            locals.var_chi_dn12 = assign19230_body0_e23464_d_n12;
            locals.var_chi_rv = 0.0;
            let assign19230_body1_e23467: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard337 = assign19230_body1_e23467;
            locals.var_guard337_rv = 0.0;
            let (assign19230_body2_e23493, assign19230_body2_e23493_d_n0, assign19230_body2_e23493_d_n2, assign19230_body2_e23493_d_n4, assign19230_body2_e23493_d_n5, assign19230_body2_e23493_d_n6, assign19230_body2_e23493_d_n8, assign19230_body2_e23493_d_n10, assign19230_body2_e23493_d_n11, assign19230_body2_e23493_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign19230_body2_e23478: f64 = (locals.var_chi * locals.var_chi);
        let assign19230_body2_e23480: f64 = (assign19230_body2_e23478 * locals.var_chi);
        let assign19230_body2_e23484: f64 = (-0.07053654284009761);
        let assign19230_body2_e23487: f64 = (locals.var_chi * 0.006115288895133179);
        let assign19230_body2_e23488: f64 = (assign19230_body2_e23484 + assign19230_body2_e23487);
        let assign19230_body2_e23489: f64 = (locals.var_chi * assign19230_body2_e23488);
        let assign19230_body2_e23490: f64 = (0.29693154855771 + assign19230_body2_e23489);
        let assign19230_body2_e23491: f64 = (assign19230_body2_e23480 * assign19230_body2_e23490);
        (assign19230_body2_e23491, ((((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * locals.var_chi) + (assign19230_body2_e23478 * locals.var_chi_dn0)) * assign19230_body2_e23490) + (assign19230_body2_e23480 * ((locals.var_chi_dn0 * assign19230_body2_e23488) + (locals.var_chi * (locals.var_chi_dn0 * 0.006115288895133179))))), ((((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * locals.var_chi) + (assign19230_body2_e23478 * locals.var_chi_dn2)) * assign19230_body2_e23490) + (assign19230_body2_e23480 * ((locals.var_chi_dn2 * assign19230_body2_e23488) + (locals.var_chi * (locals.var_chi_dn2 * 0.006115288895133179))))), ((((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * locals.var_chi) + (assign19230_body2_e23478 * locals.var_chi_dn4)) * assign19230_body2_e23490) + (assign19230_body2_e23480 * ((locals.var_chi_dn4 * assign19230_body2_e23488) + (locals.var_chi * (locals.var_chi_dn4 * 0.006115288895133179))))), ((((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * locals.var_chi) + (assign19230_body2_e23478 * locals.var_chi_dn5)) * assign19230_body2_e23490) + (assign19230_body2_e23480 * ((locals.var_chi_dn5 * assign19230_body2_e23488) + (locals.var_chi * (locals.var_chi_dn5 * 0.006115288895133179))))), ((((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * locals.var_chi) + (assign19230_body2_e23478 * locals.var_chi_dn6)) * assign19230_body2_e23490) + (assign19230_body2_e23480 * ((locals.var_chi_dn6 * assign19230_body2_e23488) + (locals.var_chi * (locals.var_chi_dn6 * 0.006115288895133179))))), ((((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * locals.var_chi) + (assign19230_body2_e23478 * locals.var_chi_dn8)) * assign19230_body2_e23490) + (assign19230_body2_e23480 * ((locals.var_chi_dn8 * assign19230_body2_e23488) + (locals.var_chi * (locals.var_chi_dn8 * 0.006115288895133179))))), ((((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * locals.var_chi) + (assign19230_body2_e23478 * locals.var_chi_dn10)) * assign19230_body2_e23490) + (assign19230_body2_e23480 * ((locals.var_chi_dn10 * assign19230_body2_e23488) + (locals.var_chi * (locals.var_chi_dn10 * 0.006115288895133179))))), ((((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * locals.var_chi) + (assign19230_body2_e23478 * locals.var_chi_dn11)) * assign19230_body2_e23490) + (assign19230_body2_e23480 * ((locals.var_chi_dn11 * assign19230_body2_e23488) + (locals.var_chi * (locals.var_chi_dn11 * 0.006115288895133179))))), ((((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) * locals.var_chi) + (assign19230_body2_e23478 * locals.var_chi_dn12)) * assign19230_body2_e23490) + (assign19230_body2_e23480 * ((locals.var_chi_dn12 * assign19230_body2_e23488) + (locals.var_chi * (locals.var_chi_dn12 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn4, locals.var_fi_dn5, locals.var_fi_dn6, locals.var_fi_dn8, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn12,)
    }
};
            locals.var_fi = assign19230_body2_e23493;
            locals.var_fi_dn0 = assign19230_body2_e23493_d_n0;
            locals.var_fi_dn2 = assign19230_body2_e23493_d_n2;
            locals.var_fi_dn4 = assign19230_body2_e23493_d_n4;
            locals.var_fi_dn5 = assign19230_body2_e23493_d_n5;
            locals.var_fi_dn6 = assign19230_body2_e23493_d_n6;
            locals.var_fi_dn8 = assign19230_body2_e23493_d_n8;
            locals.var_fi_dn10 = assign19230_body2_e23493_d_n10;
            locals.var_fi_dn11 = assign19230_body2_e23493_d_n11;
            locals.var_fi_dn12 = assign19230_body2_e23493_d_n12;
            locals.var_fi_rv = 0.0;
            let (assign19230_body3_e23523, assign19230_body3_e23523_d_n0, assign19230_body3_e23523_d_n2, assign19230_body3_e23523_d_n4, assign19230_body3_e23523_d_n5, assign19230_body3_e23523_d_n6, assign19230_body3_e23523_d_n8, assign19230_body3_e23523_d_n10, assign19230_body3_e23523_d_n11, assign19230_body3_e23523_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign19230_body3_e23504: f64 = (locals.var_chi * locals.var_chi);
        let assign19230_body3_e23507: f64 = (3.0 * 0.29693154855771);
        let assign19230_body3_e23511: f64 = (-0.07053654284009761);
        let assign19230_body3_e23512: f64 = (4.0 * assign19230_body3_e23511);
        let assign19230_body3_e23515: f64 = (locals.var_chi * 5.0);
        let assign19230_body3_e23517: f64 = (assign19230_body3_e23515 * 0.006115288895133179);
        let assign19230_body3_e23518: f64 = (assign19230_body3_e23512 + assign19230_body3_e23517);
        let assign19230_body3_e23519: f64 = (locals.var_chi * assign19230_body3_e23518);
        let assign19230_body3_e23520: f64 = (assign19230_body3_e23507 + assign19230_body3_e23519);
        let assign19230_body3_e23521: f64 = (assign19230_body3_e23504 * assign19230_body3_e23520);
        (assign19230_body3_e23521, ((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * assign19230_body3_e23520) + (assign19230_body3_e23504 * ((locals.var_chi_dn0 * assign19230_body3_e23518) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * assign19230_body3_e23520) + (assign19230_body3_e23504 * ((locals.var_chi_dn2 * assign19230_body3_e23518) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * assign19230_body3_e23520) + (assign19230_body3_e23504 * ((locals.var_chi_dn4 * assign19230_body3_e23518) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * assign19230_body3_e23520) + (assign19230_body3_e23504 * ((locals.var_chi_dn5 * assign19230_body3_e23518) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * assign19230_body3_e23520) + (assign19230_body3_e23504 * ((locals.var_chi_dn6 * assign19230_body3_e23518) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * assign19230_body3_e23520) + (assign19230_body3_e23504 * ((locals.var_chi_dn8 * assign19230_body3_e23518) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * assign19230_body3_e23520) + (assign19230_body3_e23504 * ((locals.var_chi_dn10 * assign19230_body3_e23518) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * assign19230_body3_e23520) + (assign19230_body3_e23504 * ((locals.var_chi_dn11 * assign19230_body3_e23518) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) * assign19230_body3_e23520) + (assign19230_body3_e23504 * ((locals.var_chi_dn12 * assign19230_body3_e23518) + (locals.var_chi * ((locals.var_chi_dn12 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn4, locals.var_fi_dchi_dn5, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn8, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn12,)
    }
};
            locals.var_fi_dchi = assign19230_body3_e23523;
            locals.var_fi_dchi_dn0 = assign19230_body3_e23523_d_n0;
            locals.var_fi_dchi_dn2 = assign19230_body3_e23523_d_n2;
            locals.var_fi_dchi_dn4 = assign19230_body3_e23523_d_n4;
            locals.var_fi_dchi_dn5 = assign19230_body3_e23523_d_n5;
            locals.var_fi_dchi_dn6 = assign19230_body3_e23523_d_n6;
            locals.var_fi_dchi_dn8 = assign19230_body3_e23523_d_n8;
            locals.var_fi_dchi_dn10 = assign19230_body3_e23523_d_n10;
            locals.var_fi_dchi_dn11 = assign19230_body3_e23523_d_n11;
            locals.var_fi_dchi_dn12 = assign19230_body3_e23523_d_n12;
            locals.var_fi_dchi_rv = 0.0;
            let (assign19230_body4_e23538, assign19230_body4_e23538_d_n0, assign19230_body4_e23538_d_n2, assign19230_body4_e23538_d_n4, assign19230_body4_e23538_d_n5, assign19230_body4_e23538_d_n6, assign19230_body4_e23538_d_n8, assign19230_body4_e23538_d_n10, assign19230_body4_e23538_d_n11, assign19230_body4_e23538_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign19230_body4_e23534: f64 = (locals.var_cfs1 * locals.var_fi);
        let assign19230_body4_e23536: f64 = (assign19230_body4_e23534 * locals.var_fi);
        (assign19230_body4_e23536, ((((locals.var_cfs1_dn0 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn0)) * locals.var_fi) + (assign19230_body4_e23534 * locals.var_fi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn2)) * locals.var_fi) + (assign19230_body4_e23534 * locals.var_fi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn4)) * locals.var_fi) + (assign19230_body4_e23534 * locals.var_fi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn5)) * locals.var_fi) + (assign19230_body4_e23534 * locals.var_fi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn6)) * locals.var_fi) + (assign19230_body4_e23534 * locals.var_fi_dn6)), ((((locals.var_cfs1_dn8 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn8)) * locals.var_fi) + (assign19230_body4_e23534 * locals.var_fi_dn8)), ((((locals.var_cfs1_dn10 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn10)) * locals.var_fi) + (assign19230_body4_e23534 * locals.var_fi_dn10)), ((((locals.var_cfs1_dn11 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn11)) * locals.var_fi) + (assign19230_body4_e23534 * locals.var_fi_dn11)), ((((locals.var_cfs1_dn12 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn12)) * locals.var_fi) + (assign19230_body4_e23534 * locals.var_fi_dn12)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn8, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12,)
    }
};
            locals.var_fs01 = assign19230_body4_e23538;
            locals.var_fs01_dn0 = assign19230_body4_e23538_d_n0;
            locals.var_fs01_dn2 = assign19230_body4_e23538_d_n2;
            locals.var_fs01_dn4 = assign19230_body4_e23538_d_n4;
            locals.var_fs01_dn5 = assign19230_body4_e23538_d_n5;
            locals.var_fs01_dn6 = assign19230_body4_e23538_d_n6;
            locals.var_fs01_dn8 = assign19230_body4_e23538_d_n8;
            locals.var_fs01_dn10 = assign19230_body4_e23538_d_n10;
            locals.var_fs01_dn11 = assign19230_body4_e23538_d_n11;
            locals.var_fs01_dn12 = assign19230_body4_e23538_d_n12;
            locals.var_fs01_rv = 0.0;
            let (assign19230_body5_e23557, assign19230_body5_e23557_d_n0, assign19230_body5_e23557_d_n2, assign19230_body5_e23557_d_n4, assign19230_body5_e23557_d_n5, assign19230_body5_e23557_d_n6, assign19230_body5_e23557_d_n8, assign19230_body5_e23557_d_n10, assign19230_body5_e23557_d_n11, assign19230_body5_e23557_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign19230_body5_e23549: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign19230_body5_e23551: f64 = (assign19230_body5_e23549 * 2.0);
        let assign19230_body5_e23553: f64 = (assign19230_body5_e23551 * locals.var_fi);
        let assign19230_body5_e23555: f64 = (assign19230_body5_e23553 * locals.var_fi_dchi);
        (assign19230_body5_e23555, ((((((locals.var_cfs1_dn0 * locals.var_beta) * 2.0) * locals.var_fi) + (assign19230_body5_e23551 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign19230_body5_e23553 * locals.var_fi_dchi_dn0)), ((((((locals.var_cfs1_dn2 * locals.var_beta) * 2.0) * locals.var_fi) + (assign19230_body5_e23551 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign19230_body5_e23553 * locals.var_fi_dchi_dn2)), (((((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * 2.0) * locals.var_fi) + (assign19230_body5_e23551 * locals.var_fi_dn4)) * locals.var_fi_dchi) + (assign19230_body5_e23553 * locals.var_fi_dchi_dn4)), ((((((locals.var_cfs1_dn5 * locals.var_beta) * 2.0) * locals.var_fi) + (assign19230_body5_e23551 * locals.var_fi_dn5)) * locals.var_fi_dchi) + (assign19230_body5_e23553 * locals.var_fi_dchi_dn5)), ((((((locals.var_cfs1_dn6 * locals.var_beta) * 2.0) * locals.var_fi) + (assign19230_body5_e23551 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign19230_body5_e23553 * locals.var_fi_dchi_dn6)), ((((((locals.var_cfs1_dn8 * locals.var_beta) * 2.0) * locals.var_fi) + (assign19230_body5_e23551 * locals.var_fi_dn8)) * locals.var_fi_dchi) + (assign19230_body5_e23553 * locals.var_fi_dchi_dn8)), ((((((locals.var_cfs1_dn10 * locals.var_beta) * 2.0) * locals.var_fi) + (assign19230_body5_e23551 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign19230_body5_e23553 * locals.var_fi_dchi_dn10)), ((((((locals.var_cfs1_dn11 * locals.var_beta) * 2.0) * locals.var_fi) + (assign19230_body5_e23551 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign19230_body5_e23553 * locals.var_fi_dchi_dn11)), ((((((locals.var_cfs1_dn12 * locals.var_beta) * 2.0) * locals.var_fi) + (assign19230_body5_e23551 * locals.var_fi_dn12)) * locals.var_fi_dchi) + (assign19230_body5_e23553 * locals.var_fi_dchi_dn12)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12,)
    }
};
            locals.var_fs01_dps0 = assign19230_body5_e23557;
            locals.var_fs01_dps0_dn0 = assign19230_body5_e23557_d_n0;
            locals.var_fs01_dps0_dn2 = assign19230_body5_e23557_d_n2;
            locals.var_fs01_dps0_dn4 = assign19230_body5_e23557_d_n4;
            locals.var_fs01_dps0_dn5 = assign19230_body5_e23557_d_n5;
            locals.var_fs01_dps0_dn6 = assign19230_body5_e23557_d_n6;
            locals.var_fs01_dps0_dn8 = assign19230_body5_e23557_d_n8;
            locals.var_fs01_dps0_dn10 = assign19230_body5_e23557_d_n10;
            locals.var_fs01_dps0_dn11 = assign19230_body5_e23557_d_n11;
            locals.var_fs01_dps0_dn12 = assign19230_body5_e23557_d_n12;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign19230_body6_e23588, assign19230_body6_e23588_d_n0, assign19230_body6_e23588_d_n2, assign19230_body6_e23588_d_n4, assign19230_body6_e23588_d_n5, assign19230_body6_e23588_d_n6, assign19230_body6_e23588_d_n8, assign19230_body6_e23588_d_n10, assign19230_body6_e23588_d_n11, assign19230_body6_e23588_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign19230_body6_e23570: f64 = (-0.117851130197758);
        let assign19230_body6_e23575: f64 = (-0.00163730162779191);
        let assign19230_body6_e23578: f64 = (locals.var_chi * 6.36964918866352e-5);
        let assign19230_body6_e23579: f64 = (assign19230_body6_e23575 + assign19230_body6_e23578);
        let assign19230_body6_e23580: f64 = (locals.var_chi * assign19230_body6_e23579);
        let assign19230_body6_e23581: f64 = (0.0178800506338833 + assign19230_body6_e23580);
        let assign19230_body6_e23582: f64 = (locals.var_chi * assign19230_body6_e23581);
        let assign19230_body6_e23583: f64 = (assign19230_body6_e23570 + assign19230_body6_e23582);
        let assign19230_body6_e23584: f64 = (locals.var_chi * assign19230_body6_e23583);
        let assign19230_body6_e23585: f64 = (0.707106781186548 + assign19230_body6_e23584);
        let assign19230_body6_e23586: f64 = (locals.var_chi * assign19230_body6_e23585);
        (assign19230_body6_e23586, ((locals.var_chi_dn0 * assign19230_body6_e23585) + (locals.var_chi * ((locals.var_chi_dn0 * assign19230_body6_e23583) + (locals.var_chi * ((locals.var_chi_dn0 * assign19230_body6_e23581) + (locals.var_chi * ((locals.var_chi_dn0 * assign19230_body6_e23579) + (locals.var_chi * (locals.var_chi_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn2 * assign19230_body6_e23585) + (locals.var_chi * ((locals.var_chi_dn2 * assign19230_body6_e23583) + (locals.var_chi * ((locals.var_chi_dn2 * assign19230_body6_e23581) + (locals.var_chi * ((locals.var_chi_dn2 * assign19230_body6_e23579) + (locals.var_chi * (locals.var_chi_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn4 * assign19230_body6_e23585) + (locals.var_chi * ((locals.var_chi_dn4 * assign19230_body6_e23583) + (locals.var_chi * ((locals.var_chi_dn4 * assign19230_body6_e23581) + (locals.var_chi * ((locals.var_chi_dn4 * assign19230_body6_e23579) + (locals.var_chi * (locals.var_chi_dn4 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn5 * assign19230_body6_e23585) + (locals.var_chi * ((locals.var_chi_dn5 * assign19230_body6_e23583) + (locals.var_chi * ((locals.var_chi_dn5 * assign19230_body6_e23581) + (locals.var_chi * ((locals.var_chi_dn5 * assign19230_body6_e23579) + (locals.var_chi * (locals.var_chi_dn5 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn6 * assign19230_body6_e23585) + (locals.var_chi * ((locals.var_chi_dn6 * assign19230_body6_e23583) + (locals.var_chi * ((locals.var_chi_dn6 * assign19230_body6_e23581) + (locals.var_chi * ((locals.var_chi_dn6 * assign19230_body6_e23579) + (locals.var_chi * (locals.var_chi_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn8 * assign19230_body6_e23585) + (locals.var_chi * ((locals.var_chi_dn8 * assign19230_body6_e23583) + (locals.var_chi * ((locals.var_chi_dn8 * assign19230_body6_e23581) + (locals.var_chi * ((locals.var_chi_dn8 * assign19230_body6_e23579) + (locals.var_chi * (locals.var_chi_dn8 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn10 * assign19230_body6_e23585) + (locals.var_chi * ((locals.var_chi_dn10 * assign19230_body6_e23583) + (locals.var_chi * ((locals.var_chi_dn10 * assign19230_body6_e23581) + (locals.var_chi * ((locals.var_chi_dn10 * assign19230_body6_e23579) + (locals.var_chi * (locals.var_chi_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn11 * assign19230_body6_e23585) + (locals.var_chi * ((locals.var_chi_dn11 * assign19230_body6_e23583) + (locals.var_chi * ((locals.var_chi_dn11 * assign19230_body6_e23581) + (locals.var_chi * ((locals.var_chi_dn11 * assign19230_body6_e23579) + (locals.var_chi * (locals.var_chi_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn12 * assign19230_body6_e23585) + (locals.var_chi * ((locals.var_chi_dn12 * assign19230_body6_e23583) + (locals.var_chi * ((locals.var_chi_dn12 * assign19230_body6_e23581) + (locals.var_chi * ((locals.var_chi_dn12 * assign19230_body6_e23579) + (locals.var_chi * (locals.var_chi_dn12 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn8, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12,)
    }
};
            locals.var_fb = assign19230_body6_e23588;
            locals.var_fb_dn0 = assign19230_body6_e23588_d_n0;
            locals.var_fb_dn2 = assign19230_body6_e23588_d_n2;
            locals.var_fb_dn4 = assign19230_body6_e23588_d_n4;
            locals.var_fb_dn5 = assign19230_body6_e23588_d_n5;
            locals.var_fb_dn6 = assign19230_body6_e23588_d_n6;
            locals.var_fb_dn8 = assign19230_body6_e23588_d_n8;
            locals.var_fb_dn10 = assign19230_body6_e23588_d_n10;
            locals.var_fb_dn11 = assign19230_body6_e23588_d_n11;
            locals.var_fb_dn12 = assign19230_body6_e23588_d_n12;
            locals.var_fb_rv = 0.0;
            let (assign19230_body7_e23625, assign19230_body7_e23625_d_n0, assign19230_body7_e23625_d_n2, assign19230_body7_e23625_d_n4, assign19230_body7_e23625_d_n5, assign19230_body7_e23625_d_n6, assign19230_body7_e23625_d_n8, assign19230_body7_e23625_d_n10, assign19230_body7_e23625_d_n11, assign19230_body7_e23625_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign19230_body7_e23601: f64 = (-0.117851130197758);
        let assign19230_body7_e23602: f64 = (2.0 * assign19230_body7_e23601);
        let assign19230_body7_e23606: f64 = (3.0 * 0.0178800506338833);
        let assign19230_body7_e23610: f64 = (-0.00163730162779191);
        let assign19230_body7_e23611: f64 = (4.0 * assign19230_body7_e23610);
        let assign19230_body7_e23614: f64 = (locals.var_chi * 5.0);
        let assign19230_body7_e23616: f64 = (assign19230_body7_e23614 * 6.36964918866352e-5);
        let assign19230_body7_e23617: f64 = (assign19230_body7_e23611 + assign19230_body7_e23616);
        let assign19230_body7_e23618: f64 = (locals.var_chi * assign19230_body7_e23617);
        let assign19230_body7_e23619: f64 = (assign19230_body7_e23606 + assign19230_body7_e23618);
        let assign19230_body7_e23620: f64 = (locals.var_chi * assign19230_body7_e23619);
        let assign19230_body7_e23621: f64 = (assign19230_body7_e23602 + assign19230_body7_e23620);
        let assign19230_body7_e23622: f64 = (locals.var_chi * assign19230_body7_e23621);
        let assign19230_body7_e23623: f64 = (0.707106781186548 + assign19230_body7_e23622);
        (assign19230_body7_e23623, ((locals.var_chi_dn0 * assign19230_body7_e23621) + (locals.var_chi * ((locals.var_chi_dn0 * assign19230_body7_e23619) + (locals.var_chi * ((locals.var_chi_dn0 * assign19230_body7_e23617) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn2 * assign19230_body7_e23621) + (locals.var_chi * ((locals.var_chi_dn2 * assign19230_body7_e23619) + (locals.var_chi * ((locals.var_chi_dn2 * assign19230_body7_e23617) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn4 * assign19230_body7_e23621) + (locals.var_chi * ((locals.var_chi_dn4 * assign19230_body7_e23619) + (locals.var_chi * ((locals.var_chi_dn4 * assign19230_body7_e23617) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn5 * assign19230_body7_e23621) + (locals.var_chi * ((locals.var_chi_dn5 * assign19230_body7_e23619) + (locals.var_chi * ((locals.var_chi_dn5 * assign19230_body7_e23617) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn6 * assign19230_body7_e23621) + (locals.var_chi * ((locals.var_chi_dn6 * assign19230_body7_e23619) + (locals.var_chi * ((locals.var_chi_dn6 * assign19230_body7_e23617) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn8 * assign19230_body7_e23621) + (locals.var_chi * ((locals.var_chi_dn8 * assign19230_body7_e23619) + (locals.var_chi * ((locals.var_chi_dn8 * assign19230_body7_e23617) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn10 * assign19230_body7_e23621) + (locals.var_chi * ((locals.var_chi_dn10 * assign19230_body7_e23619) + (locals.var_chi * ((locals.var_chi_dn10 * assign19230_body7_e23617) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn11 * assign19230_body7_e23621) + (locals.var_chi * ((locals.var_chi_dn11 * assign19230_body7_e23619) + (locals.var_chi * ((locals.var_chi_dn11 * assign19230_body7_e23617) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn12 * assign19230_body7_e23621) + (locals.var_chi * ((locals.var_chi_dn12 * assign19230_body7_e23619) + (locals.var_chi * ((locals.var_chi_dn12 * assign19230_body7_e23617) + (locals.var_chi * ((locals.var_chi_dn12 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn4, locals.var_fb_dchi_dn5, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn8, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn12,)
    }
};
            locals.var_fb_dchi = assign19230_body7_e23625;
            locals.var_fb_dchi_dn0 = assign19230_body7_e23625_d_n0;
            locals.var_fb_dchi_dn2 = assign19230_body7_e23625_d_n2;
            locals.var_fb_dchi_dn4 = assign19230_body7_e23625_d_n4;
            locals.var_fb_dchi_dn5 = assign19230_body7_e23625_d_n5;
            locals.var_fb_dchi_dn6 = assign19230_body7_e23625_d_n6;
            locals.var_fb_dchi_dn8 = assign19230_body7_e23625_d_n8;
            locals.var_fb_dchi_dn10 = assign19230_body7_e23625_d_n10;
            locals.var_fb_dchi_dn11 = assign19230_body7_e23625_d_n11;
            locals.var_fb_dchi_dn12 = assign19230_body7_e23625_d_n12;
            locals.var_fb_dchi_rv = 0.0;
            let (assign19230_body8_e23643, assign19230_body8_e23643_d_n0, assign19230_body8_e23643_d_n2, assign19230_body8_e23643_d_n4, assign19230_body8_e23643_d_n5, assign19230_body8_e23643_d_n6, assign19230_body8_e23643_d_n8, assign19230_body8_e23643_d_n10, assign19230_body8_e23643_d_n11, assign19230_body8_e23643_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign19230_body8_e23636: f64 = (locals.var_fb * locals.var_fb);
        let assign19230_body8_e23638: f64 = (assign19230_body8_e23636 + locals.var_fs01);
        let assign19230_body8_e23640: f64 = (assign19230_body8_e23638 + 1e-50);
        let assign19230_body8_e23641: f64 = (assign19230_body8_e23640).sqrt();
        (assign19230_body8_e23641, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign19230_body8_e23641)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign19230_body8_e23641)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign19230_body8_e23641)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign19230_body8_e23641)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign19230_body8_e23641)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign19230_body8_e23641)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign19230_body8_e23641)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign19230_body8_e23641)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fs01_dn12) / (2.0 * assign19230_body8_e23641)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn8, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12,)
    }
};
            locals.var_fs02 = assign19230_body8_e23643;
            locals.var_fs02_dn0 = assign19230_body8_e23643_d_n0;
            locals.var_fs02_dn2 = assign19230_body8_e23643_d_n2;
            locals.var_fs02_dn4 = assign19230_body8_e23643_d_n4;
            locals.var_fs02_dn5 = assign19230_body8_e23643_d_n5;
            locals.var_fs02_dn6 = assign19230_body8_e23643_d_n6;
            locals.var_fs02_dn8 = assign19230_body8_e23643_d_n8;
            locals.var_fs02_dn10 = assign19230_body8_e23643_d_n10;
            locals.var_fs02_dn11 = assign19230_body8_e23643_d_n11;
            locals.var_fs02_dn12 = assign19230_body8_e23643_d_n12;
            locals.var_fs02_rv = 0.0;
            let (assign19230_body9_e23666, assign19230_body9_e23666_d_n0, assign19230_body9_e23666_d_n2, assign19230_body9_e23666_d_n4, assign19230_body9_e23666_d_n5, assign19230_body9_e23666_d_n6, assign19230_body9_e23666_d_n8, assign19230_body9_e23666_d_n10, assign19230_body9_e23666_d_n11, assign19230_body9_e23666_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign19230_body9_e23654: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign19230_body9_e23656: f64 = (assign19230_body9_e23654 * 2.0);
        let assign19230_body9_e23658: f64 = (assign19230_body9_e23656 * locals.var_fb);
        let assign19230_body9_e23660: f64 = (assign19230_body9_e23658 + locals.var_fs01_dps0);
        let assign19230_body9_e23663: f64 = (locals.var_fs02 + locals.var_fs02);
        let assign19230_body9_e23664: f64 = (assign19230_body9_e23660 / assign19230_body9_e23663);
        (assign19230_body9_e23664, ((((((((locals.var_beta * locals.var_fb_dchi_dn0) * 2.0) * locals.var_fb) + (assign19230_body9_e23656 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0) * assign19230_body9_e23663) - (assign19230_body9_e23660 * (locals.var_fs02_dn0 + locals.var_fs02_dn0))) / (assign19230_body9_e23663 * assign19230_body9_e23663)), ((((((((locals.var_beta * locals.var_fb_dchi_dn2) * 2.0) * locals.var_fb) + (assign19230_body9_e23656 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2) * assign19230_body9_e23663) - (assign19230_body9_e23660 * (locals.var_fs02_dn2 + locals.var_fs02_dn2))) / (assign19230_body9_e23663 * assign19230_body9_e23663)), (((((((((locals.var_beta_dn4 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn4)) * 2.0) * locals.var_fb) + (assign19230_body9_e23656 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4) * assign19230_body9_e23663) - (assign19230_body9_e23660 * (locals.var_fs02_dn4 + locals.var_fs02_dn4))) / (assign19230_body9_e23663 * assign19230_body9_e23663)), ((((((((locals.var_beta * locals.var_fb_dchi_dn5) * 2.0) * locals.var_fb) + (assign19230_body9_e23656 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5) * assign19230_body9_e23663) - (assign19230_body9_e23660 * (locals.var_fs02_dn5 + locals.var_fs02_dn5))) / (assign19230_body9_e23663 * assign19230_body9_e23663)), ((((((((locals.var_beta * locals.var_fb_dchi_dn6) * 2.0) * locals.var_fb) + (assign19230_body9_e23656 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6) * assign19230_body9_e23663) - (assign19230_body9_e23660 * (locals.var_fs02_dn6 + locals.var_fs02_dn6))) / (assign19230_body9_e23663 * assign19230_body9_e23663)), ((((((((locals.var_beta * locals.var_fb_dchi_dn8) * 2.0) * locals.var_fb) + (assign19230_body9_e23656 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8) * assign19230_body9_e23663) - (assign19230_body9_e23660 * (locals.var_fs02_dn8 + locals.var_fs02_dn8))) / (assign19230_body9_e23663 * assign19230_body9_e23663)), ((((((((locals.var_beta * locals.var_fb_dchi_dn10) * 2.0) * locals.var_fb) + (assign19230_body9_e23656 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10) * assign19230_body9_e23663) - (assign19230_body9_e23660 * (locals.var_fs02_dn10 + locals.var_fs02_dn10))) / (assign19230_body9_e23663 * assign19230_body9_e23663)), ((((((((locals.var_beta * locals.var_fb_dchi_dn11) * 2.0) * locals.var_fb) + (assign19230_body9_e23656 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11) * assign19230_body9_e23663) - (assign19230_body9_e23660 * (locals.var_fs02_dn11 + locals.var_fs02_dn11))) / (assign19230_body9_e23663 * assign19230_body9_e23663)), ((((((((locals.var_beta * locals.var_fb_dchi_dn12) * 2.0) * locals.var_fb) + (assign19230_body9_e23656 * locals.var_fb_dn12)) + locals.var_fs01_dps0_dn12) * assign19230_body9_e23663) - (assign19230_body9_e23660 * (locals.var_fs02_dn12 + locals.var_fs02_dn12))) / (assign19230_body9_e23663 * assign19230_body9_e23663)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12,)
    }
};
            locals.var_fs02_dps0 = assign19230_body9_e23666;
            locals.var_fs02_dps0_dn0 = assign19230_body9_e23666_d_n0;
            locals.var_fs02_dps0_dn2 = assign19230_body9_e23666_d_n2;
            locals.var_fs02_dps0_dn4 = assign19230_body9_e23666_d_n4;
            locals.var_fs02_dps0_dn5 = assign19230_body9_e23666_d_n5;
            locals.var_fs02_dps0_dn6 = assign19230_body9_e23666_d_n6;
            locals.var_fs02_dps0_dn8 = assign19230_body9_e23666_d_n8;
            locals.var_fs02_dps0_dn10 = assign19230_body9_e23666_d_n10;
            locals.var_fs02_dps0_dn11 = assign19230_body9_e23666_d_n11;
            locals.var_fs02_dps0_dn12 = assign19230_body9_e23666_d_n12;
            locals.var_fs02_dps0_rv = 0.0;
            let assign19230_body10_e23669: f64 = if locals.var_chi < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard338 = assign19230_body10_e23669;
            locals.var_guard338_rv = 0.0;
            let (assign19230_body11_e23684, assign19230_body11_e23684_d_n0, assign19230_body11_e23684_d_n2, assign19230_body11_e23684_d_n4, assign19230_body11_e23684_d_n5, assign19230_body11_e23684_d_n6, assign19230_body11_e23684_d_n8, assign19230_body11_e23684_d_n10, assign19230_body11_e23684_d_n11, assign19230_body11_e23684_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign19230_body11_e23682: f64 = (locals.var_chi).exp();
        (assign19230_body11_e23682, (assign19230_body11_e23682 * locals.var_chi_dn0), (assign19230_body11_e23682 * locals.var_chi_dn2), (assign19230_body11_e23682 * locals.var_chi_dn4), (assign19230_body11_e23682 * locals.var_chi_dn5), (assign19230_body11_e23682 * locals.var_chi_dn6), (assign19230_body11_e23682 * locals.var_chi_dn8), (assign19230_body11_e23682 * locals.var_chi_dn10), (assign19230_body11_e23682 * locals.var_chi_dn11), (assign19230_body11_e23682 * locals.var_chi_dn12),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn8, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12,)
    }
};
            locals.var_exp_chi = assign19230_body11_e23684;
            locals.var_exp_chi_dn0 = assign19230_body11_e23684_d_n0;
            locals.var_exp_chi_dn2 = assign19230_body11_e23684_d_n2;
            locals.var_exp_chi_dn4 = assign19230_body11_e23684_d_n4;
            locals.var_exp_chi_dn5 = assign19230_body11_e23684_d_n5;
            locals.var_exp_chi_dn6 = assign19230_body11_e23684_d_n6;
            locals.var_exp_chi_dn8 = assign19230_body11_e23684_d_n8;
            locals.var_exp_chi_dn10 = assign19230_body11_e23684_d_n10;
            locals.var_exp_chi_dn11 = assign19230_body11_e23684_d_n11;
            locals.var_exp_chi_dn12 = assign19230_body11_e23684_d_n12;
            locals.var_exp_chi_rv = 0.0;
            let (assign19230_body12_e23702, assign19230_body12_e23702_d_n0, assign19230_body12_e23702_d_n2, assign19230_body12_e23702_d_n4, assign19230_body12_e23702_d_n5, assign19230_body12_e23702_d_n6, assign19230_body12_e23702_d_n8, assign19230_body12_e23702_d_n10, assign19230_body12_e23702_d_n11, assign19230_body12_e23702_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign19230_body12_e23699: f64 = (locals.var_exp_chi - 1.0);
        let assign19230_body12_e23700: f64 = (locals.var_cfs1 * assign19230_body12_e23699);
        (assign19230_body12_e23700, ((locals.var_cfs1_dn0 * assign19230_body12_e23699) + (locals.var_cfs1 * locals.var_exp_chi_dn0)), ((locals.var_cfs1_dn2 * assign19230_body12_e23699) + (locals.var_cfs1 * locals.var_exp_chi_dn2)), ((locals.var_cfs1_dn4 * assign19230_body12_e23699) + (locals.var_cfs1 * locals.var_exp_chi_dn4)), ((locals.var_cfs1_dn5 * assign19230_body12_e23699) + (locals.var_cfs1 * locals.var_exp_chi_dn5)), ((locals.var_cfs1_dn6 * assign19230_body12_e23699) + (locals.var_cfs1 * locals.var_exp_chi_dn6)), ((locals.var_cfs1_dn8 * assign19230_body12_e23699) + (locals.var_cfs1 * locals.var_exp_chi_dn8)), ((locals.var_cfs1_dn10 * assign19230_body12_e23699) + (locals.var_cfs1 * locals.var_exp_chi_dn10)), ((locals.var_cfs1_dn11 * assign19230_body12_e23699) + (locals.var_cfs1 * locals.var_exp_chi_dn11)), ((locals.var_cfs1_dn12 * assign19230_body12_e23699) + (locals.var_cfs1 * locals.var_exp_chi_dn12)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn8, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12,)
    }
};
            locals.var_fs01 = assign19230_body12_e23702;
            locals.var_fs01_dn0 = assign19230_body12_e23702_d_n0;
            locals.var_fs01_dn2 = assign19230_body12_e23702_d_n2;
            locals.var_fs01_dn4 = assign19230_body12_e23702_d_n4;
            locals.var_fs01_dn5 = assign19230_body12_e23702_d_n5;
            locals.var_fs01_dn6 = assign19230_body12_e23702_d_n6;
            locals.var_fs01_dn8 = assign19230_body12_e23702_d_n8;
            locals.var_fs01_dn10 = assign19230_body12_e23702_d_n10;
            locals.var_fs01_dn11 = assign19230_body12_e23702_d_n11;
            locals.var_fs01_dn12 = assign19230_body12_e23702_d_n12;
            locals.var_fs01_rv = 0.0;
            let (assign19230_body13_e23720, assign19230_body13_e23720_d_n0, assign19230_body13_e23720_d_n2, assign19230_body13_e23720_d_n4, assign19230_body13_e23720_d_n5, assign19230_body13_e23720_d_n6, assign19230_body13_e23720_d_n8, assign19230_body13_e23720_d_n10, assign19230_body13_e23720_d_n11, assign19230_body13_e23720_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign19230_body13_e23716: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign19230_body13_e23718: f64 = (assign19230_body13_e23716 * locals.var_exp_chi);
        (assign19230_body13_e23718, (((locals.var_cfs1_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign19230_body13_e23716 * locals.var_exp_chi_dn0)), (((locals.var_cfs1_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign19230_body13_e23716 * locals.var_exp_chi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_exp_chi) + (assign19230_body13_e23716 * locals.var_exp_chi_dn4)), (((locals.var_cfs1_dn5 * locals.var_beta) * locals.var_exp_chi) + (assign19230_body13_e23716 * locals.var_exp_chi_dn5)), (((locals.var_cfs1_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign19230_body13_e23716 * locals.var_exp_chi_dn6)), (((locals.var_cfs1_dn8 * locals.var_beta) * locals.var_exp_chi) + (assign19230_body13_e23716 * locals.var_exp_chi_dn8)), (((locals.var_cfs1_dn10 * locals.var_beta) * locals.var_exp_chi) + (assign19230_body13_e23716 * locals.var_exp_chi_dn10)), (((locals.var_cfs1_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign19230_body13_e23716 * locals.var_exp_chi_dn11)), (((locals.var_cfs1_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign19230_body13_e23716 * locals.var_exp_chi_dn12)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12,)
    }
};
            locals.var_fs01_dps0 = assign19230_body13_e23720;
            locals.var_fs01_dps0_dn0 = assign19230_body13_e23720_d_n0;
            locals.var_fs01_dps0_dn2 = assign19230_body13_e23720_d_n2;
            locals.var_fs01_dps0_dn4 = assign19230_body13_e23720_d_n4;
            locals.var_fs01_dps0_dn5 = assign19230_body13_e23720_d_n5;
            locals.var_fs01_dps0_dn6 = assign19230_body13_e23720_d_n6;
            locals.var_fs01_dps0_dn8 = assign19230_body13_e23720_d_n8;
            locals.var_fs01_dps0_dn10 = assign19230_body13_e23720_d_n10;
            locals.var_fs01_dps0_dn11 = assign19230_body13_e23720_d_n11;
            locals.var_fs01_dps0_dn12 = assign19230_body13_e23720_d_n12;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign19230_body14_e23738, assign19230_body14_e23738_d_n0, assign19230_body14_e23738_d_n2, assign19230_body14_e23738_d_n4, assign19230_body14_e23738_d_n5, assign19230_body14_e23738_d_n6, assign19230_body14_e23738_d_n8, assign19230_body14_e23738_d_n10, assign19230_body14_e23738_d_n11, assign19230_body14_e23738_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign19230_body14_e23735: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign19230_body14_e23736: f64 = (assign19230_body14_e23735).exp();
        (assign19230_body14_e23736, (assign19230_body14_e23736 * (locals.var_beta * locals.var_ps0ld_dn0)), (assign19230_body14_e23736 * (locals.var_beta * locals.var_ps0ld_dn2)), (assign19230_body14_e23736 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign19230_body14_e23736 * (locals.var_beta * locals.var_ps0ld_dn5)), (assign19230_body14_e23736 * (locals.var_beta * locals.var_ps0ld_dn6)), (assign19230_body14_e23736 * (locals.var_beta * locals.var_ps0ld_dn8)), (assign19230_body14_e23736 * (locals.var_beta * locals.var_ps0ld_dn10)), (assign19230_body14_e23736 * (locals.var_beta * locals.var_ps0ld_dn11)), (assign19230_body14_e23736 * (locals.var_beta * locals.var_ps0ld_dn12)),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn12,)
    }
};
            locals.var_exp_bps0 = assign19230_body14_e23738;
            locals.var_exp_bps0_dn0 = assign19230_body14_e23738_d_n0;
            locals.var_exp_bps0_dn2 = assign19230_body14_e23738_d_n2;
            locals.var_exp_bps0_dn4 = assign19230_body14_e23738_d_n4;
            locals.var_exp_bps0_dn5 = assign19230_body14_e23738_d_n5;
            locals.var_exp_bps0_dn6 = assign19230_body14_e23738_d_n6;
            locals.var_exp_bps0_dn8 = assign19230_body14_e23738_d_n8;
            locals.var_exp_bps0_dn10 = assign19230_body14_e23738_d_n10;
            locals.var_exp_bps0_dn11 = assign19230_body14_e23738_d_n11;
            locals.var_exp_bps0_dn12 = assign19230_body14_e23738_d_n12;
            locals.var_exp_bps0_rv = 0.0;
            let (assign19230_body15_e23757, assign19230_body15_e23757_d_n0, assign19230_body15_e23757_d_n2, assign19230_body15_e23757_d_n4, assign19230_body15_e23757_d_n5, assign19230_body15_e23757_d_n6, assign19230_body15_e23757_d_n8, assign19230_body15_e23757_d_n10, assign19230_body15_e23757_d_n11, assign19230_body15_e23757_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign19230_body15_e23754: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign19230_body15_e23755: f64 = (locals.var_cnst1over * assign19230_body15_e23754);
        (assign19230_body15_e23755, ((locals.var_cnst1over_dn0 * assign19230_body15_e23754) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((locals.var_cnst1over_dn2 * assign19230_body15_e23754) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((locals.var_cnst1over_dn4 * assign19230_body15_e23754) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((locals.var_cnst1over_dn5 * assign19230_body15_e23754) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((locals.var_cnst1over_dn6 * assign19230_body15_e23754) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((locals.var_cnst1over_dn8 * assign19230_body15_e23754) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((locals.var_cnst1over_dn10 * assign19230_body15_e23754) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((locals.var_cnst1over_dn11 * assign19230_body15_e23754) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((locals.var_cnst1over_dn12 * assign19230_body15_e23754) + (locals.var_cnst1over * (locals.var_exp_bps0_dn12 - locals.var_exp_bvbs_dn12))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn8, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12,)
    }
};
            locals.var_fs01 = assign19230_body15_e23757;
            locals.var_fs01_dn0 = assign19230_body15_e23757_d_n0;
            locals.var_fs01_dn2 = assign19230_body15_e23757_d_n2;
            locals.var_fs01_dn4 = assign19230_body15_e23757_d_n4;
            locals.var_fs01_dn5 = assign19230_body15_e23757_d_n5;
            locals.var_fs01_dn6 = assign19230_body15_e23757_d_n6;
            locals.var_fs01_dn8 = assign19230_body15_e23757_d_n8;
            locals.var_fs01_dn10 = assign19230_body15_e23757_d_n10;
            locals.var_fs01_dn11 = assign19230_body15_e23757_d_n11;
            locals.var_fs01_dn12 = assign19230_body15_e23757_d_n12;
            locals.var_fs01_rv = 0.0;
            let (assign19230_body16_e23776, assign19230_body16_e23776_d_n0, assign19230_body16_e23776_d_n2, assign19230_body16_e23776_d_n4, assign19230_body16_e23776_d_n5, assign19230_body16_e23776_d_n6, assign19230_body16_e23776_d_n8, assign19230_body16_e23776_d_n10, assign19230_body16_e23776_d_n11, assign19230_body16_e23776_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign19230_body16_e23772: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign19230_body16_e23774: f64 = (assign19230_body16_e23772 * locals.var_exp_bps0);
        (assign19230_body16_e23774, (((locals.var_cnst1over_dn0 * locals.var_beta) * locals.var_exp_bps0) + (assign19230_body16_e23772 * locals.var_exp_bps0_dn0)), (((locals.var_cnst1over_dn2 * locals.var_beta) * locals.var_exp_bps0) + (assign19230_body16_e23772 * locals.var_exp_bps0_dn2)), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * locals.var_exp_bps0) + (assign19230_body16_e23772 * locals.var_exp_bps0_dn4)), (((locals.var_cnst1over_dn5 * locals.var_beta) * locals.var_exp_bps0) + (assign19230_body16_e23772 * locals.var_exp_bps0_dn5)), (((locals.var_cnst1over_dn6 * locals.var_beta) * locals.var_exp_bps0) + (assign19230_body16_e23772 * locals.var_exp_bps0_dn6)), (((locals.var_cnst1over_dn8 * locals.var_beta) * locals.var_exp_bps0) + (assign19230_body16_e23772 * locals.var_exp_bps0_dn8)), (((locals.var_cnst1over_dn10 * locals.var_beta) * locals.var_exp_bps0) + (assign19230_body16_e23772 * locals.var_exp_bps0_dn10)), (((locals.var_cnst1over_dn11 * locals.var_beta) * locals.var_exp_bps0) + (assign19230_body16_e23772 * locals.var_exp_bps0_dn11)), (((locals.var_cnst1over_dn12 * locals.var_beta) * locals.var_exp_bps0) + (assign19230_body16_e23772 * locals.var_exp_bps0_dn12)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12,)
    }
};
            locals.var_fs01_dps0 = assign19230_body16_e23776;
            locals.var_fs01_dps0_dn0 = assign19230_body16_e23776_d_n0;
            locals.var_fs01_dps0_dn2 = assign19230_body16_e23776_d_n2;
            locals.var_fs01_dps0_dn4 = assign19230_body16_e23776_d_n4;
            locals.var_fs01_dps0_dn5 = assign19230_body16_e23776_d_n5;
            locals.var_fs01_dps0_dn6 = assign19230_body16_e23776_d_n6;
            locals.var_fs01_dps0_dn8 = assign19230_body16_e23776_d_n8;
            locals.var_fs01_dps0_dn10 = assign19230_body16_e23776_d_n10;
            locals.var_fs01_dps0_dn11 = assign19230_body16_e23776_d_n11;
            locals.var_fs01_dps0_dn12 = assign19230_body16_e23776_d_n12;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign19230_body17_e23793, assign19230_body17_e23793_d_n0, assign19230_body17_e23793_d_n2, assign19230_body17_e23793_d_n4, assign19230_body17_e23793_d_n5, assign19230_body17_e23793_d_n6, assign19230_body17_e23793_d_n8, assign19230_body17_e23793_d_n10, assign19230_body17_e23793_d_n11, assign19230_body17_e23793_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) {
        let assign19230_body17_e23788: f64 = (locals.var_chi - 1.0);
        let assign19230_body17_e23790: f64 = (assign19230_body17_e23788 + locals.var_fs01);
        let assign19230_body17_e23791: f64 = (assign19230_body17_e23790).sqrt();
        (assign19230_body17_e23791, ((locals.var_chi_dn0 + locals.var_fs01_dn0) / (2.0 * assign19230_body17_e23791)), ((locals.var_chi_dn2 + locals.var_fs01_dn2) / (2.0 * assign19230_body17_e23791)), ((locals.var_chi_dn4 + locals.var_fs01_dn4) / (2.0 * assign19230_body17_e23791)), ((locals.var_chi_dn5 + locals.var_fs01_dn5) / (2.0 * assign19230_body17_e23791)), ((locals.var_chi_dn6 + locals.var_fs01_dn6) / (2.0 * assign19230_body17_e23791)), ((locals.var_chi_dn8 + locals.var_fs01_dn8) / (2.0 * assign19230_body17_e23791)), ((locals.var_chi_dn10 + locals.var_fs01_dn10) / (2.0 * assign19230_body17_e23791)), ((locals.var_chi_dn11 + locals.var_fs01_dn11) / (2.0 * assign19230_body17_e23791)), ((locals.var_chi_dn12 + locals.var_fs01_dn12) / (2.0 * assign19230_body17_e23791)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn8, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12,)
    }
};
            locals.var_fs02 = assign19230_body17_e23793;
            locals.var_fs02_dn0 = assign19230_body17_e23793_d_n0;
            locals.var_fs02_dn2 = assign19230_body17_e23793_d_n2;
            locals.var_fs02_dn4 = assign19230_body17_e23793_d_n4;
            locals.var_fs02_dn5 = assign19230_body17_e23793_d_n5;
            locals.var_fs02_dn6 = assign19230_body17_e23793_d_n6;
            locals.var_fs02_dn8 = assign19230_body17_e23793_d_n8;
            locals.var_fs02_dn10 = assign19230_body17_e23793_d_n10;
            locals.var_fs02_dn11 = assign19230_body17_e23793_d_n11;
            locals.var_fs02_dn12 = assign19230_body17_e23793_d_n12;
            locals.var_fs02_rv = 0.0;
            let (assign19230_body18_e23811, assign19230_body18_e23811_d_n0, assign19230_body18_e23811_d_n2, assign19230_body18_e23811_d_n4, assign19230_body18_e23811_d_n5, assign19230_body18_e23811_d_n6, assign19230_body18_e23811_d_n8, assign19230_body18_e23811_d_n10, assign19230_body18_e23811_d_n11, assign19230_body18_e23811_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) {
        let assign19230_body18_e23805: f64 = (locals.var_beta + locals.var_fs01_dps0);
        let assign19230_body18_e23807: f64 = (assign19230_body18_e23805 / locals.var_fs02);
        let assign19230_body18_e23809: f64 = (assign19230_body18_e23807 * 0.5);
        (assign19230_body18_e23809, ((((locals.var_fs01_dps0_dn0 * locals.var_fs02) - (assign19230_body18_e23805 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn2 * locals.var_fs02) - (assign19230_body18_e23805 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), (((((locals.var_beta_dn4 + locals.var_fs01_dps0_dn4) * locals.var_fs02) - (assign19230_body18_e23805 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn5 * locals.var_fs02) - (assign19230_body18_e23805 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn6 * locals.var_fs02) - (assign19230_body18_e23805 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn8 * locals.var_fs02) - (assign19230_body18_e23805 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn10 * locals.var_fs02) - (assign19230_body18_e23805 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn11 * locals.var_fs02) - (assign19230_body18_e23805 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn12 * locals.var_fs02) - (assign19230_body18_e23805 * locals.var_fs02_dn12)) / (locals.var_fs02 * locals.var_fs02)) * 0.5),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12,)
    }
};
            locals.var_fs02_dps0 = assign19230_body18_e23811;
            locals.var_fs02_dps0_dn0 = assign19230_body18_e23811_d_n0;
            locals.var_fs02_dps0_dn2 = assign19230_body18_e23811_d_n2;
            locals.var_fs02_dps0_dn4 = assign19230_body18_e23811_d_n4;
            locals.var_fs02_dps0_dn5 = assign19230_body18_e23811_d_n5;
            locals.var_fs02_dps0_dn6 = assign19230_body18_e23811_d_n6;
            locals.var_fs02_dps0_dn8 = assign19230_body18_e23811_d_n8;
            locals.var_fs02_dps0_dn10 = assign19230_body18_e23811_d_n10;
            locals.var_fs02_dps0_dn11 = assign19230_body18_e23811_d_n11;
            locals.var_fs02_dps0_dn12 = assign19230_body18_e23811_d_n12;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign19230_body19_e23826, assign19230_body19_e23826_d_n0, assign19230_body19_e23826_d_n2, assign19230_body19_e23826_d_n4, assign19230_body19_e23826_d_n5, assign19230_body19_e23826_d_n6, assign19230_body19_e23826_d_n8, assign19230_body19_e23826_d_n10, assign19230_body19_e23826_d_n11, assign19230_body19_e23826_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19230_body19_e23820: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign19230_body19_e23823: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign19230_body19_e23824: f64 = (assign19230_body19_e23820 - assign19230_body19_e23823);
        (assign19230_body19_e23824, ((locals.var_vgpld_dn0 - locals.var_ps0ld_dn0) - ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), ((locals.var_vgpld_dn2 - locals.var_ps0ld_dn2) - ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), ((-locals.var_ps0ld_dn4) - ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), ((locals.var_vgpld_dn5 - locals.var_ps0ld_dn5) - ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), ((-locals.var_ps0ld_dn6) - ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), ((-locals.var_ps0ld_dn8) - ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), ((-locals.var_ps0ld_dn10) - ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), ((-locals.var_ps0ld_dn11) - ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), ((-locals.var_ps0ld_dn12) - ((locals.var_fac1_dn12 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn12))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn8, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn12,)
    }
};
            locals.var_fs0 = assign19230_body19_e23826;
            locals.var_fs0_dn0 = assign19230_body19_e23826_d_n0;
            locals.var_fs0_dn2 = assign19230_body19_e23826_d_n2;
            locals.var_fs0_dn4 = assign19230_body19_e23826_d_n4;
            locals.var_fs0_dn5 = assign19230_body19_e23826_d_n5;
            locals.var_fs0_dn6 = assign19230_body19_e23826_d_n6;
            locals.var_fs0_dn8 = assign19230_body19_e23826_d_n8;
            locals.var_fs0_dn10 = assign19230_body19_e23826_d_n10;
            locals.var_fs0_dn11 = assign19230_body19_e23826_d_n11;
            locals.var_fs0_dn12 = assign19230_body19_e23826_d_n12;
            locals.var_fs0_rv = 0.0;
            let (assign19230_body20_e23840, assign19230_body20_e23840_d_n0, assign19230_body20_e23840_d_n2, assign19230_body20_e23840_d_n4, assign19230_body20_e23840_d_n5, assign19230_body20_e23840_d_n6, assign19230_body20_e23840_d_n8, assign19230_body20_e23840_d_n10, assign19230_body20_e23840_d_n11, assign19230_body20_e23840_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19230_body20_e23834: f64 = (-1.0);
        let assign19230_body20_e23837: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign19230_body20_e23838: f64 = (assign19230_body20_e23834 - assign19230_body20_e23837);
        (assign19230_body20_e23838, (-((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0))), (-((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2))), (-((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4))), (-((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5))), (-((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6))), (-((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8))), (-((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10))), (-((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11))), (-((locals.var_fac1_dn12 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn12))),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn12,)
    }
};
            locals.var_fs0_dps0 = assign19230_body20_e23840;
            locals.var_fs0_dps0_dn0 = assign19230_body20_e23840_d_n0;
            locals.var_fs0_dps0_dn2 = assign19230_body20_e23840_d_n2;
            locals.var_fs0_dps0_dn4 = assign19230_body20_e23840_d_n4;
            locals.var_fs0_dps0_dn5 = assign19230_body20_e23840_d_n5;
            locals.var_fs0_dps0_dn6 = assign19230_body20_e23840_d_n6;
            locals.var_fs0_dps0_dn8 = assign19230_body20_e23840_d_n8;
            locals.var_fs0_dps0_dn10 = assign19230_body20_e23840_d_n10;
            locals.var_fs0_dps0_dn11 = assign19230_body20_e23840_d_n11;
            locals.var_fs0_dps0_dn12 = assign19230_body20_e23840_d_n12;
            locals.var_fs0_dps0_rv = 0.0;
            let assign19230_body21_e23843: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard339 = assign19230_body21_e23843;
            locals.var_guard339_rv = 0.0;
            let (assign19230_body22_e23856,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard339 != 0.0)) {
        let assign19230_body22_e23854: f64 = (40.0 + 1.0);
        (assign19230_body22_e23854,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign19230_body22_e23856;
            locals.var_lp_s0_rv = 0.0;
            let (assign19230_body23_e23871, assign19230_body23_e23871_d_n0, assign19230_body23_e23871_d_n2, assign19230_body23_e23871_d_n4, assign19230_body23_e23871_d_n5, assign19230_body23_e23871_d_n6, assign19230_body23_e23871_d_n8, assign19230_body23_e23871_d_n10, assign19230_body23_e23871_d_n11, assign19230_body23_e23871_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard339 == 0.0)) {
        let assign19230_body23_e23867: f64 = (-locals.var_fs0);
        let assign19230_body23_e23869: f64 = (assign19230_body23_e23867 / locals.var_fs0_dps0);
        (assign19230_body23_e23869, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign19230_body23_e23867 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign19230_body23_e23867 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign19230_body23_e23867 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign19230_body23_e23867 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign19230_body23_e23867 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign19230_body23_e23867 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign19230_body23_e23867 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign19230_body23_e23867 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn12) * locals.var_fs0_dps0) - (assign19230_body23_e23867 * locals.var_fs0_dps0_dn12)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn8, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12,)
    }
};
            locals.var_dps0 = assign19230_body23_e23871;
            locals.var_dps0_dn0 = assign19230_body23_e23871_d_n0;
            locals.var_dps0_dn2 = assign19230_body23_e23871_d_n2;
            locals.var_dps0_dn4 = assign19230_body23_e23871_d_n4;
            locals.var_dps0_dn5 = assign19230_body23_e23871_d_n5;
            locals.var_dps0_dn6 = assign19230_body23_e23871_d_n6;
            locals.var_dps0_dn8 = assign19230_body23_e23871_d_n8;
            locals.var_dps0_dn10 = assign19230_body23_e23871_d_n10;
            locals.var_dps0_dn11 = assign19230_body23_e23871_d_n11;
            locals.var_dps0_dn12 = assign19230_body23_e23871_d_n12;
            locals.var_dps0_rv = 0.0;
            let (assign19230_body24_e23896, assign19230_body24_e23896_d_n0, assign19230_body24_e23896_d_n2, assign19230_body24_e23896_d_n4, assign19230_body24_e23896_d_n5, assign19230_body24_e23896_d_n6, assign19230_body24_e23896_d_n8, assign19230_body24_e23896_d_n10, assign19230_body24_e23896_d_n11, assign19230_body24_e23896_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard339 == 0.0)) {
        let assign19230_body24_e23883: f64 = (0.5 * 0.1);
        let assign19230_body24_e23887: f64 = (locals.var_ps0ld).abs();
        let (assign19230_body24_e23892, assign19230_body24_e23892_d_n0, assign19230_body24_e23892_d_n2, assign19230_body24_e23892_d_n4, assign19230_body24_e23892_d_n5, assign19230_body24_e23892_d_n6, assign19230_body24_e23892_d_n8, assign19230_body24_e23892_d_n10, assign19230_body24_e23892_d_n11, assign19230_body24_e23892_d_n12,) = {
            if (1.0 >= assign19230_body24_e23887) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign19230_body24_e23891: f64 = (locals.var_ps0ld).abs();
                (assign19230_body24_e23891, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn12 } else { (-locals.var_ps0ld_dn12) },)
            }
        };
        let assign19230_body24_e23893: f64 = (1.0 + assign19230_body24_e23892);
        let assign19230_body24_e23894: f64 = (assign19230_body24_e23883 * assign19230_body24_e23893);
        (assign19230_body24_e23894, (assign19230_body24_e23883 * assign19230_body24_e23892_d_n0), (assign19230_body24_e23883 * assign19230_body24_e23892_d_n2), (assign19230_body24_e23883 * assign19230_body24_e23892_d_n4), (assign19230_body24_e23883 * assign19230_body24_e23892_d_n5), (assign19230_body24_e23883 * assign19230_body24_e23892_d_n6), (assign19230_body24_e23883 * assign19230_body24_e23892_d_n8), (assign19230_body24_e23883 * assign19230_body24_e23892_d_n10), (assign19230_body24_e23883 * assign19230_body24_e23892_d_n11), (assign19230_body24_e23883 * assign19230_body24_e23892_d_n12),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn8, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12,)
    }
};
            locals.var_dplim = assign19230_body24_e23896;
            locals.var_dplim_dn0 = assign19230_body24_e23896_d_n0;
            locals.var_dplim_dn2 = assign19230_body24_e23896_d_n2;
            locals.var_dplim_dn4 = assign19230_body24_e23896_d_n4;
            locals.var_dplim_dn5 = assign19230_body24_e23896_d_n5;
            locals.var_dplim_dn6 = assign19230_body24_e23896_d_n6;
            locals.var_dplim_dn8 = assign19230_body24_e23896_d_n8;
            locals.var_dplim_dn10 = assign19230_body24_e23896_d_n10;
            locals.var_dplim_dn11 = assign19230_body24_e23896_d_n11;
            locals.var_dplim_dn12 = assign19230_body24_e23896_d_n12;
            locals.var_dplim_rv = 0.0;
            let assign19230_body25_e23898: f64 = (locals.var_dps0).abs();
            let assign19230_body25_e23900: f64 = if assign19230_body25_e23898 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard340 = assign19230_body25_e23900;
            locals.var_guard340_rv = 0.0;
            let (assign19230_body26_e23922, assign19230_body26_e23922_d_n0, assign19230_body26_e23922_d_n2, assign19230_body26_e23922_d_n4, assign19230_body26_e23922_d_n5, assign19230_body26_e23922_d_n6, assign19230_body26_e23922_d_n8, assign19230_body26_e23922_d_n10, assign19230_body26_e23922_d_n11, assign19230_body26_e23922_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard339 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let (assign19230_body26_e23919,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign19230_body26_e23918: f64 = (-1.0);
                (assign19230_body26_e23918,)
            }
        };
        let assign19230_body26_e23920: f64 = (locals.var_dplim * assign19230_body26_e23919);
        (assign19230_body26_e23920, (locals.var_dplim_dn0 * assign19230_body26_e23919), (locals.var_dplim_dn2 * assign19230_body26_e23919), (locals.var_dplim_dn4 * assign19230_body26_e23919), (locals.var_dplim_dn5 * assign19230_body26_e23919), (locals.var_dplim_dn6 * assign19230_body26_e23919), (locals.var_dplim_dn8 * assign19230_body26_e23919), (locals.var_dplim_dn10 * assign19230_body26_e23919), (locals.var_dplim_dn11 * assign19230_body26_e23919), (locals.var_dplim_dn12 * assign19230_body26_e23919),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn8, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12,)
    }
};
            locals.var_dps0 = assign19230_body26_e23922;
            locals.var_dps0_dn0 = assign19230_body26_e23922_d_n0;
            locals.var_dps0_dn2 = assign19230_body26_e23922_d_n2;
            locals.var_dps0_dn4 = assign19230_body26_e23922_d_n4;
            locals.var_dps0_dn5 = assign19230_body26_e23922_d_n5;
            locals.var_dps0_dn6 = assign19230_body26_e23922_d_n6;
            locals.var_dps0_dn8 = assign19230_body26_e23922_d_n8;
            locals.var_dps0_dn10 = assign19230_body26_e23922_d_n10;
            locals.var_dps0_dn11 = assign19230_body26_e23922_d_n11;
            locals.var_dps0_dn12 = assign19230_body26_e23922_d_n12;
            locals.var_dps0_rv = 0.0;
            let (assign19230_body27_e23936, assign19230_body27_e23936_d_n0, assign19230_body27_e23936_d_n2, assign19230_body27_e23936_d_n4, assign19230_body27_e23936_d_n5, assign19230_body27_e23936_d_n6, assign19230_body27_e23936_d_n8, assign19230_body27_e23936_d_n10, assign19230_body27_e23936_d_n11, assign19230_body27_e23936_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard339 == 0.0)) {
        let assign19230_body27_e23934: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign19230_body27_e23934, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn12 + locals.var_dps0_dn12),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn8, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12,)
    }
};
            locals.var_ps0ld = assign19230_body27_e23936;
            locals.var_ps0ld_dn0 = assign19230_body27_e23936_d_n0;
            locals.var_ps0ld_dn2 = assign19230_body27_e23936_d_n2;
            locals.var_ps0ld_dn4 = assign19230_body27_e23936_d_n4;
            locals.var_ps0ld_dn5 = assign19230_body27_e23936_d_n5;
            locals.var_ps0ld_dn6 = assign19230_body27_e23936_d_n6;
            locals.var_ps0ld_dn8 = assign19230_body27_e23936_d_n8;
            locals.var_ps0ld_dn10 = assign19230_body27_e23936_d_n10;
            locals.var_ps0ld_dn11 = assign19230_body27_e23936_d_n11;
            locals.var_ps0ld_dn12 = assign19230_body27_e23936_d_n12;
            locals.var_ps0ld_rv = 0.0;
            let assign19230_body28_e23938: f64 = (locals.var_dps0).abs();
            let assign19230_body28_e23942: f64 = (locals.var_fs0).abs();
            let assign19230_body28_e23945: f64 = if ((assign19230_body28_e23938 <= 1e-12) && (assign19230_body28_e23942 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard341 = assign19230_body28_e23945;
            locals.var_guard341_rv = 0.0;
            let (assign19230_body29_e23959,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard339 == 0.0)) && (locals.var_guard341 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign19230_body29_e23959;
            locals.var_flg_conv_rv = 0.0;
            let (assign19230_body30_e23970,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19230_body30_e23968: f64 = (locals.var_lp_s0 + 1.0);
        (assign19230_body30_e23968,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign19230_body30_e23970;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_78(
        locals: &mut StampLocals,
    ) {
        let assign19250_e23976: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard343 = assign19250_e23976;
        locals.var_guard343_rv = 0.0;

        let (assign19290_e24023, assign19290_e24023_d_n0, assign19290_e24023_d_n2, assign19290_e24023_d_n4, assign19290_e24023_d_n5, assign19290_e24023_d_n6, assign19290_e24023_d_n8, assign19290_e24023_d_n10, assign19290_e24023_d_n11, assign19290_e24023_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard343 != 0.0)) {
        let assign19290_e24017: f64 = (locals.var_fb * locals.var_fb);
        let assign19290_e24020: f64 = (10.0 * 2.220446049250313e-16);
        let assign19290_e24021: f64 = (assign19290_e24017 + assign19290_e24020);
        (assign19290_e24021, ((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)), ((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)), ((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)), ((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)), ((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)), ((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)), ((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)), ((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)), ((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)),)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn8, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12,)
    }
};
        locals.var_xi0 = assign19290_e24023;
        locals.var_xi0_dn0 = assign19290_e24023_d_n0;
        locals.var_xi0_dn2 = assign19290_e24023_d_n2;
        locals.var_xi0_dn4 = assign19290_e24023_d_n4;
        locals.var_xi0_dn5 = assign19290_e24023_d_n5;
        locals.var_xi0_dn6 = assign19290_e24023_d_n6;
        locals.var_xi0_dn8 = assign19290_e24023_d_n8;
        locals.var_xi0_dn10 = assign19290_e24023_d_n10;
        locals.var_xi0_dn11 = assign19290_e24023_d_n11;
        locals.var_xi0_dn12 = assign19290_e24023_d_n12;
        locals.var_xi0_rv = 0.0;

        let (assign19300_e24038, assign19300_e24038_d_n0, assign19300_e24038_d_n2, assign19300_e24038_d_n4, assign19300_e24038_d_n5, assign19300_e24038_d_n6, assign19300_e24038_d_n8, assign19300_e24038_d_n10, assign19300_e24038_d_n11, assign19300_e24038_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard343 != 0.0)) {
        let assign19300_e24035: f64 = (10.0 * 2.220446049250313e-16);
        let assign19300_e24036: f64 = (locals.var_fb + assign19300_e24035);
        (assign19300_e24036, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn8, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12,)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn8, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn12,)
    }
};
        locals.var_xi0p12 = assign19300_e24038;
        locals.var_xi0p12_dn0 = assign19300_e24038_d_n0;
        locals.var_xi0p12_dn2 = assign19300_e24038_d_n2;
        locals.var_xi0p12_dn4 = assign19300_e24038_d_n4;
        locals.var_xi0p12_dn5 = assign19300_e24038_d_n5;
        locals.var_xi0p12_dn6 = assign19300_e24038_d_n6;
        locals.var_xi0p12_dn8 = assign19300_e24038_d_n8;
        locals.var_xi0p12_dn10 = assign19300_e24038_d_n10;
        locals.var_xi0p12_dn11 = assign19300_e24038_d_n11;
        locals.var_xi0p12_dn12 = assign19300_e24038_d_n12;
        locals.var_xi0p12_rv = 0.0;

        let (assign19320_e24064, assign19320_e24064_d_n0, assign19320_e24064_d_n2, assign19320_e24064_d_n4, assign19320_e24064_d_n5, assign19320_e24064_d_n6, assign19320_e24064_d_n8, assign19320_e24064_d_n10, assign19320_e24064_d_n11, assign19320_e24064_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard343 == 0.0)) {
        let assign19320_e24062: f64 = (locals.var_chi - 1.0);
        (assign19320_e24062, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn8, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12,)
    }
};
        locals.var_xi0 = assign19320_e24064;
        locals.var_xi0_dn0 = assign19320_e24064_d_n0;
        locals.var_xi0_dn2 = assign19320_e24064_d_n2;
        locals.var_xi0_dn4 = assign19320_e24064_d_n4;
        locals.var_xi0_dn5 = assign19320_e24064_d_n5;
        locals.var_xi0_dn6 = assign19320_e24064_d_n6;
        locals.var_xi0_dn8 = assign19320_e24064_d_n8;
        locals.var_xi0_dn10 = assign19320_e24064_d_n10;
        locals.var_xi0_dn11 = assign19320_e24064_d_n11;
        locals.var_xi0_dn12 = assign19320_e24064_d_n12;
        locals.var_xi0_rv = 0.0;

        let (assign19330_e24077, assign19330_e24077_d_n0, assign19330_e24077_d_n2, assign19330_e24077_d_n4, assign19330_e24077_d_n5, assign19330_e24077_d_n6, assign19330_e24077_d_n8, assign19330_e24077_d_n10, assign19330_e24077_d_n11, assign19330_e24077_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard343 == 0.0)) {
        let assign19330_e24075: f64 = (locals.var_xi0).sqrt();
        (assign19330_e24075, (locals.var_xi0_dn0 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn2 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn4 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn5 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn6 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn8 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn10 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn11 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn12 / (2.0 * assign19330_e24075)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn8, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn12,)
    }
};
        locals.var_xi0p12 = assign19330_e24077;
        locals.var_xi0p12_dn0 = assign19330_e24077_d_n0;
        locals.var_xi0p12_dn2 = assign19330_e24077_d_n2;
        locals.var_xi0p12_dn4 = assign19330_e24077_d_n4;
        locals.var_xi0p12_dn5 = assign19330_e24077_d_n5;
        locals.var_xi0p12_dn6 = assign19330_e24077_d_n6;
        locals.var_xi0p12_dn8 = assign19330_e24077_d_n8;
        locals.var_xi0p12_dn10 = assign19330_e24077_d_n10;
        locals.var_xi0p12_dn11 = assign19330_e24077_d_n11;
        locals.var_xi0p12_dn12 = assign19330_e24077_d_n12;
        locals.var_xi0p12_rv = 0.0;

        let (assign19340_e24088, assign19340_e24088_d_n0, assign19340_e24088_d_n2, assign19340_e24088_d_n4, assign19340_e24088_d_n5, assign19340_e24088_d_n6, assign19340_e24088_d_n8, assign19340_e24088_d_n10, assign19340_e24088_d_n11, assign19340_e24088_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19340_e24086: f64 = (locals.var_cnst0over * locals.var_xi0p12);
        (assign19340_e24086, ((locals.var_cnst0over_dn0 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_dn2 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_dn4 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_dn5 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_dn6 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_dn8 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_dn10 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_dn11 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_dn12 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn12)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn8, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12,)
    }
};
        locals.var_qbuld = assign19340_e24088;
        locals.var_qbuld_dn0 = assign19340_e24088_d_n0;
        locals.var_qbuld_dn2 = assign19340_e24088_d_n2;
        locals.var_qbuld_dn4 = assign19340_e24088_d_n4;
        locals.var_qbuld_dn5 = assign19340_e24088_d_n5;
        locals.var_qbuld_dn6 = assign19340_e24088_d_n6;
        locals.var_qbuld_dn8 = assign19340_e24088_d_n8;
        locals.var_qbuld_dn10 = assign19340_e24088_d_n10;
        locals.var_qbuld_dn11 = assign19340_e24088_d_n11;
        locals.var_qbuld_dn12 = assign19340_e24088_d_n12;
        locals.var_qbuld_rv = 0.0;

        let (assign19350_e24101, assign19350_e24101_d_n0, assign19350_e24101_d_n2, assign19350_e24101_d_n4, assign19350_e24101_d_n5, assign19350_e24101_d_n6, assign19350_e24101_d_n8, assign19350_e24101_d_n10, assign19350_e24101_d_n11, assign19350_e24101_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19350_e24098: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign19350_e24099: f64 = (1.0 / assign19350_e24098);
        (assign19350_e24099, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn12 + locals.var_xi0p12_dn12) / (assign19350_e24098 * assign19350_e24098))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign19350_e24101;
        locals.var_t1_dn0 = assign19350_e24101_d_n0;
        locals.var_t1_dn2 = assign19350_e24101_d_n2;
        locals.var_t1_dn4 = assign19350_e24101_d_n4;
        locals.var_t1_dn5 = assign19350_e24101_d_n5;
        locals.var_t1_dn6 = assign19350_e24101_d_n6;
        locals.var_t1_dn8 = assign19350_e24101_d_n8;
        locals.var_t1_dn10 = assign19350_e24101_d_n10;
        locals.var_t1_dn11 = assign19350_e24101_d_n11;
        locals.var_t1_dn12 = assign19350_e24101_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign19360_e24114, assign19360_e24114_d_n0, assign19360_e24114_d_n2, assign19360_e24114_d_n4, assign19360_e24114_d_n5, assign19360_e24114_d_n6, assign19360_e24114_d_n8, assign19360_e24114_d_n10, assign19360_e24114_d_n11, assign19360_e24114_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19360_e24110: f64 = (locals.var_cnst0over * locals.var_fs01);
        let assign19360_e24112: f64 = (assign19360_e24110 * locals.var_t1);
        (assign19360_e24112, ((((locals.var_cnst0over_dn0 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn0)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn0)), ((((locals.var_cnst0over_dn2 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn2)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn2)), ((((locals.var_cnst0over_dn4 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn4)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn4)), ((((locals.var_cnst0over_dn5 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn5)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn5)), ((((locals.var_cnst0over_dn6 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn6)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn6)), ((((locals.var_cnst0over_dn8 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn8)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn8)), ((((locals.var_cnst0over_dn10 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn10)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn10)), ((((locals.var_cnst0over_dn11 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn11)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn11)), ((((locals.var_cnst0over_dn12 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn12)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn12)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn8, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12,)
    }
};
        locals.var_qiuld = assign19360_e24114;
        locals.var_qiuld_dn0 = assign19360_e24114_d_n0;
        locals.var_qiuld_dn2 = assign19360_e24114_d_n2;
        locals.var_qiuld_dn4 = assign19360_e24114_d_n4;
        locals.var_qiuld_dn5 = assign19360_e24114_d_n5;
        locals.var_qiuld_dn6 = assign19360_e24114_d_n6;
        locals.var_qiuld_dn8 = assign19360_e24114_d_n8;
        locals.var_qiuld_dn10 = assign19360_e24114_d_n10;
        locals.var_qiuld_dn11 = assign19360_e24114_d_n11;
        locals.var_qiuld_dn12 = assign19360_e24114_d_n12;
        locals.var_qiuld_rv = 0.0;

        let (assign19370_e24125, assign19370_e24125_d_n0, assign19370_e24125_d_n2, assign19370_e24125_d_n4, assign19370_e24125_d_n5, assign19370_e24125_d_n6, assign19370_e24125_d_n8, assign19370_e24125_d_n10, assign19370_e24125_d_n11, assign19370_e24125_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
        let assign19370_e24123: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign19370_e24123, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12,)
    }
};
        locals.var_qsuld = assign19370_e24125;
        locals.var_qsuld_dn0 = assign19370_e24125_d_n0;
        locals.var_qsuld_dn2 = assign19370_e24125_d_n2;
        locals.var_qsuld_dn4 = assign19370_e24125_d_n4;
        locals.var_qsuld_dn5 = assign19370_e24125_d_n5;
        locals.var_qsuld_dn6 = assign19370_e24125_d_n6;
        locals.var_qsuld_dn8 = assign19370_e24125_d_n8;
        locals.var_qsuld_dn10 = assign19370_e24125_d_n10;
        locals.var_qsuld_dn11 = assign19370_e24125_d_n11;
        locals.var_qsuld_dn12 = assign19370_e24125_d_n12;
        locals.var_qsuld_rv = 0.0;

        let (assign19380_e24131, assign19380_e24131_d_n0, assign19380_e24131_d_n2, assign19380_e24131_d_n4, assign19380_e24131_d_n5, assign19380_e24131_d_n6, assign19380_e24131_d_n8, assign19380_e24131_d_n10, assign19380_e24131_d_n11, assign19380_e24131_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19380_e24129: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign19380_e24129, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn8, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12,)
    }
};
        locals.var_qiuld = assign19380_e24131;
        locals.var_qiuld_dn0 = assign19380_e24131_d_n0;
        locals.var_qiuld_dn2 = assign19380_e24131_d_n2;
        locals.var_qiuld_dn4 = assign19380_e24131_d_n4;
        locals.var_qiuld_dn5 = assign19380_e24131_d_n5;
        locals.var_qiuld_dn6 = assign19380_e24131_d_n6;
        locals.var_qiuld_dn8 = assign19380_e24131_d_n8;
        locals.var_qiuld_dn10 = assign19380_e24131_d_n10;
        locals.var_qiuld_dn11 = assign19380_e24131_d_n11;
        locals.var_qiuld_dn12 = assign19380_e24131_d_n12;
        locals.var_qiuld_rv = 0.0;

        let (assign19390_e24137, assign19390_e24137_d_n0, assign19390_e24137_d_n2, assign19390_e24137_d_n4, assign19390_e24137_d_n5, assign19390_e24137_d_n6, assign19390_e24137_d_n8, assign19390_e24137_d_n10, assign19390_e24137_d_n11, assign19390_e24137_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19390_e24135: f64 = (locals.var_weffcv_nf * locals.var_lov);
        (assign19390_e24135, (locals.var_weffcv_nf_dn0 * locals.var_lov), (locals.var_weffcv_nf_dn2 * locals.var_lov), (locals.var_weffcv_nf_dn4 * locals.var_lov), (locals.var_weffcv_nf_dn5 * locals.var_lov), (locals.var_weffcv_nf_dn6 * locals.var_lov), (locals.var_weffcv_nf_dn8 * locals.var_lov), (locals.var_weffcv_nf_dn10 * locals.var_lov), (locals.var_weffcv_nf_dn11 * locals.var_lov), (locals.var_weffcv_nf_dn12 * locals.var_lov),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign19390_e24137;
        locals.var_t4_dn0 = assign19390_e24137_d_n0;
        locals.var_t4_dn2 = assign19390_e24137_d_n2;
        locals.var_t4_dn4 = assign19390_e24137_d_n4;
        locals.var_t4_dn5 = assign19390_e24137_d_n5;
        locals.var_t4_dn6 = assign19390_e24137_d_n6;
        locals.var_t4_dn8 = assign19390_e24137_d_n8;
        locals.var_t4_dn10 = assign19390_e24137_d_n10;
        locals.var_t4_dn11 = assign19390_e24137_d_n11;
        locals.var_t4_dn12 = assign19390_e24137_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign19400_e24145, assign19400_e24145_d_n0, assign19400_e24145_d_n2, assign19400_e24145_d_n4, assign19400_e24145_d_n5, assign19400_e24145_d_n6, assign19400_e24145_d_n8, assign19400_e24145_d_n10, assign19400_e24145_d_n11, assign19400_e24145_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_flg_overs != 0.0)) {
        let assign19400_e24143: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign19400_e24143, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn12 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn12)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn8, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12,)
    }
};
        locals.var_qovs = assign19400_e24145;
        locals.var_qovs_dn0 = assign19400_e24145_d_n0;
        locals.var_qovs_dn2 = assign19400_e24145_d_n2;
        locals.var_qovs_dn4 = assign19400_e24145_d_n4;
        locals.var_qovs_dn5 = assign19400_e24145_d_n5;
        locals.var_qovs_dn6 = assign19400_e24145_d_n6;
        locals.var_qovs_dn8 = assign19400_e24145_d_n8;
        locals.var_qovs_dn10 = assign19400_e24145_d_n10;
        locals.var_qovs_dn11 = assign19400_e24145_d_n11;
        locals.var_qovs_dn12 = assign19400_e24145_d_n12;
        locals.var_qovs_rv = 0.0;

        let (assign19410_e24153, assign19410_e24153_d_n0, assign19410_e24153_d_n2, assign19410_e24153_d_n4, assign19410_e24153_d_n5, assign19410_e24153_d_n6, assign19410_e24153_d_n8, assign19410_e24153_d_n10, assign19410_e24153_d_n11, assign19410_e24153_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_flg_overs != 0.0)) {
        let assign19410_e24151: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign19410_e24151, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn12 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn12)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn8, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12,)
    }
};
        locals.var_qbsld = assign19410_e24153;
        locals.var_qbsld_dn0 = assign19410_e24153_d_n0;
        locals.var_qbsld_dn2 = assign19410_e24153_d_n2;
        locals.var_qbsld_dn4 = assign19410_e24153_d_n4;
        locals.var_qbsld_dn5 = assign19410_e24153_d_n5;
        locals.var_qbsld_dn6 = assign19410_e24153_d_n6;
        locals.var_qbsld_dn8 = assign19410_e24153_d_n8;
        locals.var_qbsld_dn10 = assign19410_e24153_d_n10;
        locals.var_qbsld_dn11 = assign19410_e24153_d_n11;
        locals.var_qbsld_dn12 = assign19410_e24153_d_n12;
        locals.var_qbsld_rv = 0.0;

        let (assign19420_e24161, assign19420_e24161_d_n0, assign19420_e24161_d_n2, assign19420_e24161_d_n4, assign19420_e24161_d_n5, assign19420_e24161_d_n6, assign19420_e24161_d_n8, assign19420_e24161_d_n10, assign19420_e24161_d_n11, assign19420_e24161_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_flg_overd != 0.0)) {
        let assign19420_e24159: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign19420_e24159, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn12 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn12)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn8, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12,)
    }
};
        locals.var_qovd = assign19420_e24161;
        locals.var_qovd_dn0 = assign19420_e24161_d_n0;
        locals.var_qovd_dn2 = assign19420_e24161_d_n2;
        locals.var_qovd_dn4 = assign19420_e24161_d_n4;
        locals.var_qovd_dn5 = assign19420_e24161_d_n5;
        locals.var_qovd_dn6 = assign19420_e24161_d_n6;
        locals.var_qovd_dn8 = assign19420_e24161_d_n8;
        locals.var_qovd_dn10 = assign19420_e24161_d_n10;
        locals.var_qovd_dn11 = assign19420_e24161_d_n11;
        locals.var_qovd_dn12 = assign19420_e24161_d_n12;
        locals.var_qovd_rv = 0.0;

        let (assign19430_e24169, assign19430_e24169_d_n0, assign19430_e24169_d_n2, assign19430_e24169_d_n4, assign19430_e24169_d_n5, assign19430_e24169_d_n6, assign19430_e24169_d_n8, assign19430_e24169_d_n10, assign19430_e24169_d_n11, assign19430_e24169_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_flg_overd != 0.0)) {
        let assign19430_e24167: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign19430_e24167, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn12 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn12)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn8, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12,)
    }
};
        locals.var_qbdld = assign19430_e24169;
        locals.var_qbdld_dn0 = assign19430_e24169_d_n0;
        locals.var_qbdld_dn2 = assign19430_e24169_d_n2;
        locals.var_qbdld_dn4 = assign19430_e24169_d_n4;
        locals.var_qbdld_dn5 = assign19430_e24169_d_n5;
        locals.var_qbdld_dn6 = assign19430_e24169_d_n6;
        locals.var_qbdld_dn8 = assign19430_e24169_d_n8;
        locals.var_qbdld_dn10 = assign19430_e24169_d_n10;
        locals.var_qbdld_dn11 = assign19430_e24169_d_n11;
        locals.var_qbdld_dn12 = assign19430_e24169_d_n12;
        locals.var_qbdld_rv = 0.0;

        let (assign19440_e24177,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19440_e24173: f64 = (1.0 - 1.0);
        let assign19440_e24175: f64 = (assign19440_e24173 / 2.0);
        (assign19440_e24175,)
    } else {
        (locals.var_flg_ovloops,)
    }
};
        locals.var_flg_ovloops = assign19440_e24177;
        locals.var_flg_ovloops_rv = 0.0;

        let (assign19450_e24185,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19450_e24181: f64 = (1.0 + 1.0);
        let assign19450_e24183: f64 = (assign19450_e24181 / 2.0);
        (assign19450_e24183,)
    } else {
        (locals.var_flg_ovloopd,)
    }
};
        locals.var_flg_ovloopd = assign19450_e24185;
        locals.var_flg_ovloopd_rv = 0.0;

        let (assign19460_e24195,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19460_e24189: f64 = (locals.var_flg_ovloops * locals.var_modenml);
        let assign19460_e24192: f64 = (locals.var_flg_ovloopd * locals.var_modervs);
        let assign19460_e24193: f64 = (assign19460_e24189 + assign19460_e24192);
        (assign19460_e24193,)
    } else {
        (locals.var_flg_overs,)
    }
};
        locals.var_flg_overs = assign19460_e24195;
        locals.var_flg_overs_rv = 0.0;

        let (assign19470_e24205,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19470_e24199: f64 = (locals.var_flg_ovloops * locals.var_modervs);
        let assign19470_e24202: f64 = (locals.var_flg_ovloopd * locals.var_modenml);
        let assign19470_e24203: f64 = (assign19470_e24199 + assign19470_e24202);
        (assign19470_e24203,)
    } else {
        (locals.var_flg_overd,)
    }
};
        locals.var_flg_overd = assign19470_e24205;
        locals.var_flg_overd_rv = 0.0;

        let (assign19480_e24219, assign19480_e24219_d_n0, assign19480_e24219_d_n2, assign19480_e24219_d_n5,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_flg_ovloops != 0.0)) {
        let assign19480_e24211: f64 = (locals.var_modenml * locals.var_vgse);
        let assign19480_e24215: f64 = (locals.var_vgse - locals.var_vdse);
        let assign19480_e24216: f64 = (locals.var_modervs * assign19480_e24215);
        let assign19480_e24217: f64 = (assign19480_e24211 + assign19480_e24216);
        (assign19480_e24217, ((locals.var_modenml * locals.var_vgse_dn0) + (locals.var_modervs * (locals.var_vgse_dn0 - locals.var_vdse_dn0))), ((locals.var_modenml * locals.var_vgse_dn2) + (locals.var_modervs * (locals.var_vgse_dn2 - locals.var_vdse_dn2))), ((locals.var_modenml * locals.var_vgse_dn5) + (locals.var_modervs * locals.var_vgse_dn5)),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn5,)
    }
};
        locals.var_vgbgmt = assign19480_e24219;
        locals.var_vgbgmt_dn0 = assign19480_e24219_d_n0;
        locals.var_vgbgmt_dn2 = assign19480_e24219_d_n2;
        locals.var_vgbgmt_dn5 = assign19480_e24219_d_n5;
        locals.var_vgbgmt_rv = 0.0;

        let (assign19490_e24233, assign19490_e24233_d_n0, assign19490_e24233_d_n2, assign19490_e24233_d_n5,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_flg_ovloopd != 0.0)) {
        let assign19490_e24225: f64 = (locals.var_modervs * locals.var_vgse);
        let assign19490_e24229: f64 = (locals.var_vgse - locals.var_vdse);
        let assign19490_e24230: f64 = (locals.var_modenml * assign19490_e24229);
        let assign19490_e24231: f64 = (assign19490_e24225 + assign19490_e24230);
        (assign19490_e24231, ((locals.var_modervs * locals.var_vgse_dn0) + (locals.var_modenml * (locals.var_vgse_dn0 - locals.var_vdse_dn0))), ((locals.var_modervs * locals.var_vgse_dn2) + (locals.var_modenml * (locals.var_vgse_dn2 - locals.var_vdse_dn2))), ((locals.var_modervs * locals.var_vgse_dn5) + (locals.var_modenml * locals.var_vgse_dn5)),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn5,)
    }
};
        locals.var_vgbgmt = assign19490_e24233;
        locals.var_vgbgmt_dn0 = assign19490_e24233_d_n0;
        locals.var_vgbgmt_dn2 = assign19490_e24233_d_n2;
        locals.var_vgbgmt_dn5 = assign19490_e24233_d_n5;
        locals.var_vgbgmt_rv = 0.0;

        let (assign19500_e24237,) = {
    if (locals.var_guard327 != 0.0) {
        (0.0,)
    } else {
        (locals.var_vxbgmt,)
    }
};
        locals.var_vxbgmt = assign19500_e24237;
        locals.var_vxbgmt_rv = 0.0;

        let (assign19510_e24242, assign19510_e24242_d_n0, assign19510_e24242_d_n2, assign19510_e24242_d_n4, assign19510_e24242_d_n5, assign19510_e24242_d_n6, assign19510_e24242_d_n8, assign19510_e24242_d_n10, assign19510_e24242_d_n11, assign19510_e24242_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19510_e24240: f64 = (-locals.var_vxbgmt);
        (assign19510_e24240, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign19510_e24242;
        locals.var_t0_dn0 = assign19510_e24242_d_n0;
        locals.var_t0_dn2 = assign19510_e24242_d_n2;
        locals.var_t0_dn4 = assign19510_e24242_d_n4;
        locals.var_t0_dn5 = assign19510_e24242_d_n5;
        locals.var_t0_dn6 = assign19510_e24242_d_n6;
        locals.var_t0_dn8 = assign19510_e24242_d_n8;
        locals.var_t0_dn10 = assign19510_e24242_d_n10;
        locals.var_t0_dn11 = assign19510_e24242_d_n11;
        locals.var_t0_dn12 = assign19510_e24242_d_n12;
        locals.var_t0_rv = 0.0;

        let assign19520_e24245: f64 = if locals.var_t0 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard345 = assign19520_e24245;
        locals.var_guard345_rv = 0.0;

        let (assign19530_e24253, assign19530_e24253_d_n0, assign19530_e24253_d_n2, assign19530_e24253_d_n4, assign19530_e24253_d_n5, assign19530_e24253_d_n6, assign19530_e24253_d_n8, assign19530_e24253_d_n10, assign19530_e24253_d_n11, assign19530_e24253_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
        let assign19530_e24251: f64 = (locals.var_t0 - locals.var_vbs_bnd);
        (assign19530_e24251, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign19530_e24253;
        locals.var_t1_dn0 = assign19530_e24253_d_n0;
        locals.var_t1_dn2 = assign19530_e24253_d_n2;
        locals.var_t1_dn4 = assign19530_e24253_d_n4;
        locals.var_t1_dn5 = assign19530_e24253_d_n5;
        locals.var_t1_dn6 = assign19530_e24253_d_n6;
        locals.var_t1_dn8 = assign19530_e24253_d_n8;
        locals.var_t1_dn10 = assign19530_e24253_d_n10;
        locals.var_t1_dn11 = assign19530_e24253_d_n11;
        locals.var_t1_dn12 = assign19530_e24253_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign19540_e24261, assign19540_e24261_d_n0, assign19540_e24261_d_n2, assign19540_e24261_d_n4, assign19540_e24261_d_n5, assign19540_e24261_d_n6, assign19540_e24261_d_n8, assign19540_e24261_d_n10, assign19540_e24261_d_n11, assign19540_e24261_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
        let assign19540_e24259: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign19540_e24259, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign19540_e24261;
        locals.var_t2_dn0 = assign19540_e24261_d_n0;
        locals.var_t2_dn2 = assign19540_e24261_d_n2;
        locals.var_t2_dn4 = assign19540_e24261_d_n4;
        locals.var_t2_dn5 = assign19540_e24261_d_n5;
        locals.var_t2_dn6 = assign19540_e24261_d_n6;
        locals.var_t2_dn8 = assign19540_e24261_d_n8;
        locals.var_t2_dn10 = assign19540_e24261_d_n10;
        locals.var_t2_dn11 = assign19540_e24261_d_n11;
        locals.var_t2_dn12 = assign19540_e24261_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign19550_e24269, assign19550_e24269_d_n0, assign19550_e24269_d_n2, assign19550_e24269_d_n4, assign19550_e24269_d_n5, assign19550_e24269_d_n6, assign19550_e24269_d_n8, assign19550_e24269_d_n10, assign19550_e24269_d_n11, assign19550_e24269_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
        let assign19550_e24267: f64 = (locals.var_t1 / locals.var_t2);
        (assign19550_e24267, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn12 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn12)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign19550_e24269;
        locals.var_tmf1_dn0 = assign19550_e24269_d_n0;
        locals.var_tmf1_dn2 = assign19550_e24269_d_n2;
        locals.var_tmf1_dn4 = assign19550_e24269_d_n4;
        locals.var_tmf1_dn5 = assign19550_e24269_d_n5;
        locals.var_tmf1_dn6 = assign19550_e24269_d_n6;
        locals.var_tmf1_dn8 = assign19550_e24269_d_n8;
        locals.var_tmf1_dn10 = assign19550_e24269_d_n10;
        locals.var_tmf1_dn11 = assign19550_e24269_d_n11;
        locals.var_tmf1_dn12 = assign19550_e24269_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign19560_e24277, assign19560_e24277_d_n0, assign19560_e24277_d_n2, assign19560_e24277_d_n4, assign19560_e24277_d_n5, assign19560_e24277_d_n6, assign19560_e24277_d_n8, assign19560_e24277_d_n10, assign19560_e24277_d_n11, assign19560_e24277_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
        let assign19560_e24275: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign19560_e24275, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign19560_e24277;
        locals.var_tmf2_dn0 = assign19560_e24277_d_n0;
        locals.var_tmf2_dn2 = assign19560_e24277_d_n2;
        locals.var_tmf2_dn4 = assign19560_e24277_d_n4;
        locals.var_tmf2_dn5 = assign19560_e24277_d_n5;
        locals.var_tmf2_dn6 = assign19560_e24277_d_n6;
        locals.var_tmf2_dn8 = assign19560_e24277_d_n8;
        locals.var_tmf2_dn10 = assign19560_e24277_d_n10;
        locals.var_tmf2_dn11 = assign19560_e24277_d_n11;
        locals.var_tmf2_dn12 = assign19560_e24277_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign19570_e24285, assign19570_e24285_d_n0, assign19570_e24285_d_n2, assign19570_e24285_d_n4, assign19570_e24285_d_n5, assign19570_e24285_d_n6, assign19570_e24285_d_n8, assign19570_e24285_d_n10, assign19570_e24285_d_n11, assign19570_e24285_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
        let assign19570_e24283: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign19570_e24283, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn8, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12,)
    }
};
        locals.var_tmf3 = assign19570_e24285;
        locals.var_tmf3_dn0 = assign19570_e24285_d_n0;
        locals.var_tmf3_dn2 = assign19570_e24285_d_n2;
        locals.var_tmf3_dn4 = assign19570_e24285_d_n4;
        locals.var_tmf3_dn5 = assign19570_e24285_d_n5;
        locals.var_tmf3_dn6 = assign19570_e24285_d_n6;
        locals.var_tmf3_dn8 = assign19570_e24285_d_n8;
        locals.var_tmf3_dn10 = assign19570_e24285_d_n10;
        locals.var_tmf3_dn11 = assign19570_e24285_d_n11;
        locals.var_tmf3_dn12 = assign19570_e24285_d_n12;
        locals.var_tmf3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_79(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19580_e24293, assign19580_e24293_d_n0, assign19580_e24293_d_n2, assign19580_e24293_d_n4, assign19580_e24293_d_n5, assign19580_e24293_d_n6, assign19580_e24293_d_n8, assign19580_e24293_d_n10, assign19580_e24293_d_n11, assign19580_e24293_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
        let assign19580_e24291: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign19580_e24291, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn8, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12,)
    }
};
        locals.var_tmf4 = assign19580_e24293;
        locals.var_tmf4_dn0 = assign19580_e24293_d_n0;
        locals.var_tmf4_dn2 = assign19580_e24293_d_n2;
        locals.var_tmf4_dn4 = assign19580_e24293_d_n4;
        locals.var_tmf4_dn5 = assign19580_e24293_d_n5;
        locals.var_tmf4_dn6 = assign19580_e24293_d_n6;
        locals.var_tmf4_dn8 = assign19580_e24293_d_n8;
        locals.var_tmf4_dn10 = assign19580_e24293_d_n10;
        locals.var_tmf4_dn11 = assign19580_e24293_d_n11;
        locals.var_tmf4_dn12 = assign19580_e24293_d_n12;
        locals.var_tmf4_rv = 0.0;

        let (assign19590_e24309, assign19590_e24309_d_n0, assign19590_e24309_d_n2, assign19590_e24309_d_n4, assign19590_e24309_d_n5, assign19590_e24309_d_n6, assign19590_e24309_d_n8, assign19590_e24309_d_n10, assign19590_e24309_d_n11, assign19590_e24309_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
        let assign19590_e24300: f64 = (1.0 + locals.var_tmf1);
        let assign19590_e24302: f64 = (assign19590_e24300 + locals.var_tmf2);
        let assign19590_e24304: f64 = (assign19590_e24302 + locals.var_tmf3);
        let assign19590_e24306: f64 = (assign19590_e24304 + locals.var_tmf4);
        let assign19590_e24307: f64 = (1.0 / assign19590_e24306);
        (assign19590_e24307, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign19590_e24306 * assign19590_e24306))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12,)
    }
};
        locals.var_ty = assign19590_e24309;
        locals.var_ty_dn0 = assign19590_e24309_d_n0;
        locals.var_ty_dn2 = assign19590_e24309_d_n2;
        locals.var_ty_dn4 = assign19590_e24309_d_n4;
        locals.var_ty_dn5 = assign19590_e24309_d_n5;
        locals.var_ty_dn6 = assign19590_e24309_d_n6;
        locals.var_ty_dn8 = assign19590_e24309_d_n8;
        locals.var_ty_dn10 = assign19590_e24309_d_n10;
        locals.var_ty_dn11 = assign19590_e24309_d_n11;
        locals.var_ty_dn12 = assign19590_e24309_d_n12;
        locals.var_ty_rv = 0.0;

        let (assign19600_e24332, assign19600_e24332_d_n0, assign19600_e24332_d_n2, assign19600_e24332_d_n4, assign19600_e24332_d_n5, assign19600_e24332_d_n6, assign19600_e24332_d_n8, assign19600_e24332_d_n10, assign19600_e24332_d_n11, assign19600_e24332_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
        let assign19600_e24316: f64 = (2.0 * locals.var_tmf1);
        let assign19600_e24317: f64 = (1.0 + assign19600_e24316);
        let assign19600_e24320: f64 = (3.0 * locals.var_tmf2);
        let assign19600_e24321: f64 = (assign19600_e24317 + assign19600_e24320);
        let assign19600_e24324: f64 = (4.0 * locals.var_tmf3);
        let assign19600_e24325: f64 = (assign19600_e24321 + assign19600_e24324);
        let assign19600_e24326: f64 = (-assign19600_e24325);
        let assign19600_e24328: f64 = (assign19600_e24326 * locals.var_ty);
        let assign19600_e24330: f64 = (assign19600_e24328 * locals.var_ty);
        (assign19600_e24330, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn0)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn2)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn4)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn5)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn6)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn6)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn8)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn8)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn10)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn11)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn11)), (((((-(((2.0 * locals.var_tmf1_dn12) + (3.0 * locals.var_tmf2_dn12)) + (4.0 * locals.var_tmf3_dn12))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn12)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn12)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn8, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign19600_e24332;
        locals.var_t11_dn0 = assign19600_e24332_d_n0;
        locals.var_t11_dn2 = assign19600_e24332_d_n2;
        locals.var_t11_dn4 = assign19600_e24332_d_n4;
        locals.var_t11_dn5 = assign19600_e24332_d_n5;
        locals.var_t11_dn6 = assign19600_e24332_d_n6;
        locals.var_t11_dn8 = assign19600_e24332_d_n8;
        locals.var_t11_dn10 = assign19600_e24332_d_n10;
        locals.var_t11_dn11 = assign19600_e24332_d_n11;
        locals.var_t11_dn12 = assign19600_e24332_d_n12;
        locals.var_t11_rv = 0.0;

        let (assign19610_e24342, assign19610_e24342_d_n0, assign19610_e24342_d_n2, assign19610_e24342_d_n4, assign19610_e24342_d_n5, assign19610_e24342_d_n6, assign19610_e24342_d_n8, assign19610_e24342_d_n10, assign19610_e24342_d_n11, assign19610_e24342_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
        let assign19610_e24339: f64 = (1.0 - locals.var_ty);
        let assign19610_e24340: f64 = (locals.var_t2 * assign19610_e24339);
        (assign19610_e24340, ((locals.var_t2_dn0 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn0))), ((locals.var_t2_dn2 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn2))), ((locals.var_t2_dn4 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn4))), ((locals.var_t2_dn5 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn5))), ((locals.var_t2_dn6 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn6))), ((locals.var_t2_dn8 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn8))), ((locals.var_t2_dn10 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn10))), ((locals.var_t2_dn11 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn11))), ((locals.var_t2_dn12 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn12))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12,)
    }
};
        locals.var_ty = assign19610_e24342;
        locals.var_ty_dn0 = assign19610_e24342_d_n0;
        locals.var_ty_dn2 = assign19610_e24342_d_n2;
        locals.var_ty_dn4 = assign19610_e24342_d_n4;
        locals.var_ty_dn5 = assign19610_e24342_d_n5;
        locals.var_ty_dn6 = assign19610_e24342_d_n6;
        locals.var_ty_dn8 = assign19610_e24342_d_n8;
        locals.var_ty_dn10 = assign19610_e24342_d_n10;
        locals.var_ty_dn11 = assign19610_e24342_d_n11;
        locals.var_ty_dn12 = assign19610_e24342_d_n12;
        locals.var_ty_rv = 0.0;

        let (assign19620_e24349, assign19620_e24349_d_n0, assign19620_e24349_d_n2, assign19620_e24349_d_n4, assign19620_e24349_d_n5, assign19620_e24349_d_n6, assign19620_e24349_d_n8, assign19620_e24349_d_n10, assign19620_e24349_d_n11, assign19620_e24349_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
        let assign19620_e24347: f64 = (-locals.var_t11);
        (assign19620_e24347, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn8), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn12),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn8, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign19620_e24349;
        locals.var_t11_dn0 = assign19620_e24349_d_n0;
        locals.var_t11_dn2 = assign19620_e24349_d_n2;
        locals.var_t11_dn4 = assign19620_e24349_d_n4;
        locals.var_t11_dn5 = assign19620_e24349_d_n5;
        locals.var_t11_dn6 = assign19620_e24349_d_n6;
        locals.var_t11_dn8 = assign19620_e24349_d_n8;
        locals.var_t11_dn10 = assign19620_e24349_d_n10;
        locals.var_t11_dn11 = assign19620_e24349_d_n11;
        locals.var_t11_dn12 = assign19620_e24349_d_n12;
        locals.var_t11_rv = 0.0;

        let (assign19630_e24357, assign19630_e24357_d_n0, assign19630_e24357_d_n2, assign19630_e24357_d_n4, assign19630_e24357_d_n5, assign19630_e24357_d_n6, assign19630_e24357_d_n8, assign19630_e24357_d_n10, assign19630_e24357_d_n11, assign19630_e24357_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
        let assign19630_e24355: f64 = (locals.var_vbs_bnd + locals.var_ty);
        (assign19630_e24355, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign19630_e24357;
        locals.var_t10_dn0 = assign19630_e24357_d_n0;
        locals.var_t10_dn2 = assign19630_e24357_d_n2;
        locals.var_t10_dn4 = assign19630_e24357_d_n4;
        locals.var_t10_dn5 = assign19630_e24357_d_n5;
        locals.var_t10_dn6 = assign19630_e24357_d_n6;
        locals.var_t10_dn8 = assign19630_e24357_d_n8;
        locals.var_t10_dn10 = assign19630_e24357_d_n10;
        locals.var_t10_dn11 = assign19630_e24357_d_n11;
        locals.var_t10_dn12 = assign19630_e24357_d_n12;
        locals.var_t10_rv = 0.0;

        let (assign19640_e24364, assign19640_e24364_d_n0, assign19640_e24364_d_n2, assign19640_e24364_d_n4, assign19640_e24364_d_n5, assign19640_e24364_d_n6, assign19640_e24364_d_n8, assign19640_e24364_d_n10, assign19640_e24364_d_n11, assign19640_e24364_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard345 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign19640_e24364;
        locals.var_t10_dn0 = assign19640_e24364_d_n0;
        locals.var_t10_dn2 = assign19640_e24364_d_n2;
        locals.var_t10_dn4 = assign19640_e24364_d_n4;
        locals.var_t10_dn5 = assign19640_e24364_d_n5;
        locals.var_t10_dn6 = assign19640_e24364_d_n6;
        locals.var_t10_dn8 = assign19640_e24364_d_n8;
        locals.var_t10_dn10 = assign19640_e24364_d_n10;
        locals.var_t10_dn11 = assign19640_e24364_d_n11;
        locals.var_t10_dn12 = assign19640_e24364_d_n12;
        locals.var_t10_rv = 0.0;

        let (assign19650_e24371, assign19650_e24371_d_n0, assign19650_e24371_d_n2, assign19650_e24371_d_n4, assign19650_e24371_d_n5, assign19650_e24371_d_n6, assign19650_e24371_d_n8, assign19650_e24371_d_n10, assign19650_e24371_d_n11, assign19650_e24371_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19650_e24367: f64 = (-locals.var_t10);
        let assign19650_e24369: f64 = (assign19650_e24367 - 1e-12);
        (assign19650_e24369, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn8), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn12),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12,)
    }
};
        locals.var_vxbgmtcl = assign19650_e24371;
        locals.var_vxbgmtcl_dn0 = assign19650_e24371_d_n0;
        locals.var_vxbgmtcl_dn2 = assign19650_e24371_d_n2;
        locals.var_vxbgmtcl_dn4 = assign19650_e24371_d_n4;
        locals.var_vxbgmtcl_dn5 = assign19650_e24371_d_n5;
        locals.var_vxbgmtcl_dn6 = assign19650_e24371_d_n6;
        locals.var_vxbgmtcl_dn8 = assign19650_e24371_d_n8;
        locals.var_vxbgmtcl_dn10 = assign19650_e24371_d_n10;
        locals.var_vxbgmtcl_dn11 = assign19650_e24371_d_n11;
        locals.var_vxbgmtcl_dn12 = assign19650_e24371_d_n12;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign19660_e24377, assign19660_e24377_d_n0, assign19660_e24377_d_n2, assign19660_e24377_d_n4, assign19660_e24377_d_n5, assign19660_e24377_d_n6, assign19660_e24377_d_n8, assign19660_e24377_d_n10, assign19660_e24377_d_n11, assign19660_e24377_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19660_e24375: f64 = (locals.var_cnst0over * locals.var_cox0_inv);
        (assign19660_e24375, (locals.var_cnst0over_dn0 * locals.var_cox0_inv), (locals.var_cnst0over_dn2 * locals.var_cox0_inv), (locals.var_cnst0over_dn4 * locals.var_cox0_inv), (locals.var_cnst0over_dn5 * locals.var_cox0_inv), (locals.var_cnst0over_dn6 * locals.var_cox0_inv), (locals.var_cnst0over_dn8 * locals.var_cox0_inv), (locals.var_cnst0over_dn10 * locals.var_cox0_inv), (locals.var_cnst0over_dn11 * locals.var_cox0_inv), (locals.var_cnst0over_dn12 * locals.var_cox0_inv),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn8, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn12,)
    }
};
        locals.var_fac1 = assign19660_e24377;
        locals.var_fac1_dn0 = assign19660_e24377_d_n0;
        locals.var_fac1_dn2 = assign19660_e24377_d_n2;
        locals.var_fac1_dn4 = assign19660_e24377_d_n4;
        locals.var_fac1_dn5 = assign19660_e24377_d_n5;
        locals.var_fac1_dn6 = assign19660_e24377_d_n6;
        locals.var_fac1_dn8 = assign19660_e24377_d_n8;
        locals.var_fac1_dn10 = assign19660_e24377_d_n10;
        locals.var_fac1_dn11 = assign19660_e24377_d_n11;
        locals.var_fac1_dn12 = assign19660_e24377_d_n12;
        locals.var_fac1_rv = 0.0;

        let (assign19670_e24383, assign19670_e24383_d_n0, assign19670_e24383_d_n2, assign19670_e24383_d_n4, assign19670_e24383_d_n5, assign19670_e24383_d_n6, assign19670_e24383_d_n8, assign19670_e24383_d_n10, assign19670_e24383_d_n11, assign19670_e24383_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19670_e24381: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign19670_e24381, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn12 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn12)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn8, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn12,)
    }
};
        locals.var_fac1p2 = assign19670_e24383;
        locals.var_fac1p2_dn0 = assign19670_e24383_d_n0;
        locals.var_fac1p2_dn2 = assign19670_e24383_d_n2;
        locals.var_fac1p2_dn4 = assign19670_e24383_d_n4;
        locals.var_fac1p2_dn5 = assign19670_e24383_d_n5;
        locals.var_fac1p2_dn6 = assign19670_e24383_d_n6;
        locals.var_fac1p2_dn8 = assign19670_e24383_d_n8;
        locals.var_fac1p2_dn10 = assign19670_e24383_d_n10;
        locals.var_fac1p2_dn11 = assign19670_e24383_d_n11;
        locals.var_fac1p2_dn12 = assign19670_e24383_d_n12;
        locals.var_fac1p2_rv = 0.0;

        let (assign19680_e24390, assign19680_e24390_d_n0, assign19680_e24390_d_n2, assign19680_e24390_d_n5,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19680_e24386: f64 = (-locals.var_vgbgmt);
        let assign19680_e24388: f64 = (assign19680_e24386 + p.p39);
        (assign19680_e24388, (-locals.var_vgbgmt_dn0), (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn5),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn0, locals.var_vgpld_dn2, locals.var_vgpld_dn5,)
    }
};
        locals.var_vgpld = assign19680_e24390;
        locals.var_vgpld_dn0 = assign19680_e24390_d_n0;
        locals.var_vgpld_dn2 = assign19680_e24390_d_n2;
        locals.var_vgpld_dn5 = assign19680_e24390_d_n5;
        locals.var_vgpld_rv = 0.0;

        let (assign19690_e24401, assign19690_e24401_d_n0, assign19690_e24401_d_n2, assign19690_e24401_d_n4, assign19690_e24401_d_n5, assign19690_e24401_d_n6, assign19690_e24401_d_n8, assign19690_e24401_d_n10, assign19690_e24401_d_n11, assign19690_e24401_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19690_e24394: f64 = (2.0 / locals.var_beta);
        let assign19690_e24397: f64 = (locals.var_mks_nover / locals.var_nin);
        let assign19690_e24398: f64 = (assign19690_e24397).ln();
        let assign19690_e24399: f64 = (assign19690_e24394 * assign19690_e24398);
        (assign19690_e24399, (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (((-((2.0 * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign19690_e24398) + (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign19690_e24397))), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn4, locals.var_pb2over_dn5, locals.var_pb2over_dn6, locals.var_pb2over_dn8, locals.var_pb2over_dn10, locals.var_pb2over_dn11, locals.var_pb2over_dn12,)
    }
};
        locals.var_pb2over = assign19690_e24401;
        locals.var_pb2over_dn0 = assign19690_e24401_d_n0;
        locals.var_pb2over_dn2 = assign19690_e24401_d_n2;
        locals.var_pb2over_dn4 = assign19690_e24401_d_n4;
        locals.var_pb2over_dn5 = assign19690_e24401_d_n5;
        locals.var_pb2over_dn6 = assign19690_e24401_d_n6;
        locals.var_pb2over_dn8 = assign19690_e24401_d_n8;
        locals.var_pb2over_dn10 = assign19690_e24401_d_n10;
        locals.var_pb2over_dn11 = assign19690_e24401_d_n11;
        locals.var_pb2over_dn12 = assign19690_e24401_d_n12;
        locals.var_pb2over_rv = 0.0;

        let (assign19700_e24406, assign19700_e24406_d_n0, assign19700_e24406_d_n2, assign19700_e24406_d_n4, assign19700_e24406_d_n5, assign19700_e24406_d_n6, assign19700_e24406_d_n8, assign19700_e24406_d_n10, assign19700_e24406_d_n11, assign19700_e24406_d_n12,) = {
    if (locals.var_guard327 != 0.0) {
        let assign19700_e24404: f64 = (-locals.var_vxbgmtcl);
        (assign19700_e24404, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn12,)
    }
};
        locals.var_vgb_fb_ld = assign19700_e24406;
        locals.var_vgb_fb_ld_dn0 = assign19700_e24406_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign19700_e24406_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign19700_e24406_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign19700_e24406_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign19700_e24406_d_n6;
        locals.var_vgb_fb_ld_dn8 = assign19700_e24406_d_n8;
        locals.var_vgb_fb_ld_dn10 = assign19700_e24406_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign19700_e24406_d_n11;
        locals.var_vgb_fb_ld_dn12 = assign19700_e24406_d_n12;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign19710_e24409: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard346 = assign19710_e24409;
        locals.var_guard346_rv = 0.0;

        let (assign19730_e24426, assign19730_e24426_d_n0, assign19730_e24426_d_n2, assign19730_e24426_d_n4, assign19730_e24426_d_n5, assign19730_e24426_d_n6, assign19730_e24426_d_n8, assign19730_e24426_d_n10, assign19730_e24426_d_n11, assign19730_e24426_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19730_e24423: f64 = (locals.var_beta * locals.var_cnst0over);
        let assign19730_e24424: f64 = (locals.var_cox0 / assign19730_e24423);
        (assign19730_e24424, (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn0)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn2)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * ((locals.var_beta_dn4 * locals.var_cnst0over) + (locals.var_beta * locals.var_cnst0over_dn4))) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn5)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn6)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn8)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn10)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn11)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn12)) / (assign19730_e24423 * assign19730_e24423))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12,)
    }
};
        locals.var_ty = assign19730_e24426;
        locals.var_ty_dn0 = assign19730_e24426_d_n0;
        locals.var_ty_dn2 = assign19730_e24426_d_n2;
        locals.var_ty_dn4 = assign19730_e24426_d_n4;
        locals.var_ty_dn5 = assign19730_e24426_d_n5;
        locals.var_ty_dn6 = assign19730_e24426_d_n6;
        locals.var_ty_dn8 = assign19730_e24426_d_n8;
        locals.var_ty_dn10 = assign19730_e24426_d_n10;
        locals.var_ty_dn11 = assign19730_e24426_d_n11;
        locals.var_ty_dn12 = assign19730_e24426_d_n12;
        locals.var_ty_rv = 0.0;

        let (assign19740_e24438, assign19740_e24438_d_n0, assign19740_e24438_d_n2, assign19740_e24438_d_n4, assign19740_e24438_d_n5, assign19740_e24438_d_n6, assign19740_e24438_d_n8, assign19740_e24438_d_n10, assign19740_e24438_d_n11, assign19740_e24438_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19740_e24433: f64 = (3.0 * 1.414213562373095);
        let assign19740_e24435: f64 = (assign19740_e24433 * locals.var_ty);
        let assign19740_e24436: f64 = (2.0 + assign19740_e24435);
        (assign19740_e24436, (assign19740_e24433 * locals.var_ty_dn0), (assign19740_e24433 * locals.var_ty_dn2), (assign19740_e24433 * locals.var_ty_dn4), (assign19740_e24433 * locals.var_ty_dn5), (assign19740_e24433 * locals.var_ty_dn6), (assign19740_e24433 * locals.var_ty_dn8), (assign19740_e24433 * locals.var_ty_dn10), (assign19740_e24433 * locals.var_ty_dn11), (assign19740_e24433 * locals.var_ty_dn12),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn8, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12,)
    }
};
        locals.var_ac41 = assign19740_e24438;
        locals.var_ac41_dn0 = assign19740_e24438_d_n0;
        locals.var_ac41_dn2 = assign19740_e24438_d_n2;
        locals.var_ac41_dn4 = assign19740_e24438_d_n4;
        locals.var_ac41_dn5 = assign19740_e24438_d_n5;
        locals.var_ac41_dn6 = assign19740_e24438_d_n6;
        locals.var_ac41_dn8 = assign19740_e24438_d_n8;
        locals.var_ac41_dn10 = assign19740_e24438_d_n10;
        locals.var_ac41_dn11 = assign19740_e24438_d_n11;
        locals.var_ac41_dn12 = assign19740_e24438_d_n12;
        locals.var_ac41_rv = 0.0;

        let (assign19750_e24450, assign19750_e24450_d_n0, assign19750_e24450_d_n2, assign19750_e24450_d_n4, assign19750_e24450_d_n5, assign19750_e24450_d_n6, assign19750_e24450_d_n8, assign19750_e24450_d_n10, assign19750_e24450_d_n11, assign19750_e24450_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19750_e24444: f64 = (8.0 * locals.var_ac41);
        let assign19750_e24446: f64 = (assign19750_e24444 * locals.var_ac41);
        let assign19750_e24448: f64 = (assign19750_e24446 * locals.var_ac41);
        (assign19750_e24448, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn12)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn8, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12,)
    }
};
        locals.var_ac4 = assign19750_e24450;
        locals.var_ac4_dn0 = assign19750_e24450_d_n0;
        locals.var_ac4_dn2 = assign19750_e24450_d_n2;
        locals.var_ac4_dn4 = assign19750_e24450_d_n4;
        locals.var_ac4_dn5 = assign19750_e24450_d_n5;
        locals.var_ac4_dn6 = assign19750_e24450_d_n6;
        locals.var_ac4_dn8 = assign19750_e24450_d_n8;
        locals.var_ac4_dn10 = assign19750_e24450_d_n10;
        locals.var_ac4_dn11 = assign19750_e24450_d_n11;
        locals.var_ac4_dn12 = assign19750_e24450_d_n12;
        locals.var_ac4_rv = 0.0;

        let (assign19760_e24458, assign19760_e24458_d_n0, assign19760_e24458_d_n2, assign19760_e24458_d_n4, assign19760_e24458_d_n5, assign19760_e24458_d_n6, assign19760_e24458_d_n8, assign19760_e24458_d_n10, assign19760_e24458_d_n11, assign19760_e24458_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19760_e24456: f64 = (locals.var_eg - locals.var_pb2over);
        (assign19760_e24456, (locals.var_eg_dn0 - locals.var_pb2over_dn0), (locals.var_eg_dn2 - locals.var_pb2over_dn2), (locals.var_eg_dn4 - locals.var_pb2over_dn4), (locals.var_eg_dn5 - locals.var_pb2over_dn5), (locals.var_eg_dn6 - locals.var_pb2over_dn6), (locals.var_eg_dn8 - locals.var_pb2over_dn8), (locals.var_eg_dn10 - locals.var_pb2over_dn10), (locals.var_eg_dn11 - locals.var_pb2over_dn11), (locals.var_eg_dn12 - locals.var_pb2over_dn12),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn8, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12,)
    }
};
        locals.var_ps0_min = assign19760_e24458;
        locals.var_ps0_min_dn0 = assign19760_e24458_d_n0;
        locals.var_ps0_min_dn2 = assign19760_e24458_d_n2;
        locals.var_ps0_min_dn4 = assign19760_e24458_d_n4;
        locals.var_ps0_min_dn5 = assign19760_e24458_d_n5;
        locals.var_ps0_min_dn6 = assign19760_e24458_d_n6;
        locals.var_ps0_min_dn8 = assign19760_e24458_d_n8;
        locals.var_ps0_min_dn10 = assign19760_e24458_d_n10;
        locals.var_ps0_min_dn11 = assign19760_e24458_d_n11;
        locals.var_ps0_min_dn12 = assign19760_e24458_d_n12;
        locals.var_ps0_min_rv = 0.0;

        let (assign19770_e24468, assign19770_e24468_d_n0, assign19770_e24468_d_n2, assign19770_e24468_d_n4, assign19770_e24468_d_n5, assign19770_e24468_d_n6, assign19770_e24468_d_n8, assign19770_e24468_d_n10, assign19770_e24468_d_n11, assign19770_e24468_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19770_e24465: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign19770_e24466: f64 = (locals.var_beta * assign19770_e24465);
        (assign19770_e24466, (locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign19770_e24465) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), (locals.var_beta * (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * locals.var_vxbgmtcl_dn6), (locals.var_beta * locals.var_vxbgmtcl_dn8), (locals.var_beta * locals.var_vxbgmtcl_dn10), (locals.var_beta * locals.var_vxbgmtcl_dn11), (locals.var_beta * locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign19770_e24468;
        locals.var_tx_dn0 = assign19770_e24468_d_n0;
        locals.var_tx_dn2 = assign19770_e24468_d_n2;
        locals.var_tx_dn4 = assign19770_e24468_d_n4;
        locals.var_tx_dn5 = assign19770_e24468_d_n5;
        locals.var_tx_dn6 = assign19770_e24468_d_n6;
        locals.var_tx_dn8 = assign19770_e24468_d_n8;
        locals.var_tx_dn10 = assign19770_e24468_d_n10;
        locals.var_tx_dn11 = assign19770_e24468_d_n11;
        locals.var_tx_dn12 = assign19770_e24468_d_n12;
        locals.var_tx_rv = 0.0;

        let (assign19780_e24484, assign19780_e24484_d_n0, assign19780_e24484_d_n2, assign19780_e24484_d_n4, assign19780_e24484_d_n5, assign19780_e24484_d_n6, assign19780_e24484_d_n8, assign19780_e24484_d_n10, assign19780_e24484_d_n11, assign19780_e24484_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19780_e24474: f64 = (7.0 * 1.414213562373095);
        let assign19780_e24477: f64 = (9.0 * locals.var_ty);
        let assign19780_e24480: f64 = (locals.var_tx - 2.0);
        let assign19780_e24481: f64 = (assign19780_e24477 * assign19780_e24480);
        let assign19780_e24482: f64 = (assign19780_e24474 - assign19780_e24481);
        (assign19780_e24482, (-(((9.0 * locals.var_ty_dn0) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn8) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn10) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn12) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn12))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn8, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12,)
    }
};
        locals.var_ac31 = assign19780_e24484;
        locals.var_ac31_dn0 = assign19780_e24484_d_n0;
        locals.var_ac31_dn2 = assign19780_e24484_d_n2;
        locals.var_ac31_dn4 = assign19780_e24484_d_n4;
        locals.var_ac31_dn5 = assign19780_e24484_d_n5;
        locals.var_ac31_dn6 = assign19780_e24484_d_n6;
        locals.var_ac31_dn8 = assign19780_e24484_d_n8;
        locals.var_ac31_dn10 = assign19780_e24484_d_n10;
        locals.var_ac31_dn11 = assign19780_e24484_d_n11;
        locals.var_ac31_dn12 = assign19780_e24484_d_n12;
        locals.var_ac31_rv = 0.0;

        let (assign19790_e24492, assign19790_e24492_d_n0, assign19790_e24492_d_n2, assign19790_e24492_d_n4, assign19790_e24492_d_n5, assign19790_e24492_d_n6, assign19790_e24492_d_n8, assign19790_e24492_d_n10, assign19790_e24492_d_n11, assign19790_e24492_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19790_e24490: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign19790_e24490, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn8, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12,)
    }
};
        locals.var_ac3 = assign19790_e24492;
        locals.var_ac3_dn0 = assign19790_e24492_d_n0;
        locals.var_ac3_dn2 = assign19790_e24492_d_n2;
        locals.var_ac3_dn4 = assign19790_e24492_d_n4;
        locals.var_ac3_dn5 = assign19790_e24492_d_n5;
        locals.var_ac3_dn6 = assign19790_e24492_d_n6;
        locals.var_ac3_dn8 = assign19790_e24492_d_n8;
        locals.var_ac3_dn10 = assign19790_e24492_d_n10;
        locals.var_ac3_dn11 = assign19790_e24492_d_n11;
        locals.var_ac3_dn12 = assign19790_e24492_d_n12;
        locals.var_ac3_rv = 0.0;

        let assign19800_e24496: f64 = (locals.var_ac3 * 1e-8);
        let assign19800_e24497: f64 = if locals.var_ac4 < assign19800_e24496 { 1.0 } else { 0.0 };
        locals.var_guard347 = assign19800_e24497;
        locals.var_guard347_rv = 0.0;

        let (assign19810_e24524, assign19810_e24524_d_n0, assign19810_e24524_d_n2, assign19810_e24524_d_n4, assign19810_e24524_d_n5, assign19810_e24524_d_n6, assign19810_e24524_d_n8, assign19810_e24524_d_n10, assign19810_e24524_d_n11, assign19810_e24524_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 != 0.0)) {
        let assign19810_e24504: f64 = (-7.0);
        let assign19810_e24506: f64 = (assign19810_e24504 * 1.414213562373095);
        let assign19810_e24508: f64 = (assign19810_e24506 + locals.var_ac31);
        let assign19810_e24511: f64 = (0.5 * locals.var_ac4);
        let assign19810_e24513: f64 = (assign19810_e24511 / locals.var_ac31);
        let assign19810_e24514: f64 = (assign19810_e24508 + assign19810_e24513);
        let assign19810_e24517: f64 = (9.0 * locals.var_ty);
        let assign19810_e24520: f64 = (locals.var_tx - 2.0);
        let assign19810_e24521: f64 = (assign19810_e24517 * assign19810_e24520);
        let assign19810_e24522: f64 = (assign19810_e24514 + assign19810_e24521);
        (assign19810_e24522, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn0) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn0))), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn2) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn2))), ((locals.var_ac31_dn4 + ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn4) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn4))), ((locals.var_ac31_dn5 + ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn5) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn5))), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn6) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn6))), ((locals.var_ac31_dn8 + ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn8) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn8))), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn10) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn10))), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn11) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn11))), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn12) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn12))),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn8, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12,)
    }
};
        locals.var_ac1 = assign19810_e24524;
        locals.var_ac1_dn0 = assign19810_e24524_d_n0;
        locals.var_ac1_dn2 = assign19810_e24524_d_n2;
        locals.var_ac1_dn4 = assign19810_e24524_d_n4;
        locals.var_ac1_dn5 = assign19810_e24524_d_n5;
        locals.var_ac1_dn6 = assign19810_e24524_d_n6;
        locals.var_ac1_dn8 = assign19810_e24524_d_n8;
        locals.var_ac1_dn10 = assign19810_e24524_d_n10;
        locals.var_ac1_dn11 = assign19810_e24524_d_n11;
        locals.var_ac1_dn12 = assign19810_e24524_d_n12;
        locals.var_ac1_rv = 0.0;

        let (assign19820_e24536, assign19820_e24536_d_n0, assign19820_e24536_d_n2, assign19820_e24536_d_n4, assign19820_e24536_d_n5, assign19820_e24536_d_n6, assign19820_e24536_d_n8, assign19820_e24536_d_n10, assign19820_e24536_d_n11, assign19820_e24536_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 == 0.0)) {
        let assign19820_e24533: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign19820_e24534: f64 = (assign19820_e24533).sqrt();
        (assign19820_e24534, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign19820_e24534)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn8, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12,)
    }
};
        locals.var_ac2 = assign19820_e24536;
        locals.var_ac2_dn0 = assign19820_e24536_d_n0;
        locals.var_ac2_dn2 = assign19820_e24536_d_n2;
        locals.var_ac2_dn4 = assign19820_e24536_d_n4;
        locals.var_ac2_dn5 = assign19820_e24536_d_n5;
        locals.var_ac2_dn6 = assign19820_e24536_d_n6;
        locals.var_ac2_dn8 = assign19820_e24536_d_n8;
        locals.var_ac2_dn10 = assign19820_e24536_d_n10;
        locals.var_ac2_dn11 = assign19820_e24536_d_n11;
        locals.var_ac2_dn12 = assign19820_e24536_d_n12;
        locals.var_ac2_rv = 0.0;

        let (assign19830_e24558, assign19830_e24558_d_n0, assign19830_e24558_d_n2, assign19830_e24558_d_n4, assign19830_e24558_d_n5, assign19830_e24558_d_n6, assign19830_e24558_d_n8, assign19830_e24558_d_n10, assign19830_e24558_d_n11, assign19830_e24558_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 == 0.0)) {
        let assign19830_e24544: f64 = (-7.0);
        let assign19830_e24546: f64 = (assign19830_e24544 * 1.414213562373095);
        let assign19830_e24548: f64 = (assign19830_e24546 + locals.var_ac2);
        let assign19830_e24551: f64 = (9.0 * locals.var_ty);
        let assign19830_e24554: f64 = (locals.var_tx - 2.0);
        let assign19830_e24555: f64 = (assign19830_e24551 * assign19830_e24554);
        let assign19830_e24556: f64 = (assign19830_e24548 + assign19830_e24555);
        (assign19830_e24556, (locals.var_ac2_dn0 + (((9.0 * locals.var_ty_dn0) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn0))), (locals.var_ac2_dn2 + (((9.0 * locals.var_ty_dn2) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn2))), (locals.var_ac2_dn4 + (((9.0 * locals.var_ty_dn4) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn4))), (locals.var_ac2_dn5 + (((9.0 * locals.var_ty_dn5) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn5))), (locals.var_ac2_dn6 + (((9.0 * locals.var_ty_dn6) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn6))), (locals.var_ac2_dn8 + (((9.0 * locals.var_ty_dn8) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn8))), (locals.var_ac2_dn10 + (((9.0 * locals.var_ty_dn10) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn10))), (locals.var_ac2_dn11 + (((9.0 * locals.var_ty_dn11) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn11))), (locals.var_ac2_dn12 + (((9.0 * locals.var_ty_dn12) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn12))),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn8, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12,)
    }
};
        locals.var_ac1 = assign19830_e24558;
        locals.var_ac1_dn0 = assign19830_e24558_d_n0;
        locals.var_ac1_dn2 = assign19830_e24558_d_n2;
        locals.var_ac1_dn4 = assign19830_e24558_d_n4;
        locals.var_ac1_dn5 = assign19830_e24558_d_n5;
        locals.var_ac1_dn6 = assign19830_e24558_d_n6;
        locals.var_ac1_dn8 = assign19830_e24558_d_n8;
        locals.var_ac1_dn10 = assign19830_e24558_d_n10;
        locals.var_ac1_dn11 = assign19830_e24558_d_n11;
        locals.var_ac1_dn12 = assign19830_e24558_d_n12;
        locals.var_ac1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_80(
        locals: &mut StampLocals,
    ) {
        let (assign19840_e24566, assign19840_e24566_d_n0, assign19840_e24566_d_n2, assign19840_e24566_d_n4, assign19840_e24566_d_n5, assign19840_e24566_d_n6, assign19840_e24566_d_n8, assign19840_e24566_d_n10, assign19840_e24566_d_n11, assign19840_e24566_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19840_e24564: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign19840_e24564, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn8, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12,)
    }
};
        locals.var_acd = assign19840_e24566;
        locals.var_acd_dn0 = assign19840_e24566_d_n0;
        locals.var_acd_dn2 = assign19840_e24566_d_n2;
        locals.var_acd_dn4 = assign19840_e24566_d_n4;
        locals.var_acd_dn5 = assign19840_e24566_d_n5;
        locals.var_acd_dn6 = assign19840_e24566_d_n6;
        locals.var_acd_dn8 = assign19840_e24566_d_n8;
        locals.var_acd_dn10 = assign19840_e24566_d_n10;
        locals.var_acd_dn11 = assign19840_e24566_d_n11;
        locals.var_acd_dn12 = assign19840_e24566_d_n12;
        locals.var_acd_rv = 0.0;

        let (assign19850_e24589, assign19850_e24589_d_n0, assign19850_e24589_d_n2, assign19850_e24589_d_n4, assign19850_e24589_d_n5, assign19850_e24589_d_n6, assign19850_e24589_d_n8, assign19850_e24589_d_n10, assign19850_e24589_d_n11, assign19850_e24589_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19850_e24571: f64 = (-4.0);
        let assign19850_e24573: f64 = (assign19850_e24571 * 1.414213562373095);
        let assign19850_e24576: f64 = (12.0 * locals.var_ty);
        let assign19850_e24577: f64 = (assign19850_e24573 - assign19850_e24576);
        let assign19850_e24580: f64 = (2.0 * locals.var_acd);
        let assign19850_e24581: f64 = (assign19850_e24577 + assign19850_e24580);
        let assign19850_e24584: f64 = (1.414213562373095 * locals.var_acd);
        let assign19850_e24586: f64 = (assign19850_e24584 * locals.var_acd);
        let assign19850_e24587: f64 = (assign19850_e24581 + assign19850_e24586);
        (assign19850_e24587, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn12))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn8, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12,)
    }
};
        locals.var_acn = assign19850_e24589;
        locals.var_acn_dn0 = assign19850_e24589_d_n0;
        locals.var_acn_dn2 = assign19850_e24589_d_n2;
        locals.var_acn_dn4 = assign19850_e24589_d_n4;
        locals.var_acn_dn5 = assign19850_e24589_d_n5;
        locals.var_acn_dn6 = assign19850_e24589_d_n6;
        locals.var_acn_dn8 = assign19850_e24589_d_n8;
        locals.var_acn_dn10 = assign19850_e24589_d_n10;
        locals.var_acn_dn11 = assign19850_e24589_d_n11;
        locals.var_acn_dn12 = assign19850_e24589_d_n12;
        locals.var_acn_rv = 0.0;

        let (assign19860_e24597, assign19860_e24597_d_n0, assign19860_e24597_d_n2, assign19860_e24597_d_n4, assign19860_e24597_d_n5, assign19860_e24597_d_n6, assign19860_e24597_d_n8, assign19860_e24597_d_n10, assign19860_e24597_d_n11, assign19860_e24597_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19860_e24595: f64 = (locals.var_acn / locals.var_acd);
        (assign19860_e24595, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn12 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn12)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign19860_e24597;
        locals.var_chi_dn0 = assign19860_e24597_d_n0;
        locals.var_chi_dn2 = assign19860_e24597_d_n2;
        locals.var_chi_dn4 = assign19860_e24597_d_n4;
        locals.var_chi_dn5 = assign19860_e24597_d_n5;
        locals.var_chi_dn6 = assign19860_e24597_d_n6;
        locals.var_chi_dn8 = assign19860_e24597_d_n8;
        locals.var_chi_dn10 = assign19860_e24597_d_n10;
        locals.var_chi_dn11 = assign19860_e24597_d_n11;
        locals.var_chi_dn12 = assign19860_e24597_d_n12;
        locals.var_chi_rv = 0.0;

        let (assign19870_e24607, assign19870_e24607_d_n0, assign19870_e24607_d_n2, assign19870_e24607_d_n4, assign19870_e24607_d_n5, assign19870_e24607_d_n6, assign19870_e24607_d_n8, assign19870_e24607_d_n10, assign19870_e24607_d_n11, assign19870_e24607_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19870_e24603: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign19870_e24605: f64 = (assign19870_e24603 - locals.var_vxbgmtcl);
        (assign19870_e24605, ((locals.var_chi_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_dn5 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_dn8 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_dn10 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn4, locals.var_psa_dn5, locals.var_psa_dn6, locals.var_psa_dn8, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12,)
    }
};
        locals.var_psa = assign19870_e24607;
        locals.var_psa_dn0 = assign19870_e24607_d_n0;
        locals.var_psa_dn2 = assign19870_e24607_d_n2;
        locals.var_psa_dn4 = assign19870_e24607_d_n4;
        locals.var_psa_dn5 = assign19870_e24607_d_n5;
        locals.var_psa_dn6 = assign19870_e24607_d_n6;
        locals.var_psa_dn8 = assign19870_e24607_d_n8;
        locals.var_psa_dn10 = assign19870_e24607_d_n10;
        locals.var_psa_dn11 = assign19870_e24607_d_n11;
        locals.var_psa_dn12 = assign19870_e24607_d_n12;
        locals.var_psa_rv = 0.0;

        let (assign19880_e24615, assign19880_e24615_d_n0, assign19880_e24615_d_n2, assign19880_e24615_d_n4, assign19880_e24615_d_n5, assign19880_e24615_d_n6, assign19880_e24615_d_n8, assign19880_e24615_d_n10, assign19880_e24615_d_n11, assign19880_e24615_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19880_e24613: f64 = (locals.var_psa + locals.var_vxbgmtcl);
        (assign19880_e24613, (locals.var_psa_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_psa_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_psa_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_psa_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_psa_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_psa_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_psa_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_psa_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_psa_dn12 + locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign19880_e24615;
        locals.var_t1_dn0 = assign19880_e24615_d_n0;
        locals.var_t1_dn2 = assign19880_e24615_d_n2;
        locals.var_t1_dn4 = assign19880_e24615_d_n4;
        locals.var_t1_dn5 = assign19880_e24615_d_n5;
        locals.var_t1_dn6 = assign19880_e24615_d_n6;
        locals.var_t1_dn8 = assign19880_e24615_d_n8;
        locals.var_t1_dn10 = assign19880_e24615_d_n10;
        locals.var_t1_dn11 = assign19880_e24615_d_n11;
        locals.var_t1_dn12 = assign19880_e24615_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign19890_e24623, assign19890_e24623_d_n0, assign19890_e24623_d_n2, assign19890_e24623_d_n4, assign19890_e24623_d_n5, assign19890_e24623_d_n6, assign19890_e24623_d_n8, assign19890_e24623_d_n10, assign19890_e24623_d_n11, assign19890_e24623_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19890_e24621: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign19890_e24621, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign19890_e24623;
        locals.var_t2_dn0 = assign19890_e24623_d_n0;
        locals.var_t2_dn2 = assign19890_e24623_d_n2;
        locals.var_t2_dn4 = assign19890_e24623_d_n4;
        locals.var_t2_dn5 = assign19890_e24623_d_n5;
        locals.var_t2_dn6 = assign19890_e24623_d_n6;
        locals.var_t2_dn8 = assign19890_e24623_d_n8;
        locals.var_t2_dn10 = assign19890_e24623_d_n10;
        locals.var_t2_dn11 = assign19890_e24623_d_n11;
        locals.var_t2_dn12 = assign19890_e24623_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign19900_e24638, assign19900_e24638_d_n0, assign19900_e24638_d_n2, assign19900_e24638_d_n4, assign19900_e24638_d_n5, assign19900_e24638_d_n6, assign19900_e24638_d_n8, assign19900_e24638_d_n10, assign19900_e24638_d_n11, assign19900_e24638_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19900_e24631: f64 = (locals.var_t2 * locals.var_t2);
        let assign19900_e24632: f64 = (1.0 + assign19900_e24631);
        let assign19900_e24633: f64 = (assign19900_e24632).sqrt();
        let assign19900_e24634: f64 = (locals.var_t1 / assign19900_e24633);
        let assign19900_e24636: f64 = (assign19900_e24634 - locals.var_vxbgmtcl);
        (assign19900_e24636, ((((locals.var_t1_dn0 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn8 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn10 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn12 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn8, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12,)
    }
};
        locals.var_ps0ld = assign19900_e24638;
        locals.var_ps0ld_dn0 = assign19900_e24638_d_n0;
        locals.var_ps0ld_dn2 = assign19900_e24638_d_n2;
        locals.var_ps0ld_dn4 = assign19900_e24638_d_n4;
        locals.var_ps0ld_dn5 = assign19900_e24638_d_n5;
        locals.var_ps0ld_dn6 = assign19900_e24638_d_n6;
        locals.var_ps0ld_dn8 = assign19900_e24638_d_n8;
        locals.var_ps0ld_dn10 = assign19900_e24638_d_n10;
        locals.var_ps0ld_dn11 = assign19900_e24638_d_n11;
        locals.var_ps0ld_dn12 = assign19900_e24638_d_n12;
        locals.var_ps0ld_rv = 0.0;

        let (assign19910_e24648, assign19910_e24648_d_n0, assign19910_e24648_d_n2, assign19910_e24648_d_n4, assign19910_e24648_d_n5, assign19910_e24648_d_n6, assign19910_e24648_d_n8, assign19910_e24648_d_n10, assign19910_e24648_d_n11, assign19910_e24648_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign19910_e24645: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign19910_e24646: f64 = (locals.var_cox0 * assign19910_e24645);
        (assign19910_e24646, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (-locals.var_ps0ld_dn4)), (locals.var_cox0 * (locals.var_vgpld_dn5 - locals.var_ps0ld_dn5)), (locals.var_cox0 * (-locals.var_ps0ld_dn6)), (locals.var_cox0 * (-locals.var_ps0ld_dn8)), (locals.var_cox0 * (-locals.var_ps0ld_dn10)), (locals.var_cox0 * (-locals.var_ps0ld_dn11)), (locals.var_cox0 * (-locals.var_ps0ld_dn12)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12,)
    }
};
        locals.var_qsuld = assign19910_e24648;
        locals.var_qsuld_dn0 = assign19910_e24648_d_n0;
        locals.var_qsuld_dn2 = assign19910_e24648_d_n2;
        locals.var_qsuld_dn4 = assign19910_e24648_d_n4;
        locals.var_qsuld_dn5 = assign19910_e24648_d_n5;
        locals.var_qsuld_dn6 = assign19910_e24648_d_n6;
        locals.var_qsuld_dn8 = assign19910_e24648_d_n8;
        locals.var_qsuld_dn10 = assign19910_e24648_d_n10;
        locals.var_qsuld_dn11 = assign19910_e24648_d_n11;
        locals.var_qsuld_dn12 = assign19910_e24648_d_n12;
        locals.var_qsuld_rv = 0.0;

        let (assign19920_e24654, assign19920_e24654_d_n0, assign19920_e24654_d_n2, assign19920_e24654_d_n4, assign19920_e24654_d_n5, assign19920_e24654_d_n6, assign19920_e24654_d_n8, assign19920_e24654_d_n10, assign19920_e24654_d_n11, assign19920_e24654_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn8, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12,)
    }
};
        locals.var_qbuld = assign19920_e24654;
        locals.var_qbuld_dn0 = assign19920_e24654_d_n0;
        locals.var_qbuld_dn2 = assign19920_e24654_d_n2;
        locals.var_qbuld_dn4 = assign19920_e24654_d_n4;
        locals.var_qbuld_dn5 = assign19920_e24654_d_n5;
        locals.var_qbuld_dn6 = assign19920_e24654_d_n6;
        locals.var_qbuld_dn8 = assign19920_e24654_d_n8;
        locals.var_qbuld_dn10 = assign19920_e24654_d_n10;
        locals.var_qbuld_dn11 = assign19920_e24654_d_n11;
        locals.var_qbuld_dn12 = assign19920_e24654_d_n12;
        locals.var_qbuld_rv = 0.0;

        let (assign19940_e24668, assign19940_e24668_d_n0, assign19940_e24668_d_n2, assign19940_e24668_d_n4, assign19940_e24668_d_n5, assign19940_e24668_d_n6, assign19940_e24668_d_n8, assign19940_e24668_d_n10, assign19940_e24668_d_n11, assign19940_e24668_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign19940_e24668;
        locals.var_chi_dn0 = assign19940_e24668_d_n0;
        locals.var_chi_dn2 = assign19940_e24668_d_n2;
        locals.var_chi_dn4 = assign19940_e24668_d_n4;
        locals.var_chi_dn5 = assign19940_e24668_d_n5;
        locals.var_chi_dn6 = assign19940_e24668_d_n6;
        locals.var_chi_dn8 = assign19940_e24668_d_n8;
        locals.var_chi_dn10 = assign19940_e24668_d_n10;
        locals.var_chi_dn11 = assign19940_e24668_d_n11;
        locals.var_chi_dn12 = assign19940_e24668_d_n12;
        locals.var_chi_rv = 0.0;

        let (assign19950_e24679, assign19950_e24679_d_n0, assign19950_e24679_d_n2, assign19950_e24679_d_n4, assign19950_e24679_d_n5, assign19950_e24679_d_n6, assign19950_e24679_d_n8, assign19950_e24679_d_n10, assign19950_e24679_d_n11, assign19950_e24679_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign19950_e24675: f64 = (locals.var_chi / locals.var_beta);
        let assign19950_e24677: f64 = (assign19950_e24675 - locals.var_vxbgmtcl);
        (assign19950_e24677, ((locals.var_chi_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((((locals.var_chi_dn4 * locals.var_beta) - (locals.var_chi * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_dn5 / locals.var_beta) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_dn8 / locals.var_beta) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_dn10 / locals.var_beta) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12,)
    }
};
        locals.var_ps0_inia = assign19950_e24679;
        locals.var_ps0_inia_dn0 = assign19950_e24679_d_n0;
        locals.var_ps0_inia_dn2 = assign19950_e24679_d_n2;
        locals.var_ps0_inia_dn4 = assign19950_e24679_d_n4;
        locals.var_ps0_inia_dn5 = assign19950_e24679_d_n5;
        locals.var_ps0_inia_dn6 = assign19950_e24679_d_n6;
        locals.var_ps0_inia_dn8 = assign19950_e24679_d_n8;
        locals.var_ps0_inia_dn10 = assign19950_e24679_d_n10;
        locals.var_ps0_inia_dn11 = assign19950_e24679_d_n11;
        locals.var_ps0_inia_dn12 = assign19950_e24679_d_n12;
        locals.var_ps0_inia_rv = 0.0;

        let (assign19960_e24704, assign19960_e24704_d_n0, assign19960_e24704_d_n2, assign19960_e24704_d_n4, assign19960_e24704_d_n5, assign19960_e24704_d_n6, assign19960_e24704_d_n8, assign19960_e24704_d_n10, assign19960_e24704_d_n11, assign19960_e24704_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign19960_e24689: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign19960_e24690: f64 = (locals.var_beta * assign19960_e24689);
        let assign19960_e24692: f64 = (assign19960_e24690 - 1.0);
        let assign19960_e24694: f64 = (-locals.var_chi);
        let assign19960_e24695: f64 = (assign19960_e24694).exp();
        let assign19960_e24696: f64 = (assign19960_e24692 + assign19960_e24695);
        let assign19960_e24697: f64 = (4.0 * assign19960_e24696);
        let assign19960_e24700: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign19960_e24701: f64 = (assign19960_e24697 / assign19960_e24700);
        let assign19960_e24702: f64 = (1.0 + assign19960_e24701);
        (assign19960_e24702, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + (assign19960_e24695 * (-locals.var_chi_dn0)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + (assign19960_e24695 * (-locals.var_chi_dn2)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * (((locals.var_beta_dn4 * assign19960_e24689) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + (assign19960_e24695 * (-locals.var_chi_dn4)))) * assign19960_e24700) - (assign19960_e24697 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)) + (assign19960_e24695 * (-locals.var_chi_dn5)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn5 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn6) + (assign19960_e24695 * (-locals.var_chi_dn6)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn8) + (assign19960_e24695 * (-locals.var_chi_dn8)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn8 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn10) + (assign19960_e24695 * (-locals.var_chi_dn10)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn10 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn11) + (assign19960_e24695 * (-locals.var_chi_dn11)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn12) + (assign19960_e24695 * (-locals.var_chi_dn12)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign19960_e24704;
        locals.var_tx_dn0 = assign19960_e24704_d_n0;
        locals.var_tx_dn2 = assign19960_e24704_d_n2;
        locals.var_tx_dn4 = assign19960_e24704_d_n4;
        locals.var_tx_dn5 = assign19960_e24704_d_n5;
        locals.var_tx_dn6 = assign19960_e24704_d_n6;
        locals.var_tx_dn8 = assign19960_e24704_d_n8;
        locals.var_tx_dn10 = assign19960_e24704_d_n10;
        locals.var_tx_dn11 = assign19960_e24704_d_n11;
        locals.var_tx_dn12 = assign19960_e24704_d_n12;
        locals.var_tx_rv = 0.0;

        let assign19970_e24708: f64 = (10.0 * 2.220446049250313e-16);
        let assign19970_e24709: f64 = if locals.var_tx < assign19970_e24708 { 1.0 } else { 0.0 };
        locals.var_guard348 = assign19970_e24709;
        locals.var_guard348_rv = 0.0;

        let (assign19980_e24720, assign19980_e24720_d_n0, assign19980_e24720_d_n2, assign19980_e24720_d_n4, assign19980_e24720_d_n5, assign19980_e24720_d_n6, assign19980_e24720_d_n8, assign19980_e24720_d_n10, assign19980_e24720_d_n11, assign19980_e24720_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard348 != 0.0)) {
        let assign19980_e24718: f64 = (10.0 * 2.220446049250313e-16);
        (assign19980_e24718, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign19980_e24720;
        locals.var_tx_dn0 = assign19980_e24720_d_n0;
        locals.var_tx_dn2 = assign19980_e24720_d_n2;
        locals.var_tx_dn4 = assign19980_e24720_d_n4;
        locals.var_tx_dn5 = assign19980_e24720_d_n5;
        locals.var_tx_dn6 = assign19980_e24720_d_n6;
        locals.var_tx_dn8 = assign19980_e24720_d_n8;
        locals.var_tx_dn10 = assign19980_e24720_d_n10;
        locals.var_tx_dn11 = assign19980_e24720_d_n11;
        locals.var_tx_dn12 = assign19980_e24720_d_n12;
        locals.var_tx_rv = 0.0;

        let (assign19990_e24738, assign19990_e24738_d_n0, assign19990_e24738_d_n2, assign19990_e24738_d_n4, assign19990_e24738_d_n5, assign19990_e24738_d_n6, assign19990_e24738_d_n8, assign19990_e24738_d_n10, assign19990_e24738_d_n11, assign19990_e24738_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign19990_e24728: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign19990_e24730: f64 = (assign19990_e24728 / 2.0);
        let assign19990_e24733: f64 = (locals.var_tx).sqrt();
        let assign19990_e24734: f64 = (1.0 - assign19990_e24733);
        let assign19990_e24735: f64 = (assign19990_e24730 * assign19990_e24734);
        let assign19990_e24736: f64 = (locals.var_vgpld + assign19990_e24735);
        (assign19990_e24736, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn0 / (2.0 * assign19990_e24733)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn2 / (2.0 * assign19990_e24733)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn4 / (2.0 * assign19990_e24733))))), (locals.var_vgpld_dn5 + ((((locals.var_fac1p2_dn5 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn5 / (2.0 * assign19990_e24733)))))), ((((locals.var_fac1p2_dn6 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn6 / (2.0 * assign19990_e24733))))), ((((locals.var_fac1p2_dn8 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn8 / (2.0 * assign19990_e24733))))), ((((locals.var_fac1p2_dn10 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn10 / (2.0 * assign19990_e24733))))), ((((locals.var_fac1p2_dn11 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn11 / (2.0 * assign19990_e24733))))), ((((locals.var_fac1p2_dn12 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn12 / (2.0 * assign19990_e24733))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12,)
    }
};
        locals.var_ps0_inia = assign19990_e24738;
        locals.var_ps0_inia_dn0 = assign19990_e24738_d_n0;
        locals.var_ps0_inia_dn2 = assign19990_e24738_d_n2;
        locals.var_ps0_inia_dn4 = assign19990_e24738_d_n4;
        locals.var_ps0_inia_dn5 = assign19990_e24738_d_n5;
        locals.var_ps0_inia_dn6 = assign19990_e24738_d_n6;
        locals.var_ps0_inia_dn8 = assign19990_e24738_d_n8;
        locals.var_ps0_inia_dn10 = assign19990_e24738_d_n10;
        locals.var_ps0_inia_dn11 = assign19990_e24738_d_n11;
        locals.var_ps0_inia_dn12 = assign19990_e24738_d_n12;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20000_e24749, assign20000_e24749_d_n0, assign20000_e24749_d_n2, assign20000_e24749_d_n4, assign20000_e24749_d_n5, assign20000_e24749_d_n6, assign20000_e24749_d_n8, assign20000_e24749_d_n10, assign20000_e24749_d_n11, assign20000_e24749_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign20000_e24746: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign20000_e24747: f64 = (locals.var_beta * assign20000_e24746);
        (assign20000_e24747, (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign20000_e24746) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 + locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign20000_e24749;
        locals.var_chi_dn0 = assign20000_e24749_d_n0;
        locals.var_chi_dn2 = assign20000_e24749_d_n2;
        locals.var_chi_dn4 = assign20000_e24749_d_n4;
        locals.var_chi_dn5 = assign20000_e24749_d_n5;
        locals.var_chi_dn6 = assign20000_e24749_d_n6;
        locals.var_chi_dn8 = assign20000_e24749_d_n8;
        locals.var_chi_dn10 = assign20000_e24749_d_n10;
        locals.var_chi_dn11 = assign20000_e24749_d_n11;
        locals.var_chi_dn12 = assign20000_e24749_d_n12;
        locals.var_chi_rv = 0.0;

        let (assign20010_e24774, assign20010_e24774_d_n0, assign20010_e24774_d_n2, assign20010_e24774_d_n4, assign20010_e24774_d_n5, assign20010_e24774_d_n6, assign20010_e24774_d_n8, assign20010_e24774_d_n10, assign20010_e24774_d_n11, assign20010_e24774_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign20010_e24759: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign20010_e24760: f64 = (locals.var_beta * assign20010_e24759);
        let assign20010_e24762: f64 = (assign20010_e24760 - 1.0);
        let assign20010_e24764: f64 = (-locals.var_chi);
        let assign20010_e24765: f64 = (assign20010_e24764).exp();
        let assign20010_e24766: f64 = (assign20010_e24762 + assign20010_e24765);
        let assign20010_e24767: f64 = (4.0 * assign20010_e24766);
        let assign20010_e24770: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign20010_e24771: f64 = (assign20010_e24767 / assign20010_e24770);
        let assign20010_e24772: f64 = (1.0 + assign20010_e24771);
        (assign20010_e24772, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + (assign20010_e24765 * (-locals.var_chi_dn0)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + (assign20010_e24765 * (-locals.var_chi_dn2)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * (((locals.var_beta_dn4 * assign20010_e24759) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + (assign20010_e24765 * (-locals.var_chi_dn4)))) * assign20010_e24770) - (assign20010_e24767 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)) + (assign20010_e24765 * (-locals.var_chi_dn5)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn5 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn6) + (assign20010_e24765 * (-locals.var_chi_dn6)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn8) + (assign20010_e24765 * (-locals.var_chi_dn8)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn8 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn10) + (assign20010_e24765 * (-locals.var_chi_dn10)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn10 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn11) + (assign20010_e24765 * (-locals.var_chi_dn11)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn12) + (assign20010_e24765 * (-locals.var_chi_dn12)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign20010_e24774;
        locals.var_tx_dn0 = assign20010_e24774_d_n0;
        locals.var_tx_dn2 = assign20010_e24774_d_n2;
        locals.var_tx_dn4 = assign20010_e24774_d_n4;
        locals.var_tx_dn5 = assign20010_e24774_d_n5;
        locals.var_tx_dn6 = assign20010_e24774_d_n6;
        locals.var_tx_dn8 = assign20010_e24774_d_n8;
        locals.var_tx_dn10 = assign20010_e24774_d_n10;
        locals.var_tx_dn11 = assign20010_e24774_d_n11;
        locals.var_tx_dn12 = assign20010_e24774_d_n12;
        locals.var_tx_rv = 0.0;

        let assign20020_e24778: f64 = (10.0 * 2.220446049250313e-16);
        let assign20020_e24779: f64 = if locals.var_tx < assign20020_e24778 { 1.0 } else { 0.0 };
        locals.var_guard349 = assign20020_e24779;
        locals.var_guard349_rv = 0.0;

        let (assign20030_e24790, assign20030_e24790_d_n0, assign20030_e24790_d_n2, assign20030_e24790_d_n4, assign20030_e24790_d_n5, assign20030_e24790_d_n6, assign20030_e24790_d_n8, assign20030_e24790_d_n10, assign20030_e24790_d_n11, assign20030_e24790_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard349 != 0.0)) {
        let assign20030_e24788: f64 = (10.0 * 2.220446049250313e-16);
        (assign20030_e24788, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign20030_e24790;
        locals.var_tx_dn0 = assign20030_e24790_d_n0;
        locals.var_tx_dn2 = assign20030_e24790_d_n2;
        locals.var_tx_dn4 = assign20030_e24790_d_n4;
        locals.var_tx_dn5 = assign20030_e24790_d_n5;
        locals.var_tx_dn6 = assign20030_e24790_d_n6;
        locals.var_tx_dn8 = assign20030_e24790_d_n8;
        locals.var_tx_dn10 = assign20030_e24790_d_n10;
        locals.var_tx_dn11 = assign20030_e24790_d_n11;
        locals.var_tx_dn12 = assign20030_e24790_d_n12;
        locals.var_tx_rv = 0.0;

        let (assign20040_e24808, assign20040_e24808_d_n0, assign20040_e24808_d_n2, assign20040_e24808_d_n4, assign20040_e24808_d_n5, assign20040_e24808_d_n6, assign20040_e24808_d_n8, assign20040_e24808_d_n10, assign20040_e24808_d_n11, assign20040_e24808_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign20040_e24798: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign20040_e24800: f64 = (assign20040_e24798 / 2.0);
        let assign20040_e24803: f64 = (locals.var_tx).sqrt();
        let assign20040_e24804: f64 = (1.0 - assign20040_e24803);
        let assign20040_e24805: f64 = (assign20040_e24800 * assign20040_e24804);
        let assign20040_e24806: f64 = (locals.var_vgpld + assign20040_e24805);
        (assign20040_e24806, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn0 / (2.0 * assign20040_e24803)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn2 / (2.0 * assign20040_e24803)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn4 / (2.0 * assign20040_e24803))))), (locals.var_vgpld_dn5 + ((((locals.var_fac1p2_dn5 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn5 / (2.0 * assign20040_e24803)))))), ((((locals.var_fac1p2_dn6 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn6 / (2.0 * assign20040_e24803))))), ((((locals.var_fac1p2_dn8 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn8 / (2.0 * assign20040_e24803))))), ((((locals.var_fac1p2_dn10 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn10 / (2.0 * assign20040_e24803))))), ((((locals.var_fac1p2_dn11 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn11 / (2.0 * assign20040_e24803))))), ((((locals.var_fac1p2_dn12 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn12 / (2.0 * assign20040_e24803))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12,)
    }
};
        locals.var_ps0_inia = assign20040_e24808;
        locals.var_ps0_inia_dn0 = assign20040_e24808_d_n0;
        locals.var_ps0_inia_dn2 = assign20040_e24808_d_n2;
        locals.var_ps0_inia_dn4 = assign20040_e24808_d_n4;
        locals.var_ps0_inia_dn5 = assign20040_e24808_d_n5;
        locals.var_ps0_inia_dn6 = assign20040_e24808_d_n6;
        locals.var_ps0_inia_dn8 = assign20040_e24808_d_n8;
        locals.var_ps0_inia_dn10 = assign20040_e24808_d_n10;
        locals.var_ps0_inia_dn11 = assign20040_e24808_d_n11;
        locals.var_ps0_inia_dn12 = assign20040_e24808_d_n12;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20050_e24819, assign20050_e24819_d_n0, assign20050_e24819_d_n2, assign20050_e24819_d_n4, assign20050_e24819_d_n5, assign20050_e24819_d_n6, assign20050_e24819_d_n8, assign20050_e24819_d_n10, assign20050_e24819_d_n11, assign20050_e24819_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign20050_e24816: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign20050_e24817: f64 = (locals.var_beta * assign20050_e24816);
        (assign20050_e24817, (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign20050_e24816) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 + locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign20050_e24819;
        locals.var_chi_dn0 = assign20050_e24819_d_n0;
        locals.var_chi_dn2 = assign20050_e24819_d_n2;
        locals.var_chi_dn4 = assign20050_e24819_d_n4;
        locals.var_chi_dn5 = assign20050_e24819_d_n5;
        locals.var_chi_dn6 = assign20050_e24819_d_n6;
        locals.var_chi_dn8 = assign20050_e24819_d_n8;
        locals.var_chi_dn10 = assign20050_e24819_d_n10;
        locals.var_chi_dn11 = assign20050_e24819_d_n11;
        locals.var_chi_dn12 = assign20050_e24819_d_n12;
        locals.var_chi_rv = 0.0;

        let assign20060_e24822: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard350 = assign20060_e24822;
        locals.var_guard350_rv = 0.0;

        let (assign20080_e24857,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20080_e24841: f64 = (9.0 * 1.414213562373095);
        let assign20080_e24842: f64 = (1.0 / assign20080_e24841);
        let assign20080_e24846: f64 = (7.0 * 0.049787068367863944);
        let assign20080_e24847: f64 = (5.0 + assign20080_e24846);
        let assign20080_e24851: f64 = (2.0 + 0.049787068367863944);
        let assign20080_e24852: f64 = (assign20080_e24851).sqrt();
        let assign20080_e24853: f64 = (54.0 * assign20080_e24852);
        let assign20080_e24854: f64 = (assign20080_e24847 / assign20080_e24853);
        let assign20080_e24855: f64 = (assign20080_e24842 - assign20080_e24854);
        (assign20080_e24855,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign20080_e24857;
        locals.var_ta_rv = 0.0;

        let (assign20090_e24879,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20090_e24866: f64 = (1.0 + 0.049787068367863944);
        let assign20090_e24870: f64 = (2.0 + 0.049787068367863944);
        let assign20090_e24871: f64 = (assign20090_e24870).sqrt();
        let assign20090_e24872: f64 = (2.0 * assign20090_e24871);
        let assign20090_e24873: f64 = (assign20090_e24866 / assign20090_e24872);
        let assign20090_e24876: f64 = (1.414213562373095 / 3.0);
        let assign20090_e24877: f64 = (assign20090_e24873 - assign20090_e24876);
        (assign20090_e24877,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign20090_e24879;
        locals.var_tb_rv = 0.0;

        let (assign20100_e24896, assign20100_e24896_d_n0, assign20100_e24896_d_n2, assign20100_e24896_d_n4, assign20100_e24896_d_n5, assign20100_e24896_d_n6, assign20100_e24896_d_n8, assign20100_e24896_d_n10, assign20100_e24896_d_n11, assign20100_e24896_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20100_e24888: f64 = (1.0 / 1.414213562373095);
        let assign20100_e24892: f64 = (locals.var_beta * locals.var_fac1);
        let assign20100_e24893: f64 = (1.0 / assign20100_e24892);
        let assign20100_e24894: f64 = (assign20100_e24888 + assign20100_e24893);
        (assign20100_e24894, (-((locals.var_beta * locals.var_fac1_dn0) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn2) / (assign20100_e24892 * assign20100_e24892))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn5) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn6) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn8) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn10) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn11) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn12) / (assign20100_e24892 * assign20100_e24892))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn8, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn12,)
    }
};
        locals.var_tc = assign20100_e24896;
        locals.var_tc_dn0 = assign20100_e24896_d_n0;
        locals.var_tc_dn2 = assign20100_e24896_d_n2;
        locals.var_tc_dn4 = assign20100_e24896_d_n4;
        locals.var_tc_dn5 = assign20100_e24896_d_n5;
        locals.var_tc_dn6 = assign20100_e24896_d_n6;
        locals.var_tc_dn8 = assign20100_e24896_d_n8;
        locals.var_tc_dn10 = assign20100_e24896_d_n10;
        locals.var_tc_dn11 = assign20100_e24896_d_n11;
        locals.var_tc_dn12 = assign20100_e24896_d_n12;
        locals.var_tc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_81(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20110_e24910, assign20110_e24910_d_n0, assign20110_e24910_d_n2, assign20110_e24910_d_n4, assign20110_e24910_d_n5, assign20110_e24910_d_n6, assign20110_e24910_d_n8, assign20110_e24910_d_n10, assign20110_e24910_d_n11, assign20110_e24910_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20110_e24905: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign20110_e24906: f64 = (-assign20110_e24905);
        let assign20110_e24908: f64 = (assign20110_e24906 / locals.var_fac1);
        (assign20110_e24908, ((((-(locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn8) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn12) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn12)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn8, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn12,)
    }
};
        locals.var_td = assign20110_e24910;
        locals.var_td_dn0 = assign20110_e24910_d_n0;
        locals.var_td_dn2 = assign20110_e24910_d_n2;
        locals.var_td_dn4 = assign20110_e24910_d_n4;
        locals.var_td_dn5 = assign20110_e24910_d_n5;
        locals.var_td_dn6 = assign20110_e24910_d_n6;
        locals.var_td_dn8 = assign20110_e24910_d_n8;
        locals.var_td_dn10 = assign20110_e24910_d_n10;
        locals.var_td_dn11 = assign20110_e24910_d_n11;
        locals.var_td_dn12 = assign20110_e24910_d_n12;
        locals.var_td_rv = 0.0;

        let (assign20120_e24947, assign20120_e24947_d_n0, assign20120_e24947_d_n2, assign20120_e24947_d_n4, assign20120_e24947_d_n5, assign20120_e24947_d_n6, assign20120_e24947_d_n8, assign20120_e24947_d_n10, assign20120_e24947_d_n11, assign20120_e24947_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20120_e24919: f64 = (locals.var_tb * locals.var_tb);
        let assign20120_e24921: f64 = (assign20120_e24919 * locals.var_tb);
        let assign20120_e24924: f64 = (27.0 * locals.var_ta);
        let assign20120_e24926: f64 = (assign20120_e24924 * locals.var_ta);
        let assign20120_e24928: f64 = (assign20120_e24926 * locals.var_ta);
        let assign20120_e24929: f64 = (assign20120_e24921 / assign20120_e24928);
        let assign20120_e24932: f64 = (locals.var_tb * locals.var_tc);
        let assign20120_e24935: f64 = (6.0 * locals.var_ta);
        let assign20120_e24937: f64 = (assign20120_e24935 * locals.var_ta);
        let assign20120_e24938: f64 = (assign20120_e24932 / assign20120_e24937);
        let assign20120_e24939: f64 = (assign20120_e24929 - assign20120_e24938);
        let assign20120_e24943: f64 = (2.0 * locals.var_ta);
        let assign20120_e24944: f64 = (locals.var_td / assign20120_e24943);
        let assign20120_e24945: f64 = (assign20120_e24939 + assign20120_e24944);
        (assign20120_e24945, ((-((locals.var_tb * locals.var_tc_dn0) / assign20120_e24937)) + (locals.var_td_dn0 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn2) / assign20120_e24937)) + (locals.var_td_dn2 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn4) / assign20120_e24937)) + (locals.var_td_dn4 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn5) / assign20120_e24937)) + (locals.var_td_dn5 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn6) / assign20120_e24937)) + (locals.var_td_dn6 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn8) / assign20120_e24937)) + (locals.var_td_dn8 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn10) / assign20120_e24937)) + (locals.var_td_dn10 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn11) / assign20120_e24937)) + (locals.var_td_dn11 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn12) / assign20120_e24937)) + (locals.var_td_dn12 / assign20120_e24943)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn8, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn12,)
    }
};
        locals.var_tq = assign20120_e24947;
        locals.var_tq_dn0 = assign20120_e24947_d_n0;
        locals.var_tq_dn2 = assign20120_e24947_d_n2;
        locals.var_tq_dn4 = assign20120_e24947_d_n4;
        locals.var_tq_dn5 = assign20120_e24947_d_n5;
        locals.var_tq_dn6 = assign20120_e24947_d_n6;
        locals.var_tq_dn8 = assign20120_e24947_d_n8;
        locals.var_tq_dn10 = assign20120_e24947_d_n10;
        locals.var_tq_dn11 = assign20120_e24947_d_n11;
        locals.var_tq_dn12 = assign20120_e24947_d_n12;
        locals.var_tq_rv = 0.0;

        let (assign20130_e24970, assign20130_e24970_d_n0, assign20130_e24970_d_n2, assign20130_e24970_d_n4, assign20130_e24970_d_n5, assign20130_e24970_d_n6, assign20130_e24970_d_n8, assign20130_e24970_d_n10, assign20130_e24970_d_n11, assign20130_e24970_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20130_e24956: f64 = (3.0 * locals.var_ta);
        let assign20130_e24958: f64 = (assign20130_e24956 * locals.var_tc);
        let assign20130_e24961: f64 = (locals.var_tb * locals.var_tb);
        let assign20130_e24962: f64 = (assign20130_e24958 - assign20130_e24961);
        let assign20130_e24965: f64 = (9.0 * locals.var_ta);
        let assign20130_e24967: f64 = (assign20130_e24965 * locals.var_ta);
        let assign20130_e24968: f64 = (assign20130_e24962 / assign20130_e24967);
        (assign20130_e24968, ((assign20130_e24956 * locals.var_tc_dn0) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn2) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn4) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn5) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn6) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn8) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn10) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn11) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn12) / assign20130_e24967),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn8, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn12,)
    }
};
        locals.var_tp = assign20130_e24970;
        locals.var_tp_dn0 = assign20130_e24970_d_n0;
        locals.var_tp_dn2 = assign20130_e24970_d_n2;
        locals.var_tp_dn4 = assign20130_e24970_d_n4;
        locals.var_tp_dn5 = assign20130_e24970_d_n5;
        locals.var_tp_dn6 = assign20130_e24970_d_n6;
        locals.var_tp_dn8 = assign20130_e24970_d_n8;
        locals.var_tp_dn10 = assign20130_e24970_d_n10;
        locals.var_tp_dn11 = assign20130_e24970_d_n11;
        locals.var_tp_dn12 = assign20130_e24970_d_n12;
        locals.var_tp_rv = 0.0;

        let (assign20140_e24988, assign20140_e24988_d_n0, assign20140_e24988_d_n2, assign20140_e24988_d_n4, assign20140_e24988_d_n5, assign20140_e24988_d_n6, assign20140_e24988_d_n8, assign20140_e24988_d_n10, assign20140_e24988_d_n11, assign20140_e24988_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20140_e24979: f64 = (locals.var_tq * locals.var_tq);
        let assign20140_e24982: f64 = (locals.var_tp * locals.var_tp);
        let assign20140_e24984: f64 = (assign20140_e24982 * locals.var_tp);
        let assign20140_e24985: f64 = (assign20140_e24979 + assign20140_e24984);
        let assign20140_e24986: f64 = (assign20140_e24985).sqrt();
        (assign20140_e24986, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn0))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn2))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn4))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn5))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn6))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn8))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn10))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn11))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn12 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn12)) + ((((locals.var_tp_dn12 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn12)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn12))) / (2.0 * assign20140_e24986)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign20140_e24988;
        locals.var_t5_dn0 = assign20140_e24988_d_n0;
        locals.var_t5_dn2 = assign20140_e24988_d_n2;
        locals.var_t5_dn4 = assign20140_e24988_d_n4;
        locals.var_t5_dn5 = assign20140_e24988_d_n5;
        locals.var_t5_dn6 = assign20140_e24988_d_n6;
        locals.var_t5_dn8 = assign20140_e24988_d_n8;
        locals.var_t5_dn10 = assign20140_e24988_d_n10;
        locals.var_t5_dn11 = assign20140_e24988_d_n11;
        locals.var_t5_dn12 = assign20140_e24988_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign20150_e25002, assign20150_e25002_d_n0, assign20150_e25002_d_n2, assign20150_e25002_d_n4, assign20150_e25002_d_n5, assign20150_e25002_d_n6, assign20150_e25002_d_n8, assign20150_e25002_d_n10, assign20150_e25002_d_n11, assign20150_e25002_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20150_e24996: f64 = (-locals.var_tq);
        let assign20150_e24998: f64 = (assign20150_e24996 + locals.var_t5);
        let assign20150_e25000: f64 = (assign20150_e24998).powf(0.3333333333333333);
        (assign20150_e25000, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn12) + locals.var_t5_dn12))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn12) + locals.var_t5_dn12) / assign20150_e24998))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn8, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn12,)
    }
};
        locals.var_tu = assign20150_e25002;
        locals.var_tu_dn0 = assign20150_e25002_d_n0;
        locals.var_tu_dn2 = assign20150_e25002_d_n2;
        locals.var_tu_dn4 = assign20150_e25002_d_n4;
        locals.var_tu_dn5 = assign20150_e25002_d_n5;
        locals.var_tu_dn6 = assign20150_e25002_d_n6;
        locals.var_tu_dn8 = assign20150_e25002_d_n8;
        locals.var_tu_dn10 = assign20150_e25002_d_n10;
        locals.var_tu_dn11 = assign20150_e25002_d_n11;
        locals.var_tu_dn12 = assign20150_e25002_d_n12;
        locals.var_tu_rv = 0.0;

        let (assign20160_e25016, assign20160_e25016_d_n0, assign20160_e25016_d_n2, assign20160_e25016_d_n4, assign20160_e25016_d_n5, assign20160_e25016_d_n6, assign20160_e25016_d_n8, assign20160_e25016_d_n10, assign20160_e25016_d_n11, assign20160_e25016_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20160_e25011: f64 = (locals.var_tq + locals.var_t5);
        let assign20160_e25013: f64 = (assign20160_e25011).powf(0.3333333333333333);
        let assign20160_e25014: f64 = (-assign20160_e25013);
        (assign20160_e25014, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn12 + locals.var_t5_dn12))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn12 + locals.var_t5_dn12) / assign20160_e25011))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn8, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn12,)
    }
};
        locals.var_tv = assign20160_e25016;
        locals.var_tv_dn0 = assign20160_e25016_d_n0;
        locals.var_tv_dn2 = assign20160_e25016_d_n2;
        locals.var_tv_dn4 = assign20160_e25016_d_n4;
        locals.var_tv_dn5 = assign20160_e25016_d_n5;
        locals.var_tv_dn6 = assign20160_e25016_d_n6;
        locals.var_tv_dn8 = assign20160_e25016_d_n8;
        locals.var_tv_dn10 = assign20160_e25016_d_n10;
        locals.var_tv_dn11 = assign20160_e25016_d_n11;
        locals.var_tv_dn12 = assign20160_e25016_d_n12;
        locals.var_tv_rv = 0.0;

        let (assign20170_e25033, assign20170_e25033_d_n0, assign20170_e25033_d_n2, assign20170_e25033_d_n4, assign20170_e25033_d_n5, assign20170_e25033_d_n6, assign20170_e25033_d_n8, assign20170_e25033_d_n10, assign20170_e25033_d_n11, assign20170_e25033_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20170_e25025: f64 = (locals.var_tu + locals.var_tv);
        let assign20170_e25029: f64 = (3.0 * locals.var_ta);
        let assign20170_e25030: f64 = (locals.var_tb / assign20170_e25029);
        let assign20170_e25031: f64 = (assign20170_e25025 - assign20170_e25030);
        (assign20170_e25031, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn12 + locals.var_tv_dn12),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign20170_e25033;
        locals.var_tx_dn0 = assign20170_e25033_d_n0;
        locals.var_tx_dn2 = assign20170_e25033_d_n2;
        locals.var_tx_dn4 = assign20170_e25033_d_n4;
        locals.var_tx_dn5 = assign20170_e25033_d_n5;
        locals.var_tx_dn6 = assign20170_e25033_d_n6;
        locals.var_tx_dn8 = assign20170_e25033_d_n8;
        locals.var_tx_dn10 = assign20170_e25033_d_n10;
        locals.var_tx_dn11 = assign20170_e25033_d_n11;
        locals.var_tx_dn12 = assign20170_e25033_d_n12;
        locals.var_tx_rv = 0.0;

        let (assign20180_e25046, assign20180_e25046_d_n0, assign20180_e25046_d_n2, assign20180_e25046_d_n4, assign20180_e25046_d_n5, assign20180_e25046_d_n6, assign20180_e25046_d_n8, assign20180_e25046_d_n10, assign20180_e25046_d_n11, assign20180_e25046_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20180_e25042: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign20180_e25044: f64 = (assign20180_e25042 - locals.var_vxbgmtcl);
        (assign20180_e25044, ((locals.var_tx_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), (((locals.var_tx_dn4 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), ((locals.var_tx_dn5 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn5), ((locals.var_tx_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_tx_dn8 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn8), ((locals.var_tx_dn10 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12,)
    }
};
        locals.var_ps0_inia = assign20180_e25046;
        locals.var_ps0_inia_dn0 = assign20180_e25046_d_n0;
        locals.var_ps0_inia_dn2 = assign20180_e25046_d_n2;
        locals.var_ps0_inia_dn4 = assign20180_e25046_d_n4;
        locals.var_ps0_inia_dn5 = assign20180_e25046_d_n5;
        locals.var_ps0_inia_dn6 = assign20180_e25046_d_n6;
        locals.var_ps0_inia_dn8 = assign20180_e25046_d_n8;
        locals.var_ps0_inia_dn10 = assign20180_e25046_d_n10;
        locals.var_ps0_inia_dn11 = assign20180_e25046_d_n11;
        locals.var_ps0_inia_dn12 = assign20180_e25046_d_n12;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20190_e25059, assign20190_e25059_d_n0, assign20190_e25059_d_n2, assign20190_e25059_d_n4, assign20190_e25059_d_n5, assign20190_e25059_d_n6, assign20190_e25059_d_n8, assign20190_e25059_d_n10, assign20190_e25059_d_n11, assign20190_e25059_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
        let assign20190_e25056: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign20190_e25057: f64 = (locals.var_beta * assign20190_e25056);
        (assign20190_e25057, (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign20190_e25056) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 + locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign20190_e25059;
        locals.var_chi_dn0 = assign20190_e25059_d_n0;
        locals.var_chi_dn2 = assign20190_e25059_d_n2;
        locals.var_chi_dn4 = assign20190_e25059_d_n4;
        locals.var_chi_dn5 = assign20190_e25059_d_n5;
        locals.var_chi_dn6 = assign20190_e25059_d_n6;
        locals.var_chi_dn8 = assign20190_e25059_d_n8;
        locals.var_chi_dn10 = assign20190_e25059_d_n10;
        locals.var_chi_dn11 = assign20190_e25059_d_n11;
        locals.var_chi_dn12 = assign20190_e25059_d_n12;
        locals.var_chi_rv = 0.0;

        let assign20200_e25062: f64 = if p.p30 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard351 = assign20200_e25062;
        locals.var_guard351_rv = 0.0;

        let (assign20220_e25086, assign20220_e25086_d_n0, assign20220_e25086_d_n2, assign20220_e25086_d_n4, assign20220_e25086_d_n5, assign20220_e25086_d_n6, assign20220_e25086_d_n8, assign20220_e25086_d_n10, assign20220_e25086_d_n11, assign20220_e25086_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20220_e25082: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign20220_e25084: f64 = (assign20220_e25082 + 0.1);
        (assign20220_e25084, (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5), locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn12,)
    }
};
        locals.var_vgpld_shift = assign20220_e25086;
        locals.var_vgpld_shift_dn0 = assign20220_e25086_d_n0;
        locals.var_vgpld_shift_dn2 = assign20220_e25086_d_n2;
        locals.var_vgpld_shift_dn4 = assign20220_e25086_d_n4;
        locals.var_vgpld_shift_dn5 = assign20220_e25086_d_n5;
        locals.var_vgpld_shift_dn6 = assign20220_e25086_d_n6;
        locals.var_vgpld_shift_dn8 = assign20220_e25086_d_n8;
        locals.var_vgpld_shift_dn10 = assign20220_e25086_d_n10;
        locals.var_vgpld_shift_dn11 = assign20220_e25086_d_n11;
        locals.var_vgpld_shift_dn12 = assign20220_e25086_d_n12;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign20230_e25101, assign20230_e25101_d_n0, assign20230_e25101_d_n2, assign20230_e25101_d_n4, assign20230_e25101_d_n5, assign20230_e25101_d_n6, assign20230_e25101_d_n8, assign20230_e25101_d_n10, assign20230_e25101_d_n11, assign20230_e25101_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20230_e25095: f64 = (-locals.var_vxbgmtcl);
        let assign20230_e25096: f64 = (locals.var_beta * assign20230_e25095);
        let assign20230_e25097: f64 = (assign20230_e25096).exp();
        let assign20230_e25099: f64 = (assign20230_e25097 + 1e-50);
        (assign20230_e25099, (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign20230_e25097 * ((locals.var_beta_dn4 * assign20230_e25095) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn12,)
    }
};
        locals.var_exp_bvbs = assign20230_e25101;
        locals.var_exp_bvbs_dn0 = assign20230_e25101_d_n0;
        locals.var_exp_bvbs_dn2 = assign20230_e25101_d_n2;
        locals.var_exp_bvbs_dn4 = assign20230_e25101_d_n4;
        locals.var_exp_bvbs_dn5 = assign20230_e25101_d_n5;
        locals.var_exp_bvbs_dn6 = assign20230_e25101_d_n6;
        locals.var_exp_bvbs_dn8 = assign20230_e25101_d_n8;
        locals.var_exp_bvbs_dn10 = assign20230_e25101_d_n10;
        locals.var_exp_bvbs_dn11 = assign20230_e25101_d_n11;
        locals.var_exp_bvbs_dn12 = assign20230_e25101_d_n12;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign20240_e25112, assign20240_e25112_d_n0, assign20240_e25112_d_n2, assign20240_e25112_d_n4, assign20240_e25112_d_n5, assign20240_e25112_d_n6, assign20240_e25112_d_n8, assign20240_e25112_d_n10, assign20240_e25112_d_n11, assign20240_e25112_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20240_e25110: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign20240_e25110, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn4 / locals.var_mks_nover), (locals.var_nin_dn5 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn8 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign20240_e25112;
        locals.var_t0_dn0 = assign20240_e25112_d_n0;
        locals.var_t0_dn2 = assign20240_e25112_d_n2;
        locals.var_t0_dn4 = assign20240_e25112_d_n4;
        locals.var_t0_dn5 = assign20240_e25112_d_n5;
        locals.var_t0_dn6 = assign20240_e25112_d_n6;
        locals.var_t0_dn8 = assign20240_e25112_d_n8;
        locals.var_t0_dn10 = assign20240_e25112_d_n10;
        locals.var_t0_dn11 = assign20240_e25112_d_n11;
        locals.var_t0_dn12 = assign20240_e25112_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign20250_e25123, assign20250_e25123_d_n0, assign20250_e25123_d_n2, assign20250_e25123_d_n4, assign20250_e25123_d_n5, assign20250_e25123_d_n6, assign20250_e25123_d_n8, assign20250_e25123_d_n10, assign20250_e25123_d_n11, assign20250_e25123_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20250_e25121: f64 = (locals.var_t0 * locals.var_t0);
        (assign20250_e25121, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn8, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12,)
    }
};
        locals.var_cnst1over = assign20250_e25123;
        locals.var_cnst1over_dn0 = assign20250_e25123_d_n0;
        locals.var_cnst1over_dn2 = assign20250_e25123_d_n2;
        locals.var_cnst1over_dn4 = assign20250_e25123_d_n4;
        locals.var_cnst1over_dn5 = assign20250_e25123_d_n5;
        locals.var_cnst1over_dn6 = assign20250_e25123_d_n6;
        locals.var_cnst1over_dn8 = assign20250_e25123_d_n8;
        locals.var_cnst1over_dn10 = assign20250_e25123_d_n10;
        locals.var_cnst1over_dn11 = assign20250_e25123_d_n11;
        locals.var_cnst1over_dn12 = assign20250_e25123_d_n12;
        locals.var_cnst1over_rv = 0.0;

        let (assign20260_e25134, assign20260_e25134_d_n0, assign20260_e25134_d_n2, assign20260_e25134_d_n4, assign20260_e25134_d_n5, assign20260_e25134_d_n6, assign20260_e25134_d_n8, assign20260_e25134_d_n10, assign20260_e25134_d_n11, assign20260_e25134_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20260_e25132: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign20260_e25132, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn12)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn8, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn12,)
    }
};
        locals.var_gammachi = assign20260_e25134;
        locals.var_gammachi_dn0 = assign20260_e25134_d_n0;
        locals.var_gammachi_dn2 = assign20260_e25134_d_n2;
        locals.var_gammachi_dn4 = assign20260_e25134_d_n4;
        locals.var_gammachi_dn5 = assign20260_e25134_d_n5;
        locals.var_gammachi_dn6 = assign20260_e25134_d_n6;
        locals.var_gammachi_dn8 = assign20260_e25134_d_n8;
        locals.var_gammachi_dn10 = assign20260_e25134_d_n10;
        locals.var_gammachi_dn11 = assign20260_e25134_d_n11;
        locals.var_gammachi_dn12 = assign20260_e25134_d_n12;
        locals.var_gammachi_rv = 0.0;

        let (assign20270_e25145, assign20270_e25145_d_n0, assign20270_e25145_d_n2, assign20270_e25145_d_n4, assign20270_e25145_d_n5, assign20270_e25145_d_n6, assign20270_e25145_d_n8, assign20270_e25145_d_n10, assign20270_e25145_d_n11, assign20270_e25145_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20270_e25143: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign20270_e25143, (locals.var_beta2 * locals.var_fac1p2_dn0), (locals.var_beta2 * locals.var_fac1p2_dn2), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), (locals.var_beta2 * locals.var_fac1p2_dn5), (locals.var_beta2 * locals.var_fac1p2_dn6), (locals.var_beta2 * locals.var_fac1p2_dn8), (locals.var_beta2 * locals.var_fac1p2_dn10), (locals.var_beta2 * locals.var_fac1p2_dn11), (locals.var_beta2 * locals.var_fac1p2_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign20270_e25145;
        locals.var_t0_dn0 = assign20270_e25145_d_n0;
        locals.var_t0_dn2 = assign20270_e25145_d_n2;
        locals.var_t0_dn4 = assign20270_e25145_d_n4;
        locals.var_t0_dn5 = assign20270_e25145_d_n5;
        locals.var_t0_dn6 = assign20270_e25145_d_n6;
        locals.var_t0_dn8 = assign20270_e25145_d_n8;
        locals.var_t0_dn10 = assign20270_e25145_d_n10;
        locals.var_t0_dn11 = assign20270_e25145_d_n11;
        locals.var_t0_dn12 = assign20270_e25145_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign20280_e25156, assign20280_e25156_d_n0, assign20280_e25156_d_n2, assign20280_e25156_d_n4, assign20280_e25156_d_n5, assign20280_e25156_d_n6, assign20280_e25156_d_n8, assign20280_e25156_d_n10, assign20280_e25156_d_n11, assign20280_e25156_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20280_e25154: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign20280_e25154, (locals.var_beta * locals.var_vgpld_shift_dn0), (locals.var_beta * locals.var_vgpld_shift_dn2), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), (locals.var_beta * locals.var_vgpld_shift_dn5), (locals.var_beta * locals.var_vgpld_shift_dn6), (locals.var_beta * locals.var_vgpld_shift_dn8), (locals.var_beta * locals.var_vgpld_shift_dn10), (locals.var_beta * locals.var_vgpld_shift_dn11), (locals.var_beta * locals.var_vgpld_shift_dn12),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12,)
    }
};
        locals.var_psi = assign20280_e25156;
        locals.var_psi_dn0 = assign20280_e25156_d_n0;
        locals.var_psi_dn2 = assign20280_e25156_d_n2;
        locals.var_psi_dn4 = assign20280_e25156_d_n4;
        locals.var_psi_dn5 = assign20280_e25156_d_n5;
        locals.var_psi_dn6 = assign20280_e25156_d_n6;
        locals.var_psi_dn8 = assign20280_e25156_d_n8;
        locals.var_psi_dn10 = assign20280_e25156_d_n10;
        locals.var_psi_dn11 = assign20280_e25156_d_n11;
        locals.var_psi_dn12 = assign20280_e25156_d_n12;
        locals.var_psi_rv = 0.0;

        let (assign20290_e25181, assign20290_e25181_d_n0, assign20290_e25181_d_n2, assign20290_e25181_d_n4, assign20290_e25181_d_n5, assign20290_e25181_d_n6, assign20290_e25181_d_n8, assign20290_e25181_d_n10, assign20290_e25181_d_n11, assign20290_e25181_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20290_e25165: f64 = (locals.var_gammachi * locals.var_t0);
        let assign20290_e25168: f64 = (locals.var_psi * locals.var_psi);
        let assign20290_e25169: f64 = (assign20290_e25165 + assign20290_e25168);
        let assign20290_e25170: f64 = (assign20290_e25169).ln();
        let assign20290_e25173: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign20290_e25174: f64 = (assign20290_e25173).ln();
        let assign20290_e25175: f64 = (assign20290_e25170 - assign20290_e25174);
        let assign20290_e25178: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign20290_e25179: f64 = (assign20290_e25175 + assign20290_e25178);
        (assign20290_e25179, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign20290_e25169) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign20290_e25169) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign20290_e25169) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign20290_e25173)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign20290_e25169) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign20290_e25169) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign20290_e25169) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn8)), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign20290_e25169) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign20290_e25169) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign20290_e25169) - (((locals.var_cnst1over_dn12 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn12)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn8, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12,)
    }
};
        locals.var_chi_1 = assign20290_e25181;
        locals.var_chi_1_dn0 = assign20290_e25181_d_n0;
        locals.var_chi_1_dn2 = assign20290_e25181_d_n2;
        locals.var_chi_1_dn4 = assign20290_e25181_d_n4;
        locals.var_chi_1_dn5 = assign20290_e25181_d_n5;
        locals.var_chi_1_dn6 = assign20290_e25181_d_n6;
        locals.var_chi_1_dn8 = assign20290_e25181_d_n8;
        locals.var_chi_1_dn10 = assign20290_e25181_d_n10;
        locals.var_chi_1_dn11 = assign20290_e25181_d_n11;
        locals.var_chi_1_dn12 = assign20290_e25181_d_n12;
        locals.var_chi_1_rv = 0.0;

        let (assign20300_e25194, assign20300_e25194_d_n0, assign20300_e25194_d_n2, assign20300_e25194_d_n4, assign20300_e25194_d_n5, assign20300_e25194_d_n6, assign20300_e25194_d_n8, assign20300_e25194_d_n10, assign20300_e25194_d_n11, assign20300_e25194_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20300_e25190: f64 = (locals.var_psi - locals.var_chi_1);
        let assign20300_e25192: f64 = (assign20300_e25190 - 1.0);
        (assign20300_e25192, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign20300_e25194;
        locals.var_tmf1_dn0 = assign20300_e25194_d_n0;
        locals.var_tmf1_dn2 = assign20300_e25194_d_n2;
        locals.var_tmf1_dn4 = assign20300_e25194_d_n4;
        locals.var_tmf1_dn5 = assign20300_e25194_d_n5;
        locals.var_tmf1_dn6 = assign20300_e25194_d_n6;
        locals.var_tmf1_dn8 = assign20300_e25194_d_n8;
        locals.var_tmf1_dn10 = assign20300_e25194_d_n10;
        locals.var_tmf1_dn11 = assign20300_e25194_d_n11;
        locals.var_tmf1_dn12 = assign20300_e25194_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign20310_e25207, assign20310_e25207_d_n0, assign20310_e25207_d_n2, assign20310_e25207_d_n4, assign20310_e25207_d_n5, assign20310_e25207_d_n6, assign20310_e25207_d_n8, assign20310_e25207_d_n10, assign20310_e25207_d_n11, assign20310_e25207_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20310_e25203: f64 = (4.0 * locals.var_psi);
        let assign20310_e25205: f64 = assign20310_e25203;
        (assign20310_e25205, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn12),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign20310_e25207;
        locals.var_tmf2_dn0 = assign20310_e25207_d_n0;
        locals.var_tmf2_dn2 = assign20310_e25207_d_n2;
        locals.var_tmf2_dn4 = assign20310_e25207_d_n4;
        locals.var_tmf2_dn5 = assign20310_e25207_d_n5;
        locals.var_tmf2_dn6 = assign20310_e25207_d_n6;
        locals.var_tmf2_dn8 = assign20310_e25207_d_n8;
        locals.var_tmf2_dn10 = assign20310_e25207_d_n10;
        locals.var_tmf2_dn11 = assign20310_e25207_d_n11;
        locals.var_tmf2_dn12 = assign20310_e25207_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign20320_e25222, assign20320_e25222_d_n0, assign20320_e25222_d_n2, assign20320_e25222_d_n4, assign20320_e25222_d_n5, assign20320_e25222_d_n6, assign20320_e25222_d_n8, assign20320_e25222_d_n10, assign20320_e25222_d_n11, assign20320_e25222_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let (assign20320_e25220, assign20320_e25220_d_n0, assign20320_e25220_d_n2, assign20320_e25220_d_n4, assign20320_e25220_d_n5, assign20320_e25220_d_n6, assign20320_e25220_d_n8, assign20320_e25220_d_n10, assign20320_e25220_d_n11, assign20320_e25220_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign20320_e25219: f64 = (-locals.var_tmf2);
                (assign20320_e25219, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign20320_e25220, assign20320_e25220_d_n0, assign20320_e25220_d_n2, assign20320_e25220_d_n4, assign20320_e25220_d_n5, assign20320_e25220_d_n6, assign20320_e25220_d_n8, assign20320_e25220_d_n10, assign20320_e25220_d_n11, assign20320_e25220_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign20320_e25222;
        locals.var_tmf2_dn0 = assign20320_e25222_d_n0;
        locals.var_tmf2_dn2 = assign20320_e25222_d_n2;
        locals.var_tmf2_dn4 = assign20320_e25222_d_n4;
        locals.var_tmf2_dn5 = assign20320_e25222_d_n5;
        locals.var_tmf2_dn6 = assign20320_e25222_d_n6;
        locals.var_tmf2_dn8 = assign20320_e25222_d_n8;
        locals.var_tmf2_dn10 = assign20320_e25222_d_n10;
        locals.var_tmf2_dn11 = assign20320_e25222_d_n11;
        locals.var_tmf2_dn12 = assign20320_e25222_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign20330_e25236, assign20330_e25236_d_n0, assign20330_e25236_d_n2, assign20330_e25236_d_n4, assign20330_e25236_d_n5, assign20330_e25236_d_n6, assign20330_e25236_d_n8, assign20330_e25236_d_n10, assign20330_e25236_d_n11, assign20330_e25236_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20330_e25231: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20330_e25233: f64 = (assign20330_e25231 + locals.var_tmf2);
        let assign20330_e25234: f64 = (assign20330_e25233).sqrt();
        (assign20330_e25234, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign20330_e25234)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign20330_e25236;
        locals.var_tmf2_dn0 = assign20330_e25236_d_n0;
        locals.var_tmf2_dn2 = assign20330_e25236_d_n2;
        locals.var_tmf2_dn4 = assign20330_e25236_d_n4;
        locals.var_tmf2_dn5 = assign20330_e25236_d_n5;
        locals.var_tmf2_dn6 = assign20330_e25236_d_n6;
        locals.var_tmf2_dn8 = assign20330_e25236_d_n8;
        locals.var_tmf2_dn10 = assign20330_e25236_d_n10;
        locals.var_tmf2_dn11 = assign20330_e25236_d_n11;
        locals.var_tmf2_dn12 = assign20330_e25236_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign20340_e25251, assign20340_e25251_d_n0, assign20340_e25251_d_n2, assign20340_e25251_d_n4, assign20340_e25251_d_n5, assign20340_e25251_d_n6, assign20340_e25251_d_n8, assign20340_e25251_d_n10, assign20340_e25251_d_n11, assign20340_e25251_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20340_e25247: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20340_e25248: f64 = (1.0 + assign20340_e25247);
        let assign20340_e25249: f64 = (0.5 * assign20340_e25248);
        (assign20340_e25249, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign20340_e25251;
        locals.var_t1_dn0 = assign20340_e25251_d_n0;
        locals.var_t1_dn2 = assign20340_e25251_d_n2;
        locals.var_t1_dn4 = assign20340_e25251_d_n4;
        locals.var_t1_dn5 = assign20340_e25251_d_n5;
        locals.var_t1_dn6 = assign20340_e25251_d_n6;
        locals.var_t1_dn8 = assign20340_e25251_d_n8;
        locals.var_t1_dn10 = assign20340_e25251_d_n10;
        locals.var_t1_dn11 = assign20340_e25251_d_n11;
        locals.var_t1_dn12 = assign20340_e25251_d_n12;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_82(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20350_e25270, assign20350_e25270_d_n0, assign20350_e25270_d_n2, assign20350_e25270_d_n4, assign20350_e25270_d_n5, assign20350_e25270_d_n6, assign20350_e25270_d_n8, assign20350_e25270_d_n10, assign20350_e25270_d_n11, assign20350_e25270_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20350_e25263: f64 = 2.0;
        let assign20350_e25264: f64 = (locals.var_tmf1 + assign20350_e25263);
        let assign20350_e25266: f64 = (assign20350_e25264 / locals.var_tmf2);
        let assign20350_e25267: f64 = (1.0 - assign20350_e25266);
        let assign20350_e25268: f64 = (0.5 * assign20350_e25267);
        (assign20350_e25268, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign20350_e25270;
        locals.var_t2_dn0 = assign20350_e25270_d_n0;
        locals.var_t2_dn2 = assign20350_e25270_d_n2;
        locals.var_t2_dn4 = assign20350_e25270_d_n4;
        locals.var_t2_dn5 = assign20350_e25270_d_n5;
        locals.var_t2_dn6 = assign20350_e25270_d_n6;
        locals.var_t2_dn8 = assign20350_e25270_d_n8;
        locals.var_t2_dn10 = assign20350_e25270_d_n10;
        locals.var_t2_dn11 = assign20350_e25270_d_n11;
        locals.var_t2_dn12 = assign20350_e25270_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign20360_e25285, assign20360_e25285_d_n0, assign20360_e25285_d_n2, assign20360_e25285_d_n4, assign20360_e25285_d_n5, assign20360_e25285_d_n6, assign20360_e25285_d_n8, assign20360_e25285_d_n10, assign20360_e25285_d_n11, assign20360_e25285_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20360_e25281: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20360_e25282: f64 = (0.5 * assign20360_e25281);
        let assign20360_e25283: f64 = (locals.var_psi - assign20360_e25282);
        (assign20360_e25283, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn8, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12,)
    }
};
        locals.var_chi_1 = assign20360_e25285;
        locals.var_chi_1_dn0 = assign20360_e25285_d_n0;
        locals.var_chi_1_dn2 = assign20360_e25285_d_n2;
        locals.var_chi_1_dn4 = assign20360_e25285_d_n4;
        locals.var_chi_1_dn5 = assign20360_e25285_d_n5;
        locals.var_chi_1_dn6 = assign20360_e25285_d_n6;
        locals.var_chi_1_dn8 = assign20360_e25285_d_n8;
        locals.var_chi_1_dn10 = assign20360_e25285_d_n10;
        locals.var_chi_1_dn11 = assign20360_e25285_d_n11;
        locals.var_chi_1_dn12 = assign20360_e25285_d_n12;
        locals.var_chi_1_rv = 0.0;

        let (assign20370_e25296, assign20370_e25296_d_n0, assign20370_e25296_d_n2, assign20370_e25296_d_n4, assign20370_e25296_d_n5, assign20370_e25296_d_n6, assign20370_e25296_d_n8, assign20370_e25296_d_n10, assign20370_e25296_d_n11, assign20370_e25296_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20370_e25294: f64 = (locals.var_psi - locals.var_chi_1);
        (assign20370_e25294, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12,)
    }
};
        locals.var_psi = assign20370_e25296;
        locals.var_psi_dn0 = assign20370_e25296_d_n0;
        locals.var_psi_dn2 = assign20370_e25296_d_n2;
        locals.var_psi_dn4 = assign20370_e25296_d_n4;
        locals.var_psi_dn5 = assign20370_e25296_d_n5;
        locals.var_psi_dn6 = assign20370_e25296_d_n6;
        locals.var_psi_dn8 = assign20370_e25296_d_n8;
        locals.var_psi_dn10 = assign20370_e25296_d_n10;
        locals.var_psi_dn11 = assign20370_e25296_d_n11;
        locals.var_psi_dn12 = assign20370_e25296_d_n12;
        locals.var_psi_rv = 0.0;

        let (assign20380_e25309, assign20380_e25309_d_n0, assign20380_e25309_d_n2, assign20380_e25309_d_n4, assign20380_e25309_d_n5, assign20380_e25309_d_n6, assign20380_e25309_d_n8, assign20380_e25309_d_n10, assign20380_e25309_d_n11, assign20380_e25309_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20380_e25306: f64 = (locals.var_beta * 0.1);
        let assign20380_e25307: f64 = (locals.var_psi + assign20380_e25306);
        (assign20380_e25307, locals.var_psi_dn0, locals.var_psi_dn2, (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12,)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12,)
    }
};
        locals.var_psi = assign20380_e25309;
        locals.var_psi_dn0 = assign20380_e25309_d_n0;
        locals.var_psi_dn2 = assign20380_e25309_d_n2;
        locals.var_psi_dn4 = assign20380_e25309_d_n4;
        locals.var_psi_dn5 = assign20380_e25309_d_n5;
        locals.var_psi_dn6 = assign20380_e25309_d_n6;
        locals.var_psi_dn8 = assign20380_e25309_d_n8;
        locals.var_psi_dn10 = assign20380_e25309_d_n10;
        locals.var_psi_dn11 = assign20380_e25309_d_n11;
        locals.var_psi_dn12 = assign20380_e25309_d_n12;
        locals.var_psi_rv = 0.0;

        let (assign20390_e25334, assign20390_e25334_d_n0, assign20390_e25334_d_n2, assign20390_e25334_d_n4, assign20390_e25334_d_n5, assign20390_e25334_d_n6, assign20390_e25334_d_n8, assign20390_e25334_d_n10, assign20390_e25334_d_n11, assign20390_e25334_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20390_e25318: f64 = (locals.var_gammachi * locals.var_t0);
        let assign20390_e25321: f64 = (locals.var_psi * locals.var_psi);
        let assign20390_e25322: f64 = (assign20390_e25318 + assign20390_e25321);
        let assign20390_e25323: f64 = (assign20390_e25322).ln();
        let assign20390_e25326: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign20390_e25327: f64 = (assign20390_e25326).ln();
        let assign20390_e25328: f64 = (assign20390_e25323 - assign20390_e25327);
        let assign20390_e25331: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign20390_e25332: f64 = (assign20390_e25328 + assign20390_e25331);
        (assign20390_e25332, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign20390_e25322) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign20390_e25322) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign20390_e25322) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign20390_e25326)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign20390_e25322) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign20390_e25322) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign20390_e25322) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn8)), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign20390_e25322) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign20390_e25322) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign20390_e25322) - (((locals.var_cnst1over_dn12 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn12)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn8, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn12,)
    }
};
        locals.var_chi_b = assign20390_e25334;
        locals.var_chi_b_dn0 = assign20390_e25334_d_n0;
        locals.var_chi_b_dn2 = assign20390_e25334_d_n2;
        locals.var_chi_b_dn4 = assign20390_e25334_d_n4;
        locals.var_chi_b_dn5 = assign20390_e25334_d_n5;
        locals.var_chi_b_dn6 = assign20390_e25334_d_n6;
        locals.var_chi_b_dn8 = assign20390_e25334_d_n8;
        locals.var_chi_b_dn10 = assign20390_e25334_d_n10;
        locals.var_chi_b_dn11 = assign20390_e25334_d_n11;
        locals.var_chi_b_dn12 = assign20390_e25334_d_n12;
        locals.var_chi_b_rv = 0.0;

        let (assign20400_e25347, assign20400_e25347_d_n0, assign20400_e25347_d_n2, assign20400_e25347_d_n4, assign20400_e25347_d_n5, assign20400_e25347_d_n6, assign20400_e25347_d_n8, assign20400_e25347_d_n10, assign20400_e25347_d_n11, assign20400_e25347_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20400_e25343: f64 = (locals.var_chi_b / locals.var_beta);
        let assign20400_e25345: f64 = (assign20400_e25343 - locals.var_vxbgmtcl);
        (assign20400_e25345, ((locals.var_chi_b_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_b_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((((locals.var_chi_b_dn4 * locals.var_beta) - (locals.var_chi_b * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_b_dn5 / locals.var_beta) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_b_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_b_dn8 / locals.var_beta) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_b_dn10 / locals.var_beta) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_b_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_b_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn4, locals.var_ps0_inib_dn5, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn8, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12,)
    }
};
        locals.var_ps0_inib = assign20400_e25347;
        locals.var_ps0_inib_dn0 = assign20400_e25347_d_n0;
        locals.var_ps0_inib_dn2 = assign20400_e25347_d_n2;
        locals.var_ps0_inib_dn4 = assign20400_e25347_d_n4;
        locals.var_ps0_inib_dn5 = assign20400_e25347_d_n5;
        locals.var_ps0_inib_dn6 = assign20400_e25347_d_n6;
        locals.var_ps0_inib_dn8 = assign20400_e25347_d_n8;
        locals.var_ps0_inib_dn10 = assign20400_e25347_d_n10;
        locals.var_ps0_inib_dn11 = assign20400_e25347_d_n11;
        locals.var_ps0_inib_dn12 = assign20400_e25347_d_n12;
        locals.var_ps0_inib_rv = 0.0;

        let (assign20410_e25356, assign20410_e25356_d_n0, assign20410_e25356_d_n2, assign20410_e25356_d_n4, assign20410_e25356_d_n5, assign20410_e25356_d_n6, assign20410_e25356_d_n8, assign20410_e25356_d_n10, assign20410_e25356_d_n11, assign20410_e25356_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn8, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn12,)
    }
};
        locals.var_chi_a = assign20410_e25356;
        locals.var_chi_a_dn0 = assign20410_e25356_d_n0;
        locals.var_chi_a_dn2 = assign20410_e25356_d_n2;
        locals.var_chi_a_dn4 = assign20410_e25356_d_n4;
        locals.var_chi_a_dn5 = assign20410_e25356_d_n5;
        locals.var_chi_a_dn6 = assign20410_e25356_d_n6;
        locals.var_chi_a_dn8 = assign20410_e25356_d_n8;
        locals.var_chi_a_dn10 = assign20410_e25356_d_n10;
        locals.var_chi_a_dn11 = assign20410_e25356_d_n11;
        locals.var_chi_a_dn12 = assign20410_e25356_d_n12;
        locals.var_chi_a_rv = 0.0;

        let (assign20420_e25371, assign20420_e25371_d_n0, assign20420_e25371_d_n2, assign20420_e25371_d_n4, assign20420_e25371_d_n5, assign20420_e25371_d_n6, assign20420_e25371_d_n8, assign20420_e25371_d_n10, assign20420_e25371_d_n11, assign20420_e25371_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20420_e25365: f64 = (locals.var_chi_b - locals.var_chi_a);
        let assign20420_e25368: f64 = (0.0008 * 75.0);
        let assign20420_e25369: f64 = (assign20420_e25365 - assign20420_e25368);
        (assign20420_e25369, (locals.var_chi_b_dn0 - locals.var_chi_a_dn0), (locals.var_chi_b_dn2 - locals.var_chi_a_dn2), (locals.var_chi_b_dn4 - locals.var_chi_a_dn4), (locals.var_chi_b_dn5 - locals.var_chi_a_dn5), (locals.var_chi_b_dn6 - locals.var_chi_a_dn6), (locals.var_chi_b_dn8 - locals.var_chi_a_dn8), (locals.var_chi_b_dn10 - locals.var_chi_a_dn10), (locals.var_chi_b_dn11 - locals.var_chi_a_dn11), (locals.var_chi_b_dn12 - locals.var_chi_a_dn12),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign20420_e25371;
        locals.var_tmf1_dn0 = assign20420_e25371_d_n0;
        locals.var_tmf1_dn2 = assign20420_e25371_d_n2;
        locals.var_tmf1_dn4 = assign20420_e25371_d_n4;
        locals.var_tmf1_dn5 = assign20420_e25371_d_n5;
        locals.var_tmf1_dn6 = assign20420_e25371_d_n6;
        locals.var_tmf1_dn8 = assign20420_e25371_d_n8;
        locals.var_tmf1_dn10 = assign20420_e25371_d_n10;
        locals.var_tmf1_dn11 = assign20420_e25371_d_n11;
        locals.var_tmf1_dn12 = assign20420_e25371_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign20430_e25386, assign20430_e25386_d_n0, assign20430_e25386_d_n2, assign20430_e25386_d_n4, assign20430_e25386_d_n5, assign20430_e25386_d_n6, assign20430_e25386_d_n8, assign20430_e25386_d_n10, assign20430_e25386_d_n11, assign20430_e25386_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20430_e25380: f64 = (4.0 * locals.var_chi_b);
        let assign20430_e25383: f64 = (0.0008 * 75.0);
        let assign20430_e25384: f64 = (assign20430_e25380 * assign20430_e25383);
        (assign20430_e25384, ((4.0 * locals.var_chi_b_dn0) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn2) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn4) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn5) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn6) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn8) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn10) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn11) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn12) * assign20430_e25383),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign20430_e25386;
        locals.var_tmf2_dn0 = assign20430_e25386_d_n0;
        locals.var_tmf2_dn2 = assign20430_e25386_d_n2;
        locals.var_tmf2_dn4 = assign20430_e25386_d_n4;
        locals.var_tmf2_dn5 = assign20430_e25386_d_n5;
        locals.var_tmf2_dn6 = assign20430_e25386_d_n6;
        locals.var_tmf2_dn8 = assign20430_e25386_d_n8;
        locals.var_tmf2_dn10 = assign20430_e25386_d_n10;
        locals.var_tmf2_dn11 = assign20430_e25386_d_n11;
        locals.var_tmf2_dn12 = assign20430_e25386_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign20440_e25401, assign20440_e25401_d_n0, assign20440_e25401_d_n2, assign20440_e25401_d_n4, assign20440_e25401_d_n5, assign20440_e25401_d_n6, assign20440_e25401_d_n8, assign20440_e25401_d_n10, assign20440_e25401_d_n11, assign20440_e25401_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let (assign20440_e25399, assign20440_e25399_d_n0, assign20440_e25399_d_n2, assign20440_e25399_d_n4, assign20440_e25399_d_n5, assign20440_e25399_d_n6, assign20440_e25399_d_n8, assign20440_e25399_d_n10, assign20440_e25399_d_n11, assign20440_e25399_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign20440_e25398: f64 = (-locals.var_tmf2);
                (assign20440_e25398, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign20440_e25399, assign20440_e25399_d_n0, assign20440_e25399_d_n2, assign20440_e25399_d_n4, assign20440_e25399_d_n5, assign20440_e25399_d_n6, assign20440_e25399_d_n8, assign20440_e25399_d_n10, assign20440_e25399_d_n11, assign20440_e25399_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign20440_e25401;
        locals.var_tmf2_dn0 = assign20440_e25401_d_n0;
        locals.var_tmf2_dn2 = assign20440_e25401_d_n2;
        locals.var_tmf2_dn4 = assign20440_e25401_d_n4;
        locals.var_tmf2_dn5 = assign20440_e25401_d_n5;
        locals.var_tmf2_dn6 = assign20440_e25401_d_n6;
        locals.var_tmf2_dn8 = assign20440_e25401_d_n8;
        locals.var_tmf2_dn10 = assign20440_e25401_d_n10;
        locals.var_tmf2_dn11 = assign20440_e25401_d_n11;
        locals.var_tmf2_dn12 = assign20440_e25401_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign20450_e25415, assign20450_e25415_d_n0, assign20450_e25415_d_n2, assign20450_e25415_d_n4, assign20450_e25415_d_n5, assign20450_e25415_d_n6, assign20450_e25415_d_n8, assign20450_e25415_d_n10, assign20450_e25415_d_n11, assign20450_e25415_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20450_e25410: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20450_e25412: f64 = (assign20450_e25410 + locals.var_tmf2);
        let assign20450_e25413: f64 = (assign20450_e25412).sqrt();
        (assign20450_e25413, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign20450_e25413)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign20450_e25415;
        locals.var_tmf2_dn0 = assign20450_e25415_d_n0;
        locals.var_tmf2_dn2 = assign20450_e25415_d_n2;
        locals.var_tmf2_dn4 = assign20450_e25415_d_n4;
        locals.var_tmf2_dn5 = assign20450_e25415_d_n5;
        locals.var_tmf2_dn6 = assign20450_e25415_d_n6;
        locals.var_tmf2_dn8 = assign20450_e25415_d_n8;
        locals.var_tmf2_dn10 = assign20450_e25415_d_n10;
        locals.var_tmf2_dn11 = assign20450_e25415_d_n11;
        locals.var_tmf2_dn12 = assign20450_e25415_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign20460_e25430, assign20460_e25430_d_n0, assign20460_e25430_d_n2, assign20460_e25430_d_n4, assign20460_e25430_d_n5, assign20460_e25430_d_n6, assign20460_e25430_d_n8, assign20460_e25430_d_n10, assign20460_e25430_d_n11, assign20460_e25430_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20460_e25426: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20460_e25427: f64 = (1.0 + assign20460_e25426);
        let assign20460_e25428: f64 = (0.5 * assign20460_e25427);
        (assign20460_e25428, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign20460_e25430;
        locals.var_t1_dn0 = assign20460_e25430_d_n0;
        locals.var_t1_dn2 = assign20460_e25430_d_n2;
        locals.var_t1_dn4 = assign20460_e25430_d_n4;
        locals.var_t1_dn5 = assign20460_e25430_d_n5;
        locals.var_t1_dn6 = assign20460_e25430_d_n6;
        locals.var_t1_dn8 = assign20460_e25430_d_n8;
        locals.var_t1_dn10 = assign20460_e25430_d_n10;
        locals.var_t1_dn11 = assign20460_e25430_d_n11;
        locals.var_t1_dn12 = assign20460_e25430_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign20470_e25451, assign20470_e25451_d_n0, assign20470_e25451_d_n2, assign20470_e25451_d_n4, assign20470_e25451_d_n5, assign20470_e25451_d_n6, assign20470_e25451_d_n8, assign20470_e25451_d_n10, assign20470_e25451_d_n11, assign20470_e25451_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20470_e25442: f64 = (2.0 * 0.0008);
        let assign20470_e25444: f64 = (assign20470_e25442 * 75.0);
        let assign20470_e25445: f64 = (locals.var_tmf1 + assign20470_e25444);
        let assign20470_e25447: f64 = (assign20470_e25445 / locals.var_tmf2);
        let assign20470_e25448: f64 = (1.0 - assign20470_e25447);
        let assign20470_e25449: f64 = (0.5 * assign20470_e25448);
        (assign20470_e25449, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign20470_e25451;
        locals.var_t2_dn0 = assign20470_e25451_d_n0;
        locals.var_t2_dn2 = assign20470_e25451_d_n2;
        locals.var_t2_dn4 = assign20470_e25451_d_n4;
        locals.var_t2_dn5 = assign20470_e25451_d_n5;
        locals.var_t2_dn6 = assign20470_e25451_d_n6;
        locals.var_t2_dn8 = assign20470_e25451_d_n8;
        locals.var_t2_dn10 = assign20470_e25451_d_n10;
        locals.var_t2_dn11 = assign20470_e25451_d_n11;
        locals.var_t2_dn12 = assign20470_e25451_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign20480_e25466, assign20480_e25466_d_n0, assign20480_e25466_d_n2, assign20480_e25466_d_n4, assign20480_e25466_d_n5, assign20480_e25466_d_n6, assign20480_e25466_d_n8, assign20480_e25466_d_n10, assign20480_e25466_d_n11, assign20480_e25466_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
        let assign20480_e25462: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20480_e25463: f64 = (0.5 * assign20480_e25462);
        let assign20480_e25464: f64 = (locals.var_chi_b - assign20480_e25463);
        (assign20480_e25464, (locals.var_chi_b_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_chi_b_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_chi_b_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_chi_b_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
        locals.var_chi = assign20480_e25466;
        locals.var_chi_dn0 = assign20480_e25466_d_n0;
        locals.var_chi_dn2 = assign20480_e25466_d_n2;
        locals.var_chi_dn4 = assign20480_e25466_d_n4;
        locals.var_chi_dn5 = assign20480_e25466_d_n5;
        locals.var_chi_dn6 = assign20480_e25466_d_n6;
        locals.var_chi_dn8 = assign20480_e25466_d_n8;
        locals.var_chi_dn10 = assign20480_e25466_d_n10;
        locals.var_chi_dn11 = assign20480_e25466_d_n11;
        locals.var_chi_dn12 = assign20480_e25466_d_n12;
        locals.var_chi_rv = 0.0;

        let (assign20490_e25477, assign20490_e25477_d_n0, assign20490_e25477_d_n2, assign20490_e25477_d_n4, assign20490_e25477_d_n5, assign20490_e25477_d_n6, assign20490_e25477_d_n8, assign20490_e25477_d_n10, assign20490_e25477_d_n11, assign20490_e25477_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign20490_e25473: f64 = (locals.var_chi / locals.var_beta);
        let assign20490_e25475: f64 = (assign20490_e25473 - locals.var_vxbgmtcl);
        (assign20490_e25475, ((locals.var_chi_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((((locals.var_chi_dn4 * locals.var_beta) - (locals.var_chi * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_dn5 / locals.var_beta) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_dn8 / locals.var_beta) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_dn10 / locals.var_beta) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn8, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12,)
    }
};
        locals.var_ps0ld = assign20490_e25477;
        locals.var_ps0ld_dn0 = assign20490_e25477_d_n0;
        locals.var_ps0ld_dn2 = assign20490_e25477_d_n2;
        locals.var_ps0ld_dn4 = assign20490_e25477_d_n4;
        locals.var_ps0ld_dn5 = assign20490_e25477_d_n5;
        locals.var_ps0ld_dn6 = assign20490_e25477_d_n6;
        locals.var_ps0ld_dn8 = assign20490_e25477_d_n8;
        locals.var_ps0ld_dn10 = assign20490_e25477_d_n10;
        locals.var_ps0ld_dn11 = assign20490_e25477_d_n11;
        locals.var_ps0ld_dn12 = assign20490_e25477_d_n12;
        locals.var_ps0ld_rv = 0.0;

        let (assign20500_e25490, assign20500_e25490_d_n0, assign20500_e25490_d_n2, assign20500_e25490_d_n4, assign20500_e25490_d_n5, assign20500_e25490_d_n6, assign20500_e25490_d_n8, assign20500_e25490_d_n10, assign20500_e25490_d_n11, assign20500_e25490_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign20500_e25484: f64 = (locals.var_chi - 1.0);
        let assign20500_e25486: f64 = (-locals.var_chi);
        let assign20500_e25487: f64 = (assign20500_e25486).exp();
        let assign20500_e25488: f64 = (assign20500_e25484 + assign20500_e25487);
        (assign20500_e25488, (locals.var_chi_dn0 + (assign20500_e25487 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign20500_e25487 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign20500_e25487 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign20500_e25487 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign20500_e25487 * (-locals.var_chi_dn6))), (locals.var_chi_dn8 + (assign20500_e25487 * (-locals.var_chi_dn8))), (locals.var_chi_dn10 + (assign20500_e25487 * (-locals.var_chi_dn10))), (locals.var_chi_dn11 + (assign20500_e25487 * (-locals.var_chi_dn11))), (locals.var_chi_dn12 + (assign20500_e25487 * (-locals.var_chi_dn12))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign20500_e25490;
        locals.var_t1_dn0 = assign20500_e25490_d_n0;
        locals.var_t1_dn2 = assign20500_e25490_d_n2;
        locals.var_t1_dn4 = assign20500_e25490_d_n4;
        locals.var_t1_dn5 = assign20500_e25490_d_n5;
        locals.var_t1_dn6 = assign20500_e25490_d_n6;
        locals.var_t1_dn8 = assign20500_e25490_d_n8;
        locals.var_t1_dn10 = assign20500_e25490_d_n10;
        locals.var_t1_dn11 = assign20500_e25490_d_n11;
        locals.var_t1_dn12 = assign20500_e25490_d_n12;
        locals.var_t1_rv = 0.0;

        let assign20510_e25494: f64 = (10.0 * 2.220446049250313e-16);
        let assign20510_e25495: f64 = if locals.var_t1 < assign20510_e25494 { 1.0 } else { 0.0 };
        locals.var_guard352 = assign20510_e25495;
        locals.var_guard352_rv = 0.0;

        let (assign20520_e25506, assign20520_e25506_d_n0, assign20520_e25506_d_n2, assign20520_e25506_d_n4, assign20520_e25506_d_n5, assign20520_e25506_d_n6, assign20520_e25506_d_n8, assign20520_e25506_d_n10, assign20520_e25506_d_n11, assign20520_e25506_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard352 != 0.0)) {
        let assign20520_e25504: f64 = (10.0 * 2.220446049250313e-16);
        (assign20520_e25504, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign20520_e25506;
        locals.var_t1_dn0 = assign20520_e25506_d_n0;
        locals.var_t1_dn2 = assign20520_e25506_d_n2;
        locals.var_t1_dn4 = assign20520_e25506_d_n4;
        locals.var_t1_dn5 = assign20520_e25506_d_n5;
        locals.var_t1_dn6 = assign20520_e25506_d_n6;
        locals.var_t1_dn8 = assign20520_e25506_d_n8;
        locals.var_t1_dn10 = assign20520_e25506_d_n10;
        locals.var_t1_dn11 = assign20520_e25506_d_n11;
        locals.var_t1_dn12 = assign20520_e25506_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign20530_e25516, assign20530_e25516_d_n0, assign20530_e25516_d_n2, assign20530_e25516_d_n4, assign20530_e25516_d_n5, assign20530_e25516_d_n6, assign20530_e25516_d_n8, assign20530_e25516_d_n10, assign20530_e25516_d_n11, assign20530_e25516_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign20530_e25513: f64 = (locals.var_t1).sqrt();
        let assign20530_e25514: f64 = (locals.var_cnst0over * assign20530_e25513);
        (assign20530_e25514, ((locals.var_cnst0over_dn0 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn0 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn2 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn2 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn4 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn4 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn5 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn5 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn6 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn6 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn8 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn8 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn10 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn10 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn11 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn11 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn12 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn12 / (2.0 * assign20530_e25513)))),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn8, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12,)
    }
};
        locals.var_qbuld = assign20530_e25516;
        locals.var_qbuld_dn0 = assign20530_e25516_d_n0;
        locals.var_qbuld_dn2 = assign20530_e25516_d_n2;
        locals.var_qbuld_dn4 = assign20530_e25516_d_n4;
        locals.var_qbuld_dn5 = assign20530_e25516_d_n5;
        locals.var_qbuld_dn6 = assign20530_e25516_d_n6;
        locals.var_qbuld_dn8 = assign20530_e25516_d_n8;
        locals.var_qbuld_dn10 = assign20530_e25516_d_n10;
        locals.var_qbuld_dn11 = assign20530_e25516_d_n11;
        locals.var_qbuld_dn12 = assign20530_e25516_d_n12;
        locals.var_qbuld_rv = 0.0;

        let (assign20540_e25527, assign20540_e25527_d_n0, assign20540_e25527_d_n2, assign20540_e25527_d_n4, assign20540_e25527_d_n5, assign20540_e25527_d_n6, assign20540_e25527_d_n8, assign20540_e25527_d_n10, assign20540_e25527_d_n11, assign20540_e25527_d_n12,) = {
    if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign20540_e25524: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign20540_e25525: f64 = (locals.var_cox0 * assign20540_e25524);
        (assign20540_e25525, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (-locals.var_ps0ld_dn4)), (locals.var_cox0 * (locals.var_vgpld_dn5 - locals.var_ps0ld_dn5)), (locals.var_cox0 * (-locals.var_ps0ld_dn6)), (locals.var_cox0 * (-locals.var_ps0ld_dn8)), (locals.var_cox0 * (-locals.var_ps0ld_dn10)), (locals.var_cox0 * (-locals.var_ps0ld_dn11)), (locals.var_cox0 * (-locals.var_ps0ld_dn12)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12,)
    }
};
        locals.var_qsuld = assign20540_e25527;
        locals.var_qsuld_dn0 = assign20540_e25527_d_n0;
        locals.var_qsuld_dn2 = assign20540_e25527_d_n2;
        locals.var_qsuld_dn4 = assign20540_e25527_d_n4;
        locals.var_qsuld_dn5 = assign20540_e25527_d_n5;
        locals.var_qsuld_dn6 = assign20540_e25527_d_n6;
        locals.var_qsuld_dn8 = assign20540_e25527_d_n8;
        locals.var_qsuld_dn10 = assign20540_e25527_d_n10;
        locals.var_qsuld_dn11 = assign20540_e25527_d_n11;
        locals.var_qsuld_dn12 = assign20540_e25527_d_n12;
        locals.var_qsuld_rv = 0.0;

        let assign20550_e25530: f64 = if p.p30 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard353 = assign20550_e25530;
        locals.var_guard353_rv = 0.0;

        let (assign20560_e25543, assign20560_e25543_d_n0, assign20560_e25543_d_n2, assign20560_e25543_d_n4, assign20560_e25543_d_n5, assign20560_e25543_d_n6, assign20560_e25543_d_n8, assign20560_e25543_d_n10, assign20560_e25543_d_n11, assign20560_e25543_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
        let assign20560_e25539: f64 = (-locals.var_vxbgmtcl);
        let assign20560_e25540: f64 = (locals.var_beta * assign20560_e25539);
        let assign20560_e25541: f64 = (assign20560_e25540).exp();
        (assign20560_e25541, (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign20560_e25541 * ((locals.var_beta_dn4 * assign20560_e25539) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn12,)
    }
};
        locals.var_exp_bvbs = assign20560_e25543;
        locals.var_exp_bvbs_dn0 = assign20560_e25543_d_n0;
        locals.var_exp_bvbs_dn2 = assign20560_e25543_d_n2;
        locals.var_exp_bvbs_dn4 = assign20560_e25543_d_n4;
        locals.var_exp_bvbs_dn5 = assign20560_e25543_d_n5;
        locals.var_exp_bvbs_dn6 = assign20560_e25543_d_n6;
        locals.var_exp_bvbs_dn8 = assign20560_e25543_d_n8;
        locals.var_exp_bvbs_dn10 = assign20560_e25543_d_n10;
        locals.var_exp_bvbs_dn11 = assign20560_e25543_d_n11;
        locals.var_exp_bvbs_dn12 = assign20560_e25543_d_n12;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign20570_e25554, assign20570_e25554_d_n0, assign20570_e25554_d_n2, assign20570_e25554_d_n4, assign20570_e25554_d_n5, assign20570_e25554_d_n6, assign20570_e25554_d_n8, assign20570_e25554_d_n10, assign20570_e25554_d_n11, assign20570_e25554_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
        let assign20570_e25552: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign20570_e25552, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn4 / locals.var_mks_nover), (locals.var_nin_dn5 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn8 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign20570_e25554;
        locals.var_t0_dn0 = assign20570_e25554_d_n0;
        locals.var_t0_dn2 = assign20570_e25554_d_n2;
        locals.var_t0_dn4 = assign20570_e25554_d_n4;
        locals.var_t0_dn5 = assign20570_e25554_d_n5;
        locals.var_t0_dn6 = assign20570_e25554_d_n6;
        locals.var_t0_dn8 = assign20570_e25554_d_n8;
        locals.var_t0_dn10 = assign20570_e25554_d_n10;
        locals.var_t0_dn11 = assign20570_e25554_d_n11;
        locals.var_t0_dn12 = assign20570_e25554_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign20580_e25565, assign20580_e25565_d_n0, assign20580_e25565_d_n2, assign20580_e25565_d_n4, assign20580_e25565_d_n5, assign20580_e25565_d_n6, assign20580_e25565_d_n8, assign20580_e25565_d_n10, assign20580_e25565_d_n11, assign20580_e25565_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
        let assign20580_e25563: f64 = (locals.var_t0 * locals.var_t0);
        (assign20580_e25563, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn8, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12,)
    }
};
        locals.var_cnst1over = assign20580_e25565;
        locals.var_cnst1over_dn0 = assign20580_e25565_d_n0;
        locals.var_cnst1over_dn2 = assign20580_e25565_d_n2;
        locals.var_cnst1over_dn4 = assign20580_e25565_d_n4;
        locals.var_cnst1over_dn5 = assign20580_e25565_d_n5;
        locals.var_cnst1over_dn6 = assign20580_e25565_d_n6;
        locals.var_cnst1over_dn8 = assign20580_e25565_d_n8;
        locals.var_cnst1over_dn10 = assign20580_e25565_d_n10;
        locals.var_cnst1over_dn11 = assign20580_e25565_d_n11;
        locals.var_cnst1over_dn12 = assign20580_e25565_d_n12;
        locals.var_cnst1over_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_83(
        locals: &mut StampLocals,
    ) {
        let (assign20590_e25576, assign20590_e25576_d_n0, assign20590_e25576_d_n2, assign20590_e25576_d_n4, assign20590_e25576_d_n5, assign20590_e25576_d_n6, assign20590_e25576_d_n8, assign20590_e25576_d_n10, assign20590_e25576_d_n11, assign20590_e25576_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
        let assign20590_e25574: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign20590_e25574, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn12)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn8, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn12,)
    }
};
        locals.var_cfs1 = assign20590_e25576;
        locals.var_cfs1_dn0 = assign20590_e25576_d_n0;
        locals.var_cfs1_dn2 = assign20590_e25576_d_n2;
        locals.var_cfs1_dn4 = assign20590_e25576_d_n4;
        locals.var_cfs1_dn5 = assign20590_e25576_d_n5;
        locals.var_cfs1_dn6 = assign20590_e25576_d_n6;
        locals.var_cfs1_dn8 = assign20590_e25576_d_n8;
        locals.var_cfs1_dn10 = assign20590_e25576_d_n10;
        locals.var_cfs1_dn11 = assign20590_e25576_d_n11;
        locals.var_cfs1_dn12 = assign20590_e25576_d_n12;
        locals.var_cfs1_rv = 0.0;

        let (assign20600_e25585,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign20600_e25585;
        locals.var_flg_conv_rv = 0.0;

        let (assign20610_e25594,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign20610_e25594;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_84(
        locals: &mut StampLocals,
    ) {
        let mut assign20620_loop_guard: usize = 0;
        while {
            let assign20620_cond_e25604: f64 = (40.0 + 1.0);
            let assign20620_cond_e25606: f64 = if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_lp_s0 <= assign20620_cond_e25604)) { 1.0 } else { 0.0 };
            assign20620_cond_e25606 != 0.0
        } {
            assign20620_loop_guard += 1;
            assert!(assign20620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign20620_body0_e25619, assign20620_body0_e25619_d_n0, assign20620_body0_e25619_d_n2, assign20620_body0_e25619_d_n4, assign20620_body0_e25619_d_n5, assign20620_body0_e25619_d_n6, assign20620_body0_e25619_d_n8, assign20620_body0_e25619_d_n10, assign20620_body0_e25619_d_n11, assign20620_body0_e25619_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
        let assign20620_body0_e25616: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        let assign20620_body0_e25617: f64 = (locals.var_beta * assign20620_body0_e25616);
        (assign20620_body0_e25617, (locals.var_beta * (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign20620_body0_e25616) + (locals.var_beta * (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0ld_dn12 + locals.var_vxbgmtcl_dn12)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12,)
    }
};
            locals.var_chi = assign20620_body0_e25619;
            locals.var_chi_dn0 = assign20620_body0_e25619_d_n0;
            locals.var_chi_dn2 = assign20620_body0_e25619_d_n2;
            locals.var_chi_dn4 = assign20620_body0_e25619_d_n4;
            locals.var_chi_dn5 = assign20620_body0_e25619_d_n5;
            locals.var_chi_dn6 = assign20620_body0_e25619_d_n6;
            locals.var_chi_dn8 = assign20620_body0_e25619_d_n8;
            locals.var_chi_dn10 = assign20620_body0_e25619_d_n10;
            locals.var_chi_dn11 = assign20620_body0_e25619_d_n11;
            locals.var_chi_dn12 = assign20620_body0_e25619_d_n12;
            locals.var_chi_rv = 0.0;
            let assign20620_body1_e25622: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard354 = assign20620_body1_e25622;
            locals.var_guard354_rv = 0.0;
            let (assign20620_body2_e25648, assign20620_body2_e25648_d_n0, assign20620_body2_e25648_d_n2, assign20620_body2_e25648_d_n4, assign20620_body2_e25648_d_n5, assign20620_body2_e25648_d_n6, assign20620_body2_e25648_d_n8, assign20620_body2_e25648_d_n10, assign20620_body2_e25648_d_n11, assign20620_body2_e25648_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
        let assign20620_body2_e25633: f64 = (locals.var_chi * locals.var_chi);
        let assign20620_body2_e25635: f64 = (assign20620_body2_e25633 * locals.var_chi);
        let assign20620_body2_e25639: f64 = (-0.07053654284009761);
        let assign20620_body2_e25642: f64 = (locals.var_chi * 0.006115288895133179);
        let assign20620_body2_e25643: f64 = (assign20620_body2_e25639 + assign20620_body2_e25642);
        let assign20620_body2_e25644: f64 = (locals.var_chi * assign20620_body2_e25643);
        let assign20620_body2_e25645: f64 = (0.29693154855771 + assign20620_body2_e25644);
        let assign20620_body2_e25646: f64 = (assign20620_body2_e25635 * assign20620_body2_e25645);
        (assign20620_body2_e25646, ((((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn0)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn0 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn0 * 0.006115288895133179))))), ((((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn2)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn2 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn2 * 0.006115288895133179))))), ((((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn4)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn4 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn4 * 0.006115288895133179))))), ((((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn5)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn5 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn5 * 0.006115288895133179))))), ((((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn6)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn6 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn6 * 0.006115288895133179))))), ((((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn8)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn8 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn8 * 0.006115288895133179))))), ((((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn10)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn10 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn10 * 0.006115288895133179))))), ((((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn11)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn11 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn11 * 0.006115288895133179))))), ((((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn12)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn12 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn12 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn4, locals.var_fi_dn5, locals.var_fi_dn6, locals.var_fi_dn8, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn12,)
    }
};
            locals.var_fi = assign20620_body2_e25648;
            locals.var_fi_dn0 = assign20620_body2_e25648_d_n0;
            locals.var_fi_dn2 = assign20620_body2_e25648_d_n2;
            locals.var_fi_dn4 = assign20620_body2_e25648_d_n4;
            locals.var_fi_dn5 = assign20620_body2_e25648_d_n5;
            locals.var_fi_dn6 = assign20620_body2_e25648_d_n6;
            locals.var_fi_dn8 = assign20620_body2_e25648_d_n8;
            locals.var_fi_dn10 = assign20620_body2_e25648_d_n10;
            locals.var_fi_dn11 = assign20620_body2_e25648_d_n11;
            locals.var_fi_dn12 = assign20620_body2_e25648_d_n12;
            locals.var_fi_rv = 0.0;
            let (assign20620_body3_e25678, assign20620_body3_e25678_d_n0, assign20620_body3_e25678_d_n2, assign20620_body3_e25678_d_n4, assign20620_body3_e25678_d_n5, assign20620_body3_e25678_d_n6, assign20620_body3_e25678_d_n8, assign20620_body3_e25678_d_n10, assign20620_body3_e25678_d_n11, assign20620_body3_e25678_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
        let assign20620_body3_e25659: f64 = (locals.var_chi * locals.var_chi);
        let assign20620_body3_e25662: f64 = (3.0 * 0.29693154855771);
        let assign20620_body3_e25666: f64 = (-0.07053654284009761);
        let assign20620_body3_e25667: f64 = (4.0 * assign20620_body3_e25666);
        let assign20620_body3_e25670: f64 = (locals.var_chi * 5.0);
        let assign20620_body3_e25672: f64 = (assign20620_body3_e25670 * 0.006115288895133179);
        let assign20620_body3_e25673: f64 = (assign20620_body3_e25667 + assign20620_body3_e25672);
        let assign20620_body3_e25674: f64 = (locals.var_chi * assign20620_body3_e25673);
        let assign20620_body3_e25675: f64 = (assign20620_body3_e25662 + assign20620_body3_e25674);
        let assign20620_body3_e25676: f64 = (assign20620_body3_e25659 * assign20620_body3_e25675);
        (assign20620_body3_e25676, ((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn0 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn2 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn4 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn5 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn6 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn8 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn10 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn11 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn12 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn12 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn4, locals.var_fi_dchi_dn5, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn8, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn12,)
    }
};
            locals.var_fi_dchi = assign20620_body3_e25678;
            locals.var_fi_dchi_dn0 = assign20620_body3_e25678_d_n0;
            locals.var_fi_dchi_dn2 = assign20620_body3_e25678_d_n2;
            locals.var_fi_dchi_dn4 = assign20620_body3_e25678_d_n4;
            locals.var_fi_dchi_dn5 = assign20620_body3_e25678_d_n5;
            locals.var_fi_dchi_dn6 = assign20620_body3_e25678_d_n6;
            locals.var_fi_dchi_dn8 = assign20620_body3_e25678_d_n8;
            locals.var_fi_dchi_dn10 = assign20620_body3_e25678_d_n10;
            locals.var_fi_dchi_dn11 = assign20620_body3_e25678_d_n11;
            locals.var_fi_dchi_dn12 = assign20620_body3_e25678_d_n12;
            locals.var_fi_dchi_rv = 0.0;
            let (assign20620_body4_e25693, assign20620_body4_e25693_d_n0, assign20620_body4_e25693_d_n2, assign20620_body4_e25693_d_n4, assign20620_body4_e25693_d_n5, assign20620_body4_e25693_d_n6, assign20620_body4_e25693_d_n8, assign20620_body4_e25693_d_n10, assign20620_body4_e25693_d_n11, assign20620_body4_e25693_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
        let assign20620_body4_e25689: f64 = (locals.var_cfs1 * locals.var_fi);
        let assign20620_body4_e25691: f64 = (assign20620_body4_e25689 * locals.var_fi);
        (assign20620_body4_e25691, ((((locals.var_cfs1_dn0 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn0)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn2)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn4)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn5)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn6)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn6)), ((((locals.var_cfs1_dn8 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn8)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn8)), ((((locals.var_cfs1_dn10 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn10)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn10)), ((((locals.var_cfs1_dn11 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn11)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn11)), ((((locals.var_cfs1_dn12 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn12)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn12)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn8, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12,)
    }
};
            locals.var_fs01 = assign20620_body4_e25693;
            locals.var_fs01_dn0 = assign20620_body4_e25693_d_n0;
            locals.var_fs01_dn2 = assign20620_body4_e25693_d_n2;
            locals.var_fs01_dn4 = assign20620_body4_e25693_d_n4;
            locals.var_fs01_dn5 = assign20620_body4_e25693_d_n5;
            locals.var_fs01_dn6 = assign20620_body4_e25693_d_n6;
            locals.var_fs01_dn8 = assign20620_body4_e25693_d_n8;
            locals.var_fs01_dn10 = assign20620_body4_e25693_d_n10;
            locals.var_fs01_dn11 = assign20620_body4_e25693_d_n11;
            locals.var_fs01_dn12 = assign20620_body4_e25693_d_n12;
            locals.var_fs01_rv = 0.0;
            let (assign20620_body5_e25712, assign20620_body5_e25712_d_n0, assign20620_body5_e25712_d_n2, assign20620_body5_e25712_d_n4, assign20620_body5_e25712_d_n5, assign20620_body5_e25712_d_n6, assign20620_body5_e25712_d_n8, assign20620_body5_e25712_d_n10, assign20620_body5_e25712_d_n11, assign20620_body5_e25712_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
        let assign20620_body5_e25704: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign20620_body5_e25706: f64 = (assign20620_body5_e25704 * 2.0);
        let assign20620_body5_e25708: f64 = (assign20620_body5_e25706 * locals.var_fi);
        let assign20620_body5_e25710: f64 = (assign20620_body5_e25708 * locals.var_fi_dchi);
        (assign20620_body5_e25710, ((((((locals.var_cfs1_dn0 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn0)), ((((((locals.var_cfs1_dn2 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn2)), (((((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn4)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn4)), ((((((locals.var_cfs1_dn5 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn5)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn5)), ((((((locals.var_cfs1_dn6 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn6)), ((((((locals.var_cfs1_dn8 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn8)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn8)), ((((((locals.var_cfs1_dn10 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn10)), ((((((locals.var_cfs1_dn11 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn11)), ((((((locals.var_cfs1_dn12 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn12)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn12)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12,)
    }
};
            locals.var_fs01_dps0 = assign20620_body5_e25712;
            locals.var_fs01_dps0_dn0 = assign20620_body5_e25712_d_n0;
            locals.var_fs01_dps0_dn2 = assign20620_body5_e25712_d_n2;
            locals.var_fs01_dps0_dn4 = assign20620_body5_e25712_d_n4;
            locals.var_fs01_dps0_dn5 = assign20620_body5_e25712_d_n5;
            locals.var_fs01_dps0_dn6 = assign20620_body5_e25712_d_n6;
            locals.var_fs01_dps0_dn8 = assign20620_body5_e25712_d_n8;
            locals.var_fs01_dps0_dn10 = assign20620_body5_e25712_d_n10;
            locals.var_fs01_dps0_dn11 = assign20620_body5_e25712_d_n11;
            locals.var_fs01_dps0_dn12 = assign20620_body5_e25712_d_n12;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign20620_body6_e25743, assign20620_body6_e25743_d_n0, assign20620_body6_e25743_d_n2, assign20620_body6_e25743_d_n4, assign20620_body6_e25743_d_n5, assign20620_body6_e25743_d_n6, assign20620_body6_e25743_d_n8, assign20620_body6_e25743_d_n10, assign20620_body6_e25743_d_n11, assign20620_body6_e25743_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
        let assign20620_body6_e25725: f64 = (-0.117851130197758);
        let assign20620_body6_e25730: f64 = (-0.00163730162779191);
        let assign20620_body6_e25733: f64 = (locals.var_chi * 6.36964918866352e-5);
        let assign20620_body6_e25734: f64 = (assign20620_body6_e25730 + assign20620_body6_e25733);
        let assign20620_body6_e25735: f64 = (locals.var_chi * assign20620_body6_e25734);
        let assign20620_body6_e25736: f64 = (0.0178800506338833 + assign20620_body6_e25735);
        let assign20620_body6_e25737: f64 = (locals.var_chi * assign20620_body6_e25736);
        let assign20620_body6_e25738: f64 = (assign20620_body6_e25725 + assign20620_body6_e25737);
        let assign20620_body6_e25739: f64 = (locals.var_chi * assign20620_body6_e25738);
        let assign20620_body6_e25740: f64 = (0.707106781186548 + assign20620_body6_e25739);
        let assign20620_body6_e25741: f64 = (locals.var_chi * assign20620_body6_e25740);
        (assign20620_body6_e25741, ((locals.var_chi_dn0 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn0 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn0 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn0 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn2 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn2 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn2 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn2 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn4 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn4 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn4 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn4 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn4 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn5 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn5 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn5 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn5 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn5 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn6 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn6 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn6 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn6 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn8 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn8 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn8 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn8 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn8 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn10 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn10 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn10 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn10 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn11 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn11 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn11 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn11 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn12 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn12 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn12 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn12 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn12 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn8, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12,)
    }
};
            locals.var_fb = assign20620_body6_e25743;
            locals.var_fb_dn0 = assign20620_body6_e25743_d_n0;
            locals.var_fb_dn2 = assign20620_body6_e25743_d_n2;
            locals.var_fb_dn4 = assign20620_body6_e25743_d_n4;
            locals.var_fb_dn5 = assign20620_body6_e25743_d_n5;
            locals.var_fb_dn6 = assign20620_body6_e25743_d_n6;
            locals.var_fb_dn8 = assign20620_body6_e25743_d_n8;
            locals.var_fb_dn10 = assign20620_body6_e25743_d_n10;
            locals.var_fb_dn11 = assign20620_body6_e25743_d_n11;
            locals.var_fb_dn12 = assign20620_body6_e25743_d_n12;
            locals.var_fb_rv = 0.0;
            let (assign20620_body7_e25780, assign20620_body7_e25780_d_n0, assign20620_body7_e25780_d_n2, assign20620_body7_e25780_d_n4, assign20620_body7_e25780_d_n5, assign20620_body7_e25780_d_n6, assign20620_body7_e25780_d_n8, assign20620_body7_e25780_d_n10, assign20620_body7_e25780_d_n11, assign20620_body7_e25780_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
        let assign20620_body7_e25756: f64 = (-0.117851130197758);
        let assign20620_body7_e25757: f64 = (2.0 * assign20620_body7_e25756);
        let assign20620_body7_e25761: f64 = (3.0 * 0.0178800506338833);
        let assign20620_body7_e25765: f64 = (-0.00163730162779191);
        let assign20620_body7_e25766: f64 = (4.0 * assign20620_body7_e25765);
        let assign20620_body7_e25769: f64 = (locals.var_chi * 5.0);
        let assign20620_body7_e25771: f64 = (assign20620_body7_e25769 * 6.36964918866352e-5);
        let assign20620_body7_e25772: f64 = (assign20620_body7_e25766 + assign20620_body7_e25771);
        let assign20620_body7_e25773: f64 = (locals.var_chi * assign20620_body7_e25772);
        let assign20620_body7_e25774: f64 = (assign20620_body7_e25761 + assign20620_body7_e25773);
        let assign20620_body7_e25775: f64 = (locals.var_chi * assign20620_body7_e25774);
        let assign20620_body7_e25776: f64 = (assign20620_body7_e25757 + assign20620_body7_e25775);
        let assign20620_body7_e25777: f64 = (locals.var_chi * assign20620_body7_e25776);
        let assign20620_body7_e25778: f64 = (0.707106781186548 + assign20620_body7_e25777);
        (assign20620_body7_e25778, ((locals.var_chi_dn0 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn0 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn0 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn2 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn2 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn2 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn4 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn4 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn4 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn5 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn5 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn5 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn6 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn6 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn6 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn8 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn8 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn8 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn10 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn10 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn10 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn11 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn11 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn11 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn12 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn12 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn12 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn12 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn4, locals.var_fb_dchi_dn5, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn8, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn12,)
    }
};
            locals.var_fb_dchi = assign20620_body7_e25780;
            locals.var_fb_dchi_dn0 = assign20620_body7_e25780_d_n0;
            locals.var_fb_dchi_dn2 = assign20620_body7_e25780_d_n2;
            locals.var_fb_dchi_dn4 = assign20620_body7_e25780_d_n4;
            locals.var_fb_dchi_dn5 = assign20620_body7_e25780_d_n5;
            locals.var_fb_dchi_dn6 = assign20620_body7_e25780_d_n6;
            locals.var_fb_dchi_dn8 = assign20620_body7_e25780_d_n8;
            locals.var_fb_dchi_dn10 = assign20620_body7_e25780_d_n10;
            locals.var_fb_dchi_dn11 = assign20620_body7_e25780_d_n11;
            locals.var_fb_dchi_dn12 = assign20620_body7_e25780_d_n12;
            locals.var_fb_dchi_rv = 0.0;
            let (assign20620_body8_e25798, assign20620_body8_e25798_d_n0, assign20620_body8_e25798_d_n2, assign20620_body8_e25798_d_n4, assign20620_body8_e25798_d_n5, assign20620_body8_e25798_d_n6, assign20620_body8_e25798_d_n8, assign20620_body8_e25798_d_n10, assign20620_body8_e25798_d_n11, assign20620_body8_e25798_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
        let assign20620_body8_e25791: f64 = (locals.var_fb * locals.var_fb);
        let assign20620_body8_e25793: f64 = (assign20620_body8_e25791 + locals.var_fs01);
        let assign20620_body8_e25795: f64 = (assign20620_body8_e25793 + 1e-50);
        let assign20620_body8_e25796: f64 = (assign20620_body8_e25795).sqrt();
        (assign20620_body8_e25796, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fs01_dn12) / (2.0 * assign20620_body8_e25796)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn8, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12,)
    }
};
            locals.var_fs02 = assign20620_body8_e25798;
            locals.var_fs02_dn0 = assign20620_body8_e25798_d_n0;
            locals.var_fs02_dn2 = assign20620_body8_e25798_d_n2;
            locals.var_fs02_dn4 = assign20620_body8_e25798_d_n4;
            locals.var_fs02_dn5 = assign20620_body8_e25798_d_n5;
            locals.var_fs02_dn6 = assign20620_body8_e25798_d_n6;
            locals.var_fs02_dn8 = assign20620_body8_e25798_d_n8;
            locals.var_fs02_dn10 = assign20620_body8_e25798_d_n10;
            locals.var_fs02_dn11 = assign20620_body8_e25798_d_n11;
            locals.var_fs02_dn12 = assign20620_body8_e25798_d_n12;
            locals.var_fs02_rv = 0.0;
            let (assign20620_body9_e25821, assign20620_body9_e25821_d_n0, assign20620_body9_e25821_d_n2, assign20620_body9_e25821_d_n4, assign20620_body9_e25821_d_n5, assign20620_body9_e25821_d_n6, assign20620_body9_e25821_d_n8, assign20620_body9_e25821_d_n10, assign20620_body9_e25821_d_n11, assign20620_body9_e25821_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
        let assign20620_body9_e25809: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign20620_body9_e25811: f64 = (assign20620_body9_e25809 * 2.0);
        let assign20620_body9_e25813: f64 = (assign20620_body9_e25811 * locals.var_fb);
        let assign20620_body9_e25815: f64 = (assign20620_body9_e25813 + locals.var_fs01_dps0);
        let assign20620_body9_e25818: f64 = (locals.var_fs02 + locals.var_fs02);
        let assign20620_body9_e25819: f64 = (assign20620_body9_e25815 / assign20620_body9_e25818);
        (assign20620_body9_e25819, ((((((((locals.var_beta * locals.var_fb_dchi_dn0) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn0 + locals.var_fs02_dn0))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn2) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn2 + locals.var_fs02_dn2))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), (((((((((locals.var_beta_dn4 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn4)) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn4 + locals.var_fs02_dn4))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn5) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn5 + locals.var_fs02_dn5))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn6) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn6 + locals.var_fs02_dn6))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn8) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn8 + locals.var_fs02_dn8))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn10) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn10 + locals.var_fs02_dn10))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn11) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn11 + locals.var_fs02_dn11))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn12) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn12)) + locals.var_fs01_dps0_dn12) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn12 + locals.var_fs02_dn12))) / (assign20620_body9_e25818 * assign20620_body9_e25818)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12,)
    }
};
            locals.var_fs02_dps0 = assign20620_body9_e25821;
            locals.var_fs02_dps0_dn0 = assign20620_body9_e25821_d_n0;
            locals.var_fs02_dps0_dn2 = assign20620_body9_e25821_d_n2;
            locals.var_fs02_dps0_dn4 = assign20620_body9_e25821_d_n4;
            locals.var_fs02_dps0_dn5 = assign20620_body9_e25821_d_n5;
            locals.var_fs02_dps0_dn6 = assign20620_body9_e25821_d_n6;
            locals.var_fs02_dps0_dn8 = assign20620_body9_e25821_d_n8;
            locals.var_fs02_dps0_dn10 = assign20620_body9_e25821_d_n10;
            locals.var_fs02_dps0_dn11 = assign20620_body9_e25821_d_n11;
            locals.var_fs02_dps0_dn12 = assign20620_body9_e25821_d_n12;
            locals.var_fs02_dps0_rv = 0.0;
            let assign20620_body10_e25824: f64 = if locals.var_chi < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard355 = assign20620_body10_e25824;
            locals.var_guard355_rv = 0.0;
            let (assign20620_body11_e25839, assign20620_body11_e25839_d_n0, assign20620_body11_e25839_d_n2, assign20620_body11_e25839_d_n4, assign20620_body11_e25839_d_n5, assign20620_body11_e25839_d_n6, assign20620_body11_e25839_d_n8, assign20620_body11_e25839_d_n10, assign20620_body11_e25839_d_n11, assign20620_body11_e25839_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 != 0.0)) {
        let assign20620_body11_e25837: f64 = (locals.var_chi).exp();
        (assign20620_body11_e25837, (assign20620_body11_e25837 * locals.var_chi_dn0), (assign20620_body11_e25837 * locals.var_chi_dn2), (assign20620_body11_e25837 * locals.var_chi_dn4), (assign20620_body11_e25837 * locals.var_chi_dn5), (assign20620_body11_e25837 * locals.var_chi_dn6), (assign20620_body11_e25837 * locals.var_chi_dn8), (assign20620_body11_e25837 * locals.var_chi_dn10), (assign20620_body11_e25837 * locals.var_chi_dn11), (assign20620_body11_e25837 * locals.var_chi_dn12),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn8, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12,)
    }
};
            locals.var_exp_chi = assign20620_body11_e25839;
            locals.var_exp_chi_dn0 = assign20620_body11_e25839_d_n0;
            locals.var_exp_chi_dn2 = assign20620_body11_e25839_d_n2;
            locals.var_exp_chi_dn4 = assign20620_body11_e25839_d_n4;
            locals.var_exp_chi_dn5 = assign20620_body11_e25839_d_n5;
            locals.var_exp_chi_dn6 = assign20620_body11_e25839_d_n6;
            locals.var_exp_chi_dn8 = assign20620_body11_e25839_d_n8;
            locals.var_exp_chi_dn10 = assign20620_body11_e25839_d_n10;
            locals.var_exp_chi_dn11 = assign20620_body11_e25839_d_n11;
            locals.var_exp_chi_dn12 = assign20620_body11_e25839_d_n12;
            locals.var_exp_chi_rv = 0.0;
            let (assign20620_body12_e25857, assign20620_body12_e25857_d_n0, assign20620_body12_e25857_d_n2, assign20620_body12_e25857_d_n4, assign20620_body12_e25857_d_n5, assign20620_body12_e25857_d_n6, assign20620_body12_e25857_d_n8, assign20620_body12_e25857_d_n10, assign20620_body12_e25857_d_n11, assign20620_body12_e25857_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 != 0.0)) {
        let assign20620_body12_e25854: f64 = (locals.var_exp_chi - 1.0);
        let assign20620_body12_e25855: f64 = (locals.var_cfs1 * assign20620_body12_e25854);
        (assign20620_body12_e25855, ((locals.var_cfs1_dn0 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn0)), ((locals.var_cfs1_dn2 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn2)), ((locals.var_cfs1_dn4 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn4)), ((locals.var_cfs1_dn5 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn5)), ((locals.var_cfs1_dn6 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn6)), ((locals.var_cfs1_dn8 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn8)), ((locals.var_cfs1_dn10 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn10)), ((locals.var_cfs1_dn11 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn11)), ((locals.var_cfs1_dn12 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn12)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn8, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12,)
    }
};
            locals.var_fs01 = assign20620_body12_e25857;
            locals.var_fs01_dn0 = assign20620_body12_e25857_d_n0;
            locals.var_fs01_dn2 = assign20620_body12_e25857_d_n2;
            locals.var_fs01_dn4 = assign20620_body12_e25857_d_n4;
            locals.var_fs01_dn5 = assign20620_body12_e25857_d_n5;
            locals.var_fs01_dn6 = assign20620_body12_e25857_d_n6;
            locals.var_fs01_dn8 = assign20620_body12_e25857_d_n8;
            locals.var_fs01_dn10 = assign20620_body12_e25857_d_n10;
            locals.var_fs01_dn11 = assign20620_body12_e25857_d_n11;
            locals.var_fs01_dn12 = assign20620_body12_e25857_d_n12;
            locals.var_fs01_rv = 0.0;
            let (assign20620_body13_e25875, assign20620_body13_e25875_d_n0, assign20620_body13_e25875_d_n2, assign20620_body13_e25875_d_n4, assign20620_body13_e25875_d_n5, assign20620_body13_e25875_d_n6, assign20620_body13_e25875_d_n8, assign20620_body13_e25875_d_n10, assign20620_body13_e25875_d_n11, assign20620_body13_e25875_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 != 0.0)) {
        let assign20620_body13_e25871: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign20620_body13_e25873: f64 = (assign20620_body13_e25871 * locals.var_exp_chi);
        (assign20620_body13_e25873, (((locals.var_cfs1_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn0)), (((locals.var_cfs1_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn4)), (((locals.var_cfs1_dn5 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn5)), (((locals.var_cfs1_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn6)), (((locals.var_cfs1_dn8 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn8)), (((locals.var_cfs1_dn10 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn10)), (((locals.var_cfs1_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn11)), (((locals.var_cfs1_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn12)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12,)
    }
};
            locals.var_fs01_dps0 = assign20620_body13_e25875;
            locals.var_fs01_dps0_dn0 = assign20620_body13_e25875_d_n0;
            locals.var_fs01_dps0_dn2 = assign20620_body13_e25875_d_n2;
            locals.var_fs01_dps0_dn4 = assign20620_body13_e25875_d_n4;
            locals.var_fs01_dps0_dn5 = assign20620_body13_e25875_d_n5;
            locals.var_fs01_dps0_dn6 = assign20620_body13_e25875_d_n6;
            locals.var_fs01_dps0_dn8 = assign20620_body13_e25875_d_n8;
            locals.var_fs01_dps0_dn10 = assign20620_body13_e25875_d_n10;
            locals.var_fs01_dps0_dn11 = assign20620_body13_e25875_d_n11;
            locals.var_fs01_dps0_dn12 = assign20620_body13_e25875_d_n12;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign20620_body14_e25893, assign20620_body14_e25893_d_n0, assign20620_body14_e25893_d_n2, assign20620_body14_e25893_d_n4, assign20620_body14_e25893_d_n5, assign20620_body14_e25893_d_n6, assign20620_body14_e25893_d_n8, assign20620_body14_e25893_d_n10, assign20620_body14_e25893_d_n11, assign20620_body14_e25893_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 == 0.0)) {
        let assign20620_body14_e25890: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign20620_body14_e25891: f64 = (assign20620_body14_e25890).exp();
        (assign20620_body14_e25891, (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn0)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn2)), (assign20620_body14_e25891 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn5)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn6)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn8)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn10)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn11)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn12)),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn12,)
    }
};
            locals.var_exp_bps0 = assign20620_body14_e25893;
            locals.var_exp_bps0_dn0 = assign20620_body14_e25893_d_n0;
            locals.var_exp_bps0_dn2 = assign20620_body14_e25893_d_n2;
            locals.var_exp_bps0_dn4 = assign20620_body14_e25893_d_n4;
            locals.var_exp_bps0_dn5 = assign20620_body14_e25893_d_n5;
            locals.var_exp_bps0_dn6 = assign20620_body14_e25893_d_n6;
            locals.var_exp_bps0_dn8 = assign20620_body14_e25893_d_n8;
            locals.var_exp_bps0_dn10 = assign20620_body14_e25893_d_n10;
            locals.var_exp_bps0_dn11 = assign20620_body14_e25893_d_n11;
            locals.var_exp_bps0_dn12 = assign20620_body14_e25893_d_n12;
            locals.var_exp_bps0_rv = 0.0;
            let (assign20620_body15_e25912, assign20620_body15_e25912_d_n0, assign20620_body15_e25912_d_n2, assign20620_body15_e25912_d_n4, assign20620_body15_e25912_d_n5, assign20620_body15_e25912_d_n6, assign20620_body15_e25912_d_n8, assign20620_body15_e25912_d_n10, assign20620_body15_e25912_d_n11, assign20620_body15_e25912_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 == 0.0)) {
        let assign20620_body15_e25909: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign20620_body15_e25910: f64 = (locals.var_cnst1over * assign20620_body15_e25909);
        (assign20620_body15_e25910, ((locals.var_cnst1over_dn0 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((locals.var_cnst1over_dn2 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((locals.var_cnst1over_dn4 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((locals.var_cnst1over_dn5 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((locals.var_cnst1over_dn6 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((locals.var_cnst1over_dn8 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((locals.var_cnst1over_dn10 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((locals.var_cnst1over_dn11 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((locals.var_cnst1over_dn12 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn12 - locals.var_exp_bvbs_dn12))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn8, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12,)
    }
};
            locals.var_fs01 = assign20620_body15_e25912;
            locals.var_fs01_dn0 = assign20620_body15_e25912_d_n0;
            locals.var_fs01_dn2 = assign20620_body15_e25912_d_n2;
            locals.var_fs01_dn4 = assign20620_body15_e25912_d_n4;
            locals.var_fs01_dn5 = assign20620_body15_e25912_d_n5;
            locals.var_fs01_dn6 = assign20620_body15_e25912_d_n6;
            locals.var_fs01_dn8 = assign20620_body15_e25912_d_n8;
            locals.var_fs01_dn10 = assign20620_body15_e25912_d_n10;
            locals.var_fs01_dn11 = assign20620_body15_e25912_d_n11;
            locals.var_fs01_dn12 = assign20620_body15_e25912_d_n12;
            locals.var_fs01_rv = 0.0;
            let (assign20620_body16_e25931, assign20620_body16_e25931_d_n0, assign20620_body16_e25931_d_n2, assign20620_body16_e25931_d_n4, assign20620_body16_e25931_d_n5, assign20620_body16_e25931_d_n6, assign20620_body16_e25931_d_n8, assign20620_body16_e25931_d_n10, assign20620_body16_e25931_d_n11, assign20620_body16_e25931_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 == 0.0)) {
        let assign20620_body16_e25927: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign20620_body16_e25929: f64 = (assign20620_body16_e25927 * locals.var_exp_bps0);
        (assign20620_body16_e25929, (((locals.var_cnst1over_dn0 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn0)), (((locals.var_cnst1over_dn2 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn2)), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn4)), (((locals.var_cnst1over_dn5 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn5)), (((locals.var_cnst1over_dn6 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn6)), (((locals.var_cnst1over_dn8 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn8)), (((locals.var_cnst1over_dn10 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn10)), (((locals.var_cnst1over_dn11 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn11)), (((locals.var_cnst1over_dn12 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn12)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12,)
    }
};
            locals.var_fs01_dps0 = assign20620_body16_e25931;
            locals.var_fs01_dps0_dn0 = assign20620_body16_e25931_d_n0;
            locals.var_fs01_dps0_dn2 = assign20620_body16_e25931_d_n2;
            locals.var_fs01_dps0_dn4 = assign20620_body16_e25931_d_n4;
            locals.var_fs01_dps0_dn5 = assign20620_body16_e25931_d_n5;
            locals.var_fs01_dps0_dn6 = assign20620_body16_e25931_d_n6;
            locals.var_fs01_dps0_dn8 = assign20620_body16_e25931_d_n8;
            locals.var_fs01_dps0_dn10 = assign20620_body16_e25931_d_n10;
            locals.var_fs01_dps0_dn11 = assign20620_body16_e25931_d_n11;
            locals.var_fs01_dps0_dn12 = assign20620_body16_e25931_d_n12;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign20620_body17_e25948, assign20620_body17_e25948_d_n0, assign20620_body17_e25948_d_n2, assign20620_body17_e25948_d_n4, assign20620_body17_e25948_d_n5, assign20620_body17_e25948_d_n6, assign20620_body17_e25948_d_n8, assign20620_body17_e25948_d_n10, assign20620_body17_e25948_d_n11, assign20620_body17_e25948_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) {
        let assign20620_body17_e25943: f64 = (locals.var_chi - 1.0);
        let assign20620_body17_e25945: f64 = (assign20620_body17_e25943 + locals.var_fs01);
        let assign20620_body17_e25946: f64 = (assign20620_body17_e25945).sqrt();
        (assign20620_body17_e25946, ((locals.var_chi_dn0 + locals.var_fs01_dn0) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn2 + locals.var_fs01_dn2) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn4 + locals.var_fs01_dn4) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn5 + locals.var_fs01_dn5) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn6 + locals.var_fs01_dn6) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn8 + locals.var_fs01_dn8) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn10 + locals.var_fs01_dn10) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn11 + locals.var_fs01_dn11) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn12 + locals.var_fs01_dn12) / (2.0 * assign20620_body17_e25946)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn8, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12,)
    }
};
            locals.var_fs02 = assign20620_body17_e25948;
            locals.var_fs02_dn0 = assign20620_body17_e25948_d_n0;
            locals.var_fs02_dn2 = assign20620_body17_e25948_d_n2;
            locals.var_fs02_dn4 = assign20620_body17_e25948_d_n4;
            locals.var_fs02_dn5 = assign20620_body17_e25948_d_n5;
            locals.var_fs02_dn6 = assign20620_body17_e25948_d_n6;
            locals.var_fs02_dn8 = assign20620_body17_e25948_d_n8;
            locals.var_fs02_dn10 = assign20620_body17_e25948_d_n10;
            locals.var_fs02_dn11 = assign20620_body17_e25948_d_n11;
            locals.var_fs02_dn12 = assign20620_body17_e25948_d_n12;
            locals.var_fs02_rv = 0.0;
            let (assign20620_body18_e25966, assign20620_body18_e25966_d_n0, assign20620_body18_e25966_d_n2, assign20620_body18_e25966_d_n4, assign20620_body18_e25966_d_n5, assign20620_body18_e25966_d_n6, assign20620_body18_e25966_d_n8, assign20620_body18_e25966_d_n10, assign20620_body18_e25966_d_n11, assign20620_body18_e25966_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) {
        let assign20620_body18_e25960: f64 = (locals.var_beta + locals.var_fs01_dps0);
        let assign20620_body18_e25962: f64 = (assign20620_body18_e25960 / locals.var_fs02);
        let assign20620_body18_e25964: f64 = (assign20620_body18_e25962 * 0.5);
        (assign20620_body18_e25964, ((((locals.var_fs01_dps0_dn0 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn2 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), (((((locals.var_beta_dn4 + locals.var_fs01_dps0_dn4) * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn5 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn6 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn8 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn10 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn11 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn12 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn12)) / (locals.var_fs02 * locals.var_fs02)) * 0.5),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12,)
    }
};
            locals.var_fs02_dps0 = assign20620_body18_e25966;
            locals.var_fs02_dps0_dn0 = assign20620_body18_e25966_d_n0;
            locals.var_fs02_dps0_dn2 = assign20620_body18_e25966_d_n2;
            locals.var_fs02_dps0_dn4 = assign20620_body18_e25966_d_n4;
            locals.var_fs02_dps0_dn5 = assign20620_body18_e25966_d_n5;
            locals.var_fs02_dps0_dn6 = assign20620_body18_e25966_d_n6;
            locals.var_fs02_dps0_dn8 = assign20620_body18_e25966_d_n8;
            locals.var_fs02_dps0_dn10 = assign20620_body18_e25966_d_n10;
            locals.var_fs02_dps0_dn11 = assign20620_body18_e25966_d_n11;
            locals.var_fs02_dps0_dn12 = assign20620_body18_e25966_d_n12;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign20620_body19_e25981, assign20620_body19_e25981_d_n0, assign20620_body19_e25981_d_n2, assign20620_body19_e25981_d_n4, assign20620_body19_e25981_d_n5, assign20620_body19_e25981_d_n6, assign20620_body19_e25981_d_n8, assign20620_body19_e25981_d_n10, assign20620_body19_e25981_d_n11, assign20620_body19_e25981_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
        let assign20620_body19_e25975: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign20620_body19_e25978: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign20620_body19_e25979: f64 = (assign20620_body19_e25975 - assign20620_body19_e25978);
        (assign20620_body19_e25979, ((locals.var_vgpld_dn0 - locals.var_ps0ld_dn0) - ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), ((locals.var_vgpld_dn2 - locals.var_ps0ld_dn2) - ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), ((-locals.var_ps0ld_dn4) - ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), ((locals.var_vgpld_dn5 - locals.var_ps0ld_dn5) - ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), ((-locals.var_ps0ld_dn6) - ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), ((-locals.var_ps0ld_dn8) - ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), ((-locals.var_ps0ld_dn10) - ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), ((-locals.var_ps0ld_dn11) - ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), ((-locals.var_ps0ld_dn12) - ((locals.var_fac1_dn12 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn12))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn8, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn12,)
    }
};
            locals.var_fs0 = assign20620_body19_e25981;
            locals.var_fs0_dn0 = assign20620_body19_e25981_d_n0;
            locals.var_fs0_dn2 = assign20620_body19_e25981_d_n2;
            locals.var_fs0_dn4 = assign20620_body19_e25981_d_n4;
            locals.var_fs0_dn5 = assign20620_body19_e25981_d_n5;
            locals.var_fs0_dn6 = assign20620_body19_e25981_d_n6;
            locals.var_fs0_dn8 = assign20620_body19_e25981_d_n8;
            locals.var_fs0_dn10 = assign20620_body19_e25981_d_n10;
            locals.var_fs0_dn11 = assign20620_body19_e25981_d_n11;
            locals.var_fs0_dn12 = assign20620_body19_e25981_d_n12;
            locals.var_fs0_rv = 0.0;
            let (assign20620_body20_e25995, assign20620_body20_e25995_d_n0, assign20620_body20_e25995_d_n2, assign20620_body20_e25995_d_n4, assign20620_body20_e25995_d_n5, assign20620_body20_e25995_d_n6, assign20620_body20_e25995_d_n8, assign20620_body20_e25995_d_n10, assign20620_body20_e25995_d_n11, assign20620_body20_e25995_d_n12,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
        let assign20620_body20_e25989: f64 = (-1.0);
        let assign20620_body20_e25992: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign20620_body20_e25993: f64 = (assign20620_body20_e25989 - assign20620_body20_e25992);
        (assign20620_body20_e25993, (-((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0))), (-((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2))), (-((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4))), (-((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5))), (-((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6))), (-((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8))), (-((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10))), (-((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11))), (-((locals.var_fac1_dn12 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn12))),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn12,)
    }
};
            locals.var_fs0_dps0 = assign20620_body20_e25995;
            locals.var_fs0_dps0_dn0 = assign20620_body20_e25995_d_n0;
            locals.var_fs0_dps0_dn2 = assign20620_body20_e25995_d_n2;
            locals.var_fs0_dps0_dn4 = assign20620_body20_e25995_d_n4;
            locals.var_fs0_dps0_dn5 = assign20620_body20_e25995_d_n5;
            locals.var_fs0_dps0_dn6 = assign20620_body20_e25995_d_n6;
            locals.var_fs0_dps0_dn8 = assign20620_body20_e25995_d_n8;
            locals.var_fs0_dps0_dn10 = assign20620_body20_e25995_d_n10;
            locals.var_fs0_dps0_dn11 = assign20620_body20_e25995_d_n11;
            locals.var_fs0_dps0_dn12 = assign20620_body20_e25995_d_n12;
            locals.var_fs0_dps0_rv = 0.0;
            let assign20620_body21_e25998: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard356 = assign20620_body21_e25998;
            locals.var_guard356_rv = 0.0;
            let (assign20620_body22_e26011,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 != 0.0)) {
        let assign20620_body22_e26009: f64 = (40.0 + 1.0);
        (assign20620_body22_e26009,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign20620_body22_e26011;
            locals.var_lp_s0_rv = 0.0;
            let (assign20620_body23_e26026, assign20620_body23_e26026_d_n0, assign20620_body23_e26026_d_n2, assign20620_body23_e26026_d_n4, assign20620_body23_e26026_d_n5, assign20620_body23_e26026_d_n6, assign20620_body23_e26026_d_n8, assign20620_body23_e26026_d_n10, assign20620_body23_e26026_d_n11, assign20620_body23_e26026_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 == 0.0)) {
        let assign20620_body23_e26022: f64 = (-locals.var_fs0);
        let assign20620_body23_e26024: f64 = (assign20620_body23_e26022 / locals.var_fs0_dps0);
        (assign20620_body23_e26024, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn12) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn12)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn8, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12,)
    }
};
            locals.var_dps0 = assign20620_body23_e26026;
            locals.var_dps0_dn0 = assign20620_body23_e26026_d_n0;
            locals.var_dps0_dn2 = assign20620_body23_e26026_d_n2;
            locals.var_dps0_dn4 = assign20620_body23_e26026_d_n4;
            locals.var_dps0_dn5 = assign20620_body23_e26026_d_n5;
            locals.var_dps0_dn6 = assign20620_body23_e26026_d_n6;
            locals.var_dps0_dn8 = assign20620_body23_e26026_d_n8;
            locals.var_dps0_dn10 = assign20620_body23_e26026_d_n10;
            locals.var_dps0_dn11 = assign20620_body23_e26026_d_n11;
            locals.var_dps0_dn12 = assign20620_body23_e26026_d_n12;
            locals.var_dps0_rv = 0.0;
            let (assign20620_body24_e26051, assign20620_body24_e26051_d_n0, assign20620_body24_e26051_d_n2, assign20620_body24_e26051_d_n4, assign20620_body24_e26051_d_n5, assign20620_body24_e26051_d_n6, assign20620_body24_e26051_d_n8, assign20620_body24_e26051_d_n10, assign20620_body24_e26051_d_n11, assign20620_body24_e26051_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 == 0.0)) {
        let assign20620_body24_e26038: f64 = (0.5 * 0.1);
        let assign20620_body24_e26042: f64 = (locals.var_ps0ld).abs();
        let (assign20620_body24_e26047, assign20620_body24_e26047_d_n0, assign20620_body24_e26047_d_n2, assign20620_body24_e26047_d_n4, assign20620_body24_e26047_d_n5, assign20620_body24_e26047_d_n6, assign20620_body24_e26047_d_n8, assign20620_body24_e26047_d_n10, assign20620_body24_e26047_d_n11, assign20620_body24_e26047_d_n12,) = {
            if (1.0 >= assign20620_body24_e26042) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign20620_body24_e26046: f64 = (locals.var_ps0ld).abs();
                (assign20620_body24_e26046, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn12 } else { (-locals.var_ps0ld_dn12) },)
            }
        };
        let assign20620_body24_e26048: f64 = (1.0 + assign20620_body24_e26047);
        let assign20620_body24_e26049: f64 = (assign20620_body24_e26038 * assign20620_body24_e26048);
        (assign20620_body24_e26049, (assign20620_body24_e26038 * assign20620_body24_e26047_d_n0), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n2), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n4), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n5), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n6), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n8), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n10), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n11), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n12),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn8, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12,)
    }
};
            locals.var_dplim = assign20620_body24_e26051;
            locals.var_dplim_dn0 = assign20620_body24_e26051_d_n0;
            locals.var_dplim_dn2 = assign20620_body24_e26051_d_n2;
            locals.var_dplim_dn4 = assign20620_body24_e26051_d_n4;
            locals.var_dplim_dn5 = assign20620_body24_e26051_d_n5;
            locals.var_dplim_dn6 = assign20620_body24_e26051_d_n6;
            locals.var_dplim_dn8 = assign20620_body24_e26051_d_n8;
            locals.var_dplim_dn10 = assign20620_body24_e26051_d_n10;
            locals.var_dplim_dn11 = assign20620_body24_e26051_d_n11;
            locals.var_dplim_dn12 = assign20620_body24_e26051_d_n12;
            locals.var_dplim_rv = 0.0;
            let assign20620_body25_e26053: f64 = (locals.var_dps0).abs();
            let assign20620_body25_e26055: f64 = if assign20620_body25_e26053 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard357 = assign20620_body25_e26055;
            locals.var_guard357_rv = 0.0;
            let (assign20620_body26_e26077, assign20620_body26_e26077_d_n0, assign20620_body26_e26077_d_n2, assign20620_body26_e26077_d_n4, assign20620_body26_e26077_d_n5, assign20620_body26_e26077_d_n6, assign20620_body26_e26077_d_n8, assign20620_body26_e26077_d_n10, assign20620_body26_e26077_d_n11, assign20620_body26_e26077_d_n12,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 == 0.0)) && (locals.var_guard357 != 0.0)) {
        let (assign20620_body26_e26074,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign20620_body26_e26073: f64 = (-1.0);
                (assign20620_body26_e26073,)
            }
        };
        let assign20620_body26_e26075: f64 = (locals.var_dplim * assign20620_body26_e26074);
        (assign20620_body26_e26075, (locals.var_dplim_dn0 * assign20620_body26_e26074), (locals.var_dplim_dn2 * assign20620_body26_e26074), (locals.var_dplim_dn4 * assign20620_body26_e26074), (locals.var_dplim_dn5 * assign20620_body26_e26074), (locals.var_dplim_dn6 * assign20620_body26_e26074), (locals.var_dplim_dn8 * assign20620_body26_e26074), (locals.var_dplim_dn10 * assign20620_body26_e26074), (locals.var_dplim_dn11 * assign20620_body26_e26074), (locals.var_dplim_dn12 * assign20620_body26_e26074),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn8, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12,)
    }
};
            locals.var_dps0 = assign20620_body26_e26077;
            locals.var_dps0_dn0 = assign20620_body26_e26077_d_n0;
            locals.var_dps0_dn2 = assign20620_body26_e26077_d_n2;
            locals.var_dps0_dn4 = assign20620_body26_e26077_d_n4;
            locals.var_dps0_dn5 = assign20620_body26_e26077_d_n5;
            locals.var_dps0_dn6 = assign20620_body26_e26077_d_n6;
            locals.var_dps0_dn8 = assign20620_body26_e26077_d_n8;
            locals.var_dps0_dn10 = assign20620_body26_e26077_d_n10;
            locals.var_dps0_dn11 = assign20620_body26_e26077_d_n11;
            locals.var_dps0_dn12 = assign20620_body26_e26077_d_n12;
            locals.var_dps0_rv = 0.0;
            let (assign20620_body27_e26091, assign20620_body27_e26091_d_n0, assign20620_body27_e26091_d_n2, assign20620_body27_e26091_d_n4, assign20620_body27_e26091_d_n5, assign20620_body27_e26091_d_n6, assign20620_body27_e26091_d_n8, assign20620_body27_e26091_d_n10, assign20620_body27_e26091_d_n11, assign20620_body27_e26091_d_n12,) = {
    if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 == 0.0)) {
        let assign20620_body27_e26089: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign20620_body27_e26089, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn12 + locals.var_dps0_dn12),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn8, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12,)
    }
};
            locals.var_ps0ld = assign20620_body27_e26091;
            locals.var_ps0ld_dn0 = assign20620_body27_e26091_d_n0;
            locals.var_ps0ld_dn2 = assign20620_body27_e26091_d_n2;
            locals.var_ps0ld_dn4 = assign20620_body27_e26091_d_n4;
            locals.var_ps0ld_dn5 = assign20620_body27_e26091_d_n5;
            locals.var_ps0ld_dn6 = assign20620_body27_e26091_d_n6;
            locals.var_ps0ld_dn8 = assign20620_body27_e26091_d_n8;
            locals.var_ps0ld_dn10 = assign20620_body27_e26091_d_n10;
            locals.var_ps0ld_dn11 = assign20620_body27_e26091_d_n11;
            locals.var_ps0ld_dn12 = assign20620_body27_e26091_d_n12;
            locals.var_ps0ld_rv = 0.0;
            let assign20620_body28_e26093: f64 = (locals.var_dps0).abs();
            let assign20620_body28_e26097: f64 = (locals.var_fs0).abs();
            let assign20620_body28_e26100: f64 = if ((assign20620_body28_e26093 <= 1e-12) && (assign20620_body28_e26097 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard358 = assign20620_body28_e26100;
            locals.var_guard358_rv = 0.0;
            let (assign20620_body29_e26114,) = {
    if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 == 0.0)) && (locals.var_guard358 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign20620_body29_e26114;
            locals.var_flg_conv_rv = 0.0;
            let (assign20620_body30_e26125,) = {
    if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
        let assign20620_body30_e26123: f64 = (locals.var_lp_s0 + 1.0);
        (assign20620_body30_e26123,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign20620_body30_e26125;
            locals.var_lp_s0_rv = 0.0;
        }

    }
}

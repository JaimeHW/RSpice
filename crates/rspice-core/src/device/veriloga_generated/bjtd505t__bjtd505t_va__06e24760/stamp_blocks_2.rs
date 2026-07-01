#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_16(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let (assign6330_e6502, assign6330_e6502_d_n0, assign6330_e6502_d_n1, assign6330_e6502_d_n3, assign6330_e6502_d_n4, assign6330_e6502_d_n5, assign6330_e6502_d_n6, assign6330_e6502_d_n7, assign6330_e6502_d_n8, assign6330_e6502_d_n9, assign6330_e6502_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6330_e6489: f64 = (locals.var_if0 * locals.var_evb2e1);
        let assign6330_e6491: f64 = (assign6330_e6489 * locals.var_vtinv);
        let assign6330_e6493: f64 = (assign6330_e6491 / locals.var_nff_t);
        let assign6330_e6497: f64 = (1.0 + locals.var_f1);
        let assign6330_e6498: f64 = (assign6330_e6497).sqrt();
        let assign6330_e6499: f64 = (0.5 / assign6330_e6498);
        let assign6330_e6500: f64 = (assign6330_e6493 * assign6330_e6499);
        (assign6330_e6500, ((((((((locals.var_if0_dn0 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn0)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn0)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn0 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn1 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn1)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn1)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn1 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((locals.var_if0_dn3 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn3)) * locals.var_vtinv) + (assign6330_e6489 * locals.var_vtinv_dn3)) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn3)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn3 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn4 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn4)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn4)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn4 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn5 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn5)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn5)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn5 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn6 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn6)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn6)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn6 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn7 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn7)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn7)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn7 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn8 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn8)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn8)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn8 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn9 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn9)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn9)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn9 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn10 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn10)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn10)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn10 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))),)
    } else {
        (locals.var_dn0vb2e1, locals.var_dn0vb2e1_dn0, locals.var_dn0vb2e1_dn1, locals.var_dn0vb2e1_dn3, locals.var_dn0vb2e1_dn4, locals.var_dn0vb2e1_dn5, locals.var_dn0vb2e1_dn6, locals.var_dn0vb2e1_dn7, locals.var_dn0vb2e1_dn8, locals.var_dn0vb2e1_dn9, locals.var_dn0vb2e1_dn10,)
    }
};
        locals.var_dn0vb2e1 = assign6330_e6502;
        locals.var_dn0vb2e1_dn0 = assign6330_e6502_d_n0;
        locals.var_dn0vb2e1_dn1 = assign6330_e6502_d_n1;
        locals.var_dn0vb2e1_dn3 = assign6330_e6502_d_n3;
        locals.var_dn0vb2e1_dn4 = assign6330_e6502_d_n4;
        locals.var_dn0vb2e1_dn5 = assign6330_e6502_d_n5;
        locals.var_dn0vb2e1_dn6 = assign6330_e6502_d_n6;
        locals.var_dn0vb2e1_dn7 = assign6330_e6502_d_n7;
        locals.var_dn0vb2e1_dn8 = assign6330_e6502_d_n8;
        locals.var_dn0vb2e1_dn9 = assign6330_e6502_d_n9;
        locals.var_dn0vb2e1_dn10 = assign6330_e6502_d_n10;
        locals.var_dn0vb2e1_rv = 0.0;

        let (assign6340_e6512, assign6340_e6512_d_n0, assign6340_e6512_d_n1, assign6340_e6512_d_n3, assign6340_e6512_d_n4, assign6340_e6512_d_n5, assign6340_e6512_d_n6, assign6340_e6512_d_n7, assign6340_e6512_d_n8, assign6340_e6512_d_n9, assign6340_e6512_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6340_e6506: f64 = (0.5 * locals.var_qb0);
        let assign6340_e6508: f64 = (assign6340_e6506 * locals.var_q1q);
        let assign6340_e6510: f64 = (assign6340_e6508 * locals.var_dn0vb2e1);
        (assign6340_e6510, (((assign6340_e6506 * locals.var_q1q_dn0) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn0)), (((assign6340_e6506 * locals.var_q1q_dn1) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn1)), (((((0.5 * locals.var_qb0_dn3) * locals.var_q1q) + (assign6340_e6506 * locals.var_q1q_dn3)) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn3)), (((assign6340_e6506 * locals.var_q1q_dn4) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn4)), (((assign6340_e6506 * locals.var_q1q_dn5) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn5)), (((assign6340_e6506 * locals.var_q1q_dn6) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn6)), (((assign6340_e6506 * locals.var_q1q_dn7) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn7)), (((assign6340_e6506 * locals.var_q1q_dn8) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn8)), (((assign6340_e6506 * locals.var_q1q_dn9) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn9)), (((assign6340_e6506 * locals.var_q1q_dn10) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn10)),)
    } else {
        (locals.var_dqbevb2e1, locals.var_dqbevb2e1_dn0, locals.var_dqbevb2e1_dn1, locals.var_dqbevb2e1_dn3, locals.var_dqbevb2e1_dn4, locals.var_dqbevb2e1_dn5, locals.var_dqbevb2e1_dn6, locals.var_dqbevb2e1_dn7, locals.var_dqbevb2e1_dn8, locals.var_dqbevb2e1_dn9, locals.var_dqbevb2e1_dn10,)
    }
};
        locals.var_dqbevb2e1 = assign6340_e6512;
        locals.var_dqbevb2e1_dn0 = assign6340_e6512_d_n0;
        locals.var_dqbevb2e1_dn1 = assign6340_e6512_d_n1;
        locals.var_dqbevb2e1_dn3 = assign6340_e6512_d_n3;
        locals.var_dqbevb2e1_dn4 = assign6340_e6512_d_n4;
        locals.var_dqbevb2e1_dn5 = assign6340_e6512_d_n5;
        locals.var_dqbevb2e1_dn6 = assign6340_e6512_d_n6;
        locals.var_dqbevb2e1_dn7 = assign6340_e6512_d_n7;
        locals.var_dqbevb2e1_dn8 = assign6340_e6512_d_n8;
        locals.var_dqbevb2e1_dn9 = assign6340_e6512_d_n9;
        locals.var_dqbevb2e1_dn10 = assign6340_e6512_d_n10;
        locals.var_dqbevb2e1_rv = 0.0;

        let (assign6350_e6520, assign6350_e6520_d_n0, assign6350_e6520_d_n1, assign6350_e6520_d_n3, assign6350_e6520_d_n4, assign6350_e6520_d_n5, assign6350_e6520_d_n6, assign6350_e6520_d_n7, assign6350_e6520_d_n8, assign6350_e6520_d_n9, assign6350_e6520_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6350_e6517: f64 = (p.p84 * locals.var_vt);
        let assign6350_e6518: f64 = (locals.var_qe_qs / assign6350_e6517);
        (assign6350_e6518, (locals.var_qe_qs_dn0 / assign6350_e6517), (locals.var_qe_qs_dn1 / assign6350_e6517), (((locals.var_qe_qs_dn3 * assign6350_e6517) - (locals.var_qe_qs * (p.p84 * locals.var_vt_dn3))) / (assign6350_e6517 * assign6350_e6517)), (locals.var_qe_qs_dn4 / assign6350_e6517), (locals.var_qe_qs_dn5 / assign6350_e6517), (locals.var_qe_qs_dn6 / assign6350_e6517), (locals.var_qe_qs_dn7 / assign6350_e6517), (locals.var_qe_qs_dn8 / assign6350_e6517), (locals.var_qe_qs_dn9 / assign6350_e6517), (locals.var_qe_qs_dn10 / assign6350_e6517),)
    } else {
        (locals.var_dqevb2e1, locals.var_dqevb2e1_dn0, locals.var_dqevb2e1_dn1, locals.var_dqevb2e1_dn3, locals.var_dqevb2e1_dn4, locals.var_dqevb2e1_dn5, locals.var_dqevb2e1_dn6, locals.var_dqevb2e1_dn7, locals.var_dqevb2e1_dn8, locals.var_dqevb2e1_dn9, locals.var_dqevb2e1_dn10,)
    }
};
        locals.var_dqevb2e1 = assign6350_e6520;
        locals.var_dqevb2e1_dn0 = assign6350_e6520_d_n0;
        locals.var_dqevb2e1_dn1 = assign6350_e6520_d_n1;
        locals.var_dqevb2e1_dn3 = assign6350_e6520_d_n3;
        locals.var_dqevb2e1_dn4 = assign6350_e6520_d_n4;
        locals.var_dqevb2e1_dn5 = assign6350_e6520_d_n5;
        locals.var_dqevb2e1_dn6 = assign6350_e6520_d_n6;
        locals.var_dqevb2e1_dn7 = assign6350_e6520_d_n7;
        locals.var_dqevb2e1_dn8 = assign6350_e6520_d_n8;
        locals.var_dqevb2e1_dn9 = assign6350_e6520_d_n9;
        locals.var_dqevb2e1_dn10 = assign6350_e6520_d_n10;
        locals.var_dqevb2e1_rv = 0.0;

        let (assign6360_e6532, assign6360_e6532_d_n0, assign6360_e6532_d_n1, assign6360_e6532_d_n3, assign6360_e6532_d_n4, assign6360_e6532_d_n5, assign6360_e6532_d_n6, assign6360_e6532_d_n7, assign6360_e6532_d_n8, assign6360_e6532_d_n9, assign6360_e6532_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6360_e6524: f64 = (0.2 * locals.var_vb1b2);
        let assign6360_e6527: f64 = (locals.var_dqtevb2e1 + locals.var_dqbevb2e1);
        let assign6360_e6529: f64 = (assign6360_e6527 + locals.var_dqevb2e1);
        let assign6360_e6530: f64 = (assign6360_e6524 * assign6360_e6529);
        (assign6360_e6530, (assign6360_e6524 * ((locals.var_dqtevb2e1_dn0 + locals.var_dqbevb2e1_dn0) + locals.var_dqevb2e1_dn0)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn1 + locals.var_dqbevb2e1_dn1) + locals.var_dqevb2e1_dn1)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn3 + locals.var_dqbevb2e1_dn3) + locals.var_dqevb2e1_dn3)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn4 + locals.var_dqbevb2e1_dn4) + locals.var_dqevb2e1_dn4)), (((0.2 * locals.var_vb1b2_dn5) * assign6360_e6529) + (assign6360_e6524 * ((locals.var_dqtevb2e1_dn5 + locals.var_dqbevb2e1_dn5) + locals.var_dqevb2e1_dn5))), (((0.2 * locals.var_vb1b2_dn6) * assign6360_e6529) + (assign6360_e6524 * ((locals.var_dqtevb2e1_dn6 + locals.var_dqbevb2e1_dn6) + locals.var_dqevb2e1_dn6))), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn7 + locals.var_dqbevb2e1_dn7) + locals.var_dqevb2e1_dn7)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn8 + locals.var_dqbevb2e1_dn8) + locals.var_dqevb2e1_dn8)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn9 + locals.var_dqbevb2e1_dn9) + locals.var_dqevb2e1_dn9)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn10 + locals.var_dqbevb2e1_dn10) + locals.var_dqevb2e1_dn10)),)
    } else {
        (locals.var_qb1b2, locals.var_qb1b2_dn0, locals.var_qb1b2_dn1, locals.var_qb1b2_dn3, locals.var_qb1b2_dn4, locals.var_qb1b2_dn5, locals.var_qb1b2_dn6, locals.var_qb1b2_dn7, locals.var_qb1b2_dn8, locals.var_qb1b2_dn9, locals.var_qb1b2_dn10,)
    }
};
        locals.var_qb1b2 = assign6360_e6532;
        locals.var_qb1b2_dn0 = assign6360_e6532_d_n0;
        locals.var_qb1b2_dn1 = assign6360_e6532_d_n1;
        locals.var_qb1b2_dn3 = assign6360_e6532_d_n3;
        locals.var_qb1b2_dn4 = assign6360_e6532_d_n4;
        locals.var_qb1b2_dn5 = assign6360_e6532_d_n5;
        locals.var_qb1b2_dn6 = assign6360_e6532_d_n6;
        locals.var_qb1b2_dn7 = assign6360_e6532_d_n7;
        locals.var_qb1b2_dn8 = assign6360_e6532_d_n8;
        locals.var_qb1b2_dn9 = assign6360_e6532_d_n9;
        locals.var_qb1b2_dn10 = assign6360_e6532_d_n10;
        locals.var_qb1b2_rv = 0.0;

        let (assign6370_e6540, assign6370_e6540_d_n0, assign6370_e6540_d_n1, assign6370_e6540_d_n3, assign6370_e6540_d_n4, assign6370_e6540_d_n5, assign6370_e6540_d_n6, assign6370_e6540_d_n7, assign6370_e6540_d_n8, assign6370_e6540_d_n9, assign6370_e6540_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6370_e6536: f64 = (1.0 - p.p94);
        let assign6370_e6538: f64 = (assign6370_e6536 * locals.var_qe_qs);
        (assign6370_e6538, (assign6370_e6536 * locals.var_qe_qs_dn0), (assign6370_e6536 * locals.var_qe_qs_dn1), (assign6370_e6536 * locals.var_qe_qs_dn3), (assign6370_e6536 * locals.var_qe_qs_dn4), (assign6370_e6536 * locals.var_qe_qs_dn5), (assign6370_e6536 * locals.var_qe_qs_dn6), (assign6370_e6536 * locals.var_qe_qs_dn7), (assign6370_e6536 * locals.var_qe_qs_dn8), (assign6370_e6536 * locals.var_qe_qs_dn9), (assign6370_e6536 * locals.var_qe_qs_dn10),)
    } else {
        (locals.var_qe, locals.var_qe_dn0, locals.var_qe_dn1, locals.var_qe_dn3, locals.var_qe_dn4, locals.var_qe_dn5, locals.var_qe_dn6, locals.var_qe_dn7, locals.var_qe_dn8, locals.var_qe_dn9, locals.var_qe_dn10,)
    }
};
        locals.var_qe = assign6370_e6540;
        locals.var_qe_dn0 = assign6370_e6540_d_n0;
        locals.var_qe_dn1 = assign6370_e6540_d_n1;
        locals.var_qe_dn3 = assign6370_e6540_d_n3;
        locals.var_qe_dn4 = assign6370_e6540_d_n4;
        locals.var_qe_dn5 = assign6370_e6540_d_n5;
        locals.var_qe_dn6 = assign6370_e6540_d_n6;
        locals.var_qe_dn7 = assign6370_e6540_d_n7;
        locals.var_qe_dn8 = assign6370_e6540_d_n8;
        locals.var_qe_dn9 = assign6370_e6540_d_n9;
        locals.var_qe_dn10 = assign6370_e6540_d_n10;
        locals.var_qe_rv = 0.0;

        let (assign6380_e6548, assign6380_e6548_d_n0, assign6380_e6548_d_n1, assign6380_e6548_d_n3, assign6380_e6548_d_n4, assign6380_e6548_d_n5, assign6380_e6548_d_n6, assign6380_e6548_d_n7, assign6380_e6548_d_n8, assign6380_e6548_d_n9, assign6380_e6548_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6380_e6545: f64 = (p.p94 * locals.var_qe_qs);
        let assign6380_e6546: f64 = (locals.var_qbe_qs + assign6380_e6545);
        (assign6380_e6546, (locals.var_qbe_qs_dn0 + (p.p94 * locals.var_qe_qs_dn0)), (locals.var_qbe_qs_dn1 + (p.p94 * locals.var_qe_qs_dn1)), (locals.var_qbe_qs_dn3 + (p.p94 * locals.var_qe_qs_dn3)), (locals.var_qbe_qs_dn4 + (p.p94 * locals.var_qe_qs_dn4)), (locals.var_qbe_qs_dn5 + (p.p94 * locals.var_qe_qs_dn5)), (locals.var_qbe_qs_dn6 + (p.p94 * locals.var_qe_qs_dn6)), (locals.var_qbe_qs_dn7 + (p.p94 * locals.var_qe_qs_dn7)), (locals.var_qbe_qs_dn8 + (p.p94 * locals.var_qe_qs_dn8)), (locals.var_qbe_qs_dn9 + (p.p94 * locals.var_qe_qs_dn9)), (locals.var_qbe_qs_dn10 + (p.p94 * locals.var_qe_qs_dn10)),)
    } else {
        (locals.var_qbe_qs_eff, locals.var_qbe_qs_eff_dn0, locals.var_qbe_qs_eff_dn1, locals.var_qbe_qs_eff_dn3, locals.var_qbe_qs_eff_dn4, locals.var_qbe_qs_eff_dn5, locals.var_qbe_qs_eff_dn6, locals.var_qbe_qs_eff_dn7, locals.var_qbe_qs_eff_dn8, locals.var_qbe_qs_eff_dn9, locals.var_qbe_qs_eff_dn10,)
    }
};
        locals.var_qbe_qs_eff = assign6380_e6548;
        locals.var_qbe_qs_eff_dn0 = assign6380_e6548_d_n0;
        locals.var_qbe_qs_eff_dn1 = assign6380_e6548_d_n1;
        locals.var_qbe_qs_eff_dn3 = assign6380_e6548_d_n3;
        locals.var_qbe_qs_eff_dn4 = assign6380_e6548_d_n4;
        locals.var_qbe_qs_eff_dn5 = assign6380_e6548_d_n5;
        locals.var_qbe_qs_eff_dn6 = assign6380_e6548_d_n6;
        locals.var_qbe_qs_eff_dn7 = assign6380_e6548_d_n7;
        locals.var_qbe_qs_eff_dn8 = assign6380_e6548_d_n8;
        locals.var_qbe_qs_eff_dn9 = assign6380_e6548_d_n9;
        locals.var_qbe_qs_eff_dn10 = assign6380_e6548_d_n10;
        locals.var_qbe_qs_eff_rv = 0.0;

        let (assign6390_e6556, assign6390_e6556_d_n0, assign6390_e6556_d_n1, assign6390_e6556_d_n3, assign6390_e6556_d_n4, assign6390_e6556_d_n5, assign6390_e6556_d_n6, assign6390_e6556_d_n7, assign6390_e6556_d_n8, assign6390_e6556_d_n9, assign6390_e6556_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6390_e6552: f64 = (p.p93 * locals.var_qbe_qs_eff);
        let assign6390_e6554: f64 = (assign6390_e6552 + locals.var_qbc_qs);
        (assign6390_e6554, ((p.p93 * locals.var_qbe_qs_eff_dn0) + locals.var_qbc_qs_dn0), ((p.p93 * locals.var_qbe_qs_eff_dn1) + locals.var_qbc_qs_dn1), ((p.p93 * locals.var_qbe_qs_eff_dn3) + locals.var_qbc_qs_dn3), ((p.p93 * locals.var_qbe_qs_eff_dn4) + locals.var_qbc_qs_dn4), ((p.p93 * locals.var_qbe_qs_eff_dn5) + locals.var_qbc_qs_dn5), ((p.p93 * locals.var_qbe_qs_eff_dn6) + locals.var_qbc_qs_dn6), ((p.p93 * locals.var_qbe_qs_eff_dn7) + locals.var_qbc_qs_dn7), ((p.p93 * locals.var_qbe_qs_eff_dn8) + locals.var_qbc_qs_dn8), ((p.p93 * locals.var_qbe_qs_eff_dn9) + locals.var_qbc_qs_dn9), ((p.p93 * locals.var_qbe_qs_eff_dn10) + locals.var_qbc_qs_dn10),)
    } else {
        (locals.var_qbc, locals.var_qbc_dn0, locals.var_qbc_dn1, locals.var_qbc_dn3, locals.var_qbc_dn4, locals.var_qbc_dn5, locals.var_qbc_dn6, locals.var_qbc_dn7, locals.var_qbc_dn8, locals.var_qbc_dn9, locals.var_qbc_dn10,)
    }
};
        locals.var_qbc = assign6390_e6556;
        locals.var_qbc_dn0 = assign6390_e6556_d_n0;
        locals.var_qbc_dn1 = assign6390_e6556_d_n1;
        locals.var_qbc_dn3 = assign6390_e6556_d_n3;
        locals.var_qbc_dn4 = assign6390_e6556_d_n4;
        locals.var_qbc_dn5 = assign6390_e6556_d_n5;
        locals.var_qbc_dn6 = assign6390_e6556_d_n6;
        locals.var_qbc_dn7 = assign6390_e6556_d_n7;
        locals.var_qbc_dn8 = assign6390_e6556_d_n8;
        locals.var_qbc_dn9 = assign6390_e6556_d_n9;
        locals.var_qbc_dn10 = assign6390_e6556_d_n10;
        locals.var_qbc_rv = 0.0;

        let (assign6400_e6564, assign6400_e6564_d_n0, assign6400_e6564_d_n1, assign6400_e6564_d_n3, assign6400_e6564_d_n4, assign6400_e6564_d_n5, assign6400_e6564_d_n6, assign6400_e6564_d_n7, assign6400_e6564_d_n8, assign6400_e6564_d_n9, assign6400_e6564_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6400_e6560: f64 = (1.0 - p.p93);
        let assign6400_e6562: f64 = (assign6400_e6560 * locals.var_qbe_qs_eff);
        (assign6400_e6562, (assign6400_e6560 * locals.var_qbe_qs_eff_dn0), (assign6400_e6560 * locals.var_qbe_qs_eff_dn1), (assign6400_e6560 * locals.var_qbe_qs_eff_dn3), (assign6400_e6560 * locals.var_qbe_qs_eff_dn4), (assign6400_e6560 * locals.var_qbe_qs_eff_dn5), (assign6400_e6560 * locals.var_qbe_qs_eff_dn6), (assign6400_e6560 * locals.var_qbe_qs_eff_dn7), (assign6400_e6560 * locals.var_qbe_qs_eff_dn8), (assign6400_e6560 * locals.var_qbe_qs_eff_dn9), (assign6400_e6560 * locals.var_qbe_qs_eff_dn10),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn1, locals.var_qbe_dn3, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn8, locals.var_qbe_dn9, locals.var_qbe_dn10,)
    }
};
        locals.var_qbe = assign6400_e6564;
        locals.var_qbe_dn0 = assign6400_e6564_d_n0;
        locals.var_qbe_dn1 = assign6400_e6564_d_n1;
        locals.var_qbe_dn3 = assign6400_e6564_d_n3;
        locals.var_qbe_dn4 = assign6400_e6564_d_n4;
        locals.var_qbe_dn5 = assign6400_e6564_d_n5;
        locals.var_qbe_dn6 = assign6400_e6564_d_n6;
        locals.var_qbe_dn7 = assign6400_e6564_d_n7;
        locals.var_qbe_dn8 = assign6400_e6564_d_n8;
        locals.var_qbe_dn9 = assign6400_e6564_d_n9;
        locals.var_qbe_dn10 = assign6400_e6564_d_n10;
        locals.var_qbe_rv = 0.0;

        let (assign6410_e6569, assign6410_e6569_d_n0, assign6410_e6569_d_n1, assign6410_e6569_d_n3, assign6410_e6569_d_n4, assign6410_e6569_d_n5, assign6410_e6569_d_n6, assign6410_e6569_d_n7, assign6410_e6569_d_n8, assign6410_e6569_d_n9, assign6410_e6569_d_n10,) = {
    if (locals.var_guard115 == 0.0) {
        (locals.var_qbe_qs, locals.var_qbe_qs_dn0, locals.var_qbe_qs_dn1, locals.var_qbe_qs_dn3, locals.var_qbe_qs_dn4, locals.var_qbe_qs_dn5, locals.var_qbe_qs_dn6, locals.var_qbe_qs_dn7, locals.var_qbe_qs_dn8, locals.var_qbe_qs_dn9, locals.var_qbe_qs_dn10,)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn1, locals.var_qbe_dn3, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn8, locals.var_qbe_dn9, locals.var_qbe_dn10,)
    }
};
        locals.var_qbe = assign6410_e6569;
        locals.var_qbe_dn0 = assign6410_e6569_d_n0;
        locals.var_qbe_dn1 = assign6410_e6569_d_n1;
        locals.var_qbe_dn3 = assign6410_e6569_d_n3;
        locals.var_qbe_dn4 = assign6410_e6569_d_n4;
        locals.var_qbe_dn5 = assign6410_e6569_d_n5;
        locals.var_qbe_dn6 = assign6410_e6569_d_n6;
        locals.var_qbe_dn7 = assign6410_e6569_d_n7;
        locals.var_qbe_dn8 = assign6410_e6569_d_n8;
        locals.var_qbe_dn9 = assign6410_e6569_d_n9;
        locals.var_qbe_dn10 = assign6410_e6569_d_n10;
        locals.var_qbe_rv = 0.0;

        let (assign6420_e6574, assign6420_e6574_d_n0, assign6420_e6574_d_n1, assign6420_e6574_d_n3, assign6420_e6574_d_n4, assign6420_e6574_d_n5, assign6420_e6574_d_n6, assign6420_e6574_d_n7, assign6420_e6574_d_n8, assign6420_e6574_d_n9, assign6420_e6574_d_n10,) = {
    if (locals.var_guard115 == 0.0) {
        (locals.var_qbc_qs, locals.var_qbc_qs_dn0, locals.var_qbc_qs_dn1, locals.var_qbc_qs_dn3, locals.var_qbc_qs_dn4, locals.var_qbc_qs_dn5, locals.var_qbc_qs_dn6, locals.var_qbc_qs_dn7, locals.var_qbc_qs_dn8, locals.var_qbc_qs_dn9, locals.var_qbc_qs_dn10,)
    } else {
        (locals.var_qbc, locals.var_qbc_dn0, locals.var_qbc_dn1, locals.var_qbc_dn3, locals.var_qbc_dn4, locals.var_qbc_dn5, locals.var_qbc_dn6, locals.var_qbc_dn7, locals.var_qbc_dn8, locals.var_qbc_dn9, locals.var_qbc_dn10,)
    }
};
        locals.var_qbc = assign6420_e6574;
        locals.var_qbc_dn0 = assign6420_e6574_d_n0;
        locals.var_qbc_dn1 = assign6420_e6574_d_n1;
        locals.var_qbc_dn3 = assign6420_e6574_d_n3;
        locals.var_qbc_dn4 = assign6420_e6574_d_n4;
        locals.var_qbc_dn5 = assign6420_e6574_d_n5;
        locals.var_qbc_dn6 = assign6420_e6574_d_n6;
        locals.var_qbc_dn7 = assign6420_e6574_d_n7;
        locals.var_qbc_dn8 = assign6420_e6574_d_n8;
        locals.var_qbc_dn9 = assign6420_e6574_d_n9;
        locals.var_qbc_dn10 = assign6420_e6574_d_n10;
        locals.var_qbc_rv = 0.0;

        let (assign6430_e6579, assign6430_e6579_d_n0, assign6430_e6579_d_n1, assign6430_e6579_d_n3, assign6430_e6579_d_n4, assign6430_e6579_d_n5, assign6430_e6579_d_n6, assign6430_e6579_d_n7, assign6430_e6579_d_n8, assign6430_e6579_d_n9, assign6430_e6579_d_n10,) = {
    if (locals.var_guard115 == 0.0) {
        (locals.var_qe_qs, locals.var_qe_qs_dn0, locals.var_qe_qs_dn1, locals.var_qe_qs_dn3, locals.var_qe_qs_dn4, locals.var_qe_qs_dn5, locals.var_qe_qs_dn6, locals.var_qe_qs_dn7, locals.var_qe_qs_dn8, locals.var_qe_qs_dn9, locals.var_qe_qs_dn10,)
    } else {
        (locals.var_qe, locals.var_qe_dn0, locals.var_qe_dn1, locals.var_qe_dn3, locals.var_qe_dn4, locals.var_qe_dn5, locals.var_qe_dn6, locals.var_qe_dn7, locals.var_qe_dn8, locals.var_qe_dn9, locals.var_qe_dn10,)
    }
};
        locals.var_qe = assign6430_e6579;
        locals.var_qe_dn0 = assign6430_e6579_d_n0;
        locals.var_qe_dn1 = assign6430_e6579_d_n1;
        locals.var_qe_dn3 = assign6430_e6579_d_n3;
        locals.var_qe_dn4 = assign6430_e6579_d_n4;
        locals.var_qe_dn5 = assign6430_e6579_d_n5;
        locals.var_qe_dn6 = assign6430_e6579_d_n6;
        locals.var_qe_dn7 = assign6430_e6579_d_n7;
        locals.var_qe_dn8 = assign6430_e6579_d_n8;
        locals.var_qe_dn9 = assign6430_e6579_d_n9;
        locals.var_qe_dn10 = assign6430_e6579_d_n10;
        locals.var_qe_rv = 0.0;

        let assign6450_e6585: f64 = (p.p134 * (nv3 - 0.0));
        let assign6450_e6586_q: f64 = assign6450_e6585;
        let assign6450_e6588: f64 = (assign6450_e6585 * p.p1);
        let assign6450_e6588_q: f64 = (assign6450_e6586_q * p.p1);
        locals.var_i_cth = assign6450_e6588;
        locals.var_i_cth_dn3 = (p.p134 * p.p1);
        locals.var_i_cth_rv = assign6450_e6588_q;
        locals.var_i_cth_rdn3 = (p.p134 * p.p1);

        let assign6630_e6704: f64 = (locals.var_if_ + locals.var_ir);
        let assign6630_e6706: f64 = (assign6630_e6704 / locals.var_qbi);
        locals.var_in_n = assign6630_e6706;
        locals.var_in_n_dn0 = ((((locals.var_if__dn0 + locals.var_ir_dn0) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn0)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn1 = ((((locals.var_if__dn1 + locals.var_ir_dn1) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn1)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn3 = ((((locals.var_if__dn3 + locals.var_ir_dn3) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn3)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn4 = ((((locals.var_if__dn4 + locals.var_ir_dn4) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn4)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn5 = ((((locals.var_if__dn5 + locals.var_ir_dn5) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn5)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn6 = ((((locals.var_if__dn6 + locals.var_ir_dn6) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn6)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn7 = ((((locals.var_if__dn7 + locals.var_ir_dn7) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn7)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn8 = ((((locals.var_if__dn8 + locals.var_ir_dn8) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn8)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn9 = ((((locals.var_if__dn9 + locals.var_ir_dn9) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn9)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn10 = ((((locals.var_if__dn10 + locals.var_ir_dn10) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn10)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_rv = 0.0;

        let assign6690_e6739: f64 = if locals.var_in_n > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign6690_e6739;
        locals.var_guard124_rv = 0.0;

        let (assign6700_e6747, assign6700_e6747_d_n0, assign6700_e6747_d_n1, assign6700_e6747_d_n3, assign6700_e6747_d_n4, assign6700_e6747_d_n5, assign6700_e6747_d_n6, assign6700_e6747_d_n7, assign6700_e6747_d_n8, assign6700_e6747_d_n9, assign6700_e6747_d_n10,) = {
    if (locals.var_guard124 != 0.0) {
        let assign6700_e6743: f64 = (locals.var_qbe + locals.var_qbc);
        let assign6700_e6745: f64 = (assign6700_e6743 / locals.var_in_n);
        (assign6700_e6745, ((((locals.var_qbe_dn0 + locals.var_qbc_dn0) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn0)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn1 + locals.var_qbc_dn1) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn1)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn3 + locals.var_qbc_dn3) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn3)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn4 + locals.var_qbc_dn4) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn4)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn5 + locals.var_qbc_dn5) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn5)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn6 + locals.var_qbc_dn6) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn6)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn7 + locals.var_qbc_dn7) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn7)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn8 + locals.var_qbc_dn8) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn8)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn9 + locals.var_qbc_dn9) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn9)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn10 + locals.var_qbc_dn10) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn10)) / (locals.var_in_n * locals.var_in_n)),)
    } else {
        (locals.var_taub_n, locals.var_taub_n_dn0, locals.var_taub_n_dn1, locals.var_taub_n_dn3, locals.var_taub_n_dn4, locals.var_taub_n_dn5, locals.var_taub_n_dn6, locals.var_taub_n_dn7, locals.var_taub_n_dn8, locals.var_taub_n_dn9, locals.var_taub_n_dn10,)
    }
};
        locals.var_taub_n = assign6700_e6747;
        locals.var_taub_n_dn0 = assign6700_e6747_d_n0;
        locals.var_taub_n_dn1 = assign6700_e6747_d_n1;
        locals.var_taub_n_dn3 = assign6700_e6747_d_n3;
        locals.var_taub_n_dn4 = assign6700_e6747_d_n4;
        locals.var_taub_n_dn5 = assign6700_e6747_d_n5;
        locals.var_taub_n_dn6 = assign6700_e6747_d_n6;
        locals.var_taub_n_dn7 = assign6700_e6747_d_n7;
        locals.var_taub_n_dn8 = assign6700_e6747_d_n8;
        locals.var_taub_n_dn9 = assign6700_e6747_d_n9;
        locals.var_taub_n_dn10 = assign6700_e6747_d_n10;
        locals.var_taub_n_rv = 0.0;

        let (assign6710_e6756, assign6710_e6756_d_n0, assign6710_e6756_d_n1, assign6710_e6756_d_n3, assign6710_e6756_d_n4, assign6710_e6756_d_n5, assign6710_e6756_d_n6, assign6710_e6756_d_n7, assign6710_e6756_d_n8, assign6710_e6756_d_n9, assign6710_e6756_d_n10,) = {
    if (locals.var_guard124 == 0.0) {
        let assign6710_e6752: f64 = (locals.var_taub_t * locals.var_q1q);
        let assign6710_e6754: f64 = (assign6710_e6752 * locals.var_qbi);
        (assign6710_e6754, (((locals.var_taub_t * locals.var_q1q_dn0) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn0)), (((locals.var_taub_t * locals.var_q1q_dn1) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn1)), ((((locals.var_taub_t_dn3 * locals.var_q1q) + (locals.var_taub_t * locals.var_q1q_dn3)) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn3)), (((locals.var_taub_t * locals.var_q1q_dn4) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn4)), (((locals.var_taub_t * locals.var_q1q_dn5) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn5)), (((locals.var_taub_t * locals.var_q1q_dn6) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn6)), (((locals.var_taub_t * locals.var_q1q_dn7) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn7)), (((locals.var_taub_t * locals.var_q1q_dn8) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn8)), (((locals.var_taub_t * locals.var_q1q_dn9) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn9)), (((locals.var_taub_t * locals.var_q1q_dn10) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn10)),)
    } else {
        (locals.var_taub_n, locals.var_taub_n_dn0, locals.var_taub_n_dn1, locals.var_taub_n_dn3, locals.var_taub_n_dn4, locals.var_taub_n_dn5, locals.var_taub_n_dn6, locals.var_taub_n_dn7, locals.var_taub_n_dn8, locals.var_taub_n_dn9, locals.var_taub_n_dn10,)
    }
};
        locals.var_taub_n = assign6710_e6756;
        locals.var_taub_n_dn0 = assign6710_e6756_d_n0;
        locals.var_taub_n_dn1 = assign6710_e6756_d_n1;
        locals.var_taub_n_dn3 = assign6710_e6756_d_n3;
        locals.var_taub_n_dn4 = assign6710_e6756_d_n4;
        locals.var_taub_n_dn5 = assign6710_e6756_d_n5;
        locals.var_taub_n_dn6 = assign6710_e6756_d_n6;
        locals.var_taub_n_dn7 = assign6710_e6756_d_n7;
        locals.var_taub_n_dn8 = assign6710_e6756_d_n8;
        locals.var_taub_n_dn9 = assign6710_e6756_d_n9;
        locals.var_taub_n_dn10 = assign6710_e6756_d_n10;
        locals.var_taub_n_rv = 0.0;

        let assign6720_e6759: f64 = if p.p130 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign6720_e6759;
        locals.var_guard125_rv = 0.0;

        let (assign6730_e6765, assign6730_e6765_d_n0, assign6730_e6765_d_n1, assign6730_e6765_d_n3, assign6730_e6765_d_n4, assign6730_e6765_d_n5, assign6730_e6765_d_n6, assign6730_e6765_d_n7, assign6730_e6765_d_n8, assign6730_e6765_d_n9, assign6730_e6765_d_n10,) = {
    if (locals.var_guard125 != 0.0) {
        let assign6730_e6763: f64 = (p.p93 * locals.var_taub_n);
        (assign6730_e6763, (p.p93 * locals.var_taub_n_dn0), (p.p93 * locals.var_taub_n_dn1), (p.p93 * locals.var_taub_n_dn3), (p.p93 * locals.var_taub_n_dn4), (p.p93 * locals.var_taub_n_dn5), (p.p93 * locals.var_taub_n_dn6), (p.p93 * locals.var_taub_n_dn7), (p.p93 * locals.var_taub_n_dn8), (p.p93 * locals.var_taub_n_dn9), (p.p93 * locals.var_taub_n_dn10),)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10,)
    }
};
        locals.var_taun = assign6730_e6765;
        locals.var_taun_dn0 = assign6730_e6765_d_n0;
        locals.var_taun_dn1 = assign6730_e6765_d_n1;
        locals.var_taun_dn3 = assign6730_e6765_d_n3;
        locals.var_taun_dn4 = assign6730_e6765_d_n4;
        locals.var_taun_dn5 = assign6730_e6765_d_n5;
        locals.var_taun_dn6 = assign6730_e6765_d_n6;
        locals.var_taun_dn7 = assign6730_e6765_d_n7;
        locals.var_taun_dn8 = assign6730_e6765_d_n8;
        locals.var_taun_dn9 = assign6730_e6765_d_n9;
        locals.var_taun_dn10 = assign6730_e6765_d_n10;
        locals.var_taun_rv = 0.0;

        let assign6740_e6768: f64 = if p.p130 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign6740_e6768;
        locals.var_guard126_rv = 0.0;

        let (assign6750_e6777, assign6750_e6777_d_n0, assign6750_e6777_d_n1, assign6750_e6777_d_n3, assign6750_e6777_d_n4, assign6750_e6777_d_n5, assign6750_e6777_d_n6, assign6750_e6777_d_n7, assign6750_e6777_d_n8, assign6750_e6777_d_n9, assign6750_e6777_d_n10,) = {
    if ((locals.var_guard125 == 0.0) && (locals.var_guard126 != 0.0)) {
        let assign6750_e6775: f64 = (p.p131 * locals.var_taub_n);
        (assign6750_e6775, (p.p131 * locals.var_taub_n_dn0), (p.p131 * locals.var_taub_n_dn1), (p.p131 * locals.var_taub_n_dn3), (p.p131 * locals.var_taub_n_dn4), (p.p131 * locals.var_taub_n_dn5), (p.p131 * locals.var_taub_n_dn6), (p.p131 * locals.var_taub_n_dn7), (p.p131 * locals.var_taub_n_dn8), (p.p131 * locals.var_taub_n_dn9), (p.p131 * locals.var_taub_n_dn10),)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10,)
    }
};
        locals.var_taun = assign6750_e6777;
        locals.var_taun_dn0 = assign6750_e6777_d_n0;
        locals.var_taun_dn1 = assign6750_e6777_d_n1;
        locals.var_taun_dn3 = assign6750_e6777_d_n3;
        locals.var_taun_dn4 = assign6750_e6777_d_n4;
        locals.var_taun_dn5 = assign6750_e6777_d_n5;
        locals.var_taun_dn6 = assign6750_e6777_d_n6;
        locals.var_taun_dn7 = assign6750_e6777_d_n7;
        locals.var_taun_dn8 = assign6750_e6777_d_n8;
        locals.var_taun_dn9 = assign6750_e6777_d_n9;
        locals.var_taun_dn10 = assign6750_e6777_d_n10;
        locals.var_taun_rv = 0.0;

        let (assign6760_e6785, assign6760_e6785_d_n0, assign6760_e6785_d_n1, assign6760_e6785_d_n3, assign6760_e6785_d_n4, assign6760_e6785_d_n5, assign6760_e6785_d_n6, assign6760_e6785_d_n7, assign6760_e6785_d_n8, assign6760_e6785_d_n9, assign6760_e6785_d_n10,) = {
    if ((locals.var_guard125 == 0.0) && (locals.var_guard126 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10,)
    }
};
        locals.var_taun = assign6760_e6785;
        locals.var_taun_dn0 = assign6760_e6785_d_n0;
        locals.var_taun_dn1 = assign6760_e6785_d_n1;
        locals.var_taun_dn3 = assign6760_e6785_d_n3;
        locals.var_taun_dn4 = assign6760_e6785_d_n4;
        locals.var_taun_dn5 = assign6760_e6785_d_n5;
        locals.var_taun_dn6 = assign6760_e6785_d_n6;
        locals.var_taun_dn7 = assign6760_e6785_d_n7;
        locals.var_taun_dn8 = assign6760_e6785_d_n8;
        locals.var_taun_dn9 = assign6760_e6785_d_n9;
        locals.var_taun_dn10 = assign6760_e6785_d_n10;
        locals.var_taun_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq11_value: f64 = locals.var_i_cth;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq11_value),
            3,
            multiplicity * (locals.var_i_cth_dn3),
        );
        let eq13_e245: f64 = (locals.var_qte + locals.var_qbe);
        let eq13_e245_d_n0: f64 = (locals.var_qte_dn0 + locals.var_qbe_dn0);
        let eq13_e245_d_n1: f64 = (locals.var_qte_dn1 + locals.var_qbe_dn1);
        let eq13_e245_d_n3: f64 = (locals.var_qte_dn3 + locals.var_qbe_dn3);
        let eq13_e245_d_n4: f64 = (locals.var_qte_dn4 + locals.var_qbe_dn4);
        let eq13_e245_d_n5: f64 = (locals.var_qte_dn5 + locals.var_qbe_dn5);
        let eq13_e245_d_n6: f64 = (locals.var_qte_dn6 + locals.var_qbe_dn6);
        let eq13_e245_d_n7: f64 = (locals.var_qte_dn7 + locals.var_qbe_dn7);
        let eq13_e245_d_n8: f64 = (locals.var_qte_dn8 + locals.var_qbe_dn8);
        let eq13_e245_d_n9: f64 = (locals.var_qte_dn9 + locals.var_qbe_dn9);
        let eq13_e245_d_n10: f64 = (locals.var_qte_dn10 + locals.var_qbe_dn10);
        let eq13_e247: f64 = (eq13_e245 + locals.var_qe);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + locals.var_qe_dn0);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + locals.var_qe_dn1);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + locals.var_qe_dn3);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + locals.var_qe_dn4);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + locals.var_qe_dn5);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + locals.var_qe_dn6);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + locals.var_qe_dn7);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + locals.var_qe_dn8);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + locals.var_qe_dn9);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + locals.var_qe_dn10);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e249: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq13_e248);
        let eq13_e251: f64 = (eq13_e249 * p.p1);
        let eq13_e251_d_n0: f64 = ((eq13_e248_d_n0 * ddt_scale) * p.p1);
        let eq13_e251_d_n1: f64 = ((eq13_e248_d_n1 * ddt_scale) * p.p1);
        let eq13_e251_d_n3: f64 = ((eq13_e248_d_n3 * ddt_scale) * p.p1);
        let eq13_e251_d_n4: f64 = ((eq13_e248_d_n4 * ddt_scale) * p.p1);
        let eq13_e251_d_n5: f64 = ((eq13_e248_d_n5 * ddt_scale) * p.p1);
        let eq13_e251_d_n6: f64 = ((eq13_e248_d_n6 * ddt_scale) * p.p1);
        let eq13_e251_d_n7: f64 = ((eq13_e248_d_n7 * ddt_scale) * p.p1);
        let eq13_e251_d_n8: f64 = ((eq13_e248_d_n8 * ddt_scale) * p.p1);
        let eq13_e251_d_n9: f64 = ((eq13_e248_d_n9 * ddt_scale) * p.p1);
        let eq13_e251_d_n10: f64 = ((eq13_e248_d_n10 * ddt_scale) * p.p1);
        let eq13_value: f64 = eq13_e251;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(4),
            multiplicity * (eq13_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq13_e251_d_n0), multiplicity * (eq13_e251_d_n1), multiplicity * (eq13_e251_d_n3), multiplicity * (eq13_e251_d_n4), multiplicity * (eq13_e251_d_n5), multiplicity * (eq13_e251_d_n6), multiplicity * (eq13_e251_d_n7), multiplicity * (eq13_e251_d_n8), multiplicity * (eq13_e251_d_n9), multiplicity * (eq13_e251_d_n10)],
            [],
            [],
            1.0,
        );
        let eq14_e254: f64 = (p.p3 * locals.var_qte_s);
        let eq14_e254_d_n0: f64 = (p.p3 * locals.var_qte_s_dn0);
        let eq14_e254_d_n1: f64 = (p.p3 * locals.var_qte_s_dn1);
        let eq14_e254_d_n3: f64 = (p.p3 * locals.var_qte_s_dn3);
        let eq14_e254_d_n4: f64 = (p.p3 * locals.var_qte_s_dn4);
        let eq14_e254_d_n5: f64 = (p.p3 * locals.var_qte_s_dn5);
        let eq14_e254_d_n6: f64 = (p.p3 * locals.var_qte_s_dn6);
        let eq14_e254_d_n7: f64 = (p.p3 * locals.var_qte_s_dn7);
        let eq14_e254_d_n8: f64 = (p.p3 * locals.var_qte_s_dn8);
        let eq14_e254_d_n9: f64 = (p.p3 * locals.var_qte_s_dn9);
        let eq14_e254_d_n10: f64 = (p.p3 * locals.var_qte_s_dn10);
        let eq14_e255: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq14_e254);
        let eq14_e257: f64 = (eq14_e255 * p.p1);
        let eq14_e257_d_n0: f64 = ((eq14_e254_d_n0 * ddt_scale) * p.p1);
        let eq14_e257_d_n1: f64 = ((eq14_e254_d_n1 * ddt_scale) * p.p1);
        let eq14_e257_d_n3: f64 = ((eq14_e254_d_n3 * ddt_scale) * p.p1);
        let eq14_e257_d_n4: f64 = ((eq14_e254_d_n4 * ddt_scale) * p.p1);
        let eq14_e257_d_n5: f64 = ((eq14_e254_d_n5 * ddt_scale) * p.p1);
        let eq14_e257_d_n6: f64 = ((eq14_e254_d_n6 * ddt_scale) * p.p1);
        let eq14_e257_d_n7: f64 = ((eq14_e254_d_n7 * ddt_scale) * p.p1);
        let eq14_e257_d_n8: f64 = ((eq14_e254_d_n8 * ddt_scale) * p.p1);
        let eq14_e257_d_n9: f64 = ((eq14_e254_d_n9 * ddt_scale) * p.p1);
        let eq14_e257_d_n10: f64 = ((eq14_e254_d_n10 * ddt_scale) * p.p1);
        let eq14_value: f64 = eq14_e257;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq14_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq14_e257_d_n0), multiplicity * (eq14_e257_d_n1), multiplicity * (eq14_e257_d_n3), multiplicity * (eq14_e257_d_n4), multiplicity * (eq14_e257_d_n5), multiplicity * (eq14_e257_d_n6), multiplicity * (eq14_e257_d_n7), multiplicity * (eq14_e257_d_n8), multiplicity * (eq14_e257_d_n9), multiplicity * (eq14_e257_d_n10)],
            [],
            [],
            1.0,
        );
        let eq15_e261: f64 = (locals.var_qtc + locals.var_qbc);
        let eq15_e261_d_n0: f64 = (locals.var_qtc_dn0 + locals.var_qbc_dn0);
        let eq15_e261_d_n1: f64 = (locals.var_qtc_dn1 + locals.var_qbc_dn1);
        let eq15_e261_d_n3: f64 = (locals.var_qtc_dn3 + locals.var_qbc_dn3);
        let eq15_e261_d_n4: f64 = (locals.var_qtc_dn4 + locals.var_qbc_dn4);
        let eq15_e261_d_n5: f64 = (locals.var_qtc_dn5 + locals.var_qbc_dn5);
        let eq15_e261_d_n6: f64 = (locals.var_qtc_dn6 + locals.var_qbc_dn6);
        let eq15_e261_d_n7: f64 = (locals.var_qtc_dn7 + locals.var_qbc_dn7);
        let eq15_e261_d_n8: f64 = (locals.var_qtc_dn8 + locals.var_qbc_dn8);
        let eq15_e261_d_n9: f64 = (locals.var_qtc_dn9 + locals.var_qbc_dn9);
        let eq15_e261_d_n10: f64 = (locals.var_qtc_dn10 + locals.var_qbc_dn10);
        let eq15_e263: f64 = (eq15_e261 + locals.var_qepi);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + locals.var_qepi_dn0);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + locals.var_qepi_dn1);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + locals.var_qepi_dn3);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + locals.var_qepi_dn4);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + locals.var_qepi_dn5);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + locals.var_qepi_dn6);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + locals.var_qepi_dn7);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + locals.var_qepi_dn8);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + locals.var_qepi_dn9);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + locals.var_qepi_dn10);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e265: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq15_e264);
        let eq15_e267: f64 = (eq15_e265 * p.p1);
        let eq15_e267_d_n0: f64 = ((eq15_e264_d_n0 * ddt_scale) * p.p1);
        let eq15_e267_d_n1: f64 = ((eq15_e264_d_n1 * ddt_scale) * p.p1);
        let eq15_e267_d_n3: f64 = ((eq15_e264_d_n3 * ddt_scale) * p.p1);
        let eq15_e267_d_n4: f64 = ((eq15_e264_d_n4 * ddt_scale) * p.p1);
        let eq15_e267_d_n5: f64 = ((eq15_e264_d_n5 * ddt_scale) * p.p1);
        let eq15_e267_d_n6: f64 = ((eq15_e264_d_n6 * ddt_scale) * p.p1);
        let eq15_e267_d_n7: f64 = ((eq15_e264_d_n7 * ddt_scale) * p.p1);
        let eq15_e267_d_n8: f64 = ((eq15_e264_d_n8 * ddt_scale) * p.p1);
        let eq15_e267_d_n9: f64 = ((eq15_e264_d_n9 * ddt_scale) * p.p1);
        let eq15_e267_d_n10: f64 = ((eq15_e264_d_n10 * ddt_scale) * p.p1);
        let eq15_value: f64 = eq15_e267;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq15_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq15_e267_d_n0), multiplicity * (eq15_e267_d_n1), multiplicity * (eq15_e267_d_n3), multiplicity * (eq15_e267_d_n4), multiplicity * (eq15_e267_d_n5), multiplicity * (eq15_e267_d_n6), multiplicity * (eq15_e267_d_n7), multiplicity * (eq15_e267_d_n8), multiplicity * (eq15_e267_d_n9), multiplicity * (eq15_e267_d_n10)],
            [],
            [],
            1.0,
        );
        let eq16_e270: f64 = (p.p3 * locals.var_qb1b2);
        let eq16_e270_d_n0: f64 = (p.p3 * locals.var_qb1b2_dn0);
        let eq16_e270_d_n1: f64 = (p.p3 * locals.var_qb1b2_dn1);
        let eq16_e270_d_n3: f64 = (p.p3 * locals.var_qb1b2_dn3);
        let eq16_e270_d_n4: f64 = (p.p3 * locals.var_qb1b2_dn4);
        let eq16_e270_d_n5: f64 = (p.p3 * locals.var_qb1b2_dn5);
        let eq16_e270_d_n6: f64 = (p.p3 * locals.var_qb1b2_dn6);
        let eq16_e270_d_n7: f64 = (p.p3 * locals.var_qb1b2_dn7);
        let eq16_e270_d_n8: f64 = (p.p3 * locals.var_qb1b2_dn8);
        let eq16_e270_d_n9: f64 = (p.p3 * locals.var_qb1b2_dn9);
        let eq16_e270_d_n10: f64 = (p.p3 * locals.var_qb1b2_dn10);
        let eq16_e271: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq16_e270);
        let eq16_e273: f64 = (eq16_e271 * p.p1);
        let eq16_e273_d_n0: f64 = ((eq16_e270_d_n0 * ddt_scale) * p.p1);
        let eq16_e273_d_n1: f64 = ((eq16_e270_d_n1 * ddt_scale) * p.p1);
        let eq16_e273_d_n3: f64 = ((eq16_e270_d_n3 * ddt_scale) * p.p1);
        let eq16_e273_d_n4: f64 = ((eq16_e270_d_n4 * ddt_scale) * p.p1);
        let eq16_e273_d_n5: f64 = ((eq16_e270_d_n5 * ddt_scale) * p.p1);
        let eq16_e273_d_n6: f64 = ((eq16_e270_d_n6 * ddt_scale) * p.p1);
        let eq16_e273_d_n7: f64 = ((eq16_e270_d_n7 * ddt_scale) * p.p1);
        let eq16_e273_d_n8: f64 = ((eq16_e270_d_n8 * ddt_scale) * p.p1);
        let eq16_e273_d_n9: f64 = ((eq16_e270_d_n9 * ddt_scale) * p.p1);
        let eq16_e273_d_n10: f64 = ((eq16_e270_d_n10 * ddt_scale) * p.p1);
        let eq16_value: f64 = eq16_e273;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq16_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq16_e273_d_n0), multiplicity * (eq16_e273_d_n1), multiplicity * (eq16_e273_d_n3), multiplicity * (eq16_e273_d_n4), multiplicity * (eq16_e273_d_n5), multiplicity * (eq16_e273_d_n6), multiplicity * (eq16_e273_d_n7), multiplicity * (eq16_e273_d_n8), multiplicity * (eq16_e273_d_n9), multiplicity * (eq16_e273_d_n10)],
            [],
            [],
            1.0,
        );
        let eq17_e276: f64 = (p.p3 * p.p68);
        let eq17_e278: f64 = (eq17_e276 * locals.var_vbe);
        let eq17_e278_d_n1: f64 = (eq17_e276 * locals.var_vbe_dn1);
        let eq17_e278_d_n2: f64 = (eq17_e276 * locals.var_vbe_dn2);
        let eq17_e279: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq17_e278);
        let eq17_e281: f64 = (eq17_e279 * p.p1);
        let eq17_e281_d_n1: f64 = ((eq17_e278_d_n1 * ddt_scale) * p.p1);
        let eq17_e281_d_n2: f64 = ((eq17_e278_d_n2 * ddt_scale) * p.p1);
        let eq17_value: f64 = eq17_e281;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (eq17_value),
            1,
            multiplicity * (eq17_e281_d_n1),
            2,
            multiplicity * (eq17_e281_d_n2),
        );
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * locals.var_vbc);
        let eq18_e286_d_n0: f64 = (eq18_e284 * locals.var_vbc_dn0);
        let eq18_e286_d_n1: f64 = (eq18_e284 * locals.var_vbc_dn1);
        let eq18_e287: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq18_e286);
        let eq18_e289: f64 = (eq18_e287 * p.p1);
        let eq18_e289_d_n0: f64 = ((eq18_e286_d_n0 * ddt_scale) * p.p1);
        let eq18_e289_d_n1: f64 = ((eq18_e286_d_n1 * ddt_scale) * p.p1);
        let eq18_value: f64 = eq18_e289;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (eq18_value),
            0,
            multiplicity * (eq18_e289_d_n0),
            1,
            multiplicity * (eq18_e289_d_n1),
        );
        let eq21_e305: f64 = (locals.var_xqtex + locals.var_xqex);
        let eq21_e305_d_n0: f64 = (locals.var_xqtex_dn0 + locals.var_xqex_dn0);
        let eq21_e305_d_n1: f64 = (locals.var_xqtex_dn1 + locals.var_xqex_dn1);
        let eq21_e305_d_n3: f64 = (locals.var_xqtex_dn3 + locals.var_xqex_dn3);
        let eq21_e305_d_n4: f64 = (locals.var_xqtex_dn4 + locals.var_xqex_dn4);
        let eq21_e305_d_n5: f64 = (locals.var_xqtex_dn5 + locals.var_xqex_dn5);
        let eq21_e305_d_n6: f64 = (locals.var_xqtex_dn6 + locals.var_xqex_dn6);
        let eq21_e305_d_n7: f64 = (locals.var_xqtex_dn7 + locals.var_xqex_dn7);
        let eq21_e305_d_n8: f64 = (locals.var_xqtex_dn8 + locals.var_xqex_dn8);
        let eq21_e305_d_n9: f64 = (locals.var_xqtex_dn9 + locals.var_xqex_dn9);
        let eq21_e305_d_n10: f64 = (locals.var_xqtex_dn10 + locals.var_xqex_dn10);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq21_e306);
        let eq21_e309: f64 = (eq21_e307 * p.p1);
        let eq21_e309_d_n0: f64 = ((eq21_e306_d_n0 * ddt_scale) * p.p1);
        let eq21_e309_d_n1: f64 = ((eq21_e306_d_n1 * ddt_scale) * p.p1);
        let eq21_e309_d_n3: f64 = ((eq21_e306_d_n3 * ddt_scale) * p.p1);
        let eq21_e309_d_n4: f64 = ((eq21_e306_d_n4 * ddt_scale) * p.p1);
        let eq21_e309_d_n5: f64 = ((eq21_e306_d_n5 * ddt_scale) * p.p1);
        let eq21_e309_d_n6: f64 = ((eq21_e306_d_n6 * ddt_scale) * p.p1);
        let eq21_e309_d_n7: f64 = ((eq21_e306_d_n7 * ddt_scale) * p.p1);
        let eq21_e309_d_n8: f64 = ((eq21_e306_d_n8 * ddt_scale) * p.p1);
        let eq21_e309_d_n9: f64 = ((eq21_e306_d_n9 * ddt_scale) * p.p1);
        let eq21_e309_d_n10: f64 = ((eq21_e306_d_n10 * ddt_scale) * p.p1);
        let eq21_value: f64 = eq21_e309;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (eq21_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq21_e309_d_n0), multiplicity * (eq21_e309_d_n1), multiplicity * (eq21_e309_d_n3), multiplicity * (eq21_e309_d_n4), multiplicity * (eq21_e309_d_n5), multiplicity * (eq21_e309_d_n6), multiplicity * (eq21_e309_d_n7), multiplicity * (eq21_e309_d_n8), multiplicity * (eq21_e309_d_n9), multiplicity * (eq21_e309_d_n10)],
            [],
            [],
            1.0,
        );
        let eq23_e324: f64 = (locals.var_qtex + locals.var_qex);
        let eq23_e324_d_n0: f64 = (locals.var_qtex_dn0 + locals.var_qex_dn0);
        let eq23_e324_d_n1: f64 = (locals.var_qtex_dn1 + locals.var_qex_dn1);
        let eq23_e324_d_n3: f64 = (locals.var_qtex_dn3 + locals.var_qex_dn3);
        let eq23_e324_d_n4: f64 = (locals.var_qtex_dn4 + locals.var_qex_dn4);
        let eq23_e324_d_n5: f64 = (locals.var_qtex_dn5 + locals.var_qex_dn5);
        let eq23_e324_d_n6: f64 = (locals.var_qtex_dn6 + locals.var_qex_dn6);
        let eq23_e324_d_n7: f64 = (locals.var_qtex_dn7 + locals.var_qex_dn7);
        let eq23_e324_d_n8: f64 = (locals.var_qtex_dn8 + locals.var_qex_dn8);
        let eq23_e324_d_n9: f64 = (locals.var_qtex_dn9 + locals.var_qex_dn9);
        let eq23_e324_d_n10: f64 = (locals.var_qtex_dn10 + locals.var_qex_dn10);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq23_e325);
        let eq23_e328: f64 = (eq23_e326 * p.p1);
        let eq23_e328_d_n0: f64 = ((eq23_e325_d_n0 * ddt_scale) * p.p1);
        let eq23_e328_d_n1: f64 = ((eq23_e325_d_n1 * ddt_scale) * p.p1);
        let eq23_e328_d_n3: f64 = ((eq23_e325_d_n3 * ddt_scale) * p.p1);
        let eq23_e328_d_n4: f64 = ((eq23_e325_d_n4 * ddt_scale) * p.p1);
        let eq23_e328_d_n5: f64 = ((eq23_e325_d_n5 * ddt_scale) * p.p1);
        let eq23_e328_d_n6: f64 = ((eq23_e325_d_n6 * ddt_scale) * p.p1);
        let eq23_e328_d_n7: f64 = ((eq23_e325_d_n7 * ddt_scale) * p.p1);
        let eq23_e328_d_n8: f64 = ((eq23_e325_d_n8 * ddt_scale) * p.p1);
        let eq23_e328_d_n9: f64 = ((eq23_e325_d_n9 * ddt_scale) * p.p1);
        let eq23_e328_d_n10: f64 = ((eq23_e325_d_n10 * ddt_scale) * p.p1);
        let eq23_value: f64 = eq23_e328;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(10),
            multiplicity * (eq23_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq23_e328_d_n0), multiplicity * (eq23_e328_d_n1), multiplicity * (eq23_e328_d_n3), multiplicity * (eq23_e328_d_n4), multiplicity * (eq23_e328_d_n5), multiplicity * (eq23_e328_d_n6), multiplicity * (eq23_e328_d_n7), multiplicity * (eq23_e328_d_n8), multiplicity * (eq23_e328_d_n9), multiplicity * (eq23_e328_d_n10)],
            [],
            [],
            1.0,
        );
        let eq30_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, (nv11 - 0.0));
        let eq30_e368: f64 = (locals.var_taun * eq30_e367);
        let eq30_e368_d_n0: f64 = (locals.var_taun_dn0 * eq30_e367);
        let eq30_e368_d_n1: f64 = (locals.var_taun_dn1 * eq30_e367);
        let eq30_e368_d_n3: f64 = (locals.var_taun_dn3 * eq30_e367);
        let eq30_e368_d_n4: f64 = (locals.var_taun_dn4 * eq30_e367);
        let eq30_e368_d_n5: f64 = (locals.var_taun_dn5 * eq30_e367);
        let eq30_e368_d_n6: f64 = (locals.var_taun_dn6 * eq30_e367);
        let eq30_e368_d_n7: f64 = (locals.var_taun_dn7 * eq30_e367);
        let eq30_e368_d_n8: f64 = (locals.var_taun_dn8 * eq30_e367);
        let eq30_e368_d_n9: f64 = (locals.var_taun_dn9 * eq30_e367);
        let eq30_e368_d_n10: f64 = (locals.var_taun_dn10 * eq30_e367);
        let eq30_value: f64 = eq30_e368;
        let eq30_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq30_node_derivatives: [f64; 11] = [eq30_e368_d_n0, eq30_e368_d_n1, eq30_e368_d_n3, eq30_e368_d_n4, eq30_e368_d_n5, eq30_e368_d_n6, eq30_e368_d_n7, eq30_e368_d_n8, eq30_e368_d_n9, eq30_e368_d_n10, (locals.var_taun * ddt_scale)];
        let eq30_branch_derivative_indices: [usize; 0] = [];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq30_value),
            &eq30_node_derivative_indices,
            &eq30_node_derivatives,
            &eq30_branch_derivative_indices,
            &eq30_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq11_e235_q: f64 = locals.var_i_cth_rv;
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (locals.var_i_cth_rdn3),
        );
        let eq13_e245: f64 = (locals.var_qte + locals.var_qbe);
        let eq13_e245_d_n0: f64 = (locals.var_qte_dn0 + locals.var_qbe_dn0);
        let eq13_e245_d_n1: f64 = (locals.var_qte_dn1 + locals.var_qbe_dn1);
        let eq13_e245_d_n3: f64 = (locals.var_qte_dn3 + locals.var_qbe_dn3);
        let eq13_e245_d_n4: f64 = (locals.var_qte_dn4 + locals.var_qbe_dn4);
        let eq13_e245_d_n5: f64 = (locals.var_qte_dn5 + locals.var_qbe_dn5);
        let eq13_e245_d_n6: f64 = (locals.var_qte_dn6 + locals.var_qbe_dn6);
        let eq13_e245_d_n7: f64 = (locals.var_qte_dn7 + locals.var_qbe_dn7);
        let eq13_e245_d_n8: f64 = (locals.var_qte_dn8 + locals.var_qbe_dn8);
        let eq13_e245_d_n9: f64 = (locals.var_qte_dn9 + locals.var_qbe_dn9);
        let eq13_e245_d_n10: f64 = (locals.var_qte_dn10 + locals.var_qbe_dn10);
        let eq13_e247: f64 = (eq13_e245 + locals.var_qe);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + locals.var_qe_dn0);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + locals.var_qe_dn1);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + locals.var_qe_dn3);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + locals.var_qe_dn4);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + locals.var_qe_dn5);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + locals.var_qe_dn6);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + locals.var_qe_dn7);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + locals.var_qe_dn8);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + locals.var_qe_dn9);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + locals.var_qe_dn10);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e249_q: f64 = eq13_e248;
        let eq13_e251: f64 = (eq13_e248 * p.p1);
        let eq13_e251_d_n0: f64 = (eq13_e248_d_n0 * p.p1);
        let eq13_e251_d_n1: f64 = (eq13_e248_d_n1 * p.p1);
        let eq13_e251_d_n3: f64 = (eq13_e248_d_n3 * p.p1);
        let eq13_e251_d_n4: f64 = (eq13_e248_d_n4 * p.p1);
        let eq13_e251_d_n5: f64 = (eq13_e248_d_n5 * p.p1);
        let eq13_e251_d_n6: f64 = (eq13_e248_d_n6 * p.p1);
        let eq13_e251_d_n7: f64 = (eq13_e248_d_n7 * p.p1);
        let eq13_e251_d_n8: f64 = (eq13_e248_d_n8 * p.p1);
        let eq13_e251_d_n9: f64 = (eq13_e248_d_n9 * p.p1);
        let eq13_e251_d_n10: f64 = (eq13_e248_d_n10 * p.p1);
        let eq13_e251_q: f64 = (eq13_e249_q * p.p1);
        let eq13_reactive_node_derivatives: [f64; 12] = [eq13_e251_d_n0, eq13_e251_d_n1, 0.0, eq13_e251_d_n3, eq13_e251_d_n4, eq13_e251_d_n5, eq13_e251_d_n6, eq13_e251_d_n7, eq13_e251_d_n8, eq13_e251_d_n9, eq13_e251_d_n10, 0.0];
        let eq13_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e254: f64 = (p.p3 * locals.var_qte_s);
        let eq14_e254_d_n0: f64 = (p.p3 * locals.var_qte_s_dn0);
        let eq14_e254_d_n1: f64 = (p.p3 * locals.var_qte_s_dn1);
        let eq14_e254_d_n3: f64 = (p.p3 * locals.var_qte_s_dn3);
        let eq14_e254_d_n4: f64 = (p.p3 * locals.var_qte_s_dn4);
        let eq14_e254_d_n5: f64 = (p.p3 * locals.var_qte_s_dn5);
        let eq14_e254_d_n6: f64 = (p.p3 * locals.var_qte_s_dn6);
        let eq14_e254_d_n7: f64 = (p.p3 * locals.var_qte_s_dn7);
        let eq14_e254_d_n8: f64 = (p.p3 * locals.var_qte_s_dn8);
        let eq14_e254_d_n9: f64 = (p.p3 * locals.var_qte_s_dn9);
        let eq14_e254_d_n10: f64 = (p.p3 * locals.var_qte_s_dn10);
        let eq14_e255_q: f64 = eq14_e254;
        let eq14_e257: f64 = (eq14_e254 * p.p1);
        let eq14_e257_d_n0: f64 = (eq14_e254_d_n0 * p.p1);
        let eq14_e257_d_n1: f64 = (eq14_e254_d_n1 * p.p1);
        let eq14_e257_d_n3: f64 = (eq14_e254_d_n3 * p.p1);
        let eq14_e257_d_n4: f64 = (eq14_e254_d_n4 * p.p1);
        let eq14_e257_d_n5: f64 = (eq14_e254_d_n5 * p.p1);
        let eq14_e257_d_n6: f64 = (eq14_e254_d_n6 * p.p1);
        let eq14_e257_d_n7: f64 = (eq14_e254_d_n7 * p.p1);
        let eq14_e257_d_n8: f64 = (eq14_e254_d_n8 * p.p1);
        let eq14_e257_d_n9: f64 = (eq14_e254_d_n9 * p.p1);
        let eq14_e257_d_n10: f64 = (eq14_e254_d_n10 * p.p1);
        let eq14_e257_q: f64 = (eq14_e255_q * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e257_d_n0, eq14_e257_d_n1, 0.0, eq14_e257_d_n3, eq14_e257_d_n4, eq14_e257_d_n5, eq14_e257_d_n6, eq14_e257_d_n7, eq14_e257_d_n8, eq14_e257_d_n9, eq14_e257_d_n10, 0.0];
        let eq14_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e261: f64 = (locals.var_qtc + locals.var_qbc);
        let eq15_e261_d_n0: f64 = (locals.var_qtc_dn0 + locals.var_qbc_dn0);
        let eq15_e261_d_n1: f64 = (locals.var_qtc_dn1 + locals.var_qbc_dn1);
        let eq15_e261_d_n3: f64 = (locals.var_qtc_dn3 + locals.var_qbc_dn3);
        let eq15_e261_d_n4: f64 = (locals.var_qtc_dn4 + locals.var_qbc_dn4);
        let eq15_e261_d_n5: f64 = (locals.var_qtc_dn5 + locals.var_qbc_dn5);
        let eq15_e261_d_n6: f64 = (locals.var_qtc_dn6 + locals.var_qbc_dn6);
        let eq15_e261_d_n7: f64 = (locals.var_qtc_dn7 + locals.var_qbc_dn7);
        let eq15_e261_d_n8: f64 = (locals.var_qtc_dn8 + locals.var_qbc_dn8);
        let eq15_e261_d_n9: f64 = (locals.var_qtc_dn9 + locals.var_qbc_dn9);
        let eq15_e261_d_n10: f64 = (locals.var_qtc_dn10 + locals.var_qbc_dn10);
        let eq15_e263: f64 = (eq15_e261 + locals.var_qepi);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + locals.var_qepi_dn0);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + locals.var_qepi_dn1);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + locals.var_qepi_dn3);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + locals.var_qepi_dn4);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + locals.var_qepi_dn5);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + locals.var_qepi_dn6);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + locals.var_qepi_dn7);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + locals.var_qepi_dn8);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + locals.var_qepi_dn9);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + locals.var_qepi_dn10);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e265_q: f64 = eq15_e264;
        let eq15_e267: f64 = (eq15_e264 * p.p1);
        let eq15_e267_d_n0: f64 = (eq15_e264_d_n0 * p.p1);
        let eq15_e267_d_n1: f64 = (eq15_e264_d_n1 * p.p1);
        let eq15_e267_d_n3: f64 = (eq15_e264_d_n3 * p.p1);
        let eq15_e267_d_n4: f64 = (eq15_e264_d_n4 * p.p1);
        let eq15_e267_d_n5: f64 = (eq15_e264_d_n5 * p.p1);
        let eq15_e267_d_n6: f64 = (eq15_e264_d_n6 * p.p1);
        let eq15_e267_d_n7: f64 = (eq15_e264_d_n7 * p.p1);
        let eq15_e267_d_n8: f64 = (eq15_e264_d_n8 * p.p1);
        let eq15_e267_d_n9: f64 = (eq15_e264_d_n9 * p.p1);
        let eq15_e267_d_n10: f64 = (eq15_e264_d_n10 * p.p1);
        let eq15_e267_q: f64 = (eq15_e265_q * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e267_d_n0, eq15_e267_d_n1, 0.0, eq15_e267_d_n3, eq15_e267_d_n4, eq15_e267_d_n5, eq15_e267_d_n6, eq15_e267_d_n7, eq15_e267_d_n8, eq15_e267_d_n9, eq15_e267_d_n10, 0.0];
        let eq15_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e270: f64 = (p.p3 * locals.var_qb1b2);
        let eq16_e270_d_n0: f64 = (p.p3 * locals.var_qb1b2_dn0);
        let eq16_e270_d_n1: f64 = (p.p3 * locals.var_qb1b2_dn1);
        let eq16_e270_d_n3: f64 = (p.p3 * locals.var_qb1b2_dn3);
        let eq16_e270_d_n4: f64 = (p.p3 * locals.var_qb1b2_dn4);
        let eq16_e270_d_n5: f64 = (p.p3 * locals.var_qb1b2_dn5);
        let eq16_e270_d_n6: f64 = (p.p3 * locals.var_qb1b2_dn6);
        let eq16_e270_d_n7: f64 = (p.p3 * locals.var_qb1b2_dn7);
        let eq16_e270_d_n8: f64 = (p.p3 * locals.var_qb1b2_dn8);
        let eq16_e270_d_n9: f64 = (p.p3 * locals.var_qb1b2_dn9);
        let eq16_e270_d_n10: f64 = (p.p3 * locals.var_qb1b2_dn10);
        let eq16_e271_q: f64 = eq16_e270;
        let eq16_e273: f64 = (eq16_e270 * p.p1);
        let eq16_e273_d_n0: f64 = (eq16_e270_d_n0 * p.p1);
        let eq16_e273_d_n1: f64 = (eq16_e270_d_n1 * p.p1);
        let eq16_e273_d_n3: f64 = (eq16_e270_d_n3 * p.p1);
        let eq16_e273_d_n4: f64 = (eq16_e270_d_n4 * p.p1);
        let eq16_e273_d_n5: f64 = (eq16_e270_d_n5 * p.p1);
        let eq16_e273_d_n6: f64 = (eq16_e270_d_n6 * p.p1);
        let eq16_e273_d_n7: f64 = (eq16_e270_d_n7 * p.p1);
        let eq16_e273_d_n8: f64 = (eq16_e270_d_n8 * p.p1);
        let eq16_e273_d_n9: f64 = (eq16_e270_d_n9 * p.p1);
        let eq16_e273_d_n10: f64 = (eq16_e270_d_n10 * p.p1);
        let eq16_e273_q: f64 = (eq16_e271_q * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e273_d_n0, eq16_e273_d_n1, 0.0, eq16_e273_d_n3, eq16_e273_d_n4, eq16_e273_d_n5, eq16_e273_d_n6, eq16_e273_d_n7, eq16_e273_d_n8, eq16_e273_d_n9, eq16_e273_d_n10, 0.0];
        let eq16_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e276: f64 = (p.p3 * p.p68);
        let eq17_e278: f64 = (eq17_e276 * locals.var_vbe);
        let eq17_e278_d_n1: f64 = (eq17_e276 * locals.var_vbe_dn1);
        let eq17_e278_d_n2: f64 = (eq17_e276 * locals.var_vbe_dn2);
        let eq17_e279_q: f64 = eq17_e278;
        let eq17_e281: f64 = (eq17_e278 * p.p1);
        let eq17_e281_d_n1: f64 = (eq17_e278_d_n1 * p.p1);
        let eq17_e281_d_n2: f64 = (eq17_e278_d_n2 * p.p1);
        let eq17_e281_q: f64 = (eq17_e279_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq17_e281_d_n1),
            nodes[2],
            multiplicity * (eq17_e281_d_n2),
        );
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * locals.var_vbc);
        let eq18_e286_d_n0: f64 = (eq18_e284 * locals.var_vbc_dn0);
        let eq18_e286_d_n1: f64 = (eq18_e284 * locals.var_vbc_dn1);
        let eq18_e287_q: f64 = eq18_e286;
        let eq18_e289: f64 = (eq18_e286 * p.p1);
        let eq18_e289_d_n0: f64 = (eq18_e286_d_n0 * p.p1);
        let eq18_e289_d_n1: f64 = (eq18_e286_d_n1 * p.p1);
        let eq18_e289_q: f64 = (eq18_e287_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq18_e289_d_n0),
            nodes[1],
            multiplicity * (eq18_e289_d_n1),
        );
        let eq21_e305: f64 = (locals.var_xqtex + locals.var_xqex);
        let eq21_e305_d_n0: f64 = (locals.var_xqtex_dn0 + locals.var_xqex_dn0);
        let eq21_e305_d_n1: f64 = (locals.var_xqtex_dn1 + locals.var_xqex_dn1);
        let eq21_e305_d_n3: f64 = (locals.var_xqtex_dn3 + locals.var_xqex_dn3);
        let eq21_e305_d_n4: f64 = (locals.var_xqtex_dn4 + locals.var_xqex_dn4);
        let eq21_e305_d_n5: f64 = (locals.var_xqtex_dn5 + locals.var_xqex_dn5);
        let eq21_e305_d_n6: f64 = (locals.var_xqtex_dn6 + locals.var_xqex_dn6);
        let eq21_e305_d_n7: f64 = (locals.var_xqtex_dn7 + locals.var_xqex_dn7);
        let eq21_e305_d_n8: f64 = (locals.var_xqtex_dn8 + locals.var_xqex_dn8);
        let eq21_e305_d_n9: f64 = (locals.var_xqtex_dn9 + locals.var_xqex_dn9);
        let eq21_e305_d_n10: f64 = (locals.var_xqtex_dn10 + locals.var_xqex_dn10);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e307_q: f64 = eq21_e306;
        let eq21_e309: f64 = (eq21_e306 * p.p1);
        let eq21_e309_d_n0: f64 = (eq21_e306_d_n0 * p.p1);
        let eq21_e309_d_n1: f64 = (eq21_e306_d_n1 * p.p1);
        let eq21_e309_d_n3: f64 = (eq21_e306_d_n3 * p.p1);
        let eq21_e309_d_n4: f64 = (eq21_e306_d_n4 * p.p1);
        let eq21_e309_d_n5: f64 = (eq21_e306_d_n5 * p.p1);
        let eq21_e309_d_n6: f64 = (eq21_e306_d_n6 * p.p1);
        let eq21_e309_d_n7: f64 = (eq21_e306_d_n7 * p.p1);
        let eq21_e309_d_n8: f64 = (eq21_e306_d_n8 * p.p1);
        let eq21_e309_d_n9: f64 = (eq21_e306_d_n9 * p.p1);
        let eq21_e309_d_n10: f64 = (eq21_e306_d_n10 * p.p1);
        let eq21_e309_q: f64 = (eq21_e307_q * p.p1);
        let eq21_reactive_node_derivatives: [f64; 12] = [eq21_e309_d_n0, eq21_e309_d_n1, 0.0, eq21_e309_d_n3, eq21_e309_d_n4, eq21_e309_d_n5, eq21_e309_d_n6, eq21_e309_d_n7, eq21_e309_d_n8, eq21_e309_d_n9, eq21_e309_d_n10, 0.0];
        let eq21_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e324: f64 = (locals.var_qtex + locals.var_qex);
        let eq23_e324_d_n0: f64 = (locals.var_qtex_dn0 + locals.var_qex_dn0);
        let eq23_e324_d_n1: f64 = (locals.var_qtex_dn1 + locals.var_qex_dn1);
        let eq23_e324_d_n3: f64 = (locals.var_qtex_dn3 + locals.var_qex_dn3);
        let eq23_e324_d_n4: f64 = (locals.var_qtex_dn4 + locals.var_qex_dn4);
        let eq23_e324_d_n5: f64 = (locals.var_qtex_dn5 + locals.var_qex_dn5);
        let eq23_e324_d_n6: f64 = (locals.var_qtex_dn6 + locals.var_qex_dn6);
        let eq23_e324_d_n7: f64 = (locals.var_qtex_dn7 + locals.var_qex_dn7);
        let eq23_e324_d_n8: f64 = (locals.var_qtex_dn8 + locals.var_qex_dn8);
        let eq23_e324_d_n9: f64 = (locals.var_qtex_dn9 + locals.var_qex_dn9);
        let eq23_e324_d_n10: f64 = (locals.var_qtex_dn10 + locals.var_qex_dn10);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e326_q: f64 = eq23_e325;
        let eq23_e328: f64 = (eq23_e325 * p.p1);
        let eq23_e328_d_n0: f64 = (eq23_e325_d_n0 * p.p1);
        let eq23_e328_d_n1: f64 = (eq23_e325_d_n1 * p.p1);
        let eq23_e328_d_n3: f64 = (eq23_e325_d_n3 * p.p1);
        let eq23_e328_d_n4: f64 = (eq23_e325_d_n4 * p.p1);
        let eq23_e328_d_n5: f64 = (eq23_e325_d_n5 * p.p1);
        let eq23_e328_d_n6: f64 = (eq23_e325_d_n6 * p.p1);
        let eq23_e328_d_n7: f64 = (eq23_e325_d_n7 * p.p1);
        let eq23_e328_d_n8: f64 = (eq23_e325_d_n8 * p.p1);
        let eq23_e328_d_n9: f64 = (eq23_e325_d_n9 * p.p1);
        let eq23_e328_d_n10: f64 = (eq23_e325_d_n10 * p.p1);
        let eq23_e328_q: f64 = (eq23_e326_q * p.p1);
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e328_d_n0, eq23_e328_d_n1, 0.0, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, 0.0];
        let eq23_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e367_q: f64 = (nv11 - 0.0);
        let eq30_e368: f64 = (locals.var_taun * (nv11 - 0.0));
        let eq30_e368_d_n0: f64 = (locals.var_taun_dn0 * (nv11 - 0.0));
        let eq30_e368_d_n1: f64 = (locals.var_taun_dn1 * (nv11 - 0.0));
        let eq30_e368_d_n3: f64 = (locals.var_taun_dn3 * (nv11 - 0.0));
        let eq30_e368_d_n4: f64 = (locals.var_taun_dn4 * (nv11 - 0.0));
        let eq30_e368_d_n5: f64 = (locals.var_taun_dn5 * (nv11 - 0.0));
        let eq30_e368_d_n6: f64 = (locals.var_taun_dn6 * (nv11 - 0.0));
        let eq30_e368_d_n7: f64 = (locals.var_taun_dn7 * (nv11 - 0.0));
        let eq30_e368_d_n8: f64 = (locals.var_taun_dn8 * (nv11 - 0.0));
        let eq30_e368_d_n9: f64 = (locals.var_taun_dn9 * (nv11 - 0.0));
        let eq30_e368_d_n10: f64 = (locals.var_taun_dn10 * (nv11 - 0.0));
        let eq30_e368_q: f64 = (locals.var_taun * eq30_e367_q);
        let eq30_e368_q_d_n0: f64 = (locals.var_taun_dn0 * eq30_e367_q);
        let eq30_e368_q_d_n1: f64 = (locals.var_taun_dn1 * eq30_e367_q);
        let eq30_e368_q_d_n3: f64 = (locals.var_taun_dn3 * eq30_e367_q);
        let eq30_e368_q_d_n4: f64 = (locals.var_taun_dn4 * eq30_e367_q);
        let eq30_e368_q_d_n5: f64 = (locals.var_taun_dn5 * eq30_e367_q);
        let eq30_e368_q_d_n6: f64 = (locals.var_taun_dn6 * eq30_e367_q);
        let eq30_e368_q_d_n7: f64 = (locals.var_taun_dn7 * eq30_e367_q);
        let eq30_e368_q_d_n8: f64 = (locals.var_taun_dn8 * eq30_e367_q);
        let eq30_e368_q_d_n9: f64 = (locals.var_taun_dn9 * eq30_e367_q);
        let eq30_e368_q_d_n10: f64 = (locals.var_taun_dn10 * eq30_e367_q);
        let eq30_reactive_node_derivatives: [f64; 12] = [eq30_e368_q_d_n0, eq30_e368_q_d_n1, 0.0, eq30_e368_q_d_n3, eq30_e368_q_d_n4, eq30_e368_q_d_n5, eq30_e368_q_d_n6, eq30_e368_q_d_n7, eq30_e368_q_d_n8, eq30_e368_q_d_n9, eq30_e368_q_d_n10, locals.var_taun];
        let eq30_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

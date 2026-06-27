#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let bi0 = ctx.branch_current(branches[0]);
        let bi5 = ctx.branch_current(branches[5]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi13 = ctx.branch_current(branches[13]);
        let bi14 = ctx.branch_current(branches[14]);
        let eq3_e99: f64 = (p.p51 / 3.0);
        let eq3_e101: f64 = (eq3_e99 * bi0);
        let eq3_e102: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq3_e101);
        let eq3_e102_d_b0: f64 = (eq3_e99 * ddt_scale);
        let eq3_value: f64 = eq3_e102;
        stamper.stamp_potential_branch1_local(
            0,
            eq3_value,
            0,
            eq3_e102_d_b0,
        );
        let (eq7_e110, eq7_e110_d_n0, eq7_e110_d_n1, eq7_e110_d_n2, eq7_e110_d_n3, eq7_e110_d_n4, eq7_e110_d_n5, eq7_e110_d_n6, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n9, eq7_e110_d_n10, eq7_e110_d_n11, eq7_e110_d_n12, eq7_e110_d_n13, eq7_e110_d_n14, eq7_e110_d_n15, eq7_e110_d_b0, eq7_e110_d_b1, eq7_e110_d_b2, eq7_e110_d_b3, eq7_e110_d_b4, eq7_e110_d_b5, eq7_e110_d_b6, eq7_e110_d_b7, eq7_e110_d_b8, eq7_e110_d_b9, eq7_e110_d_b10, eq7_e110_d_b11, eq7_e110_d_b12, eq7_e110_d_b13, eq7_e110_d_b14,) = {
    if s.b[97] {
        let eq7_e108: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[21]);
        let eq7_e108_d_n0: f64 = (s.dn[21][0] * ddt_scale);
        let eq7_e108_d_n1: f64 = (s.dn[21][1] * ddt_scale);
        let eq7_e108_d_n2: f64 = (s.dn[21][2] * ddt_scale);
        let eq7_e108_d_n3: f64 = (s.dn[21][3] * ddt_scale);
        let eq7_e108_d_n4: f64 = (s.dn[21][4] * ddt_scale);
        let eq7_e108_d_n5: f64 = (s.dn[21][5] * ddt_scale);
        let eq7_e108_d_n6: f64 = (s.dn[21][6] * ddt_scale);
        let eq7_e108_d_n7: f64 = (s.dn[21][7] * ddt_scale);
        let eq7_e108_d_n8: f64 = (s.dn[21][8] * ddt_scale);
        let eq7_e108_d_n9: f64 = (s.dn[21][9] * ddt_scale);
        let eq7_e108_d_n10: f64 = (s.dn[21][10] * ddt_scale);
        let eq7_e108_d_n11: f64 = (s.dn[21][11] * ddt_scale);
        let eq7_e108_d_n12: f64 = (s.dn[21][12] * ddt_scale);
        let eq7_e108_d_n13: f64 = (s.dn[21][13] * ddt_scale);
        let eq7_e108_d_n14: f64 = (s.dn[21][14] * ddt_scale);
        let eq7_e108_d_n15: f64 = (s.dn[21][15] * ddt_scale);
        let eq7_e108_d_b0: f64 = (s.db[21][0] * ddt_scale);
        let eq7_e108_d_b1: f64 = (s.db[21][1] * ddt_scale);
        let eq7_e108_d_b2: f64 = (s.db[21][2] * ddt_scale);
        let eq7_e108_d_b3: f64 = (s.db[21][3] * ddt_scale);
        let eq7_e108_d_b4: f64 = (s.db[21][4] * ddt_scale);
        let eq7_e108_d_b5: f64 = (s.db[21][5] * ddt_scale);
        let eq7_e108_d_b6: f64 = (s.db[21][6] * ddt_scale);
        let eq7_e108_d_b7: f64 = (s.db[21][7] * ddt_scale);
        let eq7_e108_d_b8: f64 = (s.db[21][8] * ddt_scale);
        let eq7_e108_d_b9: f64 = (s.db[21][9] * ddt_scale);
        let eq7_e108_d_b10: f64 = (s.db[21][10] * ddt_scale);
        let eq7_e108_d_b11: f64 = (s.db[21][11] * ddt_scale);
        let eq7_e108_d_b12: f64 = (s.db[21][12] * ddt_scale);
        let eq7_e108_d_b13: f64 = (s.db[21][13] * ddt_scale);
        let eq7_e108_d_b14: f64 = (s.db[21][14] * ddt_scale);
        (eq7_e108, eq7_e108_d_n0, eq7_e108_d_n1, eq7_e108_d_n2, eq7_e108_d_n3, eq7_e108_d_n4, eq7_e108_d_n5, eq7_e108_d_n6, eq7_e108_d_n7, eq7_e108_d_n8, eq7_e108_d_n9, eq7_e108_d_n10, eq7_e108_d_n11, eq7_e108_d_n12, eq7_e108_d_n13, eq7_e108_d_n14, eq7_e108_d_n15, eq7_e108_d_b0, eq7_e108_d_b1, eq7_e108_d_b2, eq7_e108_d_b3, eq7_e108_d_b4, eq7_e108_d_b5, eq7_e108_d_b6, eq7_e108_d_b7, eq7_e108_d_b8, eq7_e108_d_b9, eq7_e108_d_b10, eq7_e108_d_b11, eq7_e108_d_b12, eq7_e108_d_b13, eq7_e108_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e110;
        let eq7_node_derivatives: [f64; 16] = [eq7_e110_d_n0, eq7_e110_d_n1, eq7_e110_d_n2, eq7_e110_d_n3, eq7_e110_d_n4, eq7_e110_d_n5, eq7_e110_d_n6, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n9, eq7_e110_d_n10, eq7_e110_d_n11, eq7_e110_d_n12, eq7_e110_d_n13, eq7_e110_d_n14, eq7_e110_d_n15];
        let eq7_branch_derivatives: [f64; 15] = [eq7_e110_d_b0, eq7_e110_d_b1, eq7_e110_d_b2, eq7_e110_d_b3, eq7_e110_d_b4, eq7_e110_d_b5, eq7_e110_d_b6, eq7_e110_d_b7, eq7_e110_d_b8, eq7_e110_d_b9, eq7_e110_d_b10, eq7_e110_d_b11, eq7_e110_d_b12, eq7_e110_d_b13, eq7_e110_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e115, eq8_e115_d_n0, eq8_e115_d_n1, eq8_e115_d_n2, eq8_e115_d_n3, eq8_e115_d_n4, eq8_e115_d_n5, eq8_e115_d_n6, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n9, eq8_e115_d_n10, eq8_e115_d_n11, eq8_e115_d_n12, eq8_e115_d_n13, eq8_e115_d_n14, eq8_e115_d_n15, eq8_e115_d_b0, eq8_e115_d_b1, eq8_e115_d_b2, eq8_e115_d_b3, eq8_e115_d_b4, eq8_e115_d_b5, eq8_e115_d_b6, eq8_e115_d_b7, eq8_e115_d_b8, eq8_e115_d_b9, eq8_e115_d_b10, eq8_e115_d_b11, eq8_e115_d_b12, eq8_e115_d_b13, eq8_e115_d_b14,) = {
    if s.b[97] {
        let eq8_e113: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[20]);
        let eq8_e113_d_n0: f64 = (s.dn[20][0] * ddt_scale);
        let eq8_e113_d_n1: f64 = (s.dn[20][1] * ddt_scale);
        let eq8_e113_d_n2: f64 = (s.dn[20][2] * ddt_scale);
        let eq8_e113_d_n3: f64 = (s.dn[20][3] * ddt_scale);
        let eq8_e113_d_n4: f64 = (s.dn[20][4] * ddt_scale);
        let eq8_e113_d_n5: f64 = (s.dn[20][5] * ddt_scale);
        let eq8_e113_d_n6: f64 = (s.dn[20][6] * ddt_scale);
        let eq8_e113_d_n7: f64 = (s.dn[20][7] * ddt_scale);
        let eq8_e113_d_n8: f64 = (s.dn[20][8] * ddt_scale);
        let eq8_e113_d_n9: f64 = (s.dn[20][9] * ddt_scale);
        let eq8_e113_d_n10: f64 = (s.dn[20][10] * ddt_scale);
        let eq8_e113_d_n11: f64 = (s.dn[20][11] * ddt_scale);
        let eq8_e113_d_n12: f64 = (s.dn[20][12] * ddt_scale);
        let eq8_e113_d_n13: f64 = (s.dn[20][13] * ddt_scale);
        let eq8_e113_d_n14: f64 = (s.dn[20][14] * ddt_scale);
        let eq8_e113_d_n15: f64 = (s.dn[20][15] * ddt_scale);
        let eq8_e113_d_b0: f64 = (s.db[20][0] * ddt_scale);
        let eq8_e113_d_b1: f64 = (s.db[20][1] * ddt_scale);
        let eq8_e113_d_b2: f64 = (s.db[20][2] * ddt_scale);
        let eq8_e113_d_b3: f64 = (s.db[20][3] * ddt_scale);
        let eq8_e113_d_b4: f64 = (s.db[20][4] * ddt_scale);
        let eq8_e113_d_b5: f64 = (s.db[20][5] * ddt_scale);
        let eq8_e113_d_b6: f64 = (s.db[20][6] * ddt_scale);
        let eq8_e113_d_b7: f64 = (s.db[20][7] * ddt_scale);
        let eq8_e113_d_b8: f64 = (s.db[20][8] * ddt_scale);
        let eq8_e113_d_b9: f64 = (s.db[20][9] * ddt_scale);
        let eq8_e113_d_b10: f64 = (s.db[20][10] * ddt_scale);
        let eq8_e113_d_b11: f64 = (s.db[20][11] * ddt_scale);
        let eq8_e113_d_b12: f64 = (s.db[20][12] * ddt_scale);
        let eq8_e113_d_b13: f64 = (s.db[20][13] * ddt_scale);
        let eq8_e113_d_b14: f64 = (s.db[20][14] * ddt_scale);
        (eq8_e113, eq8_e113_d_n0, eq8_e113_d_n1, eq8_e113_d_n2, eq8_e113_d_n3, eq8_e113_d_n4, eq8_e113_d_n5, eq8_e113_d_n6, eq8_e113_d_n7, eq8_e113_d_n8, eq8_e113_d_n9, eq8_e113_d_n10, eq8_e113_d_n11, eq8_e113_d_n12, eq8_e113_d_n13, eq8_e113_d_n14, eq8_e113_d_n15, eq8_e113_d_b0, eq8_e113_d_b1, eq8_e113_d_b2, eq8_e113_d_b3, eq8_e113_d_b4, eq8_e113_d_b5, eq8_e113_d_b6, eq8_e113_d_b7, eq8_e113_d_b8, eq8_e113_d_b9, eq8_e113_d_b10, eq8_e113_d_b11, eq8_e113_d_b12, eq8_e113_d_b13, eq8_e113_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e115;
        let eq8_node_derivatives: [f64; 16] = [eq8_e115_d_n0, eq8_e115_d_n1, eq8_e115_d_n2, eq8_e115_d_n3, eq8_e115_d_n4, eq8_e115_d_n5, eq8_e115_d_n6, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n9, eq8_e115_d_n10, eq8_e115_d_n11, eq8_e115_d_n12, eq8_e115_d_n13, eq8_e115_d_n14, eq8_e115_d_n15];
        let eq8_branch_derivatives: [f64; 15] = [eq8_e115_d_b0, eq8_e115_d_b1, eq8_e115_d_b2, eq8_e115_d_b3, eq8_e115_d_b4, eq8_e115_d_b5, eq8_e115_d_b6, eq8_e115_d_b7, eq8_e115_d_b8, eq8_e115_d_b9, eq8_e115_d_b10, eq8_e115_d_b11, eq8_e115_d_b12, eq8_e115_d_b13, eq8_e115_d_b14];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e123, eq9_e123_d_n0, eq9_e123_d_n1, eq9_e123_d_n2, eq9_e123_d_n3, eq9_e123_d_n4, eq9_e123_d_n5, eq9_e123_d_n6, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n9, eq9_e123_d_n10, eq9_e123_d_n11, eq9_e123_d_n12, eq9_e123_d_n13, eq9_e123_d_n14, eq9_e123_d_n15, eq9_e123_d_b0, eq9_e123_d_b1, eq9_e123_d_b2, eq9_e123_d_b3, eq9_e123_d_b4, eq9_e123_d_b5, eq9_e123_d_b6, eq9_e123_d_b7, eq9_e123_d_b8, eq9_e123_d_b9, eq9_e123_d_b10, eq9_e123_d_b11, eq9_e123_d_b12, eq9_e123_d_b13, eq9_e123_d_b14,) = {
    if (!s.b[97]) {
        let eq9_e120: f64 = (s.v[19] * s.v[80]);
        let eq9_e120_d_n0: f64 = ((s.dn[19][0] * s.v[80]) + (s.v[19] * s.dn[80][0]));
        let eq9_e120_d_n1: f64 = ((s.dn[19][1] * s.v[80]) + (s.v[19] * s.dn[80][1]));
        let eq9_e120_d_n2: f64 = ((s.dn[19][2] * s.v[80]) + (s.v[19] * s.dn[80][2]));
        let eq9_e120_d_n3: f64 = ((s.dn[19][3] * s.v[80]) + (s.v[19] * s.dn[80][3]));
        let eq9_e120_d_n4: f64 = ((s.dn[19][4] * s.v[80]) + (s.v[19] * s.dn[80][4]));
        let eq9_e120_d_n5: f64 = ((s.dn[19][5] * s.v[80]) + (s.v[19] * s.dn[80][5]));
        let eq9_e120_d_n6: f64 = ((s.dn[19][6] * s.v[80]) + (s.v[19] * s.dn[80][6]));
        let eq9_e120_d_n7: f64 = ((s.dn[19][7] * s.v[80]) + (s.v[19] * s.dn[80][7]));
        let eq9_e120_d_n8: f64 = ((s.dn[19][8] * s.v[80]) + (s.v[19] * s.dn[80][8]));
        let eq9_e120_d_n9: f64 = ((s.dn[19][9] * s.v[80]) + (s.v[19] * s.dn[80][9]));
        let eq9_e120_d_n10: f64 = ((s.dn[19][10] * s.v[80]) + (s.v[19] * s.dn[80][10]));
        let eq9_e120_d_n11: f64 = ((s.dn[19][11] * s.v[80]) + (s.v[19] * s.dn[80][11]));
        let eq9_e120_d_n12: f64 = ((s.dn[19][12] * s.v[80]) + (s.v[19] * s.dn[80][12]));
        let eq9_e120_d_n13: f64 = ((s.dn[19][13] * s.v[80]) + (s.v[19] * s.dn[80][13]));
        let eq9_e120_d_n14: f64 = ((s.dn[19][14] * s.v[80]) + (s.v[19] * s.dn[80][14]));
        let eq9_e120_d_n15: f64 = ((s.dn[19][15] * s.v[80]) + (s.v[19] * s.dn[80][15]));
        let eq9_e120_d_b0: f64 = ((s.db[19][0] * s.v[80]) + (s.v[19] * s.db[80][0]));
        let eq9_e120_d_b1: f64 = ((s.db[19][1] * s.v[80]) + (s.v[19] * s.db[80][1]));
        let eq9_e120_d_b2: f64 = ((s.db[19][2] * s.v[80]) + (s.v[19] * s.db[80][2]));
        let eq9_e120_d_b3: f64 = ((s.db[19][3] * s.v[80]) + (s.v[19] * s.db[80][3]));
        let eq9_e120_d_b4: f64 = ((s.db[19][4] * s.v[80]) + (s.v[19] * s.db[80][4]));
        let eq9_e120_d_b5: f64 = ((s.db[19][5] * s.v[80]) + (s.v[19] * s.db[80][5]));
        let eq9_e120_d_b6: f64 = ((s.db[19][6] * s.v[80]) + (s.v[19] * s.db[80][6]));
        let eq9_e120_d_b7: f64 = ((s.db[19][7] * s.v[80]) + (s.v[19] * s.db[80][7]));
        let eq9_e120_d_b8: f64 = ((s.db[19][8] * s.v[80]) + (s.v[19] * s.db[80][8]));
        let eq9_e120_d_b9: f64 = ((s.db[19][9] * s.v[80]) + (s.v[19] * s.db[80][9]));
        let eq9_e120_d_b10: f64 = ((s.db[19][10] * s.v[80]) + (s.v[19] * s.db[80][10]));
        let eq9_e120_d_b11: f64 = ((s.db[19][11] * s.v[80]) + (s.v[19] * s.db[80][11]));
        let eq9_e120_d_b12: f64 = ((s.db[19][12] * s.v[80]) + (s.v[19] * s.db[80][12]));
        let eq9_e120_d_b13: f64 = ((s.db[19][13] * s.v[80]) + (s.v[19] * s.db[80][13]));
        let eq9_e120_d_b14: f64 = ((s.db[19][14] * s.v[80]) + (s.v[19] * s.db[80][14]));
        let eq9_e121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq9_e120);
        let eq9_e121_d_n0: f64 = (eq9_e120_d_n0 * ddt_scale);
        let eq9_e121_d_n1: f64 = (eq9_e120_d_n1 * ddt_scale);
        let eq9_e121_d_n2: f64 = (eq9_e120_d_n2 * ddt_scale);
        let eq9_e121_d_n3: f64 = (eq9_e120_d_n3 * ddt_scale);
        let eq9_e121_d_n4: f64 = (eq9_e120_d_n4 * ddt_scale);
        let eq9_e121_d_n5: f64 = (eq9_e120_d_n5 * ddt_scale);
        let eq9_e121_d_n6: f64 = (eq9_e120_d_n6 * ddt_scale);
        let eq9_e121_d_n7: f64 = (eq9_e120_d_n7 * ddt_scale);
        let eq9_e121_d_n8: f64 = (eq9_e120_d_n8 * ddt_scale);
        let eq9_e121_d_n9: f64 = (eq9_e120_d_n9 * ddt_scale);
        let eq9_e121_d_n10: f64 = (eq9_e120_d_n10 * ddt_scale);
        let eq9_e121_d_n11: f64 = (eq9_e120_d_n11 * ddt_scale);
        let eq9_e121_d_n12: f64 = (eq9_e120_d_n12 * ddt_scale);
        let eq9_e121_d_n13: f64 = (eq9_e120_d_n13 * ddt_scale);
        let eq9_e121_d_n14: f64 = (eq9_e120_d_n14 * ddt_scale);
        let eq9_e121_d_n15: f64 = (eq9_e120_d_n15 * ddt_scale);
        let eq9_e121_d_b0: f64 = (eq9_e120_d_b0 * ddt_scale);
        let eq9_e121_d_b1: f64 = (eq9_e120_d_b1 * ddt_scale);
        let eq9_e121_d_b2: f64 = (eq9_e120_d_b2 * ddt_scale);
        let eq9_e121_d_b3: f64 = (eq9_e120_d_b3 * ddt_scale);
        let eq9_e121_d_b4: f64 = (eq9_e120_d_b4 * ddt_scale);
        let eq9_e121_d_b5: f64 = (eq9_e120_d_b5 * ddt_scale);
        let eq9_e121_d_b6: f64 = (eq9_e120_d_b6 * ddt_scale);
        let eq9_e121_d_b7: f64 = (eq9_e120_d_b7 * ddt_scale);
        let eq9_e121_d_b8: f64 = (eq9_e120_d_b8 * ddt_scale);
        let eq9_e121_d_b9: f64 = (eq9_e120_d_b9 * ddt_scale);
        let eq9_e121_d_b10: f64 = (eq9_e120_d_b10 * ddt_scale);
        let eq9_e121_d_b11: f64 = (eq9_e120_d_b11 * ddt_scale);
        let eq9_e121_d_b12: f64 = (eq9_e120_d_b12 * ddt_scale);
        let eq9_e121_d_b13: f64 = (eq9_e120_d_b13 * ddt_scale);
        let eq9_e121_d_b14: f64 = (eq9_e120_d_b14 * ddt_scale);
        (eq9_e121, eq9_e121_d_n0, eq9_e121_d_n1, eq9_e121_d_n2, eq9_e121_d_n3, eq9_e121_d_n4, eq9_e121_d_n5, eq9_e121_d_n6, eq9_e121_d_n7, eq9_e121_d_n8, eq9_e121_d_n9, eq9_e121_d_n10, eq9_e121_d_n11, eq9_e121_d_n12, eq9_e121_d_n13, eq9_e121_d_n14, eq9_e121_d_n15, eq9_e121_d_b0, eq9_e121_d_b1, eq9_e121_d_b2, eq9_e121_d_b3, eq9_e121_d_b4, eq9_e121_d_b5, eq9_e121_d_b6, eq9_e121_d_b7, eq9_e121_d_b8, eq9_e121_d_b9, eq9_e121_d_b10, eq9_e121_d_b11, eq9_e121_d_b12, eq9_e121_d_b13, eq9_e121_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e123;
        let eq9_node_derivatives: [f64; 16] = [eq9_e123_d_n0, eq9_e123_d_n1, eq9_e123_d_n2, eq9_e123_d_n3, eq9_e123_d_n4, eq9_e123_d_n5, eq9_e123_d_n6, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n9, eq9_e123_d_n10, eq9_e123_d_n11, eq9_e123_d_n12, eq9_e123_d_n13, eq9_e123_d_n14, eq9_e123_d_n15];
        let eq9_branch_derivatives: [f64; 15] = [eq9_e123_d_b0, eq9_e123_d_b1, eq9_e123_d_b2, eq9_e123_d_b3, eq9_e123_d_b4, eq9_e123_d_b5, eq9_e123_d_b6, eq9_e123_d_b7, eq9_e123_d_b8, eq9_e123_d_b9, eq9_e123_d_b10, eq9_e123_d_b11, eq9_e123_d_b12, eq9_e123_d_b13, eq9_e123_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e131, eq10_e131_d_n0, eq10_e131_d_n1, eq10_e131_d_n2, eq10_e131_d_n3, eq10_e131_d_n4, eq10_e131_d_n5, eq10_e131_d_n6, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n9, eq10_e131_d_n10, eq10_e131_d_n11, eq10_e131_d_n12, eq10_e131_d_n13, eq10_e131_d_n14, eq10_e131_d_n15, eq10_e131_d_b0, eq10_e131_d_b1, eq10_e131_d_b2, eq10_e131_d_b3, eq10_e131_d_b4, eq10_e131_d_b5, eq10_e131_d_b6, eq10_e131_d_b7, eq10_e131_d_b8, eq10_e131_d_b9, eq10_e131_d_b10, eq10_e131_d_b11, eq10_e131_d_b12, eq10_e131_d_b13, eq10_e131_d_b14,) = {
    if (!s.b[97]) {
        let eq10_e128: f64 = (s.v[18] * s.v[79]);
        let eq10_e128_d_n0: f64 = ((s.dn[18][0] * s.v[79]) + (s.v[18] * s.dn[79][0]));
        let eq10_e128_d_n1: f64 = ((s.dn[18][1] * s.v[79]) + (s.v[18] * s.dn[79][1]));
        let eq10_e128_d_n2: f64 = ((s.dn[18][2] * s.v[79]) + (s.v[18] * s.dn[79][2]));
        let eq10_e128_d_n3: f64 = ((s.dn[18][3] * s.v[79]) + (s.v[18] * s.dn[79][3]));
        let eq10_e128_d_n4: f64 = ((s.dn[18][4] * s.v[79]) + (s.v[18] * s.dn[79][4]));
        let eq10_e128_d_n5: f64 = ((s.dn[18][5] * s.v[79]) + (s.v[18] * s.dn[79][5]));
        let eq10_e128_d_n6: f64 = ((s.dn[18][6] * s.v[79]) + (s.v[18] * s.dn[79][6]));
        let eq10_e128_d_n7: f64 = ((s.dn[18][7] * s.v[79]) + (s.v[18] * s.dn[79][7]));
        let eq10_e128_d_n8: f64 = ((s.dn[18][8] * s.v[79]) + (s.v[18] * s.dn[79][8]));
        let eq10_e128_d_n9: f64 = ((s.dn[18][9] * s.v[79]) + (s.v[18] * s.dn[79][9]));
        let eq10_e128_d_n10: f64 = ((s.dn[18][10] * s.v[79]) + (s.v[18] * s.dn[79][10]));
        let eq10_e128_d_n11: f64 = ((s.dn[18][11] * s.v[79]) + (s.v[18] * s.dn[79][11]));
        let eq10_e128_d_n12: f64 = ((s.dn[18][12] * s.v[79]) + (s.v[18] * s.dn[79][12]));
        let eq10_e128_d_n13: f64 = ((s.dn[18][13] * s.v[79]) + (s.v[18] * s.dn[79][13]));
        let eq10_e128_d_n14: f64 = ((s.dn[18][14] * s.v[79]) + (s.v[18] * s.dn[79][14]));
        let eq10_e128_d_n15: f64 = ((s.dn[18][15] * s.v[79]) + (s.v[18] * s.dn[79][15]));
        let eq10_e128_d_b0: f64 = ((s.db[18][0] * s.v[79]) + (s.v[18] * s.db[79][0]));
        let eq10_e128_d_b1: f64 = ((s.db[18][1] * s.v[79]) + (s.v[18] * s.db[79][1]));
        let eq10_e128_d_b2: f64 = ((s.db[18][2] * s.v[79]) + (s.v[18] * s.db[79][2]));
        let eq10_e128_d_b3: f64 = ((s.db[18][3] * s.v[79]) + (s.v[18] * s.db[79][3]));
        let eq10_e128_d_b4: f64 = ((s.db[18][4] * s.v[79]) + (s.v[18] * s.db[79][4]));
        let eq10_e128_d_b5: f64 = ((s.db[18][5] * s.v[79]) + (s.v[18] * s.db[79][5]));
        let eq10_e128_d_b6: f64 = ((s.db[18][6] * s.v[79]) + (s.v[18] * s.db[79][6]));
        let eq10_e128_d_b7: f64 = ((s.db[18][7] * s.v[79]) + (s.v[18] * s.db[79][7]));
        let eq10_e128_d_b8: f64 = ((s.db[18][8] * s.v[79]) + (s.v[18] * s.db[79][8]));
        let eq10_e128_d_b9: f64 = ((s.db[18][9] * s.v[79]) + (s.v[18] * s.db[79][9]));
        let eq10_e128_d_b10: f64 = ((s.db[18][10] * s.v[79]) + (s.v[18] * s.db[79][10]));
        let eq10_e128_d_b11: f64 = ((s.db[18][11] * s.v[79]) + (s.v[18] * s.db[79][11]));
        let eq10_e128_d_b12: f64 = ((s.db[18][12] * s.v[79]) + (s.v[18] * s.db[79][12]));
        let eq10_e128_d_b13: f64 = ((s.db[18][13] * s.v[79]) + (s.v[18] * s.db[79][13]));
        let eq10_e128_d_b14: f64 = ((s.db[18][14] * s.v[79]) + (s.v[18] * s.db[79][14]));
        let eq10_e129: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq10_e128);
        let eq10_e129_d_n0: f64 = (eq10_e128_d_n0 * ddt_scale);
        let eq10_e129_d_n1: f64 = (eq10_e128_d_n1 * ddt_scale);
        let eq10_e129_d_n2: f64 = (eq10_e128_d_n2 * ddt_scale);
        let eq10_e129_d_n3: f64 = (eq10_e128_d_n3 * ddt_scale);
        let eq10_e129_d_n4: f64 = (eq10_e128_d_n4 * ddt_scale);
        let eq10_e129_d_n5: f64 = (eq10_e128_d_n5 * ddt_scale);
        let eq10_e129_d_n6: f64 = (eq10_e128_d_n6 * ddt_scale);
        let eq10_e129_d_n7: f64 = (eq10_e128_d_n7 * ddt_scale);
        let eq10_e129_d_n8: f64 = (eq10_e128_d_n8 * ddt_scale);
        let eq10_e129_d_n9: f64 = (eq10_e128_d_n9 * ddt_scale);
        let eq10_e129_d_n10: f64 = (eq10_e128_d_n10 * ddt_scale);
        let eq10_e129_d_n11: f64 = (eq10_e128_d_n11 * ddt_scale);
        let eq10_e129_d_n12: f64 = (eq10_e128_d_n12 * ddt_scale);
        let eq10_e129_d_n13: f64 = (eq10_e128_d_n13 * ddt_scale);
        let eq10_e129_d_n14: f64 = (eq10_e128_d_n14 * ddt_scale);
        let eq10_e129_d_n15: f64 = (eq10_e128_d_n15 * ddt_scale);
        let eq10_e129_d_b0: f64 = (eq10_e128_d_b0 * ddt_scale);
        let eq10_e129_d_b1: f64 = (eq10_e128_d_b1 * ddt_scale);
        let eq10_e129_d_b2: f64 = (eq10_e128_d_b2 * ddt_scale);
        let eq10_e129_d_b3: f64 = (eq10_e128_d_b3 * ddt_scale);
        let eq10_e129_d_b4: f64 = (eq10_e128_d_b4 * ddt_scale);
        let eq10_e129_d_b5: f64 = (eq10_e128_d_b5 * ddt_scale);
        let eq10_e129_d_b6: f64 = (eq10_e128_d_b6 * ddt_scale);
        let eq10_e129_d_b7: f64 = (eq10_e128_d_b7 * ddt_scale);
        let eq10_e129_d_b8: f64 = (eq10_e128_d_b8 * ddt_scale);
        let eq10_e129_d_b9: f64 = (eq10_e128_d_b9 * ddt_scale);
        let eq10_e129_d_b10: f64 = (eq10_e128_d_b10 * ddt_scale);
        let eq10_e129_d_b11: f64 = (eq10_e128_d_b11 * ddt_scale);
        let eq10_e129_d_b12: f64 = (eq10_e128_d_b12 * ddt_scale);
        let eq10_e129_d_b13: f64 = (eq10_e128_d_b13 * ddt_scale);
        let eq10_e129_d_b14: f64 = (eq10_e128_d_b14 * ddt_scale);
        (eq10_e129, eq10_e129_d_n0, eq10_e129_d_n1, eq10_e129_d_n2, eq10_e129_d_n3, eq10_e129_d_n4, eq10_e129_d_n5, eq10_e129_d_n6, eq10_e129_d_n7, eq10_e129_d_n8, eq10_e129_d_n9, eq10_e129_d_n10, eq10_e129_d_n11, eq10_e129_d_n12, eq10_e129_d_n13, eq10_e129_d_n14, eq10_e129_d_n15, eq10_e129_d_b0, eq10_e129_d_b1, eq10_e129_d_b2, eq10_e129_d_b3, eq10_e129_d_b4, eq10_e129_d_b5, eq10_e129_d_b6, eq10_e129_d_b7, eq10_e129_d_b8, eq10_e129_d_b9, eq10_e129_d_b10, eq10_e129_d_b11, eq10_e129_d_b12, eq10_e129_d_b13, eq10_e129_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e131;
        let eq10_node_derivatives: [f64; 16] = [eq10_e131_d_n0, eq10_e131_d_n1, eq10_e131_d_n2, eq10_e131_d_n3, eq10_e131_d_n4, eq10_e131_d_n5, eq10_e131_d_n6, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n9, eq10_e131_d_n10, eq10_e131_d_n11, eq10_e131_d_n12, eq10_e131_d_n13, eq10_e131_d_n14, eq10_e131_d_n15];
        let eq10_branch_derivatives: [f64; 15] = [eq10_e131_d_b0, eq10_e131_d_b1, eq10_e131_d_b2, eq10_e131_d_b3, eq10_e131_d_b4, eq10_e131_d_b5, eq10_e131_d_b6, eq10_e131_d_b7, eq10_e131_d_b8, eq10_e131_d_b9, eq10_e131_d_b10, eq10_e131_d_b11, eq10_e131_d_b12, eq10_e131_d_b13, eq10_e131_d_b14];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq24_e211, eq24_e211_d_b5,) = {
    if s.b[102] {
        let eq24_e209: f64 = (bi5 * p.p42);
        let eq24_e209_d_b5: f64 = p.p42;
        (eq24_e209, eq24_e209_d_b5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e211;
        stamper.stamp_potential_branch1_local(
            5,
            eq24_value,
            5,
            eq24_e211_d_b5,
        );
        let (eq25_e218, eq25_e218_d_b5,) = {
    if s.b[102] {
        let eq25_e215: f64 = (p.p50 * bi5);
        let eq25_e215_d_b5: f64 = p.p50;
        let eq25_e216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq25_e215);
        let eq25_e216_d_b5: f64 = (eq25_e215_d_b5 * ddt_scale);
        (eq25_e216, eq25_e216_d_b5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e218;
        stamper.stamp_potential_branch1_local(
            6,
            eq25_value,
            5,
            eq25_e218_d_b5,
        );
        let (eq26_e232,) = {
    if (s.b[102] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e232;
        stamper.stamp_potential_const_local(
            7,
            eq26_value,
        );
        let (eq27_e242, eq27_e242_d_b5,) = {
    if ((!s.b[102]) && s.b[103]) {
        let eq27_e239: f64 = (p.p50 * bi5);
        let eq27_e239_d_b5: f64 = p.p50;
        let eq27_e240: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq27_e239);
        let eq27_e240_d_b5: f64 = (eq27_e239_d_b5 * ddt_scale);
        (eq27_e240, eq27_e240_d_b5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e242;
        stamper.stamp_potential_branch1_local(
            8,
            eq27_value,
            5,
            eq27_e242_d_b5,
        );
        let (eq29_e256, eq29_e256_d_n0, eq29_e256_d_n1, eq29_e256_d_n2, eq29_e256_d_n3, eq29_e256_d_n4, eq29_e256_d_n5, eq29_e256_d_n6, eq29_e256_d_n7, eq29_e256_d_n8, eq29_e256_d_n9, eq29_e256_d_n10, eq29_e256_d_n11, eq29_e256_d_n12, eq29_e256_d_n13, eq29_e256_d_n14, eq29_e256_d_n15, eq29_e256_d_b0, eq29_e256_d_b1, eq29_e256_d_b2, eq29_e256_d_b3, eq29_e256_d_b4, eq29_e256_d_b5, eq29_e256_d_b6, eq29_e256_d_b7, eq29_e256_d_b8, eq29_e256_d_b9, eq29_e256_d_b10, eq29_e256_d_b11, eq29_e256_d_b12, eq29_e256_d_b13, eq29_e256_d_b14,) = {
    if s.b[104] {
        let eq29_e254: f64 = (bi10 * s.v[36]);
        let eq29_e254_d_n0: f64 = (bi10 * s.dn[36][0]);
        let eq29_e254_d_n1: f64 = (bi10 * s.dn[36][1]);
        let eq29_e254_d_n2: f64 = (bi10 * s.dn[36][2]);
        let eq29_e254_d_n3: f64 = (bi10 * s.dn[36][3]);
        let eq29_e254_d_n4: f64 = (bi10 * s.dn[36][4]);
        let eq29_e254_d_n5: f64 = (bi10 * s.dn[36][5]);
        let eq29_e254_d_n6: f64 = (bi10 * s.dn[36][6]);
        let eq29_e254_d_n7: f64 = (bi10 * s.dn[36][7]);
        let eq29_e254_d_n8: f64 = (bi10 * s.dn[36][8]);
        let eq29_e254_d_n9: f64 = (bi10 * s.dn[36][9]);
        let eq29_e254_d_n10: f64 = (bi10 * s.dn[36][10]);
        let eq29_e254_d_n11: f64 = (bi10 * s.dn[36][11]);
        let eq29_e254_d_n12: f64 = (bi10 * s.dn[36][12]);
        let eq29_e254_d_n13: f64 = (bi10 * s.dn[36][13]);
        let eq29_e254_d_n14: f64 = (bi10 * s.dn[36][14]);
        let eq29_e254_d_n15: f64 = (bi10 * s.dn[36][15]);
        let eq29_e254_d_b0: f64 = (bi10 * s.db[36][0]);
        let eq29_e254_d_b1: f64 = (bi10 * s.db[36][1]);
        let eq29_e254_d_b2: f64 = (bi10 * s.db[36][2]);
        let eq29_e254_d_b3: f64 = (bi10 * s.db[36][3]);
        let eq29_e254_d_b4: f64 = (bi10 * s.db[36][4]);
        let eq29_e254_d_b5: f64 = (bi10 * s.db[36][5]);
        let eq29_e254_d_b6: f64 = (bi10 * s.db[36][6]);
        let eq29_e254_d_b7: f64 = (bi10 * s.db[36][7]);
        let eq29_e254_d_b8: f64 = (bi10 * s.db[36][8]);
        let eq29_e254_d_b9: f64 = (bi10 * s.db[36][9]);
        let eq29_e254_d_b10: f64 = (s.v[36] + (bi10 * s.db[36][10]));
        let eq29_e254_d_b11: f64 = (bi10 * s.db[36][11]);
        let eq29_e254_d_b12: f64 = (bi10 * s.db[36][12]);
        let eq29_e254_d_b13: f64 = (bi10 * s.db[36][13]);
        let eq29_e254_d_b14: f64 = (bi10 * s.db[36][14]);
        (eq29_e254, eq29_e254_d_n0, eq29_e254_d_n1, eq29_e254_d_n2, eq29_e254_d_n3, eq29_e254_d_n4, eq29_e254_d_n5, eq29_e254_d_n6, eq29_e254_d_n7, eq29_e254_d_n8, eq29_e254_d_n9, eq29_e254_d_n10, eq29_e254_d_n11, eq29_e254_d_n12, eq29_e254_d_n13, eq29_e254_d_n14, eq29_e254_d_n15, eq29_e254_d_b0, eq29_e254_d_b1, eq29_e254_d_b2, eq29_e254_d_b3, eq29_e254_d_b4, eq29_e254_d_b5, eq29_e254_d_b6, eq29_e254_d_b7, eq29_e254_d_b8, eq29_e254_d_b9, eq29_e254_d_b10, eq29_e254_d_b11, eq29_e254_d_b12, eq29_e254_d_b13, eq29_e254_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e256;
        let eq29_node_derivatives: [f64; 16] = [eq29_e256_d_n0, eq29_e256_d_n1, eq29_e256_d_n2, eq29_e256_d_n3, eq29_e256_d_n4, eq29_e256_d_n5, eq29_e256_d_n6, eq29_e256_d_n7, eq29_e256_d_n8, eq29_e256_d_n9, eq29_e256_d_n10, eq29_e256_d_n11, eq29_e256_d_n12, eq29_e256_d_n13, eq29_e256_d_n14, eq29_e256_d_n15];
        let eq29_branch_derivatives: [f64; 15] = [eq29_e256_d_b0, eq29_e256_d_b1, eq29_e256_d_b2, eq29_e256_d_b3, eq29_e256_d_b4, eq29_e256_d_b5, eq29_e256_d_b6, eq29_e256_d_b7, eq29_e256_d_b8, eq29_e256_d_b9, eq29_e256_d_b10, eq29_e256_d_b11, eq29_e256_d_b12, eq29_e256_d_b13, eq29_e256_d_b14];
        stamper.stamp_potential_dense_local(
            10,
            eq29_value,
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
        );
        let (eq30_e270,) = {
    if (s.b[104] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e270;
        stamper.stamp_potential_const_local(
            11,
            eq30_value,
        );
        let eq32_e278: f64 = (p.p49 * bi13);
        let eq32_e278_d_b13: f64 = p.p49;
        let eq32_e279: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq32_e278);
        let eq32_e279_d_b13: f64 = (eq32_e278_d_b13 * ddt_scale);
        let eq32_value: f64 = eq32_e279;
        stamper.stamp_potential_branch1_local(
            13,
            eq32_value,
            13,
            eq32_e279_d_b13,
        );
        let (eq33_e285, eq33_e285_d_n0, eq33_e285_d_n1, eq33_e285_d_n2, eq33_e285_d_n3, eq33_e285_d_n4, eq33_e285_d_n5, eq33_e285_d_n6, eq33_e285_d_n7, eq33_e285_d_n8, eq33_e285_d_n9, eq33_e285_d_n10, eq33_e285_d_n11, eq33_e285_d_n12, eq33_e285_d_n13, eq33_e285_d_n14, eq33_e285_d_n15, eq33_e285_d_b0, eq33_e285_d_b1, eq33_e285_d_b2, eq33_e285_d_b3, eq33_e285_d_b4, eq33_e285_d_b5, eq33_e285_d_b6, eq33_e285_d_b7, eq33_e285_d_b8, eq33_e285_d_b9, eq33_e285_d_b10, eq33_e285_d_b11, eq33_e285_d_b12, eq33_e285_d_b13, eq33_e285_d_b14,) = {
    if s.b[105] {
        let eq33_e283: f64 = (bi14 * s.v[35]);
        let eq33_e283_d_n0: f64 = (bi14 * s.dn[35][0]);
        let eq33_e283_d_n1: f64 = (bi14 * s.dn[35][1]);
        let eq33_e283_d_n2: f64 = (bi14 * s.dn[35][2]);
        let eq33_e283_d_n3: f64 = (bi14 * s.dn[35][3]);
        let eq33_e283_d_n4: f64 = (bi14 * s.dn[35][4]);
        let eq33_e283_d_n5: f64 = (bi14 * s.dn[35][5]);
        let eq33_e283_d_n6: f64 = (bi14 * s.dn[35][6]);
        let eq33_e283_d_n7: f64 = (bi14 * s.dn[35][7]);
        let eq33_e283_d_n8: f64 = (bi14 * s.dn[35][8]);
        let eq33_e283_d_n9: f64 = (bi14 * s.dn[35][9]);
        let eq33_e283_d_n10: f64 = (bi14 * s.dn[35][10]);
        let eq33_e283_d_n11: f64 = (bi14 * s.dn[35][11]);
        let eq33_e283_d_n12: f64 = (bi14 * s.dn[35][12]);
        let eq33_e283_d_n13: f64 = (bi14 * s.dn[35][13]);
        let eq33_e283_d_n14: f64 = (bi14 * s.dn[35][14]);
        let eq33_e283_d_n15: f64 = (bi14 * s.dn[35][15]);
        let eq33_e283_d_b0: f64 = (bi14 * s.db[35][0]);
        let eq33_e283_d_b1: f64 = (bi14 * s.db[35][1]);
        let eq33_e283_d_b2: f64 = (bi14 * s.db[35][2]);
        let eq33_e283_d_b3: f64 = (bi14 * s.db[35][3]);
        let eq33_e283_d_b4: f64 = (bi14 * s.db[35][4]);
        let eq33_e283_d_b5: f64 = (bi14 * s.db[35][5]);
        let eq33_e283_d_b6: f64 = (bi14 * s.db[35][6]);
        let eq33_e283_d_b7: f64 = (bi14 * s.db[35][7]);
        let eq33_e283_d_b8: f64 = (bi14 * s.db[35][8]);
        let eq33_e283_d_b9: f64 = (bi14 * s.db[35][9]);
        let eq33_e283_d_b10: f64 = (bi14 * s.db[35][10]);
        let eq33_e283_d_b11: f64 = (bi14 * s.db[35][11]);
        let eq33_e283_d_b12: f64 = (bi14 * s.db[35][12]);
        let eq33_e283_d_b13: f64 = (bi14 * s.db[35][13]);
        let eq33_e283_d_b14: f64 = (s.v[35] + (bi14 * s.db[35][14]));
        (eq33_e283, eq33_e283_d_n0, eq33_e283_d_n1, eq33_e283_d_n2, eq33_e283_d_n3, eq33_e283_d_n4, eq33_e283_d_n5, eq33_e283_d_n6, eq33_e283_d_n7, eq33_e283_d_n8, eq33_e283_d_n9, eq33_e283_d_n10, eq33_e283_d_n11, eq33_e283_d_n12, eq33_e283_d_n13, eq33_e283_d_n14, eq33_e283_d_n15, eq33_e283_d_b0, eq33_e283_d_b1, eq33_e283_d_b2, eq33_e283_d_b3, eq33_e283_d_b4, eq33_e283_d_b5, eq33_e283_d_b6, eq33_e283_d_b7, eq33_e283_d_b8, eq33_e283_d_b9, eq33_e283_d_b10, eq33_e283_d_b11, eq33_e283_d_b12, eq33_e283_d_b13, eq33_e283_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e285;
        let eq33_node_derivatives: [f64; 16] = [eq33_e285_d_n0, eq33_e285_d_n1, eq33_e285_d_n2, eq33_e285_d_n3, eq33_e285_d_n4, eq33_e285_d_n5, eq33_e285_d_n6, eq33_e285_d_n7, eq33_e285_d_n8, eq33_e285_d_n9, eq33_e285_d_n10, eq33_e285_d_n11, eq33_e285_d_n12, eq33_e285_d_n13, eq33_e285_d_n14, eq33_e285_d_n15];
        let eq33_branch_derivatives: [f64; 15] = [eq33_e285_d_b0, eq33_e285_d_b1, eq33_e285_d_b2, eq33_e285_d_b3, eq33_e285_d_b4, eq33_e285_d_b5, eq33_e285_d_b6, eq33_e285_d_b7, eq33_e285_d_b8, eq33_e285_d_b9, eq33_e285_d_b10, eq33_e285_d_b11, eq33_e285_d_b12, eq33_e285_d_b13, eq33_e285_d_b14];
        stamper.stamp_potential_dense_local(
            14,
            eq33_value,
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
        );
        let (eq34_e292, eq34_e292_d_b14,) = {
    if s.b[105] {
        let eq34_e289: f64 = (p.p48 * bi14);
        let eq34_e289_d_b14: f64 = p.p48;
        let eq34_e290: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq34_e289);
        let eq34_e290_d_b14: f64 = (eq34_e289_d_b14 * ddt_scale);
        (eq34_e290, eq34_e290_d_b14,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e292;
        stamper.stamp_potential_branch1_local(
            15,
            eq34_value,
            14,
            eq34_e292_d_b14,
        );
        let (eq35_e306,) = {
    if (s.b[105] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e306;
        stamper.stamp_potential_const_local(
            16,
            eq35_value,
        );
        let (eq36_e316, eq36_e316_d_b14,) = {
    if ((!s.b[105]) && s.b[106]) {
        let eq36_e313: f64 = (p.p48 * bi14);
        let eq36_e313_d_b14: f64 = p.p48;
        let eq36_e314: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, eq36_e313);
        let eq36_e314_d_b14: f64 = (eq36_e313_d_b14 * ddt_scale);
        (eq36_e314, eq36_e314_d_b14,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e316;
        stamper.stamp_potential_branch1_local(
            17,
            eq36_value,
            14,
            eq36_e316_d_b14,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq46_e420, eq46_e420_d_n0, eq46_e420_d_n1, eq46_e420_d_n2, eq46_e420_d_n3, eq46_e420_d_n4, eq46_e420_d_n5, eq46_e420_d_n6, eq46_e420_d_n7, eq46_e420_d_n8, eq46_e420_d_n9, eq46_e420_d_n10, eq46_e420_d_n11, eq46_e420_d_n12, eq46_e420_d_n13, eq46_e420_d_n14, eq46_e420_d_n15, eq46_e420_d_b0, eq46_e420_d_b1, eq46_e420_d_b2, eq46_e420_d_b3, eq46_e420_d_b4, eq46_e420_d_b5, eq46_e420_d_b6, eq46_e420_d_b7, eq46_e420_d_b8, eq46_e420_d_b9, eq46_e420_d_b10, eq46_e420_d_b11, eq46_e420_d_b12, eq46_e420_d_b13, eq46_e420_d_b14,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        let eq46_e415: f64 = (-s.v[118]);
        let eq46_e415_d_n0: f64 = (-s.dn[118][0]);
        let eq46_e415_d_n1: f64 = (-s.dn[118][1]);
        let eq46_e415_d_n2: f64 = (-s.dn[118][2]);
        let eq46_e415_d_n3: f64 = (-s.dn[118][3]);
        let eq46_e415_d_n4: f64 = (-s.dn[118][4]);
        let eq46_e415_d_n5: f64 = (-s.dn[118][5]);
        let eq46_e415_d_n6: f64 = (-s.dn[118][6]);
        let eq46_e415_d_n7: f64 = (-s.dn[118][7]);
        let eq46_e415_d_n8: f64 = (-s.dn[118][8]);
        let eq46_e415_d_n9: f64 = (-s.dn[118][9]);
        let eq46_e415_d_n10: f64 = (-s.dn[118][10]);
        let eq46_e415_d_n11: f64 = (-s.dn[118][11]);
        let eq46_e415_d_n12: f64 = (-s.dn[118][12]);
        let eq46_e415_d_n13: f64 = (-s.dn[118][13]);
        let eq46_e415_d_n14: f64 = (-s.dn[118][14]);
        let eq46_e415_d_n15: f64 = (-s.dn[118][15]);
        let eq46_e415_d_b0: f64 = (-s.db[118][0]);
        let eq46_e415_d_b1: f64 = (-s.db[118][1]);
        let eq46_e415_d_b2: f64 = (-s.db[118][2]);
        let eq46_e415_d_b3: f64 = (-s.db[118][3]);
        let eq46_e415_d_b4: f64 = (-s.db[118][4]);
        let eq46_e415_d_b5: f64 = (-s.db[118][5]);
        let eq46_e415_d_b6: f64 = (-s.db[118][6]);
        let eq46_e415_d_b7: f64 = (-s.db[118][7]);
        let eq46_e415_d_b8: f64 = (-s.db[118][8]);
        let eq46_e415_d_b9: f64 = (-s.db[118][9]);
        let eq46_e415_d_b10: f64 = (-s.db[118][10]);
        let eq46_e415_d_b11: f64 = (-s.db[118][11]);
        let eq46_e415_d_b12: f64 = (-s.db[118][12]);
        let eq46_e415_d_b13: f64 = (-s.db[118][13]);
        let eq46_e415_d_b14: f64 = (-s.db[118][14]);
        let eq46_e417: f64 = (eq46_e415 * (nv14 - 0.0));
        let eq46_e417_d_n0: f64 = (eq46_e415_d_n0 * (nv14 - 0.0));
        let eq46_e417_d_n1: f64 = (eq46_e415_d_n1 * (nv14 - 0.0));
        let eq46_e417_d_n2: f64 = (eq46_e415_d_n2 * (nv14 - 0.0));
        let eq46_e417_d_n3: f64 = (eq46_e415_d_n3 * (nv14 - 0.0));
        let eq46_e417_d_n4: f64 = (eq46_e415_d_n4 * (nv14 - 0.0));
        let eq46_e417_d_n5: f64 = (eq46_e415_d_n5 * (nv14 - 0.0));
        let eq46_e417_d_n6: f64 = (eq46_e415_d_n6 * (nv14 - 0.0));
        let eq46_e417_d_n7: f64 = (eq46_e415_d_n7 * (nv14 - 0.0));
        let eq46_e417_d_n8: f64 = (eq46_e415_d_n8 * (nv14 - 0.0));
        let eq46_e417_d_n9: f64 = (eq46_e415_d_n9 * (nv14 - 0.0));
        let eq46_e417_d_n10: f64 = (eq46_e415_d_n10 * (nv14 - 0.0));
        let eq46_e417_d_n11: f64 = (eq46_e415_d_n11 * (nv14 - 0.0));
        let eq46_e417_d_n12: f64 = (eq46_e415_d_n12 * (nv14 - 0.0));
        let eq46_e417_d_n13: f64 = (eq46_e415_d_n13 * (nv14 - 0.0));
        let eq46_e417_d_n14: f64 = ((eq46_e415_d_n14 * (nv14 - 0.0)) + eq46_e415);
        let eq46_e417_d_n15: f64 = (eq46_e415_d_n15 * (nv14 - 0.0));
        let eq46_e417_d_b0: f64 = (eq46_e415_d_b0 * (nv14 - 0.0));
        let eq46_e417_d_b1: f64 = (eq46_e415_d_b1 * (nv14 - 0.0));
        let eq46_e417_d_b2: f64 = (eq46_e415_d_b2 * (nv14 - 0.0));
        let eq46_e417_d_b3: f64 = (eq46_e415_d_b3 * (nv14 - 0.0));
        let eq46_e417_d_b4: f64 = (eq46_e415_d_b4 * (nv14 - 0.0));
        let eq46_e417_d_b5: f64 = (eq46_e415_d_b5 * (nv14 - 0.0));
        let eq46_e417_d_b6: f64 = (eq46_e415_d_b6 * (nv14 - 0.0));
        let eq46_e417_d_b7: f64 = (eq46_e415_d_b7 * (nv14 - 0.0));
        let eq46_e417_d_b8: f64 = (eq46_e415_d_b8 * (nv14 - 0.0));
        let eq46_e417_d_b9: f64 = (eq46_e415_d_b9 * (nv14 - 0.0));
        let eq46_e417_d_b10: f64 = (eq46_e415_d_b10 * (nv14 - 0.0));
        let eq46_e417_d_b11: f64 = (eq46_e415_d_b11 * (nv14 - 0.0));
        let eq46_e417_d_b12: f64 = (eq46_e415_d_b12 * (nv14 - 0.0));
        let eq46_e417_d_b13: f64 = (eq46_e415_d_b13 * (nv14 - 0.0));
        let eq46_e417_d_b14: f64 = (eq46_e415_d_b14 * (nv14 - 0.0));
        let eq46_e418: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq46_e417);
        let eq46_e418_d_n0: f64 = (eq46_e417_d_n0 * ddt_scale);
        let eq46_e418_d_n1: f64 = (eq46_e417_d_n1 * ddt_scale);
        let eq46_e418_d_n2: f64 = (eq46_e417_d_n2 * ddt_scale);
        let eq46_e418_d_n3: f64 = (eq46_e417_d_n3 * ddt_scale);
        let eq46_e418_d_n4: f64 = (eq46_e417_d_n4 * ddt_scale);
        let eq46_e418_d_n5: f64 = (eq46_e417_d_n5 * ddt_scale);
        let eq46_e418_d_n6: f64 = (eq46_e417_d_n6 * ddt_scale);
        let eq46_e418_d_n7: f64 = (eq46_e417_d_n7 * ddt_scale);
        let eq46_e418_d_n8: f64 = (eq46_e417_d_n8 * ddt_scale);
        let eq46_e418_d_n9: f64 = (eq46_e417_d_n9 * ddt_scale);
        let eq46_e418_d_n10: f64 = (eq46_e417_d_n10 * ddt_scale);
        let eq46_e418_d_n11: f64 = (eq46_e417_d_n11 * ddt_scale);
        let eq46_e418_d_n12: f64 = (eq46_e417_d_n12 * ddt_scale);
        let eq46_e418_d_n13: f64 = (eq46_e417_d_n13 * ddt_scale);
        let eq46_e418_d_n14: f64 = (eq46_e417_d_n14 * ddt_scale);
        let eq46_e418_d_n15: f64 = (eq46_e417_d_n15 * ddt_scale);
        let eq46_e418_d_b0: f64 = (eq46_e417_d_b0 * ddt_scale);
        let eq46_e418_d_b1: f64 = (eq46_e417_d_b1 * ddt_scale);
        let eq46_e418_d_b2: f64 = (eq46_e417_d_b2 * ddt_scale);
        let eq46_e418_d_b3: f64 = (eq46_e417_d_b3 * ddt_scale);
        let eq46_e418_d_b4: f64 = (eq46_e417_d_b4 * ddt_scale);
        let eq46_e418_d_b5: f64 = (eq46_e417_d_b5 * ddt_scale);
        let eq46_e418_d_b6: f64 = (eq46_e417_d_b6 * ddt_scale);
        let eq46_e418_d_b7: f64 = (eq46_e417_d_b7 * ddt_scale);
        let eq46_e418_d_b8: f64 = (eq46_e417_d_b8 * ddt_scale);
        let eq46_e418_d_b9: f64 = (eq46_e417_d_b9 * ddt_scale);
        let eq46_e418_d_b10: f64 = (eq46_e417_d_b10 * ddt_scale);
        let eq46_e418_d_b11: f64 = (eq46_e417_d_b11 * ddt_scale);
        let eq46_e418_d_b12: f64 = (eq46_e417_d_b12 * ddt_scale);
        let eq46_e418_d_b13: f64 = (eq46_e417_d_b13 * ddt_scale);
        let eq46_e418_d_b14: f64 = (eq46_e417_d_b14 * ddt_scale);
        (eq46_e418, eq46_e418_d_n0, eq46_e418_d_n1, eq46_e418_d_n2, eq46_e418_d_n3, eq46_e418_d_n4, eq46_e418_d_n5, eq46_e418_d_n6, eq46_e418_d_n7, eq46_e418_d_n8, eq46_e418_d_n9, eq46_e418_d_n10, eq46_e418_d_n11, eq46_e418_d_n12, eq46_e418_d_n13, eq46_e418_d_n14, eq46_e418_d_n15, eq46_e418_d_b0, eq46_e418_d_b1, eq46_e418_d_b2, eq46_e418_d_b3, eq46_e418_d_b4, eq46_e418_d_b5, eq46_e418_d_b6, eq46_e418_d_b7, eq46_e418_d_b8, eq46_e418_d_b9, eq46_e418_d_b10, eq46_e418_d_b11, eq46_e418_d_b12, eq46_e418_d_b13, eq46_e418_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e420;
        let eq46_node_derivatives: [f64; 16] = [eq46_e420_d_n0, eq46_e420_d_n1, eq46_e420_d_n2, eq46_e420_d_n3, eq46_e420_d_n4, eq46_e420_d_n5, eq46_e420_d_n6, eq46_e420_d_n7, eq46_e420_d_n8, eq46_e420_d_n9, eq46_e420_d_n10, eq46_e420_d_n11, eq46_e420_d_n12, eq46_e420_d_n13, eq46_e420_d_n14, eq46_e420_d_n15];
        let eq46_branch_derivatives: [f64; 15] = [eq46_e420_d_b0, eq46_e420_d_b1, eq46_e420_d_b2, eq46_e420_d_b3, eq46_e420_d_b4, eq46_e420_d_b5, eq46_e420_d_b6, eq46_e420_d_b7, eq46_e420_d_b8, eq46_e420_d_b9, eq46_e420_d_b10, eq46_e420_d_b11, eq46_e420_d_b12, eq46_e420_d_b13, eq46_e420_d_b14];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(3),
            multiplicity * (eq46_value),
            &eq46_node_derivatives,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let (eq57_e532, eq57_e532_d_n11,) = {
    if s.b[124] {
        let eq57_e529: f64 = (p.p58 * (nv11 - 0.0));
        let eq57_e529_d_n11: f64 = p.p58;
        let eq57_e530: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, eq57_e529);
        let eq57_e530_d_n11: f64 = (eq57_e529_d_n11 * ddt_scale);
        (eq57_e530, eq57_e530_d_n11,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e532;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (eq57_value),
            11,
            multiplicity * (eq57_e532_d_n11),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi5 = ctx.branch_current(branches[5]);
        let bi13 = ctx.branch_current(branches[13]);
        let bi14 = ctx.branch_current(branches[14]);
        let eq3_e99: f64 = (p.p51 / 3.0);
        let eq3_e101: f64 = (eq3_e99 * bi0);
        let eq3_e102_q: f64 = eq3_e101;
        stamper.stamp_potential_reactive_branch1(
            branches[0],
            branches[0],
            eq3_e99,
        );
        let (eq7_e110, eq7_e110_d_n0, eq7_e110_d_n1, eq7_e110_d_n2, eq7_e110_d_n3, eq7_e110_d_n4, eq7_e110_d_n5, eq7_e110_d_n6, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n9, eq7_e110_d_n10, eq7_e110_d_n11, eq7_e110_d_n12, eq7_e110_d_n13, eq7_e110_d_n14, eq7_e110_d_n15, eq7_e110_d_b0, eq7_e110_d_b1, eq7_e110_d_b2, eq7_e110_d_b3, eq7_e110_d_b4, eq7_e110_d_b5, eq7_e110_d_b6, eq7_e110_d_b7, eq7_e110_d_b8, eq7_e110_d_b9, eq7_e110_d_b10, eq7_e110_d_b11, eq7_e110_d_b12, eq7_e110_d_b13, eq7_e110_d_b14, eq7_e110_q, eq7_e110_q_d_n0, eq7_e110_q_d_n1, eq7_e110_q_d_n2, eq7_e110_q_d_n3, eq7_e110_q_d_n4, eq7_e110_q_d_n5, eq7_e110_q_d_n6, eq7_e110_q_d_n7, eq7_e110_q_d_n8, eq7_e110_q_d_n9, eq7_e110_q_d_n10, eq7_e110_q_d_n11, eq7_e110_q_d_n12, eq7_e110_q_d_n13, eq7_e110_q_d_n14, eq7_e110_q_d_n15, eq7_e110_q_d_b0, eq7_e110_q_d_b1, eq7_e110_q_d_b2, eq7_e110_q_d_b3, eq7_e110_q_d_b4, eq7_e110_q_d_b5, eq7_e110_q_d_b6, eq7_e110_q_d_b7, eq7_e110_q_d_b8, eq7_e110_q_d_b9, eq7_e110_q_d_b10, eq7_e110_q_d_b11, eq7_e110_q_d_b12, eq7_e110_q_d_b13, eq7_e110_q_d_b14,) = {
    if s.b[97] {
        let eq7_e108_q: f64 = s.v[21];
        (s.v[21], s.dn[21][0], s.dn[21][1], s.dn[21][2], s.dn[21][3], s.dn[21][4], s.dn[21][5], s.dn[21][6], s.dn[21][7], s.dn[21][8], s.dn[21][9], s.dn[21][10], s.dn[21][11], s.dn[21][12], s.dn[21][13], s.dn[21][14], s.dn[21][15], s.db[21][0], s.db[21][1], s.db[21][2], s.db[21][3], s.db[21][4], s.db[21][5], s.db[21][6], s.db[21][7], s.db[21][8], s.db[21][9], s.db[21][10], s.db[21][11], s.db[21][12], s.db[21][13], s.db[21][14], eq7_e108_q, s.dn[21][0], s.dn[21][1], s.dn[21][2], s.dn[21][3], s.dn[21][4], s.dn[21][5], s.dn[21][6], s.dn[21][7], s.dn[21][8], s.dn[21][9], s.dn[21][10], s.dn[21][11], s.dn[21][12], s.dn[21][13], s.dn[21][14], s.dn[21][15], s.db[21][0], s.db[21][1], s.db[21][2], s.db[21][3], s.db[21][4], s.db[21][5], s.db[21][6], s.db[21][7], s.db[21][8], s.db[21][9], s.db[21][10], s.db[21][11], s.db[21][12], s.db[21][13], s.db[21][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 16] = [eq7_e110_q_d_n0, eq7_e110_q_d_n1, eq7_e110_q_d_n2, eq7_e110_q_d_n3, eq7_e110_q_d_n4, eq7_e110_q_d_n5, eq7_e110_q_d_n6, eq7_e110_q_d_n7, eq7_e110_q_d_n8, eq7_e110_q_d_n9, eq7_e110_q_d_n10, eq7_e110_q_d_n11, eq7_e110_q_d_n12, eq7_e110_q_d_n13, eq7_e110_q_d_n14, eq7_e110_q_d_n15];
        let eq7_reactive_branch_derivatives: [f64; 15] = [eq7_e110_q_d_b0, eq7_e110_q_d_b1, eq7_e110_q_d_b2, eq7_e110_q_d_b3, eq7_e110_q_d_b4, eq7_e110_q_d_b5, eq7_e110_q_d_b6, eq7_e110_q_d_b7, eq7_e110_q_d_b8, eq7_e110_q_d_b9, eq7_e110_q_d_b10, eq7_e110_q_d_b11, eq7_e110_q_d_b12, eq7_e110_q_d_b13, eq7_e110_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e115, eq8_e115_d_n0, eq8_e115_d_n1, eq8_e115_d_n2, eq8_e115_d_n3, eq8_e115_d_n4, eq8_e115_d_n5, eq8_e115_d_n6, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n9, eq8_e115_d_n10, eq8_e115_d_n11, eq8_e115_d_n12, eq8_e115_d_n13, eq8_e115_d_n14, eq8_e115_d_n15, eq8_e115_d_b0, eq8_e115_d_b1, eq8_e115_d_b2, eq8_e115_d_b3, eq8_e115_d_b4, eq8_e115_d_b5, eq8_e115_d_b6, eq8_e115_d_b7, eq8_e115_d_b8, eq8_e115_d_b9, eq8_e115_d_b10, eq8_e115_d_b11, eq8_e115_d_b12, eq8_e115_d_b13, eq8_e115_d_b14, eq8_e115_q, eq8_e115_q_d_n0, eq8_e115_q_d_n1, eq8_e115_q_d_n2, eq8_e115_q_d_n3, eq8_e115_q_d_n4, eq8_e115_q_d_n5, eq8_e115_q_d_n6, eq8_e115_q_d_n7, eq8_e115_q_d_n8, eq8_e115_q_d_n9, eq8_e115_q_d_n10, eq8_e115_q_d_n11, eq8_e115_q_d_n12, eq8_e115_q_d_n13, eq8_e115_q_d_n14, eq8_e115_q_d_n15, eq8_e115_q_d_b0, eq8_e115_q_d_b1, eq8_e115_q_d_b2, eq8_e115_q_d_b3, eq8_e115_q_d_b4, eq8_e115_q_d_b5, eq8_e115_q_d_b6, eq8_e115_q_d_b7, eq8_e115_q_d_b8, eq8_e115_q_d_b9, eq8_e115_q_d_b10, eq8_e115_q_d_b11, eq8_e115_q_d_b12, eq8_e115_q_d_b13, eq8_e115_q_d_b14,) = {
    if s.b[97] {
        let eq8_e113_q: f64 = s.v[20];
        (s.v[20], s.dn[20][0], s.dn[20][1], s.dn[20][2], s.dn[20][3], s.dn[20][4], s.dn[20][5], s.dn[20][6], s.dn[20][7], s.dn[20][8], s.dn[20][9], s.dn[20][10], s.dn[20][11], s.dn[20][12], s.dn[20][13], s.dn[20][14], s.dn[20][15], s.db[20][0], s.db[20][1], s.db[20][2], s.db[20][3], s.db[20][4], s.db[20][5], s.db[20][6], s.db[20][7], s.db[20][8], s.db[20][9], s.db[20][10], s.db[20][11], s.db[20][12], s.db[20][13], s.db[20][14], eq8_e113_q, s.dn[20][0], s.dn[20][1], s.dn[20][2], s.dn[20][3], s.dn[20][4], s.dn[20][5], s.dn[20][6], s.dn[20][7], s.dn[20][8], s.dn[20][9], s.dn[20][10], s.dn[20][11], s.dn[20][12], s.dn[20][13], s.dn[20][14], s.dn[20][15], s.db[20][0], s.db[20][1], s.db[20][2], s.db[20][3], s.db[20][4], s.db[20][5], s.db[20][6], s.db[20][7], s.db[20][8], s.db[20][9], s.db[20][10], s.db[20][11], s.db[20][12], s.db[20][13], s.db[20][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 16] = [eq8_e115_q_d_n0, eq8_e115_q_d_n1, eq8_e115_q_d_n2, eq8_e115_q_d_n3, eq8_e115_q_d_n4, eq8_e115_q_d_n5, eq8_e115_q_d_n6, eq8_e115_q_d_n7, eq8_e115_q_d_n8, eq8_e115_q_d_n9, eq8_e115_q_d_n10, eq8_e115_q_d_n11, eq8_e115_q_d_n12, eq8_e115_q_d_n13, eq8_e115_q_d_n14, eq8_e115_q_d_n15];
        let eq8_reactive_branch_derivatives: [f64; 15] = [eq8_e115_q_d_b0, eq8_e115_q_d_b1, eq8_e115_q_d_b2, eq8_e115_q_d_b3, eq8_e115_q_d_b4, eq8_e115_q_d_b5, eq8_e115_q_d_b6, eq8_e115_q_d_b7, eq8_e115_q_d_b8, eq8_e115_q_d_b9, eq8_e115_q_d_b10, eq8_e115_q_d_b11, eq8_e115_q_d_b12, eq8_e115_q_d_b13, eq8_e115_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq9_e123, eq9_e123_d_n0, eq9_e123_d_n1, eq9_e123_d_n2, eq9_e123_d_n3, eq9_e123_d_n4, eq9_e123_d_n5, eq9_e123_d_n6, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n9, eq9_e123_d_n10, eq9_e123_d_n11, eq9_e123_d_n12, eq9_e123_d_n13, eq9_e123_d_n14, eq9_e123_d_n15, eq9_e123_d_b0, eq9_e123_d_b1, eq9_e123_d_b2, eq9_e123_d_b3, eq9_e123_d_b4, eq9_e123_d_b5, eq9_e123_d_b6, eq9_e123_d_b7, eq9_e123_d_b8, eq9_e123_d_b9, eq9_e123_d_b10, eq9_e123_d_b11, eq9_e123_d_b12, eq9_e123_d_b13, eq9_e123_d_b14, eq9_e123_q, eq9_e123_q_d_n0, eq9_e123_q_d_n1, eq9_e123_q_d_n2, eq9_e123_q_d_n3, eq9_e123_q_d_n4, eq9_e123_q_d_n5, eq9_e123_q_d_n6, eq9_e123_q_d_n7, eq9_e123_q_d_n8, eq9_e123_q_d_n9, eq9_e123_q_d_n10, eq9_e123_q_d_n11, eq9_e123_q_d_n12, eq9_e123_q_d_n13, eq9_e123_q_d_n14, eq9_e123_q_d_n15, eq9_e123_q_d_b0, eq9_e123_q_d_b1, eq9_e123_q_d_b2, eq9_e123_q_d_b3, eq9_e123_q_d_b4, eq9_e123_q_d_b5, eq9_e123_q_d_b6, eq9_e123_q_d_b7, eq9_e123_q_d_b8, eq9_e123_q_d_b9, eq9_e123_q_d_b10, eq9_e123_q_d_b11, eq9_e123_q_d_b12, eq9_e123_q_d_b13, eq9_e123_q_d_b14,) = {
    if (!s.b[97]) {
        let eq9_e120: f64 = (s.v[19] * s.v[80]);
        let eq9_e120_d_n0: f64 = ((s.dn[19][0] * s.v[80]) + (s.v[19] * s.dn[80][0]));
        let eq9_e120_d_n1: f64 = ((s.dn[19][1] * s.v[80]) + (s.v[19] * s.dn[80][1]));
        let eq9_e120_d_n2: f64 = ((s.dn[19][2] * s.v[80]) + (s.v[19] * s.dn[80][2]));
        let eq9_e120_d_n3: f64 = ((s.dn[19][3] * s.v[80]) + (s.v[19] * s.dn[80][3]));
        let eq9_e120_d_n4: f64 = ((s.dn[19][4] * s.v[80]) + (s.v[19] * s.dn[80][4]));
        let eq9_e120_d_n5: f64 = ((s.dn[19][5] * s.v[80]) + (s.v[19] * s.dn[80][5]));
        let eq9_e120_d_n6: f64 = ((s.dn[19][6] * s.v[80]) + (s.v[19] * s.dn[80][6]));
        let eq9_e120_d_n7: f64 = ((s.dn[19][7] * s.v[80]) + (s.v[19] * s.dn[80][7]));
        let eq9_e120_d_n8: f64 = ((s.dn[19][8] * s.v[80]) + (s.v[19] * s.dn[80][8]));
        let eq9_e120_d_n9: f64 = ((s.dn[19][9] * s.v[80]) + (s.v[19] * s.dn[80][9]));
        let eq9_e120_d_n10: f64 = ((s.dn[19][10] * s.v[80]) + (s.v[19] * s.dn[80][10]));
        let eq9_e120_d_n11: f64 = ((s.dn[19][11] * s.v[80]) + (s.v[19] * s.dn[80][11]));
        let eq9_e120_d_n12: f64 = ((s.dn[19][12] * s.v[80]) + (s.v[19] * s.dn[80][12]));
        let eq9_e120_d_n13: f64 = ((s.dn[19][13] * s.v[80]) + (s.v[19] * s.dn[80][13]));
        let eq9_e120_d_n14: f64 = ((s.dn[19][14] * s.v[80]) + (s.v[19] * s.dn[80][14]));
        let eq9_e120_d_n15: f64 = ((s.dn[19][15] * s.v[80]) + (s.v[19] * s.dn[80][15]));
        let eq9_e120_d_b0: f64 = ((s.db[19][0] * s.v[80]) + (s.v[19] * s.db[80][0]));
        let eq9_e120_d_b1: f64 = ((s.db[19][1] * s.v[80]) + (s.v[19] * s.db[80][1]));
        let eq9_e120_d_b2: f64 = ((s.db[19][2] * s.v[80]) + (s.v[19] * s.db[80][2]));
        let eq9_e120_d_b3: f64 = ((s.db[19][3] * s.v[80]) + (s.v[19] * s.db[80][3]));
        let eq9_e120_d_b4: f64 = ((s.db[19][4] * s.v[80]) + (s.v[19] * s.db[80][4]));
        let eq9_e120_d_b5: f64 = ((s.db[19][5] * s.v[80]) + (s.v[19] * s.db[80][5]));
        let eq9_e120_d_b6: f64 = ((s.db[19][6] * s.v[80]) + (s.v[19] * s.db[80][6]));
        let eq9_e120_d_b7: f64 = ((s.db[19][7] * s.v[80]) + (s.v[19] * s.db[80][7]));
        let eq9_e120_d_b8: f64 = ((s.db[19][8] * s.v[80]) + (s.v[19] * s.db[80][8]));
        let eq9_e120_d_b9: f64 = ((s.db[19][9] * s.v[80]) + (s.v[19] * s.db[80][9]));
        let eq9_e120_d_b10: f64 = ((s.db[19][10] * s.v[80]) + (s.v[19] * s.db[80][10]));
        let eq9_e120_d_b11: f64 = ((s.db[19][11] * s.v[80]) + (s.v[19] * s.db[80][11]));
        let eq9_e120_d_b12: f64 = ((s.db[19][12] * s.v[80]) + (s.v[19] * s.db[80][12]));
        let eq9_e120_d_b13: f64 = ((s.db[19][13] * s.v[80]) + (s.v[19] * s.db[80][13]));
        let eq9_e120_d_b14: f64 = ((s.db[19][14] * s.v[80]) + (s.v[19] * s.db[80][14]));
        let eq9_e121_q: f64 = eq9_e120;
        (eq9_e120, eq9_e120_d_n0, eq9_e120_d_n1, eq9_e120_d_n2, eq9_e120_d_n3, eq9_e120_d_n4, eq9_e120_d_n5, eq9_e120_d_n6, eq9_e120_d_n7, eq9_e120_d_n8, eq9_e120_d_n9, eq9_e120_d_n10, eq9_e120_d_n11, eq9_e120_d_n12, eq9_e120_d_n13, eq9_e120_d_n14, eq9_e120_d_n15, eq9_e120_d_b0, eq9_e120_d_b1, eq9_e120_d_b2, eq9_e120_d_b3, eq9_e120_d_b4, eq9_e120_d_b5, eq9_e120_d_b6, eq9_e120_d_b7, eq9_e120_d_b8, eq9_e120_d_b9, eq9_e120_d_b10, eq9_e120_d_b11, eq9_e120_d_b12, eq9_e120_d_b13, eq9_e120_d_b14, eq9_e121_q, eq9_e120_d_n0, eq9_e120_d_n1, eq9_e120_d_n2, eq9_e120_d_n3, eq9_e120_d_n4, eq9_e120_d_n5, eq9_e120_d_n6, eq9_e120_d_n7, eq9_e120_d_n8, eq9_e120_d_n9, eq9_e120_d_n10, eq9_e120_d_n11, eq9_e120_d_n12, eq9_e120_d_n13, eq9_e120_d_n14, eq9_e120_d_n15, eq9_e120_d_b0, eq9_e120_d_b1, eq9_e120_d_b2, eq9_e120_d_b3, eq9_e120_d_b4, eq9_e120_d_b5, eq9_e120_d_b6, eq9_e120_d_b7, eq9_e120_d_b8, eq9_e120_d_b9, eq9_e120_d_b10, eq9_e120_d_b11, eq9_e120_d_b12, eq9_e120_d_b13, eq9_e120_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_reactive_node_derivatives: [f64; 16] = [eq9_e123_q_d_n0, eq9_e123_q_d_n1, eq9_e123_q_d_n2, eq9_e123_q_d_n3, eq9_e123_q_d_n4, eq9_e123_q_d_n5, eq9_e123_q_d_n6, eq9_e123_q_d_n7, eq9_e123_q_d_n8, eq9_e123_q_d_n9, eq9_e123_q_d_n10, eq9_e123_q_d_n11, eq9_e123_q_d_n12, eq9_e123_q_d_n13, eq9_e123_q_d_n14, eq9_e123_q_d_n15];
        let eq9_reactive_branch_derivatives: [f64; 15] = [eq9_e123_q_d_b0, eq9_e123_q_d_b1, eq9_e123_q_d_b2, eq9_e123_q_d_b3, eq9_e123_q_d_b4, eq9_e123_q_d_b5, eq9_e123_q_d_b6, eq9_e123_q_d_b7, eq9_e123_q_d_b8, eq9_e123_q_d_b9, eq9_e123_q_d_b10, eq9_e123_q_d_b11, eq9_e123_q_d_b12, eq9_e123_q_d_b13, eq9_e123_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq9_reactive_node_derivatives,
            branches,
            &eq9_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e131, eq10_e131_d_n0, eq10_e131_d_n1, eq10_e131_d_n2, eq10_e131_d_n3, eq10_e131_d_n4, eq10_e131_d_n5, eq10_e131_d_n6, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n9, eq10_e131_d_n10, eq10_e131_d_n11, eq10_e131_d_n12, eq10_e131_d_n13, eq10_e131_d_n14, eq10_e131_d_n15, eq10_e131_d_b0, eq10_e131_d_b1, eq10_e131_d_b2, eq10_e131_d_b3, eq10_e131_d_b4, eq10_e131_d_b5, eq10_e131_d_b6, eq10_e131_d_b7, eq10_e131_d_b8, eq10_e131_d_b9, eq10_e131_d_b10, eq10_e131_d_b11, eq10_e131_d_b12, eq10_e131_d_b13, eq10_e131_d_b14, eq10_e131_q, eq10_e131_q_d_n0, eq10_e131_q_d_n1, eq10_e131_q_d_n2, eq10_e131_q_d_n3, eq10_e131_q_d_n4, eq10_e131_q_d_n5, eq10_e131_q_d_n6, eq10_e131_q_d_n7, eq10_e131_q_d_n8, eq10_e131_q_d_n9, eq10_e131_q_d_n10, eq10_e131_q_d_n11, eq10_e131_q_d_n12, eq10_e131_q_d_n13, eq10_e131_q_d_n14, eq10_e131_q_d_n15, eq10_e131_q_d_b0, eq10_e131_q_d_b1, eq10_e131_q_d_b2, eq10_e131_q_d_b3, eq10_e131_q_d_b4, eq10_e131_q_d_b5, eq10_e131_q_d_b6, eq10_e131_q_d_b7, eq10_e131_q_d_b8, eq10_e131_q_d_b9, eq10_e131_q_d_b10, eq10_e131_q_d_b11, eq10_e131_q_d_b12, eq10_e131_q_d_b13, eq10_e131_q_d_b14,) = {
    if (!s.b[97]) {
        let eq10_e128: f64 = (s.v[18] * s.v[79]);
        let eq10_e128_d_n0: f64 = ((s.dn[18][0] * s.v[79]) + (s.v[18] * s.dn[79][0]));
        let eq10_e128_d_n1: f64 = ((s.dn[18][1] * s.v[79]) + (s.v[18] * s.dn[79][1]));
        let eq10_e128_d_n2: f64 = ((s.dn[18][2] * s.v[79]) + (s.v[18] * s.dn[79][2]));
        let eq10_e128_d_n3: f64 = ((s.dn[18][3] * s.v[79]) + (s.v[18] * s.dn[79][3]));
        let eq10_e128_d_n4: f64 = ((s.dn[18][4] * s.v[79]) + (s.v[18] * s.dn[79][4]));
        let eq10_e128_d_n5: f64 = ((s.dn[18][5] * s.v[79]) + (s.v[18] * s.dn[79][5]));
        let eq10_e128_d_n6: f64 = ((s.dn[18][6] * s.v[79]) + (s.v[18] * s.dn[79][6]));
        let eq10_e128_d_n7: f64 = ((s.dn[18][7] * s.v[79]) + (s.v[18] * s.dn[79][7]));
        let eq10_e128_d_n8: f64 = ((s.dn[18][8] * s.v[79]) + (s.v[18] * s.dn[79][8]));
        let eq10_e128_d_n9: f64 = ((s.dn[18][9] * s.v[79]) + (s.v[18] * s.dn[79][9]));
        let eq10_e128_d_n10: f64 = ((s.dn[18][10] * s.v[79]) + (s.v[18] * s.dn[79][10]));
        let eq10_e128_d_n11: f64 = ((s.dn[18][11] * s.v[79]) + (s.v[18] * s.dn[79][11]));
        let eq10_e128_d_n12: f64 = ((s.dn[18][12] * s.v[79]) + (s.v[18] * s.dn[79][12]));
        let eq10_e128_d_n13: f64 = ((s.dn[18][13] * s.v[79]) + (s.v[18] * s.dn[79][13]));
        let eq10_e128_d_n14: f64 = ((s.dn[18][14] * s.v[79]) + (s.v[18] * s.dn[79][14]));
        let eq10_e128_d_n15: f64 = ((s.dn[18][15] * s.v[79]) + (s.v[18] * s.dn[79][15]));
        let eq10_e128_d_b0: f64 = ((s.db[18][0] * s.v[79]) + (s.v[18] * s.db[79][0]));
        let eq10_e128_d_b1: f64 = ((s.db[18][1] * s.v[79]) + (s.v[18] * s.db[79][1]));
        let eq10_e128_d_b2: f64 = ((s.db[18][2] * s.v[79]) + (s.v[18] * s.db[79][2]));
        let eq10_e128_d_b3: f64 = ((s.db[18][3] * s.v[79]) + (s.v[18] * s.db[79][3]));
        let eq10_e128_d_b4: f64 = ((s.db[18][4] * s.v[79]) + (s.v[18] * s.db[79][4]));
        let eq10_e128_d_b5: f64 = ((s.db[18][5] * s.v[79]) + (s.v[18] * s.db[79][5]));
        let eq10_e128_d_b6: f64 = ((s.db[18][6] * s.v[79]) + (s.v[18] * s.db[79][6]));
        let eq10_e128_d_b7: f64 = ((s.db[18][7] * s.v[79]) + (s.v[18] * s.db[79][7]));
        let eq10_e128_d_b8: f64 = ((s.db[18][8] * s.v[79]) + (s.v[18] * s.db[79][8]));
        let eq10_e128_d_b9: f64 = ((s.db[18][9] * s.v[79]) + (s.v[18] * s.db[79][9]));
        let eq10_e128_d_b10: f64 = ((s.db[18][10] * s.v[79]) + (s.v[18] * s.db[79][10]));
        let eq10_e128_d_b11: f64 = ((s.db[18][11] * s.v[79]) + (s.v[18] * s.db[79][11]));
        let eq10_e128_d_b12: f64 = ((s.db[18][12] * s.v[79]) + (s.v[18] * s.db[79][12]));
        let eq10_e128_d_b13: f64 = ((s.db[18][13] * s.v[79]) + (s.v[18] * s.db[79][13]));
        let eq10_e128_d_b14: f64 = ((s.db[18][14] * s.v[79]) + (s.v[18] * s.db[79][14]));
        let eq10_e129_q: f64 = eq10_e128;
        (eq10_e128, eq10_e128_d_n0, eq10_e128_d_n1, eq10_e128_d_n2, eq10_e128_d_n3, eq10_e128_d_n4, eq10_e128_d_n5, eq10_e128_d_n6, eq10_e128_d_n7, eq10_e128_d_n8, eq10_e128_d_n9, eq10_e128_d_n10, eq10_e128_d_n11, eq10_e128_d_n12, eq10_e128_d_n13, eq10_e128_d_n14, eq10_e128_d_n15, eq10_e128_d_b0, eq10_e128_d_b1, eq10_e128_d_b2, eq10_e128_d_b3, eq10_e128_d_b4, eq10_e128_d_b5, eq10_e128_d_b6, eq10_e128_d_b7, eq10_e128_d_b8, eq10_e128_d_b9, eq10_e128_d_b10, eq10_e128_d_b11, eq10_e128_d_b12, eq10_e128_d_b13, eq10_e128_d_b14, eq10_e129_q, eq10_e128_d_n0, eq10_e128_d_n1, eq10_e128_d_n2, eq10_e128_d_n3, eq10_e128_d_n4, eq10_e128_d_n5, eq10_e128_d_n6, eq10_e128_d_n7, eq10_e128_d_n8, eq10_e128_d_n9, eq10_e128_d_n10, eq10_e128_d_n11, eq10_e128_d_n12, eq10_e128_d_n13, eq10_e128_d_n14, eq10_e128_d_n15, eq10_e128_d_b0, eq10_e128_d_b1, eq10_e128_d_b2, eq10_e128_d_b3, eq10_e128_d_b4, eq10_e128_d_b5, eq10_e128_d_b6, eq10_e128_d_b7, eq10_e128_d_b8, eq10_e128_d_b9, eq10_e128_d_b10, eq10_e128_d_b11, eq10_e128_d_b12, eq10_e128_d_b13, eq10_e128_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 16] = [eq10_e131_q_d_n0, eq10_e131_q_d_n1, eq10_e131_q_d_n2, eq10_e131_q_d_n3, eq10_e131_q_d_n4, eq10_e131_q_d_n5, eq10_e131_q_d_n6, eq10_e131_q_d_n7, eq10_e131_q_d_n8, eq10_e131_q_d_n9, eq10_e131_q_d_n10, eq10_e131_q_d_n11, eq10_e131_q_d_n12, eq10_e131_q_d_n13, eq10_e131_q_d_n14, eq10_e131_q_d_n15];
        let eq10_reactive_branch_derivatives: [f64; 15] = [eq10_e131_q_d_b0, eq10_e131_q_d_b1, eq10_e131_q_d_b2, eq10_e131_q_d_b3, eq10_e131_q_d_b4, eq10_e131_q_d_b5, eq10_e131_q_d_b6, eq10_e131_q_d_b7, eq10_e131_q_d_b8, eq10_e131_q_d_b9, eq10_e131_q_d_b10, eq10_e131_q_d_b11, eq10_e131_q_d_b12, eq10_e131_q_d_b13, eq10_e131_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq25_e218, eq25_e218_d_b5, eq25_e218_q, eq25_e218_q_d_b5,) = {
    if s.b[102] {
        let eq25_e215: f64 = (p.p50 * bi5);
        let eq25_e215_d_b5: f64 = p.p50;
        let eq25_e216_q: f64 = eq25_e215;
        (eq25_e215, eq25_e215_d_b5, eq25_e216_q, eq25_e215_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[6],
            branches[5],
            eq25_e218_q_d_b5,
        );
        let (eq27_e242, eq27_e242_d_b5, eq27_e242_q, eq27_e242_q_d_b5,) = {
    if ((!s.b[102]) && s.b[103]) {
        let eq27_e239: f64 = (p.p50 * bi5);
        let eq27_e239_d_b5: f64 = p.p50;
        let eq27_e240_q: f64 = eq27_e239;
        (eq27_e239, eq27_e239_d_b5, eq27_e240_q, eq27_e239_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[8],
            branches[5],
            eq27_e242_q_d_b5,
        );
        let eq32_e278: f64 = (p.p49 * bi13);
        let eq32_e278_d_b13: f64 = p.p49;
        let eq32_e279_q: f64 = eq32_e278;
        stamper.stamp_potential_reactive_branch1(
            branches[13],
            branches[13],
            eq32_e278_d_b13,
        );
        let (eq34_e292, eq34_e292_d_b14, eq34_e292_q, eq34_e292_q_d_b14,) = {
    if s.b[105] {
        let eq34_e289: f64 = (p.p48 * bi14);
        let eq34_e289_d_b14: f64 = p.p48;
        let eq34_e290_q: f64 = eq34_e289;
        (eq34_e289, eq34_e289_d_b14, eq34_e290_q, eq34_e289_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[15],
            branches[14],
            eq34_e292_q_d_b14,
        );
        let (eq36_e316, eq36_e316_d_b14, eq36_e316_q, eq36_e316_q_d_b14,) = {
    if ((!s.b[105]) && s.b[106]) {
        let eq36_e313: f64 = (p.p48 * bi14);
        let eq36_e313_d_b14: f64 = p.p48;
        let eq36_e314_q: f64 = eq36_e313;
        (eq36_e313, eq36_e313_d_b14, eq36_e314_q, eq36_e313_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[17],
            branches[14],
            eq36_e316_q_d_b14,
        );
        let (eq46_e420, eq46_e420_d_n0, eq46_e420_d_n1, eq46_e420_d_n2, eq46_e420_d_n3, eq46_e420_d_n4, eq46_e420_d_n5, eq46_e420_d_n6, eq46_e420_d_n7, eq46_e420_d_n8, eq46_e420_d_n9, eq46_e420_d_n10, eq46_e420_d_n11, eq46_e420_d_n12, eq46_e420_d_n13, eq46_e420_d_n14, eq46_e420_d_n15, eq46_e420_d_b0, eq46_e420_d_b1, eq46_e420_d_b2, eq46_e420_d_b3, eq46_e420_d_b4, eq46_e420_d_b5, eq46_e420_d_b6, eq46_e420_d_b7, eq46_e420_d_b8, eq46_e420_d_b9, eq46_e420_d_b10, eq46_e420_d_b11, eq46_e420_d_b12, eq46_e420_d_b13, eq46_e420_d_b14, eq46_e420_q, eq46_e420_q_d_n0, eq46_e420_q_d_n1, eq46_e420_q_d_n2, eq46_e420_q_d_n3, eq46_e420_q_d_n4, eq46_e420_q_d_n5, eq46_e420_q_d_n6, eq46_e420_q_d_n7, eq46_e420_q_d_n8, eq46_e420_q_d_n9, eq46_e420_q_d_n10, eq46_e420_q_d_n11, eq46_e420_q_d_n12, eq46_e420_q_d_n13, eq46_e420_q_d_n14, eq46_e420_q_d_n15, eq46_e420_q_d_b0, eq46_e420_q_d_b1, eq46_e420_q_d_b2, eq46_e420_q_d_b3, eq46_e420_q_d_b4, eq46_e420_q_d_b5, eq46_e420_q_d_b6, eq46_e420_q_d_b7, eq46_e420_q_d_b8, eq46_e420_q_d_b9, eq46_e420_q_d_b10, eq46_e420_q_d_b11, eq46_e420_q_d_b12, eq46_e420_q_d_b13, eq46_e420_q_d_b14,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        let eq46_e415: f64 = (-s.v[118]);
        let eq46_e415_d_n0: f64 = (-s.dn[118][0]);
        let eq46_e415_d_n1: f64 = (-s.dn[118][1]);
        let eq46_e415_d_n2: f64 = (-s.dn[118][2]);
        let eq46_e415_d_n3: f64 = (-s.dn[118][3]);
        let eq46_e415_d_n4: f64 = (-s.dn[118][4]);
        let eq46_e415_d_n5: f64 = (-s.dn[118][5]);
        let eq46_e415_d_n6: f64 = (-s.dn[118][6]);
        let eq46_e415_d_n7: f64 = (-s.dn[118][7]);
        let eq46_e415_d_n8: f64 = (-s.dn[118][8]);
        let eq46_e415_d_n9: f64 = (-s.dn[118][9]);
        let eq46_e415_d_n10: f64 = (-s.dn[118][10]);
        let eq46_e415_d_n11: f64 = (-s.dn[118][11]);
        let eq46_e415_d_n12: f64 = (-s.dn[118][12]);
        let eq46_e415_d_n13: f64 = (-s.dn[118][13]);
        let eq46_e415_d_n14: f64 = (-s.dn[118][14]);
        let eq46_e415_d_n15: f64 = (-s.dn[118][15]);
        let eq46_e415_d_b0: f64 = (-s.db[118][0]);
        let eq46_e415_d_b1: f64 = (-s.db[118][1]);
        let eq46_e415_d_b2: f64 = (-s.db[118][2]);
        let eq46_e415_d_b3: f64 = (-s.db[118][3]);
        let eq46_e415_d_b4: f64 = (-s.db[118][4]);
        let eq46_e415_d_b5: f64 = (-s.db[118][5]);
        let eq46_e415_d_b6: f64 = (-s.db[118][6]);
        let eq46_e415_d_b7: f64 = (-s.db[118][7]);
        let eq46_e415_d_b8: f64 = (-s.db[118][8]);
        let eq46_e415_d_b9: f64 = (-s.db[118][9]);
        let eq46_e415_d_b10: f64 = (-s.db[118][10]);
        let eq46_e415_d_b11: f64 = (-s.db[118][11]);
        let eq46_e415_d_b12: f64 = (-s.db[118][12]);
        let eq46_e415_d_b13: f64 = (-s.db[118][13]);
        let eq46_e415_d_b14: f64 = (-s.db[118][14]);
        let eq46_e417: f64 = (eq46_e415 * (nv14 - 0.0));
        let eq46_e417_d_n0: f64 = (eq46_e415_d_n0 * (nv14 - 0.0));
        let eq46_e417_d_n1: f64 = (eq46_e415_d_n1 * (nv14 - 0.0));
        let eq46_e417_d_n2: f64 = (eq46_e415_d_n2 * (nv14 - 0.0));
        let eq46_e417_d_n3: f64 = (eq46_e415_d_n3 * (nv14 - 0.0));
        let eq46_e417_d_n4: f64 = (eq46_e415_d_n4 * (nv14 - 0.0));
        let eq46_e417_d_n5: f64 = (eq46_e415_d_n5 * (nv14 - 0.0));
        let eq46_e417_d_n6: f64 = (eq46_e415_d_n6 * (nv14 - 0.0));
        let eq46_e417_d_n7: f64 = (eq46_e415_d_n7 * (nv14 - 0.0));
        let eq46_e417_d_n8: f64 = (eq46_e415_d_n8 * (nv14 - 0.0));
        let eq46_e417_d_n9: f64 = (eq46_e415_d_n9 * (nv14 - 0.0));
        let eq46_e417_d_n10: f64 = (eq46_e415_d_n10 * (nv14 - 0.0));
        let eq46_e417_d_n11: f64 = (eq46_e415_d_n11 * (nv14 - 0.0));
        let eq46_e417_d_n12: f64 = (eq46_e415_d_n12 * (nv14 - 0.0));
        let eq46_e417_d_n13: f64 = (eq46_e415_d_n13 * (nv14 - 0.0));
        let eq46_e417_d_n14: f64 = ((eq46_e415_d_n14 * (nv14 - 0.0)) + eq46_e415);
        let eq46_e417_d_n15: f64 = (eq46_e415_d_n15 * (nv14 - 0.0));
        let eq46_e417_d_b0: f64 = (eq46_e415_d_b0 * (nv14 - 0.0));
        let eq46_e417_d_b1: f64 = (eq46_e415_d_b1 * (nv14 - 0.0));
        let eq46_e417_d_b2: f64 = (eq46_e415_d_b2 * (nv14 - 0.0));
        let eq46_e417_d_b3: f64 = (eq46_e415_d_b3 * (nv14 - 0.0));
        let eq46_e417_d_b4: f64 = (eq46_e415_d_b4 * (nv14 - 0.0));
        let eq46_e417_d_b5: f64 = (eq46_e415_d_b5 * (nv14 - 0.0));
        let eq46_e417_d_b6: f64 = (eq46_e415_d_b6 * (nv14 - 0.0));
        let eq46_e417_d_b7: f64 = (eq46_e415_d_b7 * (nv14 - 0.0));
        let eq46_e417_d_b8: f64 = (eq46_e415_d_b8 * (nv14 - 0.0));
        let eq46_e417_d_b9: f64 = (eq46_e415_d_b9 * (nv14 - 0.0));
        let eq46_e417_d_b10: f64 = (eq46_e415_d_b10 * (nv14 - 0.0));
        let eq46_e417_d_b11: f64 = (eq46_e415_d_b11 * (nv14 - 0.0));
        let eq46_e417_d_b12: f64 = (eq46_e415_d_b12 * (nv14 - 0.0));
        let eq46_e417_d_b13: f64 = (eq46_e415_d_b13 * (nv14 - 0.0));
        let eq46_e417_d_b14: f64 = (eq46_e415_d_b14 * (nv14 - 0.0));
        let eq46_e418_q: f64 = eq46_e417;
        (eq46_e417, eq46_e417_d_n0, eq46_e417_d_n1, eq46_e417_d_n2, eq46_e417_d_n3, eq46_e417_d_n4, eq46_e417_d_n5, eq46_e417_d_n6, eq46_e417_d_n7, eq46_e417_d_n8, eq46_e417_d_n9, eq46_e417_d_n10, eq46_e417_d_n11, eq46_e417_d_n12, eq46_e417_d_n13, eq46_e417_d_n14, eq46_e417_d_n15, eq46_e417_d_b0, eq46_e417_d_b1, eq46_e417_d_b2, eq46_e417_d_b3, eq46_e417_d_b4, eq46_e417_d_b5, eq46_e417_d_b6, eq46_e417_d_b7, eq46_e417_d_b8, eq46_e417_d_b9, eq46_e417_d_b10, eq46_e417_d_b11, eq46_e417_d_b12, eq46_e417_d_b13, eq46_e417_d_b14, eq46_e418_q, eq46_e417_d_n0, eq46_e417_d_n1, eq46_e417_d_n2, eq46_e417_d_n3, eq46_e417_d_n4, eq46_e417_d_n5, eq46_e417_d_n6, eq46_e417_d_n7, eq46_e417_d_n8, eq46_e417_d_n9, eq46_e417_d_n10, eq46_e417_d_n11, eq46_e417_d_n12, eq46_e417_d_n13, eq46_e417_d_n14, eq46_e417_d_n15, eq46_e417_d_b0, eq46_e417_d_b1, eq46_e417_d_b2, eq46_e417_d_b3, eq46_e417_d_b4, eq46_e417_d_b5, eq46_e417_d_b6, eq46_e417_d_b7, eq46_e417_d_b8, eq46_e417_d_b9, eq46_e417_d_b10, eq46_e417_d_b11, eq46_e417_d_b12, eq46_e417_d_b13, eq46_e417_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 16] = [eq46_e420_q_d_n0, eq46_e420_q_d_n1, eq46_e420_q_d_n2, eq46_e420_q_d_n3, eq46_e420_q_d_n4, eq46_e420_q_d_n5, eq46_e420_q_d_n6, eq46_e420_q_d_n7, eq46_e420_q_d_n8, eq46_e420_q_d_n9, eq46_e420_q_d_n10, eq46_e420_q_d_n11, eq46_e420_q_d_n12, eq46_e420_q_d_n13, eq46_e420_q_d_n14, eq46_e420_q_d_n15];
        let eq46_reactive_branch_derivatives: [f64; 15] = [eq46_e420_q_d_b0, eq46_e420_q_d_b1, eq46_e420_q_d_b2, eq46_e420_q_d_b3, eq46_e420_q_d_b4, eq46_e420_q_d_b5, eq46_e420_q_d_b6, eq46_e420_q_d_b7, eq46_e420_q_d_b8, eq46_e420_q_d_b9, eq46_e420_q_d_b10, eq46_e420_q_d_b11, eq46_e420_q_d_b12, eq46_e420_q_d_b13, eq46_e420_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq57_e532, eq57_e532_d_n11, eq57_e532_q, eq57_e532_q_d_n11,) = {
    if s.b[124] {
        let eq57_e529: f64 = (p.p58 * (nv11 - 0.0));
        let eq57_e529_d_n11: f64 = p.p58;
        let eq57_e530_q: f64 = eq57_e529;
        (eq57_e529, eq57_e529_d_n11, eq57_e530_q, eq57_e529_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[11]),
            None,
            nodes[11],
            multiplicity * (eq57_e532_q_d_n11),
        );
    }
}
